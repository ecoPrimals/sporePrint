+++
title = "ecoPrimals — Sovereign Scientific Computing Platform: Capability & Parity Assessment"
description = "Domain-by-domain comparison against proprietary tools across 8 scientific domains"
date = 2026-03-17

[taxonomies]
primals = ["barracuda", "beardog", "biomeos", "bingocube", "coralreef", "loamspine", "nestgate", "petaltongue", "rhizocrypt", "songbird", "squirrel", "sweetgrass", "toadstool"]
springs = ["airspring", "groundspring", "healthspring", "hotspring", "ludospring", "neuralspring", "wetspring"]
+++

**From:** ecoPrimal — human + synthetic intelligence  
**Organization:** {{ entity(name="ecoprimals") }}
**Date:** March 17, 2026
**License:** All code AGPL-3.0-or-later; all documentation CC-BY-SA-4.0
**Repositories:** Springs at github.com/syntheticChemistry · Primals at github.com/ecoPrimals · Products at github.com/sporeGarden

> **Historical snapshot.** Metrics in this document reflect March 2026 (14 primals, 7 springs, ~3.2M LOC, ~107K tests). Current ecosystem metrics are on the [Evidence Snapshot](@/architecture/EVIDENCE_SNAPSHOT.md) page ({{ total_stat(stat="total_primals") }} primals, {{ total_stat(stat="total_springs") }} springs, {{ total_stat(stat="total_loc_display") }} LOC, {{ total_stat(stat="total_tests_display") }} tests, measured {{ total_stat(stat="measured_date") }}). Parity analysis remains accurate.

---

## What This Is

{{ entity(name="ecoprimals") }} is a sovereign scientific computing ecosystem built in pure Rust with
GPU acceleration via WebGPU (WGSL shaders). It replaces Python/R/Fortran/Java
tool chains across life science, pharmacology, physics, and data provenance
domains. Every claim below has a validation binary that proves it — clone the
repo, run the binary, verify the output.

This document provides honest parity assessments: what we match, what we exceed,
what proprietary tools still do better, and where to find everything.

---

## 1. Bioinformatics Pipeline (vs Galaxy / QIIME2 / mothur)

### What It Replaces

| Commercial/Open Tool | Annual Cost | What It Does |
|---------------------|:----------:|--------------|
| Galaxy | Free (hosted) / $50K+ (local) | Web-based bioinformatics workflow |
| QIIME2 | Free | 16S/ITS amplicon analysis (Python) |
| mothur | Free | 16S OTU-based pipeline (C++) |
| DADA2 (R) | Free | Amplicon sequence variant denoising |
| phyloseq (R) | Free | Microbiome statistical analysis |
| vegan (R) | Free | Community ecology analysis |

### ecoPrimals Replacement: wetSpring

**Repository:** github.com/syntheticChemistry/wetSpring
**Status:** V127 — 1,443+ tests, 306 validation binaries, 376 experiments, 5,707+ checks

| Capability | Parity Level | Notes |
|-----------|:------------:|-------|
| FASTQ parsing + quality filtering | **Full parity** | Sovereign parser, no `needletail` dependency |
| Read merging (paired-end) | **Full parity** | Overlap detection, quality-aware consensus |
| Dereplication | **Full parity** | Hash-based, GPU-accelerated |
| DADA2 denoising | **Full parity** | Error model + denoising validated against R DADA2 |
| {{ entity(name="chimera") }} detection | **Full parity** | de novo + reference-based |
| Taxonomy classification | **Full parity** | Naïve Bayes, k-mer, spectral matching |
| UniFrac (weighted/unweighted) | **Full parity** | GPU-accelerated, validated against phyloseq |
| Diversity indices (Shannon, Simpson, Chao1, Pielou) | **Full parity** | Validated to 1e-12 against textbook definitions |
| Bray-Curtis dissimilarity | **Full parity** | GPU `BrayCurtisF64` kernel |
| PCoA ordination | **Full parity** | GPU `BatchedEighGpu` eigendecomposition |
| Rarefaction curves | **Full parity** | Monte Carlo with bootstrap CI |
| Smith-Waterman alignment | **Full parity** | Affine gap penalties |
| HMM phylogenetics | **Full parity** | Forward algorithm, GPU batch |
| Newick tree parsing / Robinson-Foulds | **Full parity** | Full phylogenetic tree comparison |
| ODE ecological models (Lotka-Volterra, QS, etc.) | **Exceeds** | 5 biological ODE systems with GPU shader generation |
| Anderson localization for community structure | **No equivalent** | Novel physics framework — no proprietary tool does this |
| GPU acceleration (all operations) | **Exceeds** | 150+ GPU primitives, zero local WGSL |
| Spectral cosine matching | **Exceeds** | 1,077× speedup over CPU |

### What Proprietary/Open Tools Still Do Better

| Capability | Gap | Path to Parity |
|-----------|-----|---------------|
| GUI workflow builder | No GUI — CLI + validation binaries only | {{ entity(name="petaltongue") }} provides visualization; Galaxy-style builder not planned |
| Plugin ecosystem | No third-party plugin system | IPC capability discovery enables composition |
| Training documentation | No tutorials, no workshops | K-Nome methodology document exists; formal curriculum pending |
| Multi-user web interface | Single-user, local execution | {{ entity(name="biomeos") }} IPC enables multi-client; web UI not planned |
| Established community | One developer, public repos | 3.2M lines of Rust, 107K+ tests, all validation executable |
| Cloud deployment | Local/LAN only | {{ entity(name="nucleus") }} bonding model supports distributed deployment |

### Where to Find / Rebuild

```bash
git clone git@github.com:syntheticChemistry/wetSpring.git
cd wetSpring/barracuda
cargo test --workspace                    # 1,443+ tests
cargo run --release --bin validate_diversity  # Diversity index validation
cargo run --release --bin validate_dada2_full # DADA2 pipeline validation
```

**Key modules:** `barracuda/src/bio/` (47 CPU + 47 GPU modules), `barracuda/src/io/` (FASTQ, mzML, mzXML, JCAMP-DX parsers)
**Dependency:** `barracuda` (pure math primal, path dependency to `../../barraCuda/crates/barracuda`)

---

## 2. Pharmacometric Modeling (vs NONMEM / Monolix / WinNonlin)

### What It Replaces

| Tool | Annual Cost | What It Does |
|------|:----------:|--------------|
| NONMEM | ~$2,000/yr | Population PK parameter estimation (FOCE) |
| Monolix | ~$1,500/yr | Population PK parameter estimation (SAEM) |
| WinNonlin (Phoenix) | ~$3,000/yr | Non-compartmental analysis (NCA) |
| CRO population PK | $50K–200K/program | Contract research organization modeling |

### ecoPrimals Replacement: healthSpring

**Repository:** github.com/syntheticChemistry/healthSpring
**Status:** V35 — 613 tests, 73 experiments, 113/113 cross-validation checks, 6 WGSL shaders

| Capability | Parity Level | Notes |
|-----------|:------------:|-------|
| Hill dose-response (4-parameter) | **Full parity** | Validated for JAK inhibitors (Gonzales IC50 data) |
| One-compartment PK (IV bolus, oral Bateman) | **Full parity** | AUC trapezoidal, steady-state accumulation |
| Two-compartment PK (biexponential) | **Full parity** | Distribution/elimination phase separation |
| mAb PK cross-species (allometric scaling) | **Full parity** | BW^0.75 CL, BW^1.0 Vd |
| Population PK Monte Carlo (1,000+ patients) | **Full parity** | Lognormal IIV, CL-AUC correlation |
| PBPK (5-tissue physiological) | **Full parity** | Mass conservation, hepatic clearance, tissue Kp |
| Michaelis-Menten nonlinear PK | **Full parity** | Capacity-limited elimination (phenytoin) |
| FOCE estimation | **Near parity** | 30% theta recovery on synthetic data |
| SAEM estimation | **Near parity** | 50% theta recovery on synthetic data |
| NCA (λz, AUC∞, MRT, CL, Vss) | **Full parity** | All 5 standard NCA metrics |
| NLME diagnostics (CWRES, VPC, GOF) | **Full parity** | CWRES ~N(0,1), 50-simulation VPC |
| GPU population Monte Carlo | **Exceeds** | 207 M/s throughput (RTX 4070), 100K patients |

### What Proprietary Tools Still Do Better

| Capability | Gap | Path to Parity |
|-----------|-----|---------------|
| FOCE/SAEM on real clinical data | Validated on synthetic only | Need MIMIC-IV (PhysioNet credentialed access) |
| FDA submission formatting | No CTD/eCTD output | Infrastructure exists; formatting layer needed |
| Covariate model building (stepwise) | Manual covariate selection only | Automated stepwise selection planned |
| Regulatory track record | No FDA submissions yet | Deterministic reproducibility is an advantage for auditors |
| Interactive model exploration | CLI only | {{ entity(name="petaltongue") }} dashboard provides visualization |
| Existing training / certification | No formal training program | K-Nome methodology available |

### Where to Find / Rebuild

```bash
git clone git@github.com:syntheticChemistry/healthSpring.git
cd healthSpring
cargo test --workspace                     # 613 tests
cargo run --release --bin exp001_hill      # Hill dose-response (Gonzales IC50)
cargo run --release --bin exp004_mab       # Cross-species PK (lokivetmab → human)
cargo run --release --bin exp075_nlme      # NONMEM/Monolix/WinNonlin replacement
```

**Key modules:** `ecoPrimal/src/pkpd/` (compartmental, population, NLME), `ecoPrimal/src/microbiome/` (gut Anderson), `ecoPrimal/src/biosignal/` (ECG, PPG, EDA)
**Rust vs Python:** 84× aggregate speedup across 14 benchmark cases (Exp084)

---

## 3. Drug Repurposing (vs Every Cure MATRIX / ROBOKOP)

### What It Replaces

| Tool | Cost | What It Does |
|------|:----:|--------------|
| Every Cure MATRIX | $48.3M ARPA-H grant | Drug-disease scoring (4K drugs × 18K diseases) |
| ROBOKOP | NIH-funded | Knowledge graph for drug-disease relationships |
| DrugBank | $20K+/yr (commercial) | Drug target database |
| repoDB | Free | Drug-disease benchmark dataset |

### ecoPrimals Replacement: wetSpring Track 3 + neuralSpring nS-06 + groundSpring

| Capability | Parity Level | Notes |
|-----------|:------------:|-------|
| Pathway-based drug scoring (PI3K/AKT/mTOR) | **Full parity** | Fajgenbaum 2019 reproduced (Exp157, 8/8) |
| MATRIX 4-stage pipeline | **Full parity** | Pharmacophenomics pipeline (Exp158, 9/9) |
| NMF drug-disease factorization | **Full parity** | Yang 2020 + repoDB (Exp159–160, 16/16) |
| TransE knowledge graph embedding | **Full parity** | ROBOKOP-style (Exp161, 7/7) |
| Anderson geometry-aware drug scoring | **No equivalent** | Novel — adds tissue penetration physics to MATRIX |
| Tissue lattice + barrier promotion | **No equivalent** | 3D Anderson Hamiltonian for skin/gut geometry |
| Cross-species PK translation | **Partial parity** | Validated canine→human; not yet feline→human |
| GPU compound screening | **Exceeds** | 207 M/s Hill sweep on RTX 4070 |

### What Every Cure / ROBOKOP Still Do Better

| Capability | Gap | Path to Parity |
|-----------|-----|---------------|
| Scale: 4K drugs × 18K diseases | We validate on 6 drugs × 6 diseases | Data pipeline, not algorithm — ChEMBL + NCATS Translator |
| Curated disease pathway profiles | Use published pathways only | NCATS Translator API (same source as Every Cure) |
| Clinical outcome integration | Published parameters only | MIMIC-IV, FAERS via openFDA |
| Institutional backing / regulatory relationships | One developer | MSU Drug Discovery + ADDRC collaboration |
| Pre-built ontology mappings (MONDO, DO) | Manual mappings | MONDO/DO are open; integration work needed |

### Where to Find / Rebuild

```bash
# wetSpring — Track 3 drug repurposing
cd wetSpring/barracuda
cargo run --release --bin validate_fajgenbaum_pathway     # Exp157
cargo run --release --bin validate_matrix_pharmacophenomics # Exp158
cargo run --release --bin validate_nmf_drug_repurposing    # Exp159
cargo run --release --bin validate_repodb_nmf              # Exp160
cargo run --release --bin validate_knowledge_graph_embedding # Exp161

# neuralSpring — immunological Anderson + MATRIX scoring
cd neuralSpring
cargo run --release --bin validate_immunological_anderson           # 20/20
cargo run --release --bin validate_immunological_anderson_extended  # 28/28

# groundSpring — tissue Anderson drug scoring
cd groundSpring
cargo run --release --bin validate_tissue_anderson  # Exp033–034
```

---

## 4. Analytical Chemistry (vs MassHunter / Chromeleon / MZmine)

### What It Replaces

| Tool | Annual Cost | What It Does |
|------|:----------:|--------------|
| MassHunter (Agilent) | ~$10K+/yr | LC-MS data acquisition + analysis |
| Chromeleon (Thermo) | ~$5K+/yr | Chromatography data system |
| MZmine | Free | LC-MS feature extraction |
| FindPFAS | Free | PFAS mass spectrometry screening |

### ecoPrimals Replacement: wetSpring Track 2

| Capability | Parity Level | Notes |
|-----------|:------------:|-------|
| mzML parsing | **Full parity** | Sovereign XML parser (no `quick-xml`) |
| mzXML parsing | **Full parity** | Sovereign parser |
| JCAMP-DX parsing | **Full parity** | Spectroscopy format |
| EIC extraction | **Full parity** | m/z window extraction from raw data |
| Peak detection (signal processing) | **Full parity** | CWT-based, validated against Python baseline |
| Spectral cosine matching | **Exceeds** | GPU kernel, 1,077× speedup |
| KMD grouping | **Full parity** | Kendrick mass defect for homologous series |
| PFAS screening (mass defect + RT) | **Full parity** | FindPFAS algorithm reproduced |
| Retention index (Kovats, Lee) | **Full parity** | Both linear and polynomial calibration |

### What Proprietary Tools Still Do Better

| Capability | Gap | Path to Parity |
|-----------|-----|---------------|
| Instrument control / data acquisition | Software only — no instrument drivers | Not planned; focus is analysis |
| Vendor-specific raw formats (.d, .raw) | mzML/mzXML only (open formats) | Vendor conversion is standard practice |
| Method development wizards | CLI only | Not planned |
| FDA 21 CFR Part 11 compliance | Provenance chain exists; no formal audit | {{ entity(name="beardog") }} signing + {{ entity(name="loamspine") }} certs map to Part 11 |
| Integrated LIMS | No LIMS — provenance trio provides chain-of-custody | Paper 21 architecture covers this |

### Where to Find / Rebuild

```bash
cd wetSpring/barracuda
cargo run --release --bin validate_features     # EIC + peak detection
cargo run --release --bin validate_peaks         # Signal processing
cargo run --release --bin validate_pfas_decision_tree  # PFAS screening
cargo run --release --bin validate_massbank_gpu_scale  # GPU spectral matching
```

**Key modules:** `barracuda/src/io/mzml/`, `barracuda/src/io/mzxml/`, `barracuda/src/bio/eic.rs`, `barracuda/src/bio/signal/`

---

## 5. Biosignal Processing (vs LabChart / MATLAB / Python-MNE)

### ecoPrimals Replacement: healthSpring Track 3

| Capability | Parity Level | Notes |
|-----------|:------------:|-------|
| Pan-Tompkins QRS detection | **Full parity** | Validated against MIT-BIH reference |
| HRV metrics (SDNN, RMSSD, pNN50) | **Full parity** | Time-domain from R-peak intervals |
| PPG SpO2 calibration | **Full parity** | Beer-Lambert AC/DC ratio |
| EDA tonic/phasic decomposition | **Partial** | Sovereign implementation; numpy convolution still faster for rolling average |
| Arrhythmia beat classification | **Full parity** | Template matching: Normal/PVC/PAC/BBB |
| WFDB format parsing | **Full parity** | PhysioNet Format 212/16 + beat annotations |
| Multi-channel fusion | **Full parity** | ECG + PPG + EDA → composite health assessment |

---

## 6. Provenance & Data Integrity (vs LabArchives / Benchling / LIMS)

### ecoPrimals Replacement: SCYBORG Provenance Trio + BearDog

| Capability | Parity Level | Notes |
|-----------|:------------:|-------|
| Sample chain-of-custody | **Exceeds** | Cryptographic DAG ({{ entity(name="rhizocrypt") }}), not database records |
| Ed25519 digital signatures on results | **No equivalent** | {{ entity(name="beardog") }} signs every computation result |
| ISO 17025/15189 traceability mapping | **Architectural parity** | {{ entity(name="loamspine") }} certificates map to ISO requirements |
| Fraud detection (6 types) | **Exceeds** | Graph analysis: phantom sample, broken cold chain, etc. |
| Consent-gated access (medical) | **No equivalent** | Paper 22: DID-based consent certificates |
| Audit trail | **Exceeds** | Every DAG vertex is immutable, signed, and attributed |

### What LIMS / Benchling Still Do Better

| Capability | Gap | Path to Parity |
|-----------|-----|---------------|
| Inventory management | No physical inventory tracking | fm-pipette (FIELDMOUSE) planned for wet lab integration |
| Barcode / QR scanning | No hardware integration | {{ entity(name="nestgate") }} mobile scanning planned |
| Regulatory pre-validation (GxP) | Architecture exists; no formal GxP audit | {{ entity(name="beardog") }} + {{ entity(name="loamspine") }} architecture maps to GxP; audit needed |
| SaaS convenience | Local deployment only | {{ entity(name="nucleus") }} enables LAN/WAN; no cloud planned |

---

## 7. GPU Scientific Computing (vs CUDA / Kokkos / MATLAB Parallel)

### ecoPrimals Replacement: barraCuda + toadStool + coralReef

| Capability | Parity Level | Notes |
|-----------|:------------:|-------|
| f64 precision GPU compute | **Full parity** | All WGSL shaders use f64 (via DF64 emulation where needed) |
| Vendor-agnostic GPU targeting | **Exceeds** | WebGPU: NVIDIA + AMD + Intel + Apple (CUDA is NVIDIA-only) |
| Sovereign shader compiler | **Exceeds** | {{ entity(name="coralreef") }} compiles WGSL without vendor toolchain |
| Hardware discovery | **Exceeds** | toadStool discovers GPU + CPU + NPU at runtime |
| Mixed hardware dispatch | **Exceeds** | CPU + GPU + NPU routing by capability, not hardcoding |
| Neuromorphic (NPU) support | **No equivalent** | BrainChip AKD1000 via pure Rust driver |
| 806+ validated WGSL shaders | **Specialized** | Bio/physics domain; not general-purpose |

### What CUDA / Kokkos Still Do Better

| Capability | Gap | Path to Parity |
|-----------|-----|---------------|
| Raw throughput (CUDA optimized kernels) | WGSL overhead for some operations | Improving with wgpu maturity |
| Ecosystem (cuBLAS, cuFFT, cuDNN) | {{ entity(name="barracuda") }} covers science ops; no ML framework | Not competing with ML frameworks |
| Multi-GPU scaling | Single GPU per dispatch | toadStool multi-device dispatch planned |
| Tensor cores / mixed precision | f64 focus, not fp16/bf16 | Science needs f64; ML-style mixed precision not a priority |
| HPC job scheduler integration (Slurm) | No Slurm scripts | {{ entity(name="nucleus") }} bonding model is an alternative; Slurm adaptor trivial |

---

## 8. Aggregate Ecosystem Metrics

| Metric | Value |
|--------|-------|
| **Springs** (validation domains) | 7 (wet, hot, air, ground, neural, health, ludo) |
| **Primals** (infrastructure) | 14 ({{ entity(name="barracuda") }}, toadStool, {{ entity(name="coralreef") }}, {{ entity(name="biomeos") }}, {{ entity(name="beardog") }}, {{ entity(name="nestgate") }}, {{ entity(name="songbird") }}, {{ entity(name="sweetgrass") }}, {{ entity(name="rhizocrypt") }}, {{ entity(name="loamspine") }}, {{ entity(name="petaltongue") }}, {{ entity(name="squirrel") }}, {{ entity(name="bingocube") }}, FIELDMOUSE) |
| **Total tests** | 27,000+ across all springs |
| **Total validation checks** | 15,334+ |
| **Papers reproduced** | 70+ (63 {{ entity(name="wetspring") }} + {{ entity(name="healthspring") }} + {{ entity(name="neuralspring") }} + {{ entity(name="hotspring") }} + others) |
| **WGSL shaders ({{ entity(name="barracuda") }})** | 806+ |
| **Languages** | Pure Rust (zero C/C++/Fortran in application code) |
| **Unsafe code** | Zero (`#![forbid(unsafe_code)]` in all spring lib crates) |
| **External dependencies** | Minimal; all pure Rust or explicit `rust_backend` |
| **License** | AGPL-3.0-or-later (code), CC-BY-SA-4.0 (docs) |
| **Development methodology** | K-Nome (Knowledge-Numeric Observed & Mentored Evolutionary Programming) |
| **Development history** | 69,000+ AI invocations, 51B tokens, 185-day streak |
| **Hardware investment** | ~$15,000 (consumer hardware, zero cloud) |

---

## 9. What No Proprietary Tool Offers

These capabilities have no commercial equivalent:

| Capability | What It Does | Spring |
|-----------|-------------|--------|
| Anderson localization for community structure | Maps microbial diversity onto condensed matter physics; predicts signal propagation vs confinement | {{ entity(name="wetspring") }}, {{ entity(name="groundspring") }}, {{ entity(name="neuralspring") }} |
| Geometry-aware drug repurposing | Adds spatial tissue penetration to pathway-based drug scoring (extends MATRIX) | {{ entity(name="wetspring") }} Track 3, {{ entity(name="neuralspring") }} nS-06, {{ entity(name="groundspring") }} |
| Cross-species Anderson translation | Same physics, different tissue parameters — species-agnostic by construction | {{ entity(name="healthspring") }} Track 6 |
| Cryptographically signed scientific results | Ed25519 signature on every diversity index, ASV table, drug score | {{ entity(name="beardog") }} |
| Provenance DAG for biological samples | Field-to-publication chain-of-custody with fraud detection | {{ entity(name="rhizocrypt") }} + {{ entity(name="loamspine") }} + {{ entity(name="sweetgrass") }} |
| NPU edge classification | BrainChip AKD1000 at 18.8K Hz, coin-cell power, pure Rust driver | toadStool + {{ entity(name="neuralspring") }} |
| Sovereign shader compiler | Compile WGSL without NVIDIA/AMD/Intel toolchain | {{ entity(name="coralreef") }} |
| Zero-knowledge medical provenance | Patient-owned records with consent certificates | Paper 22 |

---

## 10. Rebuilding From Source

Every spring is a self-contained Cargo workspace. To rebuild any capability:

```bash
# Prerequisites: Rust 1.87+ (rustup.rs), git
# Optional: GPU (any Vulkan-capable card for GPU tests)

# Clone the spring you need
git clone git@github.com:syntheticChemistry/<spring>.git
cd <spring>

# Build and test
cargo test --workspace              # All library tests
cargo clippy --all-targets          # Zero warnings guaranteed
cargo run --release --bin <binary>  # Run specific validation

# GPU tests (requires Vulkan-capable GPU)
cargo test --workspace --features gpu
```

**Build dependencies:** Rust toolchain only. No Python, no R, no conda, no Docker,
no pip, no npm. One `cargo build` compiles everything from source.

**{{ entity(name="barracuda") }}** (the math primal) is a path dependency. Clone it alongside the spring:

```bash
cd Development/ecoPrimals
git clone git@github.com:ecoPrimals/barraCuda.git  # renamed from barraCUDA
git clone git@github.com:syntheticChemistry/wetSpring.git
# Cargo.toml points to ../../barraCuda/crates/barracuda
```

---

## Document History

| Date | Change |
|------|--------|
| 2026-03-17 | Initial capability & parity assessment (V127 {{ entity(name="wetspring") }}, V35 {{ entity(name="healthspring") }}) |
