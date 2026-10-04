# Research brief — micronozzle reference and GPU cost investigation

Status: a user-selected research example and recovered assistant reproduction plan, **not a scheduled browser feature, implemented WarpX deck or verified result**. Basis: the visible 2026-10-03 conversation, especially the short-timescale scope request, paper question and HF cost question; see [source coverage](../source-inventory.md#gpu-plasma-simulation-basics-and-the-visible-conversation).

## Scope and reason for keeping this example

The user narrowed the performance investigation to ultrashort laser interaction with small, dense targets, using Marvel Fusion/fast ignition as motivation and explicitly setting aside full large-capsule implosion calculations. That did not establish Marvel's internal design, workloads or software, and did not select a commercial collaborator or compute provider.

The concrete public example was [Scientific Reports article s41598-025-03385-x](https://www.nature.com/articles/s41598-025-03385-x). The assistant described a relativistic electromagnetic EPOCH PIC calculation of laser-generated electrons and accelerating fields in an aluminum micronozzle with hydrogen. Its reported high-energy proton result was discussed as a simulation prediction, not experimental verification or proof of ignition.

**This consolidation did not independently reread the paper or recover its inputs.** The details below preserve the earlier assistant's report so it can be checked; they are not a runnable specification or a verified transcription of Methods. Exact geometry, case selection and code options remain blocking inputs.

## Reported setup to verify against the original source

- EPOCH; 2D Cartesian geometry with uniform extension in the missing dimension, not a demonstrated finite 3D or axisymmetric nozzle.
- Fully ionized aluminum and hydrogen initialization; the hydrogen number density was reported as `5e22 cm^-3`.
- Laser wavelength `800 nm`; representative peak intensity `1e22 W/cm^2`; baseline pulse duration `100 fs` and spot width `10 micrometers`, reported as full width at half maximum.
- Domain `100–200 micrometers × 40 micrometers`, square `10 nm` cells, approximately `1 ps` evolution.
- `100 ion + 200 electron` computational particles per **material-filled** cell. This must not be multiplied by every vacuum cell.

The assistant proposed EPOCH for faithful reproduction and WarpX for a GPU port, with PIConGPU as another existing GPU implementation to consider. No port, input deck, chosen production resolution, hardware measurement or converged comparison was delivered. An electrostatic-only model or a fluid nozzle flow is not the same calculation.

First request the exact input deck, EPOCH version, geometry definitions, initial temperatures and surface profiles, boundary and laser injection settings, timestep, particle shape/loading, filters, collision/ionization/radiation options and reference diagnostics. Determine which settings are explicit and which depended on code defaults. Reproduce the stated assumptions before separately testing their physical adequacy.

## Recoverable cost arithmetic, not runtime evidence

The chat's sizing argument can be reconstructed from its reported inputs:

```text
10 nm = 0.01 micrometers
Nx = 10,000–20,000; Ny = 4,000
Ncells = 40–80 million

Illustrative 2D explicit Yee limit:
dt <= dx / (c * sqrt(2)) ~ 0.0236 fs
1 ps / dt ~ 42,400 steps at that limit
~44,600 steps if using 95% of that limit

Ten float64 grid quantities per location:
Ncells * 10 * 8 bytes = 3.2–6.4 GB (decimal), before overhead

Particle count = 300 * number_of_material_cells
Illustration only: 20 square micrometers of material
-> 200,000 occupied cells -> 60 million particles
At an assumed 64 bytes/particle -> 3.84 GB before auxiliary storage
```

Neither the illustrative occupied area nor the bytes per particle was measured for the actual nozzle. Field arrays, guard cells, sorting, particle migration, diagnostics and allocation overhead depend on implementation. The actual solver and accuracy may require a smaller timestep.

A hypothetical extra `40 micrometers` at the same spacing adds 4,000 grid planes: `160–320 billion` cells, or `12.8–25.6 TB` for the same ten float64 arrays before particles. This is a warning about literal uniform extrusion, **not a requirement for an optimized finite-3D experiment**. A dimensional change alters physics, particle loading, timestep constraints and scaling; a fixed multiplier is not a validated 3D budget.

For the reported hydrogen density, the earlier assistant calculated reference periods of roughly `0.50 fs` for the electron plasma scale, `21 fs` for the proton plasma scale and `2.67 fs` for an 800 nm optical wave. These are cold-plasma/optical reference calculations, not measured resonances of an evolving target. The [driven-plasma proposal](driven-plasma-selfs.md) must identify its actual modes independently.

## Do not reuse the earlier HF quote as a benchmark

The user asked about “HF”; the assistant interpreted it as Hugging Face, without a separate confirmation. It quoted A100 80 GB Spaces rates of `$2.50/hour`, `$10/hour` for four and `$20/hour` for eight, and guessed roughly `$20–$150` per 2D production run. It also illustrated a naive `$50 × 4,000 = $200,000` 3D extrapolation.

Those are **historical assistant estimates, not verified current prices, measured runtimes, a provider recommendation or spending authorization**. No evidence established that the initialized problem fits a particular device, its seconds per step, or multi-GPU scaling. The confident economic conclusions of that answer should not be carried forward as project results.

Replace them with an actual profile:

```text
wall_time = initialization + Nsteps * measured_mean_step_time + output
compute_charge = billed_allocation_hours * verified_allocation_rate
```

Record hardware, GPU count/topology, precision, build/configuration, particle count over time, initialization, representative early/late step timings, diagnostics/checkpoint costs, memory peaks, variability and billing scope. Add storage and transfer charges where applicable. More GPUs can reduce wall time without reducing total cost. No budget is approved by this brief.

## Proposed reproduction and speedup experiment

The assistant's first milestone was one reproducible EPOCH reference case, the corresponding existing-GPU-code implementation, a convergence report and an end-to-end timing breakdown. Compare spectra, angular distributions, beam charge in declared ranges, absorbed/deposited energy and relevant spatial fields. Account for injected energy and energy/particles leaving boundaries; maximum proton energy or total energy conservation alone is not sufficient.

Only after a usable baseline, test whether finer resolution is needed for physical features or to control numerical artifacts. Candidate investigations were selective refinement, particle loading/resampling, charge/energy-consistent numerical methods, collision/ionization coupling, diagnostics volume and evolving load balance. The chat also considered detailed kinetic generation coupled to a cheaper transport model, but noted the risk that downstream physics changes the source itself. None was selected or demonstrated for this nozzle.

For performance reporting distinguish kernel-only throughput from total runtime, serial-CPU versus optimized-GPU baselines, and cheaper restricted models versus unchanged physical questions. For illustration, making a kernel occupying 60% of runtime 100 times faster improves the full run by only `1 / (0.4 + 0.6/100) ~ 2.46`. Doubling cell spacing in three directions and doubling the timestep gives a conditional `16×` work reduction only if matched accuracy, sampling and per-update cost justify it. Neither calculation predicts a measured speedup.

Public numerical-method leads cited in the conversation, **not verified here**: [energy-conserving relativistic PIC](https://arxiv.org/abs/2302.01893), a [later per-particle correction proposal](https://arxiv.org/abs/2605.18542), [PIC–collision coupling](https://doi.org/10.1103/PhysRevE.111.025306), and [GPU collision benchmark artifacts](https://zenodo.org/records/15031733). The earlier assistant attributed larger allowable steps and estimated work reductions to these discussions; no end-to-end GPU/HEDP advantage should be inferred without reading and reproducing the applicable case.

## Relationship to PlasmaLab

Keep this as a demanding external reference question, not a prerequisite for the first playable browser loop. The browser can eventually inspect durable configurations, observations and comparisons from an external solver, but remote execution and full HEDP are deferred. A successful toy or SELF shadow predictor does not establish a validated micronozzle solver, useful fusion heating or a 10×–100× acceleration.
