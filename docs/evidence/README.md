# Evidence and reference cases

There are no numerical reference results yet. The current Rust unit tests validate measurement requirements; they are not physics validation.

Every implemented model needs a simple reference path and an experiment brief. Store small reproducible configurations and summaries here; keep large outputs out of git and record their durable location and content hash when needed.

## Required record for a scientific result

| Field        | What to record                                                                                                      |
| ------------ | ------------------------------------------------------------------------------------------------------------------- |
| Question     | Experiment identity and the claim being tested                                                                      |
| Provenance   | Commit, model/representation versions, backend, numeric precision, hardware/runtime when relevant                   |
| Conditions   | Units/normalization, initial and boundary conditions, interventions and their simulation times, random seed if used |
| Method       | Integrator, timestep/resolution, assumptions, valid regime, reference method                                        |
| Requirement  | Named observables, horizon, error metric, requested tolerances                                                      |
| Outcome      | Measured errors, uncertainty where applicable, acceptance result and failure cases                                  |
| Reproduction | Command/configuration, inputs, expected tolerance; distinguish replay from checkpoint restart                       |
| Cost         | Measured scope, warmup, repetitions and variability, total costs at matched requirements                            |

Keep **visual plausibility**, **implementation agreement**, **numerical convergence**, and **predictive validity** separate. A successful native/WASM/GPU build establishes none of those by itself. CPU/GPU agreement does not prove model validity.

For future adaptive claims, include discovery, tracking, prediction, checking, exchanges/coupling, transition, and recovery overheads. Record rejected transitions and lost information. A slower method may still be a useful observation tool; report it accurately.

## Historical reports awaiting reproduction

The [PAN-RAD report](pan-rad-reported.md) preserves an earlier assistant's radiation–matter method, algorithm sketch, reported checks, and the negative comparison against fixed-block preconditioning. Its original executable artifacts and outputs have not been recovered or independently rerun. It is a source report, not a numerical reference result for PlasmaLab. Keep that distinction when recovering or extending the prototype.
