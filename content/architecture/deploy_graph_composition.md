+++
title = "Deploy Graph Composition"
description = "How deploy graphs become the interface between infrastructure (primals) and products (gardens) — BYOB TOML, topological sort, and session-as-primal."
date = 2026-04-20
weight = 35

[taxonomies]
trails = ["nf-pipeline"]

[extra]
foundation = true
domain = "Architecture"
maturity = "architectural"

[[extra.companions]]
url = "/products/nf-case-study/"
title = "NF Case Study"
relation = "validates"
label = "First multi-product composition using deploy graphs"

[[extra.companions]]
url = "/architecture/creative-surface/"
title = "Creative Surface"
relation = "pairs_with"
label = "Where deploy graphs meet the sporeGarden surface"
+++

## The gen4 Transition

In gen3, deploy graphs were test fixtures — static configurations that proved primals
could compose. In gen4, deploy graphs become the **product interface**: the mechanism
by which products ({{ entity(name="helixvision") }}, {{ entity(name="initiochem") }}, esotericWebb) consume
primals without source coupling.

---

## BYOB Schema

Products declare their primal requirements in a TOML deploy graph:

```toml
[graph]
name = "helix-vision-composition"
version = "0.1.0"

[[graph.node]]
name = "barracuda"
binary = "barracuda"
order = 1
capabilities = ["tensor.multiply", "tensor.fft", "tensor.eigendecompose"]

[[graph.node]]
name = "toadstool"
binary = "toadstool"
order = 2
depends_on = ["barracuda"]
capabilities = ["dispatch.submit", "dispatch.status"]

[[graph.node]]
name = "coralreef"
binary = "coralreef"
order = 2
depends_on = ["barracuda"]
capabilities = ["shader.compile", "shader.validate"]

[[graph.node]]
name = "nestgate"
binary = "nestgate"
order = 3
depends_on = ["toadstool"]
capabilities = ["cas.store", "cas.retrieve", "cas.verify"]
```

The product's launcher reads this TOML, performs a topological sort on dependencies,
spawns primals in order, and injects a PrimalBridge for IPC. The product never imports
primal code — it communicates via JSON-RPC over TCP sockets.

---

## Sovereignty Guarantee

The deploy graph enforces a critical boundary:

- **No shared crates** — products do not depend on primal source code
- **IPC-only interface** — all communication is JSON-RPC over TCP
- **Independent evolution** — primals can upgrade without product recompilation
- **Binary-only consumption** — products consume {{ entity(name="ecobin") }} binaries, not source

This means a product can be written in any language, by any team, consuming primals
it did not build. The deploy graph is the contract. JSON-RPC is the protocol.

---

## Session-as-Primal

In interactive products (games, creative tools), sessions themselves become ephemeral
{{ entity(name="biomeos") }} citizens:

- Game sessions register with {{ entity(name="biomeos") }} lifecycle management
- NPCs, matches, and worlds have {{ entity(name="rhizocrypt") }} provenance DAGs
- Creative content produces {{ entity(name="loamspine") }} certificates
- Session lifecycle follows the same spawn/health/shutdown as any primal

This means the same infrastructure that manages a molecular dynamics simulation
manages a game session. The patterns are isomorphic — both are workloads with
lifecycle, provenance, and capability requirements.

---

## Ecosystem Evolution Map

Deploy graphs create a feedback loop:

```
Product composition discovers capability gaps
    -> Gaps become tasks distributed to primals and springs
    -> Primals evolve new capabilities
    -> Springs validate new capabilities
    -> Products recompose with expanded capability set
    -> Next product discovers next gap
```

Each product composition stress-tests the ecosystem in a different domain. The
[NF case study](@/products/nf_case_study.md) is the gen5 exemplar: four products
composing to serve a biological question that no single product could answer.

---

## The GarageBand Moment

The deploy graph is sheet music. The primals are instruments. The product is the song.

A musician does not build a piano to write a concerto. They learn the instrument's
capabilities and compose within them. A product builder does not build {{ entity(name="barracuda") }}
to do protein structure prediction — they compose a deploy graph that declares what
GPU capabilities the product needs, and the ecosystem provides the instruments.

The "GarageBand moment" is when the deploy graph tooling becomes simple enough that
a domain scientist — not a systems programmer — can compose primals into a product
that serves their question. The BYOB TOML schema is the first step: declarative,
human-readable, versionable, and self-documenting.

---

*Deploy graphs are not configuration files. They are the interface between
infrastructure and science — the boundary where sovereign compute meets domain
questions, mediated by TOML and JSON-RPC rather than shared code and platform lock-in.*
