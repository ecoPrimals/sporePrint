+++
title = "Gate Status"
description = "Current fleet status — 11 gates online, 13/13 GREEN, 135K+ tests. G68 COMPLETE — 3/6 gates redeployed. Trust surfaces LIVE on nestgate.io."
date = 2026-08-08
weight = 2

[extra]
maturity = "live"
+++

Current fleet status as of August 8, 2026 (Wave 157a — gate redeploy in
progress). G68 converged, depot current on golgi, cascade auto-push operational.
3/6 NUCLEUS gates redeployed to G68-converged binaries.

## Gate Redeploy — 3/6 Complete

| Gate | Status | Details |
|------|--------|---------|
| **sporeGate** | **DONE** — 13/13 ALIVE | S369 deployed, cascade auto-push, zero drift |
| **blueGate** | **DONE** — 13/13 ALIVE | Windows 15/15 pulled. 264 MB RSS. 3 P3/P4 issues. |
| **southGate** | **DONE** — 13/13 ALIVE | 96 MB RSS, 0.058ms Tower (2.6× faster). SSH compliant. |
| **strandGate** | **DIVERGED** | v2026.05.30 binaries (2+ months stale). Needs SSH depot access. |
| **westGate** | **PENDING** | Awaiting redeploy |
| **ironGate** | **PENDING** | Awaiting redeploy |

### strandGate Divergence

strandGate cannot fetch G68 binaries: GitHub releases stale, Forgejo API parse
fails, no SSH access to golgi depot directory. **Science is unblocked** — SU(3)
campaign COMPLETE (36 configs), SU(4) running, NPU hardware live. Only primal
binary deployment is blocked. Resolution: SSH key registration on golgi for
rsync depot pull.

## G68 Convergence — 16/16 Prod-Clean

Every primal and cellMembrane has zero production G68 violations (sourDough
scanner v2). 205→0 production violations.

| Level | Primals |
|-------|---------|
| **G68** (zero violations) | sourDough, nestGate, petalTongue, bingoCube, loamSpine, barraCuda, cellMembrane, +1 |
| **G68-prod** (test-only) | squirrel, bearDog, songBird, rhizoCrypt, skunkBat, sweetGrass, coralReef, biomeOS, toadStool |

## Depot + Cascade

| Target | Binaries | Status |
|--------|----------|--------|
| **Musl** | 17/17 | At Forgejo HEAD (inc. toadStool S369) |
| **Windows** | 15/15 | squirrel.exe added this wave |

Cascade auto-push to golgi via `ExecStartPost` rsync. Pipeline:
`Forgejo → fetch → drift detect → harvest → stage → golgi push`.
synced=15, zero drift. Only manual step: per-gate NUCLEUS deploy.

## SSH Key Discipline — K-Derm Enforced

GitHub direct access cut. SSH discipline enforced on eastGate, blueGate,
southGate. All routes through the K-Derm relay chain:

```
gate → Forgejo (inner) → pepti (peptidoglycan) → golgi-ext (outer) → GitHub
```

| Entity | Forgejo SSH | GitHub SSH | Role |
|--------|-------------|------------|------|
| **golgi** | YES (host) | NO | Sole sovereign Git store |
| **golgi-ext** | NO | **YES (sole writer)** | K-Derm relay |
| **eastGate** | YES (key `eastGate`) | **REVOKED** | Overwatch |
| **blueGate** | YES | **REVOKED** | Windows dev |
| **southGate** | YES | **REVOKED** (33 repos cleaned) | Validation |

## Trust Surfaces — LIVE on nestgate.io

| Route | Status | What |
|-------|--------|------|
| `/api/content/stats` | **LIVE** | rhizoCrypt CAS via UDS — object counts, sizes, namespaces |
| `/pseudospore/` | **LIVE** | 5 pseudoSpore bundles as downloadable files |
| `/api/pseudospore/bundles` | **LIVE** | Bundle listing with provenance metadata |
| `/pseudospore/validate.sh` | **LIVE** | Downloadable verification script |

petalTongue commits: `037535e` (content stats) + `01961ce` (pseudospore routes).
QCD bundle not yet packaged — routes serve but Rung 1 bundle awaits lithoSpore.

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
| petalTongue | 6,755 | GREEN | **Trust surfaces: `/api/content/stats` + `/pseudospore/` LIVE.** |
| barraCuda | 4,959 | GREEN | MultiDevicePool. Cross-vendor. |
| squirrel | 4,613 | GREEN | G68 prod-clean. |
| coralReef | 3,512 | GREEN | G68 prod-clean. |
| rhizoCrypt | 1,791 | GREEN | G63 SO\_PEERCRED. CAS backing trust surfaces. |
| loamSpine | 1,740 | GREEN | G68 zero violations. |
| sweetGrass | 1,636 | GREEN | `capability.call` handler SHIPPED. |
| tideGlass | 214 | GREEN | 17 IPC methods. GPS converted. |
| cellMembrane | 1,327 | GREEN | Platform abstraction (15 cfg→3). G68 zero. |

**Total**: ~135,000+ tests. **13/13 GREEN.** 16/16 G68 prod-clean.

## Live Sites

| Site | URL | Status |
|------|-----|--------|
| **sporePrint** | `sporeprint.primals.eco` | **LIVE** — 338 pages, science content |
| **footPrint** | `footprint.primals.eco` | **LIVE** — CAS works, map + agent bridge |
| **nestgate.io** | `nestgate.io` | **LIVE** — 10/12 sections + trust surface routes |
| **esotericWebb** | `webb.primals.eco` | 502 — needs petalTongue WebGL pipeline |

## K-Derm Three-Domain Topology — Fully Operational

| Domain | Layer | DNS | Status |
|--------|-------|-----|--------|
| **primals.eco** | Outer | Cloudflare (wildcard) | **LIVE** — 14 Caddy routes |
| **nestgate.io** | Peptidoglycan | Sovereign Knot DNS + DNSSEC | **LIVE** — trust surfaces + dashboard |
| **primal.eco** | Inner | Sovereign Knot DNS (zero public) | **LIVE** — dnsmasq, all 11 gates |

## blueGate Windows Issues (P3/P4)

| ID | Issue | Workaround |
|----|-------|------------|
| P3 | skunkBat ignores `PRIMAL_BIND_MODE=tcp` env | Pass `--bind-mode tcp` on CLI |
| P4 | petalTongue `--port` ignored in server mode | Accept dynamic ports |
| P3 | songBird stale PID file blocks startup | Clean PID dir before start |

## Pending: Live Dashboard

This page currently shows static data. When petalTongue G19 rendering
matures, it will serve real-time health data from `biomeOS neuralAPI`.
