+++
title = "Gate Mesh — Live Topology"
description = "Real-time view of the ecoPrimals sovereign compute mesh: gates, links, latencies, and capability routing."
date = 2026-07-04
weight = 20

[taxonomies]
primals = ["songbird", "biomeos"]

[extra]
domain = "Architecture"
+++

## Overview

The ecoPrimals gate mesh is a sovereign, self-hosted network of compute gates connected via WireGuard overlay and coordinated through {{ entity(name="songbird") }}. Each gate runs a NUCLEUS composition and participates in capability-based routing — no centralized orchestrator, no exposed ports.

{{ viz_embed(src="/viz/gate-mesh?live=true") }}

## How Gates Connect

Peers discover each other through three path types, selected by songBird at runtime:

| Path Type | Mechanism | Latency | When Used |
|-----------|-----------|---------|-----------|
| **LAN direct** | `/proc/net/fib_trie` subnet detection → TCP | <1ms | Same L2 segment |
| **WireGuard overlay** | `10.13.37.0/24` via golgi hub | 5-30ms | Cross-site, same ISP |
| **TURN relay** | NAT traversal fallback | 50-200ms | Hostile NAT, mobile |

songBird's `try_lan_direct_connect` probes local subnets first. If peers share a LAN, traffic flows directly — no VPN overhead. WireGuard activates for cross-site links. TURN is the final fallback.

## Capability Routing

Services bind exclusively to `localhost`. songBird IS the port solver:

1. A gate registers capabilities via `primal.announce`
2. Callers invoke `capability.call` with a capability name
3. songBird routes to the best available provider (LAN-prefer, WAN-fallback)
4. Results flow back through the mesh transparently

This means adding a new compute node is zero-config: plug in hardware, cascade primals, `primal.announce` capabilities — the mesh absorbs.

## Current Mesh State

The visualization above updates from songBird's `mesh.peers` endpoint. Color indicates link health:

| Color | Meaning |
|-------|---------|
| Green | Reachable, latency < 5ms (LAN direct) |
| Yellow | Reachable, latency < 50ms (WireGuard) |
| Orange | Reachable, latency ≥ 50ms (relay/WAN) |
| Grey | Unreachable or offline |

When songBird is unavailable, the visualization gracefully degrades to static topology data — showing known gates and their roles without live latency.

## Enrolled Gates

| Gate | Role | Transport | Capabilities |
|------|------|-----------|--------------|
| golgi | WG hub, Forgejo, depot | VPS (relay) | `cascade.sync`, `depot.pull` |
| sporeGate | Public entry, Sovereign CI | LAN + WG | `http.proxy`, `build.release` |
| eastGate | Overwatch, primalSpring | LAN + WG | `mesh.coordinate`, `validate.all` |
| flockGate | Tower atomic evolution | WG (WAN) | `songbird.dev`, `beardog.dev`, `skunkbat.dev` |
| ironGate | GPU compute (RTX 5070) | LAN + WG | `compute.gpu`, `jupyter.execute` |
| grapheneGate | Portable trust anchor | ADB (USB) | `auth.attest`, `tower.compose` |
| strandGate | CPU compute (EPYC) | LAN (joining) | `compute.cpu`, `star.align` |

## Key Invariants

- **No single point of failure**: unplugging any gate does not kill the network. The Flint edge router is the membrane; gates are ephemeral compute.
- **songBird mesh consensus**: each gate's songBird maintains bilateral peer state. No central registry — peers discover each other via `peer.connect` and `mesh.init`.
- **Security fail-closed**: unknown peers are rejected. Trust flows through {{ entity(name="beardog") }} BTSP exchange and trusted issuer registry.
- **Zero exposed ports**: all inter-gate traffic flows through songBird mesh or WireGuard. Services never bind to public interfaces.

## Topology Evolution

The mesh grows by enrollment:

```
New hardware arrives
  → Install NUCLEUS (cascade from depot)
  → songBird starts, calls peer.connect to known seeds
  → Bilateral BTSP exchange establishes trust
  → primal.announce registers capabilities
  → Mesh absorbs — routing tables updated across all peers
```

strandGate (64-core EPYC, 256GB) will follow this pattern when hardware arrives. Future NUCs, Raspberry Pis, or cloud VMs join identically.
