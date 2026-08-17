+++
title = "Self-Hosted Distributed Scientific Compute Mesh — Gate Topology"
description = "12 operational gates across 6 OS families, 10G backbone, Tower Atomic mesh. NUCLEUS running on 6 gates. 3/3 sub-builders enmeshed. Capability-aware routing."
date = 2026-08-17
weight = 20

[taxonomies]
primals = ["beardog", "biomeos", "songbird"]

[extra]
domain = "Architecture"
maturity = "live"
+++

## Overview

> **Status (Wave 157k):** 12 operational gates across 6 OS families (Linux, Windows, Darwin, Android, iOS, SteamOS). NUCLEUS confirmed on 6 gates (eastGate, ironGate, strandGate, westGate, graftGate, southGate). 3/3 sub-builders enmeshed. NanoWire SSH Tier 1 RETIRED. bearDog `crypto.sign` LIVE on all Tower gates.

The ecoPrimals gate mesh is a sovereign, self-hosted network of compute gates connected via [Tower Atomic](@/architecture/tower_atomic.md) transport coordinated through {{ entity(name="songbird") }}. Each gate runs a NUCLEUS composition and participates in capability-based routing — no centralized orchestrator, no exposed ports.

Tower Atomic runs in production. On LAN, Tower uses direct TCP (topology-aware path selection) — same-switch gates communicate at 0.57ms. On degraded WAN paths, Tower sustains ~1.7× throughput via adaptive retry. Shadow benchmark data collected continuously across the mesh.

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

## Enrolled Gates — 12 ONLINE

| Gate | Platform | Composition | Status |
|------|----------|-------------|--------|
| **eastGate** | Linux | Full NUCLEUS + overwatch | **ONLINE** — rootPulse 6/6, bonsai-bt exp125 |
| **ironGate** | Linux | Full NUCLEUS + 14TB CAS | **ONLINE** — 13/13, 2ms dispatch, 4 mesh peers |
| **strandGate** | Linux | Full NUCLEUS + dual EPYC | **ONLINE** — DF64 shaders SHIPPED, arXiv ACTIVE |
| **westGate** | Linux | Full NUCLEUS + 50.7TB ZFS | **ONLINE** — AlphaFold ingress, rootPulse handlers |
| **sporeGate** | Linux | Foreman + depot | **ONLINE** — 13/13 x86_64 CURRENT, cascade autonomous |
| **blueGate** | Windows | ENMESHED | **ONLINE** — builder.serve :9800, depot 0/13 STALE |
| **graftGate** | Darwin (M4) | FULL NUCLEUS | **ONLINE** — 16/16 depot CURRENT, builder.serve :9800 |
| **southGate** | Linux | NUCLEUS + canary | **ONLINE** — neuralSpring 71/80, SSH ready |
| **biomeGate** | Linux | Tower 4/4 + Node Atomic | **ONLINE** — Titan V Tier 1 CONFIRMED |
| **grapheneGate** | Android | Tower Atomic | **ONLINE** — ADB deploy, Pixel 8a |
| **iosGate** | iOS | BearDogApp | **ONLINE** — 6th OS family, iPhone XS |
| **steamGate** | SteamOS | Tower Atomic | **ONLINE** — portable compute, Steam Deck |

### Physical Topology

```
House 1 (CRS310 backbone — 1G MikroTik):
  sporeGate, eastGate, biomeGate(Titan V)
  Peptidoglycan anchor: sporeGate

House 2 (Omada SX3008F — 10G):
  ironGate, strandGate(COMPUTE LIVE), westGate(50.7TB ZFS),
  blueGate(Windows), southGate(canary), graftGate(Darwin M4)
  Peptidoglycan anchor: blueGate

Link: 80m 10G AOC trunk between adjacent lots

Remote:
  golgiBody (VPS — depot relay, Forgejo)
  grapheneGate (Android — mobile)
  iosGate (iOS — mobile)
  steamGate (SteamOS — portable)
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

- The mesh currently runs on 12 gates across 2 physical sites + remote devices; multi-continent deployment is untested
- USB enrollment assumes a trusted physical carrier (no remote enrollment yet)
- LAN advantage is a topology difference — Tower routes locally, avoiding VPS round-trips
- No web dashboard; all monitoring is via CLI and JSON-RPC
- blueGate depot 0/13 STALE — Windows autonomous dispatch pending

**Hardware**: MikroTik 1G + Omada 10G switches, consumer hardware across 6 OS families  
**Date**: August 17, 2026 (Wave 157k)  
**Author**: ecoPrimal ([ORCID 0009-0004-2141-0321](https://orcid.org/0009-0004-2141-0321))
