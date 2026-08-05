# sporePrint Content Map

What exists in `content/`, how sections relate, and what may be stale.

Last reviewed: August 5, 2026 (Wave 156d — 338 pages, 25 sections, 79 entities, 5 cortical folds. Data Flow Activation Era: G18 LIVE, Phase 1 SUCCEEDED, footPrint DEPLOYED, 3.21 TB / 153 datasets, 16⁴ dual-GPU COMPLETE)

## Sections

### Landing Page (`_index.md` + `templates/index.html`)

The landing page is a hybrid: `_index.md` contains the Try It and Why sporePrint markdown content. The `index.html` template adds structured card sections:

- **Hero** — title + tagline
- **Try It** — two code blocks (build from source / guideStone artifact)
- **Stats ribbon** — 6 stat cards (checks, papers, tests, cost, hardware, zero C deps)
- **Find Your Path** — 6 audience cards linking to role-based guides
- **The Ecosystem** — 4 org cards (ecoPrimals, syntheticChemistry, sporeGarden, protoKarya)
- **Explore** — 7 quick-link cards to key pages and taxonomy indexes
- **Why sporePrint** — the spore print metaphor
- **Footer** — license, attribution

**Staleness risk**: Stats ribbon and paper count read from `config.extra.totals` dynamically. Use `spore-validate refresh` to detect metric drift.

### Science (`science/`) — 33 pages + _index

29 baseCamp papers (01–29) + gonzales_explorer + CROSS_SPRING_EVIDENCE_MAP + STRUCTURE_PREDICTION_ROADMAP + primal composition methodology.

Grouped by domain via `[extra] domain` in front matter:
- Microbiology and Ecology (01-06)
- Physics and Materials (07, 10, 24_all_silicon)
- Agriculture and Field Science (08-09)
- Human Health (13)
- Game Science and Systems (17-19)
- Economics and Provenance (20-22)
- Neuromorphic Hardware (26-27)
- Reference (gonzales_explorer, CROSS_SPRING_EVIDENCE_MAP, STRUCTURE_PREDICTION_ROADMAP)
- Ungrouped (11, 12, 14-16, 23, 25)

Uses custom `science_section.html` template.

**Staleness risk**: New papers added in whitePaper/baseCamp before sporePrint.

### Lab (`lab/`) — 132 pages + _index

Live validation results from a running NUCLEUS composition. Includes auto-merged spring notebooks. Subsections:

- `lab/springs/` — per-spring science hubs
- `lab/notebooks/` — exported Jupyter notebooks with embedded charts

**Staleness risk**: Medium — notebooks may diverge from upstream spring outputs.

### Architecture (`architecture/`) — 43 pages + _index

New in 150x: `tower_atomic.md` (Tower Atomic sovereign transport). Updated: `SOVEREIGN_CI.md` (crash-loop breaker, systemd hardening, DNSSEC), `MESH_TOPOLOGY.md` (Tower transport, traffic classes, USB enrollment).

| Page | What | Staleness risk |
|------|------|----------------|
| PRIMAL_CATALOG.md | All 15 primals — narrative identity, LOC, tests, coverage | Medium — versions, test counts |
| SPRING_CATALOG.md | All 9 springs — identity, headline results, phases, researcher map | Medium — check counts |
| ECOSYSTEM_INVENTORY.md | Master repo inventory across all 4 orgs | Medium — new repos, status changes |
| NUCLEUS_ARCHITECTURE.md | NUCLEUS composition model, atomics ladder, Neural API, Dark Forest | Low |
| DEPLOYMENT_MODEL.md | plasmidBin, BYOB model, binary distribution | Low |
| ECOSYSTEM_ARCHITECTURE.md | High-level system design | Low |
| EVOLUTION_TIMELINE.md | Historical timeline (append-only) | Low |
| SOVEREIGN_PRIOR_ART_CATALOG.md | Prior art comparison | Low |
| RENVOIS_KNOWLEDGE_TOPOLOGY.md | Typed entity graph (Diderot → Bush lineage) | Low |
| GUIDESTONE_PUBLICATION.md | Self-certification architecture | Low |
| COMPOSITION_PIPELINE.md | Spring composition model | Low |
| COMPOSITION_PATTERNS.md | Named patterns (Tower Atomic, etc.) | Low |
| FOUNDATION_CONNECTION.md | projectFOUNDATION link | Low |
| SOVEREIGN_DEPLOYMENT.md | K-Derm diderm sovereign hosting | Low |
| KDERM_DIDERM_ARCHITECTURE.md | Inner/outer membrane VPS topology | Low |
| TRANSPORT_EVOLUTION.md | Quorum-sensing evolution | Low |
| EXTERNAL_COLLABORATION.md | Cross-org interaction model | Low |
| GENERATIONAL_ARC.md | gen1→gen5 evolution framework | Low |

### Audience (`audience/`) — 5 pages + _index

| Page | Audience |
|------|----------|
| FOR_FACULTY_AND_PIS.md | PIs evaluating this for their lab |
| FOR_STUDENTS_AND_CORE_FACILITIES.md | Setup guide, 16S walkthrough |
| FOR_HARDWARE_BUILDERS_AND_HOBBYISTS.md | GPU discovery, Games@Home, hardware tiers |
| FOR_COMPLIANCE_AND_INSTITUTIONAL_REVIEW.md | FDA/ISO/HIPAA/GDPR mapping |
| CAPABILITY_PARITY_BRIEF.md | Domain-by-domain comparison vs proprietary tools |

**Staleness risk**: Capability comparisons against external tools. Researcher table (reproduced work, not endorsement).

### Methodology (`methodology/`) — 15 pages + _index

- CONSTRAINED_EVOLUTION_FORMAL.md — core methodology paper
- HOW_TO_START_A_SPRING.md — practical guide
- K_NOME_PROGRAMMING.md — K-Nome methodology
- KNOWLEDGE_COMMONS_TARGETS.md — 9 domains ready for springs
- P_NP_ENZYME_THESIS.md — conceptual
- SCYBORG_LICENSE.md — licensing model
- sharing_the_pen.md — K-NOME collaborative methodology
- constrained_optimization_ai.md — AI-assisted constrained evolution
- scyborg_exception_protocol.md — scyBorg exception handling
- inoculum_standard.md — inoculum propagation rules
- acknowledgments.md — open-source dependency credits

### Technical (`technical/`) — 8 pages + _index

- HARDWARE_COST_ANALYSIS.md, SOVEREIGN_GPU_PIPELINE_PROFILE.md
- GRANT_TECHNICAL_APPENDIX.md, MSU_ASSET_ACCELERATION.md
- KNOME_TEACHING_BRIEF.md, DRUG_DISCOVERY_PIPELINE.md
- barracuda_compute_gaps.md, neuromorphic_benchmark.md

**Staleness risk**: Hardware inventory drifts. GPU pipeline profile needs updates as coralReef evolves.

### guideStone (`guidestone/`) — 6 pages + _index

| Page | What |
|------|------|
| verification_protocol.md | The five properties, self-proving build artifacts |
| deployment_artifacts.md | Anatomy of an artifact, integrity manifest, provenance record |
| cross_substrate_validation.md | 5 substrates, 40/40 bit-identical |
| live_spore_feed.md | Automated liveSpore.json provenance feed pipeline |

### Products (`products/`) — 11 pages + _index

| Page | Product |
|------|---------|
| footprint.md | GIS home planner — first protoKarya protist (LIVE) |
| esotericWebb.md | Cross-evolution CRPG (V22 LIVE at webb.primals.eco) |
| tideglass.md | Sovereign GPS platform (Phase 0) |
| helixVision.md | Sovereign protein structure prediction (AlphaFold-quality, pure Rust) |
| blueFish.md | Sovereign data pipeline (ETL, NCBI integration) |
| lithoSpore.md | Self-verifying scientific deployment artifacts |
| lattice_qcd.md | Lattice QCD interactive explorer |
| pseudoSpore.md | pseudoSpore lifecycle documentation |
| nf-case-study.md | Multi-product composition pattern |
| composition-evolution.md | Composition model documentation |
| creative-surface.md | Creative product organizational model |

### Glossary (`glossary/`) — _index only

Plain-language definitions of every ecosystem term.

### Philosophy (`philosophy/`) — 15 pages + _index

12 atlasHugged essays (complete), bibliography, sovereign science, and The Knowledge Numeric. Author: attsi.
Sidebar groups by category: Stories (01–05), Framework (06–08), Synthesis (09–11), Reference (12 + bibliography).

### Story (`story/`) — 3 pages + _index

Builder narrative essays: I Don't Know Rust, The Sovereign Lab, 70 Papers One Stack.

### Thesis (`thesis/`) — 18 pages + _index

PhD dissertation: 16 chapters fully transplanted, front matter, references. Constrained evolution across 8 scientific domains.

### reachOut (`outreach/`) — 15 pages + _index

Partnership invitations, articles, and consulting. 4 partnership briefs (GPU, gaming, neuromorphic, Steam), 2 community landings (99pi/Radiolab, homelabbers), 8 articles (evidence + critique series), 1 consulting page (Wave 150p).

### Vision (`vision/`) — 2 pages + _index (new in Wave 150p)

| Page | What |
|------|------|
| lansing_scuffle.md | 464K SF solarpunk sovereign campus — building facts, K-Derm zones, thermal loop, humanitarian anchor |
| thermal_sovereignty_building.md | Building-scale thermal sovereignty — solar → GPU → sand → hot water → food |

## Content Summary

| Section | Pages (excl. _index) |
|---------|---------------------|
| lab | 132 |
| architecture | 42 |
| science | 33 |
| thesis | 18 |
| philosophy | 15 |
| methodology | 15 |
| outreach | 15 |
| products | 11 |
| technical | 8 |
| audience | 7 |
| guidestone | 6 |
| story | 3 |
| collaborators | 3 |
| vision | 2 |
| glossary | 0 |
| sitemap | 0 |
| landing | 1 |
| contact | 1 |
| **Total** | **311 pages (including _index files)** |

## Validation

Pre-build validation is handled by `spore-validate` (Rust crate in `crates/spore-validate/`):

- `spore-validate validate` — registry schema, totals, taxonomy tags
- `spore-validate validate --check` — scans `entity()`, `entity_metrics()`, `entity_stat()` shortcodes against registry
- `spore-validate refresh <repos_root>` — compares registry metrics against source repos

CI runs `spore-validate validate --check` + `zola check` before every deploy.

## Taxonomy-Generated Pages (not in content/)

Zola generates these automatically from front matter `[taxonomies]` tags:
- `/primals/` — index of all 15 primals
- `/primals/{name}/` — cross-reference: every page that tags this primal
- `/springs/` — index of all 9 springs
- `/springs/{name}/` — cross-reference: every page that tags this spring

These pages render using `taxonomy_list.html` and `taxonomy_single.html`, pulling display names and emojis from the entity registry in `config.toml`.

## Cross-Section Dependencies

```
_index.md (content)  <->  index.html (template)
                          Stats, audience cards, org cards, explore links are in the TEMPLATE.
                          Try It and Why sporePrint are in the CONTENT.

science/*            ->  science_section.html    (grouped by [extra] domain)
all content pages    ->  [taxonomies] tags        -> taxonomy pages
config.toml          ->  entity_registry          -> taxonomy templates + shortcodes
wateringHole         ->  PRIMAL_EMOJI_STANDARD    -> entity_registry source of truth
```
