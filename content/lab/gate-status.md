+++
title = "Gate Status"
description = "Current fleet status — 11 gates online, NUCLEUS 26/27 HEALTHY, ironGate downstream host with G19 petalTongue render PROVEN."
date = 2026-08-03
weight = 2

[extra]
maturity = "live"
+++

Current fleet status as of August 3, 2026. When petalTongue G19
rendering matures, this page will serve live data from `biomeOS neuralAPI`.

## Gate Role Taxonomy

| Gate | Role | Hardware | What Runs |
|------|------|----------|-----------|
| **ironGate** | Downstream host | i9-14900K, RTX 5070, 94 GB | esotericWebb + footPrint + squirrel + petalTongue live render |
| **westGate** | Data NAS | i9-14900K, 96 GB DDR5, 50.7 TB ZFS | tideGlass + wetSpring + groundSpring + airSpring (519 GB / 130 datasets) |
| **strandGate** | Compute dev | Dual EPYC 7452, RTX 3090 + RX 6950 XT | hotSpring + neuralSpring + GPU experiment queue |
| **biomeGate** | GPU lab | Threadripper 3970X, 3 VFIO GPUs | G32 silicon deism, coralReef diesel engine, cross-vendor validation |
| **blueGate** | Windows dev | i9-14900K, 96 GB DDR5 | ludoSpring, Windows NUCLEUS, G29 H2 DNS |
| **sporeGate** | CI / membrane | — | Sovereign CI, G34/G35, build authority, depot, DNS |
| **southGate** | Validation | — | NUCLEUS 22/22 reference gate (G17+G8 PROVEN) |
| **eastGate** | Overwatch | — | squirrel local dev, orchestration |
| **northGate** | Windows dev | RTX 5090 | Daily driver, AlphaFold data source |
| **grapheneGate** | Mobile | Pixel 8a | Tower (TCP), beacon seed |
| **golgi** | VPS relay | VPS | Forgejo + depot + sporePrint (thin-relay composition) |

## NUCLEUS Health (26/27)

NUCLEUS composition runs the full 13-primal stack:

```
cellMembrane → biomeOS → songBird → bearDog → skunkBat →
toadStool → barraCuda → coralReef → rhizoCrypt → loamSpine →
sweetGrass → nestGate → squirrel
```

biomeOS `neuralAPI` probes every primal's health endpoint. All 13 must
respond for HEALTHY status. 8/9 primals now compose zero-config.

## ironGate — First Downstream Host

ironGate creates a vertical slice through the entire primal-to-product stack:

```
squirrel (agent dispatch) → signal.plan + signal.dispatch
    │
biomeOS (composition) → graph.execute + cell graph deploy
    │
petalTongue (rendering) → WebGL/WASM live render on RTX 5070
    │
├── esotericWebb (CRPG) — V26, 471 tests, G19 scene push PROVEN
└── footPrint (GIS) — Nest Atomic + drawbridge, 478 TS tests
```

**G19 MILESTONE**: petalTongue scene push is firing on ironGate —
esotericWebb exp006 went from 21/22 PASS to 22/22 PASS. Game scenes
pushed via `visualization.render.scene` through NUCLEUS IPC to RTX 5070.

## Primal Health Dashboard

| Primal | Tests | Health |
|--------|-------|--------|
| songBird | 14,840+ | GREEN |
| bearDog | 14,019 | GREEN |
| nestGate | 13,095+ | GREEN |
| toadStool | 9,193+ | GREEN |
| biomeOS | 8,570+ | GREEN |
| petalTongue | 6,755 | GREEN |
| barraCuda | 5,037 | YELLOW |
| squirrel | 4,613 | GREEN |
| coralReef | 3,553 | GREEN |
| rhizoCrypt | 1,900 | GREEN |
| loamSpine | 1,740 | GREEN |
| sweetGrass | 1,644 | GREEN |
| cellMembrane | 1,281+ | GREEN |

**Total**: ~121,000+ tests. 12/13 GREEN. barraCuda YELLOW (PRNG validation).

## Network

- **Backbone**: 10G between Tower gates on the local mesh
- **BTSP**: 13/13 primals using BearDog-native TLS (no OpenSSL)
- **Tower Atomic**: bearDog + songBird + skunkBat provide sovereign transport
  with LAN-aware routing
- **Mesh probes**: songBird `mesh.connectivity_check` + `mesh.throughput` SHIPPED

## Pending: Live Dashboard

This page currently shows static data. When petalTongue G19 rendering
matures, it will serve real-time health data from `biomeOS neuralAPI`.

Data source: `spore-validate nucleus <profile> --probe`
