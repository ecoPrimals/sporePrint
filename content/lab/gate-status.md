+++
title = "Gate Status"
description = "Current fleet status — 11 gates online, NUCLEUS 13/13 GREEN, 135K+ tests. K-Derm DNS COMPLETE. nestgate.io LIVE."
date = 2026-08-04
weight = 2

[extra]
maturity = "live"
+++

Current fleet status as of August 4, 2026 (Wave 155v/156d). K-Derm DNS
separation COMPLETE — three-domain topology live. When petalTongue G19
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
| **eastGate** | Overwatch | — | squirrel (pushed 156d), sovereignty cleanup |
| **northGate** | Windows dev | RTX 5090 | Daily driver, AlphaFold data source |
| **grapheneGate** | Mobile | Pixel 8a | Tower (TCP), beacon seed |
| **golgi** | VPS relay | VPS | Forgejo + depot + sporePrint (thin-relay composition) |

## NUCLEUS Health (13/13)

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
├── esotericWebb (CRPG) — V30d, 677 tests, G19 scene push PROVEN
└── footPrint (GIS) — 563 tests, nestGate CAS + petalTongue RPC wired
```

**G19 MILESTONE**: petalTongue scene push is firing on ironGate —
esotericWebb exp006 went from 21/22 PASS to 22/22 PASS. Game scenes
pushed via `visualization.render.scene` through NUCLEUS IPC to RTX 5070.

## Primal Health Dashboard

| Primal | Tests | Health | Recent |
|--------|-------|--------|--------|
| songBird | 14,840+ | GREEN | mesh probes shipped |
| bearDog | 14,019 | GREEN | — |
| nestGate | 13,095+ | GREEN | `content.fetch` (HTTP→BLAKE3→CAS atomic) |
| toadStool | 9,193+ | GREEN | 48 dead deps removed |
| biomeOS | 8,570+ | GREEN | 8 signal graphs wired |
| petalTongue | 6,755 | GREEN | 0 doc warnings |
| barraCuda | 5,037+ | **GREEN** | **PRNG half-range FIXED** (was YELLOW) |
| squirrel | 4,613 | GREEN | test perf 400s→16s, 34→1 binaries |
| coralReef | 3,512 | GREEN | ShaderInfo dedup, 156b debt pass |
| rhizoCrypt | 1,900 | GREEN | G31 batch provenance pipeline |
| loamSpine | 1,740 | GREEN | certificate.history RPC |
| sweetGrass | 1,644 | GREEN | zero-copy Arc\<str\>, batch pipeline |
| tideGlass | 147 | GREEN | full Rust rebuild, 9 crates, 92.71% coverage |
| cellMembrane | 1,281+ | GREEN | — |

**Total**: ~135,000+ tests. **13/13 GREEN.** barraCuda PRNG FIXED (YELLOW→GREEN).

## K-Derm DNS Separation — Three-Domain Topology

K-Derm DNS separation is COMPLETE as of Wave 155v/156d:

| Domain | Layer | DNS | Purpose |
|--------|-------|-----|---------|
| **primals.eco** | Outer membrane | Cloudflare | Public site, 14 Caddy-routed subdomains |
| **nestgate.io** | Peptidoglycan | Sovereign Knot DNS + DNSSEC | Data identity surface, petalTongue mesh |
| **primal.eco** | Inner membrane | Sovereign Knot DNS (LAN only) | Internal mesh — 6 public A records REMOVED |

DNSSEC chain verified end-to-end (DS 2371/13/2). Wildcard `*.primals.eco`
means sporeGate owns all subdomain routing autonomously.

## Provenance Pipeline — 122× Throughput

**Discovery (Wave 155u)**: Inline provenance during bulk data download caused
a 12× throughput collapse (74 files/s → 6 files/s). **Resolution (Wave 155v)**:
Trailer pattern (download fast, braid later) achieved **122× improvement**.
Batch RPCs (`dag.event.batch` + `spine.entry.batch`) are the permanent fix.

Three data provenance states on westGate — convergence path defined.
`is_dataset_converged()` gate for springs.

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
