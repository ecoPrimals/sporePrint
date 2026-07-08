+++
title = "70 Papers, One Stack"
description = "20,695+ quantitative checks across 8 domains. Every computation content-addressed, DAG-tracked, ledger-committed, and attributed."
date = 2026-07-08
weight = 30

[taxonomies]
primals = []
springs = []
+++

*{{ total_stat(stat="validation_checks") }} quantitative checks across 8 domains. Every computation content-addressed, DAG-tracked, ledger-committed, and attributed.*

---

## What "Reproduced" Means

Reproducing a paper is not running the authors' code. It is:

1. Read the methods section
2. Identify quantitative claims (specific numbers, with error bounds)
3. Implement the methodology independently, from the equations
4. Fetch the same input data (or generate synthetic data matching the
   paper's description)
5. Run the computation
6. Compare output to published values
7. Determine whether the difference is within reported uncertainty

When I say "Bazavov SU(3) Wilson gauge 12/12," I mean: independently
implemented the lattice QCD computation, ran it on a consumer RTX 4070
using DF64 double-float emulation through Vulkan, obtained 12 of 12
target values within published error bars.

When I say "FAO-56 across 100 stations," I mean: independently
implemented Penman-Monteith reference evapotranspiration from FAO Paper
56, fetched weather data from 100 Michigan stations, computed ET₀ using
five independent methods, verified cross-method agreement within known
tolerances.

---

## The Springs

Eight validation frameworks. Each is a standalone Rust binary with its
own test suite and its own set of reproduced results.

### wetSpring — Microbiology, Metagenomics

1,902 tests. 379 experiments. 5,700+ checks. 52 papers.

Key reproductions:
- 16S rRNA pipelines
- QS gene quantification across 170 metagenomes (299,000+ checks)
- Rika Anderson deep-sea vent metagenomics (6 papers)
- Waters quorum sensing lineage (7 papers)
- Jones PFAS environmental remediation (40/40 checks)

### hotSpring — Plasma Physics, Lattice QCD

990 tests. 176 experiments. 697+ checks. 25 papers.

Key reproductions:
- Murillo Yukawa one-component plasma (5 papers, 195/195 checks)
- Bazavov SU(3) Wilson gauge lattice QCD (12/12 target values)
- Abelian Higgs model (17/17)
- Dynamical QCD with RHMC rational approximation on consumer GPU
- Kachkovskiy spectral Anderson (45 checks)

### airSpring — Agricultural Hydrology

2,631+ checks. 57 papers.

Key reproductions:
- FAO-56 Penman-Monteith ET₀ across 100 Michigan stations
- Five independent methods: Hargreaves-Samani, Makkink, Turc, Hamon,
  Priestley-Taylor
- Dong et al. precision irrigation soil moisture (326 checks)
- Error propagation through FAO-56 chain

### groundSpring — Measurement, Uncertainty

236+ checks. 21 papers. 395/395 cross-validation. 29 experiments.

The cross-cutting spring. Validates the statistical foundations the other
springs depend on:
- Anderson localization W_c = 16.26 ± 0.95 (3D critical disorder)
- Error propagation chains across FAO-56, QS, plasma coupling
- Monte Carlo convergence verification
- Uncertainty bounds on localization parameters

### neuralSpring — ML, Surrogates, NPU

3,200+ checks. 25+ papers.

Key reproductions:
- Dolson digital evolution (46/46)
- Neural network surrogates for expensive physics computations
- Edge inference on BrainChip Akida AKD1000 (physical NPU hardware)
- nS-601–605 cytokine propagation surrogates (329/329)

### healthSpring — Pharmacometrics, Clinical

940+ tests. 83 experiments. 113/113 cross-validation across 9 tracks.

Key reproductions:
- Mok testosterone replacement PK (233/233)
- Population PK modeling
- Gut-brain axis
- Heart rate variability
- Drug repurposing pipeline (MSU discovery matrix)

### ludoSpring — Game Science, HCI

791 tests. 75 experiments. 1,692+ checks.

Key reproductions:
- Fitts' Law, Hick's Law
- Flow state models (Csikszentmihalyi)
- Dynamic difficulty adjustment
- NPC dialogue generation
- Provenance lifecycle in game loops

### primalSpring — Integration, Deploy

404 tests. Validates service composition, inter-service communication,
and full deploy graph under load.

---

## The Anderson Thread

One mathematical formalism connects nearly every domain:

Anderson localization — in disordered systems, wave propagation is
exponentially suppressed. Originally about electron transport (Anderson
1958). The transfer matrix / Lyapunov exponent mathematics applies
identically to:

- **Microbiology**: QS signal diffusion in heterogeneous biofilms
- **Plasma physics**: wave propagation at critical coupling thresholds
- **Immunology**: cytokine propagation as disorder-localized transport
- **Agricultural hydrology**: uncertainty concentration in FAO-56 chains
- **Lattice QCD**: confinement parallels in gauge theories

groundSpring computes localization parameters and traces the same
formalism through biological, physical, and agricultural systems. The
eight springs are not eight unrelated projects — they are eight views of
a connected mathematical landscape.

---

## Provenance Pipeline

Every computation produces a cryptographic trace. Four layers:

### Layer 1: Content Addressing (BLAKE3)
Raw data, intermediate results, and final outputs are hashed at creation.
The hash is the identifier. If data changes by one bit, the hash changes.

### Layer 2: DAG Tracking (rhizoCrypt)
Each computation records inputs and outputs as nodes in a directed acyclic
graph. Merkle structure: every node's integrity is verifiable from its
children's hashes. Trace any result backward through the full chain.

### Layer 3: Permanent Ledger (loamSpine)
DAG entries committed to append-only ledger. Ed25519-signed by the
producing node. Entries cannot be edited or deleted. Tampering is
cryptographically detectable.

### Layer 4: Attribution Braids (sweetGrass)
Every computation carries attribution metadata: who provided data, who
wrote the pipeline, who ran it, who reviewed it. W3C PROV conventions.
Cryptographically bound to the computation.

---

## darkforest: Self-Testing

darkforest (v2.0) is a pure Rust binary (939 KB) that validates its own
ecosystem. 181 checks:

**Pen tests** (3 threat actors):
- External attacker: no credentials, probing from outside
- Authorized user: valid compute credentials, testing privilege escalation
- Restricted observer: minimal access, testing lateral movement

**Protocol fuzzing**:
- Malformed inputs for every service's JSON-RPC and BTSP endpoints
- Boundary conditions, oversized payloads, invalid tokens, type confusion

**Crypto strength** (13 checks):
- Cookie entropy, shadow hash algorithms, token tamper resistance
- Cipher negotiation, TLS certificate chain, file permissions, key rotation

Latest: **175 PASS, 0 FAIL, 6 DARK_FOREST** (informational).
Structured JSON output for audit.

---

## Summary Table

| Metric | Value |
|--------|-------|
| Springs | 8 |
| Tests | 27,000+ |
| Quantitative checks | {{ total_stat(stat="validation_checks") }} |
| Papers reproduced | 70+ |
| Scientific domains | 8 |
| Provenance layers | 4 (BLAKE3 → DAG → ledger → braid) |
| Security checks | 181 (175 PASS) |
| Cross-spring formalism | Anderson localization |
| Development time | ~69 days active |

Every claim above has a test. Every test has a source paper. Every paper has a published result. Every result has a provenance chain.

---

## Read More

- [Discovery Is Local](@/philosophy/discovery_is_local.md) — why discovery must be private, but results must be public
- [The Loaves and the Fishes](@/philosophy/the_loaves_and_the_fishes.md) — the miracle of knowing what is already there
- [The Many Rooms](@/philosophy/the_many_rooms.md) — preparing rooms in a house that does not belong to you

---

## Verify

- **Deployment**: github.com/sporeGarden/projectNUCLEUS
- **All services**: github.com/ecoPrimals
- **Scientific domains**: github.com/sporeGarden/foundation
- **Live system**: lab.primals.eco
- **Security results**: darkforest-latest.json in validation/
- **License**: AGPL-3.0-or-later
