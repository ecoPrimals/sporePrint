+++
title = "ecoPrimals Spring Catalog: Status, Science, and Evolution"
weight = 21
description = "All 9 springs — 7 science domains + neuromorphic hardware + meta-validation — checks, papers reproduced, cross-spring data flow"
date = 2026-03-31

[taxonomies]
primals = ["barracuda", "biomeos", "coralreef", "loamspine", "nestgate", "petaltongue", "rhizocrypt", "squirrel", "sweetgrass", "toadstool"]
springs = ["airspring", "groundspring", "healthspring", "hotspring", "ludospring", "neuralspring", "primalspring", "wetspring"]

[extra]
domain = "Architecture"
maturity = "reproduced"
+++

**Status**: Working paper
**Lineage**: Science validation companion to `PRIMAL_CATALOG.md`
**Last Updated**: March 31, 2026
**License**: **{{ entity(name="scyborg") }}** — AGPL-3.0-or-later (code) + ORC (game mechanics) + CC-BY-SA 4.0 (creative/docs)

---

## At a Glance

{{ total_stat(stat="total_springs") }} springs, each validating a scientific domain on sovereign hardware. Together: {{ total_stat(stat="validation_checks") }} quantitative checks, {{ total_stat(stat="papers_reproduced") }} peer-reviewed papers reproduced, 15 researchers across 9 departments. If the springs pass, the infrastructure works for real science.

---

## Abstract

The primals prove that Rust can build a sovereign computing ecosystem. The springs prove that sovereign computing can **reproduce published, peer-reviewed science** — and in some cases, do it faster, cheaper, and more transparently than the institutional tools it replaces.

The name is ecological: springs feed the ecosystem. Each spring produces validated kernels that flow into the primal infrastructure, just as geological springs feed rivers that sustain ecosystems. But springs are also *acceptance tests* — not for the science (which is already published and peer-reviewed), but for the infrastructure. {{ entity(name="barracuda") }} claims "Pure Rust GPU compute can replace the Python scientific stack." That claim requires evidence from every scientific domain the ecosystem intends to serve. A single-domain validation would prove the kernels work for plasma physics, not that the approach generalizes.

Every spring follows the same phased validation protocol: **Phase 0** (Python control — reproduce published results in the original language), **Phase 1** (Rust — cross-validate against Python within 1e-5), **Phase 2** (GPU — validate GPU output against both), **Phase 3+** (extensions — real data, cross-spring connections, published paper reproductions). Every check is automated and binary: pass or fail, no subjective "looks about right."

Each spring is grounded in published, peer-reviewed work. The published papers define the acceptance criteria — the springs reproduce their results independently, in Rust, with automated cross-validation. This is replication with rigor and full provenance: the science is already established; the question is whether a pure Rust infrastructure can reproduce it faithfully.

- **Science Springs** (§1): Seven domain springs covering physics, agriculture, biology, chemistry, geophysics, ML, health, and game science. All are public repositories under the {{ entity(name="syntheticchemistry") }} GitHub organization.
- **Meta-Spring** (§1.8): {{ entity(name="primalspring") }} 🧬♨️ validates primal composition, deploy graphs, and cross-gate bonding rather than a scientific domain.
- **The Spring Network** (§2): How springs connect to each other, to the primals, and to the {{ entity(name="basecamp") }} papers that are the scientific fruit of the methodology.

---

## 1. The Springs

---

### 1.1 hotSpring — Computational Plasma Physics, Lattice QCD, Spectral Theory

**Domain**: Dense plasmas, nuclear structure, molecular dynamics, lattice QCD, spectral theory, neuromorphic computing
{{ entity_metrics(name="hotspring") }}  
**Checks**: 697+ tests, 78 binaries, 62 WGSL shaders  
**Reproduces work by**: Michael Murillo (CMSE, MSU), Alexei Bazavov (CMSE + Physics, MSU), Ilya Kachkovskiy (Math, MSU), Rika Anderson (Biology, Carleton)  
**Repository**: [syntheticChemistry/hotSpring](https://github.com/syntheticChemistry/hotSpring)

{{ entity(name="hotspring") }} is the primary GPU science driver — the spring that proves {{ entity(name="barracuda") }} can do first-principles computational physics on consumer hardware. It is the most mature spring because physics has the least room to hide: 0.000% energy drift or the simulation is wrong.

**The headline results**: Sarkas Yukawa MD at paper parity (N=10,000, 80k steps) on a $600 RTX 4070 for $0.044 in electricity. The full AME2020 nuclear dataset (2,042 nuclei — 39× the published paper) on a single consumer GPU. Lattice QCD production β-scans (32⁴, 12 temperatures) resolving the deconfinement transition on a $500 RTX 3090 for $0.58. DF64 delivers ~14-digit precision on FP32 cores (measured: 2,130 matmul/sec on RTX 3090). Phase 0 discovered and fixed 5 silent bugs in the upstream Sarkas code — the control itself improved the science.

**What the constraint revealed**: Eliminating CUDA forced Vulkan, which exposed `SHADER_F64` on consumer GPUs. Eliminating vendor compilers forced {{ entity(name="coralreef") }}, which now compiles 93/93 cross-spring WGSL shaders to native GPU binaries. A $300 Akida NPU runs ESN inference at 2.8μs/step — 1,000× faster than GPU for streaming workloads, 9,017× less energy for transport predictions.

| Phase | Key Result |
|-------|------------|
| A–E (MD) | Python → Rust → GPU → f64 → paper parity. 0.000% energy drift. $0.044 electricity. |
| F (Nuclear EOS) | 2,042 nuclei AME2020 on consumer GPU. 478× speedup, 44.8× energy reduction. |
| Lattice QCD | SU(3) HMC + dynamical fermions. 32⁴ β-scan, deconfinement at β=5.69. |
| Spectral | Anderson localization (1D/2D/3D), Hofstadter butterfly, Lanczos eigensolver. |
| NPU | 10 SDK assumptions overturned. ESN streaming at 2.8μs/step. |

**Papers reproduced**: Sarkas Yukawa OCP, Diaw et al. (2024), SEMF→HFB nuclear EOS on full AME2020, HotQCD EOS tables, SU(3) Wilson action, dynamical fermion QCD, Abelian Higgs U(1), Anderson localization, Hofstadter butterfly, Kachkovskiy spectral theory.

**Participates in**: {{ entity(name="toadstool") }} validation (primary GPU science driver), gen3 constrained evolution evidence, {{ entity(name="metalforge") }} hardware exploration, {{ entity(name="basecamp") }} Papers 07/10/15/25.

---

### 1.2 airSpring — Precision Agriculture & Irrigation

**Domain**: Evapotranspiration (8 methods), soil moisture, IoT irrigation, Richards PDE, coupled hydrology, yield response  
{{ entity_metrics(name="airspring") }}  
**Checks**: 1,237 Python + 827 lib + 186 forge + 381 validation + 146 evolution  
**Reproduces work by**: Younsuk Dong (BAE, MSU)  
**Repository**: [syntheticChemistry/airSpring](https://github.com/syntheticChemistry/airSpring)

{{ entity(name="airspring") }} proves {{ entity(name="barracuda") }} can replace the Python/Excel toolchain for precision agriculture at every stage — from paper reproduction through GPU-accelerated sovereign computation on consumer hardware. The complete pipeline (weather data → evapotranspiration → crop coefficients → water balance → yield response) runs in Rust, on GPU, with zero institutional access required.

**The headline results**: 57 papers reproduced with full provenance. FAO-56 ET₀ matches Python to 1e-5 across 75 cross-validated values. Real data from 100 Michigan stations (15,300 station-days) achieves R²=0.97 using only free, open APIs (Open-Meteo, NOAA). 19.8× geometric mean Rust speedup over Python (24 algorithms), 13,000× at atlas scale. {{ entity(name="nucleus") }} primal with 30 science capabilities.

**What the constraint revealed**: Open data can replace institutional access — no weather station networks, no licensed datasets. The GPU pipeline (ET₀→Kc→WB→Yield in one dispatch chain) stays on-device with zero CPU round-trips. Cross-spring shader provenance traces every GPU operation to its mathematical origin: precision from {{ entity(name="hotspring") }}, biology from {{ entity(name="wetspring") }}, ML from {{ entity(name="neuralspring") }}, uncertainty from {{ entity(name="groundspring") }}.

| Phase | Key Result |
|-------|------------|
| 0 (Python) | 57 papers matched exactly. 1,237/1,237 checks. |
| 0+ (Real data) | 100 Michigan stations, R²=0.97 — open data validated. |
| 1-2 (Rust + cross-validation) | 75/75 Python↔Rust matches within 1e-5. 19.8× speedup. |
| 3 (GPU) | Pure GPU end-to-end, 0.04% seasonal parity. |
| 3.5+ (NPU/NUCLEUS) | AKD1000 + 27 workloads + 30 capabilities. Full cross-substrate. |

**Participates in**: {{ entity(name="toadstool") }} validation, Penny Irrigation (real-world target), {{ entity(name="nucleus") }} (30 ecology capabilities), {{ entity(name="basecamp") }} Papers 03/06/08/12.

---

### 1.3 wetSpring — Life Science & Analytical Chemistry

**Domain**: 16S metagenomics, LC-MS feature extraction, PFAS screening, microbial ecology
{{ entity_metrics(name="wetspring") }}  
**Checks**: 5,707+ across 376 experiments, 63/63 papers reproduced  
**Reproduces work by**: Christopher Waters (MMG, MSU), Kevin Liu (CMSE, MSU), Jesse Cahill & Chuck Smallwood (Sandia), A. Daniel Jones (BMB/Chemistry, MSU), Rika Anderson (Biology, Carleton), Andrea J. Gonzales (MSU), Erika Lisabeth (ADDRC, MSU), Richard Neubig (Drug Discovery, MSU)  
**Repository**: [syntheticChemistry/wetSpring](https://github.com/syntheticChemistry/wetSpring)

{{ entity(name="wetspring") }} proves {{ entity(name="barracuda") }} can replace the Galaxy/QIIME2/Python bioinformatics stack with sovereign Rust. The complete 16S pipeline — FASTQ→quality→merge→dereplicate→DADA2→chimera→taxonomy→diversity→UniFrac — runs in Rust with **1 runtime dependency** (flate2 for gzip). The sovereign XML parser eliminates `quick-xml`; the sovereign FASTQ parser eliminates `needletail`. 1,077× GPU speedup for spectral cosine matching.

**What the constraint revealed**: Zero local WGSL — every GPU operation is delegated to {{ entity(name="barracuda") }} via {{ entity(name="toadstool") }}. {{ entity(name="wetspring") }} consumes 79 {{ entity(name="barracuda") }} primitives without duplicating any math. The three-tier validation pattern (CPU → GPU → {{ entity(name="metalforge") }}) was pioneered here and adopted across all springs. {{ entity(name="wetspring") }} found and fixed the `log_f64` bug in {{ entity(name="toadstool") }} (coefficients halved, causing 1e-3 instead of 1e-15 precision) during Shannon entropy validation — the spring improved the infrastructure it depends on. {{ entity(name="wetspring") }} also resolved 4 {{ entity(name="barracuda") }} gaps: ODE solver, Gillespie stochastic sim, HMM Viterbi, Smith-Waterman alignment.

**63/63 papers reproduced** across 4 tracks: Waters c-di-GMP/QS, Liu comparative genomics, deep-sea metagenomics, Jones PFAS. 50/50 three-tier eligible papers have full CPU + GPU + {{ entity(name="metalforge") }} validation. Public benchmark against 4 BioProjects (22 samples) — all match paper ground truth.

| Phase | Key Result |
|-------|------------|
| 1-2 (Galaxy→Rust) | 30 sovereign bio modules, 135/135 checks. |
| 3-7 (GPU pipeline) | Complete 16S on GPU. 1,077× spectral cosine speedup. |
| V53+ (Anderson QS) | 52/52 papers, Anderson localization applied to biology. |
| V86 (Cross-spring) | 23/23 across 5 springs. -4,753 net lines (deep debt elimination). |

**Participates in**: {{ entity(name="toadstool") }} validation (bug discovery, 79 primitives consumed), {{ entity(name="biomeos") }} (microbiome monitoring), {{ entity(name="nestgate") }} (NCBI integration), {{ entity(name="basecamp") }} Papers 01/03/04/05/06.

---

### 1.4 groundSpring — Measurement Noise & Uncertainty

**Domain**: Sensor noise, inverse problems, error propagation, spectral theory, quasispecies, rare biosphere  
{{ entity_metrics(name="groundspring") }}  
**Checks**: 395/395 Rust + 287 Python + 936 Rust tests  
**Reproduces work by**: Alexei Bazavov (MSU), Christopher Waters (MSU), Kevin Liu (MSU), Emily Dolson (MSU), Ilya Kachkovskiy (MSU), Rika Anderson (Carleton), Andrea J. Gonzales (MSU)  
**Repository**: [syntheticChemistry/groundSpring](https://github.com/syntheticChemistry/groundSpring)

{{ entity(name="groundspring") }} establishes the uncertainty budget for every other spring. It decomposes measurement error into correctable bias and irreducible noise, quantifies which inputs dominate output uncertainty, and demonstrates how noise propagates through inverse problems. The framework — **decompose, identify dominant source, quantify noise floor** — is universal across domains.

**Why it matters**: {{ entity(name="groundspring") }} is the only spring that contributes to every {{ entity(name="basecamp") }} paper. Exp 003 told {{ entity(name="airspring") }} that humidity sensors matter most (66% of ET₀ uncertainty). Exp 004 told {{ entity(name="wetspring") }} that 5,000 reads is the genus saturation depth. Exp 001 told {{ entity(name="neuralspring") }} how much sensor noise to expect. Bazavov experiments (019-021) connect directly to {{ entity(name="hotspring") }} lattice QCD: jackknife provides the standard error estimation used in every lattice QCD publication. Combined pipeline: {{ entity(name="hotspring") }} (GPU simulation) → {{ entity(name="groundspring") }} (inverse problem + error bars) → {{ entity(name="neuralspring") }} (surrogate acceleration).

**30+ papers reproduced** across 7 researchers: Waters (signal specificity, bistable QS), Liu (RAWR, resampling), Kachkovskiy (Anderson, Almost-Mathieu, transport, band edge), Dolson (quasispecies), R. Anderson (drift, rare biosphere), Bazavov (jackknife, freeze-out, spectral recon), Gonzales (tissue Anderson, drug scoring).

| Phase | Key Result |
|-------|------------|
| 0-1 (Python→Rust) | 5 pillars: Signal vs Noise, Inverse Problems, Sensing, Temporal, Spatial. |
| 2 (GPU) | RTX 4070, Titan V, AKD1000 NPU validated. 102 {{ entity(name="barracuda") }} delegations. |
| 4 ({{ entity(name="nucleus") }}) | measurement.* domain, JSON-RPC 2.0, capability discovery via {{ entity(name="biomeos") }}. |

**Participates in**: All springs (uncertainty quantification), all {{ entity(name="basecamp") }} papers, {{ entity(name="hotspring") }} (spectral primitives + QCD inverse problems), {{ entity(name="wetspring") }} (quasispecies, rare biosphere).

---

### 1.5 neuralSpring — Machine Learning Primitives & Sovereign Structure Prediction

**Domain**: Neural surrogates, transformers, sequence models, transfer learning, structure prediction ({{ entity(name="helixvision") }})  
{{ entity_metrics(name="neuralspring") }}  
**Checks**: 4,500+ total (397 Python + 4,000+ Rust/GPU)  
**Reproduces work by**: Emily Dolson (CSE, MSU), Kevin Liu (CMSE, MSU), Christopher Waters (MMG, MSU), Alexei Bazavov (MSU), Ilya Kachkovskiy (MSU), Rika Anderson (Carleton), Andrea J. Gonzales (MSU)  
**Repository**: [syntheticChemistry/neuralSpring](https://github.com/syntheticChemistry/neuralSpring)

{{ entity(name="neuralspring") }} proves the **Isomorphism Theorem** — all neural architectures decompose into 6 fundamental primitives (GEMM, Attention, Normalization, Nonlinearity, Reduction, Gating), and {{ entity(name="barracuda") }}'s WGSL shader library covers all 6. This means the same GPU kernels that serve LLaMA serve OpenFold serve ResNet serve LSTM weather models. Pure Rust is **83.6× faster** than Python/NumPy (geomean, 11 domains).

**{{ entity(name="helixvision") }}** (formerly coralForge) extends the isomorphism to protein structure prediction: pure Rust f64 implementations of AlphaFold2/AlphaFold3 primitives (Evoformer, IPA, diffusion, pairformer, confidence), validated against NumPy baselines and accelerated via barraCuda/ToadStool. The same 6 primitives that serve language models serve AlphaFold.

**What the constraint revealed**: 25 papers across 4 research groups and 5 disciplines decompose into the same 6 primitives. 47 CPU operations promoted to GPU with 30/30 dispatch parity. Multi-GPU validation (RTX 4070 + Titan V) shows 384/384 bit-identical results — architecture-independent. Dolson's Iram et al. (2020) Nature Physics on counterdiabatic driving of evolution was reproduced and validated. All 17 original {{ entity(name="barracuda") }} shortcomings resolved upstream.

| Phase | Key Result |
|-------|------------|
| 0-0++ (Papers) | 25 papers + 5 WDM surrogates. 6 primitives explain all architectures. |
| 1-2 (Rust→GPU) | 47 modules, 96% papers on {{ entity(name="barracuda") }} CPU, 92% on GPU. |
| 3-5 (Dispatch) | ~97% math on GPU. Multi-GPU bit-identical. 7-45× pipeline reuse speedup. |
| {{ entity(name="helixvision") }} | AlphaFold2/3 Evoformer, IPA, diffusion — sovereign structure prediction. |

**Participates in**: {{ entity(name="biomeos") }} PathwayLearner, {{ entity(name="squirrel") }} (MCP adapter, 14 tools), {{ entity(name="helixvision") }} sovereign protein pipeline, HuggingFace Model Lab (GPT-2 on {{ entity(name="barracuda") }}), {{ entity(name="basecamp") }} Papers 01/02/04/05/06/07.

---

### 1.6 healthSpring — Human Health: PK/PD, Microbiome, Biosignal, Drug Discovery

**Domain**: Pharmacokinetics, gut microbiome, biosignal processing, endocrinology, comparative medicine, drug discovery, NLME  
{{ entity_metrics(name="healthspring") }}  
**Checks**: 795 (601 Rust + 194 Python cross-validation)  
**Reproduces work by**: Andrea J. Gonzales (MSU Pharmacology & Toxicology), Charles Mok (clinical endocrinology)  
**Repository**: [syntheticChemistry/healthSpring](https://github.com/syntheticChemistry/healthSpring)

{{ entity(name="healthspring") }} proves the {{ entity(name="ecoprimals") }} math infrastructure extends to human clinical applications. PK/PD models validated against canine data in {{ entity(name="neuralspring") }} transfer directly to human therapeutics via allometric scaling. The Anderson localization framework from wetSpring/hotSpring applies to gut microbiome colonization resistance. The **"claim verification pipeline"** — extracting quantifiable claims from clinical practice literature and validating against published registry data — is a novel methodology that generalizes to any medical reference.

**What the constraint revealed**: The testosterone-gut axis (Exp037) bridges microbiome diversity and endocrine outcomes via Anderson localization, validating a cross-track hypothesis. Sovereign NLME (FOCE/SAEM) replaces proprietary NONMEM/Monolix. Species-agnostic PK means the same code handles canine AD, feline hyperthyroid, and human TRT. ODE→WGSL codegen absorbed from {{ entity(name="wetspring") }}; uncertainty quantification absorbed from {{ entity(name="groundspring") }} — the springs feed each other.

**7 tracks** spanning the full breadth of human health computing:

| Track | Domain | Key Models |
|-------|--------|------------|
| 1 — PK/PD | Pharmacokinetics, dose-response | Hill, PBPK, population Monte Carlo, Michaelis-Menten |
| 2 — Microbiome | Gut ecology, colonization | Anderson gut lattice, C. diff, FMT, SCFA, serotonin |
| 3 — Biosignal | ECG, HRV, SpO2, EDA | Pan-Tompkins, arrhythmia classification, multi-channel fusion |
| 4 — Endocrinology | Testosterone PK, TRT | IM/pellet depot PK, testosterone-gut axis |
| 5 — NLME | Population PK estimation | Sovereign FOCE/SAEM, NCA, diagnostic plots |
| 6 — Comparative Medicine | Cross-species health | Species-agnostic PK, canine AD, feline hyperthyroid |
| 7 — Drug Discovery | Compound screening | MATRIX scoring, ADDRC HTS, iPSC validation |

**Participates in**: {{ entity(name="basecamp") }} Paper 13, {{ entity(name="toadstool") }} validation, {{ entity(name="nestgate") }} (NCBI data pipeline), {{ entity(name="biomeos") }} {{ entity(name="nucleus") }} (distributed health pipeline).

---

### 1.7 ludoSpring — Game Science, HCI, Procedural Generation

**Domain**: Game design, human-computer interaction, procedural content generation, real-time interactive systems  
{{ entity_metrics(name="ludospring") }}  
**Checks**: 1,692 validation checks + unit/integration tests  
**Reproduces work by**: Csikszentmihalyi (Flow), Fitts (1954), Yannakakis & Togelius (2018), Lazzaro (2004), Hunicke (2005), Perlin (1985), Tufte (1983)  
**Repository**: [syntheticChemistry/ludoSpring](https://github.com/syntheticChemistry/ludoSpring)

{{ entity(name="ludospring") }} proves the {{ entity(name="ecoprimals") }} pipeline produces validated science in interactive systems — the most demanding real-time domain humans build. 13 foundational HCI models validated against published research. Game genres are **interaction architectures**, not aesthetic categories: FPS = molecular explorer, roguelike = parameter space exploration, RTS = systems biology dashboard.

**Key finding**: Flow state (Csikszentmihalyi) discriminates game quality; engagement alone measures activity, not optimal experience. External control groups prove the metrics framework is content-agnostic — the same fraud detectors work across gaming, science, and medical domains (>80% structural similarity).

**What the constraint revealed**: The Anderson QS explorer uses Perlin noise as a disorder landscape with QS propagation showing localization transition — game mechanics as scientific instruments. Game metrics generalize to scientific exploration sessions. The provenance trio ({{ entity(name="rhizocrypt") }} + {{ entity(name="loamspine") }} + {{ entity(name="sweetgrass") }}) produces the same chain-of-custody tracking for game items, biological samples, and medical records. This cross-domain isomorphism drives {{ entity(name="basecamp") }} Papers 17-22.

**13 models validated**: Fitts's law, Hick's law, Steering law, GOMS/KLM, Flow theory, Dynamic Difficulty Adjustment, Four Keys to Fun, Engagement metrics, Perlin noise, Wave Function Collapse, L-systems, BSP trees, Tufte data-ink. 2 playable prototypes (Doom terminal, roguelike explorer) where every mechanic traces to a published paper. 110× 60Hz raycaster headroom on CPU.

**Participates in**: {{ entity(name="basecamp") }} Papers 17-22, {{ entity(name="toadstool") }} validation (GPU dispatch), {{ entity(name="petaltongue") }} (3 dashboard binaries), {{ entity(name="biomeos") }} {{ entity(name="nucleus") }} ({{ entity(name="toweratomic") }} validated), {{ entity(name="wetspring") }} (Anderson QS explorer).

---

### 1.8 primalSpring — Composition Validation (Meta-Spring)

**Domain**: Primal composition, deploy graph validation, cross-gate bonding, {{ entity(name="byob") }} verification  
{{ entity_metrics(name="primalspring") }}  
**Checks**: 666 tests, 85 experiments, 389 registered capability methods  
**Repository**: [syntheticChemistry/primalSpring](https://github.com/syntheticChemistry/primalSpring)

{{ entity(name="primalspring") }} is not a science domain spring — it validates that **primals compose correctly**. Where other springs ask "does the math match the paper?", {{ entity(name="primalspring") }} asks "does the deploy graph wire correctly?", "do primals bond across gates?", and "does {{ entity(name="byob") }} composition produce the expected emergent behavior?" It is the integration test suite for the ecosystem's composition model.

Every spring depends on primals composing correctly. {{ entity(name="primalspring") }} closes the loop: if it passes, the composition model works. If it fails, the error is in the wiring, not the science. {{ entity(name="biomeos") }} is the primary test subject — all 5 coordination patterns (Sequential, Parallel, ConditionalDag, Pipeline, Continuous) are validated. 13 deploy graphs (74 total nodes, 5 bond types), all nodes addressed by capability, topologically sorted. exp094 validates full NUCLEUS composition parity (Tower + Node + Nest + Cross-Atomic pipeline). JH-0 MethodGate capability check adopted by 13/13 primals.

**Participates in**: {{ entity(name="biomeos") }} (composition testing), {{ entity(name="plasmidbin") }} (validates packaged artifacts compose), all springs indirectly (guarantees the infrastructure they depend on), {{ entity(name="basecamp") }} Paper 23/26.

---

## 2. The Spring Network

### 2.1 By the Numbers

| Metric | Value |
|--------|-------|
| Total springs | {{ total_stat(stat="total_springs") }} (7 science domain + 1 meta-spring) |
| Total quantitative checks | **{{ total_stat(stat="validation_checks") }}** passing |
| Scientific domains covered | Physics, agriculture, biology, chemistry, geophysics, ML, neuromorphic computing, **human health (PK/PD, microbiome, biosignal, endocrinology)**, **game science (HCI, PCG, interactive systems)** |
| Papers reproduced | {{ total_stat(stat="papers_reproduced") }} (published, peer-reviewed, across all springs) |
| Papers queued for review | 60+ candidates across all springs + 8 Mok-derived experiments |
| Published work reproduced from | 14 researchers (MSU + Sandia + Carleton + clinical practice) across 9 departments |
| BarraCuda kernels validated by springs | 79+ distinct GPU/NPU primitives ({{ entity(name="wetspring") }} alone consumes 79 via {{ entity(name="toadstool") }} S68) |
| BarraCuda bugs found by springs | 6 (5 upstream in Sarkas, 1 in {{ entity(name="toadstool") }} log_f64) |
| Rust validation checks | 1,008 ({{ entity(name="wetspring") }}) + 4,000+ ({{ entity(name="neuralspring") }}) + 3,123+ ({{ entity(name="airspring") }}) + ~697 ({{ entity(name="hotspring") }}) + 236 ({{ entity(name="groundspring") }}) |
| WGSL shaders | 700+ cross-spring via {{ entity(name="toadstool") }} S68 universal precision |
| Languages | Python (Phase 0), Rust (Phase 1+), WGSL shaders (Phase 2+) |
| License | {{ entity(name="scyborg") }} — AGPL-3.0-or-later (code) + ORC (game mechanics) + CC-BY-SA 4.0 (creative/docs) |
| Institutional access required | Zero |
| Proprietary software required | Zero |
| Time from first spring to 11,161+ checks | ~27 days |

### 2.2 Cross-Spring Data Flow

```
groundSpring (uncertainty + spectral + quasispecies — 21 experiments, 236/236 checks, 8 domains)
    │
    ├──→ airSpring: "humidity dominates ET₀ uncertainty at 66%"
    ├──→ wetSpring: "genus saturation at 5,000 reads; quasispecies, rare biosphere"
    ├──→ neuralSpring: "expect 0.004-0.021 m³/m³ sensor noise floor; noise labels"
    ├──→ hotSpring: "inverse problem depth poorly constrained; Anderson, Almost-Mathieu, band edge spectral primitives"
    └──→ ToadStool: "27 barracuda delegations (22 CPU + 5 GPU)"

neuralSpring (ML primitives)
    │
    ├──→ airSpring: "MLP surrogate replaces FAO-56 at R²=0.999"
    ├──→ airSpring: "transfer learning bridges Michigan→NM with 200 samples"
    ├──→ hotSpring: "isomorphic GEMM serves plasma and nuclear"
    ├──→ wetSpring: "LSTM validates lstm_cell.wgsl on real weather"
    ├──→ **healthSpring**: "Hill/IC50, PK models, allometric scaling → human therapeutics"
    └──→ biomeOS: "PathwayLearner uses validated attention primitives"

hotSpring (GPU compute patterns)
    │
    ├──→ airSpring: "f64 GPU dispatch batching pattern"
    ├──→ wetSpring: "FusedMapReduceF64 pattern for bulk statistics"
    └──→ ToadStool: "195 acceptance checks, 6 bugs found"

wetSpring (biology + chemistry — 4,688+ checks, 197 experiments, 52/52 papers)
    │
    ├──→ ToadStool: "log_f64 bug found and fixed; 79 primitives consumed; three-tier validated (CPU→GPU→metalForge)"
    ├──→ ToadStool: "Anderson spectral primitives (anderson_3d, lanczos, level_spacing_ratio) validated at f64"
    ├──→ ToadStool: "Typed NCBI errors (Error::Ncbi) for sovereign data acquisition"
    ├──→ airSpring: "kriging spatial interpolation; dynamic Anderson W(t) models soil moisture coupling"
    ├──→ hotSpring: "Anderson localization applied to biology — shared spectral primitives, W_c determination"
    ├──→ neuralSpring: "ESN/LSTM anomaly detection for sentinel microbes; NPU int8 quantization validated"
    ├──→ **healthSpring**: "diversity indices, Anderson lattice → gut colonization resistance, 16S pipeline"
    └──→ groundSpring: "sequencing noise calibrates rarefaction; 86 named tolerances with provenance"

ludoSpring (game science — 410 checks, 44 experiments, 13 HCI models)
    │
    ├──→ barraCuda: "sigmoid, dot, lcg_step, state_to_f64 consumed; 8 Tier A GPU modules identified"
    ├──→ petalTongue: "3 dashboard binaries, 7 GameChannelType channels, live streaming"
    ├──→ healthSpring: "Fitts/Hick for medical UI evaluation; engagement for patient compliance"
    ├──→ wetSpring: "Perlin noise as Anderson disorder landscape; game telemetry protocol for lab UIs"
    └──→ all springs: "Flow theory + DDA for any adaptive interactive system"
```

### 2.3 Spring → Primal Connections

| Spring | Primarily Validates | Also Feeds |
|--------|-------------------|------------|
| {{ entity(name="hotspring") }} | ToadStool/BarraCuda (GPU MD, nuclear EOS) | gen3 (constrained evolution evidence) |
| {{ entity(name="airspring") }} | ToadStool/BarraCuda (Rust science crate) | Penny Irrigation (real-world application) |
| {{ entity(name="wetspring") }} | ToadStool/BarraCuda (GPU diversity, spectral) | {{ entity(name="biomeos") }} (microbiome monitoring), {{ entity(name="squirrel") }} (classifiers) |
| {{ entity(name="groundspring") }} | All springs (uncertainty budget) | {{ entity(name="neuralspring") }} (noise labels for training) |
| {{ entity(name="neuralspring") }} | ToadStool/BarraCuda (ML kernels) | {{ entity(name="biomeos") }} (PathwayLearner), {{ entity(name="squirrel") }} (inference), {{ entity(name="nucleus") }} (optimization) |
| **{{ entity(name="healthspring") }}** | ToadStool/BarraCuda (population PK, Anderson gut, biosignal) | {{ entity(name="nestgate") }} (NCBI clinical data), {{ entity(name="biomeos") }} {{ entity(name="nucleus") }} (distributed health pipeline) |
| **{{ entity(name="ludospring") }}** | ToadStool/BarraCuda (game math: noise, raycaster, metrics) | {{ entity(name="petaltongue") }} (live dashboards), {{ entity(name="biomeos") }} {{ entity(name="nucleus") }} ({{ entity(name="toweratomic") }} validated), {{ entity(name="wetspring") }} (Anderson QS cross-spring), nestgate (NCBI QS data), all springs (HCI models for any interactive system) |

### 2.4 Published Work Reproduced — Researcher × Spring Map

Each spring reproduces published, peer-reviewed science as its acceptance criteria. The table maps researchers to the springs that reimplement their work — independently, in Rust, with automated cross-validation against the original results. This is replication with rigor and full provenance, not collaboration or endorsement.

| Researcher | Department | Published Domain | Springs Reproducing Their Work |
|------------|-----------|------------------|-------------------------------|
| Michael Murillo | CMSE, MSU | Dense plasmas, WDM, molecular dynamics | {{ entity(name="hotspring") }} |
| Younsuk Dong | BAE, MSU | Precision agriculture, irrigation | {{ entity(name="airspring") }} |
| Christopher Waters | MMG, MSU | Quorum sensing, c-di-GMP | {{ entity(name="wetspring") }}, {{ entity(name="groundspring") }}, {{ entity(name="neuralspring") }} |
| Kevin Liu | CMSE, MSU | Comparative genomics, phylogenetics | {{ entity(name="wetspring") }}, {{ entity(name="groundspring") }}, {{ entity(name="neuralspring") }} |
| Alexei Bazavov | CMSE + Physics, MSU | Lattice QCD, thermodynamics | {{ entity(name="hotspring") }}, {{ entity(name="groundspring") }}, {{ entity(name="neuralspring") }} |
| Emily Dolson | CSE, MSU | Evolutionary computation | {{ entity(name="neuralspring") }}, {{ entity(name="groundspring") }} |
| Ilya Kachkovskiy | Math, MSU | Spectral theory, Anderson localization | {{ entity(name="hotspring") }}, {{ entity(name="groundspring") }}, {{ entity(name="neuralspring") }} |
| Jesse Cahill | Sandia (Bioscience) | Biosurveillance | {{ entity(name="wetspring") }} |
| Chuck Smallwood | Sandia (Bioscience) | Biosurveillance | {{ entity(name="wetspring") }} |
| A. Daniel Jones | BMB/Chemistry, MSU | Mass spectrometry, PFAS | {{ entity(name="wetspring") }} |
| Rika Anderson | Biology, Carleton College | Vent metagenomics, pangenomics | {{ entity(name="hotspring") }}, {{ entity(name="wetspring") }}, {{ entity(name="groundspring") }}, {{ entity(name="neuralspring") }} |
| Andrea J. Gonzales | Pharmacology & Toxicology, MSU | Pharmacology, cytokine signaling | {{ entity(name="wetspring") }}, {{ entity(name="neuralspring") }}, {{ entity(name="airspring") }}, {{ entity(name="healthspring") }} |
| Erika Lisabeth | Pharmacology & Toxicology, MSU (ADDRC) | Drug discovery, HTS | {{ entity(name="wetspring") }} |
| Richard Neubig | Pharmacology & Toxicology, MSU (Drug Discovery) | GPCR signaling, fibrosis | {{ entity(name="wetspring") }}, {{ entity(name="neuralspring") }} |
| Charles Mok | Clinical Practice | Clinical endocrinology, TRT | {{ entity(name="healthspring") }} |

Full profiles: `data/FACULTY_SPRING_PROFILES.md`

### 2.5 BarraCuda Gap Summary (Across All Springs)

| Gap | Requesting Springs | Priority | Published Source | Status |
|-----|-------------------|----------|-----------------|--------|
| FFT | {{ entity(name="hotspring") }} (lattice QCD), {{ entity(name="groundspring") }} (spectral recon), {{ entity(name="wetspring") }} (signal) | P0 | Bazavov papers | **Resolved** — {{ entity(name="hotspring") }} |
| ODE solver (RK4) | {{ entity(name="wetspring") }} (c-di-GMP), {{ entity(name="groundspring") }} (bifurcation) | P0 | Waters papers | **Resolved** — {{ entity(name="wetspring") }} bistable, capacitor, qs_ode validated |
| Lanczos eigensolve | {{ entity(name="hotspring") }} (Dirac spectrum), {{ entity(name="groundspring") }} (Anderson), {{ entity(name="neuralspring") }} (Hessian) | P1 | Kachkovskiy papers | **Resolved** — {{ entity(name="wetspring") }} + {{ entity(name="hotspring") }}: `lanczos`, `lanczos_eigenvalues` validated (CPU + GPU) |
| SpMV (sparse matrix-vector) | {{ entity(name="hotspring") }} (lattice gauge), {{ entity(name="groundspring") }} (spectral), {{ entity(name="neuralspring") }} (sparse) | P1 | Kachkovskiy papers | **Resolved** — implemented for Lanczos + Anderson 3D |
| HMM Viterbi | {{ entity(name="neuralspring") }} (PhyloNet-HMM), {{ entity(name="wetspring") }} (metagenomics) | P1 | Liu papers | **Resolved** — {{ entity(name="wetspring") }} `phylohmm` module validated |
| Evolutionary optimization | {{ entity(name="neuralspring") }} (counterdiabatic), {{ entity(name="groundspring") }} (Dolson) | P1 | Dolson papers | Open — unlocks constrained evolution validation |
| Smith-Waterman alignment | {{ entity(name="wetspring") }} (genomics), {{ entity(name="neuralspring") }} (sequence models) | P1 | Liu papers | **Resolved** — {{ entity(name="wetspring") }} Exp028 validated |
| Gillespie simulation | {{ entity(name="wetspring") }} (quorum sensing), {{ entity(name="groundspring") }} (biological noise) | P1 | Waters papers | **Resolved** — {{ entity(name="wetspring") }} stochastic modules validated |
| Matrix exponentiation | {{ entity(name="hotspring") }} (SU(3) HMC), {{ entity(name="groundspring") }} (transport) | P2 | Kachkovskiy, Bazavov papers | Open — general exp(A) for time evolution |
| L-BFGS optimizer | {{ entity(name="neuralspring") }} (PINN improvement) | P2 | Raissi papers | Open — closes PINN error gap (5.1% → ~0.06%) |
| Cholesky solve batch | {{ entity(name="groundspring") }} (jackknife, spectral recon) | P1 | Bazavov papers | Open — main NEW gap |

---

## 3. The Evidence

The springs answer a question the primals alone cannot: **does this infrastructure produce correct science?**

The primals prove that Rust can build a sovereign computing ecosystem. The springs prove that sovereign computing can reproduce published, peer-reviewed science — and in some cases, do it faster, cheaper, and more transparently than the institutional tools it replaces.

**Claim**: Open data can replace institutional access.
**Evidence**: {{ entity(name="airspring") }} achieves R²=0.967 across 918 station-days using only free, open APIs (Open-Meteo, NOAA CDO). No institutional weather station access required.

**Claim**: Consumer GPUs can do real science.
**Evidence**: {{ entity(name="hotspring") }} runs paper-parity Yukawa MD (N=10,000, 80k steps) on a $600 RTX 4070 for $0.044. The same computation costs $50-500 on institutional HPC.

**Claim**: Sovereign Rust can replace the Python scientific stack.
**Evidence**: {{ entity(name="wetspring") }}'s 30 Rust modules with 1 runtime dependency cover the complete 16S pipeline with 4,688+ checks across 197 experiments, 52/52 papers reproduced, and 39/39 three-tier validated (CPU→GPU→{{ entity(name="metalforge") }}). {{ entity(name="airspring") }}'s Rust crate matches Python to 1e-5 across 53 cross-validated values.

**Claim**: BarraCuda's 6 isomorphic primitives serve all domains.
**Evidence**: {{ entity(name="neuralspring") }} proves GEMM + Attention + Normalization + Nonlinearity + Reduction + Gating explain LLaMA, OpenFold, ResNet, ViT, MLP surrogates, and LSTM weather models. All 6 are WGSL shaders in BarraCuda.

**Claim**: Science validation improves the infrastructure.
**Evidence**: {{ entity(name="wetspring") }} found and fixed the `log_f64` bug in {{ entity(name="toadstool") }}. {{ entity(name="hotspring") }} found and fixed 5 silent bugs in Sarkas upstream. {{ entity(name="groundspring") }} identified humidity as the bottleneck for ET₀ accuracy. Each discovery fed back into the system.

---

**Claim**: Clinical practice literature can be computationally verified.
**Evidence**: {{ entity(name="healthspring") }}'s Track 4 (Mok testosterone) extracts quantifiable claims from a 196-page clinical book and validates each against the cited primary literature — creating a closed-loop claim verification pipeline that generalizes to any medical reference.

**Every check count in this catalog is measured, not estimated. Every paper reproduction runs on consumer hardware with no institutional access. Every spring is AGPL-3.0 and publicly available on GitHub. The science is open because the methodology demands it. 11,161+ checks across 5 domains in 27 days — the constrained evolution methodology works. 7 of 11 BarraCuda gaps have been resolved by the springs themselves (ODE, Lanczos, SpMV, HMM, Smith-Waterman, Gillespie, FFT). A $300 NPU runs ESN inference at 2.8μs/step. A $600 GPU runs the same ESN at 8.2× CPU speed when the reservoir is large enough. The silicon does what the silicon does — we just had to look.**
