use super::*;

#[test]
fn batching_observation_and_fixed_seed_do_not_change_evolution() {
    let mut first = FluxToy::new("cascade").unwrap();
    let mut second = first.clone();
    for _ in 0..120 {
        first.step(1).unwrap();
        let _ = first.observe();
    }
    second.step(120).unwrap();
    assert_eq!(first.snapshot(), second.snapshot());
}

#[test]
fn reset_restores_original_initial_conditions_and_removes_history() {
    let mut model = FluxToy::new("tearing").unwrap();
    let initial = model.snapshot();
    model.step(30).unwrap();
    model.perturb(0.4, 0.5, -1.2, 0.08).unwrap();
    let mut parameters = model.experiment.parameters.clone();
    parameters.drive = 1.6;
    model.configure(parameters).unwrap();
    model.reset(None).unwrap();
    assert_eq!(model.snapshot(), initial);
    assert!(model.reset(Some("missing")).is_err());
    assert_eq!(model.snapshot(), initial);
}

#[test]
fn full_json_checkpoint_restarts_exactly_in_the_same_runtime() {
    let mut original = FluxToy::new("driven").unwrap();
    original.step(57).unwrap();
    original.perturb(0.97, 0.46, -0.94, 0.061).unwrap();
    let mut parameters = original.experiment.parameters.clone();
    parameters.drive = 1.25;
    original.configure(parameters).unwrap();
    let encoded = serde_json::to_string(&original.snapshot()).unwrap();
    let mut restored = FluxToy::new("quiet").unwrap();
    restored
        .restore(serde_json::from_str(&encoded).unwrap())
        .unwrap();
    assert_eq!(original.snapshot(), restored.snapshot());
    original.step(120).unwrap();
    restored.step(120).unwrap();
    assert_eq!(original.snapshot(), restored.snapshot());
}

#[test]
fn bad_parameters_interventions_and_step_requests_are_atomic() {
    let mut model = FluxToy::new("quiet").unwrap();
    let initial = model.snapshot();
    for value in [-1.0, 2.01, f64::INFINITY, f64::NAN] {
        let mut p = initial.experiment.parameters.clone();
        p.drive = value;
        assert!(model.configure(p).is_err());
    }
    assert!(model.perturb(f64::NAN, 0.5, 1.0, 0.1).is_err());
    assert!(model.perturb(0.5, 0.5, 2.1, 0.1).is_err());
    assert!(model.perturb(0.5, 0.5, 1.0, 0.0).is_err());
    assert!(model.step(0).is_err());
    assert!(model.step(121).is_err());
    assert_eq!(model.snapshot(), initial);
}

#[test]
fn all_parameter_boundaries_are_checked() {
    let reference = catalog().presets[0].parameters.clone();
    for field in 0..4 {
        for value in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY, -0.1, 2.1] {
            let mut p = reference.clone();
            match field {
                0 => p.drive = value,
                1 => p.threshold = value,
                2 => p.relaxation = value,
                _ => p.stirring = value,
            }
            assert!(p.validate().is_err());
        }
    }
    assert!(
        Parameters {
            drive: 0.0,
            threshold: 0.02,
            relaxation: 0.0,
            stirring: 0.0
        }
        .validate()
        .is_ok()
    );
    assert!(
        Parameters {
            drive: 2.0,
            threshold: 1.5,
            relaxation: 1.0,
            stirring: 2.0
        }
        .validate()
        .is_ok()
    );
}

#[test]
fn corrupted_checkpoints_fail_without_replacing_live_state() {
    let mut model = FluxToy::new("tearing").unwrap();
    model.step(3).unwrap();
    let original = model.snapshot();
    let mut cases = Vec::new();
    let mut bad = original.clone();
    bad.version = 200;
    cases.push(bad);
    let mut bad = original.clone();
    bad.state.flux.pop();
    cases.push(bad);
    let mut bad = original.clone();
    bad.state.flux[500] = f64::NAN;
    cases.push(bad);
    let mut bad = original.clone();
    bad.state.activity[500] = 1.1;
    cases.push(bad);
    let mut bad = original.clone();
    bad.state.rng = 0;
    cases.push(bad);
    let mut bad = original.clone();
    bad.state.flux[0] += 0.001;
    cases.push(bad);
    let mut bad = original.clone();
    bad.experiment.parameters.drive = 2.0;
    cases.push(bad);
    let mut bad = original.clone();
    bad.experiment.assumptions.clear();
    cases.push(bad);
    let mut bad = original.clone();
    bad.experiment.interventions.push(Intervention::Perturb {
        step: 4,
        x: 0.5,
        y: 0.5,
        strength: 1.0,
        radius: 0.1,
    });
    cases.push(bad);
    for bad in cases {
        assert!(model.restore(bad).is_err());
        assert_eq!(model.snapshot(), original);
    }
}

#[test]
fn representative_presets_remain_finite_bounded_and_behaviorally_distinct() {
    let mut summaries = Vec::new();
    for preset in ["quiet", "tearing", "cascade", "driven"] {
        let mut model = FluxToy::new(preset).unwrap();
        for _ in 0..10 {
            model.step(120).unwrap();
            validate_snapshot(&model.snapshot()).unwrap();
        }
        let o = model.observe();
        assert!(o.stress_energy.is_finite());
        summaries.push((o.stress_energy, o.total_events, o.candidate_count));
    }
    assert_eq!(
        summaries[0].1, 0.0,
        "quiet baseline should not fire spontaneously"
    );
    assert!(
        summaries[2].1 > summaries[1].1,
        "low-threshold cascade should fire more than tearing"
    );
    assert!(summaries[3].1 > 0.0);
    for a in 0..summaries.len() {
        for b in a + 1..summaries.len() {
            assert_ne!(summaries[a], summaries[b]);
        }
    }
}

#[test]
fn extreme_valid_controls_and_repeated_perturbations_preserve_bounds() {
    let mut model = FluxToy::new("cascade").unwrap();
    model
        .configure(Parameters {
            drive: 2.0,
            threshold: 0.02,
            relaxation: 1.0,
            stirring: 2.0,
        })
        .unwrap();
    for _ in 0..30 {
        model.perturb(0.5, 0.5, 2.0, 0.2).unwrap();
    }
    for _ in 0..10 {
        model.step(120).unwrap();
        validate_snapshot(&model.snapshot()).unwrap();
    }
}

#[test]
fn perturbation_changes_measured_state_and_records_its_cause() {
    let mut model = FluxToy::new("quiet").unwrap();
    let before = model.observe().stress_energy;
    model.perturb(0.5, 0.5, -1.6, 0.04).unwrap();
    assert_ne!(model.observe().stress_energy, before);
    assert_eq!(model.experiment.interventions.len(), 1);
    model.step(1).unwrap();
    assert!(model.observe().events_this_step > 0);
}
