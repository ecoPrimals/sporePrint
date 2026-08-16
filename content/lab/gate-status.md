+++
title = "Gate Status"
description = "Current fleet status — 12 gates ONLINE, 0/0/0, bonsai-bt FORKED, rootPulse 6/6 REGISTERED, Titan V Tier 1 CONFIRMED, graftGate FULL NUCLEUS. Pipeline + provenance CONVERGED."
date = 2026-08-16
weight = 2

[extra]
maturity = "live"
+++

Current fleet status as of August 16, 2026 (Wave 157k — Enmeshment + Ingestion).
12 gates ONLINE. Zero P0, P1, P2. bonsai-bt FORKED — first external ingestion.
rootPulse 6/6 graphs REGISTERED. Titan V Tier 1 CONFIRMED. Pipeline + provenance CONVERGED.

## Gate Fleet — 12 Gates ONLINE

| Gate | Composition | Status |
|------|-------------|--------|
| **eastGate** | Full NUCLEUS + overwatch | rootPulse 6/6 REGISTERED. exp125 bonsai-bt LIVE. biomeOS 1,608 tests. |
| **ironGate** | Full NUCLEUS + 14TB CAS | 13/13, 2ms dispatch, 4 mesh peers |
| **strandGate** | Full NUCLEUS + dual EPYC | DF64 shaders SHIPPED. arXiv ACTIVE. |
| **westGate** | Full NUCLEUS + 50.7TB ZFS | AlphaFold ingress ACTIVE. rootPulse handlers SHIPPED. |
| **sporeGate** | Foreman + depot | 13/13 x86_64 CURRENT. Cascade autonomous. |
| **blueGate** | ENMESHED (Windows) | builder.serve ALIVE :9800. Depot 0/13 STALE. |
| **graftGate** | FULL NUCLEUS (Darwin) | builder.serve LIVE :9800. Depot 16/16 CURRENT. |
| **southGate** | NUCLEUS + canary | neuralSpring 71/80. SSH ready. |
| **biomeGate** | Tower 4/4 + Node Atomic | ONLINE. Titan V Tier 1 CONFIRMED. K80 blocked (GK210). |
| **grapheneGate** | Tower Atomic | ADB deploy. |
| **iosGate** | BearDogApp | 6th OS family. |
| **steamGate** | Tower Atomic | Portable compute. |

## bonsai-bt — First External Ingestion

**Source**: github.com/Sollimann/bonsai (MIT, v0.13.0, 207 commits, ~790 stars)
**Fork**: git.primals.eco/ecoPrimals/bonsai-bt (full mirror)

DECIDE layer meta-primal: behavior trees as execution policy between squirrel
REASON and biomeOS ROUTE. Trees are serializable, content-addressable artifacts.

**exp125 LIVE** (primalSpring): 23/24 checks pass (1 expected — no live NUCLEUS sockets
in overwatch session). 5 behavior trees validated against NUCLEUS:

| Tree | Pattern | Result |
|------|---------|--------|
| Reactive health check | Sequence over capability domains | PASS |
| Compute fallback | Select — first-success-wins | PASS |
| Provenance pipeline | hash→store→DAG→sign chain | PASS |
| Serialization round-trip | 550B JSON, BLAKE3 hashable | PASS |
| Memoryless reactive policy | Re-evaluate conditions each tick | PASS |

Architecture: `squirrel → REASON | [bonsai-bt] → DECIDE | biomeOS → ROUTE | primals → ACT | sweetGrass → WITNESS | PathwayLearner → ADAPT`

Code audit: **0 unsafe**, 3,197 LOC core, 76 tests pass, 0 TODO/FIXME, 0 default deps.

## rootPulse — 6/6 Graphs REGISTERED

biomeOS `af1dc9d3`: all 6 rootPulse graphs registered and exposed via `graph.list`.

| Graph | Purpose |
|-------|---------|
| commit | Content commitment lifecycle |
| harvest | Binary artifact collection |
| branch | Version divergence tracking |
| merge | Version convergence resolution |
| diff | Content delta computation |
| federate | Cross-gate content distribution |

**Item #10 CLOSED.** biomeOS 1,608 tests pass.

## biomeGate — Titan V Tier 1 CONFIRMED

4 measurement bugs fixed:
- D3hot reads as cold
- Tier 2 without FECS
- Sleeping GPU as warm
- Catalyst PC range

`RegisterRead` enum replaces raw `u32` at 10 sites. 23 engines visible,
PRAMIN accessible, reproducible. FECS PRI fault blocks Tier 2.
K80 blocked by missing GK210 chipset entry — software gap, path forward:
map `0xf2` onto `gk110b`.

`toadstool sovereign handoff|status|strategies` CLI shipped.

## Depot Status

| Architecture | Binaries | Status |
|-------------|----------|--------|
| **x86_64-unknown-linux-musl** | 13/13 | Current (rebuilt Aug 14) |
| **aarch64-unknown-linux-musl** | 15/15 | Current (ironGate sub-builder) |
| **aarch64-apple-darwin** | 16/16 | Current (graftGate) |
| **x86_64-pc-windows-gnu** | 0/13 | STALE (awaiting autonomous dispatch) |

## G72 Dependency Pandemic — Tier 1 COMPLETE (from 157i)

9/9 teams responded. ~114 crates shed fleet-wide. toadStool tokio 118→65 files.
Tier 2 queued: HTTP→songBird, axum 0.8, wgpu 28, YAML unification.

## Gossip Injection — 6/16 Primals LIVE

| Entity | Events | Status |
|--------|--------|--------|
| **rhizoCrypt** | 3 DAG lifecycle | LIVE |
| **loamSpine** | 4 spine events | LIVE |
| **lithoSpore** | 4 validation events | LIVE |
| **barraCuda** | 19 runtime events | LIVE |
| **esotericWebb** | 2 session lifecycle | LIVE |
| **songBird** | 1 capability advertise | LIVE |
| **wetSpring** | 2/4 | PARTIAL |
| **hotSpring** | 0/10 | SCAFFOLD |

## Three-Pillar Architecture

### Pillar 1: Neural API (The Brain)
`capability.call` routing OPERATIONAL. `biome.yaml` manifest CONVERGED.
rootPulse 6/6 graphs REGISTERED. bonsai-bt DECIDE layer ingesting.

### Pillar 2: Data Federation (The Nervous System)
CAS federation LIVE. Gossip injection 6/16 primals. `braid.verify` CLOSED.
westGate 50.7TB ZFS. AlphaFold ingress ACTIVE. 227 files fossilized (1,513 total).

### Pillar 3: Pepti Layer (The Skeleton)
Deployment solved. golgiBody = peptidoglycan relay. Sub-builders compile (3/3 enmeshed).
graftGate depot 16/16 CURRENT. Cascade autonomous.

## Remaining Infrastructure

| # | Item | Owner | Priority |
|---|------|-------|----------|
| 2 | cellMembrane UDS→TCP fallback (Windows health probes) | sporeGate | P2 |
| 4 | blueGate depot rebuild via autonomous dispatch | sporeGate | P2 |
| 5 | `rust-toolchain.toml` GNU target for Windows | ironGate | P2 |
| 11 | bearDog AEAD Neural API surfacing | ironGate | P2 |
| 12 | sweetGrass auto-announce in depot binary | sporeGate | P2 |
| 15 | AlphaFold ingress Phase B+C | westGate | ACTIVE |
| 16 | tideGlass Phase 0 | westGate | QUEUED |

## NanoWire SSH Retirement

| Tier | Scope | Status |
|------|-------|--------|
| 1 | Sub-builder CI dispatch | **RETIRED** (3/3 enmeshed) |
| 2 | gate.pull/check/info, plasmid.trigger, service.* | NEXT |
| 3-7 | Depot push, CAS, Caddy, enrollment, relay, git | Future |

## Active Code Teams

| Team | Track | Status |
|------|-------|--------|
| **eastGate — primalSpring** | exp125 bonsai-bt integration | ACTIVE |
| **westGate — cellMembrane** | AlphaFold ingress pipeline | ACTIVE |
| **strandGate — barraCuda + coralReef** | DF64 sovereign shaders | SHIPPED |
| **sporeGate — cellMembrane** | Cascade ops | SHIPPED. Autonomous. |
| **westGate — rhizoCrypt** | rootPulse handlers | SHIPPED. DORMANT. |
| **westGate — sweetGrass** | rootPulse handlers | SHIPPED. DORMANT. |

## Downstream Patterns

| Track | Status |
|-------|--------|
| bonsai-bt meta-primal (DECIDE layer) | **PHASE 0 — INGESTING** |
| tideGlass Phase 0 (gen5 sole bottleneck) | QUEUED — external review: 5-7 days |
| arXiv submission | ACTIVE (strandGate) |
| AlphaFold Neural API ingress | ACTIVE (westGate) |
| Sovereign dispatch (biomeGate) | ACTIVE (intermittent) |
| SSH → Tower Atomic graduation (NanoWire Tiers 2-7) | NEXT |
| Graph visualization | SPEC FILED |

## Live Sites

| Site | URL | Status |
|------|-----|--------|
| **sporePrint** | `sporeprint.primals.eco` | **Triage needed** — Zola build/deploy regression |
| **footPrint** | `footprint.primals.eco` | **LIVE** — CAS works |
| **nestgate.io** | `nestgate.io` | **LIVE** — trust surfaces + data braids |
| **esotericWebb** | `webb.primals.eco` | 502 — needs petalTongue WebGL (G19) |

## Pending: Live Dashboard

This page currently shows static data. When petalTongue G19 rendering
matures, it will serve real-time health data from `biomeOS neuralAPI`.
