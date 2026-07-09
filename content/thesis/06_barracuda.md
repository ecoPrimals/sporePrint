+++
title = "Chapter 6: BarraCuda"
description = "Vendor-agnostic Pure Rust GPU compute (WGSL/Vulkan, f64) — NTT-to-FFT structural evolution as the principal constrained-evolution case study."
weight = 6
date = 2026-07-09
+++

{{ maturity(level="planned") }}

## Vendor-Agnostic GPU Compute

BarraCuda is toadStool's Pure Rust GPU compute library — WGSL shaders on Vulkan, f64 precision on consumer hardware. It evolved under ML, FHE, and vendor-portability pressures; proved fit for computational physics without being designed for it.

The **NTT→FFT structural evolution** is the principal constrained-evolution case study in the codebase: ~97% kernel identity between `fhe_ntt.wgsl` and `fft_1d.wgsl`, with the FFT header noting *"Adapted from fhe_ntt.wgsl (80% structure reuse!)"*.

---

**See also:**

- [Quantitative Evidence](@/thesis/13_quantitative_evidence.md) — the NTT→FFT measurements
- [Results: hotSpring](@/thesis/08_results_hotspring.md) — BarraCuda validated against plasma physics

---

*Full content transplant pending. Source: `whitePaper/gen3/thesis/06_barracuda.md`*
