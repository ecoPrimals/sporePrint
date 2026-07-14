+++
title = "Jones — PFAS Analytical Chemistry & blueFish"
description = "Emeritus domain expert providing the product specification and validation rubric for blueFish through decades of analytical chemistry expertise."
date = 2026-05-28

[taxonomies]
primals = ["barracuda", "loamspine", "toadstool"]
springs = ["wetspring"]

[extra]
status = "active"

[[extra.companions]]
url = "/architecture/discovery-log/"
title = "Discovery Log"
relation = "evidence_for"
label = "Domain expertise revealing EPA method gaps"

[[extra.companions]]
url = "/methodology/conversation-constraint/"
title = "Conversation Constraint"
relation = "methodology"
label = "How emeritus knowledge enters the system through conversation"
+++

**Status**: Active — emeritus consulting confirmed, technical resources received
**Domain**: PFAS analytical chemistry, mass spectrometry, environmental monitoring
**Product**: {{ entity(name="bluefish") }} (sovereign PFAS analytical chemistry ETL)
**Springs Fed**: {{ entity(name="wetspring") }} Track 2

---

## The Collaboration

This is not a typical gen5 collaboration (PI + grant). It is an emeritus domain expert providing the product specification and validation rubric for {{ entity(name="bluefish") }} through accumulated institutional knowledge — 30+ years of analytical chemistry domain expertise.

---

## What Domain Expertise Revealed

### EPA Method 1633A Gap

Most labs (including commercial) do not actually perform all QC checks — particularly **ion abundance ratios**. No published data exists to evaluate QC compliance beyond single-letter qualifiers in lab reports. {{ entity(name="bluefish") }} implements full 1633A QC including ion abundance ratios — the gap commercial labs are not filling.

### Open-Source Tool Assessment

| Tool | Assessment |
|------|-----------|
| MZmine | Unacceptable failure rates |
| MS-DIAL | Unacceptable failure rates |
| XCMS | Unacceptable failure rates |
| Progenesis QI | Good graphical evaluation, **now obsolete** |

No open-source tool gives acceptable failure rates AND graphical evaluation of feature detection / measurement quality. This is {{ entity(name="bluefish") }}'s target.

### MRM Architecture

Most PFAS analyses are targeted MRM (Multiple Reaction Monitoring). Single sample produces ~100 chromatograms (2 per analyte + internal standards). Current lab workflow uses manual spreadsheet consolidation. {{ entity(name="bluefish") }} implements template-driven MRM batch processing with automated peak integration.

---

## {{ entity(name="bluefish") }} Product Specification (from Domain Data)

| Feature | Source | Evidence |
|---------|--------|----------|
| Full EPA 1633A QC (including ion abundance ratios) | Domain expertise | "Most labs don't actually perform all QC checks" |
| LOQ boundary handling | Domain expertise | State labs report below-LOQ as equal to LOQ |
| Visual QC for feature detection | Domain expertise | Progenesis QI was good at this but is now dead |
| Low failure rate peak detection | Domain expertise | MZmine, MS-DIAL, XCMS all have unacceptable failure rates |
| Sovereign mzML ingestion | Domain expertise | Chromeleon to msConvert to mzML is the standard path |
| NIST reference validation | Domain expertise | Untargeted PFAS reference spectra as validation baseline |
| MANA SODA benchmarking | Domain expertise | Cross-validation consortium datasets for quality review |
| Template-driven MRM batch processing | Domain expertise | ~100 chromatograms/sample, tedious manual setup |

---

## New Spring Validation Targets

| Target | Spring Check |
|--------|-------------|
| EPA 1633A ion abundance ratio QC | Full method compliance checks |
| LOQ boundary validation | Below-LOQ detection and reporting accuracy |
| NIST reference spectra matching | Untargeted PFAS validation suite |
| MANA SODA cross-validation parity | Multi-tool benchmark comparison |
| MRM peak integration accuracy | 100-chromatogram batch processing |

These are validation targets the ecosystem could never have generated internally — they come from decades of analytical chemistry domain expertise.

---

## New Public Data Systems

| System | Purpose | Status |
|--------|---------|--------|
| NIST PFAS Reference Data | Untargeted PFAS validation baseline | Planned |
| EPA Method 1633A | Full QC compliance specification | Planned |
| MANA SODA Benchmark Datasets | Cross-validation consortium | Planned |

---

## The gen5 Pattern

```
Domain expertise (decades of analytical chemistry)
    -> Technical data (NIST, EPA 1633A, MANA SODA, MRM architecture)
    -> blueFish product specification
    -> wetSpring Track 2 new validation targets
    -> blueFish implementation (sovereign PFAS ETL)
    -> Architecture and QC compliance review
    -> blueFish available to any PFAS lab (AGPL-3.0)
    -> Domain knowledge becomes embodied in open tooling
```

---

*Retirement from the university is not retirement from the science. Decades of analytical chemistry expertise — what tools fail, what QC labs skip, where the data quality gaps are — is now the specification for an open tool that outlives any single institution.*
