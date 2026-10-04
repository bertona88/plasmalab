# 0001 — Start with a small monorepo and explicit ownership

Date: 2026-10-04  
Status: adopted for the repository foundation

## Context

PlasmaLab starts from an empty repository. The ladder points toward adaptive representations but explicitly asks for complete, enjoyable increments and postpones a universal solver framework. We need places for the chosen technologies and scientific evidence without pretending the later systems exist.

## Decision

Use a Cargo workspace and an npm workspace in one repository. Start with one dependency-free Rust crate (`plasmalab-core`) and one Svelte/TypeScript app built with Vite (`apps/web`). Keep supplied direction documents and experiment/evidence briefs alongside code.

The initial Rust code expresses a named measurement requirement with a finite positive horizon and absolute tolerance, plus distinct evidence and result categories. It does not implement evolution. The browser shell describes the project and links to the first experiment plan; it does not duplicate scientific rules.

Keep `kernels/` as documented WGSL ownership, without a dummy shader, GPU dependency, or fake performance claim. Add WASM glue, a worker, persistence, and actual kernels only through an implemented experiment. Generate scientific wire types from Rust when that bridge becomes real.

Pin the Rust toolchain and npm dependencies, commit lockfiles, and check native Rust, the WASM target, Svelte/TypeScript, formatting, and the web build in CI. Do not deploy an unfinished laboratory automatically.

## Consequences

The repository is runnable and testable but has not completed Rung 1. The next change can focus on one complete experience. Some boundaries are documented rather than implemented; no claims of solver interchangeability, physical validity, persistence, or SELFS authority follow from this scaffold.

Revisit crate/package boundaries when a second implemented use demonstrates a need. Preserve experiments and evidence across refactors. Avoid publishing schemas or universal abstractions before the first concrete run supplies the information they need to represent.
