+++
title = "baseCamp 28 — Primal Composition as Scientific Methodology"
description = "11 experiments across 9 deploy graphs demonstrating that primal composition itself is a scientific methodology — emergent validation from constraint-driven architecture."
date = 2026-07-09

[taxonomies]
primals = ["biomeos", "barracuda", "toadstool", "nestgate", "songbird", "beardog"]
springs = ["primalspring", "hotspring", "wetspring"]
+++

{{ maturity(level="architectural") }}

**Date:** April 3, 2026
**Author:** ecoPrimals project
**Status:** Validated — 11 experiments (90 checks), 9 deploy graphs, 9 proto sketches
**License:** AGPL-3.0-or-later

---

## Abstract

gen3 validated that sovereign Rust+GPU infrastructure computes correct science
(Papers 01-22). This paper documents the next methodological question: **how do
independently evolved primals compose into systems that no single primal can
provide alone, and how do springs prove those compositions without owning
primal-layer code?**

ludoSpring V15-V18 answers this through 11 experiments, 9 NUCLEUS deploy graphs,
a typed composition recipe pattern, a BYOB deploy graph schema standard, and an
ecosystem evolution gap map. The methodology is domain-agnostic — game science
was the forcing function, but the composition patterns apply to any spring or
garden.

---

## 1. The Composition Problem

### 1.1 From Validation to Composition

Papers 01-16 proved springs can faithfully port Python science to Rust+GPU.
Papers 17-22 proved cross-domain provenance patterns work within a single spring.
But the ecosystem promise is **primal composition**: combine rhizoCrypt (DAG) +
loamSpine (certificates) + BearDog (signing) + biomeOS (orchestration) into a
system that tracks game sessions, biological samples, or medical records —
using the same code path for all three.

The composition problem has three constraints:

1. **No shared crates** — springs cannot import primals as Cargo dependencies
   for composition logic. Shared crates create dependency violations. Each layer
   (primals, springs, gardens) independently implements from documented standards.

2. **No primal ownership** — springs prove patterns and hand them off. They
   do not own, merge, or maintain primal-layer code. A spring's `deploy` module
   is a reference implementation, not the canonical one.

3. **Independent evolution** — each primal, spring, and garden evolves on its
   own timeline. Composition must tolerate version drift, missing primals, and
   partial availability.

### 1.2 The NUCLEUS Model

primalSpring (Paper 23) established the NUCLEUS composition architecture:
Tower (security + discovery), Node (compute), Nest (storage), FullNucleus
(all three). biomeOS orchestrates these via deploy graphs — TOML files declaring
nodes, dependencies, startup order, capabilities, and health checks.

ludoSpring's contribution is proving this model works for **domain science
composition** — not just structural coordination.

---

## 2. Methodology: Typed Deploy Graph Composition

### 2.1 Deploy Graph Typing (V15)

ludoSpring implements typed deploy graph parsing without importing primalSpring:

```
DeployGraph { name, nodes: Vec<GraphNode> }
GraphNode { name, binary, order, required, depends_on, health_method,
            by_capability, capabilities }
```

The `topological_waves()` function groups nodes by dependency depth, producing
wave-ordered startup sequences. This is independently derived from the same
TOML schema that primalSpring uses — proving the schema is implementable
without code coupling.

**Evidence:** exp067 (7 checks) — parses all ludoSpring deploy graphs, validates
topological ordering, probes live Tower when available.

### 2.2 Session Lifecycle IPC (V15)

Four JSON-RPC methods enable session-aware game science composition:

| Method | Purpose |
|--------|---------|
| `game.begin_session` | Initialize session with flow/DDA/engagement state |
| `game.complete_session` | Finalize and return aggregate metrics |
| `game.session_state` | Query current session snapshot |
| `game.tick_health` | Tick budget health for continuous graphs |

**Evidence:** exp069 (8 checks) — round-trips session lifecycle both locally
and via IPC.

### 2.3 Composition Recipe Pattern (V16)

The `deploy::recipe` module codifies the full validation cycle:

```
Parse graph -> Validate structure -> Topological waves -> Discover by capability
-> Walk waves (health probe) -> Run domain science -> Report
```

`validate_composition()` returns a `CompositionReport` with per-node health
status, capability satisfaction, and readiness summary. This is the bridge
between primalSpring's deployment models and garden consumption.

**Evidence:** exp072 (17 checks) — the reference implementation gardens
replicate. Graph-driven, wave-ordered, session lifecycle, composition report.

### 2.4 Primal Math Parity (V15)

exp068 validates that local barraCuda math (sigmoid, dot, lcg_step) produces
identical results whether called as a library or via IPC to a running primal.
This proves composition does not degrade mathematical fidelity.

**Evidence:** exp068 (11 checks), exp070 (2 checks — 100-tick 60Hz budget
under composition).

---

## 3. Novel Multi-Primal Compositions (V17)

V17 maps every primal to game science uses and identifies five compositions
that no single primal can provide:

### 3.1 Sovereign Save System

rhizoCrypt (DAG of actions) + loamSpine (certified milestones) + BearDog
(signed commits). Save file is a signed provenance chain — not a JSON blob.
Isomorphic to field genomics chain-of-custody (Paper 21) and medical access
logs (Paper 22).

**Evidence:** exp073 (12 checks), deploy graph `ludospring_sovereign_session.toml`.

### 3.2 Session-as-Primal

sourDough scaffolds ephemeral primals with `PrimalLifecycle` traits. biomeOS
manages their lifecycle. Game sessions, NPCs, mods, and multiplayer matches
become first-class biomeOS citizens with rhizoCrypt DAGs that outlive runtime.

**Evidence:** exp074 (6 checks), deploy graph `ludospring_session_primal.toml`.
Specification: `sourDough/specs/EPHEMERAL_PRIMAL_SCAFFOLDING.md`.

### 3.3 Attributed AI Narration

Squirrel (AI) + sweetGrass (attribution) + rhizoCrypt (DAG for context).
AI-generated text carries provable attribution — which model, which prompt,
which player action triggered it. ORC/CC-BY-SA license compliance is
structural, not manual.

**Evidence:** exp075 (8 checks), deploy graph `ludospring_narration.toml`.

### 3.4 Live Science Dashboard

ludoSpring (game science) + petalTongue (visualization) + rhizoCrypt (metric
provenance). Real-time flow/DDA/engagement dashboard where every data point
traces to a specific game tick in a specific session DAG.

**Evidence:** exp076 (4 checks), deploy graph `ludospring_live_viz.toml`.

### 3.5 Sovereign Render Pipeline

toadStool (hardware) + coralReef (shader compiler) + barraCuda (GPU compute).
Hardware-aware dispatch where the composition queries real silicon capabilities
before routing compute workloads.

**Evidence:** exp077 (11 checks), deploy graph `ludospring_hardware.toml`.

---

## 4. Proto Sketch Taxonomy (V18)

Springs provide proto deploy graph sketches for gardens to absorb, mirroring
the pattern primalSpring used for ludoSpring:

| Sketch | From | Garden gets |
|--------|------|-------------|
| Sovereign Save | exp073 | Signed DAG saves |
| Session-as-Primal | exp074 | Sessions as biomeOS citizens |
| Attributed Narration | exp075 | AI DM with attribution |
| Science Overlay | exp076 | Live flow/DDA dashboard |
| Sovereign Render | exp077 | Hardware-aware GPU pipeline |
| Multiplayer | exp073+074 | Multi-gate shared provenance |
| Creative Studio | exp076+073 | Content authoring with certs |

Plus 2 validation graph sketches. esotericWebb absorbs these as starting
patterns, adapts to its product context, and evolves independently.

---

## 5. BYOB Deploy Graph Schema (V18)

### 5.1 The Two-Schema Problem

Two deploy graph TOML schemas emerged independently:

| Schema | Origin | Used by |
|--------|--------|---------|
| BYOB (`[[graph.node]]`) | ludoSpring/primalSpring | Springs, gardens |
| Execution (`[[nodes]]`) | biomeOS | Runtime orchestration |

Springs cannot import biomeOS's execution schema (dependency violation).
biomeOS cannot require springs to use its internal format (sovereignty
violation).

### 5.2 Resolution: wateringHole Standard

The BYOB `[[graph.node]]` schema is standardized as
`wateringHole/BYOB_DEPLOY_GRAPH_SCHEMA.md` — an RFC-like document that each
layer independently implements. No shared crate. biomeOS absorbs native
`[[graph.node]]` ingestion so gardens can submit BYOB graphs directly.

This is the ecosystem's first formal deployment contract: documented once,
implemented independently by every layer.

---

## 6. Gap Discovery Methodology (V18)

### 6.1 How Composition Surfaces Gaps

Every composition experiment that encounters a missing capability, a startup
failure, or a protocol mismatch is documenting an evolution gap. The V15-V18
arc surfaced gaps in every ecosystem layer:

- **biomeOS:** Neural API health probes not wired, graph executors not on
  JSON-RPC, BYOB ingestion missing
- **Squirrel:** No NPC personality cert constraint API
- **petalTongue:** No dialogue scene support
- **Songbird:** No capability-filtered discovery
- **Provenance trio:** rhizoCrypt UDS not available, loamSpine startup panics

### 6.2 The Ecosystem Evolution Map

`ludoSpring/specs/ECOSYSTEM_EVOLUTION_MAP.md` maps each gap to:
- Owner (which primal/spring/garden)
- Priority (P0/P1/P2)
- Evidence (which experiment discovered it)
- Resolution path (what the owner needs to implement)

This produces per-layer evolution task handoffs:
- **Upstream:** primal teams receive specific gaps with reproduction steps
- **Downstream:** gardens receive proto sketches with absorption instructions
- **Cross-cutting:** wateringHole receives schema standards

---

## 7. The Sovereignty Constraint

### 7.1 Why This Matters

The composition methodology is constrained by a fundamental ecosystem rule:
**no shared crates between layers.** Springs do not import primals for
composition logic. Gardens do not import springs. Each layer implements from
documented standards (wateringHole) and validates via IPC.

This constraint is not a limitation — it is the methodology's strength. It
guarantees that:

1. Primals can evolve without breaking springs
2. Springs can experiment without polluting primals
3. Gardens can ship products without coupling to spring internals
4. The ecosystem scales beyond any single maintainer's attention

### 7.2 What Springs Produce

Springs produce three types of artifacts for ecosystem consumption:

1. **Validated experiments** — proof that a composition works (code + checks)
2. **Deploy graphs** — TOML declaring what primals compose (schema + structure)
3. **Proto sketches** — starting patterns for the next layer (garden-ready)

Springs do not produce shared libraries, canonical schemas, or primal patches.
Those are wateringHole standards and primal team responsibilities, respectively.

---

## 8. Cross-References

| Reference | Location |
|-----------|----------|
| NUCLEUS model | primalSpring Paper 23 |
| Deploy graphs | `ludoSpring/graphs/` (9 graphs) |
| Proto sketches | `ludoSpring/graphs/sketches/` (9 sketches) |
| BYOB schema | `wateringHole/BYOB_DEPLOY_GRAPH_SCHEMA.md` |
| Evolution map | `ludoSpring/specs/ECOSYSTEM_EVOLUTION_MAP.md` |
| Leverage map | `ludoSpring/specs/PRIMAL_LEVERAGE_MAP.md` |
| Ephemeral primals | `sourDough/specs/EPHEMERAL_PRIMAL_SCAFFOLDING.md` |
| Upstream handoff | `wateringHole/handoffs/LUDOSPRING_V18_UPSTREAM_EVOLUTION_HANDOFF_APR03_2026.md` |
| Downstream handoff | `wateringHole/handoffs/LUDOSPRING_V18_DOWNSTREAM_EVOLUTION_HANDOFF_APR03_2026.md` |
| Cross-spring handoff | `wateringHole/handoffs/LUDOSPRING_V18_CROSS_SPRING_EVOLUTION_HANDOFF_APR03_2026.md` |

---

## 9. Conclusion

Primal composition is not just an engineering task — it is a scientific
methodology. The constraints (no shared crates, no primal ownership, independent
evolution) produce a system where every composition is an experiment, every gap
is a discovery, and every handoff is a publication.

ludoSpring proved this methodology using game science as the forcing function.
The methodology itself is domain-agnostic. Any spring that wants to compose
primals can follow the same path: type the deploy graph, build the recipe,
run the experiments, document the gaps, create the proto sketches.

The science that got us here is gen3. The deployment surface it enables is gen4.

---

*gen3 asked "does it work?" for single primals. Paper 26 asks "does it work?"
for primal compositions. The answer is yes — with 11 experiments, 9 deploy
graphs, and a methodology any spring can replicate.*

---

## Addendum: Operational Validation (May 2026)

The composition methodology described above was operationally validated through
projectNUCLEUS on ironGate hardware. A Nest Atomic + ToadStool composition
(9 primals) ran the full ABG bioinformatics pipeline — 235+ checks across
10 workloads processing real NCBI data (PRJNA488170, 11.9M reads) — with every
artifact and step wrapped in provenance:

- **BLAKE3 content hashes** for all NCBI FASTQs and workload outputs (NestGate)
- **rhizoCrypt DAG session** tracking 24 events (data registration + workload execution + results)
- **loamSpine permanent ledger** commit (SessionCommit with Merkle root of the DAG)
- **sweetGrass attribution braid** with ed25519 witness signature (W3C PROV-O JSON-LD)

This closes the full validation arc: published science → Python ground truth →
Rust parity → primal composition dispatch → provenance-verified reproducibility.
The provenance pipeline uses direct JSON-RPC over TCP/HTTP to the trio (not
Neural API routing), validating the "no shared crates" independence principle —
the wrapper script composes primals purely through their documented RPC interfaces.

---

**See also:**

- [System Architecture](@/thesis/05_system_architecture.md) — NUCLEUS composition model and deploy graph orchestration
- [Quantitative Evidence](@/thesis/13_quantitative_evidence.md) — validation metrics across the spring framework
- [Composition Patterns](@/architecture/COMPOSITION_PATTERNS.md) — typed deploy graph recipes and sovereignty constraints
