# Knowledge Topology — Concept Lineage and Presentation Philosophy

The intellectual foundations that guide HOW sporePrint connects, presents, and relates information. These are not decorative references — they are structural constraints on how the site's information architecture works.

Last updated: June 1, 2026 (Wave 66 — Renvois de Choses implementation)

---

## The Core Problem

How do you represent connections between IDEAS, not just links between PAGES?

The web answered this with one-way untyped hyperlinks. We reject that answer. sporePrint implements a typed bidirectional knowledge graph where connections exist because of the nature of the things themselves — not because they share keywords.

---

## Intellectual Lineage (Spiritual Mentors)

### 1. Diderot (1751) — Renvois de Choses

**Source:** *Encyclopédie, ou dictionnaire raisonné des sciences, des arts et des métiers*

**Key concept:** Two kinds of cross-reference:
- **Renvois de mots** (word-references) — "I don't know this term, point me to its definition"
- **Renvois de choses** (thing-references) — "These ideas are structurally connected across disciplines"

**Diderot's four types of thing-connections:**
1. Analogy — structural similarity across domains
2. Common principle — shared foundational derivation
3. Contrast — illumination by opposition
4. Refutation — one disproves the other

**Quote:** "La partie de l'ordre encyclopédique la plus importante" — the most important part of the encyclopedic order.

**Quote:** "Le canevas serré qui fait de l'ensemble du savoir humain un continuum" — the tight weave making all human knowledge a continuum.

**How this constrains sporePrint:** Every entity page shows typed bidirectional connections. The `EdgeRelation` enum implements Diderot's taxonomy. Connections are NON-LINGUISTIC — they exist because of what things ARE, not what they're called.

### 2. Vannevar Bush (1945) — The Memex

**Source:** "As We May Think," *The Atlantic Monthly*

**Key concept:** A desk with two projection screens. The user views two documents, taps a key, and they are *permanently joined*. The connection has provenance (who, when, why). Named trails become first-class objects.

**Bush's insight:** "The process of tying two items together is the important thing."

**Properties of Memex trails:**
- Permanent — once made, the connection persists
- Attributed — who made it and when
- Named — trails are objects, not just paths
- Shareable — hand your trail to a colleague
- Forkable — branch from someone else's path

**How this constrains sporePrint:** The entity graph's edges are permanent (committed to loamSpine in Phase 2). sweetGrass provides attribution. Future Phase 3 adds named trails as first-class graph vertices.

### 3. Ted Nelson (1960s) — Xanadu

**Source:** Project Xanadu design documents

**Key concept:** Bidirectional links, transclusion, version history.

**Nelson's principles:**
- Every document knows what links TO it (bidirectionality)
- Include-by-reference, not copy-by-value (transclusion)
- Every state is permanent and addressable (versioning)

**How this constrains sporePrint:** The graph module computes inverse edges automatically. Every entity page shows INBOUND connections — what reaches toward it. Authors declare one direction; the system provides both.

### 4. The Web (1989) — The Regression

Tim Berners-Lee implemented only renvois de mots: one-way, untyped, no provenance, no permanence. Wikipedia improved breadth but not connection type — "thermodynamics" → "Rudolf Clausius" uses the same blue underline as "thermodynamics" → "information theory."

**How this constrains sporePrint:** We explicitly refuse to regress to untyped links. Every connection between entities has a typed `EdgeRelation`. The template renders the relation type as visible text.

---

## Mapping: Concepts → Implementation

| Intellectual Concept | Implementation | File(s) |
|-----|-----|-----|
| Renvois de choses (thing-connections) | `EdgeRelation` enum | `crates/spore-validate/src/model.rs` |
| Diderot's four categories | Analogy→`analogous_to`, Principle→`composes_into`/`derived_from`, Contrast→`contradicts`, Refutation→`contradicts` | `model.rs` |
| Non-linguistic connections | Typed edges between entities — relation is structural, not textual | `config.toml` edges arrays |
| "Canevas serré" (tight weave) | Entity graph with 126 bidirectional edges across 66 nodes | `static/graph/entity-graph.json` |
| Memex trails (provenance) | sweetGrass attribution braids (Phase 2/3) | Future: `graph/spine.json` |
| Memex permanent joining | loamSpine append-only ledger (Phase 2) | Future: `graph/spine.json` |
| Xanadu bidirectionality | `graph.rs` inverse edge computation | `crates/spore-validate/src/graph.rs` |
| Xanadu "document knows what links to it" | Inbound edges section on taxonomy pages | `templates/taxonomy_single.html` |
| Bush's named trails | Phase 3: trail vertices in entity graph | Future |

---

## Mapping: Primals → Intellectual Lineage

| Primal | Lineage Role |
|-----|-----|
| rhizoCrypt | Diderot's renvois de choses as typed DAG — content-addressed connections between things |
| loamSpine | Diderot's "canevas serré" made immutable — the permanent weave with inclusion proofs |
| sweetGrass | Bush's trail provenance — who made each connection, when, why, with PROV-O braids |
| The trio together | Completes the 275-year arc: Diderot (1751) → Bush (1945) → Nelson (1960s) → Trio (2026) |

---

## Edge Type Taxonomy (Diderot-Extended)

These are the structural relationship types. Each is a different KIND of intellectual connection:

### From Diderot's Four Categories:
| Edge | Diderot Category | Meaning |
|-----|-----|-----|
| `analogous_to` | Analogy | Structural similarity across domains |
| `composes_into` | Common principle | Both derive from same compositional foundation |
| `derived_from` | Common principle | Explicit intellectual lineage |
| `extends` | Common principle | Builds upon same principle |
| `contradicts` | Refutation/Contrast | Disproves or supersedes |

### Operational Extensions (Scientific Computing):
| Edge | Meaning |
|-----|-----|
| `validated_by` / `validates` | Scientific validation chain |
| `compiled_by` | Build-time transformation |
| `dispatches` | Runtime orchestration |
| `stores_for` | Persistence relationship |
| `discovers` | Network/service discovery |
| `preceded_by` | Temporal/evolutionary order |
| `reproduces` | Scientific reproduction |
| `references` | General citation |

---

## Design Principles (Derived from Lineage)

1. **Non-linguistic first** — If the connection requires shared vocabulary to explain, it's probably a renvoi de mots (word-reference). Prefer structural connections that exist because of what things ARE.

2. **Bidirectional by construction** — Authors declare one direction. The system provides both. Nothing is one-way.

3. **Typed, not tagged** — Every connection has a specific `EdgeRelation`. "Related" is not a valid relationship type. You must say HOW it's related.

4. **Provenance-tracked** — WHO connected these things, WHEN, WHY. Phase 1 is implicit (git blame). Phase 2+ is explicit (sweetGrass braids).

5. **Permanent** — Once committed, a connection does not disappear. It can be deprecated (preceded_by) but not deleted.

6. **Emergent topology** — The graph reveals connections that no human declared: entities with identical capability signatures, tier-matched primals with no shared edges, structural isomorphisms across domains. This is the prize — what Diderot called the renvois de choses.

---

## Presentation Constraints

When rendering entity pages:

- Show OUTBOUND edges (what this thing reaches toward) grouped by relation type
- Show INBOUND edges (what reaches toward this thing) separately
- Display relation type as visible text — the reader sees "validated by" not just a link
- Link to the connected entity's taxonomy page
- Do NOT collapse connections into a generic "Related" section

When writing content:

- If two things are structurally connected, add an edge in `config.toml` — don't just mention it in prose
- Prose `composes` strings remain for human narrative but the typed `edges` array is the source of truth
- When describing a primal's role, frame it in terms of its graph neighborhood (what it validates, what validates it, what it composes into)

---

## Phase Roadmap

| Phase | What | Status |
|-----|-----|-----|
| 1 | Typed edges in config.toml, validated at build time, rendered on entity pages | **Done** (Wave 66) |
| 2 | rhizoCrypt DAG as source of truth, loamSpine permanence, Merkle roots | Planned |
| 3 | Named trails, live exploration, sweetGrass attribution, sovereign serving | Future |

---

## Historical Alignment Record

This section documents when each concept was mapped and by what reasoning:

- **2026-06-01 (Wave 66):** Initial implementation of Phase 1. Diderot-Bush-Nelson lineage established. 14 edge types defined. 63 edges declared across 15 primals. Entity graph module (`graph.rs`) computes bidirectional edges and emits JSON. Connections template renders on all taxonomy pages. Architecture page at `/architecture/renvois-knowledge-topology/` documents the lineage publicly.
