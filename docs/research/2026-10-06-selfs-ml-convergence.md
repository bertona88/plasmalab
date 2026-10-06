# SELFS and learned plasma representations: convergence assessment

Date: 2026-10-06.

Status: assistant research assessment and proposed experiment design, requested by the user after commit [c1799d1](https://github.com/bertona88/plasmalab/commit/c1799d183b165e358b9f40a94563f10a90d6430d). Recommendations here are not adopted architecture decisions, trained models, reproduced paper results, or demonstrated acceleration.

Basis: the [divergent notebook](2026-10-05-attention-selfs-divergence.md), [ladder](../ladder.md), [SELFS abstract](../SELFS_WOFI_abstract.md), [project context](../project-context.md), [source inventory](../source-inventory.md), [foundation decision](../architecture/0001-foundation.md), [roadmap](../roadmap.md), and [driven-plasma brief](../experiments/driven-plasma-selfs.md). The original panpsychist framing, response analogy, and SELFS name belong to the user. The architectural synthesis, ranking, and tests below are assistant proposals. The current user explicitly requested convergence and a current literature search.

## 1. Recommendation

**Let the representation be learned. Keep response under interventions, persistent internal state, explicit physical exchanges, and a test of whether compression remains sufficient. Make a capable generic recurrent network a serious baseline.**

The strongest surviving hypothesis is:

> A specified family of plasma states may admit a compact predictive state that preserves the response distinctions needed for specified observations under specified future drives.

The research contribution would be demonstrating useful compression, transfer, or computational savings for that problem. Giving a latent vector an evocative name, adding attention, or displaying persistent colored groups does not establish it.

My first research experiment would compare a conventional kinetic reduction, a generic recurrent predictor, and a physical reduced model with a learned memory closure. Add response-grouped slots only after that comparison establishes what a compact state must retain. This is compatible with the ladder's shadow-predictor stage; it does not require an adaptive solver or a new universal world representation.

For the user's small-volume laser/target and early-heating interest, kinetic closures and nonlocal transport are the closest current HEDP targets. Whole-capsule ignition surrogates provide useful context but answer a different question. A small electrostatic benchmark can test the representation hypothesis before importing relativistic kinetics, mobile ions, collisions, radiation, or material response.

## 2. What to keep from c1799d1

| Notebook idea                                       | Assessment                  | Computable form or decisive condition                                                                                                                                     |
| --------------------------------------------------- | --------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Kinship through possible futures                    | Highest priority            | Compare outputs from branched reference states under a declared family of drives. Learn a state preserving these distinctions.                                            |
| Dormant memory and plasma echoes                    | Highest priority            | Compare instantaneous, history-based, and recurrent closures; test delayed probes after visible fields decay.                                                             |
| A learner and its adversary                         | Keep, starting simply       | Search for different physical states with the same compressed state and incompatible futures. Begin with constrained numerical search rather than another neural network. |
| A collective made of disagreement                   | Keep as a requirement       | Preserve streams, species, signs, phases, and exchanges when they affect outputs. A common mean is not enough.                                                            |
| Let the requested future guide attention            | Keep                        | Specify observable, horizon, tolerance, admissible drive, and rare-population requirements. Compare error versus retained state and total cost.                           |
| Interfaces as computational citizens                | Strong second target        | Learn a bounded response or closure with inputs, memory, outputs, and storage/flux accounting, for example nonlocal heat transport.                                       |
| Remove the beat before learning the dance           | Useful baseline or ablation | Compare laboratory coordinates with a justified drift/phase frame or learned transformation. Transform drives and observations consistently.                              |
| Attention as responsiveness                         | Optional mechanism          | Train response-conditioned pooling and signed interaction channels; compare ordinary attention, linear mixing, and no inter-slot interaction.                             |
| Give the unexplained remainder an identity          | Keep the residual test      | Compare coherent residuals with reference-resolution and sampling checks before adding a latent mode or candidate.                                                        |
| Uncertainty about the interior                      | Keep, with calibration      | Predict output distributions or errors for named horizons. Test calibration and failure detection; samples do not recover the original discarded state.                   |
| Several descriptions and an inspectable inner world | Useful browser experience   | Display competing predictions, retained information, and actual errors with reference provenance.                                                                         |
| Spacetime tokens and computational leases           | Defer authority             | Require a validated integration/coupling method and tested recovery before skipping physical work.                                                                        |
| Dynamic split/merge and an economy of attention     | Defer                       | First show that a fixed representation works. Then compare allocation with conventional error indicators, including all overhead.                                         |
| Small executable programs                           | Defer                       | Model discovery, compilation, and routing add another problem. A tiny learned closure may already be enough.                                                              |
| Learn the drive and description together            | Separate experiment         | Optimize an explicitly stated physical objective and computational objective; do not count an easier different experiment as acceleration.                                |
| Use plasma as a physical learning medium            | Separate research branch    | Requires controllability, reset, measurement, and hardware evidence. It does not help establish the first digital compression result.                                     |

The useful translation of the motivating assumptions is concrete: an interior becomes a hidden dynamical state; feeling becomes sensitivity to inputs; action/reaction becomes accounted exchanges with fields and surroundings; persistent selfhood becomes persistent predictive organization despite changing membership. The proposed test evaluates those computational claims. It does not distinguish metaphysical interpretations of the same physical trajectories.

## 3. What current HEDP and plasma ML actually do

This is a selected primary-source review through 2026-10-06, including both published work and explicitly labeled preprints. No paper's experiments were reproduced here.

### Closures inside physical solvers

Luo et al., published in September 2026, learn a Fourier neural operator mapping an electron-temperature profile to heat-flux divergence and embed it in an implicit energy solver. Their fixed-density, one-dimensional PIC benchmarks demonstrate resolution transfer and several held-out profiles. A hot-spot-trained model cannot reliably predict the separate sinusoidal test family. This gives both a practical target and a clear generalization failure [1].

Joglekar and Thomas's 2023 work adds a transported hidden variable for unresolved resonant-electron effects. A small neural network learns a growth coefficient while specified PDEs provide transport and feedback. The model transfers from periodic single-wave training to finite wavepacket behavior [2]. This is particularly close to the computational meaning of an internal state with memory.

Two 2025 studies provide important baselines. Huang, Dong, and Wang learn a fluid heat-flux closure with an FNO for nonlinear Landau damping [3]. Barbour et al. use a recurrent reservoir to close a Fourier–Hermite hierarchy, reducing required velocity resolution in selected nonlinear tests [4]. The latter's reported factor of sixteen concerns velocity resolution, not an established end-to-end speedup. These papers do not establish arbitrary-drive generalization for PlasmaLab.

### Learning the representation itself

An April 2026 preprint by Nam et al. compresses a 1,583-component atomic-population state into three latent variables and learns controlled latent dynamics. Physical-observable, stability, and steady-state constraints matter. This is a tin-plasma atomic-kinetics proof of concept under prescribed temperature/density histories and optically thin assumptions; self-consistent radiation-hydrodynamic deployment remains future work [5].

That is a direct affirmative answer to whether a network can learn the representation. The remaining questions concern which information it preserves and the conditions in which the resulting evolution is reliable.

### Diagnostics, inverse problems, and experiment-calibrated surrogates

JointDiff, published in August 2026, learns a joint generative model of simulation inputs and scalar/image outputs. Conditional generation supports forward prediction, inverse inference, and missing-output estimation; experimental fine-tuning improves transfer to NIF observations. Its training simulations use a reduced multirocket-piston model, and the task is inference rather than microscopic time stepping [6].

LLNL's 2025 ignition-prediction work combines simulation-based learning with experimental information and uncertainty treatment. It demonstrates the value of calibrated surrogates within a design program; it does not establish a general plasma evolution network [7].

Carvalho et al., published in June 2026, infer collision operators by differentiating through a Fokker–Planck solver fitted to PIC phase-space evolution. The demonstrated collisionality is that of finite-size PIC particles in the specified thermal setting; it should not automatically be identified with physical Coulomb collisions in arbitrary HEDP [8].

### Differentiable discovery and useful inductive biases

The March 2026 Joglekar et al. preprint connects differentiable plasma simulation with closure learning, diagnostic inversion, and drive optimization [9]. A differentiable solver can generate gradients for an experiment or train a small embedded function without replacing the entire physical evolution.

McGrae-Menge et al., published in February 2026, improve learned reduced plasma models using physical-symmetry data augmentation [10]. Appropriate coordinate symmetries can prevent misleading correlations. The symmetry must apply to the complete model, including its forcing and boundaries.

My inference from this selected set is that a well-defined scientific interface is a productive starting point: a heat-flux correction, transported memory variable, collision operator, atomic-state evolution, or conditional diagnostic distribution. The problem specification gives representation learning something measurable to preserve.

## 4. Why an arbitrary network might learn it

A sufficiently expressive network given enough relevant state and control information can attempt to learn the future-response map. Operator approximation results support expressivity under mathematical assumptions; they do not guarantee small data requirements, an efficient low-dimensional state, rollout stability, or transfer outside training support [11].

Separate four issues:

1. **Expressivity:** can the model class represent the desired function?
2. **Information:** do the input and retained state distinguish futures that matter?
3. **Learning:** do the data and objective expose those distinctions?
4. **Value:** is the resulting predictor sufficiently accurate, cheap, transferable, or inspectable?

The first issue cannot settle the other three.

### A precise obstruction is lost information

Let a full state be $s$, future drive be $u$, and requested response be

$$
R_H(s,u)=O\bigl(\Phi_H(s;u)\bigr).
$$

Suppose a compressor satisfies $E(s_1)=E(s_2)=z$, but for the same allowed drive

$$
d\bigl(R_H(s_1,u),R_H(s_2,u)\bigr)>2\varepsilon.
$$

Then no deterministic decoder $D(z,u)$ can be within $\varepsilon$ of both responses. The triangle inequality proves this: two points both within $\varepsilon$ of a common prediction must be within $2\varepsilon$ of each other.

A larger encoder, additional memory, or a less aggressive reduction can retain the distinction. A probabilistic decoder can report unresolved alternatives, but it cannot identify which original state was discarded merely by sampling.

For a specified Vlasov model, the complete distribution and fields already supply a state. Memory becomes useful when observation or compression removes information. An instantaneous network can succeed if its input remains sufficient; a finite recurrent network can fail if the retained memory is insufficient.

### Training must reward the right distinctions

Reconstructing a density picture is not the same objective as forecasting responses. A small high-energy population may contribute little to pixelwise error while controlling a requested tail or transport observable. Train on those observables and their response to changed forcing.

Randomized drives and multi-step prediction may already suffice for a generic network. A special response loss is an empirical option, not a necessary ingredient by definition. Its benefit needs an ablation.

### Predictive success does not identify unique entities

For an invertible latent transformation $T$, changing $z$ to $T(z)$, conjugating the dynamics, and modifying the decoder can leave every output unchanged. Prediction alone therefore need not identify a unique coordinate system or physical ontology.

The response interpretation has established precedent in predictive-state representations, where state is expressed through possible action-conditioned observations [12]. Equality of all allowed response maps can define an exact task-dependent equivalence. A finite tolerance gives approximate similarity, which is generally not transitive and should not be treated as a unique partition of nature.

Likewise, common forcing can make several populations useful to compute together without proving strong mutual coupling. Change the shared drive, detune it, or use localized admissible probes. Distinguish a reusable response template, one persistent physical candidate, and a GPU execution batch.

## 5. Attention is an implementation choice

Transolver already learns soft groupings of mesh features into a smaller token set and mixes information through those tokens [13]. Transolver-3, published in 2026, extends that family to much larger field-prediction meshes [14]. Neither establishes autonomous kinetic entities.

A 2026 AAAI study reformulates Transolver's Physics-Attention through linear attention and reports that slice-to-slice attention can hurt its tested benchmarks [15]. A serious SELFS comparison should include pooling with simpler mixing and no persistent identity.

Broad learned models are also progressing: the 2026 Walrus transformer is pretrained across nineteen continuum scenarios, including plasma examples [16]. That is meaningful evidence for general representation learning, while its stated continuum scope should not be enlarged into universal kinetic HEDP.

Slot Attention supplies a mechanism for learning a set of interchangeable representations [17]. Temporal identity, signed modes, species accounting, and physically meaningful exchanges require additional work. Positive assignment weights are compatible with signed values, but their normalization does not conserve energy or establish causality.

## 6. Network design from the assumptions

### First version: resolved physics plus learned memory

Use the normalized, periodic 1D1V electron Vlasov–Poisson problem with fixed neutralizing ions:

$$
\partial_t f+v\,\partial_x f-
(E_{\mathrm{self}}+E_{\mathrm{ext}})\,\partial_v f=0,
\qquad
\partial_xE_{\mathrm{self}}=1-\int f\,dv.
$$

Use a zero-mean self-field convention and a declared spatially varying external drive. Require periodic charge neutrality, $\int n(x)\,dx=L$, and negligible velocity-boundary flux over the comparison horizon; otherwise specify the boundary flux and include its budget terms. This is a kinetic representation benchmark, not a demonstration of mobile-ion or relativistic HEDP behavior.

Represent velocity dependence in a Fourier–Hermite solver and retain

$$
r_t=\{G_{m,k}(t):0\le m<M\}.
$$

For this first comparison, hold spatial/Fourier resolution fixed at a convergence-checked value and reduce only the velocity/Hermite moments. Reducing spatial resolution would introduce additional unresolved nonlinear terms that this closure does not supply.

Keep the known resolved equations and Poisson solve. A recurrent network supplies the missing closure coefficient or boundary contribution at the truncation:

$$
h_0=\operatorname{Enc}_\theta(s_0\text{ or warm history}),
$$

$$
h_{t+\Delta t}
=\operatorname{GRU}_\theta(h_t,r_t,E_{\mathrm{ext}}(t),\mu),
\qquad
\widehat G_{M,\cdot}(t)
=\operatorname{Dec}_\theta(h_t,r_t,E_{\mathrm{ext}}(t),\mu),
$$

$$
r_{t+\Delta t}
=\operatorname{Step}_{\mathrm{physical}}
(r_t,\widehat G_{M,\cdot}(t),E_{\mathrm{ext}}(t),\mu).
$$

These expressions specify responsibilities, not a validated integrator. The closure and recurrent update must use consistent stage times in the chosen implementation. The parameter vector $\mu$ contains declared model parameters. Real/imaginary channels retain the sign and phase of Fourier coefficients.

At forecast launch the encoder may inspect a full state or an explicitly specified warm-up history. During an autonomous forecast it uses its own evolving state and the known prescribed drive. Repeatedly reading the true future reference field or moments would test a continually corrected predictor instead.

Initial sweeps could use retained moment counts $M\in\{4,8,16,32\}$ and several small recurrent-state budgets. These are experimental choices, not accepted values. Specify whether a latent budget is global or per spatial mode, and include its total storage. A Fourier–Hermite basis is a useful first reference, not a promise of efficient representation far from its chosen velocity scale.

Keep the kinetic reference running independently to score the shadow forecast. This version directly tests the operational interior-state idea without requiring learned membership.

### Ambitious version: a learned set of response modules

If compact recurrent prediction works, replace the undivided memory with learned modules at the same total state budget:

| Component              | Proposed design                                                                           | Reason and test                                                                                                       |
| ---------------------- | ----------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------- |
| Physical inputs        | Phase-space patches or weighted samples, fields, species, drive history, model parameters | Preserve information needed to distinguish future responses.                                                          |
| Shared encoder         | A small convolutional/operator encoder; compare plain patch encoders                      | Start with standard tools whose cost is measurable.                                                                   |
| Assignment             | Soft attention into a fixed number of slots, plus an explicit residual channel            | Let membership change; do not require every structure to be a spatial blob.                                           |
| Internal state         | A recurrent vector per slot, initialized from the same information as the baseline        | Test retained history, rather than slot colors or names.                                                              |
| Input/output interface | Field/drive inputs; predicted closure contributions, currents, modes, or exchanges        | Give each prediction a physical meaning and a stated domain of validity.                                              |
| Interaction            | Signed, phase-sensitive learned messages, optionally conditioned on probe responses       | Compare generic attention and simpler mixing. Attention weights are not automatically physical coupling coefficients. |
| Decoder                | One assembly of the requested physical output and a distinct prediction record            | Avoid double-counted contributions.                                                                                   |
| Error estimate         | Per-observable, per-horizon uncertainty or ensemble disagreement                          | Calibrate on separate cases and measure missed failures.                                                              |

For example,

$$
h_i^{t+\Delta t}
=G_\theta\left(h_i^t,\operatorname{Read}_i(r_t,u_t),
\operatorname{Interact}_i(\{h_j^t\})\right)
$$

can describe the recurrent modules. Begin with fixed slot count and shared weights. Sweep the number and dimension of slots instead of immediately learning births, deaths, hierarchy, and split/merge.

A response-oriented attention variant could match a module's learned sensitivity to candidate incoming perturbations. That is an empirical architecture hypothesis. Plain attention may learn an equally useful relation; an explicit response Jacobian can be estimated from forked reference runs rather than assumed to be encoded in an attention map.

Global observational attention is permissible. If a closure later determines physical evolution, its field coupling and physical budgets must satisfy the selected numerical model. A Transformer mask is not an electromagnetic propagation solver.

### What must be enforced or checked

Learned high-order closure can preserve the structure of low-order conservation equations while still producing instability, inaccurate work, negative reconstructed distributions, or integration drift. Check these separately.

For the forced periodic benchmark, define normalized electron current $j=-\int vf\,dv$. The energy budget is

$$
\frac{d}{dt}
\left[\int\frac{v^2}{2}f\,dx\,dv+
\int\frac{E_{\mathrm{self}}^2}{2}\,dx\right]
=\int j E_{\mathrm{ext}}\,dx.
$$

The drive's work belongs in the balance. In electromagnetic extensions, field momentum, field energy, boundaries, and species exchanges also matter. Antisymmetric particle messages alone do not establish correct electromagnetic accounting.

A conservative decoder or projection must be derived for its specific interface. Soft assignments summing to one, a soft conservation penalty, or a plausible latent Hamiltonian are not general substitutes.

## 7. Training and decisive experiments

### Objectives

Train autonomous multi-step forecasts against named observations, normalized by their requested tolerances:

$$
\mathcal L_{\mathrm{roll}}
=\mathbb E_{s,u}\sum_{\tau\le H}\sum_j
w_j\,\ell\left(
\frac{\widehat y_j(\tau)-y_j(\tau)}{\varepsilon_j}
\right).
$$

An optional response loss compares changes caused by matched interventions:

$$
\mathcal L_{\mathrm{resp}}
=\sum_a\left\|
(\widehat y^{u_a}-\widehat y^{u_0})
-(y^{u_a}-y^{u_0})
\right\|^2_{\varepsilon}.
$$

Use complete trajectory, initial-state-family, and drive-family splits. Keep adjacent frames from one run within the same split. Score autonomous rollouts as well as one-step predictions. Preserve phase-sensitive and tail-sensitive outputs when required; average training loss is not a universal acceptance criterion.

Measure compression by retained degrees of freedom, memory, precision, and actual execution cost. A penalty on latent-vector magnitude is not a meaningful compression measure because latent coordinates can be rescaled. Decoder reconstruction is auxiliary unless the promised output is a reconstructed distribution.

### Step A: test whether there is something to compress

Before training a complicated network, measure response vectors from a modest set of reference states under several admissible perturbations. Inspect singular-value decay and simple fitted response models.

A low-rank empirical response matrix is a useful screening result for those states, probes, outputs, and horizons. It is not proof of a globally low-dimensional nonlinear state. Too few probes or too narrow an output can make low rank trivial.

### Step B: construct a closure counterexample

In a finite velocity discretization, find perturbations in the nullspace of the retained-moment projection, subject to nonnegative $f$. They keep the retained state fixed while changing unresolved structure. Apply the same drive and search for relevant future differences.

This establishes where additional information is necessary. It also supplies useful training and test cases. Start with constrained optimization or sampled perturbations; a learned adversary is optional.

### Step C: test memory under delayed intervention

Use linear damping as a numerical reference check, then nonlinear trapping and a delayed two-pulse/echo challenge. The echo is a deliberately discriminating memory test, not a claim about its usefulness in a dense ultrafast target.

The reference must resolve the velocity-space information and avoid numerical recurrence or excessive smoothing. Vary pulse timing, phase, amplitude, and supported spatial modes in held-out combinations. Separate model insufficiency from missing training coverage and reference discretization error.

The first output set should include complex electric-field modes, density/current or low moments, field/particle energy exchange, and echo timing/amplitude. Add a defined tail integral only if it is part of the declared task; specify its velocity range and normalization.

### Step D: run the fair comparison

| Comparison                                                            | What it tests                                                                       |
| --------------------------------------------------------------------- | ----------------------------------------------------------------------------------- |
| Converged kinetic reference and tuned conventional reduction          | Numerical reference and the non-neural alternative.                                 |
| Generic recurrent FNO or patch Transformer predictor                  | Whether a flexible learned representation already solves the problem.               |
| Resolved equations with a memoryless learned closure                  | Value of physical structure without recurrent state.                                |
| Resolved equations with recurrent learned closure                     | Incremental value of unresolved memory.                                             |
| Same recurrent model, with and without response loss                  | Whether explicit intervention supervision adds to ordinary driven rollout training. |
| Undivided memory versus fixed or learned groups at equal total memory | Whether factorization or learned membership helps.                                  |
| Ordinary attention, linear mixing, and no inter-slot interaction      | Whether the particular interaction story explains the gain.                         |

The first four are the initial comparison. Add the remaining ablations only to resolve a specific observed advantage. The generic model receives the same initialization information, histories, physical parameters, and prescribed controls. Compare common total prediction-state budgets, including the resolved variables $r$, recurrent state $h$, and temporal-history or attention caches. Also compare similar training effort, learning curves, and measured runtime; parameter count alone does not equal computational cost.

A candidate passes only relative to declared observable thresholds, horizons, and held-out regimes. Choose tolerances after establishing reference error, then fix them before the comparison. Report failure rates and worst-relevant cases alongside averages.

### Step E: determine what was established

- A good shadow forecast establishes predictive validity within the tested conditions.
- Fewer latent variables establish a form of state reduction, not automatically faster execution.
- Successful delayed-probe prediction shows preserved response information; it does not identify a unique natural entity.
- An improved error–cost curve over strong baselines supports the architectural choice.
- If a generic recurrent network wins, keep that implementation and retain SELFS as the predictive-state interpretation.
- If no compact model closes at useful accuracy, record the negative result and identify which observables or horizons require more state.

Do not claim acceleration of the authoritative run while the complete reference still executes alongside it. A later replacement trial must include initialization, encoding, decoding, checking, coupling, retained detailed state, transitions, and recovery in its cost.

For repeated runs, report data-generation and training costs separately and their amortization. A rough break-even calculation is

$$
N_{\mathrm{runs}}>
\frac{C_{\mathrm{data}}+C_{\mathrm{train}}}
{C_{\mathrm{reference/run}}-C_{\mathrm{learned/run}}},
$$

only when the denominator is positive and the learned per-run cost already includes the relevant overhead at matched requirements. No such costs have been measured for PlasmaLab.

## 8. What this could become in PlasmaLab

The near-term research outcome could be a browser experiment in which a user excites a plasma, waits until its visible field becomes quiet, changes the drive, and compares predictors with different retained state. The display would show what each predictor kept, where it failed, and how much it cost.

That provides intuition about kinetic memory even if there is no performance win. It also fits observe → track → predict before giving any description authority.

The next stronger result would be evidence that response modules transfer between compositions of streams, wavepackets, or species more efficiently than one global latent model. That would give learned entity structure a reason to exist beyond visualization.

Keep the technology ownership already chosen: Rust specifies the scientific problem and diagnostics; numerical execution remains accountable to it; WGSL/WebGPU is a possible deployment backend; TypeScript/Svelte presents interventions and evidence. Offline training can be an optional workflow with exported weights and normalization metadata. Choosing a training framework or browser inference dependency requires a separate implementation decision.

The current repository remains a foundation with no implemented plasma solver. This assessment defines a research path rather than reporting that the path has been built. The source ladder and delivery roadmap remain the authoritative records of adopted scope.

## 9. Sources and review limits

The linked primary pages, abstracts, or relevant full-text passages were checked for the claims used above. This is a targeted review, not a systematic novelty search. Publication status is stated as checked on 2026-10-06. No training dataset was downloaded and no external scientific result was independently reproduced.

1. Luo et al., “Resolution-Robust Machine Learning Heat Flux Closure for Inertial Confinement Fusion Plasmas,” PRX Intelligence 1, 013017, 3 September 2026. [Article](https://doi.org/10.1103/9l4n-mnz6); [full text](https://journals.aps.org/prxintelligence/pdf/10.1103/9l4n-mnz6). See the conclusion for failure across substantially different initial-condition families.
2. Joglekar and Thomas, “Machine learning of hidden variables in multiscale fluid simulation,” Machine Learning: Science and Technology 4, 035049 (2023). [DOI](https://doi.org/10.1088/2632-2153/acf81a); [author manuscript](https://arxiv.org/html/2306.10709v1).
3. Huang, Dong, and Wang, “Machine-learning heat flux closure for multi-moment fluid modeling of nonlinear Landau damping,” PNAS 122, e2419073122 (2025). [DOI](https://doi.org/10.1073/pnas.2419073122); [manuscript](https://arxiv.org/abs/2503.11090).
4. Barbour et al., “Machine-learning Closure for Vlasov-Poisson Dynamics in Fourier-Hermite Space,” Journal of Plasma Physics (2025). [DOI](https://doi.org/10.1017/S0022377825100822); [manuscript](https://arxiv.org/html/2504.13313v2). Its data/forecast protocol and moment-trace limitations need attention when reproducing a baseline.
5. Nam et al., “Physics-Informed Latent Space Dynamics Identification for Time-Dependent NLTE Atomic Kinetics,” preprint, 17 April 2026. [Manuscript](https://arxiv.org/html/2604.16664v1). The latent dimension and atomic-kinetics limitations are in sections 6–7.
6. Jones et al., “Joint Diffusion Approach to Multimodal Inference in Inertial Confinement Fusion,” PRX Intelligence 1, 013012, 18 August 2026. [Article](https://doi.org/10.1103/dldf-7tfm).
7. Spears et al., “Predicting fusion ignition at the National Ignition Facility with physics-informed deep learning,” Science 389, 727–731 (2025). [Paper](https://doi.org/10.1126/science.adm8201); [official LLNL account reviewed here](https://lmf.llnl.gov/news/llnl-researchers-employed-ai-driven-model-predict-fusion-ignition-shot).
8. Carvalho et al., “Learning collision operators from plasma phase space data using differentiable simulators,” Journal of Plasma Physics, 10 June 2026. [Full article](https://www.cambridge.org/core/journals/journal-of-plasma-physics/article/learning-collision-operators-from-plasma-phase-space-data-using-differentiable-simulators/AF996FF9346FDF028DE03EFCD3F408ED).
9. Joglekar et al., “Differentiable Programming for Plasma Physics: From Diagnostics to Discovery and Design,” preprint, 11 March 2026. [Manuscript](https://arxiv.org/html/2603.11231v1). An invited submission is not evidence of journal acceptance.
10. McGrae-Menge et al., “Embedding physical symmetries into machine-learned reduced plasma physics models via data augmentation,” Physical Review Research 8, 013200, 23 February 2026. [Article](https://doi.org/10.1103/9fq1-vpqz).
11. Kovachki, Lanthaler, and Mishra, “On Universal Approximation and Error Bounds for Fourier Neural Operators,” JMLR 22(290), 2021. [Primary article](https://www.jmlr.org/papers/v22/21-0806.html).
12. Littman, Sutton, and Singh, “Predictive Representations of State,” NIPS 2001. [Original paper](https://papers.nips.cc/paper_files/paper/2001/file/1e4d36177d71bbb3558e43af9577d70e-Paper.pdf). The proceedings landing page omits Singh; the original PDF supplies the author list.
13. Wu et al., “Transolver: A Fast Transformer Solver for PDEs on General Geometries,” ICML 2024. [Primary proceedings](https://proceedings.mlr.press/v235/wu24r.html).
14. Zhou et al., “Transolver-3: Scaling Up Transformer Solvers to Industrial-Scale Geometries,” ICML 2026. [Primary proceedings](https://proceedings.mlr.press/v306/zhou26ay.html).
15. Hu et al., “Transolver Is a Linear Transformer: Revisiting Physics-Attention Through the Lens of Linear Attention,” AAAI 2026, published 14 March 2026. [Primary proceedings](https://ojs.aaai.org/index.php/AAAI/article/view/37003).
16. McCabe et al., “Walrus: A Cross-domain Foundation Model for Continuum Dynamics,” ICML 2026. [Primary proceedings](https://proceedings.mlr.press/v306/mccabe26a.html).
17. Locatello et al., “Object-Centric Learning with Slot Attention,” NeurIPS 2020. [Primary proceedings](https://papers.nips.cc/paper/2020/hash/8511df98c02ab60aea1b2356c013bc0f-Abstract.html).

Learned allocation also has prior art: [LAMP, ICLR 2023](https://snap.stanford.edu/lamp/), learns refinement/coarsening with an error–cost objective on its demonstrated PDE and deformation tasks. It does not establish plasma SELF scheduling. The original notebook preserves the older plasma-echo and physical-neural-network sources behind its speculative branches.
