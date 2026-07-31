+++
title = "coralForge — Sovereign Structure Prediction Engine"
description = "Pure Rust f64 AlphaFold2/3 primitives validated against NumPy — the substrate for helixVision's sovereign protein structure prediction."
date = 2026-02-15

[taxonomies]
primals = ["barracuda", "coralreef", "toadstool"]
springs = ["neuralspring"]

[extra]
foundation = true
maturity = "architectural"

[[extra.companions]]
url = "/architecture/discovery-log/"
title = "Discovery Log"
relation = "evidence_for"
label = "f64 on consumer GPUs — the discovery that enabled coralForge"

[[extra.companions]]
url = "/products/nautilus/"
title = "nautilus"
relation = "pairs_with"
label = "Neuromorphic computing as the complementary intelligence layer"

[[extra.companions]]
url = "/science/"
title = "Science"
relation = "validated_by"
label = "175+ papers validating the computational primitives"
+++

{{ maturity(level="architectural") }} Individual AlphaFold primitives are implemented and validated (154 checks, 1e-10 tolerance vs NumPy). The end-to-end structure prediction pipeline is designed but not yet wired — see [helixVision](@/products/helixVision.md) for the full pipeline status.

---

## What It Is

coralForge is the structure prediction engine substrate — pure Rust f64
implementations of the mathematical primitives that AlphaFold2 and AlphaFold3
use for protein structure prediction. It is the computational core that
{{ entity(name="helixvision") }} builds upon.

**Repository**: syntheticChemistry/coralForge (archived — moving to sporeGarden/helixVision)
**License**: {{ entity(name="scyborg") }} (AGPL-3.0-or-later + ORC + CC-BY-SA 4.0)

---

## The Isomorphism Theorem

Every "novel" operation in AlphaFold decomposes to six fundamental primitives:

| Primitive | What It Computes |
|-----------|-----------------|
| **GEMM** | General matrix multiply — the foundation |
| **Attention** | Scaled dot-product attention (self, cross, triangular) |
| **Normalization** | LayerNorm, BatchNorm — signal scaling |
| **Nonlinearity** | GELU, SiLU, ReLU — activation functions |
| **Reduction** | Sum, mean, max across dimensions |
| **Gating** | Sigmoid-modulated signal control |

AlphaFold's Evoformer, IPA (Invariant Point Attention), diffusion module,
pairformer, and confidence heads all decompose to compositions of these six
primitives. No new mathematical primitive is required.

This is the isomorphism proof: **structure prediction is not a new kind of
computation. It is a new composition of existing computations.** The same
{{ entity(name="barracuda") }} WGSL shaders that compute molecular dynamics forces
also compute attention scores for protein structure prediction.

---

## Dual Pipeline Architecture

### AlphaFold2 Path

```
MSA (Multiple Sequence Alignment)
    -> Evoformer (48 blocks of row/column attention + triangle updates)
    -> Structure Module (IPA iterations -> 3D coordinates)
    -> Confidence (pLDDT, PAE, pTM)
```

### AlphaFold3 Path

```
Input Embedder (no MSA required for small molecules)
    -> Pairformer (pair representation attention)
    -> Diffusion Module (iterative coordinate refinement)
    -> Confidence (pLDDT, pDE, pAE)
```

Both paths share the six primitives. Both run in pure Rust f64. Both
produce bit-reproducible results across architectures.

---

## Validation

154 checks across three tiers:

| Tier | Language | Checks | What It Validates |
|------|----------|--------|------------------|
| Python reference | Python/NumPy | 62 | Correct mathematics (ground truth) |
| Rust parity | Rust f64 | 55 | Rust matches Python to 1e-10 |
| GPU acceleration | WGSL/{{ entity(name="barracuda") }} | 37 | GPU matches CPU Rust |

Cross-tier validation: every Rust result is compared to the Python reference.
Every GPU result is compared to the CPU Rust result. The chain is:
NumPy -> Rust -> WGSL -> silicon.

---

## LTEE Application

The most compelling application of coralForge is not protein engineering — it
is evolutionary biology:

1. Take frozen stocks from Lenski's Long-Term Evolution Experiment (LTEE)
2. Sequence the ancestral and evolved strains ({{ entity(name="wetspring") }} pipeline)
3. Predict protein structures for ancestral and evolved variants (coralForge)
4. Compare structural changes to fitness trajectories ({{ entity(name="hotspring") }})
5. Identify which structural innovations correspond to adaptive events

This connects computational constrained evolution (the thesis) to biological
constrained evolution (the LTEE) — both producing innovation under pressure.

---

## What Makes It Sovereign

| Property | coralForge | AlphaFold (Google) |
|----------|-----------|-------------------|
| **Code** | AGPL-3.0, all Rust | Apache 2.0, Python/JAX |
| **Precision** | f64 (double) | Mixed precision (float16/32) |
| **Dependencies** | Zero C/C++ | JAX, CUDA, Python stack |
| **GPU** | Any Vulkan (NVIDIA, AMD, Intel) | NVIDIA only (CUDA) |
| **Data** | Sovereign ({{ entity(name="nestgate") }}, no cloud) | Cloud inference |
| **Provenance** | {{ entity(name="guidestone") }} verified | None |

---

*coralForge is not an AlphaFold clone. It is a proof that structure prediction
decomposes to six primitives — and those primitives can run on sovereign
hardware, in pure Rust, with full precision, producing science that belongs
to the scientist who asked the question.*
