+++
title = "rootPulse — Emergent Version Control"
description = "Git reimagined as coordination between sovereign primitives — version control that emerges from primal composition rather than a monolithic tool."
date = 2026-05-05
weight = 40

[taxonomies]
trails = ["coordination"]

[extra]
domain = "Architecture"
maturity = "architectural"

[[extra.companions]]
url = "/architecture/coordination-triad/"
title = "Coordination Triad"
relation = "extended_by"
label = "rootPulse as one of three coordination patterns"

[[extra.companions]]
url = "/architecture/waterfall/"
title = "waterFall"
relation = "pairs_with"
label = "Temporal sync that propagates rootPulse state"
+++

## What It Is

rootPulse is not a primal. It is the **ACTION domain** of the
[coordination triad](@/architecture/coordination_triad.md) — version control
that emerges when {{ entity(name="biomeos") }} orchestrates the provenance trio
({{ entity(name="rhizocrypt") }} + {{ entity(name="loamspine") }} + {{ entity(name="sweetgrass") }})
plus {{ entity(name="nestgate") }}, {{ entity(name="beardog") }}, and {{ entity(name="songbird") }}
over TOML composition graphs.

**Key insight**: rootPulse does not reimagine Git by building a new monolith.
It reimagines Git by showing that version control is a coordination pattern —
one that emerges naturally when you have content-addressed storage, cryptographic
signing, immutable history, and semantic attribution.

---

## The Two-Tier Architecture

rootPulse separates two temporal domains that Git conflates:

### rhizoCrypt — The Ever-Branching Present

The working tier. Lock-free, ephemeral, a DAG that branches freely:

- Stage changes without blocking
- Branch without coordination
- Multiple writers, no locks
- 10-100x staging performance vs Git

This is the **future** — everything that might become a commit.

### loamSpine — The Immutable Past

The committed tier. Linear, append-only, cryptographically sealed:

- Once committed, never changed
- Signature chain from {{ entity(name="beardog") }}
- Content-addressed via {{ entity(name="nestgate") }}
- The **past** — everything that has been proven

### Dehydration Protocol

The transition from present to past:

```
rhizoCrypt DAG (branching, ephemeral, fast)
    -> dehydration (collapse DAG to linear)
    -> loamSpine commit (immutable, signed, attributed)
```

Git conflates staging, branching, and committing in a single data structure.
rootPulse separates them: staging and branching live in the DAG (fast, lock-free),
committing lives in the linear chain (slow, deliberate, permanent).

---

## Primal Composition

rootPulse coordinates six primals. None of them know about rootPulse:

| Primal | Role | Interface |
|--------|------|-----------|
| {{ entity(name="rhizocrypt") }} | DAG storage, staging, branching | `dag.stage`, `dag.dehydrate` |
| {{ entity(name="loamspine") }} | Linear commit chain | `commit.append`, `commit.verify` |
| {{ entity(name="sweetgrass") }} | Semantic attribution | `attribution.record`, `attribution.query` |
| {{ entity(name="nestgate") }} | Content-addressed blob storage | `cas.store`, `cas.retrieve` |
| {{ entity(name="beardog") }} | Cryptographic signing | `sign.commit`, `sign.verify` |
| {{ entity(name="songbird") }} | Cross-gate federation | `relay.push`, `relay.pull` |

The primals are instruments. {{ entity(name="biomeos") }} is the conductor. rootPulse
is the music that emerges when the conductor reads the score (TOML composition graph).

---

## The 6-Phase Commit

A rootPulse commit is a sequential composition of primal calls:

1. **Health check** — verify all required primals are available
2. **Session** — {{ entity(name="rhizocrypt") }} dehydrates the DAG into a commit candidate
3. **Sign** — {{ entity(name="beardog") }} signs the commit via Unix domain socket
4. **Store** — {{ entity(name="nestgate") }} stores the content-addressed blobs
5. **Commit** — {{ entity(name="loamspine") }} appends the signed, stored commit to the linear chain
6. **Attribute** — {{ entity(name="sweetgrass") }} records semantic contribution data

Each phase is a JSON-RPC call. Each phase can fail independently. The composition
graph defines the dependency order. {{ entity(name="biomeos") }} handles retry and rollback.

---

## Semantic Attribution

Git blame counts lines. {{ entity(name="sweetgrass") }} tracks meaning.

| Git blame | {{ entity(name="sweetgrass") }} attribution |
|-----------|-----|
| Who changed this line? | Who designed this module? |
| When was it changed? | What was the intent? |
| How many lines? | What kind of contribution? (design, implementation, fix, review) |

Three attribution layers:
1. **Structural** — what files, what functions, what lines
2. **Semantic** — what capability, what design decision, what trade-off
3. **Narrative** — why this approach, what alternatives were considered

---

## Beyond Version Control

The rootPulse pattern applies wherever provenance matters:

| Domain | What Gets "Committed" |
|--------|----------------------|
| Code | Source files with semantic attribution |
| Science | Experimental results with {{ entity(name="guidestone") }} verification |
| Games | Session state with provenance DAG |
| Field data | Sensor readings with calibration chain |
| Medical records | Patient data with biometric-gated access |

The pattern is the same: create content ({{ entity(name="rhizocrypt") }} DAG), prove it
({{ entity(name="beardog") }} signing), store it ({{ entity(name="nestgate") }} CAS), commit it
({{ entity(name="loamspine") }} chain), attribute it ({{ entity(name="sweetgrass") }} semantics).

---

## Implementation Status

| Component | Status |
|-----------|--------|
| Provenance trio ({{ entity(name="rhizocrypt") }} + {{ entity(name="loamspine") }} + {{ entity(name="sweetgrass") }}) | **Production** (2,308+ tests) |
| 5 composition graphs | Defined (commit, branch, merge, diff, federate) |
| 6-phase commit workflow | Specified with JSON-RPC traces |
| CLI frontend | Not yet built |
| Federation via {{ entity(name="songbird") }} | Designed |

---

*rootPulse is Git reimagined as a coordination pattern between sovereign, composable
primitives. Primals do not know about rootPulse. They provide capabilities.
{{ entity(name="biomeos") }} composes those capabilities into version control. The
music emerges from the instruments — not from a new instrument.*
