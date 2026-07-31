# sporePrint Evolution Queue

Planned changes, ordered by priority. When implemented, move to CHANGELOG.md.

Last reviewed: July 31, 2026 (Wave 155n — SITE RESTRUCTURED: conceptual → demonstration era)

**Current state**: 313 pages (23 sections), 79 entities, 4 cortical folds (Evidence, Architecture, Methodology, Getting Started), 3.60M LOC, 101,308 tests, 35 depot binaries, 3 NUCLEUS gates, ZERO P0/P1/P2.

**Wave 155n restructuring**: Top nav: Lab | Science | Architecture | Products | Get Started. 47 pages marked `foundation = true` (not in nav). Thesis/philosophy/story → Backstory section. Audience/outreach/collaborators → Foundation section. Architecture: 15 live pages stay, 29 design docs move to foundation. VALIDATED badges on 4 baseCamp papers. Homepage leads with "NUCLEUS Is Running".

Most items below pre-Wave 138 are completed — retained as fossil record.

---

## P0 — Next Session

### Periodic refresh: counts and versions
- [x] ~~Sync landing page stat cards~~ — stats ribbon reads from `config.extra.totals` dynamically
- [ ] Update Squirrel version/tests if alpha has advanced
- [x] ~~Verify plasmidBin inventory count~~ — 59 binaries across 4 architectures (16+16+13+14)
- [x] ~~Check if new baseCamp papers exist~~ — 28 papers confirmed
- [x] ~~Verify LOC in PRIMAL_CATALOG.md~~ — catalog uses `entity_metrics` shortcodes (live from registry); periodic refresh via `spore-validate refresh --write`
- [x] ~~rustChip entity registry refresh~~ — 23,655 LOC, 370 tests, 118 files

### Content gaps
- [x] ~~guideStone section has only `_index.md`~~ — 4 substantive pages
- [ ] guidePost — document when it materializes
- [x] ~~atlasHugged integration — 6 essays transplanted to `content/philosophy/`~~ (Wave 133d)

### Taxonomy completeness
- [x] ~~Audit all 222 content pages: grep content for entity names not tagged in front matter~~ → `spore-validate validate --check` audit_taxonomy_coverage (Wave 120)
- [x] ~~Build-time validation~~ → `spore-validate` Rust crate
- [x] ~~Internal link validation~~ → `spore-validate check-links` (543 links validated)

---

## P1 — Near-Term

### Content enrichment
- [x] ~~Add a "Getting Started with plasmidBin" walkthrough (clone → fetch → deploy)~~ — `lab/getting-started-plasmidbin.md` (Wave 133e)
- [x] ~~Transplant remaining 6 atlasHugged stories (01, 02, 03, 05, 09, 11)~~ — philosophy section now complete (12 essays)
- [x] ~~Create Bibliography page from CITATIONS.md~~ — `content/philosophy/bibliography.md`
- [x] ~~Cross-link Story ↔ Philosophy sections~~ — "Read More" sections with mapped pairs
- [x] ~~Fix "Try It" cold-start~~ — replaced wetSpring (broken cold-clone) with groundSpring
- [ ] Expand products pages with composition diagrams and BYOB examples
- [x] ~~Add cross-spring data flow diagram to SPRING_CATALOG.md~~ — ASCII diagram in §2.2 (Wave 134+)

### Thesis integration (gen3/ → sporePrint)
- [x] ~~gen3/thesis/ (16 chapters) — scientific thesis transplant planning~~ — `content/thesis/` scaffolded (18 files, stubs with abstracts + cross-links)
- [x] ~~gen3/thesis/ bulk content transplant~~ — all 16 chapters fully transplanted, maturity badges updated to architectural
- [x] ~~gen3/about/SOVEREIGN_SCIENCE.md~~ — transplanted to `content/philosophy/sovereign_science.md`
- [x] ~~gen3/about/SCYBORG_EXCEPTION_PROTOCOL.md~~ — transplanted to `content/methodology/scyborg_exception_protocol.md`
- [x] ~~gen3/data/FACULTY_SPRING_PROFILES.md~~ — transplanted to `content/audience/faculty_spring_profiles.md`
- [x] ~~gen3/data/BARRACUDA_SCIENTIFIC_COMPUTE_GAPS.md~~ — transplanted to `content/technical/barracuda_compute_gaps.md`
- [x] ~~gen3/data/HOTSPRING_PHASE_B_EVIDENCE.md~~ — transplanted to `content/lab/hotspring_phase_b_evidence.md`
- [x] ~~gen3/data/MURILLO_REPRODUCTION_PLAN.md~~ — transplanted to `content/lab/murillo_reproduction_plan.md`
- [x] ~~gen3/data/neuromorphic_benchmark_datasheet_v1.md~~ — transplanted to `content/technical/neuromorphic_benchmark.md`
- [x] ~~gen3/baseCamp/26~~ — transplanted as `content/science/28_primal_composition_methodology.md`
- [x] ~~gen3/baseCamp/27~~ — transplanted as `content/science/29_heterogeneous_fabric_economics.md`
- [x] ~~gen3/constrained_optimization_ai.md~~ — transplanted to `content/methodology/constrained_optimization_ai.md`
- [ ] gen3/about/LICENSING_STRATEGY.md — may overlap with scyBorg, evaluate
- [ ] gen3/primals/ — primal specification pages (15 primals, interactions, discovery log)
- [ ] gen3/data/ remaining: MISE_EN_PLACE (historical conda fossil, skip)

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

### Accessibility — WCAG 2.2 AAA Target

**Standard**: WCAG 2.2 AAA is the floor. petalTongue sets the ceiling — universal access for any human capability profile: blind developers, paraplegic scientists, cognitive differences, motor impairments. sweetGrass already follows W3C standards. sporePrint and petalTongue will converge on the same principle.

**Completed (Wave 134e):**
- [x] DOM order: `<main>` before `<aside>` (content-first for crawlers and screen readers)
- [x] WCAG AA color contrast pass: accent green #2e7d20 (4.8:1), maturity badges, skip-link
- [x] `prefers-reduced-motion`: disables smooth scroll, transitions, animations
- [x] `aria-hidden="true"` on all decorative emoji (nav, sidebar, badges)
- [x] Emoji removed from section `<title>` tags (10 sections)
- [x] Skip-link uses `:focus-visible` (not `:focus`)
- [x] Search: ARIA combobox/listbox pattern with keyboard navigation
- [x] 404 page: full site chrome with navigation
- [x] Heading hierarchy: no skipped levels in section listings
- [x] Status conveyed by text, not emoji/color alone
- [x] Mobile TOC: collapsed `<details>` instead of `display: none`
- [x] Nav toggle: `aria-controls` + `id` association
- [x] `--c-muted` defined (was referenced but missing)
- [x] `cert-line` opacity removed (was degrading readable text)
- [x] HTML5 validation: 0 structural errors in templates

**Remaining — evolve through:**
- [ ] AAA contrast ratios (7:1 for normal text, 4.5:1 for large) — pass for heading sizes, audit body text
- [x] ~~`prefers-contrast: more` / forced-colors (Windows High Contrast) support~~ — Wave 136b
- [x] ~~AI accessibility: all 23 navigational tables → ordered/unordered lists~~ — Wave 137a (tables strip links in AI fetch-to-text tools)
- [x] ~~JSON-LD `hasPart` on all sections with child pages~~ — Wave 137a (generic CollectionPage fallback + ScholarlyArticle for science); 17 sections as of Wave 150p
- [ ] viz-hydrate.js: keyboard pan/zoom, focusable nodes, labeled controls
- [x] Notebook chart alt text: meaningful descriptions for 12 matplotlib PNGs (done Wave 137b)
- [x] `<figure>` / `<figcaption>` for viz_embed shortcode + descriptive captions on all 6 viz pages (done Wave 138b)
- [ ] `<summary>` containing `<a>` anti-pattern in sidebar — needs UX redesign
- [x] Table `<caption>` and `scope` attributes for capability tables (done Wave 138b)
- [ ] Entity tooltip descriptions: move from `title` attr to visible or `aria-describedby`
- [ ] Test with Orca (Linux), NVDA (Windows), VoiceOver (macOS)
- [x] `html5validator` integration in CI pipeline (`scripts/validate_a11y.sh`, deploy.yml)
- [ ] Pa11y integration for automated WCAG rule checking
- [ ] Keyboard-only full site navigation test (manual, documented)
- [ ] 200% zoom layout test (no horizontal scroll, no content loss)
- [ ] petalTongue: evolve sporePrint jelly-string UI into typed, accessible Rust compositions

### Automation
- [x] ~~Script to pull check counts from spring repos and update stats~~ → `spore-validate refresh` compares LOC, tests, files, crates against registry
- [ ] Script to diff whitePaper/baseCamp against content/science/ for new papers
- [x] ~~Consider lychee CI for non-Zola link checking~~ → `spore-validate check-links` covers internal links; external link validation TBD

### pseudoSpore Gallery (template exists — automation remaining)
- [x] Zola template for `/lab/spores/{name}/` gallery pages (Wave 64)
- [ ] `spore-validate` reads lithoSpore `registry.toml` → generates gallery markdown
- [ ] Gallery index at `/lab/spores/` listing all available pseudoSpores
- [ ] Real download links per gallery page (currently archive names only)

### Sovereign Deployment (LIVE — DNS active since Wave 100+)
- [x] ~~DNS NS cutover: primals.eco NS → ns1/ns2.primals.eco~~ — DNS live, TLS operational
- [x] ~~peptidoglycan build pipeline~~ — replaced by Sovereign CI (Forgejo→sporeGate→golgi, Wave 119)
- [x] ~~GitHub Pages becomes extracellular shadow~~ — deploy.yml labeled "trailing shadow", VPS is sovereign-primary
- [x] ~~Post-DNS: Caddy HTTPS on golgiBody-ext~~ — LIVE with automatic cert renewal
- [ ] Post-DNS: sporePrint deploy.yml → archive to fossilRecord (disable GitHub Pages deploy)
- [x] ~~NestGate CAS integration: verify Zola `public/` outputs are content-addressable via BLAKE3~~ — `cas-manifest` subcommand (Phase 1, Wave 73)

### Search
- [ ] Evaluate elasticlunr search quality at 311+ pages
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

## Wave 64 Targets (Completed)

- [x] pseudoSpore gallery template (`/lab/spores/{name}/`)
- [x] Zola build pipeline validated on WAN (226 pages, 746ms)
- [x] Gate bootstrap AAR completed
- [x] Temporal sync sustained measurement (6 pushes, relay gap identified)
- [x] ~~Forgejo relay hook for sporePrint~~ — superseded by Sovereign CI pipeline (Wave 119)
- [x] ~~peptidoglycan-triggered rebuild pipeline~~ — superseded by Sovereign CI (Forgejo→sporeGate→golgi)
- [x] ~~DNS NS cutover~~ — DNS live (Wave 100+)

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
- [x] 86 tests passing (→ 101 as of Wave 70)
- [x] All deps justified (8 runtime + 1 dev-dep, all pure Rust — zero C toolchain)

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
- [x] `refresh-metrics.sh` RETIRED — CI now calls `spore-validate fetch-refresh --write` directly (Wave 69)
- [x] `validate_parity.sh` deprecated (Rust: `cargo test --test parity -- --ignored`)
- [x] `gonzales/DEPRECATED.md` — migration timeline (data → CAS, charts → petalTongue)
- [x] `ureq` dev-dependency added for HTTP parity tests

## Wave 69 — Sovereign Evolution (June 2, 2026)

### CSS Modularization
- [x] `static/css/main.css` (1006L) → 7 SCSS partials in `sass/css/`
- [x] Enable Zola's built-in `compile_sass` — auto-minified output (19KB vs 24KB)
- [x] Partials: nav, layout, landing, content, pages, entities, connections
- [x] `viz_embed` shortcode added for Zola build compatibility

### Script Retirement + CI Evolution
- [x] `auto-refresh.yml` wired directly to `spore-validate fetch-refresh --write`
- [x] `refresh-metrics.sh` marked RETIRED (fossil record only)
- [x] Stale `render_notebooks.sh` references updated in 7 content files

### Pure-Rust Fetch Evolution
- [x] `ForgeArchiveBackend` — plain HTTP fetch + gzip (flate2) + tar extraction
- [x] `flate2` dependency added (pure Rust via miniz_oxide, no C)
- [x] `detect_backend()` — runtime selection (git if available, else HTTP archive)
- [x] Minimal POSIX tar reader (pure Rust, handles stripped prefix paths)
- [x] `GitBackend::available()` — runtime git detection
- [x] Suitable for sovereign Forgejo on LAN (no TLS needed)

### Certification Fix
- [x] `StoredManifest` dual-field deserialization fix (merkle_root + graph_merkle)
- [x] Remove stale `serde(alias)` from serialize-only `CertificationManifest`

### Documentation
- [x] `CONTEXT.md` updated: Wave 69 state, sovereign deployment details, cutover checklist
- [x] `EVOLUTION_QUEUE.md`: sovereign deployment items marked complete, post-DNS items added
- [x] 11 airSpring notebook `/home/eastgate/` fallbacks identified as UPSTREAM fix (source .ipynb in airSpring repo)

## Wave 70 — Pure Dependencies + Typed Returns + Test Coverage (June 2, 2026)

### Zero-C Dependency Graph
- [x] `blake3 = { features = ["pure"] }` — eliminates `cc` build dependency entirely
- [x] Dependency tree now zero C toolchain: `blake3` pure-Rust, `flate2` rust_backend, no assembly
- [x] JSON-LD "zero C dependencies" claim now technically accurate

### Source Registry Parity
- [x] `sources.toml` synced with `config.toml` — 8 missing repos added (rustChip, plasmidBin, wateringHole, whitePaper, helixVision, blueFish, initioChem, cellMembrane)
- [x] New `[infra]` type category for infrastructure repos
- [x] Updated timestamp to June 2026

### Code Architecture — paths.rs Constants Module
- [x] New `paths.rs` module: `CONFIG_FILE`, `SOURCES_FILE`, `CONTENT_DIR`, `CONTENT_MANIFEST`, `ENTITY_GRAPH_JSON`, `CERTIFICATION_MANIFEST`, `GATE_MARKER`, `SPRINGS_DIR`
- [x] `require_content_dir()` helper replaces 3 duplicated guard patterns in main.rs
- [x] All path literals in main.rs, fetch.rs, provenance.rs now reference `paths::` constants
- [x] Hardcoded strings evolved to single-source-of-truth constants

### Typed Error Returns
- [x] `fetch_and_refresh()` now returns `Result<FetchResult, Error>` (was `Vec<String>`)
- [x] New `FetchResult` struct: `outcomes: Vec<FetchOutcome>`, `clone_root: PathBuf`
- [x] Caller in main.rs uses `?` propagation (no more stringified errors)

### Code Quality
- [x] `links.rs`: `link_resolves()` helper deduplicates 2 identical resolution blocks
- [x] `provenance.rs`: `chrono_free_now()` one-liner inlined (unnecessary indirection removed)
- [x] `report.rs`: `desc.clone()` eliminated (direct write without intermediate allocation)
- [x] `certify.rs`: `graph_merkle.clone()` moved before struct init (explicit binding)
- [x] `fetch.rs`: `kind_label.clone()` eliminated via restructured match arms
- [x] `fetch.rs`: `MockBackend` match arms merged (`Some(Ok(())) | None => Ok(())`)

### Integration Test Coverage
- [x] `check-links` integration test (real content)
- [x] `graph --emit` integration test (temp dir + config copy)
- [x] `graph` without emit integration test
- [x] `certify --emit` integration test (temp dir + manifest write)
- [x] `certify` without emit integration test
- [x] `provenance --write` integration test
- [x] `provenance --verify` integration test
- [x] Test count: 101 (79 unit + 19 integration + 3 refresh_write) — up from 94

## Wave 73 — CAS Foundation + Pre-Cutover (June 3, 2026)

### CAS Manifest (Phase 1)
- [x] `cas.rs` module — BLAKE3 hashing of Zola build output
- [x] `CasManifest` struct: build_id, build_hash, page_count, total_bytes, files
- [x] `cas-manifest` CLI subcommand (`--public-dir`, `--emit`)
- [x] `paths::CAS_MANIFEST` constant (static/cas/build-manifest.json)
- [x] Deterministic build hash (sorted file hashes → BLAKE3 Merkle)
- [x] 4 unit tests + 2 integration tests

### Pre-Cutover Verification
- [x] `specs/PRE_CUTOVER_VERIFICATION.md` — curl --resolve procedure
- [x] GitHub Pages URL audit: zero hardcoded github.io references

### NestGate CAS Integration Design
- [x] `specs/NESTGATE_CAS_INTEGRATION.md` — 4-phase architecture
- [x] Phase 1–4 roadmap: manifest → ingest → serve → mesh aggregation

## Wave 74 — CAS Push + Pipeline Design (June 3, 2026)

### CAS Push (Phase 2 Foundation)
- [x] `cas_push.rs` module — push build artifacts to NestGate via UDS
- [x] JSON-RPC 2.0 over UNIX domain socket (newline-delimited)
- [x] `discover_socket()` — ecosystem-standard discovery (env → BIOMEOS_SOCKET_DIR → XDG)
- [x] `push_manifest()` — content.exists dedup + content.put ingest
- [x] `cas-push` CLI subcommand (`--socket`, `--generate`, `--public-dir`)
- [x] Provenance metadata: source=sporePrint, pipeline=zola-build
- [x] `base64` dependency added (pure Rust, was transitive via ureq)
- [x] `clap` env feature enabled (NESTGATE_SOCKET from env)
- [x] 2 unit tests + 2 integration tests
- [x] Test count: 111 (85 unit + 23 integration + 3 refresh_write)

### Pre-Cutover VPS Live Test
- [x] VPS serves 245 sitemap entries at 66ms TTFB
- [x] All key sections return 200: /, architecture/, science/, lab/, etc.
- [x] CSS, search index, atom feed, certification manifest: all 200
- [x] `/js/viz-hydrate.js`: 404 (non-critical progressive enhancement)
- [x] `/wasm/`, `/gonzales/`: 404 (expected — deprecated/external)

### Pipeline Design
- [x] `specs/BUILD_DEPLOY_PIPELINE.md` — Phase A→B transition strategy
- [x] Caddy config evolution (file_server → reverse_proxy NestGate)
- [x] Deploy script design (zola build → certify → cas-manifest → cas-push)
- [x] Hybrid mode for gradual transition (shadow verification)

## Wave 85–107 — Transport Abstraction + Socket Standard + Deep Debt Zero (June 6–10, 2026)

### Transport Abstraction (Wave 85–103)
- [x] `TransportEndpoint` enum: Uds, Tcp, MeshRelay — canonical serde-tagged type
- [x] `connect_transport()` — generic stream connection from endpoint descriptor
- [x] `send_rpc()` — generic over `Box<dyn ReadWrite>` (no UDS coupling)
- [x] `TRANSPORT_ENDPOINT` env var acceptance (launcher/Songbird injection)
- [x] `resolve_transport_endpoint()` — CLI override → env → socket discovery
- [x] TCP transport implementation (connect to host:port)
- [x] MeshRelay variant defined (ready for songBird ipc.resolve integration)

### HTTP Module Extraction (Wave 104)
- [x] `http.rs` module — extracted from `fetch.rs` (get_body, request_raw, gzip_decompress, extract_tar)
- [x] HTTP redirect following with relative path fix
- [x] Bare-host URL path handling fix
- [x] 9 unit tests for HTTP/tar utilities

### Socket Discovery Unification (Wave 107)
- [x] `discovery::probe_socket(slug, primary_var)` — generic slug-based discovery
- [x] Ecosystem-standard order: explicit env → `BIOMEOS_SOCKET_DIR` → `XDG_RUNTIME_DIR`
- [x] `/tmp` fallback eliminated from production paths (PRIMAL-SOCKET-CLEANUP aligned)
- [x] `cas_push::discover_socket` delegates to `discovery::probe_socket` (DRY)
- [x] `discover` command shows SOCKET_DIRS section

### Deep Debt Sprints (Wave 85–107)
- [x] `commands.rs` extracted from `main.rs` — all subcommand handlers (745L→257L main)
- [x] `links.rs` walk logic unified into `walk_links()` core
- [x] `refresh::scan()` DRYed with closure-based drift push
- [x] `push_manifest` decomposed into `push_single_file` helper (PushFileOutcome enum)
- [x] `announce_request` canonical (discovery.rs) — single announce logic
- [x] `paths::rel_to` helper — replaces 6 duplicated `strip_prefix().unwrap_or()` calls
- [x] Release profile: `lto = true`, `strip = true`, `codegen-units = 1`
- [x] `[lints.clippy]` section in Cargo.toml (pedantic + nursery enforced at crate level)

### Metrics
- [x] 150 tests (122 unit + 25 integration + 3 refresh) — up from 111
- [x] 21 modules, 6195 lines
- [x] Zero clippy warnings (pedantic + nursery)
- [x] Zero `unwrap()` in production code (all 112 in tests only)
- [x] Zero TODO/FIXME/HACK in source
- [x] No file over 800 lines (max: commands.rs at 664)
- [x] All `#[allow]` justified (5 total: precision loss display, uniform handler sig)

## Wave 111 — Gate Expansion + Federation + Content Cascade (June 12, 2026)

### Content Health (Post-Cascade)
- [x] 222 content pages (up from 207 — lab notebook cascade from springs)
- [x] 155 internal links, 0 broken (`check-links` clean)
- [x] Certification manifest VALID (graph merkle unchanged)
- [x] Provenance manifest regenerated (content-manifest.toml synced to 259 pages)
- [x] 66 entities validated, 0 errors

### WAN Mesh Validation (flockGate)
- [x] songbird v0.2.1 fetched from REBUILT depot (BLAKE3: `3fc94365...` verified, 2026-06-12T12:37Z)
- [x] VPS port 7700 confirmed TCP-accepting (nc connection succeeds)
- [x] mesh.init succeeds locally (peer registered, `bootstrap_peers_added: 1`)
- [x] **federation.status: `enabled: true`** — wire fix CONFIRMED in new build
- [x] **latency_ms: 64ms** — WAN peer health probes WORKING (auto-reconnect functional)
- [x] Peer reachable via `path_type: "direct"`, `last_seen_ms` updating
- [x] `active_connections: 0` — resolved by songBird mesh evolution (Wave 132+, auto-advertisement)

### Observations
- Depot was rebuilt (plasmid.harvest ran) — checksums.toml dated 2026-06-12T12:37Z
- Wire fix + auto-reconnect both CONFIRMED operational on flockGate side
- 64ms RTT to VPS = healthy WAN latency (within s_wan_ipc_tolerance bounds)
- Security provider warning is non-fatal (songbird starts without bearDog)
- Persistent federation relay requires BOTH sides to run the new build

## Wave 113–119 — riboCipher + NUCLEUS Activation + Deep Debt (June 13–20, 2026)

### riboCipher Transport Signal (Wave 113)
- [x] `RIBOCIPHER_CLEAR` + `RIBOCIPHER_PROTO_NDJSON` constants (`0xEC 0x01`)
- [x] `send_ribocipher_signal()` — 2-byte preamble write + flush
- [x] `ribocipher_enabled()` — opt-in via `SPOREPRINT_RIBOCIPHER=1` env var
- [x] `connect_transport()` sends signal when enabled (all transport types)
- [x] Unit test: `ribocipher_signal_writes_correct_bytes`

### NUCLEUS Profile Validation (Wave 114–116)
- [x] `nucleus.rs` module — profile parsing, validation, IPC probe
- [x] `NucleusProfile` struct (primals, health, launch, mesh config)
- [x] `validate_profile()` — socket presence check vs declared primals
- [x] `probe_health()` — JSON-RPC `health.ping` over UDS with timeout
- [x] `probe_ribocipher_acceptance()` — mito-beacon signal + response check
- [x] `HealthContract` enum: Compliant / Partial / None (guideStone grade)
- [x] `format_probe_info()` — surfaces primal_id, status, version, latency, contract, mito
- [x] `nucleus` CLI subcommand (`--probe`, `--ribocipher`)
- [x] TOML profile discovery (XDG_CONFIG_HOME → /etc/membrane → fallback)

### Depot Integrity Verification (Wave 116)
- [x] `depot.rs` module — BLAKE3 verification of binary artifacts
- [x] `parse_checksums()` — reads `checksums.toml` manifest
- [x] `verify_depot()` — per-binary hash + size comparison
- [x] `compute_blake3()` — streaming file hash via `update_reader()`
- [x] `commands_depot.rs` — verify, list-arches, discovery display
- [x] `--partial` mode (missing binaries = warnings, not errors)
- [x] 11 unit tests

### NUCLEUS Full Activation on flockGate (Wave 119)
- [x] NestGate JWT secret configured (systemd drop-in override)
- [x] BiomeOS subcommand corrected (`neural-api`, not `server`)
- [x] Songbird PID lock conflict resolved
- [x] 13/13 primals running (all sockets live at `/run/membrane/`)
- [x] WireGuard mesh verified (4 nodes: golgi, sporeGate, pepti, flockGate)

### Deep Debt Sprint (Wave 119)
- [x] `format_probe_info()` evolved to consume `primal_id` + `status` (dead data path eliminated)
- [x] Targeted `#[allow(dead_code)]` on schema-structural fields (was blanket module allow)
- [x] `compute_blake3()` evolved to `update_reader()` (library-optimized buffering)
- [x] `GitBackend` evolved to `.arg(Path)` (idiomatic, no lossy conversion)
- [x] Discovery tests fixed for live-NUCLEUS environment (environment-sensitive slug)

### Metrics (June 20, 2026)
- 183 tests (151 unit + 29 integration + 3 refresh)
- 24 modules, 7744 lines
- Zero clippy warnings (pedantic + nursery, enforced in Cargo.toml)
- Zero `unwrap()` / `expect()` in production code (only LazyLock statics)
- Zero unsafe (forbidden crate-level)
- No file over 800 lines (max: nucleus.rs at 769)
- Edition 2024, rust-version 1.85

## Wave 123 — petalTongue Backend Wiring + Tower P1 Probe (June 22, 2026)

### petalTongue IPC Module (`petaltongue.rs`)
- [x] `PetalTongueClient` — connects via `TransportEndpoint` (same as cas_push)
- [x] `health_check()` — validates health status, version (v1.6.6), uptime
- [x] `render_graph()` — passes entity graph for SVG rendering via `visualization.render.graph`
- [x] `viz()` — request visualizations in SVG or scene-JSON format
- [x] `probe_method()` — generic method availability tester
- [x] `status()` — combined health + capability check
- [x] Discovery: `probe_socket("petaltongue", "PETALTONGUE_SOCKET")` → actual 56-method API

### CLI Subcommands
- [x] `pt-status` — validates petalTongue OPERATIONAL (health.check + methods probed)
- [x] `pt-render` — render entity graph via petalTongue IPC
- [x] `pt-viz` — request visualization in SVG/scene-JSON format
- [x] `tower-status` — P1 readiness probe for BearDog/Songbird/SkunkBat

### Tower P1 Readiness Probe
- [x] `probe_tower_status()` — probes 9 P1-critical methods across 3 Tower primals
- [x] `probe_single_method()` — generic JSON-RPC method availability with response summary
- [x] `summarize_result()` — object key or array length summary for display
- [x] `print_tower_status()` — formatted output with availability icons
- [x] Current state: 6/9 methods available (BearDog 2/3, Songbird 3/3, SkunkBat 1/3)

### Code Quality
- [x] `dispatch_standalone()` extracted from `run()` (fixes `too_many_lines` lint)
- [x] Discovery updated: petalTongue capabilities reflect actual v1.6.6 API
- [x] `pt-render` self-capability declared in discovery
- [x] All clippy nursery+pedantic clean (zero warnings)

### Metrics (June 22, 2026)
- 193 tests (161 unit + 29 integration + 3 refresh)
- 25 modules
- Zero clippy warnings (pedantic + nursery)
- Zero `unwrap()` / `expect()` in production code
- Zero unsafe (forbidden crate-level)
- Edition 2024, rust-version 1.85

### Gaps Identified (upstream P1 work)
- BearDog: `btsp.capabilities` method not yet implemented (BTSP Phase 2)
- SkunkBat: `method_gate.status` + `threat.report` not yet implemented
- petalTongue: graph render returns empty for entity-graph format (schema mismatch — needs format alignment)

## Wave 124 — Tower Activation + Refactor (June 23, 2026)

### Songbird Mesh Activation (P1 — DONE)
- [x] `mesh.init` called with `node_id: "flockgate"`, `bootstrap_peers: ["10.13.37.1:7700"]`
- [x] Result: `initialized: true`, `bootstrap_peers_added: 1`
- [x] `mesh.peers` now returns `{online: 1, peers: [{address: "10.13.37.1:7700", reachable: true}]}`
- [x] golgi peer at WG .1 visible via direct path (`path_type: "direct"`)
- [ ] `mesh.capabilities_announce` rejected ("unknown peer") — requires BTSP trust

### BearDog Trust Protocol (P1 — PARTIALLY DONE)
- [x] `auth.trust_issuer` method validated — requires `{public_key, did, gate_id}`
- [x] Self-trust registered (flockGate → flockGate): `total_trusted_issuers: 1`
- [ ] Cross-gate trust: need public keys from eastGate, sporeGate, golgi, ironGate BearDog instances
- [ ] Protocol: call `auth.public_key` on remote gate → register via `auth.trust_issuer` locally
- [ ] Token verification: issue token on gate A, verify on gate B (BTSP success criterion)

### nucleus.rs Refactor (code quality)
- [x] Tower probe code extracted to `tower.rs` (275 lines)
- [x] `nucleus.rs` reduced from 978 → 770 lines (under 800 threshold)
- [x] 5 new unit tests in `tower.rs` (probe coverage, summarize_result variants)
- [x] All references updated in `main.rs`

### Metrics (June 23, 2026)
- 198 tests (166 unit + 29 integration + 3 refresh)
- 26 modules, no file over 800 lines
- Zero clippy warnings (pedantic + nursery)
- Zero `unwrap()` / `expect()` in production code

## Wave 120 — Sovereign CI + Convergence (June 20, 2026)

### Architecture Documentation Evolution
- [x] README: deployment diagram → Sovereign CI (Forgejo→sporeGate→golgi)
- [x] SOVEREIGN_DEPLOYMENT: pepti removed, sporeGate sole build authority, single VPS
- [x] KDERM_DIDERM: physical layer mapping → pepti replaced by sporeGate periplasm
- [x] TRANSPORT_EVOLUTION: Wave 120 pipeline alongside legacy diagram
- [x] CONTEXT.md: sovereignty paragraph references sporeGate CI
- [x] PRE_CUTOVER_VERIFICATION: marked as fossil record (DNS now live)

### Code Evolution
- [x] `commands_depot.rs`: 8 unit tests added (discovery path walk, verify modes)
- [x] `discover_checksums_from()` refactored out for testability (env var + workspace walk)
- [x] `notebook.rs`: last production `expect()` eliminated (peek+copy+advance pattern)
- [x] Tower primal metrics refreshed: beardog (15380 tests), songbird (14742), skunkbat (482)

### NUCLEUS Verification
- [x] 13/13 compliant (socket-only mode)
- [x] 12/13 probe-responsive (BiomeOS uses async HTTP, not sync JSON-RPC)
- [x] Songbird restored (PID lock + stale process cleanup)
- [x] Depot freshness: 6 binaries size-drifted from Sovereign CI rebuilds

## Wave 132d–133a — Content + Deep Debt Sprint + Sovereign AAR (July 4–6, 2026)

### Content Evolution
- [x] Contact page (`content/contact.md`)
- [x] Living Systems page (`content/lab/living-systems.md`)
- [x] Sovereign CI page (`content/architecture/SOVEREIGN_CI.md`)
- [x] Compute access rewrite (`content/lab/compute-access.md`)
- [x] Static SVG diagrams: `gate-mesh.svg`, `ci-pipeline.svg`
- [x] `viz_embed` shortcode: static SVG fallback (graceful degradation without petalTongue)
- [x] Landing page: Living Systems explore card
- [x] Lab index: Living Systems section

### Deep Debt Resolution
- [x] `nucleus.rs` smart refactor (930L → 565L) — display extracted to `nucleus_display.rs`
- [x] Tower probe table: `TOWER_PROBES` → `DEFAULT_TOWER_PROBES` + profile-driven override via `PrimalEntry.probe_methods`
- [x] `SYSTEMD_SOCKET_DIR` → env-overridable via `BIOMEOS_SYSTEMD_SOCKET_DIR`
- [x] `MockBackend` isolated into `mod tests` in `fetch.rs` (was module-scope `#[cfg(test)]`)
- [x] Path literal dedup: `main.rs` → `paths::ENTITY_GRAPH_JSON`, `certify.rs` → `paths::CONTENT_DIR`
- [x] IPC consolidation: shared `ipc::send_rpc` across nucleus + tower + petaltongue
- [x] `cargo-deny` supply chain security (deny.toml)
- [x] `toml` 1.x migration (TOML spec 1.1)

### Sovereign Deployment AAR
- [x] `SPOREPRINT_SOVEREIGN_DEPLOY_AAR_133a.md` — 5 divergences identified:
  - ~~SP-DIV-01: GitHub Pages still primary (DNS cutover pending)~~ — **RESOLVED**: sovereign VPS primary since Wave 100+
  - SP-DIV-02: Dual-push required (origin + forgejo)
  - SP-DIV-03: No NUCLEUS on VPS (blocks live viz)
  - SP-DIV-04: `temporal.cascade` doesn't rebuild Zola
  - SP-DIV-05: `deploy.yml` still load-bearing
- [x] `EASTGATE_WAVE133_SPOREPRINT_VPS_NUCLEUS.md` — deployment handoff for NUCLEUS on golgi

### Metrics (July 6, 2026)
- 258 tests (226 unit + 29 integration + 3 refresh)
- 28 modules, 10,109 lines
- All files under 800 lines (max: commands.rs at 710)
- Zero clippy warnings (pedantic + nursery)
- Zero `unwrap()` / `expect()` in production code
- Zero unsafe (forbidden crate-level)
- 226 content pages

## Wave 133d — Content Transplant: atlasHugged + Story + Methodology (July 8, 2026)

### Content Evolution

- **Philosophy section filled**: 6 atlasHugged essays transplanted from whitePaper/gen3
  (The Human Search, The Temptation of Kingdoms, The Mobility Edge, Discovery Is Local,
  I Own Nothing, The Knowledge-Numeric). Philosophy `_index.md` no longer says "Coming".
- **Story section created**: New `content/story/` with 3 builder narrative essays
  (I Don't Know Rust, The Sovereign Lab, 70 Papers One Stack). These provide the human
  narrative arc for outsiders.
- **Methodology depth**: Sharing the Pen transplanted from whitePaper/gen4/knome —
  why the K-NOME methodology itself is shared under CC-BY-SA.
- **Nav + sidebar wiring**: Story link in nav bar, both Story and Philosophy expand in
  sidebar tree to show child pages.
- **Content page count**: 228 → 238 (10 new pages).
- **Integration test fix**: `cas_push_requires_manifest_or_generate` assertion updated
  for lowercase error message from transport unification.
- All private references ("attsi") stripped per transplant boundary rules.

## Wave 134 — Phase 1+2 Idiomatic Rust + petalTongue Integration
- [x] Extract `walk_content_files` / `walk_markdown_files` into `paths.rs` (4 duplicate WalkDir patterns eliminated)
- [x] Extract `connect_uds` helper into `ipc.rs` (3 duplicate UDS setups consolidated)
- [x] Create `DiagnosticCollector` struct in `error.rs` (bridge for gradual `Vec<Diagnostic>` migration)
- [x] Add `#[must_use]` to ~15 pure functions across 7 modules
- [x] Evolve `normalize_key` to `Cow<str>`, `systemd_socket_dir` to `Cow<'static, str>`
- [x] Decompose `commands::validate` into `validate_registry` + `validate_content`
- [x] Decompose `http::request_raw` into `parse_url` + `read_response` + `HttpResponse` struct
- [x] Decompose `cas_push::push_single_file` into `encode_file_payload` + RPC send
- [x] Clean `petaltongue.rs` module docs (remove stale `content.render` reference)
- [x] Delete dead `section_count.html` shortcode
- [x] Clean `gonzales_explorer.md` (~550 lines dead CSS/JS removed)
- [x] Add `build-viz` subcommand (petalTongue IPC → static SVG generation at build time)
- [x] Add `MaturityLevel` enum to `model.rs` (6 levels, build-time validation, CSS class mapping)
- [x] Wire `validate_maturity_levels` into `--check` content validation
- [x] Tests: 260 → 272 (12 new tests for extracted patterns, MaturityLevel, viz scanner)

## Wave 133c — Transport Unification + Catalog Metric Evolution (July 7, 2026)

### Code Evolution
- [x] `discovery::resolve_primal_endpoint()` — unified transport resolution for all primals
  - CLI override → `TRANSPORT_ENDPOINT` env → socket discovery (same interface for NestGate + petalTongue)
  - petalTongue commands (`pt-render`, `pt-viz`) now honor `TRANSPORT_ENDPOINT` (was socket-only)
- [x] Centralized timeout constants in `paths.rs` — `PROBE_TIMEOUT`, `TRANSPORT_CONNECT_TIMEOUT`, `TRANSPORT_IO_TIMEOUT`
  - Eliminates 4 duplicated definitions across nucleus.rs, tower.rs, cas_push.rs, http.rs
- [x] `cas_push::discover_socket()` and `petaltongue::discover_socket()` removed — replaced by shared resolver
- [x] Mid-file `use crate::` imports moved to top of file (fetch.rs, refresh.rs, notebook.rs)

### Content Evolution
- [x] PRIMAL_CATALOG.md: 11 hardcoded `**Tests**:` lines removed — `entity_metrics` shortcode renders live registry values
- [x] Narrative test count in sweetGrass section → `entity_stat` shortcode

### Metrics (July 7, 2026)
- 260 tests (228 unit + 29 integration + 3 refresh)
- 28 modules, 10,112 lines
- All files under 800 lines (max: commands.rs at 699)
- Zero clippy warnings (pedantic + nursery)
- Zero `unwrap()` / `expect()` in production code
- Zero unsafe (forbidden crate-level)
- 227 content pages
