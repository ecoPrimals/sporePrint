+++
title = "Gate Status"
description = "Current fleet status — 10 gates online, 4 running NUCLEUS, biomeOS v4.56 orchestrating 244 capabilities."
date = 2026-08-01
weight = 2

[extra]
maturity = "live"
+++

Current fleet status as of August 2026. These numbers come from `biomeOS neuralAPI`
health probes on each gate. When petalTongue G19 rendering lands, this page will
serve live data.

## NUCLEUS Gates (13/13 primals)

| Gate | Hardware | biomeOS | Capabilities | Status |
|------|----------|---------|-------------|--------|
| **westGate** | i9-14900K, 96 GB DDR5, RTX 4070 | v4.56 | 244 | HEALTHY |
| **blueGate** | i9-14900K, 96 GB DDR5 | v4.56 | 244 | HEALTHY |
| **strandGate** | Ryzen 9 5900X, 64 GB, RTX 3090 | v4.56 | 244 | HEALTHY |
| **southGate** | (validation gate) | v4.56 | 244 | ENROLLED |

## Tower Gates (transport mesh)

| Gate | Role | Platform | Mesh |
|------|------|----------|------|
| eastGate | Overwatch, dev primary | Linux | WireGuard |
| ironGate | Windows parity | Windows 11 | WireGuard |
| sandGate | Android parity | Android | WireGuard |
| nestGate | Data services, CAS | Linux | WireGuard |
| sporeGate | CI builds, depot | Linux | WireGuard |
| golgiBody | WAN serving (primals.eco) | Linux VPS | External |

## What "HEALTHY" Means

Each NUCLEUS gate runs the full 13-primal composition:

```
cellMembrane → biomeOS → songBird → bearDog → skunkBat →
toadStool → barraCuda → coralReef → rhizoCrypt → loamSpine →
sweetGrass → nestGate → squirrel
```

biomeOS `neuralAPI` probes every primal's health endpoint. All 13 must
respond for HEALTHY status. Any failure triggers cellMembrane's crash-loop
breaker for automatic recovery.

## Network

- **Backbone**: 10G between Tower gates on the local mesh
- **WireGuard**: Encrypted overlay connecting all gates
- **Tower Atomic**: bearDog + songBird + skunkBat provide sovereign transport
  with LAN-aware routing (topology awareness for local vs remote traffic)
- **BTSP**: 13/13 primals using BearDog-native TLS (no OpenSSL)

## Pending: Live Dashboard

This page currently shows static data. When petalTongue G19
Node Atomics rendering is complete, it will serve real-time health
data from `biomeOS neuralAPI` via `spore-validate pt-render`.

Data source: `spore-validate nucleus <profile> --probe`
