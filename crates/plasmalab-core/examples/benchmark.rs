//! Native CPU evidence, not browser throughput or scientific validation.
use plasmalab_core::{contract::DT, model::FluxToy};
use std::time::Instant;
fn main() {
    let steps = 1200u32;
    let mut model = FluxToy::new("cascade").unwrap();
    model.step(120).unwrap();
    let start = Instant::now();
    for _ in 0..steps / 120 {
        model.step(120).unwrap();
    }
    let elapsed = start.elapsed().as_secs_f64();
    let start = Instant::now();
    let samples = 120;
    let mut bytes = 0;
    for _ in 0..samples {
        bytes = serde_json::to_string(&model.frame()).unwrap().len();
    }
    let frame_elapsed = start.elapsed().as_secs_f64();
    println!(
        "grid=128x80 dt={DT} preset=cascade measured_steps={steps} step_seconds={elapsed:.6} steps_per_second={:.1} frame_plus_json_ms={:.3} frame_json_bytes={bytes}",
        steps as f64 / elapsed,
        frame_elapsed * 1000.0 / samples as f64
    );
    println!(
        "observations={}",
        serde_json::to_string(&model.observe()).unwrap()
    );
    for preset in ["quiet", "tearing", "cascade", "driven"] {
        let mut model = FluxToy::new(preset).unwrap();
        for checkpoint in [0, 120, 600, 1200] {
            while model.state().step < checkpoint {
                model
                    .step((checkpoint - model.state().step).min(120))
                    .unwrap();
            }
            let o = model.observe();
            println!(
                "preset_check={}",
                serde_json::json!({
                    "preset": preset, "step": checkpoint, "stress_energy": o.stress_energy,
                    "total_events": o.total_events, "candidate_count": o.candidate_count,
                })
            );
        }
    }
}
