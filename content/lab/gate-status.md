+++
title = "Gate Status"
description = "Current fleet status — 6/6 NUCLEUS, 4-gate gossip mesh LIVE, ~150K+ tests. Zero P0. Stadial shift: G72 Dependency Pandemic (664 Cargo.toml audited). Primals shed vestigial deps."
date = 2026-08-10
weight = 2

[extra]
maturity = "live"
+++

Current fleet status as of August 10, 2026 (Wave 157g — Stadial Shift).
4-gate gossip mesh LIVE. G72 Dependency Pandemic formalized. Primals shed vestigial
dependencies as compositions close the gaps they filled. Zero P0.

## Stadial → Interstadial — Climate Shift

Dependencies accumulated during the Aug 2025 stadial are now metabolically expensive.
Compositions have closed the gaps they filled. G72 Dependency Pandemic: 664 Cargo.toml
audited, 3-tier excision plan (`specs/DEPENDENCY_PANDEMIC_SPEC.md`).

| System | Status | Evidence |
|--------|--------|----------|
| **All P0s** | **RESOLVED** | P0-A bearDog, P0-B nestGate, P0-C biomeOS — all IN DEPOT. |
| **Gossip mesh** | **4-GATE LIVE** | westGate → sporeGate, eastGate, strandGate. Epidemic propagation confirmed. |
| **G72** | **ACTIVE** | 664 Cargo.toml audited. 3-tier excision. Young primals (swarmVine: 113 deps) already lean. |
| **Build system** | **MESH-NATIVE** | blueGate `builder.serve :9800` — Tower Atomic dispatch. |
| **Depot** | **UNIFIED + PRUNED** | 60 primal binaries, 4 arches, BLAKE3SUMS. G69 lineage. |
| **Neural API** | **UNBLOCKED** | `capability.call` fleet-wide. 13,910 caps. |
| **sourDough CI** | **SHIPPED** (partial) | 4 static validators in golgi post-receive (15 repos, advisory). |
| **NUCLEUS manifest** | **CONVERGED** | `biome.yaml` v1 (toadStool S375+S377: 5→2 structs). primalSpring consuming. |
| **Cascade** | **ZERO DRIFT** | 15min auto-cascade. |

## Gate Fleet — 6/6 NUCLEUS — 4-Gate Gossip Mesh

| Gate | Services | Gossip | Key Capability |
|------|----------|--------|---------------|
| **sporeGate** | **15/15** | **MESH** (3-gate) | Depot authority. sourDough CI. Pipeline enmeshed. |
| **strandGate** | **7/7** | **MESH** (3-gate) | Silicon Fold. Production campaign IN PROGRESS. 2 GPUs. |
| **westGate** | **14/14** | **MESH** (outbound) | Data NAS. First cross-gate gossip confirmed. 3.3 TB. |
| **ironGate** | **13/13** | **LISTENING** (0 peers) | Socket fix done. 170 caps. TCP 7800 reachable. |
| **blueGate** | **13/13** | **BLOCKED** | NUCLEUS alive. No swarmVine on Windows. Needs MeshRelay. |
| **southGate** | **13/13** | **BLOCKED** | NUCLEUS healthy. 4 upstream blockers. Needs depot rebuild. |
| **eastGate** | overwatch + primalSpring | **MESH** (3-gate) | Overwatch (gate-agnostic). biome.yaml consumption DONE. |

## Gossip Injection — 3/16 Primals LIVE

| Primal | Status | Events |
|--------|--------|--------|
| **rhizoCrypt** | LIVE | 3 DAG lifecycle events via `gossip.spread` |
| **loamSpine** | LIVE | 4 spine events (`cas.have`, `braid.head`, `spine.sealed`, `anchor.published`) |
| **lithoSpore** | LIVE | 4 validation events via `gossip.spread` |
| **barraCuda** | SPEC | 20 gossip keys documented, hooks pending |

Cross-gate propagation confirmed: westGate → sporeGate/eastGate/strandGate within 30s.

## G72 — Dependency Pandemic

Stadial shift: young primals (swarmVine: 11 tokio files, 113 deps) are already lean;
old primals (petalTongue: 656 deps) converge toward that pattern.

| Tier | Scope | Status |
|------|-------|--------|
| **Tier 1** | pollster in GPU springs (~350 files), trim tokio `["full"]`, dead deps, version align | **HIGH** |
| **Tier 2** | HTTP→songBird/capability.call, axum→0.8, wgpu→28, YAML unify, tokio::sync→std::sync | **MEDIUM** |
| **Tier 3** | sourDough dep validator, archaic pattern excision fleet-wide | **FUTURE** |

toadStool is G72 exemplar: S378 gated ~35k LOC behind `legacy-*` features.
118→~85 tokio files. 9.6+13.1 GiB reclaimed.

## Depot — Unified + Pruned (G69)

| Architecture | Binaries | Status |
|-------------|----------|--------|
| **x86_64-musl** | 19 | Current |
| **x86_64-windows-gnu** | 16 | Current |
| **x86_64-gnu** | 16 | Current |
| **aarch64-musl** | 13 | Partially stale (no ARM64 gates active) |

G69 lineage spec: binary evolution tracked via provenance trio (CAS/spine/braid).

## Primal Health Dashboard

| Primal | Tests | Health | Recent |
|--------|-------|--------|--------|
| songBird | 14,840+ | GREEN | `CanonicalTransport` shipped. 13,910 caps. MeshRelay pending. |
| bearDog | 14,019+ | GREEN | P0-A IN DEPOT. Spine commit signing unblocked. |
| toadStool | 9,193+ | GREEN | S378: ~35k LOC feature-gated. Manifest converged (S377). |
| biomeOS | 8,570+ | GREEN | `capability.call` fleet-wide. Graph executor next. |
| petalTongue | 6,755+ | GREEN | doom-core decoupled. G19 WebGL next. 656 deps → G72 target. |
| barraCuda | 5,025 | GREEN | Silicon Fold ABSORBED. GEMM bridge shipped. 20 gossip keys spec'd. |
| squirrel | 4,613 | GREEN | C8 done (−67K lines). G18 LIVE. |
| coralReef | 3,963 | GREEN | GEMM Phase 2 IPC. SM20 encoder. +147 tests this wave. |
| rhizoCrypt | 1,900 | GREEN | Gossip injection LIVE. 3 DAG lifecycle events. |
| loamSpine | 1,752 | GREEN | Gossip injection LIVE. 4 spine events. |
| sweetGrass | 1,636 | GREEN | `braid.verify` shipped (method #48). Behavioral tests P2. |
| cellMembrane | 1,353 | GREEN | 13-commit evolution. G69 complete. sourDough CI wired. |
| nestGate | 1,630+ | GREEN | HTTP transport parity. `dataset.convergence` shipped. |
| skunkBat | 675 | GREEN | RPC verified. Registry synced. |
| sourDough | 518 | GREEN | CI shipped (4 static validators). `rpc-surface` audit tool. |
| tideGlass | 214 | GREEN | 17 IPC methods. GPS converted. |
| swarmVine | 124 | GREEN | 113 deps (already lean). Socket discovery FIXED. |

**Total**: ~150,000+ tests. **13/13 GREEN.** 16/16 G68 prod-clean.

## Three-Pillar Architecture

### Pillar 1: Neural API (The Brain)
`capability.call` routing OPERATIONAL (1.3ms / 4ms). `biome.yaml` manifest CONVERGED
(toadStool S377). primalSpring consuming: `biome-eastgate.yaml`, 14 primals, 3
compositions. exp122 37/37 PASS. Next: multi-composition graph workflows.

### Pillar 2: Data Federation (The Nervous System)
CAS federation LIVE. Gossip injection 3/16 primals. 86/87 braid pen test PASS.
`braid.verify` atomic shipped (sweetGrass). Remaining jelly: `native_braid.py`
(1,259 LOC Python → Rust).

### Pillar 3: Pepti Layer (The Skeleton)
Deployment solved. golgiBody = peptidoglycan relay. Sub-builders compile. Gates pull.
Auto-prune. CAS archival operational (G69 Phase 1+2+3).

## Live Sites

| Site | URL | Status |
|------|-----|--------|
| **sporePrint** | `sporeprint.primals.eco` | **LIVE** — 338 pages |
| **footPrint** | `footprint.primals.eco` | **LIVE** — CAS works |
| **nestgate.io** | `nestgate.io` | **LIVE** — trust surfaces + data braids |
| **esotericWebb** | `webb.primals.eco` | 502 — needs petalTongue WebGL (G19) |

## Pending: Live Dashboard

This page currently shows static data. When petalTongue G19 rendering
matures, it will serve real-time health data from `biomeOS neuralAPI`.
