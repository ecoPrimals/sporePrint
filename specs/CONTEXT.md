# sporePrint — Project Context

Read this first. Everything an agent needs to make good decisions about sporePrint.

## What This Is

sporePrint is the public-facing website for the ecoPrimals sovereign scientific computing ecosystem. Hosted at **primals.eco** — sovereign-primary on golgiBody VPS (Caddy), with GitHub Pages as trailing shadow. Built with **Zola** (Rust static site generator).

**sporePrint is human-facing.** wateringHole is the dev-facing shared context repo. sporePrint explains what the ecosystem IS, what it does, and how to verify it. It is not a technical reference manual — it is a compass.

## Current State (July 27, 2026 — Wave 155b)

- **313 published pages** across 17 sections, organized into 5 cortical folds
- **79 typed entities** across 7 kinds (primal, spring, product, composition, concept, infra, org) with metrics, descriptions, and link targets
- **126 bidirectional edges** (typed entity graph, 14 relation types)
- **10 reading trails** threading pages across fold boundaries
- **~110 pages** carry companion metadata for cross-domain discovery
- **4 organizations**: ecoPrimals (infra), syntheticChemistry (springs), sporeGarden (products), protoKarya (protists)
- **`spore-validate` v0.3.1** — 34-module Rust crate, 289 tests, `#![forbid(unsafe_code)]`, zero C toolchain deps
- **Ecosystem totals** (155b refresh): 3.60M LOC, 93,700 tests (15 primals + 9 springs), 43 repos, genomeBin 5 targets
- **Tower Atomic PROVEN**: bearDog + songBird + skunkBat exceed WireGuard — 353× LAN via topology awareness + LAN dispatch priority, 1.7× sustained on degraded WAN. 6/6 exploration domains PROVEN LIVE. 360+ shadow benchmark files. BTSP 13/13. Crypto delegation 6/6 COMPLETE. Autonomous enrollment LIVE (F10 fossilized). genomeBin cross-platform: 5 targets. skunkBat public. Chimera Phase 0 unblocked
- **197 validation scenarios**: all PASS. 10 fossilized dimensions
- **7 gates ONLINE** (sporeGate, eastGate, ironGate, flockGate, golgiBody, grapheneGate, northGate), **5 HW READY** (strandGate, westGate, blueGate, swiftGate, southGate)
- **Glacial goals**: G1 Tower on Windows, G2 Tower on Android, G3 Nest Atomic Phase 0, G5 Chimera, G6 bearDog public, G7 gate enmeshment, G9 JOSS
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
