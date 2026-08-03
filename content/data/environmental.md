+++
title = "Environmental — Data Braids"
description = "NOAA GHCND weather data (3.5 GB) and USGS earthquake catalog (2.1 MB) with full sweetGrass provenance braids."
date = 2026-08-02
weight = 60

[taxonomies]
springs = ["groundspring", "airspring"]

[extra]
maturity = "live"
domain = "environmental"
+++

Weather and seismic data for environmental science, agriculture, and
multi-hazard analysis. Public domain datasets from US federal agencies.

---

## NOAA GHCND {#noaa-ghcnd}

| Field | Value |
|-------|-------|
| **Size** | 3.5 GB |
| **Files** | 3 |
| **Source** | [NOAA](https://www.ncbi.noaa.gov/pub/data/ghcn/daily/) |
| **License** | Public Domain |
| **Ingested** | July 29, 2026 |
| **Provenance** | 5/5 FULL |
| **Springs** | groundSpring, airSpring |

Global Historical Climatology Network daily weather observations.
100K+ stations worldwide. Daily temperature (min/max/avg),
precipitation, snowfall, and snow depth records spanning the 1700s to
present.

### What's Possible

- Correlate weather patterns with agricultural yields (**USDA NASS**)
- Feed **airSpring** atmospheric models with historical climate data
- Combine with **USGS seismic** data for multi-hazard environmental analysis
- Ground truth for precision agriculture yield prediction (**groundSpring**)

### The Braid

sweetGrass `braid.create` produced a W3C PROV-O JSON-LD attestation:

```json
{
  "@context": "https://www.w3.org/ns/prov#",
  "@id": "urn:braid:noaa-ghcnd-westgate-20260729",
  "prov:wasGeneratedBy": {
    "@type": "prov:Activity",
    "prov:used": "https://www.ncbi.noaa.gov/pub/data/ghcn/daily/",
    "prov:wasAssociatedWith": "did:eco:westgate"
  },
  "prov:wasAttributedTo": "did:eco:westgate",
  "prov:generatedAtTime": "2026-07-29T...",
  "eco:license": "Public Domain",
  "eco:blake3_root": "...",
  "eco:file_count": 3,
  "eco:size_bytes": 3758096384
}
```

---

## USGS earthquake catalog (monthly) {#usgs-earthquake}

| Field | Value |
|-------|-------|
| **Size** | 2.1 MB |
| **Files** | 1 |
| **Source** | [USGS FDSN](https://earthquake.usgs.gov/fdsnws/event/1/) |
| **License** | Public Domain |
| **Ingested** | July 29, 2026 |
| **Provenance** | 5/5 FULL |
| **Springs** | groundSpring |

Monthly earthquake catalog from the USGS FDSN web service. Magnitude,
location (lat/lon/depth), and focal mechanism for global seismic events.

### What's Possible

- Seismic pattern analysis for **groundSpring**
- Combine with **NOAA GHCND** weather for multi-hazard correlation studies
- Feed environmental risk models with real-time seismic data

### The Braid

sweetGrass `braid.create` produced a W3C PROV-O JSON-LD attestation:

```json
{
  "@context": "https://www.w3.org/ns/prov#",
  "@id": "urn:braid:usgs-earthquake-westgate-20260729",
  "prov:wasGeneratedBy": {
    "@type": "prov:Activity",
    "prov:used": "https://earthquake.usgs.gov/fdsnws/event/1/",
    "prov:wasAssociatedWith": "did:eco:westgate"
  },
  "prov:wasAttributedTo": "did:eco:westgate",
  "prov:generatedAtTime": "2026-07-29T...",
  "eco:license": "Public Domain",
  "eco:blake3_root": "...",
  "eco:file_count": 1,
  "eco:size_bytes": 2202009
}
```

---

## See Also

- [Agriculture](/data/agriculture/) — USDA NASS (agricultural context for weather data)
- [Microbial Evolution](/data/microbial-evolution/) — LTEE, SILVA (environmental genomics)
