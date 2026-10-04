# PlasmaLab

A browser laboratory for building intuition about plasmas.

The first experiment is a **2D plasmoid/reconnection-inspired toy**: disturb an opposed-field sheet, change its loading and release threshold, and inspect how patterns respond. It uses invented, bounded local rules in toy units. It is not quantitative MHD or PIC, and detected patches are not validated physical plasmoids or SELFS.

Run, pause, single-step and reset; choose four teaching presets; click or drag to perturb the field; inspect toy stress, release events and candidate regions; save a checkpoint locally or export it to a file. The in-app model panel explains the rules and assumptions. [The experiment brief](docs/experiments/plasmoid-ecology.md) gives the exact model and its limits.

**Aim at adaptive SELFS. Ship through a useful Toybox and Laboratory.** Read [the ladder](docs/ladder.md) for the direction and the [SELFS abstract](docs/SELFS_WOFI_abstract.md) for the research hypothesis. Both source documents are preserved as supplied. [Project context](docs/project-context.md) and the [source inventory](docs/source-inventory.md) distinguish user decisions, historical proposals, reviewed sources and remaining gaps.

## Start locally

Install Node.js 24 with npm and Rust through [rustup](https://rustup.rs/). The repository pins Rust and its WASM target in `rust-toolchain.toml`. Install the `wasm-bindgen` CLI version matching the locked Rust dependency, then build and run:

```sh
npm ci
cargo install wasm-bindgen-cli --version "$(node scripts/wasm-bindgen-version.mjs)" --locked
npm run dev
```

Open the local URL Vite prints. The development command builds Rust/WASM before starting Vite. After changing Rust, rerun the WASM build and reload the page. No WebGPU adapter or compute service is required.

| Command                      | Purpose                                                                   |
| ---------------------------- | ------------------------------------------------------------------------- |
| `npm run dev`                | Build WASM and start the browser development server                       |
| `npm run build:wasm`         | Rebuild the scientific model and browser WASM bindings                    |
| `npm run generate:contracts` | Regenerate browser wire types after changing Rust contracts               |
| `npm run check:contracts`    | Check generated Rust-to-TypeScript contract drift                         |
| `npm run check`              | Check Svelte and TypeScript                                               |
| `npm run build`              | Build WASM and the production app in `apps/web/dist`                      |
| `npm run preview`            | Serve the production build locally                                        |
| `npm run format`             | Format web code, configuration and authored docs                          |
| `cargo fmt --all`            | Format Rust                                                               |
| `npm run check:rust`         | Check Rust formatting, Clippy and unit tests                              |
| `npm run verify`             | Run the repository verification gates                                     |
| `npm run test:wasm`          | Test compiled WASM stepping, checkpoint continuation, rejection and reset |
| `npm run test:browser`       | Exercise the browser interaction and persistence loop                     |

Browser checks require Chromium: install it with `npx playwright install --with-deps chromium` on Linux (or `npx playwright install chromium` when system dependencies are already present). The [toybox evidence record](docs/evidence/toybox.md) describes the checks actually run and the measured performance scope.

## What is saved

Save stores one versioned checkpoint in this browser’s IndexedDB. Saving again replaces that slot. Load restores the saved model state, parameters, intervention history, counters and random generator; it does not merely reopen settings. Reset restores the initial parameters and seed and clears intervention history. Export/import uses the same checkpoint as JSON so work can leave this browser. Local storage may be cleared; keep exported files for backup.

The Rust model validates checkpoints before replacing a run. Unsupported versions and corrupt fields are rejected visibly. Same-environment continuation is the supported deterministic scope; identical numerical histories across every device or browser are not promised.

## Where things belong

| Location                 | Responsibility                                                                        |
| ------------------------ | ------------------------------------------------------------------------------------- |
| `apps/web/`              | Svelte interface, worker adapter, presentation and browser storage                    |
| `crates/plasmalab-core/` | Rust model semantics, parameters, diagnostics, detector, serialization and validation |
| `kernels/`               | WGSL ownership and acceptance rules; no kernels yet                                   |
| `docs/ladder.md`         | Long-term direction                                                                   |
| `docs/architecture/`     | Adopted boundaries and implementation decisions                                       |
| `docs/experiments/`      | Implemented toy and future experiment questions                                       |
| `docs/evidence/`         | Check results, scope and reproducibility instructions                                 |
| `.github/workflows/`     | Build, type, formatting, Rust and WASM gates                                          |

[Decision 0002](docs/architecture/0002-toybox.md) explains the boundary: Rust owns evolving state and observations; the worker returns disposable presentation samples; Svelte sends commands. The timestep is fixed independently of screen updates. WebGPU is deferred until a measured need justifies it.

Candidate detection remains at **observe**. There is no identity tracking, split/merge classification, shadow prediction, adaptive representation or physical validation. [The roadmap](docs/roadmap.md) defines the next gates. See [CONTRIBUTING.md](CONTRIBUTING.md) for development and [AGENTS.md](AGENTS.md) for coding-agent guidance. Completed work belongs in committed, pushed, remotely verified history.
