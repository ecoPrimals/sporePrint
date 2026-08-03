+++
title = "Proteomics — Data Braids"
description = "UniRef90 (30 GB) and PDB structures (361 MB, 506 targets) with full sweetGrass provenance braids."
date = 2026-08-02
weight = 40

[taxonomies]
springs = ["wetspring", "neuralspring"]

[extra]
maturity = "live"
domain = "proteomics"
+++

Protein sequence clusters and high-priority structure targets for
homology searches, MSA construction, and molecular dynamics.

---

## UniRef90 {#uniref90}

| Field | Value |
|-------|-------|
| **Size** | 30 GB |
| **Files** | 1 |
| **Source** | [UniProt Consortium](https://ftp.uniprot.org/pub/databases/uniprot/uniref/uniref90/) |
| **License** | CC-BY-4.0 |
| **Ingested** | July 29, 2026 |
| **Provenance** | 5/5 FULL |
| **Springs** | wetSpring, neuralSpring |

Clustered protein sequences at 90% identity. Reduces redundancy while
preserving diversity for homology searches and multiple sequence alignment
construction. The standard reference for evolutionary covariance analysis.

### What's Possible

- Build multiple sequence alignments for structure prediction (**neuralSpring**)
- Feed evolutionary analysis pipelines (**wetSpring**)
- Cross-reference with **PDB** for template-based modeling
- Combine with **UniProt Swiss-Prot** for functional annotation of sequence clusters

### The Braid

sweetGrass `braid.create` produced a W3C PROV-O JSON-LD attestation:

```json
{
  "@context": "https://www.w3.org/ns/prov#",
  "@id": "urn:braid:uniref90-westgate-20260729",
  "prov:wasGeneratedBy": {
    "@type": "prov:Activity",
    "prov:used": "https://ftp.uniprot.org/pub/databases/uniprot/uniref/uniref90/",
    "prov:wasAssociatedWith": "did:eco:westgate"
  },
  "prov:wasAttributedTo": "did:eco:westgate",
  "prov:generatedAtTime": "2026-07-29T...",
  "eco:license": "CC-BY-4.0",
  "eco:blake3_root": "...",
  "eco:file_count": 1,
  "eco:size_bytes": 32212254720
}
```

---

## PDB structures (506 individual) {#pdb-structures}

| Field | Value |
|-------|-------|
| **Size** | 361 MB |
| **Files** | 506 |
| **Source** | [RCSB PDB](https://files.rcsb.org/download/) |
| **License** | CC0-1.0 |
| **Ingested** | July 28, 2026 |
| **Provenance** | 5/5 FULL |
| **Springs** | hotSpring, neuralSpring |

506 individually retrieved PDB structures in PDB format. High-priority
targets selected for molecular dynamics and structure prediction
validation. Each file has its own CAS object and sweetGrass braid.

### What's Possible

- Direct input for **hotSpring** molecular dynamics simulations on sovereign hardware
- Validation targets for **neuralSpring** structure prediction
- Cross-reference with **UniProt** annotations for function assignment

### The Braid

sweetGrass `braid.create` produced a W3C PROV-O JSON-LD attestation:

```json
{
  "@context": "https://www.w3.org/ns/prov#",
  "@id": "urn:braid:pdb-structures-westgate-20260728",
  "prov:wasGeneratedBy": {
    "@type": "prov:Activity",
    "prov:used": "https://files.rcsb.org/download/",
    "prov:wasAssociatedWith": "did:eco:westgate"
  },
  "prov:wasAttributedTo": "did:eco:westgate",
  "prov:generatedAtTime": "2026-07-28T...",
  "eco:license": "CC0-1.0",
  "eco:blake3_root": "...",
  "eco:file_count": 506,
  "eco:size_bytes": 378535936
}
```

---

## See Also

- [Structural Biology](/data/structural-biology/) — PDB mmCIF mirror, UniProt (complementary)
- [Gene Expression](/data/gene-expression/) — GTEx tissue expression (protein context)
