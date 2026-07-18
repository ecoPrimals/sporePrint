+++
title = "neuralSpring ML Surrogates"
description = "Machine learning surrogates and isomorphic computation — 27 paper reproductions across physics and biology"
date = 2026-07-18
template = "spore_gallery.html"

[taxonomies]
springs = ["neuralspring"]

[extra]
domain = "ML Surrogates & Isomorphic Computation"
spore_name = "neuralSpring-ML-Surrogates"
spore_version = "1.0.0"
spore_origin = "ecoPrimals/springs/neuralSpring"
spore_spring = "neuralSpring"
spore_status = "PENDING"
modules_pass = 0
modules_total = 0
methods = ["neural ODE", "variational inference", "warm dense matter surrogate", "physics-informed ML", "isomorphic patterns"]
tools = ["Rust (baseCamp ML)", "Python (PyTorch baselines)", "WGSL (barraCuda inference)"]
+++

## Domain Profile

Machine learning surrogates, 27 paper reproductions, biophysical AI, warm
dense matter equation of state, and isomorphic computation patterns. The
baseCamp framework provides Rust-native ML primitives for scientific surrogate
models without Python dependencies in production.

**Status:** pseudoSpore v1.0.0 emitted (16 MB, 256 files). Module validation
pending — 46 control experiment baselines included. 4,900+ validation checks
across the spring.

## Module Status

| # | Module | Description | Status |
|---|--------|-------------|--------|
| 1-10 | Paper Reproductions | 27 published paper reproductions | PENDING |
| 11 | WDM Surrogate | Warm dense matter EOS emulator | PENDING |
| 12 | baseCamp Parity | CPU/GPU training determinism | PENDING |
| 13 | Isomorphic Patterns | Cross-domain transfer validation | PENDING |

**0 of 13 modules validated.** Module boundaries pending definition from
spring team (paper baselines vs WDM surrogates vs isomorphic patterns).

## Provenance

| Property | Value |
|----------|-------|
| Origin | `ecoPrimals/springs/neuralSpring` |
| Version | 1.0.0 |
| Spring | neuralSpring |
| Emission method | `litho emit-pseudospore` |
| Integrity | BLAKE3 checksums in `receipts/checksums.blake3` (251 entries) |
| Braid | FermentBraid provenance chain (who/what/when/how) |

## Download

**Archive:** `pseudoSpore_neuralSpring-ML-Surrogates_v1.0.0.tar.gz` (16 MB)
**Verify:** `litho ingest-pseudospore <path> --verify`
