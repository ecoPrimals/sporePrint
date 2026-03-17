# ecoPrimals for Hardware Builders, Hobbyists, and Gamers

**From:** Kevin Mok (BS Microbiology, MSU 2018; MS Data Science, MSU 2025)
**Date:** March 17, 2026
**Repositories:** github.com/syntheticChemistry — all AGPL-3.0-or-later

---

## The Pitch

Your gaming GPU does real science. Not "citizen science" where you donate
idle cycles to someone else's project — actual f64-precision computational
physics, drug discovery, and metagenomics that you run, own, and understand.

NVIDIA throttles consumer GeForce f64 to 1:64 of f32 throughput via CUDA.
Vulkan's `SHADER_F64` exposes native f64 at 1:2 — the silicon is already there.
ecoPrimals runs on Vulkan (via WebGPU/wgpu), bypassing CUDA entirely.
Your RTX 3060 is a science chip. You just didn't know it.

---

## What Your Hardware Can Do

### GPU Science (Not Mining, Not Folding — Original Research)

| Your Card | VRAM | f64 Science Capability | Example Workload |
|-----------|:----:|----------------------|------------------|
| GTX 1060 | 6 GB | Entry — CPU+GPU diversity pipeline | 16S metagenomics, diversity indices |
| RTX 2070 Super | 8 GB | Solid — Anderson eigensolve (L=30) | Community structure physics, spectral analysis |
| RTX 3060 | 12 GB | Good — full pipeline + moderate PCoA | Drug-disease NMF, population PK (10K patients) |
| RTX 3070 / Ti | 8 GB | Good — fast compute, moderate VRAM | Hill dose-response sweep (8K compounds), ODE batch |
| RTX 3090 | 24 GB | Excellent — large Anderson (L=60+) | Production eigensolve, large NMF, streaming pipelines |
| RTX 4060 | 8 GB | Good — Ada architecture efficiency | All of the above with better power efficiency |
| RTX 4070 | 12 GB | Very good — validated reference platform | **This is what it was all built on.** 207 M/s Hill sweep |
| RTX 5090 | 32 GB | Outstanding — production scale | Anderson L=200, 10M patient Monte Carlo |
| Titan V (HBM2) | 12 GB | Research — HBM2 bandwidth advantage | Bandwidth-bound eigensolve, spectral sweeps |
| AMD RX 6950 XT | 16 GB | Good — RADV Vulkan driver | Same WGSL shaders, no code changes |
| AMD MI50 (HBM2) | 16 GB | Research — datacenter HBM2 | Large Anderson, MI50 Instinct on consumer board |

**No CUDA. No NVIDIA lock-in. WebGPU compiles WGSL → SPIR-V (Vulkan) / Metal / DX12.**
The same binary runs on NVIDIA, AMD, Intel, and Apple GPUs.

### The f64 Discovery

CUDA on consumer GeForce cards artificially limits double-precision (f64) to
1/64th of single-precision (f32) throughput. This is a driver restriction, not
a silicon limitation. The actual hardware can do f64 at 1:2 of f32.

Vulkan exposes `VK_KHR_shader_float64` on consumer cards. wgpu (the Rust WebGPU
implementation) uses this. ecoPrimals' WGSL shaders run at native f64 speed.

**Your $300 gaming card does the same math as a $10,000 datacenter card** — at
lower throughput, but with the same precision.

---

## Build Topology: How to Build a Science Cluster

### Tier 1: Solo Gaming PC (~$800–1,500)

```
Your gaming PC
├── GPU: RTX 3060+ (Vulkan f64)
├── CPU: Any modern x86_64 (Rust compiles fast)
├── RAM: 16 GB+ (32 GB recommended)
└── Storage: 500 GB NVMe (1 TB for NCBI data)
```

**What it runs**: All springs. All validation binaries. Full 16S pipeline.
Drug repurposing NMF. Population PK. GPU Anderson eigensolve to L=30.

### Tier 2: Household Cluster (2–4 nodes, ~$2,000–5,000)

```
Node 1 (your gaming PC) ──── 10G switch ──── Node 2 (spare/used PC)
                                    │
                                    └──── Node 3 (NAS/storage)
```

**How ecoPrimals handles this**: biomeOS discovers nodes via Songbird (mDNS +
BirdSong beacon). Each node announces its capabilities. toadStool routes
workloads to the best GPU/CPU/NPU. No Slurm. No PBS. No sysadmin.

**Used hardware sweet spots** (Facebook Marketplace / eBay):
- Dual EPYC 7452 workstation: ~$800–1,200 (64 cores, 256 GB ECC)
- RTX 3090 (used): ~$500–700 (24 GB VRAM, excellent for eigensolve)
- Titan V (used): ~$300–500 (12 GB HBM2, bandwidth monster)
- 10G NIC (Mellanox ConnectX-3): ~$15–25 each
- 10G switch (MikroTik CRS305): ~$130

### Tier 3: Multi-Household Mesh (covalent bonding)

```
Your house ─────── VPN ─────── Brother's house
    │                               │
    └── eastGate                    └── flockGate
    └── strandGate
    └── westGate (76 TB ZFS)
```

This is what ecoPrimals runs on today. 10 towers, ~$15K total, assembled from
used parts. The NUCLEUS bonding model calls this "covalent bonding" — nodes
trusted via shared cryptographic seed (SoloKey FIDO2).

### Tier 4: Community Mesh (ionic bonding)

```
Your cluster ──── Faculty lab ──── Another builder
                      │
                      └── ICER HPC (metallic bonding)
```

Faculty collaborators get "ionic" bond status — scoped access via contract.
University HPC (ICER, NERSC, XSEDE) is "metallic" — homogeneous, queue-based,
delocalized compute. All three coexist.

---

## Neuromorphic Edge: BrainChip AKD1000

If you're into edge computing, the AKD1000 is a PCIe neuromorphic chip:

| Spec | Value |
|------|-------|
| Inference latency | 48.7 µs mean |
| Power | ~1.4 µJ per inference |
| Battery life (CR2032) | ~11 years at 1 Hz |
| Throughput | 18,800 inferences/sec |
| Interface | PCIe (M.2 or full-size) |
| Driver | Pure Rust (toadStool `akida-driver`) |
| Price (eval board) | ~$200 |

**What it does in ecoPrimals**: Real-time classification of soil microbiome
health, bloom detection, agricultural IoT. The ESN (Echo State Network) runs on
the NPU; the Rust driver is sovereign (no vendor SDK required beyond initial
weight programming).

**Hobbyist relevance**: Building a custom AKD1000 HAT for Raspberry Pi is
comparable complexity to any PCIe HAT project. The software stack (Rust driver)
is the hard part — and it's done.

---

## The Distributed Compute Argument

### What Folding@Home Proved

Folding@Home peaked at 2.4 exaFLOPS during COVID-19. That's ~200K volunteer
nodes donating idle GPU cycles to protein folding simulations.

### What Games@Home Could Be

Paper 19 in the ecoPrimals whitePaper argues that gameplay itself is a
distributed computation engine:

| Metric | Folding@Home | Games@Home (theoretical) |
|--------|:------------:|:------------------------:|
| Compute units | ~200K volunteer PCs | ~40M MTG players (brains) |
| Cost per unit | Free (volunteers donate) | Free (they want to play) |
| Search space | Protein conformational | Game decision tree (infinite) |
| Novelty per trajectory | 0.00 (stochastic MD) | 0.85 (human creativity) |

The provenance trio (rhizoCrypt + loamSpine + sweetGrass) tracks every game
session as a DAG — the same infrastructure that tracks scientific samples and
clinical records. ludoSpring validates this with 75 experiments and 1,692 checks.

### The Latent Compute Numbers

| Platform | Total Raw Compute (TFLOPS) |
|----------|:--------------------------:|
| All cloud providers combined | ~24,000,000 |
| All consumer GPUs worldwide | ~5,500,000,000 |

At a conservative 2.5% participation rate, citizen hardware provides
5–6× the entire centralized cloud. ecoPrimals is built to run on this.

---

## What Makes This Different From Mining / BOINC

| Feature | Crypto Mining | BOINC / Folding@Home | ecoPrimals |
|---------|:------------:|:-------------------:|:----------:|
| You understand the science | No | Usually no | **Yes** — validation binaries explain themselves |
| You own the results | No | No | **Yes** — AGPL, your hardware, your data |
| You choose the workload | No (hash function) | Somewhat (project selection) | **Yes** — pick a spring, pick an experiment |
| GPU vendor lock-in | Yes (CUDA for mining) | Mostly yes (CUDA) | **No** — WGSL/Vulkan, any vendor |
| Skill development | Minimal | Minimal | **Real** — Rust, GPU programming, scientific computing |
| Publishable output | No | Contributor credit | **Yes** — full attribution via sweetGrass provenance |

---

## Quick Start for Builders

```bash
# 1. Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 2. Clone
git clone git@github.com:syntheticChemistry/barraCuda.git
git clone git@github.com:syntheticChemistry/wetSpring.git

# 3. Build everything
cd wetSpring/barracuda && cargo build --release

# 4. Run GPU validation (requires Vulkan)
cargo run --release --bin validate_barracuda_gpu_v8

# 5. Run a benchmark against Python
cargo run --release --bin benchmark_python_vs_rust_v5

# What you need: Linux (Ubuntu/Fedora/Arch), Vulkan drivers, any GPU.
# What you don't need: CUDA, Python, Docker, cloud account, license key.
```

### Verify Your GPU's f64 Capability

```bash
# Check Vulkan f64 support
vulkaninfo | grep shaderFloat64
# Should show: shaderFloat64 = VK_TRUE

# Run the GPU diagnostic
cargo run --release --bin validate_nouveau_diagnostic_v1
```

---

## Hardware Acquisition Strategy (Budget Science Cluster)

| Component | Where to Buy | Budget | Notes |
|-----------|-------------|:------:|-------|
| RTX 3090 (used) | eBay, FB Marketplace | $500–700 | Best VRAM/dollar for science |
| Titan V (used) | eBay | $300–500 | HBM2, bandwidth-bound workloads |
| Dual EPYC workstation | FB Marketplace, surplus | $800–1,200 | 64 cores, 256 GB ECC |
| 10G NIC (Mellanox CX-3) | eBay | $15–25 | Dirt cheap, rock solid |
| 10G switch (MikroTik) | Amazon | ~$130 | 4-port + 1 SFP+ uplink |
| ZFS storage (used drives) | eBay | $10–20/TB | Redundant, checksummed |
| BrainChip AKD1000 | BrainChip store | ~$200 | PCIe neuromorphic |
| SoloKey FIDO2 | SoloKeys.com | ~$30 | Hardware security for NUCLEUS |

**Total for a serious cluster: $2,000–4,000.** That's a rounding error compared
to a $50K ICER node or $30K/year in cloud bills.

---

## Community

All repositories are public under AGPL-3.0. Clone, build, verify, extend.
Every claim has a validation binary. The science is in the code.

github.com/syntheticChemistry
