+++
title = "ecoPrimals Spring Catalog: Status, Science, and Evolution"
description = "All 7 springs — checks, papers reproduced, cross-spring data flow"
date = 2026-03-17
+++

# ecoPrimals Spring Catalog: Status, Science, and Evolution

**Status**: Working paper
**Lineage**: Science validation companion to `PRIMAL_CATALOG.md`
**Last Updated**: March 1, 2026
**License**: AGPL-3.0 — All springs are aggressively open science

---

## Abstract

This document catalogs every spring in the ecoPrimals ecosystem as of February 2026. Where the Primal Catalog (§`PRIMAL_CATALOG.md`) documents the Rust infrastructure that was built, this catalog documents the **scientific validation** that proves the infrastructure computes real physics, biology, chemistry, and mathematics correctly.

The springs exist because BarraCuda's claim — "Pure Rust GPU compute can replace the Python scientific stack" — requires evidence from every scientific domain the ecosystem intends to serve. Each spring takes published, peer-reviewed work and asks: can we reproduce it? First in Python (the original tool). Then in Rust. Then on GPU. If the answers are yes, the science is validated and the BarraCuda kernel is proven.

The springs are organized into two tiers:

- **Established Springs** (§1): Complete Phase 0 validation with passing quantitative checks. Each has Python baselines, whitePaper documentation, specs directories, and either Rust ports or GPU validation. These springs have been handed off to sub-teams for continued evolution.

- **The Spring Network** (§2): How springs connect to each other, to the primals, and to the faculty network that grounds them in published science.

---

## 1. The Springs

---

### 1.1 hotSpring — Computational Plasma Physics, Lattice QCD, Spectral Theory

**Domain**: Dense plasmas, nuclear structure, molecular dynamics, lattice QCD, spectral theory, neuromorphic computing
**Grade**: Most mature spring — Phase A through F complete, 21 experiments, 39/39 validation suites
**Checks**: 197+ quantitative checks pass (~697 tests, 78 binaries, 62 WGSL shaders)
**Faculty**: Michael Murillo (CMSE, MSU — MSDS professor), Alexei Bazavov (CMSE + Physics, MSU — master's professor), Ilya Kachkovskiy (Math, MSU — spectral theory), Rika Anderson (Biology, Carleton — *Sulfolobus* in Yellowstone hot springs, Taq corollary)
**Repository**: git@github.com:syntheticChemistry/hotSpring.git
**License**: AGPL-3.0

**What it validates**: hotSpring proves BarraCuda can do first-principles computational physics on consumer hardware. Sarkas Yukawa MD runs at paper parity (N=10,000, 80k production steps) on a $600 RTX 4070 for $0.044 in electricity. The full AME2020 nuclear dataset (2,042 nuclei, 39x the published paper) runs on a single consumer GPU. Lattice QCD production β-scans (32⁴, 12 temperatures) resolve the deconfinement transition on a $500 RTX 3090 for $0.58. A $300 Akida NPU runs ESN inference at 2.8μs/step — 1000× faster than GPU for streaming workloads. GPU ESN dispatch via WGSL crosses over CPU at reservoir size ≈512 (8.2× faster at 1024).

**Phase inventory**:

| Phase | What | Checks | Key Result |
|-------|------|--------|------------|
| A (Python control) | Sarkas MD, TTM, surrogates, nuclear EOS | 86/86 | 5 silent upstream bugs found and fixed |
| B (BarraCuda GPU) | Nuclear EOS L1/L2 | Validated | 478× speedup, 44.8× energy reduction |
| C (GPU MD) | Yukawa MD N=2,000 | 9/9 | 0.000% energy drift, 149-259 steps/s |
| D (f64 + scaling) | N=10,000 native WGSL | Validated | 5.3 min per case, native builtins |
| E (Paper parity) | N=10,000, 80k steps | 9/9 | 3.66 hours total, $0.044 electricity |
| F (Full nuclear EOS) | 2,042 nuclei AME2020 | 9/9 | Consumer GPU does nuclear structure |
| Lattice QCD | SU(3) HMC + dynamical fermion + GPU streaming | 39/39 suites | 32⁴ β-scan, χ=40.1 at β=5.69 |
| Spectral | Anderson, Hofstadter, Lanczos | 45/45 | GPU SpMV + Lanczos validated |
| NPU | metalForge cross-substrate ESN + characterization | 48/48 | 10 SDK assumptions overturned, 2.8μs streaming |
| DF64 | FP32-core double precision | Validated | 3.24 TFLOPS, 9.9× native f64 throughput |

**Papers reproduced**: Sarkas Yukawa OCP, TTM, Diaw et al. (2024) surrogate learning, SEMF→HFB nuclear EOS on full AME2020, Stanton-Murillo transport (Green-Kubo), Murillo-Weisheit screened Coulomb (Sturm bisection), HotQCD EOS tables, SU(3) pure gauge Wilson action, dynamical fermion QCD (pseudofermion HMC), Abelian Higgs U(1), Anderson localization (1D/2D/3D), Hofstadter butterfly, Kachkovskiy spectral theory.

**Papers queued**: muon g-2 HVP (2025), WDM & ignition (Papers 32-42). See `hotSpring/specs/PAPER_REVIEW_QUEUE.md`.

**BarraCuda kernels validated**: Yukawa force, Velocity Verlet, cell-list, thermostats, GEMM, BatchedEighGpu, FusedMapReduce, PPPM Ewald, SSF, SU(3) HMC (gauge force, plaquette, KE — all DF64), staggered Dirac, CG solver (GPU-resident), complex f64, Lanczos, SpMV, ESN reservoir update + readout.

**BarraCuda gaps closed** (since Feb 20): ~~FFT~~ (done), ~~complex f64~~ (done), ~~SU(3) matrix ops~~ (done), ~~Lanczos eigensolve + SpMV~~ (done). GPU-resident CG reduces readback by 15,360×. DF64 core streaming delivers 14-digit precision on FP32 cores. Bidirectional streaming pipeline dispatches 90%+ to GPU with async readback.

**metalForge (NPU + cross-substrate)**: 21 experiments including NPU characterization campaign (6 pipeline placements, Akida feedback report drafted), cross-substrate ESN comparison (CPU/GPU/NPU), streaming pipeline topology with substrate routing. NPU achieves 9,017× less energy for transport predictions. GPU can function as ESN reservoir (crossover at RS≈512). See `hotSpring/metalForge/` and `hotSpring/whitePaper/baseCamp/neuromorphic_silicon.md`.

**Participates in**: ToadStool validation (primary GPU science driver), gen3 constrained evolution evidence, metalForge hardware exploration.

---

### 1.2 airSpring — Precision Agriculture & Irrigation

**Domain**: Evapotranspiration (8 methods), soil moisture sensing, IoT irrigation scheduling, Richards PDE, coupled hydrology, yield response, ecological diversity, immunological Anderson coupling
**Grade**: Full Python→Rust→GPU→metalForge→NUCLEUS pipeline complete
**Checks**: 1,237/1,237 Python + 827 lib + 186 forge tests (27 GPU fail: upstream wgpu 28) + 381/381 validation + 146/146 evolution + 78 experiments
**Faculty**: Younsuk Dong (BAE, MSU — new lab 2026)
**Repository**: git@github.com:syntheticChemistry/airSpring.git
**License**: AGPL-3.0-or-later

**What it validates**: airSpring proves BarraCuda can replace the Python/Excel toolchain for precision agriculture at every stage — from paper reproduction through GPU-accelerated sovereign computation on consumer hardware. 57 papers reproduced with full provenance. FAO-56 ET₀ computed in Rust matches Python to 1e-5 tolerance across 75 cross-validated values. Real data from 100 Michigan stations (15,300 station-days) achieves R²=0.97. 19.8× geometric mean Rust speedup over Python (24 algorithms), 13,000× atlas-scale. 25 Tier A GPU modules + 6 f64-canonical local GPU ops + fused Welford (hotSpring S58) + fused Pearson (neuralSpring S69). Cross-spring shader provenance: hotSpring (precision/DF64), wetSpring (bio/diversity), neuralSpring (ML/stats), groundSpring (MC/uncertainty). NUCLEUS primal with 30 science capabilities.

**Phase inventory**:

| Phase | What | Checks | Key Result |
|-------|------|--------|------------|
| 0 (Python control) | 57 papers: FAO-56, soil, IoT, WB, dual Kc, Richards, biochar, yield, 8 ET₀, SCS-CN, Green-Ampt, VG, Anderson | 1,237/1,237 | Paper examples matched exactly |
| 0+ (Real data) | 100 Michigan stations, 15,300 station-days | R²=0.97 | Open-Meteo ERA5 validates open data approach |
| 1 (Rust) | 86 binaries, 827 lib + 62 forge tests | 827+62 | 19 eco modules + 25 GPU orchestrators (barraCuda 0.3.3, wgpu 28) |
| 2 (Cross-validation) | Python↔Rust value comparison | 75/75 | All values match within 1e-5 |
| 3 (GPU live) | Titan V + RTX 4070 dispatch | 78/78 + 46/46 | 0.04% seasonal parity, pure GPU end-to-end |
| 3.4 (Local GPU) | 6 f64-canonical ops via compile_shader_universal | 6 ops | SCS-CN, Stewart, Makkink, Turc, Hamon, Blaney-Criddle |
| 3.5+ (NPU/metalForge/NUCLEUS) | AKD1000, 27 workloads, 30 capabilities | 95+66+35 | Full cross-substrate pipeline |

**Papers reproduced**: 57 papers including Allen et al. (1998) FAO-56, Dong et al. (2020/2024), Kumari et al. (2025), coupled SCS-CN + Green-Ampt, Van Genuchten inverse, 8 ET₀ intercomparison.

**BarraCuda primitives consumed**: 42 touchpoints (14 batched_elementwise ops, 11 dedicated GPU shaders, 8 CPU primitives, 6 local f64-canonical ops, 3 pipeline stages). Zero local math duplication.

**BarraCuda contributions**: 3 upstream bug fixes (TS-001/003/004), Richards PDE absorbed (S40), stats metrics absorbed (S64), 6 f64-canonical local ops pending absorption.

**Participates in**: ToadStool validation (Rust science crate), Penny Irrigation (real-world application target), NUCLEUS (30 ecology capabilities), metalForge (27 workloads), bingoCube/nautilus (evolutionary reservoir).

---

### 1.3 wetSpring — Life Science & Analytical Chemistry

**Domain**: 16S metagenomics, LC-MS feature extraction, PFAS screening, microbial ecology
**Grade**: V127 — 47 CPU + 47 GPU bio modules, 1 runtime dependency (flate2)
**Checks**: 5,707+ validation checks across 376 experiments. 1,443+ Rust tests (0 failures), 214 named tolerances, 354 binaries (306 validate + 23 benchmark + 25 other). 63/63 papers reproduced, 46 at full CPU+GPU+metalForge. barraCuda v0.3.5 (150+ primitives consumed). IPC resilience (`RetryPolicy` + `CircuitBreaker`), 4-format capability parsing, batch Anderson spectral module, stable numerics (`log_sum_exp`, Kahan summation). 24 capabilities across 16 domains. JSON-RPC 2.0 server with batch requests and notifications. Zero local WGSL, zero unsafe, zero TODO/FIXME, zero mocks in production. Leverage guide: `wateringHole/WETSPRING_LEVERAGE_GUIDE.md`.
**Faculty**: Christopher Waters (MMG, MSU — undergrad professor), Kevin Liu (CMSE, MSU — master's professor), Jesse Cahill & Chuck Smallwood (Sandia), A. Daniel Jones (BMB/Chemistry, MSU — PFAS job), Rika Anderson (Biology, Carleton — vent metagenomics, pangenomics, phage ecology), Andrea J. Gonzales (Pharmacology & Toxicology, MSU — cytokine signaling, AD, dose-response), Erika Lisabeth (ADDRC Director, MSU — HTS, drug repurposing, EphA3), Richard Neubig (Drug Discovery Director, MSU — GPCR, Rho/MRTF/SRF skin fibrosis)
**Repository**: git@github.com:syntheticChemistry/wetSpring.git
**License**: AGPL-3.0

**What it validates**: wetSpring proves BarraCuda can replace the Galaxy/QIIME2/Python bioinformatics stack with sovereign Rust. The complete 16S pipeline (FASTQ→quality→merge→derep→DADA2→chimera→taxonomy→diversity→UniFrac) runs in Rust with 1 runtime dependency (flate2 for gzip). GPU spectral cosine matching achieves **1,077x speedup** over CPU. The sovereign XML parser eliminates `quick-xml`; the sovereign FASTQ parser eliminates `needletail`.

**Phase inventory**:

| Phase | What | Checks | Key Result |
|-------|------|--------|------------|
| 1-2 (Galaxy→Rust) | 16S + LC-MS + PFAS pipelines, 30 modules | 135/135 | Sovereign bioinformatics in Rust |
| 3 (GPU) | 11 ToadStool primitives, f64 precision | 38/38 | 1,077x spectral cosine speedup |
| 4 (Sovereign 16S) | DADA2 + chimera + taxonomy + UniFrac | 37/37 | Complete 16S in Rust |
| 5 (Paper parity) | Real NCBI data + VOC peaks | 51/51 | Algae pond + Reese 2019 match |
| 6 (Public benchmark) | 4 BioProjects vs paper ground truth | 202/202 | 22 samples, all match |
| 7 (GPU pipeline) | Full DADA2 + chimera + taxonomy on GPU | 88/88 | 10 samples, QF WGSL, streaming |
| V53-V58 | Anderson QS, Track 4 soil, drug repurposing, cross-spring S68 | 3,043+ | 52/52 papers, 79 primitives, 0 local WGSL |
| V59 | Science extensions + three-tier controls | 1,094+ | NCBI sovereign, cold seep, dynamic W(t), NPU sentinel, Exp190-192 |
| V66-V67 | Deep audit + experiment buildout + evolution | 733+ | 50/50 three-tier, BandwidthTier, ComputeDispatch, 10 Python baselines, Exp216-220 |
| V86 | Cross-Spring Evolution & Deep Debt (March 1, 2026) | Exp260-262 | Cross-spring validation 23/23 across 5 Springs; ESN bridge to ToadStool esn_v2 (BioEsn + multi-head); 4 module refactors, 11 new tests (1,247 total); 75 files changed, -4,753 net lines (deep debt elimination) |

**Pipelines validated**: Complete 16S (Galaxy/QIIME2 → Rust → GPU), LC-MS features (asari → Rust), PFAS screening (FindPFAS → Rust), public data benchmark (4 BioProjects, 22 samples, all match paper ground truth), real NCBI sovereign pipeline (EFetch/SRA → FASTA → diversity → Anderson), cold seep metagenome classification, dynamic Anderson W(t) perturbation models.

**Papers reproduced**: 52/52 across 4 tracks — Waters c-di-GMP/QS (Track 1), Liu comparative genomics (Track 1b), deep-sea metagenomics (Track 1c), Jones PFAS (Track 2). 50/50 three-tier eligible papers have full CPU + GPU + metalForge validation (11 extension papers promoted in V67). See `wetSpring/specs/PAPER_REVIEW_QUEUE.md`.

**BarraCuda primitives consumed**: 79 via ToadStool S68 `compile_shader_universal` — including FusedMapReduceF64, BrayCurtisF64, BatchedEighGpu, GemmF64, anderson_3d, lanczos, lanczos_eigenvalues, level_spacing_ratio, and all diversity/spectral primitives. Zero local WGSL, zero local derivative/regression math.

**BarraCuda gaps resolved by wetSpring**: ODE solver (bistable, capacitor, qs_ode), Gillespie stochastic sim, HMM Viterbi (phylohmm), Smith-Waterman alignment.

**Key discovery**: `log_f64` bug in ToadStool — coefficients halved, causing ~1e-3 precision instead of ~1e-15. Found during Shannon entropy validation, fixed immediately in ToadStool core.

**Participates in**: ToadStool validation (GPU science, bug discovery, 79 primitives consumed), biomeOS (microbiome health monitoring, science graph), Squirrel (learned classifiers), NestGate (NCBI JSON-RPC integration).

---

### 1.4 groundSpring — Measurement Noise & Uncertainty

**Domain**: Sensor noise characterization, inverse problems, error propagation, sensing systems, spectral theory, quasispecies, rare biosphere, tissue Anderson, warm dense matter, ESN regime classification
**Grade**: Phase 0 (Python) + Phase 1 (Rust) + Phase 2 (GPU) + Phase 3 (Hardware) + Phase 4 (NUCLEUS) — V104
**Checks**: 395/395 Rust validation checks (all PASS), 936 Rust tests, 287 Python tests, 102 barraCuda delegations (61 CPU + 41 GPU)
**Faculty**: Alexei Bazavov (CMSE + Physics, MSU), Christopher Waters (MMG, MSU), Kevin Liu (CMSE, MSU), Emily Dolson (CSE, MSU), Ilya Kachkovskiy (Math, MSU — Anderson localization, Almost-Mathieu, transport, band edge), Rika Anderson (Biology, Carleton — stochastic vs deterministic evolution, drift, uncertainty bridge, rare biosphere), Andrea J. Gonzales (Paper 12 — immunological Anderson, tissue geometry, drug scoring)
**Repository**: git@github.com:syntheticChemistry/groundSpring.git
**License**: AGPL-3.0-or-later

**What it validates**: groundSpring establishes the uncertainty budget for every other spring. It decomposes measurement error into correctable bias and irreducible noise, quantifies which inputs dominate output uncertainty, and demonstrates how noise propagates through inverse problems. The framework — decompose, identify dominant source, quantify noise floor — is universal across domains. 37 Rust modules across 5 pillars: Signal vs Noise, Inverse Problems, Sensing Systems, Temporal Dynamics, Spatial Propagation. 102 barraCuda delegations cover stats, spectral, ops, linalg, optimize, bio, and ESN domains. metalForge provides cross-substrate dispatch (CPU, GPU via RTX 4070/Titan V, NPU via AKD1000).

**Phase inventory**:

| Phase | What | Checks | Key Result |
|-------|------|--------|------------|
| 0 (Python) | 29 experiments across 5 pillars | 287 tests | Unified noise characterization framework |
| 1 (Rust) | 35 experiments across 8 domains | 395/395 | 37 library modules, 102 barracuda delegations |
| 2 (GPU) | metalForge cross-substrate | 12 binaries | RTX 4070, Titan V, AKD1000 NPU validated |
| 4 (NUCLEUS) | biomeOS niche deployment | 4 binaries | measurement.* domain, JSON-RPC 2.0, capability discovery |

**BarraCuda delegation**: 102 active (61 CPU + 41 GPU). 1 Tier B remaining (PRNG alignment). All Tier C absorbed. P1 batch primitives ready for absorption (uncertainty_budget, regime_classification, Tikhonov defaults, freeze-out scan).

**Papers completed**: 7 faculty, 30+ papers reproduced. Waters (signal specificity, bistable, multi-signal QS), Liu (RAWR, resampling convergence), Kachkovskiy (Anderson, Almost-Mathieu, transport, band edge), Dolson (quasispecies), R. Anderson (drift, uncertainty bridge, rare biosphere), Bazavov (jackknife, freeze-out, spectral recon), Gonzales (tissue Anderson, drug scoring).

**Papers queued**: Remaining candidates. See `groundSpring/specs/PAPER_REVIEW_QUEUE.md`.

**Cross-spring impact**: groundSpring Exp 003 told airSpring that humidity sensors matter most. Exp 004 told wetSpring that 5,000 reads is the genus saturation depth. Exp 001 told neuralSpring how much sensor noise to expect in transfer learning. Spectral theory (Anderson, Almost-Mathieu, band edge) shared with hotSpring and wetSpring. Transport, quasispecies, rare biosphere feed wetSpring and neuralSpring. **Bazavov experiments (019-021) connect directly to hotSpring lattice QCD**: freeze-out inverse validates thermodynamic observable extraction, spectral reconstruction validates ill-posed inversion from Euclidean correlators, and jackknife provides the standard error estimation used in every lattice QCD publication. Combined pipeline: hotSpring (GPU simulation) → groundSpring (inverse problem + error bars) → neuralSpring (surrogate acceleration).

**baseCamp coordination**: groundSpring now contributes to ALL 7 baseCamp papers. The spectral theory experiments (008, 009, 012, 018) validate the Anderson framework underlying papers 01, 05, 06. The Bazavov experiments (019-021) serve paper 07 (WDM/lattice QCD). The evolutionary biology experiments (014, 016, 017) serve papers 02, 04. The sensor noise experiments (001-005) serve papers 03, 04, 06. This makes groundSpring the only spring that contributes to every baseCamp paper — reflecting its role as the universal uncertainty quantification layer.

**Evolution path**:

| Phase | Target | Status |
|-------|--------|--------|
| 0 (Python) | 21 experiments, ~211 Python checks | Complete |
| 1 (Rust CPU) | 236/236 Rust checks, 280 tests, 21/21 parity | Complete |
| 2a (BarraCuda CPU) | Pure Rust math faster than Python, 27 delegations (22 CPU + 5 GPU) | Delegation inventory ready; ToadStool absorbing |
| 2b (BarraCuda GPU) | GPU implementations match CPU exactly | Pending ToadStool absorption |
| 3 (metalForge) | Cross-substrate dispatch (GPU → NPU → CPU) | Architecture defined |

**Participates in**: All springs (provides uncertainty quantification), all 7 baseCamp papers, neuralSpring (noise labels for training robustness), hotSpring (spectral primitives + lattice QCD inverse problems), wetSpring (quasispecies, rare biosphere, Anderson validation).

---

### 1.5 neuralSpring — Machine Learning Primitives, Isomorphic Patterns & Sovereign Structure Prediction

**Domain**: Neural surrogates, transformers, sequence models, transfer learning, scholarly reproduction, structure prediction (coralForge)
**Grade**: Phase 0 → 0+ → 0++ → Rust → BarraCuda CPU → GPU Tensor → metalForge WGSL → Pipeline → Cross-dispatch → Mixed-hardware → Multi-GPU → coralForge. **Most complete validation pipeline of any spring.**
**Checks**: 397/397 Python + 4000+ Rust+GPU = **4,500+ total** (1128 lib + 61 playGround + 73 forge tests, 260 binaries, 220/220 validate\_all, zero C deps via Tower Atomic, OrExit<T> zero-panic, deny.toml, structured logging, structured IpcError, typed compute.dispatch)
**Faculty**: Emily Dolson (CSE, MSU — master's professor), Kevin Liu (CMSE, MSU — master's professor), Christopher Waters (MMG, MSU — undergrad professor), Alexei Bazavov (CMSE + Physics, MSU), Ilya Kachkovskiy (Math, MSU — spectral/optimization landscape), Rika Anderson (Biology, Carleton — pangenomics, constrained evolution empirics), Andrea J. Gonzales (Pharmacology & Toxicology, MSU — Hill equation, PK modeling, CytokineBrain)
**Repository**: git@github.com:syntheticChemistry/neuralSpring.git
**License**: AGPL-3.0

**What it validates**: neuralSpring proves the **Isomorphism Theorem** — all neural architectures decompose into 6 fundamental primitives (GEMM, Attention, Normalization, Nonlinearity, Reduction, Gating), and BarraCuda's WGSL shader library covers all 6. Phase 0++ extended to 15 papers across 4 faculty groups and 5 disciplines. **coralForge** is the sovereign structure prediction engine — pure Rust f64 implementations of AlphaFold2/AlphaFold3 primitives (Evoformer, IPA, diffusion, pairformer, confidence), validated against NumPy baselines and accelerated via BarraCuda/ToadStool. Pure Rust is **83.6× faster** than Python/NumPy (geomean, 11 domains).

**Phase inventory**:

| Phase | What | Checks | Key Result |
|-------|------|--------|------------|
| 0 (Synthetic) | 5 experiments: surrogate, transformer, LSTM, transfer, isomorphic | 48/48 | 6 primitives explain all architectures |
| 0+ (Scholarly) | 5 published studies reproduced | 31/31 | PINN, DeepONet, LeNet-5, ERA5 LSTM, quantized inference |
| 0++ (Extended) | 15 papers across 4 faculty, 5 disciplines | 127/127 | Evolutionary, phylogenetic, game theory, spectral, population genetics |
| 1a (Rust) | 47 modules + gpu\_ops/ + gpu\_dispatch | 1115 lib + 9 integration + 73 forge | 260 validation/bench binaries |
| 2 (bC CPU) | BarraCuda CPU ports — 24/25 papers (96%) | 203 | Machine-precision Rust math |
| 3 (GPU Tensor) | 23/25 papers (92%), metalForge 15/25, pipeline 15/25 | 272+ | CPU→GPU portability proven |
| 4 (Dispatch) | 47 CPU→GPU ops, dispatch parity 30/30, mixed-hardware 47/47 | 124+ | ~97% math on GPU |
| 5 (Multi-GPU) | RTX 4070 + TITAN V (NVK) — 384/384 bit-identical | 384 | Architecture-independent |
| coralForge | AlphaFold2/3 Evoformer, IPA, diffusion, pairformer, confidence | 62 Py + 55 Rs | Sovereign structure prediction |
| WDM | 5 warm dense matter surrogates | 186/186 | Physics surrogates validated |
| baseCamp | 5 biophysical AI sub-theses | 128/128 | GPU pure 5/5 |

**Studies reproduced**: All 25 papers from Phase 0++ + 5 Phase 0/0+ studies + 5 WDM surrogates + 3 publication experiments + coralForge nF-01/02/03 (AlphaFold2 Evoformer + AlphaFold3 diffusion/pairformer/confidence).

**BarraCuda primitives validated**: 219+ import sites across 45+ submodules. 47 GPU-promoted dispatch ops (7 domain files). 46 upstream rewires. 25 absorbed workloads, 1 local remaining. 42 metalForge WGSL shaders. All gaps from original audit closed: HMM Viterbi (done), Gillespie (done), Lanczos (done via hotSpring lineage), pairwise distance (PairwiseL2Gpu), LogSumExp (upstream). playGround compute triangle: ToadStool/coralReef IPC clients, hot/cold dispatch benchmarks (7–45× pipeline reuse speedup).

**BarraCuda gaps remaining**: None critical. L-BFGS optimizer and compute_graph (lazy execution) are future Phase 5 items. All 17 shortcomings (S-01..S-17) resolved upstream.

**Critical gen3 connection**: Dolson's Iram et al. (2020) Nature Physics on counterdiabatic driving of evolution — **reproduced and validated** (Paper 011). coralForge extends the isomorphism to protein structure prediction: the same 6 primitives that serve language models serve AlphaFold.

**Participates in**: biomeOS PathwayLearner (validated ML primitives), Squirrel (MCP adapter — 14 tool definitions, interactive runner), ToadStool (typed IPC client, live coordination verified), coralReef (compiler client, sovereign dispatch path), NUCLEUS (isomorphic kernel sharing), all springs (surrogates, transfer learning), coralForge sovereign protein pipeline, HuggingFace Model Lab (GPT-2 inference on barraCuda).

---

### 1.6 healthSpring — Human Health: PK/PD, Microbiome, Biosignal, Endocrinology, Comparative Medicine, Drug Discovery

**Domain**: Pharmacokinetics, dose-response, population modeling, gut microbiome analytics, ECG biosignal processing, testosterone replacement therapy, endocrinology, comparative medicine, drug discovery, NLME
**Grade**: V27 — 7 tracks, 73 experiments, 601 tests, Tier 0+1+2+3 complete
**Checks**: 601 Rust tests + 194 Python cross-validation = **795 healthSpring checks**
**Faculty**: Andrea J. Gonzales (MSU Pharmacology & Toxicology), Dr. Charles Mok (clinical endocrinology, TRT literature)
**Repository**: git@github.com:syntheticChemistry/healthSpring.git
**License**: AGPL-3.0-or-later

**What it validates**: healthSpring proves the ecoPrimals math infrastructure extends to human clinical applications. PK/PD models validated against canine data in neuralSpring transfer directly to human therapeutics via allometric scaling. The Anderson localization framework from wetSpring/hotSpring applies to gut microbiome colonization resistance. The "claim verification pipeline" — extracting quantifiable claims from clinical practice literature (Mok 2018) and validating against published registry data (Saad, Sharma, Kapoor) — is a novel methodology. The testosterone-gut axis (Exp037) bridges microbiome diversity and endocrine outcomes via Anderson localization, validating the cross-track hypothesis. V25 extends to comparative medicine (species-agnostic PK, cross-species Anderson, canine AD models) and drug discovery (MATRIX scoring, ADDRC HTS, compound screening, iPSC validation). V27 absorbs ODE→WGSL codegen from wetSpring (3 `OdeSystem` impls via barraCuda `BatchedOdeRK4`), uncertainty quantification from groundSpring (bootstrap/jackknife/bias-variance/Monte Carlo), and hardens IPC cast safety.

**Phase inventory**:

| Phase | What | Checks | Key Result |
|-------|------|--------|------------|
| Tier 0 (Python) | 73 experiments across 7 tracks | 194 | All controls pass |
| Tier 1 (Rust CPU) | healthSpring ecoPrimal crate (pkpd, microbiome, biosignal, endocrine, comparative, discovery, NLME) | 601 | All lib + binary tests pass |
| Tier 2 (GPU) | 6 WGSL shaders + 3 ODE→WGSL codegen systems, fused pipeline | 42/42 parity | GPU scaling confirmed (Exp085) |
| Tier 3 (metalForge) | NUCLEUS dispatch with PCIe P2P bypass | 35/35 | Mixed hardware validated (Exp087) |

**Track inventory**:

| Track | Domain | Experiments | Key Models |
|-------|--------|-------------|------------|
| 1 — PK/PD | Pharmacokinetics, dose-response | Exp001-006, 077 | Hill equation, 1/2-compartment PK, PBPK, allometric mAb, population Monte Carlo, Michaelis-Menten |
| 2 — Microbiome | Gut ecology, colonization resistance | Exp010-013, 078-080 | Shannon/Simpson/Pielou/Chao1, Anderson gut lattice, C. diff, FMT, antibiotics, SCFA, serotonin |
| 3 — Biosignal | ECG, HRV, SpO2, EDA, arrhythmia | Exp020-023, 081-082 | Pan-Tompkins, HRV time/freq, PPG SpO2, EDA stress, multi-channel fusion |
| 4 — Endocrinology | Testosterone PK, TRT outcomes | Exp030-038 | IM/pellet depot PK, age decline, metabolic/CV/diabetes outcomes, testosterone-gut axis |
| 5 — NLME | Population PK estimation | Exp075-076 | FOCE/SAEM (sovereign NONMEM/Monolix), NCA, CWRES/VPC/GOF diagnostics |
| 6 — Comparative Medicine | Cross-species health | Exp100-106 | Species-agnostic PK, cross-species Anderson, canine AD, feline hyperthyroid |
| 7 — Drug Discovery | Compound screening | Exp090-094 | MATRIX scoring, ADDRC HTS, iPSC validation, Rho/MRTF fibrosis |

**Upstream dependencies**: wetSpring (diversity, Anderson lattice, ODE→WGSL pattern), neuralSpring (Hill/IC50, PK models, allometric scaling), groundSpring (uncertainty budget — bootstrap, jackknife, bias-variance)

**Participates in**: gen3/baseCamp Paper 13, ToadStool validation (GPU compute live), NestGate (NCBI data pipeline), biomeOS NUCLEUS (distributed pipeline live), metalForge (PCIe P2P dispatch)

---

### 1.7 ludoSpring — Game Science, HCI, Procedural Generation

**Domain**: Game design, human-computer interaction, procedural content generation, real-time interactive systems, computational game science
**Grade**: V6 — 44 experiments, 410 checks, 11 tracks, 2 playable prototypes, 3 external control groups, 4 cross-spring experiments
**Checks**: 410 validation checks + 144 unit/integration tests = **554 ludoSpring checks**
**Faculty**: Csikszentmihalyi (Flow, 1990), Fitts (1954), Yannakakis & Togelius (2018), Lazzaro (Four Keys, 2004), Hunicke (DDA, 2005), Perlin (1985), Gumin (WFC, 2016), Lindenmayer (1968), Tufte (1983)
**Repository**: git@github.com:syntheticChemistry/ludoSpring.git
**License**: AGPL-3.0-or-later

**What it validates**: ludoSpring proves the ecoPrimals Python→Rust→GPU pipeline produces validated science in interactive systems — the most demanding real-time domain humans build. 13 foundational HCI models validated against published research. Game genres are interaction architectures, not aesthetic categories: FPS = molecular explorer, roguelike = parameter space exploration, RTS = systems biology dashboard. External control groups prove the metrics framework is content-agnostic. **Key finding**: Flow state (Csikszentmihalyi) discriminates game quality; engagement alone measures activity, not optimal experience. 110× 60Hz raycaster headroom on CPU, 0.93× fastnoise-lite (C). **Cross-spring**: Anderson QS explorer (Perlin noise as disorder landscape, QS propagation with localization transition), live NCBI data integration, Tower Atomic (BearDog+Songbird) validated. Game metrics generalize to scientific exploration sessions.

**Phase inventory**:

| Phase | What | Checks | Key Result |
|-------|------|--------|------------|
| Python baseline | 7 reference implementations (stdlib only) | 22 parity | Faithful port reference |
| Rust CPU (Tracks 1-5) | 22 validation experiments, 13 HCI models | 183 | All models match published data |
| Playable prototypes (Track 6) | Doom terminal, roguelike explorer, benchmarks | 16 | Every mechanic traces to published paper |
| Telemetry (Track 7) | Protocol, Veloren, Fish Folk, A/B Street adapters | 37 | Cross-engine portability via NDJSON |
| Compute dispatch (Track 8) | CPU-GPU parity, routing, mixed hw, NUCLEUS | 49 | Architecture validated for GPU promotion |
| Benchmarks (Track 9) | Python parity, noise, raycaster, tick budget | 45 | 110× 60Hz, 0.93× fastnoise-lite |
| External controls (Track 10) | Foreign roguelike, 3-way noise, quality discrim. | 36 | Flow discriminates quality across archetypes |
| Cross-spring (Track 11) | NCBI QS, Tower Atomic, QS gene dataset, Anderson QS explorer | 44 | Game metrics validate on scientific exploration; Anderson localization visible |

**Models validated**: Fitts's law (1954), Hick's law (1952), Steering law (1997), GOMS/KLM (1983), Flow theory (1990), Dynamic Difficulty Adjustment (2005), Four Keys to Fun (2004), Engagement metrics (2018), Perlin noise (1985/2002), Wave Function Collapse (2016), L-systems (1968), BSP trees (1980), Tufte data-ink (1983).

**BarraCuda primitives consumed**: `activations::sigmoid`, `stats::dot`, `rng::lcg_step`, `rng::state_to_f64`. 8 modules at Tier A GPU readiness (pure math, embarrassingly parallel).

**Participates in**: gen3/baseCamp Paper 17, ToadStool validation (GPU dispatch architecture), petalTongue visualization (3 dashboard binaries, 7 channel types), biomeOS NUCLEUS (Tower Atomic validated — exp042), metalForge (cross-substrate dispatch for game-science pipelines), wetSpring (Anderson QS explorer — exp044), nestgate (NCBI QS data — exp041/043).

---

## 2. The Spring Network

### 2.1 By the Numbers

| Metric | Value |
|--------|-------|
| Total springs | 7 (5 established + healthSpring + ludoSpring) |
| Total quantitative checks | **12,510+** passing (11,161 established + 795 healthSpring + 554 ludoSpring) |
| Scientific domains covered | Physics, agriculture, biology, chemistry, geophysics, ML, neuromorphic computing, **human health (PK/PD, microbiome, biosignal, endocrinology)**, **game science (HCI, PCG, interactive systems)** |
| Papers reproduced | 70+ (published, peer-reviewed, across all springs) |
| Papers queued for review | 60+ candidates across all springs + 8 Mok-derived experiments |
| Faculty literature map | 14 professors (MSU + Sandia + Carleton + clinical practice) across 9 departments |
| BarraCuda kernels validated by springs | 79+ distinct GPU/NPU primitives (wetSpring alone consumes 79 via ToadStool S68) |
| BarraCuda bugs found by springs | 6 (5 upstream in Sarkas, 1 in ToadStool log_f64) |
| Rust validation checks | 1,008 (wetSpring) + 4,000+ (neuralSpring) + 3,123+ (airSpring) + ~697 (hotSpring) + 236 (groundSpring) |
| WGSL shaders | 700+ cross-spring via ToadStool S68 universal precision |
| Languages | Python (Phase 0), Rust (Phase 1+), WGSL shaders (Phase 2+) |
| License | AGPL-3.0 (all springs, no exceptions) |
| Institutional access required | Zero |
| Proprietary software required | Zero |
| Time from first spring to 11,161+ checks | ~27 days |

### 2.2 Cross-Spring Data Flow

```
groundSpring (uncertainty + spectral + quasispecies — 21 experiments, 236/236 checks, 8 domains)
    │
    ├──→ airSpring: "humidity dominates ET₀ uncertainty at 66%"
    ├──→ wetSpring: "genus saturation at 5,000 reads; quasispecies, rare biosphere"
    ├──→ neuralSpring: "expect 0.004-0.021 m³/m³ sensor noise floor; noise labels"
    ├──→ hotSpring: "inverse problem depth poorly constrained; Anderson, Almost-Mathieu, band edge spectral primitives"
    └──→ ToadStool: "27 barracuda delegations (22 CPU + 5 GPU)"

neuralSpring (ML primitives)
    │
    ├──→ airSpring: "MLP surrogate replaces FAO-56 at R²=0.999"
    ├──→ airSpring: "transfer learning bridges Michigan→NM with 200 samples"
    ├──→ hotSpring: "isomorphic GEMM serves plasma and nuclear"
    ├──→ wetSpring: "LSTM validates lstm_cell.wgsl on real weather"
    ├──→ **healthSpring**: "Hill/IC50, PK models, allometric scaling → human therapeutics"
    └──→ biomeOS: "PathwayLearner uses validated attention primitives"

hotSpring (GPU compute patterns)
    │
    ├──→ airSpring: "f64 GPU dispatch batching pattern"
    ├──→ wetSpring: "FusedMapReduceF64 pattern for bulk statistics"
    └──→ ToadStool: "195 acceptance checks, 6 bugs found"

wetSpring (biology + chemistry — 4,688+ checks, 197 experiments, 52/52 papers)
    │
    ├──→ ToadStool: "log_f64 bug found and fixed; 79 primitives consumed; three-tier validated (CPU→GPU→metalForge)"
    ├──→ ToadStool: "Anderson spectral primitives (anderson_3d, lanczos, level_spacing_ratio) validated at f64"
    ├──→ ToadStool: "Typed NCBI errors (Error::Ncbi) for sovereign data acquisition"
    ├──→ airSpring: "kriging spatial interpolation; dynamic Anderson W(t) models soil moisture coupling"
    ├──→ hotSpring: "Anderson localization applied to biology — shared spectral primitives, W_c determination"
    ├──→ neuralSpring: "ESN/LSTM anomaly detection for sentinel microbes; NPU int8 quantization validated"
    ├──→ **healthSpring**: "diversity indices, Anderson lattice → gut colonization resistance, 16S pipeline"
    └──→ groundSpring: "sequencing noise calibrates rarefaction; 86 named tolerances with provenance"

ludoSpring (game science — 410 checks, 44 experiments, 13 HCI models)
    │
    ├──→ barraCuda: "sigmoid, dot, lcg_step, state_to_f64 consumed; 8 Tier A GPU modules identified"
    ├──→ petalTongue: "3 dashboard binaries, 7 GameChannelType channels, live streaming"
    ├──→ healthSpring: "Fitts/Hick for medical UI evaluation; engagement for patient compliance"
    ├──→ wetSpring: "Perlin noise as Anderson disorder landscape; game telemetry protocol for lab UIs"
    └──→ all springs: "Flow theory + DDA for any adaptive interactive system"
```

### 2.3 Spring → Primal Connections

| Spring | Primarily Validates | Also Feeds |
|--------|-------------------|------------|
| hotSpring | ToadStool/BarraCuda (GPU MD, nuclear EOS) | gen3 (constrained evolution evidence) |
| airSpring | ToadStool/BarraCuda (Rust science crate) | Penny Irrigation (real-world application) |
| wetSpring | ToadStool/BarraCuda (GPU diversity, spectral) | biomeOS (microbiome monitoring), Squirrel (classifiers) |
| groundSpring | All springs (uncertainty budget) | neuralSpring (noise labels for training) |
| neuralSpring | ToadStool/BarraCuda (ML kernels) | biomeOS (PathwayLearner), Squirrel (inference), NUCLEUS (optimization) |
| **healthSpring** | ToadStool/BarraCuda (population PK, Anderson gut, biosignal) | NestGate (NCBI clinical data), biomeOS NUCLEUS (distributed health pipeline) |
| **ludoSpring** | ToadStool/BarraCuda (game math: noise, raycaster, metrics) | petalTongue (live dashboards), biomeOS NUCLEUS (Tower Atomic validated), wetSpring (Anderson QS cross-spring), nestgate (NCBI QS data), all springs (HCI models for any interactive system) |

### 2.4 Faculty → Spring Mapping

| Professor | Department | Published Domain | Springs |
|-----------|-----------|------------------|---------|
| Michael Murillo | CMSE, MSU | Dense plasmas, WDM, molecular dynamics | hotSpring |
| Younsuk Dong | BAE, MSU | Precision agriculture, irrigation | airSpring |
| Christopher Waters | MMG, MSU | Quorum sensing, c-di-GMP | wetSpring, groundSpring, neuralSpring |
| Kevin Liu | CMSE, MSU | Comparative genomics, phylogenetics | wetSpring, groundSpring, neuralSpring |
| Alexei Bazavov | CMSE + Physics, MSU | Lattice QCD, thermodynamics | hotSpring, groundSpring, neuralSpring |
| Emily Dolson | CSE, MSU | Evolutionary computation | neuralSpring, groundSpring |
| Ilya Kachkovskiy | Math, MSU | Spectral theory, Anderson localization | hotSpring, groundSpring, neuralSpring |
| Jesse Cahill | Sandia (Bioscience) | Biosurveillance | wetSpring |
| Chuck Smallwood | Sandia (Bioscience) | Biosurveillance | wetSpring |
| A. Daniel Jones | BMB/Chemistry, MSU | Mass spectrometry, PFAS | wetSpring |
| Rika Anderson | Biology, Carleton College | Vent metagenomics, pangenomics | hotSpring, wetSpring, groundSpring, neuralSpring |
| Andrea J. Gonzales | Pharmacology & Toxicology, MSU | Pharmacology, cytokine signaling | wetSpring, neuralSpring, airSpring, **healthSpring** |
| Erika Lisabeth | Pharmacology & Toxicology, MSU (ADDRC) | Drug discovery, HTS | wetSpring |
| Richard Neubig | Pharmacology & Toxicology, MSU (Drug Discovery) | GPCR signaling, fibrosis | wetSpring, neuralSpring |
| **Charles Mok** | **Clinical Practice** | **Clinical endocrinology, TRT** | **healthSpring** |

Full profiles: `data/FACULTY_SPRING_PROFILES.md`

### 2.5 BarraCuda Gap Summary (Across All Springs)

| Gap | Requesting Springs | Priority | Faculty Driver | Status |
|-----|-------------------|----------|---------------|--------|
| FFT | hotSpring (lattice QCD), groundSpring (spectral recon), wetSpring (signal) | P0 | Bazavov | **Resolved** — hotSpring |
| ODE solver (RK4) | wetSpring (c-di-GMP), groundSpring (bifurcation) | P0 | Waters | **Resolved** — wetSpring bistable, capacitor, qs_ode validated |
| Lanczos eigensolve | hotSpring (Dirac spectrum), groundSpring (Anderson), neuralSpring (Hessian) | P1 | Kachkovskiy | **Resolved** — wetSpring + hotSpring: `lanczos`, `lanczos_eigenvalues` validated (CPU + GPU) |
| SpMV (sparse matrix-vector) | hotSpring (lattice gauge), groundSpring (spectral), neuralSpring (sparse) | P1 | Kachkovskiy | **Resolved** — implemented for Lanczos + Anderson 3D |
| HMM Viterbi | neuralSpring (PhyloNet-HMM), wetSpring (metagenomics) | P1 | Liu | **Resolved** — wetSpring `phylohmm` module validated |
| Evolutionary optimization | neuralSpring (counterdiabatic), groundSpring (Dolson) | P1 | Dolson | Open — unlocks constrained evolution validation |
| Smith-Waterman alignment | wetSpring (genomics), neuralSpring (sequence models) | P1 | Liu | **Resolved** — wetSpring Exp028 validated |
| Gillespie simulation | wetSpring (quorum sensing), groundSpring (biological noise) | P1 | Waters | **Resolved** — wetSpring stochastic modules validated |
| Matrix exponentiation | hotSpring (SU(3) HMC), groundSpring (transport) | P2 | Kachkovskiy, Bazavov | Open — general exp(A) for time evolution |
| L-BFGS optimizer | neuralSpring (PINN improvement) | P2 | Raissi | Open — closes PINN error gap (5.1% → ~0.06%) |
| Cholesky solve batch | groundSpring (jackknife, spectral recon) | P1 | Bazavov | Open — main NEW gap |

---

## 3. The Evidence

The springs answer a question the primals alone cannot: **does this infrastructure produce correct science?**

The primals prove that Rust can build a sovereign computing ecosystem. The springs prove that sovereign computing can reproduce published, peer-reviewed science — and in some cases, do it faster, cheaper, and more transparently than the institutional tools it replaces.

**Claim**: Open data can replace institutional access.
**Evidence**: airSpring achieves R²=0.967 across 918 station-days using only free, open APIs (Open-Meteo, NOAA CDO). No institutional weather station access required.

**Claim**: Consumer GPUs can do real science.
**Evidence**: hotSpring runs paper-parity Yukawa MD (N=10,000, 80k steps) on a $600 RTX 4070 for $0.044. The same computation costs $50-500 on institutional HPC.

**Claim**: Sovereign Rust can replace the Python scientific stack.
**Evidence**: wetSpring's 30 Rust modules with 1 runtime dependency cover the complete 16S pipeline with 4,688+ checks across 197 experiments, 52/52 papers reproduced, and 39/39 three-tier validated (CPU→GPU→metalForge). airSpring's Rust crate matches Python to 1e-5 across 53 cross-validated values.

**Claim**: BarraCuda's 6 isomorphic primitives serve all domains.
**Evidence**: neuralSpring proves GEMM + Attention + Normalization + Nonlinearity + Reduction + Gating explain LLaMA, OpenFold, ResNet, ViT, MLP surrogates, and LSTM weather models. All 6 are WGSL shaders in BarraCuda.

**Claim**: Science validation improves the infrastructure.
**Evidence**: wetSpring found and fixed the `log_f64` bug in ToadStool. hotSpring found and fixed 5 silent bugs in Sarkas upstream. groundSpring identified humidity as the bottleneck for ET₀ accuracy. Each discovery fed back into the system.

---

**Claim**: Clinical practice literature can be computationally verified.
**Evidence**: healthSpring's Track 4 (Mok testosterone) extracts quantifiable claims from a 196-page clinical book and validates each against the cited primary literature — creating a closed-loop claim verification pipeline that generalizes to any medical reference.

**Every check count in this catalog is measured, not estimated. Every paper reproduction runs on consumer hardware with no institutional access. Every spring is AGPL-3.0 and publicly available on GitHub. The science is open because the methodology demands it. 11,161+ checks across 5 domains in 27 days — the constrained evolution methodology works. 7 of 11 BarraCuda gaps have been resolved by the springs themselves (ODE, Lanczos, SpMV, HMM, Smith-Waterman, Gillespie, FFT). A $300 NPU runs ESN inference at 2.8μs/step. A $600 GPU runs the same ESN at 8.2× CPU speed when the reservoir is large enough. The silicon does what the silicon does — we just had to look.**
