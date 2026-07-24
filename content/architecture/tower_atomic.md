+++
title = "Tower Atomic"
description = "Sovereign transport stack — bearDog + songBird + skunkBat replace WireGuard with capability-aware, topology-aware encrypted mesh networking. 267× faster on LAN via topology awareness, 1.7× sustained on degraded WAN paths."
weight = 35
[extra]
companion_type = "architecture"
companion_summary = "The minimum viable encrypted mesh — three primals that replace a kernel VPN with a userspace capability router."
related_pages = [
  { path = "/architecture/primal_interactions/", relation = "extends" },
  { path = "/architecture/NUCLEUS_ARCHITECTURE/", relation = "extends" },
  { path = "/products/", relation = "evidence_for" },
]
+++

Tower Atomic is the sovereign transport stack: {{ entity(name="beardog") }} (cryptography) +
{{ entity(name="songbird") }} (transport routing) + {{ entity(name="skunkbat") }} (protocol negotiation).
Together they provide encrypted peer-to-peer communication that replaces WireGuard.

```
┌─────────────────────────────────────────────────────────┐
│ Tower Atomic Stack                                       │
├─────────────────────────────────────────────────────────┤
│ skunkBat  — Protocol negotiation, bond formation         │
│ songBird  — Transport routing, NAT traversal, mesh       │
│ bearDog   — Crypto: Ed25519, X25519, ChaCha20-Poly1305   │
└─────────────────────────────────────────────────────────┘
```

This is the first **atomic** composition — three primals bonded via JSON-RPC over
UNIX domain sockets, running as independent systemd services, forming a single
capability: sovereign encrypted mesh networking.

## Why replace WireGuard?

WireGuard is excellent at what it does: encrypted kernel tunnels between
fixed endpoints. But it has structural limitations for distributed scientific
computing:

| Limitation | WireGuard | Tower Atomic |
|-----------|-----------|-------------|
| Routing awareness | All packets traverse the same tunnel | JSON-RPC dispatch routes by capability |
| Topology awareness | Fixed endpoints, no LAN discovery | `lan_addr` discovery bypasses VPS for LAN peers |
| Traffic shaping | One tunnel, all traffic | Per-capability routing to specialized stacks |
| Crypto granularity | One static key per tunnel | Per-session BTSP keys + per-capability attestation |
| Compute awareness | Just a pipe | Mesh knows which gate has which hardware |
| Edge tuning | Same overhead regardless of hardware | Tunable: minimal relay/beacon profile for NUCs |

## Benchmark results

Tower Atomic has been benchmarked against WireGuard across LAN and WAN paths
using `songbird benchmark` — a 3-phase measurement harness (setup, latency,
throughput) with p50/p95/p99 statistics.

### LAN (eastGate ↔ sporeGate, 1G MikroTik backbone)

| Metric | Tower Atomic | WireGuard | Verdict |
|--------|-------------|-----------|---------|
| Latency (avg) | 0.57 ms | 0.46 ms | Parity (sub-ms both) |
| Latency (p99) | 0.71 ms | 0.67 ms | Parity |
| Jitter | 0.015 ms | 0.015 ms | Parity |
| Setup time | 0.28 ms | 0.29 ms | Parity |

LAN latency varies by run — early measurements showed Tower 8% faster, later
runs show WG slightly ahead. Both stacks deliver sub-millisecond LAN latency.
The meaningful difference is not protocol speed but **topology awareness**.

### WAN (multi-hop, 67ms+ RTT)

| Metric | Tower Atomic | WireGuard | Verdict |
|--------|-------------|-----------|---------|
| Latency | 136.6 ms | 135.9 ms | Parity |
| Throughput (sustained) | 6.9 Mbps | 3.5–6.7 Mbps | **Tower 1.7× when WG degrades** |
| Jitter | 0.48 ms | 0.48 ms | Parity |

On WAN multi-hop paths (sporeGate → golgiBody → flockGate), Tower maintains
consistent throughput while WireGuard intermittently degrades to ~3.5 Mbps.
Shadow benchmarks across 230+ runs show Tower at **1.7× sustained** on degraded
WAN paths.

### The 267× gap

On the same LAN, WireGuard routes sporeGate↔eastGate traffic through golgiBody
VPS (153ms round-trip) because WG has no concept of LAN topology. Tower discovers
LAN peers via `lan_addr` and routes directly: **0.57ms vs 153ms**.

This is not a protocol speed advantage — it is a **topology awareness** advantage
that WireGuard structurally cannot match.

## Socket topology

The three primals communicate via UNIX domain sockets:

```
┌─ bearDog UDS server ─────────────────────────────────┐
│  beardog.sock                                         │
│  ├── security.sock    ├── btsp.sock                   │
│  ├── crypto.sock      ├── ed25519.sock                │
│  └── x25519.sock                                      │
│  Serves: ~230 methods (btsp.*, crypto.*, auth.*)      │
└───────────────────────────────────────────────────────┘
         ▲                    ▲
         │ UDS                │ UDS
┌─ songBird UDS server ──────┼──────────────────────────┐
│  songbird.sock              │                          │
│  Serves: capability.call, mesh.*, health.*,            │
│    federation.peers/status                             │
└─────────────────────────────┼──────────────────────────┘
         ▲                    │
         │ UDS                │
┌─ skunkBat UDS server ──────┘──────────────────────────┐
│  skunkbat.sock                                         │
│  Serves: security.*, defense.*, health.*               │
└────────────────────────────────────────────────────────┘
```

A cross-gate `capability.call` traverses 4 UDS hops (~0.6ms total on LAN hardware).
With BTSP session establishment on a fresh connection, add 3 bearDog hops (~0.45ms one-time).

## Six exploration domains

Tower Atomic opens exploration space that WireGuard fundamentally cannot address:

### 1. Capability-aware routing — PROVEN LIVE

songBird dispatches by capability name. `nestgate.blob_put`, `beardog.sign`,
`toadstool.dispatch` each route to the correct provider. WireGuard sends all
packets through the same undifferentiated tunnel.

### 2. Multi-stack routing — PROVEN LIVE

songBird dispatches 6 traffic classes to 5 different provider stacks through
a single mesh: SECURITY → skunkBat, HEALTH → sweetGrass, PROVENANCE → sweetGrass,
AI/INFER → squirrel, STORAGE → nestGate, VISUAL → petalTongue.

### 3. Large data transfer — PROVEN LIVE

Content-addressed blob routing via {{ entity(name="nestgate") }} CAS. songBird can
negotiate payload-optimal framing — jumbo frames on 10G backbone, chunked
streaming on WAN. Blobs route to the nearest cached copy.

### 4. Secure compute mesh — PROVEN LIVE

{{ entity(name="beardog") }} provides per-session BTSP crypto with per-capability
attestation. Different trust levels per workload. CredentialStore integration
means keys live in HSM/TEE where available.

### 5. Distributed compute — PROVEN LIVE

songBird's mesh topology knows which gate has which hardware (GPU VRAM, NPU, CPU cores).
Combined with {{ entity(name="toadstool") }} workload dispatch, Tower becomes a
compute-aware mesh — workloads route to the gate with the right substrate.
4-node targeted dispatch proven across the mesh.

### 6. Edge/SFF profile — PROVEN LIVE

On constrained hardware (NUCs, NucBox), Tower runs a minimal relay/beacon profile:
30MB RSS, 39MB total stack, `mesh.announce` with 300s TTL. WireGuard cannot
specialize its overhead for small hardware.

## Convergence timeline

| Phase | Milestone | Status |
|-------|-----------|--------|
| Phase 0 | All Tower components live independently | **COMPLETE** |
| Phase 1 | Parity benchmark: measure Tower vs WG on LAN+WAN | **PASS** |
| Phase 2 | Shadow deploy across all live topology + exploration | **COMPLETE** — 6/6 domains PROVEN LIVE |
| Phase 3 | Cutover: Tower replaces WG for inter-gate traffic | Pending Phase 2 validation |

## Mesh enrollment

Gates join the Tower mesh via HMAC-verified enrollment:

```
Enrolling gate computes:
  proof = HMAC-SHA256(family_seed, node_id|public_key|timestamp)

Sends to hub gate:
  mesh.enroll { node_id, public_key, timestamp, proof, address }

Hub verifies via bearDog:
  songBird → enrollment.verify → bearDog (HMAC check)

On success: node persisted to peers.toml, added to live mesh.
```

## Shadow deployment

Tower runs alongside WireGuard via `membrane tower.shadow --enable`. Both stacks
carry traffic simultaneously — WireGuard for production, Tower for continuous
benchmarking. Shadow metrics collect every 60 minutes across all gate pairs.

230+ benchmark files collected across 3 gates. Results consistently show Tower at
parity or exceeding WireGuard on all measured dimensions.

## Crypto composition migration

songBird is migrating inline cryptography to {{ entity(name="beardog") }} UDS
delegation. The `local-crypto-fallback` feature flag gates inline crypto —
when disabled, all cold-path cryptographic operations route through bearDog's
`crypto.*` capabilities via UDS.

`CRYPTO_COMPOSITION.md` classifies 19 crypto seams:

| Category | Seams | Strategy |
|----------|-------|----------|
| Hot-path | 5 | Chimera (in-process after library extraction) |
| Delegating | 6 | bearDog UDS (`crypto.sign`, `crypto.verify`, etc.) |
| Test-only | 5 | Isolated to `#[cfg(test)]` |
| Already delegating | 3 | No change needed |

This is composition-first engineering: validate UDS delegation works for all
cold-path crypto, measure the overhead, then chimera collapses the hot-path
into shared library calls.

## What comes next

**Chimera optimization**: Collapsing bearDog + songBird + skunkBat into a single
process eliminates 3-6 UDS hops per operation. Estimated LAN latency improvement:
12× (from ~0.6ms to ~0.05ms). This is a {{ entity(name="biomeos") }} design task —
library extraction from each primal to enable in-process composition.
