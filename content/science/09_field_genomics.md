+++
title = "Field Genomics"
description = "Field Genomics x Portable Sequencing — sovereign NCBI-to-Anderson pipeline for real-time environmental DNA. wetSpring."
date = 2026-03-17

[extra]
paper_number = 9
domain = "Agriculture and Field Science"

[taxonomies]
primals = ["barracuda", "beardog", "biomeos", "nestgate", "toadstool"]
springs = ["airspring", "groundspring", "hotspring", "neuralspring", "wetspring"]
+++

**Date:** March 1, 2026
**Status:** Architecture defined — all computational components validated independently. NPU live on AKD1000 ({{ entity(name="wetspring") }} Exp193-195, {{ entity(name="airspring") }} Exp028-029). 16S sovereign pipeline operational ({{ entity(name="wetspring") }} Exp184-185). ESN classifiers validated on hardware. **V84**: 256 experiments, 6,569+ checks, 93 {{ entity(name="toadstool") }} primitives, 26 CPU domains + 21 GPU domains validated, Python parity proven (15 domains bit-identical to SciPy), unidirectional streaming (0.10ms overhead). {{ entity(name="nestgate") }} NCBI pipeline operational (ESearch/ESummary/EFetch). {{ entity(name="biomeos") }} {{ entity(name="nucleus") }} local deployment ready (`biomeos nucleus start --mode node`). **V85**: Genomic Vault organ model (Exp259) — consent-gated encrypted storage with Merkle provenance chain for MinION data sovereignty. Vault module treats genomic data as a personal organ (consent tickets, sensitivity hierarchy, tamper-evident audit). Sovereign cipher/hash/signing placeholders ready for {{ entity(name="beardog") }} absorption (ChaCha20-Poly1305, Ed25519, BLAKE3). {{ entity(name="nucleus") }} deployed with all 6 primals READY on Eastgate (Exp258). 260 experiments, 6,656+ checks. Awaiting sequencer hardware (MinION Mk1D or Mk1C) for end-to-end integration.
**Domain:** Environmental genomics, field sequencing, edge inference, adaptive sampling
**Novelty:** First architecture combining nanopore sequencing with neuromorphic (AKD1000) edge classification via a sovereign Rust bioinformatics pipeline. NPU-driven adaptive sampling for real-time read selection. No cloud dependency, no vendor SDK, no Python runtime.
**Cross-Spring:** {{ entity(name="wetspring") }} (16S pipeline, NPU driver, ESN classifiers, Anderson QS, PFAS ML) × {{ entity(name="airspring") }} (soil sensors, water balance, agricultural IoT) × {{ entity(name="neuralspring") }} (ESN/LSTM reservoir computing, spectral analysis) × {{ entity(name="hotspring") }} (akida-driver, GPU Lanczos) × {{ entity(name="groundspring") }} (uncertainty budgets, rare biosphere, sensor noise)

---

## Abstract

Current field-deployed nanopore sequencing (Oxford Nanopore MinION) relies on
Python-based basecalling (Guppy/Dorado), cloud-connected analysis pipelines
(QIIME2, EPI2ME), and laptop-class compute. We propose an autonomous field
genomics architecture that replaces every component with sovereign Rust
equivalents and adds neuromorphic edge classification:

1. **BarraCuda** processes raw nanopore signal through a validated 16S pipeline
   (DADA2, chimera, taxonomy — 5,743+ checks across 229 experiments)
2. **AKD1000 NPU** classifies community profiles in real time (18.8K Hz,
   <10 mW, coin-cell battery life) using ESN reservoir computing
3. **NPU-driven adaptive sampling** feeds accept/reject decisions back to the
   sequencer, enriching for target organisms without wet-lab preparation
4. **{{ entity(name="metalforge") }}** routes workloads across sequencer → GPU → NPU → sequencer
   in a closed feedback loop

The result: a field station that sequences environmental DNA, classifies
community state, and acts (alert, adapt sampling, log) without human
intervention or network connectivity.

---

## 1. The Problem with Current Field Sequencing

Oxford Nanopore's MinION has proven field-deployable for environmental
monitoring:

- **Lake Erie HABs**: HABSSED pipeline detects *Microcystis* blooms from
  eDNA (Patin et al. 2022)
- **On-site HAB detection**: RosHAB provides taxonomic ID in hours
  (Pérez-Cataluña et al. 2023)
- **Soil microbiome**: Sterile sentinels + MinION differentiate crop
  rotations (Steele et al. 2024)
- **Airborne eDNA**: Shotgun sequencing of airborne eDNA assesses whole
  biomes (Nature Ecology & Evolution 2025)
- **AMR surveillance**: Real-time resistance gene monitoring in hospital
  wastewater (npj AMR 2025)

Every one of these deployments shares the same bottleneck: **downstream
analysis requires a laptop, GPU, or cloud connectivity.** The MinION is
portable; the analysis pipeline is not.

The edge compute gap is recognized. CiMBA (arXiv 2504.07298) proposes a
compute-in-memory basecalling accelerator. Fan et al. (arXiv 2510.09339)
design a RISC-V SoC for mobile genomics. Both solve basecalling. Neither
addresses the downstream classification that turns sequence data into
actionable intelligence.

That is what the AKD1000 + BarraCuda stack provides.

---

## 2. Architecture

### 2.1 Field Genomics Unit

```
┌─────────────────────────────────────────────────────────┐
│                 Field Genomics Unit                      │
│                                                         │
│  Environmental sample (water, soil, wastewater)         │
│       │ DNA extraction (rapid kit, 10 min)              │
│       ▼                                                 │
│  ┌──────────┐                                           │
│  │ MinION   │ sequences eDNA in real time               │
│  │ (Mk1D)   │ 450 bp/s per pore × 512 pores            │
│  └────┬─────┘                                           │
│       │ FAST5/POD5 raw signal                           │
│       ▼                                                 │
│  ┌──────────────────┐                                   │
│  │ BarraCuda        │ basecall + 16S + taxonomy         │
│  │ (host CPU/GPU)   │ sovereign Rust, no Python         │
│  └────┬─────────────┘                                   │
│       │ classified reads + community profile            │
│       ▼                                                 │
│  ┌──────────────────┐                                   │
│  │ AKD1000 NPU      │ ESN regime classification         │
│  │ (10 mW, DMA)     │ bloom/healthy/stressed/AMR/PFAS   │
│  └────┬─────────────┘                                   │
│       │ classification + adaptive sampling decision     │
│       ▼                                                 │
│  ┌──────────────────┐                                   │
│  │ Decision engine  │ alert / adapt / log               │
│  │ + MinKNOW API    │ NPU drives read accept/reject     │
│  └──────────────────┘                                   │
│                                                         │
│  Power: 5W solar (MinION) + coin cell (NPU standby)    │
│  Connectivity: optional (nightly sync via Songbird)     │
└─────────────────────────────────────────────────────────┘
```

### 2.2 metalForge Sequencer Substrate

{{ entity(name="metalforge") }} extends from three substrate types to four:

| Substrate | Type | Role | Power |
|-----------|------|------|:-----:|
| CPU (i9-12900K) | Compute | General math, fallback | 125W |
| GPU (RTX 4070) | Compute | Batch basecalling, Anderson spectral | 200W |
| NPU (AKD1000) | Compute | Edge classification, adaptive sampling | 30 mW |
| SEQ (MinION Mk1D) | Sensing | DNA sequencing, read generation | 5-60W |

The dispatch loop becomes a closed feedback cycle:
SEQ (generates reads) → GPU (basecalls) → NPU (classifies) → SEQ (adaptive sampling)

### 2.3 NPU-Driven Adaptive Sampling

Oxford Nanopore's adaptive sampling ejects reads in real time if they don't
match targets. Currently implemented via CPU/GPU alignment (readfish).

The AKD1000 classifies at 18.8K Hz. MinION generates ~500 reads/sec at peak.
The NPU has **37x headroom** for real-time classification of every read.

Applications:
- **Target enrichment**: Keep HAB-associated reads, reject host background
- **Threat detection**: Keep reads matching AMR genes, reject commensals
- **Rare biosphere**: Keep underrepresented taxa, reject dominants
  (guided by {{ entity(name="wetspring") }} Exp051 rare biosphere saturation framework)

---

## 3. Research Programs

### 3.1 Bloom Sentinel Live (Great Lakes HAB Monitoring)

**Springs:** {{ entity(name="wetspring") }} (16S, ESN, NPU), {{ entity(name="airspring") }} (sensor), {{ entity(name="groundspring") }} (uncertainty)
**Hardware:** MinION Mk1D + AKD1000

MinION sequences water eDNA on-site. BarraCuda 16S pipeline processes reads.
ESN bloom classifier ({{ entity(name="wetspring") }} Exp118, 123, 194) runs on AKD1000.
Real-time classification: pre-bloom / active / post-bloom / toxic.

**Local deployment:** CIGLR at UMich runs bi-weekly Saginaw Bay cyanotoxin
monitoring (July-October). NOAA GLERL has continuous buoy data in western
Lake Erie. A MinION + NPU station fills the gap between sampling events.

### 3.2 Soil Health Sentinel

**Springs:** {{ entity(name="wetspring") }} (16S, Anderson), {{ entity(name="airspring") }} (soil sensors, water balance), {{ entity(name="groundspring") }} (noise)
**Hardware:** MinION Mk1D + AKD1000 + SoilWatch 10 array

Extends Track 4 soil QS framework (Exp170-182, 321 checks) and Sub-thesis 08
(NPU agricultural IoT) with field DNA sequencing. Anderson localization
analysis classifies soil health: diverse/healthy vs disturbed vs recovering.

### 3.3 AMR Wastewater Sentinel

**Springs:** {{ entity(name="wetspring") }} (alignment, phylo placement, pangenomics), {{ entity(name="neuralspring") }} (anomaly detection)
**Hardware:** MinION Mk1D + AKD1000

Long-read metagenomics of hospital/municipal wastewater. Nanopore's long
reads (10 kb+) resolve full resistance gene cassettes + mobile genetic
elements that short reads cannot. NPU classifies threat level from community
profiles.

### 3.4 PFAS Dual-Mode Monitor

**Springs:** {{ entity(name="wetspring") }} (PFAS ML, spectral matching, Anderson community shift)
**Hardware:** MinION Mk1D + AKD1000

Nanopore 16S profiling of microbial community response to PFAS exposure,
paired with BarraCuda's validated PFAS ML pipeline (Exp041-042). Emerging
technology: biological nanopores with cyclodextrin can detect individual
PFAS molecules (SciEngine 2025) — same pore technology, chemical sensing mode.

### 3.5 Deep-Sea Autonomous Lander (Long-Term)

**Springs:** All springs (full primal stack)
**Hardware:** MinION + AKD1000 + pressure enclosure + acoustic modem

MinION on autonomous underwater lander near hydrothermal vents. Cold seep
QS analysis ({{ entity(name="wetspring") }} Exp144-145, 299K QS genes across 170 metagenomes)
on NPU. {{ entity(name="songbird") }} uplinks results via acoustic modem.

---

## 4. Cross-Spring Integration

```
                    Sub-thesis 09: Field Genomics
                              │
    ┌─────────────────────────┼─────────────────────────┐
    │                         │                         │
 wetSpring               airSpring              neuralSpring
 16S pipeline            soil sensors           ESN/LSTM classifiers
 NPU driver              water balance          spectral analysis
 Anderson QS             FAO-56 ET₀             reservoir computing
 PFAS ML                 IoT pipeline           anomaly detection
 alignment               field deployment
 phylo placement
    │                         │                         │
    │                    groundSpring                    │
    │                    uncertainty budgets             │
    │                    sensor noise                    │
    │                    rare biosphere                  │
    │                                                    │
    └──────────────── hotSpring ─────────────────────────┘
                     akida-driver
                     GPU Lanczos
                     spectral primitives
```

### Connection to Other Sub-theses

| Sub-thesis | What Field Genomics Adds |
|-----------|------------------------|
| 01 (Anderson QS) | Real-time Anderson regime detection from field eDNA, not lab samples |
| 02 (LTEE) | Longitudinal frozen fossil sequencing with sovereign pipeline |
| 03 (BioAg) | Field-deployed rhizosphere 16S monitoring for inoculant tracking |
| 04 (Sentinels) | The sequencing substrate that completes the sentinel pipeline |
| 05 (Cross-species) | In-field multi-species QS network monitoring |
| 06 (No-till) | Continuous soil community tracking across tillage treatments |
| 07 (WDM) | — (independent domain) |
| 08 (NPU Ag IoT) | Adds genomic data layer to the NPU agricultural sensor stack |

---

## 5. The BarraCuda Math Stack

All downstream modules are validated. Two new modules are needed:

| Module | Status | Description |
|--------|:------:|-------------|
| `io::nanopore` | **to build** | FAST5/POD5 raw signal reader |
| `bio::basecall` | **to build** | Signal → base conversion (or delegate to Dorado) |
| `bio::dada2` | validated | 16S ASV denoising |
| `bio::chimera` | validated | {{ entity(name="chimera") }} detection |
| `bio::taxonomy` | validated | RDP-style classification |
| `bio::diversity` | validated | Shannon, Pielou, rarefaction |
| `bio::bray_curtis` | validated | Community distance |
| `bio::anderson_qs` | validated | Disorder → regime classification |
| `bio::esn` | validated + NPU live | Echo state network reservoir |
| `bio::alignment` | validated | Smith-Waterman (long reads) |
| `bio::phylo_placement` | validated | Metagenomic read placement |
| `bio::pangenome` | validated | Core/accessory gene analysis |
| `bio::dnds` | validated | Nei-Gojobori dN/dS |
| `bio::pfas_ml` | validated | PFAS contamination ML |

---

## 6. Primal Integration

| Primal | Role |
|--------|------|
| **{{ entity(name="toadstool") }}** | GPU basecalling, NPU classification, CPU fallback. `akida-driver` for sovereign NPU. |
| **{{ entity(name="metalforge") }}** | Substrate routing: SEQ → GPU → NPU → SEQ feedback loop. |
| **{{ entity(name="nestgate") }}** | Content-addressed storage for reads, classifications, provenance. Reference DB hosting. |
| **{{ entity(name="songbird") }}** | Nightly weight sync, telemetry, multi-station coordination. Acoustic modem for underwater. |
| **{{ entity(name="beardog") }}** | PUF-based device attestation (Exp195). Sample chain of custody. |
| **{{ entity(name="sweetgrass") }}** | PROV-O tracking: sample → extraction → sequencing → classification → alert. |
| **{{ entity(name="biomeos") }}** | Capability registry, field unit boot sequence, primal lifecycle. |

---

## 7. Why This Stack Is Unique

| Feature | Current Field Sequencing | Sovereign Field Genomics |
|---------|------------------------|-------------------------|
| Basecalling | Python (Guppy/Dorado) | BarraCuda Rust (planned) |
| Classification | Cloud ML or laptop | NPU: 18.8K Hz, <10 mW |
| Adaptive sampling | CPU/GPU alignment (readfish) | NPU: sub-ms latency, 37x headroom |
| Pipeline | QIIME2/Galaxy + internet | Sovereign Rust, zero dependencies |
| Validation | Published tools (black box) | 229 experiments, 5,743+ checks |
| Hardware lock-in | ONT software stack | Pure Rust driver, AGPL-3.0 |
| Power (classification) | Laptop 45-65W | Coin cell, 11 years at 1 Hz |

---

## 8. Experiment Plan

| Exp | Name | Spring | What It Proves |
|-----|------|--------|---------------|
| 196 | Nanopore Signal Bridge | {{ entity(name="wetspring") }} | BarraCuda reads FAST5/POD5, bridges to 16S pipeline |
| 197 | NPU Adaptive Sampling | {{ entity(name="wetspring") }} | NPU classifies partial reads, drives MinKNOW accept/reject |
| 198 | Field Bloom Sentinel E2E | {{ entity(name="wetspring") }} | MinION → basecall → 16S → ESN → NPU → alert |
| 199 | Soil 16S Field Pipeline | {{ entity(name="wetspring") }} × {{ entity(name="airspring") }} | MinION soil eDNA → 16S → Anderson disorder tracking |
| 200 | Soil Health NPU Classifier | {{ entity(name="wetspring") }} × {{ entity(name="airspring") }} | NPU classifies soil community state |
| 201 | AMR Gene Detection | {{ entity(name="wetspring") }} | Long-read → resistance gene identification |
| 202 | AMR Threat NPU Classifier | {{ entity(name="wetspring") }} | NPU classifies resistance profile severity |

---

## References

Oxford Nanopore Technologies (2026). Genomics for a Changing Planet.

Pérez-Cataluña et al. (2023). Rapid on-site detection of harmful algal blooms.
Frontiers in Microbiology 14:1267652.

Patin et al. (2022). eDNA from algal blooms in Lake Erie using MinION.
bioRxiv 2022.03.12.483776.

Calderón-Franco et al. (2025). Nanopore sequencing in bacterial AMR surveillance.
npj Antimicrobials and Resistance.

Steele et al. (2024). Sterile sentinels and MinION sequencing for crop rotations.
Environmental Microbiome.

Arani et al. (2025). CiMBA: On-Device Basecalling via Compute-in-Memory.
arXiv:2504.07298.

Fan et al. (2025). Sequencing on Silicon: AI SoC for Mobile Genomics.
arXiv:2510.09339.

BrainChip Inc. (2025). AKD1500 Edge AI Co-Processor.
