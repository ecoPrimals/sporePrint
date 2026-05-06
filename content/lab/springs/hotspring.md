+++
title = "hotSpring — Computational Plasma Physics, Lattice QCD, Spectral Theory"
description = "Dense plasmas, nuclear structure, lattice QCD — 697+ tests on consumer GPUs for $0.044. Paper parity on a $600 card."
date = 2026-05-06
weight = 2

[taxonomies]
primals = ["barracuda", "toadstool", "coralreef"]
springs = ["hotspring", "groundspring", "neuralspring", "wetspring"]
+++

## Domain

Dense plasmas, nuclear structure, molecular dynamics, lattice QCD, spectral theory, neuromorphic computing.

**Repository**: [syntheticChemistry/hotSpring](https://github.com/syntheticChemistry/hotSpring)

## The Science Story

hotSpring is the primary GPU science driver — the spring that proves barraCuda can do first-principles computational physics on consumer hardware. It is the most mature spring because physics has the least room to hide: 0.000% energy drift or the simulation is wrong.

## Headline Results

- **Sarkas Yukawa MD** at paper parity (N=10,000, 80k steps) on a **$600 RTX 4070** for **$0.044** in electricity
- **Full AME2020 nuclear dataset** (2,042 nuclei — 39× the published paper) on a single consumer GPU
- **Lattice QCD** β-scans (32⁴, 12 temperatures) resolving the deconfinement transition on a $500 RTX 3090 for $0.58
- **DF64** delivers 3.24 TFLOPS of double precision on FP32 cores
- Phase 0 discovered and fixed **5 silent bugs** in the upstream Sarkas code

## Validation Phases

| Phase | Key Result |
|-------|------------|
| A–E (MD) | Python → Rust → GPU → f64 → paper parity. 0.000% energy drift. $0.044 electricity |
| F (Nuclear EOS) | 2,042 nuclei AME2020 on consumer GPU. 478× speedup, 44.8× energy reduction |
| Lattice QCD | SU(3) HMC + dynamical fermions. 32⁴ β-scan, deconfinement at β=5.69 |
| Spectral | Anderson localization (1D/2D/3D), Hofstadter butterfly, Lanczos eigensolver |
| NPU | 10 SDK assumptions overturned. ESN streaming at 2.8μs/step |

## Researchers Reproduced

| Researcher | Department | Domain |
|------------|-----------|--------|
| Michael Murillo | CMSE, MSU | Dense plasmas, WDM, molecular dynamics |
| Alexei Bazavov | CMSE + Physics, MSU | Lattice QCD, thermodynamics |
| Ilya Kachkovskiy | Math, MSU | Spectral theory, Anderson localization |
| Rika Anderson | Biology, Carleton | Pangenomics (cross-spring) |

## What the Constraint Revealed

Eliminating CUDA forced Vulkan, which exposed `SHADER_F64` on consumer GPUs. Eliminating vendor compilers forced coralReef, which now compiles 93/93 cross-spring WGSL shaders to native GPU binaries. A $300 Akida NPU runs ESN inference at 2.8μs/step — 1,000× faster than GPU for streaming workloads, 9,017× less energy for transport predictions.

## Cross-Spring Connections

- **→ airSpring**: f64 GPU dispatch batching pattern
- **→ wetSpring**: FusedMapReduceF64 pattern for bulk statistics; Anderson localization shared primitives
- **→ ToadStool**: 195 acceptance checks, 6 bugs found
- **→ neuralSpring**: isomorphic GEMM serves plasma and nuclear
- **→ groundSpring**: spectral primitives + QCD inverse problems

## baseCamp Papers

Papers 07, 10, 15, 25 — see [baseCamp Science](/science/) for full list.
