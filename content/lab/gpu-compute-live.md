+++
title = "GPU Compute — Live Evidence"
description = "Measured GPU compute performance on commodity hardware. DF64 precision, matmul throughput, vendor-agnostic via WebGPU/WGSL."
date = 2026-08-17
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

## Lattice QCD — Multi-Vendor GPU vs CPU (strandGate)

SU(N) HMC (Hybrid Monte Carlo) lattice gauge theory. Same algorithm, same machine,
both GPUs running identical WGSL shaders, `cpu_mom` validated path.
**32⁴ SU(3) production COMPLETE** — 45/45 cross-vendor configs.

| Lattice | Volume | RTX 3090 ms | RX 6950 XT ms | CPU ms | Best Speedup |
|---------|--------|-------------|---------------|--------|-------------|
| 4⁴ | 256 | 17.2 | 7.4 | 185.0 | **25.1×** |
| 8⁴ | 4,096 | 62.9 | 15.6 | 2,965.8 | **190.0×** |
| 16⁴ | 65,536 | — | 29.5 | — | — |
| 32⁴ | 1,048,576 | — | 521.7 | — | — |

Omelyan 2MN integrator, n_md=20, dt=0.02. `cpu_mom` path (CPU-generated
momenta, GPU molecular dynamics) after root-causing GPU PRNG polyfill bias.
Streaming HMC encoder: GPU utilization 43%→85-95%.

**32⁴ SU(3) production** (Aug 9-16, 2026): 45/45 configs across β=6.0/6.10/6.20.
⟨P⟩ at β=6.0, 16⁴: 0.5916 vs NS02 0.5935 (**0.3% agreement**).
Cross-vendor at β=6.20, 32⁴: **0.19% delta** (AMD vs NVIDIA).
Lattice capacity: **73⁴ dual GPU** (121× more sites via software guard bypass).

**Cross-GPU agreement**: Both GPUs produce identical plaquette values within
DF64 accumulated precision (|Δ|_GPU-GPU = 3.1×10⁻⁹ at 8⁴ — five orders
of magnitude below statistical error). Vendor-agnostic proof: same WGSL
shaders, different silicon, identical physics.

Download the full trajectory data + provenance chain:
[hotSpring QCD pseudoSpore](@/pseudospore/hotspring-qcd-sun.md)

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
