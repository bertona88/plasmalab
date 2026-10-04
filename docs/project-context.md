# Project context and decision provenance

Distilled from the supplied documents, available project-conversation summaries/excerpts, and the repository-setup discussion on 2026-10-03–04. On 2026-10-04, five retrieved turns from “Electron Experience Imagined” added the IRSF proposal below, and five available turns from “Plasma Pattern Zoology” added candidate families and experimental options. Other earlier context and prototype artifacts remain incomplete or unavailable. The [source inventory](source-inventory.md) records coverage and gaps.

This note preserves motivation and distinguishes chosen direction from implementation proposals. The [ladder](ladder.md) remains the direction; the [roadmap](roadmap.md) tracks delivery.

## Why this project exists

The user’s product intent is a **browser app to build intuition about plasmas**. A useful increment lets someone change conditions and understand the response. That learning experience remains valuable as the numerical machinery grows more capable.

The electron point-of-view conversations supplied a generative perspective: start with what an entity receives, how it responds, what it exchanges with its surroundings, and what continuity it maintains. The user’s panpsychist framing and the name **SELFS** are part of the idea’s origin. Preserve that motivation when translating it into computational questions.

The [SELFS abstract](SELFS_WOFI_abstract.md) makes the operational move: seek organizations whose coupled responses remain predictable for named observables, horizons, and tolerances. Their constituents can change, and their components need not all move alike. A SELF is therefore more than a visually coherent blob or a collection of nearby particles. The proposed computational test concerns predictive descriptions and their cost; it does not establish subjective experience.

## What is chosen, implemented, or proposed

| Item                                                                                                                                   | Status and origin                                                                                 | Consequence for future work                                                                                                                        |
| -------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------- |
| Browser experience for plasma intuition                                                                                                | User’s product intent                                                                             | Judge increments by what someone can do or understand.                                                                                             |
| Rust for scientific meaning; WGSL/WebGPU for parallel numerics; TypeScript/Svelte for the interface; browser APIs for local facilities | User’s chosen ownership split                                                                     | Preserve these responsibilities even when execution starts small.                                                                                  |
| Toybox → Laboratory → adaptive SELFS                                                                                                   | Direction supplied in the ladder                                                                  | Ship useful earlier rungs and retain them as the destination develops.                                                                             |
| Cargo/npm workspaces, Vite shell, small Rust contract crate, CI                                                                        | Implemented during repository setup; recorded in [decision 0001](architecture/0001-foundation.md) | These are practical starting choices, open to revision when a real use requires it.                                                                |
| Charge-separation oscillator as the first experiment                                                                                   | Assistant proposal during setup; [brief](experiments/charge-separation.md)                        | A concrete candidate for a complete first loop, not a model selection mandated by the user or ladder.                                              |
| Adaptive entity descriptions reduce total computational cost                                                                           | Research hypothesis in the SELFS abstract                                                         | Requires a reference comparison at matched requirements, including transition and recovery costs.                                                  |
| Intrinsic–Relational Simulation Framework (IRSF) and an “interior” renderer                                                            | Assistant speculation responding to the user’s request in “Electron Experience Imagined”          | Preserve as conceptual motivation and optional interpretation; no implementation or validation, and no user adoption shown in the retrieved turns. |

The oscillator proposal should not quietly become the project’s scientific destination. Its purpose is to exercise interaction, numerical comparison, and persistence. It does not itself test structure formation or SELF discovery. A richer model needs an explicit correspondence with that teaching experiment.

## Plasma Pattern Zoology: candidate identity, not just recurring shapes

In “Plasma Pattern Zoology” (2026-10-04), the user asked about the cellular-automaton resemblance of recursive plasmoid formation, solar plasmoids, and finally the **zoology of SELFS in plasmas**. These questions establish the interest in persistent organizations. The candidate classification, toy rules, diagnostic axes, and detector display below were assistant proposals; the available turns contain no user adoption of a particular solver or scoring scheme, no executable artifact, and no measured result.

The useful research question is whether describing a structure as an entity improves prediction of its future and exchanges compared with treating it as an arbitrary region. Persistence of relations despite constituent turnover motivates the question; appearance or a species label does not answer it.

The assistant organized candidates by the proposed **carrier of continuity**. These are overlapping research lenses, not an exhaustive taxonomy, biological ancestry, or a claim that every named structure meets the operational SELF criterion:

| Lens        | Candidates suggested in the chat                       | Organization to investigate                                                                                                                                                                                       |
| ----------- | ------------------------------------------------------ | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Topological | Plasmoids, magnetic islands, flux ropes                | Field geometry/topology, currents, magnetic flux, and twist; proposed history: formation by tearing, growth, merger, or ejection.                                                                                 |
| Flow        | Vortices, zonal flows, streamers                       | Circulation, vorticity, or a shear profile that persists as constituents change.                                                                                                                                  |
| Transport   | Blobs, filaments                                       | A localized density/pressure profile, polarization, and coherent transport. The sketched blob state was `(n, T, R, v, φ, orientation)`; units, the meaning of `R`, and predictive sufficiency were not specified. |
| Boundary    | Current sheets, double layers, shocks, sheaths         | A persistent relation between environments: field reversal/current or a potential structure. A material membrane is not required.                                                                                 |
| Kinetic     | Phase-space holes, trapped populations                 | Organization in a distribution such as `f(x, v)`; a spatial outline alone may omit the relevant identity.                                                                                                         |
| Wave        | Solitons, coherent mode/wave packets, Alfvénic packets | Phase/amplitude relations or a proposed nonlinear–dispersive balance, with finite validity.                                                                                                                       |
| Composite   | Assemblies containing several candidate structures     | Nested organization and changing lineage: sheets fragment into islands; candidates merge into a successor. Containment, interaction, and descent remain different relations.                                      |

These cues came from the assistant's scientific discussion, whose citation targets were not recoverable in the reviewed text. They need model-specific definitions and literature grounding before use as detector or closure assumptions. In particular, the chat cautioned against treating a 2D magnetic island picture as the literal geometry of a 3D flux rope.

### Proposed diagnostics and an inspectable history

The assistant suggested a profile `(P, B, M, A, R)`: persistence, boundedness/identifiability, memory, autonomy relative to immediate forcing, and regeneration after perturbation. Treat these as questions to operationalize, not an established measurement scale or universal “selfhood” score. Boundedness can refer to separability in the relevant representation; it must not become a requirement for a closed spatial body.

The illustrative profiles for a linear wave, blob, and plasmoid were arbitrary ordinal sketches, not comparative evidence. Likewise, a mock inspector showing a current-sheet age of `4.3 τ_A`, “predictability: 0.91,” three internal modes, and `TEARING` status was a product illustration, not a run. A future inspector could show candidate identity, age, the evidence for continuity, and split/merge history while the reference fields continue evolving. Any prediction display must instead identify its observable, horizon, error measure, and evaluated data, as required by the ladder.

The solar and _Sundiver_ discussion supplied an evocative analogy. Organized plasma is not evidence of life, intelligence, or subjective experience. The numerical solar-observation claims are unrecovered reported claims, not repository evidence; see the source inventory.

### Relationship to current scope

The [plasmoid-ecology brief](experiments/plasmoid-ecology.md) preserves the proposed CA-like rules and a possible physical comparison. It is a research option, not a replacement for the first slice or the abstract's driven electron–ion benchmark. A comprehensive taxonomy remains deferred by the ladder. The chat's particle-to-one-blob compression illustration does not establish a compression ratio or speedup, and entity detection does not grant evolution authority.

## A useful way to develop the POV idea

As a design proposal, an entity-focused inspection could answer four questions: **what influences this entity, how does it respond, what does it exchange, and which relations persist?** Start with quantities actually supported by the model. Attach a view to simulation observations or an explicitly identified candidate; keep its provenance visible.

This offers a route from the original intuition to an inspectable interface without requiring a new force law or an autonomous agent for every particle. It is an optional presentation direction, not an implemented feature or a requirement to add narrative text to every experiment.

### Recovered IRSF proposal: meaning and open definitions

**Source and attribution:** “Electron Experience Imagined,” 2026-10-03, conversation `6ac16e66-e678-83ed-9ff9-0aee14c9907f`, framework-request turn `bbb21e2e-a2df-4bb3-8f5f-0ace6703f6d0`. The user asked for a new simulation framework centered on interior experience, without internet search. The assistant named and sketched IRSF. That request authorizes exploration; the retrieved conversation does not show the user choosing IRSF as PlasmaLab’s architecture. The formulas, comparisons, renderer, and experiment sequence below are assistant proposals, not user decisions or verified results. The original response supplied no executable code, numerical run, or literature review.

IRSF pairs ordinary exterior dynamics with a hypothetical intrinsic state `X_i`, intended to describe what a physical state might be like from within under the panpsychist premise. It does not change the exterior equations. The assistant sketched a mapping `X_e = Φ(F, u, ψ, ∇F, curvature, interaction history)` from local electromagnetic relations, motion, quantum state, gradients, spacetime curvature, and history. This is a list of candidate inputs, not a defined function or a new measurable physical variable. Here `u` consistently denotes motion; the source formula and its prose used inconsistent symbols.

The proposed axes separate rapid differentiation from persistent organization:

| Axis                  | Source sketch                                                              | Definition still needed                                                                                                             |
| --------------------- | -------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------- |
| Differentiation `D_i` | `‖dX_i/dτ‖`, with proper time `τ`                                          | State mapping, geometry/norm, units, normalization, and supported time convention.                                                  |
| Integration `C(S)`    | Internal causal dependence divided by internal plus external dependence    | Subsystem boundary, causal estimator/interventions, dependence measure, and evaluation horizon. The ratio was explicitly schematic. |
| Memory `M_S(Δτ)`      | `I[X_S(τ); X_S(τ−Δτ)]`, with `I` an information measure                    | Ensemble or sampling procedure, estimator, lag, and distinction between statistical persistence and subjective memory.              |
| Self-reference `R_S`  | Present dynamics encode and predict the system’s own future internal state | Operational observable, prediction target, baseline, and scoring method. No equation was supplied.                                  |

The response imagined a strongly driven electron or turbulent plasma with rapid change but little persistent integration, and contrasted this with a recurrent nervous system. Those assignments and the displayed example scores were illustrative, not measurements. Neither rapid change nor a large particle count establishes a unified subject. The axes are not calibrated consciousness measures; their values depend on definitions that remain open.

### Composition hypotheses and proposed comparisons

The central speculative question was when many putative interiors might become one. The assistant introduced `X_S = B(X_1, …, X_n; Γ_S)`, where `Γ_S` describes causal relations and `B` is an unspecified binding operator; the proposed joint state is not a simple sum. Alternatives to compare on the same physical systems were integration alone, recurrent causal loops, predictive self-modeling, synchronized multiscale dynamics, and **no genuine combination**. None was selected or rejected by evidence.

The proposed sequence was one electron → two electrons → atom → molecule → many molecules → membrane → autocatalytic chemical network → neuron → interacting neurons → increasingly recurrent networks. This is a conceptual comparison program, not an implemented experiment or a requirement to add chemistry and neuroscience solvers to PlasmaLab. No common model, quantitative acceptance criterion, or observation capable of discriminating subjective binding was supplied. Comparing these hypotheses would initially compare their assigned interpretations; it would not by itself test whether anything experiences them.

### Renderer proposal and limits

The assistant proposed paired exterior and “interior” views: select a candidate, inspect relational change, persistence, integration, and self-reference, then enter an abstract view of contractions, expansions, discontinuities, symmetry changes, attractors, and bifurcations. The reason was to explore relations rather than draw a human-like mind inside a particle. The source’s label-free immersive view is a presentation idea; its meaning, assumed mapping, provenance, and limits still need to be accessible to the user.

The black-hole thought experiment motivated a local clock and a view driven by changing local relations, rather than a visual event triggered merely by crossing a global boundary. It also proposed stopping the renderer with “INTERIOR UNDEFINED — THEORY EXCEEDED” when the physical description loses reliability. These were narrative examples, not relativistic simulations. The proposal did not resolve a common clock for composite systems or how its proper-time construction would apply to its photon example. These questions need a specified physical model before implementation. Stopping an interpretation is also distinct from stopping physical evolution or recovering discarded state; the ladder’s recovery requirements still apply.

### Relationship to the current direction

IRSF’s hypothetical intrinsic state differs from a SELF’s compact **predictive** internal state. The SELFS abstract supplies an operational test for selected observations and exchanges; IRSF supplies no observable test of interior experience. Its integration and binding ideas therefore do not replace the SELF criterion. In particular, synchronized motion and predictive self-modeling are competing IRSF suggestions, not prerequisites for a SELF: the abstract permits differently responding components and externally driven organizations whose coupled behavior remains predictable.

The reusable design idea is to compare interpretations of a shared reference run while preserving exterior dynamics, provenance, and visible failure. A possible continuation, proposed in this review rather than decided in the source, is to choose a model-supported relational observation for the optional POV inspector and document its mapping before rendering it. Keep physical observations, candidate identity, shadow predictions, and speculative projections distinct using the existing result vocabulary. Leave consciousness scores, binding operators, quantum/gravity models, and automatic authority out of the first slice. This preserves the motivating question without overriding the ladder or treating a striking visualization as evidence of experience.

## Open choices for the first implementation

- Whether to use the proposed oscillator or another equally small experiment that completes the Toybox loop.
- The selected model’s normalization, admissible parameter range, integrator, timestep, observables, comparison horizon, and acceptance tolerances.
- The first concrete run/checkpoint format and generated Rust-to-browser contract, derived from actual state and required observations.
- How an eventual spatial experiment connects to the initial teaching model; which questions and observables remain comparable, and which do not.

Resolve these choices as the relevant slice is implemented. The existing folder structure does not settle them, and an unimplemented abstraction is not needed to reserve them.

## Continuity for the next contributor

At foundation completion, CI checked the web build and types, Rust requirement tests and linting, and compilation for WASM. Those checks establish scaffold health, not simulation accuracy or a working browser experiment. Consult the roadmap for subsequent progress.

Treat conversation sketches and earlier prototypes as inputs to inspect and verify before calling them implemented capabilities. A file name or a remembered result is not reproducible evidence in this repository.

Completed work must survive the temporary development machine: commit, publish to the agreed repository, and verify the remote result. The user explicitly requested this working practice. Preserve useful reasoning alongside code so the next session can continue from the decisions rather than reconstructing the conversation.
