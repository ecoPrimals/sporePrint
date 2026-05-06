+++
title = "Cross-Spring Evidence Map"
description = "Convergence analysis across all 7 springs — shared mathematical structures, open questions, and cross-domain validation."
date = 2026-03-17

[taxonomies]
primals = ["barracuda", "coralreef", "toadstool", "beardog", "rhizocrypt", "loamspine", "sweetgrass", "nestgate", "biomeos"]
springs = ["airspring", "groundspring", "healthspring", "hotspring", "ludospring", "neuralspring", "primalspring", "wetspring"]
+++

**How the {{ entity(name="basecamp") }} papers draw from multiple springs — and why that matters.**

The {{ entity(name="basecamp") }} papers are not single-spring results. Each paper draws validated
computational infrastructure from multiple springs, producing conclusions that
are more robust because they are validated from multiple directions.

This is what "sovereign scientific computing" means in practice: the same
mathematical framework (Anderson localization), implemented independently in
different domains, validated by independent experiments, producing convergent
predictions.

---

## The Anderson Thread

The most striking cross-spring pattern is Anderson localization — a condensed
matter physics framework (Anderson 1958) that appears independently across
five scientific domains:

```
Anderson localization (condensed matter physics)
    ↓
Predicted: signals propagate in 3D disordered media, localize in 2D

Validation across 5 independent biological domains:
    wetSpring    → Quorum sensing in microbial communities (Paper 01)
    wetSpring    → Cross-species signaling in symbiotic systems (Paper 05)
    wetSpring    → No-till soil health mechanism (Paper 06)
    wetSpring    → Cytokine propagation in skin tissue (Paper 12)
    hotSpring    → CG convergence proxy in lattice QCD (Paper 10)
    airSpring    → Tissue diversity in immunological models (Paper 12)
    groundSpring → Spectral theory validation (Exp 008, 012, 015, 018)
    healthSpring → Gut microbiome as Anderson lattice (Paper 13)

Single quantitative result across all domains:
    W_c = 16.26 ± 0.95 (critical disorder threshold, 3D)
    In 2D: always localizes regardless of W
```

The same parameter (W, disorder; d, dimension) governs signal behavior across
microbial ecology, immunology, soil science, plasma physics, and the gut
microbiome. This is either the most productive coincidence in computational
biology or a genuine unifying physical principle.

---

## Paper-by-Paper Cross-Spring Dependencies

### Paper 01 — Anderson Localization as QS Null Hypothesis

**Primary spring:** {{ entity(name="wetspring") }} (3,100+ checks, Exp107–356)

| Supporting Spring | What It Contributes | Key Experiments |
|------------------|--------------------|-----------------| 
| {{ entity(name="hotspring") }} | Spectral theory (Lanczos, level statistics), same Anderson math from plasma physics domain | Phase D lattice spectral |
| {{ entity(name="groundspring") }} | Spectral theory validation (Anderson 1D/2D/3D), transport models, uncertainty budgets | Exp 008, 009 |
| {{ entity(name="neuralspring") }} | ESN regime classifier — same Anderson transition classification at 96.5% accuracy | nW-05 |
| {{ entity(name="airspring") }} | Anderson coupling in soil diversity, cross-environment validation | Exp066–069 |

**Why this matters:** The 3D/2D threshold W_c = 16.26 ± 0.95 is validated from
three independent computational directions ({{ entity(name="wetspring") }} measurement, {{ entity(name="groundspring") }}
spectral theory, {{ entity(name="hotspring") }} physics). This is not a single-experiment result.

---

### Paper 06 — Anderson as the Mechanism Behind No-Till Soil Health

**Primary springs:** {{ entity(name="wetspring") }} Track 4 + {{ entity(name="airspring") }}

| Supporting Spring | What It Contributes | Key Experiments |
|------------------|--------------------|-----------------| 
| {{ entity(name="wetspring") }} | Anderson diversity measurements, 3D pore network QS modeling | Exp170–182 (321 checks) |
| {{ entity(name="airspring") }} | FAO-56 ET₀, soil moisture (Richards PDE), cover crop data | Exp066–078 |
| {{ entity(name="groundspring") }} | Uncertainty bridge: sensor noise → ξ (disorder) → r (level ratio) | Exp 015, rare biosphere |
| {{ entity(name="neuralspring") }} | LSTM time series for soil moisture, regime transition prediction | LSTM soil module |

**The mechanism:** Tillage = dimensional collapse (3D pore network → 2D surface
matrix). QS autoinducers go from propagating (3D extended) to localizing (2D
confined). Soil ecosystem services collapse because coordinated microbial activity
requires QS signal propagation. No-till preserves the 3D geometry and QS function.

**Independent validation:** Same dimensional collapse mechanism (Paper 12) appears
in AD skin (inverse direction: scratching = dimensional promotion). One physics
equation governs both.

---

### Paper 07 — Sovereign WDM Simulation on Consumer GPU

**Primary spring:** {{ entity(name="hotspring") }} (648+ checks)

| Supporting Spring | What It Contributes | Key Experiments |
|------------------|--------------------|-----------------| 
| {{ entity(name="neuralspring") }} | Surrogate models for WDM transport (5 WDM surrogates validated) | nS-WDM-01–05 |
| {{ entity(name="groundspring") }} | WDM precision/convergence/vendor-parity validation, uncertainty budgets | Exp 025–027 |
| {{ entity(name="groundspring") }} | Freeze-out inverse problem, spectral reconstruction, jackknife error bars | Exp 010, 011 |
| {{ entity(name="hotspring") }} | Anderson 4D + Wegner proxy pipeline, DF64 streaming | Exp 049–058 |

**The claim:** First lattice QCD production run (dynamical fermion, HMC) on a
consumer GPU (RTX 3090). Deconfinement at β_c = 5.69 confirmed. Smooth crossover.
The $0.044/run number validated by {{ entity(name="groundspring") }} uncertainty analysis.

---

### Paper 12 — Anderson in Immunological Signaling

**Primary springs:** {{ entity(name="wetspring") }} + {{ entity(name="neuralspring") }} + {{ entity(name="groundspring") }}

| Supporting Spring | What It Contributes | Key Experiments |
|------------------|--------------------|-----------------| 
| {{ entity(name="wetspring") }} | Anderson spectral tissue lattice, barrier disruption model, cytokine multi-compartment | Exp270–286 (157/157) |
| {{ entity(name="neuralspring") }} | Dose-response Hill (G2), PK decay (G4), ESN regime classification | nS-601–605 (329/329) |
| {{ entity(name="groundspring") }} | Spectral theory validation for tissue geometry, cytokine transport models | Exp 008, 012, 015, 018 |
| {{ entity(name="airspring") }} | GPU tissue diversity (GpuDiversity), CytokineBrain streaming | Exp066–069 (94/94) |
| {{ entity(name="healthspring") }} | JAK inhibitor PK/PD, Hill dose-response, three-compartment disorder | Track 1 + Track 7 |

**The connection to drug discovery:** 329/329 checks across five independent
implementations, four springs, two levels of validation (computational reproduction
+ cross-language parity). The Anderson-augmented MATRIX scoring (nS-605) is the
most thoroughly validated novel method in this whitepaper.

---

### Paper 13 — Sovereign Human Health Computing

**Primary spring:** {{ entity(name="healthspring") }} (73 experiments, 601+ tests)

| Supporting Spring | What It Contributes | Key Experiments |
|------------------|--------------------|-----------------| 
| {{ entity(name="wetspring") }} | Microbiome diversity → gut health Anderson W, QS gene profiling | diversity + QS modules |
| {{ entity(name="groundspring") }} | Uncertainty budgets (bootstrap/jackknife), spectral transport | uncertainty module |
| {{ entity(name="neuralspring") }} | ESN/LSTM anomaly detection, digester prediction (Paper 027) | nS-027 validated |
| {{ entity(name="airspring") }} | GpuDiversity, CytokineBrain (immune extension of agricultural modules) | Paper 12 integration |
| {{ entity(name="barracuda") }} | Canonical Hill, PopPK, diversity, MM batch, SCFA batch ops | Direct primal deps |

**The sovereignty angle:** NONMEM + Monolix + WinNonlin = ~$6,500/year in
software licenses for a pharmacometrics lab. {{ entity(name="healthspring") }} replaces all three,
runs 84× faster (CPU-only), and adds Anderson gut lattice modeling that no
commercial pharmacometric tool provides.

---

### Paper 17 — Game Design as Rigorous Science

**Primary spring:** {{ entity(name="ludospring") }} (75 experiments, 1,692 checks)

| Supporting Spring | What It Contributes | Key Experiments |
|------------------|--------------------|-----------------| 
| {{ entity(name="wetspring") }} | Anderson W model (Perlin noise as disorder landscape), Python tolerance pattern | Anderson QS mapping |
| {{ entity(name="barracuda") }} | sigmoid, dot, lcg_step primitives consumed directly | Tier A GPU |
| toadStool | `compute.dispatch.*` for real-time GPU dispatch | Direct dispatch |
| {{ entity(name="healthspring") }} | Fitts/Hick models for medical UI evaluation (cross-domain) | Engagement metrics |
| {{ entity(name="neuralspring") }} | ESN reservoir for procedural generation, game AI | Transfer learning |

**The cross-domain finding:** The same provenance architecture that tracks
game item lineage (extraction shooters) tracks biological sample lineage
(field genomics) and medical record access (HIPAA consent). Fraud detection
is structurally identical across all three domains. This is not an analogy —
it is the same code path.

---

## The groundSpring Anomaly

{{ entity(name="groundspring") }} contributes to **every** {{ entity(name="basecamp") }} paper (Papers 01–22). This
is unusual for a spring whose domain (uncertainty quantification, spectral
theory, measurement noise) sounds narrow.

Why it contributes everywhere:

| {{ entity(name="groundspring") }} Capability | Universal Need |
|------------------------|----------------|
| Jackknife error bars | Any paper with experimental uncertainty |
| Rare biosphere quantification | Any microbiome paper (what you can't detect matters) |
| Spectral theory validation | Anderson framework underlying 5+ papers |
| Uncertainty bridge: sensor → physics | Any paper with measurement data |
| WDM precision/convergence | Lattice QCD, plasma physics |
| NPU Anderson classification (AKD1000) | Any paper with edge deployment |

The lesson: uncertainty is not a single domain. It is the connective tissue
between all quantitative science. A spring that handles uncertainty well
contributes everywhere.

---

## Convergent Predictions: Where Multiple Springs Agree

These are the results where two or more springs independently arrive at
the same number:

| Result | Spring 1 | Spring 2 | Spring 3 | Agreement |
|--------|----------|----------|----------|-----------|
| W_c ≈ 16.26 (Anderson 3D) | {{ entity(name="wetspring") }} (measured) | {{ entity(name="groundspring") }} (spectral theory) | {{ entity(name="hotspring") }} (physics) | < 5% variation |
| Anderson in 2D → always localizes | {{ entity(name="wetspring") }} (QS) | {{ entity(name="groundspring") }} (math) | Paper 12 (tissue) | Exact |
| DF64 9.9× vs native f64 | {{ entity(name="hotspring") }} (benchmark) | {{ entity(name="groundspring") }} (parity) | {{ entity(name="coralreef") }} (compile) | Exact |
| ESN regime classifier >96% | {{ entity(name="neuralspring") }} (training) | {{ entity(name="wetspring") }} (application) | {{ entity(name="hotspring") }} (QCD proxy) | Cross-domain |
| Rust 84–160× faster than Python | {{ entity(name="healthspring") }} (PK/PD) | {{ entity(name="airspring") }} (ET₀, 13K×) | {{ entity(name="wetspring") }} (spectral, 1077×) | Operation-dependent |

When three independent springs produce the same result, the result is robust.
Convergent predictions across independent implementations are stronger evidence
than any single spring alone.

---

## What Springs Have Not Yet Contributed To

These papers are architecturally defined but missing wet-lab validation:

| Paper | Missing Component | Spring | Path |
|-------|-----------------|--------|------|
| 02 (LTEE) | Frozen fossil sequencing | {{ entity(name="wetspring") }} | MinION + lab collaboration |
| 04 (Sentinels) | Real-time HAB deployment | {{ entity(name="wetspring") }} (NPU live) | AKD1000 live on hardware |
| 09 (Field Genomics) | MinION nanopore sequencer | {{ entity(name="wetspring") }} | Hardware pending |
| 12 (Immuno-Anderson) | iPSC validation | {{ entity(name="healthspring") }} + Gonzales lab | Wet lab collaboration |
| 03 (BioAg) | Pistachio/almond field trial | {{ entity(name="wetspring") }} + {{ entity(name="airspring") }} | Field partner |

The computational predictions are validated. The wet-lab tests are the open frontier.

---

---

## The primalSpring Layer — Composition Validation

{{ entity(name="primalspring") }} is the eighth spring, but it validates infrastructure
rather than a scientific domain. Where science springs ask "does the Rust reproduce
the Python?", {{ entity(name="primalspring") }} asks "does the composition reproduce
the standalone binary?"

| What It Validates | How | Evidence |
|------------------|-----|---------|
| Deploy graph structure | `validate_deployment_readiness()` — checks graph nodes, binary presence, env vars, bonding | 71 TOMLs, 13 primals |
| BTSP Phase 3 AEAD | ChaCha20-Poly1305 encrypted channels between all primals | sweetGrass/rhizoCrypt reject plaintext |
| Wire Standard L3 | `capabilities.list` returns `protocol` + `transport` per primal | 13/13 conform |
| Discovery hierarchy | 5-tier escalation: Songbird IPC → biomeOS Neural → UDS → registry → TCP | Probed on live composition |
| Startup ordering | Topological sort via `topological_waves()` (Kahn's algorithm) | deploy.sh uses ordering |
| Provenance pipeline | BLAKE3 → rhizoCrypt DAG → loamSpine ledger → sweetGrass braid | 26 events, Merkle root, ed25519 witness |

### How primalSpring Connects to Science Springs

Every science spring's validated kernels eventually run through the composition
layer that {{ entity(name="primalspring") }} validates:

```
wetSpring 16S pipeline (37/37 checks standalone)
    ↓ dispatched via toadStool
    ↓ provenance tracked via rhizoCrypt → loamSpine → sweetGrass
    ↓ = same 37/37 checks in composition (zero regression)
```

The 235+ checks that pass through {{ entity(name="toadstool") }} dispatch on
projectNUCLEUS are {{ entity(name="primalspring") }}'s acceptance test. If
composition introduces regression, {{ entity(name="primalspring") }}'s validation
matrix catches it.

### What primalSpring Contributes to baseCamp

{{ entity(name="primalspring") }} does not produce baseCamp papers directly. It
produces the **proof that baseCamp science runs in composition** — the evidence
that the infrastructure is production-ready. This proof is what
{{ entity(name="foundation") }} takes to institutions: not just "the science
works" but "the science works on sovereign infrastructure, with provenance,
at commodity hardware cost."

---

## The Composition Evidence Chain

When all springs converge through composition, the evidence chain looks like this:

| Layer | What's Proven | Spring(s) |
|-------|---------------|-----------|
| Math is correct | Published results reproduced at machine-epsilon | Science springs (7) |
| Infrastructure works | 13 primals compose, communicate, and don't regress | {{ entity(name="primalspring") }} |
| Security holds | BTSP encryption, fuzzing resilience, no hidden methods | {{ entity(name="primalspring") }} + {{ entity(name="skunkbat") }} |
| Provenance is real | Content-addressed, append-only, cryptographically witnessed | Provenance trio |
| Products emerge | helixVision, esotericWebb, etc. are usable tools | Product teams |
| Institutions can adopt | Same patterns run on HPC at scale | {{ entity(name="foundation") }} |

See [Composition Pipeline](@/architecture/COMPOSITION_PIPELINE.md) for the full
flow from springs through products to institutional adoption.

---

*Spring versions at time of writing: {{ entity(name="wetspring") }} V127, {{ entity(name="airspring") }} v0.8.9,
{{ entity(name="neuralspring") }} S162, {{ entity(name="hotspring") }} v0.6.31, {{ entity(name="groundspring") }} V114, {{ entity(name="healthspring") }} V35,
{{ entity(name="ludospring") }} V24, {{ entity(name="primalspring") }} v0.9.24.*
