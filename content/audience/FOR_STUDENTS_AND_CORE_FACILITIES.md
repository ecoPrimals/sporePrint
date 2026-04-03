+++
title = "ecoPrimals for Students, Lab Technicians, and Core Facilities"
description = "Setup guide, 16S walkthrough, and how to start using the ecosystem"
date = 2026-03-17
+++

# ecoPrimals for Students, Lab Technicians, and Core Facilities

**From:** ecoPrimal — human + synthetic intelligence  
**Organization:** ecoPrimals
**Date:** March 17, 2026
**Repositories:** github.com/ecoPrimals — all AGPL-3.0-or-later

---

## What This Is (30-Second Version)

ecoPrimals is a collection of Rust programs that do the same things as
Galaxy, QIIME2, NONMEM, and MassHunter — but faster, reproducible, and free.
You run them on your own hardware. No cloud accounts, no Python environments,
no Docker, no license keys.

If you have a laptop with a GPU (any gaming card works), you can run
GPU-accelerated science. If you don't have a GPU, everything runs on CPU too.

---

## Getting Started (5 Minutes)

### Step 1: Install Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
# Follow prompts. Takes ~2 minutes.
# Restart your terminal, then:
rustc --version  # Should show 1.87+
```

### Step 2: Clone the Spring You Need

| If your lab does... | Clone this | What you get |
|--------------------|-----------|-------------|
| 16S metagenomics | `git clone git@github.com:syntheticChemistry/wetSpring.git` | Full FASTQ→diversity pipeline |
| LC-MS / PFAS | `git clone git@github.com:syntheticChemistry/wetSpring.git` | mzML parsing, peak detection, PFAS screening |
| PK/PD modeling | `git clone git@github.com:syntheticChemistry/healthSpring.git` | Hill dose-response, population PK, NLME |
| Drug repurposing | Both wetSpring + healthSpring | NMF, TransE, MATRIX scoring |
| Biosignal (ECG/PPG) | `git clone git@github.com:syntheticChemistry/healthSpring.git` | Pan-Tompkins, HRV, SpO2, arrhythmia |

### Step 3: Build and Test

```bash
cd wetSpring/barracuda   # or healthSpring/
cargo test --workspace   # Runs ALL tests. Should see 0 failures.
cargo build --release    # Builds all binaries (release mode = fast)
```

### Step 4: Run a Validation

```bash
# Pick one that matches your domain:
cargo run --release --bin validate_diversity         # Shannon, Simpson, Pielou, Chao1
cargo run --release --bin validate_dada2_full        # DADA2 denoising pipeline
cargo run --release --bin validate_pfas_decision_tree # PFAS screening
cargo run --release --bin exp001_hill                # Hill dose-response (healthSpring)
cargo run --release --bin exp075_nlme                # NONMEM/WinNonlin replacement
```

Every binary prints PASS or FAIL with explicit numerical checks. If everything
passes, the pipeline is working correctly on your hardware.

---

## For Genome Core / Sequencing Facility Staff

### What This Replaces

Your current pipeline probably looks like:

```
Illumina sequencer → FASTQ → Galaxy/QIIME2 (Python/conda) → OTU/ASV tables → R (phyloseq/vegan) → diversity stats
```

ecoPrimals collapses this to:

```
FASTQ → cargo run --release --bin validate_<experiment>
```

### The Full 16S Pipeline (wetSpring)

| Step | Traditional Tool | wetSpring Module | Validated |
|------|-----------------|-----------------|:---------:|
| FASTQ quality filtering | Trimmomatic / fastp | `bio::quality` | Yes |
| Paired-end merging | FLASH / PEAR | `bio::merge_pairs` | Yes |
| Dereplication | vsearch | `bio::derep` | Yes |
| Denoising (ASV) | DADA2 (R) | `bio::dada2` | Yes |
| Chimera detection | UCHIME / vsearch | `bio::chimera` | Yes |
| Taxonomy classification | naïve Bayes / BLAST | `bio::taxonomy` | Yes |
| Diversity indices | vegan (R) | `bio::diversity` | Yes |
| Beta diversity (UniFrac) | phyloseq (R) | `bio::unifrac` | Yes |
| Ordination (PCoA) | phyloseq (R) | `bio::pcoa` | Yes (GPU) |

**Total: 306 validation binaries, 5,707+ numerical checks, 63 papers reproduced.**

### Why This Matters for a Core Facility

1. **Reproducibility**: Same binary, same result, every time. No Python version drift.
   No R package conflicts. No "it worked on my machine."
2. **Speed**: GPU-accelerated spectral matching is 1,077× faster than CPU. Diversity
   calculations are GPU-native.
3. **Provenance**: Optional cryptographic signing on every result (BearDog Ed25519).
   Maps to ISO 17025 traceability requirements.
4. **No sysadmin**: One `cargo build` compiles everything. No Galaxy server to maintain.
   No conda environments to debug.

---

## For Graduate Students

### What You Get

A real science pipeline validated against published papers — not a toy project.
When you extend it with your own data, the infrastructure guarantees correctness.

### The K-Nome Approach

K-Nome (Knowledge-Numeric Observed & Mentored Evolutionary Programming) is how
this was built: one human with domain expertise mentoring AI (Cursor IDE) through
iterative cycles. The Rust compiler is the fitness function — code either compiles
and passes tests, or it doesn't.

**What this means for you**:
- You don't need to be a Rust expert. The compiler teaches you.
- Every module has tests that serve as executable documentation.
- The validation binaries are the ground truth — if your modification breaks
  a check, you know immediately.

### Student Project Ideas (Real Science, Not Homework)

| Project | Spring | What You'd Do | Data Source |
|---------|--------|--------------|-------------|
| Anderson eigensolve at scale | wetSpring + ICER | Run L=200 3D lattice on A100 GPUs | Computed |
| ADDRC compound triage | healthSpring | GPU Hill sweep on 8K compounds | ADDRC library |
| Soil microbiome classification | wetSpring | Real 16S through full pipeline | KBS LTER / Genomics Core |
| Population PK on real data | healthSpring | NLME on MIMIC-IV vancomycin TDM | PhysioNet |
| NPU edge deployment | toadStool | ESN classifier on BrainChip AKD1000 | Live hardware |
| Drug-disease NMF at scale | wetSpring Track 3 | NMF on ChEMBL 2M+ bioactivities | ChEMBL REST API |

Each of these is publishable. The spring's existing validation infrastructure
guarantees your results are correct.

---

## For Lab Technicians / Research Associates

### What You Need to Know

You don't need to write Rust. The validation binaries are pre-built executables.
Your workflow is:

1. **Prepare your data** (FASTQ, mzML, CSV — whatever your instrument produces)
2. **Run the relevant binary** (`cargo run --release --bin validate_*`)
3. **Check the output** (PASS/FAIL with numerical tolerances)

If something fails, the output tells you exactly which check failed and what
the expected vs actual values were.

### Common Lab Data Formats Supported

| Format | What It Is | wetSpring Module |
|--------|-----------|-----------------|
| FASTQ / FASTQ.gz | Sequencer reads | `io::fastq` (sovereign parser, handles gzip) |
| mzML | Mass spectrometry (open standard) | `io::mzml` (sovereign XML parser) |
| mzXML | Mass spectrometry (legacy open) | `io::mzxml` |
| JCAMP-DX | Spectroscopy (FTIR, NMR, UV-Vis) | `io::jcamp` |
| Newick | Phylogenetic trees | `bio::felsenstein` |
| WFDB (Format 212/16) | PhysioNet ECG/PPG waveforms | healthSpring `wfdb.rs` |
| CSV/TSV | Tabular data | Standard Rust `csv` crate |

---

## Troubleshooting

| Problem | Solution |
|---------|----------|
| `rustc not found` | Restart terminal after installing Rust |
| `cargo test` has compilation errors | Run `rustup update` to ensure Rust 1.87+ |
| GPU tests fail | Add `--features gpu` and ensure Vulkan drivers are installed |
| barraCuda not found | Clone barraCuda alongside the spring: `git clone git@github.com:ecoPrimals/barraCuda.git` in the same parent directory |
| Tests pass but I don't understand the output | Each `validate_*` binary prints human-readable pass/fail with tolerances |

---

## Hardware Requirements

| Tier | Hardware | What Works |
|------|----------|-----------|
| **Minimum** | Any x86_64 Linux/Mac with 4 GB RAM | All CPU tests and validations |
| **Recommended** | + any Vulkan-capable GPU (GTX 1060+) | CPU + GPU acceleration |
| **Optimal** | + RTX 3060 or better | Full GPU pipeline, 100K+ patient Monte Carlo |
| **NPU** | + BrainChip AKD1000 (PCIe) | Edge inference, reservoir computing |

The entire ecosystem was built on ~$15,000 of consumer hardware. No HPC required
for development or validation. ICER access expands scale, not capability.
