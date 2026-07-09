+++
title = "🧫 Lab"
description = "Live validation results, spring science hubs, and sovereign compute access. Full provenance. Run it yourself or review the evidence."
sort_by = "weight"
template = "section.html"
+++

The lab is where {{ entity(name="ecoprimals") }} science gets validated on real hardware.
Everything here ran on a sovereign compute node (i9-14900K, 96 GB DDR5, RTX 4070 / RTX 3090 / Akida NPU) through
a live {{ total_stat(stat="total_primals") }}-primal {{ entity(name="nucleus") }} composition. Every result carries a
cryptographic provenance chain: BLAKE3 content hashes → rhizoCrypt DAG → loamSpine
ledger → sweetGrass ed25519-witnessed braid.

**Security status**: All 13 primals default `127.0.0.1` bind. BTSP Phase 3 AEAD on all connections. NestGate method-level auth gating. JH-0 MethodGate adopted 13/13 primals. 445 registered capability methods. Zero DEBT markers (primalSpring Wave 35).

---

## Living Systems

The mesh is not a plan — it's running. See what's deployed, which gates are
online, what capabilities are routed, and the state of the Sovereign CI pipeline.

- [Living Systems — What's Running Now](@/lab/living-systems.md) — active gates, capabilities, JupyterHub status

---

## Spring Science

Each spring validates a scientific domain. These pages tell the full story — what was reproduced, what was discovered, and what it proved about the infrastructure.

| Spring | Domain | Checks | Papers |
|--------|--------|--------|--------|
| [wetSpring](@/lab/springs/wetspring.md) | Life science, metagenomics, PFAS | 5,707+ | 63/63 |
| [hotSpring](@/lab/springs/hotspring.md) | Plasma physics, lattice QCD, spectral | 697+ | 10+ |
| [airSpring](@/lab/springs/airspring.md) | Precision agriculture, irrigation | 2,777+ | 57 |
| [healthSpring](@/lab/springs/healthspring.md) | PK/PD, microbiome, biosignal, drugs | 795 | 7 tracks |
| [groundSpring](@/lab/springs/groundspring.md) | Measurement noise, uncertainty, Anderson | 965+ | 29 baselines |
| [primalSpring](@/lab/springs/primalspring.md) | Composition parity, deploy graphs, NUCLEUS | 666 | — (meta-spring) |

**2 more springs** (neuralSpring, ludoSpring) are documented in the [Spring Catalog](@/architecture/SPRING_CATALOG.md) and are being expanded by upstream contributors.

**Total across all {{ total_stat(stat="total_springs") }} springs**: {{ total_stat(stat="validation_checks") }} quantitative checks, {{ total_stat(stat="papers_reproduced") }} peer-reviewed papers reproduced, 15 researchers across 9 departments. ([Evidence Snapshot](@/architecture/EVIDENCE_SNAPSHOT.md) defines all metrics.)

---

## Validation & Provenance

**Validation results** — 235+ structured science checks across 8 workloads, dispatched
through {{ entity(name="toadstool") }} on a live composition. Real NCBI data (11.9M
paired-end reads) processed through both Python and Rust pipelines. Python→Rust parity
at machine-epsilon precision.

**Provenance evidence** — every artifact content-addressed, every pipeline step tracked
in a DAG session, committed to a permanent ledger, and witnessed with ed25519 signatures.
The braid is PROV-O compliant with DID attribution.

**Reproduce it yourself** — step-by-step instructions to stand up the same composition
on your own hardware and run the same workloads. No cloud. No institutional access.
Commodity hardware.

- [Getting Started with plasmidBin](@/lab/getting-started-plasmidbin.md) — pre-built binaries, 5-minute cold start
- [Reproduce Results](@/lab/reproduce.md) — full NUCLEUS deploy + validated science workloads
- [Provenance Pipeline](@/lab/provenance-pipeline.md) — how results are tracked and verified

---

## The Validation Pattern

```
Published results (papers, databases, NCBI)
        ↓
Python / established tools (QIIME2, SciPy, R vegan/phyloseq)
        ↓
Rust implementation (wetSpring, barraCuda)
        ↓
NUCLEUS composition dispatch (toadStool execute)
        ↓
Provenance chain (BLAKE3 → DAG → ledger → braid)
        ↓
Parity check + gap report
```

Each arrow is independently verifiable. The Rust matches the Python.
The composition matches standalone Rust. Gaps are documented and flow
upstream. Every successful workload is proof that the deploy graphs,
BTSP encryption, discovery hierarchy, and provenance pipeline work
in production.

---

## Public Notebooks

Interactive Jupyter notebooks that visualize baseCamp science. Each notebook loads frozen experiment data (JSON artifacts) from the spring repositories — no live primals required.

| Notebook | Spring | Story |
|----------|--------|-------|
| [16S Pipeline Validation](@/lab/notebooks/01-16s-pipeline-validation.md) | wetSpring | Flagship 16S pipeline, Galaxy/QIIME2 parity, R/vegan cross-validation |
| [Python vs Rust vs GPU](@/lab/notebooks/02-benchmark-python-vs-rust.md) | wetSpring | Benchmark evidence: timing, energy, speedup across three tiers |
| [63/63 Paper Reproductions](@/lab/notebooks/03-paper-reproductions.md) | wetSpring | 5 researchers, 6 tracks, full evidence map |
| [Cross-Spring Connections](@/lab/notebooks/04-cross-spring-connections.md) | wetSpring | 79 barraCuda primitives, constraint-driven discoveries |
| [Soil Anderson Deep Dive](@/lab/notebooks/05-soil-anderson-deep-dive.md) | wetSpring | Anderson localization in soil biology — physics meets ecology |
| [Composition Validation](@/lab/notebooks/gs-01-composition-validation.md) | groundSpring | 6 deploy graphs, guideStone Level 3, verb reconciliation |
| [Benchmark Comparison](@/lab/notebooks/gs-02-benchmark-comparison.md) | groundSpring | Rust vs Python timing, three-mode GPU, 110 barraCuda delegations |
| [Ecosystem Evidence](@/lab/notebooks/gs-03-ecosystem-evidence.md) | groundSpring | 35 experiments, 10 domains, gap resolution, security posture |
| [Cross-Spring Connections](@/lab/notebooks/gs-04-cross-spring-connections.md) | groundSpring | 5 primals consumed, 7 ecosystem flows, patterns pioneered |
| [Measurement Science Deep Dive](@/lab/notebooks/gs-05-measurement-science-deep-dive.md) | groundSpring | Five pillars, 13-tier tolerance architecture, Anderson thread |
| [Composition Validation](@/lab/notebooks/01-composition-validation.md) | primalSpring | Deploy graphs, bond types, profiles, discovery tiers |
| [Benchmark Comparison](@/lab/notebooks/02-benchmark-comparison.md) | primalSpring | Rust vs Python timing, energy, guidestone phases |
| [Ecosystem Evidence](@/lab/notebooks/03-ecosystem-evidence.md) | primalSpring | 85 experiments, gap resolution, security timeline |
| [Cross-Spring Connections](@/lab/notebooks/04-cross-spring-connections.md) | primalSpring | Primal consumption matrix, ecosystem flows |
| [BTSP Security Deep Dive](@/lab/notebooks/05-btsp-security-deep-dive.md) | primalSpring | Per-primal posture, convergence arc, discovery hierarchy |

**Run them yourself**: Clone the spring, `cd notebooks/`, `jupyter lab`. Or access via [JupyterHub](@/lab/compute-access.md).

---

## Compute Access

**JupyterHub** provides multi-user notebook access to the live {{ total_stat(stat="total_primals") }}-primal composition. Every notebook runs against real primals, not mocks. Three access tiers (observer, reviewer, user) ensure open science with appropriate boundaries.

- [Compute Access](@/lab/compute-access.md) — tiers, architecture, how to connect

---

## For PIs and Reviewers

The lab is the evidence record for the [projectFOUNDATION protocol](@/lab/projectfoundation-validation-summary.md). If you're evaluating ecoPrimals for institutional adoption:

1. **Review the spring science hubs** — each spring page shows what was reproduced and how
2. **Check the provenance pipeline** — every result is content-addressed with cryptographic chains
3. **Request reviewer access** — read-only JupyterHub access to see the live workspace
4. **Run it yourself** — the reproduction guide works on commodity hardware, no institutional access needed

The shared workspace at `/shared/abg/showcase/` contains polished results ready for institutional review.

---

## For ABG Members

If you're in the Accelerated Bioinformatics Group, the lab is also your
on-ramp. The same pipelines that produced these results are available
through JupyterHub via the compute sharing tunnel. Your workloads run on
the same composition, with the same provenance. Your science validates
the infrastructure; the infrastructure validates your science.

See [Compute Access](@/lab/compute-access.md) for how to connect, or
[Reproduce It Yourself](@/lab/reproduce.md) to run this on your own hardware.
