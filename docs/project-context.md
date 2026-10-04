# Project context and decision provenance

Distilled from the project conversations and repository setup on 2026-10-03–04. This note preserves motivation and distinguishes chosen direction from implementation proposals. The [ladder](ladder.md) remains the direction; the [roadmap](roadmap.md) tracks delivery.

## Why this project exists

The user’s product intent is a **browser app to build intuition about plasmas**. A useful increment lets someone change conditions and understand the response. That learning experience remains valuable as the numerical machinery grows more capable.

The electron point-of-view conversations supplied a generative perspective: start with what an entity receives, how it responds, what it exchanges with its surroundings, and what continuity it maintains. The user’s panpsychist framing and the name **SELFS** are part of the idea’s origin. Preserve that motivation when translating it into computational questions.

The [SELFS abstract](SELFS_WOFI_abstract.md) makes the operational move: seek organizations whose coupled responses remain predictable for named observables, horizons, and tolerances. Their constituents can change, and their components need not all move alike. A SELF is therefore more than a visually coherent blob or a collection of nearby particles. The proposed computational test concerns predictive descriptions and their cost; it does not establish subjective experience.

## What is chosen, implemented, or proposed

| Item                                                                                                                                   | Status and origin                                                                                 | Consequence for future work                                                                           |
| -------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------- |
| Browser experience for plasma intuition                                                                                                | User’s product intent                                                                             | Judge increments by what someone can do or understand.                                                |
| Rust for scientific meaning; WGSL/WebGPU for parallel numerics; TypeScript/Svelte for the interface; browser APIs for local facilities | User’s chosen ownership split                                                                     | Preserve these responsibilities even when execution starts small.                                     |
| Toybox → Laboratory → adaptive SELFS                                                                                                   | Direction supplied in the ladder                                                                  | Ship useful earlier rungs and retain them as the destination develops.                                |
| Cargo/npm workspaces, Vite shell, small Rust contract crate, CI                                                                        | Implemented during repository setup; recorded in [decision 0001](architecture/0001-foundation.md) | These are practical starting choices, open to revision when a real use requires it.                   |
| Charge-separation oscillator as the first experiment                                                                                   | Assistant proposal during setup; [brief](experiments/charge-separation.md)                        | A concrete candidate for a complete first loop, not a model selection mandated by the user or ladder. |
| Adaptive entity descriptions reduce total computational cost                                                                           | Research hypothesis in the SELFS abstract                                                         | Requires a reference comparison at matched requirements, including transition and recovery costs.     |

The oscillator proposal should not quietly become the project’s scientific destination. Its purpose is to exercise interaction, numerical comparison, and persistence. It does not itself test structure formation or SELF discovery. A richer model needs an explicit correspondence with that teaching experiment.

## A useful way to develop the POV idea

As a design proposal, an entity-focused inspection could answer four questions: **what influences this entity, how does it respond, what does it exchange, and which relations persist?** Start with quantities actually supported by the model. Attach a view to simulation observations or an explicitly identified candidate; keep its provenance visible.

This offers a route from the original intuition to an inspectable interface without requiring a new force law or an autonomous agent for every particle. It is an optional presentation direction, not an implemented feature or a requirement to add narrative text to every experiment.

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
