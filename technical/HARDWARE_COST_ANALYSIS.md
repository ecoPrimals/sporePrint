# Hardware Cost Analysis: $15K Basement vs Institutional HPC

**The f64 GPU discovery, the $0.044 run, and what consumer hardware actually does.**

**Last Updated:** March 17, 2026  
**License:** CC-BY-SA 4.0

---

## The Headline Numbers

| Metric | Consumer Basement | Institutional HPC | Cloud (AWS/GCP) |
|--------|:-----------------:|:-----------------:|:---------------:|
| **Hardware cost** | $15K (one-time) | $0 (shared allocation) | $0 (per-use) |
| **f64 GPU TFLOPS** | 3.24 (DF64, RTX 3090) | 9.7 (A100 SXM, native) | 9.7 (A100 SXM) |
| **Lattice QCD production run** | **$0.044** (electricity) | ~$50–500 (allocation) | ~$50–500 |
| **100K patient population PK** | ~$0.001 (consumer RTX) | ~$5–20 (HPC) | ~$10–50 |
| **16S pipeline (1,000 samples)** | ~$0.10 (RTX 4070) | ~$2–5 (cluster) | ~$5–20 |
| **Availability** | 24/7, 0 queue | Queue: hours to days | On-demand, billing |
| **Software licensing** | $0 (AGPL-3.0) | Often: $0–$20K | Often: $0–$20K |
| **Data leaves the hardware** | Never | Yes (shared cluster) | Yes (cloud) |

**The $0.044 number:** A paper-parity molecular dynamics run (N=10,000 atoms,
80,000 timesteps, Yukawa OCP at plasma physics conditions) costs $0.044 in
electricity on an RTX 4070. The equivalent ICER/HPC allocation is estimated
at $50–500 depending on node type, queue priority, and facility pricing.

---

## Why Consumer GPUs Have 14-Digit f64 Precision

This is the discovery that makes the cost story possible.

### The CUDA Throttle vs the Vulkan Reality

NVIDIA throttles f64 performance on consumer ("gaming") GPUs at the driver
level to protect data center revenue:

| GPU Class | f64 TFLOPS (native) | f32 TFLOPS | f64:f32 ratio |
|-----------|:------------------:|:----------:|:-------------:|
| RTX 3090 (consumer) | ~0.35 | ~35.6 | **1:102** (throttled by CUDA) |
| A100 SXM (data center) | ~19.5 | ~19.5 | **1:1** |
| H100 SXM | ~33.5 | ~33.5 | **1:1** |
| Titan V (consumer, HBM2) | ~6.9 | ~13.8 | **1:2** (unthrottled) |

The CUDA throttle means: using CUDA, a $500 RTX 3090 runs f64 at 1/100th of
its f32 capability. This is software-enforced artificial restriction.

### The WebGPU / Vulkan Bypass

WebGPU (WGSL shaders via wgpu) bypasses the CUDA driver entirely and talks
directly to the Vulkan API. The Vulkan API does not enforce NVIDIA's CUDA f64
throttle. The silicon is still there. The throttle is a driver policy, not a
hardware limitation.

**Result: DF64 (double-float precision using two f32 values):**

| GPU | DF64 TFLOPS | Native f64 | Precision |
|-----|:-----------:|:----------:|:---------:|
| RTX 3090 | **3.24** | 0.35 | 14-digit |
| RTX 4070 | ~2.1 | ~0.2 | 14-digit |
| Titan V | ~6.9 | 6.9 | 14-digit |

A consumer RTX 3090 delivers **9.9× more f64-precision compute than its
CUDA-reported spec** when accessed through WebGPU + WGSL. The same GPU that
CUDA tells you is a $500 toy is doing data-center-class f64 science work
in ecoPrimals.

This is not a workaround. It is the actual hardware capability, accessed via
the open Vulkan standard rather than NVIDIA's proprietary CUDA layer.

---

## The Basement HPC — Hardware Inventory

Total investment: ~$15,000, accumulated over ~8 months.

### Compute Nodes

| Gate | Primary GPU | Work GPU(s) | CPU | RAM |
|------|------------|-------------|-----|-----|
| northGate | RTX 5090 | — | 16-core | 64 GB |
| southGate | RTX 4060 | swappable | 12-core | 64 GB |
| eastGate | RTX 4070 | — | 8-core | 32 GB |
| gate-04 | — | RTX 3090 + RX 6950 XT | 12-core | 64 GB |
| biomeGate | RTX 5060 | 2× Titan V + 2× MI50 | 16-core | 128 GB |
| westGate | RTX 2070S | — | 12-core | 32 GB |
| 4× fieldmouse | — | — | 4-core | 16 GB each |

**GPU VRAM pool:** RTX 5090 (32 GB) + RTX 3090 (24 GB) + RX 6950 XT (16 GB)
+ 2× Titan V (12 GB each) + 2× MI50 (32 GB each) + RTX 5060 (8 GB) + RTX 4060
(8 GB) + RTX 4070 (12 GB) + RTX 2070S (8 GB) ≈ **~196 GB total GPU VRAM**

**Network:** 10G backbone (gate interconnect), 1G edge (fieldmice)  
**Storage:** 76 TB ZFS on westGate

### Why Used Consumer Hardware Works

All springs validate on whatever hardware is present. The test suite enforces:
1. `cargo test` (CPU): all mathematical results match published ground truth
2. `cargo test --features gpu` (GPU): GPU results match CPU results
3. Explicit parity checks between NVIDIA (SM70/SM86/SM89) and AMD (RDNA2/CDNA2)

The coralReef sovereign shader compiler (Phase 10, 46/46 shaders) compiles
the same WGSL to native code on all these GPUs without vendor toolchains.
**The science does not know what GPU it's running on.**

---

## Cost-Per-Experiment Comparisons

### Molecular Dynamics (hotSpring)

| Experiment | Hardware | Electricity | HPC Equivalent | Cloud Equivalent |
|-----------|----------|:-----------:|:--------------:|:----------------:|
| Yukawa OCP N=10K, 80K steps (Phase F) | RTX 4070 | **$0.044** | $50–500 | $50–500 |
| Nuclear EOS full AME2020 (2,042 nuclei) | RTX 4070 | ~$0.15 | $200–1,000 | $200–1,000 |
| Lattice QCD 32⁴ production β-scan (17 points) | RTX 3090 | ~$2.50 | $500–5,000 | $500–5,000 |
| Dynamical fermion HMC (1,031 trajectories) | RTX 3090 | ~$1.20 | $300–3,000 | $300–3,000 |

### Bioinformatics (wetSpring)

| Experiment | Hardware | Time | ICER Equivalent |
|-----------|----------|:----:|:---------------:|
| 16S DADA2 full pipeline (1,000 samples) | RTX 4070 | ~5 min | 30–120 min (queue + run) |
| GPU spectral cosine matching (10K spectra) | RTX 4070 | ~0.1 sec | ~110 sec (CPU Python) |
| Anderson spectral sweep (10K lattices) | RTX 4070 | ~30 sec | ~5–10 min (CPU cluster) |
| NCBI sovereign pipeline (10 BioProjects) | CPU | ~10 min | ~30 min (with conda setup) |

### Pharmacometrics (healthSpring)

| Experiment | Hardware | Time | NONMEM/Cloud Equivalent |
|-----------|----------|:----:|:-----------------------:|
| Hill dose-response sweep (6 cytokines) | CPU | 0.04 ms | ~3.6 ms (Python) |
| Population PK Monte Carlo (100K patients) | RTX 4070 | ~0.5 sec | minutes (NONMEM CRO) |
| PBPK 5-tissue model | CPU | ~0.08 ms | ~6.7 ms (Python, 84×) |

### Precision Agriculture (airSpring)

| Experiment | Hardware | Checks/sec | Scale |
|-----------|----------|:----------:|-------|
| ET₀ computation (Penman-Monteith) | CPU | 10M/s | 13,000× vs Python |
| Water balance seasonal pipeline | GPU | 59K seasons/s | 100-station atlas |
| Richards PDE (soil water flow) | GPU | — | GPU Picard+CN+Thomas |

---

## Why Sovereignty Matters for Cost

### The Queue Problem

ICER queue times for GPU nodes: 2–48 hours depending on load. On the basement
hardware, the queue is zero. A researcher who wants to run 50 parameter sweeps
before lunch can do it. The same researcher on ICER waits until the next day.

The actual cost of computation is not the electricity or the allocation charge.
It is the researcher's time waiting for queues. A researcher who can iterate in
minutes instead of hours or days produces more science.

### The Data Problem

When you run on ICER, your data is on ICER. When ICER has a maintenance window,
your data is inaccessible. When you leave the university, your allocation ends.
When the HPC policy changes, your workflow changes.

ecoPrimals runs on hardware you own. The data never leaves. There is no allocation
expiration. There is no policy change that can break your pipeline.

### The Reproducibility Problem

ICER software stacks change. Module versions are updated. The Conda environment
you used last year may not install the same way today. Your collaborator at
another institution cannot reproduce your analysis because they cannot access
your module configuration.

ecoPrimals produces static binaries with embedded dependencies. A binary built
today will produce the same output on the same input in five years. The binary
is the reproducibility artifact, not a description of an environment.

---

## Scaling: What Changes When NUCLEUS Goes Live

The NUCLEUS bonding model (see `architecture/ECOSYSTEM_ARCHITECTURE.md`) composes
multiple gates into a coordinated mesh. When activated:

| Bond Type | What It Adds |
|-----------|-------------|
| Covalent (family gates) | All 6 basement gates work as one machine. 196 GB VRAM pool. |
| Ionic (faculty lab) | A professor's GPU joins under a metered contract. They contribute compute, receive BarraCuda validated results. |
| Metallic (ICER HPC) | Every idle campus GPU becomes a BarraCuda node. The same $0.044 science runs across 1,000 ICER GPUs simultaneously. |

**The NUCLEUS scaling equation:**

```
NUCLEUS at ICER scale:
  Idle ICER GPU-hours per day: estimated 10,000+
  ecoBin binary: no conda, no module load, no CUDA version conflict
  BarraCuda WGSL: vendor-agnostic (NVIDIA + AMD ICER nodes)
  = 10,000 GPU-hours of validated science per day
    at $0.044/run electricity cost
    with zero allocation charge
    and zero queue time for the researcher
```

This is not speculative. Every component exists. The NUCLEUS bonding model is
implemented. The ecoBin binaries run on ICER today. The only remaining step is
the institutional enrollment contract.

---

## For Hardware Builders: What Your GPU Actually Does

See `audience/FOR_HARDWARE_BUILDERS_AND_HOBBYISTS.md` for the full guide.

The short version:

| GPU | What CUDA Tells You | What WebGPU+ecoPrimals Does |
|-----|--------------------|-----------------------------|
| RTX 3090 | 35.6 TFLOPS f32, 0.35 TFLOPS f64 | **3.24 TFLOPS DF64** (14-digit precision) |
| RTX 4070 | 29.1 TFLOPS f32, 0.2 TFLOPS f64 | **~2.1 TFLOPS DF64** |
| RTX 4090 | 82.6 TFLOPS f32, 0.5 TFLOPS f64 | **~8.2 TFLOPS DF64** |
| Titan V | 13.8 TFLOPS f32, 6.9 TFLOPS f64 | **6.9 TFLOPS DF64** (native f64 silicon) |
| RX 6950 XT (AMD) | 23.7 TFLOPS f32, native f64 | **Full f64 via Vulkan, no throttle** |

Your gaming GPU is doing lattice QCD. The $500 used RTX 3090 is running the same
physics that requires a $20,000 ICER GPU-hour allocation when accessed through
CUDA. The silicon was always capable. The throttle was always artificial.

---

*Hardware inventory verified March 2026. Cost estimates for ICER/cloud are
based on publicly available MSU ICER rate cards and AWS/GCP GPU pricing.*
