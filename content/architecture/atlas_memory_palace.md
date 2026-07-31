+++
title = "Atlas — A Memory Palace for Humans and AI"
description = "The ecoPrimals ecosystem as a navigable landscape: mountain (primals), river (springs), garden (products), water (sync) — a Lewis-and-Clark map of sovereign computation."
date = 2026-06-15
weight = 46

[taxonomies]
trails = ["first-visit"]

[extra]
foundation = true
domain = "Architecture"
maturity = "architectural"

[[extra.companions]]
url = "/architecture/creative-surface/"
title = "Creative Surface"
relation = "extends"
label = "The garden region made accessible"

[[extra.companions]]
url = "/architecture/coordination-triad/"
title = "Coordination Triad"
relation = "extends"
label = "The water region — sync and propagation"

[[extra.companions]]
url = "/architecture/primal-evolution/"
title = "Primal Evolution"
relation = "extends"
label = "The mountain region — primals at rest"
+++

## The Landscape

The ecoPrimals ecosystem is a real landscape with four regions. These pages
are a map — making that landscape legible to collaborators, evaluators,
and future agents.

```
Mountain (primals)          -- mass at rest, sovereignty
    |
    v
River (springs)             -- energy, validation
    |
    v
Garden (products)           -- information, delivery
    |
    v
Water (sync)                -- transport, propagation
    |
    +-> returns to mountain (feedback loop)
```

---

## The Mountain — Thirteen Primals

The mountain is **mass**: 13 autonomous Rust binaries composing into
Tower (trust), Node (compute), Nest (storage), and NUCLEUS (full atom).

The mountain was not designed top-down. It was discovered through constrained
evolution — the Pure Rust + JSON-RPC constraint naturally partitioned
functionality into capability domains, the way tectonic pressure folds
rock into strata.

| Tier | Primals | What They Provide |
|------|---------|------------------|
| Summit (Tower) | {{ entity(name="beardog") }}, {{ entity(name="songbird") }}, {{ entity(name="skunkbat") }} | Trust, discovery, defense |
| Terraces (Node) | {{ entity(name="toadstool") }}, {{ entity(name="barracuda") }}, {{ entity(name="coralreef") }} | Hardware, compute, compilation |
| Terraces (Nest) | {{ entity(name="nestgate") }}, {{ entity(name="rhizocrypt") }}, {{ entity(name="loamspine") }}, {{ entity(name="sweetgrass") }} | Storage, provenance, attribution |
| Base (NUCLEUS) | {{ entity(name="biomeos") }}, {{ entity(name="squirrel") }}, {{ entity(name="petaltongue") }} | Orchestration, AI, representation |

See: [Primal Catalog](@/architecture/PRIMAL_CATALOG.md),
[Primal Evolution](@/architecture/primal_evolution.md)

---

## The River — Eight Springs

The river is **energy**: scientific questions that convert primal mass into
validated computation against published baselines.

| Spring | Domain | Faculty Anchor |
|--------|--------|---------------|
| {{ entity(name="hotspring") }} | Plasma physics, MD, lattice QCD | Murillo, Bazavov |
| {{ entity(name="wetspring") }} | Genomics, PFAS, microbial signaling | Waters, Liu, Jones |
| {{ entity(name="neuralspring") }} | ML primitives, evolutionary computation | Dolson, Kachkovskiy |
| {{ entity(name="airspring") }} | Precision agriculture, environmental science | Dong |
| {{ entity(name="groundspring") }} | Statistics, error propagation, inverse problems | Cross-spring |
| {{ entity(name="healthspring") }} | Pharmacology, PK/PD, drug repurposing | Gonzales |
| ludoSpring | Game mechanics, creative validation | esotericWebb |
| {{ entity(name="primalspring") }} | Ecosystem integration scenarios | Cross-team |

Each spring has named faculty anchors — published scientists whose work was
reproduced to validate the springs. The faculty are constraints, not
credentials: their published results define what "correct" means.

See: [Spring Catalog](@/architecture/SPRING_CATALOG.md)

---

## The Garden — Products and Spores

The garden is **information**: what the world sees when the mountain's mass
is converted to energy by the river and crystallized into deliverables.

### Four Organizations

| Organization | Question | Audience |
|-------------|----------|----------|
| ecoPrimals | Does the infrastructure work? | Developers |
| syntheticChemistry | Does the science reproduce? | Scientists |
| sporeGarden | Does someone use it? | Creators, collaborators |
| protoKarya | Can it serve the wider world? | End users, institutions |

### Spore Taxonomy

| Spore Type | Size | What It Carries |
|-----------|------|----------------|
| **coldSpore** | ~KB | Metadata marker (JSON health report) |
| **liveSpore** | ~KB-MB | Active validation artifact with provenance |
| **pseudoSpore** | ~MB-GB | Self-verifying computational result (grant preliminary data) |
| **{{ entity(name="lithospore") }}** | ~GB-16 GB | Bootable sovereign USB environment |

### gen5: Someone Else's Garden

gen3 built instruments. gen4 made instruments invisible behind products.
gen5 makes products invisible behind **someone else's science**.

The [NF case study](@/products/nf_case_study.md) is the exemplar: the
collaborator does not see primals, does not see products, does not see
infrastructure. She sees her NF gene expression results, her drug
repurposing scores, her preliminary data for the CTF NDU application.
The ecosystem succeeds when her science succeeds.

See: [Products](@/products/_index.md), [Collaborators](@/collaborators/_index.md)

---

## The Water — Sync and Propagation

The water is **transport**: how changes flow across gates, remotes, and
airgaps without a central coordinator.

### The waterFall Pattern

Gravity, not pumping. Changes flow downhill from where they were created:

1. Fetch all remotes
2. Measure temporal position (ahead/behind/diverged/parity)
3. Pull from the leader
4. Push to the followers
5. The DAG is the only clock

### The Coordination Triad

| Pattern | Domain | What It Does |
|---------|--------|-------------|
| quorumSignal | SENSE | Observes, discovers, classifies |
| [rootPulse](@/architecture/rootpulse.md) | ACTION | Creates, commits, attributes |
| [waterFall](@/architecture/waterfall.md) | SYNC | Reconciles, propagates, maintains |

See: [Coordination Triad](@/architecture/coordination_triad.md)

---

## Mass, Energy, Information

The landscape follows physical equivalence:

| Concept | Region | Physical Analog |
|---------|--------|----------------|
| Primals | Mountain | Mass — compute at rest |
| Springs | River | Energy — computation doing work |
| Products/Spores | Garden | Information — crystallized results |
| waterFall | Water cycle | Transport — propagation across space |

Spores have mass (they occupy storage). Springs provide energy (they do
computational work). Products carry information (they encode verified results).
waterFall transports all three across the mesh.

---

## The Cycle

```
Mountain (mass) -> River (energy) -> Garden (information) -> Water (transport)
                                                                    |
                                                                    +-> Mountain
```

The cycle closes: collaborator feedback (new validation targets, new
biological questions, new data systems) returns to the mountain as new
primal capabilities and spring validation checks. The ecosystem grows
because external demand creates internal evolution.

---

*The atlas is not a metaphor. It is a navigation system — a memory palace
where mountain, river, garden, and water each have concrete meaning and
concrete content. Use it to find your way through the ecosystem. Start at
the mountain if you want to understand infrastructure. Start at the garden
if you want to see what ships. Start at the river if you want to verify
the science. Start at the water if you want to know how it all stays
in sync.*
