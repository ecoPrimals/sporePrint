+++
title = "NUCLEUS Composition Model"
description = "How primals compose into a system — the atomics ladder, Neural API, deploy graphs, Dark Forest, and Plasmodium."
date = 2026-03-31

[taxonomies]
primals = ["beardog", "barracuda", "biomeos", "coralreef", "loamspine", "nestgate", "rhizocrypt", "skunkbat", "songbird", "squirrel", "sweetgrass", "toadstool"]
+++

# NUCLEUS Composition Model

NUCLEUS is not a single binary. It is the **emergent state** when foundation primals are running and coordinated by biomeOS on a gate. This page describes how individual primals compose into a coherent system through named patterns, semantic routing, and cryptographic trust.

---

## The Atomics Ladder

Primals compose in layers. Each layer is a **named composition pattern** — not a separate product — defined by which primals coordinate and what behavior emerges.

### Tower Atomic

**Composition**: BearDog + Songbird  
**What emerges**: Pure Rust HTTPS — cryptographic identity + TLS + network mesh

Tower is the foundation of all networked communication. BearDog provides Ed25519 identity, key management, and genetic lineage trust. Songbird provides mesh networking, peer discovery, and federation. Together they give every gate a cryptographic identity and the ability to discover and communicate with other gates.

### Node Atomic

**Composition**: Tower + ToadStool (+ barraCuda, coralReef)  
**What emerges**: Hardware-aware sovereign compute

Node adds compute capability to Tower's networking. ToadStool discovers available hardware (CPU, GPU, NPU) and dispatches workloads. barraCuda provides the math (800+ WGSL f64 shaders), and coralReef compiles shaders to native GPU binaries. The boundary is precise: barraCuda writes math, coralReef compiles it, ToadStool dispatches it.

### Nest Atomic

**Composition**: Tower + NestGate  
**What emerges**: Secure, content-addressed storage

Nest adds persistent storage. NestGate provides content-addressed storage (CAS) with BLAKE3 hashing, deduplication, and integrity verification. Combined with Tower's networking, data can be stored locally and verified remotely.

### Full NUCLEUS

**Composition**: All 8 foundation primals  
**What emerges**: AI-coordinated sovereign computing

Full NUCLEUS is the complete foundation: networking (Tower), compute (Node), storage (Nest), orchestration (biomeOS), and AI coordination (Squirrel). biomeOS reads deploy graphs, germinates primals, wires capabilities, and routes requests via the Neural API. Squirrel — one of the eight — adds vendor-agnostic AI inference and MCP tool orchestration.

```
┌─────────────────────────────────────────────────┐
│                  Full NUCLEUS                    │
│                                                  │
│  ┌──────────────────────────────────────────┐   │
│  │  Squirrel — AI coordination (MCP)        │   │
│  └──────────────────────────────────────────┘   │
│  ┌──────────────────────────────────────────┐   │
│  │  biomeOS — orchestration, Neural API     │   │
│  └──────────────────────────────────────────┘   │
│                                                  │
│  ┌─────────────┐ ┌──────────┐ ┌────────────┐   │
│  │ Node Atomic │ │   Nest   │ │   Tower    │   │
│  │             │ │  Atomic  │ │  Atomic    │   │
│  │ ToadStool   │ │          │ │            │   │
│  │ barraCuda   │ │ NestGate │ │ BearDog    │   │
│  │ coralReef   │ │          │ │ Songbird   │   │
│  └─────────────┘ └──────────┘ └────────────┘   │
└─────────────────────────────────────────────────┘
```

---

## Neural API

biomeOS routes requests to primals using **semantic capability matching**, not hardcoded names. A consumer says what it needs (`math.matmul`, `shader.compile.wgsl`, `crypto.sign`), and biomeOS finds the primal that advertises that capability.

### How Routing Works

1. **Primals register capabilities** via JSON-RPC on startup (e.g., barraCuda registers `math.*`)
2. **Consumers request by domain** (e.g., `capability.call("math", "matmul")`)
3. **biomeOS resolves** the request to the right primal based on registered capabilities
4. **No primal knows** about other primals by name — only by capability

This means primals can be swapped, upgraded, or composed differently without changing consumers.

### Coordination Patterns

biomeOS executes **TOML deploy graphs** that define how primals coordinate:

| Pattern | Method | Behavior |
|---------|--------|----------|
| Sequential | `graph.execute` | Dependency-ordered execution |
| Parallel | `graph.execute` | Concurrent independent nodes |
| ConditionalDag | `graph.execute` | Branching with `condition` / `skip_if` |
| Pipeline | `graph.execute_pipeline` | Streaming via bounded channels (NDJSON) |
| Continuous | `graph.start_continuous` | Fixed timestep (e.g., 60 Hz game loops) |

### Learning

biomeOS includes a **PathwayLearner** that uses execution metrics to suggest optimizations: parallelization opportunities, prewarming, batching, and caching. The system learns how primals interact and improves routing over time.

---

## Deploy Graphs

A deploy graph is a **TOML DAG** that tells biomeOS what to run:

```toml
[[nodes]]
name = "crypto"
primal = "beardog"
capabilities = ["crypto.sign", "crypto.verify"]

[[nodes]]
name = "compute"
primal = "toadstool"
capabilities = ["compute.dispatch"]
depends_on = ["crypto"]

[[nodes]]
name = "storage"
primal = "nestgate"
capabilities = ["storage.put", "storage.get"]
depends_on = ["crypto"]
```

biomeOS reads the graph, **germinates** each primal (starts it, waits for IPC socket, confirms `health.check`), wires capabilities according to edges, and handles graceful degradation if optional nodes are absent.

### Niches

A **niche** is a BYOB deployment — a specific composition of primals for a specific purpose. Defined by a deploy graph + niche YAML + capability domains. Examples:

| Niche | Composition | Purpose |
|-------|-------------|---------|
| Sovereign Compute | Node Atomic + biomeOS | GPU-accelerated science workloads |
| Structural Genomics | Node + Nest + helixVision + blueFish | Local protein structure prediction pipeline |
| CRPG | Tower + rhizoCrypt + loamSpine + sweetGrass | esotericWebb game runtime |
| Full Lab | Full NUCLEUS + all products | Complete sovereign scientific computing |

### Germination

Starting a primal until it is ready for requests:

1. biomeOS runs the primal's `server` subcommand
2. Waits for the IPC socket to appear
3. Calls `health.check` to confirm readiness
4. Registers the primal's advertised capabilities
5. Wires capability routes according to the deploy graph

The analogy: a seed germinates in a niche on a gate.

---

## Dark Forest

ecoPrimals uses a **zero-metadata-leakage** discovery protocol. The goal: observers should not be able to tell that communication is occurring.

### Genetic Lineage

BearDog manages two kinds of cryptographic material:

- **Nuclear DNA** (family seed): Shared identity and permissions within a family of gates. Auto-trust within family, zero trust outside.
- **Mitochondrial DNA** (beacon seeds): Used for Dark Forest discovery — finding peers without revealing your existence to observers.

### Trust Model

- **Within family**: Auto-trust via shared family seed (BearDog verification)
- **Cross-family**: Zero trust by default; trust must be explicitly established
- **Network observers**: Cannot determine that communication is occurring (Dark Forest property)

The Dark Forest protocol is complemented by skunkBat's active threat detection — Dark Forest handles discovery privacy, skunkBat handles defensive security within the sovereign environment.

---

## Plasmodium: Multi-Gate Collectives

When two or more NUCLEUS instances **bond**, they form a **Plasmodium** — a collective that shares capabilities, models, and load without a central coordinator. Named after *Physarum polycephalum* (slime mold): no central brain, collective behavior, graceful degradation.

### How It Works

1. Local biomeOS queries **Songbird mesh** for bonded peers
2. Connects to their NUCLEUS instances
3. Aggregates capabilities, models, and resource availability
4. Routes workloads to the best gate by capability match, resources, and model affinity

### Properties

- **No master**: Any gate can query; any gate can leave
- **Dynamic membership**: Gates join and leave without disrupting the collective
- **Capability aggregation**: If Gate A has a Titan V and Gate B has an RTX 4070, the Plasmodium can route GPU workloads to whichever is better suited
- **Trust**: Inherited from BearDog genetic lineage — only bonded gates participate

---

## Post-NUCLEUS Composition

Five primals build emergent behaviors on top of NUCLEUS:

### RootPulse — Distributed Version Control

**Composition**: rhizoCrypt (ephemeral DAG) + loamSpine (immutable history) + NestGate (CAS blobs) + BearDog (signing) + sweetGrass (attribution) + Songbird (federation)  
**Coordinator**: biomeOS via Neural API

RootPulse is distributed version control as an emergent behavior. No primal contains a "VCS" — the behavior emerges from coordinating primals that each own one piece: ephemeral workspace, permanent history, blob storage, identity, attribution, discovery.

### Memory & Attribution Stack

**Composition**: rhizoCrypt + loamSpine + sweetGrass  
**Coordinator**: biomeOS

The temporal data management system: rhizoCrypt provides ephemeral working memory, loamSpine provides permanent history, sweetGrass tracks attribution. Together they form a complete provenance chain from first draft to permanent record.

---

## The Key Insight

NUCLEUS is composition, not aggregation. Each primal is a self-contained Rust binary with JSON-RPC capabilities. biomeOS discovers what is available, wires it according to deploy graphs, and routes requests by capability. Higher behaviors (RootPulse, Plasmodium) emerge from the same primitives and orchestration — not from enlarging a single binary.

The practical consequence: you deploy exactly what you need. A Tower Atomic for networking. A Node Atomic for GPU compute. A Full NUCLEUS for everything. The same primals, the same code, composed differently for different purposes.

---

*See also: [Primal Catalog](@/architecture/PRIMAL_CATALOG.md) for individual primal details,
[Deployment Model](@/architecture/DEPLOYMENT_MODEL.md) for BYOB binary distribution,
[Ecosystem Inventory](@/architecture/ECOSYSTEM_INVENTORY.md) for the full repository map.*
