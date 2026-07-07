+++
title = "Compute Access — ABG Compute Lab"
description = "Live compute environment on sovereign mesh hardware — RTX 5070 Ti GPU, 64-core EPYC CPU, JupyterHub via songBird capability routing at lab.primals.eco"
date = 2026-07-06
weight = 10

[taxonomies]
primals = ["barracuda", "biomeos", "nestgate", "toadstool", "songbird", "rhizocrypt", "loamspine", "sweetgrass", "squirrel", "coralreef", "skunkbat", "petaltongue", "beardog"]
springs = ["primalspring"]
+++

## Live at [lab.primals.eco](https://lab.primals.eco)

**JupyterHub 5.4.5** is live on {{ entity(name="songbird") }}-routed sovereign hardware at [lab.primals.eco](https://lab.primals.eco). No cloud. No exposed ports. Students, researchers, and collaborators access live compute through the mesh — the same infrastructure that produced the baseCamp results.

The compute lab runs on a multi-gate mesh where {{ entity(name="songbird") }} handles all routing via `capability.call`. Services bind to `localhost`. The mesh absorbs new hardware automatically — plug in a node, cascade primals, announce capabilities, and the mesh routes to it.

## Mesh Compute Architecture

```
Browser → lab.primals.eco
    → Cloudflare (DDoS, TLS edge)
    → Flint H1 (edge router, plasma membrane)
    → sporeGate (.3) — public entry
        → bearDog ACME gateway :443 (TLS termination)
        → songBird capability.call
            ├── ironGate (.237, LAN direct, <1ms)
            │   └── JupyterHub :8000
            │   └── RTX 5070 Ti (16GB VRAM)
            │   └── GROMACS, CUDA workloads
            ├── strandGate (joining)
            │   └── 64-core EPYC, 256GB RAM
            │   └── STAR alignment, heavy CPU
            └── (future nodes auto-absorb)
```

**Key principle**: adding compute is zero-config. New hardware runs `songBird mesh.init` → `primal.announce` capabilities → mesh routes to it. No firewall changes, no config files, no DNS updates.

## Compute Hardware

| Gate | Hardware | Role | Status |
|------|----------|------|--------|
| **ironGate** | RTX 5070 Ti (16GB VRAM), NVMe | GPU compute, GROMACS, CUDA, **JupyterHub LIVE** | Meshed, LAN direct |
| **strandGate** | 64-core EPYC, 256GB DDR5 | CPU compute, STAR alignment, heavy bioinformatics | Alive at .103, SSH pending |
| **sporeGate** | i7, 32GB | Public entry, Sovereign CI, build host | Active |
| **eastGate** | i9-14900K, 96GB DDR5, 10GbE | Overwatch, primalSpring, petalTongue | Active |

All gates are connected via LAN (sub-millisecond latency) and WireGuard overlay (cross-site). {{ entity(name="songbird") }}'s `try_lan_direct_connect` detects shared subnets and routes directly — no VPN overhead for LAN peers.

## Who Can Access

| Tier | Access | Can Do |
|------|--------|--------|
| **Admin** | Full JupyterHub, all resources | Manage users, full primal API, GPU workloads |
| **Compute** | JupyterHub, GPU + CPU | Run notebooks, submit pipelines, shared workspace |
| **Observer** | JupyterHub, limited resources | Run notebooks, read shared work |
| **Reviewer** | JupyterHub, read-only | View showcase/ only — designed for PIs |
| **External** | primals.eco (public) | View published results — no compute access |

All tiers see all work. No hidden notebooks, no private results. Open and sovereign science.

## What You Can Run

- **baseCamp pipelines**: reproduce any of the 70+ published papers on live compute
- **Spring validation**: wetSpring 16S, hotSpring MD, airSpring ET₀, healthSpring PK — real GPUs
- **Cross-spring experiments**: combine primals from multiple springs in a single notebook
- **GPU workloads**: GROMACS metadynamics, CUDA kernels, barraCuda vendor-agnostic WGSL shaders
- **CPU-heavy bioinformatics**: STAR alignment, DESeq2, WGCNA on EPYC cores (when strandGate joins)

The shared workspace at `/shared/abg/` is visible to all members. Results, notebooks, and datasets are collaborative by default.

## How to Request Access

1. Email **ecoPrimal@pm.me** with your research interest and desired tier
2. An account is created with appropriate access level
3. Navigate to [lab.primals.eco](https://lab.primals.eco) — no VPN, no port forwarding
4. Log in — your JupyterHub session starts with the shared workspace linked and all primal capabilities routed via {{ entity(name="songbird") }}

## Sovereign CI — Binary Pipeline

All 15 primals (30 binaries — x86_64-musl + aarch64-musl) are built on sporeGate's Sovereign CI, checksummed with SHA-256, and published to the depot at `membrane.primals.eco/depot/{triple}/{binary}`. Every binary is reproducible from source on Forgejo (`git.primals.eco`).

| Metric | Value |
|--------|-------|
| Primals built | 15/15 |
| Architectures | x86_64-musl, aarch64-musl |
| Total binary size | 283 MB (153 MB x86 + 130 MB arm64) |
| Build host | sporeGate (Sovereign CI) |
| Source | Forgejo (`git.primals.eco`) |
| Checksums | `checksums.toml` with SHA-256 per binary |

## Connection to sporePrint

Selected notebooks from the shared workspace are elevated to [primals.eco/lab/](/lab/) via the notebook rendering pipeline:

1. Researcher creates notebook in shared workspace
2. Notebook is reviewed and tagged for publication
3. `spore-validate render-notebooks` converts to Zola markdown with embedded charts
4. Published under `/lab/notebooks/` with full provenance metadata
5. Auto-refresh CI propagates updates to primals.eco

This connects live compute to the public evidence record.

## Related

- [Living Systems — What's Running Now](@/lab/living-systems.md) — real-time mesh status and live examples
- [Reproduce Results](@/lab/reproduce.md) — step-by-step reproduction guide
- [Provenance Pipeline](@/lab/provenance-pipeline.md) — how results are tracked and verified
- [Gate Mesh — Live Topology](@/architecture/MESH_TOPOLOGY.md) — real-time mesh visualization
