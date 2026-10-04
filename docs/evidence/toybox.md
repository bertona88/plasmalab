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

The first complete source tree, commit `3f56c73`, passed [CI run 37195274974](https://github.com/bertona88/plasmalab/actions/runs/37195274974): native Rust tests, Clippy, formatting, generated-contract agreement, WASM compilation, Svelte/TypeScript checks, production build, compiled-WASM integration and **8 browser tests (12.8 seconds)**. Svelte reported zero errors and zero warnings.

The browser tests exercise run/pause/single-step/reset; pointer and keyboard/button disturbances; parameter and preset changes with real diagnostics; model assumptions; exact browser-local save/load across page reload; JSON export/import and subsequent continuation; malformed/unsupported checkpoint rejection without state mutation; storage failure and recovery; and a 390×844 viewport without horizontal overflow. All use the actual Rust/WASM worker. No fake model state is injected.

The local sandbox executed Rust and Node/WASM checks, but its Chromium launch exited with SIGTRAP. Browser acceptance therefore ran on the normal Linux CI runner. Passing tests establish these specific interaction paths, not universal device support. The follow-up commit `a141dca` corrected contour saddle rendering and retained passing-run artifacts. It passed [CI run 37195535500](https://github.com/bertona88/plasmalab/actions/runs/37195535500), including all **8 browser tests in 13.6 seconds**. No scientific update rule changed between these commits.

Both real browser screenshots were inspected: [desktop](toybox-desktop.png) and [390-pixel layout](toybox-narrow.png). Opposed field colors, flux contours, local activity, candidate markers and diagnostics are visible; controls and persistence sections have no clipping or overlap. The narrow layout places the experiment before the control panels. This is visual plausibility and interface review, not physical validation.

The retained [browser progression record](toybox-browser.json) reports Chromium 141.0.7390.37 on the Linux x64 CI runner, four reported logical CPUs, and a 1280×720 viewport. The default tearing preset completed 60 displayed steps over 1.604934 seconds: **about 37.4 steps/second including scheduling and presentation**. The final paused/exported checkpoint was step 62 because an already requested batch completed. This short sample measures completed model progress, not rendering FPS or unrestricted solver throughput. The Windows user-agent string in the record comes from Playwright’s Desktop Chrome profile; it is not the host operating system. No specific desktop CPU or performance guarantee is inferred.

## Limits and next evidence

The rules prescribe driving, stirring, damping, seed perturbations and threshold relaxation. Attractive contours do not validate reconnection physics. No particle distributions, pressure/momentum evolution, physical energy budget, topology classifier, persistent identity, split/merge lineage, prediction, or adaptive representation is implemented.

Rung 2 needs a declared physical reference model, convergence and reference comparisons, and detector sensitivity/continuity tests before tracking or shadow prediction. The Toybox's experiment, checkpoint and observation boundaries can remain useful while that work proceeds.
