# sporePrint — Project Context

Read this first. Everything an agent needs to make good decisions about sporePrint.

## What This Is

sporePrint is the public-facing website for the ecoPrimals sovereign scientific computing ecosystem. Hosted at **primals.eco** — sovereign-primary on golgiBody VPS (Caddy), with GitHub Pages as trailing shadow. Built with **Zola** (Rust static site generator).

**sporePrint is human-facing.** wateringHole is the dev-facing shared context repo. sporePrint explains what the ecosystem IS, what it does, and how to verify it. It is not a technical reference manual — it is a compass.

## Current State (August 11, 2026 — Wave 157i, PANDEMIC RESPONDS)

- **338 published pages** across 25 sections: ~200 active in main nav, 36 in backstory, 79 foundation (not in nav)
- **79 typed entities** across 7 kinds (primal, spring, product, composition, concept, infra, org)
- **126 bidirectional edges** (typed entity graph, 14 relation types)
- **`spore-validate` v0.3.1** — 34-module Rust crate, 283+ tests, `#![forbid(unsafe_code)]`, zero C toolchain deps
- **G72 Tier 1 COMPLETE**: 9/9 teams responded, ~114 crates shed fleet-wide
- **4-gate gossip mesh LIVE**: westGate → sporeGate, eastGate, strandGate. Epidemic propagation.
- **Gossip injection**: 6/16 primals LIVE (barraCuda 19 events fully wired, esotericWebb 2, songBird 1)
- **hotSpring pseudoSpore E2E**: pure Rust pipeline shipped (compute → sign → register)
- **graftGate**: M4 Mac Mini arrived, first `aarch64-apple-darwin` target, GLACIAL→ACTIVE
- **Ecosystem totals**: 3.60M LOC, ~150,000+ tests (16 primals + 9 springs), 43 repos
- **13/13+ GREEN**. G68 COMPLETE. **6/6 NUCLEUS.** **ZERO P0. ZERO P1. 1 P2.**
- **P2 braid.verify CLOSED** (sweetGrass behavioral tests)
- **sourDough CI**: shipped (4 static validators, 15 repos, advisory)
- **arXiv 41/42**: campaign IN PROGRESS

### Wave 157i — Pandemic Responds (G72 Tier 1 Complete)
- **G72 Tier 1 COMPLETE** — 9/9 teams responded. ~114 crates shed fleet-wide. toadStool tokio 118→65 files (45% reduction).
- **nestGate** — jsonrpsee removed (1,864 LOC, -10 crates). Deep debt S146.
- **rhizoCrypt** — wiremock removed (0 usage), -46 crates (14.6%).
- **loamSpine** — url+ICU chain excised, -7 crates. RustCrypto unified.
- **sweetGrass** — **P2 braid.verify CLOSED** (5 behavioral tests). Batch+verify extraction.
- **Gossip injection 6/16** — barraCuda 19 events fully wired, esotericWebb 2, songBird 1. Was 3/16.
- **hotSpring pseudoSpore E2E** — pure Rust: compute → manifest → bundle → sign → register.
- **graftGate** — M4 Mac Mini arrived. iPhone XS tethering. GLACIAL→ACTIVE.
- **Tier 2 queued** — HTTP→songBird, axum→0.8, wgpu→28, YAML unification.

### Previous (Wave 157g — Stadial Shift + 4-Gate Gossip Mesh)
- STADIAL → INTERSTADIAL — primals shed vestigial deps as compositions close gaps.
- G72 Dependency Pandemic formalized — 664 Cargo.toml audited. 3-tier excision plan.
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
