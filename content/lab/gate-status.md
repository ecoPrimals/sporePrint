+++
title = "Gate Status"
description = "Current fleet status — 11 gates online, 13/13 GREEN, 135K+ tests. 6/6 NUCLEUS gates redeployed. NG-05 CLOSED. QCD pseudoSpore PACKAGED."
date = 2026-08-08
weight = 2

[extra]
maturity = "live"
+++

Current fleet status as of August 8, 2026 (Wave 157a — all gates redeployed).
G68 converged, depot current on golgi, cascade auto-push operational.
6/6 NUCLEUS gates running G68-converged binaries. NG-05 CLOSED.

## Gate Fleet — 6/6 NUCLEUS Redeployed

| Gate | NUCLEUS | RSS | Status |
|------|---------|-----|--------|
| **sporeGate** | 13/13 ALIVE | — | S369, cascade auto-push, zero drift |
| **blueGate** | 13/13 ALIVE | 264 MB | Windows 15/15. 3 P3/P4 issues. |
| **southGate** | 13/13 ALIVE | 96 MB | 0.058ms Tower (2.6×). SSH compliant. |
| **ironGate** | 13/13 ALIVE | 41 MB | 2,058 capabilities. 42 repos SSH clean. |
| **strandGate** | 11/13 ALIVE | 127 MB | First NUCLEUS boot. K-derm enforced. |
| **westGate** | 13/13 ALIVE | — | **NG-05 DONE**: 26 capabilities registered. 2.5 TB CAS. |

All gates running biomeOS 4.57.0 (Stage 2), G68-converged depot binaries.
SSH discipline enforced across all gates — zero `github` remotes ecosystem-wide.

## NG-05 — westGate CAS Federation CLOSED

nestGate TCP on `0.0.0.0:8080` serving CAS to mesh. Full provenance chain
registered with songBird:

| Primal | Capabilities |
|--------|-------------|
| nestGate | 6 (content.get, content.put, etc.) |
| loamSpine | 8 (ledger, certificate, etc.) |
| rhizoCrypt | 5 (dag, verification, etc.) |
| sweetGrass | 4 (braid, attribution, etc.) |
| bearDog | 3 (crypto, auth, etc.) |
| **Total** | **26 capabilities** |

CAS pool: **2.5 TB** (1.1 TB warm NVMe + 1.4 TB cold ZFS).
`songbird-register.service` for persistent registration at boot.
`capability.resolve("content.get")` → nestGate working.

**Unblocks**: nestgate.io data braids, cross-gate `content.replicate.pull`,
Neural API capability routing.

## G68 Convergence — 16/16 Prod-Clean

Every primal and cellMembrane has zero production G68 violations (sourDough
scanner v2). 205→0 production violations.

## Depot + Cascade

| Target | Binaries | Status |
|--------|----------|--------|
| **Musl** | 17/17 | At Forgejo HEAD (inc. toadStool S370) |
| **Windows** | 15/15 | squirrel.exe added this wave |

Cascade auto-push to golgi via `ExecStartPost` rsync. Pipeline:
`Forgejo → fetch → drift detect → harvest → stage → golgi push`.
synced=15, zero drift.

### cellMembrane — Sovereign Deploy Path
`plasmid.fetch --source forgejo` API parse + auth **FIXED** (`55fdff3`).
All remote gates now have a sovereign deploy path — no GitHub dependency.

### toadStool S370 — WASM Compute
15 crates compile on `wasm32-unknown-unknown`. New architecture axis:
desktop (native) + server (musl) + browser (wasm32). 16 deployment targets total.

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

### Phase 4: westGate Science Springs — UNBLOCKED
NG-05 CLOSED. 2.5 TB CAS. 26 capabilities. Cell boot ready.

### Phase 5: Inter-gate Mesh — UNBLOCKED
NG-05 enables cross-gate content access. songBird federation to westGate
configured. `content.replicate.pull` ready.

## Primal Health Dashboard

| Primal | Tests | Health | Recent |
|--------|-------|--------|--------|
| songBird | 14,840+ | GREEN | 22 bonds. NG-05: 26 capabilities registered. |
| bearDog | 14,019 | GREEN | — |
| nestGate | 13,095+ | GREEN | **NG-05: TCP on :8080 serving CAS.** |
| toadStool | 9,193+ | GREEN | **S370: WASM compute (15 crates on wasm32).** |
| biomeOS | 8,570+ | GREEN | v4.57.0 Stage 2. All 6 gates deployed. |
| petalTongue | 6,755 | GREEN | Trust surfaces LIVE. |
| barraCuda | 4,959 | GREEN | MultiDevicePool. Cross-vendor. |
| squirrel | 4,613 | GREEN | G68 prod-clean. |
| coralReef | 3,512 | GREEN | G68 prod-clean. |
| rhizoCrypt | 1,791 | GREEN | G63 SO\_PEERCRED. NG-05: 5 caps registered. |
| loamSpine | 1,740 | GREEN | NG-05: 8 caps registered. |
| sweetGrass | 1,636 | GREEN | NG-05: 4 caps registered. `capability.call` SHIPPED. |
| tideGlass | 214 | GREEN | 17 IPC methods. GPS converted. |
| cellMembrane | 1,327 | GREEN | **`plasmid.fetch --source forgejo` FIXED.** |

**Total**: ~135,000+ tests. **13/13 GREEN.** 16/16 G68 prod-clean.

## Live Sites

| Site | URL | Status |
|------|-----|--------|
| **sporePrint** | `sporeprint.primals.eco` | **LIVE** — 338 pages |
| **footPrint** | `footprint.primals.eco` | **LIVE** — CAS works |
| **nestgate.io** | `nestgate.io` | **LIVE** — trust surfaces + NG-05 unblocks data braids |
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
