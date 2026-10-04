# PlasmaLab

A browser laboratory for building intuition about plasmas.

**Aim at adaptive SELFS. Ship through a useful Toybox and Laboratory.** The enduring objects are experiments, observations, and evidence; solvers and execution backends can improve around them.

Read [the ladder](docs/ladder.md) for the project direction and the [SELFS abstract](docs/SELFS_WOFI_abstract.md) for the research hypothesis. Both source documents are preserved as supplied.

The [project context](docs/project-context.md) preserves the motivation, distinguishes chosen direction from implementation proposals, and lists the open choices for the first experiment.

The [source inventory](docs/source-inventory.md) identifies which earlier chats and documents are preserved, known only through summaries or references, or still missing. This repo is not yet a complete project-context handoff.

## Current state

This is the repository foundation, before Rung 1. It includes a runnable Svelte project shell, a small dependency-free Rust scientific core, formatting, checks, and CI. The core validates named measurement requirements and distinguishes evidence and result kinds.

**No simulation, WASM bridge, GPU kernel, persistence, structure detector, or adaptive representation is implemented yet.** The app marks its experiments and milestones as planned. The Rust core and browser shell build independently until the first experiment connects them.

## Start locally

Install Node.js 24 with npm and Rust through [rustup](https://rustup.rs/). The repository pins Rust and its WASM target in `rust-toolchain.toml`.

```sh
npm ci
npm run dev
```

Open the local URL Vite prints. The shell needs neither Rust nor WebGPU to run. To work on the scientific core or run all checks:

```sh
npm run verify
cargo check --workspace --locked --target wasm32-unknown-unknown
```

Useful commands:

| Command              | Purpose                                           |
| -------------------- | ------------------------------------------------- |
| `npm run dev`        | Browser development server                        |
| `npm run check`      | Svelte and TypeScript checks                      |
| `npm run build`      | Production browser build in `apps/web/dist`       |
| `npm run preview`    | Serve the production build locally                |
| `npm run format`     | Format web code, configuration, and authored docs |
| `cargo fmt --all`    | Format Rust                                       |
| `npm run check:rust` | Rust formatting, Clippy, and unit tests           |
| `npm run verify`     | All local foundation checks                       |

## Where things belong

| Location                 | Responsibility                                                          |
| ------------------------ | ----------------------------------------------------------------------- |
| `apps/web/`              | TypeScript/Svelte interface; future browser storage and worker adapters |
| `crates/plasmalab-core/` | Rust scientific meaning and validated requirements                      |
| `kernels/`               | WGSL ownership and acceptance rules; no kernels yet                     |
| `docs/ladder.md`         | Long-term direction                                                     |
| `docs/architecture/`     | Boundaries and recorded decisions                                       |
| `docs/experiments/`      | Experiment questions, assumptions, and milestone briefs                 |
| `docs/evidence/`         | Evidence requirements and future reference results                      |
| `.github/workflows/`     | Web, native Rust, and WASM compilation gates                            |

Do not create a crate for every future idea. Split implementations when a real second use needs it. No universal world state, backend registry, generic scheduler, or SELF authority machinery is needed for the first loop.

## Next useful increment

The proposed starting point is the [charge-separation experiment](docs/experiments/charge-separation.md): set conditions, disturb, observe, pause, inspect, save, export, and reopen with assumptions intact. The [roadmap](docs/roadmap.md) defines the evidence required before climbing each rung.

See [CONTRIBUTING.md](CONTRIBUTING.md) for development and [AGENTS.md](AGENTS.md) for coding-agent guidance. Completed work belongs in committed, pushed, remotely verified history.
