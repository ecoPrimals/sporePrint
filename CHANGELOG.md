# sporePrint Changelog

All notable changes to this whitepaper repository are documented here.
Format: `[version] — date — description`

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

## Roadmap

### [1.1.0] — Living data + CI integration
- Wire check counts to live CI badges from spring repositories
- Auto-update numbers via GitHub Actions on spring release events
- DOI via Zenodo
- ORCID attribution
- Citable as: Mok K. (2026). *sporePrint: Sovereign Scientific Computing
  via Constrained Evolution.* ecoPrimals. doi:10.XXXX/...

### Future — guidePost/
- `guidePost/` companion repo for atlasHugged — the human, ethical, and
  philosophical side. The story of why, not just what.

### Future — petalTongue integration
- Conversational navigation of site content
- Audio narration from Markdown source
- Accessibility-first interface for all users
