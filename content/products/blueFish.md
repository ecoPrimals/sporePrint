+++
title = "blueFish — Sovereign Data Pipeline"
description = "Sovereign ETL and data pipeline — NCBI integration, format conversion, no cloud lock-in."
date = 2026-03-31

[taxonomies]
primals = ["beardog", "biomeos", "nestgate"]
springs = ["wetspring"]
+++

# blueFish — Sovereign Data Pipeline

**Repository**: sporeGarden/blueFish (moving from syntheticChemistry — repo pending)  
**License**: scyBorg (AGPL-3.0-or-later + ORC + CC-BY-SA 4.0)

---

## What It Is

blueFish is a sovereign data pipeline and ETL (Extract-Transform-Load) tool for scientific data. It handles NCBI database integration, format conversion between bioinformatics standards, and data ingestion for the primal ecosystem — all without sending data to external services.

For any lab working with sequence data, taxonomic databases, or clinical datasets, blueFish provides a local pipeline that respects data sovereignty: your data stays on your hardware, processed by auditable code, with full provenance tracking.

---

## Key Capabilities

- **NCBI Integration**: Direct access to NCBI databases (GenBank, SRA, Taxonomy) with local caching and incremental updates
- **Format Conversion**: FASTA, FASTQ, SAM/BAM, VCF, GFF3, BED, and other bioinformatics formats
- **Provenance**: Every transformation step is logged with BearDog-signed provenance via the RootPulse composition
- **Offline Operation**: Once data is fetched, all processing runs locally — no network required
- **Pipeline Composition**: Integrates with biomeOS Neural API for orchestrated multi-step pipelines

---

## How It Composes

blueFish consumes primals for data integrity and orchestration:

| Primal | What It Provides |
|--------|-----------------|
| 🪺🔒 NestGate | Content-addressed storage for raw and processed datasets |
| 🐻🐕 BearDog | Cryptographic verification of data integrity |
| 🌿🖥️ biomeOS | Pipeline orchestration via deploy graphs |
| 💧🔬 wetSpring | Validation of bioinformatics outputs against published methods |

---

## Why It Matters

Most bioinformatics pipelines are shell script chains: fragile, unreproducible, and tied to specific cluster configurations. blueFish replaces that with typed Rust pipelines that compose via JSON-RPC, run identically on a laptop and a cluster, and produce cryptographically signed outputs.

The combination of blueFish (data pipeline) + helixVision (structure prediction) + wetSpring (microbiology validation) creates a sovereign structural genomics stack that runs on consumer hardware.

---

*See also: [wetSpring](@/architecture/SPRING_CATALOG.md) for microbiology validation,
[Deployment Model](@/architecture/DEPLOYMENT_MODEL.md) for the BYOB workflow,
[Ecosystem Inventory](@/architecture/ECOSYSTEM_INVENTORY.md) for the full repository map.*
