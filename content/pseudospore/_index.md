+++
title = "pseudoSpore — Sovereign Science Data"
description = "38.2 GB of real science data with full cryptographic provenance. Downloadable and verifiable on commodity hardware. No cloud. No trust required."
sort_by = "weight"
template = "section.html"
+++

Real science data. Sovereign hardware. Full provenance. Verify it yourself.

westGate has ingested **38.2 GB** across 6 scientific domains through the complete
CAS + Provenance Trio pipeline. Every object is content-addressed (BLAKE3),
DAG-tracked (rhizoCrypt), ledger-committed (loamSpine), Ed25519-signed (bearDog),
and attribution-braided (sweetGrass). **100% provenance coverage.**

This is not a plan. This data is on disk, hashed, signed, and queryable right now
on a Ryzen 9 / 96 GB DDR5 / 50.7 TB ZFS raidz1 machine running in a basement.

---

## Data Catalog

| Dataset | Size | Objects | Domain | Springs | Provenance |
|---------|------|---------|--------|---------|-----------|
| **LINCS L1000 Level 5** | 19.86 GB | 6 | Gene expression (473K signatures x 12K genes) | wetSpring, tideGlass | 5/5 FULL |
| **ChEMBL 37** | ~15 GB | 2 | Drug discovery (2.9M compounds, 24.5M bioactivities) | healthSpring, tideGlass | 5/5 FULL |
| **PDB** | 361 MB+ | 506+ | Structural biology (protein structures) | hotSpring, neuralSpring | 5/5 FULL |
| **UniProt Swiss-Prot** | 764 MB | 3 | Protein sequences | wetSpring, hotSpring | 5/5 FULL |
| **ZINC20 SMILES** | 160 MB | 110 | Compound screening | healthSpring, tideGlass | 5/5 FULL |
| **GTEx V8** | 2.4 GB | 4 | Tissue expression | wetSpring, healthSpring | 5/5 FULL |
| **SILVA 138.1** | 188 MB | 1 | 16S taxonomy reference | wetSpring | 5/5 FULL |
| **PhysioNet MIT-BIH** | 22 MB | 1 | Biosignals (ECG) | healthSpring | 5/5 FULL |
| **MassBank NIST** | 63 MB | 1 | Mass spectrometry reference | wetSpring | 5/5 FULL |
| **NOAA GHCND** | 11 MB | 2 | Weather station data | groundSpring, airSpring | 5/5 FULL |
| **LTEE REL606** | 5.8 MB | 1 | Microbial evolution reference | wetSpring | 5/5 FULL |
| **Total** | **~38.2 GB** | **4,752** | **6 domains** | **8 springs** | **100%** |

---

## What "5/5 FULL" Means

Every data object passes through five provenance stages. Each is independently verifiable:

| Stage | Primal | What It Proves |
|-------|--------|----------------|
| 1. **Content hash** | nestGate | BLAKE3 hash — the object IS what it claims to be |
| 2. **CAS storage** | nestGate | Content-addressed — identity is the hash, not a filename |
| 3. **DAG tracking** | rhizoCrypt | Parent/child relationships — where data came from |
| 4. **Ledger commit** | loamSpine | Immutable record — the object existed at this time |
| 5. **Ed25519 witness** | sweetGrass | Cryptographic signature — who committed it and when |

The chain is end-to-end. No stage trusts the previous one — each is independently checkable.

---

## What a pseudoSpore Contains

A pseudoSpore is a downloadable archive carrying data + its proof chain:

```
pseudospore-chembl37/
├── data/                      # The science data
│   ├── chembl_37.sdf.gz
│   └── chembl_37_sqlite.tar.gz
├── provenance/                # The proof chain
│   ├── blake3_checksums.txt   # BLAKE3 hashes of every file
│   ├── cas_manifest.json      # nestGate CAS object IDs
│   ├── dag_proof.json         # rhizoCrypt DAG chain
│   ├── spine_entry.json       # loamSpine ledger entry
│   ├── ed25519_signature.json # bearDog signature
│   └── attribution_braid.json # sweetGrass W3C PROV-O attribution
├── validate.sh                # Run this to verify everything
└── README.md                  # What this is, where it came from
```

`./validate.sh` checks every hash, every signature, every chain link.
**You are the verifier.** No trust required. No central authority.
The provenance travels with the data.

---

## Verify It Yourself

Three levels of verification, zero trust required:

### Level 1: Check a hash

```bash
# Download any dataset file
# Run b3sum (BLAKE3 CLI) and compare against the manifest
b3sum chembl_37.sdf.gz
# Compare output against blake3_checksums.txt
```

### Level 2: Verify the signature chain

```bash
# Inside a pseudoSpore archive:
./validate.sh
# Checks: BLAKE3 hashes → CAS IDs → DAG parents → spine entry → Ed25519 sig
# Output: PASS or FAIL for each stage
```

### Level 3: Reproduce on your own hardware

Deploy NUCLEUS on your machine (see [Getting Started](@/getting-started/_index.md)),
ingest the same public datasets, and compare your provenance chain against ours.
The hashes will match. The science is deterministic. The hardware is commodity.

---

## The Hardware

All of this runs on:

- **westGate**: i9-14900K, 96 GB DDR5, 50.7 TB ZFS raidz1
- **Cost**: ~$6K hardware + $485/month operating (electricity, ISP, VPS)
- **OS**: Linux (NixOS)
- **Network**: Sovereign mesh via Tower Atomic (bearDog + songBird + skunkBat)

No cloud. No AWS. No institutional compute allocation. Commodity hardware
you could build from Micro Center parts.

---

## Spore Taxonomy

| Class | Self-sufficient? | What it carries |
|-------|:---:|-----------------|
| **coldSpore** | No | Static marker + frozen data snapshot |
| **liveSpore** | Partial | + Journal + refresh mechanism |
| **pseudoSpore** | No | + Provenance braids, receipts, derivation configs — *proves the mountain was climbed* |
| **lithoSpore** | **Yes** | + Runtime + binaries + full data — carries everything to reproduce independently |

*The spore can't carry the mountain, but it proves the mountain was climbed.*

---

## pseudoSpore #2: Computed Science (hotSpring QCD)

The data catalog above shows **ingested** reference data — public datasets
pulled through the CAS + Provenance pipeline. But the system doesn't just
store science. It **produces** science.

[hotSpring QCD — SU(2) Lattice Gauge Theory](@/pseudospore/hotspring-qcd-su2.md)
is the second pseudoSpore: original lattice QCD trajectories computed on
strandGate using both NVIDIA RTX 3090 and AMD RX 6950 XT. Multi-vendor,
DF64 precision, full provenance. The same WGSL shaders run on both GPUs —
cross-GPU plaquette agreement within 3.1×10⁻⁹.

**arXiv paper**: [full draft open for review](/pseudospore/hotspring-qcd-su2-paper/) |
[computation audit trail](/pseudospore/hotspring-qcd-su2-audit/) |
ORCID [0009-0004-2141-0321](https://orcid.org/0009-0004-2141-0321).

---

## See Also

- [Data Braids](@/data/_index.md) — 172 GB of ingested data with sweetGrass provenance braids
- [What's Possible](@/data/possible.md) — dataset combinations that enable science
- [How Braids Work](@/data/provenance.md) — the provenance pipeline explained
- [Verify a pseudoSpore](@/pseudospore/verify.md) — step-by-step verification guide
- [hotSpring QCD pseudoSpore](@/pseudospore/hotspring-qcd-su2.md) — computed lattice gauge theory
- [lithoSpore](@/products/lithoSpore.md) — USB-deployable self-verifying artifacts
- [pseudoSpore Gallery](@/lab/spores/_index.md) — spring-validated spore archives
- [Provenance Pipeline](@/lab/provenance-pipeline.md) — how the chain works
- [Getting Started](@/getting-started/_index.md) — deploy NUCLEUS on your hardware
