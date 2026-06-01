+++
title = "initioChem — Interactive Computational Chemistry"
description = "An interactive free energy landscape explorer — the first gen4 product where computational chemistry infrastructure becomes invisible to the researcher."
date = 2026-05-31

[taxonomies]
primals = ["biomeos", "nestgate", "barracuda"]
springs = ["hotspring"]
+++

**Repository**: sporeGarden/initioChem — **In Development**
**License**: scyBorg triple (AGPL-3.0-or-later + ORC + CC-BY-SA 4.0)

---

## What It Is

initioChem is an interactive computational chemistry explorer that lets
researchers visualize and analyze free energy landscapes (FELs) without
knowing anything about the infrastructure that produces them.

The researcher sees conformational landscapes, puckering coordinates, and
energy surfaces. They never see NestGate, barraCuda, or capability graphs.
The science is visible; the infrastructure is invisible.

---

## How It Composes

| Primal/Spring | What It Provides | User Experience |
|---------------|-----------------|-----------------|
| hotSpring | Validated metadynamics computation | "Run my simulation" |
| barraCuda | GPU acceleration (WGSL compute shaders) | "It's fast" |
| NestGate | Content-addressed data storage | "My results are saved" |
| petalTongue | Rendering (templates, visualization) | "Show me the landscape" |
| sweetGrass | Provenance tracking | "Where did this result come from?" |

---

## The pseudoSpore Connection

initioChem is the interactive surface for hotSpring's pseudoSpores.
The CompChem GuideStone v1.6.1 (7/8 modules passing) provides the
validated computation substrate. initioChem provides the interactive
exploration layer:

```
hotSpring produces pseudoSpore (validated computation)
  → initioChem renders pseudoSpore data interactively
  → researcher explores conformational landscapes
  → provenance tracked by sweetGrass
  → results exportable as new pseudoSpore (researcher's science)
```

---

## Current Status

- **Seeded** (v0.1.0) — architecture defined, first ABG pseudoSpore delivered
- **Composition**: hotSpring + barraCuda + NestGate + petalTongue + sweetGrass
- **Validation substrate**: hotSpring CompChem GuideStone (190/190 checks)
- **gen5 connection**: ABG whole-cell modeling thread (Karr 2012 → Thornburg 2026)

---

## Design Principles

1. **Zero configuration**: Researcher opens initioChem, loads a pseudoSpore, explores
2. **No primal knowledge required**: The interface is chemistry, not infrastructure
3. **Self-verifying data**: Every landscape displayed can be independently re-derived
4. **GPU-accelerated by default**: barraCuda handles visualization compute
5. **Sovereign**: Runs entirely on researcher's hardware, no cloud dependency
