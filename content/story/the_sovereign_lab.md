+++
title = "The Sovereign Lab"
description = "10 towers. 130 cores. 188 GB VRAM. 125 TB storage. $15K. No cloud."
date = 2026-07-08
weight = 20

[taxonomies]
primals = []
springs = []
+++

*10 towers. 130 cores. 188 GB VRAM. 125 TB storage. $15K. No cloud.*

---

## Hardware

### The Cluster

| Node | CPU | GPU | RAM | NVMe | Role |
|------|-----|-----|-----|------|------|
| northGate | i9-14900K | RTX 5090 (32 GB) | 192 GB DDR5 | ~8 TB | Flagship compute |
| ironGate | i9-14900K | RTX 4070 (12 GB) | 96 GB DDR5 | 3.6 TB | NUCLEUS deploy, validation |
| southGate | Ryzen 5800X3D | RTX 4060 (8 GB) + swappable | 128 GB DDR4 | ~5 TB | Heavy compute |
| strandGate | Dual EPYC 7452 (64c) | RTX 3090 + RX 6950 XT | 256 GB ECC | ~20 TB | Bioinformatics, multi-GPU |
| biomeGate | TR 3970X (32c) | RTX 5060 + Titan V + K80 | 256 GB DDR4 | ~5 TB | HBM2 test bench |
| eastGate | i9-12900 | RTX 4070 + Akida NPU | 32 GB DDR5 | 2 TB | Utility, neuromorphic |
| westGate | i7-4771 | RTX 2070 Super | 32 GB DDR3 | 2 TB | Cold storage / NAS |
| swiftGate | Ryzen 5800X | RTX 3070 FE | 64 GB DDR4 | ~2 TB | Mobile compute |
| flockGate | i9-13900K | RTX 3070 Ti | 64 GB DDR5 | 2 TB | Mesh compute |
| kinGate | i7-6700K | RTX 3070 | 32 GB DDR4 | ~1 TB | Staging |

Plus 4 SFF nodes (1 GMKtec NucBox M6, 3 Intel NUCs).

### Aggregates

| Metric | Value |
|--------|-------|
| CPU cores | 130+ |
| GPU VRAM (installed) | ~188 GB |
| GPU VRAM (float pool) | +68 GB (2x 3090, 2x Titan V, 2x MI50) |
| HBM2 | 56 GB (Titan V + MI50) |
| System RAM | ~1.2 TB |
| NVMe | ~49 TB |
| HDD (ZFS) | ~76 TB |
| NPUs | 3x BrainChip Akida AKD1000 |
| HSMs | 4x SoloKey FIDO2 |
| Total investment | ~$15,000 |

### Acquisition Strategy

Most bought used. Prices paid:

| Component | Source | Price |
|-----------|--------|------:|
| Tesla K80 | eBay | $50 |
| Titan V (x2) | eBay | $400 each |
| RTX 3090 (x2) | FB Marketplace | $700 each |
| Dual EPYC workstation | Surplus | ~$1,000 |
| 10G NIC (Mellanox CX-3, x4) | eBay | $15-25 each |
| 10G switch (MikroTik CRS305) | Amazon | $130 |

---

## What Sovereignty Means (Testable)

Four claims. Each verifiable.

**You own the hardware.** The CPUs, GPUs, and drives are physical objects
in a physical room. No API endpoint can be deprecated. No terms of
service can change. No pricing tier can be adjusted. If the internet goes
down, the science still runs.

**You own the data.** Every dataset fetched from NCBI, UniProt, or KEGG is
BLAKE3-hashed at download time and stored locally. The fetch is a one-time
event. The data doesn't expire. If the upstream source changes an
assembly, your local copy preserves the version you validated against, and
the provenance record shows both states.

**You own the compute.** No metered API calls. No GPU-hour billing. The
RTX 4070 runs Vulkan f64 GPU compute through DF64 double-float emulation
— the same precision NVIDIA reserves for datacenter cards, unlocked
through a sovereign shader pipeline (coralReef) that bypasses CUDA.

**You own the provenance.** Every computation produces a DAG entry
(rhizoCrypt), committed to a permanent ledger (loamSpine), attributed via
Ed25519-signed braids (sweetGrass). The chain is cryptographic. You can
verify any result without trusting anyone.

---

## Dependency Map

40+ external dependencies across 7 clusters. Honest assessment:

### Cloudflare (highest priority replacement)
DNS, TLS termination, tunnel for external access. Replacement path
specified: BearDog TLS (ChaCha20-Poly1305 with ACME), Songbird NAT
traversal, self-hosted authoritative DNS. Baselines capturing hourly.

### GitHub (longest pole)
Source hosting, CI, binary releases, Pages. Forgejo installed as
calibration instrument. 74 workflow files to port.

### Package Registries
crates.io, PyPI, Conda. Low urgency. Vendor escape hatch exists
(cargo vendor, pip download, conda pack).

### AI APIs
Anthropic, OpenAI. Optional. Ollama works locally. Long-term path is
sovereign inference through barraCuda WGSL compute.

### Science Data APIs
NCBI, UniProt, KEGG. Irreplaceable external data sources, but once
fetched, data is local forever. Not a service dependency — a data
dependency with caching.

### Internal Primal Gaps
5 of 6 resolved by upstream Phase 60. MethodGate enforced on 10/{{ total_stat(stat="total_primals") }}
services. Ionic token auth live. Resource envelopes enforced.

### Irreducible Externals
Domain registrar. Linux kernel. NVIDIA GPU drivers. Let's Encrypt
certificate chain. $5/month VPS for NAT relay. Accepted constraints.

**Summary**: ~20% sovereign by service count, ~80% by criticality.
Everything touching the science (compute, data, provenance, attribution)
is fully sovereign. What remains external is infrastructure plumbing.

---

## The ABG Model

An external bioinformatics research group connects through JupyterHub
at lab.primals.eco via Cloudflare tunnel. Four tiers:

| Tier | Access | Resources | Enforcement |
|------|--------|-----------|-------------|
| Admin | Full control | 48 GB RAM, 16 cores | Root-equivalent |
| Compute | Kernels, dispatch, workspace | 32 GB RAM, 8 cores | venv, wheelhouse, per-user scratch |
| Reviewer | Dashboards only, no execution | Read-only showcase | NoKernelManager, chmod 550, symlinks |
| Observer | Rendered output + provenance | Read-only | No terminal, no kernel, no file access |

Every restriction mechanism-enforced: filesystem permissions (root-owned),
kernel blocking, iptables owner-match (drops internet for restricted
users, preserves LAN), hidepid=2, ACLs on system binaries.

---

## Monthly Operating Cost

| Cost | Amount |
|------|-------:|
| Electricity | ~$150 |
| Internet | ~$80 |
| Domain / DNS | ~$5 |
| Hardware depreciation | ~$250 |
| **Total** | **~$485** |

No VC funding. No grants. No institutional backing. One person pays
the electricity. The attribution pipeline (sweetGrass) exists so that
when community contributions flow, credit flows back proportionally.

The parts list and the monthly bill are transparent. The cost makes sense when you see [what it replaces](@/philosophy/the_temptation_of_kingdoms.md).

The repos are open. Build your own.

---

## Read More

- [I Own Nothing](@/philosophy/i_own_nothing.md) — provenance chains, the economics of giving it all away
- [The Mobility Edge](@/philosophy/the_mobility_edge.md) — why a concept from Anderson localization describes network sovereignty
- [The New City](@/philosophy/the_new_city.md) — the architecture where the cost is shared, not hidden

---

## Verify

- **Deployment**: github.com/sporeGarden/projectNUCLEUS
- **All services**: github.com/ecoPrimals
- **Hardware inventory**: documented in the ecosystem
- **Live system**: lab.primals.eco
- **License**: AGPL-3.0-or-later
