+++
title = "Structural Biology — Data Braids"
description = "PDB mmCIF mirror (88 GB, 257K structures) and UniProt Swiss-Prot (764 MB) with full sweetGrass provenance braids."
date = 2026-08-02
weight = 10

[taxonomies]
springs = ["hotspring", "neuralspring"]

[extra]
maturity = "live"
domain = "structural-biology"
+++

Four datasets anchoring protein structure and function analysis.
All ingested on westGate through the full Provenance Trio pipeline.

---

## PDB mmCIF (full mirror) {#pdb-mmcif}

| Field | Value |
|-------|-------|
| **Size** | 88 GB |
| **Files** | 257,179 |
| **Source** | [RCSB PDB](https://files.rcsb.org/pub/pdb/data/structures/divided/mmCIF/) |
| **License** | CC0-1.0 |
| **Ingested** | July 30, 2026 |
| **Provenance** | Manifest + BLAKE3 |
| **Springs** | hotSpring, neuralSpring |

Complete Protein Data Bank mirror in mmCIF format. 257K experimentally
determined 3D structures of proteins, nucleic acids, and complex assemblies
resolved by X-ray crystallography, cryo-EM, and NMR.

### The Braid

sweetGrass `braid.create` produced a W3C PROV-O JSON-LD attestation:

```json
{
  "@context": "https://www.w3.org/ns/prov#",
  "@id": "urn:braid:pdb-mmcif-westgate-20260730",
  "prov:wasGeneratedBy": {
    "@type": "prov:Activity",
    "prov:used": "rsync://rsync.rcsb.org/ftp_data/structures/divided/mmCIF/",
    "prov:wasAssociatedWith": "did:eco:westgate"
  },
  "prov:wasAttributedTo": "did:eco:westgate",
  "prov:generatedAtTime": "2026-07-30T...",
  "eco:license": "CC0-1.0",
  "eco:blake3_root": "...",
  "eco:file_count": 257179,
  "eco:size_bytes": 94489280512
}
```

### What's Possible

- Cross-reference with **ChEMBL 37** binding data for structure-activity analysis
- Feed **neuralSpring** for structure prediction validation against experimental structures
- Combine with **UniProt Swiss-Prot** for function-structure mapping across the proteome
- Input for **hotSpring** molecular dynamics simulations on sovereign hardware

---

## UniProt Swiss-Prot {#uniprot-swissprot}

| Field | Value |
|-------|-------|
| **Size** | 764 MB |
| **Files** | 3 |
| **Source** | [UniProt Consortium](https://ftp.uniprot.org/pub/databases/uniprot/current_release/knowledgebase/complete/) |
| **License** | CC-BY-4.0 |
| **Ingested** | July 29, 2026 |
| **Provenance** | 5/5 FULL |
| **Springs** | wetSpring, hotSpring |

570K+ manually curated and reviewed protein sequence entries with
functional annotations, post-translational modifications, and
cross-references to 180+ external databases.

### What's Possible

- Map tissue-specific expression (**GTEx V8**) to protein function annotations
- Combine with **PDB** for sequence-structure-function triangulation
- Feed **wetSpring** evolutionary analysis with curated functional context

### The Braid

sweetGrass `braid.create` produced a W3C PROV-O JSON-LD attestation:

```json
{
  "@context": "https://www.w3.org/ns/prov#",
  "@id": "urn:braid:uniprot-swissprot-westgate-20260729",
  "prov:wasGeneratedBy": {
    "@type": "prov:Activity",
    "prov:used": "https://ftp.uniprot.org/pub/databases/uniprot/current_release/knowledgebase/complete/",
    "prov:wasAssociatedWith": "did:eco:westgate"
  },
  "prov:wasAttributedTo": "did:eco:westgate",
  "prov:generatedAtTime": "2026-07-29T...",
  "eco:license": "CC-BY-4.0",
  "eco:blake3_root": "...",
  "eco:file_count": 3,
  "eco:size_bytes": 801112064
}
```

---

## UniProt TrEMBL (unreviewed) {#uniprot-trembl}

| Field | Value |
|-------|-------|
| **Size** | 148 GB |
| **Files** | 3 |
| **Source** | [UniProt Consortium](https://ftp.uniprot.org/pub/databases/uniprot/current_release/knowledgebase/complete/) |
| **License** | CC-BY-4.0 |
| **Ingested** | August 1, 2026 |
| **Provenance** | 5/5 FULL |
| **Springs** | wetSpring, neuralSpring |

251M+ unreviewed protein sequences from automated annotation.
Complete proteome coverage for computational biology workflows.

### What's Possible

- Massive sequence space for homology searches across all known life
- Combine with **UniRef90** for clustered analysis at different identity thresholds
- Feed **neuralSpring** for structure prediction at scale

### The Braid

sweetGrass `braid.create` produced a W3C PROV-O JSON-LD attestation:

```json
{
  "@context": "https://www.w3.org/ns/prov#",
  "@id": "urn:braid:uniprot-trembl-westgate-20260801",
  "prov:wasGeneratedBy": {
    "@type": "prov:Activity",
    "prov:used": "https://ftp.uniprot.org/pub/databases/uniprot/current_release/knowledgebase/complete/",
    "prov:wasAssociatedWith": "did:eco:westgate"
  },
  "prov:wasAttributedTo": "did:eco:westgate",
  "prov:generatedAtTime": "2026-08-01T...",
  "eco:license": "CC-BY-4.0",
  "eco:blake3_root": "...",
  "eco:file_count": 3,
  "eco:size_bytes": 158913789952
}
```

---

## PDB70 HHblits database {#pdb70}

| Field | Value |
|-------|-------|
| **Size** | 27 GB |
| **Files** | 4 |
| **Source** | [Söding Lab](https://wwwuser.gwdg.de/~compbiol/data/hhsuite/databases/hhsuite_dbs/) |
| **License** | CC-BY-SA-4.0 |
| **Ingested** | August 1, 2026 |
| **Provenance** | 5/5 FULL |
| **Springs** | neuralSpring |

PDB70 clustered at 70% sequence identity for HHblits remote homology detection.
Template-based structure prediction and profile-profile alignment.

### What's Possible

- Template detection for **neuralSpring** structure prediction
- Combine with **PDB mmCIF** for full template-based modeling pipeline
- Remote homology detection for proteins with no close PDB match

### The Braid

sweetGrass `braid.create` produced a W3C PROV-O JSON-LD attestation:

```json
{
  "@context": "https://www.w3.org/ns/prov#",
  "@id": "urn:braid:pdb70-westgate-20260801",
  "prov:wasGeneratedBy": {
    "@type": "prov:Activity",
    "prov:used": "https://wwwuser.gwdg.de/~compbiol/data/hhsuite/databases/hhsuite_dbs/",
    "prov:wasAssociatedWith": "did:eco:westgate"
  },
  "prov:wasAttributedTo": "did:eco:westgate",
  "prov:generatedAtTime": "2026-08-01T...",
  "eco:license": "CC-BY-SA-4.0",
  "eco:blake3_root": "...",
  "eco:file_count": 4,
  "eco:size_bytes": 28991029248
}
```

---

## See Also

- [Data Braids Index](/data/) — all datasets
- [Proteomics](/data/proteomics/) — UniRef90, PDB structures (complementary)
- [Drug Discovery](/data/drug-discovery/) — ChEMBL, PubChem (cross-reference targets)
