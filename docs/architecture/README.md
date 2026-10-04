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
