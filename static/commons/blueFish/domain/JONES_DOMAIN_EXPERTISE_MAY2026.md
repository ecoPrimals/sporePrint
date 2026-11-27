# Domain Expertise: A. Daniel Jones — PFAS Analytical Chemistry

**Source**: Email exchange, May 27-28, 2026
**Contact**: A. Daniel Jones, Professor Emeritus, MSU Biochemistry
**Status**: Retired, willing to consult informally on blueFish
**Relevance**: 30+ year expert in analytical MS, PFAS detection, metabolomics

---

## Critical Gaps Jones Identified (blueFish Targets)

### Gap 1: Ion Abundance Ratio QC (Most Labs Don't Do It)

EPA Method 1633A calls for ion abundance ratio checks. Jones states:

> "I'm not convinced that most labs (including commercial labs) actually
> perform all of the quality control checks (particularly ion abundance
> ratios) called for in this method, and I know of no published data that
> would allow for evaluation of how well QC has been performed."

**Implication for blueFish:** Implement full 1633A ion abundance ratio
verification. This is the single biggest automated QC gap — labs skip it
because it's tedious, and no tool enforces it. blueFish does.

**Validation target:** Parse EPA 1633A PDF for exact ratio tolerances,
implement as pass/fail checks on each analyte transition.

### Gap 2: LOQ Reporting Misrepresentation

> "Some state labs report any measurements below the limit of quantification
> (LOQ) as being the value of the LOQ (rather than < LOQ) because they
> cannot verify that measurements are lower."

Jones personally corrected Grayling, MI water reports — contamination
not as bad as reported.

**Implication for blueFish:** Implement proper LOQ handling:
- Never report sub-LOQ values as equal to LOQ
- Flag `< LOQ` with confidence intervals where possible
- Detect this reporting error in ingested external lab reports
- Provide calibration curve verification for LOQ determination

### Gap 3: Open-Source Tools Have Unacceptable Failure Rates

Jones assessed all major open-source MS processing tools:

| Tool | URL | Jones Assessment |
|------|-----|-----------------|
| MZmine | github.com/mzmine | "Failure rates I have found unacceptable" |
| MS-DIAL | zenodo.org/records/12589462 | Same |
| XCMS | sneumann.github.io/xcms/ | Same |
| Progenesis QI | Nonlinear Dynamics | Good graphical eval, **now obsolete** |

**Implication for blueFish:** The competitive landscape is:
- Open source: exists but unreliable
- Good proprietary: dead (Progenesis QI obsolete)
- Active proprietary: vendor-locked (Chromeleon, MassHunter)

blueFish fills the gap: **low failure rate + graphical QC + sovereign + validated**.

### Gap 4: No Graphical Feature Detection Evaluation in Open Tools

> "They do not give the user graphical options to evaluate how well the
> software is detecting and measuring features in the data"

**Implication for blueFish:** Visual QC is mandatory, not optional.
petalTongue integration for chromatogram/spectrum display with overlay
of detected features, integration boundaries, and QC pass/fail markers.

---

## Technical Architecture Insights from Jones

### Targeted PFAS Analysis (MRM) — The Primary Workflow

From Jones's second reply:

> "Most PFAS analyses are targeted (multiple reaction monitoring, or MRM)
> analyses, so analysis of a single sample involves generation of about
> 100 different chromatograms (two each for most analytes, plus one each
> for the extracted and non-extracted internal standards). Each of the
> analyte chromatograms will have one (sometimes more) peaks integrated
> and reported."

**Architecture implications:**

| Property | Value |
|----------|-------|
| Chromatograms per sample | ~100 |
| Per analyte | 2 chromatograms (quantifier + qualifier transition) |
| Internal standards | 1 chromatogram each (extracted + non-extracted) |
| Peak integration | 1+ peaks per chromatogram |
| Output | Grand consolidated spreadsheet (current: Ehab exports manually) |
| Setup effort | "Quite tedious compared to untargeted analyses" |

**blueFish design decisions:**
1. Template-driven MRM method setup (define transitions once, apply to all samples)
2. Batch processing of 100 chromatograms per sample (parallelizable)
3. Automated peak integration with visual QC overlay
4. Consolidated reporting with per-analyte pass/fail (ion ratio, RT, LOQ)
5. Export to lab-standard spreadsheet format + sovereign provenance

### Untargeted Analysis — Secondary Workflow

Jones pointed to MANA SODA for untargeted work:

> "You might do well to connect with the MANA SODA group (focused on
> metabolomics; info at https://sites.google.com/metabolomicsna.org/soda/home),
> as they provide an assortment of resources including datasets and
> software that you may find useful."

**Why this matters:**
- MANA SODA provides common datasets for cross-tool comparison
- "Encourage people to apply different processing software on common
  datasets to facilitate [data quality review]"
- blueFish can validate against these benchmark datasets

### File Format Chain

```
Instrument (Thermo/Agilent/Waters/Shimadzu)
  → Vendor proprietary format (RAW, .d, etc.)
    → Chromeleon Console export (or msConvert)
      → .mzML or .CDF (open format)
        → blueFish ingestion (Rust parser, wetSpring Track 2)
          → Peak detection + integration + QC
            → Validated report
```

**Key converter:** Proteowizard msConvert (https://proteowizard.sourceforge.io/)
- Standard tool for vendor → open format conversion
- blueFish does NOT need to parse proprietary formats directly
- Focus: excellent mzML/CDF processing downstream of conversion

---

## Reference Materials for Validation Library

### From Jones (Priority 1 — He Specifically Recommended These)

| Resource | URL | blueFish Use |
|----------|-----|-------------|
| NIST PFAS Reference Data | https://www.nist.gov/programs-projects/and-polyfluoroalkyl-substances-pfas/research/reference-data-and-tools/ | Untargeted PFAS validation baseline |
| EPA Method 1633A | https://www.epa.gov/system/files/documents/2024-12/method-1633a-december-5-2024-508-compliant.pdf | Targeted analysis gold standard (40 PFAS, full QC spec) |
| Proteowizard msConvert | https://proteowizard.sourceforge.io/ | Reference converter (validate our mzML parsing against their output) |

### From Jones (Priority 2 — Cross-Validation Sources)

| Resource | URL | blueFish Use |
|----------|-----|-------------|
| MANA SODA | https://sites.google.com/metabolomicsna.org/soda/home | Common datasets for tool comparison/validation |
| MZmine | https://github.com/mzmine | Competitor analysis — test same data, compare results |
| MS-DIAL | https://zenodo.org/records/12589462 | Competitor analysis |
| XCMS | https://sneumann.github.io/xcms/ | Competitor analysis |

---

## blueFish Product Requirements (Derived from Jones Expertise)

### Must-Have (Jones-validated gaps)

1. **EPA 1633A full compliance** — implement all QC checks including ion abundance ratios
2. **Visual peak integration QC** — user can see detected peaks, integration boundaries, pass/fail overlays
3. **Proper LOQ handling** — never report `< LOQ` as `= LOQ`
4. **Low failure rate peak detection** — beat MZmine/MS-DIAL/XCMS on reliability
5. **MRM batch processing** — handle 100 chromatograms per sample efficiently
6. **Consolidated reporting** — spreadsheet export matching current lab workflows

### Should-Have

7. **MANA SODA benchmark results** — validate against common datasets
8. **NIST reference spectra matching** — for untargeted PFAS identification
9. **Cross-tool comparison mode** — run same data through blueFish + MZmine, show differences
10. **Template-driven method setup** — define MRM transitions once, apply to batches

### Nice-to-Have (Future)

11. **Chromeleon method import** — parse Chromeleon processing methods, reproduce in blueFish
12. **Real-time instrument integration** — bypass Chromeleon entirely (long-term)
13. **Multi-lab QC dashboard** — compare QC compliance across reporting labs

---

## Relationship Model

```
Dan Jones (Emeritus, remote CA)
  → Occasional consulting: does this QC implementation match practice?
  → Domain validation: is blueFish catching what labs miss?
  → No data access (retired, no lab)
  → Best use: architecture review, requirement validation, gap identification

Dr. Aswin Kumar Ilango (New lab manager, on-campus)
  → Potential data source (active lab)
  → Not yet contacted
  → Next step: introduction via Dan or Cheryl

Sima Kumar (RA III, Murphy Lab)
  → Peer contact, on-campus
  → Potential test user for blueFish beta

Cheryl Murphy (PI)
  → Decision-maker for lab collaboration
  → CC'd on all emails
```

---

## Next Actions for blueFish

1. **Download EPA 1633A PDF** — parse for exact QC parameters (ion abundance ratio tolerances, RT windows, calibration requirements)
2. **Scrape NIST PFAS reference data** — ingest as validation baseline spectra
3. **Contact MANA SODA** — request access to benchmark datasets
4. **Build MRM pipeline architecture** — 100 chromatograms/sample, template-driven, batch
5. **Implement ion abundance ratio check** — the single biggest gap Jones identified
6. **Build visual QC overlay** — petalTongue integration for chromatogram + peak display
7. **Reply to Jones** — accept consulting relationship, add personal note
8. **Contact Aswin Kumar Ilango** — introduce self, explore data access
