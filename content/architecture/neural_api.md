+++
title = "Neural API — Adaptive Multi-Layer Orchestration"
description = "The biomeOS orchestration layer — capability graph execution over 13 primals, composition collapse, and bidirectional learning."
date = 2026-05-05
weight = 41

[taxonomies]
trails = ["coordination"]

[extra]
domain = "Architecture"
maturity = "architectural"

[[extra.companions]]
url = "/architecture/coordination-triad/"
title = "Coordination Triad"
relation = "extended_by"
label = "Neural API as one of three coordination patterns"

[[extra.companions]]
url = "/architecture/primal-interactions/"
title = "Primal Interactions"
relation = "pairs_with"
label = "IPC architecture that Neural API orchestrates"
+++

## What It Is

The Neural API is {{ entity(name="biomeos") }}'s orchestration layer — the central nervous
system that integrates the [coordination triad](@/architecture/coordination_triad.md)
and enables products to consume primal capabilities through graph execution.

**One-line thesis**: 13 primals expose 427 methods. The Neural API collapses these
into ~20 atomic signals organized by tier (Tower/Node/Nest/NUCLEUS), so complex
systems like [rootPulse](@/architecture/rootpulse.md) and product compositions emerge
rather than being engineered as monoliths.

---

## The Combinatorial Problem

9 springs validate across 15 primals. Each spring needs different capabilities
from different primals. Without composition collapse, every spring (and every product)
must independently discover, connect to, and manage 427 methods.

The Neural API solves this with three layers:

```
Layer 1: Primals (427 methods, JSON-RPC)
    |
Layer 2: biomeOS Neural API (atomic signals, graph execution)
    |
Layer 3: Emergent systems (rootPulse, RPGPT, helixVision, etc.)
```

Products compose at Layer 3. The Neural API handles Layer 2. Primals provide Layer 1.
No product needs to know about all 427 methods — only the atomic signals relevant
to its domain.

---

## Atomic Tiers

| Tier | Scope | What It Provides | Primals |
|------|-------|-----------------|---------|
| **Tower** | Trust + identity | Authentication, signing, mesh federation | {{ entity(name="beardog") }}, {{ entity(name="songbird") }}, {{ entity(name="cellmembrane") }} |
| **Node** | Compute + dispatch | GPU ops, workload scheduling, shader compilation | {{ entity(name="barracuda") }}, {{ entity(name="toadstool") }}, {{ entity(name="coralreef") }}, {{ entity(name="squirrel") }} |
| **Nest** | Storage + provenance | Content-addressed storage, provenance DAG, attribution | {{ entity(name="nestgate") }}, {{ entity(name="rhizocrypt") }}, {{ entity(name="loamspine") }}, {{ entity(name="sweetgrass") }} |
| **NUCLEUS** | Full composition | All capabilities composed | All primals via {{ entity(name="biomeos") }} |

---

## Five Coordination Patterns

| Pattern | Execution Model | Use Case |
|---------|----------------|----------|
| **Sequential** | A -> B -> C | rootPulse 6-phase commit |
| **Parallel** | A + B + C simultaneously | Multi-spring validation |
| **ConditionalDag** | If A then B else C | Capability-dependent routing |
| **Pipeline** | A \| B \| C (streaming) | Continuous data processing |
| **Continuous** | 60 Hz feedback loop | Game sessions, live monitoring |

Graphs are defined in TOML:

```toml
[graph]
name = "rootpulse-commit"
pattern = "sequential"

[[graph.node]]
name = "health-check"
signal = "tower.health"
order = 1

[[graph.node]]
name = "dehydrate"
signal = "nest.dehydrate"
order = 2
depends_on = ["health-check"]

[[graph.node]]
name = "sign"
signal = "tower.sign"
order = 3
depends_on = ["dehydrate"]
```

---

## Bidirectional Learning

The Neural API has a learning loop:

1. **Forward pass** — execute the capability graph, collect metrics
2. **Backward pass** — feed metrics to the PathwayLearner
3. **Optimization** — discover patterns, collapse redundant paths, improve routing

Each execution produces data that makes the next execution more efficient. The
system learns which primals respond fastest, which capability paths produce the
best results, and which subgraphs can be parallelized.

---

## Composition Collapse

The philosophical commitment: when you can express an operation as a composition
of existing capabilities, you do not build a new service. You compose.

| Emergent System | What It Composes | Why Not a Service |
|----------------|-----------------|------------------|
| [rootPulse](@/architecture/rootpulse.md) | rhizoCrypt + loamSpine + sweetGrass + nestGate + bearDog | VCS is coordination, not a service |
| RPGPT | squirrel + petalTongue + rhizoCrypt | Game engine is primal composition |
| [NF pipeline](@/products/nf_case_study.md) | helixVision + healthSpring + initioChem | Multi-product science is composition |

None of these systems required new primals. They required new compositions of
existing primals — exactly what the Neural API enables.

---

## Implementation Status

| Phase | Status | What It Delivers |
|-------|--------|-----------------|
| 1 | **Complete** | Graph execution engine |
| 2 | **Complete** | MetricsCollector, basic routing |
| 3 | **Partial** | ConditionalDag, Pipeline execution |
| 3.5A | **Complete** | 32 composition graphs for atomic signals |
| 3.5B-D | Designed | Per-tier graph definitions |
| 4 | Specified | PathwayLearner wiring |
| 5 | Research | Self-evolution, pattern discovery |

---

## Emergent Systems

Systems that emerge from Neural API composition rather than being built:

| System | Domain | Status |
|--------|--------|--------|
| rootPulse | Version control | Provenance trio production-ready |
| RPGPT | Game engine | esotericWebb prototype |
| AlphaFold-class pipeline | Protein structure | {{ entity(name="helixvision") }} primitives validated |
| Barrick pipeline | Microbial evolution | {{ entity(name="guidestone") }}-LTEE designed |
| Field genomics | Environmental science | footPrint GIS live |

---

*The Neural API is the kernel substrate — domain-agnostic graph execution that
makes 427 primal methods composable into any system. Intelligence emerges from
simple components and feedback loops, not from complexity. The primals are simple.
The connections are rich. The results are emergent.*
