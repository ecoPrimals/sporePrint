+++
title = "Sovereign WDM Simulation on Consumer GPU"
description = "Plasma Physics x GPU Compute — warm dense matter on consumer GPU, guideStone v0.7.0 certified. hotSpring. 59/59 checks."
date = 2026-03-17

[extra]
paper_number = 7
domain = "Physics and Materials"

[[extra.companions]]
url = "/thesis/08-results-hotspring/"
title = "Chapter 8: Results — hotSpring"
relation = "extends"
label = "Computational plasma physics — Sarkas MD, nuclear EOS, lattice QCD"

[taxonomies]
primals = ["barracuda", "beardog", "biomeos", "coralreef", "songbird", "toadstool"]
springs = ["groundspring", "hotspring", "neuralspring"]
+++

**Date:** March 14, 2026 (updated)
**Status:** **Validated + Live Kokkos Parity + Precision Stability + Precision Brain + VFIO PBDMA Context Load** — {{ entity(name="hotspring") }} v0.6.31, **848 lib tests**, 115 binaries, 85 WGSL shaders. All plasma MD, lattice QCD, and nuclear HFB reproduction complete. GPU promotion: Papers 43 (gradient flow, 38.5× speedup) and 44 (BGK dielectric, 12/12 physics checks). **Full multi-tier precision stability analysis** (Exp 046): 9 cancellation families audited across f32/DF64/f64/CKKS FHE. Stable BCS v² and plasma W(z) algorithms enable DF64 throughput (16× on consumer GPUs) without precision loss. **Precision brain** (Exp 049): self-routing hardware calibration, NVVM device poisoning discovered and gated, dual-GPU cooperative patterns (Split BCS 2.2×, Split HMC, Redundant, PCIe 1.2 GB/s). **Live Kokkos benchmark** (Exp 053): 9/9 Yukawa cases, **12.4× gap** ({{ entity(name="barracuda") }} 212 steps/s vs Kokkos-CUDA 2,630 steps/s at N=2000) — gap dominated by native f64 fallback (1:32 on Ampere), DF64 safe-path fix expected to close to ~2×. DF64 transcendental poisoning bug discovered and fixed. **VFIO PBDMA context load** (Exp 058): 3 critical Volta register discoveries (preempt 0x002638, ACK 0x002A00, SIGNATURE validation), PBDMA2 loads RAMFC with zero errors; USERD DMA read remaining. {{ entity(name="coralreef") }} P10 Iter 52+. Dual Titan V mmiotrace planned. Zero clippy warnings (lib+bins), zero unsafe, all AGPL-3.0-only. 60/60 Sarkas observable checks (N=10k, 80k steps, $0.044). Deconfinement phase transition at β_c=5.69 on RTX 3090 (32⁴, 13.6h, $0.58). DF64 core streaming delivers 9.9× native f64 throughput. Verlet neighbor list achieves 992 steps/s (κ=3). Transport coefficients D*, η*, λ* via GPU Green-Kubo.
**Domain:** Plasma physics, computational science, distributed computing
**Novelty:** No prior work demonstrates full WDM transport coefficient
reproduction on consumer GPU via vendor-agnostic shaders; no prior work
frames distributed consumer GPU networks as alternatives to institutional
HPC for plasma physics
**Cross-Spring:** {{ entity(name="hotspring") }} (MD + transport + lattice QCD) × {{ entity(name="neuralspring") }}
(surrogate learning + LSTM) × {{ entity(name="groundspring") }} V113 (uncertainty propagation — GemmF64 transpose (Tikhonov KᵀK/KᵀG), RetryPolicy + CircuitBreaker, 4-format capability parsing, exit_code constants, 102 barracuda delegations)

---

## Abstract

We demonstrate that warm dense matter (WDM) — the plasma regime central
to inertial confinement fusion (ICF), planetary interiors, and stellar
evolution — can be simulated on consumer GPU hardware using the BarraCuda
compute stack. The 70-author "Roadmap for warm dense matter physics"
(Murillo et al., arXiv:2505.02494, revised Feb 13, 2026) identifies
computational accessibility as a critical bottleneck: state-of-the-art
codes require institutional HPC allocations, creating artificial scarcity
in who can do WDM science.

We propose that {{ entity(name="hotspring") }}'s validated pipeline — Yukawa MD (9/9 GPU),
Green-Kubo transport (13/13), nuclear EOS (195/195), screened Coulomb
(23/23), and lattice QCD (full GPU pipeline) — already contains the
primitives needed for WDM simulation at modest system sizes. The gap is
not capability but scale, and scale is a distribution problem, not an
algorithm problem.

We frame this as the **GPU-as-hot-water-heater** thesis: every consumer
GPU running WGSL shaders through open Vulkan drivers can contribute to
WDM computation while its waste heat serves domestic purposes. A network
of 1,000 idle consumer GPUs, each contributing 1 GPU-hour/day, provides
365,000 GPU-hours/year — equivalent to a mid-tier institutional HPC
allocation — at zero marginal compute cost and with waste heat as a
useful byproduct.

---

## 1. The Problem: WDM Computation Is Artificially Scarce

### 1.1 What Is Warm Dense Matter?

WDM occupies the regime between cold condensed matter and hot classical
plasma: temperatures of 10⁴–10⁸ K, densities of 0.1–100 g/cm³. At these
conditions, neither cold-matter approximations (band theory, perturbation
theory) nor hot-plasma approximations (Debye-Hückel, ideal gas) apply.
The electrons are partially degenerate, ions are strongly coupled, and
quantum effects coexist with classical dynamics.

WDM matters because it describes:
- **ICF fuel**: the deuterium-tritium capsule during NIF implosion
- **Planetary cores**: Jupiter, Saturn, super-Earths
- **Stellar interiors**: white dwarf envelopes, brown dwarfs
- **Astrophysical shocks**: supernovae, neutron star mergers

### 1.2 The Computational Bottleneck

The WDM roadmap (arXiv:2505.02494) identifies several open computational
challenges:
1. **Transport coefficients** at WDM conditions (partially ionized,
   strongly coupled) — extending Stanton-Murillo (2016) beyond the
   classical regime
2. **Equation of state** for mixtures under compression — beyond SEMF/HFB
3. **Dynamic structure factor S(q,ω)** — the key experimental diagnostic
   for X-ray Thomson scattering (XRTS) at NIF
4. **Orbital-free DFT** for large-scale WDM simulations
5. **Wavepacket MD** for quantum ion dynamics

Current state: these calculations require institutional HPC (Frontier,
Summit, Perlmutter). A typical WDM transport calculation uses 4–8 GPUs
for days. Allocation proposals take months. Results are published behind
paywalls.

### 1.3 The Sovereign Alternative

{{ entity(name="hotspring") }} has already reproduced Stanton-Murillo transport on a single
RTX 4070 at ~$0.02 compute cost. The Yukawa MD pipeline runs at 34.7×
CPU speed on GPU. The lattice QCD pipeline achieves 40× CPU speed with
streaming GPU HMC. The nuclear EOS pipeline covers 195 nuclei.

The question is not whether consumer GPU can do WDM physics — {{ entity(name="hotspring") }}
has already proven it can for classical plasma. The question is whether
the extensions to WDM conditions (partial ionization, quantum effects,
higher temperatures) are tractable on the same hardware.

---

## 2. What hotSpring Already Has

### 2.1 Validated Primitives

| Primitive | {{ entity(name="hotspring") }} Paper | Checks | WDM Extension |
|-----------|----------------|--------|---------------|
| Yukawa MD (all-pairs + cell-list) | Paper 1 | 9/9 GPU | Extend to screened potentials with partial ionization |
| Green-Kubo transport (D*, η*, λ*) | Paper 5 | 13/13 | Extend to WDM conditions (higher T, Z*) |
| Nuclear EOS (SEMF→HFB) | Paper 4 | 195/195 | Use as cold-curve input for WDM EOS |
| Screened Coulomb eigensolve | Paper 6 | 23/23 | Yukawa screening at WDM parameters |
| FFT (1D + 3D, f64) | {{ entity(name="toadstool") }} | 14/14 GPU | Required for S(q,ω) computation |
| Lattice QCD HMC | Papers 8-12 | Full GPU | Monte Carlo sampling methodology |
| Streaming GPU dispatch | Paper 10+ | 9/9 | Zero CPU→GPU transfer architecture |

### 2.2 The FFT Gap Is Closed

{{ entity(name="toadstool") }} commit `1ffe8b1a` delivered `Fft1DF64` and `Fft3DF64` with
roundtrip validation to 1e-10 on RTX 3090. This was THE major blocker
for S(q,ω) computation. With FFT available, the dynamic structure factor
becomes:

```
S(q,ω) = (1/N) |∫ Σ_j exp(iq·r_j(t)) exp(-iωt) dt|²
```

which is a spatial Fourier transform (over particle positions) followed
by a temporal Fourier transform (over MD trajectory). Both transforms now
run on GPU.

### 2.3 What's Missing

| Gap | Effort | Priority |
|-----|--------|----------|
| Partial ionization model (Z* from Thomas-Fermi or average-atom) | Medium | P1 |
| Electron-ion coupling (two-temperature model extension) | Medium | P1 |
| Wavepacket evolution (quantum ion dynamics) | High | P3 |
| Orbital-free kinetic energy functional | High | P3 |
| Multi-component mixture EOS | Medium | P2 |

---

## 3. The GPU-as-Hot-Water-Heater Thesis

### 3.1 The Core Argument

A consumer GPU running WDM simulations at 200W produces:
- **Useful computation**: ~10 TFLOPS f64 sustained
- **Waste heat**: 200W thermal output

In a residential setting, this waste heat can offset space heating or
water heating costs. A GPU mining cryptocurrency wastes energy on proof
of meaningless work. A GPU running WDM simulations wastes energy on
proof of meaningful work — the same Joules produce both heat and science.

### 3.2 The Economics

| Resource | Institutional HPC | Consumer GPU Network |
|----------|-------------------|---------------------|
| Hardware | $600M (Frontier) | $600/node × N nodes |
| Allocation | Competitive proposal (months) | Volunteer (immediate) |
| Energy | Grid power at industrial rate | Residential, offset by heating |
| Access | Credential-gated | Open (AGPL-3.0) |
| Results | Journal paywall | Public domain |
| Vendor lock | CUDA (NVIDIA-only) | WGSL/Vulkan (any GPU) |

### 3.3 Why WGSL/Vulkan Matters

BarraCuda's WGSL shaders run on any GPU exposing Vulkan with
SHADER_F64. This includes:
- NVIDIA (RTX 2070 through 5090, Titan V)
- AMD (RX 6000+, MI-series)
- Intel (Arc A-series)
- Qualcomm (Adreno, via Android Vulkan)
- Apple (via MoltenVK translation layer)

CUDA locks WDM computation to NVIDIA. WGSL liberates it. The same
physics shader runs identically on a $300 used RTX 2070 and a $2000
RTX 5090 — only the speed changes, not the math.

### 3.4 Distributed Architecture

The {{ entity(name="nucleus") }} mesh ({{ entity(name="biomeos") }} + {{ entity(name="beardog") }} + {{ entity(name="songbird") }}) provides:
- **Task distribution**: BOINC-style work units, but with covalent trust
- **Verification**: deterministic MD trajectories verify via hash
- **Aggregation**: independent parameter sweeps combine trivially
- **Fault tolerance**: any node can fail; work units redistribute

WDM transport calculations are embarrassingly parallel across the
(κ, Γ, T) parameter space. Each point is an independent MD run. A
network of 100 GPUs can sweep 100 parameter points simultaneously.

---

## 4. Experimental Design

### 4.1 Phase 1: Reproduce FPEOS on Consumer GPU

**Target**: Militzer's First-Principles Equation of State database
(Berkeley, open C++/Python code). Reproduce EOS tables for hydrogen,
helium, and carbon at WDM conditions.

**Method**: Port average-atom + MD pipeline to BarraCuda. Validate
against published FPEOS tables. Measure accuracy vs compute cost.

**Success criterion**: Agreement with FPEOS tables to within published
uncertainties, running on a single RTX 4070.

### 4.2 Phase 2: WDM Transport Coefficients

**Target**: Extend Stanton-Murillo (Paper 5) to WDM conditions.

**Method**: Run Yukawa MD at elevated temperatures (T > 10 eV) with
density-dependent screening. Compute Green-Kubo transport. Compare
to published DFT-MD values from the roadmap comparison studies.

**Success criterion**: D*, η*, λ* within 50% of DFT-MD for at least
3 (ρ, T) points in the WDM regime.

### 4.3 Phase 3: Dynamic Structure Factor

**Target**: Compute S(q,ω) from MD trajectories for comparison with
NIF XRTS experimental data.

**Method**: Spatial FFT of particle positions → intermediate scattering
function F(q,t) → temporal FFT → S(q,ω). All on GPU via validated
FFT primitives.

**Success criterion**: S(q,ω) peak positions and widths match published
MD results for hydrogen at WDM conditions.

### 4.4 Phase 4: Distributed Parameter Sweep

**Target**: Full (ρ, T) sweep of transport coefficients using the
{{ entity(name="nucleus") }} mesh (2 GPUs initially, scaling to N).

**Method**: {{ entity(name="biomeos") }} distributes MD work units across available GPUs.
Each GPU runs an independent (κ, Γ) point. Results aggregate into a
transport table.

**Success criterion**: Linear scaling of throughput with GPU count.
Deterministic verification of all results via trajectory hash.

---

## 5. Connection to NIF and the Ignition Era

### 5.1 NIF Context

The National Ignition Facility achieved fusion energy gain in December
2022, with 6 subsequent successful shots reaching peak gain of 2.3× at
5.2 MJ (as of the Feb 2026 NIF/JLF User Groups Meeting). This has
created unprecedented demand for WDM simulation to design future targets,
understand capsule physics, and optimize implosion conditions.

### 5.2 Murillo's Role

Michael Murillo (MSU, Computational Mathematics, Science, & Engineering)
co-authored the WDM roadmap (arXiv:2505.02494) and has published
extensively on transport coefficients in dense plasma. His Stanton-Murillo
(2016) transport paper is {{ entity(name="hotspring") }} Paper 5 — the first complete
reproduction in our pipeline. His screened Coulomb work is Paper 6.

{{ entity(name="hotspring") }} was built to validate BarraCuda against Murillo's published
results. Extending to WDM conditions is the natural next step — using
the same infrastructure, the same validation methodology, the same
consumer hardware.

### 5.3 What This Proves

If a single graduate student with a $600 GPU can reproduce WDM transport
coefficients that currently require institutional HPC allocations, it
demonstrates that:

1. **The computation is not the bottleneck** — the algorithm is
2. **Access to physics is artificially scarce** — not technically scarce
3. **Open-source GPU stacks can do real science** — not just benchmarks
4. **Distributed consumer GPU networks are viable** — not just theoretical

---

## 6. Connection to Other baseCamp Sub-Theses

| Sub-Thesis | Connection |
|-----------|------------|
| 01 (Anderson QS) | Anderson localization is spectral theory; WDM uses same eigensolve primitives |
| 02 (Frozen Fossil) | Constrained evolution under extreme thermal constraint (WDM is the ultimate thermal constraint) |
| 03 (Bioag) | Distributed sensing → distributed computing; same {{ entity(name="nucleus") }} infrastructure |
| 04 (Sentinel) | WDM diagnostics (XRTS) are a form of environmental sensing under extreme conditions |
| 05 (Cross-species) | Multi-component plasma mixtures are the physics analog of multi-species communities |
| 06 (No-till) | Both apply physics principles (Anderson, transport) to applied problems using consumer compute |

### 6.1 neuralSpring Integration

{{ entity(name="neuralspring") }} contributes validated ML surrogates, spectral analysis, and
df64-validated protein folding primitives directly to WDM science:

- **df64 core streaming** (Session 88): All 15 {{ entity(name="helixvision") }} WGSL shaders
  evolved to the hotSpring/ToadStool three-zone pattern (f64 buffer I/O →
  df64 compute → f64 output). Validates that df64 generalizes from nuclear
  physics to ML workloads. Two precision tiers: arithmetic 3.6e-8 to 5.6e-7,
  transcendental 1.7e-4 to 3.4e-4. 37/37 GPU checks on RTX 4070
- **WDM surrogates** (nW-01 through nW-05): **All 5 complete** (Session 88+)
  - nW-01: MLP transport surrogate (D*, η*, λ*) — 30/30 Rust checks
  - nW-02: MLP EOS surrogate P(ρ,T), E(ρ,T) — 36/36 + 15/15 GPU checks
  - nW-03: LSTM reservoir S(q,ω) peak predictor — 27/27 Rust checks, R²=0.98
  - nW-04: MLP classical→WDM transfer learning — 6/6 Rust checks
  - nW-05: ESN regime classifier (Classical/WDM/Degenerate) — 39/39 Rust checks, 96.5% accuracy
- **Reservoir computing**: nW-03 (LSTM) and nW-05 (ESN) demonstrate that
  reservoir computing (fixed random weights + ridge regression readout) is
  effective for WDM sequence analysis and regime classification
- **Spectral analysis**: {{ entity(name="neuralspring") }}'s `eigh_f64` eigendecomposition and
  `spectral_entropy` (rewired to `barracuda::stats::shannon_from_frequencies`
  in Session 81) apply directly to plasma eigenmode analysis
- **Session 90 status**: 669 lib tests, 179 binaries, 179/179 validators,
  131+ named tolerances, 42 upstream rewires, 36 Python baselines (all
  deterministically seeded), 3,162+ total validation checks. nF-02 AlphaFold2
  Evoformer block pipeline validated end-to-end. Phase B GPU gaps all closed:
  ODE batch integration, FST variance decomposition, introgression HMM chain

### 6.2 groundSpring V113 Integration

{{ entity(name="groundspring") }} V113 (GemmF64 transpose (Tikhonov KᵀK/KᵀG), RetryPolicy + CircuitBreaker, 4-format capability parsing, exit_code constants. V112: `OrExit<T>`, `parse_benchmark()`, socket_env_var(), provenance trio. 102 barracuda delegations,
29/29 validation binaries, 140 {{ entity(name="metalforge") }} checks)
provides the inverse problem and uncertainty machinery for WDM science.

{{ entity(name="groundspring") }}'s Bazavov experiments (Exp 019-021) are **direct lattice QCD
contributions** — the same physics that {{ entity(name="hotspring") }} simulates on GPU:

- **Exp 020 — Freeze-out inverse problem** (Bazavov et al., Phys Rev D 93,
  014512, 2016): Recovers freeze-out temperature T₀ and curvature κ₂ from
  heavy-ion collision data via Taylor expansion and 2D grid search. This is
  the inverse problem {{ entity(name="hotspring") }}'s lattice QCD pipeline generates data for —
  {{ entity(name="groundspring") }} validates the statistical inference that turns lattice output
  into physical observables. 8/8 Py, 8/8 Rust checks
- **Exp 021 — Spectral function reconstruction** (Bazavov et al., arXiv
  2501.12259, 2025): Tikhonov-regularized inversion of Euclidean correlators
  to recover spectral functions via Laplace-transform kernel and Cholesky
  decomposition. This is the ill-posed inverse problem at the heart of
  extracting physics from lattice QCD data — the same mathematical challenge
  faced by WDM dynamic structure factor S(q,ω) extraction. 8/8 Py, 8/8
  Rust checks
- **Exp 019 — Jackknife error estimation** (Bazavov et al., Phys Rev D 111,
  094508, 2025): Delete-one and block jackknife for subpercent precision
  error bars. This is the standard error estimation method used in every
  lattice QCD publication — {{ entity(name="groundspring") }} validates the statistical machinery
  that {{ entity(name="hotspring") }} will need for production uncertainty quantification. 9/9
  Py, 9/9 Rust checks

**Why this matters for WDM**: {{ entity(name="hotspring") }} generates raw simulation data
(trajectories, correlators, transport integrals). Converting that data into
physical observables with rigorous uncertainty requires exactly the inverse
problem and error estimation machinery that {{ entity(name="groundspring") }} validates.
The freeze-out curve is a thermodynamic observable extracted from lattice data;
the spectral function is a dynamic observable extracted from Euclidean correlators.
{{ entity(name="groundspring") }} proves the extraction math works at benchmark precision before
{{ entity(name="hotspring") }} applies it to production WDM data.

**Combined pipeline**: {{ entity(name="hotspring") }} (GPU simulation) → {{ entity(name="groundspring") }} (inverse
problem + error bars) → {{ entity(name="neuralspring") }} (surrogate acceleration). This is the
full lattice QCD workflow, validated independently in three springs.

### 6.3 groundSpring WDM Uncertainty Budget (Exp 025–027)

{{ entity(name="groundspring") }} Experiments 025–027 provide the **uncertainty budget** that
validates the numerical claims in this paper's consumer-GPU WDM pipeline:

- **Exp 025 — f32 vs f64 precision drift**: Measures systematic bias from
  single-precision arithmetic across WDM observables (pair correlation g(r),
  diffusion D*, viscosity η*, thermal conductivity λ*). Key result: **28%
  systematic bias** in f32 transport coefficients at Γ>10, proving f64
  (or DF64 emulation) is mandatory for production WDM. 6/6 Py, 6/6 Rust checks
- **Exp 026 — System-size convergence**: Finite-size scaling analysis for
  WDM MD simulations. Extrapolation to thermodynamic limit via 1/N^(1/3)
  linear fit achieves R² > 0.999, confirming current systems are within 1%
  of D_inf. Establishes minimum system sizes for each observable. 8/8 Py,
  8/8 Rust checks
- **Exp 027 — GPU vendor parity**: Cross-vendor comparison of WDM trajectory
  output between GPU architectures (RTX 4070 vs Titan V). Differences at
  1e-12 relative level, confirming IEEE 754 compliance and reproducibility
  across consumer and workstation GPUs. Critical for {{ entity(name="nucleus") }} multi-gate
  dispatch where different gates run different GPU hardware. 6/6 Py, 6/6
  Rust checks

**Why this matters**: Before claiming "$19 WDM on consumer GPU," the
uncertainty budget must prove that (a) f32 alone is insufficient (Exp 025),
(b) the system sizes used actually converge (Exp 026), and (c) results are
reproducible across GPU vendors (Exp 027). These three experiments convert
the cost projection in Section 8 from speculation to validated science.

### 6.4 VFIO Sovereign Dispatch Breakthrough (Exp 058)

The sovereign VFIO compute path on Volta (Titan V, GV100) achieved a
critical milestone: **PBDMA context load** — the hardware PFIFO scheduler
successfully loads our RAMFC channel context into PBDMA2. This is the first
successful context load on the sovereign dispatch path without any kernel
GPU driver.

Three register-level discoveries made this possible:

1. **GV100 runlist preempt** (`0x002638`): Write `BIT(runl_id)` to force
   scheduler re-evaluation. Volta's per-runlist preempt is at 0x002638,
   not the older per-channel preempt at 0x002634.
2. **Runlist completion ACK** (`0x002A00`): After runlist submission, the
   scheduler fires PFIFO_INTR bit 30. Software MUST read 0x002A00 and
   write `BIT(runl_id)` to acknowledge. Without this, the scheduler will
   not dispatch channels to PBDMAs.
3. **SIGNATURE validation**: PBDMA enforces `RAMFC::SIGNATURE = 0xFACE`.
   Used as a diagnostic (write 0xDEAD → observe error → confirm fresh
   context load).

**Current status**: PBDMA2 has our context loaded with zero errors. The
GPFIFO address, USERD address, channel ID, and all RAMFC fields are
correctly mapped into PBDMA operational registers. One gap remains: the
PBDMA does not read GP_PUT from USERD in system memory (suspected IOMMU
DMA path issue).

**Hardware plan**: GTX 1050 (headless display) + 2x Titan V — one on
nouveau as an mmiotrace oracle, one on VFIO as the target. The oracle
will capture nouveau's complete PBDMA dispatch sequence for replication
on the VFIO target.

**{{ entity(name="coralreef") }} pin**: Phase 10, Iteration 52+ (Experiment Q: VramFullDispatch)

See: `hotSpring/experiments/058_VFIO_PBDMA_CONTEXT_LOAD.md`

---

## 7. Reproduction Targets

See `hotSpring/specs/PAPER_REVIEW_QUEUE.md` Tier 4 for the full list
of WDM reproduction targets (Papers 32-42).

### Priority Order

1. **Paper 33 (atoMEC)**: Average-atom model — ideal Phase 0 Python control
2. **Paper 32 (FPEOS)**: EOS tables — data validation, interpolation
3. **Paper 35 (WDM transport)**: Extend Stanton-Murillo to WDM conditions
4. **Paper 38 (S(q,ω))**: Dynamic structure factor — FFT + MD
5. **Paper 36 (Dragon OF-DFT)**: Orbital-free DFT — new primitive
6. **Paper 40 (XRTS diagnostics)**: Model-free temperature extraction

---

## 8. Cost Projection

| Phase | Hardware | Time | Cost |
|-------|----------|------|------|
| Phase 1 (FPEOS reproduction) | 1× RTX 4070 | ~1 week | ~$2 |
| Phase 2 (WDM transport) | 1× RTX 4070 | ~2 weeks | ~$5 |
| Phase 3 (S(q,ω)) | 1× RTX 4070 | ~1 week | ~$2 |
| Phase 4 (distributed sweep) | 2× GPU ({{ entity(name="nucleus") }}) | ~1 month | ~$10 |
| **Total** | | ~2 months | **~$19** |

Compare to institutional WDM allocation: ~100,000 GPU-hours at ~$1/GPU-hr
= $100,000. Even at 100× less accuracy, the cost ratio is 5,000:1.

---

## 9. The Broader Vision

This sub-thesis is not about competing with Frontier. It is about proving
that the mathematical workflow is correct, portable, and accessible. If
the physics runs correctly on one consumer GPU, it runs correctly on any
consumer GPU. If it runs on any consumer GPU, it runs on every idle GPU.
If it runs on every idle GPU, WDM simulation becomes a public utility
rather than an institutional privilege.

The shaders are the mathematics. The GPU is the substrate. The heat is
the byproduct. The science is the point.
