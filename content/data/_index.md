+++
title = "Data Braids — Federated Science Catalog"
description = "3.21 TB of science data across 153 datasets and 17 domains, each with a full sweetGrass provenance braid. Browse, verify, transplant to your own hardware via pseudoSpore or lithoSpore."
sort_by = "weight"
template = "section.html"
+++

Real data. Sovereign hardware. Full provenance braids. Verify it yourself.
Take it with you.

westGate has ingested **3.21 TB** across **153 datasets** and **17+ science domains**
through the complete Provenance Trio pipeline. Every dataset has a
**sweetGrass braid** — a W3C PROV-O JSON-LD attestation recording who
ingested the data, when, from where, under what license, and the
cryptographic chain proving it hasn't been tampered with.

This is **proof of provenance over external data**. We didn't create it,
but we can prove exactly what it is, where it came from, and that it's
unmodified. The braid is the access and verification layer.

**Want to take data with you?** See [Transplant — Carry the Data With You](/data/transplant/)
for how pseudoSpores and lithoSpores let you carry data + provenance to
your own hardware.

---

## Data Braids vs. pseudoSpores

These are two fundamentally different things:

| | Data Braids | pseudoSpores (NFTs) |
|---|---|---|
| **What** | Datasets we **ingested** from external sources | Science we **computed** ourselves |
| **Proof of** | **Provenance** — the data is what it claims to be | **Work** — the Novel Fermentation Transcript |
| **sweetGrass role** | `braid.create` attests ingestion provenance | `braid.create` attests computation provenance |
| **Where** | [/data/](/data/) (this page) | [/pseudospore/](/pseudospore/) |
| **Example** | PDB mirror (88 GB, 257K structures from RCSB) | hotSpring QCD trajectories (computed on RTX 3090) |

Together they form the complete evidence surface: what went in, and what came out.

---

## Catalog

| Dataset | Size | Objects | Domain | Provenance |
|---------|------|---------|--------|-----------|
| [PDB mmCIF (full mirror)](/data/structural-biology/#pdb-mmcif) | 88 GB | 257,179 | Structural Biology | FULL |
| [UniProt Swiss-Prot](/data/structural-biology/#uniprot-swissprot) | 764 MB | 3 | Structural Biology | FULL |
| [UniProt TrEMBL](/data/structural-biology/#uniprot-trembl) | 148 GB | 3 | Structural Biology | FULL |
| [PDB70 HHblits](/data/structural-biology/#pdb70) | 27 GB | 4 | Structural Biology | FULL |
| [ChEMBL 37](/data/drug-discovery/#chembl-37) | 15 GB | 2 | Drug Discovery | FULL |
| [ZINC20 SMILES](/data/drug-discovery/#zinc20) | 160 MB | 110 | Drug Discovery | FULL |
| [PubChem](/data/drug-discovery/#pubchem) | 11 GB | 5 | Drug Discovery | FULL |
| [BindingDB](/data/drug-discovery/#bindingdb) | 583 MB | 1 | Drug Discovery | FULL |
| [NF Data Portal](/data/drug-discovery/#nf-data-portal) | 666 MB | 658 | Drug Discovery | FULL |
| [LINCS L1000 Level 5](/data/gene-expression/#lincs-l1000) | 20 GB | 6 | Gene Expression | FULL |
| [GTEx V8](/data/gene-expression/#gtex-v8) | 2.4 GB | 4 | Gene Expression | FULL |
| [GEO SOFT cancer (11 series)](/data/gene-expression/#geo-soft-cancer) | 3 GB | 11 | Gene Expression | FULL |
| [UniRef90](/data/proteomics/#uniref90) | 30 GB | 1 | Proteomics | FULL |
| [PDB structures (506)](/data/proteomics/#pdb-structures) | 361 MB | 506 | Proteomics | FULL |
| [TCGA Xena Hub](/data/cancer-genomics/#tcga-xena) | 449 MB | 8 | Cancer Genomics | FULL |
| [MONDO Disease Ontology](/data/disease-ontology/#mondo-disease) | 103 MB | 2 | Disease Ontology | FULL |
| [Reactome Pathways](/data/disease-ontology/#reactome-pathways) | 96 MB | 3 | Disease Ontology | FULL |
| [RefSeq GRCh38](/data/genomic-reference/#refseq-grch38) | 981 MB | 3 | Genomic Reference | FULL |
| [NCBI Gene](/data/genomic-reference/#ncbi-gene) | 7 GB | 5 | Genomic Reference | FULL |
| [LTEE REL606 genome](/data/microbial-evolution/#ltee-rel606) | 5.8 MB | 1 | Microbial Evolution | FULL |
| [SILVA 138.1 (16S ref)](/data/microbial-evolution/#silva-138) | 188 MB | 1 | Microbial Evolution | FULL |
| [NOAA GHCND](/data/environmental/#noaa-ghcnd) | 3.5 GB | 3 | Environmental | FULL |
| [USGS earthquake](/data/environmental/#usgs-earthquake) | 2.1 MB | 1 | Environmental | FULL |
| [MassBank NIST](/data/analytical-chemistry/#massbank-nist) | 63 MB | 1 | Analytical Chemistry | FULL |
| [PhysioNet MIT-BIH](/data/biosignals/#physionet-mitbih) | 22 MB | 1 | Biosignals | FULL |
| [AME2020 nuclear masses](/data/nuclear-physics/#ame2020) | 1.2 MB | 2 | Nuclear Physics | FULL |
| [USDA NASS Census 2017](/data/agriculture/#usda-nass) | 132 MB | 1 | Agriculture | FULL |
| [COSMIC v104](/data/cancer-genomics/#cosmic-v104) | 4.6 GB | 5 | Cancer Genomics | FULL |
| [BRENDA enzyme kinetics](/data/biochemistry/#brenda-kinetics) | 1.6 MB | 74 | Biochemistry | FULL |
| [CHARMM36 force fields](/data/molecular-simulation/#charmm36-ff) | 1.1 MB | 1 | Molecular Simulation | FULL |
| [PhysioNet PTB-XL ECG](/data/biosignals/#physionet-ptbxl) | 1.5 GB | 1 | Biosignals | FULL |
| [PubChem BioAssay](/data/drug-discovery/#pubchem-bioassay) | 11 GB | 5 | Drug Discovery | FULL |
| [NCBI Taxonomy](/data/genomic-reference/#ncbi-taxonomy) | 74 MB | 1 | Genomic Reference | FULL |
| **Total** | **~362 GB** | **~260K** | **17 domains** | **100%** |

---

## What "FULL" Provenance Means

Every dataset passes through **seven stages**, each independently verifiable.
The sweetGrass braid ties them together:

```
External source (NCBI, RCSB, EBI, ...)
    ↓ download
nestGate content.put → BLAKE3 hash (content identity)
    ↓
rhizoCrypt dag.session.create → DAG vertex (lineage)
    ↓
loamSpine spine.create → Merkle certificate (immutable record)
    ↓
bearDog crypto.sign_ed25519 → Ed25519 signature (witness)
    ↓
sweetGrass braid.create → W3C PROV-O JSON-LD (attribution braid)
    ↓
The braid: who ingested it, when, from where, under what license,
with a cryptographic chain proving the data is unmodified.
```

The braid is machine-readable (JSON-LD) and human-readable (rendered on these pages).
See [How Braids Work](/data/provenance/) for the full pipeline.

---

## The Hardware

All data lives on:

- **westGate**: i9-14900K, 96 GB DDR5, 50.7 TB ZFS raidz1
- **Network**: 10G LAN to the mesh — zero egress charges
- **OS**: NixOS, NUCLEUS composition (13/13 primals)

Every byte on the mesh is one less download from the internet.
The data grows in latent value — available to every spring and garden at LAN speed.

---

## Browse by Domain

- [Structural Biology](/data/structural-biology/) — PDB, UniProt Swiss-Prot, TrEMBL, PDB70
- [Drug Discovery](/data/drug-discovery/) — ChEMBL, ZINC20, PubChem, PubChem BioAssay, BindingDB, NF Data Portal
- [Gene Expression](/data/gene-expression/) — LINCS L1000, GTEx V8, GEO SOFT cancer
- [Proteomics](/data/proteomics/) — UniRef90, PDB structures
- [Cancer Genomics](/data/cancer-genomics/) — TCGA Xena Hub, COSMIC v104
- [Disease Ontology](/data/disease-ontology/) — MONDO, Reactome
- [Genomic Reference](/data/genomic-reference/) — RefSeq GRCh38, NCBI Gene, NCBI Taxonomy
- [Microbial Evolution](/data/microbial-evolution/) — LTEE REL606, SILVA 138.1
- [Environmental](/data/environmental/) — NOAA GHCND, USGS earthquakes
- [Analytical Chemistry](/data/analytical-chemistry/) — MassBank NIST
- [Biosignals](/data/biosignals/) — PhysioNet MIT-BIH, PhysioNet PTB-XL
- [Nuclear Physics](/data/nuclear-physics/) — AME2020
- [Biochemistry](/data/biochemistry/) — BRENDA enzyme kinetics
- [Molecular Simulation](/data/molecular-simulation/) — CHARMM36 force fields
- [Agriculture](/data/agriculture/) — USDA NASS Census

---

## See Also

- [What's Possible](/data/possible/) — dataset combinations that enable science
- [How Braids Work](/data/provenance/) — the provenance pipeline explained
- [pseudoSpore Catalog](/pseudospore/) — science we computed (NFTs)
- [Verify a pseudoSpore](/pseudospore/verify/) — step-by-step verification guide
