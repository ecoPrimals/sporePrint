+++
title = "Agriculture — Data Braids"
description = "USDA NASS Census 2017 (132 MB) with full sweetGrass provenance braid. Ground truth for precision agriculture."
date = 2026-08-02
weight = 100

[taxonomies]
springs = ["airspring"]

[extra]
maturity = "live"
domain = "agriculture"
+++

Agricultural census data for yield prediction, economic modeling,
and precision agriculture development.

---

## USDA NASS Census 2017 {#usda-nass}

| Field | Value |
|-------|-------|
| **Size** | 132 MB |
| **Files** | 1 |
| **Source** | [USDA NASS](https://www.nass.usda.gov/AgCensus/) |
| **License** | Public Domain |
| **Ingested** | July 29, 2026 |
| **Provenance** | 5/5 FULL |
| **Springs** | airSpring |

Complete Census of Agriculture 2017. Farm counts, acreage, production
volumes, economics, and demographics for every US county. The most
comprehensive agricultural dataset available — conducted every 5 years
by the USDA National Agricultural Statistics Service.

### What's Possible

- Correlate agricultural production with weather patterns (**NOAA GHCND**)
- Feed **airSpring** precision agriculture models with county-level ground truth
- Economic modeling of farm operations across regions and crop types
- Combine with satellite data (future AmeriFlux/ERA5) for yield prediction

### The Braid

sweetGrass `braid.create` produced a W3C PROV-O JSON-LD attestation:

```json
{
  "@context": "https://www.w3.org/ns/prov#",
  "@id": "urn:braid:usda-nass-westgate-20260729",
  "prov:wasGeneratedBy": {
    "@type": "prov:Activity",
    "prov:used": "https://www.nass.usda.gov/AgCensus/",
    "prov:wasAssociatedWith": "did:eco:westgate"
  },
  "prov:wasAttributedTo": "did:eco:westgate",
  "prov:generatedAtTime": "2026-07-29T...",
  "eco:license": "Public Domain",
  "eco:blake3_root": "...",
  "eco:file_count": 1,
  "eco:size_bytes": 138412032
}
```

---

## See Also

- [Environmental](/data/environmental/) — NOAA GHCND (weather correlation)
- [What's Possible](/data/possible/) — precision agriculture combination
