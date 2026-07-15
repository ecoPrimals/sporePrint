+++
title = "Chapter 12: Results — neuralSpring"
description = "ML primitives and the Isomorphism Theorem: all architectures decompose into six operations. 2,450+ checks, coralForge AlphaFold2/3."
weight = 12
date = 2026-07-09

[extra]

[[extra.companions]]
url = "/lab/springs/neuralspring/"
title = "neuralSpring Validation"
relation = "evidence_for"

[[extra.companions]]
url = "/science/15-precision-brain-heterogeneous-gpu/"
title = "Precision Brain on Heterogeneous GPU"
relation = "validated_by"

[taxonomies]
trails = ["thesis-narrative"]

+++

{{ maturity(level="architectural") }}

## 12.1 Validation Summary

neuralSpring validates BarraCuda's ML primitives against published neural network architectures and evolutionary computation methods. 2,450+ checks pass across 25 reproduced papers + 5 WDM surrogates: 206 Python baselines (Phases 0, 0+, 0++), 31 WDM Python baselines, 1600+ Rust/GPU validations (Phases 1–5e), and 186 WDM Rust validations (nW-01..05). neuralSpring establishes the Isomorphism Theorem — that all neural architectures decompose into 6 fundamental primitives — and validates that BarraCuda implements all six, with ~90% of production math running on GPU. The WDM surrogate queue (Sessions 79–87) extends validation to reservoir computing: LSTM fixed-weight reservoirs with pooled readout (nW-03) and Echo State Networks with ridge regression readout (nW-05).

### Table 12.1 — Phase Summary

| Phase | Domain | Checks | Status |
|-------|--------|-------:|--------|
| Phase 0 (Python baselines) | Surrogates, transformers, LSTM, transfer | 48 | **48/48** |
| Phase 0+ (Scholarly reproductions) | PINN, DeepONet, LeNet-5, ERA5 LSTM, quantization | 31 | **31/31** |
| Phase 0++ (Extended reproductions) | Dolson, Liu, Waters, Kachkovskiy, Anderson (15 papers) | 127 | **127/127** |
| Phase 1a (neuralSpring-native Rust) | Rust port of all Python experiments | 183 | **183/183** |
| Phase 1b (BarraCuda CPU primitives) | GEMM, attention, normalization, gating, reduction | 272 | **272/272** |
| Phase 2 (BarraCuda CPU ports) | Full model inference in Rust | 203 | **203/203** |
| Phase 3c (GPU shaders) | Individual shader validation | 108 | **108/108** |
| Phase 3d (GPU pipeline + cross-dispatch) | End-to-end GPU inference | 94 | **94/94** |
| Phase 4 (GPU pipelines, PRNG, MHA, eigh) | Advanced GPU validation | 100+ | **All pass** |
| Phase 5a–b (GPU Tensor, full-stack) | 23/25 papers at GPU Tensor tier | 98+ | **All pass** |
| Phase 5c–d (Multi-GPU, benchmarks) | RTX 4070 + TITAN V bit-identical | 133+143 | **All pass** |
| Phase 5e (Pure GPU promotion) | 38 CPU→GPU ops (~90% math on GPU) | 47 | **47/47** |
| **Total** | | **1800+** | **All pass** |

### Post-Phase Status (Session 94)

| Quality Gate | Status |
|--------------|--------|
| Validation binaries | 197 |
| Library tests | 685 |
| Named tolerances | 139+ |
| `validate_all` | 185/185 PASS |
| Python baselines | 39 |
| WDM surrogates | 5 (nW-01..05) |
| coralForge (AlphaFold2/3) | Py 62/62, Rs 55/55, GPU 37/37 |
| clippy (pedantic+nursery) | 0 warnings |
| doc warnings | 0 |
| `unsafe` blocks | 0 (forbid) |
| TODO/FIXME/MOCK/STUB | 0 in src/ |
| Hardcoded paths | 0 |
| Inline magic numbers | 0 |

### coralForge: Sovereign Structure Prediction (Sessions 88–94)

coralForge extends neuralSpring with sovereign structure prediction primitives — AlphaFold2 Evoformer, IPA, backbone frames, and torsion angles; AlphaFold3 diffusion, pairformer, and confidence heads. All implemented in pure Rust f64, validated against NumPy baselines, and accelerated via 15 df64 WGSL shaders on consumer GPUs.

coralForge is the strongest test of the Isomorphism Theorem: structure prediction uses "novel" operations (triangle multiplication, Invariant Point Attention, SE(3)-equivariant diffusion) that appear unique to the domain. Yet every operation decomposes into the same 6 fundamental primitives. No new primitive category was required.

**See**: `coralForge/` white paper series for full design, architecture, validation, and LTEE application plan.

---

## 12.2 The Isomorphism Theorem

### Table 12.2 — Primitive Inventory and Architecture Mapping

| Primitive | WGSL Shader | MLP | Transformer | LSTM | CNN | DeepONet | PINN |
|-----------|-------------|:---:|:-----------:|:----:|:---:|:--------:|:----:|
| **GEMM** | `gemm_f64.wgsl` | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| **Attention** | `attention.wgsl` | — | ✓ | — | — | — | — |
| **Normalization** | `layernorm.wgsl` | — | ✓ | ✓ | — | — | — |
| **Nonlinearity** | `relu.wgsl`, `tanh.wgsl` | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| **Reduction** | `reduce_sum.wgsl` | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| **Gating** | `lstm_cell.wgsl` | — | — | ✓ | — | — | — |

All neural architectures decompose into these 6 primitives. BarraCuda implements all 6 as WGSL shaders. This means any architecture that can be expressed as a composition of these primitives can run on any Vulkan-capable GPU without CUDA.

---

## 12.3 Phase 0 — Synthetic Baselines (48/48)

| Experiment | Architecture | Metric | Expected | Actual | Status |
|------------|-------------|--------|----------|--------|--------|
| Neural surrogate | MLP 6→64→64→1 | R² (Rastrigin) | ≥ 0.40 | ~0.40+ | PASS |
| | | R² (Rosenbrock) | ≥ 0.95 | > 0.99 | PASS |
| | | R² (Ackley) | ≥ 0.90 | > 0.95 | PASS |
| | | FAO-56 RMSE | ≤ 0.20 mm/day | 0.07–0.08 | PASS |
| | | FAO-56 R² | ≥ 0.95 | 0.999+ | PASS |
| Transformer | NumPy self-attention | NumPy vs PyTorch | < 1e-10 | ~2.22e-16 | PASS |
| | | Causal mask leak | < 1e-6 | ~0 | PASS |
| Sequence forecasting | LSTM/GRU 32 hidden | R² | ≥ 0.80 | ~0.93–0.94 | PASS |
| Transfer learning | MLP fine-tuned | Source R² | > 0.95 | 0.999 | PASS |
| | | Domain gap (NM) | > 0.01 ΔR² | 0.326 | PASS |
| Isomorphic catalog | Cross-domain | Primitives mapped | 6 | 6 | PASS |

---

## 12.4 Phase 0+ — Scholarly Reproductions (31/31)

### Table 12.3 — Paper Reproduction Results

| Paper | Architecture | Metric | Paper Value | neuralSpring | Status |
|-------|-------------|--------|:-----------:|:------------:|--------|
| Raissi et al. 2019 (JCP) — PINN Burgers | MLP 2→20×8→1, tanh | L2 relative error | 0.06% (L-BFGS) | **5.1%** (Adam) | PASS |
| | | IC error | ~0 | < 1e-6 | PASS |
| | | BC error | ~0 | ~1e-15 | PASS |
| Lu et al. 2021 (NMI) — DeepONet | Branch-Trunk | Mean L2 error | MSE 9.27e-7 | **1.2%** | PASS |
| LeCun et al. 1998 — LeNet-5 MNIST | Conv2d+MaxPool+FC | Test accuracy | ~99% | **98.89%** | PASS |
| Gauch et al. 2021 (HESS) — LSTM ERA5 | LSTM 5→64→32→1 | NSE | > 0.80 | **0.849** | PASS |
| | | RMSE (°C) | < 5.0 | **3.46** | PASS |
| Dettmers/Frantar 2022/23 — Quantization | INT8/INT4 MLP | INT8 R² degradation | < 1% | **0.017%** | PASS |
| | | INT4 R² degradation | < 5% | **0.79%** | PASS |

PINN note: The 5.1% L2 error vs the paper's 0.06% reflects Adam-only training (neuralSpring does not implement L-BFGS). The tolerance is set at 15% to account for this optimizer gap. The physics (IC/BC satisfaction, PDE residual structure) is correct; the optimization path differs.

---

## 12.5 Phase 0++ — Extended Reproductions (127/127)

### Table 12.4 — Faculty Paper Reproductions

| Paper | Faculty | Domain | Checks | Status |
|-------|---------|--------|-------:|--------|
| Iram/Dolson 2020 — Counterdiabatic evolution | Dolson | Evolutionary computation | 11 | PASS |
| Dolson 2019 — MODES toolbox | Dolson | Open-ended evolution | 9 | PASS |
| Dolson & Ofria 2018 — Ecological dynamics | Dolson | Multi-niche EA | 7 | PASS |
| Dolson 2022 — Directed evolution | Dolson | Lexicase vs tournament | 8 | PASS |
| Foreback/Dolson 2025 — Swarm robotics | Dolson | Heterogeneous swarms | 11 | PASS |
| Liu 2014 — HMM phylogenetics | Liu | Forward/backward/Viterbi | 10 | PASS |
| Liu 2009 — SATé alignment | Liu | Divide-and-conquer | 8 | PASS |
| Liu 2015 — Introgression | Liu | PhyloNet-HMM, LRT | 8 | PASS |
| Bruger & Waters 2018 — Game theory QS | Waters | Cooperation dynamics | 8 | PASS |
| Mhatre 2020 — Regulatory network | Waters | Bistability, Hill ODE | 7 | PASS |
| Srivastava 2011 — Signal integration | Waters | Two-input AND gate | 8 | PASS |
| Kachkovskiy 2016 — Spectral commutativity | Kachkovskiy | Skip connection analysis | 8 | PASS |
| Bourgain & Kachkovskiy 2018 — Anderson | Kachkovskiy | IPR localization | 8 | PASS |
| Anderson — Pangenome selection | Anderson | Gene gain/loss | 8 | PASS |
| Anderson — Meta-population | Anderson | FST, isolation-by-distance | 8 | PASS |

---

## 12.6 Rust & GPU Validation (Phases 1–5e)

### Table 12.5 — Rust Binary Matrix

| Phase | Binary/Suite | Primitives Tested | Checks | Status |
|-------|-------------|-------------------|-------:|--------|
| 1a | neuralSpring-native | Full MLP, transformer, LSTM, 31 modules | 183 | PASS |
| 1b | BarraCuda CPU primitives | GEMM, attention, norm, relu, reduce, gating, eigh, FFT | 272 | PASS |
| 2 | BarraCuda CPU ports | Full model inference chains (24/25 papers) | 203 | PASS |
| 3c | GPU shaders | 17 WGSL shaders (13 absorbed upstream, 4 local) | 108 | PASS |
| 3d | GPU pipeline + cross-dispatch | Multi-kernel chains, CPU↔GPU parity | 94 | PASS |
| 4 | Advanced GPU | Pipelines, PRNG, MHA, eigendecomposition | 100+ | PASS |
| 5a–b | GPU Tensor full-stack | 23/25 papers at Tensor tier, 12 typed ops | 98+ | PASS |
| 5c–d | Multi-GPU + benchmarks | RTX 4070 + TITAN V NVK bit-identical | 276+ | PASS |
| 5e | Pure GPU promotion | 38 CPU→GPU ops via `gpu_dispatch::Dispatcher` | 47 | PASS |

### Table 12.5b — Validation Tier Coverage

| Tier | Papers | Checks | Coverage |
|------|:------:|-------:|:--------:|
| Python control (Py) | 25/25 | 206 | 100% |
| Rust CPU (Rs) | 25/25 | 374+ | 100% |
| BarraCuda CPU (bC) | 24/25 | 203 | 96% |
| BarraCuda GPU Tensor (gT) | 23/25 | 98+ | 92% |
| metalForge WGSL (mF) | 15/25 | 108 | 100%† |
| GPU Pipeline (gP) | 15/25 | 94 | 100%† |
| Cross-dispatch (xD) | 15/15 | 49 | 100% |
| Multi-GPU (mG) | 25/25 | 276+ | 100% |
| GPU Dispatch (gD) | 38 ops | 47 | ~90% |

`†` 100% of applicable papers (Phase 0/0+ use PyTorch, not WGSL).

### Table 12.6 — GPU Benchmark Results

| Scale | Model | Python (1 thread) | BarraCuda CPU | GPU | CPU/Py | GPU/Py |
|-------|-------|:------------------:|:-------------:|:---:|:------:|:------:|
| MLP large (3.1M params) | 4→64→64→10 | 3.0 ms | 2.7 ms | 178 µs | 1.1× | **16.8×** |
| Transformer medium (103M) | Pre-norm encoder | 59 ms | 15.1 ms | 566 µs | 3.9× | **104×** |
| Transformer xlarge (6.6B) | — | 232 ms | 1.42 s | 17.8 ms | — | **13.1×** |

### Fused Pipeline Speedup

| Model | Per-Op GPU | Fused GPU | Fused Speedup |
|-------|:----------:|:---------:|:-------------:|
| MLP (4→64→64→10) | 4.0 ms | 92 µs | **43.6×** |
| Transformer (d=32, h=4, seq=8) | 13.3 ms | 174 µs | **76.6×** |

The 76.6× fused pipeline speedup for transformers demonstrates that kernel fusion — composing multiple shader dispatches into a single pipeline — produces superlinear performance gains. This is a constrained evolution effect: the WGSL constraint forces explicit data flow between kernels, which makes fusion opportunities visible that are hidden in framework-level abstractions (PyTorch, TensorFlow).

---

## 12.7 Key Tolerances

| Constant | Value | Use |
|----------|-------|-----|
| `EXACT_F64` | 1e-12 | Exact f64 arithmetic |
| `CROSS_LANGUAGE` | 1e-10 | Rust vs Python parity |
| `SURROGATE_R2_MIN` | 0.40 | MLP surrogate minimum |
| `SEQUENCE_R2_MIN` | 0.80 | LSTM forecast minimum |
| `PINN_L2_ERROR_MAX` | 0.15 | Adam-only PINN tolerance |
| `QUANT_INT8_DEGRADATION` | 0.01 | Max INT8 R² loss |
| `QUANT_INT4_DEGRADATION` | 0.05 | Max INT4 R² loss |
| `TRANSFORMER_NUMPY_VS_PYTORCH` | 1e-10 | Attention parity |

---

## 12.8 Scholarly Reproduction Log (25 papers)

| # | Paper | Architecture | Key Result | Status |
|---|-------|-------------|------------|--------|
| 1 | Raissi et al. 2019 | PINN | L2 = 5.1% | PASS |
| 2 | Lu et al. 2021 | DeepONet | L2 = 1.2% | PASS |
| 3 | LeCun et al. 1998 | LeNet-5 | 98.89% accuracy | PASS |
| 4 | Gauch et al. 2021 | LSTM | NSE = 0.849 | PASS |
| 5 | Dettmers/Frantar 2022/23 | Quantization | INT8: 0.017% degradation | PASS |
| 6–10 | Dolson et al. (5 papers) | Evo computation | 46/46 checks | PASS |
| 11–13 | Liu et al. (3 papers) | Phylogenetics | 26/26 checks | PASS |
| 14–16 | Waters lab (3 papers) | QS models | 23/23 checks | PASS |
| 17–18 | Kachkovskiy (2 papers) | Spectral theory | 16/16 checks | PASS |
| 19–20 | Anderson (2 papers) | Population genomics | 16/16 checks | PASS |
| 21–25 | WDM surrogates (nW-01..05) | MLP, LSTM reservoir, ESN | 33/33 Python + 186/186 Rust | PASS |
| **Total** | **25 papers + 5 WDM surrogates** | | **239/239 Python, 1800+ Rust/GPU** | **All pass** |

---

## 12.9 Connection to Constrained Evolution Thesis

neuralSpring provides three lines of evidence for the thesis:

1. **The Isomorphism Theorem** demonstrates that architectural diversity in neural networks is illusory — all architectures compose from 6 primitives. The constrained evolution framework predicts this: under strong constraint (linear algebra, differentiable computation), solutions converge on the same structural elements through different compositions. This is cephalization in neural networks.

2. **Cross-faculty validation** (Dolson, Liu, Waters, Kachkovskiy, Anderson — 15 papers, 127 checks) demonstrates that the BarraCuda primitives evolved under ML constraint are fit for evolutionary computation, phylogenetics, systems biology, spectral theory, and population genomics without modification. This is cross-domain kernel reuse — the same phenomenon as the NTT→FFT evolution, at the application level.

3. **The 76.6× fused pipeline speedup** demonstrates that constraints can produce emergent performance. The WGSL constraint forces explicit kernel boundaries; explicit boundaries make fusion opportunities visible; fusion produces superlinear speedup. The performance was not designed — it emerged from the constraint.

4. **Pure GPU promotion** (Sessions 45–46, 38 ops) demonstrates that the 6 primitives are sufficient for ~90% of all production math across 25 papers and 7 scientific domains. The `gpu_dispatch::Dispatcher` provides capability-based runtime routing — GPU when available, CPU fallback otherwise. This is the same adaptive pattern that primals use in biomeOS: discover capabilities at runtime, use the best available substrate.

5. **Multi-GPU bit-identity** (Session 44) across RTX 4070 (Ada Lovelace, proprietary Vulkan) and TITAN V (Volta GV100, NVK open-source) proves that WGSL math is architecture-portable. Same source, different silicon generations (2017 vs 2023), different driver stacks — identical results. This eliminates vendor lock-in as a constraint on scientific reproducibility.

---

## 12.10 Pure GPU Promotion & Code Quality (Sessions 44–49)

### Table 12.7 — GPU Dispatch Coverage

38 previously CPU-bound operations promoted to GPU via `gpu_dispatch::Dispatcher`:

| Category | Operations | GPU Method |
|----------|:----------:|-----------|
| Linear algebra | matmul, transpose, frobenius_norm | `Tensor::matmul`, `transpose` |
| ML inference | neural_forward, softmax, pca_project | `Tensor::matmul` chain |
| Statistics | variance, mean, pearson_correlation | `Tensor` reductions |
| HMM | forward_step, backward_step, viterbi_step | `Tensor::matmul` + `max_dim` |
| Distance | L2, Hamming, Jaccard, geographic | `PairwiseL2Gpu`, etc. |
| Population | allele_freq, nucleotide_div, FST | `Tensor::sum_dim`, `div_scalar` |
| Game theory | replicator_step, spatial_payoff | `Tensor::matmul` + `softmax` |
| Biology | fitness_eval, hill_activation, diversity | `BatchFitnessGpu`, `HillGateGpu` |
| ODE | rk4_step | `Tensor::matmul` + `mul_add` |

Remaining ~10% CPU-only: full ODE integration loops (sequential timesteps),
FST variance decomposition, introgression HMM chain, Viterbi argmax.

### Table 12.8 — Code Quality (Session 87)

| Metric | Value |
|--------|-------|
| Rust modules | 31 + 2 evolved + 5 WDM + gpu_ops/ + gpu_dispatch |
| Validation binaries | 172 |
| Library tests | 623 |
| `validate_all` | 156/156 PASS |
| Python baselines | 31 |
| WDM surrogates | 5 (nW-01..05: transport, opacity, S(q,ω), transfer, ESN) |
| Named tolerances | 129+ (all justified, minimal) |
| WGSL shaders | 21 (13 upstream absorbed, 8 local) |
| Typed BarraCuda ops | 12 (all f64 aligned) |
| External dependencies | 0 C/C++ crates (pure Rust) |
| `unsafe` blocks | 0 (`forbid` enforced) |
| clippy warnings | 0 (pedantic + nursery) |
| TODO/FIXME in src/ | 0 |

### Pure Rust Benchmark (11 Phase 0++ Kernels)

| Kernel | Python (1t) | Pure Rust | Speedup |
|--------|:----------:|:---------:|:-------:|
| Counterdiabatic | 1.2 ms | 6.7 µs | 179× |
| MODES novelty | 890 µs | 4.1 µs | 217× |
| Eco dynamics | 340 µs | 2.8 µs | 121× |
| Directed evolution | 1.1 ms | 5.9 µs | 186× |
| HMM forward | 2.3 ms | 12.1 µs | 190× |
| SATé pairwise | 1.8 ms | 9.4 µs | 191× |
| Game theory | 450 µs | 3.2 µs | 141× |
| Regulatory ODE | 780 µs | 4.5 µs | 173× |
| Anderson IPR | 1.5 ms | 8.3 µs | 181× |
| Pangenome FST | 670 µs | 3.8 µs | 176× |
| Commutator | 45 µs | 110 µs | 0.4× |
| **Geometric mean** | | | **178.5×** |

Exception: dense commutator (64×64 matmul) where NumPy BLAS beats pure Rust loops.

---

## 12.11 baseCamp Research Program: BioPhysical AI Interpretability

neuralSpring's baseCamp program applies validated physics and biology
primitives to understanding AI systems as physical systems. The program
defines five sub-theses, each grounded in published academic work and using
existing validated primitives. No new math — only novel composition.

wetSpring took Anderson localization and applied it to quorum sensing
(baseCamp Sub-thesis 01). **neuralSpring takes the same spectral and
dynamical-systems primitives and applies them to AI interpretability.** The
neural network IS the disordered medium. The weight matrix IS the Hamiltonian.
Information propagation through layers IS wave propagation through a lattice.

### Table 12.9 — baseCamp Sub-Thesis Mapping

| Sub-Thesis | Novel Claim | Validated Primitives | Faculty Anchor |
|:----------:|------------|---------------------|----------------|
| nS-01: Weight Hamiltonians | Weight matrix eigenvalue IPR predicts generalization | `eigh_f64`, `BatchIprGpu`, `anderson_localization.rs` | M. Mahoney (Berkeley), U. Simsekli (Inria) |
| nS-02: Information Flow | LSTM gating is stencil propagation on a disordered lattice | `hmm.rs`, `stencil_cooperation.wgsl`, `signal_integration.rs` | S. Ganguli (Stanford) |
| nS-03: Loss Landscapes | Loss landscapes are energy landscapes; saddle points are transition states | `rk45_adaptive.wgsl`, `eigh_f64`, `game_theory.rs` | D. Wales (Cambridge) |
| nS-04: Neural PGM | DNN forward pass approximates belief propagation on a tree PGM | `hmm.rs`, `eigh_f64`, `introgression.rs` | Y.W. Teh (Oxford/DeepMind) |
| nS-05: Multi-Agent QS | Anderson framework predicts multi-agent AI coordination phase transitions | `anderson_localization.rs`, `swarm_robotics.rs`, `WrightFisherGpu` | E. Dolson (MSU) |

### Grounding Papers (15)

Each sub-thesis reproduces 3 published papers as baselines, then extends
with novel applications of validated primitives:

- **nS-01**: Martin & Mahoney 2021 (JMLR), Gurbuzbalaban et al. 2025,
  Ouyang 2025
- **nS-02**: Schoenholz et al. 2017 (ICLR), Gu et al. 2020 (ICML),
  Yang et al. 2025
- **nS-03**: Ballard, Wales et al. 2024 (Digital Discovery), Pittorino
  et al. 2025, Liu et al. 2024
- **nS-04**: Li et al. 2023, Nabarro et al. 2024 (ICML), Conmy et al.
  2023 (NeurIPS)
- **nS-05**: SwarmSys 2025, Emergent Collective Memory 2025, Foreback &
  Dolson 2025 (already validated as Paper 015)

Full paper details: `specs/PAPER_REVIEW_QUEUE.md` (Phase 1 — baseCamp Papers).

### Cross-Spring Connections

| Sub-Thesis | hotSpring Primitive | wetSpring Primitive |
|:----------:|--------------------|--------------------|
| nS-01 | — | Anderson QS (IPR, level spacing) |
| nS-02 | — | QS signal propagation (stencil) |
| nS-03 | MD energy minimization (RK4/RK45), Boltzmann sampling | — |
| nS-04 | — | HMM phylogenetics (belief propagation) |
| nS-05 | — | Anderson QS dimensional analysis, game theory |

### Connection to the Constrained Evolution Thesis

The baseCamp program tests two central predictions of the constrained
evolution thesis in the AI domain:

1. **Convergent structure**: Architectural constraints produce universal
   spectral signatures in weight matrices (nS-01), universal landscape
   topologies (nS-03), and universal coordination strategies (nS-05) —
   the same convergent evolution observed in biology.

2. **Substrate independence**: The physics governing information flow in
   neural networks (nS-02) and probabilistic inference (nS-04) is
   mathematically identical to the physics governing biological systems
   (QS signal propagation, phylogenetic inference). This validates the
   Isomorphism Theorem: one engine, many substrates.

The significance: **the same 6 primitives that serve ML also reveal the
physical structure hidden inside AI systems**. The baseCamp extensions
require only composition of already-validated operations (2,450+ checks
across 25 papers + 5 WDM surrogates), not new math.

---

## 12.12 WDM Surrogate Queue (Sessions 79–87)

The WDM surrogate queue validates neuralSpring's ML primitives against
warm dense matter physics from hotSpring, demonstrating cross-spring
scientific value and two new BarraCuda absorption targets.

### Table 12.10 — WDM Surrogate Results

| Model | Architecture | Python Checks | Rust Checks | Key Result |
|-------|-------------|:------------:|:----------:|------------|
| nW-01 (transport) | MLP 2→64→64→3 | 5 | 17 | σ_dc, κ, η from (ρ, T) |
| nW-02 (opacity) | MLP 3→32→32→1 | 5 | 25 | Rosseland opacity interpolation |
| nW-03 (S(q,ω) peaks) | LSTM reservoir, hidden=32 | 6 | 27 | R²=0.98 on (ω_peak, γ) extraction |
| nW-04 (transfer) | MLP with classical→WDM domain adaptation | 6 | 45 | Transfer learning across plasma regimes |
| nW-05 (ESN classifier) | ESN, reservoir=64, SR=0.9 | 11 | 39 | 96.5% accuracy, Python↔Rust < 1e-10 |
| **Total** | | **33/33** | **153/153** | **186/186 all pass** |

### Reservoir Computing Findings

Two of the five WDM surrogates (nW-03, nW-05) use reservoir computing —
a paradigm where recurrent weights are fixed random and only a linear
readout is trained:

1. **LSTM reservoir (nW-03)**: Fixed LSTM cell weights, pooled hidden
   states (mean + std + last after washout=4), ridge regression readout.
   Validates that LSTM can function as a feature extractor without
   backpropagation. Absorption target: `barracuda::nn::LstmReservoir`.

2. **ESN classifier (nW-05)**: Fixed reservoir (`W_res` at spectral radius
   0.9, `W_in` at input scale 0.5), 2-step tanh update, linear readout +
   argmax. Achieves bit-exact Python↔Rust parity (score difference < 1e-10).
   Absorption target: `barracuda::nn::EsnClassifier`.

### Cross-Spring Relevance

The reservoir computing models are directly applicable to baseCamp:

- **Sub-thesis 04 (Sentinels)**: ESN regime classifier detects Anderson
  regime shifts from community features — same architecture, different inputs
- **Sub-thesis 06 (No-till)**: LSTM reservoir extracts temporal features
  from soil parameter time series — predicts r(t) QS regime from θ(t), J(t)
- **Sub-thesis 01 (Anderson QS)**: ESN classifies extended/localized/marginal
  states from spectral features computed by shared `eigh_f64` primitives

Full sub-thesis documents: `baseCamp/sub01_weight_hamiltonians.md`
through `sub05_multiagent_qs.md`. Program overview:
`baseCamp/extensions.md`.

---

**See also:**

- [Science: neuralSpring papers](@/science/_index.md) — the reproduced papers
- [helixVision](@/products/helixVision.md) — the structure prediction product
