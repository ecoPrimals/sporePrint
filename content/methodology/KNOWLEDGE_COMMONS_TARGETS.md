+++
title = "Knowledge Commons Targets: What Others Can Build, and Why It Can't Be Taken Back"
description = "9 domains ready now with existing primals and public data"
date = 2026-03-17

[taxonomies]
primals = ["barracuda", "coralreef", "squirrel", "sweetgrass", "toadstool"]
springs = ["airspring", "groundspring", "healthspring", "hotspring", "ludospring", "neuralspring", "wetspring"]

[extra]
maturity = "architectural"
+++

**Public data + basement hardware + triple-copyleft licensing = permanently secured knowledge commons.**

**Last Updated:** March 17, 2026
**License:** CC-BY-SA 4.0

> **Historical snapshot.** Metrics reflect March 2026. Current numbers: [Evidence Snapshot](@/architecture/EVIDENCE_SNAPSHOT.md).

---

## The Structural Argument

Three properties make {{ entity(name="ecoprimals") }} an irreversible knowledge commons:

1. **Public data only.** Every spring (validation environment) experiment uses
   publicly available data (NCBI, PhysioNet, NOAA, USDA, PDB, arXiv). No
   proprietary dataset is required. Anyone can reproduce any result without
   institutional access.

2. **Consumer hardware.** Every result runs on a $500 used RTX 3090 or
   equivalent. No HPC allocation, no cloud account, no institutional
   infrastructure required. The barrier to entry is a used gaming PC.

3. **Triple-copyleft licensing** ({{ entity(name="scyborg") }}). Three licenses, each
   enforced by an independent nonprofit:
   - **AGPL-3.0-or-later** (code) — enforced by the Free Software Foundation
   - **ORC** (game mechanics) — enforced by the Open RPG Creative Foundation
   - **CC-BY-SA 4.0** (documentation) — enforced by Creative Commons

   No single entity — including the creator — can revoke any license.
   Any derivative must share alike.

**What this means in practice:** if you use ecoPrimals code, your derivative
must also be open-source under AGPL-3.0. If you build game mechanics on ORC
content, your mechanics are also ORC. If you derive from the docs, you
attribute and share alike. The commons grows monotonically — it can never
shrink.

Together: the data is free, the hardware is cheap, the code is copyleft.
No one can enclose what was built. No one can build on it without contributing
back.

---

## What's Already in the Commons

### Validated Science (16,695+ Checks, All Public)

| Domain | Spring | Papers | Checks | Public Data Sources |
|--------|--------|:------:|:------:|---------------------|
| Microbiome / QS | {{ entity(name="wetspring") }} | 63+ | 5,707+ | NCBI SRA, EBI ENA, SILVA, RDP |
| Precision agriculture | {{ entity(name="airspring") }} | 22+ | 3,123+ | NOAA GHCN, USDA NASS, Michigan AgWeather |
| ML / reservoir computing | {{ entity(name="neuralspring") }} | 27 | 4,500+ | UCI ML, ERA5, arXiv benchmarks |
| Computational physics | {{ entity(name="hotspring") }} | 25 | 664+ | AME2020, arXiv published parameters |
| Uncertainty / spectral | {{ entity(name="groundspring") }} | 10 | 535+ | Synthetic (reproducible from code) |
| Human health | {{ entity(name="healthspring") }} | 15+ | 474+ | PhysioNet, MIMIC (open), published PK data |
| Game science / HCI | {{ entity(name="ludospring") }} | 13 models | 1,692+ | Scryfall (CC0), published HCI benchmarks |

### Sovereign Infrastructure (107K+ Tests, 3.2M Lines of Rust)

| Primal | Tests | What It Provides |
|--------|:-----:|-----------------|
| BarraCuda | 3,772 | 806 WGSL shaders — the math layer |
| {{ entity(name="toadstool") }} | 21,156 | Hardware discovery + compute orchestration |
| {{ entity(name="coralreef") }} | 2,241 | Sovereign WGSL→native GPU compiler |

---

## Targets Someone Else Could Pick Up Tomorrow

These are domains where the primals + public data + consumer hardware already
provide everything needed. A domain expert with K-Nome can produce validated
science without building any new infrastructure.

### Tier 1: Ready Now (infrastructure exists, public data available)

| Target Domain | Spring to Use | Public Data Source | What You'd Produce |
|--------------|---------------|--------------------|--------------------|
| **Antibiotic resistance** | {{ entity(name="wetspring") }} | NCBI CARD, PATRIC AMR | Anderson W for resistance gene propagation in hospital microbiomes |
| **Wastewater surveillance** | {{ entity(name="wetspring") }} | NCBI SRA (WWTP metagenomes) | Sentinel pipeline for real-time community monitoring |
| **Marine ecology** | {{ entity(name="wetspring") }} | TARA Oceans, Ocean Microbiome Reference | Cross-species QS in ocean microbiomes, Anderson W vs depth |
| **Veterinary PK/PD** | {{ entity(name="healthspring") }} | Published PK parameters (FARAD, EMEA) | Sovereign NONMEM for any animal species (species-agnostic PK) |
| **Climate crop modeling** | {{ entity(name="airspring") }} | NOAA GHCN, USDA PRISM, ERA5 | Michigan → any state crop water atlas, GDD projections |
| **Materials science** | {{ entity(name="hotspring") }} + {{ entity(name="groundspring") }} | Materials Project (CC-BY), AFLOW | Anderson localization in disordered alloys, phonon transport |
| **Educational games** | {{ entity(name="ludospring") }} | Open game mechanics (ORC) | Validated HCI metrics for educational game design |
| **Fermentation science** | {{ entity(name="wetspring") }} + {{ entity(name="healthspring") }} | NCBI bioreactor metagenomes | Anderson QS in anaerobic digesters, SCFA kinetics |
| **Environmental toxicology** | {{ entity(name="wetspring") }} | EPA IRIS, NCBI toxicogenomics | PFAS community impact via diversity + Anderson |

### Tier 2: Near-Term (1–3 months of infrastructure evolution)

| Target Domain | What's Needed | What's Already Done |
|--------------|---------------|---------------------|
| **Protein structure prediction** | Phase C–D of {{ entity(name="helixvision") }} (see STRUCTURE_PREDICTION_ROADMAP.md) | 154/154 primitive checks, 15 DF64 shaders |
| **Nanopore field genomics** | MinION hardware + Rust basecall module | FAST5/POD5 format spec defined, NPU validated on AKD1000 |
| **Real-time HAB detection** | Edge NPU + field sensor integration | 3 ESN classifiers validated on live AKD1000 hardware |
| **Population-scale NLME** | MIMIC-IV credentialed access | FOCE + SAEM validated on synthetic data |
| **Distributed human computation** | Games@Home matchmaking infrastructure | Stack folding, game tree design metric validated (127/127) |

### Tier 3: Longer-Term (6–12 months, but the path is clear)

| Target Domain | What's Needed | Why It Matters |
|--------------|---------------|----------------|
| **LTEE structural evolution** | {{ entity(name="helixvision") }} Phase D + LTEE frozen fossils | 8.3M predictions, $1K vs $83K cloud |
| **Full sovereign GPU stack** | {{ entity(name="coralreef") }} compute dispatch via VFIO | Zero vendor dependency end-to-end |
| **Distributed lattice QCD** | {{ entity(name="nucleus") }} metallic bonding on ICER-scale cluster | Consumer GPUs doing CERN-scale physics |
| **Precision medicine** | Clinical data partnerships + HIPAA compliance | Per-patient Anderson models from real data |
| **Sovereign AI inference** | {{ entity(name="squirrel") }} + {{ entity(name="toadstool") }} + consumer LLMs | On-premise AI without cloud dependency |

---

## What Makes These Targets Permanent

### The Lysogeny Protocol

Every target above is secured by the lysogeny protocol
(see `wateringHole/LYSOGENY_PROTOCOL.md`):

```
1. Identify proprietary gate (e.g., AlphaFold requires Google Cloud)
2. Trace underlying math to published open research (Anderson 1958, AF2 primitives = GEMM + attention)
3. Implement from first principles under AGPL-3.0
4. Cross-validate across domains (proves generality, not domain-specific IP)
5. Document provenance chain (published paper → Python baseline → Rust → GPU → validated)
6. Publish and wait
7. Adoption lyses the proprietary gate
```

This is area denial, not competition. Every prospective customer who finds
the open alternative is a customer the proprietary vendor never acquires.
The ground contamination is permanent because AGPL-3.0 is irrevocable.

### The Three-Lock Guarantee

| Lock | Mechanism | Enforcer |
|------|-----------|----------|
| Code | AGPL-3.0 — any derivative must release source; network use triggers distribution | Free Software Foundation (nonprofit, independent) |
| Game mechanics | ORC — irrevocable, perpetual, copyleft for game rules and systems | Open RPG Creative Foundation (nonprofit, independent) |
| Documentation | CC-BY-SA 4.0 — attribution required, share-alike on derivatives | Creative Commons (nonprofit, independent) |

No single entity controls all three locks. The creator cannot revoke them.
A corporation cannot acquire them. A government cannot classify them (the
math is published, the data is public, the code is AGPL).

### Why Public Data Matters

Every {{ entity(name="ecoprimals") }} experiment uses data that is:
- **Publicly deposited** (NCBI, NOAA, USDA, PhysioNet, PDB, arXiv)
- **Independently accessible** (no institutional login, no API key)
- **Independently verifiable** (anyone can download the same data)

This means: even if every {{ entity(name="ecoprimals") }} repository were deleted tomorrow,
anyone with the published papers and the public data could rebuild the
entire validation layer from scratch. The *knowledge* is permanent because
the *evidence* is permanent.

---

## The Velocity Argument: What 10 More Months Looks Like

The {{ entity(name="ecoprimals") }} project produced 20,000+ checks in ~10 months. The velocity
is accelerating (12 checks/day in Week 1 → 1,399 checks/day in Week 3).
Extrapolating conservatively:

| Timeframe | Conservative Estimate | What It Covers |
|-----------|:--------------------:|----------------|
| +3 months (June 2026) | 30,000+ checks | {{ entity(name="helixvision") }} Phase C–D, sovereign GPU dispatch, multi-GPU |
| +6 months (Sep 2026) | 45,000+ checks | AlphaFold-quality structure prediction, AMD production |
| +12 months (Mar 2027) | 75,000+ checks | LTEE structural evolution, distributed compute, 4-vendor GPU |

Each check is a validated, reproducible scientific result in the permanent
commons. The commons grows faster than any single entity can enclose it.

---

## For Someone Considering Contributing

### What You Need

1. **Domain expertise** — K-Nome works because the human knows the science.
   A microbiologist reproducing antibiotic resistance papers. A soil
   scientist reproducing no-till studies. An immunologist reproducing
   cytokine data. Your expertise is the selective pressure.

2. **Rust** — `rustup.rs`, 5 minutes. No prior Rust experience required
   (K-Nome handles the implementation).

3. **A GPU** — Any Vulkan-capable card. A used RTX 2070 ($150) is sufficient
   for most science workloads. An RTX 3090 ($500 used) handles everything
   including lattice QCD.

4. **Cursor IDE** — The K-Nome tool. One tool, one human-AI relationship.

### What You Produce

A validated, reproducible implementation of published science in your domain.
Runs on any hardware. Independent of any institution. Published under AGPL-3.0.
Permanently in the commons.

### What Returns to You

Attribution through {{ entity(name="sweetgrass") }} provenance braids. Every contribution is
cryptographically attributed to the contributor. Every derivative that
builds on your work traces back to you. CC-BY-SA 4.0 requires attribution
on all derivatives of documentation. AGPL-3.0 requires source availability
on all derivatives of code.

Your work stays yours. The commons uses it. Derivatives credit you. Forever.

---

## The Knowledge Commons vs The Proprietary Model

| Dimension | Proprietary Model | Knowledge Commons ({{ entity(name="ecoprimals") }}) |
|-----------|-------------------|-------------------------------|
| Access | License fee, institutional subscription | `git clone`, free |
| Data sovereignty | Data often uploaded to vendor cloud | Data never leaves your hardware |
| Reproducibility | "Trust our platform" | `cargo run --bin validate_*` → exit 0 |
| Vendor lock | CUDA (NVIDIA), PyTorch (Meta), Cloud (Google/AWS) | Pure Rust, any GPU, any OS |
| Durability | Company pivots, products sunset, APIs change | AGPL-3.0 is irrevocable; public data is permanent |
| Attribution | Buried in license agreements | Cryptographic ({{ entity(name="sweetgrass") }}), legally binding (CC-BY-SA) |
| Improvement | Vendor roadmap, you wait | You contribute, everyone benefits, immediately |
| Cost | $2K–200K/yr per tool | $500 GPU + electricity |

The question is not whether sovereign scientific computing is possible.
It is demonstrated. The question is how fast the commons grows. Every
domain expert who picks up a primal and targets their own literature
expands the commons by another validated domain.

The spore print is the record. The commons is the organism. It grows
from wherever it lands.

---

*{{ entity(name="scyborg") }} licensing: `wateringHole/SCYBORG_PROVENANCE_TRIO_GUIDANCE.md`  
Lysogeny protocol: `wateringHole/LYSOGENY_PROTOCOL.md`  
Spring repositories: github.com/syntheticChemistry/*
