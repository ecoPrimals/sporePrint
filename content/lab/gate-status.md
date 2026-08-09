+++
title = "Gate Status"
description = "Current fleet status — 11 gates online, 13/13 GREEN, 135K+ tests. Vertebrate evolution complete: 12/16 self-audited, zero phantom APIs. P0-B resolved. Depot rebuild in progress."
date = 2026-08-09
weight = 2

[extra]
maturity = "live"
+++

Current fleet status as of August 9, 2026 (Wave 157a — Vertebrate Evolution Complete).
12 teams self-audited. P0-B RESOLVED. P0-A code-fixed. sporeGate rebuilding depot.
Gates pull from golgi postPrimordial — no self-builds.

## P0 Issues — 1 Code-Open, 2 Depot-Stale

### P0-A: bearDog Sign Surface — CODE FIXED, DEPOT STALE
Code fixed (`766951004`): health socket guard, `-32601` for non-health methods,
socket rename `beardog-default` → `beardog-health`. Depot binary still stale —
awaiting sporeGate rebuild.

### P0-B: nestGate API Surface — RESOLVED
`content.ingest` was shipped since S136 (590 LOC, 7 tests). `content.stat` also
shipped (`4cafa535`). westGate's P0-B was a **stale depot binary**, not a missing
feature. Self-audit: zero phantom methods. Registry synced.

### P0-C: biomeOS FD Leak — OPEN
Auto-discovery loop opens sockets and never closes them. 14→58,613 FDs after
4 `capability.call` invocations. `capability.resolve` works (7ms). Direct primal
UDS works (0.2ms). Only forwarding leaks. Code fix not yet shipped.

## Deployment Discipline — postPrimordial

**Pattern leading to divergence**: gates were self-building primals from source,
resulting in each gate running different versions. westGate's P0-B was a stale
binary problem. ironGate reports nestgate+toadstool exit after startup (primal
binary issues).

**Correct pattern**: sporeGate is the sole depot builder. sporeGate rebuilds all
primals → pushes to golgi → gates pull from golgi via `plasmid.fetch`. No gate
builds its own primals.

## Gate Fleet — 6/6 NUCLEUS Redeployed

| Gate | NUCLEUS | RSS | Status |
|------|---------|-----|--------|
| **sporeGate** | 13/13 ALIVE | — | **Rebuilding depot** — all 16 primals from HEAD |
| **blueGate** | 13/13 ALIVE | 264 MB | Windows 15/15. 3 P3/P4 issues. |
| **southGate** | 13/13 ALIVE | 96 MB | 0.058ms Tower (2.6×). SSH compliant. |
| **ironGate** | 13/13 ALIVE | 41 MB | G68 redeploy 11/13. nestgate+toadstool binary issues. |
| **strandGate** | 11/13 ALIVE | 127 MB | First NUCLEUS boot. K-derm enforced. |
| **westGate** | 13/13 ALIVE | — | 989K files braided. 3.3 TB CAS. 14/14 services. |

## Vertebrate Evolution — 12/16 Self-Audited

12 primal teams responded with self-audits. Zero phantom methods across all
audited primals. Each verified RPC surface against `capability_registry.toml`.

| Primal | Self-Audit | Key Evolution |
|--------|-----------|---------------|
| **bearDog** | DONE (P0-A) | Health guard: no more silent swallowing. Socket naming fixed. |
| **nestGate** | DONE (P0-B) | `content.ingest` confirmed. `content.stat` shipped. Registry synced. |
| **songBird** | DONE | `CanonicalTransport` trait shipped (`33e9a8be`). 9 transports converging. |
| **swarmVine** | DONE | Deep audit, async dispatch, zero-copy. **39→124 tests** (82% coverage). |
| **petalTongue** | DONE | doom-core decoupled (ludoSpring-ready). Dep prune. RPC self-audit. |
| **skunkBat** | DONE | RPC surface verified, registry synced. |
| **rhizoCrypt** | DONE | 40/40 registry-handler parity. Fixed undeclared `dag.session.tree_hash`. |
| **loamSpine** | DONE | 54/54 RPC verified. `persist_tip` abstraction. −89 LOC. |
| **coralReef** | DONE | 18/18 RPC methods verified against registry. |
| **barraCuda** | DONE | Zero phantom APIs. 4,996 tests. |
| **cellMembrane** | DONE | `LimitNOFILE=65536` in systemd units. `capability_registry` 75→103. |
| **sourDough** | DONE | `rpc-surface` audit tool shipped (`aa1a2f8`): detects stubs + divergence live. |

**Remaining**: biomeOS (P0-C code fix), toadStool (S371 in progress), sweetGrass, bingoCube.

## Depot Rebuild — sporeGate In Progress

sporeGate rebuilding all primals from current HEAD. Key binaries that changed:

| Primal | Key Commit | What Changed |
|--------|-----------|-------------|
| **bearDog** | `766951004` | P0-A fix: health guard, -32601 for non-health, socket rename |
| **nestGate** | `4cafa535`+ | P0-B: `content.stat` shipped. `content.ingest` in code since S136. |
| **songBird** | `33e9a8be` | `CanonicalTransport` trait + swarmVine delegation |
| **swarmVine** | `2cd4964` | Deep audit, 124 tests, async dispatch |
| **petalTongue** | `87a2530` | doom-core decoupled, dep prune, RPC self-audit |
| **skunkBat** | `1ad84c1` | RPC surface verified, registry synced |
| **sourDough** | `aa1a2f8` | `rpc-surface` audit tool |

After rebuild: regenerate BLAKE3SUMS, push to golgi. Gates pull — no self-builds.

## Mesh Status — Code-Complete, Production-Blocked

`capability.resolve` works (7ms). Direct primal UDS works (0.2ms).
Vine-bat OPERATIONAL (gossip.spread → metadata.analyze → accept/reject).
P0-C (FD leak) makes `capability.call` forwarding unusable.

## G68 Convergence — 16/16 Prod-Clean

Every primal and cellMembrane has zero production G68 violations (sourDough
scanner v2). 205→0 production violations.

## SSH Key Discipline — K-Derm Enforced

All gates route through the K-Derm relay chain. Zero `github` remotes
ecosystem-wide:

```
gate → Forgejo (inner) → pepti (peptidoglycan) → golgi-ext (outer) → GitHub
```

## Trust Surfaces — LIVE

| Route | Status |
|-------|--------|
| `/api/content/stats` | **LIVE** — rhizoCrypt CAS via UDS |
| `/pseudospore/` | **LIVE** — 5 bundles + QCD v1.0.0-rung1 PACKAGED |
| `/api/pseudospore/bundles` | **LIVE** — bundle listing with provenance |
| `/pseudospore/validate.sh` | **LIVE** — verification script |

## Phase Execution Status

### Phase 1: Cell Boot — SUCCEEDED
First-ever cell attachment on ironGate. esotericWebb exp006 21/22 PASS.

### Phase 2: footPrint — DEPLOYED + LIVE
708 tests. `footprint.primals.eco` → ironGate :3002 via golgi Caddy.

### Phase 3: squirrel + petalTongue — G18 LIVE
9 primal providers on ironGate. petalTongue G19 render — NEXT.

### Phase 4: westGate Science Springs — DEPOT-BLOCKED
989K files braided. 153 datasets. 3.3 TB. Spine commits deferred until
bearDog depot binary ships with signing (P0-A code-fixed, depot-stale).

### Phase 5: Inter-gate Mesh — PRODUCTION-BLOCKED
Mesh code-complete. `capability.call` forwarding unusable (P0-C FD leak).

## Primal Health Dashboard

| Primal | Tests | Health | Recent |
|--------|-------|--------|--------|
| songBird | 14,840+ | GREEN | `CanonicalTransport` shipped. 9 transports converging. |
| bearDog | 14,019+ | GREEN | **P0-A CODE FIXED** (`766951004`). Depot rebuild needed. |
| nestGate | 1,630+ | GREEN | **P0-B RESOLVED**. `content.ingest` confirmed. `content.stat` shipped. |
| toadStool | 9,193+ | GREEN | S371: WASM split 24/48. Self-audit pending. |
| biomeOS | 8,570+ | GREEN | **P0-C OPEN**: FD leak in discovery loop. |
| petalTongue | 6,755+ | GREEN | doom-core decoupled. Trust surfaces LIVE. |
| barraCuda | 4,996 | GREEN | Zero phantom APIs. Self-audit DONE. |
| squirrel | 4,613 | GREEN | G68 prod-clean. |
| coralReef | 3,580 | GREEN | 18/18 RPC verified. Self-audit DONE. |
| rhizoCrypt | 1,900 | GREEN | 40/40 registry-handler parity. Self-audit DONE. |
| loamSpine | 1,752 | GREEN | 54/54 RPC verified. `persist_tip` abstraction. |
| sweetGrass | 1,636 | GREEN | Self-audit pending. |
| cellMembrane | 1,327+ | GREEN | `capability_registry` 75→103. `LimitNOFILE` wired. |
| skunkBat | 675 | GREEN | RPC surface verified. Registry synced. |
| sourDough | 518 | GREEN | `rpc-surface` audit tool shipped. |
| swarmVine | 124 | GREEN | **39→124 tests** (82% coverage). Async dispatch. |
| tideGlass | 214 | GREEN | 17 IPC methods. GPS converted. |

**Total**: ~135,000+ tests. **13/13 GREEN.** 16/16 G68 prod-clean.

## Live Sites

| Site | URL | Status |
|------|-----|--------|
| **sporePrint** | `sporeprint.primals.eco` | **LIVE** — 338 pages |
| **footPrint** | `footprint.primals.eco` | **LIVE** — CAS works |
| **nestgate.io** | `nestgate.io` | **LIVE** — trust surfaces + data braids unblocked |
| **esotericWebb** | `webb.primals.eco` | 502 — needs petalTongue WebGL |

## K-Derm Three-Domain Topology — Fully Operational

| Domain | Layer | DNS | Status |
|--------|-------|-----|--------|
| **primals.eco** | Outer | Cloudflare (wildcard) | **LIVE** — 14 Caddy routes |
| **nestgate.io** | Peptidoglycan | Sovereign Knot DNS + DNSSEC | **LIVE** — trust surfaces + data braids unblocked |
| **primal.eco** | Inner | Sovereign Knot DNS (zero public) | **LIVE** — all 11 gates |

## Pending: Live Dashboard

This page currently shows static data. When petalTongue G19 rendering
matures, it will serve real-time health data from `biomeOS neuralAPI`.
