+++
title = "LTEE Extensions"
description = "Evolutionary Biology x Genomics — falsifiable Anderson-QS predictions for LTEE populations, sovereign structure prediction. wetSpring."
date = 2026-03-17

[extra]
paper_number = 2
domain = "Microbiology and Ecology"

[taxonomies]
primals = ["toadstool"]
springs = ["airspring", "groundspring", "neuralspring", "wetspring"]
+++

**Date:** March 1, 2026
**Status:** Proposal with quantitative predictions (builds on thesis Ch. 14). Anderson anomaly catalog complete (Exp143: 9 anomalies, 3 genuine NP solutions). W_c = 16.26 ± 0.95 now quantified (Exp150). Correlated disorder effects quantified (Exp151). Dilution amplification (W_eff = W/occupancy) validated (Exp137). Track 4 no-till data provides agricultural time-series test bed (Exp170-178). **{{ entity(name="helixvision") }}** (formerly coralForge, {{ entity(name="neuralspring") }} S94): sovereign structure prediction engine for LTEE structural evolution analysis — AlphaFold2/3 primitives validated (Py 62/62, Rs 55/55, GPU 37/37 = 154 checks). See `helixVision/06_LTEE_APPLICATION.md` for full pipeline.
**Domain:** Evolutionary biology, microbial genomics, structural biology
**Novelty:** Specific, falsifiable Anderson-QS predictions for LTEE populations;
integration of constrained evolution signatures across lab, field, and
agricultural sample archives; **sovereign structure prediction for tracking protein fold evolution across 75,000+ generations**

---

## Abstract

The constrained evolution framework generates specific, testable predictions
for how microbial populations evolve under environmental constraint. We
propose applying these predictions to three complementary sample archives:
the Lenski Long-Term Evolution Experiment (LTEE, 75,000+ generations at MSU),
permafrost thaw microbial communities (deep-time natural experiment), and
agricultural soil time series (contemporary managed constraint). Each archive
tests a different facet of the theory: the LTEE tests convergence and
contingency in a controlled setting; permafrost tests constraint release
after millennia of stasis; agricultural time series test constraint
engineering through human management.

---

## 1. The LTEE as the Gold Standard

### 1.1 What Exists

The Lenski LTEE (Michigan State University, started 1988) maintains 12
replicate populations of Escherichia coli in glucose minimal medium with
daily serial transfer. The frozen fossil record preserves samples from every
500 generations — a time machine for evolution.

Key published findings (Lenski et al. 1991, Wiser et al. 2013, Blount et al.
2008, 2012, Tenaillon et al. 2016):
- Fitness increases follow a power law, not asymptote
- Parallel evolution across replicates (convergent pathways)
- Historical contingency (Ara-3 citrate innovation required potentiating mutations)
- Genome streamlining in late generations
- Substantial neutral/hitchhiker fraction in fixed mutations

### 1.2 What the Constrained Evolution Framework Predicts

From thesis Chapter 14, we predict:

| Signature | Prediction | Detection Method |
|-----------|------------|-----------------|
| Convergent solutions | Phenotype-convergent > sequence-identical across replicates | Pathway analysis of WGS across populations |
| Hitchhiker fraction | ~30-50% of fixed mutations are neutral/hitchhiking | dN/dS ratios over time, allele trajectory analysis |
| Power-law dynamics | Mutation accumulation and diversity follow power laws | Time-series regression on diversity metrics |
| Genome streamlining | Late generations show loss-of-function in non-essential genes | Pseudogene accumulation analysis per 500-gen interval |
| Historical contingency | Potentiating mutation patterns precede innovations | Retrospective allele tracking in frozen samples |

### 1.3 The Anderson-QS Layer

A novel prediction from Sub-thesis 01 (Anderson localization):

> If LTEE populations are grown in **biofilm** format (3D structure) vs
> **planktonic** format (shaken flask), QS regulon expression should differ
> according to Anderson's dimensional prediction.

E. coli has a partial QS system (sdiA receptor, no synthase — the
eavesdropper strategy). In biofilm, sdiA should respond to any added
AHL signal; in planktonic, the same signal should fail to coordinate due
to dilution-amplified Anderson disorder (Exp137: W_eff = W_base / occupancy).

This is a testable bench experiment at MSU using existing LTEE populations.

## 2. Permafrost Thaw Communities

### 2.1 Rationale

Permafrost preserves microbial communities under absolute constraint (frozen,
no metabolism, no evolution) for 10,000-100,000+ years. When thawed, these
communities resume evolution under modern conditions — a natural constraint-
release experiment.

### 2.2 Constrained Evolution Predictions

- **Immediate diversity crash**: frozen community meets modern competitors it
  has never co-evolved with → rapid selection under novel constraint
- **QS re-emergence timing**: if thawed community forms biofilm (3D), Anderson
  predicts QS within the community structure regardless of diversity.
  If community remains dispersed in meltwater (planktonic), QS fails.
- **Convergent adaptation**: thawed populations should show accelerated
  convergence toward the same adaptive peaks occupied by modern analogs
  (the fitness landscape is shaped by constraint, not starting genotype)

### 2.3 Sample Sources

- Arctic permafrost cores: active layer microbial ecology well-characterized
  (Mackelprang et al. 2011, Jansson & Taş 2014)
- Antarctic dry valley soils: extremely low diversity (J near 0 →
  low Anderson disorder → QS possible even in 2D mats)
- Rika Anderson's deep-sea vent archives (Carleton College): Sulfurovum
  populations under continuous extreme constraint

## 3. Agricultural Soil Time Series

### 3.1 Rationale

Agriculture is managed constraint: tillage, irrigation, crop rotation,
and agrochemical application reshape the soil microbiome annually.
Long-term agricultural experiments (LTAR, Broadbalk at Rothamsted) maintain
archived soil samples spanning decades.

### 3.2 Anderson-QS Predictions for Agriculture

| Practice | Effect on biome geometry | Anderson prediction |
|----------|------------------------|---------------------|
| No-till | 3D pore structure preserved | QS-active, diverse signaling |
| Conventional till | 3D structure disrupted → 2D surface | QS suppressed, reduced coordination |
| Cover crop | Root rhizosphere = 3D niche | QS re-established in root zone |
| Fumigation | Diversity crash → J near 0 → W near 0 | QS trivially active (ordered lattice) but few species left to communicate |

Testable with archived soil DNA: compare QS gene prevalence (luxI/luxR,
lasI/lasR) between no-till and conventional-till plots from the same LTAR
site and year.

{{ entity(name="airspring") }} contributes complementary agricultural time series: a 60-year
water balance (Wooster OH, Triplett-Van Doren dataset) parallel to LTEE
as a long-term archive; cover crop dual Kc + no-till validation (40/40
checks) providing agronomic context for QS predictions in tilled vs
no-till soil; and the Michigan Crop Water Atlas (100 stations, 80 years
simulated) as a massive temporal dataset for agricultural time series
analysis.

### 3.3 Pivot Bio Connection

The Pivot Bio model (engineering N-fixation via seed-coat inoculant) relies
on the inoculant reaching, colonizing, and maintaining QS-mediated gene
regulation in the root zone. The Anderson model predicts this works because:

1. Root surface → 3D biofilm structure → QS-active (Exp127-130)
2. Inoculant is a monoculture → J near 0 → W near 0.5 → deep in extended regime
3. As soil diversity invades the biofilm → W increases → but stays below W_c
   in 3D

Failure mode: if the inoculant disperses into bulk soil without forming
biofilm → planktonic dilution → Anderson localization → QS regulation fails →
N-fixation gene expression drops. This predicts that **biofilm-forming
ability** of the inoculant strain is the critical success factor.

## 4. Integration: Three Archives, One Framework

```
     LTEE                Permafrost            Agricultural
  (controlled)          (natural)              (managed)
      │                     │                      │
  75K gens              10K-100K yrs           50-150 yrs
  12 replicates         1 thaw event           annual cycles
      │                     │                      │
      └─────────┬───────────┴──────────────────────┘
                │
    Constrained Evolution Predictions:
    • Convergent pathways (not identical mutations)
    • Power-law temporal dynamics
    • Hitchhiker burden proportional to constraint strength
    • Anderson geometry determines QS activity
    • Historical contingency for innovations
```

If all three archives show the same signatures, the constrained evolution
principle is established as domain-general, independent of timescale,
control level, or environmental specifics.

## 5. Practical Requirements

### 5.1 Computational Pipeline

The {{ entity(name="wetspring") }} sovereign pipeline (16S, DADA2, chimera, taxonomy — all
validated, Exp001-070) can process all three sample types. Additionally:

- WGS assembly (MAGs from metagenomes) → pangenomics for convergence detection
- QS gene annotation via the HMM profiles built for Exp140-142
- Anderson geometry assignment from sample metadata (biofilm/planktonic/mat)
- Level spacing ratio computation via {{ entity(name="toadstool") }} primitives (anderson_3d, lanczos)

### 5.2 neuralSpring Integration

{{ entity(name="neuralspring") }} adds ML primitives for LTEE analysis (S135: 966 lib tests,
232 binaries, 220/220 validate_all, 3,034+ checks, 5 WDM surrogates complete):

- **HMM / PhyloNet-HMM** (Liu Papers 016-018): Introgression detection applied
  to LTEE genomes — identify horizontal transfer events and adaptive introgression
  across 75,000 generations
- **Transfer learning** (Exp 004, nW-04): Cross-environment adaptation models.
  Training on one LTEE population, testing on others, mirrors the
  constrained-evolution prediction that convergent pathways (not identical
  mutations) recur across replicates. nW-04 demonstrates classical→WDM transfer
  learning — the same framework applies to training on one LTEE replicate and
  transferring to another
- **LSTM time series** (Study 004, NSE=0.849; nW-03 LSTM reservoir, R²=0.98):
  Predict temporal dynamics of mutation accumulation and fitness plateaus across
  the 12 LTEE populations. nW-03's pooled-readout LSTM (mean + std + last hidden
  state after washout) validates the sequence processing pipeline for extracting
  temporal features from biological time series
- **ESN regime classifier** (nW-05, 96.5% accuracy): Classify evolutionary
  regimes (e.g., pre-citrate vs post-citrate Ara-3) from population genomic
  features using reservoir computing. Fixed-weight ESN + ridge readout requires
  no backpropagation — suitable for rapid regime detection on streaming data

### 5.3 groundSpring Integration

{{ entity(name="groundspring") }} contributes uncertainty quantification and stochastic modeling
directly relevant to LTEE analysis:

- **Exp 014 — Drift vs selection** (R. Anderson 2022): Wright-Fisher fixation
  probability and Kimura neutral theory. Quantifies when stochastic drift
  dominates deterministic selection — the central question for interpreting
  LTEE mutation fixation trajectories. Predicts the ~30-50% hitchhiker
  fraction in §1.2 above. 7/7 Py, 7/7 Rust checks
- **Exp 017 — Quasispecies threshold** (Dolson 2023): Eigen's error threshold
  predicts when mutation rate destroys genetic information. Directly applicable
  to LTEE's observed power-law fitness dynamics — the mutation-selection
  balance determines whether adaptive trajectories are signal (selection)
  or noise (drift). 6/6 Rust checks
- **Exp 016 — Rare biosphere signal detection** (R. Anderson 2015): Sequencing
  depth determines the boundary between real biological signal and sampling
  artifact. Critical for LTEE frozen fossil analysis — detecting rare mutant
  lineages that will later become dominant (pre-potentiating mutations for
  the citrate innovation). 10/10 Rust checks
- **Exp 019 — Jackknife estimation** (Bazavov 2025): Subpercent precision
  error bars via delete-one and block jackknife. The standard method for
  quantifying uncertainty in population genomic measurements from LTEE
  samples. 9/9 Rust checks
- **Exp 004 — Sequencing noise rarefaction**: Genus saturation at 5,000 reads;
  phyla robust at 100 reads. Establishes the sampling floor for LTEE
  community-level analysis. 15/15 Rust checks

**Cross-spring pipeline for LTEE**: {{ entity(name="wetspring") }} (16S/WGS pipeline) →
{{ entity(name="groundspring") }} (sampling noise floor + jackknife error bars + drift vs
selection classification) → {{ entity(name="neuralspring") }} (HMM introgression + ESN regime
detection). This three-spring pipeline covers the full LTEE analysis
workflow from raw sequencing to evolutionary inference.

### 5.4 Wet Lab (MSU Resources)

- MSU Genomics Core: Illumina sequencing for LTEE frozen samples
- MSU RTSF: high-throughput sequencing for soil/environmental DNA
- LTEE access: Lenski Lab, MSU Department of Microbiology and Molecular Genetics
- Soil archives: MSU LTAR (Kellogg Biological Station)

## 6. Falsification Criteria

This sub-thesis is falsifiable:

- If LTEE populations show identical mutations (not pathway convergence) → falsified
- If no-till and tilled soil have the same QS gene prevalence → Anderson prediction falsified
- If permafrost thaw communities skip the diversity crash → constraint-release model falsified
- If biofilm vs planktonic LTEE shows no difference in sdiA expression → Anderson-QS model falsified

The predictions are specific, quantitative, and testable with existing
infrastructure at MSU.
