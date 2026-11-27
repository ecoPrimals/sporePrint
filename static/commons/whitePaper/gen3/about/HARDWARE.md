# Basement HPC — Node Inventory

**Total Investment**: ~$15,000
**Location**: Basement, Lansing MI area
**Network**: Local ethernet mesh with 10 Gigabit backbone (switch acquired, NICs installed, cables pending)
**Last Updated**: March 16, 2026

---

## Overview

The ecoPrimals ecosystem runs on a heterogeneous cluster of physical machines,
purchased and assembled over several years. westGate was originally built during
Army service (2012–2015). southGate was a gaming tower upgraded for the mesh.
northGate was all new parts. Everything else was Facebook Marketplace finds and
Newegg sales.

The hardware is heterogeneous by nature — DDR3 to DDR5, RTX 2070 to RTX 5090,
32 GB to 256 GB RAM — and the architecture treats this heterogeneity as a
feature. Each gate runs the NUCLEUS atomics that match its hardware, and biomeOS
coordinates the mesh so workloads land on the right gate.

---

## Topology: Display Tier + Work Tier

The mesh has evolved from fixed "one GPU per gate" to a two-tier model:

| Tier | Purpose | Cards |
|------|---------|-------|
| **Display** | Desktop output, lightweight ML, Cursor IDE | Small GDDR cards (RTX 4060, 5060, 4070) — permanent per gate |
| **Work** | Heavy compute, HPC, HBM2 testing, science | Large swappable cards (RTX 3090, Titan V, MI50) — slot into any gate's PCIe |

Every gate is a system that can be parallelized by adding more GPUs on PCIe.
Conceptually, a PCIe x16 slot can split across multiple smaller cards (e.g.,
~8× RTX 3050 pooled via HDMI output), as toadStool is developing unidirectional
hardware data flows. The 3090s and HBM2 cards physically move between boards
based on current workload.

biomeGate is the current **HBM2 test bench** — the hotSpring team works there,
and the Titan Vs + MI50s are slotted in for sovereign HBM2 compiler development
via coralReef.

---

## Towers

### northGate — Flagship AI/LLM Compute

| Component | Specification |
|-----------|---------------|
| **CPU** | Intel i9-14900K |
| **Display GPU** | — (RTX 5090 serves both roles) |
| **Work GPU** | NVIDIA RTX 5090 |
| **RAM** | 192 GB DDR5 |
| **Storage** | 1 TB Samsung 990 Pro + 4 TB Samsung 990 EVO |
| **Network** | 10G NIC (copper) |
| **Role** | Primary AI/LLM compute, main hub |

Heavy Node Atomic. Where Squirrel routes large inference jobs and toadStool runs
GPU-intensive barraCuda workloads. The RTX 5090 is the most capable single GPU
in the mesh.

### southGate — Gaming + Heavy Compute

| Component | Specification |
|-----------|---------------|
| **CPU** | AMD Ryzen 5800X3D |
| **Display GPU** | NVIDIA RTX 4060 |
| **RAM** | 128 GB DDR4 |
| **Storage** | 1 TB 990 EVO (Windows) + 4 TB 990 EVO (Linux) |
| **Network** | 10G NIC (copper) |
| **PCIe** | Available slots for work cards |
| **Role** | Gaming + heavy compute |

Balanced Node Atomic. Originally a gaming tower, upgraded for the mesh with
128 GB RAM. Dual boot for gaming (Windows) and compute (Linux). Work cards
(3090s) slot in as needed for compute runs.

### eastGate — Utility Compute + Neuromorphic

| Component | Specification |
|-----------|---------------|
| **CPU** | Intel i9-12900 |
| **Display GPU** | NVIDIA RTX 4070 |
| **NPU** | 1× BrainChip Akida AKD1000 |
| **RAM** | 32 GB DDR5-4800 |
| **Storage** | 2 TB 990 EVO Plus |
| **Network** | 10G NIC (copper) |
| **Role** | Utility compute, neuromorphic workloads |

Node Atomic with neuromorphic capability. The Akida AKD1000 NPU handles
toadStool's neuromorphic inference workloads alongside traditional GPU compute.
**V60 (Feb 2026):** First live AKD1000 validation via pure Rust `akida-driver` —
3 ESN classifiers (QS/Bloom/Disorder) at 18.8K Hz, online (1+1)-ES evolution at
136 gen/sec, 12.9K Hz temporal streaming (p99=76 µs), PUF fingerprint (6.34
bits), coin-cell 11 years. Phase C sovereign driver achieved — zero vendor SDK
dependency.

### strandGate — Bioinformatics Powerhouse

| Component | Specification |
|-----------|---------------|
| **CPU** | Dual AMD EPYC 7452 (64 cores / 128 threads total) |
| **GPU** | NVIDIA RTX 3090 + AMD RX 6950 XT |
| **NPU** | 1× BrainChip Akida (PCIe) |
| **RAM** | 256 GB ECC DDR4 |
| **Storage** | 2× 2 TB NVMe + 4× 4 TB NVMe (Hyper M.2) + HDD/SSD pool |
| **OS** | Pop!_OS |
| **Role** | Bioinformatics, multi-GPU + neuromorphic |

Heavy Node Atomic, the most capable multi-substrate compute node. The 64-core
dual EPYC with 256 GB ECC handles CPU-bound bioinformatics (Kraken2, alignment).
The RTX 3090 + RX 6950 XT provide dual-vendor GPU compute — barraCuda's WGSL
shaders run on both NVIDIA and AMD, validating the universal compute claim.

### biomeGate — Semi-Mobile Mini HPC / HBM2 Test Bench

| Component | Specification |
|-----------|---------------|
| **CPU** | AMD Threadripper 3970X (32 cores / 64 threads) |
| **Display GPU** | NVIDIA RTX 5060 |
| **HBM2 (work)** | 2× NVIDIA Titan V (12 GB HBM2 each) + 2× AMD Instinct MI50 (16 GB HBM2 each) |
| **NPU** | 1× BrainChip Akida (moved from strandGate) |
| **RAM** | 256 GB DDR4 (non-ECC) |
| **Storage** | 1 TB NVMe (boot) + 2× 2 TB NVMe (work/scratch) |
| **Role** | HBM2 sovereign compiler development, semi-mobile heavy compute |

Heavy Node Atomic and the project's **HBM2 test bench**. The hotSpring team
works here. The Threadripper 3970X provides 32 cores for CPU-parallel baselines
and verification. The RTX 5060 handles display and lightweight ML. The 4 HBM2
cards (56 GB aggregate, ~28 TFLOPS FP64) are the coralReef sovereign compiler
development targets:

| Card | Qty | HBM2 | FP64 | Arch | coralReef Target |
|------|-----|------|------|------|------------------|
| Titan V | 2 | 12 GB each | 7.4 TFLOPS | Volta SM70 | `sm_70` |
| MI50 | 2 | 16 GB each | 6.6 TFLOPS | Vega 20 GCN 5.0 | `gfx906` |

The Titan V runs NVK for driver-independent verification. The MI50s came with
3D-printed blower shrouds for standard case use. PCIe lanes are expandable —
the Threadripper's 64 lanes support the full HBM2 array plus display GPU.

### westGate — Cold Storage Archive

| Component | Specification |
|-----------|---------------|
| **CPU** | Intel i7-4771 |
| **GPU** | NVIDIA RTX 2070 Super |
| **RAM** | 32 GB DDR3 |
| **Storage** | ~76 TB HDD (ZFS) + 2 TB NVMe cache |
| **Network** | 10G NIC (copper) |
| **Role** | Cold storage, NAS, caching layer |

Heavy Nest Atomic. The oldest gate — built during Army service, still running.
The 76 TB ZFS pool is the cold storage archive for the entire mesh. nestGate on
westGate manages content-addressed blob storage with ZFS snapshots (100 snapshots
in 0.17s). DDR3 and an i7-4771 are not fast, but cold storage needs reliable and
big, not fast.

### swiftGate — Mobile Compute

| Component | Specification |
|-----------|---------------|
| **CPU** | AMD Ryzen 5800X |
| **GPU** | NVIDIA RTX 3070 FE |
| **RAM** | 64 GB DDR4 |
| **Storage** | NVMe |
| **Role** | Mobile/compact compute node |

Balanced Node Atomic. Compact form factor for deployments that need to move.

### flockGate — Flock Compute

| Component | Specification |
|-----------|---------------|
| **CPU** | Intel i9-13900K |
| **GPU** | NVIDIA RTX 3070 Ti |
| **RAM** | 64 GB DDR5 |
| **Storage** | 2 TB NVMe |
| **OS** | Ubuntu 24.04.3 |
| **Role** | Brother's node / flock compute |

Node Atomic operated by a family member. Connected to the mesh via covalent
bonding (shared family seed). Demonstrates the multi-household covalent mesh
pattern.

### kinGate — Staging / Utility

| Component | Specification |
|-----------|---------------|
| **CPU** | Intel i7-6700K |
| **GPU** | NVIDIA RTX 3070 |
| **RAM** | 32 GB DDR4 (2×16 @ ~3400) |
| **Storage** | SATA / NVMe (PCIe 3.0) |
| **OS** | Pop!_OS |
| **Role** | Staging / utility compute |

Light Node Atomic. Staging and testing node for primal deployments before
promotion to production gates.

### blueGate — Expansion

| Component | Specification |
|-----------|---------------|
| **CPU** | TBD |
| **GPU** | None currently |
| **RAM** | 128 GB DDR4 |
| **Storage** | 2 TB NVMe |
| **OS** | Windows |
| **Role** | General compute / expansion node |

Future expansion. Windows-primary for testing cross-platform ecoBin deployments.
GPU pending.

---

## Work Card Pool

Large GPUs that float between gates based on current workload. Any gate with
available PCIe slots can host work cards.

| Card | Qty | VRAM | Type | Current Location |
|------|-----|------|------|------------------|
| RTX 3090 | 3 | 24 GB GDDR6X each | GDDR work | strandGate (1 resident) + 2 swappable |
| Titan V | 2 | 12 GB HBM2 each | HBM2 work | biomeGate (HBM2 bench) |
| MI50 | 2 | 16 GB HBM2 each | HBM2 work | biomeGate (HBM2 bench) |

**Total work card pool**: 7 cards, 128 GB aggregate VRAM (72 GB GDDR + 56 GB HBM2)

---

## Small Form Factor Nodes

### GMKtec NucBox M6

| Component | Specification |
|-----------|---------------|
| **RAM** | 32 GB |
| **Role** | Compact node / edge compute |

Mini PC for lightweight NUCLEUS deployments — Tower Atomic relay, discovery
node, or edge compute.

### Intel NUC (NUC6CAY) ×3

| Component | Specification |
|-----------|---------------|
| **CPU** | Intel Celeron J3455 |
| **Role** | Lightweight relay / discovery nodes |

Three low-power NUCs for always-on Tower Atomic deployments: songBird relay,
STUN server, Dark Forest beacon nodes. These demonstrate that Tower Atomic runs
on minimal hardware — the same Pure Rust HTTPS stack that runs on the i9-14900K
runs on a Celeron.

---

## Mobile & Security Hardware

### Pixel 8a — Mobile Mesh Node

| Component | Specification |
|-----------|---------------|
| **SoC** | Google Tensor G3 |
| **OS** | Android |
| **Role** | Mobile bearDog / HSM seed carrier / mesh node |

The mobile device described in the bonding model's "HSM-Anchored Composition"
pattern. Carries the hardware seed (SoloKey authentication) and runs bearDog for
cryptographic operations. Validated with Android StrongBox HSM and
SELinux-enforced abstract socket IPC.

### SoloKeys ×4 — Hardware Security Modules

| Component | Specification |
|-----------|---------------|
| **Type** | FIDO2 / WebAuthn |
| **Interface** | USB |
| **Role** | Physical roots of trust for bearDog key hierarchy |

Four SoloKey FIDO2 devices provide the physical anchor for the genetic trust
model.

---

## Networking

| Component | Specification | Status |
|-----------|---------------|--------|
| 10G Switch | 10 Gigabit Ethernet | Acquired |
| 10G NICs (copper) ×4 | northGate, southGate, eastGate, westGate | Installed |
| 10G Cables (Cat6a / DAC) | — | Pending purchase |

The 10G backbone connects the four primary towers. Once cabled, the covalent
mesh's inter-gate bandwidth jumps from 1 Gbps to 10 Gbps — relevant for
nestGate blob replication, toadStool distributed compute, and songBird
federation traffic.

---

## Neuromorphic

| Component | Location | Count |
|-----------|----------|-------|
| BrainChip Akida AKD1000 (PCIe) | strandGate | 1 |
| BrainChip Akida AKD1000 | eastGate | 1 |
| BrainChip Akida AKD1000 | biomeGate (from strandGate) | 1 |
| **Total** | | **3 cards** |

---

## Aggregate Capabilities

| Resource | Total |
|----------|-------|
| **Towers** | 10 |
| **Small form factor** | 4 (1× NucBox + 3× Intel NUC) |
| **CPU Cores** | 130+ (across all gates) |
| **GPU VRAM (display tier)** | ~44 GB (RTX 5090 32 GB + RTX 5060 8 GB + RTX 4070 12 GB + RTX 4060 8 GB — resident per gate) |
| **GPU VRAM (work pool)** | ~128 GB (3× RTX 3090 72 GB + 2× Titan V 24 GB + 2× MI50 32 GB) |
| **GPU VRAM (other gates)** | ~36 GB (RTX 2070S 8 GB + RTX 3070 FE 8 GB + RTX 3070 Ti 8 GB + RTX 3070 8 GB + RX 6950 XT 16 GB) |
| **GPU VRAM (total)** | ~208 GB |
| **HBM2** | 56 GB (2× Titan V + 2× MI50) |
| **System RAM** | ~1.2 TB (across all gates) |
| **Storage** | ~105 TB+ (76 TB ZFS cold + NVMe hot across nodes) |
| **GPU Vendors** | NVIDIA + AMD (barraCuda universal compute validated on both) |
| **GPU Architectures** | Pascal → Turing → Ampere → Ada → Blackwell (NVIDIA) + Volta (HBM2) + Vega 20 (HBM2) + RDNA 2 (AMD) |
| **coralReef HBM2 Targets** | `sm_70` (Volta), `gfx906` (Vega 20) |
| **CPU Vendors** | Intel + AMD (i7-4771 through i9-14900K, 5800X through Threadripper 3970X through Dual EPYC 7452) |
| **Memory Types** | DDR3, DDR4, DDR4 ECC, DDR5 |
| **NPU** | 3× BrainChip Akida AKD1000 |
| **HSM** | 4× SoloKey FIDO2 + Android StrongBox |
| **Network** | 10G backbone (4 towers) + 1G mesh (remaining) |

---

## The Heterogeneity Principle

A traditional HPC deploys identical nodes and distributes identical workloads.
That's metallic bonding in the NUCLEUS model: delocalized, homogeneous,
sub-specialized.

This basement HPC is the opposite. Every gate is different. DDR3 sits next to
DDR5. A 12-year-old i7 sits next to a brand-new i9. NVIDIA sits next to AMD.
HBM2 sits next to GDDR6X. A Celeron NUC runs the same primal binaries as a
64-core EPYC. That's covalent bonding: heterogeneous hardware, united by genetic
trust, each specializing in what it does best.

The display/work tier split extends this principle. Gates are no longer defined
by their fixed GPU — they are PCIe-parallelizable systems. Work cards move to
where the science is. toadStool's unidirectional hardware data flows enable
pooling multiple cards on a single gate, and eventually across gates via the
10G backbone. The topology is fluid; the architecture handles it because the
hardware demanded it.

---

**Built from scratch. Assembled in a basement. 10 towers, 4 HBM2 cards, 3 NPUs, ~$15,000. One developer. The metal is real.**
