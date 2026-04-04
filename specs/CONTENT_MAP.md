# sporePrint Content Map

What exists in `content/`, how sections relate, and what may be stale.

Last reviewed: April 2026

## Sections

### Landing Page (`_index.md` + `templates/index.html`)

The landing page is a hybrid: `_index.md` contains the Try It and Why sporePrint markdown content. The `index.html` template adds structured card sections:

- **Hero** — title + tagline
- **Try It** — two code blocks (build from source / guideStone artifact)
- **Stats ribbon** — 6 stat cards (checks, papers, tests, cost, hardware, zero C deps)
- **Find Your Path** — 6 audience cards linking to role-based guides
- **The Ecosystem** — 3 org cards (ecoPrimals, syntheticChemistry, sporeGarden)
- **Explore** — 6 quick-link cards to key pages and taxonomy indexes
- **Why sporePrint** — the spore print metaphor
- **Footer** — license, attribution

**Staleness risk**: Stats in the template are hardcoded — check counts and paper counts drift as springs evolve. Update both `_index.md` bullet points AND `index.html` stat cards.

### Architecture (`architecture/`) — 8 pages

| Page | What | Staleness risk |
|------|------|----------------|
| PRIMAL_CATALOG.md | All 14 primals — narrative identity, LOC, tests, coverage | Medium — versions, test counts |
| SPRING_CATALOG.md | All 8 springs — identity, headline results, phases, researcher map | Medium — check counts |
| ECOSYSTEM_INVENTORY.md | Master repo inventory across all 3 orgs | Medium — new repos, status changes |
| NUCLEUS_ARCHITECTURE.md | NUCLEUS composition model, atomics ladder, Neural API, Dark Forest | Low |
| DEPLOYMENT_MODEL.md | plasmidBin, BYOB model, binary distribution | Low |
| ECOSYSTEM_ARCHITECTURE.md | High-level system design | Low |
| EVOLUTION_TIMELINE.md | Historical timeline (append-only) | Low |
| SOVEREIGN_PRIOR_ART_CATALOG.md | Prior art comparison | Low |

### Audience (`audience/`) — 5 pages

| Page | Audience |
|------|----------|
| FOR_FACULTY_AND_PIS.md | PIs evaluating this for their lab |
| FOR_STUDENTS_AND_CORE_FACILITIES.md | Setup guide, 16S walkthrough |
| FOR_HARDWARE_BUILDERS_AND_HOBBYISTS.md | GPU discovery, Games@Home, hardware tiers |
| FOR_COMPLIANCE_AND_INSTITUTIONAL_REVIEW.md | FDA/ISO/HIPAA/GDPR mapping |
| CAPABILITY_PARITY_BRIEF.md | Domain-by-domain comparison vs proprietary tools |

**Staleness risk**: Capability comparisons against external tools. Researcher table (reproduced work, not endorsement).

### Science (`science/`) — 28 pages

25 baseCamp papers + CROSS_SPRING_EVIDENCE_MAP + STRUCTURE_PREDICTION_ROADMAP + 24_all_silicon_science.

Grouped by domain via `[extra] domain` in front matter:
- Microbiology and Ecology (01-06)
- Physics and Materials (07, 10, 24_all_silicon)
- Agriculture and Field Science (08-09)
- Human Health (13)
- Game Science and Systems (17-19)
- Economics and Provenance (20-22)
- Reference (CROSS_SPRING_EVIDENCE_MAP, STRUCTURE_PREDICTION_ROADMAP)
- Ungrouped (11, 12, 14-16, 23, 25)

Uses custom `science_section.html` template.

**Staleness risk**: New papers added in whitePaper/baseCamp before sporePrint.

### Products (`products/`) — 3 pages

| Page | Product |
|------|---------|
| esotericWebb.md | Sovereign creative tool (composable primal-powered) |
| helixVision.md | Sovereign protein structure prediction (AlphaFold-quality, pure Rust) |
| blueFish.md | Sovereign data pipeline (ETL, NCBI integration) |

### Methodology (`methodology/`) — 5 pages

- CONSTRAINED_EVOLUTION_FORMAL.md — core methodology paper
- HOW_TO_START_A_SPRING.md — practical guide
- K_NOME_PROGRAMMING.md — K-Nome methodology
- KNOWLEDGE_COMMONS_TARGETS.md — 9 domains ready for springs
- P_NP_ENZYME_THESIS.md — conceptual

### Technical (`technical/`) — 6 pages

- HARDWARE_COST_ANALYSIS.md, SOVEREIGN_GPU_PIPELINE_PROFILE.md
- GRANT_TECHNICAL_APPENDIX.md, MSU_ASSET_ACCELERATION.md
- KNOME_TEACHING_BRIEF.md, DRUG_DISCOVERY_PIPELINE.md

**Staleness risk**: Hardware inventory drifts. GPU pipeline profile needs updates as coralReef evolves.

### guideStone (`guidestone/`) — section only

guideStone verification class documentation. Currently just `_index.md`.

## Taxonomy-Generated Pages (not in content/)

Zola generates these automatically from front matter `[taxonomies]` tags:
- `/primals/` — index of all 14 primals
- `/primals/{name}/` — cross-reference: every page that tags this primal
- `/springs/` — index of all 8 springs
- `/springs/{name}/` — cross-reference: every page that tags this spring

These pages render using `taxonomy_list.html` and `taxonomy_single.html`, pulling display names and emojis from the entity registry in `config.toml`.

## Cross-Section Dependencies

```
_index.md (content)  ←→  index.html (template)
                          Stats, audience cards, org cards, explore links are in the TEMPLATE.
                          Try It and Why sporePrint are in the CONTENT.

science/*            →  science_section.html    (grouped by [extra] domain)
all content pages    →  [taxonomies] tags        → taxonomy pages
config.toml          →  entity_registry          → taxonomy templates + shortcodes
wateringHole         →  PRIMAL_EMOJI_STANDARD    → entity_registry source of truth
```
