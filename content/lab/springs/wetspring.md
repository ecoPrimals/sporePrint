+++
title = "GPU-Accelerated 16S Bioinformatics Without Galaxy or CUDA — wetSpring"
description = "Self-hosted 16S rRNA pipeline replacing Galaxy/QIIME2: DADA2 denoising, GPU-accelerated UniFrac, LC-MS, PFAS screening. 63/63 papers reproduced in pure Rust."
date = 2026-05-06
weight = 1

[taxonomies]
primals = ["barracuda", "toadstool", "biomeos", "nestgate", "squirrel", "coralreef"]
springs = ["wetspring", "hotspring", "neuralspring", "groundspring"]
+++

## Domain

16S metagenomics, LC-MS feature extraction, PFAS screening, microbial ecology.

**Repository**: [syntheticChemistry/wetSpring](https://github.com/syntheticChemistry/wetSpring)

## The Science Story

wetSpring proves barraCuda can replace the Galaxy/QIIME2/Python bioinformatics stack with sovereign Rust. The complete 16S pipeline — FASTQ → quality → merge → dereplicate → DADA2 → chimera → taxonomy → diversity → UniFrac — runs in Rust with **1 runtime dependency** (flate2 for gzip). The sovereign XML parser eliminates `quick-xml`; the sovereign FASTQ parser eliminates `needletail`.

**63/63 papers reproduced** across 4 research tracks: Waters c-di-GMP/QS, Liu comparative genomics, deep-sea metagenomics, Jones PFAS. 50/50 three-tier eligible papers have full CPU + GPU + metalForge validation.

## Headline Results

- **5,707+ checks** across 376 experiments — all passing
- **1,077× GPU speedup** for spectral cosine matching
- **30 sovereign bio modules**, 1 runtime dependency
- Public benchmark against 4 BioProjects (22 samples) — all match paper ground truth

## Validation Phases

| Phase | Key Result |
|-------|------------|
| 1-2 (Galaxy→Rust) | 30 sovereign bio modules, 135/135 checks |
| 3-7 (GPU pipeline) | Complete 16S on GPU. 1,077× spectral cosine speedup |
| V53+ (Anderson QS) | 52/52 papers, Anderson localization applied to biology |
| V86 (Cross-spring) | 23/23 across 5 springs. -4,753 net lines (deep debt elimination) |

## Researchers Reproduced

| Researcher | Department | Domain |
|------------|-----------|--------|
| Christopher Waters | MMG, MSU | Quorum sensing, c-di-GMP |
| Kevin Liu | CMSE, MSU | Comparative genomics, phylogenetics |
| Jesse Cahill & Chuck Smallwood | Bioscience, Sandia | Biosurveillance |
| A. Daniel Jones | BMB/Chemistry, MSU | Mass spectrometry, PFAS |
| Rika Anderson | Biology, Carleton | Vent metagenomics, pangenomics |

## What the Constraint Revealed

Zero local WGSL — every GPU operation delegates to barraCuda via ToadStool. wetSpring consumes 79 barraCuda primitives without duplicating any math. The three-tier validation pattern (CPU → GPU → metalForge) was pioneered here and adopted across all springs.

wetSpring found and fixed the `log_f64` bug in ToadStool (coefficients halved, causing 1e-3 instead of 1e-15 precision) during Shannon entropy validation — the spring improved the infrastructure it depends on. Also resolved 4 barraCuda gaps: ODE solver, Gillespie stochastic sim, HMM Viterbi, Smith-Waterman alignment.

## Cross-Spring Connections

- **→ ToadStool**: log_f64 bug found and fixed; 79 primitives consumed; three-tier validated
- **→ airSpring**: kriging spatial interpolation; dynamic Anderson W(t) models soil moisture coupling
- **→ hotSpring**: Anderson localization applied to biology — shared spectral primitives
- **→ neuralSpring**: ESN/LSTM anomaly detection for sentinel microbes; NPU int8 quantization validated
- **→ healthSpring**: diversity indices, Anderson lattice → gut colonization resistance, 16S pipeline
- **→ groundSpring**: sequencing noise calibrates rarefaction; 86 named tolerances with provenance

## Public Notebooks

Interactive Jupyter notebooks that visualize wetSpring's frozen experiment data:

- **01 — 16S Pipeline Validation**: Galaxy bootstrap, Track 2 LC-MS, R/vegan diversity parity, NCBI real data
- **02 — Python vs Rust vs GPU**: Benchmark timings, speedup charts, energy consumption
- **03 — Paper Reproductions**: 63/63 papers across 5 researchers and 6 tracks
- **04 — Cross-Spring Connections**: 79 barraCuda primitives, constraint-driven discoveries
- **05 — Soil Anderson Deep Dive**: Track 4 domain exemplar, QS-pore geometry, chemotaxis

Clone the repository, `cd notebooks/`, `jupyter lab`. Or access via [JupyterHub](/lab/compute-access/).

## baseCamp Papers

Papers 01, 03, 04, 05, 06 — see [baseCamp Science](/science/) for full list.
