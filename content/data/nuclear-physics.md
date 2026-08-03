+++
title = "Nuclear Physics — Data Braids"
description = "AME2020 nuclear masses (1.2 MB) with full sweetGrass provenance braid. The reference table for GPU-accelerated binding energy calculations."
date = 2026-08-02
weight = 90

[taxonomies]
springs = ["hotspring"]

[extra]
maturity = "live"
domain = "nuclear-physics"
+++

Experimental nuclear mass data — the ground truth for lattice QCD
and nuclear binding energy calculations on sovereign GPU hardware.

---

## AME2020 nuclear masses {#ame2020}

| Field | Value |
|-------|-------|
| **Size** | 1.2 MB |
| **Files** | 2 |
| **Source** | [IAEA Nuclear Data Services](https://www-nds.iaea.org/amdc/) |
| **License** | Public Domain |
| **Ingested** | July 28, 2026 |
| **Provenance** | 5/5 FULL |
| **Springs** | hotSpring |

Atomic Mass Evaluation 2020. Experimentally measured and predicted
masses for 3,500+ nuclides. The international reference table for
nuclear binding energy calculations, maintained by the IAEA Nuclear
Data Section.

### What's Possible

- Direct input for **hotSpring** nuclear binding energy calculations on GPU
- Validate lattice QCD results against experimental nuclear masses
- Cross-reference with hotSpring QCD trajectories
  (see [pseudoSpore: hotSpring QCD](/pseudospore/hotspring-qcd-su2/))

This dataset + hotSpring + consumer GPU = nuclear physics validated
against experimental data, running in a basement. The data braid proves
the AME2020 table is unmodified; the hotSpring NFT proves the physics
was computed correctly.

### The Braid

sweetGrass `braid.create` produced a W3C PROV-O JSON-LD attestation:

```json
{
  "@context": "https://www.w3.org/ns/prov#",
  "@id": "urn:braid:ame2020-westgate-20260728",
  "prov:wasGeneratedBy": {
    "@type": "prov:Activity",
    "prov:used": "https://www-nds.iaea.org/amdc/",
    "prov:wasAssociatedWith": "did:eco:westgate"
  },
  "prov:wasAttributedTo": "did:eco:westgate",
  "prov:generatedAtTime": "2026-07-28T...",
  "eco:license": "Public Domain",
  "eco:blake3_root": "...",
  "eco:file_count": 2,
  "eco:size_bytes": 1258291
}
```

---

## See Also

- [pseudoSpore: hotSpring QCD](/pseudospore/hotspring-qcd-su2/) — computed lattice QCD (NFT)
- [Structural Biology](/data/structural-biology/) — PDB (protein structure context)
