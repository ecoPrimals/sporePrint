+++
title = "Transplant — Carry the Data With You"
description = "How to take science data and its full provenance chain from ecoPrimals to your own hardware via pseudoSpore or lithoSpore. The spore can't carry the mountain, but it proves the mountain was climbed."
date = 2026-08-03
weight = 195

[extra]
maturity = "live"
+++

Every dataset in the [Data Braids catalog](/data/) can be taken with you.
Not just the files — the full provenance chain that proves where the data
came from, when it was ingested, and that it hasn't been modified.

The spore can't carry the mountain, but it proves the mountain was climbed.

---

## Two Paths

### pseudoSpore — Lightweight Transplant

A pseudoSpore is a downloadable archive carrying data plus its proof chain:

```
pseudospore-chembl37/
├── data/                      # The science data files
│   ├── chembl_37.sdf.gz
│   └── chembl_37_sqlite.tar.gz
├── provenance/                # The proof chain
│   ├── blake3_checksums.txt   # BLAKE3 hashes of every file
│   ├── cas_manifest.json      # nestGate CAS object IDs
│   ├── dag_proof.json         # rhizoCrypt DAG lineage
│   ├── spine_entry.json       # loamSpine ledger entry
│   ├── ed25519_signature.json # bearDog cryptographic witness
│   └── attribution_braid.json # sweetGrass W3C PROV-O attribution
├── validate.sh                # Run this to verify everything
└── README.md                  # What this is, where it came from
```

**What you need**: `b3sum` (BLAKE3 CLI), a shell, optionally `jq`.
No ecoPrimals software required for verification.

**How to verify**:

```bash
tar xzf pseudospore-chembl37.tar.gz
cd pseudospore-chembl37/
./validate.sh
```

The script checks every hash, every CAS identity, every DAG link,
every ledger entry, and every signature. PASS or FAIL — no ambiguity.

### lithoSpore — Full Transplant

A lithoSpore is a self-contained, USB-deployable artifact that carries
everything needed to run the science independently:

```
lithospore-chembl37/
├── data/                      # The science data
├── provenance/                # The proof chain (same as pseudoSpore)
├── runtime/                   # NUCLEUS binaries for your platform
│   ├── linux-x86_64/
│   ├── windows-x86_64/
│   └── linux-aarch64/
├── springs/                   # Validation domain binaries
│   └── healthspring/
├── validate.sh                # Verify data + provenance
├── run.sh                     # Boot NUCLEUS + spring + data
└── README.md
```

**What you need**: A machine. Any machine. USB port optional.

The lithoSpore boots a NUCLEUS composition, loads the data, and runs
the spring validation pipeline. No internet, no cloud, no dependencies
beyond what's in the archive. The science runs on commodity hardware
in a basement, a field station, or an air-gapped lab.

---

## What Travels With the Data

Every pseudoSpore and lithoSpore carries a 5-stage cryptographic
provenance chain:

| Stage | Primal | What It Proves | Verifiable With |
|-------|--------|----------------|-----------------|
| 1. Content hash | nestGate | The files are what they claim to be | `b3sum` |
| 2. CAS identity | nestGate | Identity is the hash, not a filename | `jq` |
| 3. DAG lineage | rhizoCrypt | Where the data came from (parent/child graph) | `jq` |
| 4. Ledger commit | loamSpine | The data existed at this time (append-only) | `jq` |
| 5. Ed25519 witness | sweetGrass | Who committed it and when (cryptographic sig) | `openssl` or any Ed25519 impl |

Each stage is independently checkable. No stage trusts the previous one.
The chain is end-to-end.

---

## What Doesn't Travel

**Gate identity**: When you ingest data into your own NUCLEUS, your gate
signs it with your key, not ours. The provenance chain extends — your
signature attests that you verified our chain and then ingested the data
on your hardware. The data's identity (BLAKE3 hash) stays the same.
The chain of custody grows.

**Our infrastructure**: You don't need our gates, our mesh, or our network.
The provenance is self-contained. The verification tools are standard
(b3sum, jq, any Ed25519 implementation). The science is deterministic.
Same data + same computation = same results, regardless of whose hardware
runs it.

---

## The PI's Workflow

```
1. Browse the Data Braids catalog
   See what's available, how big, which domain, which springs

2. Read the braid
   Each dataset shows its W3C PROV-O JSON-LD attestation inline
   — who ingested it, when, from where, with what license

3. Download a pseudoSpore
   Data + provenance manifest + validate.sh

4. Verify on your hardware
   ./validate.sh → PASS/FAIL for every stage

5. Ingest into your own NUCLEUS (optional)
   Your gate extends the provenance chain with your own signature
   The BLAKE3 hashes match — same data, new custodian

6. Run science
   Springs operate on the data — hotSpring for physics,
   wetSpring for biology, tideGlass for pharmacology

7. Publish results as your own pseudoSpore
   Your computation + your provenance + your signature
   The chain links back to the original data source
```

---

## For Grant Applications

If you are writing a grant application that references ecoPrimals data:

- **Data availability**: All datasets are publicly downloadable as
  pseudoSpore archives from [primals.eco/pseudospore/](/pseudospore/)
- **Provenance**: Every file carries a BLAKE3 hash, CAS identity, DAG
  lineage, append-only ledger entry, and Ed25519 signature
- **Reproducibility**: `./validate.sh` independently verifies the
  entire chain with no ecoPrimals software required
- **License**: Individual dataset licenses are listed in each braid
  (CC0, CC-BY, Public Domain — see the [catalog](/data/))
- **Hardware requirements**: Any x86_64 or ARM64 machine with 8+ GB RAM
- **Software requirements**: `b3sum` for Level 1 verification,
  a lithoSpore for complete offline reproduction

The data and its proof chain are designed to survive your hardware,
your grad students, and your funding cycles. The hashes don't expire.

---

## See Also

- [Data Braids Catalog](/data/) — all available datasets
- [Verify a pseudoSpore](/pseudospore/verify/) — step-by-step verification guide
- [pseudoSpore Catalog](/pseudospore/) — downloadable archives
- [lithoSpore](/products/lithoSpore/) — USB-deployable self-verifying artifacts
- [How Braids Work](/data/provenance/) — the 7-stage provenance pipeline
