+++
title = "Gate Status"
description = "Current fleet status — 6/6 NUCLEUS, 4-gate gossip mesh LIVE, G72 Tier 1 COMPLETE (9/9 teams, ~114 crates shed), gossip 6/16, ~150K+ tests. Zero P0."
date = 2026-08-11
weight = 2

[extra]
maturity = "live"
+++

Current fleet status as of August 11, 2026 (Wave 157i — Pandemic Responds).
G72 Tier 1 COMPLETE: 9/9 teams responded, ~114 crates shed fleet-wide.
Gossip injection expanded to 6/16 primals. hotSpring pseudoSpore E2E shipped.
darwinGate M4 arrived. Zero P0.

## G72 Dependency Pandemic — Tier 1 COMPLETE

All 9 teams responded. ~114 crates shed fleet-wide. toadStool tokio 118→65 files
(45% reduction). Tier 2 queued: HTTP consolidation, axum 0.8, wgpu 28, YAML unification.

| Team | Deps Shed | Impact | Status |
|------|-----------|--------|--------|
| **toadStool** | 7 dead deps removed, 6 promoted to workspace, tokio 118→65 files, plugin-loading/vulkano/core-wgpu excised | ~73 GiB reclaimed. `tokio::fs` eliminated (28 files → `std::fs`). | **G72 EXEMPLAR** |
| **nestGate** | jsonrpsee removed (1,864 LOC), crossbeam umbrella→channel, dead bincode | -10 crates. Deep debt S146 (fake success paths eliminated). | **TIER 1 DONE** |
| **rhizoCrypt** | wiremock removed (0 usage), hashbrown dedup | **-46 crates (14.6%)**. Deep debt sweep, vertex builder extraction. | **TIER 1 DONE** |
| **coralReef** | futures/tokio-util gated behind `tarpc-transport`, tokio/process→dev-deps | Feature-surface trim. `#[allow]→#[expect]` Rust 2024 idiom. | **TIER 1 DONE** |
| **sweetGrass** | tokio `["full"]`→7 features, dead bincode/chrono removed | **P2 braid.verify CLOSED** (5 behavioral tests). Batch+verify submodule extraction. | **TIER 1 DONE** |
| **loamSpine** | url+ICU chain excised, chacha20poly1305 0.10→0.11 | -7 crates. RustCrypto unified. Deep debt + test refactoring. | **TIER 1 DONE** |
| **cellMembrane** | tokio rt-multi-thread→dev-deps, time/macros removed | Socket name dedup (3→1 canonical). NUCLEUS install lifecycle extraction. | **TIER 1 DONE** |
| **tideGlass** | tokio rt-multi-thread→rt (current-thread) | Lean gen5 primal. Already 21 transitive deps. | **TIER 1 DONE** |
| **wetSpring** | Verified clean (pollster removed V211) | Primary work: gossip injection. | **TIER 1 VERIFIED** |

**Tier 2 queued**: HTTP client consolidation (nestGate ureq→songBird, loamSpine ureq→capability.call), axum 0.7→0.8 (5 projects), wgpu 22→28 (toadStool), YAML unification.

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
| **darwinGate** | — | **PENDING** | M4 Mac Mini arrived. Bootstrap imminent. `aarch64-apple-darwin`. |

## Gossip Injection — 6/16 Primals LIVE (was 3/16)

| Entity | Events | Status |
|--------|--------|--------|
| **rhizoCrypt** | 3 DAG lifecycle | LIVE |
| **loamSpine** | 4 spine events | LIVE |
| **lithoSpore** | 4 validation events | LIVE (registry synced) |
| **barraCuda** | **19 runtime events** (compute, tower, shader, dispatch) | **LIVE** |
| **esotericWebb** | 2 session lifecycle | **LIVE** (V33) |
| **songBird** | 1 capability advertise | LIVE |
| **wetSpring** | 2/4 (PipelineComplete, ProvenanceWitness) | PARTIAL |
| **hotSpring** | 0/10 (scaffold, not hooked) | SCAFFOLD |

Cross-gate propagation: 4-gate mesh (sporeGate, eastGate, strandGate, westGate).
ironGate listening, not yet peered. blueGate + southGate blocked (need MeshRelay + depot rebuild).

## Science Pipeline — hotSpring pseudoSpore E2E

Pure Rust pipeline shipped:
- `arxiv_production_campaign` → `arxiv_analysis` → `pseudospore_manifest` → `pseudospore_bundle` → `pseudospore_sign` (bearDog Ed25519) → `pseudospore_register` (westGate CAS + ironGate NFT)
- 32⁴ thermalization fix (dt 0.01→0.005, warmup 500→1500)
- 10 gossip events defined (scaffold — not yet hooked)

## darwinGate — M4 Mac Mini

**Hardware**: M4 Mac Mini (Apple Silicon, aarch64-apple-darwin)
**Network**: iPhone XS tethering via USB
**Role**: First apple-darwin gate. Self-builds `aarch64-apple-darwin` binaries for depot.

Setup: Bootstrap Rust toolchain → clone from Forgejo → self-compile Tower Atomic → gate enrollment → depot push → NUCLEUS lifecycle validation (launchd vs systemd).

**darwinGate: GLACIAL → ACTIVE (G12).**

## Depot — Unified + Pruned (G69)

| Architecture | Binaries | Status |
|-------------|----------|--------|
| **x86_64-musl** | 19 | Current |
| **x86_64-windows-gnu** | 16 | Current |
| **x86_64-gnu** | 16 | Current |
| **aarch64-musl** | 13 | Partially stale |
| **aarch64-apple-darwin** | — | Pending (darwinGate) |

G69 lineage spec: binary evolution tracked via provenance trio (CAS/spine/braid).

## Primal Health Dashboard

| Primal | Tests | Health | Recent |
|--------|-------|--------|--------|
| songBird | 14,840+ | GREEN | `CanonicalTransport` shipped. 13,910 caps. MeshRelay pending. |
| bearDog | 14,019+ | GREEN | P0-A IN DEPOT. Spine commit signing unblocked. |
| toadStool | 9,193+ | GREEN | G72 exemplar: tokio 118→65 files. ~73 GiB reclaimed. |
| biomeOS | 8,570+ | GREEN | `capability.call` fleet-wide. Graph executor next. |
| petalTongue | 6,755+ | GREEN | doom-core decoupled. G19 WebGL next. 656 deps → G72 Tier 2 target. |
| barraCuda | 5,025 | GREEN | 19 gossip events LIVE. Silicon Fold ABSORBED. |
| squirrel | 4,613 | GREEN | C8 done (−67K lines). G18 LIVE. |
| coralReef | 3,963 | GREEN | `#[allow]→#[expect]` Rust 2024. GEMM Phase 2 IPC. |
| rhizoCrypt | 1,900 | GREEN | G72: -46 crates (wiremock). Gossip LIVE. |
| loamSpine | 1,752 | GREEN | G72: -7 crates (url+ICU). Gossip LIVE. |
| sweetGrass | 1,636 | GREEN | **P2 braid.verify CLOSED**. 5 behavioral tests. |
| cellMembrane | 1,353 | GREEN | G72: socket name dedup 3→1. NUCLEUS install extraction. |
| nestGate | 1,630+ | GREEN | G72: jsonrpsee removed (-1,864 LOC, -10 crates). |
| skunkBat | 675 | GREEN | RPC verified. Registry synced. |
| sourDough | 518 | GREEN | CI shipped (4 static validators). `rpc-surface` audit tool. |
| tideGlass | 214 | GREEN | G72: tokio trimmed. 17 IPC methods. |
| swarmVine | 124 | GREEN | 113 deps (already lean). Socket discovery FIXED. |

**Total**: ~150,000+ tests. **13/13 GREEN.** 16/16 G68 prod-clean.

## Three-Pillar Architecture

### Pillar 1: Neural API (The Brain)
`capability.call` routing OPERATIONAL (1.3ms / 4ms). `biome.yaml` manifest CONVERGED
(toadStool S377). primalSpring consuming: `biome-eastgate.yaml`, 14 primals, 3
compositions. exp122 37/37 PASS. Next: multi-composition graph workflows.

### Pillar 2: Data Federation (The Nervous System)
CAS federation LIVE. Gossip injection 6/16 primals. 86/87 braid pen test PASS.
`braid.verify` CLOSED (sweetGrass — 5 behavioral tests). Remaining jelly: `native_braid.py`
(1,259 LOC Python → Rust).

### Pillar 3: Pepti Layer (The Skeleton)
Deployment solved. golgiBody = peptidoglycan relay. Sub-builders compile. Gates pull.
Auto-prune. CAS archival operational (G69 Phase 1+2+3).

## Immediate Work — Post-Pandemic

| Priority | Goal | Owner | Effort |
|----------|------|-------|--------|
| **HIGH** | **songBird MeshRelay** | songBird | Days — blueGate + southGate blocked |
| **HIGH** | **Depot rebuild** with gossip + MeshRelay binaries | sporeGate | Hours |
| **HIGH** | **darwinGate bootstrap** | overwatch + primalSpring | Days |
| **HIGH** | **sourDough `convergence` + `rpc-surface` live CI** | cellMembrane + sourDough | Days |
| **MED** | **G72 Tier 2**: HTTP→songBird, axum→0.8, wgpu→28 | Fleet-wide | Sprint |
| **MED** | **Remaining gossip hooks** | hotSpring (10), wetSpring (2), barraCuda (3 edge) | Days |
| **LOW** | **Full bidirectional gossip peering** | All gates | Hours |

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
