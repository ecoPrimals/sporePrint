+++
title = "Gate Status"
description = "Current fleet status — 11 gates online, NUCLEUS 13/13 GREEN, 135K+ tests. G18 signal dispatch LIVE. Phase 1 cell boot SUCCEEDED. footPrint Phase 2 DEPLOYED."
date = 2026-08-05
weight = 2

[extra]
maturity = "live"
+++

Current fleet status as of August 5, 2026 (Wave 156d — Data Flow Activation Era).
All 6 NUCLEUS gates synced to v4.57+. G18 signal dispatch LIVE on ironGate.
When petalTongue G19 rendering matures, this page will serve live data from
`biomeOS neuralAPI`.

## Gate Fleet — All 6 NUCLEUS Gates v4.57+

| Gate | NUCLEUS | Status |
|------|---------|--------|
| **sporeGate** | 14/14 v4.57+ | Sovereign CI. 52/52 harvest. LAN-first Tower (1ms). |
| **ironGate** | 10/10 v4.57+ | **G18 dispatch LIVE (9 providers). 12.7 TB CAS. songBird federation.** |
| **westGate** | 14/14 v4.57 | 3.21 TB / 153 datasets. GPS converted. `nucleus attach` ready. |
| **strandGate** | v4.57+ (restart deferred) | GPU 100% QCD production. 16⁴ dual-GPU COMPLETE. |
| **blueGate** | 14/14 v4.57+ | Depot sync done. UniBin CLI. |
| **southGate** | 13/13 v4.57+ | Re-validated (97h uptime). Tower 0.15ms, 19 Gbps. |
| **biomeGate** | Source-built | GPU lab. 3 VFIO GPUs. |
| **golgi** | Thin relay | `footprint.primals.eco` → ironGate :3002. |
| **eastGate** | Overwatch | squirrel 156d. Code hub. |
| **northGate** | — | Daily driver. RTX 5090. |
| **grapheneGate** | Tower | Mobile. Pixel 8a. |

## Phase Execution Status

### Phase 1: Cell Boot — SUCCEEDED

`biomeos nucleus attach esotericwebb_cell.toml` on ironGate. **First-ever cell
attachment in the ecosystem.** exp006 21/22 PASS (1 skip from socket path
migration, 0 fail). Scene push to petalTongue firing post-attach.

### Phase 2: footPrint — DEPLOYED + LIVE

systemd active on ironGate. CAS E2E verified (TCP local-trust). 708 tests.
Agent bridge live. **golgi Caddy routing DONE** (`footprint.primals.eco` →
ironGate :3002). Remaining: auto-load default project, squirrel connection,
CSP dedup (Express + Caddy both emit headers).

### Phase 3: squirrel + petalTongue — G18 LIVE

squirrel G18 signal dispatch **LIVE** on ironGate — 9 primal providers,
cross-primal routing validated (squirrel → rhizoCrypt 1ms, squirrel → bearDog
crypto hash). petalTongue G19 live render on RTX 5070 — NEXT.

### Phase 4: westGate Science Springs — UNBLOCKED

westGate v4.57 (14/14 HEALTHY), `nucleus attach` available. tideGlass GPS
data CONVERTED (11 JSON, 103 MB CAS-ingested). Cell TOMLs exist for all 4
springs. **Next**: `biomeos nucleus attach --cell tideglass_cell.toml`.

### Phase 5: Inter-gate Mesh — FUTURE

songBird probes ready, nestGate `content.fetch` ready. Blocks healthSpring,
lithoSpore, neuralSpring, hotSpring, wetSpring.

## ironGate — G18 Signal Dispatch + 12.7 TB CAS

```
squirrel (agent dispatch) → signal.plan + signal.dispatch — 9 providers
    │
biomeOS (composition) → graph.execute + cell graph deploy
    │
petalTongue (rendering) → WebGL/WASM live render on RTX 5070
    │
├── esotericWebb (CRPG) — V31b, 484 tests, cell boot SUCCEEDED
└── footPrint (GIS) — 708 tests, Phase 2 DEPLOYED, CAS E2E
```

**NUCLEUS storage**: 12.7 TB ext4 disk (`/dev/sdc1`) at `/mnt/nestgate`.
nestGate v0.5.0 with BLAKE3 CAS. TCP local-trust + UDS BTSP.
songBird federation to westGate configured (LAN `192.168.4.149:7700`).

## Primal Health Dashboard

| Primal | Tests | Health | Recent |
|--------|-------|--------|--------|
| songBird | 14,840+ | GREEN | 22 drawbridge bonds. LAN-first Tower (1ms). |
| bearDog | 14,019 | GREEN | 94 orphans purged |
| nestGate | 13,095+ | GREEN | **`content.query` SHIPPED.** ZFS REST. nestgate.io wired. |
| toadStool | 9,193+ | GREEN | S351: -48 dead deps. Symlink fix. |
| biomeOS | 8,570+ | GREEN | **v4.57: `nucleus attach` — CELL BOOT SUCCEEDED.** |
| petalTongue | 6,755 | GREEN | nestgate.io branded. TCP hardened. |
| barraCuda | 4,959 | GREEN | MultiDevicePool. Cross-vendor validated. |
| squirrel | 4,613 | GREEN | 156d sovereignty. 27 deprecated aliases removed. |
| coralReef | 3,512 | GREEN | ShaderInfo dedup, identity tests. |
| rhizoCrypt | 1,791 | GREEN | **G63 SO\_PEERCRED SHIPPED.** CAS local-trust. |
| loamSpine | 1,740 | GREEN | OnceLock UID cache. Tower/custodian BTSP. |
| sweetGrass | 1,636 | GREEN | **LedgerClient v0.8.0** → loamSpine. |
| tideGlass | 214 | GREEN | **17 IPC methods.** GPS converted. `content.query` wired. |
| cellMembrane | 1,281+ | GREEN | Harvest scheduler. CI-DIV fixes. |

**Total**: ~135,000+ tests. **13/13 GREEN.**

## Convoy Provenance — 145/s (460× Total)

westGate convoy at **145 files/s** — 4-worker native socket convoy replaces
socat subprocess. 460× total improvement from initial 0.3/s. CAS pool now
452 GB. Disk I/O sole bottleneck (15.4% iowait on spinning raidz1).

Convergence sweep (Aug 4): 0 CONVERGED, 89 PARTIAL, 32 PRIMORDIAL, 5 CAS-ONLY.
7.9M files remaining at ~15h ETA.

## K-Derm Three-Domain Topology — Fully Operational

| Domain | Layer | DNS | Status |
|--------|-------|-----|--------|
| **primals.eco** | Outer | Cloudflare (wildcard) | **LIVE** — 14 Caddy routes |
| **nestgate.io** | Peptidoglycan | Sovereign Knot DNS + DNSSEC | **LIVE** — branded data surface |
| **primal.eco** | Inner | Sovereign Knot DNS (zero public) | **LIVE** — dnsmasq, all 11 gates resolving |

## Live Sites

| Site | URL | Status |
|------|-----|--------|
| **sporePrint** | `sporeprint.primals.eco` | **LIVE** — Zola static, science content |
| **footPrint** | `footprint.primals.eco` | **LIVE** — CAS works, map + agent bridge |
| **nestgate.io** | `nestgate.io` | **LIVE** — dashboard, needs mesh bridge |
| **esotericWebb** | `webb.primals.eco` | 502 — needs petalTongue WebGL pipeline (G19) |

## Network

- **Backbone**: 10G between Tower gates on the local mesh
- **BTSP**: 13/13 primals using BearDog-native TLS (no OpenSSL)
- **Tower Atomic**: 353× faster than WG on LAN. All components shipped.
- **songBird drawbridge**: 22 bonds, inter-gate `content.get` dispatch validated
- **songBird federation**: ironGate → westGate configured (LAN, TCP :7700)
- **Mesh probes**: songBird `mesh.connectivity_check` + `mesh.throughput` SHIPPED

## Pending: Live Dashboard

This page currently shows static data. When petalTongue G19 rendering
matures, it will serve real-time health data from `biomeOS neuralAPI`.

Data source: `spore-validate nucleus <profile> --probe`
