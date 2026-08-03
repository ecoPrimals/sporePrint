+++
title = "Genomic Reference — Data Braids"
description = "RefSeq GRCh38 (981 MB) and NCBI Gene (7 GB) with full sweetGrass provenance braids. The coordinate system for human genomics."
date = 2026-08-02
weight = 57

[taxonomies]
springs = ["wetspring", "healthspring"]

[extra]
maturity = "live"
domain = "genomic-reference"
+++

Reference genome and gene annotation databases providing the coordinate
system and functional context for all human genomics work.
All braided on westGate.

---

## RefSeq GRCh38 {#refseq-grch38}

| Field | Value |
|-------|-------|
| **Size** | 981 MB |
| **Files** | 3 |
| **Source** | [NCBI RefSeq](https://ftp.ncbi.nlm.nih.gov/genomes/all/GCF/000/001/405/GCF_000001405.40_GRCh38.p14/) |
| **License** | Public Domain |
| **Ingested** | August 1, 2026 |
| **Provenance** | 5/5 FULL |
| **Springs** | wetSpring |

Human reference genome GRCh38.p14. Primary assembly plus alternate loci.
The coordinate system for all human genomics — every variant, gene, and
regulatory element is positioned on this assembly.

### What's Possible

- Coordinate system for variant calling and genome annotation
- Combine with **NCBI Gene** for variant-to-gene mapping
- Anchor for **wetSpring** genomic analysis pipelines
- Reference for alignment and structural variant detection

### The Braid

sweetGrass `braid.create` produced a W3C PROV-O JSON-LD attestation:

```json
{
  "@context": "https://www.w3.org/ns/prov#",
  "@id": "urn:braid:refseq-grch38-westgate-20260801",
  "prov:wasGeneratedBy": {
    "@type": "prov:Activity",
    "prov:used": "https://ftp.ncbi.nlm.nih.gov/genomes/all/GCF/000/001/405/GCF_000001405.40_GRCh38.p14/",
    "prov:wasAssociatedWith": "did:eco:westgate"
  },
  "prov:wasAttributedTo": "did:eco:westgate",
  "prov:generatedAtTime": "2026-08-01T...",
  "eco:license": "Public Domain",
  "eco:blake3_root": "...",
  "eco:file_count": 3,
  "eco:size_bytes": 1028653056
}
```

---

## NCBI Gene (Homo sapiens) {#ncbi-gene}

| Field | Value |
|-------|-------|
| **Size** | 7 GB |
| **Files** | 5 |
| **Source** | [NCBI Gene](https://ftp.ncbi.nlm.nih.gov/gene/DATA/) |
| **License** | Public Domain |
| **Ingested** | August 2, 2026 |
| **Provenance** | 5/5 FULL |
| **Springs** | wetSpring, healthSpring |
| **Gardens** | tideGlass |

Gene information, gene2GO mappings, gene2refseq cross-references.
Comprehensive gene annotation for human and model organisms.
tideGlass Module 6 base data.

### What's Possible

- Gene ID resolution for cross-database linking across all datasets
- GO term enrichment for **LINCS** perturbation analysis
- Combine with **RefSeq** for variant-to-gene mapping
- Cross-reference with **UniProt** for protein-level annotation
- Feed **tideGlass** Module 6 for gene-aware pharmacology

### The Braid

sweetGrass `braid.create` produced a W3C PROV-O JSON-LD attestation:

```json
{
  "@context": "https://www.w3.org/ns/prov#",
  "@id": "urn:braid:ncbi-gene-westgate-20260802",
  "prov:wasGeneratedBy": {
    "@type": "prov:Activity",
    "prov:used": "https://ftp.ncbi.nlm.nih.gov/gene/DATA/",
    "prov:wasAssociatedWith": "did:eco:westgate"
  },
  "prov:wasAttributedTo": "did:eco:westgate",
  "prov:generatedAtTime": "2026-08-02T...",
  "eco:license": "Public Domain",
  "eco:blake3_root": "...",
  "eco:file_count": 5,
  "eco:size_bytes": 7516192768
}
```

---

## See Also

- [Gene Expression](/data/gene-expression/) — LINCS L1000, GTEx V8 (expression data)
- [Proteomics](/data/proteomics/) — UniRef90 (protein-level cross-reference)
- [Microbial Evolution](/data/microbial-evolution/) — LTEE REL606 (microbial genomics)
