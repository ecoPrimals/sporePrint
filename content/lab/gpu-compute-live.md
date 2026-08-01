+++
title = "GPU Compute — Live Evidence"
description = "Measured GPU compute performance on commodity hardware. DF64 precision, matmul throughput, vendor-agnostic via WebGPU/WGSL."
date = 2026-08-01
weight = 3

[extra]
maturity = "live"
validated_on_hardware = true
+++

> **Validated on live hardware** — strandGate RTX 3090, westGate RTX 4070.
> All numbers are measured, not theoretical.

## Measured Performance (strandGate — RTX 3090)

| Benchmark | Result | Method |
|-----------|--------|--------|
| DF64 matmul | 2,130 ops/sec | 512×512, measured via `barraCuda.matmul` |
| DF64 precision | ~14 significant digits | Double-float emulation on FP32 ALUs |
| barraCuda capabilities | 98 methods LIVE | GPU compute, linear algebra, FFT, SVD, ML |
| Shader language | WGSL (WebGPU) | No CUDA dependency. Runs on any Vulkan GPU |

## What DF64 Is

DF64 (Double-Float 64) emulates f64 precision using pairs of f32 values.
Consumer GPUs throttle native f64 to 1/32 or 1/64 of their FP32 rate.
DF64 bypasses this by running two FP32 operations per logical f64 operation,
achieving ~14 significant digits of precision at FP32 throughput rates.

**Trade-off**: DF64 uses 2x the FP32 ALU bandwidth per operation. The precision
is real (verified against f64 reference). The throughput is lower than native
f64 on datacenter GPUs (A100, H100) that have full-rate FP64 units. The advantage
is running on $500 consumer hardware instead of $15,000 datacenter cards.

## Lattice QCD — GPU vs CPU (strandGate RTX 3090)

Real SU(2) HMC (Hybrid Monte Carlo) lattice gauge theory trajectories. Same algorithm,
same machine, GPU dispatch via barraCuda vs CPU dispatch via toadStool:

| Lattice | Volume | GPU ms/traj | CPU ms/traj | Speedup | Accept Rate |
|---------|--------|-------------|-------------|---------|-------------|
| 4^4 | 256 | 9.4 | 93.1 | 9.9x | 19-20/20 |
| 8^4 | 4,096 | 25.8 | 1,490.7 | **57.8x** | 20/20 |
| 8^3 x 4 | 2,048 | 14.8 | 751.8 | 50.8x | 20/20 |
| 16^3 x 4 | 16,384 | 150.4 | 5,985.9 | 39.8x | 5/5 |
| 16^3 x 8 | 32,768 | 316.4 | 11,959.7 | 37.8x | 5/5 |
| **16^4** | **65,536** | **625.9** | **24,007.7** | **38.4x** | 5/5 |

Production rate: **5,500 trajectories/hour** sustained on a single RTX 3090.
Thermalization: 1,000 trajectories in ~10 minutes. 10,000-traj run in ~1.7 hours.
Shader pipeline: WGSL → coralReef compilation → PTX (sm_86). DF64 precision.

These are GPU-vs-CPU comparisons on identical hardware running identical algorithms —
the speedup reflects GPU parallelism on lattice site updates.

Download the full trajectory data + provenance chain:
[hotSpring QCD pseudoSpore](@/pseudospore/hotspring-qcd-su2.md)

## GPU Workloads Running in Production

| Workload | Status | Gate |
|----------|--------|------|
| Lattice QCD (HMC trajectories) | LIVE — measured above | strandGate |
| Matrix multiply (dense) | LIVE | strandGate, westGate |
| SVD decomposition | LIVE | strandGate |
| FFT (1D, 2D) | LIVE | strandGate |
| AlphaFold MSA scoring | Capacity assessed | strandGate |
| Neuromorphic (Akida) | VFIO passthrough | westGate |

## Vendor Independence

barraCuda compute runs on any GPU with Vulkan 1.2+ support:

- NVIDIA (tested: RTX 3090, RTX 4070)
- AMD (tested: consumer Radeon via Vulkan)
- Intel Arc (supported via Vulkan, not yet fleet-tested)

No CUDA. No ROCm. No vendor SDK. Pure WGSL shaders dispatched through
the WebGPU API.

## Pending: Live Benchmarks

This page currently shows static measurements. When petalTongue G19
Node Atomics rendering is complete, it will serve real-time benchmark
results from `barraCuda` via `spore-validate pt-render`.

Data source: `spore-validate nucleus strandGate --probe`
