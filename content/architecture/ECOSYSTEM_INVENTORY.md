+++
title = "Ecosystem Inventory"
weight = 22
description = "Complete repository inventory across all three ecoPrimals organizations — every repo, its purpose, and current status."
date = 2026-06-20

[taxonomies]
primals = ["beardog", "barracuda", "biomeos", "bingocube", "coralreef", "loamspine", "nestgate", "petaltongue", "rhizocrypt", "skunkbat", "songbird", "squirrel", "sweetgrass", "toadstool"]
springs = ["airspring", "groundspring", "healthspring", "hotspring", "ludospring", "neuralspring", "primalspring", "wetspring"]
+++

**Last Updated**: June 20, 2026

Every repository across the three {{ entity(name="ecoprimals") }} organizations. All repositories (except {{ entity(name="whitepaper") }}) are **{{ entity(name="scyborg") }}-licensed**: AGPL-3.0-or-later for code, ORC for game mechanics, CC-BY-SA 4.0 for creative/docs. All are intended to be fully public. Repos already on GitHub link directly; the rest are in the process of being source-published, with binaries available now via [plasmidBin](https://github.com/ecoPrimals/plasmidBin).

---

## ecoPrimals — Infrastructure & Primals (~21 repos)

The core organization. Contains all primals (the sovereign Rust binaries), infrastructure repos, and tooling.

### Foundation Primals (8)

These form the [NUCLEUS](@/architecture/NUCLEUS_ARCHITECTURE.md) deployment architecture.

| Primal | Domain | Status | Tests |
|--------|--------|--------|-------|
| 🐻🐕 bearDog | Cryptographic spine — {{ entity(name="toweratomic") }}, {{ entity(name="darkforest") }}, Pure Rust Tor | Source publishing in progress | 5,041 |
| 🎵🐦 [songBird](https://github.com/ecoPrimals/songBird) | Nervous system — TLS 1.3, O(n) discovery hub, 4-tier NAT | **Public** | 1,763 |
| 🪺🔒 [nestGate](https://github.com/ecoPrimals/nestGate) | Data layer — content-addressed storage, ZFS, isomorphic IPC | **Public** | 1,474 |
| 🐸🍄 [toadStool](https://github.com/ecoPrimals/toadStool) | Compute layer — GPU/NPU/CPU dispatch, f64 discovery | **Public** | 1,000+ |
| 🐿️🧠 [squirrel](https://github.com/ecoPrimals/squirrel) | AI brain — vendor-agnostic MCP routing, sovereign inference | **Public** | 7,165 |
| 🌿🖥️ [biomeOS](https://github.com/ecoPrimals/biomeOS) | Conductor — {{ entity(name="neuralapi") }}, {{ entity(name="nucleus") }} composition, {{ entity(name="darkforest") }} | **Public** | 8,351 |
| 🪸🌊 [coralReef](https://github.com/ecoPrimals/coralReef) | Sovereign compiler — WGSL to native GPU, no LLVM/Mesa/vendor SDK | **Public** | 3,038 |
| 🐟⚡ [barraCuda](https://github.com/ecoPrimals/barraCuda) | Math engine — 800+ WGSL shaders, f64 science on consumer GPUs | **Public** | 3,348+ |

### Post-NUCLEUS Primals (5)

Higher-order capabilities that compose on the foundation. Active codebases, evolving toward full {{ entity(name="nucleus") }} integration.

| Primal | Domain | Status | Tests |
|--------|--------|--------|-------|
| 🌸👅 [petalTongue](https://github.com/ecoPrimals/petalTongue) | The face — 5-mode {{ entity(name="unibin") }} UI (desktop, TUI, web, headless, status) | **Public** | 6,040 |
| 🌱🔐 [rhizoCrypt](https://github.com/ecoPrimals/rhizoCrypt) | Scratch pad — ephemeral DAG, 6 slice modes, dehydration to {{ entity(name="loamspine") }} | **Public** | 509 |
| 🍯🌾 [sweetGrass](https://github.com/ecoPrimals/sweetGrass) | Attribution — W3C PROV-O provenance, Braid model, fair credit | **Public** | 496 |
| 🪨📖 [loamSpine](https://github.com/ecoPrimals/loamSpine) | Fossil record — immutable ledger, Loam certificates, federation | **Public** | 416 |
| {{ entity(name="skunkbat") }} | Immune system — metadata-only threat detection, graduated response | Source publishing in progress | — |

### Tooling & Infrastructure (9)

| Repository | Purpose | Status |
|------------|---------|--------|
| 🍞🧪 [sourDough](https://github.com/ecoPrimals/sourDough) | Starter culture — scaffolds new primals, produces {{ entity(name="genomebin") }} packages | **Public** |
| 🎲🧊 [bingoCube](https://github.com/ecoPrimals/bingoCube) | Human trust bridge — BLAKE3 progressive reveal, visual/audio identity verification | **Public** |
| 🧫🔬 [cellMembrane](https://github.com/ecoPrimals/cellMembrane) | Deployment layer — K-Derm topology, gate enrollment, NUCLEUS systemd, cascade pipeline | **Public** (680 tests) |
| 🧪🤖 agentReagents | Agent chemistry — composable reagent patterns for sovereign AI agents | Publishing soon |
| ⚖️📊 benchScale | Scaling studies — cross-primal benchmarks, composition cost characterization | Publishing soon |
| 💧📡 [wateringHole](https://github.com/ecoPrimals/wateringHole) | Ecosystem communications, standards, glossary — shared dev context | **Public** |
| 🖨️🌐 [sporePrint](https://github.com/ecoPrimals/sporePrint) | This website — [primals.eco](https://primals.eco) | **Public** |
| 🧬📦 [plasmidBin](https://github.com/ecoPrimals/plasmidBin) | Binary distribution surface — genomeBins, ecoBins, metadata.toml | **Public** |
| 📄🔒 {{ entity(name="whitepaper") }} | Research documentation — will bud public sub-repos over time | Private |

---

## syntheticChemistry — Science Validation (~10 repos)

All springs are public. Each spring validates one scientific domain through executable experiments with quantified checks.

### Springs (8)

| Spring | Domain | Status | Checks | Repo |
|--------|--------|-------|--------|------|
| 💧🔬 {{ entity(name="wetspring") }} | Microbiology, 16S, metagenomics | Active | 1,200+ | [syntheticChemistry/wetSpring](https://github.com/syntheticChemistry/wetSpring) |
| ♨️🧪 {{ entity(name="hotspring") }} | Physics, thermodynamics, Anderson localization | Active | 2,500+ | [syntheticChemistry/hotSpring](https://github.com/syntheticChemistry/hotSpring) |
| 🌬️💨 {{ entity(name="airspring") }} | Atmospheric, climate, fluid dynamics | Active | 800+ | [syntheticChemistry/airSpring](https://github.com/syntheticChemistry/airSpring) |
| 🧠⚡ {{ entity(name="neuralspring") }} | ML primitives, isomorphism, structure prediction | Active | 4,500+ | [syntheticChemistry/neuralSpring](https://github.com/syntheticChemistry/neuralSpring) |
| 🌍🪨 {{ entity(name="groundspring") }} | Geoscience, soil, hydrology | Active | 600+ | [syntheticChemistry/groundSpring](https://github.com/syntheticChemistry/groundSpring) |
| 🏥💊 {{ entity(name="healthspring") }} | PK/PD, microbiome, biosignal, drug discovery | Active | 795+ | [syntheticChemistry/healthSpring](https://github.com/syntheticChemistry/healthSpring) |
| 🎮🎲 {{ entity(name="ludospring") }} | Game science, HCI, procedural generation | V30 | 1,692+ | [syntheticChemistry/ludoSpring](https://github.com/syntheticChemistry/ludoSpring) |
| {{ entity(name="primalspring") }} | Composition validation, deploy graphs, {{ entity(name="byob") }} | Active | 959 (85 scenarios) | [syntheticChemistry/primalSpring](https://github.com/syntheticChemistry/primalSpring) |

### Infrastructure (3)

| Repository | Purpose | Status | Tests | Repo |
|------------|---------|--------|-------|------|
| {{ entity(name="rustchip") }} | Pure Rust Akida NPU driver — standalone extraction from {{ entity(name="toadstool") }} neuromorphic layer | Active | 367 | [syntheticChemistry/rustChip](https://github.com/syntheticChemistry/rustChip) |
| ⚡🔌 ionChannel | Inter-spring communication layer | — | — | syntheticChemistry/ionChannel |

**{{ entity(name="rustchip") }} capabilities:**

- **VFIO backend**: Pure Rust container/group/device lifecycle, BAR mapping, DMA — no kernel module, user-level via udev
- **FBZ parser**: Reverse-engineered Akida model format (varint + Snappy + zero-padding probe)
- **Silicon model**: AKD1000/AKD1500 register map, 80-NPU mesh discovery, 10 MB SRAM
- **Novel systems**: HybridESN, multi-tenancy (7 programs), online evolution (136 gen/s), temporal PUF, adaptive sentinel
- **Hardware validation**: 10 BEYOND_SDK discoveries, 5,978 live calls in {{ entity(name="hotspring") }} lattice QCD
- **Compute trio integration**: Output feeds {{ entity(name="barracuda") }} shaders via `&[f32]`. VFIO patterns mirror {{ entity(name="coralreef") }} ember/glowplug architecture
- **License**: {{ entity(name="scyborg") }} triple (AGPL + CC-BY-SA + ORC) with symbiotic exception for hardware partners
- **Science page**: [Neuromorphic Sovereign Driver](/science/26-neuromorphic-sovereign-driver/)

### Archived

The following repos have been archived or are being relocated:

- **coralForge** → renamed to **{{ entity(name="helixvision") }}**, moved to {{ entity(name="sporegarden") }}
- **blueFish** → moved to {{ entity(name="sporegarden") }}
- **agentReagents** (duplicate) → archived, canonical version in {{ entity(name="ecoprimals") }}
- **benchScale** (duplicate) → archived, canonical version in {{ entity(name="ecoprimals") }}

---

## sporeGarden — Products (3 repos)

User-facing products that compose primals into complete applications. Each product demonstrates that primal composition produces real, usable software.

| Product | What | Status | Repo |
|---------|------|--------|------|
| {{ entity(name="esotericwebb") }} | Cross-evolution CRPG — player state as DAG, save games as Loam certificates | **Public** | [sporeGarden/esotericWebb](https://github.com/sporeGarden/esotericWebb) |
| {{ entity(name="helixvision") }} | Sovereign structure prediction — AlphaFold2/3 reimagined in pure Rust f64 | Moving to {{ entity(name="sporegarden") }} | sporeGarden/helixVision (pending) |
| 🐟🔵 blueFish | Sovereign data pipeline — NCBI/UniProt/PDB ingestion, format conversion | Moving from {{ entity(name="syntheticchemistry") }} | sporeGarden/blueFish (pending) |

---

## By the Numbers

| Metric | Value |
|--------|-------|
| Total repositories | ~37 (22 {{ entity(name="ecoprimals") }} + 12 {{ entity(name="syntheticchemistry") }} + 3 {{ entity(name="sporegarden") }}) |
| Foundation primals | 8 |
| Post-{{ entity(name="nucleus") }} primals | 5 |
| Meta/tooling | 3 ({{ entity(name="sourdough") }}, {{ entity(name="bingocube") }}, agentReagents/benchScale publishing soon) |
| Infrastructure repos | 5 ({{ entity(name="wateringhole") }}, {{ entity(name="sporeprint") }}, {{ entity(name="plasmidbin") }}, cellMembrane, {{ entity(name="whitepaper") }}) |
| Science springs | 8 (7 domain + 1 meta-spring) |
| User-facing products | 3 ({{ entity(name="esotericwebb") }}, {{ entity(name="helixvision") }}, blueFish) |
| Public repos (today) | 13 primals + 8 springs + 3 infra + 1 product = **25** |
| Source publishing in progress | ~5 (bearDog, skunkBat, agentReagents, benchScale, whitePaper) |
| License | **{{ entity(name="scyborg") }}** — AGPL-3.0-or-later (code) + ORC (game mechanics) + CC-BY-SA 4.0 (creative/docs) |
| Total Rust LOC | 3,209,814 (2.6M primals + 614K springs, measured via tokei April 2026) |
| WGSL shaders | 952 files, 73,838 lines |
| Total test functions | 107,143 (98K primals + 9K springs) |
| C dependencies | Zero (entire ecosystem) |

---

*This inventory is the ground truth for {{ entity(name="sporeprint") }}. When in doubt about what exists, check here.
For deeper dives: [Primal Catalog](@/architecture/PRIMAL_CATALOG.md),
[Spring Catalog](@/architecture/SPRING_CATALOG.md),
[Deployment Model](@/architecture/DEPLOYMENT_MODEL.md).*
