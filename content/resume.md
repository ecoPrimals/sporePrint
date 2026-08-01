+++
title = "Platform Resume"
description = "ecoPrimals platform resume — sovereign scientific computing. Pure Rust, no cloud, no CUDA, no vendor lock-in. Built by a single individual with AI augmentation."
date = 2026-08-01

[extra]
maturity = "live"
+++

**Identity**: ecoPrimal | **ORCID**: [0009-0004-2141-0321](https://orcid.org/0009-0004-2141-0321)
**License**: AGPL-3.0-or-later | **Website**: [primals.eco](https://primals.eco)
**Code**: [git.primals.eco](https://git.primals.eco) (sovereign) | [github.com/ecoPrimals](https://github.com/ecoPrimals) (mirror)
**Wave**: 155n post-threshold | **Date**: Aug 1, 2026

---

## What ecoPrimals Is

Sovereign scientific computing infrastructure. Pure Rust. No cloud. No CUDA.
No vendor lock-in. No commercial dependencies in the inner membrane.

15 binaries compose into a distributed organism that runs on commodity hardware
you own. The system ingests real science data, proves provenance cryptographically,
and delivers reproducible results — on a $485/month metabolic budget.

A single individual designed and built the entire ecosystem using AI-augmented
constrained evolution (K-NOME methodology). The architect is a bench scientist
with microbiology and data science credentials who built this because proprietary
stacks failed real lab workflows.

---

## What ecoPrimals Has Proven

### Infrastructure (gen4 — COMPLETE)

| Metric | Value |
|--------|-------|
| Primals (infrastructure binaries) | 15 (13 active + 2 dormant) |
| Springs (science validation suites) | 9 |
| Total primal tests | {{ total_stat(stat="total_tests") }} |
| Lines of Rust | {{ total_stat(stat="total_loc") }} across 43 repositories |
| Published papers reproduced | 175+ with explicit numerical tolerances |
| Depot binaries | 35 (16 Linux-musl + 4 Linux-gnu + 15 Windows) |
| NUCLEUS gates (validated deployments) | 4 (Linux x3, validation gate x1) + Windows parity on ironGate |
| Sovereign CI | Push-to-deploy for all 13 primals. Automated build, test, depot, publish. |
| Provenance chain | 7/7 COMPLETE — CAS → DAG → Merkle → Spine → Ed25519 → Attribution braid |
| Hardware investment | ~$15K commodity (EPYC, Ryzen, RTX 3090/4060/5090, 50.7 TB ZFS) |
| Metabolic cost | ~$485/month (electricity, ISP, VPS) |
| C dependencies in application code | Zero |
| Unsafe Rust blocks | Zero (except hardware-touching GPU/VFIO) |

### Science Data (gen5 — THESIS PROVEN ON LIVE DATA)

| Achievement | Evidence |
|-------------|----------|
| First real science data ingested | 506 PDB protein structures + ChEMBL 37 (2.9M compounds, 24.5M bioactivities, 33.79 GB) |
| Provenance coverage | 100% — every object BLAKE3 hashed, DAG tracked, spine committed, Ed25519 signed, attribution braided |
| Pipeline throughput | 16.5 GB/s BLAKE3 hashing. Zero pipeline failures at 33.79 GB scale. |
| Data systems cataloged | 115 public databases mapped, 44 wired in Rust |

### Portability (PROVEN)

| Achievement | Evidence |
|-------------|----------|
| Cold reconstitution | NUCLEUS deployed from public depot on new hardware with no pre-existing trust, no WireGuard, no inherited identity |
| Validation scorecard | 22/22 PASS across trust, stability, atomic composition, and portability checks |
| Stability | 20 hours continuous, 32 active sockets, 76 MB resident memory, 13/13 processes |
| Trust enforcement | 29,294 foreign peer rejections via BTSP — security boundary is the cryptographic family, not the network |

---

## Architecture

### The Composition Model

```
Tower Atomic    = bearDog (crypto) + songBird (network) + skunkBat (defense)
Nest Atomic     = Tower + nestGate (CAS) + rhizoCrypt (DAG) + loamSpine (ledger) + sweetGrass (attribution)
Node Atomic     = Tower + toadStool (hardware) + barraCuda (math) + coralReef (shaders)
NUCLEUS         = All 13 primals, orchestrated by biomeOS
```

No primal imports another primal's code. All composition happens at runtime via
capability-based discovery. biomeOS discovers what's available and coordinates it.
Complexity emerges from coordination, not from expanding scope.

### Trust Model (BTSP)

Every primal handshake uses the BearDog Trust Security Protocol — a 3-phase
cryptographic enrollment that establishes trust via genetic lineage (shared
family seed), not network topology. A gate behind a firewall and a gate on a
friend's LAN use the same trust model. WireGuard is transport optimization,
not security. Proven: southGate ran NUCLEUS for 20 hours without WireGuard,
rejecting 29,294 foreign peers mathematically.

### Provenance Model

Every piece of data that enters the system receives:

1. **BLAKE3 content hash** (content-addressed storage in nestGate)
2. **DAG tracking** (ephemeral working memory in rhizoCrypt)
3. **Spine commitment** (immutable linear ledger in loamSpine)
4. **Ed25519 signature** (cryptographic signing via bearDog)
5. **Attribution braid** (semantic provenance via sweetGrass, W3C PROV-O)

The chain is end-to-end: from raw data ingestion to verifiable scientific artifact.
Provenance 7/7 validated on Linux (ZFS) and Windows. 8 consecutive passes.

### Deployment Model

Sovereign CI on sporeGate: code push → cargo build (musl, gnu, windows) →
BLAKE3 verification → depot publish → gate pull. Sub-builder dispatch to
blueGate (Windows) via SSH. Auto-publish to primals.eco via Forgejo post-receive
hook. 35 binaries across 3 platforms, continuously validated.

---

## Scientific Domains

| Domain | Spring | Papers Reproduced | Key Methods |
|--------|--------|-------------------|-------------|
| Metagenomics | wetSpring | 20+ | 16S pipelines, DADA2, UniFrac, quorum sensing, community modeling |
| Lattice QCD | hotSpring | 15+ | HMC, Wilson gauge, gradient flow, pseudofermion, metadynamics |
| Precision agriculture | airSpring | 12+ | FAO-56 ET₀ (7 methods), Richards equation, SCS-CN runoff, Shannon diversity |
| Pharmacology | healthSpring | 10+ | Population PK (NONMEM parity), drug-disease NMF, RGES scoring |
| Analytical chemistry | blueFish | 5+ | EPA 1633A PFAS, HPLC/MS integration, NIST verification |
| Neural architectures | neuralSpring | 8+ | ESN/LSM reservoirs, Kuramoto oscillators, edge-of-chaos |
| Genomics pipelines | helixVision | 5+ | AlphaFold structure prediction, LINCS connectivity |
| Game engines | ludoSpring | 3+ | ECS, continuous 60Hz tick, scene composition |
| Ecosystem coordination | primalSpring | 5+ | Atomic composition testing, Plasmodium formation, Dark Forest |

175+ papers total. Each reproduction includes explicit numerical tolerances,
`cargo run --bin validate_*` binaries, and comparison against published figures.

---

## GPU Compute (Vendor-Agnostic)

Consumer GPUs allocate far fewer FP64 ALUs than FP32 — typically 1:32 or 1:64
by hardware design. ecoPrimals uses DF64 (double-float emulation: two FP32 ops
per logical FP64 op) to achieve ~14 significant digits of precision at FP32
throughput rates, bypassing CUDA entirely via Vulkan/WebGPU (wgpu) + WGSL shaders:

| Component | What It Does |
|-----------|-------------|
| barraCuda | 806 WGSL shaders — the mathematics (measured: 2,130 matmul/sec on RTX 3090) |
| coralReef | Sovereign WGSL→native shader compiler (naga parser + lowering) |
| toadStool | Hardware discovery + compute dispatch (CPU, GPU, NPU, WASM) |

Validated: RTX 3090 (24 GB), RTX 4060, RTX 5090 (32 GB), RX 6950 XT (16 GB).
No CUDA. No ROCm. No vendor SDK. Runs on any GPU with Vulkan 1.2+ support.

---

## Methodology: K-NOME

K-NOME (Knowledge-Navigated Ontological Meta-Evolution) is the development
methodology: one human architect defines constraints (Pure Rust, zero unsafe,
capability-based IPC, AGPL-3.0), and AI agents implement within those constraints.

The human holds the vision and the architecture. The agents execute the evolution.
Every commit is transparent, every decision is constrained by the type system,
every primal is independently testable.

This produces a solo-operator output that would typically require teams of dozens:
{{ total_stat(stat="total_loc") }} LOC, {{ total_stat(stat="total_tests") }} tests, 35 binaries, 4 validated gates, 175+ papers — built and
maintained by one person with AI augmentation.

---

## What ecoPrimals Can Do For You

**For researchers**: Ingest your data with cryptographic provenance on hardware
you own. Replace Galaxy, QIIME2, NONMEM, or CUDA-locked pipelines with sovereign
alternatives validated against published results. Verify: `cargo test --workspace`.

**For labs**: Sovereign CI builds your binaries. Depot serves them. Gates deploy
them. Provenance traces every object. No cloud account required. No vendor contract.
Cost: hardware you may already own + $485/month.

**For collaborators**: Clone a primal. Run `./validate`. See PASS/FAIL against
published science. Build on it. The AGPL-3.0 license means the code is free
forever. Consulting available for deployment, integration, and training.

**For grant reviewers**: {{ total_stat(stat="total_tests") }} tests. 175+ papers reproduced. 4 validated
deployments on commodity hardware. Sovereign CI. Cryptographic provenance.
Cross-platform (Linux, Windows, Android). AGPL-3.0. Zero vendor lock-in.
Technical appendix available.

---

## Verification

Everything claimed in this document is verifiable:

| Claim | How to Verify |
|-------|---------------|
| {{ total_stat(stat="total_tests") }} tests | `cargo test --workspace` on any primal |
| Provenance 7/7 | Run provenance trio validation on any NUCLEUS gate |
| 35 depot binaries | `curl https://depot.primals.eco/checksums.toml` |
| Portability | Deploy NUCLEUS from public depot on any Linux machine with `~/.local/bin/` |
| 175+ papers | Each spring contains `validate_*` binaries with published comparison |
| Trust model | Launch NUCLEUS without WireGuard — BTSP enforces trust, network doesn't matter |
| Source code | [git.primals.eco](https://git.primals.eco) (sovereign Forgejo) or [github.com/ecoPrimals](https://github.com/ecoPrimals) (mirror) |

---

## Contact

**Project**: eco.primal@primal.eco
**ORCID**: [0009-0004-2141-0321](https://orcid.org/0009-0004-2141-0321)
**Website**: [primals.eco](https://primals.eco)
**Code**: [git.primals.eco](https://git.primals.eco) | [github.com/ecoPrimals](https://github.com/ecoPrimals)
**License**: AGPL-3.0-or-later

*Built by a single individual. Designed for everyone.*

---

*subGen Wave 155n — ecoPrimals platform resume. gen5 thesis proven on live data.
The architect is a bench scientist who built sovereign infrastructure because
the alternatives failed real science. The identity disconnect between human and
project is deliberate and owned. The work speaks for itself.*
