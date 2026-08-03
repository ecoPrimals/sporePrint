+++
title = "Microbial Evolution — Data Braids"
description = "LTEE REL606 genome (5.8 MB) and SILVA 138.1 16S reference (188 MB) with full sweetGrass provenance braids."
date = 2026-08-02
weight = 50

[taxonomies]
springs = ["wetspring"]

[extra]
maturity = "live"
domain = "microbial-evolution"
+++

Reference genome and taxonomy database for evolutionary biology
and microbial community analysis. The anchor datasets for wetSpring.

---

## LTEE REL606 genome {#ltee-rel606}

| Field | Value |
|-------|-------|
| **Size** | 5.8 MB |
| **Files** | 1 |
| **Source** | [NCBI](https://www.ncbi.nlm.nih.gov/nuccore/CP000819.1) |
| **License** | Public Domain |
| **Ingested** | July 28, 2026 |
| **Provenance** | 5/5 FULL |
| **Springs** | wetSpring |
| **Gardens** | lithoSpore |

Reference genome for *E. coli* B strain REL606 — the ancestral strain
of Lenski's Long-Term Evolution Experiment (LTEE). 4.6M bp, 4,432 genes.
The starting point for tracking 75,000+ generations of evolution in the
longest-running evolution experiment in history.

### What's Possible

- Anchor for LTEE evolutionary dynamics analysis (**wetSpring** breseq pipeline)
- Combine with **SILVA 138.1** for phylogenetic context of evolved populations
- Feed **lithoSpore** for self-verifying genomic artifacts
- Map mutations across 75K generations to functional annotations in **UniProt**

### The Braid

sweetGrass `braid.create` produced a W3C PROV-O JSON-LD attestation:

```json
{
  "@context": "https://www.w3.org/ns/prov#",
  "@id": "urn:braid:ltee-rel606-westgate-20260728",
  "prov:wasGeneratedBy": {
    "@type": "prov:Activity",
    "prov:used": "https://www.ncbi.nlm.nih.gov/nuccore/CP000819.1",
    "prov:wasAssociatedWith": "did:eco:westgate"
  },
  "prov:wasAttributedTo": "did:eco:westgate",
  "prov:generatedAtTime": "2026-07-28T...",
  "eco:license": "Public Domain",
  "eco:blake3_root": "...",
  "eco:file_count": 1,
  "eco:size_bytes": 6082560
}
```

---

## SILVA 138.1 (16S ref) {#silva-138}

| Field | Value |
|-------|-------|
| **Size** | 188 MB |
| **Files** | 1 |
| **Source** | [SILVA](https://www.arb-silva.de/download/archive/) |
| **License** | CC-BY-4.0 |
| **Ingested** | July 29, 2026 |
| **Provenance** | 5/5 FULL |
| **Springs** | wetSpring |

16S rRNA reference taxonomy database. Gold standard for microbial
community classification via amplicon sequencing. Used by DADA2,
QIIME2, and every major microbiome analysis pipeline.

### What's Possible

- Classify microbial communities from 16S amplicon data
  (**wetSpring** GPU-accelerated DADA2 pipeline)
- Combine with **LTEE REL606** for evolutionary context of *E. coli* populations

### The Braid

sweetGrass `braid.create` produced a W3C PROV-O JSON-LD attestation:

```json
{
  "@context": "https://www.w3.org/ns/prov#",
  "@id": "urn:braid:silva-138-westgate-20260729",
  "prov:wasGeneratedBy": {
    "@type": "prov:Activity",
    "prov:used": "https://www.arb-silva.de/download/archive/",
    "prov:wasAssociatedWith": "did:eco:westgate"
  },
  "prov:wasAttributedTo": "did:eco:westgate",
  "prov:generatedAtTime": "2026-07-29T...",
  "eco:license": "CC-BY-4.0",
  "eco:blake3_root": "...",
  "eco:file_count": 1,
  "eco:size_bytes": 197132288
}
```

---

## See Also

- [Environmental](/data/environmental/) — NOAA, USGS (environmental context)
- [Proteomics](/data/proteomics/) — UniRef90 (protein-level evolution)
