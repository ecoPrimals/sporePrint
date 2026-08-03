+++
title = "Cancer Genomics — Data Braids"
description = "TCGA Pan-Cancer (449 MB, 33 cancer types) with full sweetGrass provenance braid. Clinical and molecular data for drug sensitivity modeling."
date = 2026-08-02
weight = 50

[taxonomies]
springs = ["healthspring"]

[extra]
maturity = "live"
domain = "cancer-genomics"
+++

Clinical and molecular cancer data for drug sensitivity modeling and
translational analysis. All braided on westGate.

---

## TCGA Xena Hub {#tcga-xena}

| Field | Value |
|-------|-------|
| **Size** | 449 MB |
| **Files** | 8 |
| **Source** | [UCSC Xena / NCI](https://xenabrowser.net/datapages/?cohort=TCGA%20Pan-Cancer%20(PANCAN)) |
| **License** | Public Domain |
| **Ingested** | August 1, 2026 |
| **Provenance** | 5/5 FULL |
| **Springs** | healthSpring |
| **Gardens** | tideGlass |

TCGA Pan-Cancer expression, mutation, and clinical data via UCSC Xena Hub.
33 cancer types, 11K+ samples. tideGlass Module 5 base data.

### What's Possible

- Combine with GDSC/CCLE expression (**GEO SOFT cancer**) for multi-dataset
  cancer drug sensitivity modeling
- Cross-reference with **ChEMBL** bioactivity and **Reactome** pathways for
  mechanism-of-action discovery across 33 cancer types
- Feed **healthSpring** clinical outcome models with molecular profiles
- Map drug targets to **PDB** structures for structure-based analysis

### The Braid

sweetGrass `braid.create` produced a W3C PROV-O JSON-LD attestation:

```json
{
  "@context": "https://www.w3.org/ns/prov#",
  "@id": "urn:braid:tcga-xena-westgate-20260801",
  "prov:wasGeneratedBy": {
    "@type": "prov:Activity",
    "prov:used": "https://xenabrowser.net/datapages/?cohort=TCGA%20Pan-Cancer%20(PANCAN)",
    "prov:wasAssociatedWith": "did:eco:westgate"
  },
  "prov:wasAttributedTo": "did:eco:westgate",
  "prov:generatedAtTime": "2026-08-01T...",
  "eco:license": "Public Domain",
  "eco:blake3_root": "...",
  "eco:file_count": 8,
  "eco:size_bytes": 470810624
}
```

---

## See Also

- [Gene Expression](/data/gene-expression/) — LINCS L1000, GTEx V8 (expression context)
- [Drug Discovery](/data/drug-discovery/) — ChEMBL (bioactivity for drug sensitivity)
- [Disease Ontology](/data/disease-ontology/) — Reactome pathways (mechanism context)
