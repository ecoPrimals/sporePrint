+++
title = "Gate Status"
description = "Current fleet status — 11 gates online, 13/13 GREEN, 135K+ tests. 3 P0s OPEN: bearDog sign stub, nestGate API mismatch, biomeOS FD leak. Mesh code-complete, production-blocked."
date = 2026-08-09
weight = 2

[extra]
maturity = "live"
+++

Current fleet status as of August 9, 2026 (Wave 157a — Vertebrate Evolution).
westGate 7-session retrospective exposed 3 P0 issues. Mesh code-complete,
production-blocked. Primals self-audit RPC surfaces.

## P0 Issues — 3 OPEN

### P0-A: bearDog Sign Surface Missing
Depot binary v0.9.0 returns health response for ALL methods including
`crypto.sign_ed25519`. All spine commits unsigned. loamSpine `session.commit` fails.
**Fix**: bearDog team rebuilds depot binary with actual Ed25519 signing + socket
naming fix (`beardog-default.sock` → `beardog-{family_id}.sock`).

### P0-B: nestGate API Surface Mismatch
`content.ingest` (directory walk + CAS) does not exist in nestGate v0.5.0.
`content.stat` also missing. Pipeline must do directory walks in Python (3× I/O,
33% payload inflation from base64).
**Fix**: nestGate team ships native `content.ingest(directory)` + `content.stat(hash)`.

### P0-C: biomeOS FD Leak
Auto-discovery loop opens sockets and never closes them. 14→58,613 FDs after
4 `capability.call` invocations. `capability.resolve` works (7ms). Direct primal
UDS works (0.2ms). Only forwarding leaks.
**Fix**: biomeOS team adds socket cleanup in discovery loop.

## Gate Fleet — 6/6 NUCLEUS Redeployed

| Gate | NUCLEUS | RSS | Status |
|------|---------|-----|--------|
| **sporeGate** | 13/13 ALIVE | — | S369, cascade auto-push, zero drift |
| **blueGate** | 13/13 ALIVE | 264 MB | Windows 15/15. 3 P3/P4 issues. |
| **southGate** | 13/13 ALIVE | 96 MB | 0.058ms Tower (2.6×). SSH compliant. |
| **ironGate** | 13/13 ALIVE | 41 MB | 2,058 capabilities. 42 repos SSH clean. |
| **strandGate** | 11/13 ALIVE | 127 MB | First NUCLEUS boot. K-derm enforced. |
| **westGate** | 13/13 ALIVE | — | 26 capabilities. 3.3 TB CAS. 989K files braided. |

All gates running biomeOS 4.57.0 (Stage 2), G68-converged depot binaries.
SSH discipline enforced across all gates — zero `github` remotes ecosystem-wide.

## Vertebrate Evolution — Primal Self-Audit

G64 cephalization gave the ecosystem a nervous system (Neural API, biomeOS routing,
Tower mesh). westGate's 7-session retrospective (989K files braided, 153 datasets,
3.3 TB) revealed that primal API surfaces diverge silently from what consumers expect.
Six Python jelly strings exist because primal APIs don't do what they claim.

Each primal team self-audits: verify actual RPC surface matches
`capability_registry.toml`, abstract repeated patterns behind shared traits,
delegate cross-focus to its right home.

| Primal | Binary | Evolution Task |
|--------|--------|----------------|
| **bearDog** | 8.3 MB | **P0-A**: Rebuild with actual crypto. Fix socket naming. |
| **nestGate** | 8.5 MB | **P0-B**: Ship `content.ingest` + `content.stat`. |
| **biomeOS** | 20.4 MB | **P0-C**: Fix FD leak in discovery loop. |
| **songBird** | 23.8 MB | Abstract 9 transport crates → shared `Transport` trait. Excise `mesh.capabilities_announce` → swarmVine. |
| **petalTongue** | 33.8 MB | Move `doom-core` → ludoSpring. Converge 656 deps. |
| **toadStool** | 12.4 MB | S371 `core` 272K → natural WASM split. 24/48 done. |

## Mesh Status — Code-Complete, Production-Blocked

`capability.resolve` works (7ms). Direct primal UDS works (0.2ms).
Vine-bat OPERATIONAL (gossip.spread → metadata.analyze → accept/reject).
But P0-C (FD leak) makes `capability.call` unusable for production workloads.
westGate bypasses biomeOS entirely.

## Depot + Cascade

| Target | Binaries | Status |
|--------|----------|--------|
| **Musl** | 17/17 | At Forgejo HEAD (inc. toadStool S371) |
| **Windows** | 15/15 | squirrel.exe added this wave |

songBird 24 MB FIXED (`af0d8fa8`). bearDog STALE (health-only stub — P0-A).
Cascade auto-push to golgi via `ExecStartPost` rsync. synced=15, zero drift.

### cellMembrane — Sovereign Deploy Path
`plasmid.fetch --source forgejo` API parse + auth **FIXED** (`55fdff3`).
All remote gates now have a sovereign deploy path — no GitHub dependency.

### toadStool S371 — WASM Compute
`core` 272K natural WASM split. 24/48 done. 15 crates compile on
`wasm32-unknown-unknown`. Desktop (native) + server (musl) + browser (wasm32).

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

QCD pseudoSpore bundle PACKAGED by lithoSpore. `validate.sh` needs
bundle-specific BLAKE3 + DAG + Ed25519 wiring. Freeze/sign pending.

## Phase Execution Status

### Phase 1: Cell Boot — SUCCEEDED
First-ever cell attachment on ironGate. esotericWebb exp006 21/22 PASS.

### Phase 2: footPrint — DEPLOYED + LIVE
708 tests. `footprint.primals.eco` → ironGate :3002 via golgi Caddy.

### Phase 3: squirrel + petalTongue — G18 LIVE
9 primal providers on ironGate. petalTongue G19 render — NEXT.

### Phase 4: westGate Science Springs — PRODUCTION-BLOCKED
989K files braided. 153 datasets. 3.3 TB. But spine commits unsigned (P0-A)
and `content.ingest` doesn't exist (P0-B).

### Phase 5: Inter-gate Mesh — PRODUCTION-BLOCKED
Mesh code-complete. `capability.call` unusable due to FD leak (P0-C).
westGate bypasses biomeOS entirely.

## Primal Health Dashboard

| Primal | Tests | Health | Recent |
|--------|-------|--------|--------|
| songBird | 14,840+ | GREEN | 22 bonds. 24 MB FIXED. 9 transports → shared trait. |
| bearDog | 14,019 | GREEN | **P0-A**: depot stub returns health for all methods. |
| nestGate | 13,095+ | GREEN | **P0-B**: `content.ingest` doesn't exist. API surface audit. |
| toadStool | 9,193+ | GREEN | **S371**: WASM split 24/48. |
| biomeOS | 8,570+ | GREEN | **P0-C**: FD leak in discovery loop. 14→58K FDs. |
| petalTongue | 6,755 | GREEN | doom-core → ludoSpring. Trust surfaces LIVE. |
| barraCuda | 4,959 | GREEN | MultiDevicePool. Cross-vendor. |
| squirrel | 4,613 | GREEN | G68 prod-clean. |
| coralReef | 3,512 | GREEN | G68 prod-clean. |
| rhizoCrypt | 1,791 | GREEN | G63 SO\_PEERCRED. 5 caps registered. |
| loamSpine | 1,740 | GREEN | 8 caps registered. Spine commits deferred (P0-A). |
| sweetGrass | 1,636 | GREEN | 4 caps registered. `capability.call` SHIPPED. |
| tideGlass | 214 | GREEN | 17 IPC methods. GPS converted. |
| cellMembrane | 1,327 | GREEN | `plasmid.fetch --source forgejo` FIXED. |

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
