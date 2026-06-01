# sporePrint Evolution Queue

Planned changes, ordered by priority. When implemented, move to CHANGELOG.md.

Last reviewed: June 1, 2026 (Wave 68 — Deep Debt + Viz Modularization)

---

## P0 — Next Session

### Periodic refresh: counts and versions
- [x] ~~Sync landing page stat cards~~ — stats ribbon reads from `config.extra.totals` dynamically (no hardcoded numbers to update)
- [ ] Update Squirrel version/tests if alpha has advanced
- [ ] Verify plasmidBin inventory count still accurate
- [x] ~~Check if new baseCamp papers exist~~ — papers 26 (neuromorphic driver) and 27 (nature preserve) added. Paper count updated to 27 in science index and landing page
- [ ] Verify LOC estimates in PRIMAL_CATALOG.md — run tokei on each primal repo for ground truth
- [x] ~~rustChip entity registry refresh~~ — updated to 23,733 LOC, 367 tests, 118 files; added glowplug, science demos, HW/SW separation; `measured_date` set to 2026-04-30

### Content gaps
- [x] ~~guideStone section has only `_index.md`~~ — now has 4 substantive pages: verification_protocol, deployment_artifacts, cross_substrate_validation, live_spore_feed
- [ ] guidePost (planned, paired with guideStone in wateringHole glossary) — document when it materializes
- [x] ~~Some science pages have ungrouped domains~~ — verified: only CROSS_SPRING_EVIDENCE_MAP and STRUCTURE_PREDICTION_ROADMAP lack domains (reference docs, not papers, intentionally ungrouped)
- [ ] atlasHugged integration — `content/philosophy/` section stub exists. When essays are ready for public release, add them as individual pages. This is a separate, intentional act — do not auto-publish from whitePaper

### Taxonomy completeness
- [ ] Audit all 207 content pages: grep content for entity names not tagged in front matter
- [x] ~~Build-time validation script~~ → replaced by `spore-validate` Rust crate (`crates/spore-validate/`)
- [x] ~~Internal link validation~~ → `spore-validate check-links` (149 links validated)

---

## P1 — Near-Term

### Content enrichment
- [ ] Add a "Getting Started with plasmidBin" walkthrough (clone → fetch → deploy)
- [ ] Expand products pages with composition diagrams and BYOB examples
- [ ] Add cross-spring data flow diagram to SPRING_CATALOG.md or a dedicated page

### Visual evolution
- [x] ~~Add a full site map page (`/sitemap/`)~~ — sitemap/_index.md now lists all sections with page counts
- [ ] Consider adding entity taxonomy counts to the site tree sidebar
- [ ] Mobile experience: test site tree usability on phones, consider defaulting `<details>` to closed
- [ ] Investigate whether the per-page TOC (right sidebar) and site tree (left sidebar) coexist well on medium screens

### Semantic validation (the "grammar compiler")
- [x] ~~Pre-build validation script~~ → `spore-validate validate` (Rust typed replacement)
- [x] ~~Entity metrics in one place, referenced everywhere~~ → `config.toml` registry + shortcodes
- [x] ~~Entity names in prose match taxonomy tags~~ → `spore-validate validate --check` (scans 2,488 shortcodes)
- [ ] Entity registry entries match wateringHole PRIMAL_EMOJI_STANDARD (cross-repo check)
- [x] ~~Metrics in registry haven't drifted from source repos~~ → `spore-validate refresh <repos_root>`

### petalTongue integration
- [x] ~~When petalTongue can consume sporePrint content, add documentation on the modality pipeline~~ — Wave 67: DocumentNode types, content_render.rs, document_compiler.rs implemented
- [x] ~~Reference SPOREPRINT_CONTENT_DELIVERY_SPECIFICATION.md from petalTongue specs/~~ — content pipeline is now native to petalTongue (content_render module)

---

## P2 — Future

### Accessibility
- [ ] Audit site for WCAG 2.1 AA compliance
- [ ] Test with screen readers (VoiceOver, NVDA)
- [ ] Verify Catppuccin color contrast ratios (4.5:1 minimum for text)
- [ ] Ensure all emoji pairs have `aria-label` alternatives in templates

### Automation
- [x] ~~Script to pull check counts from spring repos and update stats~~ → `spore-validate refresh` compares LOC, tests, files, crates against registry
- [ ] Script to diff whitePaper/baseCamp against content/science/ for new papers
- [x] ~~Consider lychee CI for non-Zola link checking~~ → `spore-validate check-links` covers internal links; external link validation TBD

### pseudoSpore Gallery (Wave 64 target)
- [ ] Zola template for `/lab/spores/{name}/` gallery pages
- [ ] `spore-validate` reads lithoSpore `registry.toml` → generates gallery markdown
- [ ] Gallery index at `/lab/spores/` listing all available pseudoSpores
- [ ] "Download lithoSpore" link per gallery page

### Sovereign Deployment
- [ ] DNS cutover: primals.eco A record → golgiBody-ext (137.184.197.151)
- [ ] peptidoglycan build pipeline: `zola build` → rsync → golgiBody-ext → Caddy
- [ ] GitHub Pages becomes extracellular shadow (backup only)

### Search
- [ ] Evaluate elasticlunr search quality for the current 205 pages
- [ ] Consider whether taxonomy pages should be included in the search index
- [ ] Evaluate faceted search (filter by primal/spring) if page count grows significantly

---

## Resolved (April 2026)

These were in the original queue and have been completed:

- [x] Taxonomies for primals and springs (build-validated typed tags)
- [x] Entity registry in config.toml (single source of truth for display names and emojis)
- [x] Entity shortcode (`{{ entity(name="beardog") }}`)
- [x] Site tree sidebar navigation with current-page highlighting
- [x] Card-based landing page (replaced all markdown tables)
- [x] Streamlined nav bar (10 items → 7)
- [x] coralReef and barraCuda sections in PRIMAL_CATALOG (§1.7, §1.8)
- [x] bingoCube mentioned in catalog (§3.1 tooling section)
- [x] NUCLEUS_ARCHITECTURE.md — composition model documentation
- [x] sporeGarden product pages (esotericWebb, helixVision, blueFish)
- [x] Dark Forest, Neural API, RootPulse documented in NUCLEUS_ARCHITECTURE
- [x] Faculty references reframed as "Reproduces work by" / "Literature Anchor"
- [x] Grade metrics replaced with LOC, test counts, coverage
- [x] ludoSpring stats updated (V30, 75 experiments, 1,692 checks)
- [x] Narrative rewrite of all primal catalog sections (identity + origin + constraint)
- [x] Narrative rewrite of all spring catalog sections (identity + headline results + constraint)
- [x] Entity registry expanded with LOC, tests, files, crates, domain, tier (measured via tokei)
- [x] Aggregate totals in `[extra.totals]` with sum validation
- [x] 4 shortcodes: `entity`, `entity_metrics`, `entity_stat`, `total_stat`
- [x] Landing page stats ribbon reads from `config.extra.totals`
- [x] All 23 catalog metrics lines replaced with `entity_metrics` shortcode calls
- [x] Pre-build validation script (`scripts/validate_registry.py`) in CI
- [x] Real LOC numbers: 3.2M Rust, 107K tests, 952 WGSL (old estimates were 220K)

## Wave 64 Targets

- [x] pseudoSpore gallery template (`/lab/spores/{name}/`)
- [x] Zola build pipeline validated on WAN (226 pages, 746ms)
- [x] Gate bootstrap AAR completed
- [x] Temporal sync sustained measurement (6 pushes, relay gap identified)
- [ ] Forgejo relay hook for sporePrint (eastGate action)
- [ ] peptidoglycan-triggered rebuild pipeline
- [ ] DNS cutover: primals.eco → golgiBody-ext (Wave 65)

## Wave 66 — Knowledge Topology (Completed June 1, 2026)

### Renvois de Choses — Typed Entity Graph
- [x] `Edge` struct + `EdgeRelation` enum in `model.rs` (14 relation types)
- [x] `edges` field on Entity (optional, backward compatible)
- [x] 63 edges populated across all 15 primals in `config.toml`
- [x] `graph.rs` module — validates targets, computes inverses, emits JSON
- [x] `graph` CLI subcommand (`spore-validate graph --emit`)
- [x] `static/graph/entity-graph.json` — 66 nodes, 126 edges
- [x] Connections template on taxonomy pages (outbound + inbound edges)
- [x] Architecture page: `/architecture/renvois-knowledge-topology/`
- [x] `specs/KNOWLEDGE_TOPOLOGY.md` — intellectual lineage and design constraints

### Deep Debt Resolution
- [x] `Diagnostic` refactored from enum variants to proper struct + severity
- [x] All hardcoded directory names replaced with runtime discovery
- [x] Forge URL configurable via `SPOREPRINT_FORGE_URL` env var
- [x] Notebook language detected from metadata (not hardcoded "python")
- [x] Taxonomy names derived from `EntityKind::taxonomy_pairs()` type system
- [x] Zero clippy warnings (pedantic + nursery)
- [x] 86 tests passing
- [x] All deps justified (7 runtime + 1 dev-dep, all pure Rust)

### guideStone Self-Certification (Completed June 1, 2026)
- [x] `certify.rs` module — BLAKE3 Merkle root of sorted entity graph
- [x] `CertificationManifest` struct with all verifiable claims
- [x] `certify` CLI subcommand (`--emit` writes, default validates existing)
- [x] `static/certification/manifest.json` generated
- [x] Certification badge in base.html footer (Verify link)
- [x] Architecture page: `/architecture/guidestone-publication/`
- [x] CI integration: certify --emit before zola build in deploy.yml
- [x] 89 tests passing (3 new certify tests)
- [x] `blake3` dependency added (pure Rust, aligns with BearDog)
- [x] `render_notebooks.sh` removed (vestigial JELLY STRING, fully absorbed by Rust)

### Docs + Housekeeping (June 1, 2026)
- [x] README.md updated (66 entities, 205 pages, 89 tests, certify docs)
- [x] RUST_TOOLING_VISION.md updated (14 modules, graph + certify documented)
- [x] TAXONOMY_STANDARD.md entity count updated (66)
- [x] CONTENT_MAP.md counts refreshed (205 pages, 18 architecture, 128 lab)
- [x] CONTEXT.md updated (certify subcommand, 14 modules, 89 tests)
- [x] cargo clean (925 MiB freed)
- [x] public/ build dir cleaned

## Wave 67 — Nest Atomic / Pure-Primal Evolution (June 1, 2026)

### petalTongue Content Scene Graph
- [x] `document.rs` in petal-tongue-scene — DocumentNode, PageMeta, Inline, EntityRef types
- [x] 4 unit tests (serialization, defaults, round-trip)
- [x] `toml` workspace dependency added to petal-tongue-scene

### Content Rendering Pipeline
- [x] `content_render.rs` in petalTongue binary — front-matter parser, markdown compiler, shortcode resolver
- [x] `split_front_matter()` — TOML `+++` delimiter handling
- [x] `parse_front_matter()` — TOML to PageMeta with taxonomies and extras
- [x] `compile_markdown()` — pulldown-cmark to DocumentNode tree (headings, code, lists, tables, blockquotes)
- [x] `resolve_shortcodes()` — `{{ entity(name="...") }}` expansion against registry
- [x] `parse_document()` — full pipeline entry point
- [x] 8 unit tests passing

### Document Modality Compilers
- [x] `document_compiler.rs` in petal-tongue-scene/modality — DocumentNode to ModalityOutput
- [x] `compile_to_html()` — full HTML page rendering (semantic markup, entity links, nav tree, tables)
- [x] `compile_to_description()` — accessible text for screen readers (indented, structured)
- [x] 5 unit tests passing

### Web Content Route
- [x] `content_fallback()` enhanced with Accept header negotiation
- [x] Markdown content detected and rendered through DocumentNode pipeline
- [x] Modality selection: `text/html` (visual), `text/plain` (description), `application/json` (metadata)
- [x] Query parameter override: `?modality=description` or `?modality=json`

### projectNUCLEUS Deploy Graph
- [x] `graphs/sporeprint_composition.toml` — Nest Atomic + petalTongue + spore-validate
- [x] Includes `nest_atomic` fragment (7 nodes: tower + nestgate + provenance trio)
- [x] petalTongue node: `web --backend content-provider --port 8080`
- [x] spore-validate node: `certify --emit` (run_once)
- [x] Deployment hints: Caddy reverse proxy, primals.eco domain

### primalSpring Validation
- [x] `s_sporeprint_pure_primal.rs` scenario (5 phases)
- [x] Phase 1: Content parsing (front-matter validation)
- [x] Phase 2: Entity resolution (registry coverage, shortcode resolution rate)
- [x] Phase 3: Modality output structure (heading + body feasibility)
- [x] Phase 4: Composition graph (nest_atomic, petaltongue, capabilities)
- [x] Phase 5: Certification manifest (deploy.yml, manifest.json, merkle_root)
- [x] Registered in build_registry() (58 scenarios total)

### Local Nest Validation (June 1, 2026)
- [x] `content_direct.rs` — filesystem backend (reads .md from disk, renders via DocumentNode)
- [x] `load_entity_registry()` — config.toml parser (66 entities loaded)
- [x] `build_nav_tree()` — content directory walker (11 sections discovered)
- [x] Wired into web_mode/mod.rs router (`--backend content-direct`)
- [x] Release binary serves sporePrint content on localhost:8080
- [x] Static file serving (Zola convention: /css/main.css from static/)
- [x] Multi-modal output: HTML, description (accessible text), JSON (scene graph)
- [x] `validate_parity.sh` — 5-phase parity check (22/22 pass vs Zola reference)
- [x] Entity shortcode resolution confirmed (no unresolved `{{ entity(...) }}`)
- [x] Accept header negotiation + `?modality=` query param override

## Wave 68 — Deep Debt Resolution + Live Visualizations (June 1, 2026)

### Live Ecosystem Visualizations
- [x] `viz_data/entity_graph.rs` — Force-directed layout (Fruchterman-Reingold) of 66 entities
- [x] `viz_data/kderm.rs` — 5-layer cross-section with VPS nodes + relay animation
- [x] `viz_data/nucleus.rs` — Nested composition layers with expand/collapse animation
- [x] `viz_data/mod.rs` — VizRegistry for capability-based discovery
- [x] `/viz/*` routes serving SVG, scene-JSON, description, animation-JSON
- [x] `{{ viz_embed(src="/viz/...") }}` shortcode expansion to inline SVG
- [x] `petal-tongue-wasm` deployed to `static/wasm/` (593KB)
- [x] `viz-hydrate.js` — progressive enhancement (pan/zoom/animate/tooltips)
- [x] Content pages updated: ECOSYSTEM_VISUALIZATION, KDERM, NUCLEUS

### Deep Debt + Overstep Cleanup
- [x] `LazyLock<Regex>` statics replace 3 production `unwrap()`/`expect()` (links.rs, content.rs)
- [x] 16 notebook files: `/home/eastgate/` → `ECOPRIMALS_ROOT` env var
- [x] `VizRegistry` decouples viz route handler from hardcoded slug matching
- [x] `viz_data.rs` (882L) → 4 modules (114 + 242 + 187 + 143 = 686L, max 242)
- [x] `tests/parity.rs` — 6 Rust integration tests (replaces shell script)
- [x] `refresh-metrics.sh` deprecated (evolution: `spore-validate refresh --fetch`)
- [x] `validate_parity.sh` deprecated (Rust: `cargo test --test parity -- --ignored`)
- [x] `gonzales/DEPRECATED.md` — migration timeline (data → CAS, charts → petalTongue)
- [x] `ureq` dev-dependency added for HTTP parity tests
