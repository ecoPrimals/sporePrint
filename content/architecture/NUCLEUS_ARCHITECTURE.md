+++
title = "NUCLEUS Composition Model"
weight = 8
description = "How primals compose into a system — the atomics ladder, Neural API, deploy graphs, Dark Forest, and Plasmodium."
date = 2026-03-31

[taxonomies]
primals = ["beardog", "barracuda", "biomeos", "coralreef", "loamspine", "nestgate", "petaltongue", "rhizocrypt", "skunkbat", "songbird", "squirrel", "sweetgrass", "toadstool"]
+++

{{ entity(name="nucleus") }} is not a single binary. It is the **emergent state** when foundation primals are running and coordinated by {{ entity(name="biomeos") }} on a gate. This page describes how individual primals compose into a coherent system through named patterns, semantic routing, and cryptographic trust.

---

## Composition Diagram

<div id="viz-nucleus" class="viz-container">
{{ viz_embed(src="/viz/nucleus-composition", caption="NUCLEUS composition layers: primals, springs, and deploy graph relationships") }}
</div>

<script type="module" src="/js/viz-hydrate.js"></script>

## The Atomics Ladder

Primals compose in layers. Each layer is a **named composition pattern** — not a separate product — defined by which primals coordinate and what behavior emerges.

### Tower Atomic

**Composition**: {{ entity(name="beardog") }} + {{ entity(name="songbird") }}  
**What emerges**: Pure Rust HTTPS — cryptographic identity + TLS + network mesh

Tower is the foundation of all networked communication. {{ entity(name="beardog") }} provides Ed25519 identity, key management, and genetic lineage trust. {{ entity(name="songbird") }} provides mesh networking, peer discovery, and federation. Together they give every gate a cryptographic identity and the ability to discover and communicate with other gates.

### Node Atomic

**Composition**: Tower + {{ entity(name="toadstool") }} (+ {{ entity(name="barracuda") }}, {{ entity(name="coralreef") }})  
**What emerges**: Hardware-aware sovereign compute

Node adds compute capability to Tower's networking. {{ entity(name="toadstool") }} discovers available hardware (CPU, GPU, NPU) and dispatches workloads. {{ entity(name="barracuda") }} provides the math (800+ WGSL f64 shaders), and {{ entity(name="coralreef") }} compiles shaders to native GPU binaries. The boundary is precise: {{ entity(name="barracuda") }} writes math, {{ entity(name="coralreef") }} compiles it, {{ entity(name="toadstool") }} dispatches it.

### Nest Atomic

**Composition**: Tower + {{ entity(name="nestgate") }}  
**What emerges**: Secure, content-addressed storage

Nest adds persistent storage. {{ entity(name="nestgate") }} provides content-addressed storage (CAS) with BLAKE3 hashing, deduplication, and integrity verification. Combined with Tower's networking, data can be stored locally and verified remotely.

### Full NUCLEUS

**Composition**: All 8 foundation primals  
**What emerges**: AI-coordinated sovereign computing

Full {{ entity(name="nucleus") }} is the complete foundation: networking (Tower), compute (Node), storage (Nest), orchestration ({{ entity(name="biomeos") }}), and AI coordination ({{ entity(name="squirrel") }}). {{ entity(name="biomeos") }} reads deploy graphs, germinates primals, wires capabilities, and routes requests via the {{ entity(name="neuralapi") }}. {{ entity(name="squirrel") }} — one of the eight — adds vendor-agnostic AI inference and MCP tool orchestration.

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

{{ entity(name="biomeos") }} routes requests to primals using **semantic capability matching**, not hardcoded names. A consumer says what it needs (`math.matmul`, `shader.compile.wgsl`, `crypto.sign`), and {{ entity(name="biomeos") }} finds the primal that advertises that capability.

### How Routing Works

1. **Primals register capabilities** via JSON-RPC on startup (e.g., {{ entity(name="barracuda") }} registers `math.*`)
2. **Consumers request by domain** (e.g., `capability.call("math", "matmul")`)
3. **{{ entity(name="biomeos") }} resolves** the request to the right primal based on registered capabilities
4. **No primal knows** about other primals by name — only by capability

This means primals can be swapped, upgraded, or composed differently without changing consumers.

### Coordination Patterns

{{ entity(name="biomeos") }} executes **TOML deploy graphs** that define how primals coordinate:

| Pattern | Method | Behavior |
|---------|--------|----------|
| Sequential | `graph.execute` | Dependency-ordered execution |
| Parallel | `graph.execute` | Concurrent independent nodes |
| ConditionalDag | `graph.execute` | Branching with `condition` / `skip_if` |
| Pipeline | `graph.execute_pipeline` | Streaming via bounded channels (NDJSON) |
| Continuous | `graph.start_continuous` | Fixed timestep (e.g., 60 Hz game loops) |

### Learning

{{ entity(name="biomeos") }} includes a **PathwayLearner** that uses execution metrics to suggest optimizations: parallelization opportunities, prewarming, batching, and caching. The system learns how primals interact and improves routing over time.

---

## Deploy Graphs

A deploy graph is a **TOML DAG** that tells {{ entity(name="biomeos") }} what to run:

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

{{ entity(name="biomeos") }} reads the graph, **germinates** each primal (starts it, waits for IPC socket, confirms `health.check`), wires capabilities according to edges, and handles graceful degradation if optional nodes are absent.

### Niches

A **niche** is a {{ entity(name="byob") }} deployment — a specific composition of primals for a specific purpose. Defined by a deploy graph + niche YAML + capability domains. Examples:

| {{ entity(name="niche") }} | Composition | Purpose |
|-------|-------------|---------|
| Sovereign Compute | {{ entity(name="nodeatomic") }} + {{ entity(name="biomeos") }} | GPU-accelerated science workloads |
| Structural Genomics | Node + Nest + {{ entity(name="helixvision") }} + blueFish | Local protein structure prediction pipeline |
| CRPG | Tower + {{ entity(name="rhizocrypt") }} + {{ entity(name="loamspine") }} + {{ entity(name="sweetgrass") }} | {{ entity(name="esotericwebb") }} game runtime |
| Full Lab | Full {{ entity(name="nucleus") }} + all products | Complete sovereign scientific computing |

### Deployment Compositions

Niches define abstract compositions for purpose. **Deployment compositions** are
the concrete instances running on gates — each maps to a niche profile with
specific primals and operational roles:

| Composition | Primals | Gate Example | Purpose |
|-------------|---------|--------------|---------|
| **Full {{ entity(name="nucleus") }}** | All {{ total_stat(stat="total_primals") }} | eastGate, ironGate | Complete sovereign stack — all capabilities |
| **Tower** | {{ entity(name="beardog") }} + {{ entity(name="songbird") }} + {{ entity(name="skunkbat") }} | grapheneGate, new gates | Minimal secure mesh entry point |
| **JupyterHub host** | {{ entity(name="songbird") }} (drawbridge) + {{ entity(name="beardog") }} + {{ entity(name="biomeos") }} | ironGate | `lab.primals.eco` via mesh relay |
| **sporePrint host** | {{ entity(name="petaltongue") }} + {{ entity(name="nestgate") }} + {{ entity(name="songbird") }} + {{ entity(name="beardog") }} | golgi VPS | Sovereign website with live mesh visualization |
| **Cold storage** | {{ entity(name="nestgate") }} + {{ entity(name="sweetgrass") }} + {{ entity(name="rhizocrypt") }} | westGate | ZFS CAS archive with provenance |
| **Compute dispatch** | {{ entity(name="toadstool") }} + {{ entity(name="barracuda") }} + {{ entity(name="coralreef") }} + {{ entity(name="biomeos") }} | strandGate | GPU/CPU compute mesh node |

The {{ entity(name="songbird") }} **drawbridge** pattern enables capability-based routing
into a composition: `SONGBIRD_DRAWBRIDGE_ROUTES=/hub=jupyter,/api=inference` makes
songBird auto-register capabilities at startup and announce them to mesh peers.
Remote gates can then `capability.call("jupyter")` — songBird routes to the
local drawbridge endpoint.

Each deployment composition has a matching **projectNUCLEUS** deploy graph that
codifies the exact primal set, launch ordering, and health checks.

### Germination

Starting a primal until it is ready for requests:

1. {{ entity(name="biomeos") }} runs the primal's `server` subcommand
2. Waits for the IPC socket to appear
3. Calls `health.check` to confirm readiness
4. Registers the primal's advertised capabilities
5. Wires capability routes according to the deploy graph

The analogy: a seed germinates in a niche on a gate.

---

## Dark Forest

{{ entity(name="ecoprimals") }} uses a **zero-metadata-leakage** discovery protocol. The goal: observers should not be able to tell that communication is occurring.

### Genetic Lineage

{{ entity(name="beardog") }} manages two kinds of cryptographic material:

- **Nuclear DNA** (family seed): Shared identity and permissions within a family of gates. Auto-trust within family, zero trust outside.
- **Mitochondrial DNA** (beacon seeds): Used for {{ entity(name="darkforest") }} discovery — finding peers without revealing your existence to observers.

### Trust Model

- **Within family**: Auto-trust via shared family seed ({{ entity(name="beardog") }} verification)
- **Cross-family**: Zero trust by default; trust must be explicitly established
- **Network observers**: Cannot determine that communication is occurring ({{ entity(name="darkforest") }} property)

The {{ entity(name="darkforest") }} protocol is complemented by {{ entity(name="skunkbat") }}'s active threat detection — {{ entity(name="darkforest") }} handles discovery privacy, {{ entity(name="skunkbat") }} handles defensive security within the sovereign environment.

---

## Plasmodium: Multi-Gate Collectives

When two or more {{ entity(name="nucleus") }} instances **bond**, they form a **{{ entity(name="plasmodium") }}** — a collective that shares capabilities, models, and load without a central coordinator. Named after *Physarum polycephalum* (slime mold): no central brain, collective behavior, graceful degradation.

### How It Works

1. Local {{ entity(name="biomeos") }} queries **{{ entity(name="songbird") }} mesh** for bonded peers
2. Connects to their {{ entity(name="nucleus") }} instances
3. Aggregates capabilities, models, and resource availability
4. Routes workloads to the best gate by capability match, resources, and model affinity

### Properties

- **No master**: Any gate can query; any gate can leave
- **Dynamic membership**: Gates join and leave without disrupting the collective
- **Capability aggregation**: If Gate A has a Titan V and Gate B has an RTX 4070, the {{ entity(name="plasmodium") }} can route GPU workloads to whichever is better suited
- **Trust**: Inherited from {{ entity(name="beardog") }} genetic lineage — only bonded gates participate

---

## Post-NUCLEUS Composition

Five primals build emergent behaviors on top of {{ entity(name="nucleus") }}:

### RootPulse — Distributed Version Control

**Composition**: {{ entity(name="rhizocrypt") }} (ephemeral DAG) + {{ entity(name="loamspine") }} (immutable history) + {{ entity(name="nestgate") }} (CAS blobs) + {{ entity(name="beardog") }} (signing) + {{ entity(name="sweetgrass") }} (attribution) + {{ entity(name="songbird") }} (federation)  
**Coordinator**: {{ entity(name="biomeos") }} via {{ entity(name="neuralapi") }}

{{ entity(name="rootpulse") }} is distributed version control as an emergent behavior. No primal contains a "VCS" — the behavior emerges from coordinating primals that each own one piece: ephemeral workspace, permanent history, blob storage, identity, attribution, discovery.

### Memory & Attribution Stack

**Composition**: {{ entity(name="rhizocrypt") }} + {{ entity(name="loamspine") }} + {{ entity(name="sweetgrass") }}  
**Coordinator**: {{ entity(name="biomeos") }}

The temporal data management system: {{ entity(name="rhizocrypt") }} provides ephemeral working memory, {{ entity(name="loamspine") }} provides permanent history, {{ entity(name="sweetgrass") }} tracks attribution. Together they form a complete provenance chain from first draft to permanent record.

---

## The Key Insight

{{ entity(name="nucleus") }} is composition, not aggregation. Each primal is a self-contained Rust binary with JSON-RPC capabilities. {{ entity(name="biomeos") }} discovers what is available, wires it according to deploy graphs, and routes requests by capability. Higher behaviors ({{ entity(name="rootpulse") }}, {{ entity(name="plasmodium") }}) emerge from the same primitives and orchestration — not from enlarging a single binary.

The practical consequence: you deploy exactly what you need. A {{ entity(name="toweratomic") }} for networking. A {{ entity(name="nodeatomic") }} for GPU compute. A Full {{ entity(name="nucleus") }} for everything. The same primals, the same code, composed differently for different purposes.

### Thin Relay: NUCLEUS for Hosting (Wave 134c)

sporePrint itself runs on {{ entity(name="nucleus") }} infrastructure — {{ entity(name="nestgate") }}
serves the static site on any gate that includes it. The **thin-relay**
composition profile formalizes this: a VPS or edge node running {{ entity(name="songbird") }}
(mesh relay), {{ entity(name="nestgate") }} (sporePrint hosting), and membrane (cascade
auto-fetch) provides a sovereign web presence without a full {{ entity(name="nucleus") }}.

As sporePrint evolves toward richer interactive features (guideStone artifacts,
{{ entity(name="squirrel") }} AI chat, live {{ entity(name="barracuda") }} visualizations), the thin-relay
naturally grows toward a full {{ entity(name="nucleus") }} — adding primals incrementally,
driven by what the website needs. The composition model makes this seamless:
update the composition field in the manifest, and the gate starts running the
additional primals.

```
thin-relay                  → full NUCLEUS
songBird + nestGate + membrane → + squirrel (AI) → + petalTongue (rendering)
                                → + toadStool (compute) → + barraCuda (GPU viz)
```

---

*See also: [Primal Catalog](@/architecture/PRIMAL_CATALOG.md) for individual primal details,
[Deployment Model](@/architecture/DEPLOYMENT_MODEL.md) for {{ entity(name="byob") }} binary distribution,
[Ecosystem Inventory](@/architecture/ECOSYSTEM_INVENTORY.md) for the full repository map.*
