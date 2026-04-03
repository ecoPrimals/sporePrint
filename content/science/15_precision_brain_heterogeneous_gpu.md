+++
title = "Precision Brain on Heterogeneous GPU"
description = "Neuroscience x GPU Computing"
date = 2026-03-17

[extra]
paper_number = 15
+++

# Sub-Thesis 15: Self-Routing Precision Brain for Heterogeneous GPU Compute

**Date:** March 11, 2026
**Status:** **Validated + Live Kokkos Benchmark** — hotSpring v0.6.28, 847 lib tests, bench_precision_eval clean exit, both GPUs profiled. coralReef Iter 30 sovereign validation: 45/46 compile, 12/12 NVVM bypass, FMA lowering (`FmaPolicy::Separate`) unlocks F64Precise via sovereign path. Exp 050-051. toadStool S145 absorbed PrecisionBrain with `PrecisionHint` routing + `NvkZeroGuard`. barraCuda `a012076` absorbed PrecisionBrain/HardwareCalibration/PrecisionTier from hotSpring v0.6.25. **Exp 053**: Live Kokkos parity benchmark validates precision routing in production — DF64 transcendental poisoning bug discovered and fixed (silent zero-force on Ampere proprietary), native f64 fallback engages correctly when `has_nvvm_df64_poisoning_risk()` is true. 12.4× gap vs Kokkos-CUDA dominated by 1:32 f64 rate; safe DF64 exp path is the single biggest unlock.
**Domain:** GPU computing, precision engineering, hardware discovery
**Novelty:** Data-driven self-routing precision brain that discovers hardware
capabilities at startup, routes physics workloads to optimal precision tier
per-domain, and survives driver-level device poisoning — without static
heuristics or vendor-specific codepaths
**Cross-Spring:** hotSpring (precision profiling, physics domains) ×
barraCuda (precision tiers, shader compilation) × toadStool (hardware
discovery, GPU dispatch)

---

## Abstract

Consumer GPUs provide three fundamentally different levels of floating-point
precision: F32 (native, fast), F64 (native, throttled on consumer silicon),
and DF64 (double-float emulation using f32 pairs, unlocking FP32 core arrays
for 14-digit arithmetic). Each GPU model, each driver, and each shader type
exhibits different behavior across these tiers. Static routing tables (e.g.,
"RTX 3090 uses DF64 for throughput") are fragile — driver updates, shader
complexity, and transcendental function support all change the picture.

We demonstrate a **self-routing precision brain** that:

1. **Probes** the GPU at startup with minimal test shaders (arithmetic and
   transcendental) across all four precision tiers (F32, F64, F64Precise, DF64)
2. **Records** which tiers compile, dispatch, and produce correct results
3. **Routes** physics workloads to the best available tier based on domain
   requirements (precision floor, FMA sensitivity) combined with measured
   hardware capabilities
4. **Survives** a critical NVIDIA driver failure mode where DF64 compilation
   permanently poisons the wgpu device

The brain is portable — it depends only on `GpuF64` and `PrecisionTier`,
both fundamental barraCuda abstractions. Dropping it into any spring gives
that spring automatic precision routing for the hardware it's running on.

## Discovery: NVVM Device Poisoning

The RTX 3090's proprietary NVIDIA driver routes WGSL through naga → SPIR-V
→ NVVM for compilation. The NVVM compiler cannot handle f64 transcendental
functions (`exp`, `log`) in DF64 or F64Precise modes. A single failed
compilation permanently invalidates the entire wgpu device — all subsequent
buffer creation, dispatch, and readback operations fail with
`"Buffer is invalid"`.

This is not a wgpu bug or a barraCuda bug — it is a fundamental limitation
of the NVIDIA proprietary driver's NVVM backend. NVK (Mesa's open-source
NVIDIA driver) handles all tiers correctly, including DF64 transcendentals.

### Implications for Sovereign Computing

1. **Driver diversity is a security feature**: A single driver bug should
   not kill a compute pipeline. The brain's ability to detect and route
   around this failure demonstrates why sovereignty requires probing, not
   assuming.
2. **NVK is the correct long-term path**: NVK handles all precision tiers
   correctly. The proprietary driver's NVVM limitation is a barrier to
   full DF64 transcendental throughput on consumer NVIDIA GPUs.
3. **coralReef bypass**: The sovereign WGSL→native compilation path may
   bypass the NVVM issue entirely by compiling directly to SASS/GFX,
   never touching NVVM.

## Architecture

### Three-Layer Design

```
Layer 1: HardwareCalibration
  Probe each tier with minimal shaders
  Record compile/dispatch/accuracy per tier
  Infer transcendental safety from driver identity
  Output: TierCapability[4] + flags

Layer 2: PrecisionBrain
  Consume calibration data
  Build route_table[7] for all PhysicsDomains
  O(1) lookup: domain → tier
  Compile method wraps GpuF64 pipeline creation

Layer 3: Physics Pipeline
  Call brain.route(domain) → tier
  Call brain.compile(domain, source) → pipeline
  Never touch GpuF64 compilation directly
```

### Probe Order

F32 → F64 → F64Precise → DF64 (safest to riskiest). If any probe poisons
the device, subsequent probes are skipped and those tiers are marked as
unavailable.

### F64 Throttle Detection

```
ratio = f64_dispatch_us / f32_dispatch_us
if ratio > 8.0: card has 1:64 FP64:FP32 ratio (consumer)
```

When F64 is severely throttled and DF64 is available, throughput-bound
workloads route to DF64 for higher throughput.

## Results

### Heterogeneous Pair Profile

| Metric | Titan V (GV100, NVK) | RTX 3090 (GA102, proprietary) |
|--------|------|------|
| FP64 cores | 2560 | 82 |
| FP32 cores | 5120 | 10496 |
| VRAM | 12 GB HBM2 | 24 GB GDDR6X |
| PCIe | 3.0 x16 | 4.0 x16 |
| F64 | ✓ full | ✓ full |
| DF64 | ✓ full | △ arith only |
| F64Precise | ✓ full | △ arith only |
| Brain role | Precision oracle | Throughput engine |

### Dual-Card Cooperative Patterns

The two GPUs complement each other:

- **Split compute**: 3090 runs throughput work, Titan V validates precision
- **Redundant compute**: Both run identical trajectories, difference measures numerical noise
- **PCIe bridge**: 1.7 ms for 512KB roundtrip (1.2 GB/s effective)

## Connection to Constrained Evolution

The precision brain embodies the constrained evolution principle: it does
not assume what hardware can do — it probes, records, and routes. When
dropped into a new spring on unknown hardware, it discovers capabilities
and builds an optimal routing table. This is the same pattern as NVK
discovery, NPU capability probing, and the absorption cycle — sovereignty
through self-knowledge, not external authority.

## References

1. hotSpring Experiment 049: Precision Brain + Heterogeneous GPU Evaluation
2. hotSpring `barracuda/src/hardware_calibration.rs` — safe per-tier probe
3. hotSpring `barracuda/src/precision_brain.rs` — self-routing brain
4. hotSpring wateringHole handoff: `HOTSPRING_V0625_PRECISION_BRAIN_NVVM_POISONING_HANDOFF_MAR10_2026.md`
