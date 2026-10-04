//! Versioned, model-specific wire contracts. TypeScript declarations come from these types.
use serde::{Deserialize, Serialize};
use ts_rs::TS;

pub const MODEL: &str = "bounded-flux-toy";
pub const MODEL_VERSION: &str = "1.0.0";
pub const FORMAT_VERSION: u32 = 1;
pub const WIDTH: usize = 128;
pub const HEIGHT: usize = 80;
pub const DT: f64 = 0.12;
pub const FIELD_BOUND: f64 = 12.0;
pub const MAX_HISTORY: usize = 10_000;
pub const MAX_STEPS: u32 = 1_000_000_000;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, TS)]
#[serde(deny_unknown_fields)]
pub struct Parameters {
    pub drive: f64,
    pub threshold: f64,
    pub relaxation: f64,
    pub stirring: f64,
}
impl Parameters {
    pub fn validate(&self) -> Result<(), String> {
        for (key, value, min, max) in [
            ("drive", self.drive, 0.0, 2.0),
            ("threshold", self.threshold, 0.02, 1.5),
            ("relaxation", self.relaxation, 0.0, 1.0),
            ("stirring", self.stirring, 0.0, 2.0),
        ] {
            if !value.is_finite() || value < min || value > max {
                return Err(format!("{key} must be finite and between {min} and {max}"));
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, TS)]
#[serde(deny_unknown_fields)]
pub struct InitialConditions {
    pub seed: u32,
    pub width: u32,
    pub height: u32,
    pub parameters: Parameters,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, TS)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub enum Intervention {
    Parameters {
        step: u32,
        parameters: Parameters,
    },
    Perturb {
        step: u32,
        x: f64,
        y: f64,
        strength: f64,
        radius: f64,
    },
}
impl Intervention {
    pub fn step(&self) -> u32 {
        match self {
            Self::Parameters { step, .. } | Self::Perturb { step, .. } => *step,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, TS)]
#[serde(deny_unknown_fields)]
pub struct Experiment {
    pub id: String,
    pub version: u32,
    pub model: String,
    pub model_version: String,
    pub preset: String,
    pub parameters: Parameters,
    pub initial_conditions: InitialConditions,
    pub interventions: Vec<Intervention>,
    pub requested_observations: Vec<String>,
    pub assumptions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, TS)]
#[serde(deny_unknown_fields)]
pub struct SimulationState {
    pub step: u32,
    pub flux: Vec<f64>,
    pub activity: Vec<f64>,
    pub rng: u32,
    pub events_this_step: u32,
    pub total_events: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, TS)]
#[serde(deny_unknown_fields)]
pub struct Snapshot {
    pub format: String,
    pub version: u32,
    pub experiment: Experiment,
    pub state: SimulationState,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, TS)]
pub struct Candidate {
    /// Observation-local index, deliberately not a tracked identity.
    pub id: u32,
    pub x: f64,
    pub y: f64,
    pub area: u32,
    pub radius: f64,
    pub polarity: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, TS)]
pub struct Observations {
    /// Mean 0.5 |grad psi|² in grid units; not a conserved physical energy.
    pub stress_energy: f64,
    pub active_sites: u32,
    pub events_this_step: u32,
    pub total_events: f64,
    pub candidate_count: u32,
    pub largest_candidate: u32,
    pub candidates: Vec<Candidate>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, TS)]
pub struct FieldSample {
    pub flux: Vec<f64>,
    pub current: Vec<f64>,
    pub activity: Vec<f64>,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, TS)]
pub struct Frame {
    pub experiment: Experiment,
    pub width: u32,
    pub height: u32,
    pub step: u32,
    pub time: f64,
    pub dt: f64,
    pub field: FieldSample,
    pub observations: Observations,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, TS)]
pub struct Preset {
    pub id: String,
    pub name: String,
    pub description: String,
    pub parameters: Parameters,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, TS)]
pub struct ParameterSpec {
    pub key: String,
    pub label: String,
    pub min: f64,
    pub max: f64,
    pub step: f64,
    pub description: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, TS)]
pub struct Catalog {
    pub model: String,
    pub model_version: String,
    pub width: u32,
    pub height: u32,
    pub dt: f64,
    pub presets: Vec<Preset>,
    pub parameters: Vec<ParameterSpec>,
    pub assumptions: Vec<String>,
    pub rules: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, TS)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub enum Command {
    Catalog,
    Observe,
    Step {
        count: u32,
    },
    Reset {
        #[ts(optional)]
        preset: Option<String>,
    },
    Configure {
        parameters: Parameters,
    },
    Perturb {
        x: f64,
        y: f64,
        #[ts(optional)]
        strength: Option<f64>,
        #[ts(optional)]
        radius: Option<f64>,
    },
    Snapshot,
    Restore {
        snapshot: Box<Snapshot>,
    },
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, TS)]
pub struct Response {
    pub ok: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub frame: Option<Frame>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub snapshot: Option<Snapshot>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub catalog: Option<Catalog>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub error: Option<String>,
}
impl Response {
    pub fn empty() -> Self {
        Self {
            ok: true,
            frame: None,
            snapshot: None,
            catalog: None,
            error: None,
        }
    }
    pub fn error(error: String) -> Self {
        Self {
            ok: false,
            error: Some(error),
            ..Self::empty()
        }
    }
}

pub fn catalog() -> Catalog {
    let presets = [
        ("quiet", "Quiet sheet", "Weak seeds decay. Increase loading or poke the sheet to create a disturbance.", [0.08, 0.9, 0.6, 0.08]),
        ("tearing", "At the threshold", "A few broad seeds compete as loading amplifies the sheet's flux contrast.", [0.8, 0.3, 0.55, 0.35]),
        ("cascade", "Island cascade", "Many short seeds, stronger loading, and a low threshold create frequent local release events.", [1.6, 0.19, 0.85, 0.75]),
        ("driven", "Stirred sheet", "A strong imposed flow stretches and brings seeded patches together. Explore their changing outlines.", [1.0, 0.26, 0.65, 1.8]),
    ].into_iter().map(|(id, name, description, p)| Preset {
        id: id.into(), name: name.into(), description: description.into(),
        parameters: Parameters { drive: p[0], threshold: p[1], relaxation: p[2], stirring: p[3] },
    }).collect();
    let parameters = [
        ("drive", "Sheet loading", 0.0, 2.0, 0.05, "Amplifies signed flux departures around the central sheet; an invented, saturating feedback."),
        ("threshold", "Release threshold", 0.02, 1.5, 0.01, "The |−8 laplacian ψ| threshold for a local reconnection-like release; lower means easier triggering."),
        ("relaxation", "Release strength", 0.0, 1.0, 0.05, "How strongly triggered activity smooths nearby flux gradients."),
        ("stirring", "External stirring", 0.0, 2.0, 0.05, "Strength of the imposed shear and vortical transport; this is prescribed flow, not solved momentum."),
    ].into_iter().map(|(key,label,min,max,step,description)| ParameterSpec {
        key:key.into(),label:label.into(),min,max,step,description:description.into(),
    }).collect();
    Catalog {
        model: MODEL.into(), model_version: MODEL_VERSION.into(), width: WIDTH as u32,
        height: HEIGHT as u32, dt: DT, presets, parameters,
        assumptions: [
            "Invented 2D flux toy. Not quantitative MHD, PIC, or a model of a particular plasma.",
            "All fields, distances, and time use toy grid units; no SI calibration or conservation of physical energy is claimed.",
            "Horizontal boundaries are periodic; the top and bottom flux are fixed to an opposed-field background.",
            "Loading, seeded noise, damping, and imposed stirring add or remove flux/stress. Reconnection-like events are local threshold-triggered cell firings, not verified topology changes.",
            "Candidate rings mark connected flux-anomaly patches, not confirmed plasmoids or SELFS. IDs are observation-local; no identity tracking, split/merge classification, or prediction is performed.",
            "Snapshots restore every evolving field, counters, random generator, and experiment history. Exact continuation is promised only in the same model version and execution environment.",
        ].into_iter().map(str::to_owned).collect(),
        rules: [
            "The scalar flux ψ defines a visual magnetic-like field B=(∂yψ,−∂xψ). Its background points in opposite directions above and below the sheet.",
            "Every fixed 0.12-unit tick advects and smooths ψ. Near the sheet, loading amplifies deviations with a saturating feedback; damping pulls them toward the background.",
            "If |−8 laplacian ψ| exceeds the threshold while local activity is below 0.25, that cell fires. It can fire again after cooling even without a new threshold crossing. Activity spreads, fades, and temporarily increases smoothing.",
            "Clicking adds a bounded Gaussian flux bump; reversing the sign removes flux locally. This is an intervention, not a physical laser or particle injection.",
            "Contours and colors are presentation. Stress is mean ½|∇ψ|²; cell events count threshold firings. Candidate detection only reads the state.",
        ].into_iter().map(str::to_owned).collect(),
    }
}
