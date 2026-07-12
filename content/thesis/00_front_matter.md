+++
title = "Front Matter"
description = "Dissertation title page and abstract: constrained evolution across biology and computation, ecoPrimals platform, eight validation springs, and proposed LTEE sequencing at MSU."
weight = 0
date = 2026-07-09
+++

{{ maturity(level="architectural") }}

---

**A Dissertation**

Submitted to the ecoPrimals project  
in partial fulfillment of the requirements  
for the degree of

**Doctor of Philosophy**

Interdisciplinary science and engineering

2026 [projected]

---

**The ecoPrimals project**

Background: microbiology and data science

---

## Abstract

This dissertation formalizes and validates a general principle of constrained evolution: that environmental constraints do not merely accelerate convergence to known solutions, but reshape fitness landscapes so that systems specialize toward constraint-specific optima through independent evolutionary trajectories. The argument rests on three pillars.

**Biological foundation.** Three lines of evidence establish the principle in living systems. (1) *Thermus aquaticus* in Yellowstone hot springs produced Taq polymerase — a thermostable enzyme that enabled PCR — because thermal constraint defined the fitness landscape, not because heat made "better" enzymes (Brock, 1967; Chien et al., 1976). (2) Lenski's Long-Term Evolution Experiment demonstrated that 12 identical *E. coli* populations under glucose limitation produced 12 different evolutionary trajectories, all increasing fitness for the constrained environment, with only one lineage evolving the headline innovation of citrate metabolism (Lenski et al., 1991; Blount et al., 2008; Wiser et al., 2013). (3) Anderson's population genomics of *Sulfolobus islandicus* in the same Yellowstone hot springs showed structured population differentiation under thermal constraint, while her deep-sea subsurface work revealed that extreme energy limitation can cause genetic drift to dominate natural selection — a failure mode where constraint outstrips the population's capacity to respond (Campbell et al., 2017; Anderson, 2021; Anderson et al., 2022).

**Computational system.** The principle was applied to construct the ecoPrimals ecosystem: 757,000 lines of Rust across 11 primals, 914 WGSL shaders, and 104,000+ tests, built by a single developer with AI assistance over approximately 10 months. The Rust type system served as the environmental constraint (analogous to temperature in hot springs), the Pure Rust directive and capability-based architecture served as selective direction (analogous to nutrient limitation in the LTEE), and AI-assisted code generation served as the mutation operator (analogous to DNA replication with error). The compile-time fitness check eliminates unsound solutions before they reach the binary — analogous to a ribosome that refuses to translate lethal mRNA.

The system includes BarraCuda, a vendor-agnostic scientific computing engine that runs f64 GPU compute via WGSL/Vulkan on any GPU (NVIDIA, AMD, Intel) without CUDA dependency. BarraCuda's NTT-to-FFT evolution — where a Number Theoretic Transform evolved under cryptographic constraints shares character-identical main compute kernels with the Fast Fourier Transform needed for physics simulation — provides quantitative evidence that the constrained evolution principle operates in computational systems.

**Empirical validation.** Eight scientific validation suites ("springs") prove the system computes real science correctly across physics, agriculture, biology, chemistry, machine learning, and other domains. 11,161+ quantitative checks pass across 70+ reproduced peer-reviewed papers, all on consumer hardware ($600 GPU, $15K basement HPC), all open-source (AGPL-3.0), produced in approximately 69 days (~$0.93 total compute). The springs validate both the infrastructure and the methodology: each spring consumes BarraCuda kernels evolved under constraint and produces empirical evidence that the constrained evolution methodology works.

**Proposed biological validation.** The LTEE frozen fossil record — 75,000+ generations of *E. coli* frozen at 500-generation intervals, physically housed at Michigan State University — provides the opportunity to test the constrained evolution principle biologically, not by analogy. Whole-genome sequencing across timepoints and populations, analyzed with the computational tools validated by the springs, could reveal whether the same statistical signatures (convergent solutions, power-law fitness dynamics, hitchhiker mutations, historical contingency for innovation) appear in both biological and computational evolution under constraint.

The dissertation contributes: (1) a formal theory of constrained evolution bridging biology and computation, (2) a sovereign scientific computing platform validated across 8 domains, (3) 11,161+ empirical data points demonstrating the methodology, and (4) a concrete proposal for biological validation using MSU's LTEE frozen library.

---

## Acknowledgments

This work would not be possible without the faculty and researchers whose published science defines its validation targets. The ecoPrimals project acknowledges them as scientists whose results we reproduce; they are not collaborators or endorsers of this work.

The broader research community — its libraries, its labs, its intellectual culture — made this work possible.

---

## Dedication

To the microorganisms that evolved under constraint — and produced capabilities the unconstrained never needed to discover.
