+++
title = "Anderson as No-Till Soil Health Mechanism"
description = "Soil Ecology x Physics — no-till as dimensional collapse of QS geometry. 9 papers reproduced, full three-tier. wetSpring + airSpring. 321+ checks."
date = 2026-03-17

[extra]
paper_number = 6
domain = "Microbiology and Ecology"
+++

# Sub-Thesis 06: Anderson Localization as the Mechanism Behind No-Till Soil Health

**Date:** March 1, 2026
**Status:** **Validated** — Track 4 complete: 9 papers reproduced (Exp170-178 CPU, Exp179 CPU parity, Exp180 GPU, Exp181 streaming, Exp182 metalForge), 321 validation checks, full three-tier (CPU + GPU + metalForge). Anderson-QS soil pore geometry, no-till meta-analysis, 31-year tillage factorial, biofilm aggregate, structure→function mapping, tillage microbiome — all validated against published data (Martínez-García 2023, Feng 2024, Mukherjee 2024, Islam 2014, Zuber 2016, Liang 2015, Tecon & Or 2017, Rabot 2018, Wang 2025). V59 extension: dynamic W(t) models (Exp186 — tillage perturbation W(t), antibiotic perturbation, seasonal cycling) validated via three-tier controls (Exp190 CPU, Exp191 GPU Anderson spectral, Exp192 metalForge CPU↔GPU parity). **airSpring extension (v0.6.1)**: GPU math portability 46/46 (Exp 047, all 13 modules), Anderson coupling 55+95 (Exp 045), NestGate NCBI provider validated (23/23), GPU van Genuchten θ(h)/K(h) (ops 9-10), GPU pedotransfer (op 13), GPU uncertainty (jackknife/bootstrap/diversity) — 25 Tier A GPU modules, ToadStool S79 sync. **V84:** Paper math controlled for all 9 Track 4 papers (Exp251), bootstrap CI + jackknife cross-validation on diversity stats (Exp252-253), Kriging spatial interpolation GPU-validated (Exp254-255) — ready for real LTER/EMP soil time series via NestGate→NUCLEUS pipeline. **V85:** NUCLEUS data pipeline validated end-to-end (Exp257 — three-tier routing for field data acquisition). EMP Atlas 30K samples confirms Anderson-QS across all soil-relevant biomes (Exp256). Genomic Vault organ model (Exp259) enables consent-gated encrypted storage for field soil DNA samples — ready for LTER time series with provenance chain
**Domain:** Soil microbial ecology, condensed matter physics, precision agriculture
**Novelty:** No prior work applies Anderson localization to explain no-till vs
conventional tillage outcomes; no prior work models tillage as dimensional collapse
of a QS-active system
**Cross-Spring:** wetSpring (Anderson QS + 16S) × airSpring v0.8.8 (soil moisture + ET₀ + GPU van Genuchten θ/K + pedotransfer + uncertainty stack, 880 lib + 280 integration + 61 forge tests, 87 experiments, 20 ops upstream, PrecisionRoutingAdvice, barraCuda 0.3.5 wgpu 28, BYOB niche, zero unsafe everywhere, 60 tolerances in 4 submodules, zero-panic 47/47) ×
groundSpring V113 (uncertainty, spectral theory, rare biosphere, jackknife — GemmF64 transpose (Tikhonov KᵀK/KᵀG), RetryPolicy + CircuitBreaker, 4-format capability parsing, exit_code constants, 102 barracuda delegations) × neuralSpring (LSTM time series)

---

## Abstract

We extend the Anderson localization framework from Sub-thesis 01 to explain
a 60-year empirical puzzle: why does no-till farming produce superior soil
health outcomes? We propose that conventional tillage constitutes a **dimensional
collapse** of the soil pore network — converting a 3D Anderson lattice (where
quorum sensing signals propagate) into a disrupted, effectively lower-dimensional
system (where Anderson localization suppresses all QS coordination). No-till
preserves the 3D geometry that the Anderson model requires for microbial
communication. Cover crops modulate disorder (W) within the QS-active regime.
Seed-coat inoculants (Pivot Bio) exploit the Anderson-optimal geometry at the
root surface.

We frame David Brandt's 50-year Carroll, Ohio operation and Ohio State
University's 60-year Triplett-Van Doren experiment as the **experimental data**
(perturbed systems with known tillage history), and Sub-thesis 01's 28 natural
biomes as the **control baseline** (unperturbed 3D systems). This design enables
the first physics-based null hypothesis for why no-till works.

The time-series dimension — seasonal moisture oscillation, community diversity
drift, and multi-decade geometry restoration — extends the static Anderson
framework from Sub-thesis 01 into a dynamic model where the level spacing
ratio r(t) tracks QS regime transitions across temporal scales from days
(rainfall events) to decades (no-till adoption).

---

## 1. Introduction

### 1.1 The No-Till Puzzle

No-till agriculture — farming without mechanical inversion of the soil — has
been practiced since the 1960s. Empirical results from long-term trials
consistently show improvements in soil organic matter, aggregate stability,
microbial biomass, mycorrhizal abundance, and water infiltration (Islam et al.
2014; Triplett & Dick 2008). David Brandt demonstrated these outcomes over 50
years on 1,150 acres in Fairfield County, Ohio, earning recognition as the
"Godfather of Soil Health."

Yet the mechanism remains poorly articulated. The standard explanation —
"no-till preserves soil structure" — is descriptive, not predictive. It does
not answer: *why* does preserved structure lead to coordinated microbial
function? What specific physical property of undisturbed soil enables the
microbial community to produce ecosystem services (N-fixation, carbon cycling,
pathogen suppression) that tilled soil cannot?

### 1.2 The Anderson Answer

We propose: **no-till works because it preserves the three-dimensional pore
geometry required for Anderson-extended quorum sensing signal propagation.**

Anderson localization (Anderson 1958) predicts that in d <= 2, all wave states
localize for any disorder W > 0. In d >= 3, a metal-insulator transition
exists at critical disorder W_c ~ 16.5. Sub-thesis 01 showed that all 28
natural biomes sustain QS in 3D (extended states) but are QS-suppressed in
2D (localized states).

Tillage is the agricultural equivalent of a dimensional collapse:

| Condition | Soil geometry | Anderson dimension | QS prediction |
|-----------|--------------|-------------------|---------------|
| Native prairie | Intact 3D pore network | d = 3 | QS-active |
| No-till (established) | Preserved 3D aggregates | d = 3 | QS-active |
| No-till (transitioning) | Rebuilding 2D→3D | d ~ 2.5 (percolation) | QS marginal |
| Conventional till (fresh) | Destroyed aggregates | d ≈ 2 (surface) | QS-suppressed |
| Compacted (traffic pan) | Collapsed pore space | d < 2 | QS-suppressed |

### 1.3 The Experimental Design

**Experiment:** OSU Triplett-Van Doren No-Tillage and Crop Rotation Experiment
(est. 1962, Wooster silt loam + Hoytville clay) and the David Brandt farm
(no-till since 1971, cover crops since 1978). These provide 60+ years of
tilled vs no-till side-by-side data with documented soil health metrics,
microbial biomass, and aggregate stability.

**Control:** Sub-thesis 01's 28 natural biome predictions — unperturbed 3D
Anderson systems where QS propagation follows physics predictions. If no-till
soil converges toward natural ecosystem Anderson parameters (level spacing
ratio, disorder, effective dimension), the mechanism is validated.

---

## 2. The Model

### 2.1 Tillage as Dimensional Collapse

Soil aggregate structure determines the effective connectivity of the pore
network. We model this connectivity as the lattice dimension d_eff of the
Anderson Hamiltonian:

    H = Σ_i ε_i |i><i| + t Σ_<i,j> |i><j|

where ε_i are on-site energies (species identity at pore position i),
t is the hopping parameter (diffusion rate of QS autoinducers between
connected pores), and the sum <i,j> runs over connected pore neighbors.

The key insight: **tillage reduces the coordination number** (average number
of connected neighbors per pore). In a 3D cubic lattice, coordination number
z = 6. After tillage destroys aggregate structure:

| Tillage intensity | Aggregate stability | Coordination (z) | Effective d |
|-------------------|--------------------|--------------------|-------------|
| None (native) | High (>80%) | ~6 | 3.0 |
| No-till (mature) | High (70-85%) | ~5-6 | 2.8-3.0 |
| Minimum till | Moderate (40-60%) | ~4 | 2.0-2.5 |
| Conventional till | Low (20-40%) | ~3 | 1.5-2.0 |
| Intensive till | Very low (<20%) | ~2 | 1.0-1.5 |

Below z = 4 (d_eff ≈ 2), Anderson localization predicts all QS signals
localize regardless of community diversity. The microbial community may be
equally diverse in tilled and no-till soil, but only the no-till community
can coordinate.

### 2.2 Cover Crop Diversity as Disorder Tuning

Cover crop cocktails (Brandt's specialty) increase rhizosphere microbial
diversity (higher Pielou evenness J → higher Anderson disorder W). From
Sub-thesis 01:

    W = 0.5 + 14.5 * J

Cover crop mixes push J upward (more even communities). The critical
question: does increased diversity break QS coordination?

Anderson's theorem answers this: **in 3D, not until W exceeds W_c ~ 16.5.**
Since even the most diverse natural communities reach W ≈ 15 (J ≈ 1.0),
and real agricultural soils have J ~ 0.5-0.8 (W ~ 7.75-12), cover crop
diversity stays comfortably in the QS-active regime — *as long as the
geometry remains 3D*.

This is the prediction: **cover crops are beneficial only in no-till systems
where 3D geometry is preserved.** In tilled soil, the same diversity increase
provides no QS benefit because d_eff < 2 already localizes all signals.

### 2.3 Soil Moisture as Dynamic Geometry

Soil pore connectivity depends on water content. Pores filled with water
transmit diffusible autoinducers; air-filled pores do not. This creates a
time-varying effective dimension:

    d_eff(t) = f(θ(t), aggregate_stability, pore_size_distribution)

where θ(t) is volumetric water content. airSpring (v0.5.1) already computes θ(t)
via the FAO-56 water balance (validated, 1109/1109 Python + 651 Rust tests +
1393 atlas checks, 54 binaries). The `eco::anderson` module (Exp 045) now
implements the full coupling chain: θ → S_e → pore_connectivity → z → d_eff → QS
regime (55+95 checks, 1e-10 cross-validation). The van Genuchten θ(h) pipeline was
extracted into a focused `eco::van_genuchten` module (150 lines) and wired to
upstream `barracuda::optimize::brent` for pressure head inversion. The coupling is:

    θ(t) → pore_connectivity(t) → z(t) → d_eff(t) → r(t) → QS_regime(t)

This transforms the static Anderson model into a **dynamic system** where
the QS regime oscillates with moisture:

| Season | θ typical | Pore connectivity | d_eff | QS regime |
|--------|-----------|-------------------|-------|-----------|
| Spring (thaw) | High (0.35-0.45) | Full reconnection | 3.0 | QS-active |
| Summer (drought) | Low (0.15-0.20) | Partial disconnect | 2.0-2.5 | QS marginal |
| Fall (rain) | Moderate (0.25-0.35) | Reconnecting | 2.5-3.0 | QS recovering |
| Winter (frozen) | N/A (ice) | Frozen channels | ~1.5 | QS-suppressed |

In no-till soil, stable aggregates maintain pore connectivity even at lower
moisture — the system resists dimensional collapse during drought. In tilled
soil, destroyed aggregates lose connectivity faster as moisture drops.

---

## 3. The Brandt Natural Experiment

### 3.1 David Brandt's Farm (1971-2023)

David Brandt (1946-2023) began no-till farming in 1971 on 1,150 acres in
Carroll, Ohio, and adopted cover crop cocktails in 1978. Over 50 years, his
farm demonstrated (Islam et al. 2014, ISWCR 2:97):

- Total microbial biomass: significantly increased vs tilled controls
- Active carbon: significantly increased (composite soil health measure)
- Soil aggregate stability: substantially improved with cover crop cocktails
- Carbon sequestration: consistent accumulation, plateauing at ~20 years
- Mycorrhizal fungi: greater abundance (long-term no-till studies)

### 3.2 Anderson Interpretation

| Brandt observation | Anderson mechanism |
|--------------------|--------------------|
| Increased microbial biomass | Preserved 3D geometry → QS-active → coordinated growth |
| Increased active carbon | QS-coordinated carbon cycling enzymes functional |
| Improved aggregate stability | Biofilm EPS production (QS-regulated) stabilizes aggregates |
| 20-year carbon plateau | Anderson regime saturates — r(t) reaches steady state |
| Greater mycorrhizal abundance | AM hyphal networks = 3D geometry → MHB QS coordination |
| Cover crop cocktail synergy | Diversity increases W but stays below W_c in preserved 3D |

The 20-year carbon sequestration plateau is particularly revealing. The
Anderson model predicts this is the time required for:

1. Physical aggregate rebuilding (2D→3D geometry restoration, years 0-5)
2. Microbial community reorganization (QS networks establish, years 5-10)
3. QS-mediated carbon cycling reaching equilibrium (years 10-20)
4. Steady state: d_eff stabilized, W stabilized, r stabilized (year 20+)

### 3.3 OSU Triplett-Van Doren Experiment (1962-present)

The longest-running no-till experiment in the US provides the controlled
version of Brandt's farm:

- **Two soil types**: Wooster (well-drained silt loam) and Hoytville
  (poorly drained clay loam) — different pore architectures
- **Side-by-side**: Tilled vs no-till plots, same climate, same management
  except tillage
- **60+ years**: Long enough to observe the full Anderson transition

Anderson prediction: the Wooster (well-drained) site should show stronger
QS-active signals because well-drained silt loam maintains air-filled pores
that create defined 3D channels for autoinducer diffusion. Hoytville
(poorly drained clay) may have water-saturated pores that favor liquid-phase
diffusion but restrict aerobic QS circuits.

---

## 4. The Pivot Bio Connection

### 4.1 Geometry-Optimized Inoculants

Pivot Bio's PROVEN (corn) and RETURN (soybean) products use engineered
seed-coat N-fixation inoculants. The seed coat places the inoculant directly
on the emerging root surface — the Anderson-optimal geometry for QS
establishment (Sub-thesis 03, Section 5).

### 4.2 The Anderson Stack

The robust agricultural system is an Anderson stack where each layer
reinforces the others:

```
Layer 1: No-till           → preserves 3D geometry (d_eff ≈ 3)
Layer 2: Cover crops       → tunes diversity (W ~ 8-12, below W_c)
Layer 3: Seed inoculant    → places QS bacteria at root biofilm (3D)
Layer 4: Soil monitoring   → tracks moisture → predicts d_eff(t)
Layer 5: LSTM prediction   → forecasts QS regime transitions
```

Each layer addresses a different Anderson parameter:
- Layer 1: dimension (d)
- Layer 2: disorder (W)
- Layer 3: initial condition (placement in extended regime)
- Layer 4: time-varying geometry (d_eff(t))
- Layer 5: prediction and intervention timing

### 4.3 Why Monoculture Fails

Conventional monoculture farming attacks every layer simultaneously:

| Practice | Anderson impact |
|----------|-----------------|
| Tillage | Destroys 3D geometry → d_eff < 2 → universal localization |
| Monoculture | Reduces diversity → low W, but irrelevant if d < 2 |
| Broadcast fertilizer | Bypasses QS-mediated N regulation entirely |
| No cover crop | No diversity buffer; soil exposed to erosion (further geometry loss) |
| No monitoring | No feedback on QS regime state |

The Anderson framework predicts that monoculture farming creates a system
where microbial QS coordination is physically impossible — not because the
microbes aren't there, but because the geometry doesn't permit signal
propagation.

---

## 5. Time Series and Seasonality

### 5.1 Seasonal Anderson Oscillation

The level spacing ratio r(t) is not a constant. It oscillates with:

**Moisture cycle** (days to weeks):
- Rainfall → θ increases → pore reconnection → d_eff rises → r approaches GOE
- Drought → θ decreases → pore disconnection → d_eff drops → r approaches Poisson
- airSpring's FAO-56 water balance computes θ(t) for arbitrary climate data

**Community cycle** (weeks to months):
- Growing season: root exudates recruit rhizosphere community → J shifts → W changes
- Cover crop termination: carbon pulse → diversity spike → W jumps
- Winter dormancy: reduced metabolic activity → effective J drops → W decreases

**Multi-year transition** (years to decades):
- Year 0 (begin no-till): collapsed geometry, d_eff ≈ 2
- Years 1-5: aggregate rebuilding, d_eff transitions through percolation threshold
- Years 5-15: 3D network matures, mycorrhizal networks establish
- Years 15-20: Anderson regime saturates, r(t) oscillations stabilize
- Year 20+: Steady state — system behaves like natural ecosystem baseline

### 5.2 The LSTM Time Series Model

neuralSpring has validated LSTM on ERA5 weather data (NSE=0.849, RMSE=3.46°C
on 4 years Michigan data, Study 004). The same architecture can predict
QS regime from soil parameters:

**Inputs**: [θ(t), T_soil(t), J(t), tillage_history, aggregate_stability,
cover_crop_stage, precipitation(t)]

**Output**: r(t) — predicted level spacing ratio → QS regime classification

**Training data**: OSU Triplett-Van Doren + Brandt farm time series, with
Anderson regime computed from measured soil properties.

**Validation**: Cross-validate against Sub-thesis 01's static predictions
for natural biomes (the control baseline).

### 5.3 Predictions

1. **r(t) in no-till soil oscillates above GOE/Poisson midpoint (0.459)**
   in all seasons except deep winter freeze. In tilled soil, r(t) stays
   below the midpoint year-round.

2. **Cover crop termination produces a transient W spike** that briefly
   approaches W_c in 3D but does not cross it. The QS regime dips but
   recovers within days.

3. **The 20-year no-till transition** maps to a percolation transition
   in d_eff: below the percolation threshold, the 3D pore network is
   disconnected and Anderson localization dominates; above it, the
   system is QS-active. Aggregate stability is the macroscopic proxy
   for d_eff.

4. **Well-drained soils** (Wooster-type) reach QS-active steady state
   faster than poorly-drained soils (Hoytville-type) because drainage
   prevents waterlogging that can paradoxically disconnect aerobic
   QS circuits.

5. **Drought stress in no-till soil** has a quantitative Anderson prediction:
   QS fails when θ drops below the pore percolation threshold. No-till
   soil's better aggregate stability means this threshold is lower
   (more drought-resilient QS) than in tilled soil.

---

## 6. Experimental Design

### 6.1 Computational Validation (wetSpring)

Extend the Anderson lattice experiments to model soil pore geometry:

- **Exp A**: 3D Anderson lattice with variable coordination number z
  (simulates tillage intensity as geometry parameter)
- **Exp B**: Time-varying disorder W(t) in 3D lattice (simulates seasonal
  diversity oscillation)
- **Exp C**: Coupled moisture-geometry model: θ(t) from airSpring water
  balance → d_eff(t) → Anderson eigenvalue computation → r(t)
- **Exp D**: Process OSU/Brandt 16S data through sovereign Rust pipeline,
  compute J and W for tilled vs no-till plots

### 6.2 Data Sources

| Source | Data | Access | Format |
|--------|------|--------|--------|
| Islam et al. (2014) | Brandt farm soil health metrics | Open (ISWCR) | Published tables |
| OSU Triplett-Van Doren | 60-year tilled vs no-till | Open (OARDC) | Published + NCBI SRA (check) |
| Nature Comms 2023 (Martínez-García) | QS in porous media | Open (journal) | Published models + data |
| Nature Comms 2024 (分 et al.) | Microbial communities in soil pores | Open (journal) | Published pore-scale data |
| NCBI SRA | No-till 16S studies (~105K entries) | Open (NCBI) | FASTQ — **NestGate provider validated 23/23** |
| Open-Meteo ERA5 | Ohio weather (1962-present) | Open (CC BY 4.0) | API — **115 CSVs, 80yr MI data ready** |
| USDA Web Soil Survey | Wooster + Hoytville soil properties | Open (USDA) | API |

### 6.3 Reproduction Targets

| Paper | Spring | What to Reproduce | Why |
|-------|--------|-------------------|-----|
| Martínez-García et al. (2023) Nat Comms | wetSpring | QS + spatial structure in porous media | Direct validation of QS-geometry coupling |
| 分 et al. (2024) Nat Comms | wetSpring | Microbial diversity in different pore sizes | Pore-scale Anderson geometry data |
| Islam et al. (2014) ISWCR | airSpring | Brandt farm soil health time series | No-till long-term data |
| Allen et al. (1998) FAO-56 Ch 7 | airSpring | Dual Kc for cover crop water balance | Already in airSpring queue (#8) |

### 6.4 Cross-Spring Integration

| Component | Spring | Connection |
|-----------|--------|------------|
| Anderson eigenvalue computation | wetSpring | Extends Exp107-143 |
| 16S diversity pipeline | wetSpring | Sovereign Rust pipeline |
| Soil moisture θ(t) | airSpring v0.8.8 | FAO-56 water balance (1284 Python + 880 Rust lib + 280 integration + 61 forge tests, 91 binaries). Saxton-Rawls pedotransfer (Exp 023 + GPU op=13) + Anderson coupling (Exp 045: θ→S_e→d_eff→QS regime, 55+95 checks) + GPU math portability (Exp 047: 46/46 all 13 modules) + GPU uncertainty (jackknife/bootstrap/diversity) + all 20 ops upstream (`BatchedElementwiseF64`), PrecisionRoutingAdvice wired |
| ET₀ seasonal pattern | airSpring v0.8.8 | Validated (1284/1284 Python, 880 Rust lib + 280 integration + 61 forge tests, 14.3× Rust speedup (24/24 parity), 8 ET₀ methods including Blaney-Criddle upstream op=19) |
| van Genuchten θ(h)/K(h) GPU | airSpring v0.8.8 | `eco::van_genuchten` module + `gpu::van_genuchten` (ops 9-10), `barracuda::optimize::brent` (R-S66 wired). GPU path via `BatchedVanGenuchten` enables batch soil hydraulics at atlas scale |
| Sampling uncertainty | groundSpring V113 | Exp004 (genus saturation at 5,000 reads). Exp016 (rare biosphere — detecting rare soil taxa at low abundance). Exp019 (jackknife — subpercent error bars for soil microbiome metrics). Exp015 (uncertainty bridge — sensor noise → Anderson ξ → QS regime uncertainty, the pipeline that makes Anderson predictions testable from real soil sampling data). **Exp022** (ET₀→Anderson propagation: humidity-dominated CV 0.043 → ξ CV 0.040, confirming environment→localization uncertainty budget). **Exp023** (no-till vs tilled 16S sampling: H′=3.88 vs 1.57, saturation at 500 reads, quantifies diversity loss from tillage). **Exp024** (aggregate stability noise: d_eff regimes distinguishable, noise floor 0.12–0.14, proves soil structure uncertainty doesn't mask Anderson regime classification). wetSpring GPU rarefaction uses dedicated `BatchedMultinomialGpu` for diversity curves. **V113**: GemmF64 transpose (Tikhonov KᵀK/KᵀG), RetryPolicy + CircuitBreaker, 4-format capability parsing, exit_code constants. V112: OrExit<T>, parse_benchmark(), socket_env_var(), provenance trio. 102 barracuda delegations — 29/29 validation binaries, 140 metalForge checks |
| LSTM time series prediction | neuralSpring | Study 004 (ERA5 LSTM, NSE=0.849). S135: 966 lib tests, 232 binaries, 220/220 validate_all, 150+ tolerances, 46 upstream rewires. nW-03 LSTM reservoir (R²=0.98) validates pooled-readout sequence processing — same architecture predicts r(t) from soil parameters. nW-05 ESN classifier (96.5% accuracy) validates regime classification — same architecture classifies QS regimes (extended/marginal/localized) from (J, d_eff) inputs |

---

## 7. Connection to Constrained Evolution

No-till soil is a constrained environment: water-limited, seasonally
oscillating, biologically diverse. The microbiome evolves under these
constraints toward whatever community structure can persist — and the
Anderson model predicts which evolved communities can coordinate via QS.

Tillage is the removal of a constraint (geometry). Paradoxically, removing
the geometric constraint does not free the microbiome — it traps it.
Without 3D pore architecture, QS signals localize, microbial coordination
fails, and ecosystem services collapse. Farmers compensate with synthetic
inputs (fertilizer, pesticides) that bypass the need for microbial
coordination entirely.

No-till is the restoration of the geometric constraint. The Anderson model
explains why the restoration takes ~20 years (geometry rebuilding),
why cover crops help (diversity tuning within the QS-active regime), and
why seed-coat inoculants like Pivot Bio's work (root-surface 3D biofilm
geometry).

This is constrained evolution as engineering: understanding the constraint
landscape to design interventions that work with the physics rather than
against it. David Brandt did this empirically for 50 years. The Anderson
framework provides the physics.

---

## 8. The Key Insight

**No-till farming works because it preserves the 3D geometry that Anderson
localization theory requires for quorum sensing to propagate.**

Tillage is, in physics terms, a dimensional collapse — it converts a 3D
pore network into a disrupted, effectively 2D surface system where all
QS signals localize and microbial coordination fails. Every farmer who
has watched soil "come alive" after years of no-till is watching the
Anderson metal-insulator transition in reverse: geometry restoration →
extended states → QS coordination → ecosystem function.

David Brandt ran a 50-year Anderson experiment. The OSU Triplett-Van Doren
experiment has run for 60 years. The data is sitting there. The physics
framework to interpret it is Sub-thesis 01. This paper connects them.

---

## References

Anderson, P.W. (1958). Absence of Diffusion in Certain Random Lattices.
Physical Review 109(5):1492-1505.

Islam, R., Brandt, D., Dick, W.A. et al. (2014). No-till and conservation
agriculture in the United States: An example from the David Brandt farm,
Carroll, Ohio. ISWCR 2:97-107.

Martínez-García, R. et al. (2023). Spatial structure, chemotaxis and quorum
sensing shape bacterial biomass accumulation in complex porous media.
Nature Communications 14:8332.

Triplett, G.B. & Dick, W.A. (2008). No-tillage crop production: A revolution
in agriculture! Agronomy Journal 100(S3):S153-S165.

Waters, C.M. & Bassler, B.L. (2005). Quorum Sensing: Cell-to-Cell
Communication in Bacteria. Annual Review of Cell and Developmental Biology
21:319-346.

Feng, K. et al. (2024). Composition and metabolism of microbial communities
in soil pores. Nature Communications 15:3578.

OSU Soil Fertility Lab. Long-term Tillage and Crop Rotation Experiment.
soilfertility.osu.edu/research/long-term-tillage-plots
