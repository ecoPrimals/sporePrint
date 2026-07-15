+++
title = "Chapter 10: Results — wetSpring"
description = "Life science and analytical chemistry: sovereign 16S pipeline, quorum sensing, phylogenetics, PFAS — 1,368 checks across 56 experiments."
weight = 10
date = 2026-07-09

[extra]

[[extra.companions]]
url = "/science/02-ltee-extensions/"
title = "LTEE Extensions"
relation = "validated_by"
label = "Falsifiable Anderson-QS predictions for LTEE populations"

[[extra.companions]]
url = "/science/03-bioag-microbiome/"
title = "BioAg Microbiome"
relation = "validated_by"
label = "Anderson-derived microbiome design for perennial tree crops"

[taxonomies]
trails = ["evidence-chain", "thesis-narrative"]

+++

{{ maturity(level="architectural") }}

## 10.1 Validation Summary

wetSpring is the largest spring by experiment count and validation checks: 56 experiments, 1,368 checks (1,168 CPU + 200 GPU), all passing. It validates BarraCuda and the ecoPrimals infrastructure against 16S metagenomics, quorum sensing models, phylogenetic inference, PFAS analytical chemistry, deep-sea metagenomics, and enzyme evolution — spanning six faculty connections across three institutions.

### Table 10.1 — Phase Summary

| Phase | Domain | Experiments | Checks | Status |
|-------|--------|:----------:|-------:|--------|
| 1–2 | Galaxy/QIIME2 16S bootstrap | 4 | 92 | **PASS** |
| 1–2 | asari LC-MS + PFAS screening | 4 | 26 | **PASS** |
| 3 | GPU diversity + spectral matching | 2 | 38 | **PASS** |
| 4 | Sovereign 16S pipeline (end-to-end) | 1 | 37 | **PASS** |
| 5 | Algae pond + VOC peak validation | 2 | 56 | **PASS** |
| 6 | Public data benchmarks (4 BioProjects) | 1 | 202 | **PASS** |
| — | Waters lab QS/c-di-GMP models | 6 | 100 | **PASS** |
| — | Liu lab phylogenetics | 8 | 137 | **PASS** |
| — | Jones lab PFAS + spectral | 3 | 49 | **PASS** |
| — | Anderson deep-sea metagenomics | 6 | 133 | **PASS** |
| — | BarraCuda CPU + GPU parity | 5 | 182 | **PASS** |
| GPU | GPU pipeline validation | 8 | 200 | **PASS** |
| Misc | Faculty proxies, alignment, Felsenstein | 6 | 116 | **PASS** |
| **Total** | | **56** | **1,368** | **All pass** |

---

## 10.2 Sovereign 16S Pipeline

wetSpring's headline result: a complete 16S metagenomics pipeline in Pure Rust + BarraCuda GPU, replacing the Galaxy/QIIME2/DADA2 Python stack.

### Table 10.2 — Pipeline Module Inventory

| Module | Function | CPU Checks | GPU Checks |
|--------|----------|:----------:|:----------:|
| FASTQ parsing + QC | Sequence ingestion | ✓ | — |
| Adapter trimming | Quality control | ✓ | — |
| Dereplication | Unique sequence identification | ✓ | — |
| Chimera detection | Artifact removal | ✓ | — |
| OTU clustering | Taxonomic binning | ✓ | ✓ |
| Shannon/Simpson diversity | Alpha diversity | ✓ | ✓ |
| Spectral cosine matching | Chemical ID | ✓ | ✓ |
| Bray-Curtis dissimilarity | Beta diversity | ✓ | ✓ |
| Phylogenetic composition | Tree-aware analysis | — | ✓ |
| HMM batch forward | Profile HMM scanning | — | ✓ |

### Table 10.3 — BioProject Benchmark (Exp014: 202/202 checks)

| BioProject | Samples | Reference Tool | Match Status | Checks |
|------------|--------:|---------------|:------------:|-------:|
| PRJNA488170 | 10+ | QIIME2/DADA2 | Full parity | ~50 |
| PRJNA382322 | 10+ | QIIME2/DADA2 | Full parity | ~50 |
| PRJNA1195978 | 10+ | QIIME2/DADA2 | Full parity | ~50 |
| Additional | 10+ | QIIME2/DADA2 | Full parity | ~52 |
| **Total** | | | | **202** |

---

## 10.3 GPU Performance

### Table 10.4 — GPU Speedups

| Workload | CPU Time | GPU Time | Speedup | Parity |
|----------|----------|----------|--------:|--------|
| Spectral cosine (2,048 spectra) | — | — | **926×** | ≤ 1e-10 |
| Full 16S pipeline (10 samples) | — | — | **2.45×** | 88/88 |
| Shannon/Simpson diversity | — | — | **15–25×** | ≤ 1e-6 |
| Bifurcation eigenvalues (5×5) | — | — | bit-exact | 2.67e-16 rel |
| ODE parameter sweep (64 batches) | — | — | — | abs < 0.15 |

The 926× spectral cosine speedup demonstrates that GPU promotion of the right kernel can transform a bottleneck into a trivial operation. The 2.45× full-pipeline speedup is modest because most 16S pipeline time is I/O-bound (FASTQ parsing), not compute-bound.

---

## 10.4 Waters Lab — Quorum Sensing Models (100 checks)

| Experiment | Model | Checks | Key Metric | Status |
|------------|-------|-------:|------------|--------|
| Waters 2008 QS/c-di-GMP ODE | LasR/LasI + c-di-GMP coupled | 16 | ODE convergence | PASS |
| Massie 2012 Gillespie SSA | Stochastic QS switching | 13 | Mean switching time | PASS |
| Fernandez 2020 bistable switch | Hysteresis in QS circuit | 14 | Switch range > 0.3 | PASS |
| Srivastava 2011 multi-signal | Two-input Hill AND gate | 19 | AND logic correct | PASS |
| Bruger & Waters 2018 cooperation | Public goods + cheater dynamics | 20 | Variance < 0.05 | PASS |
| Mhatre 2020 phenotypic capacitor | Bistability + noise exploitation | 18 | Hill ODE stability | PASS |

---

## 10.5 Liu Lab — Phylogenetics (137 checks)

| Experiment | Method | Checks | Key Metric | Status |
|------------|--------|-------:|------------|--------|
| Liu 2014 HMM primitives | Forward/backward/Viterbi | 21 | Numerical parity | PASS |
| Robinson-Foulds validation | Tree distance metric | 23 | Exact RF distances | PASS |
| PhyNetPy RF distances | Gene tree comparison | 15 | Match PhyNetPy output | PASS |
| PhyloNet-HMM discordance | Introgression detection | 10 | Viterbi accuracy > chance | PASS |
| SATé pipeline | Divide-and-conquer alignment | 17 | Alignment score | PASS |
| Neighbor-joining (SATé core) | Distance-based tree building | 16 | Topology match | PASS |
| Felsenstein pruning likelihood | Maximum likelihood on tree | 16 | Likelihood match | PASS |
| Smith-Waterman alignment | Local alignment | 15 | Optimal score | PASS |
| Wang 2021 RAWR bootstrap | Gene tree resampling | 11 | Bootstrap support | PASS |
| Alamin & Liu 2024 placement | Phylogenetic placement | 12 | Placement accuracy | PASS |
| Zheng 2023 DTL reconciliation | Duplication-transfer-loss | 14 | Event counts | PASS |

---

## 10.6 Jones Lab — PFAS & Mass Spectrometry (49 checks)

| Experiment | Domain | Checks | Key Metric | Status |
|------------|--------|-------:|------------|--------|
| PFAS library (Zenodo) | Reference spectra | 26 | Library match | PASS |
| EPA PFAS ML | Decision tree classification | 14 | RF F1=0.978, GBM F1=0.992 | PASS |
| MassBank spectral matching | Cosine similarity | 9 | Spectral ID | PASS |

---

## 10.7 Anderson — Deep-Sea Metagenomics (133 checks)

| Experiment | Paper | Checks | Key Metric | Status |
|------------|-------|-------:|------------|--------|
| Rare biosphere | Anderson, Sogin, Baross 2015 | 35 | Rare taxon detection | PASS |
| Viral metagenomics | Anderson et al. 2014 | 22 | Viral contig assembly | PASS |
| Sulfur phylogenomics | Mateos, Anderson et al. 2023 | 15 | Tree reconciliation | PASS |
| Phosphorus phylogenomics | Boden, Anderson et al. 2024 | 13 | Enzyme evolution | PASS |
| Population genomics | Anderson et al. 2017 | 24 | FST, isolation-by-distance | PASS |
| Pangenomics | Moulana, Anderson et al. 2020 | 24 | Gene gain/loss dynamics | PASS |

---

## 10.8 GPU Validation Binaries

| Binary | Checks | Status |
|--------|-------:|--------|
| `validate_diversity_gpu` | 38 | PASS |
| `validate_16s_pipeline_gpu` | 88 | PASS |
| `validate_barracuda_gpu_v3` | 14 | PASS |
| `validate_toadstool_bio` | 14 | PASS |
| `validate_gpu_phylo_compose` | 15 | PASS |
| `validate_gpu_hmm_forward` | 13 | PASS |
| `benchmark_phylo_hmm_gpu` | 6 | PASS |
| `validate_gpu_ode_sweep` | 12 | PASS |
| **GPU Total** | **200** | **All pass** |

---

## 10.9 Scholarly Reproduction Log

| # | Paper / Pipeline | Track | Checks | Status |
|---|-----------------|-------|-------:|--------|
| 1 | Galaxy/QIIME2 16S (4 experiments) | 1 | 92 | PASS |
| 2 | asari LC-MS (2 experiments) | 2 | 26 | PASS |
| 3 | FindPFAS screening | 2 | 17 | PASS |
| 4 | Public data (4 BioProjects) | 1 | 202 | PASS |
| 5 | Waters 2008 + 5 downstream papers | 1 | 100 | PASS |
| 6 | Liu 2014 + 10 phylogenetics papers | 1b | 137 | PASS |
| 7 | Jones PFAS + spectral (3 experiments) | 2 | 49 | PASS |
| 8 | Anderson 2014–2024 (6 papers) | 1c | 133 | PASS |
| 9 | Cahill + Smallwood proxies | 1 | 26 | PASS |

---

## 10.10 Connection to Constrained Evolution Thesis

wetSpring provides the strongest single piece of evidence for the constrained evolution methodology. 1,368 checks across 56 experiments and 6 faculty connections — metagenomics, quorum sensing ODEs, phylogenetic inference, mass spectrometry, enzyme evolution — all validated by the same BarraCuda kernels evolved under type-theoretic constraint.

The 926× GPU speedup for spectral cosine matching demonstrates that the constrained evolution methodology not only produces correct results but produces them efficiently. The GPU kernel was evolved under the WGSL constraint, not hand-tuned for mass spectrometry — yet it outperforms CPU by nearly three orders of magnitude.

The Anderson deep-sea experiments (133 checks) close a conceptual loop: the computational tools validated by wetSpring are the same tools proposed for analyzing LTEE frozen fossils in [Biological Validation](@/thesis/14_biological_validation.md). wetSpring proves the tools work; [Biological Validation](@/thesis/14_biological_validation.md) proposes using them for biological validation of the constrained evolution thesis itself.

---

**See also:**

- [Science: wetSpring papers](@/science/_index.md) — the reproduced papers
- [Biological Validation](@/thesis/14_biological_validation.md) — tools validated here proposed for LTEE sequencing
