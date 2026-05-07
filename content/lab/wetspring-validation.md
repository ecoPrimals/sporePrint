+++
title = "wetSpring Validation Results"
description = "235+ bioinformatics checks across 8 workloads, dispatched through a live 13-primal NUCLEUS composition with full provenance chains."
date = 2026-05-06
weight = 20

[extra]
domain = "Bioinformatics"

[taxonomies]
springs = ["wetspring"]
primals = ["toadstool", "rhizocrypt", "loamspine", "sweetgrass", "nestgate", "barracuda"]
+++

## Overview

10 workloads dispatched through {{ entity(name="toadstool") }} on a live 13-primal
{{ entity(name="nucleus") }} composition. 235+ structured science checks passed across
6 bioinformatics domains. Real NCBI data (11.9M paired-end reads, PRJNA488170)
processed through both Python and Rust pipelines. Every result provenance-verified.

**Hardware**: Intel i9-14900K, 96 GB DDR5, RTX 4070 / RTX 3090
**Composition**: Full NUCLEUS (13 primals)
**Provenance**: DAG → Merkle root → loamSpine ledger → sweetGrass braid

---

## Results by Domain

### 16S Microbiome Pipeline

| Workload | Checks | Data | Duration |
|----------|--------|------|----------|
| 16S Rust Validation | 37/37 PASS | Synthetic pipeline vectors | <1s |
| Algae 16S (real data) | 34/34 PASS | SRR7760408 — 11.9M reads | 23s |
| Python 16S Baseline | SUCCESS | SRR7760408 — 50K reads | 1s |

Full DADA2 denoising, chimera detection, taxonomy assignment, UniFrac distances.
Python→Rust parity at `tol=0.000000` for Shannon and Simpson diversity indices.
Source: Nannochloropsis outdoor 16S (Wageningen, DOI: 10.1007/s00253-022-11815-3).

### Diversity Metrics

| Workload | Checks | Domains Covered |
|----------|--------|-----------------|
| Diversity Indices | 27/27 PASS | Alpha diversity, beta diversity, PCoA |
| R Industry Parity | 53/53 PASS | vegan 2.7.3, DADA2 1.22.0, phyloseq 1.38.0 |

53 checks against R gold-standard packages at exact parity — Shannon, Simpson,
Bray-Curtis, rarefaction, Chao1, Pielou, UniFrac (weighted + unweighted),
cophenetic distances.

### Pharmacometrics & Physics

| Workload | Checks | Domains Covered |
|----------|--------|-----------------|
| Gonzales CPU Parity | 43/43 PASS | Hill equation, PK decay, Anderson spectral |
| Python Benchmark | SUCCESS | Cross-domain timing baseline |

IC50→barrier mapping, dose-response modeling, Anderson 2D/3D lattice
computations with deterministic seed parity.

### Immunology & Metagenomics

| Workload | Checks | Reference |
|----------|--------|-----------|
| Fajgenbaum Pathway | 8/8 PASS | JCI 2019 — PI3K/AKT/mTOR, sirolimus ranking |
| Cold Seep Pipeline | 8/8 PASS | Ruff et al. — 50 synthetic communities |
| Real NCBI Pipeline | 25/25 PASS | Sovereign diversity + Anderson |

Fajgenbaum correctly identifies PI3K/AKT/mTOR as highest-activation pathway (0.92)
and ranks sirolimus #1 for drug repurposing.

---

## Provenance Chain

Every workload result is tracked through a 9-phase provenance pipeline:

| Phase | Operation | Primals Used |
|-------|-----------|-------------|
| 1 | Health check all primals | All 13 |
| 2 | Create rhizoCrypt DAG session | {{ entity(name="rhizocrypt") }} |
| 3 | Create loamSpine spine | {{ entity(name="loamspine") }} |
| 4 | Register NCBI data artifacts (BLAKE3) | {{ entity(name="nestgate") }}, {{ entity(name="rhizocrypt") }} |
| 5 | Execute workloads with DAG tracking | {{ entity(name="toadstool") }}, {{ entity(name="rhizocrypt") }} |
| 6 | Dehydrate DAG → Merkle root | {{ entity(name="rhizocrypt") }} |
| 7 | Commit to permanent ledger | {{ entity(name="loamspine") }} |
| 8 | Create attribution braid | {{ entity(name="sweetgrass") }} |
| 9 | Write manifest + braid JSON | Filesystem |

The braid is PROV-O compliant with DID attribution and ed25519 witness signature
from {{ entity(name="beardog") }}'s Tower-tier key hierarchy.

---

## BLAKE3 Data Artifacts

Every input is content-addressed before pipeline execution:

| Artifact | BLAKE3 Hash (prefix) | Size |
|----------|---------------------|------|
| SRR7760408 R1 | `6250f200f9ff45e0...` | 2.1 GB |
| SRR7760408 R2 | `cd89f43d74d09c64...` | 2.4 GB |
| SRR5534045 R1 | `096878541679cd06...` | 444 MB |
| SRR5534045 R2 | `bee510af71ac9149...` | 451 MB |

Source: NCBI SRA accessions SRR7760408 (PRJNA488170) and SRR5534045 (PRJNA382322).
Anyone can download the same reads and verify the hashes.

---

## Interactive Exploration

The [Gonzales Interactive Explorer](@/science/gonzales_explorer.md) provides
live charts for the IC50, PK decay, tissue geometry, hormesis, and cross-species
data validated in the Gonzales CPU Parity workload above.

---

## Reproduce These Results

See [Reproduce It Yourself](@/lab/reproduce.md) for step-by-step instructions
to run the same workloads on your own hardware.
