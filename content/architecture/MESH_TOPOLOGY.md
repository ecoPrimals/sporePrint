+++
title = "Self-Hosted Distributed Scientific Compute Mesh — Gate Topology"
description = "10 operational gates, 10G backbone, WireGuard + Tower Atomic mesh. NUCLEUS running on 3 gates. Capability-aware routing, USB enrollment."
date = 2026-07-31
weight = 20

[taxonomies]
primals = ["beardog", "biomeos", "songbird"]

[extra]
domain = "Architecture"
maturity = "live"
+++

## Overview

> **Status (Wave 155n):** 10 operational gates (9 active + southGate validation). NUCLEUS confirmed on 3 gates (westGate, blueGate, strandGate). 10G MikroTik backbone. WireGuard overlay + Tower Atomic in shadow mode. bearDog `crypto.sign` LIVE on all Tower gates.

The ecoPrimals gate mesh is a sovereign, self-hosted network of compute gates connected via [Tower Atomic](@/architecture/tower_atomic.md) transport (and legacy WireGuard overlay) coordinated through {{ entity(name="songbird") }}. Each gate runs a NUCLEUS composition and participates in capability-based routing — no centralized orchestrator, no exposed ports.

Tower Atomic runs alongside WireGuard in shadow mode. On LAN, Tower uses direct TCP (topology-aware path selection) while WireGuard routes through the overlay — so Tower avoids overhead that WireGuard was never designed to avoid. On degraded WAN paths, Tower sustains ~1.7× WireGuard throughput via adaptive retry. 360+ shadow benchmark files collected continuously across the mesh.

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

### Why Tower Exists Alongside WireGuard

WireGuard is an excellent VPN — but it's a VPN, not a LAN-aware mesh. It has no concept of network topology: two gates on the same switch still route through the VPS hub (153ms round-trip). Tower discovers LAN peers via `lan_addr` and routes directly (0.57ms). This isn't "faster than WireGuard" — it's solving a different problem (topology-aware routing) that WireGuard doesn't attempt.

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
| **golgiBody** | Linux (VPS) | Sole depot (39 genomeBins), enrollment endpoint, Forgejo, DNSSEC | **ONLINE** |
| **sporeGate** | Linux | Build authority, genomeBin harvester, depot rebuild | **ONLINE** |
| **eastGate** | Linux | Code hub, overwatch, biomeOS evolution | **ONLINE** |
| **westGate** | Linux | **Nest Atomic LIVE** — 8 services, 1,704 capabilities, ZFS 25.4TB + 2TB L2ARC | **ONLINE** |
| **strandGate** | Linux | **Tower+Compute LIVE** — Dual EPYC, 256GB, RTX 3090, Compute Trio | **ONLINE** |
| **ironGate** | Linux | 4x HDD (14TB+), HDD enclave experiment | **ONLINE** |
| **flockGate** | Linux | Nest Atomic validation | **ONLINE** |
| **grapheneGate** | Android | Tower LIVE, G2: mobile trust boundary | **ONLINE** |
| **northGate** | Windows | RTX 5090, AlphaFold source (~1TB), G1 target | **ONLINE** |
| **blueGate** | Windows | G1: Tower on Windows, peptidoglycan anchor H2 | **ONLINE** |
| **swiftGate** | Windows | G1: Tower on Windows | **ONLINE** |
| **southGate** | Linux | Omada 10G — enrollment pending | **HW READY** |
| fieldGate | — | Dead CMOS | Offline |
| biomeGate | — | Kernel recovery | Offline |

### Physical Topology

```
House 1 (CRS310 backbone — 1G MikroTik):
  sporeGate, eastGate, northGate, biomeGate(offline)
  Peptidoglycan anchor: sporeGate

House 2 (Omada SX3008F — 10G):
  ironGate, strandGate(COMPUTE LIVE), westGate(NEST ATOMIC LIVE),
  blueGate(ONLINE), swiftGate(ONLINE),
  southGate(HW ready), fieldGate(offline)
  Peptidoglycan anchor: blueGate

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
- LAN advantage over WireGuard is a topology difference, not a protocol speed difference — Tower routes locally, WireGuard routes through VPS
- No web dashboard; all monitoring is via CLI and JSON-RPC
- Tower Atomic source code: songBird is public, bearDog and skunkBat are public (AGPL-3.0)

**Hardware**: MikroTik 1G switches, consumer x86_64 Linux boxes, WireGuard baseline  
**Date**: July 2026  
**Author**: ecoPrimal ([ORCID 0009-0004-2141-0321](https://orcid.org/0009-0004-2141-0321))
