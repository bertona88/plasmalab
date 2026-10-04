# Historical PAN-RAD report — not independently reproduced

Status: a recovered assistant report, **not a verified repository result or an imported runnable prototype**. The available basis is the complete “Develop New HEDP Algorithm” assistant-response text surfaced earlier in the architecture conversation. Its source file is no longer retrievable in the current Files listing; the referenced ZIP, Python code, tests, and JSON outputs are also unavailable. See [source inventory](../source-inventory.md#referenced-artifacts-not-imported).

The user's prompt supplied a panpsychist/HEDP research framing. The assistant chose the radiation–matter problem, named **PAN-RAD — Potential-based Adaptive Neighborhood Radiation solver**, proposed the method, and reported having built and run it. Those are assistant-origin choices and reported outcomes, not user adoption of a PlasmaLab solver. This review inspected the report, not the implementation, and ran none of its numerical checks.

## The reported scientific construction

The stated model is a stationary material with radiation energy density `E`, material temperature `T`, positive diffusion coefficient `D`, positive exchange rate `γ`, and constant positive volumetric heat capacity `C`. It uses closed boundaries; coefficients may vary in space but were not temperature-dependent in the described implementation.

```text
∂E/∂t = ∇·(D∇E) + γ(aT⁴ − E)
C ∂T/∂t = γ(E − aT⁴)
```

For each cell the response defines integrated reservoirs `r_i = V_i E_i` and `m_i = V_i C_i T_i`, collected in `y`. An oriented incidence matrix `B` has a donor `−1` and receiver `+1` in each transfer column. The proposed update solves for integrated transfers `q`:

```text
y_new = y_old + B q
1ᵀ B = 0
```

This is the report's conservation argument: every proposed transfer debits one reservoir and credits another, so total closed-system energy is unchanged algebraically at every iterate. It is not an after-the-fact energy repair. This bookkeeping identity alone does not verify the discretization, implementation, or physical accuracy.

The reported reservoir potentials are `φ_r = r/V = E` and `φ_m = a(m/(VC))⁴ = aT⁴`. With positive diagonal edge conductances `K` (diffusion faces `AD/ℓ`, local exchange `Vγ`), backward Euler is written as `q = −Δt K Bᵀ φ(y_old + Bq)`. The response gives the objective and Hessian:

```text
J(q) = qᵀK⁻¹q/(2Δt) + Σ_v Ψ_v(y_v)
Ψ_r(r) = r²/(2V)
Ψ_m(m) = a m⁵/[5(VC)⁴]
H = (Δt K)⁻¹ + Bᵀ diag(φ′(y)) B
```

For the stated positive-energy model it reports a symmetric positive-definite Newton matrix, solved with preconditioned conjugate gradients. A fraction-to-boundary line search and objective-decrease test were used to preserve positivity without clipping or renormalization. `J` was explicitly a numerical potential, not physical entropy or a consciousness measure. The response warns not to assume this convex construction survives new constitutive laws unchanged.

## Adaptive neighborhoods and the original algorithm sketch

Each interaction is scored by `π_e = g_e²/H_ee`, where `g` is the objective gradient. The report calls this a local quadratic correction-value estimate: **solver inconsistency, not physical importance or time-discretization error**. It explicitly does not claim to have invented curvature-aware selection.

The proposed preconditioner seeds at high-score interactions and grows neighborhoods using normalized Hessian coupling weighted by the neighboring residual score:

```text
ω_ef = |H_ef| / sqrt(H_ee H_ff) · (1 + sqrt(π_f/(π_max + ε)))
```

The described patch limit was eight transfers, with every interaction assigned to exactly one patch. Each patch's Hessian block was factored for the global solve. Local information chose which unknowns to solve together; no equations were dropped and the complete residual was still checked globally.

The following is the original report's pseudocode, not Python and not an executable substitute for the missing source:

```text
Given positive reservoir energies y_old and a time step dt:

    q = 0

    repeat:
        y = y_old + B q

        evaluate full transfer residual
        if converged:
            return y

        form objective gradient g and Hessian H

        score each interaction: pi[e] = g[e]^2 / H[e,e]
        construct residual-seeded, strongly coupled patches
        factor the patch Hessian blocks

        dq = global_PCG(H, -g, patch_preconditioner)

        alpha = positivity-preserving objective line search
        q = q + alpha * dq
```

## Reported outcomes, with their limits

For three synthetic 64-cell cases with 127 transfer unknowns, the response reports these total conjugate-gradient iterations across Newton iterations for **one physical timestep**:

| Case                       | Diagonal | Fixed physics-local blocks | Adaptive PAN-RAD blocks |
| -------------------------- | -------- | -------------------------- | ----------------------- |
| Moderate exchange          | 90       | 46                         | 55                      |
| Stiff exchange             | 124      | 74                         | 82                      |
| Heterogeneous coefficients | 200      | 98                         | 113                     |

The negative result matters: adaptive patches improved on the diagonal baseline but **lost to the fixed blocks in all three cases**, at the same maximum block size. The report says adaptive patch construction also made the small Python runs slower in wall time. It supplies no usable timing table here and labels those timings single runs, not statistical benchmarks. Do not convert these iteration counts into speedup claims.

The reported limited checks were maximum relative energy drift about `2 × 10^-16`; agreement with a sparse-direct solve of the same equations below `5.6 × 10^-16` in relative maximum difference; positive monitored reservoirs; timestep-refinement orders `0.958, 0.978, 0.989`; unchanged uniform equilibrium with zero Newton iterations; and uniform stiff relaxation toward `E = aT⁴`. These values have **not** been reproduced or independently checked here. Direct-solve agreement tests implementation consistency, not physical validity. The source explicitly says the relaxation check is not a full radiation-transport asymptotic-preserving test.

## Recovery instructions and relationship to PlasmaLab

The original report linked `PAN_RAD_prototype.zip`, `pan_rad/pan_rad.py`, `pan_rad/benchmark_results.json`, and `pan_rad/verification_results.json`, and supplied these commands after extraction:

```sh
cd pan_rad
python -m pip install -r requirements.txt
python pan_rad.py
python test_pan_rad.py
```

These commands are preserved instructions, **not runnable instructions for this repository today**. `requirements.txt`, `test_pan_rad.py`, the source, inputs, exact runtime, tolerances, and result files still need recovery. Do not fabricate replacement code or result JSON under the original names. When originals become available, preserve and hash them, inspect dependencies and safety before running, record the actual environment, and keep rerun results distinct from this report.

The reusable lesson is conservative exchange accounting and a strong fixed baseline that an adaptive strategy must beat at matched requirements and full cost. The report's next proposed target was a cheaper residual-aware rebuilding criterion, retaining the fixed-block baseline. Temperature-dependent opacity, multigroup emission, hydrodynamics, non-orthogonal meshes, and scalable coarse corrections were left for derivation and testing.

This is a radiation–matter kernel report, not a reconnection model, a PIC implementation, a detector, or a demonstrated SELF representation switch. Its possible relevance to the ladder is bookkeeping and evaluation discipline; adopting its solver would be a separate scientific decision. The report disclaims established novelty and does not establish a general 10×–100× gain. Its embedded literature links were not independently reviewed during this import.
