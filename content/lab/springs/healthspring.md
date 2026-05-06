+++
title = "healthSpring — Human Health: PK/PD, Microbiome, Biosignal, Drug Discovery"
description = "Pharmacokinetics, gut microbiome, biosignal processing — 795 checks across 7 clinical tracks, sovereign NLME replaces NONMEM"
date = 2026-05-06
weight = 4

[taxonomies]
primals = ["barracuda", "toadstool", "biomeos", "nestgate"]
springs = ["healthspring", "wetspring", "neuralspring", "groundspring"]
+++

## Domain

Pharmacokinetics, gut microbiome, biosignal processing, endocrinology, comparative medicine, drug discovery, NLME.

**Repository**: [syntheticChemistry/healthSpring](https://github.com/syntheticChemistry/healthSpring)

## The Science Story

healthSpring proves the ecoPrimals math infrastructure extends to human clinical applications. PK/PD models validated against canine data in neuralSpring transfer directly to human therapeutics via allometric scaling. The Anderson localization framework from wetSpring/hotSpring applies to gut microbiome colonization resistance. The "claim verification pipeline" — extracting quantifiable claims from clinical practice literature and validating against published registry data — is a novel methodology that generalizes to any medical reference.

## Headline Results

- **795 checks** (601 Rust + 194 Python cross-validation)
- **7 clinical tracks** spanning the full breadth of human health computing
- **Sovereign NLME** (FOCE/SAEM) replaces proprietary NONMEM/Monolix
- **Species-agnostic PK** — same code handles canine AD, feline hyperthyroid, and human TRT
- **Testosterone-gut axis** (Exp037) bridges microbiome and endocrine via Anderson localization

## Clinical Tracks

| Track | Domain | Key Models |
|-------|--------|------------|
| 1 — PK/PD | Pharmacokinetics, dose-response | Hill, PBPK, population Monte Carlo, Michaelis-Menten |
| 2 — Microbiome | Gut ecology, colonization | Anderson gut lattice, C. diff, FMT, SCFA, serotonin |
| 3 — Biosignal | ECG, HRV, SpO2, EDA | Pan-Tompkins, arrhythmia classification, multi-channel fusion |
| 4 — Endocrinology | Testosterone PK, TRT | IM/pellet depot PK, testosterone-gut axis |
| 5 — NLME | Population PK estimation | Sovereign FOCE/SAEM, NCA, diagnostic plots |
| 6 — Comparative Medicine | Cross-species health | Species-agnostic PK, canine AD, feline hyperthyroid |
| 7 — Drug Discovery | Compound screening | MATRIX scoring, ADDRC HTS, iPSC validation |

## Researchers Reproduced

| Researcher | Department | Domain |
|------------|-----------|--------|
| Andrea J. Gonzales | Pharmacology & Toxicology, MSU | Pharmacology, cytokine signaling |
| Charles Mok | Clinical Practice | Clinical endocrinology, TRT |

## What the Constraint Revealed

The testosterone-gut axis (Exp037) bridges microbiome diversity and endocrine outcomes via Anderson localization, validating a cross-track hypothesis. ODE→WGSL codegen absorbed from wetSpring; uncertainty quantification absorbed from groundSpring — the springs feed each other.

## Cross-Spring Connections

- **← neuralSpring**: Hill/IC50, PK models, allometric scaling → human therapeutics
- **← wetSpring**: diversity indices, Anderson lattice → gut colonization resistance, 16S pipeline
- **← groundSpring**: uncertainty quantification for clinical measurements
- **← ludoSpring**: Fitts/Hick for medical UI evaluation; engagement for patient compliance
- **→ biomeOS NUCLEUS**: distributed health pipeline

## baseCamp Papers

Paper 13 — see [baseCamp Science](/science/) for full list.
