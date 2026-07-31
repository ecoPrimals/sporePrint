+++
title = "blueFish — Sovereign Data Pipeline"
description = "Sovereign ETL and data pipeline — NCBI integration, format conversion, no cloud lock-in."
date = 2026-03-31

[taxonomies]
primals = ["beardog", "biomeos", "nestgate", "rhizocrypt", "loamspine", "sweetgrass"]
springs = ["wetspring", "primalspring"]

[extra]
foundation = true
+++

**Repository**: sporeGarden/blueFish (moving from {{ entity(name="syntheticchemistry") }} — repo pending)  
**License**: {{ entity(name="scyborg") }} (AGPL-3.0-or-later + ORC + CC-BY-SA 4.0)

---

## What It Is

{{ maturity(level="architectural") }} blueFish is a sovereign data pipeline and ETL (Extract-Transform-Load) tool for scientific data. It handles NCBI database integration, format conversion between bioinformatics standards, and data ingestion for the primal ecosystem — all without sending data to external services.

For any lab working with sequence data, taxonomic databases, or clinical datasets, blueFish provides a local pipeline that respects data sovereignty: your data stays on your hardware, processed by auditable code, with full provenance tracking.

The composition architecture is defined and the constituent primals are validated independently. The blueFish product packaging and integration layer is in development.

---

## Key Capabilities

- **NCBI Integration**: Direct access to NCBI databases (GenBank, SRA, Taxonomy) with local caching and incremental updates
- **Format Conversion**: FASTA, FASTQ, SAM/BAM, VCF, GFF3, BED, and other bioinformatics formats
- **Provenance**: Every transformation step is logged with {{ entity(name="beardog") }}-signed provenance via the {{ entity(name="rootpulse") }} composition
- **Offline Operation**: Once data is fetched, all processing runs locally — no network required
- **Pipeline Composition**: Integrates with {{ entity(name="biomeos") }} {{ entity(name="neuralapi") }} for orchestrated multi-step pipelines

---

## How It Composes

blueFish consumes primals for data integrity and orchestration:

| Primal | What It Provides |
|--------|-----------------|
| {{ entity(name="nestgate") }} | Content-addressed storage for raw and processed datasets |
| {{ entity(name="beardog") }} | Cryptographic verification of data integrity |
| {{ entity(name="biomeos") }} | Pipeline orchestration via deploy graphs |
| 💧🔬 {{ entity(name="wetspring") }} | Validation of bioinformatics outputs against published methods |

---

## Why It Matters

Most bioinformatics pipelines are shell script chains: fragile, unreproducible, and tied to specific cluster configurations. blueFish replaces that with typed Rust pipelines that compose via JSON-RPC, run identically on a laptop and a cluster, and produce cryptographically signed outputs.

The combination of blueFish (data pipeline) + {{ entity(name="helixvision") }} (structure prediction) + {{ entity(name="wetspring") }} (microbiology validation) creates a sovereign structural genomics stack that runs on consumer hardware.

---

*See also: [wetSpring](@/architecture/SPRING_CATALOG.md) for microbiology validation,
[Deployment Model](@/architecture/DEPLOYMENT_MODEL.md) for the {{ entity(name="byob") }} workflow,
[Ecosystem Inventory](@/architecture/ECOSYSTEM_INVENTORY.md) for the full repository map.*
