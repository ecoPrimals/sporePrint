+++
title = "Disease Ontology — Data Braids"
description = "MONDO (103 MB) and Reactome pathways (96 MB) with full sweetGrass provenance braids. Disease classification and pathway enrichment."
date = 2026-08-02
weight = 55

[taxonomies]
springs = ["healthspring", "wetspring"]

[extra]
maturity = "live"
domain = "disease-ontology"
+++

Disease classification and biological pathway databases for drug target
identification and pathway enrichment analysis. All braided on westGate.

---

## MONDO Disease Ontology {#mondo-disease}

| Field | Value |
|-------|-------|
| **Size** | 103 MB |
| **Files** | 2 |
| **Source** | [Monarch Initiative](https://mondo.monarchinitiative.org/) |
| **License** | CC-BY-4.0 |
| **Ingested** | August 1, 2026 |
| **Provenance** | 5/5 FULL |
| **Springs** | healthSpring |
| **Gardens** | tideGlass |

Unified disease ontology merging OMIM, Orphanet, EFO, DOID, and NCIt.
Disease-gene and disease-phenotype mappings. tideGlass Module 4 base data.

### What's Possible

- Map disease terms to drug targets via **ChEMBL** bioactivity
- Cross-reference disease-gene associations with **NCBI Gene** annotations
- Feed **tideGlass** Module 4 for disease-aware drug repurposing
- Combine with **Reactome** pathways for disease-specific pathway enrichment

### The Braid

sweetGrass `braid.create` produced a W3C PROV-O JSON-LD attestation:

```json
{
  "@context": "https://www.w3.org/ns/prov#",
  "@id": "urn:braid:mondo-disease-westgate-20260801",
  "prov:wasGeneratedBy": {
    "@type": "prov:Activity",
    "prov:used": "https://mondo.monarchinitiative.org/",
    "prov:wasAssociatedWith": "did:eco:westgate"
  },
  "prov:wasAttributedTo": "did:eco:westgate",
  "prov:generatedAtTime": "2026-08-01T...",
  "eco:license": "CC-BY-4.0",
  "eco:blake3_root": "...",
  "eco:file_count": 2,
  "eco:size_bytes": 107986944
}
```

---

## Reactome Pathway Database {#reactome-pathways}

| Field | Value |
|-------|-------|
| **Size** | 96 MB |
| **Files** | 3 |
| **Source** | [Reactome](https://reactome.org/download-data) |
| **License** | CC-BY-4.0 |
| **Ingested** | August 1, 2026 |
| **Provenance** | 5/5 FULL |
| **Springs** | healthSpring, wetSpring |
| **Gardens** | tideGlass |

2,700+ curated biological pathways covering metabolism, signaling, gene expression,
transport, and disease. Gold standard for pathway enrichment analysis.
tideGlass Module 4 base data.

### What's Possible

- Pathway enrichment for **LINCS** perturbation signatures
- Drug-pathway impact analysis with **ChEMBL**
- Combine **MONDO** disease terms with Reactome pathways for
  disease-specific therapeutic target identification
- Feed **wetSpring** systems biology analysis with curated pathway knowledge

### The Braid

sweetGrass `braid.create` produced a W3C PROV-O JSON-LD attestation:

```json
{
  "@context": "https://www.w3.org/ns/prov#",
  "@id": "urn:braid:reactome-westgate-20260801",
  "prov:wasGeneratedBy": {
    "@type": "prov:Activity",
    "prov:used": "https://reactome.org/download-data",
    "prov:wasAssociatedWith": "did:eco:westgate"
  },
  "prov:wasAttributedTo": "did:eco:westgate",
  "prov:generatedAtTime": "2026-08-01T...",
  "eco:license": "CC-BY-4.0",
  "eco:blake3_root": "...",
  "eco:file_count": 3,
  "eco:size_bytes": 100663296
}
```

---

## See Also

- [Cancer Genomics](/data/cancer-genomics/) — TCGA Xena (clinical data)
- [Gene Expression](/data/gene-expression/) — LINCS L1000 (perturbation signatures)
- [Drug Discovery](/data/drug-discovery/) — ChEMBL (bioactivity)
