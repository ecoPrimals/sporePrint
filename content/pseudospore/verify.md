+++
title = "Verify a pseudoSpore"
description = "Step-by-step: download a pseudoSpore, check every hash, verify every signature. Zero trust required."
date = 2026-08-01
weight = 10

[extra]
maturity = "live"
validated_on_hardware = true
+++

> **Validated on live hardware** — westGate (ZFS raidz1) and ironGate (Windows).
> Every verification step below has been run against real ingested data.

This page walks through verifying a pseudoSpore archive from download to
cryptographic proof. The goal: you trust nothing we say, and prove everything
yourself.

---

## Prerequisites

You need:

- **b3sum** — BLAKE3 CLI hasher ([install](https://github.com/BLAKE3-team/BLAKE3))
- **A terminal** — bash, PowerShell, or any shell
- **Optional**: `jq` for reading JSON provenance files

No ecoPrimals software is required for Level 1 verification. The hashes
are standard BLAKE3 — any implementation will produce the same result.

---

## Step 1: Download

```bash
# Example: grab the LTEE REL606 pseudoSpore (smallest — 5.8 MB)
curl -LO https://primals.eco/pseudospore/ltee-rel606.tar.gz
tar xzf ltee-rel606.tar.gz
cd pseudospore-ltee-rel606/
```

---

## Step 2: Check Data Integrity (BLAKE3)

```bash
# Verify every data file against recorded hashes
b3sum --check provenance/blake3_checksums.txt
```

If every line prints `OK`, the data files are exactly what was ingested.
BLAKE3 is a cryptographic hash — any modification, however small, produces
a completely different hash.

---

## Step 3: Verify CAS Identity

```bash
# Each file's BLAKE3 hash IS its CAS (Content-Addressed Storage) identity
# The cas_manifest.json maps filenames to their nestGate object IDs
cat provenance/cas_manifest.json | jq '.objects[] | {name, blake3, cas_id}'
```

The CAS ID is derived from the BLAKE3 hash. If the hash matches (Step 2),
the CAS identity is proven. Content-addressed means the name doesn't matter —
the content IS the identity.

---

## Step 4: Verify DAG Lineage

```bash
# The DAG proof shows parent→child relationships
cat provenance/dag_proof.json | jq '.sessions[] | {session_id, parent, merkle_root}'
```

Each DAG session records which objects were processed together and seals
them with a Merkle root. The Merkle root is independently computable
from the object hashes.

---

## Step 5: Verify Ledger Entry

```bash
# The spine entry is the permanent record
cat provenance/spine_entry.json | jq '{entry_id, timestamp, merkle_root, storage_backend}'
```

loamSpine is an append-only ledger. Each entry references the DAG session's
Merkle root. The entry proves the data existed at the recorded timestamp
on the recorded storage backend.

---

## Step 6: Verify Ed25519 Signature

```bash
# The signature covers the full chain
cat provenance/ed25519_signature.json | jq '{signer, public_key, signed_hash, timestamp}'
```

bearDog signs the chain with Ed25519. The signed hash covers the spine
entry's Merkle root. If you have the public key (published at primals.eco),
you can verify the signature independently.

---

## Step 7: Check Attribution

```bash
# W3C PROV-O compliant attribution braid
cat provenance/attribution_braid.json | jq '{agent, activity, entity, generated_at}'
```

sweetGrass records who produced the data, what activity generated it,
and when. The attribution follows W3C PROV-O (Provenance Ontology) so
any PROV-compliant tool can parse it.

---

## The One-Step Version

```bash
# If you just want PASS/FAIL:
./validate.sh
```

`validate.sh` runs Steps 2–6 automatically and reports PASS or FAIL
for each stage. It uses only standard tools (b3sum, openssl/ed25519,
jq) — no ecoPrimals binaries required.

---

## What This Proves

If all steps pass, you have proven:

1. **The data is unmodified** (BLAKE3 hash match)
2. **The identity is content-derived** (CAS = hash, not filename)
3. **The lineage is recorded** (DAG parent→child chain)
4. **The record is permanent** (ledger entry with timestamp)
5. **The chain is signed** (Ed25519 cryptographic witness)
6. **The attribution is semantic** (W3C PROV-O compliant)

No trust in ecoPrimals is required. No trust in the server is required.
No trust in the network is required. The proof travels with the data.

---

## Reproduce From Scratch

For the strongest verification: deploy NUCLEUS on your own hardware,
ingest the same public dataset from the same public source, and compare
your provenance chain against ours.

The BLAKE3 hashes will match (deterministic hashing of identical data).
The CAS IDs will match (derived from hashes). The DAG structure will
match (same ingestion pipeline). Your Ed25519 signature will be different
(your key, not ours) — but the data it signs will be identical.

See [Getting Started](@/getting-started/_index.md) for deployment instructions.
