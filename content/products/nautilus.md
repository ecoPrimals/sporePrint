+++
title = "nautilus — Neuromorphic Reservoir Computing"
description = "Adaptive intelligence through evolutionary BingoCube reservoirs, ESN integration, and heterogeneous brain architecture on sovereign hardware."
date = 2026-03-01

[taxonomies]
primals = ["barracuda", "toadstool", "squirrel"]
springs = ["neuralspring", "hotspring"]

[extra]
maturity = "implemented"

[[extra.companions]]
url = "/products/coralforge/"
title = "coralForge"
relation = "pairs_with"
label = "Structure prediction engine — deterministic complement to reservoir computing"

[[extra.companions]]
url = "/architecture/discovery-log/"
title = "Discovery Log"
relation = "evidence_for"
label = "Capability hunting methodology that found NPU export paths"

[[extra.companions]]
url = "/architecture/primal-evolution/"
title = "Primal Evolution"
relation = "architecture"
label = "How nautilus evolved from gen1 through gen3"
+++

{{ maturity(level="implemented") }} 31 tests passing; 5.3% LOO / 2.6% blind prediction error; AKD1000 NPU int4 export validated (MSE=0.004); 4-layer brain architecture validated (Exp 028-030).

---

## What It Is

nautilus is an adaptive intelligence layer that combines evolutionary reservoir
computing with neuromorphic hardware deployment. It turns structured randomness
(BingoCube boards) into prediction engines that evolve, deploy to NPU hardware,
and coordinate across heterogeneous compute substrates.

**Crate**: `bingocube-nautilus`
**License**: {{ entity(name="scyborg") }}

---

## The BingoCube Insight

A bingo caller knows everything — every number, every board, every outcome.
But the caller does not track which boards have which patterns. Computers can.

BingoCube boards are structured random projections — high-dimensional feature
spaces encoded as bingo boards. Each board maps input features to board
positions. Populations of boards evolve under selection pressure, producing
reservoir computers that predict without backpropagation.

---

## Evolution as Time

Traditional reservoir computing (Echo State Networks, Liquid State Machines)
uses temporal recurrence — feedback loops through time. But the BrainChip
AKD1000 NPU is feed-forward only. No recurrence allowed.

nautilus solves this with a key insight: **evolution replaces recurrence**.

Instead of feedback through time within a single network, populations of
BingoCube boards evolve across generations. Each generation is a "time step."
The evolutionary trajectory encodes temporal dynamics in the population
structure rather than in network weights.

This maps perfectly to the AKD1000's feed-forward constraint — evolution
happens off-chip, and the current best board deploys as a static,
feed-forward int4 network.

---

## The 4-Layer Brain Architecture

Instead of serial GPU blocking (run one model, wait, run the next), nautilus
runs heterogeneous substrates concurrently:

| Layer | Substrate | Role | Biological Analog |
|-------|-----------|------|------------------|
| **Motor cortex** | RTX 3090 (24 GB GDDR6X) | Heavy compute, training | Primary motor area |
| **Pre-motor** | Titan V (12 GB HBM2) | Medium compute, inference | Pre-motor planning |
| **Cortex** | CPU (many cores, large RAM) | Orchestration, data management | Cerebral cortex |
| **Cerebellum** | AKD1000 NPU (1 mW/inference) | Anomaly detection, fast response | Cerebellar timing |

The NPU coordinates and interrupts: when the AKD1000 detects an anomaly
in its feed-forward pipeline, it signals the CPU cortex, which dispatches
heavier analysis to the GPU layers. The brain architecture is concurrent,
not serial.

---

## Drift and Edges

### The Drift Problem

When selection pressure is weak, evolved populations drift randomly. Without
monitoring, the reservoir degrades — predictions become noise.

nautilus includes a DriftMonitor based on Anderson localization theory:
the N_e*s drift boundary determines when evolution is selecting vs. drifting.
When drift is detected, directed mutagenesis (edge seeding) reintroduces
structured variation.

### Concept Edge Detection

BingoCube boards can detect concept edges — boundaries where prediction
accuracy drops sharply. These edges correspond to domain boundaries,
phase transitions, or distributional shifts in the input data.

---

## AKD1000 Deployment

The BrainChip AKD1000 processes 300K inferences/second at ~1 mW. nautilus
exports evolved BingoCube boards as int4 quantized networks:

| Property | Value |
|----------|-------|
| Export precision | int4 (4-bit integer) |
| Quantization MSE | 0.004 |
| Inference power | ~1 mW |
| Inference rate | 300K/sec |
| Board deployment | Static feed-forward (evolution handles recurrence) |

The quenched-to-dynamical transfer achieves 540x cost reduction compared to
running full evolved populations on GPU.

---

## Validation

| Test Category | Count | What It Validates |
|---------------|-------|------------------|
| Board evolution | 12 | Population dynamics, selection pressure, convergence |
| ESN integration | 8 | Gen 1 (fixed) vs Gen 2 (evolved) reservoir transfer |
| Brain architecture | 6 | Concurrent 4-layer pipeline, interrupt handling |
| Save/restore | 5 | Evolved population persistence and reproducibility |
| **Total** | **31** | |

Prediction metrics: 5.3% leave-one-out error, 2.6% blind prediction error
on benchmark tasks.

---

*nautilus turns structured randomness into adaptive intelligence. The bingo
caller knows everything but does not track — nautilus tracks. Evolution
replaces recurrence. The brain runs concurrently. The anomaly detector
consumes milliwatts. And the reservoir evolves under the same constrained
evolution that drives the entire ecosystem.*
