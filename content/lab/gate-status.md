+++
title = "Gate Status"
description = "Current fleet status — 11 gates online, 13/13 GREEN, 135K+ tests. G68 COMPLETE — 16/16 prod-clean. Depot current. SSH key discipline enforced."
date = 2026-08-08
weight = 2

[extra]
maturity = "live"
+++

Current fleet status as of August 8, 2026 (Wave 157a — G68 convergence + SSH key
discipline). All primal teams clear. Depot current on golgi. Gate redeploy to
modern G68-converged binaries next.

## G68 Convergence — 16/16 Prod-Clean

Every primal and cellMembrane has zero production G68 violations (sourDough
scanner v2). 205→0 production violations. The ecosystem is deploy-ready.

| Level | Primals |
|-------|---------|
| **G68** (zero violations) | sourDough, nestGate, petalTongue, bingoCube, loamSpine, barraCuda, cellMembrane, +1 |
| **G68-prod** (test-only) | squirrel, bearDog, songBird, rhizoCrypt, skunkBat, sweetGrass, coralReef, biomeOS, toadStool |

## Depot — All Current on golgi

| Target | Binaries | Status |
|--------|----------|--------|
| **Musl** | 17/17 | At Forgejo HEAD (inc. toadStool S369) |
| **Windows** | 15/15 | squirrel.exe added this wave |

toadStool S369: full cross-arch (15/15 targets + iOS). Cascade timer:
synced=15, zero drift.

## Gate Fleet

| Gate | NUCLEUS | Status |
|------|---------|--------|
| **sporeGate** | 13/13 ALIVE v4.57+ | **Current.** 3 cascade cycles validated. toadStool S369 deployed. |
| **ironGate** | 10/10 v4.57+ | G18 dispatch LIVE. 12.7 TB CAS. Redeploy from depot next. |
| **westGate** | 14/14 v4.57 | 3.21 TB / 153 datasets. Redeploy from depot next. |
| **strandGate** | v4.57+ | GPU QCD production. Redeploy from depot next. |
| **blueGate** | 14/14 v4.57+ | Redeploy from depot next. |
| **southGate** | 13/13 v4.57+ | Redeploy from depot next. |
| **biomeGate** | Source-built | GPU lab. 3 VFIO GPUs. |
| **golgi** | Thin relay | Forgejo + depot + sporePrint. Caddy routing. |
| **eastGate** | Overwatch | GitHub SSH **REVOKED** — Forgejo only. |
| **northGate** | — | Daily driver. RTX 5090. |
| **grapheneGate** | Tower | Mobile. Pixel 8a. |

## SSH Key Discipline — K-Derm Relay Enforced

GitHub direct access cut from eastGate (Wave 157a). `github` remotes removed
from all 23 repos. All gates route through the K-Derm relay chain:

```
gate → Forgejo (inner) → pepti (peptidoglycan) → golgi-ext (outer) → GitHub
```

| Entity | Forgejo SSH | GitHub SSH | Role |
|--------|-------------|------------|------|
| **golgi** | YES (host) | NO | Sole sovereign Git store |
| **golgi-ext** | NO | **YES (sole writer)** | K-Derm relay |
| **eastGate** | YES (key `eastGate`) | **REVOKED** | Overwatch |
| **All other gates** | YES (per-gate key) | NO | Inner membrane |

## Phase Execution Status

### Phase 1: Cell Boot — SUCCEEDED
First-ever cell attachment on ironGate. esotericWebb exp006 21/22 PASS.

### Phase 2: footPrint — DEPLOYED + LIVE
708 tests. `footprint.primals.eco` → ironGate :3002 via golgi Caddy.

### Phase 3: squirrel + petalTongue — G18 LIVE
9 primal providers on ironGate. petalTongue G19 render — NEXT.

### Phase 4: westGate Science Springs — UNBLOCKED
tideGlass GPS data converted. Cell TOMLs ready. Awaits gate redeploy.

### Phase 5: Inter-gate Mesh — FUTURE
songBird probes + nestGate `content.fetch` ready.

## Primal Health Dashboard

| Primal | Tests | Health | Recent |
|--------|-------|--------|--------|
| songBird | 14,840+ | GREEN | 22 drawbridge bonds. LAN-first Tower. |
| bearDog | 14,019 | GREEN | — |
| nestGate | 13,095+ | GREEN | `content.query` SHIPPED. nestgate.io wired. |
| toadStool | 9,193+ | GREEN | **S369: 15/15 cross-arch + iOS.** |
| biomeOS | 8,570+ | GREEN | v4.57.0 Stage 2. Cell boot SUCCEEDED. |
| petalTongue | 6,755 | GREEN | nestgate.io 10/12 dashboard sections. |
| barraCuda | 4,959 | GREEN | MultiDevicePool. Cross-vendor. |
| squirrel | 4,613 | GREEN | 156d sovereignty. G68 prod-clean. |
| coralReef | 3,512 | GREEN | G68 prod-clean. |
| rhizoCrypt | 1,791 | GREEN | G63 SO\_PEERCRED. G68 prod-clean. |
| loamSpine | 1,740 | GREEN | G68 zero violations. |
| sweetGrass | 1,636 | GREEN | `capability.call` handler SHIPPED. |
| tideGlass | 214 | GREEN | 17 IPC methods. GPS converted. |
| cellMembrane | 1,327 | GREEN | Platform abstraction (15 cfg→3). G68 zero. |

**Total**: ~135,000+ tests. **13/13 GREEN.** 16/16 G68 prod-clean.

## Live Trust Surfaces

| Site | URL | Status |
|------|-----|--------|
| **sporePrint** | `sporeprint.primals.eco` | **LIVE** — 338 pages, science content |
| **footPrint** | `footprint.primals.eco` | **LIVE** — CAS works, map + agent bridge |
| **nestgate.io** | `nestgate.io` | **LIVE** — 10/12 dashboard sections. Data braids NOT live. |
| **esotericWebb** | `webb.primals.eco` | 502 — needs petalTongue WebGL pipeline |

## K-Derm Three-Domain Topology — Fully Operational

| Domain | Layer | DNS | Status |
|--------|-------|-----|--------|
| **primals.eco** | Outer | Cloudflare (wildcard) | **LIVE** — 14 Caddy routes |
| **nestgate.io** | Peptidoglycan | Sovereign Knot DNS + DNSSEC | **LIVE** — 10/12 sections |
| **primal.eco** | Inner | Sovereign Knot DNS (zero public) | **LIVE** — dnsmasq, all 11 gates |

## Network

- **BTSP**: 13/13 primals using BearDog-native TLS (no OpenSSL)
- **Tower Atomic**: 353× faster than WG on LAN. All components shipped.
- **songBird drawbridge**: 22 bonds, inter-gate `content.get` dispatch validated
- **SSH discipline**: GitHub access via K-Derm relay only (golgi-ext sole writer)

## Pending: Live Dashboard

This page currently shows static data. When petalTongue G19 rendering
matures, it will serve real-time health data from `biomeOS neuralAPI`.
