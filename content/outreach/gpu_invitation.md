+++
title = "An Invitation to GPU Manufacturers — Vendor-Agnostic Scientific Compute Validation"
description = "ecoPrimals validates GPU hardware capability via WGSL/WebGPU across 10 scientific domains — f64 precision, no CUDA. A real-world validation suite for any GPU exposing Vulkan."
weight = 1
date = 2026-07-26

[taxonomies]
primals = ["barracuda", "toadstool", "coralreef"]

[extra]
foundation = true
maturity = "live"

[[extra.companions]]
url = "/technical/sovereign-gpu-pipeline-profile/"
title = "Cross-Vendor f64 GPU Computing"
relation = "evidence_for"

[[extra.companions]]
url = "/lab/notebooks/02-benchmark-python-vs-rust/"
title = "GPU-Accelerated DADA2 Benchmark"
relation = "evidence_for"
+++

**This is a standing invitation. A human reads and responds to every message at [eco.primal@pm.me](mailto:eco.primal@pm.me).**

---

## The Value Proposition

ecoPrimals has {{ total_stat(stat="wgsl_files") }} validated WGSL compute shaders that exercise
GPU hardware across 10 scientific domains — linear algebra, FFT, Monte Carlo,
bioinformatics, lattice QCD, molecular dynamics, pharmacometrics, agriculture,
game science, and signal processing. All at f64 precision through Vulkan/WebGPU.

This is a real-world scientific GPU validation suite that proves hardware
capability independent of CUDA or any vendor SDK.

**Tested hardware**: NVIDIA RTX 4070, RTX 5090, AMD RDNA2, Intel Arc.

---

## What We Prove About Your Hardware

| Domain | Shaders | What they exercise |
|--------|---------|-------------------|
| Linear algebra | 60+ | GEMM, SVD, eigendecomposition, sparse operations |
| Statistics | 60+ | Welford, Pearson, bootstrap, jackknife |
| Signal processing | 40+ | FFT, convolution, Savitzky-Golay, CWT |
| [Bioinformatics](@/lab/notebooks/02-benchmark-python-vs-rust.md) | 94+ | DADA2 denoising, diversity indices, phylogenetics |
| [Physics](@/products/lattice_qcd.md) | 70+ | SU(3) gauge theory, Anderson localization, MD |
| Pharmacometrics | 30+ | Hill, PBPK, PopPK, ODE systems |

Every shader compiles, runs, and validates on consumer hardware. The results
trace back to published reference values. See the
[full GPU pipeline profile](@/technical/SOVEREIGN_GPU_PIPELINE_PROFILE.md).

---

## What We're Asking

Not investment or endorsement. **Access to hardware validation programs.**

1. **Early driver access** — we catch GPU compute regressions before gamers do
2. **Hardware loan program** — we'll validate your newest silicon against 10 science domains
3. **Bug reports** — we file detailed, reproducible shader bug reports (we already do this)

The validation suite is AGPL-3.0 and public. You can run it today on any
hardware that exposes Vulkan drivers.

---

*{{ total_stat(stat="total_loc_display") }} Rust. {{ total_stat(stat="wgsl_lines_display") }} WGSL. {{ total_stat(stat="total_tests_display") }} tests. The proof of work is the work itself.*
