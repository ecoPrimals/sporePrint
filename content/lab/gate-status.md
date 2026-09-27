+++
title = "Gate Status"
description = "Current fleet status — Wave 158 REWAKE. 3 gates ONLINE, 3 OFFLINE (power rebalance), northGate ENROLLING, Jupiter 2 ARRIVED. detroit.primals.eco LIVE (128 pages). guerillaGorilla FORMALIZED. Windows depot 12/17. Pipeline + provenance CONVERGED."
date = 2026-09-27
weight = 2

[extra]
maturity = "live"
+++

Current fleet status as of September 27, 2026 (Wave 158 — Rewake + Legal Primal + Mesh Expansion).
**REWAKE** posture: eastGate + golgiBody + sporeGate **ONLINE**. northGate **ENROLLING** (Windows 11, RTX 5090).
House 2 gates **OFFLINE** — power rebalance next week. detroit.primals.eco **LIVE** (128+ pages, dynasty expansion).
{{ entity(name="guerillagorilla") }} **FORMALIZED** — amicusContra named. Milk-V Jupiter 2 **ARRIVED** (RISC-V RVA23, 7th arch family).

## Gate Fleet — Wave 158 Rewake

| Gate | Composition | Location | Status |
|------|-------------|----------|--------|
| **eastGate** | Full NUCLEUS + overwatch | House 2 | ✅ ONLINE. rustChip cleaned (367 tests). Wave 158 cascade. |
| **golgiBody** | Caddy + Forgejo + Zola + cascade | Cloud (DO) | ✅ ONLINE. Cascade autonomous. detroit + primals.eco serving. |
| **sporeGate** | Foreman + depot + cascade hub | House 1 | ✅ ONLINE. Windows depot 12/17. primalSpring IPC gated. |
| **northGate** | Tower Atomic (target) | House 1 | 🔄 ENROLLING. Windows 11, RTX 5090. Pushing to Forgejo. |
| **Jupiter 2** | NEW (RISC-V RVA23) | House 1 | 🆕 ARRIVED. 7th arch family. Bring-up pending. |
| **NUC bench** | Tower Atomic (target) | House 1 | 🆕 DDR3 NUCs — sub-builders, site hosts, mesh nodes. |
| **biomeGate** | Tower 4/4 + Node Atomic | House 1 | ⏸️ OFFLINE. Power on needed. Titan V Tier 1. |
| **graftGate** | FULL NUCLEUS (Darwin) | House 1 | ⏸️ OFFLINE. Power on needed. Depot 16/16. |
| **ironGate** | Full NUCLEUS + 14TB CAS | House 2 | ⏸️ OFFLINE. Power rebalance next week. |
| **strandGate** | Full NUCLEUS + dual EPYC | House 2 | ⏸️ OFFLINE. Power rebalance next week. 45 QCD configs banked. |
| **westGate** | Full NUCLEUS + 50.7TB ZFS | House 2 | ⏸️ OFFLINE. Power rebalance next week. |
| **blueGate** | ENMESHED (Windows) | House 2 | ⏸️ OFFLINE. Rack move incomplete. |
| **southGate** | NUCLEUS + canary | House 2 | ⏸️ OFFLINE. Power rebalance next week. |
| **grapheneGate** | Tower Atomic | Mobile | ADB deploy. |
| **iosGate** | BearDogApp | Mobile | 6th OS family. |
| **steamGate** | Tower Atomic | Mobile | Portable compute. |

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

## Depot Status — 5-Target Architecture

| Architecture | Binaries | Status |
|-------------|----------|--------|
| **x86_64-unknown-linux-musl** | **19/19** | ✅ CURRENT (Sep 15-25) |
| **x86_64-unknown-linux-gnu** | **14/14** | ✅ CURRENT |
| **aarch64-unknown-linux-musl** | **16/16** | ✅ CURRENT (ironGate sub-builder) |
| **aarch64-apple-darwin** | **16/16** | ✅ CURRENT (graftGate) |
| **x86_64-pc-windows-gnu** | **12/17** | 🔄 5 blocked on team unix fixes |

### Windows depot: 5 primals blocked

| Primal | Owner | Issue |
|--------|-------|-------|
| toadStool | strandGate | 34 files with `tokio::net::Unix*` |
| petalTongue | ironGate | 1 file: `UnixStream` in `cas_send_uds()` |
| sweetGrass | westGate | `#[cfg(unix)]` block on `AppState.crypto` |
| sourDough | graftGate | 11 errors (not audited) |
| membrane | sporeGate | UDS→TCP fallback needed |

Fix pattern (proven in primalSpring `24f71cb7`): `cargo check --target x86_64-pc-windows-gnu` → gate with `#[cfg(unix)]` + `#[cfg(not(unix))]` fallback.

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
| 17 | barraCuda: configurable warmup count in GpuHmcConfig | strandGate | **P1** |
| 18 | barraCuda: plaquette time-series export for autocorrelation | strandGate | **P1** |
| 2 | cellMembrane UDS→TCP fallback (Windows health probes) | sporeGate | P2 |
| 4 | blueGate depot rebuild via autonomous dispatch | sporeGate | P2 |
| 4b | Windows depot: 5 team unix fixes | strandGate, ironGate, westGate, graftGate, sporeGate | P2 |
| 5 | `rust-toolchain.toml` GNU target for Windows | ironGate | P2 |
| 11 | bearDog AEAD Neural API surfacing | ironGate | P2 |
| 12 | sweetGrass auto-announce in depot binary | sporeGate | P2 |
| 19 | Ecosystem: pin `rust-toolchain.toml` versions | all primals | P2 |
| 20 | Ecosystem: `cargo test --workspace --no-run` CI gate | sporeGate | P2 |
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
| **strandGate — hotSpring** | SU(3) production — 45 configs banked | Protocol correction NEXT |
| **strandGate — barraCuda + coralReef** | DF64 sovereign shaders | SHIPPED. P1: warmup + time-series |
| **biomeGate — toadStool** | Vendor tool excision + K80 sovereign | SHIPPED (10 commits). 216 tests recovered. |
| **sporeGate — cellMembrane** | Cascade ops + Windows depot | ONLINE. Windows 12/17. Autonomous. |
| **westGate — cellMembrane** | AlphaFold ingress pipeline | ACTIVE |

## Downstream Patterns

| Track | Status |
|-------|--------|
| **northGate enrollment** | 🔄 WG mesh via golgi relay. Target: Tower Atomic → vine-bat zero-SSH |
| **{{ entity(name="guerillagorilla") }} formalization** | ACTIVE — amicusContra named, dispersal site concept |
| **Milk-V Jupiter 2 bring-up** | 🆕 RISC-V RVA23. `riscv64gc-unknown-linux-musl` target |
| **NUC bench composition** | 🆕 Tower Atomic on DDR3 NUCs — mesh expansion |
| bonsai-bt meta-primal (DECIDE layer) | **PHASE 0 — INGESTING** |
| arXiv submission | ACTIVE (strandGate) — primals.eco LIVE, UNBLOCKED |
| sporePrint → NUCLEUS live data surface | CONCEPT — static → live, phased evolution |
| tideGlass Phase 0 (gen5 sole bottleneck) | QUEUED |
| SSH → Tower Atomic graduation (NanoWire Tiers 2-7) | NEXT |

## golgiBody Recovery (Sep 15, 2026)

Cascade was **crash-looping for 3 weeks** (Aug 27 – Sep 15). Root cause:
stale `ecosystem_manifest.toml` with `mobility = "portable"` (unknown variant
in deployed membrane binary). 1,784 sense failures before manual reset.

| Fix | Detail |
|-----|--------|
| Cascade restored | Reset wateringHole checkout, deployed depot membrane binary |
| Disk recovered | 79% → 70% (journal cap, ghost binaries, reserved blocks) |
| Zola rebuild | Forced rebuild — sporePrint content serving fresh |
| Binary immutability | `chattr +i` on caddy, hbbs, hbbr, membrane, zola |
| GSC automation | Google Search Console API operational, sitemap resubmitted |

## Live Sites

| Site | URL | Status |
|------|-----|--------|
| **sporePrint** | `sporeprint.primals.eco` | **LIVE** — Zola fix shipped, cascade rebuilt |
| **detroit** | `detroit.primals.eco` | **LIVE** — public accountability (128+ pages, dynasty expansion, BLAKE3 braided) |
| **footPrint** | `footprint.primals.eco` | **LIVE** — CAS works |
| **nestgate.io** | `nestgate.io` | **LIVE** — trust surfaces + data braids |
| **esotericWebb** | `webb.primals.eco` | 502 — needs petalTongue WebGL (G19) |

## Public Accountability — {{ entity(name="guerillagorilla") }}

{{ entity(name="detroit") }} at detroit.primals.eco applies ecoPrimals infrastructure to public accountability.
Same substrate, different domain: git-signed evidence, BLAKE3 content manifests,
three independent surfaces (live site + sovereign repo + GitHub mirror).

{{ entity(name="guerillagorilla") }} is the **legal meta-primal** — a methodology where the infrastructure
IS the evidence integrity layer. Fourth independent instantiation of the {{ entity(name="provenancetrio") }}.
The same properties that make science reproducible make public records tamper-proof:

| Science | Accountability |
|---------|---------------|
| Reproducible results | Verifiable claims |
| BLAKE3 provenance | Git-signed commits |
| Peer review | Public verification |
| No cloud dependency | No platform dependency |

Named capabilities:

| Capability | Role | Description |
|------------|------|-------------|
| **fEAR** | HOW WE HEAR | Evidence intake, OSINT, signal detection |
| **preSCENT** | HOW WE SMELL | Pattern recognition, network mapping, presence broadcast |
| **STRIDe** | HOW WE MOVE | Filing, publication, strategic action |
| **amicusContra** | HOW WE PROJECT | *New.* Outward function — project toward cases operator is not party to |
| {{ entity(name="pursuitpredation") }} | WHY IT WORKS | Metabolic asymmetry — operator cost constant, adversary cost escalating |

**amicusContra directionality**: Downward (stabilize those beneath — prepare rooms), Lateral (cross-protection — mutual defense), Upward (hold those above to account — force reproducibility).

*"Beside the small. Against unaccountable power. For the record."*

Dispersal site: `guerillagorilla.primals.eco` (planned). {{ entity(name="detroit") }} is Case Study #1. Infrastructure: Forgejo, Zola, Caddy, golgiBody. Protected by First Amendment, Michigan UPEPA, fair report.

See: [guerillaGorilla methodology page](@/outreach/guerilla_gorilla.md) | [dispersal pattern](https://git.primals.eco/ecoPrimals/wateringHole/src/branch/main/specs/DISPERSAL_PATTERN.md)

## Pending: Live Dashboard

This page currently shows static data. When petalTongue G19 rendering
matures, it will serve real-time health data from `biomeOS neuralAPI`.
