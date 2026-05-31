+++
title = "hotSpring CompChem GuideStone"
description = "Computational chemistry validation — free energy landscapes via well-tempered metadynamics"
date = 2026-05-31
template = "spore_gallery.html"

[taxonomies]
springs = ["hotspring"]

[extra]
domain = "Computational Chemistry"
spore_name = "hotSpring-CompChem-GuideStone"
spore_version = "1.6.1"
spore_origin = "ecoPrimals/springs/hotSpring"
spore_spring = "hotSpring"
spore_status = "COMPLETE"
modules_pass = 7
modules_total = 8
methods = ["well-tempered metadynamics", "free energy landscapes", "Cremer-Pople puckering", "2D collective variables"]
tools = ["GROMACS", "PLUMED", "Rust (Tier 2 validator)", "Python (Tier 1 baseline)"]
+++

## Domain Profile

This pseudoSpore validates computational chemistry methods for carbohydrate
conformational analysis. It reproduces free energy landscapes (FELs) for
xylose ring puckering in enzyme-bound and free states using well-tempered
metadynamics with GROMACS/PLUMED.

The guideStone verification class ensures that every energy surface can be
independently re-derived from raw simulation data using documented commands.

## Module Status

| # | Module | Collective Variables | Duration | Status |
|---|--------|---------------------|----------|--------|
| 1 | Alanine dipeptide FEL | φ/ψ 2D Ramachandran | 10 ns | PASS |
| 2 | Xylose puckering (free) | θ 1D Cremer-Pople | 10 ns | PASS |
| 3 | Enzyme-bound puckering | θ 1D Cremer-Pople | 10 ns | PASS |
| 4 | Free xylose 2D | qx, qy 2D Cremer-Pople | 20 ns | PASS |
| 5 | Enzyme-bound 2D | qx, qy 2D Cremer-Pople | 20 ns | PASS |
| 6 | Cross-tier parity | Rust vs Python (all) | — | PASS |
| 7 | Convergence analysis | Block averaging | — | PASS |
| 8 | GPU parity | barraCuda vs CPU | — | PENDING |

**7 of 8 modules passing.** Module 8 (GPU parity) awaits barraCuda FEL kernel deployment.

## Provenance

| Property | Value |
|----------|-------|
| Origin | `ecoPrimals/springs/hotSpring` |
| Version | 1.6.1 |
| Spring | hotSpring |
| Emission method | `litho emit-pseudospore` |
| Integrity | BLAKE3 checksums in `receipts/checksums.blake3` |
| Braid | FermentBraid provenance chain (who/what/when/how) |

## Zero-Trust Derivation

Every output in this pseudoSpore can be independently verified:

```bash
# Extract and verify checksums
cd pseudoSpore_hotSpring-CompChem-GuideStone_v1.6.1/
b3sum --check receipts/checksums.blake3

# Re-derive a free energy surface from raw HILLS data
plumed sum_hills --hills data/xylose-puckering-fel/HILLS \
  --mintozero --outfile /tmp/fes_theta.dat
diff outputs/xylose-puckering-fel/fes_theta.dat /tmp/fes_theta.dat
```

## Tier Parity

| Tier | Implementation | Role |
|------|---------------|------|
| Tier 1 | Python (NumPy/SciPy) | Golden reference values |
| Tier 2 | Rust (hotSpring binary) | Production validator |
| Tier 3 | WGSL (barraCuda kernel) | GPU-accelerated (pending) |

Cross-tier RMSD tolerances: < 1.0 kJ/mol for 1D, < 2.0 kJ/mol for 2D surfaces.

## Download

The lithoSpore archive contains all raw data, configurations, outputs, and
verification scripts needed to independently reproduce every result.

**Archive:** `pseudoSpore_hotSpring-CompChem-GuideStone_v1.6.1.tar.gz`
**Verify:** `litho ingest-pseudospore <path> --verify`
