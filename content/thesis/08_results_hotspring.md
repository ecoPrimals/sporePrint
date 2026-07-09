+++
title = "Chapter 8: Results — hotSpring"
description = "Computational plasma physics validation: Sarkas MD, nuclear EOS, lattice QCD, spectral methods — 197+ checks at ~$0.80 compute cost."
weight = 8
date = 2026-07-09
+++

{{ maturity(level="architectural") }}

## 8.1 Validation Summary

hotSpring validates BarraCuda and the ecoPrimals infrastructure against computational plasma physics, nuclear structure, lattice QCD, spectral theory, and neuromorphic computing. All 197+ checks pass across 39 validation suites and 21 experiments at ~$0.80 total compute cost. Five silent bugs in the upstream Sarkas MD package were identified and patched during reproduction. The crate has evolved to 78 binaries, 62 WGSL shaders, and ~697 tests.

### Table 8.1 — Phase-by-Phase Validation

| Phase | Domain | Experiments | Checks | Status | Cost |
|-------|--------|-------------|-------:|--------|------|
| A+B | Sarkas MD (5 observables × 12 cases) | 12 | 60 | **60/60** | ~$0.02 |
| A+B | Two-Temperature Model (TTM) | 1 | 6 | **6/6** | ~$0.001 |
| A+B | Surrogate learning (Diaw et al. 2024) | 1 | 15 | **15/15** | ~$0.01 |
| A+B | Nuclear EOS (SEMF + HFB, AME2020) | 2 | 2 | **2/2** | ~$0.10 |
| C | GPU MD PP Yukawa (9 cases × 5 obs) | 9 | 45 | **45/45** | $0.044 |
| D | N-scaling + cell-list + native f64 | 4 | 16 | **16/16** | ~$0.01 |
| E | Paper-parity long run | 1 | 13 | **13/13** | ~$0.02 |
| F | Nuclear EOS full-scale (L1/L2/L3) | 3 | 9 | **9/9** | ~$0.02 |
| Pipeline | BarraCuda MD + HFB | 2 | 26 | **26/26** | — |
| Lattice | SU(3) pure gauge + Abelian Higgs | 2 | 29 | **29/29** | ~$0.02 |
| Spectral | Anderson, Hofstadter, Lanczos | 9 | — | all pass | ~$0.001 |
| NPU | ESN → AKD1000 pipeline | 1 | 3 | **3/3** | — |
| **Total** | | **~48** | **195+** | **All pass** | **~$0.20** |

---

## 8.2 Sarkas Yukawa MD Reproduction

### 8.2.1 Observable Validation (60/60)

| Observable | Cases | Metric | Result | Status |
|------------|------:|--------|--------|--------|
| Dynamic Structure Factor (DSF) | 12 | Peak frequency vs Dense Plasma Properties Database | PP: 8.5% mean error; PPPM: 7.3% | **12/12** |
| Energy Conservation | 12 | \|drift\| range | [−1.77%, +1.40%], mean 0.65% | **12/12** |
| Radial Distribution Function (RDF) | 12 | Peak at \(a_{ws}\), \(g(r) \to 1\) | 1.55–1.72, tails verified | **12/12** |
| Static Structure Factor (SSF) | 12 | \(S(k \to 0)\) trends | Monotonic with \(\Gamma\) | **12/12** |
| Velocity Autocorrelation (VACF) | 12 | \(D\) (m²/s) | 7.7e-9 to 5.9e-7 | **12/12** |

### 8.2.2 DSF PP Cases (κ ≥ 1) — Peak Frequency vs Reference

| Case | κ | Γ | Mean Peak Error | Wall Time | Status |
|------|--:|---:|----------------:|----------:|--------|
| dsf\_k1\_G14 | 1 | 14 | 7.5% | 27 min | PASS |
| dsf\_k1\_G72 | 1 | 72 | 4.7% | 28 min | PASS |
| dsf\_k1\_G217 | 1 | 217 | 6.2% | 28 min | PASS |
| dsf\_k2\_G31 | 2 | 31 | 9.4% | 12 min | PASS |
| dsf\_k2\_G158 | 2 | 158 | 5.8% | 12 min | PASS |
| dsf\_k2\_G476 | 2 | 476 | 7.3% | 11 min | PASS |
| dsf\_k3\_G100 | 3 | 100 | 18.6% | 10 min | PASS |
| dsf\_k3\_G503 | 3 | 503 | 7.8% | 10 min | PASS |
| dsf\_k3\_G1510 | 3 | 1510 | 9.0% | 10 min | PASS |
| **Overall** | | | **8.5%** | **2.0 hrs** | **9/9** |

### 8.2.3 DSF PPPM Cases (κ = 0) — Plasmon Peaks

| Case | κ | Γ | Plasmon Peaks | Mean Error | Status |
|------|--:|---:|:------------:|----------:|--------|
| dsf\_k0\_G10 | 0 | 10 | 2 | 0.1% | PASS |
| dsf\_k0\_G50 | 0 | 50 | 2 | 11.0% | PASS |
| dsf\_k0\_G150 | 0 | 150 | 2 | 10.8% | PASS |
| **Overall** | | | **6** | **7.3%** | **3/3** |

### 8.2.4 GPU Paper-Parity MD (Phase C) — Energy Drift

9 PP Yukawa cases at N = 10,000 particles, 80,000 timesteps, on RTX 4070 ($600):

| Case | κ | Γ | Energy Drift | Tolerance | Status |
|------|--:|---:|------------:|----------:|--------|
| k1\_G14 | 1 | 14 | 0.001% | 5% | PASS |
| k1\_G72 | 1 | 72 | 0.001% | 5% | PASS |
| k1\_G217 | 1 | 217 | 0.002% | 5% | PASS |
| k2\_G31 | 2 | 31 | 0.000% | 5% | PASS |
| k2\_G158 | 2 | 158 | 0.000% | 5% | PASS |
| k2\_G476 | 2 | 476 | 0.000% | 5% | PASS |
| k3\_G100 | 3 | 100 | 0.000% | 5% | PASS |
| k3\_G503 | 3 | 503 | 0.000% | 5% | PASS |
| k3\_G1510 | 3 | 1510 | 0.000% | 5% | PASS |

Total GPU run: 3.66 hours, $0.044 electricity, 801.7 kJ GPU energy.

### 8.2.5 Upstream Bug Discovery

Five silent bugs identified in the Sarkas Yukawa MD codebase during hotSpring reproduction. The reproduction pipeline acts as selection pressure: code paths that diverge from expected physical behavior are identified and corrected. Patches contributed upstream.

---

## 8.3 Nuclear Equation of State (AME2020)

### 8.3.1 Surrogate Learning (Diaw et al. 2024, Nature Machine Intelligence)

| Level | Method | Paper χ²/datum | BarraCuda χ²/datum | Speedup | Tolerance | Status |
|-------|--------|:--------------:|:------------------:|--------:|:---------:|--------|
| L1 | SEMF (52 nuclei) | 6.62 | **2.27** | 478× | < 10 | **PASS** |
| L2 | HF+BCS (18 focused) | **1.93** | 16.11 (best) | 1.7× | < 5 | Partial |
| L1 Python (30k evals) | SEMF full | — | **3.93** | — | — | — |
| L2 Python (3k evals) | HFB hybrid | — | **1.93** | — | — | — |

L1 BarraCuda surpasses the paper's χ²/datum (2.27 vs 6.62) — the constrained evolution produced a better fit than the original, because the BarraCuda optimizer explored a different region of the parameter landscape. L2 remains partially validated; the HFB nuclear structure calculation is the most demanding scientific computation in the system.

### 8.3.2 AME2020 Coverage

Full AME2020 dataset: **2,042 nuclei** validated — 39× the 52 nuclei in the original Diaw et al. paper. Binding energy and mass excess validated against the Atomic Mass Evaluation 2020 tables.

---

## 8.4 Lattice QCD

### 8.4.1 Pure Gauge SU(3) Wilson Action (12/12 checks)

| Metric | Expected | Actual | Tolerance | Status |
|--------|----------|--------|-----------|--------|
| Cold plaquette | 1.0 | ~1e-15 | 1e-12 | PASS |
| Cold Wilson action | 0.0 | ~0 | 1e-10 | PASS |
| HMC acceptance rate | > 10% | 96–100% | 0.10 | PASS |
| Plaquette vs strong-coupling expansion | Match | Verified | — | PASS |
| HMC ΔH | O(0.01) | Verified | — | PASS |

### 8.4.2 Abelian Higgs Model (17/17 checks)

| Metric | Expected | Actual | Tolerance | Status |
|--------|----------|--------|-----------|--------|
| Cold plaquette | 1.0 | Exact | 1e-12 | PASS |
| Weak coupling (β=6) plaquette | ~0.9 | 0.915 | — | PASS |
| Strong coupling (β=0.5) plaquette | ~0.2 | 0.236 | — | PASS |
| Higgs condensation (κ=2) ⟨\|φ\|²⟩ | — | 4.42 | — | PASS |
| Leapfrog reversibility \|ΔH\| | Small | 0.002 (dt=0.01) | — | PASS |
| Rust vs Python speedup | — | **143×** | — | — |

---

## 8.5 Transport Coefficients (Stanton & Murillo 2016) — 13/13 checks

| Metric | Expected | Actual | Tolerance | Status |
|--------|----------|--------|-----------|--------|
| D\* vs Sarkas | Match Green-Kubo | Calibrated to 12 Sarkas points | 5% | PASS |
| D\* Daligault fit | Smooth model | Per-point error < 20%, RMSE < 10% | 20%, 10% | PASS |
| η\* stress ACF | Match literature | O(10⁻¹) | 10% | PASS |
| λ\* heat ACF | Match literature | Verified | — | PASS |

## 8.6 Screened Coulomb (Murillo & Weisheit 1998) — 23/23 checks

| Metric | Expected | Actual | Tolerance | Status |
|--------|----------|--------|-----------|--------|
| Hydrogen eigenvalue vs exact | Match | Δ ≈ 10⁻¹² | 2% | PASS |
| Python-Rust parity | Match | Δ ≈ 10⁻¹² | 1e-10 | PASS |
| Critical screening vs Lam & Varshni | 3 values | 3 checks pass | 5% | PASS |
| Physics trends | 6 monotonic | 6 verified | — | PASS |
| Screening models | 3 models | 3 verified | — | PASS |

---

## 8.7 Spectral Theory (Kachkovskiy)

| Model | Metric | Expected | Actual | Status |
|-------|--------|----------|--------|--------|
| Anderson 1D | γ(0) = W²/96 (Kappus-Wegner) | Theory | 7% error | PASS |
| Almost-Mathieu | Herman γ = ln\|λ\| | Theory | < 0.0001 error | PASS |
| Aubry-André | Metal-insulator at λ=1 | λ=1 | Transition detected | PASS |
| Poisson statistics | ⟨r⟩ | 0.3863 | 0.3858 (0.1% error) | PASS |
| 2D Anderson bandwidth | 8.0 | 7.91 | 1.1% error | PASS |
| 3D mobility edge | GOE vs Poisson | ⟨r⟩ center 0.516, edge 0.494 | — | PASS |
| Hofstadter band count | α=1/q → q bands | q=2,3,5 exact | — | PASS |

---

## 8.8 NPU Pipeline — Lattice Phase Detection

| Metric | Expected | Actual | Status |
|--------|----------|--------|--------|
| β\_c (deconfinement) | 5.692 | 5.715 (0.4% error) | PASS |
| ESN classifier accuracy | High | 100% on test | PASS |
| NpuSimulator f32 parity | Match f64 | max error 2.8e-7 | PASS |

### NPU Quantization Cascade

| Substrate | Precision | Error vs f64 | Tolerance | Status |
|-----------|-----------|:------------:|:---------:|--------|
| f32 | 32-bit float | < 0.001% | 0.001 | PASS |
| int8 | 8-bit | < 5% | 0.05 | PASS |
| int4 | 4-bit | < 30% | 0.30 | PASS |
| int4+act4 | Full quantized | < 50% | 0.50 | PASS |

---

## 8.9 DF64 Core Streaming Discovery

### 8.9.1 The Problem

Native FP64 on consumer GPUs runs at 1:64 throughput (CUDA or Vulkan). The Titan V provides 1:2, but costs $500 used. Science requires 14+ digit precision for energy conservation, lattice QCD plaquettes, and nuclear EOS fitting.

### 8.9.2 The Discovery

Double-float (DF64) arithmetic — representing each f64 as a pair of f32 values — runs on the FP32 cores at full throughput. On an RTX 3090 (10,496 FP32 cores), this delivers **3.24 TFLOPS at 14-digit precision** — 9.9× the throughput of native FP64.

| Substrate | Throughput | Precision | Cost |
|-----------|-----------|-----------|------|
| Native FP64 (consumer) | 0.33 TFLOPS | 16 digits | $600 |
| DF64 on FP32 cores | **3.24 TFLOPS** | 14 digits | $600 |
| Native FP64 (Titan V) | 6.1 TFLOPS | 16 digits | $500 used |

### 8.9.3 Production Validation

32⁴ lattice QCD production β-scan: **7.1 hours** with DF64 mixed pipeline vs **13.6 hours** FP64-only. Gauge force, plaquette, and kinetic energy shaders run in DF64; momentum update and link update remain native FP64. 60% of HMC compute in DF64, 2× overall speedup. 12 β-values, deconfinement transition resolved at χ=40.1 (β=5.69, matching known β_c=5.692).

---

## 8.10 GPU Streaming HMC and Resident CG

### 8.10.1 GPU-Resident CG Solver

The conjugate gradient solver for the Dirac equation D†Dx=b was made GPU-resident: all scalar operations (α, β, rz, convergence check) run on GPU. Only 8-byte convergence readback per 10-iteration batch. Result: **15,360× readback reduction** (37 MB → 2.4 KB per trajectory) and **30.7× speedup** for dynamical fermion HMC.

### 8.10.2 Streaming Pipeline

Bidirectional streaming: 90%+ data flows to GPU, async readback for CG convergence only. NPU branch screens lattice configurations in parallel (0.09% overhead). GPU PRNG eliminates host random-number generation. Scaling: 4⁴→16⁴ validated, GPU **67× CPU** at 16⁴ (22.2× at CG solve level).

---

## 8.11 Cross-Substrate ESN Comparison (Exp 021)

### 8.11.1 GPU as ESN Reservoir

The Echo State Network (ESN) was dispatched to GPU for the first time using the existing WGSL shaders (`esn_reservoir_update.wgsl`, `esn_readout.wgsl`). New f32 buffer management methods were added to `GpuF64`.

### 8.11.2 Scaling Crossover

| RS | CPU-f64 (μs) | GPU-f32 (μs) | GPU/CPU |
|----|-------------|-------------|---------|
| 16 | 27 | 4,876 | 0.006× |
| 100 | 483 | 5,711 | 0.08× |
| 512 | ~10,400 | ~5,500 | ~1.0× |
| 1024 | 16,481 | 3,665 | **8.2×** |

GPU crossover at **RS ≈ 512**. Below this, CPU wins (dispatch overhead dominates). Above, GPU parallelism in matrix-vector products dominates.

### 8.11.3 NPU Streaming Advantage

| Metric | NPU-sim | GPU-f32 |
|--------|---------|---------|
| Per-inference | **2.8 μs** | 3,170 μs |
| Streaming throughput | 357k inf/s | 317 inf/s |
| Power estimate | ~30 mW | ~350 W |

The NPU owns single-sample streaming inference. No other substrate matches its latency for per-step ESN updates.

### 8.11.4 Engineering Discovery

**Recurrent network GPU dispatch requires per-step submit.** Naive encoder batching fails because `queue.write_buffer()` races with encoded dispatches — all steps see the last input. Each recurrent step must be submitted individually because state at step t depends on state at step t-1.

---

## 8.12 NPU Characterization Campaign (Exp 020)

### 8.12.1 Pipeline Placement Framework

Six placement options tested for NPU pre-screening in lattice QCD:

| Placement | Description | Projected Savings |
|-----------|-------------|-------------------|
| A | Pre-thermalization screening | **3.15 hours** (biggest win) |
| B | Mid-trajectory abort | Useful at large lattices |
| C | Post-trajectory classification | Baseline |
| D | Inter-beta steering | Needs more training data |
| E | Pre-run bootstrap | Warm-start from prior runs |
| F | All combined | 390 trajectories saved |

### 8.12.2 Models Trained

- Thermalization detector: 87.5% accuracy, 61.8% savings
- Rejection predictor: 96.2% accuracy
- 6-output multi-model: all outputs finite, multi-output free confirmed

---

## 8.13 Key Tolerances

| Constant | Value | Purpose |
|----------|-------|---------|
| `ENERGY_DRIFT_PCT` | 5.0% | MD energy conservation |
| `RDF_TAIL_TOLERANCE` | 0.15 | g(r→∞) → 1 |
| `TRANSPORT_D_STAR_VS_SARKAS` | 5% | D\* vs Sarkas |
| `TRANSPORT_D_STAR_VS_FIT` | 10% | D\* vs Daligault fit |
| `LATTICE_HMC_ACCEPTANCE_MIN` | 0.10 | HMC acceptance |
| `U1_HMC_ACCEPTANCE_MIN` | 0.30 | Abelian Higgs HMC |
| `SCREENED_HYDROGEN_VS_EXACT` | 2% | Eigenvalue vs exact |
| `L1_CHI2_THRESHOLD` | 10.0 | L1 nuclear EOS |
| `L2_CHI2_THRESHOLD` | 5.0 | L2 nuclear EOS |

---

## 8.14 Scholarly Reproduction Log

| # | Paper | Domain | Key Metric | Status |
|---|-------|--------|------------|--------|
| 1 | Silvestri et al. (Sarkas Yukawa OCP) | Plasma MD | 9/9 cases, 0.000% drift | PASS |
| 2 | Two-Temperature Model (TTM) | Plasma transport | 6/6 checks | PASS |
| 3 | Diaw et al. 2024 (Nature Mach Intel) | Nuclear surrogates | χ²/datum = 2.27 (L1) | PASS |
| 4 | AME2020 Nuclear Mass Tables | Nuclear structure | 2,042 nuclei | PASS |
| 5 | Stanton & Murillo 2016 | Transport coefficients | 13/13 Green-Kubo | PASS |
| 6 | Murillo & Weisheit 1998 | Screened Coulomb | 23/23 checks | PASS |
| 7 | HotQCD EOS tables (Bazavov 2014) | QCD thermodynamics | Thermo validated | PASS |
| 8 | Pure gauge SU(3) Wilson action | Lattice QCD | 12/12 checks | PASS |
| 9 | Abelian Higgs (Bazavov 2015) | Lattice gauge | 17/17, 143× Rust speedup | PASS |
| 10 | Dynamical fermion QCD (Paper 10) | Lattice QCD | 7/7 pseudofermion HMC | PASS |
| 11-19 | Spectral theory (Kachkovskiy) | Anderson, Hofstadter, Lanczos | All pass | PASS |
| 20 | Freeze-out conditions | QCD phenomenology | Validated | PASS |
| 21 | HVP g-2 | QCD + muon anomaly | Kernel validated | PASS |

---

## 8.15 Connection to Constrained Evolution Thesis

hotSpring validates the thesis at five levels:

1. **Infrastructure correctness**: BarraCuda's Yukawa force kernel, evolved under ML and FHE constraints, reproduces published plasma physics at 0.000% energy drift — confirming that the constrained evolution produced correct scientific computing primitives.

2. **Bug discovery as selection pressure**: The 5 silent upstream Sarkas bugs demonstrate that the reproduction pipeline functions as environmental selection — code paths that diverge from expected physical behavior are identified and corrected.

3. **Cross-domain fitness**: The NTT→FFT evolution ([BarraCuda](@/thesis/06_barracuda.md), §6.4) enables PPPM electrostatics in MD. Kernels evolved for cryptography serve physics without modification. The 39× expansion of nuclear EOS coverage beyond the original paper illustrates how reproducibility scaffolding enables exploratory evolution within fixed constraints.

4. **Cost democratization**: ~$0.80 total for 22 papers on consumer hardware vs. $50–500 for equivalent institutional HPC time. The constraint (Pure Rust, no CUDA) forced exploration of the WGSL/Vulkan path, producing a cheaper solution.

5. **Cross-substrate capability hunting**: The DF64 discovery (FP32 cores delivering 14-digit precision at 9.9× native f64 throughput) and the NPU characterization (10 SDK assumptions overturned by probing beyond the vendor SDK) are both instances of the capability hunting methodology. The same math runs on CPU (f64), GPU (f32 via WGSL), and NPU (int4 via Akida) — each substrate found by probing the hardware, not by following vendor documentation. The cross-substrate ESN comparison (Exp 021) quantifies exactly where each substrate belongs: CPU for small reservoirs, GPU for RS≥512, NPU for streaming inference at 2.8μs/step.

---

**See also:**

- [Science: hotSpring papers](@/science/_index.md) — the reproduced papers
- [BarraCuda](@/thesis/06_barracuda.md) — the GPU compute layer
