+++
title = "Chapter 11: Results — groundSpring"
description = "Measurement noise and uncertainty across ten domains — 376 checks forming the tolerance foundation for all springs."
weight = 11
date = 2026-07-09
+++

{{ maturity(level="architectural") }}

## 11.1 Validation Summary

groundSpring asks: how much can we trust the numbers? It quantifies measurement noise, error propagation, and uncertainty across ten scientific domains — agricultural sensors, meteorological observations, microbiome sequencing, geophysics, biological signaling, spectral theory, eco-evolutionary dynamics, inverse problems, warm dense matter, and immunological physics. 376 checks pass across 33 experiments, with full Python baselines (Phase 0), Rust validation (Phase 1), and GPU acceleration (Phase 2) via 81 barraCuda delegations (47 CPU + 34 GPU). 786+ workspace tests, 375 Python tests, 28/28 mathematical parity proven. groundSpring serves as the uncertainty foundation that informs tolerances for every other spring.

### Table 11.1 — Experiment Summary

| Experiment | Domain | Checks | Status |
|------------|--------|-------:|--------|
| 001: Sensor noise decomposition | Agricultural sensing | 32 | **32/32** |
| 002: Weather model vs observation | Meteorology | 5 | **5/5** |
| 003: Error propagation FAO-56 | ET₀ uncertainty | 8 | **8/8** |
| 004: Sequencing depth & taxonomic noise | Microbiome | 16 | **16/16** |
| 005: Seismic wave propagation | Geophysics | 10 | **10/10** |
| 006–011: Bio signaling + spectral | Signal specificity, RAWR, Anderson, Almost-Mathieu, bistable, multisignal | 68 | **68/68** |
| 012–018: Transport + eco-evo | Spin chain, resampling, drift, uncertainty bridge, rare biosphere, quasispecies, band edge | 57 | **57/57** |
| 019–021: Inverse problems | Jackknife, freeze-out, spectral recon (Bazavov) | 25 | **25/25** |
| 022–028: Cross-spring + hardware | ET₀→Anderson, no-till, aggregate, precision drift, size convergence, vendor parity, NPU | 60 | **60/60** |
| 029–032: NUCLEUS integration | Real GHCND, real NCBI 16S, NUCLEUS stack, IRIS seismic | 55 | **55/55** |
| 033: Tissue Anderson | Immunological Anderson (Paper 12, Gonzales) | 29 | **29/29** |
| **Total** | **10 domains** | **376** | **All pass** |

---

## 11.2 Sensor Noise Characterization (Exp001: 32/32)

### Table 11.2 — Sensor Noise Decomposition

| Sensor | Soil | Bias (MBE) | Random σ | Bias Fraction | Noise Floor (m³/m³) |
|--------|------|:----------:|:--------:|:-------------:|:-------------------:|
| CS616 | Sand | -0.010 | 0.014 | 34.6% | 0.006 |
| CS616 | Loamy sand | -0.030 | 0.025 | 59.2% | 0.021 |
| CS616 | Sandy clay loam | -0.020 | 0.034 | 26.3% | 0.012 |
| EC5 | Sand | +0.030 | 0.023 | 62.3% | 0.004 |
| EC5 | Loamy sand | -0.030 | 0.018 | 73.5% | 0.006 |
| EC5 | Sandy clay loam | -0.050 | 0.027 | 77.0% | 0.020 |

Key finding: EC5 sensors are **bias-dominated** (62–77% of total error), while CS616 sensors show mixed noise profiles (26–59% bias). Site-specific calibration removes 50–80% of total error — validating Dong et al.'s emphasis on soil-specific calibration coefficients.

---

## 11.3 Weather Model vs Observation (Exp002: 5/5)

### Table 11.3 — ERA5 Representation Error

| Metric | Finding |
|--------|---------|
| ERA5 vs station temperature | Representation error quantified |
| ERA5 vs station humidity | Representation error quantified |
| ERA5 vs station radiation | Representation error quantified |
| Measurement vs representation error ratio | Characterized across 3 variables |
| Methodology | Validated against literature expectations |

This experiment establishes that reanalysis products (ERA5) introduce representation error distinct from measurement error — essential for interpreting airSpring's real-data validation (R² = 0.967 is partly limited by ERA5 representation error, not pipeline error).

---

## 11.4 Error Propagation in FAO-56 (Exp003: 8/8)

### Table 11.4 — Variance Decomposition of ET₀

| Input Variable | Variance Contribution |
|----------------|:--------------------:|
| Relative humidity | **65.6%** |
| Solar radiation | 20.1% |
| Temperature | 10.0% |
| Wind speed | 4.3% |

| Metric | Value |
|--------|-------|
| ET₀ estimate | 3.879 ± 0.142 mm/day |
| Coefficient of variation | 3.7% |
| 90% confidence interval | [3.647, 4.118] mm/day |
| Monte Carlo vs analytical ratio | 1.009 |

Key finding: **humidity dominates ET₀ uncertainty** at 65.6% — a practical result for sensor deployment. If a grower can only afford one high-precision sensor, it should measure humidity. The Monte Carlo vs analytical ratio (1.009) validates that the variance decomposition is self-consistent.

---

## 11.5 Sequencing Depth & Taxonomic Noise (Exp004: 16/16)

### Table 11.5 — Saturation Analysis

| Metric | Threshold | Value |
|--------|-----------|-------|
| All phyla detected | Minimum reads | **100 reads** |
| Shannon 5% convergence | Minimum reads | **500 reads** |
| Genus saturation | Minimum reads | **5,000 reads** |
| Noise floor at 100,000 reads | Shannon | ±0.004 |
| Noise floor at 100,000 reads | Genus count | ±0.4 genera |

Key finding: genus-level taxonomic resolution requires ~5,000 reads — below this threshold, stochastic sampling noise dominates biological signal. This directly informs wetSpring's 16S pipeline: benchmarks that claim sub-genus resolution from < 5,000 reads per sample are within the noise floor.

---

## 11.6 Seismic Wave Propagation (Exp005: 10/10)

### Table 11.6 — Inversion Under Noise

| Scenario | Horizontal Error | Depth Error | Status |
|----------|:----------------:|:-----------:|--------|
| Clean inversion (no noise) | 0.00 km | 0.00 km | PASS |
| Noisy (±0.5 s timing error) | 0.9 km | 7.7 km | PASS |
| MC uncertainty envelope | ±2.1 km (90th: 3.9 km) | ±8.5 km | PASS |
| 3 stations | 28 km | — | PASS |
| 5 stations | < 1 km | — | PASS |
| 7 stations | < 1 km | — | PASS |

Key finding: depth resolution degrades 4× faster than horizontal resolution under noise — a known but quantified limitation of surface-station seismometry. The transition from 3 to 5 stations produces a dramatic accuracy improvement (28 km → < 1 km), demonstrating a phase transition in inverse problem conditioning.

---

## 11.7 Cross-Spring Noise Handoff

groundSpring provides uncertainty bounds consumed by other springs:

| Source Metric | From | Used By | Value |
|---------------|------|---------|-------|
| Soil sensor noise floor | Exp001 | airSpring sensor calibration | 0.004–0.021 m³/m³ |
| ET₀ uncertainty | Exp003 | airSpring water balance | ±0.142 mm/day |
| Sequencing saturation | Exp004 | wetSpring 16S diversity | 5,000 reads min |
| ERA5 representation error | Exp002 | airSpring real-data pipeline | R² ceiling |
| Monte Carlo vs analytical | Exp003 | All springs (methodology) | Ratio = 1.009 |

---

## 11.8 Connection to Constrained Evolution Thesis

groundSpring is the immune system of the spring framework. It does not produce headline results; it ensures that the results produced by other springs are meaningful. Without quantified uncertainty, a check that "passes" might pass because the tolerance is too loose.

The humidity dominance result (65.6% of ET₀ variance) demonstrates that measurement noise is not uniform — it has structure, and that structure should inform experimental design. This parallels the constrained evolution thesis: the constraint (noise) shapes what is observable, and understanding the constraint is prerequisite to understanding the fitness landscape.

groundSpring's evolution from Phase 0 (Python-only, 5 experiments) to Phase 2 (Rust + GPU, 33 experiments, 81 barraCuda delegations) demonstrates that GPU acceleration can benefit even uncertainty quantification — Monte Carlo simulations over parameter spaces, stochastic ODE integration, and spectral analysis all show 2–47× speedups on GPU. The evolution path validated the constrained evolution thesis: the constraint (noise) shapes what is observable, and the compute substrate evolves to match the problem's demands, not a fixed template. The three-tier validation architecture (CPU → barracuda-CPU → barracuda-GPU) proves that mathematical parity is maintained at each tier while performance improves.

---

**See also:**

- [Science: groundSpring papers](@/science/_index.md) — the reproduced papers
