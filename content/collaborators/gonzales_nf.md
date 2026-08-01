+++
title = "Gonzales — NF Data Mining & Drug Discovery"
description = "Neurofibromatosis data mining collaboration: the first multi-product composition for external science, with CTF NDU grant alignment."
date = 2026-06-06

[taxonomies]
primals = ["barracuda", "biomeos", "coralreef", "nestgate", "rhizocrypt", "sweetgrass", "toadstool"]
springs = ["wetspring", "healthspring", "hotspring", "neuralspring"]
trails = ["nf-pipeline"]

[extra]
foundation = true
status = "engaged"

[[extra.companions]]
url = "/products/nf-case-study/"
title = "NF Case Study"
relation = "pairs_with"
label = "The multi-product composition driving this collaboration"

[[extra.companions]]
url = "/products/tideglass/"
title = "tideGlass"
relation = "extends"
label = "GPS platform rebuild as first concrete deliverable"

[[extra.companions]]
url = "/methodology/sovereign-publication/"
title = "Sovereign Publication"
relation = "methodology"
label = "How this collaboration publishes — code not papers"
+++

**Status**: Engaged — product composition mapped, CTF NDU aligned
**Domain**: Drug discovery transcriptomics, neurofibromatosis data mining, One Health cross-species
**Products**: {{ entity(name="helixvision") }} + {{ entity(name="healthspring") }} + {{ entity(name="initiochem") }} + coralForge (multi-product composition)
**Springs Fed**: {{ entity(name="wetspring") }}, {{ entity(name="healthspring") }}, {{ entity(name="neuralspring") }}, {{ entity(name="hotspring") }}
**Funding**: CTF NF Data Utilization Award (NDU) — up to $125K total ($50K Y1 + $75K Y2)

---

## Two Active Threads

### Thread 0: GPS Platform Rebuild — tideGlass

The collaborator assigned the GPS platform (Bin Chen Lab) for rebuild as a pseudoSpore — the first concrete deliverable.

**Platform**: tideGlass (cross-spring biological current parser)
**Source**: Deep learning-based screening and design of novel therapeutics that reverse disease-associated transcriptional phenotype — Cell (2026)

**What GPS does**: Predicts drug-induced gene expression changes from chemical structures (SMILES to expression profile), then screens for compounds that *reverse* disease transcriptomic signatures. Three modules:
- **RCL**: Robust Collaborative Learning — cleans noisy drug-induced profiles
- **GPS4Drug**: Structure to expression prediction + virtual screening
- **MolSearch**: Multi-objective lead optimization via MCTS

**pseudoSpore target**: Full lineage trace, reproduce every figure, rebuild sovereign, package with automated validation checks.

### Thread 1: NF Data Mining Project

Neurofibromatosis data mining tied to the Children's Tumor Foundation. This is the first multi-product composition for an external PI.

**The Biological Connection**: NF1 loss = hyperactive RAS/MAPK. The collaborator studies JAK/STAT via oclacitinib (782/782 validated checks). NF1 tumors have hyperactivated STAT3. Same signaling biology, different disease context.

**Full case study**: [NF Case Study](@/products/nf_case_study.md)

---

## Product Composition

| Product | Role | Existing Validation |
|---------|------|-------------------|
| {{ entity(name="helixvision") }} | Gene expression from NF Data Portal | {{ entity(name="wetspring") }} 6,656+ checks |
| {{ entity(name="healthspring") }} | MATRIX drug repurposing (MEK, JAK, mTOR inhibitors) | {{ entity(name="healthspring") }} 233/233 |
| {{ entity(name="initiochem") }} | MEK inhibitor conformational dynamics | {{ entity(name="hotspring") }} 190/190 |
| coralForge | NF1 variant structural impact | {{ entity(name="neuralspring") }} 154 checks |

---

## Data Sources

| System | Purpose | Status |
|--------|---------|--------|
| NF Data Portal / Synapse | Gene expression, genomic variants, drug screening | Survey |
| Pluto.bio NF datasets | ~108M data points across 12 datasets | Survey |
| PubChem / ChEMBL | Compound activity for RAS/MAPK, JAK/STAT targets | Survey |

---

## New Spring Validation Targets

NF work will create validation targets the ecosystem has never seen:

| Result | Spring Fed | Validation Target |
|--------|-----------|-------------------|
| NF gene expression signatures | {{ entity(name="wetspring") }} | Differential expression in NF1 tumors |
| RAS/MAPK pathway enrichment | {{ entity(name="wetspring") }} | Pathway analysis on NF Data Portal data |
| NF drug repurposing MATRIX scores | {{ entity(name="healthspring") }} | MEK/JAK/mTOR inhibitor ranking |
| Neurofibromin structural variants | {{ entity(name="neuralspring") }} | NF1 mutation structural impact |
| MEK inhibitor binding dynamics | {{ entity(name="hotspring") }} | Selumetinib/trametinib FEL landscapes |

---

## The gen5 Pattern

```
Collaborator brings NF biological question + institutional access + PI authority
    -> Products compose for her domain
    -> Data ingested from NF Data Portal, PubChem, ChEMBL
    -> Computational preliminary data produced (pseudoSpore for NF)
    -> Collaborator uses results for CTF NDU application
    -> If funded: Year 1 bioinformatics -> Year 2 wet lab validation
    -> Results feed back as new spring validation targets
    -> Ecosystem strengthens from external demand
```

---

*The collaborator did not apply to the ecosystem. She invited it to her science. The NF project is her research question answered by tools that happen to be sovereign, validated, and open. If the ecosystem cannot serve her biology, the architecture is wrong. If it can, gen5 is real.*
