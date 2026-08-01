+++
title = "Provenance Dashboard"
description = "Provenance 7/7 — full signed chain from content hash to witnessed braid. Ed25519 signatures on Linux (ZFS) and Windows."
date = 2026-08-01
weight = 4

[extra]
maturity = "live"
validated_on_hardware = true
+++

> **Validated on live hardware** — westGate (Linux/ZFS) and ironGate (Windows).
> 5th consecutive E2E pass. Provenance 7/7 COMPLETE.

## The 7-Step Chain

Every scientific artifact produced by NUCLEUS carries a cryptographic
provenance chain. Each step is independently verifiable:

| Step | Primal | What It Does | Status |
|------|--------|-------------|--------|
| 1. Content hash | rhizoCrypt | BLAKE3 hash of raw artifact | LIVE |
| 2. DAG insertion | rhizoCrypt | Content-addressed DAG node with parent links | LIVE |
| 3. Session commit | rhizoCrypt | DAG session sealed with Merkle root | LIVE |
| 4. Ledger write | loamSpine | Permanent append-only ledger entry | LIVE |
| 5. Attribution | sweetGrass | DID-based authorship + contribution record | LIVE |
| 6. Witness | sweetGrass | Ed25519 signature on the full chain | LIVE |
| 7. Braid | sweetGrass | PROV-O compliant witnessed braid | LIVE |

## Cross-Platform Validation

The provenance pipeline has been validated end-to-end on:

- **Linux (ZFS)**: westGate — 3,256 CAS objects, ZFS snapshots as backup layer
- **Windows**: ironGate — full chain validated, same Ed25519 keys
- **5th consecutive pass**: Zero regressions across 5 sequential validation runs

## What This Proves

The provenance chain means every scientific result produced by
NUCLEUS can answer:

1. **What** was computed (BLAKE3 content hash)
2. **When** it was computed (DAG session timestamp)
3. **Where** it was stored (ledger entry with storage backend)
4. **Who** produced it (DID attribution)
5. **How** it was verified (Ed25519 witness signature)

This is the same chain whether the computation runs on westGate (Linux),
ironGate (Windows), or sandGate (Android). The provenance is platform-independent.

## Pending: Live Chain Viewer

This page currently shows static validation status. When petalTongue G19
Node Atomics rendering is complete, it will serve a live view of recent
provenance records from the Nest Atomic composition.

Data source: `spore-validate nucleus westGate --probe` (provenance trio methods)
