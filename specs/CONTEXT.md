# sporePrint — Project Context

Read this first. Everything an agent needs to make good decisions about sporePrint.

## What This Is

sporePrint is the public-facing website for the ecoPrimals sovereign scientific computing ecosystem. Hosted at **primals.eco** — sovereign-primary on golgiBody VPS (Caddy), with GitHub Pages as trailing shadow. Built with **Zola** (Rust static site generator).

**sporePrint is human-facing.** wateringHole is the dev-facing shared context repo. sporePrint explains what the ecosystem IS, what it does, and how to verify it. It is not a technical reference manual — it is a compass.

## Current State (August 8, 2026 — Wave 157a, GATE REDEPLOY + TRUST SURFACES LIVE)

- **338 published pages** across 25 sections: ~200 active in main nav, 36 in backstory, 79 foundation (not in nav)
- **79 typed entities** across 7 kinds (primal, spring, product, composition, concept, infra, org)
- **126 bidirectional edges** (typed entity graph, 14 relation types)
- **`spore-validate` v0.3.1** — 34-module Rust crate, 283 tests, `#![forbid(unsafe_code)]`, zero C toolchain deps
- **Ecosystem totals**: 3.60M LOC, 135,000+ tests (15 primals + 9 springs), 43 repos
- **13/13 GREEN**. G68 COMPLETE. **3/6 NUCLEUS gates redeployed.**
- **Trust surfaces LIVE**: nestgate.io `/api/content/stats`, `/pseudospore/` (5 bundles), `validate.sh`
- **SSH key discipline ENFORCED**: eastGate, blueGate, southGate all compliant.
- **3.21 TB / 153 datasets** on westGate. Depot: Musl 17/17, Windows 15/15. Cascade auto-push.
- **arXiv 41/42**: SU(3) COMPLETE (36 configs), SU(4) running. Trust surface partially unblocked.
- **strandGate DIVERGED**: needs SSH depot access to golgi.
- **ZERO P0/P1/P2**

### Wave 157a — Gate Redeploy + Trust Surfaces
- **3/6 gates redeployed** — sporeGate 13/13, blueGate 13/13 (Windows), southGate 13/13 (96 MB RSS)
- **Trust surfaces LIVE** — nestgate.io `/api/content/stats` + `/pseudospore/` + `validate.sh`
- **Cascade auto-push** — ExecStartPost rsync to golgi. synced=15, zero drift.
- **strandGate DIVERGED** — 2+ months stale, GitHub stale, Forgejo parse fails, no SSH to golgi
- **SU(3) COMPLETE** (36 configs), SU(4) running, NPU hardware live
- **blueGate Windows issues** — 3 P3/P4 (skunkBat env, petalTongue port, songBird PID)
- **SSH discipline** — eastGate, blueGate, southGate compliant. K-Derm relay chain enforced.

### Previous Eras
- **156d**: G18 LIVE, Phase 1 SUCCEEDED, footPrint DEPLOYED, 16⁴ dual-GPU COMPLETE, Convoy 145/s
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
