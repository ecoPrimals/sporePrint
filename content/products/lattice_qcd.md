+++
title = "Lattice QCD on Consumer GPUs Without CUDA — Pure Rust Gauge Theory"
description = "Lattice QCD on consumer GPUs without CUDA or vendor SDKs. Pure Rust + WGSL, 59/59 validation checks, 3 published papers reproduced, single static binary. Runs on NVIDIA, AMD, Intel."
date = 2026-04-04

[taxonomies]
primals = ["barracuda", "coralreef", "toadstool", "biomeos"]
springs = ["hotspring", "groundspring"]

[extra]
maturity = "reproduced"
+++

**Organization**: {{ entity(name="sporegarden") }} (product naming in progress)  
**License**: {{ entity(name="scyborg") }} (AGPL-3.0-or-later + ORC + CC-BY-SA 4.0)  
**Status**: Engine validated, product packaging in development

---

## Lattice QCD on Consumer GPUs — No CUDA, No Cluster

Run lattice QCD on a consumer GPU (NVIDIA, AMD, Intel via Vulkan) — no CUDA,
no vendor SDK, no HPC cluster access required. A single static Rust binary
produces MILC-compatible gauge configurations. The physics engine already exists across {{ entity(name="hotspring") }}, {{ entity(name="barracuda") }}, {{ entity(name="coralreef") }}, and {{ entity(name="toadstool") }}. The product is the packaging: a {{ entity(name="guidestone") }}-certified deployment artifact that a lattice physicist can `scp` to any machine and run.

The established lattice QCD toolchain — [QUDA](https://github.com/lattice/quda) (C++/CUDA, GPU), [MILC](https://github.com/lattice/milc_qcd) (C, CPU), [Chroma](https://github.com/lattice/chroma) (C++, JeffersonLab) — requires CUDA, vendor SDKs, MPI, and HPC cluster access. This product replaces all of it with a single static binary.

---

## What Already Works

The physics engine is validated. The springs are the acceptance tests.

| Capability | Primal | Evidence |
|-----------|--------|---------|
| Wilson gauge action + SU(3) | {{ entity(name="barracuda") }} | Plaquette at beta=6.0: 0.5929 (literature ~0.594) |
| Gradient flow (W6, W7, CK4, LSCFRK3) | {{ entity(name="barracuda") }} | Convergence orders 2.06/2.08/2.11, LSCFRK3 coefficients derived from first principles |
| Staggered fermions + HMC | {{ entity(name="barracuda") }} | Dynamical N_f=4 adaptive Omelyan — in progress |
| f64 on consumer GPUs | {{ entity(name="barracuda") }} + {{ entity(name="coralreef") }} | Vulkan SHADER_F64: native f64 at 1:2 throughput on RTX 4070 |
| Sovereign GPU compiler | {{ entity(name="coralreef") }} | WGSL to native GPU binary — no LLVM, no NVCC, no vendor SDK |
| Hardware dispatch | {{ entity(name="toadstool") }} | NVIDIA SM70-SM89, AMD RDNA2 (GFX1030), auto-detection |
| Cross-substrate parity | {{ entity(name="guidestone") }} | 40/40 bit-identical across 5 substrates (x86_64, aarch64, NVIDIA, AMD, CPU-only) |
| {{ entity(name="guidestone") }} certification | hotSpring-guideStone-v0.7.0 | 59/59 checks, 3 published papers reproduced, self-leveling benchmark |

Three published papers independently validated by the original author (TC Chuna, MSU/Murillo Group):

| Paper | Citation | Result |
|-------|----------|--------|
| Gradient flow | Bazavov & Chuna, arXiv:2101.05320 | 14/14 checks — integrators, t0/w0 scale, convergence |
| BGK dielectric | Chuna & Murillo, PRE 111, 035206 | 25/25 checks — Mermin, f-sum, DSF, conductivity |
| Kinetic-fluid coupling | Haack et al., JCP (2024) | 20/20 checks — BGK relaxation, Sod shock, coupled interface |

---

## What the Product Adds

The engine does the physics. The product packages it for lattice physicists.

| Feature | What It Does |
|---------|-------------|
| ILDG-compatible output | Gauge configurations in the International Lattice Data Grid format — directly consumable by MILC, Chroma, and existing analysis tools |
| Measurement pipeline | Plaquette, Polyakov loop, topological charge, Wilson flow observables — the standard lattice measurements |
| Self-leveling benchmark | `./hotspring benchmark` characterizes unknown hardware against published lattice results — the physics is the benchmark |
| {{ entity(name="deploygraph") }} composition | {{ entity(name="hotspring") }} + {{ entity(name="barracuda") }} + {{ entity(name="coralreef") }} composed via {{ entity(name="biomeos") }} as a single {{ entity(name="byob") }} {{ entity(name="niche") }} |
| Portable artifact | Static musl binary, dual-arch (x86_64 + aarch64), OCI container, USB-deployable |

---

## How It Composes

| Layer | What | Primal |
|-------|------|--------|
| Math | WGSL f64 shaders: gauge action, force, HMC, gradient flow, spectral | {{ entity(name="barracuda") }} |
| Compilation | WGSL to native GPU binary (NVIDIA + AMD) | {{ entity(name="coralreef") }} |
| Dispatch | Hardware discovery, GPU scheduling, workload routing | {{ entity(name="toadstool") }} |
| Validation | {{ entity(name="hotspring") }} — the spring that proves the physics | {{ entity(name="hotspring") }} |

---

## Why It Matters

| | QUDA + MILC + Chroma | This product |
|--|:---:|:---:|
| Language | C, C++, Fortran | **Rust** |
| GPU backend | CUDA (NVIDIA only) | **Vulkan / WGSL** (NVIDIA, AMD, Intel) |
| Precision | f64 on compute-class only | **f64 on consumer GPUs** ($600 RTX 4070) |
| Dependencies | CUDA SDK, MPI, autoconf, LLVM | **Zero** (static binary) |
| Installation | Days (build MILC, QUDA, configure MPI, test) | **Minutes** (`tar xf && ./hotspring validate`) |
| Cost | HPC cluster allocation | **$4K basement workstation** |
| Deployment | Cluster job scripts | **USB drive** |
| Memory safety | Manual C/C++ | **Compiler-guaranteed** |

NVIDIA's CUDA pricing model throttles consumer f64 to 1:64 throughput to protect the compute-class product line. Vulkan's `SHADER_F64` extension exposes the native 1:2 ratio. The $600 RTX 4070 does the same f64 physics as a $10,000 A100 — CUDA just doesn't let you see it.

---

## Current Status

| Component | Maturity | Detail |
|-----------|----------|--------|
| Physics engine | {{ maturity(level="reproduced") }} | 59/59 checks, 3 papers, cross-vendor GPU parity |
| ILDG output format | {{ maturity(level="planned") }} | In development — MILC-compatible gauge configs |
| Measurement pipeline | {{ maturity(level="implemented") }} | Plaquette and flow observables working; Polyakov loop and topological charge next |
| Product packaging | {{ maturity(level="planned") }} | Product naming and sporeGarden repo pending |

---

*See also: [guideStone](@/guidestone/_index.md) for the verification class,
[Paper 10 — First Dynamical QCD on Consumer GPU](@/science/10_dynamical_qcd_production.md),
[Paper 07 — Sovereign WDM Simulation](@/science/07_sovereign_wdm.md),
[Primal Catalog](@/architecture/PRIMAL_CATALOG.md) for barraCuda and coralReef details.*
