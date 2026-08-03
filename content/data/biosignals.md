+++
title = "Biosignals — Data Braids"
description = "PhysioNet MIT-BIH arrhythmia database (22 MB) with full sweetGrass provenance braid."
date = 2026-08-02
weight = 80

[taxonomies]
springs = ["healthspring"]

[extra]
maturity = "live"
domain = "biosignals"
+++

Cardiac waveform data for arrhythmia detection algorithm development
and sovereign health monitoring validation.

---

## PhysioNet MIT-BIH {#physionet-mitbih}

| Field | Value |
|-------|-------|
| **Size** | 22 MB |
| **Files** | 1 |
| **Source** | [PhysioNet](https://physionet.org/content/mitdb/) |
| **License** | ODbl-1.0 |
| **Ingested** | July 29, 2026 |
| **Provenance** | 5/5 FULL |
| **Springs** | healthSpring |

MIT-BIH Arrhythmia Database. 48 half-hour two-channel ambulatory ECG
recordings from 47 subjects studied by the BIH Arrhythmia Laboratory.
Gold standard for cardiac arrhythmia detection algorithm development
and benchmarking.

### What's Possible

- Train and validate arrhythmia detection models (**healthSpring**)
- Baseline for sovereign health monitoring on gate hardware
- Combine with **GTEx V8** tissue expression for cardiac gene expression context

### The Braid

sweetGrass `braid.create` produced a W3C PROV-O JSON-LD attestation:

```json
{
  "@context": "https://www.w3.org/ns/prov#",
  "@id": "urn:braid:physionet-mitbih-westgate-20260729",
  "prov:wasGeneratedBy": {
    "@type": "prov:Activity",
    "prov:used": "https://physionet.org/content/mitdb/",
    "prov:wasAssociatedWith": "did:eco:westgate"
  },
  "prov:wasAttributedTo": "did:eco:westgate",
  "prov:generatedAtTime": "2026-07-29T...",
  "eco:license": "ODbl-1.0",
  "eco:blake3_root": "...",
  "eco:file_count": 1,
  "eco:size_bytes": 23068672
}
```

---

## See Also

- [Gene Expression](/data/gene-expression/) — GTEx V8 (tissue context)
- [Drug Discovery](/data/drug-discovery/) — ChEMBL (cardiac drug targets)
