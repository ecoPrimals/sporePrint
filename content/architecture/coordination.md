+++
title = "Ecosystem Coordination"
description = "Public coordination standards, glossary, and operational documents — the wateringHole repository as the ecosystem's inter-team communication layer."
date = 2026-07-14
weight = 47

[taxonomies]
trails = ["coordination"]

[extra]
foundation = true
domain = "Architecture"
maturity = "implemented"

[[extra.companions]]
url = "/architecture/coordination-triad/"
title = "Coordination Triad"
relation = "extends"
label = "The three runtime coordination patterns"

[[extra.companions]]
url = "/guidestone/deployable-artifact-standard/"
title = "Deployable Artifact Standard"
relation = "pairs_with"
label = "Artifact standard governed by coordination documents"
+++

## The wateringHole

The [wateringHole](https://github.com/ecoPrimals/wateringHole) repository is the
public coordination layer for the ecoPrimals ecosystem. It contains standards,
glossaries, handoffs, and operational documents that teams reference when building
primals, springs, and products.

wateringHole is public — anyone can read it. It is the single source of truth for
cross-team coordination vocabulary.

---

## Core Standards

| Document | Purpose | Link |
|----------|---------|------|
| **GLOSSARY.md** | Ecosystem terminology — the canonical definitions | [View](https://github.com/ecoPrimals/wateringHole/blob/main/GLOSSARY.md) |
| **STANDARDS_AND_EXPECTATIONS.md** | Quality checklist for all ecosystem contributions | [View](https://github.com/ecoPrimals/wateringHole/blob/main/STANDARDS_AND_EXPECTATIONS.md) |
| **COMPOSITION_ROUTING_STANDARD.md** | How compositions register and route capabilities | [View](https://github.com/ecoPrimals/wateringHole/blob/main/COMPOSITION_ROUTING_STANDARD.md) |
| **DIDERM_DOMAIN_ARCHITECTURE.md** | Domain trust model — K-Derm topology for deployments | [View](https://github.com/ecoPrimals/wateringHole/blob/main/DIDERM_DOMAIN_ARCHITECTURE.md) |
| **ECOSYSTEM_COMMUNICATION_STANDARD.md** | Handoffs, FRAGOs, blurbs — how teams communicate | [View](https://github.com/ecoPrimals/wateringHole/blob/main/ECOSYSTEM_COMMUNICATION_STANDARD.md) |
| **K_DERM_TOPOLOGY_STANDARD.md** | Cell envelope topology — inner/outer membrane naming | [View](https://github.com/ecoPrimals/wateringHole/blob/main/K_DERM_TOPOLOGY_STANDARD.md) |
| **GLACIAL_SHIFT_READINESS.md** | Stadial entry criteria — what must pass before an interstadial opens | [View](https://github.com/ecoPrimals/wateringHole/blob/main/GLACIAL_SHIFT_READINESS.md) |

---

## sporePrint Publishing

| Document | Purpose | Link |
|----------|---------|------|
| **CONTENT_GUIDE.md** | How to publish to sporePrint — editorial workflow and standards | [View](https://github.com/ecoPrimals/wateringHole/blob/main/sporePrint/CONTENT_GUIDE.md) |
| **SPRING_EVOLUTION_TARGETS.md** | Spring-specific evolution targets for content pipeline | [View](https://github.com/ecoPrimals/wateringHole/blob/main/sporePrint/SPRING_EVOLUTION_TARGETS.md) |

---

## How to Use wateringHole

**For ecosystem contributors**: wateringHole is the checklist. Before shipping
a primal method, check STANDARDS_AND_EXPECTATIONS. Before naming a concept,
check GLOSSARY. Before deploying a composition, check COMPOSITION_ROUTING_STANDARD.

**For external evaluators**: wateringHole shows how the ecosystem coordinates.
The standards are public. The glossary is public. The communication patterns
are public. Transparency is structural, not aspirational.

**For collaborators**: the ECOSYSTEM_COMMUNICATION_STANDARD describes how
handoffs, status reports, and after-action reviews work. If you receive a
blurb or a FRAGO, this document explains the format.

---

## Relationship to sporePrint

sporePrint (this site) publishes the public-facing story of the ecosystem.
wateringHole maintains the operational standards that teams follow while
building what sporePrint describes. They complement each other:

| sporePrint | wateringHole |
|-----------|-------------|
| What the ecosystem is | How the ecosystem coordinates |
| Architecture, philosophy, evidence | Standards, checklists, handoffs |
| Public narrative | Public operations |
| Content for evaluators and collaborators | Documents for builders |

sporePrint links to wateringHole; wateringHole links to sporePrint.
Neither is authoritative over the other. Together they provide the full
public picture.

---

## Related Architecture Pages

These sporePrint pages have direct counterparts in wateringHole:

- [K-Derm Diderm Architecture](@/architecture/KDERM_DIDERM_ARCHITECTURE.md) references
  wateringHole's [K_DERM_TOPOLOGY_STANDARD](https://github.com/ecoPrimals/wateringHole/blob/main/K_DERM_TOPOLOGY_STANDARD.md)
- [NUCLEUS Architecture](@/architecture/NUCLEUS_ARCHITECTURE.md) references
  wateringHole's [COMPOSITION_ROUTING_STANDARD](https://github.com/ecoPrimals/wateringHole/blob/main/COMPOSITION_ROUTING_STANDARD.md)
- [Sovereign CI](@/architecture/SOVEREIGN_CI.md) references
  wateringHole's provision infrastructure
- [Stadial/Interstadial Pattern](@/architecture/stadial_interstadial.md) references
  wateringHole's [GLACIAL_SHIFT_READINESS](https://github.com/ecoPrimals/wateringHole/blob/main/GLACIAL_SHIFT_READINESS.md)

---

*wateringHole is named for the place in the savanna where different species
meet — not because they are allied, but because they need the same resource.
Teams, primals, and collaborators meet at wateringHole because they need the
same coordination standards. The water belongs to everyone.*
