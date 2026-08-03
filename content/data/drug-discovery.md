+++
title = "Drug Discovery — Data Braids"
description = "ChEMBL 37 (15 GB), ZINC20 (160 MB), and PubChem (11 GB) with full sweetGrass provenance braids. The foundation for computational drug repurposing."
date = 2026-08-02
weight = 20

[taxonomies]
springs = ["healthspring"]

[extra]
maturity = "live"
domain = "drug-discovery"
+++

Five datasets forming the core of the computational drug discovery pipeline.
Together they map chemical space (PubChem), bioactivity (ChEMBL + BindingDB),
screening libraries (ZINC20), and disease-specific drug sensitivity (NF Data Portal).
All braided on westGate.

---

## ChEMBL 37 {#chembl-37}

| Field | Value |
|-------|-------|
| **Size** | 15 GB |
| **Files** | 2 |
| **Source** | [EMBL-EBI](https://ftp.ebi.ac.uk/pub/databases/chembl/ChEMBLdb/latest/) |
| **License** | CC-BY-SA-3.0 |
| **Ingested** | July 29, 2026 |
| **Provenance** | 5/5 FULL |
| **Springs** | healthSpring |
| **Gardens** | tideGlass |

2.9M compounds, 24.5M bioactivity measurements, 1.6M assays. The largest
open drug discovery database mapping chemical structures to biological
targets. This is the primary input for tideGlass pharmacometric modeling.

### What's Possible

- Combine with **LINCS L1000** gene expression signatures for drug repurposing
  without wet lab access — the gen5 critical path for tideGlass
- Cross-reference with **PDB** for structure-based virtual screening
- Map compounds to **PubChem** identifiers for cross-database linking
- Feed **healthSpring** clinical models with bioactivity data

### The Braid

sweetGrass `braid.create` produced a W3C PROV-O JSON-LD attestation:

```json
{
  "@context": "https://www.w3.org/ns/prov#",
  "@id": "urn:braid:chembl-37-westgate-20260729",
  "prov:wasGeneratedBy": {
    "@type": "prov:Activity",
    "prov:used": "https://ftp.ebi.ac.uk/pub/databases/chembl/ChEMBLdb/latest/",
    "prov:wasAssociatedWith": "did:eco:westgate"
  },
  "prov:wasAttributedTo": "did:eco:westgate",
  "prov:generatedAtTime": "2026-07-29T...",
  "eco:license": "CC-BY-SA-3.0",
  "eco:blake3_root": "...",
  "eco:file_count": 2,
  "eco:size_bytes": 16106127360
}
```

---

## ZINC20 SMILES (drug-like subset) {#zinc20}

| Field | Value |
|-------|-------|
| **Size** | 160 MB |
| **Files** | 110 |
| **Source** | [UCSF Irwin Lab](https://zinc20.docking.org/) |
| **License** | Free for research |
| **Ingested** | July 29, 2026 |
| **Provenance** | 5/5 FULL |
| **Springs** | healthSpring |
| **Gardens** | tideGlass |

Drug-like compound subset from ZINC20 in SMILES format.
Commercially available molecules filtered for drug-likeness (Lipinski rules).
Virtual screening library for tideGlass.

### What's Possible

- Virtual screening library for **tideGlass** — compounds ready for docking
- Combine with **ChEMBL** bioactivity to prioritize screening candidates
- Cross-reference with **PDB** binding sites for structure-based screening

### The Braid

sweetGrass `braid.create` produced a W3C PROV-O JSON-LD attestation:

```json
{
  "@context": "https://www.w3.org/ns/prov#",
  "@id": "urn:braid:zinc20-westgate-20260729",
  "prov:wasGeneratedBy": {
    "@type": "prov:Activity",
    "prov:used": "https://zinc20.docking.org/",
    "prov:wasAssociatedWith": "did:eco:westgate"
  },
  "prov:wasAttributedTo": "did:eco:westgate",
  "prov:generatedAtTime": "2026-07-29T...",
  "eco:license": "Free for research",
  "eco:blake3_root": "...",
  "eco:file_count": 110,
  "eco:size_bytes": 167772160
}
```

---

## PubChem (SMILES + InChI-Key + Synonym + Mass) {#pubchem}

| Field | Value |
|-------|-------|
| **Size** | 11 GB |
| **Files** | 5 |
| **Source** | [NCBI](https://ftp.ncbi.nlm.nih.gov/pubchem/) |
| **License** | Public Domain |
| **Ingested** | July 30, 2026 |
| **Provenance** | 5/5 FULL |
| **Springs** | healthSpring, wetSpring |
| **Gardens** | tideGlass |

Chemical compound identifiers, structures (SMILES/InChI-Key), synonyms,
and molecular masses from the world's largest free chemistry database.
The glue layer for cross-database chemical identity resolution.

### What's Possible

- Chemical identifier resolution for cross-database linking (ChEMBL, ZINC, MassBank)
- Feed **MassBank** spectral matching with exact masses for unknown compound identification
- Map **ChEMBL** bioactivity to PubChem compound metadata

### The Braid

sweetGrass `braid.create` produced a W3C PROV-O JSON-LD attestation:

```json
{
  "@context": "https://www.w3.org/ns/prov#",
  "@id": "urn:braid:pubchem-westgate-20260730",
  "prov:wasGeneratedBy": {
    "@type": "prov:Activity",
    "prov:used": "https://ftp.ncbi.nlm.nih.gov/pubchem/",
    "prov:wasAssociatedWith": "did:eco:westgate"
  },
  "prov:wasAttributedTo": "did:eco:westgate",
  "prov:generatedAtTime": "2026-07-30T...",
  "eco:license": "Public Domain",
  "eco:blake3_root": "...",
  "eco:file_count": 5,
  "eco:size_bytes": 11811160064
}
```

---

## BindingDB binding affinity {#bindingdb}

| Field | Value |
|-------|-------|
| **Size** | 583 MB |
| **Files** | 1 |
| **Source** | [BindingDB](https://www.bindingdb.org/rwd/bind/index.jsp) |
| **License** | CC-BY-3.0 |
| **Ingested** | August 2, 2026 |
| **Provenance** | 5/5 FULL |
| **Springs** | healthSpring |
| **Gardens** | tideGlass |

2.9M+ binding affinity measurements (Ki, Kd, IC50, EC50) linking drug
compounds to protein targets. Structure-activity relationship analysis
at the binding level.

### What's Possible

- Combine with **ChEMBL** for comprehensive bioactivity landscape across two databases
- Cross-reference with **PDB** for structure-based affinity prediction
- Feed **tideGlass** compound ranking with experimentally measured affinities

### The Braid

sweetGrass `braid.create` produced a W3C PROV-O JSON-LD attestation:

```json
{
  "@context": "https://www.w3.org/ns/prov#",
  "@id": "urn:braid:bindingdb-westgate-20260802",
  "prov:wasGeneratedBy": {
    "@type": "prov:Activity",
    "prov:used": "https://www.bindingdb.org/rwd/bind/index.jsp",
    "prov:wasAssociatedWith": "did:eco:westgate"
  },
  "prov:wasAttributedTo": "did:eco:westgate",
  "prov:generatedAtTime": "2026-08-02T...",
  "eco:license": "CC-BY-3.0",
  "eco:blake3_root": "...",
  "eco:file_count": 1,
  "eco:size_bytes": 611319808
}
```

---

## NF Data Portal (Synapse) {#nf-data-portal}

| Field | Value |
|-------|-------|
| **Size** | 666 MB |
| **Files** | 658 |
| **Source** | [NF Data Portal / Sage Bionetworks](https://nf.synapse.org/) |
| **License** | Synapse Terms of Use |
| **Ingested** | August 2, 2026 |
| **Provenance** | 5/5 FULL |
| **Springs** | healthSpring |
| **Gardens** | tideGlass |

NF1 high-throughput drug screening (8K compounds + structures),
NF2 Synodos drug screen, NF2 kinomics (peptide-level, protein-level,
differential expression). tideGlass Module 7 — completes the 7/7 base data.

### What's Possible

- NF-specific drug repurposing: combine NF drug sensitivity with **LINCS** perturbation
  signatures for neurofibromatosis therapeutic candidates
- Cross-reference NF kinomics with **ChEMBL** kinase inhibitor bioactivity
- Feed **tideGlass** Module 7 for the NF extension of the drug repurposing pipeline

### The Braid

sweetGrass `braid.create` produced a W3C PROV-O JSON-LD attestation:

```json
{
  "@context": "https://www.w3.org/ns/prov#",
  "@id": "urn:braid:nf-data-portal-westgate-20260802",
  "prov:wasGeneratedBy": {
    "@type": "prov:Activity",
    "prov:used": "https://nf.synapse.org/",
    "prov:wasAssociatedWith": "did:eco:westgate"
  },
  "prov:wasAttributedTo": "did:eco:westgate",
  "prov:generatedAtTime": "2026-08-02T...",
  "eco:license": "Synapse Terms of Use",
  "eco:blake3_root": "...",
  "eco:file_count": 658,
  "eco:size_bytes": 698351616
}
```

---

## See Also

- [Gene Expression](/data/gene-expression/) — LINCS L1000 (the drug repurposing partner)
- [Structural Biology](/data/structural-biology/) — PDB (binding site structures)
- [What's Possible](/data/possible/) — the drug repurposing combination
