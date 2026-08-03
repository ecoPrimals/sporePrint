+++
title = "Analytical Chemistry — Data Braids"
description = "MassBank NIST reference spectra (63 MB) with full sweetGrass provenance braid. PFAS detection baseline."
date = 2026-08-02
weight = 70

[taxonomies]
springs = ["wetspring"]

[extra]
maturity = "live"
domain = "analytical-chemistry"
+++

Mass spectrometry reference data for environmental chemistry
and metabolomics compound identification.

---

## MassBank NIST reference spectra {#massbank-nist}

| Field | Value |
|-------|-------|
| **Size** | 63 MB |
| **Files** | 1 |
| **Source** | [MassBank Consortium](https://massbank.eu/MassBank/) |
| **License** | CC-BY-4.0 |
| **Ingested** | July 29, 2026 |
| **Provenance** | 5/5 FULL |
| **Springs** | wetSpring |

NIST-validated mass spectrometry reference spectra. Standard compounds
for metabolomics and environmental chemistry, including PFAS
(per- and polyfluoroalkyl substances) detection reference compounds.

### What's Possible

- Spectral matching for **PFAS detection** in environmental samples
  (**wetSpring** environmental chemistry pipeline)
- Cross-reference with **PubChem** for compound identification
  from unknown spectra
- Feed sovereign environmental monitoring workflows

### The Braid

sweetGrass `braid.create` produced a W3C PROV-O JSON-LD attestation:

```json
{
  "@context": "https://www.w3.org/ns/prov#",
  "@id": "urn:braid:massbank-nist-westgate-20260729",
  "prov:wasGeneratedBy": {
    "@type": "prov:Activity",
    "prov:used": "https://massbank.eu/MassBank/",
    "prov:wasAssociatedWith": "did:eco:westgate"
  },
  "prov:wasAttributedTo": "did:eco:westgate",
  "prov:generatedAtTime": "2026-07-29T...",
  "eco:license": "CC-BY-4.0",
  "eco:blake3_root": "...",
  "eco:file_count": 1,
  "eco:size_bytes": 66060288
}
```

---

## See Also

- [Drug Discovery](/data/drug-discovery/) — PubChem (compound cross-reference)
- [Environmental](/data/environmental/) — NOAA, USGS (environmental context)
