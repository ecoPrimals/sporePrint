+++
title = "Compute Access — ABG Compute Lab"
description = "Live compute environment running the full 13-primal NUCLEUS composition on sovereign hardware — request access at lab.primals.eco"
date = 2026-05-07
weight = 10

[taxonomies]
primals = ["barracuda", "biomeos", "nestgate", "toadstool", "songbird", "rhizocrypt", "loamspine", "sweetgrass", "squirrel", "coralreef", "skunkbat", "petaltongue", "beardog"]
springs = ["primalspring"]
+++

## Live at [lab.primals.eco](https://lab.primals.eco)

A **13-primal NUCLEUS composition** runs on a sovereign compute node with consumer-grade hardware (RTX 4070, RTX 3090, Akida NPU). All primals communicate via BTSP Phase 3 AEAD (ChaCha20-Poly1305), bind to `127.0.0.1` by default, and are accessible through a named Cloudflare Tunnel with zero exposed ports.

**JupyterHub** provides multi-user notebook access at [lab.primals.eco](https://lab.primals.eco). Every notebook runs against real primals, not mocks — the same infrastructure that produced the baseCamp results. All 13 primals, JupyterHub, and the Cloudflare tunnel run as persistent systemd services that survive reboots.

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
3. Navigate to [lab.primals.eco](https://lab.primals.eco) — no VPN, no port forwarding needed
4. Log in with your credentials — your JupyterHub session starts with the shared workspace linked and all 13 primal ports available as environment variables

For PIs and administrators: the shared workspace demonstrates what your researchers want to run on institutional HPC. Every notebook has full provenance — point your HPC team at the exact pipeline.

## Architecture

```
lab.primals.eco → Cloudflare Tunnel (named: nucleus-lab)
    │
    └── JupyterHub :8000 (PAM auth, tiered pre_spawn_hook)
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
        └── NUCLEUS composition   ← 13 primals on 127.0.0.1 (systemd)
            ├── biomeOS :9800 (coordinator)
            ├── barraCuda :9740 (GPU compute)
            ├── ToadStool :9400 (shader dispatch)
            ├── NestGate :9500 (auth + BTSP)
            ├── Songbird :9200 (discovery)
            ├── BearDog :9100 (scheduling)
            ├── coralReef :9730 (data pipeline)
            ├── loamSpine :9700 (storage)
            ├── rhizoCrypt :9601 (encryption)
            ├── sweetGrass :9850 (provenance)
            ├── Squirrel :9300 (AI coordination)
            ├── skunkBat :9140 (anomaly detection)
            └── petalTongue :9900 (dashboards)
```

## Connection to sporePrint

Selected notebooks from the shared workspace are elevated to [primals.eco/lab/](/lab/) via the notebook rendering pipeline. The elevation process:

1. Researcher creates notebook in shared workspace
2. Notebook is reviewed and tagged for publication
3. `render_notebooks.sh` converts to static HTML with embedded charts
4. Published under `/lab/notebooks/` with full provenance metadata
5. Auto-refresh CI propagates updates to primals.eco

This connects live compute to the public evidence record.

## Related

- [Reproduce Results](@/lab/reproduce.md) — step-by-step reproduction guide
- [Provenance Pipeline](/lab/provenance-pipeline/) — how results are tracked and verified
- [Sovereign Compute Sharing](https://github.com/ecoPrimals/wateringHole/blob/main/compute-sharing/SOVEREIGN_COMPUTE_SHARING.md) — the full architecture spec
