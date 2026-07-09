# sporePrint Changelog

All notable changes to this whitepaper repository are documented here.
Format: `[version] — date — description`

---

## [3.9.0] — 2026-07-09 — Thesis Scaffold + lithoSpore Product + Philosophy Subtabs (Wave 134c)

**Thesis section scaffolded, lithoSpore product page, philosophy sidebar grouped, cross-references linked.**

### Added

- **Thesis section** (`content/thesis/`): 16-chapter PhD dissertation scaffold mirroring
  gen3/thesis/ structure. Each chapter has frontmatter, abstract, maturity badge, and
  cross-links to existing sporePrint pages. Section index with thesis statement,
  six-part TOC, audience reading paths, and lineage tracing.
- **lithoSpore product page** (`content/products/lithoSpore.md`): spore taxonomy table
  (coldSpore→liveSpore→pseudoSpore→lithoSpore), three operating modes, three-tier
  validation, pseudoSpore lifecycle (emit/ingest/promote), deployment vision.
- **Philosophy sidebar subtabs**: atlasHugged essays grouped under The Stories (01–03),
  The Framework (04–08), The Synthesis (09–12), and Reference (bibliography).
- **Section page grouping**: philosophy section card listing also groups by subsection.
- **44 cross-references linked**: Inter-essay references (Document NN, Chapter N)
  converted to proper Zola internal links across 6 essays.

### Changed

- **Products `_index.md`**: lithoSpore entry expanded from one-line bullet to description
  with link to product page.
- **Sidebar tree**: thesis section added with Foundations/Theory/System/Validation/
  Analysis/Synthesis/Back Matter grouping.
- **Methodology `_index.md`**: added thesis section to reading table.
- **Philosophy `_index.md`**: added thesis cross-link in Discussion section.
- **pseudoSpore gallery `_index.md`**: added cross-link to lithoSpore product page.
- **Content pages**: 245 → 249 (18 thesis files + 1 product page).

---

## [3.8.0] — 2026-07-08 — Philosophy Complete + Content Enrichment (Wave 134b)

**All 12 atlasHugged essays live, bibliography page, cross-links, number sync.**

### Added

- **6 philosophy essays** transplanted: The City of Omelas (01), The Orthogonal
  Synthesis (02), The New City (03), The Loaves and the Fishes (05), The Many
  Rooms (09), The Love Letter (11). Philosophy section now has the complete
  narrative foundation beneath the existing conceptual essays.
- **Bibliography page**: `content/philosophy/bibliography.md` — full academic
  citations from CITATIONS.md (sacred texts, philosophy, science, literature).
- **Story ↔ Philosophy cross-links**: "Read More" sections in all 3 story essays
  linking to paired philosophy essays.
- **Philosophy ↔ Story cross-links**: Story section index maps each builder
  narrative to its philosophical counterpart.

### Changed

- **Philosophy `_index.md`**: Complete 12-essay table with sequence numbering,
  reading order guidance, bibliography link.
- **Philosophy essay weights**: Updated to match atlasHugged numbering (1–12).
- **Landing page "Try It"**: Replaced `wetSpring` (broken cold-clone due to path
  deps) with `groundSpring` (works cold from fresh clone).
- **Number sync**: Story essays now use `{{ total_stat() }}` shortcodes for
  validation checks and primal counts. Hardcoded "12,510" → "20,695+", "14" → "15".
- **Content pages**: 238 → 245 (6 essays + 1 bibliography).

## [3.7.0] — 2026-07-08 — Phase 1+2 Idiomatic Rust + petalTongue Integration (Wave 134)

**Codebase evolution sprint — shared abstractions, function decomposition, build-time petalTongue wiring.**

### Added

- **Shared content walker** (`paths.rs`): `walk_markdown_files` and `walk_content_files`
  iterators eliminate 4 duplicate `WalkDir` patterns across content, links, provenance, certify.
- **IPC connect helper** (`ipc.rs`): `connect_uds` consolidates 3 duplicate UDS connection
  setups from nucleus and tower modules.
- **`DiagnosticCollector`** (`error.rs`): typed accumulator with `error()`, `warning()`,
  `promote_warnings()`, `into_result()` — bridge for gradual `Vec<Diagnostic>` migration.
- **`MaturityLevel` enum** (`model.rs`): 6 typed levels (Implemented, Reproduced, Certified,
  Architectural, Planned, Unaudited) with `css_class()`, `label()`, `from_str_loose()`.
  Build-time validation via `validate_maturity_levels` in `--check` mode.
- **`build-viz` subcommand**: scans content for `viz_embed` shortcodes, calls petalTongue
  IPC to generate SVGs at build time, writes to `static/viz/`. Graceful fallback when
  petalTongue is offline.
- **`scan_viz_embeds`** (`commands.rs`): regex scanner for viz_embed shortcode names.
- **12 new tests**: DiagnosticCollector (2), MaturityLevel roundtrip/css/display/unknown/
  case-insensitive (5), maturity validation (2), viz scanner (3).

### Changed

- **`#[must_use]`** added to ~15 pure functions across error, model, paths, ipc, discovery,
  content, links modules.
- **`Cow<str>` evolution**: `normalize_key` returns `Cow<'_, str>` (zero-alloc fast path);
  `systemd_socket_dir` returns `Cow<'static, str>`.
- **Function decomposition**: `commands::validate` split into `validate_registry` +
  `validate_content`; `http::request_raw` into `parse_url` + `read_response` +
  `HttpResponse`; `cas_push::push_single_file` into `encode_file_payload` + RPC send.
- **`tower.rs`**: `match` → `let...else` for clippy pedantic compliance.
- **`petaltongue.rs`**: module docs corrected (removed stale `content.render` reference).
- **Root docs refreshed**: README, CONTEXT, EVOLUTION_QUEUE, RUST_TOOLING_VISION updated
  to 272 tests, 239 pages, 6 shortcodes, Wave 134 metrics.

### Removed

- **`section_count.html`** shortcode — unused, deleted.
- **`gonzales_explorer.md`** dead code: ~550 lines of inline CSS + Plotly JS referencing
  nonexistent `static/gonzales/` removed. Scientific content preserved with petalTongue
  evolution note.

## [3.6.0] — 2026-07-08 — Content Transplant (atlasHugged + Story + Methodology)

**Wave 133d content transplant — fills the largest content gaps on primals.eco.**

### Added

- **Philosophy section**: 6 atlasHugged essays transplanted from whitePaper/gen3:
  The Human Search, The Temptation of Kingdoms, The Mobility Edge, Discovery Is
  Local, I Own Nothing, The Knowledge-Numeric. Philosophy section now has real
  content instead of "Coming" promises.
- **Story section**: New `content/story/` with 3 builder narrative essays:
  I Don't Know Rust, The Sovereign Lab, 70 Papers One Stack. Nav bar and sidebar
  updated with Story link.
- **Methodology depth**: Sharing the Pen (`content/methodology/sharing_the_pen.md`)
  transplanted from whitePaper/gen4/knome — K-NOME methodology sharing argument.

### Changed

- **Philosophy `_index.md`**: "Coming: atlasHugged Essays" replaced with linked
  table of 6 real essays.
- **Nav bar**: Story link added between Lab and divider.
- **Sidebar tree**: Story and Philosophy sections now expand to show child pages.
- **Integration test fix**: `cas_push_requires_manifest_or_generate` assertion
  updated to match lowercase error message from transport unification refactor.
- Content page count: 228 → 238.

### Removed

- "attsi" references stripped from all public-facing story essays per transplant
  boundary rules.

---

## [3.5.0] — 2026-07-07 — Transport Unification + Catalog Metric Evolution

**Unified transport resolution, centralized timeouts, deduplicated discovery,
catalog metric evolution.**

### Changed

- **Unified transport resolution**: `petalTongue` commands (`pt-render`, `pt-viz`)
  now honor `TRANSPORT_ENDPOINT` env var, matching NestGate's injection pattern.
  New `discovery::resolve_primal_endpoint()` replaces per-primal `discover_socket()`
  wrappers (CLI → TRANSPORT_ENDPOINT → socket discovery).
- **Centralized timeout constants**: `PROBE_TIMEOUT` (3s), `TRANSPORT_CONNECT_TIMEOUT`
  (15s), `TRANSPORT_IO_TIMEOUT` (30s) moved to `paths.rs` — eliminates 4 duplicated
  constant definitions across `nucleus.rs`, `tower.rs`, `cas_push.rs`, `http.rs`.
- **Catalog metric evolution**: Removed 11 hardcoded `**Tests**:` lines from
  PRIMAL_CATALOG.md — `entity_metrics` shortcode renders live registry values.
  Remaining narrative test counts use `entity_stat` shortcodes.
- **Import consolidation**: Mid-file `use` statements in `fetch.rs`, `refresh.rs`,
  `notebook.rs` moved to top of file (idiomatic Rust module layout).

### Removed

- `cas_push::discover_socket()` — replaced by `discovery::resolve_primal_endpoint()`
- `petaltongue::discover_socket()` — replaced by `discovery::resolve_primal_endpoint()`

### Metrics

- 260 tests (228 unit + 29 integration + 3 refresh) — up from 258
- 28 modules, 10,112 lines
- All files under 800 lines (max: commands.rs at 699)
- Zero clippy warnings (pedantic + nursery)
- Zero `unwrap()` / `expect()` in production code
- 227 content pages, 190 internal links verified

---

## [3.4.0] — 2026-07-07 — Content Enrichment: Forensic Consistency Sprint

**Evidence Snapshot, maturity badges, metric unification, claim calibration,
accessibility improvements, contact page evolution.**

### Added

- **Evidence Snapshot page** (`architecture/EVIDENCE_SNAPSHOT.md`): Single canonical
  source of truth for all ecosystem metrics, definitions, and measurement methodology.
  Every metric uses `total_stat` shortcodes — always current.
- **Maturity badge shortcode** (`maturity.html`): 6 levels — Implemented, Reproduced,
  Certified, Architecture-ready, Planned, Unaudited. Used across products and science.
- **Config aliases**: `total_primals`, `total_springs`, `content_pages` added to
  `[extra.totals]` — fixes broken `total_primals` shortcode references.
- **Historical snapshot banners**: All 12 March 2026 docs (audience/, technical/,
  methodology/) now carry blockquote banners linking to Evidence Snapshot with
  current metrics via shortcodes.

### Changed

- **Product pages calibrated**: helixVision, lattice QCD, blueFish, esotericWebb,
  initioChem — all now carry maturity badges and calibrated claims. helixVision
  description updated: "AlphaFold-quality" → target, with primitive-level parity
  demonstrated and full-pipeline benchmarks pending.
- **Metric unification**: Replaced hardcoded numbers with `total_stat` shortcodes
  in lab/_index, _index, science/_index, ECOSYSTEM_INVENTORY, SPRING_CATALOG,
  SOVEREIGN_PRIOR_ART_CATALOG, KNOME_EVOLUTION, glossary, contact, sitemap,
  architecture/_index.
- **Clone commands fixed**: reproduce.md now uses HTTPS URLs first
  (`https://github.com/...`) instead of SSH (`git@github.com:...`).
- **Contact page softened**: Organization section rewritten — institution-friendly
  while maintaining professional boundaries. Metrics now via shortcodes.
- **Sitemap tree collapsed**: `<details open>` → `<details>` — content appears
  first for screen readers, SEO, and text-mode readers.
- **OpenGraph metadata**: `og:site_name` updated from "sporePrint — ecoPrimals"
  to "ecoPrimals — Sovereign Scientific Computing" for better discoverability.
- **Spring/domain count fixes**: "8 scientific domains" → "6 research domains"
  in science/_index description (matches actual section count).
  "7 springs" → "8 springs" in COMPOSITION_PIPELINE, CROSS_SPRING_EVIDENCE_MAP.

### Fixed

- Broken `total_primals` shortcode: config had `primal_count` but content used
  `total_primals` — added alias.
- 12+ pages with conflicting LOC counts (3.2M vs 3.46M), test counts (27K vs
  107K vs 113K), validation checks (12K vs 15K vs 20K), WGSL counts (628 vs 806
  vs 952), and spring counts (7 vs 8) — all now either unified via shortcodes or
  marked as historical snapshots.

---

## [3.3.0] — 2026-07-06 — Deep Debt Sprint + Sovereign AAR

**Smart refactoring, profile-driven probes, env-overridable socket dirs,
mock isolation, path dedup, content pages, static SVG diagrams, sovereign
deployment AAR.**

### Added

- **`nucleus_display.rs`**: Display/reporting extracted from `nucleus.rs` —
  `print_result`, `format_probe_info`, `count_by_contract`, etc. (385L new module)
- **Profile-driven Tower probes**: `PrimalEntry.probe_methods` in NUCLEUS TOML
  profiles overrides the default hardcoded probe table. `probe_tower_status()`
  now accepts `Option<&NucleusProfile>`.
- **`BIOMEOS_SYSTEMD_SOCKET_DIR` env override**: `discovery::systemd_socket_dir()`
  checks env before falling back to `/run/membrane`.
- **Content pages**: contact, living systems, sovereign CI, compute-access rewrite.
- **Static SVG diagrams**: `gate-mesh.svg`, `ci-pipeline.svg`.
- **`viz_embed` shortcode**: static SVG fallback rendering (graceful degradation).
- **4 new unit tests**: `build_probe_targets` (3), `systemd_socket_dir` (1).

### Changed

- **`nucleus.rs`**: 930L → 565L (display concern extracted).
- **`tower.rs`**: `TOWER_PROBES` → `DEFAULT_TOWER_PROBES` (fallback only).
- **`fetch.rs`**: `MockBackend` moved into `mod tests` (was module-scope `#[cfg(test)]`).
- **`main.rs`**: `"static/graph/entity-graph.json"` → `paths::ENTITY_GRAPH_JSON`.
- **`certify.rs`**: `"content"` → `crate::paths::CONTENT_DIR`.

### Sovereign AAR

- **5 divergences identified** in `SPOREPRINT_SOVEREIGN_DEPLOY_AAR_133a.md`:
  GitHub Pages primary, dual-push, no NUCLEUS on VPS, cascade doesn't rebuild
  Zola, deploy.yml still load-bearing.
- **VPS NUCLEUS deployment handoff** filed for golgi (4 binaries, 70MB, fits 10GB VPS).

### Metrics

- **258 total tests** (226 unit + 29 integration + 3 refresh_write)
- **28 modules**, 10,109 lines — all under 800L
- **226 content pages**

---

## [3.2.1] — 2026-07-05 — Coverage Sprint

**34 new tests across 5 modules — main, nucleus, commands, petaltongue, content.
All modules now have unit tests. Mock-stream IPC testing for petalTongue client.**

### Added

- **`main.rs` unit tests (7)**: `discover_springs_root` (3 paths), `load_entity_graph_for_render`
  (valid/missing/invalid JSON), `dispatch_standalone` (None paths).
- **`nucleus.rs` format tests (9)**: `format_probe_info` (full/partial/none),
  `format_probe_error` (message/no-health-method/none), `count_by_contract`,
  `has_ribo_result`, `ValidationResult::passed`.
- **`commands.rs` drift tests (5)**: `drift_pct` (positive/negative/zero/new/large).
- **`petaltongue.rs` mock-stream IPC tests (7)**: `health_check`, `render_graph`
  (success + error), `viz` (success + error), `probe_method` (success/method-not-found/other-error).
  Added `from_stream` test constructor.
- **`content.rs` check_integrity tests (4)**: valid shortcodes, unknown entity detection,
  name normalization, `entity_metrics` shortcode parsing.

### Fixed

- **`ipc.rs` / `petaltongue.rs`**: `MockStream` constructors use `Self` (pedantic
  clippy `use_self` lint).

### Metrics

- **254 total tests** (222 unit + 29 integration + 3 refresh_write)
- All modules now have `#[cfg(test)]` — 0 untested modules

---

## [3.2.0] — 2026-07-04 — Living Topology + Dep Evolution

**Mesh topology content page, dependency evolution (toml 1.x / TOML spec 1.1),
version alignment, and stale doc reconciliation.**

### Added

- **`content/architecture/MESH_TOPOLOGY.md`**: Living topology page with
  `viz_embed(src="/viz/gate-mesh?live=true")` — documents gate mesh architecture,
  capability routing, enrollment flow, and live health color mapping. Prepared
  for petalTongue `LiveMeshState` wire-up (Work Item 4 of living topology handoff).

### Changed

- **`toml` 0.8 → 1.x**: TOML spec 1.1 support. Zero breaking changes for our usage.
- **Crate version**: 0.3.0 → 0.3.1 (aligns with IPC consolidation release).
- **`specs/CONTEXT.md`**: Wave 128 → Wave 132d, version reference corrected.
- **`wateringHole/sporePrint/CONTENT_GUIDE.md`**: Replaced stale
  `scripts/render_notebooks.sh` reference with `cargo run -- render-notebooks`
  (script retired Wave 69).

---

## [3.1.0] — 2026-07-04 — Deep Debt Resolution + IPC Consolidation

**Deep debt cleanup and evolution across all 27 modules. Zero-copy idioms,
shared IPC module, static capability declarations, supply chain security,
and coverage boost. All quality gates pass. Zero mocks in production.**

### Architecture

- **Shared `ipc.rs` module**: Centralizes NDJSON JSON-RPC 2.0 client logic
  (previously duplicated across `cas_push`, `nucleus`, `petaltongue`, `tower`).
  Enforces response ID correlation (JSON-RPC §5) and `health.liveness` fallback
  for legacy primals.
- **Static capability declarations**: `discovery.rs` capabilities evolved from
  `Vec<String>` heap allocations to `&'static [&'static str]` slices compiled
  into `.rodata`. Zero runtime allocation for self-knowledge.
- **Zero-copy HTTP**: Body extraction via `Vec::split_off()` instead of `.to_vec()`.
  Scoped raw buffer drops in CAS push before JSON encoding.
- **`OnceLock` for env reads**: Forge URL cached for process lifetime.

### Added

- `deny.toml` — `cargo-deny` supply chain security (all deps pure Rust, no
  advisories, SPDX-compliant licenses, crates.io-only sources)
- `#![warn(missing_docs)]` — documentation lint active (guards future lib extraction)
- 14 new tests: mock-stream IPC roundtrip, certify emit/validate, refresh
  write_updates, count_metrics isolation, stored manifest deserialization
- SPDX license headers on all 15 templates + 7 SCSS files

### Changed

- `tower.rs`: inline JSON-RPC → `ipc::send_rpc` + `ipc::is_method_not_found`
- `nucleus.rs`: `probe_socket_health` → `ipc::probe_health` with `health.liveness`
- `petaltongue.rs`: inline `send_rpc` → `ipc::send_rpc` delegation
- `cas_push.rs`: scoped `contents` drop before JSON payload construction
- `fetch.rs`: `default_forge_url() -> String` → `-> &'static str` via `OnceLock`
- `http.rs`: header parsing before `split_off` to avoid borrow conflicts
- Integration test isolation: notebook rendering outputs to tempdir

### Removed

- `scripts/refresh-metrics.sh` — retired Wave 69, fossil record in git history

### Metrics

- Tests: 206 → **220** (188 unit + 29 integration + 3 refresh_write)
- Coverage: 60.77% → **64.87%** (ipc: 97%, certify: 71%, refresh: 78%)
- `cargo deny check` — clean (zero advisories, zero yanked)
- Zero TODO/FIXME/HACK in source
- Zero hardcoded IPs/ports in production code
- All `#[allow()]` justified (16 total)
- All files under 800 lines

---

## [3.0.0] — 2026-06-01 — Sovereign Self-Hosting + Provenance Data System

**sporePrint is now sovereign-primary. VPS serves the site via Caddy + Let's Encrypt.
GitHub Pages becomes the trailing extracellular shadow. BLAKE3 content addressing
enables provenance trio integration.**

### Sovereign Deployment
- VPS auto-rebuild pipeline: Forgejo push → relay chain → `sporeprint-rebuild.sh`
- systemd timer (15-min fallback) on golgiBody-ext
- Caddy TLS config with domain routing + Let's Encrypt CAA
- Sovereign DNS A records for primals.eco → golgiBody-ext
- GitHub Pages marked as trailing shadow in deploy.yml

### Provenance
- `provenance` subcommand: BLAKE3 content addressing for all 218 pages
- `content-manifest.toml`: deterministic root hash, per-page hashes + titles
- `--verify`: validate current content against manifest (integrity check)
- `--diff`: show new/changed/removed pages since last manifest
- `--write`: persist manifest for version-controlled provenance tracking

### Config
- `deploy_locations` now lists `golgiBody-ext` as primary
- `shadow_status` → `sovereign-primary`
- `sovereign_url` and `sovereign_rebuild` fields added

---

## [2.0.0] — 2026-05-31 — Deep Debt Resolution + Sovereign Evolution

**Complete code quality overhaul. trait-based architecture, 90%+ coverage,
zero dead code, capability-based discovery, shared utilities, and new
subcommands. sporePrint is now spring-grade quality.**

### Architecture

- **Trait-based VCS**: `VcsBackend` trait with `GitBackend` (production) and
  `MockBackend` (testing) — enables full test coverage without network I/O
- **Shared `time.rs`**: Pure Rust UTC date utility, deduplicated from 3 modules
- **`report.rs`**: Entity registry summarization — consumes all model fields,
  eliminates crate-level `dead_code` lint allowance
- **`links.rs`**: Internal link validation (absorbs external tools)
- **`error.rs`**: `thiserror`-based typed error hierarchy with `Diagnostic` enum
- **CSS semantic split**: monolith → `base.css` (tokens) + `main.css` (components)
- **Capability-based discovery**: `.gate` workspace walk, configurable origins

### Added

- `check-links` subcommand — validates 149 internal `@/` links across 207 files
- `render-notebooks --discover` — auto-discovers notebooks from ecosystem workspace
- `validate --verbose` — full entity report with all fields + totals display
- Private repo gating via `SPOREPRINT_REFRESH_PAT` environment variable
- `FetchOutcome` enum with structured results (replaces string messages)
- SPDX license headers on all Rust source files
- `static/css/base.css` — design system tokens extracted from monolith
- `static/gonzales/js/config.js` — capability-based API endpoint discovery

### Changed

- **Error handling**: `process::exit` → `thiserror` + `Result` propagation + `ExitCode`
- **Crate root**: `#![forbid(unsafe_code)]` enforced
- **Linting**: zero warnings for clippy pedantic + nursery (no `#[allow()]` in production)
- **`dead_code`**: removed crate-level allowance; all fields now consumed via `report.rs`
- **`fetch.rs`**: trait-based with `Source.clone_url()`, `Source.kind`, private filtering
- **`explorer.js`**: 1097L → 533L (config extracted to `config.js` at 140L)
- **`render_notebooks.sh`**: hardcoded paths → `.gate` file discovery
- All external `date` command calls replaced with pure Rust `time::today_utc()`
- `Diagnostic::message()` used in production output (not just tests)

### Removed

- 1,162 tracked build artifacts (`crates/spore-validate/target/`) — never should
  have been committed. `.gitignore` now catches `target/` at any depth.
- Crate-level `[lints.rust] dead_code = "allow"` — all fields truly consumed
- Duplicate date computation (3 copies → 1 in `time.rs`)
- TOML escape error in notebook front matter that broke Zola build

### Metrics

- Test coverage: 32.6% → **90.3%** (llvm-cov)
- Tests: 11 → **80** (50 unit + 12 integration + 3 refresh-write + 15 link/time)
- Modules: 6 → **12** (+ error, fetch rewrite, links, report, time, notebook rewrite)
- Max file size: 466L (all well under 1000L limit)
- Release build: 5.56s clean (lean deps, pure Rust)
- Binary size: 4.4MB (12 modules, 7 deps)
- `zola build`: 736ms, zero errors

---

## [1.1.0] — 2026-04-30 — Fully Rust Toolchain

**The Python validation script is replaced by `spore-validate`, a typed Rust
binary. The sporePrint pipeline is now 100% Rust — from validation to
generation to deployment.**

### Added

- **`crates/spore-validate/`** — Rust crate replacing `scripts/validate_registry.py`
  - Typed entity model: `EntityKind` enum (7 kinds), `Tier` enum (4 tiers),
    `Entity` struct with per-kind field validation
  - `validate` subcommand (default): registry field checks, totals verification,
    content taxonomy cross-references — full parity with Python script
  - `--check` flag: scans 2,488 entity shortcodes in prose, validates all
    resolve to registry keys
  - `--strict` flag: promotes warnings to errors
  - `refresh` subcommand: cross-repo metric comparison — discovers repos in
    `primals/`, `springs/`, `infra/` directories, counts Rust LOC, tests,
    files, and crates, reports drift with percentage change
  - 11 unit tests covering model deserialization, validation logic, totals
    verification, front matter extraction, and line counting
- **`content/philosophy/_index.md`** — atlasHugged integration stub explaining
  the "why" of ecoPrimals (AGPL-3.0, attribution, sovereignty)
- **Science pages 26–27**: neuromorphic sovereign driver (rustChip), nature
  preserve applied NPU science

### Changed

- **CI pipeline** (`deploy.yml`): `python3 scripts/validate_registry.py` →
  `cargo build --release` + `spore-validate validate --check` with cargo cache
- **rustChip entity** in `config.toml`: updated to 23,733 LOC, 367 tests,
  118 files; description includes glowplug, science demos, HW/SW separation
- **Landing page**: "25 baseCamp Papers" → "27 baseCamp Papers"
- **Specs updated**: `CONTEXT.md`, `EVOLUTION_QUEUE.md`, `CONTENT_MAP.md`,
  `RUST_TOOLING_VISION.md` all reflect the implemented tooling

### Metrics

- Release binary: 94ms runtime (vs 146ms Python)
- 60 entities validated, 2,488 shortcodes scanned
- 24 repos scannable via `refresh`, 87 metric drifts detected across ecosystem
- 11 Rust tests, 0 Python dependency

---

## [1.0.0] — 2026-04-03 — Zola: Rust to the Very Edge

**The site becomes sovereign infrastructure. Zola (Rust static site
generator) replaces raw GitHub Markdown rendering. Full content refresh.
guideStone section. Missing papers added. Search enabled.**

### Architecture

- **Zola 0.22.1** — single Rust binary, zero runtime dependencies
- Custom theme: dark mode (prefers-color-scheme), responsive sidebar,
  accessible (skip links, ARIA, focus outlines, semantic HTML)
- Built-in search via Elasticlunr (no external JS frameworks)
- GitHub Actions CI/CD: build with Zola, deploy to GitHub Pages
- CNAME moved to `static/CNAME` for Zola's build pipeline

### Content Migration

- All Markdown files migrated from flat directories to `content/` with
  TOML front matter (title, description, date, extras)
- Section indexes (`_index.md`) for audience, science, architecture,
  methodology, technical, guidestone
- Landing page (`content/_index.md`) replaces root README as site content
- New repo README for developer documentation

### New Content

- **`content/guidestone/_index.md`** — The verification class: five properties,
  first deployment artifact (hotSpring-guideStone-v0.7.0), self-leveling
  benchmark, onboarding pattern, cross-substrate validation (5 substrates,
  40/40 bit-identical), metrological analogy
- **Missing science papers added:**
  - Paper 14: Sovereign Compute Hardware (131+ experiments, deep debt evolution)
  - Paper 23: Mass-Energy-Information Equivalence (unifying hypothesis)
  - Paper 24: All-Silicon Science (sovereign GPU pipeline, both vendors)
  - Paper 24b: Esoteric Webb Composition Patterns (gen4 creative infrastructure)
  - Paper 25: Self-Tuning Simulation (physics-validated parameter discovery)

### Content Refresh

- Landing page updated: guideStone verification path, pre-built artifact
  verification option (no Rust required), gen4 references, updated ecosystem
  diagram with guideStone layer, physicist/collaborator audience path
- "Last Updated" date bumped to April 3, 2026
- guideStone callout in ecosystem glance section
- Foundation papers table updated with guideStone certification references

### Design Principles

- **Rust to the edge**: Zola generates the site. No Ruby, no Node, no Python.
- **Markdown-first**: All content stays in `.md` with TOML front matter.
  Agents and humans read and edit identically.
- **Self-contained**: No CDN, no Google Fonts, no analytics scripts.
- **Accessible**: Semantic HTML, ARIA landmarks, skip links, keyboard nav.
- **No external theme dependency**: Custom templates and CSS.

### Document count at v1.0.0: 49 pages, 6 sections

---

## [0.1.0] — 2026-03-17 — Initial Scaffold

**First spore print.**

### Added

**Audience docs (from publicRelease/):**
- `audience/FOR_FACULTY_AND_PIS.md` — what ecoPrimals replaces in a lab
- `audience/FOR_STUDENTS_AND_CORE_FACILITIES.md` — setup guide, 16S walkthrough
- `audience/FOR_HARDWARE_BUILDERS_AND_HOBBYISTS.md` — f64 Vulkan discovery, Games@Home
- `audience/FOR_COMPLIANCE_AND_INSTITUTIONAL_REVIEW.md` — FDA/ISO/HIPAA/GDPR mapping
- `audience/CAPABILITY_PARITY_BRIEF.md` — domain-by-domain comparison vs proprietary tools

**Technical docs (from publicRelease/):**
- `technical/DRUG_DISCOVERY_PIPELINE.md` — Anderson-augmented MATRIX, 329/329 checks
- `technical/MSU_ASSET_ACCELERATION.md` — Genomics Core, ICER, ADDRC, MSDS integration
- `technical/GRANT_TECHNICAL_APPENDIX.md` — NIH/NSF/USDA/DOE validation evidence
- `technical/KNOME_TEACHING_BRIEF.md` — K-Nome as pedagogy for real science

**Methodology (from whitePaper/gen3/):**
- `methodology/CONSTRAINED_EVOLUTION_FORMAL.md` — the core methodology paper
- `methodology/K_NOME_PROGRAMMING.md` — K-Nome operational framework
- `methodology/P_NP_ENZYME_THESIS.md` — P≠NP enzyme argument

**Architecture (from whitePaper/gen3/):**
- `architecture/ECOSYSTEM_ARCHITECTURE.md` — UniBin/ecoBin/genomeBin, NUCLEUS, Neural API
- `architecture/PRIMAL_CATALOG.md` — all 14 primals with capabilities and test counts
- `architecture/SPRING_CATALOG.md` — all 7 springs with checks, papers, cross-spring flow
- `architecture/SOVEREIGN_PRIOR_ART_CATALOG.md` — lysogeny prior art record

**Science (from whitePaper/gen3/baseCamp/):**
- `science/README.md` — baseCamp index with reading order by discipline
- 21 baseCamp papers (Papers 01–22, all available)

**Root:**
- `README.md` — whitepaper index, 4-audience guide, 5-minute verification
- `LICENSE` — CC-BY-SA 4.0 (docs) + AGPL-3.0 (code)
- `CHANGELOG.md` — this file

**Spring checks at scaffold time:**

| Spring | Checks |
|--------|:------:|
| wetSpring | 5,707+ |
| airSpring | 3,123+ |
| neuralSpring | 4,500+ |
| hotSpring | 664+ |
| groundSpring | 535+ |
| healthSpring | 474+ |
| ludoSpring | 1,692+ |
| **Total** | **16,695+** |

---

## [0.2.0] — 2026-03-17 — Data, analysis, and polish

### Added

- `technical/HARDWARE_COST_ANALYSIS.md` — The f64 Vulkan discovery ($0.044/run,
  9.9× DF64 uplift, $15K basement vs ICER/cloud cost analysis, NUCLEUS scaling
  math, GPU DF64 TFLOPS table for consumer cards)
- `architecture/EVOLUTION_TIMELINE.md` — Day-by-day 27-day sprint record.
  Velocity analysis (checks/day per sprint), what the timeline proves about
  K-Nome methodology, the infrastructure that made velocity possible
- `science/CROSS_SPRING_EVIDENCE_MAP.md` — The Anderson thread across 5 domains,
  paper-by-paper cross-spring dependency tables, convergent predictions where
  independent springs agree on the same number, the groundSpring anomaly,
  open questions awaiting wet-lab validation

### Updated

- `README.md` — Added "The Five Numbers" table ($0.044, 9.9×, 27 days, 20,695+,
  175+), new Hardware/Cost reading path, references to 3 new docs, improved
  document map with annotations

### Document count at v0.2.0: 47 files

---

## [0.3.0] — 2026-03-17 — Spring profiling, sovereign pipeline, knowledge commons

### Added

- `technical/SOVEREIGN_GPU_PIPELINE_PROFILE.md` — Complete vendor replacement
  story: what's already replaced (CUDA runtime, cuBLAS, nvcc, nvidia.ko →
  toadStool, BarraCuda, coralReef, coral-glowplug), current performance vs
  Kokkos (27×→3.7× gap closure), 3/6/12-month roadmap, proprietary cost table,
  full sovereign stack diagram. 27,169+ combined tests across 4 primals.
- `science/STRUCTURE_PREDICTION_ROADMAP.md` — coralForge: sovereign AlphaFold-
  quality structure prediction. The isomorphism proof (6 universal primitives),
  current status (154/154 checks), performance targets (~3 min/sequence on
  consumer GPU), LTEE at scale ($1K vs $83K cloud), drug docking and enzyme
  engineering extensions.
- `methodology/KNOWLEDGE_COMMONS_TARGETS.md` — What others can pick up with
  existing primals: 9 Tier 1 domains ready now (antibiotic resistance,
  wastewater surveillance, marine ecology, veterinary PK/PD, climate crops,
  materials science, educational games, fermentation, environmental tox).
  The three-lock guarantee (AGPL + ORC + CC-BY-SA). Velocity projection
  (75,000+ checks by March 2027). Why it can't be taken back.

### Updated

- `README.md` — Added sovereign GPU pipeline reading path, structure prediction
  in science section, knowledge commons in "what others can build" section,
  document map annotations for all new files.

### Document count at v0.3.0: 50 files

---

## [0.4.0] — 2026-03-17 — How to start a spring: the operational playbook

### Added

- `methodology/HOW_TO_START_A_SPRING.md` — The complete operational playbook
  for anyone starting their own spring. Carries the core K-Nome insight: LLMs
  work because of the data, the data is human language, every human is already
  an expert practitioner. You don't need to know how to code — you need to know
  how to talk. Includes: Phase 0→1→2→3+ protocol, the conversation patterns
  (analogy, correction, narrative, taste, redirection), a concrete worked
  example (fermentation spring, week by week), cost model ($150 GPU +
  electricity), honest constraints, and getting-started commands. Links to
  KNOWLEDGE_COMMONS_TARGETS.md for what domains are ready now.

### Updated

- `README.md` — Added HOW_TO_START_A_SPRING to methodology reading path,
  document map, and "What Others Can Build" section.

### Document count at v0.4.0: 51 files

---

## Roadmap

### [2.1.0] — pseudoSpore Gallery + Sovereign Deploy
- pseudoSpore gallery pages (`/lab/spores/{name}/`) with lithoSpore registry
- DNS cutover: primals.eco → golgiBody-ext (137.184.197.151)
- peptidoglycan build pipeline → golgiBody-ext Caddy
- GitHub Pages becomes extracellular shadow

### Future — petalTongue Integration
- gonzales JS files (JELLY STRING) absorbed by server-rendered SVG + WASM
- Conversational navigation of site content via petalTongue
- Audio narration from Markdown source

### Future — projectFOUNDATION Ingestion
- Replace GitHub Actions dispatch with Foundation-driven content publishing
- Temporal sync-driven rebuilds on flockGate (WAN shadow)
