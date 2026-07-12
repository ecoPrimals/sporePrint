+++
title = "Chapter 5: System Architecture"
description = "ecoPrimals sovereign platform: primals, capability-based composition, NUCLEUS — architecture emerged from Pure Rust constraint."
weight = 5
date = 2026-07-09

[extra]

[[extra.companions]]
url = "/architecture/ecosystem-architecture/"
title = "Ecosystem Architecture"
relation = "evidence_for"
label = "Primals, springs, composition, Neural API — the live system"

[[extra.companions]]
url = "/architecture/nucleus-architecture/"
title = "NUCLEUS Architecture"
relation = "evidence_for"
label = "Atomics ladder, deploy graphs, lifecycle — the composition model"

+++

{{ maturity(level="architectural") }}

## 5.1 Overview

This chapter describes the ecoPrimals architecture: a sovereign computing platform comprising 11 primals that coordinate through capability-based composition. The central claim: **the architecture emerged from constraint, not from design**. Tower Atomic (Pure Rust HTTPS), the bonding model, and the NUCLEUS deployment pattern were not planned — they emerged when the Pure Rust constraint eliminated OpenSSL and forced exploration of the composition pattern.

**Reference**: [Ecosystem Architecture](@/architecture/ECOSYSTEM_ARCHITECTURE.md), [Primal Catalog](@/architecture/PRIMAL_CATALOG.md).

---

## 5.2 The Twelve Primals

### 5.2.1 Foundation Primals (NUCLEUS)

| Primal | Domain | Key Capability |
|--------|--------|----------------|
| **BearDog** | Cryptography | 91 methods: Ed25519, X25519, AES-GCM, BLAKE3, X.509, genetic lineage, Dark Forest |
| **Songbird** | Networking | TLS 1.3, BirdSong discovery, Pure Rust Tor (3,345 lines), NAT traversal |
| **ToadStool** | Compute | Universal workload execution, BarraCuda (124 ops), CPU/GPU/NPU/WASM |
| **NestGate** | Storage | Content-addressed blobs (BLAKE3), ZFS, quota, discovery |
| **Squirrel** | AI Coordination | MCP, model routing, multi-provider inference, vendor-agnostic |
| **biomeOS** | Orchestrator | Neural API, capability discovery, NUCLEUS composition, Dark Forest |

### 5.2.2 Post-NUCLEUS Primals

| Primal | Domain | Key Capability |
|--------|--------|----------------|
| **petalTongue** | Representation | Multi-modal UI (visual, audio, TUI, web), accessibility-first |
| **rhizoCrypt** | Ephemeral Memory | Content-addressed DAG, working memory, lock-free |
| **sweetGrass** | Attribution | W3C PROV-O provenance, fair attribution, braids |
| **LoamSpine** | Permanence | Immutable ledger, Loam certificates, federated sync |
| **skunkBat** | Defense | Threat detection, metadata-only reconnaissance, graduated response |
| **sourDough** | Fermentation | Data pipeline orchestration, batch processing, workflow DAGs |

BarraCuda (914 WGSL shaders, f64, vendor-agnostic) is ToadStool's GPU compute subsystem — not a standalone primal but the single largest crate in the ecosystem (792 Rust source files).

### 5.2.3 Measured Scale by Primal

| Primal | Rust Lines | #[test] | Phase |
|--------|----------:|--------:|-------|
| BearDog | 577,341 | 11,872 | 1 |
| Songbird | 401,900 | 7,314 | 1 |
| Squirrel | 681,933 | 25,359 | 1 |
| ToadStool | 788,209 | 13,503 | 1 |
| NestGate | 645,445 | 14,807 | 1 |
| biomeOS | 321,534 | 14,179 | 2 |
| petalTongue | 160,970 | 6,186 | 2 |
| sweetGrass | 66,168 | 3,769 | 2 |
| sourDough | 50,746 | 3,622 | 2 |
| LoamSpine | 44,453 | 1,952 | 2 |
| rhizoCrypt | 27,565 | 278 | 2 |
| skunkBat | 7,366 | 48 | 2 |
| **Total** | **3,773,630** | **102,889** | |

Note: line counts via `wc -l` (including comments, blank lines, tests). Springs add 119,669 Rust + 103,663 Python + 48,698 WGSL = 272,030 additional lines. Grand total compiled code (Rust + WGSL): ~3.8M lines.

---

## 5.3 Capability-Based Composition

### 5.3.1 Primitives and JSON-RPC IPC

Each primal exposes **primitives** — atomic operations accessible via JSON-RPC 2.0 over platform-agnostic transports. Primals have zero compile-time coupling; no primal imports another's code. Coordination happens at runtime through capability discovery orchestrated by biomeOS.

**Transport matrix**: Unix domain sockets (Linux, macOS), abstract sockets (Android), named pipes (Windows), TCP (cross-device).

### 5.3.2 The Neural API

biomeOS maintains a capability registry. Callers request `capability.call("crypto.sign", ...)`; biomeOS discovers the primal, routes the request, returns the result. The caller never knows which primal handles it. Semantic routing enables hot-swapping, graceful degradation, and pathway learning.

---

## 5.4 NUCLEUS Deployment Model

### 5.4.1 Tower, Node, Nest Atomics

| Atomic | Components | Capability Added |
|--------|------------|------------------|
| **Tower** | BearDog + Songbird | Pure Rust HTTPS, zero C dependencies |
| **Node** | Tower + ToadStool | Compute (CPU/GPU/NPU/WASM) |
| **Nest** | Tower + NestGate | Content-addressed storage |
| **Full NUCLEUS** | Tower + Node + Nest + Squirrel | Complete sovereign stack |

### 5.4.2 Tower Atomic — The Headline Innovation

The Pure Rust constraint made OpenSSL impossible. Rather than a monolithic Pure Rust TLS library, constrained evolution produced a composition: BearDog provides 72 cryptographic methods via JSON-RPC; Songbird implements the TLS 1.3 state machine and delegates all crypto to BearDog. Result: 93% validation across 87 production sites, 366ms average HTTPS latency, zero C, zero unsafe.

**This is the citrate metabolism of ecoPrimals**: an innovation that emerged from constraint, not from design.

### 5.4.3 genomeBin and Deployment

genomeBin wraps ecoBin with deployment machinery: system detection, binary extraction, service integration (systemd/launchd/OpenRC), health validation, ecosystem registration. One command deploys on any platform.

---

## 5.5 The Bonding Model

### 5.5.1 Chemistry-Inspired Trust Levels

| Bond Type | Trust Level | Use Case |
|-----------|-------------|----------|
| **Covalent** | Genetic (family seed) | Basement HPC mesh — Northgate, Southgate, Strandgate, Westgate |
| **Ionic** | Contract-based, metered | Cloud burst compute, external researcher GPU rental |
| **Metallic** | Sub-specialization | Rack of compute-only or storage-only gates |
| **Weak** | Pre-trust | OpenAI API, unknown beacons, default state |

### 5.5.2 Dark Forest Protocol

Zero metadata leakage: beacons encrypted with ChaCha20-Poly1305 are indistinguishable from random noise. Challenge-before-reveal; lineage relay for family members; physical anchor (SoloKey FIDO2). Better than Signal/Tor for metadata privacy.

---

## 5.6 Empirical Scale

| Metric | Value | Method |
|--------|-------|--------|
| Total Rust lines (primals) | 3,773,630 | `wc -l` |
| Total Rust lines (springs) | 119,669 | `wc -l` |
| Total WGSL lines | 48,698 | `wc -l` |
| WGSL shader files | 628 | `find *.wgsl` |
| #[test] annotations (primals) | 102,889 | `grep -c` |
| #[test] annotations (springs) | 1,294 | `grep -c` |
| Validation checks (springs) | 11,161+ | Automated CI |
| C dependencies | Zero (application layer) | `cargo tree` |
| IPC | JSON-RPC 2.0, Unix sockets | — |
| Platforms | Linux, macOS, Android, Windows, FreeBSD, illumos, WASM | — |

Squirrel (681,933 lines) is the largest primal by Rust volume; BearDog has the most cryptographic surface area (91 methods); ToadStool houses the most WGSL (914 shaders via BarraCuda). The disparity between phase 1 and phase 2 primals reflects evolutionary maturity — the five foundation primals have undergone ~10 months of constrained evolution; the seven post-NUCLEUS primals are younger.

---

## 5.7 Architecture as Evidence

The architecture was not designed on a whiteboard. It emerged from ~6–8 months of constrained evolution. The architecture ladder (UniBin → ecoBin → genomeBin) was not planned; each stage responded to the previous stage's limitations. The bonding model emerged when the HPC grew from one machine to several.

If constrained evolution operates as Chapter 3 predicts, the architecture's coherence is expected: populations under consistent constraint specialize toward fitness, and the resulting structure reflects the constraint environment. The ecoPrimals architecture reflects Rust's type system, the Pure Rust directive, and capability-based coordination — because those constraints shaped every evolutionary step.

---

## References

See [Ecosystem Architecture](@/architecture/ECOSYSTEM_ARCHITECTURE.md), [Primal Catalog](@/architecture/PRIMAL_CATALOG.md), [Constrained Evolution — Formal](@/methodology/CONSTRAINED_EVOLUTION_FORMAL.md) §4.3 (convergent evolution), §4.4 (firefly analogy).

---

**See also:**

- [Ecosystem Architecture](@/architecture/ECOSYSTEM_ARCHITECTURE.md) — the full architecture document
- [Primal Catalog](@/architecture/PRIMAL_CATALOG.md) — per-primal specifications
- [BarraCuda](@/thesis/06_barracuda.md) — GPU compute as a case study
