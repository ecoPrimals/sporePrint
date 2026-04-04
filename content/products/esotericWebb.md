+++
title = "esotericWebb — Cross-Evolution CRPG"
description = "A CRPG that composes primals via JSON-RPC — the first proof that sovereign infrastructure produces real games."
date = 2026-03-31

[taxonomies]
primals = ["biomeos", "loamspine", "rhizocrypt", "sweetgrass"]
springs = ["ludospring"]
+++

**Repository**: [sporeGarden/esotericWebb](https://github.com/sporeGarden/esotericWebb) — **Public**  
**License**: {{ entity(name="scyborg") }} (AGPL-3.0-or-later + ORC + CC-BY-SA 4.0)

---

## What It Is

{{ entity(name="esotericwebb") }} is a cross-evolution CRPG (computer role-playing game) that uses the {{ entity(name="ecoprimals") }} infrastructure as its engine. It is not a tech demo — it is a playable game that composes real primals via JSON-RPC to deliver gameplay mechanics grounded in validated science.

The game exists to prove a thesis: that sovereign, composable infrastructure can produce creative software as good as anything built on proprietary engines, while giving the player full data sovereignty and the developer zero vendor lock-in.

---

## How It Composes

{{ entity(name="esotericwebb") }} consumes three post-{{ entity(name="nucleus") }} primals and orchestration from {{ entity(name="biomeos") }}:

| Primal | What It Provides | Game Mechanic |
|--------|-----------------|---------------|
| {{ entity(name="rhizocrypt") }} | Ephemeral DAG workspace, Merkle verification | Save states as cryptographic DAGs — every choice is verifiable |
| {{ entity(name="loamspine") }} | Immutable linear history, certificates | Game timeline as an append-only log — no retroactive edits |
| {{ entity(name="sweetgrass") }} | Attribution and provenance tracking | Every asset, quest, and NPC decision traces back to its source |
| {{ entity(name="biomeos") }} | {{ entity(name="neuralapi") }} orchestration | Routes game events to the right primal by capability |

The composition is {{ entity(name="byob") }}: {{ entity(name="esotericwebb") }} fetches primal binaries from [plasmidBin](https://github.com/ecoPrimals/plasmidBin), runs them locally, and communicates via JSON-RPC. No cloud. No accounts. No telemetry.

---

## The Science Connection

{{ entity(name="ludospring") }} validates 13 foundational HCI models (Fitts, Hick, Flow, DDA, Perlin, WFC, L-systems). {{ entity(name="esotericwebb") }} is where those validated models meet a player. Every game mechanic traces to a published paper through {{ entity(name="ludospring") }}'s validation chain.

The cross-spring experiments ({{ entity(name="ludospring") }} Track 11) proved that game metrics generalize to scientific exploration sessions. {{ entity(name="esotericwebb") }} is the interactive surface where that finding becomes a product.

---

## Current Status

{{ entity(name="esotericwebb") }} is the first {{ entity(name="sporegarden") }} product. It demonstrates end-to-end primal composition in a domain (gaming) where user experience is the primary metric.

---

*See also: [ludoSpring](@/architecture/SPRING_CATALOG.md) for the science validation,
[NUCLEUS Architecture](@/architecture/NUCLEUS_ARCHITECTURE.md) for the composition model,
[Deployment Model](@/architecture/DEPLOYMENT_MODEL.md) for the {{ entity(name="byob") }} workflow.*
