+++
title = "What's Possible — Dataset Combinations"
description = "Data braid A + data braid B + spring C = science D. The projectFOUNDATION thread lineage rendered as a research planning surface."
date = 2026-08-02
weight = 190

[extra]
maturity = "live"
+++

The real power of a local data federation isn't any single dataset.
It's the **combinations**. Every dataset in the [catalog](/data/) is
available at 10G LAN speed to every spring and garden on the mesh.
No download. No egress charge. No API rate limit.

This page shows what science becomes possible when you combine datasets
that are already local, already braided, already verified.

Think of it as a library card catalog that also tells you which
experiments are ready to run.

---

## Drug Repurposing Pipeline

**Datasets**: [ChEMBL 37](/data/drug-discovery/#chembl-37) +
[LINCS L1000](/data/gene-expression/#lincs-l1000) +
[GTEx V8](/data/gene-expression/#gtex-v8) +
[PubChem](/data/drug-discovery/#pubchem)

**Springs**: healthSpring | **Gardens**: tideGlass

**The science**: ChEMBL maps 2.9M compounds to biological targets. LINCS
records how 473K drug perturbations change gene expression across 77 cell
lines. GTEx shows where genes are normally expressed across 54 tissues.
PubChem resolves chemical identifiers across databases.

**What you can do**: Computational drug repurposing without wet lab access.
Given a disease gene signature, find compounds in ChEMBL that reverse it
(LINCS), predict tissue-specific effects (GTEx), and resolve the compound
identity (PubChem). This is the **gen5 critical path** for tideGlass.

**Data status**: All four datasets braided. 48 GB total. Ready for tideGlass.

---

## Structure-Function Triangulation

**Datasets**: [PDB mmCIF](/data/structural-biology/#pdb-mmcif) +
[UniProt Swiss-Prot](/data/structural-biology/#uniprot-swissprot) +
[UniRef90](/data/proteomics/#uniref90)

**Springs**: neuralSpring, hotSpring

**The science**: PDB provides 257K experimentally determined 3D structures.
UniProt provides 570K curated functional annotations. UniRef90 provides
clustered sequences for evolutionary covariance analysis.

**What you can do**: Complete protein characterization from sequence to
structure to function. Build MSAs from UniRef90, predict structures via
neuralSpring, validate against PDB experimental data, and annotate with
UniProt function. The full bioinformatics stack on sovereign hardware.

**Data status**: All three datasets braided. 119 GB total. Ready for
neuralSpring structure prediction and hotSpring molecular dynamics.

---

## Environmental Genomics + Climate

**Datasets**: [LTEE REL606](/data/microbial-evolution/#ltee-rel606) +
[SILVA 138.1](/data/microbial-evolution/#silva-138) +
[NOAA GHCND](/data/environmental/#noaa-ghcnd) +
[USGS earthquake](/data/environmental/#usgs-earthquake)

**Springs**: wetSpring, groundSpring, airSpring

**The science**: LTEE tracks 75K+ generations of *E. coli* evolution.
SILVA classifies microbial communities via 16S amplicon sequencing.
NOAA provides daily weather for 100K+ stations since the 1700s.
USGS logs global seismic events with magnitude and focal mechanism.

**What you can do**: Multi-domain environmental analysis. Correlate
microbial evolution with environmental forcing. Track how climate
variation affects microbial community composition. Overlay seismic events
with weather patterns for multi-hazard models. All computed locally,
all provenance-tracked.

**Data status**: All four datasets braided. 3.7 GB total. Ready for
wetSpring DADA2 and groundSpring geospatial analysis.

---

## Nuclear Physics on Consumer GPU

**Datasets**: [AME2020](/data/nuclear-physics/#ame2020) +
[PDB structures](/data/proteomics/#pdb-structures)

**Springs**: hotSpring

**The science**: AME2020 provides experimentally measured masses for 3,500+
nuclides — the international reference for nuclear binding energy. hotSpring
computes lattice QCD on consumer GPUs (RTX 3090, RX 6950 XT) using
DF64 precision WGSL shaders.

**What you can do**: Validate GPU-computed nuclear binding energies against
experimental AME2020 data. This is the data braid (AME2020) + NFT
(hotSpring QCD trajectories) convergence — external reference data meets
computed results, both with full provenance.

**Data status**: AME2020 braided. hotSpring QCD trajectories computed.
See [hotSpring QCD pseudoSpore](/pseudospore/hotspring-qcd-sun/) for the
computed results.

---

## Precision Agriculture

**Datasets**: [USDA NASS Census 2017](/data/agriculture/#usda-nass) +
[NOAA GHCND](/data/environmental/#noaa-ghcnd)

**Springs**: airSpring, groundSpring

**The science**: USDA Census provides farm counts, acreage, production,
and economics for every US county. NOAA GHCND provides historical daily
weather for stations near those counties.

**What you can do**: Yield prediction models correlating agricultural
production with historical climate. Sovereign precision agriculture on
local compute — no cloud subscription, no per-query API fees, no vendor
dependency. The data covers decades of agricultural output and centuries
of weather.

**Data status**: Both datasets braided. 3.6 GB total. Ready for airSpring.

---

## PFAS Environmental Detection

**Datasets**: [MassBank NIST](/data/analytical-chemistry/#massbank-nist) +
[PubChem](/data/drug-discovery/#pubchem)

**Springs**: wetSpring

**The science**: MassBank provides NIST-validated reference mass spectra
for standard compounds, including PFAS reference standards. PubChem
provides exact masses and chemical identifiers for cross-referencing.

**What you can do**: Match unknown spectra from environmental samples
against verified reference spectra. Identify PFAS contamination using
spectral fingerprinting without commercial software licenses. The
sovereign alternative to vendor-locked analytical chemistry platforms.

**Data status**: Both datasets braided. 11 GB total. Ready for wetSpring
environmental chemistry.

---

## What's Not Here Yet

The [Data Federation Schedule](https://git.primals.eco/ecoPrimals/wateringHole)
tracks additional datasets being ingested:

| Dataset | Size | Domain | Status |
|---------|------|--------|--------|
| UniProt TrEMBL | ~147 GB | Proteomics | In progress |
| PDB70 HHsearch | ~27 GB | Proteomics | In progress |
| GEO SOFT (curated) | ~50 GB | Gene expression | Queued |
| TCGA | ~200 GB | Cancer genomics | Planned (Batch 4) |
| AlphaFold DB v4 | ~23 TB | Structure prediction | Planned (Batch 5) |

Each will get a sweetGrass braid and appear in this catalog when ingestion
completes. The ZFS pool on westGate has 50.7 TB available — even at full
Batch 5 capacity, storage is not a constraint.

---

## The Pattern

Every combination follows the same pattern:

```
Data Braid A  (proof of provenance)
    +
Data Braid B  (proof of provenance)
    +
Spring C      (computation engine)
    =
Science D     (Novel Fermentation Transcript — NFT)
```

The braids prove the inputs are genuine. The NFT proves the computation
was done correctly. Together: end-to-end verifiable science from public
data to published result, running on commodity hardware in a basement.

---

## See Also

- [Data Braids Index](/data/) — browse all braided datasets
- [How Braids Work](/data/provenance/) — the provenance pipeline
- [pseudoSpore Catalog](/pseudospore/) — computed results (NFTs)
