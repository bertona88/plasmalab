# Contributing

Start with [the ladder](docs/ladder.md), [architecture](docs/architecture/README.md), and [roadmap](docs/roadmap.md). A useful contribution adds a user action, observation, or comparison while preserving something needed by the next rung.

## Development

Use Node 24, the pinned Rust toolchain and the matching `wasm-bindgen-cli` version in the README. Run `npm ci` at the repository root. `npm run dev` builds the Rust/WASM toy and starts the app. `cargo test --workspace --locked` exercises model, detector, validation and checkpoint contracts. See the README for the full command list. Rebuild WASM after Rust changes; Vite does not compile Rust automatically during an already running session.

Use a focused branch for subsequent changes. Before submitting:

```sh
npm run format
cargo fmt --all
npm run verify
cargo check --workspace --locked --target wasm32-unknown-unknown
npm run test:browser
git diff --check
```

Generate browser wire types with `npm run generate:contracts` after changing Rust contracts; `npm run check:contracts` checks drift. Do not hand-edit generated declarations.

The source direction documents are excluded from automatic formatting so their supplied text remains intact. Do not edit generated lockfiles by hand. Avoid introducing dependencies merely to reserve a future architectural boundary.

## Scientific changes

Describe the question, model and version, units or normalization, initial and boundary conditions, interventions, supported measurements, and known limits. Name the reference, observable, error measure, tolerance, and comparison horizon. Distinguish requested accuracy from measured accuracy.

Report whether the evidence checks implementation agreement, numerical convergence, or predictive validity. Performance claims must include the relevant total cost at matched requirements. Use [the evidence guidance](docs/evidence/README.md).

When changing the persisted format, update its version, round-trip tests, unsupported-version behavior, and migration policy. The current toy restores an actual checkpoint and rejects unsupported schema/model versions. Keep restoration atomic: invalid input must not replace the running state. Reopening, rerunning, and reproducing within tolerance are separate promises.

## Review and durability

The pull-request template asks for both a user-facing outcome and what survives into the next rung. Record consequential changes to ownership or scientific contracts in a short architecture decision. Commit and push completed work; check that the remote branch points to the intended commit. Branch protection and publishing/deployment are separate repository settings.
