+++
title = "K-Derm Reconciliation"
description = "From gram-staining to cell envelope architecture — bridging the gen3 gram-negative/gram-positive framing to the canonical K-Derm model."
date = 2026-05-25
weight = 37

[taxonomies]
trails = ["sovereignty"]

[extra]
foundation = true
domain = "Architecture"
maturity = "architectural"

[[extra.companions]]
url = "/architecture/sovereign-transaction-membrane/"
title = "Sovereign Transaction Membrane"
relation = "extended_by"
label = "The membrane model this reconciliation clarifies"

[[extra.companions]]
url = "/architecture/kderm-diderm-architecture/"
title = "K-Derm/Diderm Architecture"
relation = "extends"
label = "The primary architecture document"
+++

## Why This Exists

The [K-Derm Diderm Architecture](@/architecture/KDERM_DIDERM_ARCHITECTURE.md) page
describes the canonical model. This reconciliation document explains how the terminology
evolved from gen3/gen4 to K-Derm, providing a bridge for readers encountering both
vocabularies.

---

## The Problem

gen4 documents used gram-negative/gram-positive terminology with conflicting inner/outer
membrane labels. The same phrase "inner membrane" referred to different layers depending
on the document. This is the Franklin's Current problem — different documents using the
same word to mean different things.

K-Derm resolves this with **absolute positions** and **structural names**:

---

## Terminology Mapping

| gen4 Term | K-Derm Term | Definition |
|-----------|------------|------------|
| Gram-positive | Monoderm | Single-membrane topology (inner membrane only) |
| Gram-negative | Diderm | Double-membrane topology (inner + outer, with periplasm) |
| Inner membrane (gen4 HPC docs) | Plasma membrane | Gate firewall boundary |
| Inner membrane (gen4 deployment docs) | Periplasm + outer membrane | VPS routing layer |
| Intracellular | Cytoplasm | Inside the plasma membrane (HPC mesh, GPU) |
| Extracellular | Extracellular | Public internet, untrusted space |

---

## Absolute Layer Model

K-Derm uses absolute positions, innermost to outermost:

```
Cytoplasm (innermost)
    -> Plasma membrane (gate firewall)
    -> Periplasm (routing, telemetry, selective transport)
    -> Outer membrane (VPS, TLS termination)
    -> Extracellular (public internet)
```

Every component has exactly one position. No ambiguity. No "it depends on the
document" — the layer is the layer.

---

## Bonding Reconciliation

| Bond Type | K-Derm Layer | {{ entity(name="beardog") }} Trust Level |
|-----------|-------------|------|
| Covalent (aquaporin) | Cytoplasm / plasma | Highest — gate-to-gate mesh |
| Ionic (gated ion) | Periplasm / outer | Controlled — collaborator sharing |
| Ceremony (voltage-gated) | Outer membrane crossing | Earned — entropy ceremony required |
| Weak (passive diffusion) | Extracellular boundary | Lowest — public read-only |

Each bond type maps to a {{ entity(name="beardog") }} BTSP cipher suite. Higher entropy
ceremonies produce stronger bonds. The membrane is the enforcement layer.

---

## NUCLEUS Atomics in K-Derm

The three NUCLEUS atomics map to membrane boundaries:

| Atomic | K-Derm Position | Role |
|--------|----------------|------|
| Tower | All boundary crossings | Mediates inter-layer communication |
| Node | Cytoplasm only | Compute inside the plasma membrane |
| Nest | Cytoplasm only | Storage inside the plasma membrane |

Tower is the electron shell analogy: it mediates every transition between layers,
just as electron shells mediate chemical bonding between atoms.

---

## K-Derm Extensions Beyond gen4

The K-Derm model enables concepts that gen4's gram-negative framing could not express:

| Extension | What It Means |
|-----------|--------------|
| **Recursive nesting** | A gate can contain sub-gates with their own membranes |
| **Endosymbiosis** | Collaborator compositions running inside the organism with their own boundary |
| **Gatehouse bond escalation** | Trust level can be upgraded through ceremony without redeployment |
| **Vesicle transport** | Data packages (braids) carry membrane coat proteins across layers |
| **Membrane potential** | Active trust state maintained by ongoing ceremony, not just initial key |

---

## What Does NOT Change

The K-Derm reconciliation changes **vocabulary**, not **architecture**:

- Particle model (primals as biological particles) — unchanged
- Bonding model (covalent/ionic/ceremony/weak) — unchanged
- Three communication channels — unchanged
- Sovereignty standards — unchanged
- BTSP trust protocol — unchanged

The organism is the same organism. K-Derm gives it consistent anatomical terminology.

---

*K-Derm is not a new architecture. It is the same architecture with the ambiguity
removed — absolute positions, structural names, and a reconciliation path from every
gen4 document to the canonical model.*
