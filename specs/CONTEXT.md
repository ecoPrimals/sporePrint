# sporePrint — Project Context

Read this first. Everything an agent needs to make good decisions about sporePrint.

## What This Is

sporePrint is the public-facing website for the ecoPrimals sovereign scientific computing ecosystem. Hosted at **primals.eco** — sovereign-primary on golgiBody-ext VPS (Caddy), with GitHub Pages as trailing shadow. Built with **Zola** (Rust static site generator).

**sporePrint is human-facing.** wateringHole is the dev-facing shared context repo. sporePrint explains what the ecosystem IS, what it does, and how to verify it. It is not a technical reference manual — it is a compass.

## Current State (July 10, 2026 — Wave 136b)

- **259 content pages** across 16 sections (Architecture, Audience, Contact, Glossary, guideStone, Lab, Methodology, Philosophy, Primals, Products, Science, Sitemap, Springs, Story, Technical, Thesis) + landing + lab notebooks
- **2 taxonomies**: `primals` (15 terms), `springs` (8 terms) — build-validated typed tags
- **Entity registry** in `config.toml` — 66 typed entities across 7 kinds (primal, spring, product, composition, concept, infra, org) with metrics, descriptions, and link targets
- **Typed entity graph** — 126 bidirectional edges (63 declared + 63 inverse) across 66 nodes, implementing Diderot's renvois de choses. 14 edge relation types. Validated at build time. Rendered as "Connections" panel on taxonomy pages.
- **6 shortcodes**: `entity` (linked name), `entity_metrics` (LOC/tests/files line), `entity_stat` (single metric), `total_stat` (aggregate), `maturity` (6-level maturity badges), `viz_embed` (SVG with WASM progressive enhancement)
- **Evidence Snapshot** — canonical metrics page pulling all numbers from registry via shortcodes. All March 2026 docs carry historical snapshot banners linking to it.
- **`spore-validate` v0.3.1** — 34-module Rust crate (11,014L): typed validation, link checking, notebook rendering, metric sync, graph building, certification, CAS manifest + push, discovery, NUCLEUS profile validation + display + probing, depot integrity verification, petalTongue IPC client, Tower P1 readiness probe (TOML-driven), HTTP/tar utilities, trait-based VCS, shared IPC module, parity integration tests. 284 tests (252 unit + 29 integration + 3 refresh_write, 6 parity ignored), `#![forbid(unsafe_code)]`, `#![warn(missing_docs)]`, zero clippy warnings (pedantic + nursery), zero C toolchain dependencies (blake3 pure-Rust, flate2 miniz_oxide), `cargo-deny` clean, `toml` 1.x (TOML spec 1.1). Module architecture: `main.rs` (thin entry, 67L) → `cli.rs` (Clap types) → `dispatch.rs` (command routing) → `commands*.rs` (validate/provenance/discover). NUCLEUS probing extracted to `nucleus_probe.rs`. Tower probes data-driven via `default_tower_probes.toml` (embedded, `include_str!`). All files under 680L (max: nucleus.rs at 670). CAS push error handling evolved to structured `PushFileOutcome::Error(String)`. Fetch returns `Result<Vec<FetchOutcome>, Error>` (was silent `Vec::new()` on failure). Integration test harness deduplicated via `tests/common/mod.rs`.
- **Proto-nucleate manifest** — `spore-validate nucleus --profile <path>` validates running NUCLEUS against deployment profiles. Topology classes: Full (13/13), Tower (3/3), Nest (7/7), Fieldmouse (canary), Relay (2/2). Socket probing via standard discovery chain. `--probe` flag validates guideStone health contract (`{status, primal, version}`). `--ribocipher` flag tests mito-beacon signal acceptance per primal (diagnoses genetics-layer wiring — Wave 114 exit criterion #7).
- **Depot integrity verification** — `spore-validate depot-verify --checksums <path> --depot <dir> --arch <target>` validates binary artifacts against BLAKE3 checksums from `checksums.toml`. Supports `--partial` mode for incremental depot validation (pass when all present binaries verify). Streaming BLAKE3 computation handles large binaries. Directly supports Wave 114 exit criteria (WAN/LAN depot pull validation).
- **Graph subcommand** — `spore-validate graph --emit` builds entity graph and writes `static/graph/entity-graph.json`
- **Certify subcommand** — `spore-validate certify --emit` computes BLAKE3 Merkle root, emits guideStone certification manifest to `static/certification/manifest.json`
- **Self-certifying publication** — every page carries a certification badge linking to the verifiable manifest; any reader can reproduce with `spore-validate certify`
- **Accessibility: WCAG 2.2 AAA target** — AA contrast ratios achieved (4.8:1+ accent), `prefers-reduced-motion` support, `prefers-contrast: more` (light + dark), `forced-colors: active` (Windows High Contrast), ARIA landmarks/combobox/listbox, decorative emoji hidden, keyboard navigation, semantic heading hierarchy, text-only status indicators, mobile TOC, 404 with full chrome. Screen reader testing is evolution target. petalTongue will set the universal access standard — any human capability profile, no intermediary.
- **Site tree sidebar** — collapsible section-level navigation with current-page highlighting
- **Card-based landing page** — stats ribbon, audience cards, org cards, explore cards (no tables)
- **Full-text search** — Zola's built-in elasticlunr, indexed at build time
- **Sovereign deployment** — golgi VPS serving via Caddy at 67ms TTFB (sovereign-primary since Wave 136). Security headers (HSTS preload, X-Frame DENY, nosniff, Permissions-Policy), fail2ban on Forgejo SSH, proper 404, ACME auto-renewal, HTTP/3, gzip. GitHub Pages is trailing shadow. Build pipeline: Sovereign CI (Forgejo hook → sporeGate build → rsync to golgi).
- **Pure-primal evolution path** — petalTongue DocumentNode types + content rendering pipeline implemented. sporePrint can be served by Nest Atomic composition (petalTongue web → NestGate CAS → provenance trio). Zola remains as validation oracle.
- **Local nest validation** — `content-direct` backend reads raw markdown from disk, renders through DocumentNode pipeline with entity shortcode resolution and multi-modal output (HTML, description, JSON). Parity confirmed 22/22 with Zola (also as Rust integration tests: `tests/parity.rs`).
- **Live ecosystem visualizations** — Entity graph (force-directed, 66 nodes), K-Derm topology (5-layer cross-section with relay animation), NUCLEUS composition (nested layers with expand/collapse). Server-side SVG with WASM progressive enhancement.
- **VizRegistry** — Capability-based discovery of available visualizations at petalTongue startup. No hardcoded route dispatch — registry pattern enables future viz additions without modifying route handlers.
- **Deep debt resolved** — LazyLock regex statics, parameterized notebook paths, modularized viz_data, deprecated shell scripts superseded by Rust. push_manifest decomposed (PushFileOutcome enum). HTTP/tar extracted from fetch.rs. announce_request canonical. One `#[allow(dead_code)]` remaining: `MaturityLevel::css_class` (P2 petalTongue Tera integration). refresh::scan DRYed with closure-based drift push. HTTP redirect handling fixed for relative paths. Zero production `unwrap()`/`expect()` (all in tests or static regex `LazyLock`). `connect_timeout` + I/O timeouts on all transports. `Content-Length` validation for truncation detection. `DiagnosticCollector` fully exercised (5 new tests). Test harness helpers deduplicated to `tests/common/mod.rs`.
- **primalSpring validation: 70/70 PASS** — `sporeprint-pure-primal-parity` scenario passes all checks (content parsing, entity resolution, modality output, composition graph, certification manifest). Certification manifest now emits `schema_version` + `merkle_root` fields per primalSpring expectations.
- **Metrics freshness** — all 25 entity metrics refreshed (3.46M LOC, 114K tests ecosystem-wide). Drift tolerance maintained.
- **WAN depot status (Wave 119)** — Depot rebuilt from HEAD (2026-06-15T14:05Z), 14 x86_64 + 14 aarch64 binaries. WAN fetch verified via `depot-verify --partial`. Persistent federation peer enrollment completed by cellMembrane. Full depot path: `membrane.primals.eco/depot/{arch}/{slug}` → b3sum verification → launch.
- **petalTongue backend wiring (Wave 123)** — IPC client validates petalTongue v1.6.6 health (56 methods, healthy, up). `pt-status`, `pt-render`, `pt-viz` subcommands. Entity graph format alignment pending (upstream schema mismatch).
- **Tower P1 readiness (Wave 123)** — `tower-status` probe reports 6/9 methods available. BearDog: `auth.public_key` + `auth.trusted_issuers` live. Songbird: mesh methods exist (need `mesh.init` activation). SkunkBat: `auth.check` live. Gaps fed upstream.

## Repository Structure

```
sporePrint/
├── config.toml              # Zola config + taxonomies + entity_registry
├── content/                 # All site content (Markdown + TOML front matter)
│   ├── _index.md            # Landing page (minimal — cards are in index.html template)
│   ├── architecture/        # 20+ pages: catalogs, inventory, NUCLEUS, deployment, topology, certify
│   ├── audience/            # 5 pages: role-based entry points
│   ├── guidestone/          # guideStone verification class
│   ├── methodology/         # 10 pages: constrained evolution, K-Nome, spring guide, sharing the pen, scyBorg, inoculum
│   ├── thesis/              # 18 pages: PhD dissertation (16 chapters + index + refs, fully transplanted)
│   ├── philosophy/          # 14 pages: atlasHugged essays (12) + bibliography + sovereign science
│   ├── story/               # 3 pages: builder narrative essays (the journey)
│   ├── products/            # 6 pages: esotericWebb, helixVision, blueFish, lattice QCD, lithoSpore
│   ├── science/             # 34 pages: baseCamp papers (29) + gonzales + reference docs
│   └── technical/           # 8 pages: hardware, GPU pipeline, grants, teaching, barracuda gaps, neuromorphic
├── templates/
│   ├── base.html            # Layout: nav, site tree sidebar, footer, search
│   ├── index.html           # Landing page: hero + cards (stats, audience, orgs, explore)
│   ├── page.html            # Single page: breadcrumbs, TOC sidebar, content
│   ├── section.html         # Section listing: card grid of child pages
│   ├── science_section.html # Science section: groups papers by [extra] domain
│   ├── taxonomy_list.html   # All terms in a taxonomy (e.g., /primals/)
│   ├── taxonomy_single.html # All pages for one term (e.g., /primals/beardog/)
│   └── shortcodes/
│       └── entity.html      # {{ entity(name="beardog") }} → linked emoji+name
├── static/
│   ├── css/base.css         # Design tokens (Catppuccin Mocha/Latte)
│   ├── css/main.css         # Component styles
│   ├── CNAME                # primals.eco
│   └── search.css
├── specs/                   # THIS DIRECTORY — internal, not built
├── crates/spore-validate/   # Rust validation crate (34 modules, 284 tests)
├── .github/workflows/       # deploy.yml, auto-refresh.yml
└── CHANGELOG.md
```

## Key Technical Facts

- **Zola** — TOML front matter, Tera templates, strict mode
- **Catppuccin** color palette (Mocha dark / Latte light, auto via `prefers-color-scheme`)
- **`spore-validate check-links`** validates internal links across content files
- **zola build** generates taxonomy pages automatically from front matter tags
- **JavaScript**: viz-hydrate.js (WASM progressive enhancement, SVG fallback)
- **Static SVG favicon** — `static/favicon.svg`
- **`minify_html = true`** — output is minified
- **`#![forbid(unsafe_code)]`** — enforced at spore-validate crate root

## Three Organizations

| Org | Role | URL |
|-----|------|-----|
| ecoPrimals | 17 primals + tooling (infrastructure) | github.com/ecoPrimals |
| syntheticChemistry | 8 springs (science validation) | github.com/syntheticChemistry |
| sporeGarden | Products (esotericWebb, helixVision, blueFish) | github.com/sporeGarden |

## Content Principles

1. **Tufte-esque**: every element justifies its space — no decorative filler
2. **Executable claims**: if we state a number, the reader can run a binary to verify it
3. **Agentic-friendly**: TOML front matter + structured Markdown = machine-parseable
4. **PII-minimal**: no personal names beyond published researchers, no locations, no employment history
5. **Replication, not endorsement**: researchers are listed as published work being reproduced, not collaborators or endorsers (see `CONTENT_VOICE.md`)
6. **Metrics over grades**: lines of code, test count, coverage percentage — not subjective letter grades

## Dependencies on Other Repos

| Repo | What sporePrint gets from it |
|------|------------------------------|
| wateringHole | GLOSSARY, PRIMAL_EMOJI_STANDARD, PUBLIC_SURFACE_STANDARD, LINK_INTEGRITY_STANDARD |
| whitePaper | gen3/ primal narratives, gen4/ guideStone architecture, baseCamp paper content |
| plasmidBin | Deployment model facts, metadata.toml format, current inventory counts |
| petalTongue | SPOREPRINT_CONTENT_DELIVERY_SPECIFICATION (future: how petalTongue consumes this site) |

## What Needs Periodic Refresh

Metrics flow from repos → `config.toml` entity registry → shortcodes in content. Update `config.toml` and everything else follows.

- **Entity registry metrics** (LOC, tests, files, crates) — run `spore-validate refresh <repos_root>` to detect drift, then update `config.toml`
- **Aggregate totals** in `[extra.totals]` — recompute from individual entries (`spore-validate validate` checks these)
- Squirrel version (fast-moving)
- plasmidBin inventory (new primals get metadata.toml entries)
- Science paper descriptions (new experiments get added to springs)
- Taxonomy tags in front matter when new content references new entities

## Nav Bar Structure

Science · Architecture · Lab · Thesis · Story · guideStone · Philosophy · Products | Primals · Springs · GitHub

Audience, Methodology, and Technical are accessible via the site tree sidebar.
