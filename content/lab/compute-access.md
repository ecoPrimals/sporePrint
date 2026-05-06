+++
title = "Compute Access — JupyterHub on ironGate"
description = "Live compute environment running the full 13-primal NUCLEUS composition on sovereign hardware — request access"
date = 2026-05-06
weight = 10

[taxonomies]
primals = ["barracuda", "biomeos", "nestgate", "toadstool", "songbird", "rhizocrypt", "loamspine", "sweetgrass", "squirrel", "coralreef", "skunkbat", "petaltongue", "beardog"]
springs = ["primalspring"]
+++

## What's Running

A **13-primal NUCLEUS composition** runs on ironGate — a sovereign compute node with consumer-grade hardware (RTX 4070, RTX 3090, Akida NPU). All primals communicate via BTSP Phase 3 AEAD (ChaCha20-Poly1305), bind to `127.0.0.1` by default, and are accessible through Cloudflare Tunnel with zero exposed ports.

**JupyterHub** provides multi-user notebook access to this live composition. Every notebook runs against real primals, not mocks — the same infrastructure that produced the baseCamp results.

## Who Can Access

| Tier | Access | Can Do |
|------|--------|--------|
| **abg-full** | Full JupyterHub, shared workspace | Run notebooks, write to shared space, submit pipelines |
| **abg-limited** | JupyterHub, read shared workspace | Run notebooks, read shared work, own home directory |
| **abg-reviewer** | JupyterHub, read-only | Read all shared work, copy notebooks, no write |
| **external** | Read-only key | View published results on primals.eco — no compute access |

All tiers see all work. No hidden notebooks, no private results. This is open and sovereign science.

## What You Can Run

- **baseCamp pipelines**: reproduce any of the 29+ published papers on the live composition
- **Spring validation**: run wetSpring 16S, hotSpring MD, airSpring ET₀, healthSpring PK on real GPUs
- **Cross-spring experiments**: combine primals from multiple springs in a single notebook
- **Your own science**: use barraCuda GPU compute, ToadStool shader dispatch, biomeOS coordination for new work

The shared workspace at `/shared/abg/` is visible to all members. Results, notebooks, and datasets are collaborative by default.

## How to Request Access

1. Contact the ecoPrimals team with your research interest and desired tier
2. An account is created with appropriate access level
3. Connect via Cloudflare Tunnel — no VPN, no port forwarding needed
4. Your JupyterHub session starts with symlinks to the shared workspace and template notebooks

For PIs and administrators: the shared workspace demonstrates what your researchers want to run on institutional HPC. Every notebook has full provenance — point your HPC team at the exact pipeline.

## Architecture

```
JupyterHub (PAM auth, tiered pre_spawn_hook)
    │
    ├── /shared/abg/          ← collaborative workspace (all tiers read)
    │   ├── notebooks/        ← shared Jupyter notebooks
    │   ├── results/          ← pipeline outputs
    │   ├── datasets/         ← shared input data
    │   └── showcase/         ← polished results for external review
    │
    ├── ~/                    ← per-user home directory
    │
    └── NUCLEUS composition   ← 13 primals on localhost
        ├── biomeOS (coordinator)
        ├── barraCuda (GPU compute)
        ├── ToadStool (shader dispatch)
        ├── NestGate (auth + BTSP)
        ├── Songbird (discovery)
        └── ... (8 more primals)
```

## Connection to sporePrint

Selected notebooks from the shared workspace are elevated to [primals.eco](/lab/) via the notebook rendering pipeline. The elevation process:

1. Researcher creates notebook in shared workspace
2. Notebook is reviewed and tagged for publication
3. `render_notebooks.sh` converts to static HTML
4. Published under `/lab/` with full provenance metadata

This connects live compute to the public evidence record.

## Related

- [Reproduce Results](/lab/reproduce-ecosystem-results/) — step-by-step reproduction guide
- [Provenance Pipeline](/lab/provenance-pipeline/) — how results are tracked and verified
- [Sovereign Compute Sharing](https://github.com/ecoPrimals/wateringHole/blob/main/compute-sharing/SOVEREIGN_COMPUTE_SHARING.md) — the full architecture spec
