+++
title = "Grant Technical Appendix: Validation Evidence by Agency Program"
description = "Validation evidence by agency program — NIH, NSF, USDA, DOE, ARPA-H"
date = 2026-03-17
+++

# Grant Technical Appendix: Validation Evidence by Agency Program

**Audience:** Grant reviewers — NIH, NSF, USDA, DOE, ARPA-H  
**Purpose:** Quantitative evidence supporting capability claims, organized by funding agency's priorities  
**License:** CC-BY-SA 4.0  
**Last Updated:** March 17, 2026

---

## How to Use This Document

Every capability claim below references:
1. A specific public spring repository (verifiable)
2. A specific binary that produces explicit PASS/FAIL output
3. A specific experiment number with documented methodology
4. A published paper being reproduced (peer-reviewed ground truth)

Any reviewer can clone the repository, run the binary, and verify the claim.
Exit 0 means all checks pass. Exit 1 means something failed.

---

## NIH — Biomedical Science, Drug Discovery, Clinical Translation

### Priority Alignment

NIH NIGMS, NIAMS, NIDDK, and NCI all fund computational approaches to drug
discovery, microbiome science, pharmacometrics, and reproducible research.
ecoPrimals addresses the current reproducibility crisis in computational
biomedicine by producing signed, independently verifiable results.

### Validated Claims

**Drug Repurposing with Spatial Geometry (Novel Method)**

Anderson localization applied to cytokine signaling in skin tissue. Extends
the Fajgenbaum MATRIX framework (ARPA-H $48.3M) with a tissue geometry
dimension that standard pathway scoring lacks.

| Claim | Evidence | Verify |
|-------|----------|--------|
| 6 JAK/cytokine pathway drugs scored with Anderson geometry | healthSpring Exp090, neuralSpring nS-601–605 | `cargo run --bin validate_drug_discovery_pipeline` |
| Oclacitinib IC50 reproduced to < 1e-10 from Gonzales 2014 | healthSpring, 80 cross-language checks | `cargo test drug_discovery` |
| Lokivetmab PK decay reproduced exactly (R²=1.0) | neuralSpring, 3/3 dose-duration predictions | `cargo run --bin validate_pk_curves` |
| 3-compartment tissue lattice (immune + skin + neural) | wetSpring Exp273–286, 157/157 checks | `cargo run --bin validate_anderson_immunological` |
| Anderson-augmented MATRIX ranking, 329/329 PASS | Full three-tier validation | `cargo test --workspace` |

**Microbiome Science**

| Claim | Evidence | Verify |
|-------|----------|--------|
| 16S pipeline reproduces 4 public BioProjects | wetSpring Exp001–042, 3,100+ checks | `cargo run --bin validate_diversity` |
| DADA2-equivalent denoising in pure Rust | wetSpring, 42 experiments | `cargo run --bin validate_16s_pipeline` |
| GPU spectral matching 1,077× faster than CPU Python | wetSpring Exp034 | `cargo run --bin benchmark_gpu_vs_cpu` |
| Anderson disorder W quantified for diverse communities | wetSpring, W_c = 16.26 ± 0.95 (3,100+ checks) | `cargo run --bin validate_anderson_critical` |
| NCBI E-utilities sovereign pipeline (no institutional access) | wetSpring Exp170–195 | `cargo run --bin validate_ncbi_pipeline` |

**Pharmacometrics (Sovereign NONMEM/Monolix)**

| Claim | Evidence | Verify |
|-------|----------|--------|
| FOCE algorithm (NONMEM method) in pure Rust | healthSpring Track 5, Exp070–080 | `cargo run --bin validate_nlme` |
| SAEM algorithm (Monolix method) in pure Rust | healthSpring Track 5 | `cargo run --bin validate_saem` |
| Population PK Monte Carlo (10K patients) | healthSpring Exp054–056 | `cargo run --bin validate_population_pk` |
| PBPK 5-compartment model (Rust 84× faster than Python) | healthSpring Exp084 | `cargo run --bin benchmark_pbpk` |
| Pan-Tompkins ECG QRS detection | healthSpring Track 3, 12/12 checks | `cargo run --bin validate_ecg_pipeline` |

**Reproducibility Infrastructure**

Every computation:
- Signed with Ed25519 (BearDog primal)
- Content-addressed in BLAKE3 (NestGate primal)
- W3C PROV-O provenance record (sweetGrass primal)
- HIPAA alignment: data stays local, only provenance receipts are shared
- FDA 21 CFR Part 11 mapping: see `FOR_COMPLIANCE_AND_INSTITUTIONAL_REVIEW.md`

### Relevant NIH Programs

| Program | Alignment |
|---------|-----------|
| NIGMS R01 (Pharmacology/Toxicology) | Sovereign pharmacometrics, reproducible PK/PD |
| NCI R01 (Drug Discovery) | Anderson-augmented MATRIX, ADDRC HTS integration |
| NIAMS (Skin/Musculoskeletal) | Anderson-AD pipeline, Gonzales catalog |
| NLM (Data Science) | Reproducible biomedical computing, provenance infrastructure |
| BD2K/NIH Bridge2AI | Sovereign data infrastructure, signed science |
| ARPA-H | Drug repurposing (direct MATRIX extension) |

---

## NSF — Fundamental Science, Methods, and Computing

### Priority Alignment

NSF CISE, BIO, and PHY fund sovereign computing infrastructure, biological
discovery, and novel computational methods. The constrained evolution methodology
and Anderson framework extensions represent exactly the cross-disciplinary
fundamental research NSF values.

### Validated Claims

**Constrained Evolution Methodology**

| Claim | Evidence | Verify |
|-------|----------|--------|
| 15,334+ quantitative science checks across 7 domains | All 7 public spring repositories | `cargo test --workspace` in each |
| 100+ published papers reproduced | Spring catalog: hotSpring (25), wetSpring (63+), neuralSpring (27), etc. | See `SPRING_CATALOG.md` |
| Consumer GPU reproduces HPC results | hotSpring: deconfinement β_c=5.69 on RTX 3090 | `cargo run --bin validate_lattice_qcd_scan` |
| f64 GPU compute via WebGPU/Vulkan (bypasses CUDA throttle) | hotSpring DF64: 3.24 TFLOPS at 14-digit precision | `cargo run --bin benchmark_df64` |
| Cross-platform: same results on NVIDIA and AMD | airSpring: 20.6× CPU speedup, parity verified | `cargo run --bin benchmark_cross_vendor` |

**Anderson Framework (Novel Application)**

| Claim | Evidence | Verify |
|-------|----------|--------|
| Anderson 3D localization: W_c = 16.26 ± 0.95 | wetSpring Exp107–156, 3,100+ checks | `cargo run --bin validate_anderson_3d` |
| 2D always localizes regardless of W | wetSpring Exp140–143, numerical confirmation | `cargo run --bin validate_anderson_2d` |
| O₂-modulated Anderson W model (r=0.851) | wetSpring Exp356 (V110), 18/18 checks | `cargo run --bin validate_o2_anderson` |
| Anderson in soil (no-till mechanism) | wetSpring Track 4, airSpring, 321 checks | `cargo run --bin validate_notill_anderson` |
| Anderson in immunological tissue | wetSpring Exp270–286, 157/157 checks | `cargo run --bin validate_anderson_immunological` |
| Anderson in cytokine propagation cross-species | neuralSpring nS-601–605, 329/329 | `cargo run --bin validate_cytokine_anderson` |

**Sovereign GPU Computing**

| Claim | Evidence | Verify |
|-------|----------|--------|
| Pure Rust GPU (no CUDA), zero C dependencies | All springs: `cargo deny check` passes | `cargo deny check --workspace` |
| 806 WGSL f64 shaders in barraCuda | barraCuda primal (public) | github.com/ecoPrimals/barraCuda |
| Sovereign WGSL→native compiler (coralReef) | 46/46 shaders compile to SM70/SM86/RDNA2 | github.com/ecoPrimals/coralReef |
| NPU (AKD1000) live: 3 classifiers, 136 gen/sec | hotSpring Exp193–195, live hardware | `cargo run --bin validate_npu_live` |
| GPU-NPU-CPU parity (metalForge cross-substrate) | groundSpring, 395/395 checks + 140 metalForge | `cargo run --bin validate_cross_substrate` |

**Computational Biology**

| Claim | Evidence | Verify |
|-------|----------|--------|
| First consumer-GPU dynamical fermion QCD | hotSpring Exp024, 1,031+ trajectories | `cargo run --bin validate_dynamical_qcd` |
| Reservoir computing (ESN) from bingo boards | hotSpring/ToadStool Exp029, 5.3% LOO error | `cargo run --bin validate_nautilus` |
| Cross-domain fraud detection (game = science = medical) | ludoSpring Exp062–066, 193 checks | `cargo run --bin validate_provenance_pipeline` |

### Relevant NSF Programs

| Program | Alignment |
|---------|-----------|
| CISE OAC (Cyberinfrastructure) | ecoBin sovereign compute, NUCLEUS mesh, WebGPU |
| CISE SHF (Software and Hardware Foundations) | Constrained evolution methodology, Rust type-system constraints |
| BIO DBI (Biological Infrastructure) | Sovereign bioinformatics, wetSpring 16S, field genomics |
| PHY Computational Physics | hotSpring QCD, plasma physics, Anderson spectral |
| BIO MCB (Molecular & Cellular Biosciences) | Anderson-QS, microbiome science, LTEE extensions |
| CISE IIS (Information + Intelligent Systems) | K-Nome pedagogy, ML reproducibility |

---

## USDA — Agricultural Science, Food Security, Precision Agriculture

### Priority Alignment

USDA NIFA and ARS fund precision agriculture technology, soil health monitoring,
water management, and food production sustainability. airSpring directly addresses
the computational foundation these programs need.

### Validated Claims

**Precision Agriculture Computing**

| Claim | Evidence | Verify |
|-------|----------|--------|
| FAO-56 ET₀ (8 methods) in Rust, R²=0.967 on real data | airSpring Exp001–015, 918 station-days | `cargo run --bin validate_et0_all_methods` |
| 100-station Michigan Crop Water Atlas | airSpring Exp016–020, 100 stations, 30 years | `cargo run --bin validate_crop_water_atlas` |
| Richards PDE (soil water flow) GPU-accelerated | airSpring Exp021–025, GPU/CPU parity | `cargo run --bin validate_richards_pde` |
| Yield response (Stewart model) validated | airSpring, 24/24 GPU parity checks | `cargo run --bin validate_yield_response` |
| 13,000× Rust vs Python at atlas scale | airSpring Exp016, benchmark binary | `cargo run --bin benchmark_atlas_scale` |

**Soil Microbiome and No-Till**

| Claim | Evidence | Verify |
|-------|----------|--------|
| Anderson localization explains no-till soil health | wetSpring Track 4, airSpring, 321 checks | `cargo run --bin validate_notill_anderson` |
| Tillage = 3D→2D dimensional collapse in pore network | wetSpring, groundSpring theoretical + computation | `cargo run --bin validate_soil_dimension` |
| Precision microbiome design for tree crops | baseCamp Paper 03, wetSpring + airSpring | `cargo run --bin validate_rhizosphere_qs` |
| AKD1000 NPU agricultural IoT (coin-cell viable) | airSpring Exp028–029, 88 checks | `cargo run --bin validate_npu_agricultural` |

**Water Management**

| Claim | Evidence | Verify |
|-------|----------|--------|
| SCS-CN runoff (sovereign, no SWAT dependency) | airSpring, GPU-accelerated | `cargo run --bin validate_scs_runoff` |
| Green-Ampt infiltration | airSpring, CPU + GPU | `cargo run --bin validate_infiltration` |
| Biochar isotherm modeling (Freundlich/Langmuir) | airSpring Exp026, 12 isotherms | `cargo run --bin validate_isotherms` |
| Lysimeter water balance validated | airSpring Exp030, data vs model | `cargo run --bin validate_lysimeter` |

### Relevant USDA Programs

| Program | Alignment |
|---------|-----------|
| NIFA AFRI Sustainable Agriculture | No-till Anderson mechanism, soil microbiome design |
| NIFA AFRI Water | ET₀, water balance, Richards PDE, runoff |
| NIFA Precision Agriculture | NPU edge IoT, atlas-scale crop water |
| ARS Partnerships | Crop Water Atlas, real sensor data pipeline |
| NIFA Food Security | Yield response, crop stress, cover crop benefits |

---

## DOE — Energy, Materials, and Large-Scale Computation

### Priority Alignment

DOE BES, BER, ASCR, and NNSA fund computational plasma physics, lattice QCD,
nuclear structure, materials science, and HPC software. ecoPrimals hotSpring
addresses these directly with consumer-GPU demonstration that decentralizes
access to HPC-class science.

### Validated Claims

**Plasma Physics**

| Claim | Evidence | Verify |
|-------|----------|--------|
| Yukawa OCP molecular dynamics (N=10K, 80K steps) on consumer GPU | hotSpring Phase C, 9/9 cases, 0.000% energy drift | `cargo run --bin validate_yukawa_md` |
| $0.044 electricity cost for paper-parity long MD run | hotSpring Phase F benchmark | `cargo run --bin benchmark_md_cost` |
| WDM transport coefficients (Green-Kubo) on RTX 3090 | hotSpring Tier 2, 32+ checks | `cargo run --bin validate_wdm_transport` |
| DF64 (FP32 cores, 14-digit precision): 3.24 TFLOPS | hotSpring DF64 streaming benchmark | `cargo run --bin benchmark_df64_streaming` |
| Kokkos parity: 12.4× gap identified, DF64 exp blocker documented | hotSpring Exp053, 9/9 cases | `cargo run --bin validate_kokkos_parity` |

**Lattice QCD**

| Claim | Evidence | Verify |
|-------|----------|--------|
| First consumer-GPU dynamical fermion HMC | hotSpring Exp024, 1,031 trajectories, 17 β points | `cargo run --bin validate_dynamical_qcd` |
| Deconfinement transition β_c = 5.69 on consumer GPU | hotSpring Exp024, 32⁴ lattice | `cargo run --bin validate_qcd_scan` |
| SU(3) pure gauge + HMC: validated against published | hotSpring, Papers 19–45, 664+ checks | `cargo test lattice_qcd` |
| GPU-resident CG (15,360× readback reduction) | hotSpring Exp020 | `cargo run --bin validate_gpu_cg` |
| Anderson localization as CG convergence proxy | hotSpring Exp024, CG-disorder correlation | `cargo run --bin validate_anderson_qcd_proxy` |

**Nuclear Structure**

| Claim | Evidence | Verify |
|-------|----------|--------|
| Nuclear EOS (SEMF→HFB) — full AME2020 (2,042 nuclei) | hotSpring Phase D–E, 195/195 total | `cargo run --bin validate_nuclear_eos` |
| Hartree-Fock-Bogoliubov on consumer GPU | hotSpring Phase E, consumer RTX 3090 | `cargo run --bin validate_hfb` |

**Sovereign GPU Stack**

| Claim | Evidence | Verify |
|-------|----------|--------|
| WebGPU/WGSL replaces CUDA (zero C) | All ecoPrimals: `cargo deny check` | `cargo deny check --workspace` |
| Consumer f64 via Vulkan: bypasses CUDA 1:64 throttle | hotSpring DF64, 9.9× vs native f64 | `cargo run --bin benchmark_f64_discovery` |
| Cross-vendor: NVIDIA SM70–SM89, AMD RDNA2 validated | coralReef P10 Iter 52+, 46/46 | github.com/ecoPrimals/coralReef |
| VFIO PCIe device lifecycle (glowplug daemon) | coralReef glowplug, production-grade | See SOVEREIGN_COMPUTE_EVOLUTION.md |

### Relevant DOE Programs

| Program | Alignment |
|---------|-----------|
| BES (Materials Sciences) | WDM transport, plasma physics, lattice QCD |
| ASCR (Computational Science) | WebGPU sovereign stack, WGSL compiler |
| NP (Nuclear Physics) | Lattice QCD, nuclear EOS, HFB |
| FES (Fusion Energy) | WDM, plasma transport coefficients |
| NNSA (Stockpile Stewardship) | NES, EOS, transport at extreme conditions |

---

## Validation Summary Table

| Domain | Spring | Papers Reproduced | Checks | Status |
|--------|--------|:-----------------:|:------:|:------:|
| Microbiome / QS | wetSpring | 63+ | 5,707+ | All PASS |
| Precision Ag | airSpring | 22+ | 3,123+ | All PASS |
| ML / Reservoir | neuralSpring | 27 | 4,500+ | All PASS |
| Computational Physics | hotSpring | 25 | 664+ | All PASS |
| Uncertainty / Spectral | groundSpring | 10 | 535+ | All PASS |
| Human Health | healthSpring | 15+ | 474+ | All PASS |
| Game Science / HCI | ludoSpring | 13 HCI models | 1,692+ | All PASS |
| **Total** | | **175+** | **20,695+** | **All PASS** |

---

## Standard Verification Protocol

Any reviewer, without contacting the authors:

```bash
# Install Rust (5 minutes)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Clone any spring
git clone https://github.com/syntheticChemistry/wetSpring
cd wetSpring/barracuda

# Full test suite
cargo test --workspace
# Expected: 1,443+ tests, 0 failures

# Specific validation binary (exit 0 = all pass, exit 1 = failure)
cargo run --release --bin validate_diversity
cargo run --release --bin validate_anderson_3d
cargo run --release --bin validate_16s_pipeline

# License and dependency audit
cargo deny check
# Expected: no violations (AGPL-3.0-or-later, zero C deps)
```

No institution affiliation required. No data access required.
No API keys, no cloud accounts, no proprietary software.

---

*All spring repositories: github.com/ecoPrimals/*  
*Infrastructure: github.com/ecoPrimals/*  
*Full science catalog: see `CAPABILITY_PARITY_BRIEF.md` for domain-by-domain comparison vs proprietary tools*
