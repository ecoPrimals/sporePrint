# sporePrint — Project Context

Read this first. Everything an agent needs to make good decisions about sporePrint.

## What This Is

sporePrint is the public-facing website for the ecoPrimals sovereign scientific computing ecosystem. Hosted at **primals.eco** — sovereign-primary on golgiBody-ext VPS (Caddy), with GitHub Pages as trailing shadow. Built with **Zola** (Rust static site generator).

**sporePrint is human-facing.** wateringHole is the dev-facing shared context repo. sporePrint explains what the ecosystem IS, what it does, and how to verify it. It is not a technical reference manual — it is a compass.

## Current State (July 8, 2026 — Wave 134)

- **238 content pages** across 10 sections (+ Story, Philosophy expanded) + landing + lab notebooks
- **2 taxonomies**: `primals` (15 terms), `springs` (8 terms) — build-validated typed tags
- **Entity registry** in `config.toml` — 66 typed entities across 7 kinds (primal, spring, product, composition, concept, infra, org) with metrics, descriptions, and link targets
- **Typed entity graph** — 126 bidirectional edges (63 declared + 63 inverse) across 66 nodes, implementing Diderot's renvois de choses. 14 edge relation types. Validated at build time. Rendered as "Connections" panel on taxonomy pages.
- **6 shortcodes**: `entity` (linked name), `entity_metrics` (LOC/tests/files line), `entity_stat` (single metric), `total_stat` (aggregate), `maturity` (6-level maturity badges), `viz_embed` (SVG with WASM progressive enhancement)
- **Evidence Snapshot** — canonical metrics page pulling all numbers from registry via shortcodes. All March 2026 docs carry historical snapshot banners linking to it.
- **`spore-validate` v0.3.1** — 28-module Rust crate: typed validation, link checking, notebook rendering, metric sync, graph building, certification, CAS manifest + push, discovery, NUCLEUS profile validation + display, depot integrity verification, petalTongue IPC client, Tower P1 readiness probe (profile-driven), HTTP/tar utilities, trait-based VCS, shared IPC module, parity integration tests. 272 tests (240 unit + 29 integration + 3 refresh_write, 6 parity ignored), `#![forbid(unsafe_code)]`, `#![warn(missing_docs)]`, zero clippy warnings (pedantic + nursery), zero C toolchain dependencies (blake3 pure-Rust, flate2 miniz_oxide), `cargo-deny` clean, `toml` 1.x (TOML spec 1.1). Transport-agnostic CAS push via canonical `TransportEndpoint` enum (UDS/TCP/MeshRelay) with riboCipher Tier 1 signal support. Unified transport resolution via `discovery::resolve_primal_endpoint()` — all primal connections (NestGate, petalTongue) honor CLI override → `TRANSPORT_ENDPOINT` env → socket discovery. Centralized timeout constants in `paths.rs` (`PROBE_TIMEOUT`, `TRANSPORT_CONNECT_TIMEOUT`, `TRANSPORT_IO_TIMEOUT`). Discovery probes `BIOMEOS_SOCKET_DIR`, env-overridable `BIOMEOS_SYSTEMD_SOCKET_DIR` (default `/run/membrane/`), and `XDG_RUNTIME_DIR`. Capabilities derived from `discovery::SELF` with static `&'static [&'static str]` slices — no duplicated announce logic. Shared `ipc` module centralizes JSON-RPC 2.0 NDJSON framing with response ID correlation and `health.liveness` fallback. petalTongue IPC wired via `PetalTongueClient` (health.check, visualization.render.graph, visualization.export). Tower P1 probe validates methods via profile-driven `probe_methods` or default table, using shared `ipc::send_rpc`. Nucleus display extracted to `nucleus_display.rs` — all files under 800L (max 699).
- **Proto-nucleate manifest** — `spore-validate nucleus --profile <path>` validates running NUCLEUS against deployment profiles. Topology classes: Full (13/13), Tower (3/3), Nest (7/7), Fieldmouse (canary), Relay (2/2). Socket probing via standard discovery chain. `--probe` flag validates guideStone health contract (`{status, primal, version}`). `--ribocipher` flag tests mito-beacon signal acceptance per primal (diagnoses genetics-layer wiring — Wave 114 exit criterion #7).
- **Depot integrity verification** — `spore-validate depot-verify --checksums <path> --depot <dir> --arch <target>` validates binary artifacts against BLAKE3 checksums from `checksums.toml`. Supports `--partial` mode for incremental depot validation (pass when all present binaries verify). Streaming BLAKE3 computation handles large binaries. Directly supports Wave 114 exit criteria (WAN/LAN depot pull validation).
- **Graph subcommand** — `spore-validate graph --emit` builds entity graph and writes `static/graph/entity-graph.json`
- **Certify subcommand** — `spore-validate certify --emit` computes BLAKE3 Merkle root, emits guideStone certification manifest to `static/certification/manifest.json`
- **Self-certifying publication** — every page carries a certification badge linking to the verifiable manifest; any reader can reproduce with `spore-validate certify`
- **Site tree sidebar** — collapsible section-level navigation with current-page highlighting
- **Card-based landing page** — stats ribbon, audience cards, org cards, explore cards (no tables)
- **Full-text search** — Zola's built-in elasticlunr, indexed at build time
- **Sovereign deployment** — golgi VPS serving via Caddy at 67ms TTFB (vs GitHub Pages 111ms). DNS NS cutover pending (eastGate manual action). Build pipeline: Sovereign CI (Forgejo hook → sporeGate build → rsync to golgi). sporeGate is the sole build authority (pepti decommissioned Wave 120).
- **Pure-primal evolution path** — petalTongue DocumentNode types + content rendering pipeline implemented. sporePrint can be served by Nest Atomic composition (petalTongue web → NestGate CAS → provenance trio). Zola remains as validation oracle.
- **Local nest validation** — `content-direct` backend reads raw markdown from disk, renders through DocumentNode pipeline with entity shortcode resolution and multi-modal output (HTML, description, JSON). Parity confirmed 22/22 with Zola (also as Rust integration tests: `tests/parity.rs`).
- **Live ecosystem visualizations** — Entity graph (force-directed, 66 nodes), K-Derm topology (5-layer cross-section with relay animation), NUCLEUS composition (nested layers with expand/collapse). Server-side SVG with WASM progressive enhancement.
- **VizRegistry** — Capability-based discovery of available visualizations at petalTongue startup. No hardcoded route dispatch — registry pattern enables future viz additions without modifying route handlers.
- **Deep debt resolved** — LazyLock regex statics, parameterized notebook paths, modularized viz_data, deprecated shell scripts superseded by Rust. push_manifest decomposed (PushFileOutcome enum). HTTP/tar extracted from fetch.rs. announce_request canonical. Zero dead_code allows on production paths. refresh::scan DRYed with closure-based drift push. HTTP redirect handling fixed for relative paths. Zero production `unwrap()`/`expect()` (all in tests or static regex `LazyLock`). `connect_timeout` + I/O timeouts on all transports. `Content-Length` validation for truncation detection.
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
│   ├── methodology/         # 6 pages: constrained evolution, K-Nome, spring guide, sharing the pen
│   ├── philosophy/          # atlasHugged essays (6 — the "why")
│   ├── story/               # Builder narrative essays (3 — the journey)
│   ├── products/            # 4 pages: esotericWebb, helixVision, blueFish, lattice_qcd
│   ├── science/             # 32+ pages: baseCamp papers + gonzales + reference docs
│   └── technical/           # 6 pages: hardware, GPU pipeline, grants, teaching
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
├── crates/spore-validate/   # Rust validation crate (28 modules, 272 tests)
├── .github/workflows/       # deploy.yml, auto-refresh.yml
└── CHANGELOG.md
```

## Key Technical Facts

- **Zola** — TOML front matter, Tera templates, strict mode
- **Catppuccin** color palette (Mocha dark / Latte light, auto via `prefers-color-scheme`)
- **`spore-validate check-links`** validates internal links (149 links across 207 files)
- **zola build** generates taxonomy pages automatically from front matter tags
- **JavaScript**: viz-hydrate.js (WASM progressive enhancement, SVG fallback)
- **Inline SVG favicon** — no external assets
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

Science · Architecture · Lab · Story · guideStone · Philosophy · Products | Primals · Springs · GitHub

Audience, Methodology, and Technical are accessible via the site tree sidebar. Architecture highlights when browsing Methodology or Technical paths.
## Wave 64 Build Metrics (flockGate WAN)

| Metric | Value |
|--------|-------|
| Pages rendered | 226 |
| Build time | 746ms |
| Build host | flockGate (i9-13900K, NVMe) |
| Zola version | zola 0.20.0 |
| Date | 2026-05-31 |

## Wave 65: K-Derm Relay Confirmed

As of Wave 65, the Forgejo → GitHub relay is operational for sporePrint.
Push to `forgejo` only — the K-Derm diderm chain handles GitHub propagation
automatically via `golgi-post-receive-relay.sh` on golgiBody-inner.

## Wave 67: Glacial Cutover Coordination

flockGate status: **CLEAR**. Content cutover to sovereign hosting is Phase 2,
pending DNS NS registrar cutover (eastGate manual action). After DNS:
golgiBody-ext gets HTTPS, then Caddy routes to petalTongue:8080. GitHub
Pages becomes shadow-only, then fossilized.

Gate ownership confirmed in `GATE_TEAM_COORDINATION_MATRIX.md`. primalSpring's
`signals` vocabulary has evolved to `compositions` across all graphs/scenarios —
sporePrint content is unaffected (science content references biological signals,
not infrastructure signal graphs).

## Wave 69: S3 Content Cutover Preparation

Mission from eastGate (FRAGO `wave69-flockgate-content-cutover`):

1. **S3 Content Cutover (P2)** — VPS shadow is LIVE (NestGate + Caddy at 67ms
   TTFB). Build pipeline already targets VPS via relay-chain. DNS NS cutover
   remains the blocker (eastGate manual action). After DNS: GitHub Pages deploy
   workflow archived, VPS becomes sole primary.
2. **WAN Relay Maintenance (P3)** — Temporal cascade sync operating (~1.3s
   Forgejo propagation, ~3s end-to-end). Report TURN anomalies to eastGate.

**Cutover readiness checklist:**
- [x] VPS serving validated (22/22 content-direct parity)
- [x] Build pipeline sovereign (systemd-timer + relay-chain on peptidoglycan)
- [x] `deploy.yml` labeled as trailing shadow
- [x] Certification manifest emits primalSpring-expected fields
- [x] WAN relay validated (flockGate → Forgejo → peptidoglycan → VPS)
- [ ] DNS NS cutover (eastGate manual action, blocked on S1 TLS graduation)
- [ ] Post-cutover: archive `deploy.yml`, verify HTTPS via Caddy
- [x] NestGate CAS integration: cas-manifest (Wave 73) + cas-push (Wave 74)
- [ ] CAS route registration (path→hash mapping for NestGate HTTP serving)
