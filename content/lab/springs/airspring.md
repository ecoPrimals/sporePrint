+++
title = "airSpring — Precision Agriculture & Irrigation"
description = "Evapotranspiration, soil moisture, IoT irrigation — 57 papers reproduced, R²=0.97 on open data, 13,000× speedup at atlas scale"
date = 2026-05-06
weight = 3

[taxonomies]
primals = ["barracuda", "toadstool", "biomeos"]
springs = ["airspring", "hotspring", "wetspring", "neuralspring", "groundspring"]
+++

## Domain

Evapotranspiration (8 methods), soil moisture, IoT irrigation, Richards PDE, coupled hydrology, yield response.

**Repository**: [syntheticChemistry/airSpring](https://github.com/syntheticChemistry/airSpring)

## The Science Story

airSpring proves barraCuda can replace the Python/Excel toolchain for precision agriculture at every stage — from paper reproduction through GPU-accelerated sovereign computation on consumer hardware. The complete pipeline (weather data → evapotranspiration → crop coefficients → water balance → yield response) runs in Rust, on GPU, with zero institutional access required.

## Headline Results

- **57 papers reproduced** with full provenance
- **FAO-56 ET₀** matches Python to 1e-5 across 75 cross-validated values
- **Real data** from 100 Michigan stations (15,300 station-days) achieves **R²=0.97** using only free, open APIs (Open-Meteo, NOAA)
- **19.8× geometric mean** Rust speedup over Python (24 algorithms), **13,000×** at atlas scale
- NUCLEUS primal with 30 science capabilities

## Validation Phases

| Phase | Key Result |
|-------|------------|
| 0 (Python) | 57 papers matched exactly. 1,237/1,237 checks |
| 0+ (Real data) | 100 Michigan stations, R²=0.97 — open data validated |
| 1-2 (Rust + cross-validation) | 75/75 Python↔Rust matches within 1e-5. 19.8× speedup |
| 3 (GPU) | Pure GPU end-to-end, 0.04% seasonal parity |
| 3.5+ (NPU/NUCLEUS) | AKD1000 + 27 workloads + 30 capabilities. Full cross-substrate |

## Researcher Reproduced

| Researcher | Department | Domain |
|------------|-----------|--------|
| Younsuk Dong | BAE, MSU | Precision agriculture, irrigation |

## What the Constraint Revealed

Open data can replace institutional access — no weather station networks, no licensed datasets. The GPU pipeline (ET₀ → Kc → WB → Yield in one dispatch chain) stays on-device with zero CPU round-trips. Cross-spring shader provenance traces every GPU operation to its mathematical origin: precision from hotSpring, biology from wetSpring, ML from neuralSpring, uncertainty from groundSpring.

## Cross-Spring Connections

- **← groundSpring**: "humidity dominates ET₀ uncertainty at 66%"
- **← neuralSpring**: "MLP surrogate replaces FAO-56 at R²=0.999"; "transfer learning bridges Michigan→NM with 200 samples"
- **← hotSpring**: f64 GPU dispatch batching pattern
- **← wetSpring**: kriging spatial interpolation; dynamic Anderson W(t) models soil moisture coupling
- **→ Penny Irrigation**: real-world target application

## baseCamp Papers

Papers 03, 06, 08, 12 — see [baseCamp Science](/science/) for full list.
