# sporePrint — ecoPrimals: Sovereign Scientific Computing

**A spore print is the permanent impression an organism leaves for others to
identify, reproduce, and grow from.**

**Author:** Kevin Mok (BS Microbiology, MSU 2018; MS Data Science, MSU 2025)  
**License:** CC-BY-SA 4.0 (documents) · AGPL-3.0-or-later (code)  
**Status:** Living document — grows with the ecosystem  
**Last Updated:** March 17, 2026

---

## What This Is

The ecoPrimals project is a sovereign scientific computing ecosystem — 14
production primals, 7 validated science springs, 20,000+ quantitative science
checks — built by one developer over approximately 10 months using a methodology
called K-Nome.

This repository is the spore print: the public record of what was built, how,
and why. Every claim is verifiable. Every spring is public. Every binary exits
0 on pass, 1 on failure.

**The science lives in public repositories. Clone one and run it.** Nothing
here requires institutional access, proprietary software, or cloud infrastructure.

---

## The Four Audiences

This whitepaper is written for four distinct readers. Start here:

| You are... | Start with |
|------------|-----------|
| **A faculty member or PI** evaluating this work | [→ For Faculty and PIs](audience/FOR_FACULTY_AND_PIS.md) |
| **A student or core facility** wanting to use it | [→ For Students and Core Facilities](audience/FOR_STUDENTS_AND_CORE_FACILITIES.md) |
| **A hardware builder or hobbyist** with a GPU | [→ For Hardware Builders and Hobbyists](audience/FOR_HARDWARE_BUILDERS_AND_HOBBYISTS.md) |
| **A compliance officer, IRB, or legal reviewer** | [→ For Compliance and Institutional Review](audience/FOR_COMPLIANCE_AND_INSTITUTIONAL_REVIEW.md) |

Not sure? Read the [Capability Parity Brief](audience/CAPABILITY_PARITY_BRIEF.md) —
a direct comparison against proprietary tools across 8 scientific domains.

---

## The Ecosystem at a Glance

### Seven Science Springs (All Public, AGPL-3.0)

Springs are validation environments — Rust binaries that reproduce published
science and validate computational methods against known results.

| Spring | Domain | Repository | Checks |
|--------|--------|-----------|:------:|
| wetSpring | Life science, microbiome, quorum sensing, field genomics | [syntheticChemistry/wetSpring](https://github.com/syntheticChemistry/wetSpring) | 5,707+ |
| airSpring | Precision agriculture, ET₀, soil hydrology, phenology | [syntheticChemistry/airSpring](https://github.com/syntheticChemistry/airSpring) | 3,123+ |
| neuralSpring | ML primitives, reservoir computing, spectral analysis | [syntheticChemistry/neuralSpring](https://github.com/syntheticChemistry/neuralSpring) | 4,500+ |
| hotSpring | Plasma physics, lattice QCD, GPU sovereign compute | [syntheticChemistry/hotSpring](https://github.com/syntheticChemistry/hotSpring) | 664+ |
| groundSpring | Uncertainty quantification, noise, spectral theory | [syntheticChemistry/groundSpring](https://github.com/syntheticChemistry/groundSpring) | 535+ |
| healthSpring | Human health, PK/PD, microbiome, biosignal, drug discovery | [syntheticChemistry/healthSpring](https://github.com/syntheticChemistry/healthSpring) | 474+ |
| ludoSpring | Game science, HCI, provenance, distributed compute | [syntheticChemistry/ludoSpring](https://github.com/syntheticChemistry/ludoSpring) | 1,692+ |
| **Total** | | | **16,695+** |

### Three Public Infrastructure Primals (All AGPL-3.0)

| Primal | Domain | Repository |
|--------|--------|-----------|
| ToadStool | Universal compute orchestration — CPU, GPU, NPU, edge | [ecoPrimals/toadStool](https://github.com/ecoPrimals/toadStool) |
| BarraCuda | Pure mathematics — 806 WGSL f64 shaders, precision strategy | [ecoPrimals/barraCuda](https://github.com/ecoPrimals/barraCuda) |
| coralReef | Sovereign WGSL→native GPU compiler | [ecoPrimals/coralReef](https://github.com/ecoPrimals/coralReef) |

---

## The Five-Minute Verification

```bash
# Install Rust (one time, ~5 minutes)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Clone any spring
git clone https://github.com/syntheticChemistry/wetSpring
cd wetSpring/barracuda

# Run the full test suite
cargo test --workspace
# Expected: 1,443+ tests, 0 failures

# Run a specific validation binary (exit 0 = all checks pass)
cargo run --release --bin validate_diversity
cargo run --release --bin validate_anderson_3d

# Audit licenses and dependencies
cargo deny check
# Expected: no violations (AGPL-3.0, zero C dependencies)
```

If those commands run and produce the expected output, the claims in this
whitepaper are verified. No institutional access required. No API keys. No cloud.

---

## Reading Guide

### For a Scientist Evaluating the Science

1. **[science/README.md](science/README.md)** — All 22 baseCamp papers with
   domain, key finding, validation status, and cross-spring dependencies.
   Reading order guides organized by discipline.

2. **[science/01_anderson_qs.md](science/01_anderson_qs.md)** — The headline
   result: Anderson localization as a null hypothesis for quorum sensing.
   3,100+ checks. W_c = 16.26 ± 0.95 quantified. Three-tier validation.

3. **Spring repositories** — Every experiment is a numbered binary you can
   run. The science is not summarized — it is executable.

### For Someone Evaluating the Methodology

1. **[methodology/CONSTRAINED_EVOLUTION_FORMAL.md](methodology/CONSTRAINED_EVOLUTION_FORMAL.md)** —
   The core methodology paper. Environmental constraints (Rust's type system,
   Pure Rust directive, capability-based architecture) drive specialization
   toward fitness. Grounded in three biological lines of evidence: Taq
   polymerase, Lenski's LTEE, Rika Anderson's extremophile population genomics.

2. **[methodology/K_NOME_PROGRAMMING.md](methodology/K_NOME_PROGRAMMING.md)** —
   Knowledge-Numeric Observed & Mentored Evolutionary Programming. The
   operational methodology that produced this ecosystem. Darwinian substrate
   (compiler selects) + Lamarckian process (human expertise heritable through
   mentoring). 69,000+ iterations, 51B tokens, 185-day streak, one developer.

3. **[methodology/P_NP_ENZYME_THESIS.md](methodology/P_NP_ENZYME_THESIS.md)** —
   Theoretical extension: enzymes as evidence that P ≠ NP.

### For Someone Evaluating the Infrastructure

1. **[architecture/ECOSYSTEM_ARCHITECTURE.md](architecture/ECOSYSTEM_ARCHITECTURE.md)** —
   The technical architecture: UniBin → ecoBin → genomeBin ladder, NUCLEUS
   deployment composition, bonding model, Neural API, Dark Forest security.

2. **[architecture/PRIMAL_CATALOG.md](architecture/PRIMAL_CATALOG.md)** —
   All 14 primals: capabilities, test counts, production status.

3. **[architecture/SPRING_CATALOG.md](architecture/SPRING_CATALOG.md)** —
   All 7 springs: current phase, check counts, papers reproduced,
   cross-spring data flow.

### For Grant Reviewers

**[technical/GRANT_TECHNICAL_APPENDIX.md](technical/GRANT_TECHNICAL_APPENDIX.md)** —
Validation evidence by agency program (NIH, NSF, USDA, DOE, ARPA-H). Every
claim references a public binary with explicit pass/fail output.

### For Potential Collaborators

- **[technical/DRUG_DISCOVERY_PIPELINE.md](technical/DRUG_DISCOVERY_PIPELINE.md)** —
  Anderson-augmented MATRIX drug repurposing. iPSC → HTS → Anderson → validation
  pipeline. Gonzales lab, ADDRC, MSU Drug Discovery integration paths.

- **[technical/MSU_ASSET_ACCELERATION.md](technical/MSU_ASSET_ACCELERATION.md)** —
  How university infrastructure (Genomics Core, ICER HPC, ADDRC, MSDS talent)
  accelerates validated pipelines.

- **[technical/KNOME_TEACHING_BRIEF.md](technical/KNOME_TEACHING_BRIEF.md)** —
  K-Nome as pedagogy for producing real science instead of toy models.

---

## The Science Papers (baseCamp)

These are independent scientific explorations produced by applying the
ecoPrimals stack to questions driven by curiosity and domain expertise.
Each stands alone as a potential publication. Together they demonstrate that
sovereign scientific computing produces real, publishable science across
multiple domains.

### Foundation Papers

| Paper | Title | Domain | Status |
|-------|-------|--------|--------|
| [01](science/01_anderson_qs.md) | Anderson Localization as QS Null Hypothesis | Physics × Microbiology | **Validated** — 3,100+ checks |
| [06](science/06_notill_anderson.md) | Anderson as the Mechanism Behind No-Till Soil Health | Soil Ecology × Physics | **Validated** — Track 4 complete |
| [07](science/07_sovereign_wdm.md) | Sovereign WDM Simulation on Consumer GPU | Plasma Physics × Distributed Computing | **Validated** — hotSpring v0.6.31 |
| [12](science/12_immunological_anderson.md) | Anderson in Immunological Signaling | Immunology × Physics × Pharmacology | **Validated** — 329/329 checks |
| [17](science/17_game_design_rigorous_science.md) | Game Design as Rigorous Science | Game Science × HCI | **Validated** — 1,692/1,692 checks |

### Full baseCamp Index

→ **[science/README.md](science/README.md)** — all 22 papers with reading
order guides for microbiologists, physicists, soil scientists, immunologists,
GPU researchers, game designers, economists, and field genomics researchers.

---

## Document Map

```
sporePrint/
├── README.md                     ← this file (whitepaper index)
├── LICENSE                       ← CC-BY-SA 4.0 (docs) + AGPL-3.0 (code)
├── CHANGELOG.md                  ← whitepaper evolution log
│
├── audience/                     ← start here — find your reading path
│   ├── FOR_FACULTY_AND_PIS.md
│   ├── FOR_STUDENTS_AND_CORE_FACILITIES.md
│   ├── FOR_HARDWARE_BUILDERS_AND_HOBBYISTS.md
│   ├── FOR_COMPLIANCE_AND_INSTITUTIONAL_REVIEW.md
│   └── CAPABILITY_PARITY_BRIEF.md
│
├── technical/                    ← capability and collaboration docs
│   ├── DRUG_DISCOVERY_PIPELINE.md
│   ├── MSU_ASSET_ACCELERATION.md
│   ├── GRANT_TECHNICAL_APPENDIX.md
│   └── KNOME_TEACHING_BRIEF.md
│
├── methodology/                  ← how it was built
│   ├── CONSTRAINED_EVOLUTION_FORMAL.md
│   ├── K_NOME_PROGRAMMING.md
│   └── P_NP_ENZYME_THESIS.md
│
├── architecture/                 ← what was built
│   ├── ECOSYSTEM_ARCHITECTURE.md
│   ├── PRIMAL_CATALOG.md
│   ├── SPRING_CATALOG.md
│   └── SOVEREIGN_PRIOR_ART_CATALOG.md
│
└── science/                      ← the baseCamp papers
    ├── README.md                 ← science index + reading order by discipline
    ├── 01_anderson_qs.md
    ├── 02_ltee_extensions.md
    ├── 03_bioag_microbiome.md
    ├── 04_sentinel_microbes.md
    ├── 05_cross_species_signaling.md
    ├── 06_notill_anderson.md
    ├── 07_sovereign_wdm.md
    ├── 08_npu_agricultural_iot.md
    ├── 09_field_genomics.md
    ├── 10_dynamical_qcd_production.md
    ├── 11_bingocube_nautilus_shell.md
    ├── 12_immunological_anderson.md
    ├── 13_sovereign_human_health.md
    ├── 15_precision_brain_heterogeneous_gpu.md
    ├── 16_anaerobic_aerobic_qs.md
    ├── 17_game_design_rigorous_science.md
    ├── 18_rpgpt_sovereign_rpg_engine.md
    ├── 19_games_at_home_distributed_human_computation.md
    ├── 20_novel_ferment_transcript_economics.md
    ├── 21_sovereign_sample_provenance.md
    └── 22_zero_knowledge_medical_provenance.md
```

---

## What Is Not Here

This repository contains the science, methodology, architecture, and
public-facing capability documentation. It does not contain:

- **PhD thesis** (`whitePaper/gen3/thesis/`) — working draft, not yet public
- **atlasHugged** — the philosophical and human side of this work; will be
  published separately as `guidePost/` when ready
- **About / personal background** — available in `whitePaper/gen3/about/`
  for non-anonymous academic handoff; not included in the public science repo
- **Faculty outreach materials** — internal, in private whitePaper layers
- **wateringHole** — internal ecosystem guidance for primals; not public

The companion repo `guidePost/` (planned) will hold `atlasHugged/` — the
ethical, human, and philosophical layer: five questions for John Galt, the
orthogonal synthesis, the new city, the love letter, the temptation of kingdoms.

---

## The Headline Number

**20,000+ quantitative science checks across 7 domains, all passing, produced
by one developer over ~10 months on consumer hardware costing $15K.**

Every check is a validation binary. Every binary exits 0 on pass, 1 on failure.
The science is not a paper claim — it is executable evidence.

---

*A spore print is how mycologists identify species they have never seen before.
You press the cap to paper and leave it overnight. In the morning: the permanent
record of what the organism is, what it can produce, and how to grow it yourself.*

*This is ours.*
