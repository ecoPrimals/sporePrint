+++
title = "Living Systems — What's Running Now"
description = "Real-time status of the ecoPrimals sovereign mesh: active gates, deployed primals, capability routing, and live JupyterHub compute."
date = 2026-07-06
weight = 5

[taxonomies]
primals = ["songbird", "beardog", "biomeos", "petaltongue"]
springs = ["primalspring"]
+++

## The Mesh Is Alive

The ecoPrimals ecosystem is not a description of future work. It is running.
Seven gates are enrolled, three are meshed with bilateral direct TCP, and
{{ entity(name="songbird") }} routes `capability.call` across all of them.
This page shows what is actually deployed and operational.

{{ viz_embed(src="/viz/gate-mesh?live=true") }}

## Active Gates

| Gate | Status | Transport | What's Running |
|------|--------|-----------|----------------|
| **sporeGate** | 🟢 Online | LAN + WG | Sovereign CI, Caddy TLS, songBird mesh hub, Forgejo |
| **eastGate** | 🟢 Online | LAN + WG (10GbE) | Overwatch, primalSpring ({{ entity_stat(name="primalspring", stat="tests_display") }} tests), petalTongue |
| **ironGate** | 🟢 Meshed | LAN + WG | RTX 5070 Ti, **JupyterHub 5.4.5 LIVE**, songBird |
| **flockGate** | 🟡 WAN | WG via golgi | Tower atomic evolution: songBird, bearDog, skunkBat dev |
| **golgi** | 🟢 Online | VPS | WireGuard hub, Forgejo host, depot (30/30 ecobins), cascade timer |
| **grapheneGate** | 🟢 Online | ADB (USB) | Pixel 8a, 12/13 primals LIVE (nestGate + coralReef: Android UDS pending) |
| **strandGate** | 🟡 Alive | .103 (SSH pending) | 64-core EPYC, 256GB — hardware alive, SSH enrollment pending |

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

**Wave 133a result**: 30/30 ecobins built (15 x86_64 + 15 aarch64), 283 MB total,
all checksummed and published. 11/14 primals build with zero manual intervention;
3 require documented workarounds (being converged).

## Mesh Health

The mesh is self-healing. If a gate goes offline:

- {{ entity(name="songbird") }} detects peer loss via heartbeat timeout
- Capability routing tables update across all remaining peers
- Services that depended on the lost gate get routed to alternates
- When the gate returns, `peer.connect` re-establishes bilateral trust

**Key invariant**: unplugging any single gate does not kill the network.
The Flint edge router is the plasma membrane. Gates are ephemeral compute.

## What's Next

| Item | Gate | Status |
|------|------|--------|
| ~~JupyterHub deploy~~ | ironGate | **LIVE** — JupyterHub 5.4.5, `lab.primals.eco → 200` |
| `jupyter` capability registration | ironGate | Pending `primal.announce` |
| bearDog ACME gateway activation (Caddy retirement) | sporeGate | After flockGate CryptoProvider fix |
| strandGate SSH enrollment | strandGate | Hardware alive at .103, SSH key pending |
| Bidirectional relay (GitHub ↔ Forgejo parity) | golgi | cellMembrane deploy |
| Live mesh visualization (petalTongue wire-up) | eastGate | sporePrint + petalTongue |

## Related

- [Compute Access](@/lab/compute-access.md) — tiers, hardware, how to connect
- [Gate Mesh — Live Topology](@/architecture/MESH_TOPOLOGY.md) — mesh architecture and enrollment
- [Reproduce Results](@/lab/reproduce.md) — run the same pipelines on your hardware
