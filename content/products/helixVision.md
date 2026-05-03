+++
title = "helixVision — Self-Hosted Structure Prediction"
description = "AlphaFold2/3-quality protein structure prediction in pure Rust f64 — no cloud, no PyTorch, no CUDA, no data leaves the lab."
date = 2026-03-31

[taxonomies]
primals = ["barracuda", "beardog", "biomeos", "coralreef", "loamspine", "toadstool"]
springs = ["neuralspring"]
+++

**Repository**: sporeGarden/helixVision (moving from {{ entity(name="syntheticchemistry") }} — repo pending)  
**License**: {{ entity(name="scyborg") }} (AGPL-3.0-or-later + ORC + CC-BY-SA 4.0)  
**Formerly**: coralForge (syntheticChemistry/coralForge, now archived)

---

## What It Is

{{ entity(name="helixvision") }} is self-hosted protein structure prediction: AlphaFold2/3-quality results running locally on consumer hardware in pure Rust, with full f64 precision, complete data ownership, and cryptographic provenance. No cloud APIs. No PyTorch. No CUDA SDK. No data sent to Google.

This is not planned — it is an active codebase with **154 passing checks** (62 Python baseline + 55 Rust + 37 GPU), validated against NumPy to 1e-10 tolerance.

---

## The Isomorphism

AlphaFold's architecture decomposes into **6 universal primitives** — the same primitives BarraCuda already has as validated WGSL shaders:

| AlphaFold Operation | Primitive Decomposition |
|--------------------|-------------------------|
| Triangle multiplication | Batched outer product (GEMM) + sigmoid gating + reduction |
| Triangle attention | Scaled dot-product attention + pair bias + softmax |
| Invariant Point Attention | Q·K^T/√d attention + L2 distance + softmax |
| Diffusion denoising (AF3) | Scale + add per step |
| Confidence heads (pLDDT, PAE) | Linear (GEMM) + softmax + weighted sum |

{{ entity(name="helixvision") }} does not introduce new computation. Every operation is a composition of primitives that already exist, are tested, and run on consumer GPUs.

---

## How It Composes

| Layer | What | Primal |
|-------|------|--------|
| Math | WGSL f64 shaders for all 6 primitives | {{ entity(name="barracuda") }} |
| Compilation | WGSL → native GPU binary (NVIDIA + AMD) | {{ entity(name="coralreef") }} |
| Dispatch | Hardware discovery, execution, routing | {{ entity(name="toadstool") }} |
| Provenance | Ed25519 signing of every prediction | {{ entity(name="beardog") }} |
| History | Append-only prediction log | {{ entity(name="loamspine") }} |
| Orchestration | Pipeline routing via {{ entity(name="neuralapi") }} | {{ entity(name="biomeos") }} |

The pipeline: FASTA sequence → MSA search → Feature embedding → Evoformer × 48 → Structure module × 8 → Coordinates → Confidence (pLDDT, PAE, pDE) → Provenance chain.

---

## Performance Targets

| Metric | Cloud AlphaFold | {{ entity(name="helixvision") }} (consumer GPU) |
|--------|:---------------:|:--------------------------:|
| Precision | f32 (PyTorch default) | **f64** (native or DF64) |
| Cost per prediction | ~$0.01 (cloud API) | **~$0.0001** (electricity) |
| LTEE full analysis (8.3M predictions) | **~$83,000** | **~$1,000** (6 months, 4× RTX 4070) |
| Data sovereignty | Data sent to Google | **Data stays local** |
| Provenance | None | **Ed25519 signed, full chain** |
| Dependencies | PyTorch, JAX, CUDA | **Rust + wgpu (zero C deps)** |

---

## Validation Status

- **Phase A-B (Complete)**: All AlphaFold2/3 primitives decomposed, implemented in Rust, validated to 1e-10 vs NumPy, GPU-accelerated
- **Phase C (Next)**: Wire BarraCuda GEMM to Evoformer operations
- **Phase D**: End-to-end pipeline (FASTA → structure → confidence → provenance)
- **Phase E**: LTEE structural evolution analysis (8.3M predictions)
- **Phase F**: Standalone `helix-vision` crate on crates.io

Full roadmap: [Structure Prediction Roadmap](@/science/STRUCTURE_PREDICTION_ROADMAP.md)

---

## What It Enables

- **Drug discovery**: Structure-based docking from sequence alone (no $50K/yr Schrodinger license)
- **Metagenomic structural census**: Predict protein structures for entire microbial communities
- **Vaccine/antigen design**: Signed provenance chain from target selection to final construct
- **Enzyme engineering**: Computational enzyme design via structure prediction + P≠NP enzyme thesis
- **LTEE analysis**: 8.3M structure predictions across 75,000 generations of *E. coli* evolution

---

*See also: [Structure Prediction Roadmap](@/science/STRUCTURE_PREDICTION_ROADMAP.md),
[neuralSpring](@/architecture/SPRING_CATALOG.md) for validation evidence,
[barraCuda](@/architecture/PRIMAL_CATALOG.md) for the math engine.*
