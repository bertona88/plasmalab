# First experiment brief — displace, release, observe

Status: proposed first vertical slice; no solver is implemented.

## Question and experience

How does a restoring response change when the initial electron displacement or velocity changes?

Set the initial conditions, release a displaced electron population, pause the motion, inspect displacement/velocity/energy, apply a velocity impulse, save, and reopen. Keep the approximation visible. This small oscillator is a teaching model for a collective response, not a kinetic plasma simulation or a demonstration of emergent structure.

## Proposed scope

Use the linear, cold, collisionless electron-displacement approximation with a fixed neutralizing ion background, no magnetic field, no spatially resolved variation, and no damping. Normalize time by inverse electron plasma frequency and displacement by an explicitly stated reference length. In normalized coordinates the planned model is `d²x/dτ² = -x`. Its amplitude must remain in the declared small-displacement regime; decide and document the chosen normalization and admitted parameter bounds before implementation.

Use a fixed-step Rust numerical implementation and an independent analytic oscillator reference. TypeScript sends user intent and displays observations; it does not implement the update equation. Start with displacement, velocity, normalized energy, and phase error. A velocity impulse is an explicit intervention at a simulation time and changes the energy budget; compare energy conservation only between interventions and account for the impulse work separately.

## Acceptance

- Declare the integrator, supported bounds, timestep, units/normalization, and numerical tolerances before treating a run as valid.
- Compare to the analytic solution over a named horizon and show timestep convergence. Reject nonfinite inputs and unsupported configurations in Rust.
- Preserve fixed physical steps across different presentation cadences. Pause/step/reset must be unambiguous.
- Save versioned experiment identity, initial conditions, model/representation version, simulation time, full oscillator state, integrator configuration, interventions, and relevant observations.
- Demonstrate reopen/continue and export/import. Distinguish resetting to initial conditions from resuming the checkpoint. Retain assumptions on reload.
- Expose the reference error and the scope of the approximation; attractive motion alone is not evidence of validity.

## What survives upward

The question about a disturbed electron–ion system, experiment identity, interventions, observations, reference comparisons, and durable history. A later spatial electrostatic/kinetic experiment can investigate richer responses, but it needs an explicit mapping of parameters and observables. An oscillator state cannot simply be reinterpreted as a particle distribution. This model alone does not justify structure detection, SELF compression, or a representation transition.
