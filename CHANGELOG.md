# sporePrint Changelog

All notable changes to this whitepaper repository are documented here.
Format: `[version] — date — description`

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

## Roadmap

### [0.2.0] — Methodology completion
- Complete baseCamp Paper 14 (sovereign compute hardware) and add to science/
- Add `methodology/CONSTRAINED_EVOLUTION_FORMAL.md` companion diagrams
- Add `architecture/` diagrams (NUCLEUS bonding model, spring data flow)

### [0.3.0] — Cross-validation + living data
- Wire check counts to live CI badges from spring repositories
- Add `science/CROSS_SPRING_EVIDENCE_MAP.md` — how baseCamp papers draw from multiple springs
- Begin peer review prep for Paper 01 (Anderson QS)

### [1.0.0] — Standalone published whitepaper
- Own git history (extracted from ecoPrimals monorepo)
- GitHub Pages rendering (academic layout)
- DOI via Zenodo
- ORCID attribution
- Citable as: Mok K. (2026). *sporePrint: Sovereign Scientific Computing
  via Constrained Evolution.* ecoPrimals. doi:10.XXXX/...

### Future — guidePost/
- `guidePost/` companion repo for atlasHugged — the human, ethical, and
  philosophical side: five questions for John Galt, the orthogonal synthesis,
  the love letter, the temptation of kingdoms. The story of why, not just what.
