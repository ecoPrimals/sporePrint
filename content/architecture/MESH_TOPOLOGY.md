+++
title = "Self-Hosted Distributed Scientific Compute Mesh — Gate Topology"
description = "Self-hosted distributed scientific computing on commodity hardware. Multi-gate encrypted mesh with 353× LAN throughput advantage, capability-aware routing, and USB enrollment."
date = 2026-07-07
weight = 20

[taxonomies]
primals = ["beardog", "biomeos", "songbird"]

[extra]
domain = "Architecture"
maturity = "implemented"
+++

## Overview

The ecoPrimals gate mesh is a sovereign, self-hosted network of compute gates connected via [Tower Atomic](@/architecture/tower_atomic.md) transport (and legacy WireGuard overlay) coordinated through {{ entity(name="songbird") }}. Each gate runs a NUCLEUS composition and participates in capability-based routing — no centralized orchestrator, no exposed ports.

Tower Atomic runs alongside WireGuard in shadow mode — **353× faster** on LAN via topology awareness and **1.7× sustained** on degraded WAN paths. 360+ shadow benchmark files collected continuously across the mesh.

{{ viz_embed(src="/viz/gate-mesh?live=true", caption="Gate mesh topology: eastGate, sporeGate, golgi, and WireGuard overlay connections") }}

## How Gates Connect

Peers discover each other through four path types, selected by songBird at runtime:

| Path Type | Mechanism | Latency | When Used |
|-----------|-----------|---------|-----------|
| **LAN direct** | `lan_addr` in peers.toml → TCP | <1ms | Same MikroTik switch |
| **Tower Atomic** | Encrypted TCP via songBird mesh | 0.6ms LAN, 60ms WAN | All inter-gate communication |
| **WireGuard overlay** | `10.13.37.0/24` via golgi hub | 5-30ms LAN, 67-154ms WAN | Legacy (being replaced by Tower) |
| **TURN relay** | NAT traversal fallback via golgiBody | 50-200ms | Hostile NAT, mobile |

songBird discovers LAN peers via `lan_addr` and routes directly — bypassing the VPS entirely. This is the core advantage over WireGuard: same-switch gates communicate at 0.57ms instead of 153ms through the VPS hub.

### The 353× Gap

On the same LAN, WireGuard routes sporeGate↔eastGate traffic through golgiBody VPS (153ms) because WG has no concept of LAN topology. Tower discovers LAN peers and routes directly: **0.57ms vs 153ms**.

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

| Gate | Platform | Role | Status |
|------|----------|------|--------|
| **golgiBody** | Linux (VPS) | Sole depot, enrollment endpoint, TURN relay, Forgejo, DNSSEC | **ONLINE** |
| **sporeGate** | Linux | Build authority, genomeBin harvester, cascade hub | **ONLINE** |
| **eastGate** | Linux | Code hub, overwatch, coordination | **ONLINE** |
| **ironGate** | Linux | 4x HDD (14TB+), JupyterHub, GPU compute | **ONLINE** |
| **flockGate** | Linux | Nest Atomic Phase 0 validation, nestGate BTSP | **ONLINE** |
| **grapheneGate** | Android | Tower LIVE, G2: mobile trust boundary | **ONLINE** |
| **northGate** | Windows | RTX 5090, G1: Tower on Windows validation | **ONLINE** |
| **strandGate** | Linux | Dual EPYC 7452, 256GB, RTX 3090 — bioinformatics compute | **HW READY** |
| **westGate** | Linux | 5x14TB (70TB raw) — ZFS cold pool, NestGate CAS | **HW READY** |
| **blueGate** | Windows | Flint2 2.5G — distributed builder, G1 proof | **HW READY** |
| **swiftGate** | Windows | Flint2 2.5G — full NUCLEUS on Windows | **HW READY** |
| **southGate** | Linux | Omada 10G — second sovereign site | **HW READY** |
| fieldGate | — | Dead CMOS | Offline |
| biomeGate | — | Kernel recovery | Offline |

### Physical Topology

```
House 1 (CRS310 backbone — 1G MikroTik):
  sporeGate, eastGate, northGate, biomeGate(offline)

House 2 (Omada SX3008F — 10G):
  ironGate, strandGate(HW ready), westGate(HW ready),
  blueGate(HW ready), swiftGate(HW ready),
  southGate(HW ready), fieldGate(offline)

Link: 80m 10G AOC trunk between adjacent lots

Remote:
  golgiBody (VPS — sole depot)
  flockGate (WAN — Tower primal teams)
  grapheneGate (Android — mobile)
```

## Key Invariants

- **No single point of failure**: unplugging any gate does not kill the network. The Flint edge router is the membrane; gates are ephemeral compute.
- **songBird mesh consensus**: each gate's songBird maintains bilateral peer state. No central registry — peers discover each other via `peer.connect` and `mesh.init`.
- **Security fail-closed**: unknown peers are rejected. Trust flows through {{ entity(name="beardog") }} BTSP exchange and trusted issuer registry.
- **Zero exposed ports**: all inter-gate traffic flows through songBird mesh or WireGuard. Services never bind to public interfaces.

## Topology Evolution

The mesh grows by autonomous enrollment (F10 fossilized):

```
New hardware arrives
  → gate-enroll.sh (Linux) or gate-enroll.ps1 (Windows)
  → WG peer registered, Forgejo SSH key, family seed delivered
  → Clone 43+ repos from Forgejo over mesh
  → membrane gate.bootstrap → fetch genomeBins from golgiBody depot
  → primalSpring scenarios pass → head published → ONLINE
  → Self-registration — gates declare name + composition
```

strandGate (64-core EPYC, 256GB, House 2) will follow this pattern once SSH
access is established. fieldGate and future NUCs, Raspberry Pis, or cloud VMs
join identically — the mesh absorbs any hardware that runs NUCLEUS.

### USB Enrollment (Offline)

Gates can also be enrolled offline via USB:

```bash
gate-usb-bootstrap.sh   # Prepare USB with WG keys, primal binaries, MitoBeacon identity
stage_usb.sh --enroll    # Enroll the gate from USB
```

The USB carries WireGuard keys, RustDesk credentials, primal binaries, MitoBeacon
identity, and `peers.toml`. Gates join the mesh without any network access to the hub.

## Traffic Classes

{{ entity(name="songbird") }} routes 6 traffic classes to specialized provider stacks:

| Class | Provider | Socket |
|-------|----------|--------|
| SECURITY | {{ entity(name="skunkbat") }} | skunkbat.sock |
| HEALTH | {{ entity(name="sweetgrass") }} | sweetgrass.sock |
| PROVENANCE | {{ entity(name="sweetgrass") }} | sweetgrass.sock |
| AI/INFER | {{ entity(name="squirrel") }} | squirrel.sock |
| STORAGE | {{ entity(name="nestgate") }} | nestgate.sock |
| VISUAL | {{ entity(name="petaltongue") }} | petaltongue.sock |

WireGuard sends all 6 through the same undifferentiated tunnel. Tower Atomic
routes each class to the correct provider via `capability.call` dispatch.

## Shadow Metrics

Tower shadow deployment collects benchmark data every 60 minutes across all gate
pairs. 360+ benchmark files have been collected, providing continuous parity evidence.
Results are stored in `benchScale/tower_shadow/` and consumed by
{{ entity(name="primalspring") }} validation scenarios.

## Limitations

- The mesh currently runs on 6 gates across 2 physical sites; multi-continent deployment is untested
- USB enrollment assumes a trusted physical carrier (no remote enrollment yet)
- 353× LAN advantage is measured on 1G links; 10G backbone testing is in progress
- No web dashboard; all monitoring is via CLI and JSON-RPC
- Tower Atomic source code: songBird is public, bearDog and skunkBat are public (AGPL-3.0)

**Hardware**: MikroTik 1G switches, consumer x86_64 Linux boxes, WireGuard baseline  
**Date**: July 2026  
**Author**: ecoPrimal ([ORCID 0009-0004-2141-0321](https://orcid.org/0009-0004-2141-0321))
