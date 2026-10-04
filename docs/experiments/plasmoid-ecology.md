# First toy experiment — a disturbed flux sheet

Status: implemented first-slice toy, selected by the user’s 2026-10-04 implementation mission. Model `bounded-flux-toy`, version `1.0.0`. The earlier zoology proposals below are retained as historical motivation; they did not supply a numerical law. See [decision 0002](../architecture/0002-toybox.md) for architecture and [toybox evidence](../evidence/toybox.md) for checks actually performed.

## Question and experience

The user's “cellular automata kinda stuff?” question prompted an assistant proposal: **what minimal local rules produce chains, mergers, cascades, or persistent islands?** The later request for a zoology of SELFS adds a different question: **does tracking an organized structure and its internal state improve prediction over an arbitrary region or simpler baseline?** The first question concerns an invented universe; the second needs a declared model and a predictive test.

The proposed cascade is `current sheet → islands → thin sheets between islands → further tearing → more islands`, with possible mergers and ejections. “Ecology” describes this evolving pattern vocabulary; it does not assert organisms or biological reproduction. Solar reconnection was offered as motivation, not as a validated model for this experiment.

## Implemented model and units

Rust owns the model in `crates/plasmalab-core/src/`. The grid is 128 × 80 cells, periodic horizontally, with top and bottom fixed to the background flux. Every step advances `0.12` toy time units. Field values, distances, stress and time have no SI calibration.

The scalar flux `ψ` supplies a magnetic-like visual field `B = (∂yψ, −∂xψ)`. The background has opposing horizontal directions above and below the central sheet. The evolved quantities are flux and a bounded local activity field. Prescribed stirring transports flux, smoothing reduces gradients, damping pulls deviations toward the background, and an invented saturating loading term amplifies deviations near the sheet. The model solves no momentum, density, pressure, particle distribution or Maxwell system.

The update is explicit on unit grid spacing. With `q = ψ − ψ₀`, `ψ₀(y) = 0.12√((y − 39.5)² + 36)`, `w(y) = exp(−((y − 39.5)/13)²)` and `ξ` a seeded uniform draw in `[-1, 1]`:

```text
ψ_next = ψ + 0.12 [η Δψ − u D_upwind,x ψ − v D_upwind,y ψ
                    + 0.18 loading w tanh(q / 0.4) − 0.06 q
                    + 0.006 loading w ξ]
η = 0.06 + 0.65 release_strength × activity
```

`Δ` is the four-neighbor Laplacian. First-order upwind differences implement transport by a prescribed shear plus vortical flow; the exact velocity expression is in [model.rs](../../crates/plasmalab-core/src/model.rs). Velocity is computed from the current step, so the model has no independent momentum state. Activity fires to `1` on a qualifying event; otherwise it advances by `0.12 (0.25 Δactivity − 0.5 activity)`. Flux smoothing uses the previous activity value, so a new firing affects it on the following tick. Updates use separate old/new buffers. The admitted controls keep transport/diffusion/damping weights nonnegative; bounded forcing preserves the flux bound during stepping. User perturbations are separately clipped.

A threshold event occurs when `|−8 ∇²ψ|` exceeds the chosen release threshold while local activity is below `0.25`. An event excites activity, which spreads and fades and temporarily increases smoothing. A cell can fire again once activity decays below `0.25` while its current-like value remains above threshold; no newly detected threshold crossing is required. An event count therefore counts **cell firings**, not independently confirmed topological reconnections. A Gaussian perturbation is a bounded flux intervention, not laser or particle injection. Flux is bounded to `[-12, 12]`; this is a rule of the toy, not a physical saturation law.

| Parameter         | Allowed range | Meaning                                                                     |
| ----------------- | ------------- | --------------------------------------------------------------------------- |
| Sheet loading     | 0–2           | Amplifies signed flux departures near the sheet through saturating feedback |
| Release threshold | 0.02–1.5      | Current-like curvature level needed to fire a local release                 |
| Release strength  | 0–1           | Extra smoothing during local activity                                       |
| External stirring | 0–2           | Strength of prescribed transport; flow is imposed, not dynamically solved   |

Four presets provide repeatable starting points. They change both seeded wave shapes and control values, so use a single slider on the same starting state to isolate one parameter’s effect.

| Preset           | Intended teaching contrast                                                           |
| ---------------- | ------------------------------------------------------------------------------------ |
| Quiet sheet      | Weak initial departures decay; a deliberate perturbation can create activity         |
| At the threshold | A few broad seeds respond to loading and the release threshold                       |
| Island cascade   | Many short seeds with stronger loading and easier releases produce more cell firings |
| Stirred sheet    | Strong prescribed flow deforms seeded patches and changes their connected outlines   |

Preset names describe intended toy behavior, not physical instability regimes. Changing a parameter during a run is recorded at its current simulation step. Reset restores the original parameters and seed, clears intervention history and rebuilds initial conditions. Choosing a preset starts from that preset’s defaults and seed `73129`; save/load restores the checkpoint state.

### Observations, not additional dynamics

Toy stress is the mean `½|∇ψ|²` over non-boundary grid rows, using centered differences in grid units. It is not a conserved physical energy. Loading, noise, imposed transport, damping, boundaries and user interventions can change it. Diagnostics also report active sites, cell firings, accumulated firings, candidate count and largest candidate area. The presentation plots sampled diagnostics; it does not decide the simulation’s update.

The detector selects same-sign departures `|ψ − ψ₀| > 0.22` in the central 60% of grid height and uses four-neighbor connectivity with periodic horizontal adjacency. It discards components smaller than 10 cells or occupying at least 75% of horizontal columns, so an unbroken stripe is not labeled as a localized candidate. These are chosen detection rules, not a separatrix or magnetic-topology test. It reports observation-local IDs, a periodic horizontal center, mean vertical center, cell area, an area-equivalent radius and polarity. The equivalent radius is `sqrt(area/π)` in grid cells (returned normalized by grid height); rings in the view are approximate patch indicators, not exact boundaries. Counts can change as the chosen detector’s regions appear, touch or disappear; no persistent identity or split/merge classification is inferred. Detection never modifies the evolving flux or activity. This remains the **observe** stage of the ladder.

Seeded perturbations are intentionally present in initial conditions. Evolving island-like contours are therefore not evidence that physical tearing has emerged from first principles. The user can explore loading, release and transport effects without the model claiming that an attractive cascade establishes MHD accuracy, criticality or a SELF.

## Earlier proposals and alternative physical route

### Historical toy proposal: editable local rules

Before implementation, the assistant suggested cells carrying magnetic orientation/flux, current-sheet stress, plasma density, and optionally local resistivity. The illustrative rules were:

- Opposed magnetic neighbors build current/stress.
- A threshold `J > J_c` triggers a reconnection-like update.
- A sufficiently long stressed region nucleates an island/plasmoid-like pattern.
- Adjacent patterns may merge; stretched gaps may create new sheets and split again.

Slow stress injection and threshold-triggered avalanches were compared with sandpile models and self-organized criticality. This is a proposed analogy, not evidence of criticality or a physical tearing criterion. The symbols and rules are sketches, not code or a complete update law. No neighborhood, flux representation, boundary conditions, update order, thresholds, conserved budget, or timing was supplied in that source. The implemented rule above makes those choices explicit and does not implement a nonlocal “long stressed region” nucleation rule.

The suggested interaction is to mutate or disable rules and compare which patterns persist. Save the rule set/version, conditions, seed if used, update schedule, interventions, and state so a result can be revisited. Label the model as invented: producing recognizable islands does not establish quantitative MHD behavior.

### Physics route: structures emerge from an explicit model

The assistant suggested reduced or resistive MHD rather than prescribed split/merge rules. These are alternatives to evaluate, not a selected solver. Before proceeding, choose dimensionality, equations and valid regime, normalization, initial/boundary conditions, drive, dissipation, numerical method, resolution, and a reference case. The chat supplied none of these implementation choices.

Do not equate toy thresholds with physical instability conditions or infer a 3D flux rope from a 2D island outline. Any toy/physics comparison needs an explicit correspondence of questions and observables; shared terminology is insufficient.

## A possible staged investigation

The following steps are handoff proposals derived during this repository review, not experiments already agreed or performed in the chat:

1. For the toy route, vary drive/thresholds and disable nucleation or merger rules in otherwise matched runs. Record pattern lifetimes and event histories; do not infer physical validity or criticality from attractive cascades.
2. For a measured physical reference run, declare a detector and continuity criterion. Record formation, loss, split, and merger candidates with ambiguous associations visible. A disappearance may be a tracking failure rather than physical dissolution.
3. Test a compact state as a shadow predictor for one named observable and horizon against a simpler predictor or matched arbitrary-region baseline. Specify errors and acceptance before evaluation; include perturbations not used to construct the predictor where practical. Assess the proposed persistence/memory/autonomy/regeneration questions through these tests rather than assigning the chat's illustrative scores.

Rust already owns the toy’s semantics; a stronger model should preserve this ownership, with WGSL executing justified parallel updates and Svelte exposing interventions and observations. At Rung 2, detections and forecasts read the evolving reference; they cannot prescribe tearing, mergers, or authoritative field evolution. Use the [evidence requirements](../evidence/README.md) for any resulting claim.

## Scope and next questions

The user’s implementation mission selected this toy over the earlier [charge-separation proposal](charge-separation.md). It is a teaching experience and an observation boundary, not a replacement for the SELFS abstract’s proposed kinetic driven electron–ion comparison. A later physical model must justify its equations and correspondence to the toy; it does not inherit physical validity from familiar outlines.

Candidate continuity through deformation and mergers is still open. So are detector sensitivity, an internal state that improves a named prediction, and whether any future representation benefit survives detection/tracking/checking costs. Representation replacement, compression performance and recovery from lost detail remain later tests. The evidence record reports implementation checks and browser behavior, not a physical reconnection result.
