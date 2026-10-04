# Experiment option — plasmoid cascades and changing candidate identities

Status: assistant proposal distilled from the available “Plasma Pattern Zoology” turns, 2026-10-04. No implementation, adopted model choice, or numerical results. See [project context](../project-context.md#plasma-pattern-zoology-candidate-identity-not-just-recurring-shapes) for candidate families and [source inventory](../source-inventory.md) for source limitations.

## Questions and experience

The user's “cellular automata kinda stuff?” question prompted an assistant proposal: **what minimal local rules produce chains, mergers, cascades, or persistent islands?** The later request for a zoology of SELFS adds a different question: **does tracking an organized structure and its internal state improve prediction over an arbitrary region or simpler baseline?** The first question concerns an invented universe; the second needs a declared model and a predictive test.

The proposed cascade is `current sheet → islands → thin sheets between islands → further tearing → more islands`, with possible mergers and ejections. “Ecology” describes this evolving pattern vocabulary; it does not assert organisms or biological reproduction. Solar reconnection was offered as motivation, not as a validated model for this experiment.

## Two proposed routes

### Toy universe: editable local rules

The assistant suggested cells carrying magnetic orientation/flux, current-sheet stress, plasma density, and optionally local resistivity. The illustrative rules were:

- Opposed magnetic neighbors build current/stress.
- A threshold `J > J_c` triggers a reconnection-like update.
- A sufficiently long stressed region nucleates an island/plasmoid-like pattern.
- Adjacent patterns may merge; stretched gaps may create new sheets and split again.

Slow stress injection and threshold-triggered avalanches were compared with sandpile models and self-organized criticality. This is a proposed analogy, not evidence of criticality or a physical tearing criterion. The symbols and rules are sketches, not code or a complete update law. No neighborhood, flux representation, boundary conditions, update order, thresholds, conserved budget, or timing was supplied. Decide these explicitly before implementation; in particular, resolve whether “long stressed region” can actually be evaluated locally.

The suggested interaction is to mutate or disable rules and compare which patterns persist. Save the rule set/version, conditions, seed if used, update schedule, interventions, and state so a result can be revisited. Label the model as invented: producing recognizable islands does not establish quantitative MHD behavior.

### Physics route: structures emerge from an explicit model

The assistant suggested reduced or resistive MHD rather than prescribed split/merge rules. These are alternatives to evaluate, not a selected solver. Before proceeding, choose dimensionality, equations and valid regime, normalization, initial/boundary conditions, drive, dissipation, numerical method, resolution, and a reference case. The chat supplied none of these implementation choices.

Do not equate toy thresholds with physical instability conditions or infer a 3D flux rope from a 2D island outline. Any toy/physics comparison needs an explicit correspondence of questions and observables; shared terminology is insufficient.

## A possible staged investigation

The following steps are handoff proposals derived during this repository review, not experiments already agreed or performed in the chat:

1. For the toy route, vary drive/thresholds and disable nucleation or merger rules in otherwise matched runs. Record pattern lifetimes and event histories; do not infer physical validity or criticality from attractive cascades.
2. For a measured physical reference run, declare a detector and continuity criterion. Record formation, loss, split, and merger candidates with ambiguous associations visible. A disappearance may be a tracking failure rather than physical dissolution.
3. Test a compact state as a shadow predictor for one named observable and horizon against a simpler predictor or matched arbitrary-region baseline. Specify errors and acceptance before evaluation; include perturbations not used to construct the predictor where practical. Assess the proposed persistence/memory/autonomy/regeneration questions through these tests rather than assigning the chat's illustrative scores.

Rust would own implemented scientific/toy semantics, WGSL would execute justified parallel updates, and Svelte would expose interventions and observations. At Rung 2, detections and forecasts read the evolving reference; they cannot prescribe tearing, mergers, or authoritative field evolution. Use the [evidence requirements](../evidence/README.md) for any resulting claim.

## Scope and open choices

This option complements the proposed [charge-separation slice](charge-separation.md); it does not change the roadmap priority. The oscillator does not provide a spatial state for this experiment. Nor does an MHD island study replace the SELFS abstract's proposed kinetic driven electron–ion comparison without a separately justified change of question.

Open choices include which route merits implementation first, what makes identity continuous through a merger, how detector sensitivity affects lifetime/lineage, which internal state adds predictive information, and whether a candidate's benefit survives the full detection/tracking/checking cost. Representation replacement, compression performance, and recovery from lost detail remain later tests. No simulation result follows from this brief.
