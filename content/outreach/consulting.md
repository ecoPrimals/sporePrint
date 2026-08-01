+++
title = "Sovereign Consulting"
description = "The code is AGPL-3.0 and free forever. Deployment, training, and integration consulting for departments and companies that need help running the stack."
date = 2026-07-20
weight = 7

[taxonomies]
primals = ["barracuda", "coralreef", "nestgate", "songbird", "toadstool"]
springs = ["groundspring", "healthspring", "hotspring", "wetspring"]
trails = ["first-visit"]

[extra]
foundation = true

[[extra.companions]]
url = "/methodology/scyborg-licensing/"
title = "scyBorg Triple License"
relation = "pairs_with"
label = "The licensing model that makes this possible"

[[extra.companions]]
url = "/audience/for-faculty-and-pis/"
title = "For Faculty and PIs"
relation = "pairs_with"
label = "What the stack replaces in your lab"

[[extra.companions]]
url = "/audience/capability-parity-brief/"
title = "Capability Parity Brief"
relation = "evidence_for"
label = "Domain-by-domain comparison against proprietary tools"

[[extra.companions]]
url = "/architecture/sovereign-prior-art-catalog/"
title = "Sovereign Prior Art Catalog"
relation = "evidence_for"
label = "52 innovations locked in the commons"
+++

**A human reads and responds to every inquiry at [eco.primal@pm.me](mailto:eco.primal@pm.me).**

## The Code Is Free

Every binary, every shader, every primal — **AGPL-3.0-or-later**, free for
humans, forever. This is not a trial. There is no "enterprise edition." The
[scyBorg triple license](@/methodology/SCYBORG_LICENSING.md) covers code
(AGPL-3.0), mechanics (ORC), and documentation (CC-BY-SA 4.0), each governed
by an independent nonprofit. No single entity can re-close the commons.

You can clone every repository, build every binary, deploy on your own hardware,
and never contact us. That is the design.

---

## What the Stack Replaces

ecoPrimals is a pure Rust scientific computing platform that replaces
proprietary tools across multiple domains. The full comparison is in the
[Capability Parity Brief](@/audience/CAPABILITY_PARITY_BRIEF.md); here are
the highlights:

| Proprietary Tool | What It Costs You | ecoPrimals Replacement |
|-----------------|-------------------|----------------------|
| NONMEM | ~$2,000/year/seat | {{ entity(name="hotspring") }} — pharmacometric modeling |
| Monolix / WinNonlin | ~$3,000–5,000/year | {{ entity(name="hotspring") }} — dose-response, PK/PD |
| Galaxy / QIIME2 | Free but cloud-dependent | {{ entity(name="wetspring") }} — 16S pipeline, local |
| MassHunter / Chromeleon | $10,000–50,000 + maintenance | {{ entity(name="healthspring") }} — LC-MS analysis |
| AlphaFold (cloud) | GPU hours, API limits | {{ entity(name="barracuda") }} + {{ entity(name="coralreef") }} — local Vulkan, no CUDA |
| CUDA SDK | Vendor lock-in | {{ entity(name="barracuda") }} — WebGPU/WGSL, any GPU |
| CRO outsourcing | $50,000–200,000/study | Full in-house pipeline with provenance |

See [For Faculty and PIs](@/audience/FOR_FACULTY_AND_PIS.md) for domain-specific
entry points and clone-and-validate instructions.

---

## Who Pays, Who Doesn't

**Free — always:**

- Individual humans running it on their own hardware
- Students at any institution
- Community colleges and LCCs
- K-12 schools
- Nonprofits and community organizations
- University departments for research and teaching

**Consulting available — market rates:**

- Companies deploying on institutional hardware
- Pharmaceutical companies replacing CRO pipelines
- Government labs and national facilities
- Any organization that needs help but can't (or won't) fork and figure it out

---

## What "Consulting" Means

This is not a subscription. Not a license fee. Not a managed service.

Consulting means a contractor engagement: the person who built the stack helps
you deploy it, train your team, validate your pipelines, and integrate with
your existing infrastructure. When the engagement ends, you own the deployment.
No recurring fees. No phone-home telemetry. No hostage data.

**Typical engagements:**

| Service | What You Get |
|---------|-------------|
| Deployment | Stack running on your hardware, behind your firewall |
| Training | Your team can operate, update, and extend the stack independently |
| Validation | Your regulatory workflows (21 CFR Part 11, GxP, CLIA) mapped to primal provenance |
| Integration | Existing LIMS, EHR, or data pipelines connected to spring endpoints |
| Air-gapped install | BSL-3/4, ITAR, classified environments — USB gate enrollment, no internet required |

After the engagement: you have the code, the binaries, the documentation,
and the knowledge. You can call again if you need to. You don't have to.

---

## Regulated Environments

The stack is designed for deployment in regulated and air-gapped environments:

- **{{ entity(name="beardog") }}** — cryptographic identity and credential management
  with HSM platform backends (TPM, SecretService, DPAPI, Android Keystore)
- **{{ entity(name="loamspine") }}** — append-only provenance ledger for audit trails
- **{{ entity(name="rhizocrypt") }}** — BLAKE3 content-addressed data integrity
- **{{ entity(name="sweetgrass") }}** — attribution and lineage tracking
- **USB gate enrollment** — offline bootstrapping of new machines into the mesh
  via `gate-usb-bootstrap.sh`, no internet connection required

The [Sovereign Prior Art Catalog](@/architecture/SOVEREIGN_PRIOR_ART_CATALOG.md)
documents 52 innovations permanently locked in the commons — including
provenance chain verification, GPU dispatch, and federation protocols that
regulated environments require.

---

## The AGPL-3.0 Moat

Why give the code away and charge for expertise?

Because the expertise is scarce and the code is not. Anyone can read the source.
Building a sovereign scientific computing platform from {{ total_stat(stat="total_loc_display") }}
lines of Rust across {{ total_stat(stat="total_primals") }} primals and
{{ total_stat(stat="total_springs") }} springs — and knowing which pieces to
deploy for a specific lab's workflow — is not something you learn from the README.

The AGPL-3.0 license means no one can fork and close it. No "open core" bait.
No eventual re-licensing. The commons stays common. The consulting model funds
the builder, not a corporation.

---

*The stack has {{ total_stat(stat="total_tests_display") }} tests as of
{{ total_stat(stat="measured_date") }}. Every claim is verifiable.
Clone the repo. Run the tests. If they fail, that's a bug — file it.*
