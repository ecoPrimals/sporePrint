+++
title = "healthSpring Clinical PK-PD"
description = "Clinical pharmacokinetics and pharmacodynamics — drug interaction prediction and population modeling"
date = 2026-05-31
template = "spore_gallery.html"

[taxonomies]
springs = ["healthspring"]

[extra]
domain = "Clinical Pharmacology"
spore_name = "healthSpring-Clinical-PKPD"
spore_version = "1.0.0"
spore_origin = "ecoPrimals/springs/healthSpring"
spore_spring = "healthSpring"
spore_status = "PENDING"
modules_pass = 0
modules_total = 6
methods = ["compartmental ODE", "Hill equation", "Monte Carlo population PK", "PBPK modeling", "Anderson localization"]
tools = ["Rust (ODE solver)", "WGSL (barraCuda Monte Carlo)", "Python (baselines)"]
+++

## Domain Profile

Clinical pharmacokinetics and pharmacodynamics validation covering PBPK
compartmental modeling, dose-response curves, drug-drug interaction prediction,
microbiome-mediated metabolism, and GPU-accelerated population pharmacokinetics.

**Status:** pseudoSpore v1.0.0 emitted (375 KB, 374 files). Module validation
pending — spring team needs to run validators and populate `validation.json`.

## Module Status

| # | Module | Description | Status |
|---|--------|-------------|--------|
| 1 | PBPK Compartments | Multi-compartment ODE time-concentration curves | PENDING |
| 2 | PD Response | Hill equation dose-response surfaces | PENDING |
| 3 | Drug-Drug Interaction | CYP450 inhibition AUC fold-change prediction | PENDING |
| 4 | Microbiome Metabolism | Anderson localization colonization resistance | PENDING |
| 5 | Population PK | GPU Monte Carlo parameter distributions (N=10,000) | PENDING |
| 6 | Symbiont PK-PD | LTEE B5 colonization + therapeutic molecule production | PENDING |

**0 of 6 modules passing.** Awaiting spring validation runs.

## Provenance

| Property | Value |
|----------|-------|
| Origin | `ecoPrimals/springs/healthSpring` |
| Version | 1.0.0 |
| Spring | healthSpring |
| Emission method | `litho emit-pseudospore` |
| Integrity | BLAKE3 checksums in `receipts/checksums.blake3` (363 entries) |
| Braid | FermentBraid provenance chain (who/what/when/how) |

## Validation Checks

From `domain_profile.toml`:

- Compartment mass balance (±0.1%)
- Hill curve monotonicity and EC50 inflection
- DDI AUC fold-change within 2-fold of published clinical data
- Microbiome diversity indices within ecological bounds
- Population PK 95% CI covers reference ranges
- Cross-tier parity (Rust vs Python golden values)
- GPU/CPU determinism (barraCuda bit-identical to cpu_fallback)

## Translation Groups

| Group | Domain Concept | Computation |
|-------|---------------|-------------|
| pbpk_compartments | Organ-level ADME | Multi-compartment RK4 ODE |
| pd_response | Dose-response curves | Hill function with EC50/Emax |
| drug_interaction | CYP450 inhibition/induction | Mechanistic static models |
| microbiome_metabolism | Gut xenobiotic metabolism | Anderson + Michaelis-Menten |
| population_pk | Inter-individual variability | GPU Monte Carlo sampling |
| symbiont_pkpd | Engineered symbiont therapy | Logistic + Hill coupled ODE |
