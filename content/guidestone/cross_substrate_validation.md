+++
title = "Cross-Substrate Validation"
description = "Five substrates, 40/40 bit-identical — the evidence that physics does not depend on instruction set, C library, GPU vendor, or operating system."
date = 2026-04-30
weight = 3

[taxonomies]
primals = ["toadstool", "barracuda", "coralreef"]
springs = ["hotspring", "groundspring"]
+++

## The Claim

A {{ entity(name="guidestone") }}-certified artifact produces the same physics
on any hardware it runs on. Not "approximately the same." Not "within a
few ULP." For the core validation suite: **bit-identical**.

This is a strong claim. This page presents the evidence.

---

## The Five Substrates

The first {{ entity(name="guidestone") }} artifact — `hotSpring-guideStone-v0.7.0` —
was validated across five substrates chosen to maximize diversity along
every axis that could affect floating-point results:

| # | Substrate | Arch | C Library | GPU | Kernel |
|---|-----------|------|-----------|-----|--------|
| 1 | Ubuntu 22.04 | x86_64 | glibc 2.35 | None (CPU only) | 5.15 |
| 2 | Ubuntu 22.04 | x86_64 | glibc 2.35 | NVIDIA RTX 3090 | 5.15 |
| 3 | Ubuntu 22.04 | x86_64 | glibc 2.35 | AMD RX 6950 XT | 5.15 |
| 4 | Alpine 3.19 | x86_64 | musl 1.2.4 | None (CPU only) | 6.6 |
| 5 | Ubuntu 22.04 | aarch64 | glibc 2.35 | None (qemu-user) | 5.15 |

Dimensions varied:
- **Instruction set**: x86_64 vs aarch64
- **C library**: glibc vs musl (though the binary is statically linked,
  this tests that no libc behavior leaks through)
- **GPU vendor**: NVIDIA vs AMD vs no GPU
- **GPU compiler**: {{ entity(name="coralreef") }} SASS (SM86) vs RDNA2 (GFX1030)
- **Kernel version**: 5.15 vs 6.6

---

## The Results

### Per-Substrate Check Results

| Substrate | Checks | Result |
|-----------|:------:|--------|
| Ubuntu x86_64, CPU | 59/59 | PASS |
| Ubuntu x86_64, RTX 3090 | 59/59 | PASS |
| Ubuntu x86_64, RX 6950 XT | 59/59 | PASS |
| Alpine x86_64, CPU | 59/59 | PASS |
| Ubuntu aarch64, CPU | 59/59 | PASS |

### Cross-Substrate Comparison

After all five substrates passed independently, outputs were compared
pairwise. For each of the 40 observable quantities (plaquettes, energies,
correlation functions, flow scales):

**40/40 bit-identical across all five substrates.**

Not "within tolerance." Not "within 1 ULP." The IEEE 754 double-precision
bit patterns are the same bytes on every substrate.

---

## Why Bit-Identity Is Possible

Bit-identical results across architectures are not the default in
scientific computing. Most HPC codes accept "within tolerance" because
floating-point non-associativity, FMA contraction, and thread scheduling
make exact reproducibility impractical.

{{ entity(name="guidestone") }} achieves it through four mechanisms:

### 1. Canonical Reduction Order

Parallel reductions (summing an array across GPU threads) use a fixed
binary tree structure rather than hardware-dependent scheduling. This
eliminates the primary source of floating-point non-determinism in GPU
computation.

{{ entity(name="barracuda") }} WGSL shaders implement this explicitly.
The reduction tree is part of the specification, not an implementation
detail.

### 2. Explicit FMA Policy

Fused multiply-add (FMA) changes results by absorbing the intermediate
rounding. {{ entity(name="coralreef") }} emits FMA instructions with
documented contraction semantics. The same FMA policy applies whether
the target is NVIDIA SASS or AMD GFX1030.

### 3. Pure Rust Arithmetic

The CPU path uses Rust's `f64` arithmetic with explicit operation ordering.
No LAPACK, no BLAS, no vendor math library. The same Rust source compiles
to both x86_64 and aarch64 with identical semantics because there is no
C library in the hot path.

### 4. Tolerance Decomposition

{{ entity(name="groundspring") }} decomposes the uncertainty budget for
every observable. When the dominant uncertainty is gauge sampling variance
(statistical), the deterministic tolerance is set far below it. Bit-identity
is achievable because the numerical tolerance headroom is orders of
magnitude larger than the floating-point representation differences.

---

## What Bit-Identity Does Not Cover

The 40/40 bit-identical result applies to the **core validation observables**
— quantities computed from reference gauge configurations with fixed random
seeds and deterministic integration paths.

Quantities that involve:

- **Monte Carlo sampling** with different random seeds — statistically
  consistent, not bit-identical
- **Iterative solvers** with hardware-dependent convergence — results
  agree within named tolerance, not bit-identical
- **Timing-dependent operations** — wall time varies, physics does not

The distinction is precise: deterministic computations (same input, same
algorithm, same operation order) are bit-identical. Stochastic computations
(sampling, random initialization) are statistically consistent within
derived tolerances.

---

## The Implication

When a PI runs `./hotspring validate` on their laptop and gets 59/59 PASS,
they know:

1. The physics on their machine matches the physics on every other machine
   that has ever run this artifact
2. The match is not approximate — it is exact for deterministic quantities
3. The tolerances for stochastic quantities are derived, not guessed, and
   the dominant uncertainty source is named
4. No vendor SDK, no institutional license, no cloud subscription was
   required to achieve this

The computation is the proof. The substrate is irrelevant. This is what
{{ entity(name="guidestone") }} means.
