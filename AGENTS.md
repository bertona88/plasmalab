# Working on PlasmaLab

Read `docs/ladder.md` before architectural work. It is the direction; `docs/SELFS_WOFI_abstract.md` is a research hypothesis, not a validated algorithm. Consult `docs/architecture/0001-foundation.md` and `docs/roadmap.md` for the current implementation scope.

Read `docs/project-context.md` for motivation and decision provenance. In particular, the first oscillator experiment is an assistant proposal, not a model choice mandated by the user or the ladder.

## Ownership

- Rust owns scientific rules, assumptions, units, validation, and model semantics.
- WGSL/WebGPU executes parallel numerical work accountable to those rules.
- TypeScript/Svelte expresses interventions and observations. It must not own authoritative physical state or quietly reimplement a model.
- Browser APIs own local storage, workers, file exchange, and optional audio.
- A rendering frame does not define a physical timestep. Drop presentation work or slow simulated progress before changing scientific fidelity.

## Build through the ladder

- Prefer complete, narrow user experiences. Do not build a universal solver framework, arbitrary model coupling, generic multirate scheduling, or distributed execution speculatively.
- Keep experiment identity distinct from a run, solver, backend, and stored state. Keep model-specific state model-specific.
- Distinguish simulation observations, detected candidates, predictions, and presentation projections, with provenance and applicable uncertainty.
- SELFS progress through observe → track → predict → advise → act. Rung 2 observers and shadow predictors cannot mutate authoritative evolution.
- A representation transition needs accounting without duplicate contributions, explicit losses, validity checks, and an actual recovery or stop policy. An error alarm cannot reconstruct discarded detail.
- Do not equate visual plausibility, implementation agreement, convergence, and predictive validity. State what was tested and at what cost.

## Repository practice

- Keep the supplied direction documents intact unless the user requests a revision. Put implementation decisions in `docs/architecture/`.
- Maintain accurate status in the README, roadmap, and app. A scaffold is not a completed rung.
- Add dependencies, crates, generated schemas, workers, and GPU kernels when an implemented slice needs them.
- Do not add a license or change distribution rights without the owner's decision.
- Use `npm ci`; commit `package-lock.json` and `Cargo.lock`. Never commit build products, secrets, or dependency directories.
- Run `npm run verify` and the WASM compilation check for affected changes. Tests should protect scientific or integration risks; avoid tests that merely repeat implementation text.
- Include assumptions, acceptance criteria, and relevant evidence with scientific changes. Explain any checks that could not run.
- At completion, commit and push to the agreed repository and verify the remote commit. Never force-push or overwrite unrelated work. Do not claim work is saved remotely until verified.
