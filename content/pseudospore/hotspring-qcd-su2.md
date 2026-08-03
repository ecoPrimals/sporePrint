+++
title = "pseudoSpore: hotSpring QCD — SU(2) Lattice Gauge Theory"
description = "SU(2) lattice gauge trajectories computed on RTX 3090 + RX 6950 XT. Multi-vendor, DF64 precision, full provenance, downloadable and verifiable. Rung 1 toward lattice QCD."
date = 2026-08-01
weight = 5

[taxonomies]
primals = ["barracuda", "toadstool", "coralreef"]
springs = ["hotspring"]

[extra]
maturity = "live"
validated_on_hardware = true
+++

> **Computed on live hardware** — strandGate RTX 3090 + RX 6950 XT, Dual EPYC 7452.
> Every trajectory has full CAS + Provenance Trio coverage.
> arXiv draft: [Preprint under refinement — validation experiment queue in progress](/pseudospore/hotspring-qcd-su2-paper/)

This pseudoSpore is different from the data catalog. The catalog shows
**ingested** reference data (ChEMBL, PDB, LINCS). This shows **computed**
data — original SU(2) lattice gauge theory trajectories generated on
sovereign hardware. Rung 1 toward vendor-agnostic lattice QCD.

The system doesn't just store science. It produces science.

---

## What Was Computed

SU(2) gauge theory HMC (Hybrid Monte Carlo) trajectories. Wilson gauge action,
Omelyan 2MN integrator, Metropolis accept/reject. The standard lattice gauge
theory algorithm, applied here with the SU(2) gauge group as the first rung
toward SU(3) and full QCD.

The computation ran through the hotSpring validation pipeline:

```
hotSpring (physics domain)
  → barraCuda (GPU math — WGSL shaders)
    → coralReef (shader compilation — WGSL → SPIR-V via naga)
      → Vulkan driver (SPIR-V → native GPU ISA)
        → toadStool (hardware dispatch — RTX 3090 + RX 6950 XT)
```

DF64 precision (double-float emulation on FP32 cores) for physics accuracy.
~14 significant digits per operation, ~9 digits for accumulated observables —
validated against f64 reference implementations.

**Note on momentum generation**: HMC momentum is generated on CPU (`cpu_mom`
workaround) due to a GPU PRNG polyfill bias discovered during plaquette
validation. Gauge updates and force computations remain fully GPU-accelerated.
The PRNG bias has been root-caused and a GPU-native fix is in development.

---

## Lattice Scaling Results

All measured on strandGate (Dual EPYC 7452, 128 threads). Both GPUs tested
with identical WGSL shaders, same algorithm, `cpu_mom` validated path:

| Lattice | Volume | RTX 3090 ms/traj | RX 6950 XT ms/traj | CPU ms/traj | Best Speedup |
|---------|--------|------------------|--------------------|-----------|----|
| 4^4 | 256 | 17.2 | 7.4 | 185.0 | **25.1x** |
| 8^4 | 4,096 | 62.9 | 15.6 | 2,965.8 | **190.0x** |

GPU-vs-CPU comparisons on identical hardware running identical algorithms
(Omelyan 2MN integrator, n_md=20, dt=0.02). The RX 6950 XT achieves higher
throughput than the RTX 3090 at these volumes — likely due to RDNA2 compute
unit scheduling for the workgroup dispatch pattern used in lattice kernels.

**Cross-GPU agreement**: Both GPUs produce identical plaquette values to
within DF64 accumulated precision (|Δ|_GPU-GPU = 3.1×10⁻⁹ at 8⁴, five
orders of magnitude below statistical error).

## Plaquette Validation

Production HMC: 200 thermalization + 200 production trajectories at β=2.3.

| Lattice | β | ⟨P⟩ (GPU, cpu_mom) | ⟨P⟩ (f64 CPU) | |Δ|/σ | Accept |
|---------|---|---------------------|----------------|-------|--------|
| 4^4 | 2.3 | 0.15023811 ± 5.08e-4 | 0.15067734 ± 5.27e-4 | 0.60 | 100% |
| 8^4 | 2.3 | 0.15092764 ± 1.12e-4 | 0.15105782 ± 1.14e-4 | 0.82 | 99.5% |

|Δ|/σ < 1 demonstrates GPU molecular dynamics produces statistically
identical physics to the CPU reference implementation.

## DF64 Precision

DF64 plaquette computation validated against native f64 CPU reference
on identical lattice configurations:

| Configuration | ⟨P⟩ CPU (f64) | ⟨P⟩ GPU (DF64) | |Δ| | Relative Error |
|---------------|---------------|----------------|-----|----------------|
| Cold start (U=I, 4^4) | 1.000000000000000 | 1.000000000000000 | 0 | 0 |
| Hot start (4^4, seed=42) | 0.069413282606898 | 0.069413282772277 | 1.65e-10 | 2.4e-9 |
| Thermalized (4^4, 200 HMC) | 0.154412193829055 | 0.154412194382328 | 5.53e-10 | 3.6e-9 |

~9 significant digits for accumulated observables (plaquette sums over
6×256 = 1,536 oriented plaquettes). Per-operation DF64 preserves ~14
digits; the reduction is consistent with error propagation in floating-point
summation over O(10³) terms. Both precision levels exceed Monte Carlo
statistical uncertainties by orders of magnitude.

---

## Shader Pipeline

The computation uses custom WGSL compute shaders compiled by coralReef:

```
WGSL source (gauge_update, df64_leapfrog)
  → coralReef naga parser
    → SPIR-V intermediate
      → PTX (NVIDIA) / RDNA IL (AMD)
        → GPU dispatch via wgpu/Vulkan 1.4
```

No CUDA. No ROCm. No vendor SDK. Designed to support any GPU with
Vulkan 1.2+ support. Physics validation presently covers NVIDIA (RTX 3090)
and AMD (RX 6950 XT). Intel Xe support is architecturally present but
not yet physics-validated.

---

## What the pseudoSpore Contains

```
pseudospore-hotspring-qcd-su2/
├── trajectories/              # Raw HMC trajectory data
│   ├── lattice_4x4x4x4/      # 4⁴ production run
│   └── lattice_8x8x8x8/      # 8⁴ production run
├── benchmarks/                # Timing data, scaling curves
│   ├── gpu_hmc_scaling.csv
│   ├── cross_gpu_validation.csv  # RTX 3090 vs RX 6950 XT
│   └── cpu_vs_gpu_comparison.csv
├── shaders/                   # The actual WGSL compute kernels
│   ├── gauge_update_df64.wgsl
│   ├── df64_leapfrog.wgsl
│   ├── plaquette_df64.wgsl
│   └── metropolis.wgsl
├── provenance/                # Full chain for every output
│   ├── blake3_checksums.txt
│   ├── cas_manifest.json
│   ├── dag_proof.json
│   ├── spine_entry.json
│   ├── ed25519_signature.json
│   └── attribution_braid.json
├── hardware/                  # Silicon deism evidence
│   ├── gpu_profile_rtx3090.json   # SM86, 24 GB VRAM
│   ├── gpu_profile_rx6950xt.json  # RDNA2, 16 GB VRAM
│   └── gate_identity.json         # strandGate, Dual EPYC 7452
├── validate.sh
└── README.md
```

The `hardware/` directory records exactly which silicon produced the results —
both GPU architectures are profiled with model, VRAM, and Vulkan driver version.
Cross-GPU validation data in `benchmarks/` proves hardware independence.

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

This SU(2) lattice gauge theory computation ran on consumer GPUs — both
NVIDIA and AMD — in a basement. The same WGSL shaders, different silicon,
identical physics. Here are the trajectories. Here's the provenance chain
proving every byte. Here are the shaders. Download it, verify it,
reproduce it on your own hardware.

No AWS bill. No CUDA license. No vendor lock-in. WGSL shaders
compiled by coralReef, dispatched by toadStool, computed by barraCuda,
stored by nestGate, proven by the Provenance Trio. On consumer GPUs.

arXiv preprint: under refinement (hep-lat, cross-list cs.DC).
Validation experiment queue in progress.

---

## arXiv Status {#arxiv-status}

**Status**: Preprint under refinement. Validation experiment queue in progress.

| Section | Data | Validation |
|---------|------|-----------|
| 1. Introduction + Scope | Written | — |
| 2. Method (gauge theory, DF64, shaders, provenance) | Written | — |
| 3.1 Lattice scaling (RTX 3090 + RX 6950 XT) | Data in | — |
| 3.2 Plaquette values | Data in | **Normalization under review** |
| 3.3 DF64 precision validation | Data in | — |
| 3.4 Multi-vendor results | Data in | — |
| 3.5 Autocorrelation | Data in | Needs more statistics |
| 4. Discussion | Written | Experiment queue pending |
| 5. Reproducibility | Written | pseudoSpore not yet frozen |
| 6. Conclusion | Written | — |

**Blocking**: Plaquette normalization verification, β-scan, HMC diagnostics.
See [experiment queue](/pseudospore/hotspring-qcd-su2-paper/) for full list.

---

## See Also

- [pseudoSpore Catalog](@/pseudospore/_index.md) — all available pseudoSpores
- [Verify a pseudoSpore](@/pseudospore/verify.md) — step-by-step verification
- [GPU Compute — Live Evidence](@/lab/gpu-compute-live.md) — full benchmark data
- [Lattice QCD on Consumer GPUs](@/products/lattice_qcd.md) — product page
- [hotSpring Hub](@/lab/springs/hotspring.md) — physics validation domain
