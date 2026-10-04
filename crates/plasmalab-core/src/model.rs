//! Bounded, driven flux toy. These equations are invented teaching rules.
//!
//! For unit grid spacing, ψ₀(y)=0.12√((y−39.5)²+6²), q=ψ−ψ₀,
//! w(y)=exp(−((y−39.5)/13)²), and J=−8 Δψ. The explicit update is
//! ψ' = ψ + dt [η Δψ − u D_upwind,x ψ − v D_upwind,y ψ
//!             + .18 drive w tanh(q/.4) − .06 q + .006 drive w ξ].
//! η=.06+.65 relaxation a. Activity fires to 1 if |J|>threshold and a<.25;
//! otherwise a'=a+dt(.25 Δa−.5a). ξ is seeded uniform noise in [−1,1].
//! The flow is prescribed; it is not plasma momentum. Its vortical part comes
//! from a streamfunction, plus an imposed horizontal shear. Discrete upwinding
//! dissipates flux contrasts. No physical conservation law is asserted.
//!
//! With validated controls, dt(4η+|u|+|v|+.06)<1, so transport/diffusion/damping
//! have nonnegative convex weights. Bounded forcing preserves |ψ|<=12.
//! Interventions are separately clipped to this documented bound.
use crate::contract::*;
use crate::detector::detect;
use std::f64::consts::TAU;

#[derive(Debug, Clone)]
pub struct FluxToy {
    experiment: Experiment,
    state: SimulationState,
    background: Vec<f64>,
    sheet: Vec<f64>,
    next_flux: Vec<f64>,
    next_activity: Vec<f64>,
}

pub fn background_at(y: usize) -> f64 {
    let dy = y as f64 - (HEIGHT - 1) as f64 * 0.5;
    0.12 * (dy * dy + 36.0).sqrt()
}

fn random_signed(rng: &mut u32) -> f64 {
    *rng ^= *rng << 13;
    *rng ^= *rng >> 17;
    *rng ^= *rng << 5;
    (*rng as f64 / u32::MAX as f64) * 2.0 - 1.0
}

impl FluxToy {
    pub fn new(preset: &str) -> Result<Self, String> {
        let catalog = catalog();
        let selected = catalog
            .presets
            .iter()
            .find(|item| item.id == preset)
            .ok_or_else(|| format!("Unknown preset: {preset}"))?;
        let seed = 73129;
        let experiment = Experiment {
            id: format!("flux-toy-{preset}-{seed}"),
            version: 1,
            model: MODEL.into(),
            model_version: MODEL_VERSION.into(),
            preset: preset.into(),
            parameters: selected.parameters.clone(),
            initial_conditions: InitialConditions {
                seed,
                width: WIDTH as u32,
                height: HEIGHT as u32,
                parameters: selected.parameters.clone(),
            },
            interventions: Vec::new(),
            requested_observations: [
                "stress_energy",
                "active_sites",
                "cell_threshold_events",
                "flux_anomaly_candidates",
            ]
            .into_iter()
            .map(str::to_owned)
            .collect(),
            assumptions: catalog.assumptions,
        };
        Ok(Self::from_initial(experiment))
    }

    fn from_initial(experiment: Experiment) -> Self {
        let mut rng = experiment.initial_conditions.seed;
        let background: Vec<_> = (0..HEIGHT).map(background_at).collect();
        let sheet: Vec<_> = (0..HEIGHT)
            .map(|y| {
                let dy = y as f64 - (HEIGHT - 1) as f64 * 0.5;
                (-(dy / 13.0).powi(2)).exp()
            })
            .collect();
        let mut flux = vec![0.0; WIDTH * HEIGHT];
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let phase = TAU * x as f64 / WIDTH as f64;
                let seed_wave = match experiment.preset.as_str() {
                    "quiet" => 0.025 * (2.0 * phase).cos(),
                    "tearing" => 0.35 * (3.0 * phase).cos() + 0.08 * phase.sin(),
                    "cascade" => 0.38 * (7.0 * phase).cos() + 0.19 * (13.0 * phase + 0.7).sin(),
                    _ => 0.6 * (2.0 * phase + 0.05 * y as f64).cos() + 0.15 * (5.0 * phase).sin(),
                };
                let noise_amplitude = if experiment.preset == "quiet" {
                    0.001
                } else {
                    0.018
                };
                let noise = random_signed(&mut rng) * noise_amplitude;
                flux[y * WIDTH + x] = background[y] + sheet[y] * (seed_wave + noise);
                if y == 0 || y == HEIGHT - 1 {
                    flux[y * WIDTH + x] = background[y];
                }
            }
        }
        Self {
            experiment,
            state: SimulationState {
                step: 0,
                flux,
                activity: vec![0.0; WIDTH * HEIGHT],
                rng,
                events_this_step: 0,
                total_events: 0.0,
            },
            background,
            sheet,
            next_flux: vec![0.0; WIDTH * HEIGHT],
            next_activity: vec![0.0; WIDTH * HEIGHT],
        }
    }

    pub fn experiment(&self) -> &Experiment {
        &self.experiment
    }
    pub fn state(&self) -> &SimulationState {
        &self.state
    }

    pub fn reset(&mut self, preset: Option<&str>) -> Result<(), String> {
        if let Some(preset) = preset {
            *self = Self::new(preset)?;
        } else {
            let mut experiment = self.experiment.clone();
            experiment.interventions.clear();
            experiment.parameters = experiment.initial_conditions.parameters.clone();
            *self = Self::from_initial(experiment);
        }
        Ok(())
    }

    pub fn configure(&mut self, parameters: Parameters) -> Result<(), String> {
        parameters.validate()?;
        self.history_capacity()?;
        self.experiment
            .interventions
            .push(Intervention::Parameters {
                step: self.state.step,
                parameters: parameters.clone(),
            });
        self.experiment.parameters = parameters;
        Ok(())
    }

    fn history_capacity(&self) -> Result<(), String> {
        if self.experiment.interventions.len() >= MAX_HISTORY {
            Err("Intervention history is full (10,000); export this experiment and reset to begin a new history.".into())
        } else {
            Ok(())
        }
    }

    pub fn perturb(&mut self, x: f64, y: f64, strength: f64, radius: f64) -> Result<(), String> {
        validate_perturbation(x, y, strength, radius)?;
        self.history_capacity()?;
        let cx = x * WIDTH as f64;
        let cy = y * (HEIGHT - 1) as f64;
        let r = radius * HEIGHT as f64;
        for row in 1..HEIGHT - 1 {
            for col in 0..WIDTH {
                let raw_dx = (col as f64 - cx).abs();
                let dx = raw_dx.min(WIDTH as f64 - raw_dx);
                let dy = row as f64 - cy;
                let bump = strength * (-0.5 * (dx * dx + dy * dy) / (r * r)).exp();
                let index = row * WIDTH + col;
                self.state.flux[index] =
                    (self.state.flux[index] + bump).clamp(-FIELD_BOUND, FIELD_BOUND);
            }
        }
        self.experiment.interventions.push(Intervention::Perturb {
            step: self.state.step,
            x,
            y,
            strength,
            radius,
        });
        Ok(())
    }

    /// Fixed timestep is a model property; presentation cadence can only change tick count.
    pub fn step(&mut self, count: u32) -> Result<(), String> {
        if !(1..=120).contains(&count) || self.state.step > MAX_STEPS - count {
            return Err(
                "Step count must be 1–120 and the run must remain below 1,000,000,000 steps."
                    .into(),
            );
        }
        for _ in 0..count {
            self.tick();
        }
        Ok(())
    }

    fn tick(&mut self) {
        let p = &self.experiment.parameters;
        let time = self.state.step as f64 * DT;
        let phase = time * 0.032;
        let kx = TAU * 3.0 / WIDTH as f64;
        let ky = TAU / (HEIGHT - 1) as f64;
        let mut sin_x = [0.0; WIDTH];
        let mut cos_x = [0.0; WIDTH];
        for x in 0..WIDTH {
            let a = kx * x as f64 + phase;
            (sin_x[x], cos_x[x]) = a.sin_cos();
        }
        let mut events = 0;
        for y in 0..HEIGHT {
            let dy = y as f64 - (HEIGHT - 1) as f64 * 0.5;
            let (sin_y, cos_y) = (ky * y as f64).sin_cos();
            let shear = (dy / 9.0).tanh();
            for x in 0..WIDTH {
                let i = y * WIDTH + x;
                if y == 0 || y == HEIGHT - 1 {
                    self.next_flux[i] = self.background[y];
                    self.next_activity[i] = 0.0;
                    continue;
                }
                let left = y * WIDTH + (x + WIDTH - 1) % WIDTH;
                let right = y * WIDTH + (x + 1) % WIDTH;
                let up = i - WIDTH;
                let down = i + WIDTH;
                let f = &self.state.flux;
                let a = &self.state.activity;
                let lap = f[left] + f[right] + f[up] + f[down] - 4.0 * f[i];
                let event = (8.0 * lap).abs() > p.threshold && a[i] < 0.25;
                let activity = if event {
                    events += 1;
                    1.0
                } else {
                    a[i] + DT
                        * (0.25 * (a[left] + a[right] + a[up] + a[down] - 4.0 * a[i]) - 0.5 * a[i])
                };
                self.next_activity[i] = activity;
                let eta = 0.06 + 0.65 * p.relaxation * a[i];
                // Streamfunction amplitude is chosen so the validated CFL bound holds.
                let u = p.stirring * (0.3 * shear + 0.2 * sin_x[x] * cos_y);
                let v = -p.stirring * 0.2 * (kx / ky) * cos_x[x] * sin_y;
                let advection = if u >= 0.0 {
                    -u * (f[i] - f[left])
                } else {
                    -u * (f[right] - f[i])
                } + if v >= 0.0 {
                    -v * (f[i] - f[up])
                } else {
                    -v * (f[down] - f[i])
                };
                let q = f[i] - self.background[y];
                let source = 0.18 * p.drive * self.sheet[y] * (q / 0.4).tanh() - 0.06 * q
                    + 0.006 * p.drive * self.sheet[y] * random_signed(&mut self.state.rng);
                self.next_flux[i] = f[i] + DT * (eta * lap + advection + source);
            }
        }
        std::mem::swap(&mut self.state.flux, &mut self.next_flux);
        std::mem::swap(&mut self.state.activity, &mut self.next_activity);
        self.state.step += 1;
        self.state.events_this_step = events;
        self.state.total_events += events as f64;
    }

    pub fn observe(&self) -> Observations {
        let f = &self.state.flux;
        let mut energy = 0.0;
        for y in 1..HEIGHT - 1 {
            for x in 0..WIDTH {
                let i = y * WIDTH + x;
                let dx =
                    (f[y * WIDTH + (x + 1) % WIDTH] - f[y * WIDTH + (x + WIDTH - 1) % WIDTH]) * 0.5;
                let dy = (f[i + WIDTH] - f[i - WIDTH]) * 0.5;
                energy += 0.5 * (dx * dx + dy * dy);
            }
        }
        let candidates = detect(f, &self.background, WIDTH, HEIGHT);
        Observations {
            stress_energy: energy / (WIDTH * (HEIGHT - 2)) as f64,
            active_sites: self.state.activity.iter().filter(|&&a| a > 0.25).count() as u32,
            events_this_step: self.state.events_this_step,
            total_events: self.state.total_events,
            candidate_count: candidates.len() as u32,
            largest_candidate: candidates.iter().map(|c| c.area).max().unwrap_or(0),
            candidates,
        }
    }

    pub fn frame(&self) -> Frame {
        let f = &self.state.flux;
        let mut current = vec![0.0; WIDTH * HEIGHT];
        for y in 1..HEIGHT - 1 {
            for x in 0..WIDTH {
                let i = y * WIDTH + x;
                current[i] = -8.0
                    * (f[y * WIDTH + (x + 1) % WIDTH]
                        + f[y * WIDTH + (x + WIDTH - 1) % WIDTH]
                        + f[i - WIDTH]
                        + f[i + WIDTH]
                        - 4.0 * f[i]);
            }
        }
        Frame {
            experiment: self.experiment.clone(),
            width: WIDTH as u32,
            height: HEIGHT as u32,
            step: self.state.step,
            time: self.state.step as f64 * DT,
            dt: DT,
            field: FieldSample {
                flux: f.clone(),
                current,
                activity: self.state.activity.clone(),
            },
            observations: self.observe(),
        }
    }

    pub fn snapshot(&self) -> Snapshot {
        Snapshot {
            format: "plasmalab-flux-snapshot".into(),
            version: FORMAT_VERSION,
            experiment: self.experiment.clone(),
            state: self.state.clone(),
        }
    }

    /// Validate a complete candidate before replacing any authoritative state.
    pub fn restore(&mut self, snapshot: Snapshot) -> Result<(), String> {
        validate_snapshot(&snapshot)?;
        let mut replacement = Self::from_initial(snapshot.experiment);
        replacement.state = snapshot.state;
        *self = replacement;
        Ok(())
    }

    pub fn dispatch(&mut self, command: Command) -> Result<Response, String> {
        let mut response = Response::empty();
        match command {
            Command::Catalog => {
                response.catalog = Some(catalog());
                return Ok(response);
            }
            Command::Snapshot => {
                response.snapshot = Some(self.snapshot());
                return Ok(response);
            }
            Command::Observe => {}
            Command::Step { count } => self.step(count)?,
            Command::Reset { preset } => self.reset(preset.as_deref())?,
            Command::Configure { parameters } => self.configure(parameters)?,
            Command::Perturb {
                x,
                y,
                strength,
                radius,
            } => self.perturb(x, y, strength.unwrap_or(-0.9), radius.unwrap_or(0.07))?,
            Command::Restore { snapshot } => self.restore(*snapshot)?,
        }
        response.frame = Some(self.frame());
        Ok(response)
    }
}

fn validate_perturbation(x: f64, y: f64, strength: f64, radius: f64) -> Result<(), String> {
    if !x.is_finite()
        || !(0.0..=1.0).contains(&x)
        || !y.is_finite()
        || !(0.0..=1.0).contains(&y)
        || !strength.is_finite()
        || !(-2.0..=2.0).contains(&strength)
        || !radius.is_finite()
        || !(0.015..=0.2).contains(&radius)
    {
        Err("Perturbations require x,y in [0,1], strength in [−2,2], radius in [0.015,0.2].".into())
    } else {
        Ok(())
    }
}

pub fn validate_snapshot(snapshot: &Snapshot) -> Result<(), String> {
    let e = &snapshot.experiment;
    let s = &snapshot.state;
    if snapshot.format != "plasmalab-flux-snapshot"
        || snapshot.version != FORMAT_VERSION
        || e.model != MODEL
        || e.model_version != MODEL_VERSION
        || e.version != 1
    {
        return Err(
            "Unsupported snapshot/model version; this file cannot be resumed by this model.".into(),
        );
    }
    if e.id.trim().is_empty()
        || e.id.len() > 200
        || !catalog().presets.iter().any(|p| p.id == e.preset)
        || e.initial_conditions.width != WIDTH as u32
        || e.initial_conditions.height != HEIGHT as u32
        || e.initial_conditions.seed == 0
        || s.rng == 0
        || s.step > MAX_STEPS
    {
        return Err(
            "Invalid experiment identity, initial conditions, random state, or step.".into(),
        );
    }
    e.parameters.validate()?;
    e.initial_conditions.parameters.validate()?;
    if e.assumptions != catalog().assumptions
        || e.requested_observations != SelfObservations::names()
    {
        return Err(
            "Snapshot scientific assumptions or observation requests do not match this model."
                .into(),
        );
    }
    if s.flux.len() != WIDTH * HEIGHT
        || s.activity.len() != WIDTH * HEIGHT
        || s.flux
            .iter()
            .any(|v| !v.is_finite() || v.abs() > FIELD_BOUND)
        || s.activity
            .iter()
            .any(|v| !v.is_finite() || !(0.0..=1.0).contains(v))
        || s.events_this_step as usize > WIDTH * (HEIGHT - 2)
        || !s.total_events.is_finite()
        || s.total_events < s.events_this_step as f64
        || s.total_events.fract() != 0.0
        || s.total_events > s.step as f64 * (WIDTH * (HEIGHT - 2)) as f64
    {
        return Err("Snapshot fields or counters are invalid/out of bounds.".into());
    }
    for y in [0, HEIGHT - 1] {
        for x in 0..WIDTH {
            if s.flux[y * WIDTH + x] != background_at(y) || s.activity[y * WIDTH + x] != 0.0 {
                return Err("Snapshot violates fixed boundary conditions.".into());
            }
        }
    }
    if e.interventions.len() > MAX_HISTORY {
        return Err("Snapshot intervention history is too large.".into());
    }
    let mut previous = 0;
    let mut parameters = &e.initial_conditions.parameters;
    for intervention in &e.interventions {
        if intervention.step() < previous || intervention.step() > s.step {
            return Err("Snapshot history has invalid step ordering.".into());
        }
        previous = intervention.step();
        match intervention {
            Intervention::Parameters { parameters: p, .. } => {
                p.validate()?;
                parameters = p;
            }
            Intervention::Perturb {
                x,
                y,
                strength,
                radius,
                ..
            } => validate_perturbation(*x, *y, *strength, *radius)?,
        }
    }
    if parameters != &e.parameters {
        return Err("Current parameters disagree with intervention history.".into());
    }
    Ok(())
}

struct SelfObservations;
impl SelfObservations {
    fn names() -> Vec<String> {
        [
            "stress_energy",
            "active_sites",
            "cell_threshold_events",
            "flux_anomaly_candidates",
        ]
        .into_iter()
        .map(str::to_owned)
        .collect()
    }
}

#[cfg(test)]
#[path = "model_tests.rs"]
mod tests;
