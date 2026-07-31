# sporePrint — Project Context

Read this first. Everything an agent needs to make good decisions about sporePrint.

## What This Is

sporePrint is the public-facing website for the ecoPrimals sovereign scientific computing ecosystem. Hosted at **primals.eco** — sovereign-primary on golgiBody VPS (Caddy), with GitHub Pages as trailing shadow. Built with **Zola** (Rust static site generator).

**sporePrint is human-facing.** wateringHole is the dev-facing shared context repo. sporePrint explains what the ecosystem IS, what it does, and how to verify it. It is not a technical reference manual — it is a compass.

## Current State (July 30, 2026 — Wave 155m, NUCLEUS ACHIEVED)

- **313 published pages** across 17 sections, organized into 5 cortical folds
- **79 typed entities** across 7 kinds (primal, spring, product, composition, concept, infra, org) with metrics, descriptions, and link targets
- **126 bidirectional edges** (typed entity graph, 14 relation types)
- **10 reading trails** threading pages across fold boundaries
- **~110 pages** carry companion metadata for cross-domain discovery
- **4 organizations**: ecoPrimals (infra), syntheticChemistry (springs), sporeGarden (products), protoKarya (protists)
- **`spore-validate` v0.3.1** — 34-module Rust crate, 289 tests, `#![forbid(unsafe_code)]`, zero C toolchain deps
- **Ecosystem totals** (155m): 3.60M LOC, 101,308 tests (15 primals + 9 springs), 43 repos, 35 depot binaries (16 musl + 4 gnu + 15 Windows)
- **NUCLEUS ACHIEVED**: 3 gates running full NUCLEUS (13/13 primals). gen4 COMPLETE
- **Provenance 7/7 COMPLETE**: Full signed chain validated on Linux (ZFS) + Windows
- **Sovereign CI LIVE**: push-to-deploy automated, zero human intervention
- **ZERO P0s. ZERO P1s. ZERO blocking P2s**
- **27 signal graphs**. 13 fossilized dimensions
- **10 gates ONLINE**. 3 NUCLEUS gates (westGate, blueGate, strandGate)
- **Next**: gen5 — NUCLEUS as platform. steamGate (Steam Deck). AlphaFold ingestion. JOSS
- **Sovereign CI pipeline**: Forgejo → sporeGate build → depot → all gates. No GitHub Actions
- **Crash-loop breaker**: cellMembrane `gate.crash-loop` — self-recovery shipped
- **DNSSEC**: all 3 domains signed (primals.eco, primal.eco, nestgate.io)
- **Phase 2 transport abstraction**: 14/14 primals complete — trait + backend patterns replace all `#[cfg]` fences
- **Content-Addressed Convergence**: 6/6 layers complete — content identity supersedes temporal identity
- **Identity model**: ecoPrimal (developer), attsi (philosopher) — dual-voice, no PII
- **Sovereign deployment**: golgiBody VPS via Caddy, sovereign CI (Forgejo → sporeGate → golgi), golgiBody sole depot
- **6 surfaces LIVE from WAN**: footPrint, esotericWebb (V22), sporePrint, TOPO-VIS, Forgejo, JupyterHub
- **4-org Forgejo**: 43/43 repos mirrored, origin=Forgejo, push mirrors → GitHub
- **Autonomous enrollment**: gate-enroll.sh (Linux) / gate-enroll.ps1 (Windows), genetic enrollment (mito + nuclear lineage)
- **Three-domain model**: primals.eco (intra-membrane), primal.eco (inner membrane), nestgate.io (data service)
- **URL standard**: `prefix.primals.eco` subdomains for all compositions (path-based prohibited)
- **Tracks converged**: Evolution (A) and Fleet Convergence (B) unified — next frontier is cross-platform proof
- **P0 CLEAR all gates**

## Site Topology — Cortical Folds

| Fold | Sections | % of site |
|------|----------|-----------|
| Evidence | Lab, Science | 60% |
| Architecture | Architecture, Technical, Products | 19% |
| Methodology | Methodology, guideStone | 7% |
| Philosophy | Philosophy, Story, Thesis | 12% |
| Engagement | Glossary, Audience, Outreach, Collaborators | 8% |

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
