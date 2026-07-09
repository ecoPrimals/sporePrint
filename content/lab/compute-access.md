+++
title = "Compute Access — ABG Compute Lab"
description = "Live compute environment on sovereign mesh hardware — RTX 5070 Ti GPU, 64-core EPYC CPU, JupyterHub via songBird capability routing at lab.primals.eco"
date = 2026-05-07
weight = 10

[taxonomies]
primals = ["barracuda", "biomeos", "nestgate", "toadstool", "songbird", "rhizocrypt", "loamspine", "sweetgrass", "squirrel", "coralreef", "skunkbat", "petaltongue", "beardog"]
springs = ["primalspring"]
+++

## Live at [lab.primals.eco](https://lab.primals.eco)

**JupyterHub 5.4.5** is live on [songBird](/primals/#songbird)-routed sovereign hardware at [lab.primals.eco](https://lab.primals.eco). No cloud. No exposed ports. Students, researchers, and collaborators connect through the WireGuard sovereign mesh — songBird's drawbridge routes HTTP traffic from golgi (VPS) to ironGate (compute node) via capability-based dispatch.

The compute substrate runs on a 64-core AMD EPYC 9124 with an RTX 5070 Ti GPU, 128GB ECC RAM, and NVMe storage. All primals communicate via BTSP Phase 3 AEAD (ChaCha20-Poly1305) and bind to `127.0.0.1` by default. Every notebook runs against real primals, not mocks — the same infrastructure that produced the baseCamp results.

## Who Can Access

| Tier | Linux Group | Access | Can Do |
|------|-------------|--------|--------|
| **Admin** | `abg-admin` | Full JupyterHub, 48G / 16 cores | Run notebooks, manage users, full primal API access |
| **Compute** | `abg-compute` | JupyterHub, 32G / 8 cores | Run notebooks, submit pipelines, write to shared space |
| **Observer** | `abg-observer` | JupyterHub, 8G / 4 cores | Run notebooks, read shared work, own home directory |
| **Reviewer** | `abg-reviewer` | JupyterHub, 4G / 2 cores | Read showcase/ only, no execute, designed for PIs |
| **External** | — | Read-only | View published results on primals.eco — no compute access |

All tiers see all work. No hidden notebooks, no private results. This is open and sovereign science.

## What You Can Run

- **baseCamp pipelines**: reproduce any of the 29+ published papers on the live composition
- **Spring validation**: run wetSpring 16S, hotSpring MD, airSpring ET₀, healthSpring PK on real GPUs
- **Cross-spring experiments**: combine primals from multiple springs in a single notebook
- **Your own science**: use barraCuda GPU compute, ToadStool shader dispatch, biomeOS coordination for new work

The shared workspace at `/shared/abg/` is visible to all members. Results, notebooks, and datasets are collaborative by default.

## How to Request Access

1. Contact the ecoPrimals team with your research interest and desired tier
2. An account is created with appropriate Linux group membership
3. Navigate to [lab.primals.eco](https://lab.primals.eco) — no VPN, no port forwarding, no cloud tunnel
4. Log in with your credentials — your JupyterHub session starts with the shared workspace linked and all 13 primal ports available as environment variables

For PIs and administrators: the shared workspace demonstrates what your researchers want to run on institutional HPC. Every notebook has full provenance — point your HPC team at the exact pipeline.

## Architecture

```
Browser → lab.primals.eco
    │
    ├─ DNS → golgi VPS (157.230.3.183)
    │        Caddy :443 (TLS termination, Let's Encrypt)
    │        └─ reverse_proxy → WireGuard mesh
    │
    ├─ WireGuard → sporeGate (10.13.37.2)
    │              songBird drawbridge :7780
    │              └─ capability.call("jupyter") → ironGate
    │
    └─ ironGate (10.13.37.7)
       JupyterHub :8000 (PAM auth, tiered pre_spawn_hook)
       │
       ├── /shared/abg/          ← collaborative workspace (all tiers read)
       │   ├── commons/          ← shared Jupyter notebooks
       │   ├── projects/         ← collaborative project workspaces
       │   ├── data/             ← shared input data
       │   ├── showcase/         ← polished results for external review
       │   └── templates/        ← starter notebooks
       │
       ├── ~/notebooks/          ← per-user home (symlinks to shared)
       │
       └── NUCLEUS composition   ← primals on 127.0.0.1 (systemd)
           ├── barraCuda (GPU compute — RTX 5070 Ti)
           ├── ToadStool (shader dispatch)
           ├── coralReef (data pipeline)
           ├── bearDog (crypto + auth)
           └── songBird (mesh routing + capability discovery)
```

The path from browser to notebook is fully sovereign: DNS → golgi TLS → WireGuard tunnel → songBird capability routing → JupyterHub. No Cloudflare. No cloud tunnels. Every hop is inspectable.

## Connection to sporePrint

Selected notebooks from the shared workspace are elevated to [primals.eco/lab/](/lab/) via the notebook rendering pipeline. The elevation process:

1. Researcher creates notebook in shared workspace
2. Notebook is reviewed and tagged for publication
3. `spore-validate render-notebooks` converts to Zola markdown with embedded charts
4. Published under `/lab/notebooks/` with full provenance metadata
5. Auto-refresh CI propagates updates to primals.eco

This connects live compute to the public evidence record.

## Related

- [Reproduce Results](@/lab/reproduce.md) — step-by-step reproduction guide
- [Provenance Pipeline](/lab/provenance-pipeline/) — how results are tracked and verified
- [Sovereign Compute Sharing](https://github.com/ecoPrimals/wateringHole/blob/main/compute-sharing/SOVEREIGN_COMPUTE_SHARING.md) — the full architecture spec
