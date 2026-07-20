+++
title = "esotericWebb — Cross-Evolution CRPG"
description = "A CRPG that composes primals via JSON-RPC — V22 LIVE at webb.primals.eco. Scene binding fixed, 6/9 primals connected."
date = 2026-03-31

[taxonomies]
primals = ["biomeos", "loamspine", "rhizocrypt", "sweetgrass"]
springs = ["ludospring"]

[extra]
maturity = "live"
+++

**Repository**: [sporeGarden/esotericWebb](https://github.com/sporeGarden/esotericWebb) — **Public**  
**License**: {{ entity(name="scyborg") }} (AGPL-3.0-or-later + ORC + CC-BY-SA 4.0)

---

## What It Is

{{ entity(name="esotericwebb") }} is a cross-evolution CRPG (computer role-playing game) that uses the {{ entity(name="ecoprimals") }} infrastructure as its engine. It composes real primals via JSON-RPC to deliver gameplay mechanics grounded in validated science. The game's composition architecture and science integration are in active development — content and playable experience are being built.

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

{{ maturity(level="live") }} {{ entity(name="esotericwebb") }} is **V22** at [webb.primals.eco](https://webb.primals.eco). {{ entity_stat(name="esotericwebb", stat="tests_display") }} tests, 6/9 primals connected, scene binding fixed (game_scene + fallback). systemd user unit enabled.

| Surface | URL | Status |
|---------|-----|--------|
| flockGate binary | flockGate:8090 (mesh) | **502** — process down, needs restart |
| Public route | [webb.primals.eco](https://webb.primals.eco) | **502** — Caddy TLS fine, backend down |

### Remaining

| Step | Status |
|------|--------|
| ~~systemd enable on flockGate~~ | **Done** (Wave 150f) |
| ~~Caddy vhost `webb.primals.eco`~~ | **Done** (Wave 150e) |
| ~~E2E guided demo scenario~~ | **Shipped** (V18) — `aldric` NPC false-positive pending |
| GET handler for browser navigation | P2 (currently POST/JSON-RPC only) |
| V22 binary to depot | P2 (local build only) |
| Deploy petalTongue v1.7+ | P2 (activates full scene graph pipeline) |
| biomeOS neural-api + executors | P2 (GAP-017, GAP-018) |

---

*See also: [ludoSpring](@/architecture/SPRING_CATALOG.md) for the science validation,
[NUCLEUS Architecture](@/architecture/NUCLEUS_ARCHITECTURE.md) for the composition model,
[Deployment Model](@/architecture/DEPLOYMENT_MODEL.md) for the {{ entity(name="byob") }} workflow.*
