+++
title = "Living Systems — What's Running Now"
description = "Real-time status of the ecoPrimals sovereign mesh: active gates, deployed primals, capability routing, and live JupyterHub compute."
date = 2026-07-07
weight = 5

[taxonomies]
primals = ["songbird", "beardog", "biomeos", "petaltongue"]
springs = ["primalspring"]
+++

## The Mesh Is Alive

The ecoPrimals ecosystem is not a description of future work. It is running.
Ten gates are online across two physical sites linked by an 80m 10G AOC trunk.
**Three gates now run full NUCLEUS** — all 13 primals composed and orchestrated
by {{ entity(name="biomeos") }}. [Provenance 7/7](@/architecture/NUCLEUS_ARCHITECTURE.md)
is **COMPLETE** — the full signed chain (CAS → DAG → Merkle → Spine → Ed25519 →
Attribution braid) works on live hardware, validated across Linux and Windows.
[Sovereign CI](@/architecture/SOVEREIGN_CI.md) automates push-to-deploy with
35 depot binaries. **ZERO P0s, P1s, or blocking P2s.** gen4 is COMPLETE. This
page shows what is actually deployed and operational.

{{ viz_embed(src="/viz/gate-mesh?live=true", caption="Live gate mesh: sovereign compute nodes and their network connections") }}

## Active Gates

| Gate | Status | Platform | What's Running |
|------|--------|----------|----------------|
| **sporeGate** | Online | Linux | Build authority, genomeBin harvester, depot rebuild |
| **eastGate** | Online | Linux | Overwatch, primalSpring ({{ entity_stat(name="primalspring", stat="tests_display") }} tests), biomeOS evolution |
| **westGate** | **Nest Atomic LIVE** | Linux | **8 services, 1,704 capabilities, ZFS 25.4TB + 2TB L2ARC, 6 PDBs in CAS** |
| **strandGate** | **Tower+Compute LIVE** | Linux | Dual EPYC 7452, 256GB, RTX 3090, Compute Trio deployed |
| **ironGate** | Online | Linux | 4x HDD (14TB+), HDD enclave experiment |
| flockGate | **DOWN** | Linux | Rebooted, RustDesk locked out |
| **golgiBody** | Online | Linux (VPS) | Sole depot (39 genomeBins), enrollment endpoint, Forgejo |
| **northGate** | Online | Windows | RTX 5090, AlphaFold source (~1TB), G1 target |
| **grapheneGate** | Online | Android | Tower LIVE, G2: mobile trust boundary |
| **blueGate** | Online | Windows | G1: Tower on Windows, peptidoglycan anchor H2 |
| **swiftGate** | Online | Windows | G1: Tower on Windows |
| **southGate** | HW Ready | Linux | Omada 10G, enrollment pending |
| fieldGate | Offline | — | Dead CMOS |
| biomeGate | Offline | — | Kernel recovery |

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

## Sovereign CI Pipeline

All {{ total_stat(stat="total_primals") }} primals are continuously built from source
on sporeGate's Sovereign CI. The pipeline:

```
Developer pushes to Forgejo (git.primals.eco)
    → golgi cascade timer (15-min quorum)
    → sporeGate pulls, builds x86_64-musl + aarch64-musl
    → BLAKE3 checksums computed
    → Binaries published to depot (membrane.primals.eco/depot/)
    → Gates cascade + pull from depot
```

**Wave 133e result**: 30/30 ecobins in pepti (15 x86_64 + 15 aarch64),
all checksummed. 13/13 primals converged — zero CI workarounds, zero code debt.
4–5 binaries pending rebuild from latest source.

## Mesh Health

The mesh is self-healing. If a gate goes offline:

- {{ entity(name="songbird") }} detects peer loss via heartbeat timeout
- Capability routing tables update across all remaining peers
- Services that depended on the lost gate get routed to alternates
- When the gate returns, `peer.connect` re-establishes bilateral trust

**Key invariant**: unplugging any single gate does not kill the network.
The Flint edge router is the plasma membrane. Gates are ephemeral compute.

## What's Next

| Item | Wave | Status |
|------|------|--------|
| ~~JupyterHub deploy~~ | 132 | **LIVE** — JupyterHub 5.4.5, `lab.primals.eco → 200` |
| Pepti rebuild (5 stale binaries) | 134a | **NEXT** — songBird, skunkBat, nestGate, coralReef, sweetGrass |
| WAN-DISPATCH-01 FULL PASS | 134a | After pepti rebuild — songBird drawbridge committed |
| grapheneGate 13/13 from fresh pepti | 134a | After pepti rebuild |
| ~~bearDog CryptoProvider fix (UNIT-DIV-04)~~ | 134b | **DONE** — resolved, DNS live |
| ~~DNS cutover: `primals.eco` → golgi (bearDog ACME)~~ | 134b | **DONE** — sovereign DNS live since Wave 100+ |
| strandGate SSH enrollment | 134b | Physical access to House 2 needed |
| Live mesh visualization (petalTongue on golgi) | 134b+ | sporePrint host composition |

## Related

- [Tower Atomic](@/architecture/tower_atomic.md) — sovereign transport stack replacing WireGuard
- [Gate Mesh Topology](@/architecture/MESH_TOPOLOGY.md) — gate topology, enrollment, traffic classes
- [Sovereign CI](@/architecture/SOVEREIGN_CI.md) — Forgejo → sporeGate → depot
- [Compute Access](@/lab/compute-access.md) — tiers, hardware, how to connect
- [Reproduce Results](@/lab/reproduce.md) — run the same pipelines on your hardware
