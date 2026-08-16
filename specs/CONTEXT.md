# sporePrint — Project Context

Read this first. Everything an agent needs to make good decisions about sporePrint.

## What This Is

sporePrint is the public-facing website for the ecoPrimals sovereign scientific computing ecosystem. Hosted at **primals.eco** — sovereign-primary on golgiBody VPS (Caddy), with GitHub Pages as trailing shadow. Built with **Zola** (Rust static site generator).

**sporePrint is human-facing.** wateringHole is the dev-facing shared context repo. sporePrint explains what the ecosystem IS, what it does, and how to verify it. It is not a technical reference manual — it is a compass.

## Current State (August 16, 2026 — Wave 157k, ENMESHMENT + INGESTION)

- **338 published pages** across 25 sections: ~200 active in main nav, 36 in backstory, 79 foundation (not in nav)
- **79 typed entities** across 7 kinds (primal, spring, product, composition, concept, infra, org)
- **126 bidirectional edges** (typed entity graph, 14 relation types)
- **`spore-validate` v0.3.1** — 34-module Rust crate, 283+ tests, `#![forbid(unsafe_code)]`, zero C toolchain deps
- **12 gates ONLINE**: eastGate, ironGate, strandGate, westGate, sporeGate, blueGate, graftGate, southGate, biomeGate, grapheneGate, iosGate, steamGate
- **ZERO P0, P1, P2** — all cleared
- **bonsai-bt FORKED**: DECIDE layer meta-primal. exp125 23/24 checks. Phase 0 ingesting.
- **rootPulse 6/6 graphs REGISTERED** (biomeOS `af1dc9d3`). Item #10 CLOSED.
- **Titan V Tier 1 CONFIRMED** (biomeGate). 4 measurement bugs fixed. `RegisterRead` enum.
- **graftGate FULL NUCLEUS** (Darwin). 16/16 depot CURRENT. builder.serve LIVE.
- **Gossip injection**: 6/16 primals LIVE (barraCuda 19 events, esotericWebb 2, songBird 1)
- **G72 Tier 1 COMPLETE**: 9/9 teams, ~114 crates shed fleet-wide
- **Ecosystem totals**: 3.60M LOC, ~150,000+ tests (16 primals + 9 springs), 43 repos
- **227 files fossilized** (1,513 total records). 11 active handoffs remain.
- **arXiv 41/42**: campaign IN PROGRESS
- **NOTE**: primals.eco Zola build/deploy regression — triage needed on sporeGate

### Wave 157k — Enmeshment + Ingestion
- **12 gates ONLINE** — biomeGate (Tower+Node), grapheneGate (ADB), iosGate (BearDogApp), steamGate (portable) join fleet
- **0/0/0** — all P0, P1, P2 cleared
- **bonsai-bt FORKED** — MIT, 3,197 LOC, 0 unsafe. DECIDE layer between squirrel REASON and biomeOS ROUTE.
- **exp125 LIVE** — 23/24 checks (5 behavior trees against NUCLEUS). EcoAction generic over Neural API.
- **rootPulse 6/6 REGISTERED** — commit, harvest, branch, merge, diff, federate. biomeOS 1,608 tests.
- **Titan V Tier 1 CONFIRMED** — 4 measurement bugs fixed. RegisterRead enum. K80 blocked (GK210).
- **graftGate FULL NUCLEUS** — aarch64-apple-darwin 16/16 depot CURRENT. builder.serve LIVE :9800.
- **tideGlass Phase 0** — external review from northGate. 5-7 days estimate. "Pivot point is now."
- **227 fossilized** — 10 files this wave. 1,513 total records.
- **NanoWire Tier 1 RETIRED** — 3/3 sub-builders enmeshed (no SSH for CI dispatch).

### Previous (Wave 157i — Pandemic Responds)
- G72 Tier 1 COMPLETE (9/9 teams, ~114 crates shed). Gossip 6/16 LIVE.
- hotSpring pseudoSpore E2E shipped. darwinGate→graftGate M4 arrived.

### Previous (Wave 157g — Stadial Shift + 4-Gate Gossip Mesh)
- STADIAL → INTERSTADIAL. G72 Dependency Pandemic formalized.
- 4-gate gossip mesh LIVE. biome.yaml CONVERGED. sourDough CI shipped.

### Previous (Wave 157d — Depot Unified + G69 + Deep Debt)
- ZERO P0 — all 3 resolved. Depot unified + pruned. G69 lineage spec.
- Build system mesh-native. Neural API unblocked. 13,910 caps.
- spore-validate deep debt resolved (8 items).

### Previous Eras
- **157a early**: G68 convergence, SSH key discipline, trust surfaces LIVE
- **156d**: G18 LIVE, Phase 1 SUCCEEDED, footPrint DEPLOYED, 16⁴ dual-GPU COMPLETE
- **155n–155v**: K-Derm DNS, Data Braids, G19 PROVEN, tideGlass Rust rebuild

## Site Topology — Active Pages by Section

| Section | Active | Foundation | Total | Nav |
|---------|--------|-----------|-------|-----|
| Lab | 132 | 0 | 132 | Main |
| Science | 33 | 0 | 33 | Main |
| Architecture | 14 | 29 | 43 | Main |
| Products | 7 | 4 | 11 | Main |
| pseudoSpore | 8 | 0 | 8 | Main |
| Data | 18 | 0 | 18 | Main |
| Getting Started | 0 (+index) | 0 | 1 | Main |
| Technical | 4 | 4 | 8 | Main |
| Thesis | 18 | 0 | 18 | Backstory |
| Philosophy | 15 | 0 | 15 | Backstory |
| Story | 3 | 0 | 3 | Backstory |
| guideStone | 6 | 0 | 6 | Foundation |
| Methodology | 1 | 14 | 15 | Foundation |
| Outreach | 0 | 16 | 16 | Foundation |
| Audience | 0 | 7 | 7 | Foundation |
| Collaborators | 0 | 3 | 3 | Foundation |
| Vision | 0 | 2 | 2 | Foundation |
| **Total** | **259** | **79** | **338** | |

## Key Technical Facts

- **Zola** — TOML front matter, Tera templates, strict mode
- **Catppuccin** color palette (Mocha dark / Latte light)
- **`spore-validate check-links`** validates internal links
- **JavaScript**: viz-hydrate.js (WASM progressive enhancement)
- **`#![forbid(unsafe_code)]`** — enforced at crate root
- **Entity shortcodes**: `entity`, `entity_metrics`, `entity_stat`, `total_stat`, `maturity`, `viz_embed`
- **Self-certifying**: guideStone manifest at `/certification/manifest.json`
- **Content-manifest**: BLAKE3 per-page hashes at `/content-manifest.toml`
- **AI surface**: 5 `llms*.txt` files, identity.json, entity-graph.json, robots.txt (fully open)

## Dependencies on Other Repos

| Repo | What sporePrint gets from it |
|------|------------------------------|
| wateringHole | Standards, glossary, handoffs — linked directly (public repo) |
| whitePaper | Deep docs transplanted (architecture, philosophy, methodology) |
| plasmidBin | Deployment model facts, depot inventory |
| petalTongue | Visualization pipeline, content rendering |

## What Needs Periodic Refresh

Metrics flow from repos → `config.toml` entity registry → shortcodes in content.

- **Entity registry metrics** — `spore-validate refresh <repos_root>` detects drift
- **Aggregate totals** in `[extra.totals]` — recompute from individual entries
- **Content manifest** — `spore-validate provenance --write` after content changes
- **Certification manifest** — `spore-validate certify --emit` after entity/edge changes
