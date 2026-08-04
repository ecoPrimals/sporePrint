# sporePrint — Project Context

Read this first. Everything an agent needs to make good decisions about sporePrint.

## What This Is

sporePrint is the public-facing website for the ecoPrimals sovereign scientific computing ecosystem. Hosted at **primals.eco** — sovereign-primary on golgiBody VPS (Caddy), with GitHub Pages as trailing shadow. Built with **Zola** (Rust static site generator).

**sporePrint is human-facing.** wateringHole is the dev-facing shared context repo. sporePrint explains what the ecosystem IS, what it does, and how to verify it. It is not a technical reference manual — it is a compass.

## Current State (August 4, 2026 — Wave 155u/156b, DEMONSTRATION ERA)

- **338 published pages** across 25 sections: ~200 active in main nav, 36 in backstory, 79 foundation (not in nav)
- **79 typed entities** across 7 kinds (primal, spring, product, composition, concept, infra, org)
- **126 bidirectional edges** (typed entity graph, 14 relation types)
- **`spore-validate` v0.3.1** — 34-module Rust crate, 283 tests, `#![forbid(unsafe_code)]`, zero C toolchain deps
- **Ecosystem totals**: 3.60M LOC, 130,000+ tests (15 primals + 9 springs), 43 repos
- **13/13 GREEN** — barraCuda PRNG FIXED (last holdout)
- **11 NUCLEUS gates**: eastGate, westGate, blueGate, strandGate, southGate, sporeGate, flockGate, golgiBody, biomeGate, ironGate, redGate
- **519 GB / 130+ datasets** on westGate across 17+ domains
- **ZERO P0/P1/P2**

### Demonstration Era Changes (Wave 155n → 155u/156b)
- **Nav triage**: pseudoSpore | Data | Lab | Science | Architecture | Products | Get Started
- **13/13 GREEN**: barraCuda PRNG FIXED. esotericWebb V29. 130K+ tests.
- **Data Braids**: 16 domain pages with inline W3C PROV-O braids + transplant page
- **Provenance × Acquisition divergence**: 12× throughput gap discovered — trailer pattern + batch RPC fix path
- **arXiv UNBLOCKED**: strandGate validation COMPLETE, paper relabel pending
- **79 foundation pages**: architecture (29), methodology (14), outreach (16), audience (7), products (4), technical (4), collaborators (3), vision (2)
- **Backstory**: thesis (18), philosophy (15), story (3) — accessible via nav footer
- **7 VALIDATED badges** on baseCamp papers (10, 14, 21, 24, 7, 17, 28)
- **6 architecture pages** upgraded to `maturity = "live"`
- **Auto-publish**: golgi-ext 15-min systemd timer (Forgejo pull → zola build → Caddy serves)
- **Homepage**: 11 gates, 519 GB, 130K+ tests, 13/13 GREEN
- **G19 MILESTONE**: petalTongue scene push PROVEN on ironGate (RTX 5070)
- **esotericWebb V29**: G18 signal dispatch, petalTongue live site, deep debt
- **ironGate downstream**: esotericWebb + footPrint (563 tests) + squirrel + petalTongue
- **squirrel**: test perf 400s→16s, 34→1 binaries
- **tideGlass**: full Rust rebuild, 9 crates, 147 tests, 92.71% coverage

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
