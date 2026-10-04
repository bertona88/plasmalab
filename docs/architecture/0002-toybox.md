# 0002 — Complete the toy loop with a Rust model and worker boundary

Date: 2026-10-04  
Status: adopted for the first vertical slice

## Context and choice

The user’s [implementation mission](../source-inventory.md#first-slice-implementation-mission-2026-10-04) selects a 2D plasmoid/reconnection-inspired toy unless a more mature experiment is already present. The foundation contained no solver, so the earlier assistant-proposed oscillator does not take priority. The [ladder](../ladder.md) permits an explicitly invented universe at Rung 1.

Use a small, bounded Rust scalar-field toy, compiled to WASM and hosted in a module worker. Svelte presents commands, observations and disposable render samples. Add no WebGPU runtime until measurements and a reference comparison justify it. The complete interaction is the architectural test; this decision does not claim PIC, MHD, physical reconnection or validated SELFS.

## Ownership and execution

Rust owns the model identity/version, admitted parameters, presets, initial state, fixed timestep, interventions, evolution, diagnostics, candidate detection and checkpoint validation. The [experiment brief](../experiments/plasmoid-ecology.md) describes the exact invented rules. A candidate detector reads model state and has no evolution authority.

The TypeScript execution adapter offers a narrow asynchronous command, frame, snapshot and restore interface. Its worker owns the Rust/WASM model. Commands cross the boundary; callers receive observations and copied presentation samples, not a mutable view of authoritative solver storage. This modest-grid implementation copies samples for rendering, so it establishes ownership rather than a zero-copy performance claim. A later GPU executor can keep its working buffers resident while returning the same experiment-level observations; implementing one still requires an equivalence test and may refine the presentation contract.

Physical evolution uses fixed toy timesteps. The browser requests two model ticks, then waits at least 40 ms before the next batch, and renders completed frames; wall-clock frame intervals never become a model timestep. Slow machines make less simulated progress instead of silently changing the update rule. Pause stops requesting tick batches; single-step requests one model tick through the same boundary. Browser rendering and field colors have no route back into authoritative fields except explicit user interventions.

## Experiment, checkpoint and observations

Experiment metadata is separate from evolving state. It identifies the question, model/version, parameters and initial conditions, supported observations, assumptions and interventions. A checkpoint adds the model-specific state necessary to resume. A render frame contains a projection plus observations; it is not a restart artifact or a universal world state.

Rust derives the browser wire contract from the same serialized types that define the model API. Generated contracts are checked for drift. The TypeScript side can express user intent and render the result without maintaining another implementation of the toy rules.

Checkpoints use JSON format `plasmalab-flux-snapshot`, schema version `1`, model `bounded-flux-toy` version `1.0.0`. They contain experiment identity, initial and current parameters, seed, step-indexed intervention history, requested observations and assumptions, plus the current flux, activity, step, random-generator state and event counters. Restore validates the complete payload before replacing the current model. Unsupported versions and invalid state cause an explicit error; there is no guessed migration or silent reset. Loading means restoring a saved state, while reset rebuilds the initial state with its original parameters and seed and clears history. The event history explains interventions; the checkpoint fields supply resumption.

Browser storage uses IndexedDB with one local checkpoint slot. Saving again replaces that slot. Export/import uses the same JSON checkpoint, with an 8 MB size limit and Rust validation on import. Browser storage is origin-local and can be cleared; export is the portable backup. Storage failure is reported while leaving the running model available. This is local continuity, with no account, cloud synchronization or durable-backup claim for IndexedDB alone.

## Consequences and next boundaries

The first slice exercises the lasting responsibilities without inventing a generic solver registry, universal state, or multirate scheduler. CPU execution is sufficient for an initial measured baseline. The future WGSL implementation must preserve the declared toy semantics or introduce an explicit new model version.

Candidates remain observations at **observe**. Frame-local labels do not claim continuity, lineage, physical plasmoids or computational authority. Tracking and split/merge histories require their own association rules, ambiguity handling and tests. A stronger physical model needs equations, regime, units, boundary conditions, resolution and reference checks; it cannot inherit validation from this toy’s appearance.

The format limits interventions to 10,000 and runs to 1,000,000,000 steps, reporting errors at those limits rather than silently truncating history or wrapping counters. Derived diagnostics are recomputed from the restored state. Presentation charts keep at most 100 recent observation samples, use independent vertical scales, and clear on reset or load; that chart history is not a scientific archive.

Deterministic stepping and checkpoint continuation are implementation checks within tested builds. Cross-browser/CPU bitwise equality, physical validity, convergence to MHD/PIC, and adaptive speedups are separate, unestablished claims. See the [roadmap](../roadmap.md) for gates before adding those responsibilities.
