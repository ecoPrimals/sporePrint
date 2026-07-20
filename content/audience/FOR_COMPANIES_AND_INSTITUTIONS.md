+++
title = "For Companies and Institutions"
description = "AGPL-3.0 implications, proprietary stack replacement, consulting model, and regulated environment deployment — what companies need to know before evaluating ecoPrimals."
date = 2026-07-20

[taxonomies]
primals = ["barracuda", "beardog", "coralreef", "loamspine", "nestgate", "rhizocrypt", "songbird", "sweetgrass", "toadstool"]
springs = ["groundspring", "healthspring", "hotspring", "wetspring"]
trails = ["first-visit", "grant-ready"]

[extra]

[[extra.companions]]
url = "/methodology/scyborg-licensing/"
title = "scyBorg Triple License"
relation = "pairs_with"
label = "Full licensing framework — AGPL + ORC + CC-BY-SA"

[[extra.companions]]
url = "/audience/CAPABILITY_PARITY_BRIEF/"
title = "Capability Parity Brief"
relation = "evidence_for"
label = "Domain-by-domain parity assessment against 40+ proprietary tools"

[[extra.companions]]
url = "/architecture/SOVEREIGN_PRIOR_ART_CATALOG/"
title = "Sovereign Prior Art Catalog"
relation = "evidence_for"
label = "52 innovations permanently locked in the commons"

[[extra.companions]]
url = "/outreach/consulting/"
title = "Sovereign Consulting"
relation = "pairs_with"
label = "How to engage expert help for deployment"
+++

## What This Is

{{ entity(name="ecoprimals") }} is a pure Rust sovereign scientific computing
platform: {{ total_stat(stat="total_primals") }} primals,
{{ total_stat(stat="total_springs") }} springs,
{{ total_stat(stat="total_loc_display") }} lines of code,
{{ total_stat(stat="total_tests_display") }} tests. It runs on your hardware,
behind your firewall, with no cloud dependency. Every binary builds from source.
Every claim has a validation binary.

This page covers what institutional evaluators need to know.

---

## AGPL-3.0 — What It Means for You

All code is licensed under **AGPL-3.0-or-later**. The full licensing framework
is documented in the [scyBorg Triple License](@/methodology/SCYBORG_LICENSING.md).

**What you can do:**

- Deploy on your own infrastructure — internal use is not distribution
- Modify the source for your workflows — your patches stay yours if internal
- Run it as an internal service for your employees — no copyleft trigger
- Benchmark against your existing stack — clone, build, measure

**What triggers copyleft:**

- **Distribution** — shipping modified binaries to customers
- **Network use** — exposing modified code as a service to external users (the "A" in AGPL)

If you modify the code and use it only internally, you have no obligation to
release your changes. If you expose modifications as a service or distribute
binaries, you must share the corresponding source under AGPL-3.0.

**The symbiotic exception:**

Internal modifications that improve upstream are welcome as contributions.
Accepted patches become part of the commons. Your name goes in the commit log.
This is collaboration, not obligation.

---

## What It Replaces

The stack provides sovereign alternatives to proprietary tools across 8 scientific
domains. Full domain-by-domain analysis is in the
[Capability Parity Brief](@/audience/CAPABILITY_PARITY_BRIEF.md).

| Domain | Proprietary Tools | ecoPrimals Replacement |
|--------|------------------|----------------------|
| Pharmacometrics | NONMEM, Monolix, WinNonlin | {{ entity(name="hotspring") }} |
| Genomics / Metagenomics | Galaxy, QIIME2, mothur | {{ entity(name="wetspring") }} |
| Proteomics / LC-MS | MassHunter, Chromeleon, Skyline | {{ entity(name="healthspring") }} |
| GPU Compute | CUDA SDK, vendor lock-in | {{ entity(name="barracuda") }} — WebGPU/WGSL, any vendor |
| Protein Structure | AlphaFold (cloud), Rosetta | {{ entity(name="barracuda") }} + {{ entity(name="coralreef") }} — local |
| Data Provenance | Manual audit trails, paper logs | {{ entity(name="loamspine") }} + {{ entity(name="rhizocrypt") }} + {{ entity(name="sweetgrass") }} |
| Scientific Storage | Cloud object stores, NFS | {{ entity(name="nestgate") }} — content-addressed, local |
| Deployment / Federation | Kubernetes, proprietary orchestration | {{ entity(name="songbird") }} + {{ entity(name="toadstool") }} — mesh federation |

---

## Consulting Engagement Model

ecoPrimals is not a SaaS product. There is no subscription, no license fee,
no account manager, no upsell path.

If your organization needs help deploying, training, validating, or integrating
the stack, consulting is available as a contractor engagement. When it ends,
you own the deployment. Details are on the
[Sovereign Consulting](@/outreach/consulting.md) page.

**Free for:** individuals, students, LCCs, community colleges, K-12, nonprofits,
university departments.

**Consulting at market rates for:** companies, pharmaceutical firms, government
labs, national facilities.

---

## Regulated and Air-Gapped Environments

The stack is built for environments where data cannot leave the building:

**Provenance and audit:**
- {{ entity(name="loamspine") }} — append-only provenance ledger, BLAKE3-verified
- {{ entity(name="rhizocrypt") }} — content-addressed integrity for every artifact
- {{ entity(name="sweetgrass") }} — attribution and lineage tracking across the mesh
- {{ entity(name="beardog") }} — cryptographic identity with HSM backends
  (TPM, Linux SecretService, Windows DPAPI, Android Keystore)

**Air-gapped deployment:**
- USB gate enrollment — `gate-usb-bootstrap.sh` provisions new machines into
  the mesh without internet. WireGuard keys, RustDesk credentials, and primal
  binaries are carried on a signed USB drive
- No phone-home telemetry. No license server. No activation

**Regulatory mapping:**
- 21 CFR Part 11 — electronic records and signatures via provenance chain
- GxP compliance — every computation is reproducible with signed inputs/outputs
- ITAR / classified — fully air-gapped mesh with no external dependencies
- BSL-3/4 containment — isolated network segments with gateway-only access

See [For Compliance and Institutional Review](@/audience/FOR_COMPLIANCE_AND_INSTITUTIONAL_REVIEW.md)
for the full regulatory mapping.

---

## Hardware Validation

The stack runs on commodity hardware. No specialized appliances, no vendor-specific
accelerators.

**Validated GPU vendors:**
- NVIDIA (GeForce, RTX, Quadro) via WebGPU/WGSL — no CUDA required
- AMD (RDNA2+) via Vulkan/WebGPU
- Intel (Arc) via Vulkan/WebGPU

**Validated NPU:**
- BrainChip AKD1000 — pure Rust driver, three-substrate pipeline
  (CPU → GPU → NPU)

**Validated architectures:**
- x86-64 (Intel, AMD — consumer and server)
- ARM64 (Raspberry Pi 5, Apple Silicon via cross-compilation)

The [Sovereign Prior Art Catalog](@/architecture/SOVEREIGN_PRIOR_ART_CATALOG.md)
documents 52 innovations permanently in the commons, including vendor-agnostic
GPU dispatch, three-tier precision, and federation protocols.

---

## How to Evaluate

1. Clone any primal or spring repository — all are public at
   [github.com/ecoPrimals](https://github.com/ecoPrimals)
2. Build from source: `cargo build --release`
3. Run validation: `cargo test` — {{ total_stat(stat="total_tests_display") }}
   tests across the ecosystem
4. Review the [Evidence Snapshot](@/architecture/EVIDENCE_SNAPSHOT.md) for
   current metrics (updated {{ total_stat(stat="measured_date") }})
5. If you need help: [Sovereign Consulting](@/outreach/consulting.md)

---

*All content on this site is CC-BY-SA 4.0. All code is AGPL-3.0-or-later.
All game mechanics are ORC. These licenses are governed by independent nonprofits
and are structurally irrevocable.*
