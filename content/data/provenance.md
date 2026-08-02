+++
title = "How Braids Work — The Provenance Pipeline"
description = "How sweetGrass braids prove data provenance: from download through BLAKE3, CAS, DAG, ledger, signature, to W3C PROV-O attribution."
date = 2026-08-02
weight = 200

[taxonomies]
primals = ["nestgate", "rhizocrypt", "loamspine", "beardog", "sweetgrass"]

[extra]
maturity = "live"
+++

Every dataset in the [Data Braids catalog](/data/) passes through the same
pipeline. Seven stages, five primals, one braid. Each stage is
independently verifiable. No stage trusts the previous one.

---

## The Pipeline

```
External source (NCBI, RCSB, EBI, NOAA, USGS, ...)
    ↓ download to westGate ZFS
    ↓
┌──────────────────────────────────────────────────────┐
│ Stage 1: BLAKE3 Hash                                 │
│ Primal: nestGate                                     │
│ Method: content.put                                  │
│ Output: 256-bit content hash                         │
│ Proves: the data IS what it claims to be             │
└──────────────────────────────────────────────────────┘
    ↓
┌──────────────────────────────────────────────────────┐
│ Stage 2: CAS Storage                                 │
│ Primal: nestGate                                     │
│ Method: content.put (content-addressed)              │
│ Output: CAS object ID (derived from hash)            │
│ Proves: identity is the content, not a filename      │
└──────────────────────────────────────────────────────┘
    ↓
┌──────────────────────────────────────────────────────┐
│ Stage 3: DAG Tracking                                │
│ Primal: rhizoCrypt                                   │
│ Methods: dag.session.create, dag.event.append         │
│ Output: DAG vertex with Merkle root                  │
│ Proves: parent/child relationships — where data      │
│         came from and what was processed together     │
└──────────────────────────────────────────────────────┘
    ↓
┌──────────────────────────────────────────────────────┐
│ Stage 4: Ledger Commit                               │
│ Primal: loamSpine                                    │
│ Method: spine.create                                 │
│ Output: Spine ID + genesis hash, Merkle certificate  │
│ Proves: immutable record — the data existed at this  │
│         time on this storage backend                 │
└──────────────────────────────────────────────────────┘
    ↓
┌──────────────────────────────────────────────────────┐
│ Stage 5: Cryptographic Signature                     │
│ Primal: bearDog                                      │
│ Method: crypto.sign_ed25519                          │
│ Output: Ed25519 signature + public key               │
│ Proves: who committed it and when (gate identity)    │
└──────────────────────────────────────────────────────┘
    ↓
┌──────────────────────────────────────────────────────┐
│ Stage 6: Attribution Braid                           │
│ Primal: sweetGrass                                   │
│ Method: braid.create                                 │
│ Output: W3C PROV-O JSON-LD attestation               │
│ Proves: provenance — who ingested it, when, from     │
│         where, under what license                    │
│ URN: urn:braid:<hash>                                │
└──────────────────────────────────────────────────────┘
    ↓
┌──────────────────────────────────────────────────────┐
│ Stage 7: Contribution Record                         │
│ Primal: sweetGrass                                   │
│ Method: contribution.record                          │
│ Output: Contribution ID                              │
│ Proves: attribution chain — all agents involved      │
└──────────────────────────────────────────────────────┘
```

---

## What a Braid Looks Like

sweetGrass `braid.create` produces a W3C PROV-O compliant JSON-LD document:

```json
{
  "@context": "https://www.w3.org/ns/prov#",
  "@id": "urn:braid:a1b2c3d4...",
  "@type": "prov:Entity",
  "prov:wasGeneratedBy": {
    "@type": "prov:Activity",
    "prov:used": "https://ftp.ebi.ac.uk/pub/databases/chembl/...",
    "prov:wasAssociatedWith": "did:eco:westgate",
    "prov:startedAtTime": "2026-07-29T03:14:00Z",
    "prov:endedAtTime": "2026-07-29T03:47:22Z"
  },
  "prov:wasAttributedTo": "did:eco:westgate",
  "prov:generatedAtTime": "2026-07-29T03:47:22Z",
  "eco:data_hash": "b3:...",
  "eco:mime_type": "application/x-sqlite3",
  "eco:size": 16106127360,
  "eco:license": "CC-BY-SA-3.0",
  "eco:source_org": "EMBL-EBI"
}
```

The `@context` makes this parseable by any W3C PROV-O compliant tool.
The `eco:` namespace carries ecoPrimals-specific metadata. The braid URN
(`urn:braid:...`) is the permanent reference for this attestation.

---

## The Five Primals

| Primal | Role | IPC |
|--------|------|-----|
| **nestGate** | Content-addressed storage, BLAKE3 hashing | UDS (Unix domain socket) |
| **rhizoCrypt** | Ephemeral DAG — lineage tracking in present time | UDS |
| **loamSpine** | Immutable ledger — permanence in past time | UDS |
| **bearDog** | Ed25519 signing — cryptographic witness | UDS |
| **sweetGrass** | Attribution braids — W3C PROV-O semantic layer | UDS |

Together they form the **Provenance Trio** (rhizoCrypt + loamSpine + sweetGrass),
plus nestGate (storage) and bearDog (signatures). All communicate via
JSON-RPC 2.0 over Unix domain sockets with riboCipher framing.

---

## Why This Matters

The data library is not a mirror. It's not a cache. It's a
**cryptographically verified, locally queryable federation** where
sweetGrass braids are the access and verification layer.

Any visitor can:

1. **Read the braid** — see exactly what was ingested, when, from where
2. **Check the hash** — run `b3sum` on the data and compare against the braid
3. **Verify the signature** — confirm the Ed25519 witness with the gate's public key
4. **Trace the DAG** — follow parent/child relationships through the lineage
5. **Inspect the ledger** — confirm the spine entry timestamp and Merkle root

No trust in ecoPrimals is required. The proof travels with the data.

---

## Real Pipeline Code

The ingestion pipeline runs on westGate as Python scripts calling
primal RPCs over Unix domain sockets. The core loop:

```python
# 1. Hash and store
b3hash = blake3_hash(filepath)
rpc("nestgate", "content.put", {"data": encoded, "content_type": mime})

# 2. DAG session
rpc("rhizocrypt", "dag.session.create", {"name": dataset_name})
rpc("rhizocrypt", "dag.event.append", {"hash": b3hash, "event_type": "ingest"})

# 3. Ledger commit
rpc("loamspine", "spine.create", {"name": dataset_name, "owner": "westgate"})

# 4. Signature
rpc("beardog", "crypto.sign_ed25519", {"message": base64(b3hash)})

# 5. Braid
rpc("sweetgrass", "braid.create", {
    "data_hash": b3hash,
    "author": "westgate",
    "mime_type": mime,
    "size": filesize
})
```

Every dataset in the [catalog](/data/) went through this exact pipeline.
100% provenance coverage. Zero exceptions.

---

## See Also

- [Data Braids Index](/data/) — browse all braided datasets
- [Verify a pseudoSpore](/pseudospore/verify/) — step-by-step verification
- [Provenance Pipeline](/lab/provenance-pipeline/) — lab notebook with test results
