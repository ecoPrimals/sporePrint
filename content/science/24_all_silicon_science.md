+++
title = "All-Silicon Science"
description = "GPU Hardware x Computational Physics — systematic mapping of physics to all 9 GPU silicon unit types, sovereign compiler. hotSpring + coralReef."
date = 2026-03-30

[extra]
paper_number = 24
domain = "Physics and Materials"

[taxonomies]
primals = ["barracuda", "coralreef", "toadstool"]
springs = ["hotspring"]
+++

**Date:** March 30, 2026 (updated — sovereign pipeline operational, AMD scratch memory working)
**Status:** Silicon saturation profiling **complete**. **Sovereign GPU pipeline operational** — coralReef NVIDIA GPFIFO working on RTX 3090, **AMD scratch/local memory working on RX 6950 XT** (Exp 124: FLAT_SCRATCH prolog fix). TMU PRNG, subgroup reduce, ROP atomics **LIVE** in production RHMC. The sovereign compiler path (coralReef) eliminates wgpu/Vulkan/naga for both vendors, enabling direct access to every silicon unit without driver abstraction overhead. 7/8 HW parity tests pass, 1672 unit tests pass. Capacity: RTX 3090 L=46⁴ dynamical (23.6 GB), RX 6950 XT L=40⁴ (13.5 GB). 870 lib tests, 139 binaries, 99 WGSL shaders.
**Domain:** GPU hardware architecture × computational physics × all-silicon pipeline
**Novelty:** No prior work systematically maps computational physics operations to
all 9 GPU silicon unit types with empirical throughput measurements and tolerance
characterization, nor proposes a tolerance-based routing system that automatically
selects hardware units based on mathematical precision requirements.
**Cross-Spring:** hotSpring × barraCuda × toadStool × coralReef × ALL springs

---

## Abstract

Modern GPU dies contain at least 9 distinct hardware units: shader cores, tensor
cores, RT cores, texture units, ROPs, rasterizer, depth buffer, tessellator, and
video encoder. Each was designed for a specific graphics operation but computes a
general mathematical function. The DF64 discovery proved the pattern: fp32 ALUs
"designed for pixel colors" emulate fp64 at 8-16x throughput. This sub-thesis
extends that discovery systematically across every unit on the die.

We map 11 QCD operations to their optimal silicon units, empirically validate the
TMU pathway (1.89x throughput for table lookup on RTX 3090), discover that AMD
RDNA2 outperforms NVIDIA Ampere on DF64 arithmetic by 38%, and propose a
tolerance-based routing system where the mathematical precision requirement — not
the programmer's hardware choice — determines which silicon executes the work.

## Key Results

### TMU Table Lookup (texture_unit)

Precomputed exp(x) in a 1024-entry texture. Compute shader accesses via
`textureLoad`. TMU hardware performs the memory fetch through its spatial cache,
bypassing the general-purpose memory hierarchy.

- **RTX 3090 (328 TMUs):** 1.89x throughput over compute exp()
- **RX 6950 XT (96 TMUs):** 1.24x throughput
- **llvmpipe (0 TMUs):** 0.92x (CPU emulation, no hardware TMU)

Speedup correlates with TMU count ratio: NVIDIA has 3.4x more TMUs.

### AMD DF64 Advantage

AMD RDNA2 produces 38% better DF64 throughput (23.4M vs 16.9M ops/s) and
higher DF64 fidelity than NVIDIA Ampere. This is a genuine architectural
advantage for double-float science computing that no existing framework
exploits.

### The Hidden Computers

Each GPU unit computes a specific mathematical function at silicon speed:

| Unit | Mathematical Function | Science Application |
|------|----------------------|-------------------|
| Shader Core | FP arithmetic (add, mul, fma) | All compute (baseline) |
| Texture Unit | 2D interpolated lookup | EOS tables, exp/log |
| Tensor Core | Matrix multiply-accumulate | CG solver, FFT butterfly |
| RT Core | BVH traversal + intersection | MD neighbor search |
| ROP | Per-pixel scatter-add | Force accumulation |
| Rasterizer | Point-in-polygon + interpolation | Particle binning |
| Depth Buffer | Per-pixel min reduction | Voronoi diagrams |
| Tessellator | Adaptive mesh subdivision | AMR |
| Video Encoder | Block transform + entropy | Trajectory compression |

## Architectural Contribution

### Tolerance-Based Routing

The key insight: barraCuda specifies tolerance (e.g., 1e-14 relative error),
not hardware (e.g., "use fp64"). toadStool maps tolerance to hardware based
on measured performance surface data from spring experiments.

```
Application: barraCuda.math.pairwise.yukawa(particles, tolerance=1e-14)
                         │
                   toadStool routing (measured performance surface)
                         │
         ┌───────────────┼───────────────────┐
    shader_core     texture_unit          tensor_core
    (DF64 force)    (EOS lookup)      (CG preconditioner)
```

A new hardware unit requires only: coralReef learns to emit its instructions,
toadStool learns to discover and route to it. barraCuda and all springs are
unchanged.

### The Compound Effect

If each of the 8 non-shader-core units yields even 3-5x improvement on the
operations it accelerates:

| Configuration | Effective TFLOPS | Source |
|--------------|-----------------|--------|
| Native fp64 only | 0.33 | RTX 3090 fp64 rate |
| + DF64 | 3.24 | fp32 ALU double-float |
| + TMU tables | ~5 | Table lookup offloads transcendentals |
| + Tensor CG | ~20 | TF32 MMA for solver |
| + RT neighbors | ~25 | BVH spatial queries |
| + ROP scatter | ~30 | Hardware accumulation |
| + Full pipeline | ~50-100 | All units running in parallel |

A single consumer GPU running the all-silicon pipeline could match a small
HPC cluster for science throughput.

## Relation to Constrained Evolution Thesis

The GPU's hardware units evolved under the constraint of real-time graphics:
rasterizers for triangles, depth buffers for occlusion, TMUs for textures.
The science adapts to these constraints — mapping physics operations onto the
I/O contracts that already exist in silicon. This is constrained evolution at
the hardware level: the "organism" (scientific computation) doesn't redesign
the "environment" (GPU silicon) but finds unexpected fitness in the existing
landscape.

## Answered Questions (March 29, 2026)

1. **Can TMU accelerate PRNG in production physics?** YES — Box-Muller via
   `textureLoad` offloads log/cos/sin to TMU hardware. Wired into production
   RHMC via `GpuHmcStreamingPipelines::new_with_tmu`.

2. **Can ROP atomics accelerate force accumulation?** YES — Fixed-point i32
   `atomicAdd` (scale 2^20, ~6 digits) enables parallel pole dispatch with
   zero inter-pole barriers. AMD 6x faster atomics vs NVIDIA (93.6 vs 15.6 Gatom/s).

3. **Can subgroup operations accelerate CG reductions?** YES — `subgroupAdd()`
   eliminates shared-memory barriers for intra-subgroup reduction. Feature-gated:
   automatic fallback when `wgpu::Features::SUBGROUP` unavailable.

4. **What is the maximum physics a consumer card can do?** RTX 3090: L=46⁴
   dynamical RHMC (4.5M sites, 23.6 GB). RX 6950 XT: L=40⁴ (2.6M sites, 13.5 GB).
   Both fit 32⁴ (5.5 GB). Bottleneck: `wgpu::Limits::max_buffer_size` (4 GB per-allocation)
   and total VRAM.

## Open Questions

1. Does TMU linear filtering (`textureSampleLevel`) close the accuracy gap
   to sub-0.1% while maintaining the throughput advantage?
2. Can the rasterizer's spatial query throughput exceed compute-shader
   binning by 10-50x for particle methods as hypothesized?
3. Does the depth buffer Voronoi trick work for distance fields in 3D
   (using multi-view rendering)?
4. Can tensor core MMA at TF32 serve as a CG preconditioner while DF64
   handles the refinement — mixed-precision CG across silicon units?
5. What is the overhead of mixed command streams (compute + draw + RT)
   versus sequential dispatch?
6. How much does TMU PRNG improve wall-clock time at large lattice sizes
   where PRNG is a larger fraction of total cost?
7. Does the i32 fixed-point precision (2^20 scale, ~6 digits) introduce
   measurable systematic error in observables at 32⁴+ volumes?

## References

- hotSpring Exp 096: `experiments/096_SILICON_SCIENCE_TMU_QCD_MAPPING.md`
- hotSpring Silicon Saturation: `whitePaper/baseCamp/silicon_science.md`, `silicon_characterization_at_scale.md`
- wateringHole handoff: `HOTSPRING_V0632_SILICON_SATURATION_PRIMAL_EVOLUTION_HANDOFF_MAR29_2026.md`
- wateringHole: `GPU_FIXED_FUNCTION_SCIENCE_REPURPOSING.md`
- toadStool: `specs/ALL_SILICON_PIPELINE.md`
- Sub-thesis 14: `14_sovereign_compute_hardware.md`
- Sub-thesis 25: `25_self_tuning_simulation.md`
- Dekker, T. J. (1971). "A floating-point technique for extending the available precision"
