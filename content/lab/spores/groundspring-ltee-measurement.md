+++
title = "groundSpring LTEE Measurement"
description = "Measurement noise and uncertainty quantification — LTEE reproductions with three-tier parity"
date = 2026-07-18
template = "spore_gallery.html"

[taxonomies]
springs = ["groundspring"]

[extra]
domain = "Measurement Noise & Uncertainty"
spore_name = "groundSpring-LTEE-Measurement"
spore_version = "1.0.0"
spore_origin = "ecoPrimals/springs/groundSpring"
spore_spring = "groundSpring"
spore_status = "PENDING"
modules_pass = 0
modules_total = 0
methods = ["bias-variance decomposition", "Monte Carlo propagation", "bootstrap resampling", "jackknife estimation", "Anderson localization", "Nelder-Mead optimization"]
tools = ["Rust (Tier 2 validator)", "Python (Tier 1 baseline)", "barraCuda GPU"]
+++

## Domain Profile

Measurement noise and uncertainty quantification across 12 scientific domains.
Covers sensor noise characterization, inverse problems, error propagation,
calibration datasets, and statistical validation. The LTEE subset reproduces
five Barrick Lab papers (B1-B4, B6) with three-tier parity: Python baseline,
Rust validator, and GPU delegation.

**Status:** pseudoSpore v1.0.0 emitted (279 KB, 252 files). Module validation
pending — 5 LTEE modules + 29 benchmark baselines included. All BLAKE3
checksums anchored via `LITHOSPORE_INGESTION_MANIFEST.toml`.

## LTEE Module Status

| # | Module | Paper | Checks (Py/Rust) | Status |
|---|--------|-------|-------------------|--------|
| 1 | ltee-fitness | Wiser et al. 2013 (B2) | 9/9 + 10/10 | PENDING |
| 2 | ltee-mutation | Barrick et al. 2009 (B1) | 8/8 + 8/8 | PENDING |
| 3 | ltee-clonal | Good et al. 2017 (B3) | 8/8 + 8/8 | PENDING |
| 4 | ltee-citrate | Blount et al. 2008/2012 (B4) | 8/8 + 8/8 | PENDING |
| 5 | ltee-biobrick | Nat Comms 2024 (B6) | 7/7 + 34/34 | PENDING |

**0 of 5 modules validated.** Awaiting groundSpring `cargo test` fix
(`bingoCube/nautilus` dependency).

## Benchmark Baselines (29 domains)

Sensor noise, observation gap, error propagation, seismic inversion,
Anderson localization, quasispecies threshold, drift/selection,
rare biosphere, resampling convergence, vendor parity, and more.
Each baseline has `benchmark_*.json` golden values with documented tolerances.

## Provenance

| Property | Value |
|----------|-------|
| Origin | `ecoPrimals/springs/groundSpring` |
| Version | 1.0.0 |
| Spring | groundSpring |
| Emission method | `litho emit-pseudospore` |
| Integrity | BLAKE3 checksums in `receipts/checksums.blake3` (241 entries) |
| Braid | FermentBraid provenance chain (who/what/when/how) |

## Download

**Archive:** `pseudoSpore_groundSpring-LTEE-Measurement_v1.0.0.tar.gz` (279 KB)
**Verify:** `litho ingest-pseudospore <path> --verify`
