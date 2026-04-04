+++
title = "Ecosystem Inventory"
description = "Complete repository inventory across all three ecoPrimals organizations — every repo, its purpose, and current status."
date = 2026-03-31

[taxonomies]
primals = ["beardog", "barracuda", "biomeos", "bingocube", "coralreef", "loamspine", "nestgate", "petaltongue", "rhizocrypt", "skunkbat", "songbird", "squirrel", "sweetgrass", "toadstool"]
springs = ["airspring", "groundspring", "healthspring", "hotspring", "ludospring", "neuralspring", "primalspring", "wetspring"]
+++

# Ecosystem Inventory

**Last Updated**: March 31, 2026

Every repository across the three ecoPrimals organizations. All repositories (except whitePaper) are **scyBorg-licensed**: AGPL-3.0-or-later for code, ORC for game mechanics, CC-BY-SA 4.0 for creative/docs. All are intended to be fully public. Repos already on GitHub link directly; the rest are in the process of being source-published, with binaries available now via [plasmidBin](https://github.com/ecoPrimals/plasmidBin).

---

## ecoPrimals — Infrastructure & Primals (~21 repos)

The core organization. Contains all primals (the sovereign Rust binaries), infrastructure repos, and tooling.

### Foundation Primals (8)

These form the [NUCLEUS](@/architecture/NUCLEUS_ARCHITECTURE.md) deployment architecture.

| Primal | Domain | Status | Tests |
|--------|--------|--------|-------|
| 🐻🐕 bearDog | Cryptographic spine — Tower Atomic, Dark Forest, Pure Rust Tor | Source publishing in progress | 5,041 |
| 🎵🐦 songBird | Nervous system — TLS 1.3, O(n) discovery hub, 4-tier NAT | Source publishing in progress | 1,763 |
| 🪺🔒 nestGate | Data layer — content-addressed storage, ZFS, isomorphic IPC | Source publishing in progress | 1,474 |
| 🐸🍄 [toadStool](https://github.com/ecoPrimals/toadStool) | Compute layer — GPU/NPU/CPU dispatch, f64 discovery | **Public** | 1,000+ |
| 🐿️🧠 [squirrel](https://github.com/ecoPrimals/squirrel) | AI brain — vendor-agnostic MCP routing, sovereign inference | **Public** | 7,165 |
| 🌿🖥️ biomeOS | Conductor — Neural API, NUCLEUS composition, Dark Forest | Source publishing in progress | 661+ |
| 🪸🌊 [coralReef](https://github.com/ecoPrimals/coralReef) | Sovereign compiler — WGSL to native GPU, no LLVM/Mesa/vendor SDK | **Public** | 3,038 |
| 🐟⚡ [barraCuda](https://github.com/ecoPrimals/barraCuda) | Math engine — 800+ WGSL shaders, f64 science on consumer GPUs | **Public** | 3,348+ |

### Post-NUCLEUS Primals (5)

Higher-order capabilities that compose on the foundation. Active codebases, evolving toward full NUCLEUS integration.

| Primal | Domain | Status | Tests |
|--------|--------|--------|-------|
| 🌸👅 petalTongue | The face — 5-mode UniBin UI (desktop, TUI, web, headless, status) | Source publishing in progress | — |
| 🌱🔐 rhizoCrypt | Scratch pad — ephemeral DAG, 6 slice modes, dehydration to LoamSpine | Source publishing in progress | 509 |
| 🍯🌾 sweetGrass | Attribution — W3C PROV-O provenance, Braid model, fair credit | Source publishing in progress | 496 |
| 🪨📖 loamSpine | Fossil record — immutable ledger, Loam certificates, federation | Source publishing in progress | 416 |
| 🦨🦇 skunkBat | Immune system — metadata-only threat detection, graduated response | Source publishing in progress | — |

### Tooling & Infrastructure (8)

| Repository | Purpose | Status |
|------------|---------|--------|
| 🍞🧪 sourDough | Starter culture — scaffolds new primals, produces genomeBin packages | Source publishing in progress |
| 🎲🔒 bingoCube | Human trust bridge — BLAKE3 progressive reveal, visual/audio identity verification | Publishing soon |
| 🧪🤖 agentReagents | Agent chemistry — composable reagent patterns for sovereign AI agents | Publishing soon |
| ⚖️📊 benchScale | Scaling studies — cross-primal benchmarks, composition cost characterization | Publishing soon |
| 💧📡 [wateringHole](https://github.com/ecoPrimals/wateringHole) | Ecosystem communications, standards, glossary — shared dev context | **Public** |
| 🖨️🌐 [sporePrint](https://github.com/ecoPrimals/sporePrint) | This website — [primals.eco](https://primals.eco) | **Public** |
| 🧬📦 [plasmidBin](https://github.com/ecoPrimals/plasmidBin) | Binary distribution surface — genomeBins, ecoBins, metadata.toml | **Public** |
| 📄🔒 whitePaper | Research documentation — will bud public sub-repos over time | Private |

---

## syntheticChemistry — Science Validation (~10 repos)

All springs are public. Each spring validates one scientific domain through executable experiments with quantified checks.

### Springs (8)

| Spring | Domain | Status | Checks | Repo |
|--------|--------|-------|--------|------|
| 💧🔬 wetSpring | Microbiology, 16S, metagenomics | Active | 1,200+ | [syntheticChemistry/wetSpring](https://github.com/syntheticChemistry/wetSpring) |
| ♨️🧪 hotSpring | Physics, thermodynamics, Anderson localization | Active | 2,500+ | [syntheticChemistry/hotSpring](https://github.com/syntheticChemistry/hotSpring) |
| 🌬️💨 airSpring | Atmospheric, climate, fluid dynamics | Active | 800+ | [syntheticChemistry/airSpring](https://github.com/syntheticChemistry/airSpring) |
| 🧠⚡ neuralSpring | ML primitives, isomorphism, structure prediction | Active | 4,500+ | [syntheticChemistry/neuralSpring](https://github.com/syntheticChemistry/neuralSpring) |
| 🌍🪨 groundSpring | Geoscience, soil, hydrology | Active | 600+ | [syntheticChemistry/groundSpring](https://github.com/syntheticChemistry/groundSpring) |
| 🏥💊 healthSpring | PK/PD, microbiome, biosignal, drug discovery | Active | 795+ | [syntheticChemistry/healthSpring](https://github.com/syntheticChemistry/healthSpring) |
| 🎮🎲 ludoSpring | Game science, HCI, procedural generation | V30 | 1,692+ | [syntheticChemistry/ludoSpring](https://github.com/syntheticChemistry/ludoSpring) |
| 🧬♨️ primalSpring | Composition validation, deploy graphs, BYOB | Active | 303+ | [syntheticChemistry/primalSpring](https://github.com/syntheticChemistry/primalSpring) |

### Other (2)

| Repository | Purpose | Repo |
|------------|---------|------|
| 🦀🧠 rustChip | Neuromorphic hardware exploration (BrainChip Akida in Rust) | [syntheticChemistry/rustChip](https://github.com/syntheticChemistry/rustChip) |
| ⚡🔌 ionChannel | Inter-spring communication layer | syntheticChemistry/ionChannel |

### Archived

The following repos have been archived or are being relocated:

- **coralForge** → renamed to **helixVision**, moved to sporeGarden
- **blueFish** → moved to sporeGarden
- **agentReagents** (duplicate) → archived, canonical version in ecoPrimals
- **benchScale** (duplicate) → archived, canonical version in ecoPrimals

---

## sporeGarden — Products (3 repos)

User-facing products that compose primals into complete applications. Each product demonstrates that primal composition produces real, usable software.

| Product | What | Status | Repo |
|---------|------|--------|------|
| 🔮🕸️ esotericWebb | Cross-evolution CRPG — player state as DAG, save games as Loam certificates | **Public** | [sporeGarden/esotericWebb](https://github.com/sporeGarden/esotericWebb) |
| 🧬👁️ helixVision | Sovereign structure prediction — AlphaFold2/3 reimagined in pure Rust f64 | Moving to sporeGarden | sporeGarden/helixVision (pending) |
| 🐟🔵 blueFish | Sovereign data pipeline — NCBI/UniProt/PDB ingestion, format conversion | Moving from syntheticChemistry | sporeGarden/blueFish (pending) |

---

## By the Numbers

| Metric | Value |
|--------|-------|
| Total repositories | ~34 (21 ecoPrimals + 10 syntheticChemistry + 3 sporeGarden) |
| Foundation primals | 8 |
| Post-NUCLEUS primals | 5 |
| Meta/tooling | 1 (sourDough) + 3 publishing soon (bingoCube, agentReagents, benchScale) |
| Infrastructure repos | 4 (wateringHole, sporePrint, plasmidBin, whitePaper) |
| Science springs | 8 (7 domain + 1 meta-spring) |
| User-facing products | 3 (esotericWebb, helixVision, blueFish) |
| Public repos (today) | 4 primals + 8 springs + 3 infra + 1 product = **16** |
| Source publishing in progress | ~14 |
| License | **scyBorg** — AGPL-3.0-or-later (code) + ORC (game mechanics) + CC-BY-SA 4.0 (creative/docs) |
| Total validated checks | 16,000+ across all springs |
| Total primal tests | 25,000+ across all primals |

---

*This inventory is the ground truth for sporePrint. When in doubt about what exists, check here.
For deeper dives: [Primal Catalog](@/architecture/PRIMAL_CATALOG.md),
[Spring Catalog](@/architecture/SPRING_CATALOG.md),
[Deployment Model](@/architecture/DEPLOYMENT_MODEL.md).*
