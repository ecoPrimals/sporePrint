+++
title = "Provenance Pipeline"
description = "How every computation becomes cryptographically witnessed — BLAKE3 content hashing, DAG sessions, permanent ledger, ed25519-signed attribution braids."
date = 2026-05-06
weight = 30

[extra]
domain = "Provenance"

[taxonomies]
primals = ["rhizocrypt", "loamspine", "sweetgrass", "beardog", "nestgate"]
+++

## The Provenance Trio

Three primals form the provenance chain. Each handles a different temporal
scope of evidence:

| Primal | Scope | What It Stores |
|--------|-------|---------------|
| {{ entity(name="rhizocrypt") }} | Ephemeral | DAG sessions — directed acyclic graphs of events within a pipeline run. BLAKE3 Merkle trees. Sessions are cheap to create and dehydrate to a single root hash. |
| {{ entity(name="loamspine") }} | Permanent | Append-only ledger — Merkle roots are committed here as permanent entries. Certificate minting for auditable history. |
| {{ entity(name="sweetgrass") }} | Attributed | Ed25519-witnessed braids — PROV-O compliant attribution with DID identity. The cryptographic witness that ties computation to an entity. |

The pipeline flows downward: ephemeral → permanent → attributed. Each layer
adds a stronger guarantee.

---

## Pipeline Phases

The `provenance_pipeline.sh` script wraps workload execution in 9 phases:

**Phase 1 — Health Check**: Verify all 13 primals are alive and responsive.

**Phase 2 — DAG Session**: Create a {{ entity(name="rhizocrypt") }} DAG session.
Every subsequent event (data registration, workload execution, result capture)
becomes a vertex in the DAG.

**Phase 3 — Spine Creation**: Create a {{ entity(name="loamspine") }} spine —
the named ledger that will receive the permanent commit.

**Phase 4 — Data Registration**: Hash all input artifacts with BLAKE3 and
register them as DAG vertices. NCBI FASTQ files, reference genomes, workload
TOML specs — every input is content-addressed before any computation begins.

**Phase 5 — Workload Execution**: Dispatch each workload through
{{ entity(name="toadstool") }}. Record start, completion, exit code, and output
BLAKE3 hash as DAG events.

**Phase 6 — Dehydrate**: Collapse the DAG session into a single Merkle root.
This root is the content hash of all events — data registrations, workload
results, and their dependency relationships.

**Phase 7 — Permanent Commit**: Write the Merkle root to the
{{ entity(name="loamspine") }} ledger. This is append-only — the entry cannot
be modified or deleted.

**Phase 8 — Attribution Braid**: {{ entity(name="sweetgrass") }} creates a
PROV-O compliant braid referencing the Merkle root, attributed to the operator's
DID, and witnessed with an ed25519 signature from {{ entity(name="beardog") }}'s
Tower-tier key hierarchy.

**Phase 9 — Manifest**: Write a human-readable `PROVENANCE_MANIFEST.md` and
machine-readable `braid.json` to the results directory.

---

## What This Proves

The provenance chain answers three questions:

**Integrity**: Did anyone tamper with the data or results?
→ Compare BLAKE3 hashes. Merkle root covers all events. One changed byte
breaks the chain.

**Attribution**: Who ran this computation and when?
→ The braid's DID attribution and ed25519 witness. Verifiable by anyone
with the public key.

**Auditability**: What exactly happened during the pipeline?
→ Query the DAG session for the full event graph. Query the ledger for
the permanent commit history.

---

## Verification

Anyone can verify the chain independently:

```bash
# 1. Re-hash the input data
b3sum SRR7760408_1.fastq.gz
# Compare against the manifest hash

# 2. Query the loamSpine ledger for the spine audit trail
curl -s -X POST http://localhost:9700 \
  -H 'Content-Type: application/json' \
  -d '{"jsonrpc":"2.0","method":"spine.get","params":{"spine_id":"<id>"},"id":1}'

# 3. Verify the sweetGrass braid witness
curl -s -X POST http://localhost:9850/jsonrpc \
  -H 'Content-Type: application/json' \
  -d '{"jsonrpc":"2.0","method":"braid.get","params":{"data_hash":"<merkle_root>"},"id":1}'
```

The braid carries the public key in `did:key` format. The signature is
standard ed25519. Any cryptographic library can verify it.

---

## Evolution

The current pipeline wraps {{ entity(name="toadstool") }} execution with
shell-scripted RPC calls to the trio. The evolution path:

| Phase | What Changes |
|-------|-------------|
| Now | Shell wrapper (`provenance_pipeline.sh`) brackets execution |
| Next | `[provenance]` section in workload TOMLs — {{ entity(name="toadstool") }} calls trio natively |
| Then | Multi-witness: {{ entity(name="beardog") }} co-signs, BTSP certificate chain in braid |
| Later | Cross-gate provenance: workloads on strandGate produce braids committed on ironGate |
