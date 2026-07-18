+++
title = "airSpring Agricultural Meteorology"
description = "FAO-56 evapotranspiration and irrigation science — 8 ET₀ methods across Michigan's Crop Water Atlas"
date = 2026-07-18
template = "spore_gallery.html"

[taxonomies]
springs = ["airspring"]

[extra]
domain = "Agricultural Meteorology & Irrigation"
spore_name = "airSpring-Agricultural-Meteorology"
spore_version = "1.0.0"
spore_origin = "ecoPrimals/springs/airSpring"
spore_spring = "airSpring"
spore_status = "PENDING"
modules_pass = 0
modules_total = 0
methods = ["FAO-56 ET₀ (8 methods)", "Richards equation", "soil heat flux", "dual crop coefficient", "drought indexing"]
tools = ["Rust (Tier 2 validator)", "Python (Tier 1 baseline)", "barraCuda GPU"]
+++

## Domain Profile

Agricultural meteorology and irrigation science validation. Implements all
eight FAO-56 reference evapotranspiration methods plus Richards equation
soil water transport, applied to the Michigan Crop Water Atlas (100 stations,
80 years of daily NOAA GHCN-D observations).

**Status:** pseudoSpore v1.0.0 emitted (568 KB, 278 files). Module validation
pending — 62 benchmark baselines included. 1,446 Rust tests in spring workspace.

## Module Status

| # | Module | Description | Status |
|---|--------|-------------|--------|
| 1 | ET₀ Methods | 8 reference ET₀ calculations (Penman-Monteith, Hargreaves, etc.) | PENDING |
| 2 | Richards Equation | 1D soil water transport | PENDING |
| 3 | Dual Kc | Dual crop coefficient partitioning | PENDING |
| 4 | Climate Scenarios | Drought index and scenario analysis | PENDING |
| 5 | Atlas Suite | Full 100-station × 80-year atlas | PENDING |
| 6 | Cross-tier Parity | GPU/CPU determinism | PENDING |

**0 of 6 modules validated.** Atlas outputs generated at runtime, not stored.

## Provenance

| Property | Value |
|----------|-------|
| Origin | `ecoPrimals/springs/airSpring` |
| Version | 1.0.0 |
| Spring | airSpring |
| Emission method | `litho emit-pseudospore` |
| Integrity | BLAKE3 checksums in `receipts/checksums.blake3` (267 entries) |
| Braid | FermentBraid provenance chain (who/what/when/how) |

## Download

**Archive:** `pseudoSpore_airSpring-Agricultural-Meteorology_v1.0.0.tar.gz` (568 KB)
**Verify:** `litho ingest-pseudospore <path> --verify`
