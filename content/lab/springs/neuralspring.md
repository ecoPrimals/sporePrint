+++
title = "neuralSpring — Neural Architectures, Structure Prediction, ML Surrogates"
description = "Proves the Isomorphism Theorem — all neural architectures decompose into 6 primitives. 1,425 tests, 113,515 LOC, AlphaFold2/3 primitives validated in pure Rust f64."
date = 2026-05-07
weight = 6

[taxonomies]
primals = ["barracuda", "toadstool", "squirrel", "biomeos", "coralreef"]
springs = ["neuralspring", "hotspring", "wetspring", "groundspring", "healthspring"]
+++

## Domain

Neural network primitives (GEMM, Attention, Normalization, Nonlinearity, Reduction, Gating), protein structure prediction (AlphaFold2/3 Evoformer, IPA, diffusion), ML surrogates, NPU inference, transfer learning.

**Repository**: [syntheticChemistry/neuralSpring](https://github.com/syntheticChemistry/neuralSpring)

## The Science Story

neuralSpring proves the **Isomorphism Theorem**: every neural architecture — from LSTM to Transformer to AlphaFold — decomposes into exactly 6 computational primitives (GEMM, Attention, Normalization, Nonlinearity, Reduction, Gating). This is not a simplification; it is a mathematical fact. The consequence: implement 6 primitives correctly on GPU, and every architecture follows.

The spring validates this across 25 papers, 4 research groups, and 5 disciplines. AlphaFold2/3's Evoformer, IPA module, and confidence heads all decompose into the same 6 primitives that power LSTM time-series prediction and ESN reservoir computing.

## Headline Results

- **1,425 tests** passing, 0 failed
- **113,515 lines of Rust** across 3 crates
- **6 primitives** → every neural architecture (GEMM, Attention, Normalization, Nonlinearity, Reduction, Gating)
- **AlphaFold2/3 primitives** (Evoformer, IPA, diffusion modules) validated in pure Rust f64 — end-to-end pipeline [architectural](@/products/helixVision.md)
- **83.6x faster** than Python/NumPy on equivalent workloads
- **47 CPU ops promoted to GPU**, 384/384 bit-identical multi-GPU results
- **NPU inference** at 2.8 us/step on AKD1000 — 1,000x faster than GPU for streaming

## Validation Phases

| Phase | Key Result |
|-------|------------|
| Primitives | 6 primitives implemented in CPU + GPU (WGSL via coralReef) |
| Architecture | LSTM, ESN, HMM, Transformer, Evoformer — all decompose into 6 primitives |
| Structure Prediction | AlphaFold2/3 in f64 Rust — Evoformer, IPA, diffusion, pairformer, confidence |
| Transfer Learning | Cross-species PK (canine → human), cross-domain surrogates (airSpring Michigan→NM with 200 samples) |
| NPU | AKD1000 int8 quantization validated, ESN streaming at 2.8 us/step, coin-cell 11 years |

## Researchers Reproduced

| Researcher | Department | Domain |
|------------|-----------|--------|
| John Jumper | DeepMind | AlphaFold2/3 protein structure prediction |
| Andrea J. Gonzales | Pharmacology, MSU | Hill/IC50, PK models, allometric scaling |
| Rika Anderson | Biology, Carleton | Pangenomics, evolutionary inference |

## What the Constraint Revealed

Eliminating PyTorch/JAX forced the primitives-first approach. When you cannot import a framework, you must understand what the framework does. The 6-primitive decomposition emerged from this constraint — and it turns out to be architecturally cleaner than any framework. GPU portability comes free because coralReef compiles the same WGSL shaders for every vendor. NPU support required only mapping primitives to spiking equivalents.

The isomorphism also enables **cross-spring transfer**: the same GEMM that powers lattice QCD in hotSpring powers protein folding in neuralSpring and LSTM prediction in airSpring. The primitive is substrate-independent.

## Cross-Spring Connections

- **→ hotSpring**: Isomorphic GEMM serves plasma physics and nuclear structure
- **→ wetSpring**: ESN/LSTM anomaly detection for sentinel microbes; NPU int8 quantization
- **→ airSpring**: MLP surrogate replaces FAO-56 at R²=0.999; transfer learning bridges Michigan→NM
- **→ healthSpring**: Hill/IC50, PK models, allometric scaling → human therapeutics
- **→ groundSpring**: Sensor noise floors for training data validation
- **→ {{ entity(name="lithospore") }}**: ML surrogate enrichment for LTEE modules (additive)
- **→ Squirrel**: MCP adapter with 14 tools for AI-assisted science

## baseCamp Papers

Papers 01, 02, 04, 05, 06, 07, 08, 10, 11, 12, 16 — see [baseCamp Science](/science/) for full list.

neuralSpring contributes ML methods to 11 of 26 baseCamp papers.
