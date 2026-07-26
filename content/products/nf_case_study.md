+++
title = "NF Case Study — First Multi-Product Composition for External Science"
description = "Neurofibromatosis data mining as the gen5 proof case: multiple products composing to serve an external scientist's question, where the deliverable is her science."
date = 2026-05-28

[taxonomies]
primals = ["barracuda", "biomeos", "coralreef", "nestgate", "rhizocrypt", "sweetgrass", "toadstool"]
springs = ["wetspring", "healthspring", "hotspring", "neuralspring", "airspring", "groundspring"]
trails = ["nf-pipeline"]

[extra]
maturity = "architectural"

[[extra.companions]]
url = "/collaborators/gonzales-nf/"
title = "Gonzales — NF Data Mining"
relation = "pairs_with"
label = "The collaborator driving this case study"

[[extra.companions]]
url = "/products/tideglass/"
title = "tideGlass"
relation = "extends"
label = "The sovereign pallet that enables field deployment"

[[extra.companions]]
url = "/architecture/deploy-graph-composition/"
title = "Deploy Graph Composition"
relation = "architecture"
label = "How multiple products compose for this case"
+++

{{ maturity(level="architectural") }} Product composition mapped; computational infrastructure validated; grant alignment in progress.

---

## Why This Project Matters

The NF data mining project is not one product solving one problem. It is the first instance where multiple gen4 products compose to serve an external domain expert's scientific question — and the deliverable is her science, not an infrastructure demo.

This makes it the gen5 proof case: the moment the ecosystem demonstrates that someone else's science comes out the other end.

---

## The Biological Question

### Neurofibromatosis Type 1 (NF1)

NF1 is caused by loss-of-function mutations in the NF1 gene, which encodes neurofibromin — a RAS-GAP (GTPase-activating protein). Loss of neurofibromin means hyperactive RAS/MAPK signaling, driving tumor formation: neurofibromas (benign), plexiform neurofibromas (can transform), and malignant peripheral nerve sheath tumors (MPNST).

### The JAK/STAT Connection

The collaborator already studies JAK/STAT signaling through published oclacitinib work (782/782 checks across G1-G6, {{ entity(name="wetspring") }} Exp 273-286). NF1 tumors exhibit hyperactivated STAT3. This is not a domain shift — it is the same signaling biology extended to a rare disease:

| Published Work | NF Extension |
|----------------|--------------|
| Oclacitinib JAK1 selectivity (G2, G5) | JAK/STAT hyperactivation in NF1 tumors |
| IL-31 dose-response (G1) | Cytokine signaling in neurofibroma microenvironment |
| Three-compartment model: immune/skin/neural (G6) | NF1 affects all three compartments |
| Anderson localization of cytokine propagation | RAS/MAPK propagation through NF1 tissue geometry |

The Fajgenbaum MATRIX with Anderson geometry extension applies directly to NF drug repurposing.

### Key Pathways

- **RAS/MAPK**: Primary driver. NF1 loss leads to hyperactive RAS and uncontrolled proliferation. MEK inhibitors (selumetinib, trametinib) are the current therapeutic focus.
- **JAK/STAT**: Secondary/connected. STAT3 hyperactivation in NF tumors. JAK inhibitors may have synergistic potential.
- **PI3K/mTOR**: Cross-talk with RAS. mTOR inhibitors (rapamycin, everolimus) under investigation.

---

## Data Sources

### NF Data Portal (Synapse)

The Children's Tumor Foundation (CTF) NF Data Portal on Synapse.org hosts curated datasets for neurofibromatosis research:

- Gene expression datasets (bulk RNA-seq, single-cell)
- Genomic variants (WGS/WES of NF1 tumors)
- Drug screening data (compound libraries against NF cell lines)
- Clinical data (de-identified patient records)

### Pluto.bio NF Datasets

~108M data points across 12 datasets — gene expression, drug response, pathway analysis. Publicly browsable.

### Compound Data

- **PubChem / ChEMBL**: Compound activity data for RAS/MAPK and JAK/STAT targets
- **DrugBank**: Approved drug profiles for repurposing candidates

---

## Product Composition

This is the first project requiring multiple gen4 products working together. No single product is sufficient.

### {{ entity(name="helixvision") }} — Gene Expression Analysis

**Role**: Primary genomics pipeline for NF data mining.

- Ingest NF Data Portal gene expression datasets (bulk + single-cell)
- Differential expression analysis: NF1 tumor vs. normal tissue
- Pathway enrichment: RAS/MAPK, JAK/STAT, PI3K/mTOR
- Community profiling of tumor microenvironment
- ESN anomaly detection for outlier identification

**Springs consumed**: {{ entity(name="wetspring") }} (6,656+ checks), {{ entity(name="neuralspring") }}/coralForge (154 checks)

### {{ entity(name="healthspring") }} — Drug Repurposing

**Role**: MATRIX scoring for NF drug candidates.

- Anderson-augmented MATRIX: `Score = pathway_match x tissue_geometry x disorder_factor`
- Dose-response modeling for MEK inhibitors, JAK inhibitors, mTOR inhibitors
- Population PK extrapolation from published NF clinical trials
- Cross-disease comparison: AD (published work) vs. NF (new domain)

**Springs consumed**: {{ entity(name="healthspring") }} (233/233 checks), {{ entity(name="groundspring") }} (statistics/uncertainty)

### {{ entity(name="initiochem") }} — Conformational Dynamics

**Role**: FEL exploration for NF-relevant drug targets.

- Neurofibromin structure: RAS-GAP domain, variant effects on catalytic activity
- MEK inhibitor binding landscapes: selumetinib, trametinib conformational dynamics
- Cross-target comparison with CAZyme FEL pipeline (already validated, 190/190)

**Springs consumed**: {{ entity(name="hotspring") }} (500+ checks, Exp 220 FEL pipeline)

### coralForge — Structure Prediction

**Role**: NF1 variant structural impact.

- AlphaFold2/3 predictions for NF1 mutation effects on neurofibromin structure
- RAS-GAP interface disruption modeling
- Structural basis for LOF classification

**Springs consumed**: {{ entity(name="neuralspring") }} (154 checks)

---

## The Composition Map

```
NF Data Portal (Synapse)      PubChem / ChEMBL
    |                              |
    v                              v
helixVision                  healthSpring
(gene expression,            (MATRIX scoring,
 pathway enrichment,          dose-response,
 tumor microenvironment)      population PK)
    |                              |
    |    coralForge                |
    |    (NF1 variant structure,   |
    |     RAS-GAP modeling)        |
    |         |                    |
    |    initioChem                |
    |    (MEK inhibitor FEL,       |
    |     neurofibromin dynamics)  |
    |         |                    |
    v         v                    v
    +---------+--------------------+
                  |
                  v
    pseudoSpore (NF preliminary data)
    +-- Gene expression analysis results
    +-- Drug repurposing MATRIX scores
    +-- Structural variant analysis
    +-- Conformational dynamics
    +-- Cross-disease comparison (AD -> NF)
    +-- Self-verifying, foundation-ready
```

---

## Grant Alignment

### Children's Tumor Foundation — NF Data Utilization Award (NDU)

Total budget: up to **$125K** over two years. Indirect costs capped at 10%.

- **Year 1** (up to $50K): Data exploration and bioinformatics analysis — gene expression mining, pathway enrichment, drug repurposing scoring, computational preliminary data.
- **Year 2** (up to $75K): In vitro validation of pathways, targets, or biomarkers identified in Year 1 — iPSC models, compound screening, wet lab confirmation of computational predictions.

### What the Ecosystem Brings

- Validated computational pipelines across 5 springs
- GPU compute on sovereign hardware
- AI-accelerated pipeline coordination
- Self-verifying artifact packaging (pseudoSpore pattern)
- Data management and provenance ({{ entity(name="nestgate") }}, {{ entity(name="rhizocrypt") }}, {{ entity(name="sweetgrass") }})
- Zero cost to university — all AGPL-3.0, all sovereign

---

## What This Proves About gen5

### Multi-product composition is necessary

NF cannot be served by {{ entity(name="helixvision") }} alone. The biological question spans genomics (helixVision), pharmacology ({{ entity(name="healthspring") }}), structural biology (coralForge), and dynamics ({{ entity(name="initiochem") }}). gen5 science demands products that compose with each other, not just products that compose primals.

### External data ingestion changes the game

gen3/gen4 reproduced published papers. gen5 ingests data the ecosystem has never seen — NF Data Portal datasets, CTF curated collections, unpublished screening results. The springs must handle data they were not trained on.

### The collaborator owns the science

The pseudoSpore produced for NF is the collaborator's preliminary data. It goes into her CTF NDU application. She is the PI. The ecosystem provided the computation; she provides the science and the authority. This is the gen5 success metric: the science belongs to the collaborator.

### The feedback loop creates new validation targets

NF results become new spring validation checks:
- NF gene expression analysis -> new {{ entity(name="wetspring") }} validation targets
- NF drug repurposing scores -> new {{ entity(name="healthspring") }} validation targets
- NF structural variants -> new {{ entity(name="neuralspring") }} validation targets
- NF conformational dynamics -> new {{ entity(name="hotspring") }} validation targets

The ecosystem gains validation targets it could never have generated internally.

---

## Connection to Existing Validated Work

The NF project builds on 782/782 checks already passing against the collaborator's published work:

| Existing Validation | NF Extension |
|---------------------|--------------|
| {{ entity(name="wetspring") }} Exp 273-286: G1-G6 reproduced (359/359) | Same pipeline, NF gene expression datasets |
| {{ entity(name="neuralspring") }} nS-601-605: modeling + MATRIX (329/329) | MATRIX scoring extended to NF drug targets |
| {{ entity(name="airspring") }}: CytokineBrain cross-species (94/94) | Cytokine signaling in NF microenvironment |
| {{ entity(name="healthspring") }}: PK/PD, population PK (233/233) | Population PK for NF drug candidates |
| {{ entity(name="hotspring") }} Exp 220: CAZyme FEL (190/190) | FEL pipeline applied to MEK inhibitor dynamics |

Total existing foundation: **782/782** (published data) + **190/190** (FEL pipeline) + **6,656+** (genomics pipeline) + **500+** (MD pipeline) = a validated computational base spanning every domain the NF project touches.

---

*The NF project is a collaboration proposal and a proof of concept for the gen5 operating model — where the ecosystem's validated computational base serves an external scientist's question. The spring validation foundation exists (782/782 checks across relevant domains). The NF-specific deliverables (pseudoSpore, NF Data Portal ingestion, tideGlass GPS rebuild) are targets, not yet produced. The gen5 model is being demonstrated, not demonstrated.*
