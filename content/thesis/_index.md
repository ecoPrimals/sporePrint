+++
title = "Thesis"
description = "Constrained Evolution — Kevin Mok's working thesis connecting biological adaptation, AI-assisted software development, pure Rust scientific computing, and sovereign infrastructure through reproducible validation across eight domains."
sort_by = "weight"
template = "section.html"
+++

## Constrained Evolution

*Environmental Pressure, Sovereign Computing, and the Convergence of Biological and Computational Systems*

**Author**: Kevin Mok — BS Microbiology (Michigan State University, 2018), MS Data Science (Michigan State University, 4.0, 2025)
**Status**: Working draft — all 16 chapters transplanted, undergoing refinement
**License**: AGPL-3.0

> **Historical snapshot.** Metrics in the thesis body reflect the gen3 working draft (circa February 2026: ~757K LOC, ~104K tests, 11 primals, 5 springs). Current ecosystem metrics: {{ total_stat(stat="total_loc_display") }} LOC, {{ total_stat(stat="total_tests_display") }} tests, {{ total_stat(stat="total_primals") }} primals, {{ total_stat(stat="total_springs") }} springs ([Evidence Snapshot](@/architecture/EVIDENCE_SNAPSHOT.md)). The thesis numbers will be reconciled as chapters undergo refinement.

---

## Thesis Statement

Strategic environmental constraints — in both biological and computational systems — do not merely accelerate convergence to known solutions. They reshape fitness landscapes, driving specialization toward constraint-specific optima through independent evolutionary trajectories. This principle, observed in thermophilic adaptation, controlled laboratory evolution, natural population genomics, and AI-assisted software development within a strong type system, constitutes a general theory of constrained evolution applicable across domains.

This thesis presents the theory, the computational system built under its principles, and the empirical scientific validation proving the system computes real physics, biology, chemistry, and mathematics correctly.

---

## Structure

### Part I — Foundations

| # | Chapter | |
|---|---------|---|
| 00 | [Front Matter](@/thesis/00_front_matter.md) | Abstract, acknowledgments, and dissertation metadata |
| 01 | [Introduction](@/thesis/01_introduction.md) | Taq polymerase motivation, thesis statement, five contributions |
| 02 | [Literature Review](@/thesis/02_literature_review.md) | Extremophile biology, LTEE, type theory, AI-assisted development |

### Part II — Theory

| # | Chapter | |
|---|---------|---|
| 03 | [Theoretical Framework](@/thesis/03_theoretical_framework.md) | Formal constrained evolution: fitness landscapes, biology→computation mapping, predictions |
| 04 | [Accept and Generate](@/thesis/04_pnp_enzyme.md) | Nature's strategy for hard problems — generators, verifiers, enzymes |

### Part III — The System

| # | Chapter | |
|---|---------|---|
| 05 | [System Architecture](@/thesis/05_system_architecture.md) | ecoPrimals sovereign platform: primals, compositions, NUCLEUS |
| 06 | [BarraCuda](@/thesis/06_barracuda.md) | Vendor-agnostic Pure Rust GPU compute (WGSL/Vulkan, f64) |

### Part IV — Experimental Validation

| # | Chapter | |
|---|---------|---|
| 07 | [Experimental Methodology](@/thesis/07_experimental_methodology.md) | The spring framework: phased validation across domains |
| 08 | [Results: hotSpring](@/thesis/08_results_hotspring.md) | Computational plasma physics — Sarkas MD, nuclear EOS, lattice QCD |
| 09 | [Results: airSpring](@/thesis/09_results_airspring.md) | Precision agriculture — FAO-56 ET, sensor calibration |
| 10 | [Results: wetSpring](@/thesis/10_results_wetspring.md) | Life science & analytical chemistry — 16S, QS, phylogenetics, PFAS |
| 11 | [Results: groundSpring](@/thesis/11_results_groundspring.md) | Measurement noise & uncertainty — the tolerance foundation |
| 12 | [Results: neuralSpring](@/thesis/12_results_neuralspring.md) | ML primitives, Isomorphism Theorem, coralForge |

### Part V — Analysis

| # | Chapter | |
|---|---------|---|
| 13 | [Quantitative Evidence](@/thesis/13_quantitative_evidence.md) | NTT→FFT evolution, convergent IPC, fastidious specialization |
| 14 | [Biological Validation](@/thesis/14_biological_validation.md) | LTEE frozen-fossil sequencing proposal at MSU |

### Part VI — Synthesis

| # | Chapter | |
|---|---------|---|
| 15 | [Discussion](@/thesis/15_discussion.md) | Strengths, limitations, alternative explanations, trade-offs |
| 16 | [Conclusion](@/thesis/16_conclusion.md) | Five contributions, future work, closing synthesis |

### Back Matter

| | | |
|---|---|---|
| A | [References](@/thesis/references.md) | Full bibliography |
| B | Appendix: Hardware Inventory | See [Contact](@/contact.md) |
| C | Appendix: AI Methodology | See [Sharing the Pen](@/methodology/sharing_the_pen.md) |
| D | Appendix: Spring Validation | See [Spring Catalog](@/architecture/SPRING_CATALOG.md) |

---

## How to Read This

**If you are a committee member**: Start with [Introduction](@/thesis/01_introduction.md) for the thesis statement, then [Theoretical Framework](@/thesis/03_theoretical_framework.md) for the core argument, then [Methodology](@/thesis/07_experimental_methodology.md) and any results chapter in your domain.

**If you are evaluating the science**: Start with [Methodology](@/thesis/07_experimental_methodology.md) and the results chapter for your domain. Each spring is a self-contained validation study with public repositories you can clone and run.

**If you are evaluating the system**: Start with [System Architecture](@/thesis/05_system_architecture.md) and [BarraCuda](@/thesis/06_barracuda.md), then [Quantitative Evidence](@/thesis/13_quantitative_evidence.md) for the NTT→FFT constrained evolution case study.

**If you are interested in the biology**: Start with [Theoretical Framework](@/thesis/03_theoretical_framework.md) sections on Taq, Lenski, and Anderson, then [Biological Validation](@/thesis/14_biological_validation.md) for the LTEE sequencing proposal.

---

## Lineage

This thesis evolved through three generations:

1. **Inoculum** — `constrained_optimization_ai.md` — first formulation during the ecoPrimals build. Rough metrics, illustrative analogies, no empirical validation.

2. **Working Paper** — [Constrained Evolution — Formal](@/methodology/CONSTRAINED_EVOLUTION_FORMAL.md) — reframed from "optimization" to "evolution" after engaging with the biological literature. Added Lenski LTEE, Taq, Anderson genomics, and the firefly/symbiotic composition arguments.

3. **Dissertation** — this section — full academic work with literature review, formal mathematical framework, five chapters of empirical results, proposed biological validation, and honest discussion of limitations.

The philosophical counterpart lives in [atlasHugged](@/philosophy/_index.md). Where the thesis asks *"does constrained evolution work?"*, atlasHugged asks *"why does it matter?"* — in particular, [The Human Search](@/philosophy/the_human_search.md) is the "napkin version" of Chapter 03.

---

## A Note on AI-Assisted Writing

This thesis, like the system it describes, was produced with AI assistance (Cursor IDE, Claude). The [Methodology](@/thesis/07_experimental_methodology.md) chapter documents this explicitly. The AI is the mutation operator; the author provides constraint and selection. The thesis is itself a product of the constrained evolution methodology it formalizes.

The public spring repositories contain runnable experiments that verify every quantitative claim. The science stands independent of how it was written. See [Sharing the Pen](@/methodology/sharing_the_pen.md) for the full methodology.
