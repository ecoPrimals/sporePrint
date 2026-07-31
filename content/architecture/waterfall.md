+++
title = "waterFall — Temporal Ecosystem Sync"
description = "Autonomic heartbeat of the ecosystem — temporal reconciliation that keeps multi-gate, multi-remote systems convergent without a central coordinator."
date = 2026-05-05
weight = 39

[taxonomies]
trails = ["coordination"]

[extra]
foundation = true
domain = "Architecture"
maturity = "architectural"

[[extra.companions]]
url = "/architecture/coordination-triad/"
title = "Coordination Triad"
relation = "extended_by"
label = "waterFall as one of three coordination patterns"

[[extra.companions]]
url = "/architecture/rootpulse/"
title = "rootPulse"
relation = "pairs_with"
label = "Version state that waterFall propagates"
+++

## What It Is

waterFall is not a primal. It is the **SYNC domain** of the
[coordination triad](@/architecture/coordination_triad.md) — an autonomic,
temporal reconciliation pattern that keeps multi-gate, multi-remote ecosystems
convergent without a central coordinator.

**Key metaphor**: Gravity. Changes flow downhill from where they were created.
No pump, no coordinator, no central authority — just the natural tendency of
information to flow toward where it is needed.

---

## The Problem

Spatial Git sync fails when a reachable remote is stale. `git pull origin main`
assumes origin is authoritative. But in a sovereign ecosystem with multiple gates,
multiple Forgejo instances, and multiple GitHub mirrors, **no single remote is
authoritative**. The DAG is the authority.

waterFall replaces spatial preference with **temporal awareness**: fetch all remotes,
measure ahead/behind for each, pull from the leader, push to the followers.

---

## Six Principles

1. **Temporal over spatial** — the most recent commit wins, not the closest remote
2. **DAG as clock** — the git DAG is the only reliable time source
3. **Async over coordinated** — gates sync independently, not in lockstep
4. **Complementary membranes** — inner membrane (LAN) and outer membrane (VPS) sync separately
5. **Gravity not pumping** — changes flow naturally; no cron job forces convergence
6. **Waves over events** — sync happens in waves with sense/measure/act phases

---

## Four Sync Levels

| Level | Scope | Boundary |
|-------|-------|----------|
| **Local** | Working directory to local repo | Developer machine |
| **Gate** | Local repos to gate-level Forgejo | LAN firewall |
| **Membrane** | Gate Forgejo to VPS mirrors | Inner/outer membrane |
| **Ecosystem** | All gates, all mirrors, all USB depots | The whole organism |

Each level nests inside the next. Gate sync uses local sync. Membrane sync uses gate sync.
Ecosystem sync uses membrane sync. The pattern is recursive.

---

## Temporal Position Matrix

For each repository × remote pair, waterFall classifies the temporal relationship:

| Position | Meaning | Action |
|----------|---------|--------|
| Ahead | Local is newer | Push to remote |
| Behind | Remote is newer | Pull from remote |
| Diverged | Both have unique commits | Flag for resolution |
| Parity | Identical state | No action |

The matrix is computed for every repo on every sync wave. The entire ecosystem's
temporal state is visible at a glance.

---

## impulsePotential

Code sync alone lacks intent. impulsePotential adds structured, time-bounded
messages that propagate through the same git infrastructure:

| Impulse Type | Purpose | Lifespan |
|-------------|---------|----------|
| FRAGO | Fragmentary order — immediate directive | Until superseded |
| STATUS | Current state report | One wave |
| AAR | After-action review | Permanent |
| RELEASE | Binary release notification | Until next release |
| SYNC | Sync coordination signal | One wave |

Impulses ride the sync heartbeat like action potentials ride nerve fibers — they carry
intent alongside the code, using the same transport, with the same temporal semantics.

---

## Airgap Federation

waterFall is transport-agnostic. The same sync pattern works over:

- SSH (standard git remote)
- HTTPS (GitHub, Forgejo mirrors)
- file:// (local path, NFS mount)
- USB depot (sneakernet)

This means a gate that is temporarily disconnected can sync via USB drive. The
temporal position matrix does not care how the bits arrived — only when they were
created. A USB drive delivered by mail produces the same sync result as a fiber
connection.

---

## Agentic Sync

When waterFall encounters divergence, it does not hard-stop. Five phases of
graduated resolution:

1. **Structural** — CI fixes that can be auto-merged
2. **SYNC impulses** — notify affected gates of the divergence
3. **Policy-driven** — per-repo merge strategy (fast-forward, rebase, flag)
4. **Provenance-recorded** — resolution documented in {{ entity(name="rhizocrypt") }} DAG,
   signed by {{ entity(name="beardog") }}, attributed by {{ entity(name="sweetgrass") }}
5. **Cross-gate** — context braids enable gate-to-gate divergence resolution

---

## Production Status

| Component | Status |
|-----------|--------|
| `membrane temporal.cascade` (Rust engine) | **Production** (Wave 66) |
| Temporal position matrix | **Live** |
| Multi-remote classification | **Live** |
| impulsePotential messages | Designed |
| Agentic divergence resolution | Designed |
| Cross-gate federation | Designed |

---

*waterFall is the autonomic heartbeat. It does not create artifacts (that is
rootPulse). It does not sense the environment (that is quorumSignal). It ensures
that what was created arrives where it is needed — reliably, temporally, without
a coordinator deciding who gets what when.*
