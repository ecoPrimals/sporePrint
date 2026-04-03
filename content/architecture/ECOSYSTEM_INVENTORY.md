+++
title = "Ecosystem Inventory"
description = "Complete repository inventory across all three ecoPrimals organizations — every repo, its purpose, and current status."
date = 2026-03-31
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
| 🐻🐕 bearDog | Cryptography, identity, Dark Forest | Source publishing in progress | 5,041 |
| 🎵🐦 songBird | Networking, mesh, discovery | Source publishing in progress | 1,763 |
| 🪺🔒 nestGate | Content-addressed storage, CAS | Source publishing in progress | 1,474 |
| 🐸🍄 [toadStool](https://github.com/ecoPrimals/toadStool) | Universal compute orchestration | **Public** | 1,000+ |
| 🐿️🧠 [squirrel](https://github.com/ecoPrimals/squirrel) | AI coordination, MCP, inference | **Public** | 7,165 |
| 🌿🖥️ biomeOS | Orchestration, Neural API, NUCLEUS | Source publishing in progress | 661+ |
| 🪸🌊 [coralReef](https://github.com/ecoPrimals/coralReef) | Shader compiler, GPU dispatch | **Public** | 3,038 |
| 🐟⚡ [barraCuda](https://github.com/ecoPrimals/barraCuda) | Pure math, WGSL f64 shaders | **Public** | 3,348+ |

### Post-NUCLEUS Primals (5)

Higher-order capabilities that compose on the foundation. Active codebases, evolving toward full NUCLEUS integration.

| Primal | Domain | Status | Tests |
|--------|--------|--------|-------|
| 🌸👅 petalTongue | Content delivery, visualization, accessibility | Source publishing in progress | — |
| 🌱🔐 rhizoCrypt | Ephemeral DAG workspace, Merkle verification | Source publishing in progress | 509 |
| 🍯🌾 sweetGrass | Attribution, citation, provenance tracking | Source publishing in progress | 496 |
| 🪨📖 loamSpine | Immutable linear history, certificates | Source publishing in progress | 416 |
| 🦨🦇 skunkBat | Defensive security, threat detection | Source publishing in progress | — |

### Tooling & Infrastructure (8)

| Repository | Purpose | Status |
|------------|---------|--------|
| 🍞🧪 sourDough | Scaffolding & packaging CLI — generates primals, produces genomeBin artifacts | Source publishing in progress |
| 🎲🔒 bingoCube | Human-verifiable cryptographic commitment (BLAKE3 progressive reveal) | Publishing soon |
| 🧪🤖 agentReagents | AI agent toolkit — reagent patterns for sovereign AI composition | Publishing soon |
| ⚖️📊 benchScale | Benchmark & performance characterization, cross-primal scaling | Publishing soon |
| 💧📡 [wateringHole](https://github.com/ecoPrimals/wateringHole) | Ecosystem communications, standards, glossary — shared dev context | **Public** |
| 🖨️🌐 [sporePrint](https://github.com/ecoPrimals/sporePrint) | This website — [primals.eco](https://primals.eco) | **Public** |
| 🧬📦 [plasmidBin](https://github.com/ecoPrimals/plasmidBin) | Binary distribution surface — genomeBins, ecoBins, metadata.toml | **Public** |
| 📄🔒 whitePaper | Research documentation — will bud public sub-repos over time | Private |

---

## syntheticChemistry — Science Validation (~10 repos)

All springs are public. Each spring validates one scientific domain through executable experiments with quantified checks.

### Springs (8)

| Spring | Domain | Grade | Checks | Repo |
|--------|--------|-------|--------|------|
| 💧🔬 wetSpring | Microbiology, 16S, metagenomics | V12 | 1,200+ | [syntheticChemistry/wetSpring](https://github.com/syntheticChemistry/wetSpring) |
| ♨️🧪 hotSpring | Physics, thermodynamics, Anderson localization | V20 | 2,500+ | [syntheticChemistry/hotSpring](https://github.com/syntheticChemistry/hotSpring) |
| 🌬️💨 airSpring | Atmospheric, climate, fluid dynamics | V8 | 800+ | [syntheticChemistry/airSpring](https://github.com/syntheticChemistry/airSpring) |
| 🧠⚡ neuralSpring | ML primitives, isomorphism, structure prediction | Multi-phase | 4,500+ | [syntheticChemistry/neuralSpring](https://github.com/syntheticChemistry/neuralSpring) |
| 🌍🪨 groundSpring | Geoscience, soil, hydrology | V6 | 600+ | [syntheticChemistry/groundSpring](https://github.com/syntheticChemistry/groundSpring) |
| 🏥💊 healthSpring | PK/PD, microbiome, biosignal, drug discovery | V27 | 795+ | [syntheticChemistry/healthSpring](https://github.com/syntheticChemistry/healthSpring) |
| 🎮🎲 ludoSpring | Game science, HCI, procedural generation | V30 | 1,692+ | [syntheticChemistry/ludoSpring](https://github.com/syntheticChemistry/ludoSpring) |
| 🧬♨️ primalSpring | Composition validation, deploy graphs, BYOB | Phase 13 | 303+ | [syntheticChemistry/primalSpring](https://github.com/syntheticChemistry/primalSpring) |

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
| 🔮🕸️ esotericWebb | Cross-evolution CRPG — composes rhizoCrypt, loamSpine, sweetGrass via JSON-RPC | **Public** | [sporeGarden/esotericWebb](https://github.com/sporeGarden/esotericWebb) |
| 🧬👁️ helixVision | Sovereign structure prediction — AlphaFold2/3 in pure Rust f64, formerly coralForge | Moving to sporeGarden | sporeGarden/helixVision (pending) |
| 🐟🔵 blueFish | Sovereign data pipeline/ETL — NCBI integration, format conversion | Moving from syntheticChemistry | sporeGarden/blueFish (pending) |

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
