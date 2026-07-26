+++
title = "For Homelabbers and LLM Enthusiasts — Start Using the Stack"
description = "You run your own hardware. You prompt your own models. Here's an ecosystem of 15 sovereign Rust programs that does real science on commodity GPUs — and how to start using it."
weight = 6
date = 2026-07-14

[taxonomies]
primals = ["barracuda", "toadstool", "biomeos", "songbird", "squirrel", "beardog"]
springs = ["hotspring", "neuralspring"]

[extra]
maturity = "live"

[[extra.companions]]
url = "/architecture/economics/"
title = "Ecosystem Economics"
relation = "extends"
label = "The flywheel — how your contributions become geological record"

[[extra.companions]]
url = "/architecture/ecosystem-architecture/"
title = "Ecosystem Architecture"
relation = "architecture"
label = "The full architectural model — 15 primals, NUCLEUS, bonding"

[[extra.companions]]
url = "/methodology/conversation-constraint/"
title = "Conversation Constraint"
relation = "methodology"
label = "Zero human-written code — how AI built the whole thing"
+++

**Questions? A human reads and responds to every message at [eco.primal@pm.me](mailto:eco.primal@pm.me).**

---

## What This Is

{{ total_stat(stat="total_loc_display") }} lines of Rust. {{ total_stat(stat="total_tests_display") }} tests. {{ total_stat(stat="wgsl_lines_display") }} lines of GPU shader code. 15 composable programs. 8 validation domains. Running science-grade compute on consumer GPUs through Vulkan/WGSL — no CUDA, no cloud, no vendor lock-in.

All AGPL-3.0-or-later. Self-hosted. Sovereign.

If you run a homelab, this is what you've been building toward without knowing it.

---

## For r/homelab — Your Hardware Is Already Enough

### What You Already Have

| Your Hardware | What It Runs |
|--------------|-------------|
| Any x86_64 Linux box | NUCLEUS — full composition of 15 primals |
| Any NVIDIA GPU (GTX 1060+) | {{ entity(name="barracuda") }} — science-grade compute via Vulkan |
| Any NVMe/SSD | {{ entity(name="nestgate") }} — content-addressed storage with BLAKE3 integrity |
| WireGuard already set up? | {{ entity(name="songbird") }} federation already speaks your language |
| Akida/Coral NPU? | {{ entity(name="squirrel") }} + nautilus — neuromorphic reservoir computing |

### What You Get

Your tower becomes a **gate** in the sovereign mesh. It gets a cryptographic identity ({{ entity(name="beardog") }} Ed25519), joins the federation ({{ entity(name="songbird") }}), and can run validated science: protein structure prediction, lattice QCD, molecular dynamics, metagenomics, PFAS analytical chemistry — all on your own hardware.

Every computation your gate performs is BLAKE3-hashed and provenance-tracked. Your hardware's contribution is in the geological record. Not a thank-you email — a cryptographic attestation.

### Why You'd Bother

- **Real science runs on your hardware.** Not benchmarks. Not demos. 175+ peer-reviewed papers reproduced computationally, validated to published tolerances.
- **Zero cloud dependency.** The inner membrane has no external dependencies. Your gate works on an airgapped network.
- **Musl-static binaries.** `scp` a binary, `chmod +x`, run. No package manager, no container runtime required.
- **Composition, not configuration.** NUCLEUS is a composition of programs, not a YAML file. Add or remove primals by starting or stopping binaries.

---

## For LLM Enthusiasts — The AI Angle

### How This Was Built

Zero human-written code in {{ total_stat(stat="total_loc_display") }} lines of Rust.

Not an exaggeration. Not a "mostly AI" caveat. The human has a microbiology background and a data science degree. The human chose Rust *because* they didn't know it — forcing every interaction to stay in conversation with AI assistants.

The methodology is called **K-NOME** (Knowledge-Numeric Orchestrated Mentoring Ecosystem). The human mentors intent. The AI implements. The friction between them produces working software. 13+ months. 3-6 machines running parallel AI conversations. Every line of code emerged from conversation.

If you care about AI-assisted development, this is the largest existence proof that it works at scale.

### The Squirrel Vision

{{ entity(name="squirrel") }} is the AI integration primal. The long-term intent is to expose a squirrel instance as the **site curator** for sporePrint — an AI that knows the full topology of the ecosystem and can guide visitors, answer questions, and surface relevant content.

This is not a chatbot bolted onto a static site. It's a primal that runs locally, has access to the entity graph, the content manifest, the provenance DAGs, and the sweetGrass attribution braids. It knows what exists because it's part of the system that certifies what exists.

**Status**: architectural. The primal exists, the API surface is designed. The curation role is a near-term goal.

### What You Can Do Now

- **Read the methodology**: [K-NOME Programming](/methodology/k-nome-programming/) explains the conversational development method
- **See the prompts**: [The Prompt Bank](/methodology/prompt-bank/) — real prompts from 13 months of K-NOME development
- **Try the science**: The [Lab](/lab/) has 130+ pages of live validation results — every one generated by AI-assisted code running on consumer hardware

---

## Getting Started (Practical)

### Step 1: Browse the Evidence

Start at the [Evidence Snapshot](/architecture/evidence-snapshot/) — a one-page summary of what the ecosystem can prove today.

### Step 2: Pick a Domain

| If you're interested in... | Start here |
|--------------------------|-----------|
| GPU compute | [barraCuda](/primals/barracuda/) — vendor-agnostic WGSL compute |
| Cryptography | [BearDog](/primals/beardog/) — pure Rust TLS/signing |
| Bioinformatics | [wetSpring](/springs/wetspring/) — sovereign 16S pipeline |
| Protein structure | [coralForge](/products/coralforge/) — AlphaFold primitives |
| Game science | [ludoSpring](/springs/ludospring/) — evolutionary game theory |
| Neuromorphic | [nautilus](/products/nautilus/) — reservoir computing + NPU |

### Step 3: Deploy a Gate

When the deployment pathway is formalized, you'll download a musl-static binary, configure your gate name and WireGuard peer, and start NUCLEUS. Your tower joins the mesh. Your GPU starts doing science.

---

## The Flywheel Invitation

The ecosystem's economics are simple: contributions (hardware, money, effort) get {{ entity(name="sweetgrass") }}-attributed. Your gate's BLAKE3 hashes are in the provenance chain of every computation it performs. That attribution compounds.

The preferred contribution: **run your own mesh to prove ours.** A recycled tower, a NUC, whatever fits — deploy NUCLEUS, connect via federation, and your hardware becomes part of the sovereign compute fabric.

We'd rather you prove the architecture works than send money. Money covers the metabolic cost. Your gate proves the thesis.

See [Ecosystem Economics](/architecture/economics/) for the full model.

---

*You already own the hardware. You already run your own infrastructure. The only thing missing is the scientific workloads that justify the electricity bill. We have those.*
