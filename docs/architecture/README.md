# Architecture boundaries

The [ladder](../ladder.md) is authoritative direction. [Decision 0001](0001-foundation.md) translates it into the smallest current repository structure.

| Concern              | Owner                                           | Current implementation                                  | Next concrete addition                                |
| -------------------- | ----------------------------------------------- | ------------------------------------------------------- | ----------------------------------------------------- |
| Scientific meaning   | Rust                                            | Measurement requirements and evidence/result categories | One model, its assumptions, and reference diagnostics |
| Numerical execution  | Model-specific Rust baseline; later WGSL/WebGPU | None                                                    | Fixed-step CPU baseline, then browser WASM adapter    |
| Human experience     | TypeScript/Svelte                               | Honest foundation shell and roadmap links               | Experiment controls and observation view              |
| Local continuity     | Browser APIs                                    | None                                                    | Versioned save/reopen plus file export/import         |
| Scientific evidence  | Rust tests and experiment evidence records      | Input-validation unit tests only                        | Analytic comparison and timestep convergence          |
| SELFS interpretation | Future observers/predictors                     | None                                                    | Only after a measured reference run exists            |

## Contracts to preserve

An **experiment** owns the question, conventions, requested measurements and accuracy, initial conditions, and interventions. A **run** names a specific model/representation version, execution configuration, and evolution. A **checkpoint** carries enough model-specific state for the promised resumption. An **observation** names its run, simulation time, observable, units, method, and applicable uncertainty. A **presentation** projects observations for a person.

These are responsibilities, not a requirement to implement five frameworks today. There is no canonical full save schema yet. Introduce it with the first real state to save, version it from the beginning, and keep experiment identity independent of execution. Do not publish an empty envelope as a restart format.

Rust is the authority for scientific contracts. Once the first browser integration requires serialized types, derive or generate the wire schema from Rust and check it for drift in CI; do not hand-maintain independent physics definitions in TypeScript. This scaffold has no duplicated wire contract.

The interface will request operations such as an intervention or a named observation through a narrow adapter. It must not depend on a live mutable Rust array, a GPU buffer layout, or rendering geometry as physical state. Keep large state near execution and transfer bounded observations. A worker can host the first real WASM run when that slice is built; worker code is not needed for today's static shell.

## Boundaries for later work

- A second model must declare supported questions and explicit correspondence with the first. Shared metadata does not establish equivalent state or valid coupling.
- Detection, tracking, and shadow prediction read observations. Their identities are independent of buffer indexes and particle membership. Lineage, containment, and interaction are different relations.
- Overlapping observed candidates are allowed. Overlapping computational responsibility must never double-count evolved content or exchanged quantities.
- Adaptive authority requires a recorded decision, accounting, named validity criteria, information-loss policy, and tested recovery or explicit stop. Defer this machinery until one concrete transition can be evaluated.
- Rendering, observation cadence, and physics stepping are separate. A later multirate method needs its own numerical justification.
- CPU/GPU comparisons establish only the tested implementation agreement. Cross-device bitwise equality is not promised.
- Local browser work must remain useful without a service. Optional remote execution is a later backend, not a prerequisite for adaptive SELFS.

## Persistence acceptance criteria

The first save format must preserve experiment identity, model and representation versions, conventions, initial conditions, interventions at simulation times, important observations, and the state needed for the documented resume behavior. Export/import is required alongside local storage. History alone is not a checkpoint.

Reject unsupported versions or corrupt/nonfinite state visibly; never silently reset the experiment or guess a migration. Schema migration must preserve known meaning and identify any lost capability. Deterministic replay is only promised where tested; otherwise record numerical tolerances and environment information for comparisons.

## Recovered alternatives and design sketches

Basis: the separately supplied “Computational Architecture Patterns” excerpts, recorded in the [source inventory](../source-inventory.md#computational-architecture-patterns). These were **assistant proposals**, not additional adopted decisions. They explain alternatives behind the ladder; [decision 0001](0001-foundation.md) still describes the current foundation.

The lean alternative put small Rust/WASM models behind a Svelte interface and Canvas/WebGL presentation, to ship and debug a complete interaction quickly. Its proposed direct simulation-array interface was convenient but risked tying the UI to CPU storage. The browser-laboratory alternative added a simulation worker, resident GPU buffers and WGSL execution, transmitting commands and compact diagnostics instead of large state every frame. The moonshot added shadow predictors and a representation manager; it was not a reason to implement that machinery before a reference experiment existed.

An earlier “add WebGPU immediately” recommendation and the current deferred GPU dependency are therefore different staging proposals, not a contradiction to conceal by changing current code. Keep the execution boundary without installing a speculative runtime. A native `wgpu` route and optional later backend were discussed, not promised multi-GPU capability or equivalent browser fallbacks.

Other suggestions were on-demand derived-data computation; separate lineage, containment and interaction graphs; selected-region spatial/spectral views and time-series phase diagnostics; local presets/snapshots/events; and optional sonification of observed oscillations or reconnection events. Audio mappings are presentation choices, not extra physical dynamics. The sampling and temporal history needed for frequency/k-space diagnostics remain implementation choices, not something supplied by the old diagram.

Suggested packages included `wasm-bindgen`, `serde`/`serde-wasm-bindgen`, `wgpu`, `bytemuck`, `glam`, `petgraph` and `rustfft`. They are not a dependency mandate or verified compatibility matrix. Bevy, a large Rust web framework, early Rayon/WASM thread pools and a full scientific framework were alternatives the assistant advised postponing to avoid overhead and setup complexity. Browser support, memory-sharing prerequisites and actual bottlenecks need checking when a concrete slice requires them.

The excerpt's illustrative cadence (`electrons dt`, `fields 2dt`, `ions 16dt`, diagnostics/detection `64dt`, display around 16 ms) is **not a justified coupled numerical method**. Do not paste it into a scheduler. Decouple presentation/diagnostics cadence from physical stepping; any change to coupled physical rates must be validated as part of the model.

### Original illustrative Rust, not implementation artifacts

These declarations are retained from the three-architecture assistant excerpt because they show how the proposed interface changed. They are alternative sketches, not one module. Referenced types are undefined, there are no method bodies or dependency versions, and the chat supplied no compilable crate or test results for them. Read them as historical design input; do not add them to `plasmalab-core` or treat them as an API to implement verbatim.

Lean sketch:

```rust
trait PlasmaModel {
    fn reset(&mut self, config: &Config);
    fn step(&mut self, dt: f32);
    fn render_field(&self) -> &[f32];
}
```

Browser-laboratory sketch:

```rust
trait PlasmaModel {
    fn configure(&mut self, experiment: &Experiment);
    fn advance(&mut self, runtime: &mut Runtime, dt: f32);
    fn diagnostics(&self) -> Diagnostics;
}
```

Predictive-entity sketch:

```rust
struct SelfState {
    id: SelfId,
    model: Predictor,
    confidence: f32,
    prediction_error: f32,
    age: f64,
    children: Vec<SelfId>,
}
```

What survives is a narrow scientific/execution boundary and prediction records. What does not follow is that every model exposes one borrowed scalar render field, that arbitrary caller-selected timesteps are valid, or that one unexplained confidence number establishes predictive authority. Concrete contracts must name observations, horizons, units, errors and supported operations as the ladder requires. Graph type suggestions and ping-pong buffer diagrams were likewise illustrative; no original GPU kernel or executable prototype was supplied here.
