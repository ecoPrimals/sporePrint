# sporePrint — Project Context

Read this first. Everything an agent needs to make good decisions about sporePrint.

## What This Is

sporePrint is the public-facing website for the ecoPrimals sovereign scientific computing ecosystem. Hosted at **primals.eco** — sovereign-primary on golgiBody VPS (Caddy), with GitHub Pages as trailing shadow. Built with **Zola** (Rust static site generator).

**sporePrint is human-facing.** wateringHole is the dev-facing shared context repo. sporePrint explains what the ecosystem IS, what it does, and how to verify it. It is not a technical reference manual — it is a compass.

## Current State (August 9, 2026 — Wave 157a, VERTEBRATE EVOLUTION COMPLETE)

- **338 published pages** across 25 sections: ~200 active in main nav, 36 in backstory, 79 foundation (not in nav)
- **79 typed entities** across 7 kinds (primal, spring, product, composition, concept, infra, org)
- **126 bidirectional edges** (typed entity graph, 14 relation types)
- **`spore-validate` v0.3.1** — 34-module Rust crate, 283 tests, `#![forbid(unsafe_code)]`, zero C toolchain deps
- **Ecosystem totals**: 3.60M LOC, 135,000+ tests (16 primals + 9 springs), 43 repos
- **13/13 GREEN**. G68 COMPLETE. **6/6 NUCLEUS gates redeployed.**
- **12/16 primals self-audited** — zero phantom APIs
- **P0-B RESOLVED**: nestGate `content.ingest` was shipped (stale depot was root cause)
- **P0-A CODE FIXED**: bearDog health guard (`766951004`). Depot-stale.
- **P0-C OPEN**: biomeOS FD leak (14→58K FDs, `capability.call` unusable)
- **Depot rebuild in progress**: sporeGate sole builder → golgi → gates pull
- **arXiv 41/42**: validate.sh + freeze/sign remain

### Wave 157a — Vertebrate Evolution Complete
- **12 teams self-audited** — zero phantom methods across all audited primals
- **P0-B RESOLVED** — `content.ingest` shipped since S136. Stale depot binary, not missing feature.
- **P0-A CODE FIXED** — bearDog health guard, socket rename. Depot rebuild needed.
- **P0-C OPEN** — biomeOS FD leak. `capability.call` forwarding unusable.
- **songBird** — `CanonicalTransport` trait shipped. 9 transports converging.
- **swarmVine** — 39→124 tests (82% coverage). Async dispatch.
- **petalTongue** — doom-core decoupled (ludoSpring-ready)
- **sourDough** — `rpc-surface` audit tool shipped: detects stubs + divergence live
- **Deployment discipline** — sporeGate sole depot builder. No gate self-builds.

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
