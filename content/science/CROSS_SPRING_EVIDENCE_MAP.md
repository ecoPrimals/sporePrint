+++
title = "Cross-Spring Evidence Map"
description = "How baseCamp papers draw from multiple springs and where independent implementations converge"
date = 2026-03-17
+++

# Cross-Spring Evidence Map

**How the baseCamp papers draw from multiple springs — and why that matters.**

The baseCamp papers are not single-spring results. Each paper draws validated
computational infrastructure from multiple springs, producing conclusions that
are more robust because they are validated from multiple directions.

This is what "sovereign scientific computing" means in practice: the same
mathematical framework (Anderson localization), implemented independently in
different domains, validated by independent experiments, producing convergent
predictions.

---

## The Anderson Thread

The most striking cross-spring pattern is Anderson localization — a condensed
matter physics framework (Anderson 1958) that appears independently across
five scientific domains:

```
Anderson localization (condensed matter physics)
    ↓
Predicted: signals propagate in 3D disordered media, localize in 2D

Validation across 5 independent biological domains:
    wetSpring    → Quorum sensing in microbial communities (Paper 01)
    wetSpring    → Cross-species signaling in symbiotic systems (Paper 05)
    wetSpring    → No-till soil health mechanism (Paper 06)
    wetSpring    → Cytokine propagation in skin tissue (Paper 12)
    hotSpring    → CG convergence proxy in lattice QCD (Paper 10)
    airSpring    → Tissue diversity in immunological models (Paper 12)
    groundSpring → Spectral theory validation (Exp 008, 012, 015, 018)
    healthSpring → Gut microbiome as Anderson lattice (Paper 13)

Single quantitative result across all domains:
    W_c = 16.26 ± 0.95 (critical disorder threshold, 3D)
    In 2D: always localizes regardless of W
```

The same parameter (W, disorder; d, dimension) governs signal behavior across
microbial ecology, immunology, soil science, plasma physics, and the gut
microbiome. This is either the most productive coincidence in computational
biology or a genuine unifying physical principle.

---

## Paper-by-Paper Cross-Spring Dependencies

### Paper 01 — Anderson Localization as QS Null Hypothesis

**Primary spring:** wetSpring (3,100+ checks, Exp107–356)

| Supporting Spring | What It Contributes | Key Experiments |
|------------------|--------------------|-----------------| 
| hotSpring | Spectral theory (Lanczos, level statistics), same Anderson math from plasma physics domain | Phase D lattice spectral |
| groundSpring | Spectral theory validation (Anderson 1D/2D/3D), transport models, uncertainty budgets | Exp 008, 009 |
| neuralSpring | ESN regime classifier — same Anderson transition classification at 96.5% accuracy | nW-05 |
| airSpring | Anderson coupling in soil diversity, cross-environment validation | Exp066–069 |

**Why this matters:** The 3D/2D threshold W_c = 16.26 ± 0.95 is validated from
three independent computational directions (wetSpring measurement, groundSpring
spectral theory, hotSpring physics). This is not a single-experiment result.

---

### Paper 06 — Anderson as the Mechanism Behind No-Till Soil Health

**Primary springs:** wetSpring Track 4 + airSpring

| Supporting Spring | What It Contributes | Key Experiments |
|------------------|--------------------|-----------------| 
| wetSpring | Anderson diversity measurements, 3D pore network QS modeling | Exp170–182 (321 checks) |
| airSpring | FAO-56 ET₀, soil moisture (Richards PDE), cover crop data | Exp066–078 |
| groundSpring | Uncertainty bridge: sensor noise → ξ (disorder) → r (level ratio) | Exp 015, rare biosphere |
| neuralSpring | LSTM time series for soil moisture, regime transition prediction | LSTM soil module |

**The mechanism:** Tillage = dimensional collapse (3D pore network → 2D surface
matrix). QS autoinducers go from propagating (3D extended) to localizing (2D
confined). Soil ecosystem services collapse because coordinated microbial activity
requires QS signal propagation. No-till preserves the 3D geometry and QS function.

**Independent validation:** Same dimensional collapse mechanism (Paper 12) appears
in AD skin (inverse direction: scratching = dimensional promotion). One physics
equation governs both.

---

### Paper 07 — Sovereign WDM Simulation on Consumer GPU

**Primary spring:** hotSpring (648+ checks)

| Supporting Spring | What It Contributes | Key Experiments |
|------------------|--------------------|-----------------| 
| neuralSpring | Surrogate models for WDM transport (5 WDM surrogates validated) | nS-WDM-01–05 |
| groundSpring | WDM precision/convergence/vendor-parity validation, uncertainty budgets | Exp 025–027 |
| groundSpring | Freeze-out inverse problem, spectral reconstruction, jackknife error bars | Exp 010, 011 |
| hotSpring | Anderson 4D + Wegner proxy pipeline, DF64 streaming | Exp 049–058 |

**The claim:** First lattice QCD production run (dynamical fermion, HMC) on a
consumer GPU (RTX 3090). Deconfinement at β_c = 5.69 confirmed. Smooth crossover.
The $0.044/run number validated by groundSpring uncertainty analysis.

---

### Paper 12 — Anderson in Immunological Signaling

**Primary springs:** wetSpring + neuralSpring + groundSpring

| Supporting Spring | What It Contributes | Key Experiments |
|------------------|--------------------|-----------------| 
| wetSpring | Anderson spectral tissue lattice, barrier disruption model, cytokine multi-compartment | Exp270–286 (157/157) |
| neuralSpring | Dose-response Hill (G2), PK decay (G4), ESN regime classification | nS-601–605 (329/329) |
| groundSpring | Spectral theory validation for tissue geometry, cytokine transport models | Exp 008, 012, 015, 018 |
| airSpring | GPU tissue diversity (GpuDiversity), CytokineBrain streaming | Exp066–069 (94/94) |
| healthSpring | JAK inhibitor PK/PD, Hill dose-response, three-compartment disorder | Track 1 + Track 7 |

**The connection to drug discovery:** 329/329 checks across five independent
implementations, four springs, two levels of validation (computational reproduction
+ cross-language parity). The Anderson-augmented MATRIX scoring (nS-605) is the
most thoroughly validated novel method in this whitepaper.

---

### Paper 13 — Sovereign Human Health Computing

**Primary spring:** healthSpring (73 experiments, 601+ tests)

| Supporting Spring | What It Contributes | Key Experiments |
|------------------|--------------------|-----------------| 
| wetSpring | Microbiome diversity → gut health Anderson W, QS gene profiling | diversity + QS modules |
| groundSpring | Uncertainty budgets (bootstrap/jackknife), spectral transport | uncertainty module |
| neuralSpring | ESN/LSTM anomaly detection, digester prediction (Paper 027) | nS-027 validated |
| airSpring | GpuDiversity, CytokineBrain (immune extension of agricultural modules) | Paper 12 integration |
| barraCuda | Canonical Hill, PopPK, diversity, MM batch, SCFA batch ops | Direct primal deps |

**The sovereignty angle:** NONMEM + Monolix + WinNonlin = ~$6,500/year in
software licenses for a pharmacometrics lab. healthSpring replaces all three,
runs 84× faster (CPU-only), and adds Anderson gut lattice modeling that no
commercial pharmacometric tool provides.

---

### Paper 17 — Game Design as Rigorous Science

**Primary spring:** ludoSpring (75 experiments, 1,692 checks)

| Supporting Spring | What It Contributes | Key Experiments |
|------------------|--------------------|-----------------| 
| wetSpring | Anderson W model (Perlin noise as disorder landscape), Python tolerance pattern | Anderson QS mapping |
| barraCuda | sigmoid, dot, lcg_step primitives consumed directly | Tier A GPU |
| toadStool | `compute.dispatch.*` for real-time GPU dispatch | Direct dispatch |
| healthSpring | Fitts/Hick models for medical UI evaluation (cross-domain) | Engagement metrics |
| neuralSpring | ESN reservoir for procedural generation, game AI | Transfer learning |

**The cross-domain finding:** The same provenance architecture that tracks
game item lineage (extraction shooters) tracks biological sample lineage
(field genomics) and medical record access (HIPAA consent). Fraud detection
is structurally identical across all three domains. This is not an analogy —
it is the same code path.

---

## The groundSpring Anomaly

groundSpring contributes to **every** baseCamp paper (Papers 01–22). This
is unusual for a spring whose domain (uncertainty quantification, spectral
theory, measurement noise) sounds narrow.

Why it contributes everywhere:

| groundSpring Capability | Universal Need |
|------------------------|----------------|
| Jackknife error bars | Any paper with experimental uncertainty |
| Rare biosphere quantification | Any microbiome paper (what you can't detect matters) |
| Spectral theory validation | Anderson framework underlying 5+ papers |
| Uncertainty bridge: sensor → physics | Any paper with measurement data |
| WDM precision/convergence | Lattice QCD, plasma physics |
| NPU Anderson classification (AKD1000) | Any paper with edge deployment |

The lesson: uncertainty is not a single domain. It is the connective tissue
between all quantitative science. A spring that handles uncertainty well
contributes everywhere.

---

## Convergent Predictions: Where Multiple Springs Agree

These are the results where two or more springs independently arrive at
the same number:

| Result | Spring 1 | Spring 2 | Spring 3 | Agreement |
|--------|----------|----------|----------|-----------|
| W_c ≈ 16.26 (Anderson 3D) | wetSpring (measured) | groundSpring (spectral theory) | hotSpring (physics) | < 5% variation |
| Anderson in 2D → always localizes | wetSpring (QS) | groundSpring (math) | Paper 12 (tissue) | Exact |
| DF64 9.9× vs native f64 | hotSpring (benchmark) | groundSpring (parity) | coralReef (compile) | Exact |
| ESN regime classifier >96% | neuralSpring (training) | wetSpring (application) | hotSpring (QCD proxy) | Cross-domain |
| Rust 84–160× faster than Python | healthSpring (PK/PD) | airSpring (ET₀, 13K×) | wetSpring (spectral, 1077×) | Operation-dependent |

When three independent springs produce the same result, the result is robust.
Convergent predictions across independent implementations are stronger evidence
than any single spring alone.

---

## What Springs Have Not Yet Contributed To

These papers are architecturally defined but missing wet-lab validation:

| Paper | Missing Component | Spring | Path |
|-------|-----------------|--------|------|
| 02 (LTEE) | Frozen fossil sequencing | wetSpring | MinION + lab collaboration |
| 04 (Sentinels) | Real-time HAB deployment | wetSpring (NPU live) | AKD1000 live on hardware |
| 09 (Field Genomics) | MinION nanopore sequencer | wetSpring | Hardware pending |
| 12 (Immuno-Anderson) | iPSC validation | healthSpring + Gonzales lab | Wet lab collaboration |
| 03 (BioAg) | Pistachio/almond field trial | wetSpring + airSpring | Field partner |

The computational predictions are validated. The wet-lab tests are the open frontier.

---

*Spring versions at time of writing: wetSpring V127, airSpring v0.8.9,
neuralSpring S162, hotSpring v0.6.31, groundSpring V114, healthSpring V35,
ludoSpring V24.*
