+++
title = "Anaerobic-Aerobic QS"
description = "Microbial Ecology x Quorum Sensing — anaerobic-aerobic transition modeling via Anderson framework. wetSpring."
date = 2026-03-17

[extra]
paper_number = 16
domain = "Microbiology and Ecology"
+++

# Paper 16: Anaerobic-Aerobic QS Phase Transition — Microbial Signaling Across Oxygen Regimes

**Status**: Stage 1 in progress — computational foundation complete (V108). **neuralSpring Paper 027 benchmark COMPLETE (S142)**: ESN methane yield predictor, 36/36 CPU + 23/23 bC/gT PASS
**Date**: March 10, 2026
**Literature Anchor**: Wei Liao (ADREC, MSU BAE)
**Springs**: wetSpring (QS framework), healthSpring (anaerobic gut), airSpring (soil O₂ zones), neuralSpring (ML), groundSpring (spectral)
**Bench Source**: MSUBI bioreactor experience (5 years), ADREC interview (2024)

---

## The Question

Quorum sensing is primarily studied in aerobic organisms — *Vibrio*, *Pseudomonas*,
aerobic biofilms. The Anderson-QS model (Paper 01) maps QS propagation to
Anderson localization in disordered lattices, with the disorder landscape W
determined by species diversity, spatial geometry, and signal attenuation.

But most microbial environments are not uniformly aerobic. Anaerobic digesters
are fully oxygen-absent. The gut lumen is largely anaerobic with oxygen
gradients at the mucosal interface. Soil has aerobic and anaerobic zones
determined by water saturation — a flooded pore is anaerobic, a drained pore
is aerobic. Hydrothermal vents range from micro-aerophilic to strictly
anaerobic.

**What happens to quorum sensing when the same organisms face aerobic vs
anaerobic conditions?**

This is not a trivial environmental variable. Oxygen availability triggers
global transcriptional reprogramming:

- **FNR** (fumarate and nitrate reduction regulator) — the master anaerobic
  switch in Enterobacteriaceae, controlling >100 genes
- **ArcAB** (aerobic respiration control) — two-component system that
  represses aerobic metabolism genes under anaerobic conditions
- **Rex/ResDE** — redox-sensing regulators in Firmicutes (Bacillus, Clostridia)

If these global regulators also modulate QS gene expression — autoinducer
synthase, receptor expression, signal transduction cascades — then switching
from aerobic to anaerobic doesn't just change metabolism. It changes the
communication network. The Anderson disorder landscape W is not a fixed
property of the community; it undergoes a phase transition when oxygen
is removed.

---

## The Hypothesis

In the Anderson-QS model, the disorder parameter W encodes the heterogeneity
of signaling efficiency across the microbial community lattice. The localization
length ξ determines whether signals propagate (extended state, ξ >> L) or
attenuate (localized state, ξ << L).

**Hypothesis**: Anaerobic conditions cause a measurable shift in W for
facultative anaerobic communities, driven by transcriptional reprogramming
of QS genes under FNR/ArcAB/Rex regulation. This shift may increase
or decrease localization, depending on:

1. Whether anaerobic QS autoinducers have different diffusion coefficients
   than aerobic ones (AI-2 is universal, but AHL production may be
   oxygen-dependent)
2. Whether receptor density changes under anaerobic gene regulation
3. Whether the spatial organization of the community changes (biofilm
   architecture differs under anaerobic conditions)
4. Whether cross-species signal interference increases or decreases
   when the autoinducer repertoire changes

**Prediction 1**: Communities dominated by facultative anaerobes show
bimodal W distributions — one mode for aerobic, one for anaerobic —
with a transition zone at intermediate oxygen levels.

**Prediction 2**: Strictly anaerobic communities (Clostridia, methanogens)
have evolved QS systems optimized for their W regime, distinct from
aerobic QS systems even when the mathematical framework is identical.

**Prediction 3**: The gut mucosal oxygen gradient creates a spatial
gradient in W — localized near the mucosal surface (aerobic, diverse
signals) and extended in the lumen (anaerobic, reduced autoinducer
repertoire, lower effective W).

---

## Why This Matters

### For ADREC (anaerobic digestion)

Digester performance depends on microbial community stability. Process
upsets — overloading, temperature shocks, toxic substrate — destabilize
the community. If QS coordinates community behavior in digesters, then
understanding anaerobic QS dynamics could:

- Predict process upsets before they crash biogas yield
- Design inoculants that communicate effectively in anaerobic conditions
- Explain why co-digestion (mixed substrates) sometimes stabilizes and
  sometimes destabilizes community function

Liao's group has the controlled environments and the community sequencing
data. This model gives a quantitative framework for interpreting it.

### For the gut (healthSpring)

The gut is an anaerobic digester. The same microbial ecology that
determines biogas yield in an ADREC digester determines nutrient
extraction, immune modulation, and pathogen resistance in the gut.
The healthSpring Anderson gut lattice (Exp032) models the colon
as a disordered system — Paper 16 explains why W differs between
the aerobic mucosal surface and the anaerobic lumen.

### For soil (airSpring)

Paper 06 (no-till Anderson) models QS in the soil pore network.
Soil water content determines which pores are waterlogged (anaerobic)
and which are drained (aerobic). This creates a dynamic, spatially
heterogeneous oxygen landscape. Paper 16 provides the mechanism
for how QS propagation changes across this landscape.

### For fermentation (bench experience + atlasHugged)

Fermentation — the oldest biotechnology, independently discovered
across every human civilization — is anaerobic microbial ecology.
The atlasHugged essay (`08_DISCOVERY_IS_LOCAL.md`) frames fermentation
as proof that biological systems operate independently of human
understanding. Paper 16 gives the quantitative model.

Ancient beers, wines, pickles, and starter cultures relied on
anaerobic QS for community stability. We now understand the
microbiology. The QS-disorder framework provides the physics.

---

## Experimental Plan (staged)

### Stage 1: Literature + Model (no wetlab)

- Map known QS systems in anaerobic vs aerobic organisms
- Identify FNR/ArcAB/Rex regulated QS genes from literature
- Build Anderson lattice models with oxygen-dependent W
- Predict localization length shift for model communities
- ~~Reproduce Wang et al. 2020 ML digester prediction (neuralSpring benchmark)~~ **DONE (S142)** — `digestion_prediction.rs`, ESN 512-neuron reservoir, R²=0.84 test, Py 9/9, Rust 11 lib tests, CPU 36/36, bC/gT 23/23 PASS. GPU↔CPU diff ≤7.1e-5.

### Stage 2: Public Data (no wetlab)

- Apply wetSpring 16S pipeline to public anaerobic digester datasets
  (NCBI BioProjects from ADREC and similar labs)
- Measure diversity-disorder mapping in anaerobic communities
- Compare W distributions: aerobic biofilms vs anaerobic digesters vs
  gut microbiome vs soil
- Reproduce Yang et al. 2016 community-performance linkage

### Stage 3: ADREC Collaboration (requires contact)

- Apply the model to ADREC's digester community time-series data
- Test whether W predicts digester stability/upset
- Design experiment: same inoculum, aerobic vs anaerobic, measure
  QS gene expression and community composition simultaneously
- Facultative anaerobe panel: E. coli, Klebsiella, Bacillus —
  measure autoinducer production under aerobic vs anaerobic conditions

---

## Cross-Spring Requirements

| Spring | Contribution | Status |
|--------|-------------|--------|
| wetSpring | Anderson-QS framework, 16S pipeline, diversity analytics, Bray-Curtis GPU | **Ready** |
| healthSpring | Anderson gut lattice model (Exp032), anaerobic gut as test case | **Ready** |
| airSpring | Soil aerobic/anaerobic zonation, Paper 06 pore network model | **Ready** |
| neuralSpring | ESN/LSTM for digester time series, regime classification. **Paper 027 (Wang/Liao 2020) COMPLETE**: `digestion_prediction.rs`, ESN methane yield predictor, bC/gT validated | **Ready + Benchmark Complete** |
| groundSpring | Spectral theory for oxygen-dependent W, uncertainty budgets | **Ready** |
| barraCuda | GPU diversity, GPU Anderson, GPU Bray-Curtis | **Ready** |

The infrastructure is entirely validated. What's missing is the science:
anaerobic-specific QS data and the experiments to test the predictions.

---

## V108 Computational Foundation (March 10, 2026)

wetSpring V108 completes the Stage 1 computational foundation for Paper 16.
Five papers from Wei Liao's group at MSU BAE / ADREC are now fully reproduced:

| Paper | Experiment | Checks | Key Model |
|-------|:----------:|:------:|-----------|
| Yang et al. 2016 (co-digestion) | Exp336 | 12 | Modified Gompertz, Shannon diversity |
| Chen et al. 2016 (culture conditions) | Exp337 | 14 | Anderson W shifts with conditions, evenness/methane correlation |
| Rojas-Sossa et al. 2017 (coffee residues) | Exp338 | 10 | Substrate inhibition → higher W |
| Rojas-Sossa et al. 2019 (AFEX corn stover) | Exp339 | 11 | Pretreatment → lower disorder |
| Zhong et al. 2016 (fungal fermentation) | Exp340 | 10 | Monod kinetics, aerobic-anaerobic W shift |

Full 6-tier validation chain (Exp341-346, 136 additional checks):

| Tier | Experiment | Checks | Result |
|------|:----------:|:------:|--------|
| Paper math control v6 | Exp341 | 38 | All 63 papers, mathematical invariants GREEN |
| BarraCuda CPU v26 | Exp342 | 33 | Pure Rust math for Gompertz/Monod/Haldane/diversity |
| Python parity v5 | Exp343 | 13 | SciPy/NumPy reference values match |
| CPU vs GPU v10 | Exp344 | 14 | GPU portability for Track 6 math |
| Pure GPU streaming v12 | Exp345 | 12 | Unidirectional pipeline, zero CPU round-trips |
| metalForge v18 | Exp346 | 16 | Cross-substrate CPU=GPU=NPU |

**Key validated capabilities for Stage 2:**

- Modified Gompertz biogas model: `H(t) = P * exp(-exp((Rm*e/P)*(λ-t) + 1))`
- First-order kinetics: `B(t) = B_max * (1 - exp(-k*t))`
- Monod growth: `μ = μ_max * S / (Ks + S)`
- Haldane inhibition: `μ = μ_max * S / (Ks + S + S²/Ki)`, `S_opt = √(Ks * Ki)`
- Anderson W mapping: `W = W_max * (1 - evenness)`, `W_digester > W_soil` confirmed
- P(QS) comparison via `norm_cdf` operational for aerobic vs anaerobic

**neuralSpring S142 — Paper 027 (Wang/Liao 2020):**

| Component | Checks | Key Result |
|-----------|:------:|------------|
| Python control (`digestion_prediction.py`) | 9/9 PASS | R²=0.91 train, R²=0.85 test, RMSE=8.1 mL/gVS |
| Rust module (`digestion_prediction.rs`) | 11 lib tests | Process model + ESN predictor + JSON parity |
| CPU validator (`validate_digestion_prediction`) | 36/36 PASS | Analytical parity + physical expectations |
| bC/gT validator (`validate_barracuda_digestion`) | 23/23 PASS | GPU↔CPU diff ≤7.1e-5, physics preserved |

ESN architecture: 512-neuron reservoir, 2-step recurrence, additive process model
with T×OLR interaction. Same architecture as nW-05 (WDM classifier), different
domain (bioprocess engineering) — isomorphic thesis proof (Exp 005 extension).

**Next**: Stage 2 — apply 16S pipeline to real Liao group community datasets from NCBI SRA.

---

## Connection to Other Papers

| Paper | Connection |
|-------|-----------|
| 01 (Anderson-QS) | Foundation — Paper 16 extends the disorder model to oxygen-variable regimes |
| 03 (Bioag microbiome) | Soil aerobic/anaerobic zones are the field version of this question |
| 04 (Sentinels) | ESN regime classifier deployed at a digester = ADREC process monitor |
| 05 (Cross-species QS) | Anaerobic consortia are inherently cross-species |
| 06 (No-till Anderson) | Waterlogging = anaerobic pores = different W regime |
| 12 (Immuno-Anderson) | Gut inflammation changes mucosal oxygen → changes W at tissue interface |
| 13 (Health computing) | Gut microbiome modeling (healthSpring) is the human anaerobic digester |

---

## The Bigger Picture

The same QS systems could be aerobic or anaerobic — maybe gene
transcription changes, maybe microbes have both for different
conditions or niches. This is not a corner case; it is the
default for most natural microbial communities, which exist at
oxygen gradients rather than uniform conditions. The digesters
at ADREC are a controlled, instrumented version of what happens
in every waterlogged soil pore, every gut villus, every
stratified lake.

The bioreactor is the bridge between the lab bench and the
Anderson lattice.
