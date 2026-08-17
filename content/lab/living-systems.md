+++
title = "Living Systems — What's Running Now"
description = "Real-time status of the ecoPrimals sovereign mesh: 12 gates ONLINE, 6 NUCLEUS, 4-architecture depot, rootPulse 6/6 REGISTERED."
date = 2026-08-17
weight = 5

[taxonomies]
primals = ["songbird", "beardog", "biomeos", "petaltongue"]
springs = ["primalspring"]
+++

## The Mesh Is Alive

This is not a description of future work. It is running.

**12 gates ONLINE. NUCLEUS on 6 gates.** Cascade autonomous across 4 architectures. rootPulse 6/6 graphs REGISTERED. bonsai-bt DECIDE layer ingesting. **ZERO P0s. ZERO P1s. ZERO P2s.** Pipeline + provenance CONVERGED.

{{ viz_embed(src="/viz/gate-mesh?live=true", caption="Live gate mesh: sovereign compute nodes and their network connections") }}

## Active Gates

| Gate | Composition | Status |
|------|-------------|--------|
| **eastGate** | Full NUCLEUS + overwatch | rootPulse 6/6 REGISTERED. exp125 bonsai-bt LIVE. biomeOS {{ entity_stat(name="biomeos", stat="tests_display") }} tests. |
| **ironGate** | Full NUCLEUS + 14TB CAS | 13/13, 2ms dispatch, 4 mesh peers |
| **strandGate** | Full NUCLEUS + dual EPYC | DF64 shaders SHIPPED. arXiv ACTIVE. 32⁴ SU(3) production COMPLETE. |
| **westGate** | Full NUCLEUS + 50.7TB ZFS | AlphaFold ingress ACTIVE. rootPulse handlers SHIPPED. |
| **sporeGate** | Foreman + depot | 13/13 x86_64 CURRENT. Cascade autonomous. |
| **blueGate** | ENMESHED (Windows) | builder.serve ALIVE :9800. Depot 0/13 STALE. |
| **graftGate** | FULL NUCLEUS (Darwin) | builder.serve LIVE :9800. Depot 16/16 CURRENT. M4 Mac Mini. |
| **southGate** | NUCLEUS + canary | neuralSpring 71/80. SSH ready. |
| **biomeGate** | Tower 4/4 + Node Atomic | ONLINE. Titan V Tier 1 CONFIRMED. K80 blocked (GK210). |
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

## Depot — 4-Architecture Binary Distribution

| Architecture | Binaries | Status |
|-------------|----------|--------|
| **x86_64-unknown-linux-musl** | 13/13 | Current (rebuilt Aug 14) |
| **aarch64-unknown-linux-musl** | 15/15 | Current (ironGate sub-builder) |
| **aarch64-apple-darwin** | 16/16 | Current (graftGate) |
| **x86_64-pc-windows-gnu** | 0/13 | STALE (awaiting autonomous dispatch) |

3/3 sub-builders enmeshed. NanoWire SSH Tier 1 **RETIRED** — builders communicate
via Tower Atomic, no SSH dispatch. Cascade autonomous.

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
| **FIX primals.eco** — Zola build/deploy regression | **CRITICAL** | Blocks arXiv reviewer send |
| bonsai-bt Phase 0→1 (sourDough scaffold) | HIGH | exp125 23/24. DECIDE layer. |
| blueGate depot rebuild via autonomous dispatch | HIGH | 0/13 STALE |
| tideGlass Phase 0 START | HIGH | QUEUED — external review 5-7 days |
| arXiv reviewer send (Murillo, Chuna, Bazavov) | HIGH | BLOCKED on primals.eco fix |
| bearDog AEAD Neural API surfacing | MED | ironGate |
| cellMembrane UDS→TCP fallback (Windows) | MED | sporeGate |
| Graph visualization spec | MED | ironGate + eastGate |
| SSH → Tower Atomic graduation (NanoWire Tiers 2-7) | NEXT | Tier 1 RETIRED |

## Related

- [Tower Atomic](@/architecture/tower_atomic.md) — sovereign transport stack replacing WireGuard
- [Gate Mesh Topology](@/architecture/MESH_TOPOLOGY.md) — gate topology, enrollment, traffic classes
- [Sovereign CI](@/architecture/SOVEREIGN_CI.md) — Forgejo → sporeGate → depot
- [Compute Access](@/lab/compute-access.md) — tiers, hardware, how to connect
- [Reproduce Results](@/lab/reproduce.md) — run the same pipelines on your hardware
