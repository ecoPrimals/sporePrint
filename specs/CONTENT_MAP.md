# sporePrint Content Map

What exists in `content/`, how sections relate, and what may be stale.

Last reviewed: March 31, 2026

## Sections

### Landing Page (`_index.md`)
- Hero + Try It (build from source / guideStone artifact)
- The Numbers (aggregate stats)
- Find Your Path (role-based routing table)
- Three Organizations (ecoPrimals / syntheticChemistry / sporeGarden)
- Ecosystem at a Glance (springs table, public primals, guideStone)
- Why "sporePrint"

**Staleness risk**: Check counts, test counts, paper counts drift as springs evolve.

### Architecture (`architecture/`)
| Page | What | Staleness risk |
|------|------|----------------|
| ECOSYSTEM_ARCHITECTURE.md | High-level system design | Low — structural, not version-dependent |
| PRIMAL_CATALOG.md | All 14 primals with status, capabilities, visibility | Medium — Squirrel, biomeOS versions move fast |
| SPRING_CATALOG.md | All 8 springs with checks, papers, faculty | Medium — check counts, experiment counts |
| DEPLOYMENT_MODEL.md | plasmidBin BYOB model | Low — model is stable |
| EVOLUTION_TIMELINE.md | Historical timeline | Low — append-only |
| SOVEREIGN_PRIOR_ART_CATALOG.md | Comparison with prior art | Low |

### Audience (`audience/`)
Role-based entry points. 5 pages:
- FOR_FACULTY_AND_PIS.md
- FOR_STUDENTS_AND_CORE_FACILITIES.md
- FOR_HARDWARE_BUILDERS_AND_HOBBYISTS.md
- FOR_COMPLIANCE_AND_INSTITUTIONAL_REVIEW.md
- CAPABILITY_PARITY_BRIEF.md

**Staleness risk**: Faculty table, capability comparisons against external tools.

### Science (`science/`)
25 baseCamp papers + 2 reference docs (CROSS_SPRING_EVIDENCE_MAP, STRUCTURE_PREDICTION_ROADMAP).
Grouped by domain via `[extra] domain` in front matter:
- Microbiology and Ecology (papers 01-06)
- Physics and Materials (papers 07-10)
- Agriculture and Field Science (papers 11-12)
- Human Health (paper 13)
- Game Science and Systems (paper 17)
- Economics and Provenance (paper 15)
- Ungrouped / Reference

Uses custom `science_section.html` template for domain grouping.

**Staleness risk**: New papers get added in whitePaper/baseCamp before sporePrint.

### Methodology (`methodology/`)
- CONSTRAINED_EVOLUTION_FORMAL.md
- HOW_TO_START_A_SPRING.md
- KNOWLEDGE_COMMONS_TARGETS.md

**Staleness risk**: Low — these are methodological, not version-dependent.

### Technical (`technical/`)
- SOVEREIGN_GPU_PIPELINE_PROFILE.md
- HARDWARE_COST_ANALYSIS.md
- KNOME_TEACHING_BRIEF.md
- GRANT_TECHNICAL_APPENDIX.md
- MSU_ASSET_ACCELERATION.md

**Staleness risk**: Hardware inventory drifts. GPU pipeline profile may need updates as coralReef evolves.

### guideStone (`guidestone/`)
guideStone verification class and deployment artifact documentation.

**Staleness risk**: Low until next guideStone version ships.

## Cross-Section Dependencies

```
_index.md  ──references──→  all section _index pages
                             architecture/PRIMAL_CATALOG.md
                             architecture/SPRING_CATALOG.md
                             architecture/DEPLOYMENT_MODEL.md
                             guidestone/_index.md
                             audience/* (Find Your Path table)
                             methodology/* (How to Start / Knowledge Commons)

science/*  ──domain tags──→  science_section.html template
architecture/SPRING_CATALOG  ──faculty──→  audience/FOR_FACULTY_AND_PIS
```
