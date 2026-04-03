+++
title = "sporePrint"
description = "Sovereign scientific computing. The science is executable, the infrastructure is inspectable, the claims can be reproduced by anyone with commodity hardware."
+++

🍄 The public record and verification portal for **ecoPrimals** — a sovereign
scientific computing ecosystem. The science is executable, the infrastructure
is inspectable, and every claim can be reproduced on commodity hardware.

---

## 🚀 Try It

**In 5 minutes, you can verify everything on this site:**

```bash
# Option 1: Build from source
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
git clone https://github.com/syntheticChemistry/wetSpring && cd wetSpring/barracuda
cargo test --workspace          # 1,443+ tests, 0 failures
cargo run --release --bin validate_anderson_3d   # exit 0 = pass
cargo deny check                # zero license violations, zero C dependencies
```

```bash
# Option 2: Pre-built guideStone artifact (no Rust required)
tar xf hotspring-guidestone-v0.7.0.tar.gz && cd validation/
./hotspring validate            # 59/59 pass, ~3 min, any x86_64 Linux
./hotspring benchmark           # characterize your hardware
```

If those commands run, the claims are verified. No institutional access.
No proprietary software. No cloud.

- 🧬 **Run real scientific pipelines locally** — genomics, protein structure,
  lattice QCD, pharmacometrics, precision agriculture, signal processing
- 📄 **Reproduce published results** — 175+ papers across 8 domains, each as a
  binary you can run and verify
- ⚡ **Use any GPU** — NVIDIA, AMD, Intel — no CUDA lock-in, no vendor toolchain
- 🔒 **Own your data and compute** — nothing leaves your machine, no cloud, no
  API keys, no institutional access required
- ✅ **Validate with guideStone** — self-verifying artifacts that prove their own
  correctness on any hardware, any architecture
- 📈 **Scale from one desktop to a lab cluster** — same code, same binaries

---

## 📊 The Numbers

| | |
|---|---|
| ✅ **20,695+** checks | Validation binaries across 8 scientific domains — exit 0 on pass |
| 📄 **175+** papers | Reproduced from peer-reviewed literature |
| ⚡ **$0.044** per run | Electricity cost for paper-parity lattice QCD (RTX 4070) |
| 📈 **9.9×** f64 uplift | Consumer GPUs via DF64/WebGPU vs CUDA native f64 |
| ⏱️ **27 days** | From first spring to 10,796+ checks across 5 domains |
| 🧪 **27,169+** tests | Combined across BarraCuda 🐟⚡, ToadStool 🐸🍄, and coralReef 🪸🌊 |
| 💰 **$15K** total hardware | The entire system runs on consumer hardware |
| 🔬 **5 substrates** | guideStone cross-validated: x86_64, aarch64, NVIDIA, AMD, Alpine |

---

## 🧭 Find Your Path

| | You are... | Start with |
|---|------------|-----------|
| 🎓 | **A faculty member or PI** evaluating this work | [For Faculty and PIs](@/audience/FOR_FACULTY_AND_PIS.md) |
| 🔬 | **A student or core facility** wanting to use it | [For Students and Core Facilities](@/audience/FOR_STUDENTS_AND_CORE_FACILITIES.md) |
| 🛠️ | **A hardware builder or hobbyist** with a GPU | [For Hardware Builders and Hobbyists](@/audience/FOR_HARDWARE_BUILDERS_AND_HOBBYISTS.md) |
| 📋 | **A compliance officer, IRB, or legal reviewer** | [For Compliance and Institutional Review](@/audience/FOR_COMPLIANCE_AND_INSTITUTIONAL_REVIEW.md) |
| ⚛️ | **A physicist or computational scientist** | [guideStone](@/guidestone/_index.md) — the self-leveling benchmark and deployment artifact |

Not sure? Read the [Capability Parity Brief](@/audience/CAPABILITY_PARITY_BRIEF.md) —
a direct comparison against proprietary tools across 8 scientific domains.

---

## 🏗️ The Three Organizations

ecoPrimals is distributed across three GitHub organizations, each with a
distinct role:

| | Organization | Role | Contains | GitHub |
|---|-------------|------|----------|--------|
| 🔧 | **ecoPrimals** | Infrastructure | 14 primals including barraCuda 🐟⚡, toadStool 🐸🍄, coralReef 🪸🌊 + infra repos · [Full catalog](@/architecture/PRIMAL_CATALOG.md) | [github.com/ecoPrimals](https://github.com/ecoPrimals) |
| 🧪 | **syntheticChemistry** | Science validation | 8 springs: wetSpring 💧♨️, hotSpring 🔥♨️, airSpring 🌬️♨️, and 5 more · [Full catalog](@/architecture/SPRING_CATALOG.md) | [github.com/syntheticChemistry](https://github.com/syntheticChemistry) |
| 🌱 | **sporeGarden** | Products | esotericWebb 🔮🕸️, helixVision 🧬👁️, and future tools for scientists and creatives | [github.com/sporeGarden](https://github.com/sporeGarden) |

**Primals** build capabilities. **Springs** validate that those capabilities
produce correct science. **Products** compose validated capabilities into tools
people use.

---

## 🌿 The Ecosystem at a Glance

### ⚙️ How It Connects

**Springs** are validation environments — Rust binaries that reproduce
published science. **Primals** are infrastructure components — the math
engine, the hardware orchestrator, the compiler. **guideStone** is the
verification class — it certifies that the output is reproducible,
self-verifying, and tolerance-documented. Everything is pure Rust,
AGPL-3.0, zero C dependencies.

### ♨️ Seven Science Springs (All Public, AGPL-3.0)

| | Spring | Domain | Repository | Checks |
|---|--------|--------|-----------|:------:|
| 💧 | wetSpring | Life science, microbiome, quorum sensing, field genomics | [syntheticChemistry/wetSpring](https://github.com/syntheticChemistry/wetSpring) | 5,707+ |
| 🌬️ | airSpring | Precision agriculture, ET₀, soil hydrology, phenology | [syntheticChemistry/airSpring](https://github.com/syntheticChemistry/airSpring) | 3,123+ |
| 🧠 | neuralSpring | ML primitives, reservoir computing, spectral analysis | [syntheticChemistry/neuralSpring](https://github.com/syntheticChemistry/neuralSpring) | 4,500+ |
| 🔥 | hotSpring | Plasma physics, lattice QCD, GPU sovereign compute | [syntheticChemistry/hotSpring](https://github.com/syntheticChemistry/hotSpring) | 664+ |
| ⛰️ | groundSpring | Uncertainty quantification, noise, spectral theory | [syntheticChemistry/groundSpring](https://github.com/syntheticChemistry/groundSpring) | 535+ |
| ❤️ | healthSpring | Human health, PK/PD, microbiome, biosignal, drug discovery | [syntheticChemistry/healthSpring](https://github.com/syntheticChemistry/healthSpring) | 474+ |
| 🎮 | ludoSpring | Game science, HCI, provenance, distributed compute | [syntheticChemistry/ludoSpring](https://github.com/syntheticChemistry/ludoSpring) | 1,692+ |
| | **Total** | | | **16,695+** |

### 🔧 Three Public Infrastructure Primals (All AGPL-3.0)

| | Primal | Domain | Repository |
|---|--------|--------|-----------|
| 🐸🍄 | ToadStool | Universal compute orchestration — CPU, GPU, NPU, edge | [ecoPrimals/toadStool](https://github.com/ecoPrimals/toadStool) |
| 🐟⚡ | BarraCuda | Pure mathematics — 806+ WGSL f64 shaders, precision strategy | [ecoPrimals/barraCuda](https://github.com/ecoPrimals/barraCuda) |
| 🪸🌊 | coralReef | Sovereign WGSL→native GPU compiler | [ecoPrimals/coralReef](https://github.com/ecoPrimals/coralReef) |

### 🪨 guideStone — The Verification Class (gen4)

guideStone certifies that an artifact produces reproducible, self-verifying
output. Five properties: deterministic, reference-traceable, self-verifying,
environment-agnostic, tolerance-documented. The first deployment artifact
(`hotSpring-guideStone-v0.7.0`) was validated across 5 substrates and 2
architectures with 40/40 bit-identical cross-substrate observables.

[Read more about guideStone →](@/guidestone/_index.md)

---

## 🍄 Why "sporePrint"

A spore print is how mycologists identify species they have never seen before.
You press the cap to paper and leave it overnight. In the morning: the permanent
record of what the organism is, what it can produce, and how to grow it yourself.

This site is the spore print for ecoPrimals. The permanent, public, verifiable
impression of a sovereign scientific computing ecosystem. Clone it. Run it.
Verify it. Grow from it.

**Want to build your own?**
[How to Start a Spring](@/methodology/HOW_TO_START_A_SPRING.md) —
you need domain knowledge, focus, patience, and a used GPU. No CS degree required.
[Knowledge Commons Targets](@/methodology/KNOWLEDGE_COMMONS_TARGETS.md) —
9 domains ready now with existing primals and public data.

---

*20,695+ checks. 175+ papers. 8 domains. Consumer hardware. One system. AGPL-3.0.*
*🍄 The science is not a claim — it is executable evidence.*

**License:** CC-BY-SA 4.0 (documents) · AGPL-3.0-or-later (code) · **Developer:** ecoPrimal — human + synthetic intelligence
