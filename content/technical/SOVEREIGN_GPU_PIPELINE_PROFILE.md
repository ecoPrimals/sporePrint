+++
title = "Cross-Vendor f64 Scientific GPU Computing in Rust and WGSL — No CUDA Required"
description = "Pure Rust scientific GPU compute via WebGPU/WGSL: f64 precision on NVIDIA, AMD, and Intel GPUs without CUDA. {{ total_stat(stat='wgsl_files') }} validated shaders across 10 scientific domains."
date = 2026-03-17

[taxonomies]
primals = ["barracuda", "coralreef", "toadstool"]
springs = ["hotspring", "wetspring"]

[extra]

[[extra.companions]]
url = "/thesis/06-barracuda/"
title = "Chapter 6: BarraCuda"
relation = "evidence_for"

[[extra.companions]]
url = "/science/14-sovereign-compute-hardware/"
title = "Sovereign Compute Hardware"
relation = "pairs_with"
+++

**Cross-vendor f64 scientific GPU computing without CUDA.** This page documents
a pure Rust GPU compute pipeline using WebGPU/WGSL that runs on NVIDIA, AMD,
and Intel GPUs — no vendor SDK, no proprietary dependencies, no cloud.

**Last Updated:** July 31, 2026  
**License:** CC-BY-SA 4.0

> **✓ VALIDATED ON LIVE HARDWARE** — strandGate RTX 3090: 2,130 matmul/sec, 746 pipelines/sec, 98 capabilities LIVE. Dual-vendor proof: RTX 3090 + RX 6950 XT, 100% pass rate. {{ total_stat(stat="wgsl_files") }} validated WGSL shaders. Current numbers: [Evidence Snapshot](@/architecture/EVIDENCE_SNAPSHOT.md) (measured {{ total_stat(stat="measured_date") }}).

---

## The Claim

Four public primals — BarraCuda (math), {{ entity(name="toadstool") }} (orchestration), {{ entity(name="coralreef") }}
(compiler), and coral-glowplug (hardware lifecycle) — together replace the
NVIDIA CUDA toolchain for scientific computing. Not all of it yet. But a
clear, advancing front that already produces paper-parity lattice QCD on a
$500 consumer GPU.

This is not a research prototype. These are production-grade primals with
{{ total_stat(stat="total_tests_display") }} combined test functions across {{ total_stat(stat="total_loc_display") }} lines of Rust, zero unsafe code, and zero C dependencies in
application code.

---

## What's Already Replaced

### 1. CUDA Runtime → toadStool + wgpu/Vulkan

| CUDA Component | Replacement | Status | Tests |
|----------------|-------------|--------|:-----:|
| `cudaGetDeviceProperties()` | toadStool hardware discovery (multi-adapter, multi-vendor) | **Production** | 21,156 |
| `cudaSetDevice()` | Capability-based selection (f64 probe, VRAM, workgroup limits) | **Production** | — |
| `cudaMalloc/cudaFree` | wgpu buffer management (BarraCuda `GuardedDeviceHandle`) | **Production** | — |
| `cudaMemcpy` | wgpu buffer map/unmap with staging | **Production** | — |
| `cudaLaunchKernel` | `queue.submit()` with WGSL compute shaders | **Production** | — |
| `cudaDeviceSynchronize()` | `device.poll(Maintain::Wait)` | **Production** | — |
| Device enumeration | `toadstool-sysmon` (pure Rust `/proc`, no `sysinfo` crate) | **Production** | — |

**Key difference:** toadStool discovers hardware at runtime by capability,
not by vendor ID. The same code discovers NVIDIA, AMD, Intel, and BrainChip
NPU. There is no concept of "CUDA device 0" — there is "the device that
supports f64 and has >8GB VRAM."

### 2. cuBLAS / cuFFT / cuDNN → BarraCuda WGSL Shaders

| Library | BarraCuda Equivalent | Shaders | Parity | Gap |
|---------|---------------------|:-------:|--------|-----|
| cuBLAS (GEMM) | `GemmF64`, `BatchedGemmF64`, DF64 GEMM | 40+ | Near parity (3.7× Kokkos gap, down from 27×) | Throughput on large matrices |
| cuFFT | 1D/2D/3D FFT, NTT, INTT | 20+ | Full parity for science ops | No cuFFT callback equiv |
| cuDNN (basic) | Conv2D, pooling, attention, softmax, LayerNorm | 30+ | Partial — science ML ops | No full ML framework |
| cuSPARSE | SpMV, SpMM | 10+ | Science ops | Not general-purpose |
| cuRAND | LCG, Mersenne Twister | 5+ | Full parity | |
| cuSOLVER | Eigensolve, LU, Cholesky | 15+ | Full parity for f64 | |
| Thrust | Reduction, scan, sort | 20+ | Full parity | |

**{{ total_stat(stat="wgsl_files") }} WGSL shaders total** — every one is f64-canonical (native f64 on pro GPUs,
DF64 emulation on consumer GPUs). Key domains:

| Domain | Shader Count | Example Operations |
|--------|:-----------:|-------------------|
| Linear algebra | 80+ | GEMM, eigensolve, SVD, LU, Cholesky |
| Statistics | 60+ | Welford, Pearson, bootstrap, jackknife |
| Signal processing | 40+ | FFT, convolution, Savitzky-Golay, CWT |
| Bioinformatics | 94+ | Diversity, alignment, phylogeny, [DADA2](@/lab/notebooks/02-benchmark-python-vs-rust.md) |
| Physics | 70+ | MD, spectral, Anderson, transport |
| Pharmacometrics | 30+ | Hill, PBPK, PopPK, ODE systems |
| ML primitives | 50+ | Attention, GELU, LayerNorm, softmax |
| Precision | 40+ | DF64 arithmetic, transcendentals |

### 3. nvcc / ptxas → coralReef Sovereign Compiler

| NVIDIA Tool | {{ entity(name="coralreef") }} Replacement | Status | Evidence |
|-------------|----------------------|--------|----------|
| nvcc (CUDA→PTX) | naga WGSL→SPIR-V + custom lowering | **Production** | 2,241 tests |
| ptxas (PTX→SASS) | Pure Rust SPIR-V→SASS codegen | **Production** | 46/46 shaders compile to SM70/SM86 |
| NVVM (SPIR-V→PTX) | Bypassed — 12/12 NVVM poisoning patterns solved | **Sovereign** | f64 transcendentals, DF64, FMA |
| libnvidia-compiler | Zero dependency — entire compile path is pure Rust | **Sovereign** | `#![forbid(unsafe_code)]` on glowplug |

**What "46/46 shaders compile" means:** 46 representative {{ entity(name="barracuda") }} shaders —
covering every domain (bio, physics, ML, linear algebra) — compile from WGSL
to native SASS (SM70 Volta, SM86 Ampere) and native RDNA2 (GFX1030) without
ANY vendor toolchain. No nvcc, no ptxas, no ROCm. Pure Rust compiler.

### 4. NVIDIA Kernel Driver → coral-glowplug + VFIO

| Driver Layer | Sovereign Replacement | Status |
|-------------|----------------------|--------|
| nvidia.ko (kernel module) | VFIO-pci (upstream Linux kernel) | **Production** |
| nvidia-drm (display) | Not needed (compute-only VFIO) | Bypassed |
| libnvidia-glcore | Not needed (Vulkan/wgpu path) | Bypassed |
| nvidia-uvm (unified memory) | Direct VRAM R/W via BAR0 | Validated (24/26 tests) |
| Device lifecycle | coral-glowplug (systemd daemon, JSON-RPC) | **Production** |
| Boot binding | VFIO-first boot (before display manager) | **Production** |
| Power management | D3hot→D0 sovereign recovery, HBM2 BIOS-trained VRAM survives | Validated |
| Firmware execution | FECS direct execution from host-loaded IMEM | Proven (Exp 068) |

**coral-glowplug is a systemd daemon** that manages GPU lifecycle without any
NVIDIA software. It binds GPUs to vfio-pci at boot, provides hot-swap
personality management (`VfioPersonality`, `NouveauPersonality`, `AmdgpuPersonality`),
health monitoring, and graceful shutdown — all via JSON-RPC 2.0 over Unix socket.

---

## Current Performance vs CUDA/Kokkos

| Benchmark | CUDA/Kokkos | {{ entity(name="ecoprimals") }} (wgpu) | {{ entity(name="ecoprimals") }} (DF64) | Notes |
|-----------|:-----------:|:-----------------:|:-----------------:|:---:|
| Yukawa MD (N=10K, 80K steps) | ~1 hr (HPC) | 3.66 hrs (RTX 4070) | — | 3.7× slower than CUDA |
| Lattice QCD 32⁴ β-scan | — | 13.6 hrs ($0.58) | — | First on consumer GPU |
| Nuclear EOS L1 | 184 s (Python) | **2.3 s** | — | Rust vs Python (compiled vs interpreted) |
| f64 throughput | A100 native f64 | 0.35 TFLOPS (native) | 2,130 matmul/sec (measured) | Apples-to-oranges: A100 is native f64, DF64 is emulated ~14-digit |
| Kokkos Verlet stepper | 1.0× reference | — | 0.27× (3.7× gap) | Active optimization |

**The gap is narrowing:** {{ entity(name="hotspring") }} Kokkos parity tracking shows 27×→12.4×→3.7×
improvement over three months. The remaining gap is primarily in DF64
transcendental functions (exp, log, sin, cos) where NVIDIA's NVVM has
hand-optimized silicon paths. {{ entity(name="coralreef") }}'s sovereign transcendentals use
Newton-Raphson iteration at slightly higher latency.

---

## What's Coming Next (Near-Term, Given Velocity)

Based on the 27-day sprint velocity (architecture/EVOLUTION_TIMELINE.md), with
BarraCuda gaining ~50 new shaders/month and {{ entity(name="coralreef") }} closing 2–3 NVVM bypass
patterns per iteration:

### 3 Months (June 2026)

| Target | What Changes |
|--------|-------------|
| Kokkos gap → <2× | DF64 transcendental optimization in {{ entity(name="coralreef") }} (FMA fusion, Newton-Raphson refinement) |
| {{ entity(name="coralreef") }} dispatch | Full compute dispatch via VFIO (compile + launch on same sovereign path) |
| Multi-GPU | toadStool multi-adapter dispatch (RTX 3090 + Titan V in parallel) |
| {{ entity(name="barracuda") }} 1,000 shaders | Cover remaining cuBLAS L3 ops, sparse ops, Krylov solvers |
| AMD E2E production | RX 6950 XT full pipeline: compile + dispatch + validate |

### 6 Months (September 2026)

| Target | What Changes |
|--------|-------------|
| {{ entity(name="helixvision") }} Phase D | End-to-end protein structure prediction pipeline (FASTA→MSA→Evoformer→structure) |
| AlphaFold timing parity | ~3 min/sequence on consumer GPU vs ~5 min cloud AlphaFold |
| Intel backend | Arc GPUs via {{ entity(name="coralreef") }} third backend |
| {{ entity(name="genomebin") }} deployment | Self-extracting single-file sovereign GPU stack |
| Kokkos gap → <1.5× | Approaching throughput parity on science workloads |

### 12 Months (March 2027)

| Target | What Changes |
|--------|-------------|
| Full NVVM replacement | All cuBLAS/cuFFT/cuDNN science ops at parity |
| {{ entity(name="barracuda") }} 2,000+ shaders | Coverage comparable to CUDA ecosystem for science |
| {{ entity(name="coralreef") }} sovereign dispatch production | Complete GPU lifecycle: boot → compile → dispatch → recover |
| Four-vendor GPU support | NVIDIA, AMD, Intel, Apple (Metal via wgpu) |

---

## The Proprietary Cost of What We Replace

| Tool | Cost | What {{ entity(name="ecoprimals") }} Replaces It With |
|------|:----:|--------------------------------|
| CUDA Toolkit | Free (vendor lock) | wgpu/Vulkan (open standard, cross-vendor) |
| cuBLAS/cuFFT | Free (NVIDIA-only) | BarraCuda {{ total_stat(stat="wgsl_files") }} WGSL shaders (any GPU with Vulkan) |
| nvcc compiler | Free (NVIDIA-only) | {{ entity(name="coralreef") }} (pure Rust, SM70–SM89 + RDNA2) |
| NVIDIA driver | Free (proprietary binary) | coral-glowplug + VFIO (upstream Linux kernel) |
| A100 GPU | ~$10K–15K | RTX 3090 (~$500 used), DF64: 2,130 matmul/sec measured |
| HPC allocation | $50–500/run | $0.044/run (electricity, consumer GPU) |
| MATLAB Parallel | ~$2K/yr + GPU toolbox | toadStool + BarraCuda (AGPL-3.0, free) |
| Kokkos | Free (C++, complex build) | BarraCuda (Rust, `cargo build`) |
| LAMMPS | Free (Fortran/C++, HPC) | {{ entity(name="hotspring") }} (Rust, consumer GPU) |

**Annual lab savings (conservative):** $0 in software licenses (all were
technically free) but ~$50K–200K in HPC allocations avoided for a lab
running regular plasma, QCD, or bioinformatics GPU workloads. Plus the
unmeasurable value of zero-queue 24/7 access and data that never leaves
the lab.

---

## The Sovereign Stack Diagram

```
Application Layer
  └── Springs (wetSpring, hotSpring, neuralSpring, etc.)
      └── Science experiments: reproduce published papers

Math Layer
  └── BarraCuda ({{ total_stat(stat="wgsl_files") }} WGSL shaders)
      └── f64-canonical math: what to compute

Orchestration Layer
  └── ToadStool S157 (96 JSON-RPC methods, 21,156 tests)
      └── Hardware discovery: where and how to compute

Compiler Layer
  └── coralReef Phase 10 Iter 53 (2,241 tests)
      └── WGSL → SPIR-V → native SASS/RDNA2

Hardware Layer
  └── coral-glowplug (systemd daemon, JSON-RPC)
      └── VFIO device lifecycle: boot, bind, dispatch, recover

Nothing above depends on:
  ✗ NVIDIA CUDA toolkit
  ✗ nvcc / ptxas / NVVM
  ✗ nvidia.ko kernel module
  ✗ Any C/C++ library
  ✗ Any cloud service
  ✗ Any vendor-specific API
```

Every layer is pure Rust, AGPL-3.0, and publicly auditable. The entire
scientific computing stack — from math primitives to GPU binary compilation
to hardware lifecycle management — belongs to the user.

---

---

## Reproduce It

```bash
git clone https://github.com/ecoPrimals/barraCuda && cd barraCuda
cargo test --workspace          # all tests pass
cargo run --release --bin validate  # exit 0 = pass
```

**Hardware tested**: NVIDIA RTX 4070, RTX 5090, AMD RDNA2, Intel Arc  
**Precision**: f64 via Vulkan `SHADER_F64` extension  
**Date**: July 2026  
**Author**: ecoPrimal ([ORCID 0009-0004-2141-0321](https://orcid.org/0009-0004-2141-0321))

## Limitations

- f64 GPU support requires Vulkan `shaderFloat64` — some mobile and integrated GPUs lack this
- Shader compilation is pure Rust (coralReef) but currently targets SPIR-V; Metal/DX12 backends are planned
- Performance comparisons to CUDA are indirect — we benchmark scientific output, not raw FLOPS
- No CUDA interop; this is a clean replacement, not a compatibility layer

*Repositories:  
[ecoPrimals/barraCuda](https://github.com/ecoPrimals/barraCuda) ·
[ecoPrimals/toadStool](https://github.com/ecoPrimals/toadStool) ·
[ecoPrimals/coralReef](https://github.com/ecoPrimals/coralReef) ·
[syntheticChemistry/hotSpring](https://github.com/syntheticChemistry/hotSpring)*
