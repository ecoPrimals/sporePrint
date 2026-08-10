+++
title = "Gate Status"
description = "Current fleet status — 6/6 NUCLEUS, 13/13+ GREEN, 116,930+ tests. Zero P0. Depot unified + pruned (60 binaries, 4 arches). G69 lineage spec. Mesh-native build. Neural API unblocked."
date = 2026-08-09
weight = 2

[extra]
maturity = "live"
+++

Current fleet status as of August 9, 2026 (Wave 157d — Depot Unified + G69 Lineage Spec).
All P0s resolved. Depot unified + pruned. Mesh-native build system. Neural API call
path unblocked. G69 depot lineage spec published.

## Infrastructure Phase — COMPLETE

| System | Status | Evidence |
|--------|--------|----------|
| **All P0s** | **RESOLVED** | P0-A bearDog (`766951004`), P0-B nestGate (stale depot), P0-C biomeOS FD (`6a51638d`). |
| **Build system** | **MESH-NATIVE** | blueGate `builder.serve :9800` — Tower Atomic dispatch. 14/14 vertebrate (23 min). Authorities: `[blueGate, sporeGate, eastGate]`. |
| **Depot** | **UNIFIED + PRUNED** | 60 primal binaries across 4 arches. BLAKE3SUMS all arches. G69 lineage spec. |
| **Neural API** | **CALL PATH UNBLOCKED** | `capability.resolve` (7ms) + `capability.call` (1 pooled conn per forward). |
| **G68** | **16/16 COMPLETE** | 205→0 production violations. |
| **Cascade** | **ZERO DRIFT** | 15min auto-cascade, auto-push, auto-harvest. |
| **SSH discipline** | **ENFORCED** | Zero github remotes. K-Derm relay chain. |

## Gate Fleet — 6/6 NUCLEUS

| Gate | Status | Key Evolution |
|------|--------|---------------|
| **sporeGate** | **15/15 ALIVE** | Topology owner, fallback builder. 13,910 caps. Vine-bat + gossip resolve. |
| **blueGate** | **13/13 ALIVE** | **PRIMARY BUILDER** — 14/14 vertebrate (23 min), mesh-native dispatch :9800. |
| **southGate** | **13/13 ALIVE** | Validation gate. 0.058ms Tower. G17 + G8 proven. |
| **ironGate** | **13/13 ALIVE** | Downstream host. G18 LIVE. esotericWebb V32 CELL. RTX 5070. 12.7 TB CAS. |
| **strandGate** | **13/13 ALIVE** | Silicon Fold + Node Atomic AAR: 15/15 units, AMD 20x root cause. coralReef 18/18 IPC. |
| **westGate** | **13/13 ALIVE** | Data NAS. 3.3 TB / 989K files braided. 2.5 TB CAS federated. |

## Depot — Unified + Pruned (G69)

Depot cleaned: test/demo/bench binaries pruned. Only primal binaries remain.
cellMembrane `depot.prune` (`1e9d32b`) — registry-driven cleanup, `--dry-run`,
BLAKE3SUMS regen.

| Architecture | Binaries | Status |
|-------------|----------|--------|
| **x86_64-musl** | 19 | Current |
| **x86_64-windows-gnu** | 16 | Current |
| **x86_64-gnu** | 16 | Current |
| **aarch64-musl** | 13 | Partially stale (no ARM64 gates active) |

**G69 Depot Lineage Spec**: binary evolution tracked via provenance trio — same
CAS/spine/braid pattern used for data braids. Binary provenance joins the same
verification chain as science data.

## Deployment Discipline — postPrimordial

sporeGate is the sole depot topology owner. blueGate is the primary builder.
Gates pull from golgi via `plasmid.fetch`. No gate self-builds.

```
blueGate builds → sporeGate relay → golgi depot → gates pull
                                                    ↓
                                          BLAKE3SUMS verified
```

## Mesh-Native Build System

blueGate `builder.serve :9800` — Tower Atomic dispatch, no SSH. Authorities:
`[blueGate, sporeGate, eastGate]`. 14/14 vertebrate primals built in 23 minutes.
66 Windows binaries now on golgi.

## Neural API — Call Path Unblocked

P0-C fix deployed fleet-wide. Both call paths operational:
- `capability.resolve` — 7ms (mesh-wide discovery)
- `capability.call` — 1 pooled connection per forward (no FD leak)
- 13,910 capabilities registered (up from 1,987 pre-vertebrate)

## Vertebrate Evolution — Self-Audit Summary

12/16 primals self-audited. Zero phantom methods across all audited primals.

| Primal | Self-Audit | Key Result |
|--------|-----------|------------|
| **bearDog** | DONE | P0-A fix IN DEPOT. Health guard, socket naming. |
| **nestGate** | DONE | P0-B RESOLVED. `content.ingest` + `content.stat` shipped. |
| **biomeOS** | DONE | P0-C fix IN DEPOT. FD leak resolved. |
| **songBird** | DONE | `CanonicalTransport` trait shipped. 9 transports converging. |
| **swarmVine** | DONE | 39→124 tests (82% coverage). Async dispatch. |
| **petalTongue** | DONE | doom-core decoupled (ludoSpring-ready). |
| **skunkBat** | DONE | RPC surface verified, registry synced. |
| **rhizoCrypt** | DONE | 40/40 zero phantoms. `dag.session.tree_hash` fixed. |
| **loamSpine** | DONE | 54/54 JSON-RPC + 37/37 tarpc. `persist_tip` abstraction. |
| **coralReef** | DONE | 18/18 IPC. Integer subgroup fix. 3,702 tests. |
| **barraCuda** | DONE | Silicon Fold ABSORBED. 5 abstractions. 5,025 tests. |
| **cellMembrane** | DONE | G69 `depot.prune`. Deep debt. 1,347 tests. |
| **sourDough** | DONE | `rpc-surface` audit tool shipped. |

**Remaining**: toadStool (S371), sweetGrass, bingoCube.

## Primal Health Dashboard

| Primal | Tests | Health | Recent |
|--------|-------|--------|--------|
| songBird | 14,840+ | GREEN | `CanonicalTransport` shipped. 13,910 caps. |
| bearDog | 14,019+ | GREEN | P0-A IN DEPOT. Spine commit signing unblocked. |
| nestGate | 1,630+ | GREEN | P0-B IN DEPOT. `content.ingest` + `content.stat`. |
| toadStool | 9,193+ | GREEN | S371: WASM split 24/48. Node Atomic AAR. |
| biomeOS | 8,570+ | GREEN | P0-C IN DEPOT. `capability.call` fleet-wide. |
| petalTongue | 6,755+ | GREEN | doom-core decoupled. G19 WebGL next. |
| barraCuda | 5,025 | GREEN | **Silicon Fold ABSORBED**. Buffer 512M→1G. |
| squirrel | 4,613 | GREEN | C8 done (−67K lines). G18 LIVE. |
| coralReef | 3,702 | GREEN | 18/18 IPC. Integer subgroup fix. GEMM tiling next. |
| rhizoCrypt | 1,900 | GREEN | 40/40 zero phantoms. Self-audit DONE. |
| loamSpine | 1,752 | GREEN | 91/91 methods verified. `persist_tip`. |
| sweetGrass | 1,636 | GREEN | Self-audit pending. |
| cellMembrane | 1,347 | GREEN | G69 `depot.prune`. Deep debt. |
| skunkBat | 675 | GREEN | RPC verified. Registry synced. |
| sourDough | 518 | GREEN | `rpc-surface` audit tool shipped. |
| tideGlass | 214 | GREEN | 17 IPC methods. GPS converted. |
| swarmVine | 124 | GREEN | 39→124 tests (82%). Windows port handoff filed. |

**Total**: ~116,930+ tests. **13/13 GREEN.** 16/16 G68 prod-clean.

## Live Sites

| Site | URL | Status |
|------|-----|--------|
| **sporePrint** | `sporeprint.primals.eco` | **LIVE** — 338 pages |
| **footPrint** | `footprint.primals.eco` | **LIVE** — CAS works |
| **nestgate.io** | `nestgate.io` | **LIVE** — trust surfaces + data braids |
| **esotericWebb** | `webb.primals.eco` | 502 — needs petalTongue WebGL (G19) |

## Trust Surfaces — LIVE

| Route | Status |
|-------|--------|
| `/api/content/stats` | **LIVE** — rhizoCrypt CAS via UDS |
| `/pseudospore/` | **LIVE** — 5 bundles + QCD v1.0.0-rung1 PACKAGED |
| `/api/pseudospore/bundles` | **LIVE** — bundle listing with provenance |
| `/pseudospore/validate.sh` | **LIVE** — verification script |

## K-Derm Three-Domain Topology — Fully Operational

| Domain | Layer | DNS | Status |
|--------|-------|-----|--------|
| **primals.eco** | Outer | Cloudflare (wildcard) | **LIVE** — 14 Caddy routes |
| **nestgate.io** | Peptidoglycan | Sovereign Knot DNS + DNSSEC | **LIVE** — trust surfaces |
| **primal.eco** | Inner | Sovereign Knot DNS (zero public) | **LIVE** — all 11 gates |

## Phase Execution Status

### Phase 1: Cell Boot — SUCCEEDED
First-ever cell attachment on ironGate. esotericWebb exp006 21/22 PASS.

### Phase 2: footPrint — DEPLOYED + LIVE
708 tests. `footprint.primals.eco` → ironGate :3002 via golgi Caddy.

### Phase 3: squirrel + petalTongue — G18 LIVE
9 primal providers on ironGate. petalTongue G19 render — NEXT.

### Phase 4: westGate Science Springs — SIGNING UNBLOCKED
989K files braided. 153 datasets. 3.3 TB. bearDog P0-A IN DEPOT —
spine commit signing unblocked.

### Phase 5: Inter-gate Mesh — NEURAL API UNBLOCKED
P0-C fixed. `capability.call` fleet-wide. 13,910 caps. 11 mesh peers.

## Pending: Live Dashboard

This page currently shows static data. When petalTongue G19 rendering
matures, it will serve real-time health data from `biomeOS neuralAPI`.
