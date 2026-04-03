+++
title = "Structure Prediction Roadmap: helixVision — Sovereign AlphaFold-Quality"
description = "helixVision (formerly coralForge) — sovereign AlphaFold-quality protein structure prediction in pure Rust f64, moving to sporeGarden."
date = 2026-03-31
+++

# Structure Prediction Roadmap: helixVision — Sovereign AlphaFold-Quality

> **Note:** helixVision was previously known as coralForge. The codebase originated in
> [syntheticChemistry/neuralSpring](https://github.com/syntheticChemistry/neuralSpring) and is
> moving to sporeGarden/helixVision as a
> standalone product. Source code references may still use `coral_forge` module names during transition.

**Sovereign protein structure prediction on consumer hardware.
No cloud. No PyTorch. No CUDA. No data leaves the lab.**

**Last Updated:** March 31, 2026  
**License:** CC-BY-SA 4.0

---

## The Goal

AlphaFold2/3 revolutionized structural biology. It is also a Google
DeepMind product: requires PyTorch/JAX, CUDA, cloud APIs, and sends
sequence data to external servers. For any lab handling pre-publication
sequences, patient genomics, or proprietary protein engineering — this
is a non-starter.

**helixVision** is the ecoPrimals path to sovereign structure prediction:
AlphaFold2/3-quality results running locally on consumer hardware in
pure Rust, with full data sovereignty and cryptographic provenance.

---

## Where We Are Now (Phase A–B: Complete)

### The Isomorphism Proof

AlphaFold's "novel" neural architecture decomposes into **6 universal
primitives** — the same primitives used everywhere else in machine learning:

| AlphaFold Operation | Primitive Decomposition |
|--------------------|-------------------------|
| Triangle multiplication | Batched outer product (GEMM) + sigmoid gating + reduction |
| Triangle attention | Scaled dot-product attention + pair bias + softmax |
| Outer product mean | GEMM + reduction (mean) |
| Invariant Point Attention | Q·K^T/√d attention + L2 distance + softmax |
| Diffusion denoising (AF3) | Scale + add per step |
| SE(3) equivariant noise | GEMM + Gaussian sampling |
| Confidence heads (pLDDT, PAE) | Linear (GEMM) + softmax + weighted sum |

**This is the key insight:** AlphaFold does not introduce any new category
of computation. Every operation is a composition of GEMM, attention,
normalization, nonlinearity, reduction, and gating — the same primitives
BarraCuda already has as validated WGSL shaders.

### Current Validation

**154 checks passing** (62 Python + 55 Rust + 37 GPU):

| Component | Python | Rust | GPU |
|-----------|:------:|:----:|:---:|
| Evoformer primitives | 12 | 9 | 15 WGSL shaders |
| Evoformer block | 19 | 18 | — |
| IPA + structure module | 12 | 9 | — |
| Diffusion (AF3) | 29 | 26 | — |
| Pairformer (AF3) | 14 | 13 | — |
| Confidence heads (AF3) | 19 | 16 | — |
| DF64 WGSL pipeline | — | — | 37 |

**Precision:** All tolerances named (e.g., `CROSS_LANGUAGE` 1e-10,
`FOLDING_EPS` 1e-10). 15 DF64 WGSL shaders validated with max diff
< 1e-6 vs f64 CPU (GELU 5.6e-7, SDPA 1.1e-7). f64 canonical throughout.

### What This Means

Every building block of AlphaFold2 and AlphaFold3 has been:
1. Decomposed into universal primitives
2. Implemented in pure Rust
3. Validated against NumPy baselines to 1e-10 tolerance
4. Accelerated to GPU via BarraCuda WGSL shaders
5. Verified on consumer hardware (RTX 4070, Titan V)

The blocks are proven. The pipeline is next.

---

## The Phases Ahead

### Phase C — BarraCuda Integration (Next)

Wire helixVision primitives to BarraCuda canonical operations:
- `GemmF64::execute_gemm_ex()` for all GEMM ops
- GPU attention via existing `BatchedScaledDotProduct`
- GPU LayerNorm via existing `BatchedLayerNorm`
- Rayon for CPU parallelism on non-GPU paths

**Target:** ~50% wall-time reduction for the Pairformer block.

### Phase D — End-to-End Pipeline

```
FASTA sequence → MSA search → Feature embedding
  → Evoformer × 48 → Structure module × 8
  → Coordinates → Confidence (pLDDT, PAE, pDE)
  → Provenance chain (BearDog signing, loamSpine cert)
```

**Remaining components:**
- MSA search: MMseqs2 port or sovereign k-mer search
- Template search: PDB template library (public, ~200 GB)
- Recycling loop: Evoformer output fed back N times
- Amber relaxation: Optional energy minimization post-prediction

**Validation gate:** LDDT > 0.7 on at least one CASP target (e.g., T1024).

### Phase E — LTEE Structural Evolution Analysis

The primary scientific application. Lenski's Long-Term Evolution Experiment:
75,000+ generations of *E. coli* under glucose-minimal constraint, frozen
at 500-generation intervals. helixVision predicts structures at each
timepoint and population.

**Scale:** ~8.3 million predictions (4,600 genes × 150 timepoints × 12
populations).

**Questions only helixVision can answer:**
- Do independently evolved populations converge on the same structural
  solutions? (Structural convergence beyond sequence convergence)
- Do structural changes follow power-law dynamics?
  (Constrained evolution prediction)
- Can Ara-3 citrate utilization precursors be identified retroactively
  from structural evolution?
- Do hitchhiker mutations have structural consequences?
- Does genome streamlining (gene loss) produce compensatory structural
  changes in retained genes?

### Phase F — Standalone Publication

Standalone `helix-vision` crate on crates.io. Companion paper documenting
the isomorphism proof, validation evidence, and LTEE application.

---

## Performance Targets vs AlphaFold

| Metric | Cloud AlphaFold | helixVision (consumer GPU) |
|--------|:---------------:|:-------------------------:|
| Time per sequence | ~5 min (A100) | **~3 min** (RTX 4070, target) |
| Precision | f32 (PyTorch default) | **f64** (native or DF64) |
| LDDT accuracy | >0.7 on CASP targets | >0.7 (Phase D gate) |
| Cost per prediction | ~$0.01 (cloud API) | **~$0.0001** (electricity) |
| LTEE full analysis (8.3M) | **~$83,000** | **~$1,000** (6 months, 4× RTX 4070) |
| Data sovereignty | Data sent to Google | **Data stays local** |
| Provenance | None | **Ed25519 signed, full chain** |
| Dependencies | PyTorch, JAX, CUDA | **Rust + wgpu (zero C deps)** |
| Vendor lock | NVIDIA A100/H100 | **Any Vulkan GPU** |

The LTEE number is the most meaningful comparison: $83,000 and rate limits
on cloud AlphaFold vs $1,000 electricity and unlimited local predictions.
For a lab doing structural genomics at scale, this is the difference between
"we can't afford it" and "we already did it."

---

## Beyond AlphaFold: What Sovereign Structure Prediction Enables

### Drug Discovery (Paper 12 + helixVision)

The Anderson-augmented MATRIX scoring pipeline (329/329 checks validated)
currently uses published IC50 and pathway data. helixVision adds:

```
Drug candidate → helixVision structure → binding site geometry
  → Anderson tissue penetration model → combined score
```

Structure-based docking from sequence alone. No crystal structure required.
No commercial docking software (Schrödinger ~$50K/yr, MOE ~$20K/yr).

### Metagenomic Structural Census

wetSpring's sovereign 16S pipeline identifies what organisms are present.
helixVision predicts what their proteins look like:

```
Environmental sample → wetSpring 16S → community composition
  → Gene calling → helixVision structure → structural diversity index
  → Anderson W(structural) — disorder measured in protein space
```

This does not exist elsewhere. No one has applied Anderson localization
to structural diversity of metagenomic communities.

### Vaccine and Antigen Design

Structural prediction + provenance = a signed record of every design
iteration from target selection to final construct:

```
Pathogen genome → helixVision structure → epitope identification
  → Antigen design → rhizoCrypt DAG (design history)
  → loamSpine cert (design certificate) → sweetGrass (attribution)
```

### Enzyme Engineering (P≠NP Connection)

The P≠NP enzyme thesis (methodology/P_NP_ENZYME_THESIS.md) argues that
enzymes are nature's generative solutions to chemical NP problems.
helixVision enables computational enzyme design:

```
Target reaction → retrosynthetic analysis → enzyme class identification
  → helixVision structure → active site engineering → validation
```

If you can predict structure from sequence, and you can design sequence
for function, you have a sovereign enzyme engineering pipeline.

---

## What Someone Else Could Pick Up

helixVision is in neuralSpring (public, AGPL-3.0). The primitives are
validated. Anyone with Rust and a GPU can:

1. **Complete Phase C** — wire BarraCuda GEMM to helixVision Evoformer
   (estimated 2–4 weeks for a competent Rust developer)
2. **Build the MSA search** — MMseqs2 is open-source; a Rust port is
   tractable (estimated 4–8 weeks)
3. **Run Phase D validation** — CASP targets are public, PDB is public,
   the pipeline is modular
4. **Apply to their own domain** — any lab with sequences and questions
   about structure can use the validated primitives

The scyBorg license (AGPL-3.0) means: anyone who uses it must share
their improvements. Every advance returns to the commons. The pipeline
gets better for everyone, permanently.

---

*Source: `whitePaper/helixVision/` (20 documents), neuralSpring `src/coral_forge/`  
Validation: 154/154 checks PASS (62 Python + 55 Rust + 37 GPU)  
Repositories: [syntheticChemistry/neuralSpring](https://github.com/syntheticChemistry/neuralSpring),
[ecoPrimals/barraCuda](https://github.com/ecoPrimals/barraCuda)*
