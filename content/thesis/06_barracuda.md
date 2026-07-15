+++
title = "Chapter 6: BarraCuda"
description = "Vendor-agnostic Pure Rust GPU compute (WGSL/Vulkan, f64) — NTT-to-FFT structural evolution as the principal constrained-evolution case study."
weight = 6
date = 2026-07-09

[extra]

[[extra.companions]]
url = "/technical/sovereign-gpu-pipeline-profile/"
title = "Sovereign GPU Pipeline Profile"
relation = "evidence_for"
label = "How toadStool/wgpu replaces the CUDA stack in practice"

[taxonomies]
trails = ["thesis-narrative"]

+++

{{ maturity(level="architectural") }}

## 6.1 Overview

BarraCuda is ToadStool's GPU compute library: Pure Rust, WGSL shaders, Vulkan backend, f64 precision on consumer hardware. It evolved under three selective pressures — machine learning, fully homomorphic encryption, and universal GPU portability — and proved fit for computational physics without being designed for it. The NTT→FFT structural evolution is the principal case study of constrained evolution in the codebase.

**Reference**: `gen3/data/BARRACUDA_SCIENTIFIC_COMPUTE_GAPS.md`, [Quantitative Evidence](@/thesis/13_quantitative_evidence.md) §13.2.

---

## 6.2 Technical Specification

### 6.2.1 Stack

| Layer | Technology |
|-------|------------|
| Language | Pure Rust |
| Shaders | WGSL (WebGPU Shading Language) |
| Backend | Vulkan (via wgpu) |
| Precision | f32 native; f64 via u64 emulation where needed |
| Vendor support | NVIDIA, AMD, Intel — no CUDA dependency |

### 6.2.2 Scale (Measured February 2026)

| Metric | Value | Method |
|--------|-------|--------|
| WGSL shader files | 628 | `find *.wgsl \| wc -l` |
| WGSL lines | 48,698 | `wc -l` on all .wgsl |
| Rust source files (barracuda crate) | 792 | `find *.rs \| wc -l` |
| Rust lines (ToadStool total) | 788,209 | `wc -l` on all .rs |
| #[test] annotations (ToadStool) | 13,503 | `grep -c '#\[test\]'` |
| Operations | 124+ implemented, 263 target | Kernel registry |

---

## 6.3 Key Kernels

### 6.3.1 Core Primitives

| Kernel Class | Examples | Domain |
|--------------|----------|--------|
| **GEMM** | matmul, matmul_fp64 | ML, lattice QCD, spectral methods |
| **FFT** | NTT, FFT (from NTT evolution) | FHE, PPPM, structure factors |
| **Yukawa force** | All-pairs f64 with PBC | Molecular dynamics (hotSpring Phase C) |
| **Velocity Verlet** | Split half-kick, drift, Berendsen | MD time integration |
| **Cell list** | Spatial hashing, neighbor lists | O(N log N) force computation |
| **Eigensolvers** | BatchedEighGpu, Jacobi | HFB, PCA, covariance |
| **SU(3)** | Pure gauge Wilson action | Lattice QCD |

### 6.3.2 Cross-Domain Reuse

neuralSpring's Isomorphism Theorem: all neural architectures decompose into 6 primitives (GEMM, Attention, Normalization, Nonlinearity, Reduction, Gating). BarraCuda implements all six. The same primitives serve hotSpring (plasma physics), wetSpring (biology), and neuralSpring (ML) without modification.

---

## 6.4 The NTT→FFT Evolution (Detailed Case Study)

### 6.4.1 Background

BarraCuda's Number Theoretic Transform (NTT) evolved for fully homomorphic encryption — polynomial multiplication in ℤ_q. The Fast Fourier Transform, needed for physics (lattice QCD, spectral methods, PPPM), shares the Cooley-Tukey butterfly structure.

### 6.4.2 Structural Comparison (Measured from Source)

Source files: `fhe_ntt.wgsl` (263 lines), `fft_1d.wgsl` (186 lines), `fft_1d_f64.wgsl` (197 lines).

| Component | NTT (`fhe_ntt.wgsl`) | FFT (`fft_1d.wgsl`) | Match |
|-----------|---------------------|---------------------|-------|
| Domain arithmetic library | 93 lines (U64 emulation) | 16 lines (complex mul/exp) | Different (domain-specific) |
| Buffer bindings | 4 bindings (u32 arrays) | 4 bindings (f32 arrays) | **Same structure** |
| Load from input | 4 lines | 4 lines | **Identical structure** |
| Load twiddle factor | 4 lines | 4 lines | **Identical structure** |
| Store to output | 4 lines | 5 lines | **Identical structure** |
| Modular arithmetic wrappers | 20 lines | 0 | Unique to NTT |
| Butterfly struct | 4 lines | 4 lines | **Identical structure** |
| Butterfly function | 5 lines: `u=(a+tb)%q, v=(a-tb)%q` | 5 lines: `u=a+tb, v=a-tb` | **Identical** (NTT adds mod) |
| `bit_reverse_index` | 8 lines | 8 lines | **100% identical** |
| Main compute kernel | 40 lines | 39 lines | **~97% identical** |
| `bit_reverse` kernel | 26 lines | 26 lines | **~97% identical** |

Shared structural core (load/store, butterfly, bit-reversal, indexing, dispatch): ~93 lines identical between NTT and FFT. NTT adds ~118 lines of U64 emulation and modular wrappers; FFT adds ~19 lines of complex arithmetic. The main compute kernel — stage indexing, stride computation, block decomposition, twiddle lookup — is character-for-character identical.

The FFT is *shorter* than NTT because complex floats (`vec2<f32>`) map to GPU hardware natively, while NTT requires U64 emulation from u32 pairs (WGSL lacks native u64). The f64 variant (`fft_1d_f64.wgsl`, 197 lines) uses a `Complex64` struct but retains the identical butterfly/indexing skeleton.

### 6.4.3 Interpretation

No one designed BarraCuda for physics. The FHE constraint required NTT; NTT required the Cooley-Tukey butterfly; the butterfly *is* the FFT's skeleton. **This is Taq polymerase in code**: the hot spring (FHE constraint) produced an enzyme (NTT butterfly) useful far beyond its original environment.

---

## 6.5 Vendor-Agnostic Democratization

### 6.5.1 The CUDA Lock-in Problem

Institutional scientific computing is dominated by NVIDIA/CUDA. AMD and Intel GPUs are second-class citizens. NVIDIA enforces this segmentation not only through software ecosystem lock-in but through deliberate f64 throttling: consumer GPUs (GeForce RTX) have CUDA f64 throughput artificially limited to 1:64 of f32, while compute-class GPUs (A100, H100) run f64 at 1:2 or 1:1. The silicon often has the same double-precision units; the throttle is in the CUDA driver.

### 6.5.2 The f64 Discovery

A Titan V (compute-class GPU) was procured on the assumption that consumer GPUs cannot perform double-precision science. While configuring the f64 pipeline for the Titan V via Vulkan — CUDA had already been eliminated by the Pure Rust directive — we discovered that **Vulkan's `SHADER_F64` extension exposes native f64 hardware on consumer GPUs at 1:2 throughput**.

The RTX 4070's double-precision hardware exists in the silicon. CUDA throttles it to 1:64 to protect the compute-class product line. Vulkan does not impose this throttle. WGSL shaders using the `f64` type compile to native hardware double-precision instructions via SPIR-V, bypassing CUDA's artificial limitation entirely.

This discovery was not the result of reverse engineering or exploitation. The `SHADER_F64` extension is a standard Vulkan feature, documented in the Vulkan specification (VkPhysicalDeviceShaderFloat64Features). It simply is not widely known in the scientific computing community because the dominant CUDA ecosystem has no incentive to advertise it.

### 6.5.3 Implication: Consumer GPUs as Science Hardware

The $600 RTX 4070 does real science at f64 precision:

- Yukawa MD: 0.000% energy drift over 80,000 steps (9/9 cases)
- Nuclear EOS: χ²/datum = 2.27 (surpassing the original paper's 6.62)
- Lattice QCD: plaquettes matching strong-coupling expansion, HMC acceptance 96–100%

The Pure Rust constraint eliminated CUDA and forced exploration of Vulkan. Vulkan revealed a capability that the conventional approach (CUDA) actively suppresses for commercial reasons. The constraint did not just find a workaround — it found something the community didn't know existed. This is the strongest single-point evidence for capability hunting as a methodology: the constraint forced the discovery.

### 6.5.4 Methodology: Capability Hunting

The f64 discovery exemplifies a broader pattern in BarraCuda's development. Rather than accepting vendor SDK boundaries (CUDA says consumer f64 is 1:64; therefore consumer GPUs can't do science), the approach is:

1. Analyze the need (f64 for MD, lattice QCD, nuclear structure)
2. Probe the hardware for actual capabilities (Vulkan device features, `SHADER_F64`)
3. Experiment until you understand what's really there (write the shader, measure)
4. Validate against published science (springs)

This is resource hunting in the ecological sense. Organisms don't build food — they find it by probing their environment and adapting to what's available. The `hotSpring/metalForge/` experiments extend this across substrate boundaries: GPU → NPU → CPU mixed pipelines, where each substrate is probed for its actual capabilities rather than its marketed capabilities.

AMD and Intel validation is pending dedicated hardware availability. The WGSL/Vulkan architecture requires only the `SHADER_F64` extension, which is supported on most discrete GPUs manufactured since 2020.

---

## 6.6 Cost Model

### 6.6.1 Paper-Parity Plasma Physics

| Run | Parameters | Time | Cost |
|-----|------------|------|------|
| Sarkas Yukawa OCP MD | 9 cases, N=10,000, 80k steps | 3.66 hours | **$0.044** |

Electricity at $0.12/kWh (Lansing Board of Water & Light). The same computation costs $50–500 on institutional HPC. Total hotSpring compute (18 papers, 195+ checks): ~$0.20.

### 6.6.2 Implications

If constrained evolution produces a system that reproduces published science at $0.01–$0.10 per paper on consumer hardware, the methodology has economic implications for scientific computing accessibility (Mesnard & Barba, 2017).

---

## 6.7 Gaps and Evolution Path

The BARRACUDA_SCIENTIFIC_COMPUTE_GAPS.md predicted ~60–70% ML/physics overlap. hotSpring Phase C measured **100%** for Yukawa MD: all needed math (exp_f64, sqrt_f64, pow_f64, sum_reduce, histc) existed from ML/FHE. Only new *compositions* (force kernels, Velocity Verlet, PBC) were needed.

Remaining gaps: complex FFT (from NTT), Bessel functions (TTM cylindrical coordinates), spherical harmonics (FMM replacement), high-quality PRNG (Monte Carlo). Many are shared with next-generation ML (e.g., spherical harmonics for SE(3)-equivariant networks).

---

## References

Diaw, A., Murillo, M. S., & Stanton, L. (2024). Learning transport properties of strongly coupled plasmas from neural surrogates. *Nature Machine Intelligence*.

Mesnard, O., & Barba, L. A. (2017). Reproducible and replicable computational fluid dynamics. *Computing in Science & Engineering*, 19(4), 44–55.

Murillo, M. S., & Weisheit, J. C. (1998). Dense plasmas, screened interactions, and atomic ionization. *Physics Reports*, 302, 1–65.

See `gen3/data/BARRACUDA_SCIENTIFIC_COMPUTE_GAPS.md`, [Quantitative Evidence](@/thesis/13_quantitative_evidence.md), `hotSpring/barracuda/EVOLUTION_READINESS.md`.

---

**See also:**

- [Quantitative Evidence](@/thesis/13_quantitative_evidence.md) — the NTT→FFT measurements
- [Results: hotSpring](@/thesis/08_results_hotspring.md) — BarraCuda validated against plasma physics
