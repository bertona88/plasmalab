# PlasmaLab — The Ladder

**Direction:** build toward Architecture 3; deliver Architectures 1 and 2 as worthwhile milestones.  
**Date:** 2026-10-04  
**Status:** long-term architectural direction, not an implementation specification.

> Build a laboratory that can eventually change what it treats as an entity. Make every step toward that destination something worth opening and playing with.

## 1. One destination, three increasingly capable experiences

The destination is the **Emergent Plasma Operating System**: an environment where several physical representations can coexist, persistent structures can become candidate SELFS, and evidence can eventually guide which representation does the computational work.

The **Plasma Toybox** and **Browser Plasma Laboratory** are not alternative products or disposable prototypes. They are earlier, complete experiences within that same direction. Each should remain useful after the next arrives.

The architectural commitment is to preserve the boundaries that make this growth possible—not to build the entire moonshot before anything is enjoyable. Start with small implementations of lasting responsibilities. Introduce generality when a second real use demonstrates why it is needed.

A successful ladder compounds three things: the user's intuition, the project's scientific evidence, and the application's reusable capabilities.

## 2. The destination we are protecting

The central long-term question is:

> What kind of thing should we simulate here, for the question being asked, over the time horizon that matters?

Particles, fields, fluids, boundaries, waves, and compact collective models are possible descriptions—not a fixed hierarchy in which one is always best. A SELF is useful when its internal description predicts selected behavior and exchanges within a stated tolerance. Its identity need not depend on unchanged constituents, simple spatial boundaries, or identical motion.[^selfs]

Eventually, PlasmaLab should be able to recognize such a description, test it, give it limited computational responsibility, and withdraw that responsibility when its validity ends.

That is the research destination. The architecture must make it testable without assuming that useful compression, reliable transitions, or a speed advantage have already been demonstrated.

## 3. The three rungs

### Rung 1 — Plasma Toybox: “I can make something happen.”

Deliver a complete loop: choose an experiment, disturb it, watch the response, pause, inspect, save, and return. Favor a small number of memorable experiences over a large menu of unfinished physics.

A simple model is enough. A toy universe is welcome when it is visibly presented as a toy universe; it does not acquire physical validity by sharing the interface with a more physical model.[^toy]

The foundations already distinguish the experiment, its evolving simulation, the observations, and the presentation. Early tracking can be modest. Execution can be modest too: using a small CPU model does not mean making the interface depend on CPU-owned arrays forever.

**What survives upward:** experiments, interaction vocabulary, observation concepts, saved history, and the separation between simulation and presentation.

**Completion evidence:** someone can discover a cause-and-effect relationship, save the experiment, and revisit it with the model's assumptions still visible.

**The reward:** “I changed the conditions, and now I understand why the picture changed.”

### Rung 2 — Browser Plasma Laboratory: “I can investigate what happened.”

Add stronger numerical models, accelerated execution where useful, meaningful diagnostics, and comparisons against reference cases. Deepen a selected physical problem before accumulating unrelated solvers.

Structures become inspectable candidates with identities and histories. The application can show formation, persistence, interaction, splitting, and merging rather than just colored fields. A compact predictor can forecast a candidate's behavior while the reference simulation continues to determine the actual evolution.

At this rung, SELFS are **observers and shadow predictors**, not replacements for the underlying calculation. Their predictions can succeed or fail visibly without changing the experiment they are being tested against.

**What survives upward:** validated reference cases, performance measurements, structure histories, prediction records, and an execution boundary that does not own scientific meaning.

**Completion evidence:** a user can compare an observation or prediction with a reference outcome, see its error, and understand the applicable limits. Numerical checks and comparisons accompany visual demonstrations.

**The reward:** “The app recognized a structure, followed it, and showed me what it could—and could not—predict.”

### Rung 3 — Adaptive SELFS Laboratory: “The simulation changes how it understands the system.”

Give selected predictive descriptions responsibility for part of the evolution. Begin with one well-defined transition in a narrow regime, not universal switching among every model.

A representation change must account for the quantities it takes responsibility for, its exchanges with the surrounding system, its uncertainty, and its recovery path. It must not count the same contribution twice or pretend discarded information can always be recovered.[^selfs]

Move from proposals to supervised trials and only then to guarded automatic decisions. Keep a reference path available for comparison. Rejecting a proposed simplification is a legitimate outcome, not a product failure.

**What survives upward:** the Toybox remains an accessible learning space; the Laboratory remains the comparison and validation environment. Neither is retired to make room for adaptation.

**Completion evidence:** demonstrate one useful representation change at a defined accuracy and prediction horizon, including its full computational cost and a tested response to loss of validity. Do not generalize that result beyond its tested scope.

**The reward:** “This structure became a useful computational entity—and I can see the evidence for that decision.”

## 4. Architectural choices that make the ladder possible

### Keep the experiment more durable than its solver

Make the experiment the enduring object: the question, conditions, interventions, measurements, conventions, and desired accuracy. A particular solver is one way of pursuing that experiment.

This lets familiar experiments acquire better execution and new representations without becoming unrelated demos. It also creates a common basis for later comparisons and adaptive decisions.

Sharing an experiment does not imply that every model supports the same initial state or observables. Require explicit correspondence where models differ; expose unsupported questions rather than manufacturing equivalence. Preserve comparability, not the fiction of universal interchangeability.

### Separate scientific meaning, execution, and experience

Retain the chosen division of responsibility: Rust carries scientific rules and meaning; WGSL/WebGPU provides parallel numerical execution; TypeScript/Svelte provides the human interface; browser facilities support the local experience.[^architecture]

Treat this as an ownership principle, not a promise that a particular library or backend will last forever. Numerical execution must remain accountable to the scientific model wherever the arithmetic runs.

The interface should express interventions and request observations, not manipulate hidden solver details. Presentation should not become the authoritative physical state. Small implementations may live together initially while retaining these distinctions.

### Share scientific contracts, not one universal world object

Particle, field, fluid, and collective descriptions should not be forced into a single internal representation. Let each model retain the state and methods it genuinely needs.

The common ground is what it represents, which observations it supports, what it assumes, where it is valid, and how it exchanges conserved quantities with other descriptions. These are the commitments needed for later composition.

Build explicit bridges for useful pairs of representations. Do not assume that adding two interchangeable solvers also solves the much harder problem of coupling them within one evolving experiment.

### Keep simulation, observation, and interpretation distinguishable

A simulated field, a detected structure, a predicted trajectory, and an evocative visualization are different kinds of result. Preserve their provenance and uncertainty instead of allowing them to blend into one apparently authoritative picture.

At Rung 1, this keeps visual effects from defining the physics. At Rung 2, it lets competing detectors and predictors examine the same run. At Rung 3, it prevents a candidate interpretation from becoming computational authority without an explicit decision.

Alternative viewpoints, including relational or POV experiences, should remain projections with stated meaning. They do not silently add new physical laws or validate the hypotheses that inspired them.

### Make identity independent of storage and ownership

An entity's identity should not be its memory address, mesh patch, rendered outline, or fixed list of particles. Track continuity in the organization being described, with room for uncertainty and revised interpretations.

Keep present interactions, containment, and historical lineage conceptually distinct. A structure can interact with one candidate, contain another, and descend from several earlier candidates without those relations meaning the same thing.

Observation may allow overlapping candidates. Computational responsibility must still account for contributions unambiguously. A rich SELF graph is not permission to evolve or count the same physical content twice.

### Let SELFS earn authority in stages

Use a deliberate progression: **observe → track → predict → advise → act**.

A detector proposes that something exists. A tracker proposes continuity. A predictor makes a testable claim. An advisor proposes a representation change. Only an accepted representation is allowed to determine part of the evolution.

Each stage is useful on its own and creates evidence for the next. Prediction must be evaluated for named observables and horizons, not summarized by an unexplained universal confidence score. Where practical, test responses to interventions beyond those used to construct the predictor.[^selfs]

This is the main bridge between an engaging visualization product and an adaptive research system.

### Treat transitions as scientific decisions, not performance shortcuts

Changing representation changes what information is retained and which assumptions govern subsequent evolution. Require explicit accounting for exchanges, approximation error, and the limits of the new description.

Before reducing detail, decide how loss of validity will be handled: retained internal information, appropriate checks, a sufficiently detailed restart point, or an explicit stop. An alarm about prediction error is not itself a way to reconstruct lost microscopic information.[^selfs]

Prefer transitions with a credible recovery strategy. Do not promise mathematical reversibility where it does not exist. Make a rejected or withdrawn transition explainable to the user.

### Separate the pace of interaction from the pace of physics

Preserve a responsive experience without tying physical time to screen updates. Rendering, observation, and validated physical evolution have different responsibilities.

Later, models may use different physical timescales, but those rates must belong to a justified coupled numerical method—not a generic scheduler that advances each subsystem whenever convenient. Keep the architectural opening for multirate work without treating arbitrary rate ratios as a valid method.

When resources are limited, reduce presentation work or slow progress through simulated time before silently changing the physical question. Changes in scientific fidelity should be deliberate and visible.

### Make evidence and total cost part of the architecture

Every ambitious path needs a simpler reference path. Keep baseline experiments, comparison criteria, and numerical diagnostics as lasting product capabilities rather than temporary development tools.

Distinguish visual plausibility, implementation agreement, numerical convergence, and predictive validity. Passing one does not establish all the others.

Judge adaptive representations against a suitable baseline at matched requirements. Include discovery, tracking, prediction, checking, coupling, transition, and recovery costs—not only the cheaper step after reduction. A slower adaptive method can still provide a useful observation tool without being presented as a computational improvement.[^selfs]

### Make experiments and their history durable

Treat saved work as part of the laboratory, not a convenience added at the end. Preserve the experiment, model and representation versions, interventions, important observations, and enough state to support the promised form of resumption or comparison.

An event history explains what happened; it is not automatically a complete restart state. Distinguish reopening a saved state, rerunning an experiment, and reproducing results within a tolerance. Do not promise identical numerical histories across all execution environments.

Keep saved formats evolvable and results exportable. Browser-local storage supports immediate continuity; durable backup must not depend on one browser session or a temporary machine. Project documents and completed work should be committed and pushed to the agreed repository, with the remote result verified.

### Keep the browser central without making it the scientific boundary

The browser remains the primary place to explore, intervene, and understand. A useful local experience should not depend on a compute service.

Allow execution to move behind the same experiment and observation concepts: modest local computation first, accelerated local computation next, optional external execution later. Prefer keeping large working state close to its execution rather than making every interface depend on copying it.

Remote execution is a later capability, not the definition of Rung 3. Adaptive SELFS can first succeed in a small local experiment. Larger hardware should not become a substitute for demonstrating the scientific idea.

## 5. Keep the milestones rewarding

Build complete experiences through the architecture rather than finishing entire invisible layers in isolation. Each increment should give the user a new action, a new observation, or a new comparison—and preserve something useful for the next rung.

Choose a recurring family of experiments as a thread through the ladder. The same question can return as an interactive toy, an explicitly modeled numerical experiment, a structure-tracking exercise, a prediction test, and finally a limited adaptive-representation trial. The correspondence between these versions should be explained, not assumed.

Make advances and failures visible. A newly tracked structure, a forecast compared with its outcome, or an explained decision to restore detail can be as compelling as a larger simulation.

The release test is: **what can the user now do or understand that they could not before?** The architectural test is: **what did this increment preserve or establish for the next rung?** Favor work that answers both.

## 6. What to commit to now—and what to defer

Commit now to the destination, enduring experiment identity, explicit scientific assumptions, replaceable execution, distinguishable evidence, durable history, and staged authority for SELFS. These choices prevent expensive conceptual dead ends without requiring a large implementation.

Defer a universal solver framework, unrestricted model mixing, automatic representation discovery, general-purpose adaptive scheduling, distributed execution, and a comprehensive taxonomy. Keep their conceptual doors open; do not build speculative infrastructure merely because the destination might eventually need it.

Treat new solvers, richer dimensions, and additional hardware as opportunities to test these boundaries. Some internals will be replaced. The goal is not a promise of zero rewrites; it is to preserve experiments, evidence, and user understanding while the machinery improves.

## 7. The rule for climbing

Move upward when the next responsibility has earned its place—not when every imaginable feature of the current rung is complete.

The Toybox earns the Laboratory by producing questions worth measuring. The Laboratory earns adaptive SELFS by producing predictions worth trusting within limits. Adaptive SELFS earns broader responsibility by demonstrating useful decisions, honest costs, and recoverable failures.

**Aim at 3. Ship through 1 and 2. Preserve the playfulness, accumulate the evidence, and let computational authority grow only as fast as understanding.**

---

## Basis

This document records the direction chosen in the PlasmaLab conversation on 2026-10-04. The architectural choices and milestone gates above are proposed commitments; they are not claims that the destination is implemented or validated.

[^selfs]: Project document `SELFS_WOFI_abstract.md`, 2026-10-03: “Abstract,” “Prima verifica proposta,” and “Limiti delle affermazioni.” Establishes predictive identity, coherent accounting, the need to plan for lost detail, matched-accuracy evaluation, and the unvalidated status of the computational hypothesis.

[^toy]: Project conversation “Plasma Pattern Zoology”: the distinction between physics mode and a deliberately simplified, CA-like toy universe for studying emergent patterns.

[^architecture]: Project conversation “Computational Architecture Patterns” and the three-architecture proposal: the chosen technology responsibilities and the Toybox → Laboratory → adaptive SELFS progression. This document develops those proposals into a long-term direction rather than prescribing their earlier illustrative implementation details.
