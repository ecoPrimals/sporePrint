+++
title = "wetSpring Life Science Analytics"
description = "Life science and analytical chemistry — 16S rRNA, LTEE variant calling, LC-MS/PFAS, and ODE models"
date = 2026-07-18
template = "spore_gallery.html"

[taxonomies]
springs = ["wetspring"]

[extra]
domain = "Life Science & Analytical Chemistry"
spore_name = "wetSpring-Life-Science-Analytics"
spore_version = "1.0.0"
spore_origin = "ecoPrimals/springs/wetSpring"
spore_spring = "wetSpring"
spore_status = "PENDING"
modules_pass = 0
modules_total = 0
methods = ["16S rRNA diversity", "variant calling", "LC-MS quantification", "ODE population dynamics", "HMM basecalling"]
tools = ["Rust (UniBin)", "Python (Tier 1 baseline)", "barraCuda GPU"]
+++

## Domain Profile

Life science and analytical chemistry validation. Covers 16S rRNA community
analysis, LTEE variant calling (breseq parity), LC-MS/PFAS quantification,
ODE population dynamics, Anderson physics, and drug repurposing workflows.
The UniBin framework unifies validation across 7 entity groups with 346
scenarios and 5,967+ validation checks.

**Status:** pseudoSpore v1.0.0 emitted (782 KB, 180 files). Module validation
pending — JSON baselines included, 5.2 GB FASTQ data excluded (SRA manifest
needed for lazy-fetch). 2,160 workspace tests.

## Module Status

| # | Module | Description | Status |
|---|--------|-------------|--------|
| 1 | 16S Diversity | Rarefaction, Shannon, Simpson indices | PENDING |
| 2 | Variant Calling | breseq-parity LTEE mutations | PENDING |
| 3 | LC-MS/PFAS | Chromatographic peak quantification | PENDING |
| 4 | ODE Models | Lotka-Volterra, SIR, population dynamics | PENDING |
| 5 | HMM Basecalling | GPU-accelerated basecall parity | PENDING |
| 6 | Anderson Physics | Localization transition in biological noise | PENDING |
| 7 | Drug Repurposing | Target similarity network scoring | PENDING |

**0 of 7 modules validated.** Awaiting spring validation runs and FASTQ scoping.

## Provenance

| Property | Value |
|----------|-------|
| Origin | `ecoPrimals/springs/wetSpring` |
| Version | 1.0.0 |
| Spring | wetSpring |
| Emission method | `litho emit-pseudospore` |
| Integrity | BLAKE3 checksums in `receipts/checksums.blake3` (169 entries) |
| Braid | FermentBraid provenance chain (who/what/when/how) |

## Download

**Archive:** `pseudoSpore_wetSpring-Life-Science-Analytics_v1.0.0.tar.gz` (782 KB)
**Verify:** `litho ingest-pseudospore <path> --verify`
