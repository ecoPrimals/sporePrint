+++
title = "Renvois de Choses: Knowledge Topology via the Provenance Trio"
description = "How typed bidirectional connections between ideas — not links between pages — complete a 275-year arc from Diderot through Bush and Nelson to the provenance trio."
weight = 14
[taxonomies]
primals = ["rhizocrypt", "loamspine", "sweetgrass"]

[extra]
domain = "Architecture"
maturity = "architectural"
+++

## The Problem: Links Between Words, Not Things

The web implemented *renvois de mots* — word-references. Click a blue hyperlink and you arrive at a page. The page doesn't know you came. The link has no type, no provenance, no inverse. It connects strings of text, not ideas.

This is not what was intended.

## The Intellectual Lineage

### Diderot (1751) — The Encyclopédie

Denis Diderot distinguished two kinds of cross-references in his *Encyclopédie*:

- **Renvois de mots** — word-references: "see also: fermentation." Navigational. Linguistic. The reader doesn't know a term.
- **Renvois de choses** — thing-references: connections that reveal structural relationships across disciplines. Analogy, common principle, contrast, refutation. The connection exists because of the *nature of the things themselves*, not because of shared vocabulary.

Diderot called the renvois de choses "la partie de l'ordre encyclopédique la plus importante" — the most important part of the encyclopedic order — and described them as "le canevas serré qui fait de l'ensemble du savoir humain un continuum" — the tight weave making all human knowledge a continuum.

He identified four types of thing-connections:
1. **Analogy** — these things share structural form
2. **Common principle** — these things derive from the same foundation
3. **Contrast** — these things illuminate each other by opposition
4. **Refutation** — this thing disproves that thing

### Bush (1945) — The Memex

Vannevar Bush's "As We May Think" described a desk with two projection screens. A researcher views two documents simultaneously, taps a key, and they are **permanently joined**. Key properties:

- The connection has **provenance** — who made it, when, why
- Named **trails** become first-class objects — shareable, forkable, attributable
- "The process of tying two items together is the important thing"
- The owner builds a personal graph of knowledge through daily use

Bush drew the man at the desk, surrounded by documents, creating associative connections that no hierarchical index could express.

### Nelson (1960s) — Xanadu

Ted Nelson's Project Xanadu introduced:

- **Bidirectional links** — every document knows what points TO it
- **Transclusion** — include-by-reference, not copy-by-value
- **Version history** — every state is permanent and addressable

Xanadu never shipped. The web implemented its degenerate case.

### The Web (1989) — Regression

Tim Berners-Lee's hypertext implemented only renvois de mots:
- One-way links (the target doesn't know about the source)
- Untyped (a link is a link is a link)
- No provenance (who made this connection? when? why?)
- No permanence (broken links are the default state)
- No trails (navigation history is private and ephemeral)

Wikipedia improved on breadth but not on connection type. Its links remain renvois de mots — word-references from one article to another. The connection between "thermodynamics" and "information theory" is the same blue underline as the connection between "thermodynamics" and "Rudolf Clausius."

## The Provenance Trio Completes the Lineage (2026)

Three primals — {{ entity(name="rhizocrypt") }}, {{ entity(name="loamspine") }}, and {{ entity(name="sweetgrass") }} — together implement what Diderot envisioned and Bush mechanized:

| Intellectual Need | Historical Source | Trio Implementation |
|---|---|---|
| Non-linguistic connections between things | Diderot's renvois de choses | {{ entity(name="rhizocrypt") }}: typed DAG edges between content-addressed vertices |
| Permanent record of connections | Diderot's "canevas serré" | {{ entity(name="loamspine") }}: append-only Spine with inclusion proofs |
| Provenance of who connected what, when, why | Bush's named trails | {{ entity(name="sweetgrass") }}: W3C PROV-O attribution braids |
| Bidirectional discovery | Nelson's Xanadu links | Graph module: inverse edges computed at build time |
| Typed relationships | Diderot's four categories | `EdgeRelation` enum: 14 relationship types |
| Associative trails | Bush's Memex trails | Future: named paths as first-class graph vertices |

## Edge Types as a Taxonomy of Intellectual Relationships

The edge types in our entity graph map directly to Diderot's categories:

### Analogy (structural similarity)
- `analogous_to` — these entities share structural form without sharing code

### Common Principle (shared foundation)
- `composes_into` — both derive from the same compositional foundation
- `derived_from` — explicit intellectual lineage
- `extends` — builds upon the same principle

### Contrast (illumination by opposition)
- `contradicts` — this entity disproves or supersedes that one

### Beyond Diderot (operational relationships)
- `validated_by` / `validates` — scientific validation chains
- `compiled_by` — build-time transformation
- `dispatches` — runtime orchestration
- `stores_for` — persistence relationship
- `discovers` — network/service discovery
- `preceded_by` — temporal/evolutionary order
- `reproduces` — scientific reproduction
- `references` — general citation

## What This Makes Possible

### Non-linguistic emergence

Once the graph exists, queries like "entities with identical capability categories" or "foundation-tier primals that share no direct edges" become computable **without language**. The connections emerge from the topology of things — precisely Diderot's renvois de choses.

### Bidirectional discovery

Every entity page shows both outbound connections (what it reaches toward) and inbound connections (what reaches toward it). Follow any idea in any direction. Nelson's dream, achieved without Xanadu.

### Cryptographic permanence

Once a connection enters {{ entity(name="loamspine") }}, it is permanent, provable, and inclusion-proofed. The "canevas serré" becomes immutable infrastructure.

### Attribution braids

{{ entity(name="sweetgrass") }} tracks WHO made each connection, WHEN, WHY. Bush's dream of named, shareable, forkable trails with full provenance — but with cryptographic guarantees.

## Architecture

The graph is built in three phases:

**Phase 1 (current):** Typed edges in `config.toml`, validated by `spore-validate`, rendered as bidirectional connections on every entity page. No running composition required.

**Phase 2 (next):** {{ entity(name="rhizocrypt") }} DAG as source of truth. Edges become content-addressed vertices. Merkle roots committed to {{ entity(name="loamspine") }}. Dehydrated to static JSON at build time.

**Phase 3 (future):** Named trails as first-class objects. Authors create associative paths through the graph. Readers fork trails. {{ entity(name="sweetgrass") }} braids attribution. Live graph queries served from {{ entity(name="cellmembrane") }} VPS.

The 275-year arc from Diderot's desk to Bush's Memex to Nelson's Xanadu to today's provenance trio is not metaphor. It is implementation lineage. Each step identified the same need — non-linguistic connections between ideas with provenance — and each successive attempt came closer to a complete solution. The provenance trio completes it.
