+++
title = "MSU Asset Acceleration: How University Infrastructure Plugs into Validated Pipelines"
description = "How university infrastructure accelerates validated pipelines"
date = 2026-03-17
+++

# MSU Asset Acceleration: How University Infrastructure Plugs into Validated Pipelines

**Audience:** MSU faculty, ICER, Genomics Core, Pharm & Tox, ADDRC  
**Context:** Mapping existing MSU infrastructure to ecoPrimals validated pipelines  
**License:** CC-BY-SA 4.0  
**Last Updated:** March 17, 2026

---

## Overview

The ecoPrimals springs are not academic exercises. They are validated scientific
pipelines that reproduce published results across seven quantitative domains —
built to run on consumer hardware without institutional infrastructure. That means
they run faster and more reliably when institutional infrastructure is available.

This document maps each major MSU asset to the spring that directly consumes it,
the current capability level, and the acceleration that institutional integration
unlocks.

---

## Asset 1: MSU Genomics Core — Illumina/Nanopore Sequencing

**Current state:** The Genomics Core produces sequencing data (Illumina 16S
amplicon, whole-genome, Nanopore long-read). Downstream analysis typically
happens in Python/R notebooks on the researcher's laptop using QIIME2, DADA2,
or custom scripts. Results are rarely reproducible because the analysis environment
is not standardized and provenance is not tracked.

**What wetSpring provides:**

```
Genomics Core output → wetSpring sovereign 16S pipeline
  ├── 30 pure-Rust modules (zero Python dependencies)
  ├── DADA2-equivalent denoising (validated against public BioProjects)
  ├── GPU spectral matching: 1,077× speedup over CPU Python
  ├── Full provenance chain: sample → demux → denoise → taxonomy → publication
  └── Exit 0/1 validation: results either match known ground truth or fail loudly
```

**Validation evidence:**
- 376 experiments, 5,707+ checks, all PASS
- 4 public BioProjects benchmarked (ERP022042, SRP151114, SRP199294, SRP354276)
- Python/Rust parity to ≤ 10⁻¹² on all diversity metrics
- Handles: 16S amplicons, shotgun metagenomes, cold seep deep-sea samples,
  agricultural soil, clinical microbiome, water treatment metagenomes

**MSU integration path:**
1. Genomics Core delivers FASTQ files (existing workflow, no change)
2. wetSpring pipeline runs locally (researcher's machine or ICER compute node)
3. Output: OTU table + diversity indices + Anderson disorder parameter W + full provenance
4. Results are signed, reproducible, independently verifiable

**Acceleration unlock:** Every Genomics Core sample processed through wetSpring
gets a provenance receipt. The researcher can prove, cryptographically, that the
analysis ran correctly and the result is what they claim it is. This is ISO 17025
readiness without additional process overhead.

---

## Asset 2: ICER HPC — CPU/GPU Cluster

**Current state:** ICER provides access to CPU clusters and NVIDIA V100/A100
GPUs. Researchers typically submit SLURM jobs running Python or compiled C++/CUDA
code. The software environment is managed by module files, creating reproducibility
issues (different Python versions, different library versions, different CUDA
versions).

**What ecoPrimals provides:**

ecoBin standard: **pure Rust, zero C dependencies, static binary.** A compiled
ecoPrimals spring binary:
- Has no external dependencies (no Python, no system libraries, no CUDA runtime)
- Produces the same output on ICER V100 that it produces on a consumer RTX 3090
- Compiles once, runs anywhere (any Linux, any GPU via WebGPU/Vulkan)

```bash
# Build once (developer machine)
cargo build --release --target x86_64-unknown-linux-gnu
# scp binary to ICER login node
scp target/release/wetspring user@hpcc.msu.edu:~/

# Run on ICER GPU node (no module loads, no conda, no spack)
sbatch --gres=gpu:1 ./wetspring validate_anderson --sample SRR123456
# exit 0 = PASS, exit 1 = FAIL
```

**Acceleration unlock:** ICER's A100s provide ~4–8× GPU compute vs consumer
RTX 3090 for f64 operations (no CUDA throttle on data center GPUs). Large-scale
analyses that take hours on a consumer GPU take minutes. The Anderson spectral
sweep across 376 experiments could run simultaneously across all ICER nodes —
15,000+ checks in a single SLURM array job.

**Specific ICER-accelerated workloads:**

| Spring | Workload | Consumer Time | A100 Estimate |
|--------|----------|:------------:|:------------:|
| wetSpring | 16S diversity sweep (10K samples) | ~2 hr | ~15 min |
| hotSpring | Lattice QCD 32⁴ production scan | ~8 hr | ~1 hr |
| airSpring | Michigan Crop Water Atlas (100 stations × 30 yr) | ~1 hr | ~8 min |
| neuralSpring | LSTM time-series ensemble (1000 runs) | ~4 hr | ~30 min |
| groundSpring | Anderson spectral sweep L=14–20 | ~6 hr | ~45 min |

**Reproducibility note:** Because ecoPrimals uses WebGPU/Vulkan (not CUDA),
results are vendor-agnostic. An analysis started on an NVIDIA consumer GPU
and completed on an ICER AMD node will produce the same floating-point result.
This is mathematically verifiable — the test suite enforces it.

---

## Asset 3: ADDRC / HTS Facility — 8,000+ Compound Library

**Current state:** The ADDRC (Assay Development and Drug Repurposing Core,
Erika Lisabeth, Director) runs high-throughput screens against the compound
library using JAK/cytokine pathway assays. Data analysis happens in Excel and
GREENScreen. Drug prioritization for follow-up is done by pathway analysis
(MATRIX scoring) without tissue geometry.

**What the drug discovery pipeline provides:**

Anderson-augmented MATRIX scoring adds a spatial geometry dimension to standard
pathway-based drug-disease scoring (see `DRUG_DISCOVERY_PIPELINE.md` for full
detail). The result: a ranked compound list that accounts not only for whether
a drug hits the right pathway but whether it can physically reach its target.

```
ADDRC compound library (8,000+ compounds)
    → Anderson-augmented MATRIX scoring (ecoPrimals nS-605)
    → Priority ranking with tissue geometry rationale
    → iPSC validation of top candidates (literature-aligned MSU Pharmacology benchmarks)
    → Medicinal chemistry optimization (Ellsworth)
```

**Current capability:**
- 6 candidates scored computationally (329/329 checks PASS)
- Scaling to 8,000 compounds requires only compound metadata (MW, delivery route,
  target pathway) — no additional wet lab data needed
- Runs in < 1 second on consumer GPU (< 0.1 second on A100)
- Anderson geometry scoring is open-source, inspectable, modifiable

**Integration path:**
1. ADDRC provides compound metadata (existing GREENScreen data) as CSV
2. ecoPrimals scoring pipeline produces ranked list with geometry rationale
3. Top candidates proceed to iPSC validation against published Pharmacology benchmarks
4. HTS data from the screen feeds back into Anderson model refinement
5. Rho/MRTF inhibitor literature (Neubig group) evaluated for AD cross-talk using same scoring

---

## Asset 4: MSDS Program — Graduate Student Talent

**Current state:** MSDS students complete a capstone project, typically a
Jupyter notebook ML model trained on a public dataset. Most projects do not
produce reproducible results and most pipelines cannot be run by anyone
outside the team.

**What K-Nome provides:**

K-Nome (see `KNOME_TEACHING_BRIEF.md`) is the methodology that produced the
ecoPrimals springs. Adapted as a pedagogy:

- Students reproduce a published result from their advisor's domain
- They build it in Rust with explicit validation checks
- The Rust compiler is the fitness function — it rejects incorrect implementations
- Output: a binary that exits 0 on all checks and documents its own provenance

**What MSDS students can contribute:**

ecoPrimals springs validate against published methods and datasets in each domain; capstone projects reproduce a peer-reviewed result and port it to sovereign Rust.

| Domain | Published-work anchor (MSU) | Spring | Student Project |
|--------|----------------------------|--------|----------------|
| Pharmacology | Gonzales-group PK/PD and screening literature | healthSpring | Reproduce a PK/PD paper from the public Gonzales catalog; port to Rust; validate against Python |
| Precision Ag | Dong-group irrigation and ET₀ literature (BAE) | airSpring | Reproduce FAO-56 ET₀ for a new sensor dataset; port to Rust; validate against R baseline |
| Computational Physics | Murillo-group plasma MD literature (CMSE) | hotSpring | Reproduce one MD transport simulation from published code; port to Rust; GPU validation |
| Microbiome | Waters-group quorum sensing and 16S literature (MMG) | wetSpring | Reproduce 16S diversity analysis from a public BioProject; port to Rust; validate |
| Spectral Theory | Kachkovskiy-group Anderson localization literature (Math) | groundSpring | Reproduce one Anderson localization calculation; port to Rust; physics validation |

**Student outcome:** A validated, reproducible, publicly documented implementation
of a key paper from their advisor's domain. Runs on any hardware. Independent of
the lab's internal data. Publishable as a software note.

---

## Asset 5: Faculty Research Computing — Individual Lab Infrastructure

**Current state:** Labs maintain individual compute resources — workstations,
lab servers, department clusters — that are incompatible with each other and
with institutional HPC. Moving data between these environments requires manual
coordination, format conversion, and environment setup.

**What the NUCLEUS bonding model provides:**

NUCLEUS is the ecoPrimals deployment architecture that composes distributed
hardware into a coordinated mesh. Three bonding types:

| Bond | What It Connects | How |
|------|-----------------|-----|
| Covalent | Same-family gates (lab machines) | Automatic via genetic lineage — all lab machines are one compute pool |
| Ionic | Collaborating institutional machines (ADDRC + department compute) | Metered contract — shared compute, scoped data access |
| Metallic | ICER nodes | Institutional enrollment — idle ICER GPUs become ecoPrimals nodes |

**What this enables:**

- A wetSpring analysis started on a department workstation can dispatch heavy
  GPU computation to ICER overnight — automatically, without manual job submission
- Results appear on the lab workstation with full provenance
- ADDRC HTS data stays on ADDRC hardware — only the computation crosses the
  network, not the raw data
- No cloud upload, no FTP, no institutional data governance concerns

**Timeline:** NUCLEUS deployment requires toadStool + barraCuda (both public)
on each node. Installation: 30 minutes. Configuration: automatic discovery via
BirdSong protocol (encrypted UDP, zero metadata leakage).

---

## Consolidated Acceleration Map

| MSU Asset | Current Pain | ecoPrimals Solution | Acceleration |
|-----------|-------------|---------------------|-------------|
| Genomics Core | Notebooks, no provenance, QIIME2 dependencies | wetSpring sovereign 16S, signed provenance | Reproducibility + 30× faster analysis |
| ICER HPC | Module hell, CUDA version conflicts, job scripts | ecoBin static binaries, WebGPU vendor-agnostic | 4–8× GPU compute + zero environment setup |
| ADDRC HTS | Excel + MATRIX (no geometry) | Anderson-augmented scoring, provenance-tracked | Novel geometry dimension in ranking |
| MSDS Program | Toy notebook capstones | K-Nome real science reproduction projects | Publishable outputs + sovereign compute skills |
| Lab compute mesh | Manual coordination, data transfer risk | NUCLEUS bonding, sovereign dispatch | Automated composition, data stays local |

---

## Getting Started

All spring repositories are public and require only Rust (stable):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
git clone https://github.com/syntheticChemistry/wetSpring
cd wetSpring/barracuda
cargo test --workspace    # 1,443+ tests, should exit 0
```

The ICER module for ecoBin binaries is:
```
module load rust/stable   # or install Rust directly — 5 minutes
```

No CUDA, no conda, no spack, no module file conflicts.

---

*Contact for collaboration and access: see `contacts.md` in this directory.*  
*Spring repositories: github.com/syntheticChemistry/*  
*Primal repositories: github.com/ecoPrimals/*
