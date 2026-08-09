# sporePrint — Project Context

Read this first. Everything an agent needs to make good decisions about sporePrint.

## What This Is

sporePrint is the public-facing website for the ecoPrimals sovereign scientific computing ecosystem. Hosted at **primals.eco** — sovereign-primary on golgiBody VPS (Caddy), with GitHub Pages as trailing shadow. Built with **Zola** (Rust static site generator).

**sporePrint is human-facing.** wateringHole is the dev-facing shared context repo. sporePrint explains what the ecosystem IS, what it does, and how to verify it. It is not a technical reference manual — it is a compass.

## Current State (August 9, 2026 — Wave 157a, VERTEBRATE EVOLUTION — 3 P0s OPEN)

- **338 published pages** across 25 sections: ~200 active in main nav, 36 in backstory, 79 foundation (not in nav)
- **79 typed entities** across 7 kinds (primal, spring, product, composition, concept, infra, org)
- **126 bidirectional edges** (typed entity graph, 14 relation types)
- **`spore-validate` v0.3.1** — 34-module Rust crate, 283 tests, `#![forbid(unsafe_code)]`, zero C toolchain deps
- **Ecosystem totals**: 3.60M LOC, 135,000+ tests (16 primals + 9 springs), 43 repos
- **13/13 GREEN**. G68 COMPLETE. **6/6 NUCLEUS gates redeployed.**
- **3 P0s OPEN**: bearDog sign stub (P0-A), nestGate API mismatch (P0-B), biomeOS FD leak (P0-C)
- **Mesh**: code-complete, production-blocked by P0-C
- **westGate**: 989K files braided, 153 datasets, 3.3 TB
- **arXiv 41/42**: validate.sh + freeze/sign remain

### Wave 157a — Vertebrate Evolution
- **3 P0s OPEN** — westGate 7-session retrospective exposed critical gaps:
  - **P0-A**: bearDog depot binary returns health for ALL methods (spine commits unsigned)
  - **P0-B**: nestGate `content.ingest` doesn't exist (API surface diverged from consumers)
  - **P0-C**: biomeOS FD leak (14→58K FDs after 4 `capability.call` invocations)
- **Mesh code-complete, production-blocked** — `capability.resolve` works, forwarding leaks
- **Vertebrate evolution** — primals self-audit RPC surface, abstract shared traits
- **songBird** — 9 transports → shared `Transport` trait. 24 MB FIXED.
- **petalTongue** — doom-core → ludoSpring. 656 deps.
- **toadStool S371** — WASM split 24/48. `core` 272K natural split.
- **Vine-bat OPERATIONAL** — gossip.spread → metadata.analyze → accept/reject

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
