+++
title = "Sovereign Human Health"
description = "Precision Medicine x Pharmacometrics — sovereign PK/PD modeling, biosignal analysis, drug discovery. healthSpring."
date = 2026-03-17

[extra]
paper_number = 13
domain = "Human Health"

[taxonomies]
primals = ["barracuda", "biomeos", "coralreef", "petaltongue", "songbird", "sweetgrass"]
springs = ["airspring", "groundspring", "healthspring", "neuralspring", "wetspring"]
+++

**Spring**: healthSpring (V35)
**Domain**: Human Health × Pharmacology × Microbiome × Biosignal × Endocrinology × NLME × Clinical Translation × Comparative Medicine × Drug Discovery × biomeOS Niche Deployment
**License**: AGPL-3.0-or-later
**Date**: March 17, 2026
**Reproduces work by**: Andrea J. Gonzales (MSU Pharmacology & Toxicology), Charles Mok (clinical endocrinology)
**Status**: **IPC Resilience + Sovereign Dispatch** — V35: 7 tracks complete, 613 tests, 113/113 cross-validation checks, 6 WGSL shaders, 79 JSON-RPC capabilities. Sovereign GPU dispatch via `CoralReefDevice`; IPC resilience patterns: `CircuitBreaker`, `RetryPolicy`, `DispatchOutcome`. Composition guidance: `GROUNDSPRING_V114_PRIMAL_COMPOSITION_GUIDANCE_MAR17_2026.md`. V34: Deep debt evolution. V33: `IpcError::is_recoverable()`, `ipc::protocol` module, centralized `cast` module. V32: Structured `tracing`, `health.liveness`/`health.readiness` probes, resilient provenance trio IPC. V31: `OrExit<T>`, `IpcError`, enriched `capability.list`, `#![forbid(unsafe_code)]`. Zero clippy warnings workspace-wide (pedantic + nursery), zero unsafe, zero TODO/FIXME, zero `#[allow()]`, all files under 1000 LOC. AGPL-3.0-or-later across all files.

---

## Thesis

Sovereign scientific computing can replace Python/NONMEM/proprietary tool chains for human health applications — pharmacokinetics, microbiome analytics, real-time biosignal processing, and endocrine outcome modeling — using pure Rust validated against published data, **with live GPU acceleration** via barraCuda WGSL shaders and heterogeneous dispatch via toadStool/metalForge. **V22**: healthSpring now operates as a biomeOS niche — a composed set of primals and workflow graphs orchestrated by the Neural API, with all science capabilities exposed via JSON-RPC 2.0 and discoverable via capability routing.

Population-level validation means nothing without per-person translation. The pipeline closes this loop: a `PatientTrtProfile` generates a patient-specific clinical scenario — parameterized by age, weight, testosterone level, comorbidities — rendered in petalTongue's clinical mode via the SAME DAVE motor command channel. The clinician sees the patient, not the infrastructure.

GPU-native execution is validated: a single consumer GPU handles population-scale health computations (10M elements, 207 M/s throughput) with the same WGSL shaders portable to edge devices, TPU, and NPU. Mixed hardware dispatch via NUCLEUS topology routes workloads to CPU, GPU, or NPU based on scale and substrate availability.

V14 adds sovereign replacements for three commercial pharmacometric tools: NONMEM (FOCE estimation), Monolix (SAEM estimation), and WinNonlin (NCA metrics). NLME diagnostics (CWRES, VPC, GOF) complete the population PK pipeline. A WFDB parser enables direct PhysioNet biosignal ingestion. Kokkos-equivalent benchmarks validate GPU-portable patterns. The full petalTongue pipeline now spans 5 tracks with 28 nodes, 121 channels, and 14 scenarios — making the complete healthSpring pipeline human-visible and actionable.

The springs validate science. healthSpring makes the drug.

---

## Tracks

### Track 1: Pharmacokinetic / Pharmacodynamic Modeling (Exp001–006, 077)

Pure Rust PK/PD tools extending neuralSpring nS-601–605 (veterinary) to human therapeutics via allometric scaling.

| Exp | Title | Key Result |
|-----|-------|-----------|
| 001 | Hill dose-response (4 JAK inhibitors + canine reference) | 4-parameter Hill equation, IC50/EC50 validated |
| 002 | One-compartment PK (IV bolus + oral Bateman + multiple dosing) | AUC trapezoidal, steady-state accumulation ratio |
| 003 | Two-compartment PK (biexponential α/β phases) | Distribution/elimination phase separation |
| 004 | mAb PK cross-species transfer (lokivetmab → human) | Allometric scaling (BW^0.75 CL, BW^1.0 Vd) |
| 005 | Population PK Monte Carlo (1,000 virtual patients) | Lognormal IIV, CL-AUC correlation r = -0.92 |
| 006 | PBPK 5-tissue physiological compartments | Mass conservation, hepatic clearance, tissue Kp |
| 077 | Michaelis-Menten nonlinear PK (phenytoin) | Capacity-limited elimination, dose-dependent half-life, supralinear AUC |

**Lineage**: Paper 12 veterinary→human bridge (Gonzales iPSC, Neubig drug discovery) → healthSpring human therapeutics.

**V16**: Exp077 adds Michaelis-Menten (capacity-limited) PK — the first nonlinear elimination model. Phenytoin reference parameters from Ludden 1977. GPU-ready via `michaelis_menten_batch_f64.wgsl` (per-patient parallel Euler ODE).

### Track 2: Gut Microbiome and Colonization Resistance (Exp010–013, 078–080)

Extends wetSpring's Anderson localization framework from soil to gut.

| Exp | Title | Key Result |
|-----|-------|-----------|
| 010 | Shannon/Simpson/Pielou/Chao1 diversity indices | Validated against SRA published data |
| 011 | Anderson localization in gut lattice (1D ξ) | Localization length predicts colonization resistance |
| 012 | C. difficile colonization resistance score | Composite: diversity + ξ + Firmicutes ratio |
| 013 | FMT microbiota transplant for rCDI | Engraftment fraction → diversity restoration via Bray-Curtis |
| 078 | Antibiotic perturbation model (ciprofloxacin) | Exponential kill + recovery dynamics, Dethlefsen reference |
| 079 | SCFA production (acetate, propionate, butyrate) | Michaelis-Menten fermentation kinetics, fiber-to-SCFA validation |
| 080 | Gut-brain serotonin axis | 5-HT production from tryptophan via gut microbiota, Yano 2015 reference |

**Lineage**: Paper 01/06 Anderson QS framework → Paper 12 immunological Anderson → healthSpring gut colonization.

**V13 fix**: Anderson/IPR computation now uses true eigenvectors from QL diagonalization, not Hamiltonian diagonal.

**V16**: Exp078 models antibiotic disruption and recovery of the gut microbiome. Exp079 validates SCFA production via Michaelis-Menten kinetics (GPU-ready via `scfa_batch_f64.wgsl`). Exp080 adds the gut-brain serotonin axis (tryptophan → 5-HT cross-track hypothesis D5 linking microbiome to neurochemistry).

### Track 3: Biosignal Processing (Exp020–023, 081–082)

Real-time physiological signal analysis on sovereign hardware.

| Exp | Title | Key Result |
|-----|-------|-----------|
| 020 | Pan-Tompkins QRS detection (ECG R-peak) | Bandpass + derivative + MWI + threshold |
| 021 | HRV metrics (SDNN, RMSSD, pNN50) | Time-domain HRV from R-peak intervals |
| 022 | PPG SpO2 R-value calibration | Beer-Lambert AC/DC ratio → SpO2 |
| 023 | Multi-channel fusion (ECG + PPG + EDA) | FusedHealthAssessment: HR + SpO2 + stress index |
| 081 | EDA electrodermal stress detection | Tonic/phasic decomposition, skin conductance response peaks |
| 082 | Arrhythmia beat classification (template matching) | Cross-correlation beat typing: Normal, PVC, PAC, BBB |

**NPU target**: Pan-Tompkins is a streaming signal pipeline ideal for Akida AKD1000 (<1ms, microwatt).

**V16**: Exp081 validates sovereign EDA analysis (tonic/phasic decomposition + SCR detection). Exp082 adds template-matching beat classification — GPU-ready via `beat_classify_batch_f64.wgsl` (per-beat normalized cross-correlation).

### Track 4: Endocrinology — Testosterone PK and TRT Outcomes (Exp030–038)

Clinical claim verification pipeline: extracting quantifiable claims from Dr. Charles Mok's clinical reference and validating against published registry data.

| Exp | Title | Key Result |
|-----|-------|-----------|
| 030 | Testosterone PK: IM injection steady-state | Weekly vs biweekly, trough analysis |
| 031 | Testosterone PK: pellet depot (5-month) | Zero-order release, 10mg/lb dosing |
| 032 | Age-related testosterone decline | Harman 2001 BLSA: -1.6%/yr after 30 |
| 033 | TRT metabolic response: weight/BMI/waist | Saad 2013 registry (n=411) |
| 034 | TRT cardiovascular: lipids + CRP + BP | Sharma 2015 (VA, n=83,010) |
| 035 | TRT diabetes: HbA1c + insulin sensitivity | Kapoor 2006 RCT |
| 036 | Population TRT Monte Carlo (10K patients) | Lognormal IIV + age-adjusted decline |
| 037 | Testosterone-gut axis (cross-track 2×4) | Pielou evenness → Anderson ξ → metabolic response |
| 038 | HRV × TRT cardiovascular (cross-track D3) | SDNN improvement → composite cardiac risk |

**Novel contribution**: Exp037 bridges gut microbiome diversity with TRT metabolic outcomes via Anderson localization — a testable hypothesis for clinical investigation. Exp038 validates HRV as a surrogate endpoint for cardiovascular benefit of TRT.

### Track 5: NLME Population Pharmacokinetics (Exp075–076)

Sovereign replacement for NONMEM (FOCE), Monolix (SAEM), and WinNonlin (NCA).

| Exp | Title | Key Result |
|-----|-------|-----------|
| 075 | NLME cross-validation (FOCE/SAEM, NCA, diagnostics) | FOCE 30% theta recovery, SAEM 50%, NCA λz/AUC∞ 5%, CWRES <2.0, GOF R²≥0 |
| 076 | Full pipeline petalTongue validation (5 tracks, 28 nodes) | 197/197 structural checks, 121 channels, all 7 DataChannel types |

**Novel contribution**: First sovereign pure-Rust NLME stack. FOCE + SAEM estimation with NCA and full diagnostics (CWRES, VPC, GOF) — no NONMEM, no Monolix, no WinNonlin, no Fortran, no Python. Deterministic reproducibility (same seed → identical results). VPC Monte Carlo simulation is a GPU promotion candidate (embarrassingly parallel).

**WFDB parser**: `ecoPrimal/src/wfdb.rs` — streaming PhysioNet Format 212/16 decoder with beat annotation parsing. Enables direct ingestion from MIT-BIH, MIMIC-III, and other PhysioNet databases.

**Kokkos-equivalent benchmarks**: `ecoPrimal/benches/kokkos_parity.rs` — reduction, scatter, Monte Carlo, ODE batch, NLME iteration. Validates GPU-portable patterns ahead of shader promotion.

**Industry benchmark mapping**: SnapGene, Chromeleon, NONMEM, Monolix, WinNonlin profiled. Sovereign replacements mapped to ecoPrimals stack. See `healthSpring/specs/PAPER_REVIEW_QUEUE.md`.

### Validation Track (Exp040)

| Exp | Title | Key Result |
|-----|-------|-----------|
| 040 | barraCuda CPU parity (15 analytical contracts) | Hill, Bateman, allometric, Shannon, Simpson, population PK |

### Integrated Diagnostics (Exp050–052)

| Exp | Title | Key Result |
|-----|-------|-----------|
| 050 | Integrated 4-track patient diagnostic | Cross-track composite risk score |
| 051 | Population diagnostic Monte Carlo (1,000 patients) | Population-level diagnostic distribution |
| 052 | petalTongue scenario schema validation | DataChannel, ClinicalRange conformance |

### CPU vs GPU Parity & Mixed Dispatch (Exp060–062)

| Exp | Title | Key Result |
|-----|-------|-----------|
| 060 | CPU vs GPU parity matrix (3 kernels × 3 scales) | 27/27 parity checks through toadStool Pipeline |
| 061 | Mixed hardware dispatch (NUCLEUS topology) | 22/22 dispatch route checks (CPU+GPU+NPU) |
| 062 | PCIe P2P transfer validation (Gen3/4/5) | 26/26 bandwidth and overhead checks |

### Clinical Translation (Exp063–065)

| Exp | Title | Key Result |
|-----|-------|-----------|
| 063 | Patient-parameterized TRT scenarios (5 archetypes) | Per-person: 8 nodes + 8 edges, clinical mode preset |
| 064 | IPC push to petalTongue | Unix socket JSON-RPC, live scenario update |
| 065 | Live streaming dashboard | ECG, HRV, PK via StreamSession with backpressure |

### Compute & Benchmark (Exp066–072)

| Exp | Title | Key Result |
|-----|-------|-----------|
| 066 | barraCuda CPU benchmark | Hill, PopPK, Diversity timing vs Python |
| 067 | GPU parity extended | Additional kernel validation |
| 068 | GPU benchmark | Throughput at scale |
| 069 | toadStool dispatch matrix | Stage assignment validation |
| 070 | PCIe P2P bypass | NPU→GPU direct transfer |
| 071 | Mixed system pipeline | CPU+GPU+NPU coordinated execution |
| 072 | Compute dashboard | toadStool streaming → petalTongue live gauges |

### Paper Queue Validation (Exp077–082)

| Exp | Title | Key Result |
|-----|-------|-----------|
| 077 | Michaelis-Menten nonlinear PK (phenytoin) | Capacity-limited elimination, dose-dependent t½, supralinear AUC |
| 078 | Antibiotic perturbation model (ciprofloxacin) | Exponential kill + recovery, Dethlefsen reference |
| 079 | SCFA production (acetate/propionate/butyrate) | MM fermentation kinetics, Cummings 1987 validation |
| 080 | Gut-brain serotonin axis | Tryptophan → 5-HT, microbiota-mediated, Yano 2015 |
| 081 | EDA electrodermal stress detection | Tonic/phasic decomposition, SCR peak detection |
| 082 | Arrhythmia beat classification | Template-matching: Normal/PVC/PAC/BBB, MIT-BIH reference |

### GPU V16 Portability (Exp083)

| Exp | Title | Key Result |
|-----|-------|-----------|
| 083 | GPU V16 parity (3 shaders + metalForge + toadStool) | 25/25: CPU determinism, physiological ranges, scalar parity, routing, shaders |

### petalTongue Evolution (Exp073–074)

| Exp | Title | Key Result |
|-----|-------|-----------|
| 073 | Clinical TRT live dashboard | PK trough streaming, HRV improvement, cardiac risk replace |
| 074 | Interaction roundtrip | Mock petalTongue: render, append, replace, gauge, capabilities, subscribe — 12/12 |

---

## Metrics

| Metric | Value |
|--------|-------|
| Experiments | 61 |
| Rust lib tests | 365 (barraCuda) |
| Rust forge tests | 33 (metalForge) |
| Rust toadStool tests | 36 |
| Doc-tests | 4 |
| **Total tests** | **458** |
| Python cross-validation checks | 113/113 |
| Criterion benchmarks | 14 |
| CPU parity bench cases | 14 (Exp084: Rust 84× faster than Python) |
| petalTongue pipeline | 28 nodes, 29 edges, 121 channels, 14 scenarios |
| NLME validation | FOCE + SAEM + NCA + CWRES + VPC + GOF (Exp075, 19 checks) |
| GPU parity checks | 27/27 (Exp060) + 25/25 (Exp083) |
| GPU fused pipeline checks | 11/11 (Exp054) |
| Mixed dispatch checks | 22/22 (Exp061) |
| PCIe transfer checks | 26/26 (Exp062) |
| WGSL compute shaders | 6 (3 V15 + 3 V17) |
| Patient archetypes | 5 (Exp063) |
| Unsafe blocks | 0 |
| Clippy warnings | 0 (`#![deny(clippy::pedantic)]` in all lib crates, `-W clippy::nursery`) |
| Max file size | 819 lines (under 1000-line limit) |

---

## GPU Pipeline (Tier 2) — LIVE

### WGSL Shaders (f64 precision)

| Shader | GpuOp | Pattern | Validated |
|--------|-------|---------|:---------:|
| `hill_dose_response_f64.wgsl` | HillSweep | Element-wise, power via f32 exp/log | Exp053 |
| `population_pk_f64.wgsl` | PopulationPkBatch | Embarrassingly parallel MC (u32 PRNG) | Exp053 |
| `diversity_f64.wgsl` | DiversityBatch | Workgroup-level reduction | Exp053 |
| `michaelis_menten_batch_f64.wgsl` | MichaelisMentenBatch | Per-patient Euler ODE + Wang hash PRNG | Exp083 |
| `scfa_batch_f64.wgsl` | ScfaBatch | Element-wise MM kinetics (3-output) | Exp083 |
| `beat_classify_batch_f64.wgsl` | BeatClassifyBatch | Per-beat normalized cross-correlation | Exp083 |

### Architecture

| Component | Purpose |
|-----------|---------|
| `GpuContext` | Persistent wgpu device/queue, eliminates per-dispatch init |
| `execute_fused()` | Unidirectional pipeline: upload → N compute passes → readback |
| `Pipeline::execute_gpu()` | toadStool dispatches stages via `GpuContext` |
| `Pipeline::execute_auto()` | metalForge routes per stage (GPU if element count > threshold) |

### Scaling (RTX 4070, release build)

| Operation | GPU Crossover | Peak Speedup | Peak Throughput |
|-----------|:------------:|:------------:|:---------------:|
| Hill dose-response | **100K** | 2.0x (5M) | 207 M/s |
| Population PK MC | **5M** | 1.15x (5M) | 365 M/s |
| Fused pipeline (overhead) | — | **31.7x** vs individual | — |

---

## V14 NLME + Full Pipeline Evolution

| Change | Impact |
|--------|--------|
| NLME population PK (FOCE + SAEM) | Sovereign NONMEM/Monolix replacement in `pkpd/nlme.rs`. 30 subjects, theta/omega/sigma recovery. |
| NCA | Sovereign WinNonlin replacement in `pkpd/nca.rs`. λz, AUC∞, MRT, CL, Vss. |
| NLME diagnostics (CWRES, VPC, GOF) | `pkpd/diagnostics.rs`. CWRES ~N(0,1), VPC 50 simulations, GOF scatter. |
| WFDB parser | `wfdb.rs` — PhysioNet Format 212/16 streaming decode + beat annotations. |
| Kokkos-equivalent benchmarks | `benches/kokkos_parity.rs` — 5 GPU-portable patterns validated on CPU. |
| Full petalTongue pipeline | 28 nodes (was 22), 29 edges (was 22), 121 channels (was 65), 14 scenarios (was 13). |
| Exp075 | NLME cross-validation: 19 binary checks (FOCE/SAEM/NCA/CWRES/GOF). |
| Exp076 | Full pipeline validation: 197 binary checks across all 5 tracks + full study. |
| Industry benchmarks | SnapGene, Chromeleon, NONMEM, Monolix, WinNonlin profiled and mapped. |

---

## V19 Full-Stack Portability Evolution

| Change | Impact |
|--------|--------|
| Exp085 GPU scaling bench (47/47) | 4 scales (64→4096) × 3 V16 ops: MM PK (linear scaling confirmed, 64→96K µs), SCFA (sub-µs at 100, 13 µs at 10K), Beat classify (1.4→129 µs). Fused 3-op pipeline: 6ms CPU. metalForge routes small→CPU, large→GPU. |
| Exp086 toadStool V16 dispatch (24/24) | All V16 StageOps through `execute_cpu` + `execute_streaming` with per-stage callbacks. Streaming matches CPU result. All V16 stages map to `GpuOp` via `to_gpu_op()`. Mixed V15+V16 pipeline (Generate→Hill→Reduce). |
| Exp087 mixed NUCLEUS V16 dispatch (35/35) | Eastgate Tower topology (CPU+GPU+NPU, PCIe Gen4). V16 workload routing at 8 scales. PCIe P2P bypass GPU↔NPU (31.5 GB/s, 5.1 µs for 160KB). NPU→GPU dispatch plan for biosignal→classification pipeline. GPU-only pipeline: 0 transitions. Full 5-stage mixed pipeline: GPU→GPU→GPU→NPU→CPU with 2 transitions. |
| Python control (10/10) | Cross-validates MM AUC, SCFA ratios, beat correlation, and scaling linearity from Rust timing results. |

---

## V20 petalTongue V16 Visualization Evolution

V20 makes healthSpring's validated science **visible**. Six V16 primitives and the compute pipeline now have `petalTongue` scenario builders producing real-data visualizations.

| Change | Impact |
|--------|--------|
| **V16 scenario builder** | 6 nodes with real math: MM PK dose curves, antibiotic recovery, SCFA saturation, serotonin pathway, EDA decomposition, arrhythmia templates |
| **Compute pipeline scenarios** | GPU scaling curves, NUCLEUS topology, mixed dispatch plan — all as petalTongue DataChannels |
| **Full study extended** | 28 → 34 nodes, 29 → 38 edges, all 7 DataChannel types in unified graph |
| **Exp088 unified dashboard** | 326 validation checks across all scenarios, JSON dump + IPC push, quick-start guide |
| **Exp089 patient explorer** | CLI-parameterized diagnostic + V16 analysis, streams to petalTongue, combined diagnostic+V16 scenario |
| **dump_scenarios extended** | 14 → 16 scenario JSONs (added healthspring-v16.json, healthspring-compute.json) |

---

## V18 CPU Parity Benchmark Evolution

| Change | Impact |
|--------|--------|
| Exp084 V16 CPU parity bench | 14 matching benchmark cases across 6 V16 primitives: Python baseline (17/17 checks) vs Rust CPU (33/33 checks). |
| Rust 84× overall speedup | Aggregate mean across all 14 bench cases: Python total 52,034 µs vs Rust 622 µs. |
| SCFA 160×, Antibiotic 233× | Michaelis-Menten math: compiled Rust eliminates interpreter overhead entirely. |
| Beat classification 149× | Template matching via normalized cross-correlation: 1000 beats in 204 µs (Rust) vs 30,377 µs (Python). |
| Serotonin 149×, Tryptophan 155× | Sigmoid diversity-factor computation: Rust inlines and optimizes exp/sigmoid chains. |
| MM PK simulate 33× | Euler ODE integration: 10,000 steps in 110 µs (Rust) vs 3,626 µs (Python). |
| EDA SCL/phasic — numpy faster | numpy `convolve` uses compiled C/BLAS internally; naive Rust rolling average is 3× slower. Target for SIMD optimization in barraCuda. |
| `bench_results_v16_rust_cpu.json` | Machine-readable timing results for downstream CI comparison. |
| `bench_results_v16_python.json` | Python baseline timings with provenance (Python version, numpy version). |
| `compare_v16_benchmarks.py` | Side-by-side speedup table generator for Rust vs Python timing data. |

---

## V17 GPU Portability Evolution

| Change | Impact |
|--------|--------|
| `michaelis_menten_batch_f64.wgsl` | Per-patient Michaelis-Menten ODE via Euler integration on GPU. Wang hash + xorshift32 PRNG for lognormal Vmax variation. |
| `scfa_batch_f64.wgsl` | Batch SCFA production (acetate/propionate/butyrate) via element-wise Michaelis-Menten fermentation kinetics. |
| `beat_classify_batch_f64.wgsl` | Per-beat template-matching classification via normalized cross-correlation. Normal/PVC/PAC/BBB typing. |
| metalForge 3 new `Workload` variants | `MichaelisMentenBatch`, `ScfaBatch`, `BeatClassifyBatch` — cross-system routing with GPU/CPU threshold selection. |
| toadStool 3 new `StageOp` variants | Streaming pipeline dispatch for all V16 primitives — CPU fallback + GPU promotion. |
| Exp083 GPU V16 parity | 25/25 validation checks: CPU determinism, physiological ranges, scalar API parity, metalForge routing, shader compilation, memory estimates. |
| `GpuContext` fused support | All 6 `GpuOp` variants dispatchable through `execute_fused()` unidirectional pipeline. |
| `bytemuck` param structs | `MmParams`, `ScfaGpuParams`, `BeatClassifyParams` — zero-copy GPU uniform upload. |

---

## V16 Paper Queue Complete Evolution

| Change | Impact |
|--------|--------|
| Exp077 Michaelis-Menten PK | Capacity-limited (nonlinear) elimination model. Phenytoin reference (Ludden 1977). Dose-dependent half-life, supralinear AUC. |
| Exp078 Antibiotic perturbation | Gut microbiome disruption + recovery dynamics. Exponential kill model, Dethlefsen ciprofloxacin reference. |
| Exp079 SCFA production | Fiber-to-SCFA fermentation via Michaelis-Menten kinetics. Acetate, propionate, butyrate validated against Cummings 1987. |
| Exp080 Gut-brain serotonin | Tryptophan → 5-HT production via gut microbiota. Cross-track hypothesis D5 (Yano 2015). |
| Exp081 EDA stress detection | Tonic/phasic decomposition, skin conductance response peak detection. |
| Exp082 Arrhythmia classification | Template-matching beat typing: Normal, PVC, PAC, BBB. MIT-BIH annotation reference. |
| 6 Python control scripts | Exp077–082 each have `control_*.py` cross-validation. 167 total Python checks. |
| 14 Criterion benchmarks | `benches/paper_queue.rs` — MM PK, antibiotic, SCFA, serotonin, EDA, arrhythmia at 3 scales. |
| Paper queue 30/30 | All 30 reviewed papers now have validated experiments. |

---

## V15 Upstream Rewire Evolution

| Change | Impact |
|--------|--------|
| `PrecisionRouting` | Mirrors toadStool S128 precision dispatch. CPU f64, GPU split/emulated f64, NPU quantized. |
| `rng` delegate | LCG/xorshift delegate to canonical `barracuda::rng`. |
| `eigensolver` delegate | QL diagonalization delegates to `barracuda::special`. |
| Cross-spring shader docs | WGSL shader evolution path documented across springs. |
| Upstream parity benchmarks | Kokkos patterns validated against canonical upstream. |

---

## V13 Deep Audit Evolution

| Change | Impact |
|--------|--------|
| Anderson eigensolver | QL algorithm for correct eigenvalue/eigenvector computation from tridiagonal Hamiltonian |
| Smart clinical.rs refactor | 1177 → 374 + 819 lines, domain-coherent split |
| LCG PRNG centralization | New `rng.rs` module, 4 files updated |
| Math deduplication | `evenness_to_disorder` and `lognormal_params` delegate to canonical source |
| Capability-based discovery | Glob-based Songbird socket search replaces hardcoded path |
| Flaky test fix | AtomicU64 paths + kernel connection queuing replaces Barrier synchronization |
| Doc-tests | 4 added (shannon_index, hill_dose_response, auc_trapezoidal, state_to_f64) |

---

## Per-Person Clinical Translation

The critical addition: the pipeline from validated population models to individual patient scenarios.

### Pipeline

```
Published data → Computational model → Population validation
    → PatientTrtProfile (age, weight, T level, comorbidities)
        → trt_clinical_scenario() → 8-node graph + edges + channels + ranges
            → petalTongue clinical mode (motor commands: hide sidebars, skip awakening, fit view)
                → Clinician sees THIS patient's projected trajectory
```

### SAME DAVE Neuroanatomy (petalTongue Integration)

The SAME DAVE model (Sensory Afferent, Motor Efferent) provides self-aware control of the visualization system:

| Channel | Direction | healthSpring Use |
|---------|-----------|-----------------|
| Scenario load | Afferent | Patient scenario → graph topology |
| Mode preset | Efferent | `mode: "clinical"` → bundle of motor commands |
| Panel visibility | Efferent | `show_panels` → SetPanelVisibility motor commands |
| Awakening control | Efferent | `awakening_enabled: false` → skip startup animation |
| Zoom control | Efferent | `initial_zoom: "fit"` → FitToView motor command |
| IPC push | Afferent | JSON-RPC → scenario update without restart |

---

## Cross-Paper Dependencies

```
Paper 01 (Anderson-QS) ──→ gut lattice model (Exp011, 037)
Paper 06 (No-Till)     ──→ Anderson framework for biological substrates
Paper 12 (Immunological Anderson) ──→ veterinary→human PK bridge (Exp004), allometric scaling
Paper 08 (NPU Edge IoT) ──→ biosignal NPU target (Exp020-023)
Paper 07 (Sovereign WDM) ──→ GPU dispatch methodology, metalForge architecture
```

---

## Reading Order

**Standalone**: Paper 13 is self-contained for anyone interested in computational pharmacology or clinical decision support.

**In context**: 12 (immunological Anderson, veterinary PK lineage) → 13 (human health applications) → 01 (Anderson framework)

**Biosignal focus**: 13 §Track 3 (sovereign biosignal) → 08 (NPU edge IoT) → 04 (sentinels)

**Microbiome focus**: 13 §Track 2 (gut Anderson) → 01 (Anderson-QS) → 06 (no-till Anderson) → 03 (bioag microbiome)

---

## Data Sources

All validation data derives from published, open-access sources:

- Harman 2001 (BLSA testosterone decline)
- Saad 2013/2016, Traish 2014 (TRT registries)
- Sharma 2015 (VA cardiovascular cohort)
- Kapoor 2006 (RCT diabetes)
- Kabashima 2020, Silverberg 2021 (mAb Phase III)
- Gabrielsson & Weiner (PBPK reference model)
- Kleiger 1987 (HRV mortality landmark study)
- SRA public 16S datasets for microbiome baselines

No proprietary clinical data is used. Mok clinical reference provides hypotheses; registry data provides validation.

---

## V22 — biomeOS Niche Deployment

V22 transforms healthSpring from experiment binaries into a biomeOS BYOB niche:

| Component | File | Purpose |
|-----------|------|---------|
| Primal binary | `ecoPrimal/src/bin/healthspring_primal/` | 79 capabilities via JSON-RPC 2.0 over Unix socket |
| IPC dispatch | `ecoPrimal/src/ipc/dispatch/` | Method → science function routing for 6 domains |
| Niche manifest | `graphs/healthspring_niche.toml` | Declares the niche: primals + workflow graphs |
| Patient assessment | `graphs/healthspring_patient_assessment.toml` | ConditionalDag: 4 parallel science tracks → composite |
| TRT scenario | `graphs/healthspring_trt_scenario.toml` | Sequential TRT clinical workflow |
| Microbiome analysis | `graphs/healthspring_microbiome_analysis.toml` | Sequential diversity → Anderson → SCFA pipeline |
| Biosignal monitor | `graphs/healthspring_biosignal_monitor.toml` | Continuous 250 Hz real-time monitoring |

healthSpring is now a niche, not a node. The primal provides capabilities; the graphs define composition. biomeOS's Neural API orchestrates and optimizes via the Pathway Learner. See `wateringHole/SPRING_NICHE_SETUP_GUIDE.md` for how other springs can follow this pattern.

## V32 — Cross-Spring Ecosystem Convergence

V32 absorbs proven patterns from all ecoPrimals components:

| Feature | Source | Impact |
|---------|--------|--------|
| Structured `tracing` | All 6 sibling springs | `eprintln!` → `tracing::info!/warn!/error!` with env-filter (`RUST_LOG`) |
| `health.liveness` + `health.readiness` | coralReef Iter 51 | Lightweight probes for orchestrator health monitoring |
| Resilient provenance trio IPC | sweetGrass v0.7.18 | Circuit breaker (5s cooldown) + exponential backoff retry |
| `IpcError` ecosystem type | biomeOS/airSpring/groundSpring | Structured error enum with `RpcError` and `Timeout` variants |
| `OrExit<T>` trait | wetSpring V123 | Panic-free error handling for validation/utility binaries |
| Enriched `capability.list` | biomeOS Pathway Learner | `operation_dependencies` + `cost_estimates` for execution graph planning |

613 tests, 79 JSON-RPC capabilities, 6 WGSL shaders, zero clippy warnings, zero unsafe. Sovereign GPU dispatch via `CoralReefDevice`; IPC resilience: `CircuitBreaker`, `RetryPolicy`, `DispatchOutcome`. See `GROUNDSPRING_V114_PRIMAL_COMPOSITION_GUIDANCE_MAR17_2026.md` for composition guidance.

---

**License**: AGPL-3.0-or-later
