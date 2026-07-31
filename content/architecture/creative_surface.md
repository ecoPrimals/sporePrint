+++
title = "Creative Surface Architecture"
description = "The sporeGarden organizational model — how products relate to infrastructure through the 'powered by' consumption pattern."
date = 2026-04-25
weight = 36

[extra]
foundation = true
domain = "Architecture"
maturity = "architectural"

[[extra.companions]]
url = "/architecture/deploy-graph-composition/"
title = "Deploy Graph Composition"
relation = "pairs_with"
label = "The technical substrate beneath the creative surface"

[[extra.companions]]
url = "/collaborators/abg-initiochem/"
title = "ABG — initioChem"
relation = "validates"
label = "Citizen science through the sporeGarden surface"

[[extra.companions]]
url = "/architecture/atlas-memory-palace/"
title = "Atlas Memory Palace"
relation = "extends"
label = "The garden region in the Atlas landscape"
+++

## Four Organizations

The ecosystem is organized into four organizations, each answering a different question:

| Organization | Question | Audience |
|-------------|----------|----------|
| **ecoPrimals** | Does the infrastructure work? | Developers |
| **syntheticChemistry** | Does the science reproduce? | Scientists |
| **sporeGarden** | Does someone use it? | Creators, scientists, collaborators |
| **protoKarya** | Can it serve the wider world? | End users, institutions |

This separation is not organizational convenience — it reflects the biological distinction
between mycelium (infrastructure), fruiting conditions (springs), the cultivation
surface (products), and the wider-world organisms (protists) that consume them.

---

## The "Powered By" Model

sporeGarden products consume primals but do not import them:

- **JSON-RPC TCP** — all primal communication via standard IPC
- **{{ entity(name="songbird") }} discovery** — products discover primals at runtime via mesh
- **Graceful degradation** — products work with reduced capability when primals are unavailable
- **{{ entity(name="sweetgrass") }} attribution** — products carry provenance for every primal they consumed
- **{{ entity(name="rhizocrypt") }} tracing** — every computation has a hash chain

No shared crates. No source-level coupling. No platform rent. Different organization,
binary-only interface, independent release cycles.

---

## The Spore Metaphor

```
ecoPrimals = mycelium (underground network, substrate decomposition, nutrient transport)
springs    = fruiting conditions (temperature, humidity, substrate chemistry)
sporeGarden = cultivation surface (where fruiting bodies emerge for the world to see)
```

Users interact with the cultivation surface. They see {{ entity(name="helixvision") }},
{{ entity(name="initiochem") }}, esotericWebb — products with user interfaces, workflows,
and deliverables. They do not see the mycelium.

---

## Four Product Layers

Every sporeGarden product has four layers:

1. **Primal binaries** — sovereign compute primitives (ecoPrimals provides)
2. **PrimalBridge** — IPC adapter connecting product to primals (deploy graph)
3. **Product engine** — domain logic (the product itself)
4. **Creative content** — user-facing configuration, data, or media (YAML/TOML)

The layers compose vertically. A user's YAML configuration feeds the product engine,
which dispatches through the PrimalBridge to primal binaries. The user never touches
layers 1-2. The product developer works in layers 2-3. The primal developer works in
layer 1.

---

## Trust Model

| Boundary | Trust Mechanism |
|----------|----------------|
| Creator to engine | Deterministic validation — the engine rejects invalid content |
| Product to primals | {{ entity(name="ecobin") }} compliance — binaries satisfy structural requirements |
| Collaborator to product | Provenance DAG — every result traces to its computation |

---

## Lean Consumption

gen3 springs consumed primals via crate imports — tight coupling, shared dependency
trees, synchronized versions. gen4 products consume primals via TCP capabilities —
loose coupling, independent evolution, graceful degradation.

This is the difference between a cell importing a gene (gen3) and a cell secreting
a signal molecule (gen4). The signal (JSON-RPC capability) crosses the membrane.
The gene (source code) stays inside.

---

## Projected Catalog

### sporeGarden Products

| Product | Domain | Status |
|---------|--------|--------|
| esotericWebb | Creative gaming with primal composition | Active |
| {{ entity(name="helixvision") }} | Self-hosted protein structure prediction | Implemented |
| {{ entity(name="initiochem") }} | Conformational dynamics and FEL | Implemented |
| {{ entity(name="bluefish") }} | PFAS analytical chemistry ETL | Specification |
| {{ entity(name="lithospore") }} | Bootable sovereign USB environment | Designed |

### protoKarya Protists

| Protist | Domain | Status |
|---------|--------|--------|
| [footPrint](@/products/footprint.md) | GIS home planning | **Partially live** |
| [tideGlass](@/products/tideglass.md) | Sovereign GPS platform | Phase 0 |

---

*The creative surface is where the ecosystem meets the world. Users see products,
not primals. Scientists see results, not infrastructure. The mycelium does the work.
The fruiting body gets the credit. That is the design.*
