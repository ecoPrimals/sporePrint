+++
title = "Living Systems — What's Running Now"
description = "Real-time status of the ecoPrimals sovereign mesh: Wave 158 REWAKE. 5-target depot. primals.eco LIVE. detroit.primals.eco LIVE (128+ pages). northGate ENROLLING. Jupiter 2 ARRIVED."
date = 2026-09-27
weight = 5

[taxonomies]
primals = ["songbird", "beardog", "biomeos", "petaltongue"]
springs = ["primalspring"]
+++

## The Mesh Is Alive

This is not a description of future work. It is running.

**Wave 158 REWAKE.** eastGate + golgiBody + sporeGate **ONLINE**. northGate **ENROLLING** (Windows 11, RTX 5090). Milk-V Jupiter 2 **ARRIVED** (RISC-V RVA23 — 7th architecture family). primals.eco **LIVE**. detroit.primals.eco **LIVE** (128+ pages, dynasty expansion). 5-target depot (19/19 + 14/14 + 16/16 + 16/16 + 12/17). **ZERO P0s. ZERO P1s. ZERO P2s.** Pipeline + provenance CONVERGED.

{{ viz_embed(src="/viz/gate-mesh?live=true", caption="Live gate mesh: sovereign compute nodes and their network connections") }}

## Active Gates

| Gate | Composition | Status |
|------|-------------|--------|
| **eastGate** | Full NUCLEUS + overwatch | ✅ ONLINE. rustChip standalone (367 tests). Wave 158 cascade. |
| **golgiBody** | Caddy + Forgejo + Zola + cascade | ✅ ONLINE. Cascade autonomous. detroit + primals.eco serving. |
| **sporeGate** | Foreman + depot + cascade hub | ✅ ONLINE. Windows depot 12/17. primalSpring IPC gated. |
| **northGate** | Tower Atomic (target) | 🔄 ENROLLING. Windows 11, RTX 5090, 96GB DDR5. |
| **Jupiter 2** | NEW (RISC-V RVA23) | 🆕 Milk-V Jupiter 2. 7th arch family. Bring-up pending. |
| **NUC bench** | Tower Atomic (target) | 🆕 DDR3 NUCs — sub-builders, site hosts, mesh nodes. |
| **biomeGate** | Tower 4/4 + Node Atomic | ⏸️ OFFLINE. Power on needed. Titan V Tier 1. |
| **graftGate** | FULL NUCLEUS (Darwin) | ⏸️ OFFLINE. Power on needed. Depot 16/16. |
| **ironGate** | Full NUCLEUS + 14TB CAS | ⏸️ OFFLINE. Power rebalance next week. |
| **strandGate** | Full NUCLEUS + dual EPYC | ⏸️ OFFLINE. 45 QCD configs banked. Power rebalance. |
| **westGate** | Full NUCLEUS + 50.7TB ZFS | ⏸️ OFFLINE. Power rebalance next week. |
| **blueGate** | ENMESHED (Windows) | ⏸️ OFFLINE. Rack move incomplete. |
| **southGate** | NUCLEUS + canary | ⏸️ OFFLINE. Power rebalance next week. |
| **grapheneGate** | Tower Atomic | ADB deploy. Pixel 8a. |
| **iosGate** | BearDogApp | 6th OS family. iPhone XS. |
| **steamGate** | Tower Atomic | Portable compute. Steam Deck. |

## Live Capabilities

When a gate starts {{ entity(name="songbird") }}, it announces its capabilities to
the mesh. Other gates can then invoke any capability by name — songBird routes to the
best available provider.

### Currently Routed

| Capability | Provider | Path | Use |
|------------|----------|------|-----|
| `http.proxy` | sporeGate | LAN direct | HTTP routing to mesh services |
| `peer.connect` | all meshed gates | bilateral TCP | Mesh peering, 0ms LAN |
| `capability.call` | sporeGate → ironGate | LAN direct | Cross-gate compute dispatch |
| `build.release` | sporeGate | local | Sovereign CI binary builds |
| `cascade.sync` | golgi | WG | 15-min quorum cascade timer |

### Deploying

| Capability | Provider | Status |
|------------|----------|--------|
| `jupyter.execute` | ironGate | **JupyterHub 5.4.5 LIVE** — `lab.primals.eco → 200` |
| `footprint.serve` | sporeGate | **LIVE** — [footprint.primals.eco](https://footprint.primals.eco) (200, 216ms) |
| `esotericwebb.serve` | flockGate | **LIVE** — [webb.primals.eco](https://webb.primals.eco) (200) |
| `ws.bridge` | sporeGate | **LIVE** — petalTongue `/ws` JSON-RPC on :8080 (Wave 150g) |
| `compute.gpu` | ironGate | RTX 5070 Ti ready, capability registration in progress |
| `compute.cpu` | strandGate | Awaiting hardware enrollment |

## JupyterHub — Live Compute

JupyterHub 5.4.5 is running on ironGate, serving at `lab.primals.eco`. The path is:

```
Browser → lab.primals.eco
    → bearDog :443 (ACME TLS)
    → songBird capability.call("jupyter")
    → ironGate :8000 (LAN direct, <1ms)
    → JupyterHub session
```

**What makes this different from a cloud notebook**: your computation runs on
sovereign hardware in a private lab. No telemetry. No vendor. The mesh handles
routing — if ironGate goes offline, songBird can route to strandGate (once enrolled)
or any future compute node. The notebook doesn't know which gate ran it.

### Example Workloads Available

| Workload | Hardware | Spring |
|----------|----------|--------|
| 16S metagenomics pipeline | CPU | wetSpring |
| GROMACS metadynamics (CAZyme FEL) | RTX 5070 Ti GPU | hotSpring |
| Salmon RNA-seq quantification | CPU + NVMe | wetSpring |
| STAR alignment (large genomes) | 64-core EPYC (strandGate) | wetSpring |
| ET₀ irrigation modeling | CPU | airSpring |
| PK/PD compartmental modeling | CPU | healthSpring |

Each workload runs against the same infrastructure that produced the
[baseCamp results](@/science/_index.md). Every run gets a provenance chain.

## Depot — 5-Target Binary Distribution

| Architecture | Binaries | Status |
|-------------|----------|--------|
| **x86_64-unknown-linux-musl** | **19/19** | ✅ CURRENT (Sep 15-25) |
| **x86_64-unknown-linux-gnu** | **14/14** | ✅ CURRENT |
| **aarch64-unknown-linux-musl** | **16/16** | ✅ CURRENT (ironGate sub-builder) |
| **aarch64-apple-darwin** | **16/16** | ✅ CURRENT (graftGate) |
| **x86_64-pc-windows-gnu** | **12/17** | 🔄 5 blocked on team unix fixes |

3/3 sub-builders enmeshed. NanoWire SSH Tier 1 **RETIRED** — builders communicate
via Tower Atomic, no SSH dispatch. Cascade autonomous. RISC-V (`riscv64gc-unknown-linux-musl`) target pending Jupiter 2 bring-up.

## Sovereign CI Pipeline

All {{ total_stat(stat="total_primals") }} primals are continuously built from source
on sporeGate's Sovereign CI. The pipeline:

```
Developer pushes to Forgejo (git.primals.eco)
    → golgi cascade timer (15-min quorum)
    → sporeGate pulls, builds x86_64-musl + aarch64-musl
    → ironGate sub-builder: aarch64-musl
    → graftGate sub-builder: aarch64-apple-darwin
    → BLAKE3 checksums computed
    → Binaries published to depot (membrane.primals.eco/depot/)
    → Gates cascade + pull from depot
```

13/13 primals converged — zero CI workarounds, zero code debt.

## Mesh Health

The mesh is self-healing. If a gate goes offline:

- {{ entity(name="songbird") }} detects peer loss via heartbeat timeout
- Capability routing tables update across all remaining peers
- Services that depended on the lost gate get routed to alternates
- When the gate returns, `peer.connect` re-establishes bilateral trust

**Key invariant**: unplugging any single gate does not kill the network.
The Flint edge router is the plasma membrane. Gates are ephemeral compute.

## What's Next

| Item | Priority | Status |
|------|----------|--------|
| **northGate enrollment** | **HIGH** | 🔄 ENROLLING. WG mesh → Tower Atomic → vine-bat zero-SSH |
| **{{ entity(name="guerillagorilla") }} formalization** | **HIGH** | amicusContra named. Dispersal site concept. |
| **Windows depot: 5 team unix fixes** | **HIGH** | 12/17 done. 5 primals blocked on `#[cfg(unix)]` gating |
| arXiv reviewer send (Murillo, Chuna, Bazavov) | **HIGH** | UNBLOCKED — primals.eco LIVE |
| {{ entity(name="detroit") }} expansion | **HIGH** | LIVE — 128+ pages, dynasty expansion (35+ actors) |
| bonsai-bt Phase 0→1 (sourDough scaffold) | HIGH | exp125 23/24. DECIDE layer. |
| **Milk-V Jupiter 2 bring-up** | MED | RISC-V RVA23. 7th arch. `riscv64gc-unknown-linux-musl` |
| **NUC bench composition** | MED | Tower Atomic on DDR3 NUCs — mesh expansion |
| **House 2 power rebalance** | MED | ironGate, strandGate, westGate, southGate, blueGate |
| sporePrint → NUCLEUS live data surface | MED | Concept: static → live phased evolution |
| tideGlass Phase 0 START | MED | QUEUED |
| SSH → Tower Atomic graduation (NanoWire Tiers 2-7) | NEXT | Tier 1 RETIRED |

## Related

- [Tower Atomic](@/architecture/tower_atomic.md) — sovereign transport stack replacing WireGuard
- [Gate Mesh Topology](@/architecture/MESH_TOPOLOGY.md) — gate topology, enrollment, traffic classes
- [Sovereign CI](@/architecture/SOVEREIGN_CI.md) — Forgejo → sporeGate → depot
- [Compute Access](@/lab/compute-access.md) — tiers, hardware, how to connect
- [Reproduce Results](@/lab/reproduce.md) — run the same pipelines on your hardware
