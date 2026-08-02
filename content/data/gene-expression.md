+++
title = "Gene Expression — Data Braids"
description = "LINCS L1000 (20 GB, 473K signatures) and GTEx V8 (2.4 GB, 54 tissues) with full sweetGrass provenance braids."
date = 2026-08-02
weight = 30

[taxonomies]
springs = ["wetspring", "healthspring"]

[extra]
maturity = "live"
domain = "gene-expression"
+++

Three datasets covering drug perturbation effects, baseline tissue expression,
and cancer cell line profiling. The combination is the foundation for
computational drug repurposing.

---

## LINCS L1000 Level 5 + metadata {#lincs-l1000}

| Field | Value |
|-------|-------|
| **Size** | 20 GB |
| **Files** | 6 |
| **Source** | [NCBI GEO / Broad Institute](https://www.ncbi.nlm.nih.gov/geo/query/acc.cgi?acc=GSE92742) |
| **License** | CC-BY-4.0 |
| **Ingested** | July 29, 2026 |
| **Provenance** | 5/5 FULL |
| **Springs** | wetSpring |
| **Gardens** | tideGlass |

473K gene expression signatures across 12K genes. Drug perturbation,
gene knockdown, and overexpression profiles across 77 cell lines.
The core dataset for computational drug repurposing — each signature
records how a cell's gene expression changes in response to a chemical
or genetic perturbation.

### What's Possible

- Combine with **ChEMBL 37** for drug mechanism inference — match chemical
  bioactivity to gene expression changes
- Map perturbation signatures to **GTEx** tissue expression for
  tissue-specific drug effect prediction
- The **gen5 critical path** for tideGlass drug repurposing: LINCS + ChEMBL +
  GTEx = computational pharmacology without wet lab access

---

## GTEx V8 expression {#gtex-v8}

| Field | Value |
|-------|-------|
| **Size** | 2.4 GB |
| **Files** | 4 |
| **Source** | [GTEx Consortium / Broad Institute](https://gtexportal.org/home/downloads/adult-gtex/bulk_tissue_expression) |
| **License** | dbGaP (public summary data) |
| **Ingested** | July 29, 2026 |
| **Provenance** | 5/5 FULL |
| **Springs** | wetSpring, healthSpring |

Gene expression across 54 human tissues from 948 donors.
TPM and read count matrices for tissue-specific expression analysis.
The baseline map of where genes are expressed in the human body.

### What's Possible

- Map tissue-specific protein expression with **UniProt Swiss-Prot**
- Identify tissue selectivity of drug candidates with **ChEMBL + LINCS**
- Feed **healthSpring** clinical models with tissue context

---

## GEO SOFT cancer series (11 series) {#geo-soft-cancer}

| Field | Value |
|-------|-------|
| **Size** | 3 GB |
| **Files** | 11 |
| **Source** | [NCBI GEO](https://www.ncbi.nlm.nih.gov/geo/) |
| **License** | Public Domain |
| **Ingested** | August 1, 2026 |
| **Provenance** | 5/5 FULL |
| **Springs** | wetSpring, healthSpring |
| **Gardens** | tideGlass |

11 GEO cancer expression series including CCLE (GSE36139), GDSC (GSE68379),
and other cancer cell line profiling datasets. Pre-processed SOFT format.

### What's Possible

- Drug sensitivity modeling: combine CCLE/GDSC expression with **ChEMBL** bioactivity
- Cancer-specific perturbation profiles for **tideGlass**
- Cross-reference with **TCGA Xena** clinical data for translational analysis

---

## See Also

- [Drug Discovery](/data/drug-discovery/) — ChEMBL, ZINC20, PubChem (the pharmacology layer)
- [Cancer Genomics](/data/cancer-genomics/) — TCGA Xena (clinical data)
- [Structural Biology](/data/structural-biology/) — PDB, UniProt (protein targets)
- [What's Possible](/data/possible/) — the drug repurposing combination
