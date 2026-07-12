+++
title = "Anderson in Immunological Signaling"
description = "Immunology x Physics — Anderson localization in immune signaling, drug repurposing pipeline. healthSpring. 329/329 checks."
date = 2026-03-17

[extra]
paper_number = 12
domain = "Human Health"

[taxonomies]
springs = ["groundspring", "healthspring", "neuralspring", "wetspring"]
+++

**Date:** March 2, 2026 (Sessions 105–108)
**Status:** Computational implementation COMPLETE — all nS-601..605 experiments validated. {{ entity(name="wetspring") }} V92D+: Exp273-279 (157/157 immunological Anderson) + Gonzales reproductions (Exp280-286: 202/202) full three-tier. Paper-math chain complete: Exp291 Paper Control v4 (45/45) includes Gonzales P42-P47 (IC50, PK, IL-31, pruritus, three-compartment, selectivity). CPU v22 validates Hill/PK/Anderson in 0.8ms. GPU v9 proves portability. Streaming v9 confirms W↔P(QS) r=-0.924. {{ entity(name="metalforge") }} v14 validates cross-system.
Gonzales modeling (Hill dose-response, PK decay, pruritus time-series), 3D tissue lattice
(multi-layer Hamiltonian, barrier promotion spectrum, three-compartment disorder), and
Fajgenbaum MATRIX scoring (6 drug candidates, pathway × geometry × disorder) fully
implemented and cross-validated (Python 48/48 + Rust 240/240 + 27 unit tests, GPU 4,
dispatch 3, mixed hardware 7). S108: module refactored (1023→3 files: mod.rs + lattice.rs
+ matrix.rs), provenance wired, scripts synced, doc sweep complete.
**Domain:** Immunology × condensed matter physics × pharmacology × drug repurposing
**Novelty:** No prior work applies Anderson localization to cytokine signal
propagation in tissue; no prior work adds spatial geometry to drug repurposing scoring
**Cross-Spring:** {{ entity(name="wetspring") }} (Anderson spectral) × {{ entity(name="neuralspring") }} (ESN regime classifier,
LSTM time series) × {{ entity(name="groundspring") }} (transport, uncertainty, spectral validation)

---

## Abstract

We extend the Anderson localization framework from microbial quorum sensing
(Papers 01, 05, 06) to immunological cytokine signaling in skin tissue. The
core observation: Th2 cytokines (IL-4, IL-13, IL-31) are diffusible signals
propagating through a disordered biological medium (heterogeneous skin tissue
with mixed cell populations). The same physics that governs autoinducer
propagation through microbial communities governs cytokine propagation through
inflamed tissue.

We map the atopic dermatitis (AD) disease cycle — allergen exposure → Th2
activation → cytokine release → neuro-immune itch signaling → barrier
disruption → amplification — onto the Anderson framework and show that barrier
disruption constitutes a *dimensional promotion* (inverse of the tillage
dimensional collapse in Paper 06): scratching opens 3D diffusion channels
through normally 2D-barrier skin, enabling cytokine signal delocalization.

We then connect this to the Fajgenbaum drug repurposing paradigm (MATRIX,
ARPA-H $48.3M) by adding a spatial geometry dimension to pathway-based
drug-disease scoring: a drug must both (a) target the right pathway AND
(b) physically reach its target through tissue geometry. Anderson localization
quantifies condition (b).

---

## 1. Source Literature — The Gonzales Catalog

### 1.1 Publications to Ingest

All authored or co-authored by Andrea J. Gonzales (Zoetis → MSU Pharmacology
& Toxicology, 2025–present). These constitute the experimental foundation
for the immunological Anderson extension.

| # | Citation | Key Data | Spring Target |
|---|----------|----------|---------------|
| G1 | Gonzales AJ et al. (2013) "Interleukin-31: its role in canine pruritus and naturally occurring canine atopic dermatitis." *Vet Dermatol* 24:48-53 | IL-31 elevated in AD dog serum; IV IL-31 induces pruritus in beagles; IL-31 activates peripheral nerves | {{ entity(name="wetspring") }}: IL-31 as diffusible signal, W mapping from tissue heterogeneity |
| G2 | Gonzales AJ et al. (2014) "Oclacitinib (APOQUEL) is a novel JAK inhibitor with activity against cytokines involved in allergy." *J Vet Pharmacol Ther* 37:317-324 | JAK1 IC50 = 10 nM; blocks IL-2, IL-4, IL-6, IL-13, IL-31 (IC50 36-249 nM); minimal off-target | {{ entity(name="neuralspring") }}: dose-response modeling, IC50 as Anderson barrier height |
| G3 | Gonzales AJ et al. (2016) "IL-31-induced pruritus in dogs: a novel experimental model." *Vet Dermatol* 27:34-e10 | Standardized IL-31 pruritus model in beagles; oclacitinib superior to prednisolone/dexamethasone at 1, 6, 11, 16 hr | {{ entity(name="wetspring") }}: time-series pruritus data for LSTM; model as controlled Anderson perturbation |
| G4 | Fleck TJ,...,Gonzales AJ (2021) "Onset and duration of action of lokivetmab in IL-31 induced pruritus." *Vet Dermatol* 32:681-e182 | Cytopoint: 3 hr onset, dose-dependent duration (14/28/42 days at 0.125/0.5/2.0 mg/kg); lab model correlates with clinical field trials | {{ entity(name="neuralspring") }}: pharmacokinetic decay as signal extinction; ESN classifier for regime transitions |
| G5 | Gonzales AJ et al. (2024) "Oclacitinib is a selective JAK1 inhibitor with efficacy in canine flea allergic dermatitis." *J Vet Pharmacol Ther* 47:447-453 | JAK1 selectivity confirmed in different allergic model | {{ entity(name="wetspring") }}: cross-disease validation of same Anderson pathway |
| G6 | McCandless EE, Rugg CA, Fici GJ et al. (2014) "Allergen-induced production of IL-31 by canine Th2 cells and identification of immune, skin, and neuronal target cells." *Vet Immunol Immunopathol* 157:42-48 | IL-31 produced by Th2 cells after allergen presentation by Langerhans cells; target cells = immune, skin, neuronal | {{ entity(name="wetspring") }}: cell-type heterogeneity → disorder W; three-compartment Anderson lattice |

### 1.2 Companion Literature (Not Gonzales-Authored)

| # | Citation | Relevance |
|---|----------|-----------|
| F1 | Fajgenbaum DC et al. (2019) "Identifying and targeting pathogenic PI3K/AKT/mTOR signaling in IL-6 blockade–refractory iMCD." *J Clin Invest* | Proves pathway-based drug repurposing; mTOR cross-talks with JAK/STAT |
| F2 | Every Cure / MATRIX — ARPA-H $48.3M (2024) | 4,000 drugs × 18,000 diseases = 75M pairs scored. Open-source platform. |
| D1 | Simpson et al. (2020) "Dupilumab Phase 3 trials." *N Engl J Med* | Human anti-IL-4Rα for AD — blocks IL-4 + IL-13. Cross-species validation of Gonzales's canine work |
| D2 | Silverberg et al. (2023) "JAK inhibitors in AD." *J Am Acad Dermatol* | Upadacitinib, abrocitinib for human AD — human equivalents of Apoquel |
| N1 | Oetjen et al. (2023) "Sensory neurons co-opt immune cells for AD pathogenesis." *Cell* | IL-4/IL-13 directly sensitize sensory neurons — neuro-immune axis |
| N2 | Cohen et al. (2022) "Neuro-immune interactions in AD." *Sci Immunol* | Bidirectional neuron-immune cell communication in skin |

---

## 2. The Anderson Mapping

### 2.1 Tissue as Anderson Lattice

| Anderson QS (Paper 01) | Immunological Extension |
|------------------------|------------------------|
| Lattice site | Cell position in tissue |
| On-site energy ε_i | Cell type identity (keratinocyte, Th2, neuron, mast cell, eosinophil) |
| Hopping parameter t | Cytokine diffusion coefficient in extracellular matrix |
| Disorder W | Cell-type heterogeneity (Pielou evenness of cell population) |
| Dimension d | Tissue geometry (epidermis ≈ 2D barrier; dermis ≈ 3D matrix) |
| Level spacing ratio r | Diagnostic: cytokine signal extended (propagating) vs localized (confined) |

### 2.2 Skin Layer Geometry

| Layer | Thickness | Geometry | Cell types | Anderson prediction |
|-------|-----------|----------|------------|---------------------|
| Stratum corneum | 10-20 µm | 2D barrier, dead cells | None (acellular) | Impermeable — no signal propagation |
| Viable epidermis | 50-100 µm | Quasi-2D (4-8 cell layers) | Keratinocytes, Langerhans cells, melanocytes | Low d_eff (2-2.5) → signals localize → contained |
| Basement membrane | <1 µm | 2D boundary | Structural | Barrier between compartments |
| Papillary dermis | 100-200 µm | 3D matrix (collagen + vessels + nerves) | Fibroblasts, Th2 cells, mast cells, eosinophils, dendritic cells, nerve endings | d = 3 → signals propagate → cytokine signaling active |
| Reticular dermis | 1-3 mm | 3D dense matrix | Fibroblasts, vessels | d = 3, low W → deep extended regime |

### 2.3 The AD Disease Cycle as Anderson Phase Transitions

```
Healthy skin:
  Epidermis (2D) → cytokines LOCALIZED (contained, homeostatic)
  Dermis (3D) → cytokines EXTENDED (but low production = no pathology)

AD initiation:
  Allergen → Langerhans → Th2 → IL-4, IL-13, IL-31 production in dermis
  Dermis (3D) → cytokines PROPAGATE to sensory nerve endings → ITCH

Barrier disruption (scratching):
  Epidermis physically breached → NEW 3D channels through barrier
  d_eff of epidermal layer INCREASES (2D → quasi-3D)
  Cytokines now propagate from dermis THROUGH barrier to surface
  External allergens now penetrate INTO dermis
  = DIMENSIONAL PROMOTION (inverse of Paper 06 tillage collapse)

Chronic AD:
  Persistent 3D channels → persistent signal delocalization
  Th2 amplification loop → increasing W (more immune cell types infiltrate)
  BUT still below W_c in 3D → signals KEEP propagating → chronic inflammation

Treatment:
  Cytopoint: removes IL-31 molecule → no signal to propagate (signal elimination)
  Apoquel: blocks JAK1 receptor → cells can't respond even if signal arrives (transduction block)
  Barrier repair: restores 2D epidermis → Anderson localization re-confines signals (geometry intervention)
  Dupilumab: blocks IL-4Rα → eliminates IL-4 + IL-13 simultaneously (receptor block)
```

### 2.4 The Dimensional Promotion–Collapse Duality

Paper 06 (no-till): Tillage is dimensional COLLAPSE (3D → 2D) → QS fails →
soil ecosystem services collapse.

Paper 12 (AD): Scratching is dimensional PROMOTION (2D → 3D) → cytokine
signaling delocalizes → inflammatory cascade amplifies.

**Same physics, opposite direction, opposite outcome:**
- In soil: losing 3D = losing coordination = BAD
- In AD skin: gaining 3D = gaining pathological propagation = BAD

The Anderson framework is agnostic — it predicts signal propagation. Whether
propagation is beneficial or pathological depends on the biological context.

---

## 3. The Fajgenbaum Bridge — Geometry-Aware Drug Repurposing

### 3.1 Standard MATRIX Score

Fajgenbaum's MATRIX: Score(drug, disease) = f(pathway overlap, literature
evidence, molecular similarity, clinical data)

This is pathway-only. It asks: "Does the drug hit a relevant target?"

### 3.2 Anderson-Augmented Score

Anderson extension: Score(drug, disease, tissue) = f(pathway overlap) ×
g(tissue geometry, drug delivery route, molecular size)

The geometry factor g() encodes:
- Can the drug physically reach the target cell in the relevant tissue?
- What is the effective Anderson dimension of the target tissue?
- Does the drug need to cross a 2D barrier (epidermis) to reach a 3D
  compartment (dermis)?
- Large molecules (mAbs like Cytopoint): systemic delivery → 3D dermal
  access → good. Topical delivery → 2D barrier blocks → poor.
- Small molecules (oclacitinib): oral → systemic → 3D dermal access.
  Topical → can penetrate barrier → reaches both compartments.

### 3.3 Repurposing Targets for AD (Anderson-Filtered)

| Drug (Original Use) | Pathway | Anderson Geometry Score | Repurposing Logic |
|---------------------|---------|----------------------|-------------------|
| Rapamycin/sirolimus (transplant) | mTOR (cross-talks JAK/STAT via PI3K/AKT) | HIGH — small molecule, systemic, reaches 3D dermis | mTOR activated downstream of IL-4/IL-13 in keratinocytes; Fajgenbaum proved rapamycin works for cytokine storms |
| Tofacitinib (RA) | JAK1/JAK3 | HIGH — already confirmed in human AD trials | Direct pathway match — human equivalent of Apoquel |
| Tanezumab (OA pain, Phase 3) | Anti-NGF mAb | HIGH — systemic mAb reaches 3D dermis | NGF elevated in AD skin; Gonzales's team already proved anti-NGF works in OA (Librela/Solensia) |
| Trametinib (melanoma) | MEK/ERK (downstream IL-31RA) | MODERATE — systemic, but MEK inhibition has broad effects | ERK pathway activated by IL-31; could modulate keratinocyte dysfunction |
| Crisaborole (mild AD, topical) | PDE4 | LOW → MODERATE — topical, must cross 2D barrier | Already approved for AD but limited by penetration; Anderson predicts better efficacy in barrier-compromised skin |
| Nemolizumab (prurigo nodularis) | Anti-IL-31RA mAb | HIGH — systemic, targets same receptor as Cytopoint | Direct IL-31 pathway; human equivalent of Cytopoint approach |

---

## 4. Spring Integration Plan

### 4.1 wetSpring Experiments (Proposed)

| Exp | Description | Validates |
|-----|-------------|-----------|
| Exp 270 | Anderson lattice with skin-layer geometry: 2D epidermis (L=5-8) + 3D dermis (L=20) + barrier interface. Compute r for cytokine propagation across layers | Core Anderson prediction for immunological signaling |
| Exp 271 | Barrier disruption model: remove sites from 2D epidermal layer → measure r transition as d_eff increases → quantify "dimensional promotion" threshold | AD scratch cycle as inverse of Paper 06 tillage collapse |
| Exp 272 | Cell-type heterogeneity sweep: vary W (immune cell diversity) in 3D dermal compartment → confirm cytokine signals remain extended up to W_c | Prediction that inflammation increases W but stays below W_c in 3D |
| Exp 273 | NCBI Protein search: IL-31RA, IL-4Rα, OSMR expression in skin tissue metagenomes → map receptor distribution as lattice site occupancy | Empirical lattice construction from gene expression data |
| Exp 274 | Cross-species skin comparison: canine (thin epidermis) vs human (thick epidermis) → different d_eff barriers → different Anderson predictions for cytokine propagation depth | One Health Anderson comparison — validates Gonzales's comparative approach |

### 4.2 neuralSpring Connections

| Component | Application |
|-----------|-------------|
| ESN regime classifier (nW-05, 96.5%) | Classify AD skin state (healthy/flare/chronic/treated) from cytokine profile → Anderson regime |
| LSTM time series (nW-03, R²=0.98) | Predict pruritus score r(t) from treatment + time post-dose → model Cytopoint/Apoquel pharmacodynamics |
| Dose-response modeling | IC50 curves for JAK inhibitors as Anderson barrier heights: drug concentration maps to effective W reduction |

### 4.3 groundSpring Connections

| Experiment | Application |
|------------|-------------|
| Exp 012 — Spin chain transport | Models cytokine signal propagation distance through linear tissue channels (nerve tracts, vessels) |
| Exp 008 — Anderson localization | Validates 2D/3D spectral diagnostics used for skin compartment classification |
| Exp 015 — Uncertainty bridge | Sensor noise → cytokine measurement uncertainty → Anderson regime classification confidence |
| Exp 018 — Band edge structure | Tissue periodicity (epidermal cell layers) creates band gaps for cytokine propagation — predicts frequency-dependent signal filtering |

### 4.4 Reproduction Targets

| Paper | Spring | What to Reproduce | Why |
|-------|--------|-------------------|-----|
| Gonzales (2014) — Oclacitinib JAK1 selectivity | {{ entity(name="neuralspring") }} | IC50 dose-response curves for JAK1 vs JAK2 vs JAK3 | Quantify pathway specificity as Anderson parameter |
| Gonzales (2016) — IL-31 pruritus model | {{ entity(name="wetspring") }} + {{ entity(name="neuralspring") }} | Time-series pruritus scores (1, 6, 11, 16 hr) for oclacitinib vs steroids | Validate LSTM prediction of treatment response |
| Fleck/Gonzales (2021) — Lokivetmab pharmacodynamics | {{ entity(name="neuralspring") }} | Dose-dependent duration curves (0.125/0.5/2.0 mg/kg) | Pharmacokinetic decay as signal extinction in Anderson model |
| McCandless (2014) — IL-31 cell targets | {{ entity(name="wetspring") }} | Three-compartment lattice (immune + skin + neural target cells) | Empirical basis for multi-compartment Anderson lattice |

---

## 5. Computational Results (Session 107)

### 5.1 nS-601: Gonzales Dose-Response Modeling

All 6 Gonzales cytokine pathways (G2) modeled via generalized Hill equation
`response = E_max × [drug]^n / ([drug]^n + IC50^n)`. Validated n=1 (standard)
and n=2 (cooperative) forms. Cytokine-specific barrier heights computed as
`W = ln(IC50) × scale`:

| Pathway | IC50 (nM) | Barrier W | Interpretation |
|---------|-----------|-----------|----------------|
| JAK1    | 10        | 2.303     | Lowest barrier — most potent target |
| IL-2    | 36        | 3.584     | |
| IL-6    | 36        | 3.584     | |
| IL-31   | 63        | 4.143     | Key pruritus pathway |
| IL-4    | 159       | 5.069     | |
| IL-13   | 249       | 5.517     | Highest barrier — least sensitive |

**Result:** Barrier height ordering JAK1 < IL-31 < IL-13 confirmed
computationally. All 6 dose-response sweeps monotonically increasing.
Saturation at 1000× IC50 > 99.9%. Python 5/5 checks, Rust 80+ cross-language
parity checks — all PASS.

### 5.2 nS-602: Pruritus Time-Series (Gonzales 2016 G3)

Treatment effect modeled as exponential recovery from initial suppression:
`score(t) = nadir + (baseline - nadir) × (1 - exp(-decay_rate × t))`

- Baseline: 8.0 (untreated clinical score)
- Suppression: 70% (oclacitinib peak effect)
- Nadir: 2.4 at t=0 post-dose
- Asymptote → baseline at t→∞

Time-series validated at 0, 24, 72, 168, 336, 672 hours: monotonically
recovering toward baseline. Cross-language parity to 1e-10.

### 5.3 nS-603: Lokivetmab Pharmacokinetics (Fleck/Gonzales 2021 G4)

- **PK decay:** `C(t) = C_0 × exp(-k × t)` where `k = ln(2)/half_life`
- **Duration regression:** `duration = 10.10 × ln(dose) + 35.00`
  - G4 data is perfectly log-linear (equal spacing in ln-dose and duration)
  - Exact fit: R² = 1.0, zero residual at all 3 dose levels
  - Monotonically increasing with dose (validated 0.05–4.0 mg/kg)

| Dose (mg/kg) | Actual (days) | Predicted (days) | Error |
|:------------:|:-------------:|:----------------:|:-----:|
| 0.125        | 14.0          | 14.0             | 0.00  |
| 0.5          | 28.0          | 28.0             | 0.00  |
| 2.0          | 42.0          | 42.0             | 0.00  |

**Errata:** Prior version reported intercept = 33.28 with constant ~1.7-day
bias (R² = 0.971). The G4 dose-duration data is perfectly collinear in
log-space — 3 points, 2 parameters, zero residual is expected. The bias
was a regression initialization error (intercept off by 1.72). Corrected
to slope = 10.10, intercept = 35.00.

### 5.4 nS-604: Three-Compartment Tissue Lattice (3D Systems)

McCandless (2014) G6 three-compartment extension implemented:

| Compartment | Healthy Cell Fractions | Pielou J | Disorder W |
|-------------|----------------------|:--------:|:----------:|
| Immune (Th2, mast, eo, DC) | [0.25, 0.25, 0.25, 0.25] | 1.000 | 10.00 |
| Skin (keratinocytes, LC) | [0.80, 0.10, 0.05, 0.05] | 0.511 | 5.11 |
| Neural (sensory, motor) | [0.50, 0.50] | 1.000 | 10.00 |

Cross-compartment variance = 5.31 (healthy) vs 0.03 (inflamed) — inflammation
homogenizes disorder across compartments, enabling cross-compartment cytokine
propagation.

**Tissue lattice Hamiltonian:** Multi-layer Anderson matrices with configurable
layer sizes and per-layer disorder. Symmetric, real eigenvalues, finite.

**Barrier promotion spectrum:** 5-step sweep from intact (d=2.0) to fully
breached (d=3.0). Level spacing ratio r transitions through the Anderson
critical region. All d_eff ∈ [2, 3], all r ∈ [0, 1].

### 5.5 nS-605: Fajgenbaum MATRIX Drug Repurposing

Anderson-augmented scoring: `combined = pathway × geometry × (1 - 0.3 × W)`.
Six drug candidates evaluated against AD flare and chronic profiles.

**AD Flare Profile** (barrier_breach=0.4, d_eff=2.7, W=0.75):

| Rank | Drug | Pathway | Geometry | Combined |
|:----:|------|:-------:|:--------:|:--------:|
| 1 | **Tofacitinib** | 0.920 | 0.775 | **0.713** |
| 2 | Rapamycin | 0.850 | 0.774 | 0.658 |
| 3 | Nemolizumab | 0.900 | 0.663 | 0.596 |
| 4 | Tanezumab | 0.780 | 0.660 | 0.515 |
| 5 | Trametinib | 0.650 | 0.775 | 0.503 |
| 6 | Crisaborole | 0.700 | 0.713 | 0.499 |

**Key findings:**
- Tofacitinib ranks #1 for both flare and chronic AD — direct pathway match
  (human equivalent of Apoquel) combined with small molecule geometry advantage
- Large mAbs (Tanezumab 148kDa, Nemolizumab 145kDa) penalized by geometry factor
  despite strong pathway scores
- Crisaborole (topical, 0.251kDa) benefits from chronic barrier breach
  (chronic geom 0.730 > flare geom 0.713)
- Trametinib ranks #5 — MEK pathway mismatch reduces overall score
- Score factorization `combined = pathway × geometry_eff` verified for all
  candidates to 1e-10

**Integrated score:** Dose-response at 100 nM × MATRIX = 0.909 × 0.713 = 0.648
for Tofacitinib — demonstrating full pipeline from concentration to repurposing
recommendation.

### 5.6 Validation Summary

| Tier | Checks | Status |
|------|:------:|:------:|
| Python baseline (original) | 20/20 | PASS |
| Python baseline (extended) | 28/28 | PASS |
| Rust cross-language (original) | 53/53 | PASS |
| Rust cross-language (extended) | 187/187 | PASS |
| Rust unit tests | 27/27 | PASS |
| BarraCuda GPU | 4/4 | PASS |
| Compute dispatch | 3/3 | PASS |
| Mixed hardware ({{ entity(name="nucleus") }}) | 7/7 | PASS |
| **Total** | **329** | **ALL PASS** |

---

## 6. Cross-Paper Connections

### Paper 01 → Paper 12
Anderson QS in microbial communities → Anderson cytokine signaling in tissue.
Same math, different biology. The level spacing ratio r, disorder W, dimension d,
and W_c all transfer directly.

### Paper 04 → Paper 12
Sentinel microbes detect environmental perturbation via Anderson regime shift.
Paper 12 extends: immune cell populations detect disease perturbation (AD flare)
via the same Anderson regime shift. The ESN classifier (validated on AKD1000)
can classify AD tissue state from cytokine measurements.

### Paper 05 → Paper 12
Cross-species signaling in symbiotic systems (lichen, coral, rhizobia).
Paper 12 extends: cross-cell-type signaling in immunological systems (Th2 → neuron,
mast cell → keratinocyte, eosinophil → fibroblast). Same Anderson geometry
governs whether signals reach their cross-type targets.

### Paper 06 → Paper 12
No-till = dimensional collapse → QS fails → ecosystem services lost.
AD scratching = dimensional promotion → cytokine delocalization → pathological cascade.
Same physics, opposite direction. Duality documented.

---

## 7. Gonzales Lab + MSU Drug Discovery Collaboration

The Gonzales lab is the biological validation layer. The MSU Drug Discovery
program (ADDRC) is the screening infrastructure. Together they create a
complete pipeline from computational prediction to experimental validation.

**Gonzales Lab:**
- Empirical cytokine data (IL-31 pruritus scores, dose-response curves)
- iPSC-derived skin models across species (canine, feline, human)
- Plate-based screening infrastructure
- Drug discovery pipeline and regulatory expertise

**MSU Drug Discovery — ADDRC (Erika Lisabeth, Director):**
- High-throughput screening facility in the same department (Pharm & Tox)
- 8,000+ compound library, liquid-handling robots, plate readers
- HTS assay development and drug repurposing expertise
- GREENScreen informatics for compound management and data analysis

**MSU Drug Discovery — Additional (Richard Neubig, Edmund Ellsworth):**
- Neubig: Rho/MRTF/SRF inhibitors for skin fibrosis and melanoma —
  potential cross-talk with JAK/STAT in AD barrier models
- Ellsworth: Medicinal chemistry optimization downstream of HTS hits

**What we bring:**
- Anderson localization spatial modeling (no one else has this for immunology)
- Fajgenbaum MATRIX drug repurposing with tissue geometry scoring (nS-605)
- Statistical analysis and ML (data science for screening data)
- Bioinformatics (sequencing analysis, NCBI pipelines)
- Automated data workflows
- GPU/NPU-accelerated diversity + regime classification

**The pipeline:**

```
Anderson-augmented MATRIX scores (nS-605, 6 candidates scored)
    → ADDRC high-throughput screening (Lisabeth, 8,000+ compounds)
    → iPSC skin model validation (Gonzales, canine/feline/human)
    → Medicinal chemistry optimization (Ellsworth)
    → Pre-clinical development
```

---

## 8. Open Questions for Spring Evolution

1. What is W for inflamed vs healthy dermal tissue? (Need: single-cell
   transcriptomics data to compute Pielou evenness of cell populations)
2. What is the effective d_eff of barrier-disrupted epidermis? (Need:
   3D imaging of AD skin to quantify channel geometry)
3. Does the Anderson W_c hold for cytokine propagation as it does for
   QS autoinducers? (Need: diffusion coefficient data for IL-31 in ECM)
4. Can the ESN regime classifier distinguish AD flare from healthy skin
   using cytokine panel data? (Need: published cytokine profiling datasets)
5. Does rapamycin's efficacy in cytokine storms (Fajgenbaum) predict
   efficacy in AD via the mTOR/JAK cross-talk? (Testable with Gonzales's
   iPSC models + ADDRC screening infrastructure)
6. Can the Anderson-augmented MATRIX scoring (nS-605) guide compound
   selection from the ADDRC's 8,000+ library for AD-specific screening?
   (Need: ADDRC compound metadata + Gonzales iPSC validation assay)
7. Does Neubig's Rho/MRTF/SRF skin fibrosis pathway cross-talk with
   JAK/STAT in AD barrier disruption? (Testable: screen Rho inhibitors
   in Gonzales iPSC models, score with Anderson geometry)
8. Can the PK duration model (nS-603) be refined with additional dose
   levels from companion-animal clinical data? (Need: lokivetmab outcomes
   at doses beyond the 3 published in G4)

---

## 9. Reading Order for This Paper

**For an immunologist**: §2 (Anderson mapping) → §3 (Fajgenbaum bridge) → §1 (source literature) → §4 (Spring experiments)

**For a Spring developer**: §4 (integration plan) → §2 (the mapping) → §5 (cross-paper connections) → §1 (papers to reproduce)

**For a drug discovery researcher**: §3 (Fajgenbaum bridge) → §2.3 (AD disease cycle) → §6 (collaboration potential) → §1 (Gonzales catalog)
