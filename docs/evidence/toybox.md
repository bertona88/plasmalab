# Bounded flux toy: implementation evidence

Date: 2026-10-04. Model `bounded-flux-toy` version `1.0.0`, checkpoint format version 1.

This record checks the first [plasmoid-inspired toy experiment](../experiments/plasmoid-ecology.md). It is implementation evidence for invented rules, not physical validation, numerical convergence to MHD/PIC, or evidence for adaptive SELFS.

## Reproduce

```sh
npm ci
cargo install wasm-bindgen-cli --version "$(node scripts/wasm-bindgen-version.mjs)" --locked
npm run verify
cargo check --workspace --locked --target wasm32-unknown-unknown
npx playwright install --with-deps chromium
npm run test:browser
cargo run --release --locked --example benchmark
npm run benchmark:wasm
```

The production build includes the actual Rust/WASM model and its worker. Browser tests inspect checkpoints exported by the application, rather than substituting a JavaScript simulation. Generated TypeScript declarations are checked against their Rust source.

## Scientific and restart checks

The native test suite covers fixed-seed determinism and step batching; reset of initial conditions; parameter and command rejection; exact JSON checkpoint continuation; atomic rejection of incompatible or malformed checkpoints; observation purity; known connected patches, a periodic seam and rejected sheet-spanning components; distinct preset behavior; and a 1,200-step maximum-control boundedness run with finite field/activity values. Existing measurement-requirement checks remain included.

The separately built WASM artifact passes deterministic stepping, full checkpoint continuation after perturbation, invalid-version rejection without mutation, and reset. These tests support same-model/same-runtime continuation. Transcendental functions and different runtimes are not promised to give universally identical trajectories.

The applicable invariant is boundedness, not conserved physical energy: the validated fixed timestep makes transport/diffusion/damping weights nonnegative, bounded forcing keeps flux within ±12, and activity stays in [0,1]. Field boundaries remain fixed. Detection is observation-only.

## Preset contrast

Native default runs with seed 73129, 128×80 cells, fixed timestep 0.12, and no user interventions:

| Preset           | Initial candidates | Candidates at step 120 | Candidates at step 1,200 | Total cell firings at step 1,200 | Stress at step 1,200 (rounded) |
| ---------------- | -----------------: | ---------------------: | -----------------------: | -------------------------------: | -----------------------------: |
| Quiet sheet      |                  0 |                      0 |                        0 |                                0 |                       0.005553 |
| At the threshold |                  6 |                      6 |                        6 |                          104,752 |                       0.023003 |
| Island cascade   |                 14 |                     11 |                        1 |                          169,127 |                       0.042217 |
| Stirred sheet    |                  4 |                      6 |                        2 |                           93,777 |                       0.008672 |

This shows contrast in the implemented toy rules. Changes in candidate counts do not identify physical formation, splitting or merger events. Candidates are thresholded flux-anomaly patches; their labels are reassigned each observation. The native benchmark reproduces these probes.

## Measured cost

Environment: shared Linux x86-64 execution host, Rust 1.90 release build and Node v24.19.0. CPU model is not exposed. These are backend measurements, not desktop-browser FPS or a hardware guarantee.

- Native cascade: 120 warmup ticks followed by 1,200 measured ticks. Two runs during development measured 1,821.6 and 3,506.9 ticks/second. A separate 120-sample frame/detection/JSON loop measured 1.959 and 0.960 ms/frame respectively. Shared-host load varied; no statistical confidence interval is claimed.
- WASM cascade under Node: 120 warmup ticks per run; 200 measured ticks in batches of two, each returning a full observed frame with JSON encode/decode. Three consecutive measurements were 875.0, 618.0 and 478.7 ms, or **228.6, 323.6 and 417.8 ticks/second**. Each final response was about 601 KB. Worker messaging, canvas rendering and persistence were excluded.

The interface requests two fixed ticks per update, waits for their result, then waits 40 ms before the next request. Thus 50 ticks/second is an upper scheduling limit, not an achieved throughput claim. Load slows simulated progress; it does not change the model timestep or skip physical evolution.

Copied full-resolution JSON samples are deliberately adequate for this small slice. Larger states need measured observation decimation or transferable presentation buffers near their executor. The current measurements do not justify a WebGPU rewrite.

## Browser acceptance

Browser acceptance and final build status are pending the integration run. The local sandbox can execute Rust and Node/WASM but Chromium launch exits with SIGTRAP; browser checks are configured for the repository's normal Linux CI environment. Do not infer a passed browser gate from the backend checks above.

## Limits and next evidence

The rules prescribe driving, stirring, damping, seed perturbations and threshold relaxation. Attractive contours do not validate reconnection physics. No particle distributions, pressure/momentum evolution, physical energy budget, topology classifier, persistent identity, split/merge lineage, prediction, or adaptive representation is implemented.

Rung 2 needs a declared physical reference model, convergence and reference comparisons, and detector sensitivity/continuity tests before tracking or shadow prediction. The Toybox's experiment, checkpoint and observation boundaries can remain useful while that work proceeds.
