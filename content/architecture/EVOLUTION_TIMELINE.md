+++
title = "Evolution Timeline: 27 Days, Seven Domains, 20,695+ Checks"
weight = 80
description = "27-day sprint day-by-day record and velocity analysis"
date = 2026-03-17

[taxonomies]
primals = ["beardog", "barracuda", "biomeos", "coralreef", "loamspine", "nestgate", "petaltongue", "rhizocrypt", "skunkbat", "songbird", "sourdough", "squirrel", "sweetgrass", "toadstool"]
springs = ["airspring", "groundspring", "healthspring", "hotspring", "ludospring", "neuralspring", "wetspring"]

[extra]
domain = "Architecture"
maturity = "reproduced"
+++

**The velocity of the {{ entity(name="ecoprimals") }} springs is evidence for the K-Nome methodology.**

This document is a timestamped record of how quickly validated,
reproducible science was produced when the infrastructure existed and
the methodology worked. The primals that the springs depend on took
~8 months to build. The springs took ~27 days from first to ~10,800 checks.

---

## The Context: What Existed Before the Springs

Before Feb 1, 2026, {{ entity(name="ecoprimals") }} had:
- 14 production primals ({{ entity(name="beardog") }}, {{ entity(name="songbird") }}, {{ entity(name="nestgate") }}, {{ entity(name="toadstool") }}, BarraCuda,
  {{ entity(name="coralreef") }}, {{ entity(name="biomeos") }}, {{ entity(name="petaltongue") }}, {{ entity(name="rhizocrypt") }}, {{ entity(name="loamspine") }}, {{ entity(name="sweetgrass") }}, {{ entity(name="squirrel") }},
  {{ entity(name="skunkbat") }}, {{ entity(name="sourdough") }})
- {{ total_stat(stat="total_tests_display") }} tests across primals
- A validated GPU compute stack (BarraCuda WGSL shaders, toadStool dispatch)
- A pure Rust architecture with no C dependencies
- No springs — no scientific validation experiments

The springs were built on top of this existing infrastructure. The velocity
below reflects what happens when a capable substrate meets a methodical approach
to scientific reproduction.

---

## The 27-Day Sprint

### Week 1: Computational Physics (hotSpring)

| Dates | Event | Checks |
|-------|-------|:------:|
| **Feb 1–7** | **{{ entity(name="hotspring") }} Phase A**: Reproduce published plasma MD results in Python. 86 checks pass. **5 silent bugs found in Sarkas upstream codebase.** | 86 |
| **Feb 7–10** | **{{ entity(name="hotspring") }} Phase B–C**: BarraCuda GPU validation. Nuclear EOS on consumer GPU. Full Yukawa MD on RTX 4070 via f64 WGSL shaders. 9/9 pair-potential cases, 0.000% energy drift. | +195 |
| **Feb 10–14** | **{{ entity(name="hotspring") }} Phase D–F**: Paper-parity long runs (N=10,000, 80K steps, **$0.044 electricity**). Full AME2020 nuclear dataset (2,042 nuclei). gen3/ papers written. | +195 |

**End of Week 1:** 476 checks. One developer. Consumer RTX 4070. $0.044/run.

---

### Week 2: Agriculture + Life Science (airSpring + wetSpring)

| Dates | Event | Checks |
|-------|-------|:------:|
| **Feb 14–15** | **{{ entity(name="airspring") }} Phase 0**: Reproduce FAO-56, Dong (2020, 2024) sensor calibration. Python + Rust + cross-validation. 326 checks. Real data pipeline: 918 station-days, R²=0.967. | 326 |
| **Feb 15–16** | **{{ entity(name="wetspring") }} Phase 0**: Sovereign 16S pipeline in Rust. 30 modules, 1 external dependency. GPU spectral matching: 1,077× speedup. Public data benchmark vs 4 BioProjects. | +540 |
| **Feb 16–17** | **{{ entity(name="wetspring") }} Phase 1**: Full DADA2 + chimera + taxonomy on GPU. 1,116 total checks across 42 experiments. | +576 |
| **Feb 17** | **{{ entity(name="groundspring") }} Phase 0**: 5 experiments across 4 scientific domains. 71/71 checks. | +71 |
| **Feb 17–18** | **{{ entity(name="neuralspring") }} Phase 0**: 10 experiments — 5 synthetic, 5 scholarly reproductions (PINN, DeepONet, LeNet-5, ERA5 LSTM, quantized inference). 75/75 Python checks. | +75 |

**End of Week 2:** ~2,600+ cumulative checks. **Five domains in 7 days.**

---

### Week 3: Deep Physics + Scale (hotSpring + neuralSpring + airSpring)

| Dates | Event | Checks |
|-------|-------|:------:|
| **Feb 18–20** | **{{ entity(name="neuralspring") }} Rust validation**: 9 BarraCuda validation binaries, 549 Rust checks, 66 GPU shader checks. Fused pipeline: 43–78× speedup. | +615 |
| **Feb 19–20** | **Bazavov extension**: Lattice QCD infrastructure (SU(3), HMC, Dirac CG) in {{ entity(name="hotspring") }}. | +80 |
| **Feb 20–22** | **GPU streaming HMC**, GPU-resident CG (15,360× readback reduction), dynamical fermion QCD, production β-scan (32⁴ on RTX 3090 — deconfinement at **β_c = 5.69**). | +120 |
| **Feb 22–24** | **DF64 core streaming**: FP32 cores deliver **3.24 TFLOPS at 14-digit precision** (9.9× native f64). Titan V NVK validation. | +80 |
| **Feb 24–25** | **Cross-spring evolution map**: 164+ WGSL shaders. Debt reduction audit (0 clippy, 0 TODOs, 0 mocks). | — |
| **Feb 25–26** | **{{ entity(name="airspring") }} v0.4.5–v0.4.8**: 22 experiments (was 5). Richards PDE, biochar isotherms, dual Kc, cover crops, yield response, lysimeter, sensitivity, Priestley-Taylor, Thornthwaite, GDD, pedotransfer. 100-station Michigan Crop Water Atlas. 3,123+ checks total. | +2,800 |
| **Feb 25–26** | **{{ entity(name="groundspring") }} Phase 1**: 21 experiments across 8 scientific domains. 236/236 checks. Universal coverage (contributes to ALL 7 {{ entity(name="basecamp") }} papers). | +165 |
| **Feb 26** | **{{ entity(name="wetspring") }} V59**: 197 experiments, 4,688+ checks, 52/52 papers, 39/39 three-tier. Science extensions: NCBI sovereign pipeline, cold seep metagenomes, dynamic Anderson W(t), DF64 Anderson, NPU sentinel. 184 binaries. | +3,572 |

**End of Week 3: ~10,800+ cumulative checks. Seven domains. 27 days.**

---

## The Benchmark Moment

> **Total time from first spring (Feb 1) to 10,796+ checks across 5 domains: ~27 days.**

This is the number that validates the methodology. Not because it's fast (though it is), but because each check is a *validated scientific result* — a binary that exits 0 when the computation matches a published ground truth, exits 1 when it doesn't.

For comparison:
- A typical PhD student reproduces one paper's numerical results in 3–6 months
- A typical lab reproduces 2–5 papers per year for their domain
- {{ entity(name="ecoprimals") }} reproduced 175+ papers across 7 domains in ~27 days of spring work
  (built on 8 months of primal infrastructure)

---

## What the Timeline Proves

### 1. The Substrate Matters

The springs were fast because the infrastructure existed. BarraCuda's 806 WGSL
shaders, toadStool's hardware dispatch, the capability-based IPC — the springs
consumed these instead of building them. The 8-month primal build phase is the
hidden investment that made 27-day sprints possible.

### 2. Constraint Accelerates

Every spring starts with the same constraint: reproduce a published paper. This
is not fuzzy. The paper has numbers. Your code either matches them or it doesn't.
The binary exits 0 or 1. This is faster than open-ended development because the
fitness function is external and pre-defined.

### 3. K-Nome Propagates Patterns

Patterns that work in one spring propagate to others immediately. The Anderson
localization framework, first validated in {{ entity(name="wetspring") }} (microbiology), was applied
to soil science ({{ entity(name="airspring") }}), immunology ({{ entity(name="healthspring") }}), spectral theory
({{ entity(name="groundspring") }}), and lattice QCD ({{ entity(name="hotspring") }}) within weeks. A domain-agnostic
methodology produces domain-agnostic results.

### 4. Discovery Is a Byproduct of Reproduction

5 bugs found in the Sarkas MD codebase during {{ entity(name="hotspring") }} Phase A.
The deconfinement temperature β_c = 5.69 confirmed on consumer hardware.
O₂-modulated Anderson W model (r=0.851) found during {{ entity(name="wetspring") }} Exp356.
Anderson in immunological tissue: no prior work exists.

These were not planned discoveries. They emerged from the constraint of
reproducing published science on verified hardware.

---

## Spring Velocity Over Time

| Sprint | Duration | Checks | Checks/Day |
|--------|:--------:|:------:|:----------:|
| {{ entity(name="hotspring") }} Phase A (plasma, Python) | 7 days | 86 | 12 |
| {{ entity(name="hotspring") }} Phase B–F (GPU + nuclear) | 7 days | 390 | 56 |
| {{ entity(name="airspring") }} Phase 0 (ET₀, real data) | 1 day | 326 | 326 |
| {{ entity(name="wetspring") }} Phase 0–1 (16S, GPU) | 2 days | 1,116 | 558 |
| {{ entity(name="neuralspring") }} Phase 0+ (ML, GPU) | 4 days | 690 | 173 |
| {{ entity(name="groundspring") }} Phase 0–1 (8 domains) | 9 days | 307 | 34 |
| {{ entity(name="wetspring") }} V59 (scale-up to 197 exp) | 3 days | 3,572 | 1,191 |
| {{ entity(name="airspring") }} v0.4.5–v0.4.8 (22 exp) | 2 days | 2,797 | 1,399 |

The rising velocity reflects two things: the methodology improving over time,
and the BarraCuda math library growing (each new primitive is immediately
available to all springs).

---

## After the 27-Day Sprint

The springs continued evolving after Feb 26:

| Date | Milestone |
|------|-----------|
| Mar 2026 | {{ entity(name="wetspring") }} V127: 376 experiments, 5,707+ checks, 354 binaries |
| Mar 2026 | {{ entity(name="airspring") }} v0.8.9: 891 lib tests, {{ entity(name="provenancetrio") }} integration |
| Mar 2026 | {{ entity(name="neuralspring") }} S162: 27 papers, 4,500+ checks, 92% line coverage |
| Mar 2026 | {{ entity(name="healthspring") }} V35: 613 tests, 79 capabilities, IPC resilience |
| Mar 2026 | {{ entity(name="ludospring") }} V24: 75 experiments, 1,692 checks, 13 HCI models |
| Mar 2026 | {{ entity(name="groundspring") }} V114: 39 modules, 715+ tests, 102 GPU delegations |
| **Mar 17, 2026** | **Total: 20,695+ checks, 175+ papers, 7 springs (8th — ludoSpring — added later)** |

---

## The Infrastructure Behind the Velocity

The springs do not build their own GPU stack. They consume:

```
barraCuda v0.3.5
  ├── 806 WGSL f64 shaders
  ├── Precision strategy: f64 / DF64 / f32 by hardware
  ├── Bio: diversity, alignment, phylogeny, biosignal, drug models
  ├── Physics: MD, spectral, Anderson, QCD, plasma transport
  ├── Math: FFT, eigensolve, NTT, matrix ops, statistics
  └── GPU dispatch: batch, streaming, mixed hardware

toadStool S156+
  ├── Hardware discovery (CPU + GPU + NPU at runtime)
  ├── Compute orchestration (96+ JSON-RPC methods)
  └── Cross-substrate: NVIDIA, AMD, BrainChip AKD1000

coralReef Phase 10, Iter 52+
  ├── Sovereign WGSL → SPIR-V → native SASS/RDNA2
  ├── 46/46 shaders compiled without vendor toolchain
  └── NVVM bypass: 12/12 patterns
```

When {{ entity(name="wetspring") }} writes a new GPU kernel, it writes to BarraCuda (the shared math
primal). When BarraCuda absorbs it, every other spring inherits it. The velocity
compounds.
