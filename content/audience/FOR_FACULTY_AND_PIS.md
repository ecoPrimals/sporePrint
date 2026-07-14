+++
title = "ecoPrimals for Principal Investigators — What This Actually Replaces in Your Lab"
description = "What ecoPrimals replaces in a lab, what it costs, and what it produces"
date = 2026-03-17

[taxonomies]
primals = ["barracuda", "beardog", "biomeos", "petaltongue"]
springs = ["groundspring", "healthspring", "hotspring", "neuralspring", "wetspring"]
trails = ["first-visit", "grant-ready"]
+++

**From:** ecoPrimal — human + synthetic intelligence  
**Organization:** {{ entity(name="ecoprimals") }}
**Date:** March 17, 2026
**Repositories:** github.com/ecoPrimals — all AGPL-3.0-or-later

> **Historical snapshot.** Metrics in this document reflect March 2026 (~3.2M LOC, ~107K tests). Current ecosystem metrics are on the [Evidence Snapshot](@/architecture/EVIDENCE_SNAPSHOT.md) page ({{ total_stat(stat="total_loc_display") }} LOC, {{ total_stat(stat="total_tests_display") }} tests, measured {{ total_stat(stat="measured_date") }}). Technical analysis remains accurate.

---

## The Short Version

Your lab probably runs some combination of Python/R bioinformatics, commercial
pharmacometric software, and ad hoc data management. {{ entity(name="ecoprimals") }} is a pure Rust
stack that replaces most of that with faster, reproducible, GPU-accelerated
alternatives — and adds things no commercial tool does (cryptographic provenance,
physics-based drug scoring, vendor-agnostic GPU compute).

Every claim below has a `cargo run --bin validate_*` binary that proves it.
You can clone the repo and verify on your own hardware.

---

## Find Your Domain

**Physics & Materials** — The {{ entity(name="guidestone") }} deployment artifact validates published lattice QCD, plasma physics, molecular dynamics, and spectral theory results on commodity hardware. A single binary, no CUDA, no vendor SDK. Consumer GPUs do real f64 science via Vulkan. See the [guideStone](@/guidestone/_index.md) section and [Papers 01, 06, 07, 10, 14, 23, 25](@/science/_index.md).

**Pharmacology & Immunology** — The {{ entity(name="basecamp") }} paper program has reproduced dose-response curves, pharmacokinetics, tissue-geometry modeling, and drug repurposing scoring from published veterinary and human data. Anderson localization applied to cytokine signaling is original work. See [Papers 12, 13, 22](@/science/_index.md).

**Microbiology & Genomics** — Sovereign 16S pipelines, metagenomics, phylogenetics, PFAS detection, and quorum sensing models — all in pure Rust, all reproducing published results. See [Papers 02, 03, 04, 05, 09, 16](@/science/_index.md).

**Game Science & Creative Computing** — Rigorous HCI models, game design as science, distributed computation, and {{ entity(name="esotericwebb") }} as a proof that sovereign infrastructure produces real creative software. See [Papers 17, 18, 19, 24](@/science/_index.md).

---

## What You Actually Save

| Tool | What You Pay | {{ entity(name="ecoprimals") }} Replacement | Status |
|------|:----------:|----------------------|--------|
| NONMEM | ~$2,000/yr | {{ entity(name="healthspring") }} FOCE estimation | Validated on synthetic (Exp075) |
| Monolix | ~$1,500/yr | {{ entity(name="healthspring") }} SAEM estimation | Validated on synthetic (Exp075) |
| WinNonlin (Phoenix) | ~$3,000/yr | {{ entity(name="healthspring") }} NCA (λz, AUC∞, MRT, CL, Vss) | Full parity (Exp075) |
| CRO population PK | $50K–200K/program | GPU Monte Carlo (100K patients, RTX 4070) | Validated (Exp005) |
| Galaxy server (local) | $50K+ setup | {{ entity(name="wetspring") }} 16S pipeline (sovereign Rust) | Full parity, 306 binaries |
| QIIME2 + conda | Free + sysadmin time | {{ entity(name="wetspring") }} (no Python, no conda, no Docker) | Full parity |
| MassHunter/Chromeleon | ~$10K+/yr | {{ entity(name="wetspring") }} Track 2 (mzML/EIC/peaks/PFAS) | Full parity on analysis (no instrument control) |

**Minimum annual savings: $6,500 in licenses alone. CRO avoidance: $50K+ per program.**

---

## What You Actually Get That's Better

### Speed

| Operation | Python/R | {{ entity(name="ecoprimals") }} (CPU) | {{ entity(name="ecoprimals") }} (GPU) |
|-----------|:--------:|:-----------------:|:----------------:|
| Hill dose-response (6 cytokines) | ~3.6 ms | ~0.04 ms (84×) | ~0.02 ms (207 M/s) |
| SCFA kinetics | ~1.2 ms | ~0.007 ms (160×) | GPU-ready |
| Beat classification (1000 beats) | ~30 ms | ~0.2 ms (149×) | GPU-ready |
| Shannon/Simpson/Pielou diversity | ~0.5 ms | ~0.01 ms | GPU kernel validated |
| Spectral cosine matching | baseline | — | 1,077× speedup |
| Population PK (10K patients) | minutes | seconds | seconds (100K on GPU) |

### Reproducibility

- Every validation binary has hardcoded expected values with explicit tolerances
- `#![forbid(unsafe_code)]` — zero undefined behavior, guaranteed by the compiler
- `cargo clippy` with pedantic + nursery lints: zero warnings across every spring
- Deterministic: same input → same output, always. No Jupyter state, no Python version drift
- One build command: `cargo build --release`. No conda, no pip, no Docker, no sysadmin

### What No Commercial Tool Offers

| Capability | What It Does |
|-----------|-------------|
| **Anderson localization for community structure** | Maps microbial diversity onto condensed matter physics; predicts cytokine/QS signal propagation vs confinement in tissue or soil |
| **Geometry-aware drug repurposing** | Adds spatial tissue penetration to Fajgenbaum MATRIX pathway scoring — a drug must reach its target through real tissue geometry |
| **Cryptographically signed results** | Every diversity index, IC50, drug score gets an Ed25519 signature. Non-repudiable. |
| **Sample chain-of-custody** | Cryptographic DAG from sample collection to publication. Maps to ISO 17025/15189 traceability. Detects 6 fraud types automatically |
| **Vendor-agnostic GPU** | WebGPU (WGSL) runs on NVIDIA, AMD, Intel, Apple. No CUDA lock-in |
| **NPU edge deployment** | BrainChip AKD1000 at 18.8K Hz inference, coin-cell power. Pure Rust driver |

---

## How It Works With Your Existing Infrastructure

### If You Have Sequencing (Genomics Core / RTSF)

```
Your sequencer → FASTQ files
    → wetSpring 16S pipeline (FASTQ→QC→merge→derep→DADA2→chimera→taxonomy→diversity→UniFrac)
    → Anderson localization analysis (novel community structure physics)
    → Provenance chain (every step signed, auditable, ISO-mappable)
```

{{ entity(name="wetspring") }} replaces the Galaxy/QIIME2/mothur/R pipeline with a single `cargo run`.
All 306 validation binaries pass. 63 published papers reproduced.

### If You Have an HTS Core (like ADDRC)

```
Compound library → plate reader → IC50 data
    → healthSpring GPU Hill sweep (207 M/s on RTX 4070)
    → MATRIX pathway scoring (Fajgenbaum 2019 reproduced)
    → Anderson geometry scoring (tissue penetration physics)
    → Ranked candidates → back to wet lab validation
```

The GPU shader can score 8,000 compounds × 6 cytokine pathways in seconds.
Traditional screening informatics (ActivityBase, GREENScreen) scores compounds
but doesn't consider tissue geometry — drugs that can't reach their target
score well in silico but fail in vivo.

### If You Have ICER Access

```
ICER A100 allocation → barraCuda WGSL shaders (vendor-agnostic)
    → Anderson eigensolve at L=200 (production scale)
    → Population PK at 10M patients
    → MATRIX scoring at 4K × 18K scale (72M evaluations)
```

WGSL shaders compiled by wgpu run on any Vulkan-capable GPU. No CUDA required.
No NVIDIA lock-in. The same binary that runs on your lab's RTX 3060 runs on
ICER's A100s.

### If You Run Clinical Trials

```
Patient data → healthSpring PK/PD pipeline
    → NONMEM-equivalent FOCE estimation (sovereign, no Fortran)
    → NCA (λz, AUC∞, MRT, CL, Vss — WinNonlin replacement)
    → NLME diagnostics (CWRES, VPC, GOF)
    → petalTongue visualization → clinical dashboard
    → BearDog signed results → audit trail
```

Every intermediate result is signed. The provenance chain maps to
21 CFR Part 11 requirements. No Fortran compiler. No proprietary binary.

---

## What We Honestly Can't Do Yet

| Gap | Why | Timeline |
|-----|-----|----------|
| **Real clinical data** | FOCE/SAEM validated on synthetic only | MIMIC-IV access closes this gap |
| **FDA submission formatting** | Infrastructure exists, no CTD/eCTD layer | Formatting, not algorithms |
| **GUI workflow builder** | CLI + validation binaries only | {{ entity(name="petaltongue") }} provides dashboards; Galaxy-style builder not planned |
| **Multi-user web interface** | Local/LAN only | {{ entity(name="biomeos") }} IPC supports multi-client; web tier not planned |
| **Instrument control** | Analysis only, no instrument drivers | We analyze what instruments produce, not drive them |
| **Established community** | One developer, public repos | {{ total_stat(stat="total_loc_display") }} lines of Rust, {{ total_stat(stat="total_tests_display") }} tests, {{ total_stat(stat="papers_reproduced") }} papers reproduced, all validation executable |
| **Formal GxP audit** | Architecture maps to GxP; no auditor has reviewed it | Needs institutional partner |
| **Training/workshops** | No formal curriculum | K-Nome methodology documented; course design planned |

---

## How To Evaluate

```bash
# Pick the spring relevant to your domain:
git clone git@github.com:syntheticChemistry/healthSpring.git  # PK/PD, clinical
git clone git@github.com:syntheticChemistry/wetSpring.git     # 16S, metagenomics, LC-MS

# Build (requires Rust 1.87+ from rustup.rs — 2 minute install)
cd healthSpring && cargo test --workspace   # 613 tests, 0 failures
cd wetSpring/barracuda && cargo test --workspace  # 1,443 tests, 0 failures

# Run a specific validation
cargo run --release --bin exp001_hill       # Hill dose-response
cargo run --release --bin validate_diversity # Shannon/Simpson/Pielou/Chao1

# No Python. No R. No conda. No Docker. No licenses. No cloud account.
```

---

## Published Work Reproduced With Full Provenance

The springs reproduce published, peer-reviewed science as acceptance tests for the infrastructure. Each entry below is a researcher whose published work has been independently reimplemented in Rust, cross-validated against the original Python/R/MATLAB results, and promoted to GPU — with every check automated, every tolerance explicit, and every result fully public under the {{ entity(name="scyborg") }} license.

| Researcher | Department | Published Domain | Spring | Papers Reproduced |
|------------|-----------|------------------|--------|:-----------------:|
| Christopher Waters | MMG, MSU | Quorum sensing, biofilm | {{ entity(name="wetspring") }} | 7 |
| Kevin Liu | CMSE, MSU | Phylogenetics, HMM | {{ entity(name="wetspring") }} | 6 |
| Michael Murillo | CMSE, MSU | Plasma physics, MD | {{ entity(name="hotspring") }} | 22 |
| Andrea Gonzales | PhmTox, MSU | JAK inhibitors, AD | {{ entity(name="wetspring") }}, {{ entity(name="neuralspring") }}, {{ entity(name="healthspring") }} | 6 (G1–G6) |
| Rika Anderson | Biology, Carleton | Metagenomics, pangenomics | {{ entity(name="wetspring") }} | 6 |
| A. Daniel Jones | BMB, MSU | PFAS mass spectrometry | {{ entity(name="wetspring") }} | 2 |
| Ilya Kachkovskiy | Math, MSU | Spectral theory, Anderson | {{ entity(name="wetspring") }}, {{ entity(name="groundspring") }} | 1+ |
| Jesse Cahill | Sandia | Algal monitoring | {{ entity(name="wetspring") }} | 1 |
| Chuck Smallwood | Sandia | Bloom surveillance | {{ entity(name="wetspring") }} | 1 |

Total across all springs: **{{ total_stat(stat="papers_reproduced") }} papers reproduced, {{ total_stat(stat="total_tests_display") }} test functions, {{ total_stat(stat="total_loc_display") }} lines of Rust.** Every reproduction is executable: `cargo run --release --bin validate_*` reproduces the result on your hardware.

---

## Contact

ecoPrimal — github.com/ecoPrimals
Written and developed by ecoPrimal: human + synthetic intelligence.
Built on ~$15,000 of consumer hardware. Zero cloud bills. Zero licenses.
