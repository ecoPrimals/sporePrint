# sporePrint — ecoPrimals: Sovereign Scientific Computing

**The public record and verification portal for ecoPrimals, a sovereign
scientific computing ecosystem where the science is executable, the
infrastructure is inspectable, and the claims can be reproduced by anyone
with commodity hardware.**

**Developer:** ecoPrimal — human + synthetic intelligence  
**Organization:** ecoPrimals  
**License:** CC-BY-SA 4.0 (documents) · AGPL-3.0-or-later (code)  
**Status:** Living document — grows with the ecosystem  
**Last Updated:** March 17, 2026

---

## What You Can Do With This

ecoPrimals lets you:

- **Run real scientific pipelines locally** — genomics, protein structure,
  lattice QCD, pharmacometrics, precision agriculture, signal processing
- **Reproduce published results** — 175+ papers across 7 domains, each as a
  binary you can run and verify
- **Use any GPU** — NVIDIA, AMD, Intel — no CUDA lock-in, no vendor toolchain
- **Own your data and compute** — nothing leaves your machine, no cloud, no
  API keys, no institutional access required
- **Scale from one desktop to a lab cluster** — same code, same binaries

**In 5 minutes, you can verify this yourself:**

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
git clone https://github.com/ecoPrimals/wetSpring && cd wetSpring/barracuda
cargo test --workspace          # 1,443+ tests, 0 failures
cargo run --release --bin validate_anderson_3d   # exit 0 = pass
cargo deny check                # zero license violations, zero C dependencies
```

If those commands run, the claims in this repository are verified. No
institutional access. No proprietary software. No cloud.

---

## How the System Works

```
┌─────────────────────────────────────────────────────────────┐
│                    What You See (Public)                     │
│                                                             │
│  sporePrint ──── this repo: proof and documentation         │
│       │                                                     │
│       ▼                                                     │
│  Springs ──────── 7 science repos, each with executable     │
│  (wetSpring,      experiments that reproduce published      │
│   hotSpring,      papers on commodity hardware              │
│   neuralSpring,                                             │
│   airSpring,          ┌──────────────────────────┐          │
│   groundSpring,       │  What Powers It (Public)  │          │
│   healthSpring,       │                          │          │
│   ludoSpring)         │  BarraCuda ── 806 GPU    │          │
│       │               │    math shaders (the     │          │
│       │               │    sovereign cuBLAS)     │          │
│       ▼               │                          │          │
│  Validation ────────► │  ToadStool ── hardware   │          │
│  binaries that        │    discovery + compute   │          │
│  exit 0 on pass,      │    orchestration         │          │
│  1 on failure         │                          │          │
│                       │  coralReef ── sovereign  │          │
│                       │    GPU compiler (the     │          │
│                       │    sovereign nvcc)       │          │
│                       └──────────────────────────┘          │
└─────────────────────────────────────────────────────────────┘
```

**Springs** are validation environments — Rust binaries that reproduce
published science. **Primals** are infrastructure components — the math
engine, the hardware orchestrator, the compiler. Everything is pure Rust,
AGPL-3.0, zero C dependencies.

---

## The Numbers

| | |
|---|---|
| **20,695+** checks | Validation binaries across 7 scientific domains — exit 0 on pass |
| **175+** papers | Reproduced from peer-reviewed literature |
| **$0.044** per run | Electricity cost for paper-parity lattice QCD (RTX 4070) |
| **9.9×** f64 uplift | Consumer GPUs via DF64/WebGPU vs CUDA native f64 |
| **27 days** | From first spring to 10,796+ checks across 5 domains |
| **27,169+** tests | Combined across BarraCuda, ToadStool, and coralReef infrastructure |
| **$15K** total hardware | The entire system runs on consumer hardware in a basement |

See [technical/HARDWARE_COST_ANALYSIS.md](technical/HARDWARE_COST_ANALYSIS.md)
for the full cost analysis and
[architecture/EVOLUTION_TIMELINE.md](architecture/EVOLUTION_TIMELINE.md)
for the day-by-day velocity record.

---

## Find Your Path

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
| wetSpring | Life science, microbiome, quorum sensing, field genomics | [ecoPrimals/wetSpring](https://github.com/ecoPrimals/wetSpring) | 5,707+ |
| airSpring | Precision agriculture, ET₀, soil hydrology, phenology | [ecoPrimals/airSpring](https://github.com/ecoPrimals/airSpring) | 3,123+ |
| neuralSpring | ML primitives, reservoir computing, spectral analysis | [ecoPrimals/neuralSpring](https://github.com/ecoPrimals/neuralSpring) | 4,500+ |
| hotSpring | Plasma physics, lattice QCD, GPU sovereign compute | [ecoPrimals/hotSpring](https://github.com/ecoPrimals/hotSpring) | 664+ |
| groundSpring | Uncertainty quantification, noise, spectral theory | [ecoPrimals/groundSpring](https://github.com/ecoPrimals/groundSpring) | 535+ |
| healthSpring | Human health, PK/PD, microbiome, biosignal, drug discovery | [ecoPrimals/healthSpring](https://github.com/ecoPrimals/healthSpring) | 474+ |
| ludoSpring | Game science, HCI, provenance, distributed compute | [ecoPrimals/ludoSpring](https://github.com/ecoPrimals/ludoSpring) | 1,692+ |
| **Total** | | | **16,695+** |

### Three Public Infrastructure Primals (All AGPL-3.0)

| Primal | Domain | Repository |
|--------|--------|-----------|
| ToadStool | Universal compute orchestration — CPU, GPU, NPU, edge | [ecoPrimals/toadStool](https://github.com/ecoPrimals/toadStool) |
| BarraCuda | Pure mathematics — 806 WGSL f64 shaders, precision strategy | [ecoPrimals/barraCuda](https://github.com/ecoPrimals/barraCuda) |
| coralReef | Sovereign WGSL→native GPU compiler | [ecoPrimals/coralReef](https://github.com/ecoPrimals/coralReef) |

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
   mentoring). 69,000+ iterations, 51B tokens, 185-day streak, one dev (ecoPrimal).

3. **[methodology/P_NP_ENZYME_THESIS.md](methodology/P_NP_ENZYME_THESIS.md)** —
   Theoretical extension: enzymes as evidence that P ≠ NP.

4. **[methodology/HOW_TO_START_A_SPRING.md](methodology/HOW_TO_START_A_SPRING.md)** —
   The operational playbook. You don't need to know how to code. You need
   to know how to talk. Phase 0→1→2→3+ protocol, conversation patterns,
   concrete example (fermentation spring), what it costs, what it produces.

### For Someone Evaluating the Cost / Hardware

1. **[technical/HARDWARE_COST_ANALYSIS.md](technical/HARDWARE_COST_ANALYSIS.md)** —
   The f64 Vulkan discovery, the $0.044 run, $15K basement vs ICER/cloud cost
   analysis, NUCLEUS scaling math.

2. **[technical/SOVEREIGN_GPU_PIPELINE_PROFILE.md](technical/SOVEREIGN_GPU_PIPELINE_PROFILE.md)** —
   What we've already replaced (CUDA runtime, cuBLAS, nvcc, NVIDIA driver),
   current performance vs Kokkos, the 3/6/12-month roadmap, and the full
   sovereign stack diagram.

3. **[audience/FOR_HARDWARE_BUILDERS_AND_HOBBYISTS.md](audience/FOR_HARDWARE_BUILDERS_AND_HOBBYISTS.md)** —
   What your GPU actually does, how to build toward the NUCLEUS bonding model,
   Games@Home distributed compute.

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

→ **[science/CROSS_SPRING_EVIDENCE_MAP.md](science/CROSS_SPRING_EVIDENCE_MAP.md)** —
how baseCamp papers draw from multiple springs, where independent implementations
converge on the same results, and what's still open.

→ **[science/STRUCTURE_PREDICTION_ROADMAP.md](science/STRUCTURE_PREDICTION_ROADMAP.md)** —
coralForge: sovereign AlphaFold-quality structure prediction. Current status
(154/154 checks), the isomorphism proof, LTEE application ($1K vs $83K cloud),
and what it enables beyond protein folding (drug docking, metagenomics,
enzyme engineering).

---

## Document Map

```
sporePrint/
├── README.md                       ← this file (whitepaper index)
├── LICENSE                         ← CC-BY-SA 4.0 (docs) + AGPL-3.0 (code)
├── CHANGELOG.md                    ← whitepaper evolution log
│
├── audience/                       ← start here — find your reading path
│   ├── FOR_FACULTY_AND_PIS.md      ← what it replaces, costs, benchmarks
│   ├── FOR_STUDENTS_AND_CORE_FACILITIES.md
│   ├── FOR_HARDWARE_BUILDERS_AND_HOBBYISTS.md
│   ├── FOR_COMPLIANCE_AND_INSTITUTIONAL_REVIEW.md
│   └── CAPABILITY_PARITY_BRIEF.md  ← 8-domain parity vs proprietary tools
│
├── technical/                      ← capability and collaboration docs
│   ├── DRUG_DISCOVERY_PIPELINE.md  ← Anderson MATRIX, 329/329 checks
│   ├── MSU_ASSET_ACCELERATION.md   ← Genomics Core, ICER, ADDRC integration
│   ├── GRANT_TECHNICAL_APPENDIX.md ← NIH/NSF/USDA/DOE evidence tables
│   ├── KNOME_TEACHING_BRIEF.md     ← K-Nome pedagogy
│   ├── HARDWARE_COST_ANALYSIS.md   ← $0.044/run, f64 Vulkan discovery, $15K HPC
│   └── SOVEREIGN_GPU_PIPELINE_PROFILE.md ← vendor replacement: what's done, what's next
│
├── methodology/                    ← how it was built
│   ├── CONSTRAINED_EVOLUTION_FORMAL.md ← the methodology paper
│   ├── K_NOME_PROGRAMMING.md           ← K-Nome operational framework
│   ├── P_NP_ENZYME_THESIS.md           ← P≠NP enzyme argument
│   ├── KNOWLEDGE_COMMONS_TARGETS.md    ← what others can build, scyBorg permanence
│   └── HOW_TO_START_A_SPRING.md        ← operational playbook: anyone can do this
│
├── architecture/                   ← what was built
│   ├── ECOSYSTEM_ARCHITECTURE.md   ← UniBin/ecoBin/NUCLEUS/Neural API
│   ├── PRIMAL_CATALOG.md           ← 14 primals, capabilities, test counts
│   ├── SPRING_CATALOG.md           ← 7 springs, checks, papers, data flow
│   ├── SOVEREIGN_PRIOR_ART_CATALOG.md
│   └── EVOLUTION_TIMELINE.md       ← 27-day sprint, day-by-day record
│
└── science/                        ← the baseCamp papers
    ├── README.md                   ← science index + reading order by discipline
    ├── CROSS_SPRING_EVIDENCE_MAP.md   ← convergence across springs, open questions
    ├── STRUCTURE_PREDICTION_ROADMAP.md ← coralForge → AlphaFold-quality, LTEE
    ├── 01_anderson_qs.md           ← Paper 01: Anderson QS (W_c = 16.26 ± 0.95)
    ├── 02_ltee_extensions.md
    ├── 03_bioag_microbiome.md
    ├── 04_sentinel_microbes.md
    ├── 05_cross_species_signaling.md
    ├── 06_notill_anderson.md       ← Paper 06: dimensional collapse in soil
    ├── 07_sovereign_wdm.md         ← Paper 07: $0.044 lattice QCD
    ├── 08_npu_agricultural_iot.md
    ├── 09_field_genomics.md
    ├── 10_dynamical_qcd_production.md
    ├── 11_bingocube_nautilus_shell.md
    ├── 12_immunological_anderson.md ← Paper 12: Anderson in cytokines (329/329)
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

### What Others Can Build

**[methodology/HOW_TO_START_A_SPRING.md](methodology/HOW_TO_START_A_SPRING.md)** —
The operational playbook for starting your own spring. You don't need a CS
degree, prior Rust experience, or institutional access. You need domain
knowledge (in anything), focus, patience, and a used GPU.

**[methodology/KNOWLEDGE_COMMONS_TARGETS.md](methodology/KNOWLEDGE_COMMONS_TARGETS.md)** —
9 domains ready now with existing primals + public data:
antibiotic resistance, wastewater surveillance, marine ecology, materials science,
climate crop modeling, veterinary PK/PD, and more. Why public data + consumer
hardware + scyBorg = permanently secured knowledge commons.

The companion repo `guidePost/` (planned) will hold `atlasHugged/` — the
ethical, human, and philosophical layer: five questions for John Galt, the
orthogonal synthesis, the new city, the love letter, the temptation of kingdoms.

---

## Why "sporePrint"

A spore print is how mycologists identify species they have never seen before.
You press the cap to paper and leave it overnight. In the morning: the permanent
record of what the organism is, what it can produce, and how to grow it yourself.

This repository is the spore print for ecoPrimals. The permanent, public,
verifiable impression of a sovereign scientific computing ecosystem. Clone it.
Run it. Verify it. Grow from it.

---

*20,695+ checks. 175+ papers. 7 domains. Consumer hardware. One system. AGPL-3.0.*  
*The science is not a claim — it is executable evidence.*
