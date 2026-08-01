+++
title = "pseudoSpore: hotSpring QCD — SU(2) Lattice Gauge Theory"
description = "Lattice QCD trajectories computed on a single RTX 3090. DF64 precision, full provenance, downloadable and verifiable. The system produces science, not just stores it."
date = 2026-08-01
weight = 5

[taxonomies]
primals = ["barracuda", "toadstool", "coralreef"]
springs = ["hotspring"]

[extra]
maturity = "live"
validated_on_hardware = true
+++

> **Computed on live hardware** — strandGate RTX 3090, Dual EPYC 7452.
> Every trajectory has full CAS + Provenance Trio coverage.

This pseudoSpore is different from the data catalog. The catalog shows
**ingested** reference data (ChEMBL, PDB, LINCS). This shows **computed**
data — original lattice QCD trajectories generated on sovereign hardware.

The system doesn't just store science. It produces science.

---

## What Was Computed

SU(2) gauge theory HMC (Hybrid Monte Carlo) trajectories. Wilson gauge action,
leapfrog integrator, Metropolis accept/reject. The standard lattice gauge
theory algorithm used in production QCD research worldwide.

The computation ran through the hotSpring validation pipeline:

```
hotSpring (physics domain)
  → barraCuda (GPU math — WGSL shaders)
    → coralReef (shader compilation — WGSL → PTX)
      → toadStool (hardware dispatch — RTX 3090, SM86, 24 GB VRAM)
```

DF64 precision (double-float emulation on FP32 cores) for physics accuracy.
~14 significant digits — validated against f64 reference implementations.

---

## Lattice Scaling Results

All measured on strandGate RTX 3090. Same algorithm, same machine,
GPU dispatch via barraCuda vs CPU dispatch via toadStool:

| Lattice | Volume | GPU ms/traj | CPU ms/traj | Speedup | Accept Rate |
|---------|--------|-------------|-------------|---------|-------------|
| 4^4 | 256 | 9.4 | 93.1 | 9.9x | 19-20/20 |
| 8^4 | 4,096 | 25.8 | 1,490.7 | **57.8x** | 20/20 |
| 8^3 x 4 | 2,048 | 14.8 | 751.8 | 50.8x | 20/20 |
| 16^3 x 4 | 16,384 | 150.4 | 5,985.9 | 39.8x | 5/5 |
| 16^3 x 8 | 32,768 | 316.4 | 11,959.7 | 37.8x | 5/5 |
| **16^4** | **65,536** | **625.9** | **24,007.7** | **38.4x** | 5/5 |

These are GPU-vs-CPU comparisons on identical hardware running identical algorithms.
The speedup reflects GPU parallelism on lattice site updates.

**Production rate**: 5,500 trajectories/hour sustained on a single RTX 3090.
**Thermalization**: 1,000 trajectories in ~10 minutes.
**Production run**: 10,000 trajectories in ~1.7 hours.

---

## Shader Pipeline

The computation uses custom WGSL compute shaders compiled by coralReef:

```
WGSL source (gauge_update, df64_leapfrog)
  → coralReef naga parser
    → SPIR-V intermediate
      → PTX (sm_86 for RTX 3090)
        → GPU dispatch via wgpu/Vulkan
```

No CUDA. No ROCm. No vendor SDK. The same shaders run on any GPU with
Vulkan 1.2+ support. Tested on NVIDIA (RTX 3090, 4060, 5090) and
AMD (RX 6950 XT).

---

## What the pseudoSpore Contains

```
pseudospore-hotspring-qcd-su2/
├── trajectories/              # Raw HMC trajectory data
│   ├── lattice_8x8x8x8/      # 8⁴ production run
│   └── lattice_16x16x16x16/  # 16⁴ production run
├── benchmarks/                # Timing data, scaling curves
│   ├── gpu_hmc_scaling.csv
│   └── cpu_vs_gpu_comparison.csv
├── shaders/                   # The actual WGSL compute kernels
│   ├── gauge_update_f32.wgsl
│   └── df64_leapfrog.wgsl
├── provenance/                # Full chain for every output
│   ├── blake3_checksums.txt
│   ├── cas_manifest.json
│   ├── dag_proof.json
│   ├── spine_entry.json
│   ├── ed25519_signature.json
│   └── attribution_braid.json
├── hardware/                  # Silicon deism evidence
│   ├── gpu_profile.json       # RTX 3090, SM86, 24 GB VRAM
│   └── gate_identity.json     # strandGate, Dual EPYC 7452
├── validate.sh
└── README.md
```

The `hardware/` directory is unique to compute pseudoSpores — it records
exactly which silicon produced the results. `gpu_profile.json` includes
GPU model, SM architecture, VRAM, and Vulkan driver version.
`gate_identity.json` identifies the gate and its BTSP enrollment lineage.

---

## Verify It

```bash
tar xzf pseudospore-hotspring-qcd-su2.tar.gz
cd pseudospore-hotspring-qcd-su2/

# Check every hash
b3sum --check provenance/blake3_checksums.txt

# Full chain verification
./validate.sh

# Or reproduce: run the same HMC on your own GPU
# See: getting-started for NUCLEUS deployment
```

---

## The Point

This lattice QCD computation ran on a $1,500 GPU in a basement.
Here are the trajectories. Here's the provenance chain proving every byte.
Here are the shaders that ran on the GPU. Download it, verify it,
reproduce it on your own hardware.

No AWS bill. No CUDA license. No vendor lock-in. Pure Rust shaders
compiled by coralReef, dispatched by toadStool, computed by barraCuda,
stored by nestGate, proven by the Provenance Trio. On a consumer GPU.

---

## See Also

- [pseudoSpore Catalog](@/pseudospore/_index.md) — all available pseudoSpores
- [Verify a pseudoSpore](@/pseudospore/verify.md) — step-by-step verification
- [GPU Compute — Live Evidence](@/lab/gpu-compute-live.md) — full benchmark data
- [Lattice QCD on Consumer GPUs](@/products/lattice_qcd.md) — product page
- [hotSpring Hub](@/lab/springs/hotspring.md) — physics validation domain
