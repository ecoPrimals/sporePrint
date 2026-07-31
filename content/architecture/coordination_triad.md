+++
title = "Coordination Triad"
description = "Three coordination patterns — quorumSignal (sense), rootPulse (action), waterFall (sync) — integrated through the Neural API."
date = 2026-05-01
weight = 38

[taxonomies]
trails = ["coordination"]

[extra]
foundation = true
domain = "Architecture"
maturity = "architectural"

[[extra.companions]]
url = "/architecture/waterfall/"
title = "waterFall"
relation = "extends"
label = "Temporal sync — one leg of the triad"

[[extra.companions]]
url = "/architecture/rootpulse/"
title = "rootPulse"
relation = "extends"
label = "Emergent version control — one leg of the triad"

[[extra.companions]]
url = "/architecture/neural-api/"
title = "Neural API"
relation = "extends"
label = "Adaptive orchestration — one leg of the triad"

[[extra.companions]]
url = "/architecture/coordination/"
title = "Ecosystem Coordination"
relation = "pairs_with"
label = "Public coordination standards in wateringHole"
+++

## The Pattern

Three coordination patterns form the nervous system of the ecosystem. None is a primal.
Each is a pattern that emerges when {{ entity(name="biomeos") }} orchestrates existing primals
through TOML composition graphs.

```
                    Neural API
                  (integration)
                       |
         +-------------+-------------+
         |             |             |
    quorumSignal   rootPulse    waterFall
     (SENSE)       (ACTION)      (SYNC)
     observe       create       propagate
```

| Pattern | Domain | Biological Analog | What It Does |
|---------|--------|------------------|-------------|
| **quorumSignal** | SENSE | Quorum sensing | Observes environment, discovers capabilities, classifies drift |
| **[rootPulse](@/architecture/rootpulse.md)** | ACTION | Muscle contraction | Creates, mutates, proves — version control as emergent coordination |
| **[waterFall](@/architecture/waterfall.md)** | SYNC | Autonomic heartbeat | Reconciles, propagates, maintains temporal coherence across gates |
| **[Neural API](@/architecture/neural_api.md)** | Integration | Central nervous system | Executes capability graphs, routes signals, learns from traces |

---

## Why Three Patterns

Each pattern addresses a different temporal concern:

- **Sense** (quorumSignal): What is the current state? What has changed? What capabilities are available?
- **Action** (rootPulse): Create a versioned artifact with provenance. The commit, the branch, the merge.
- **Sync** (waterFall): Propagate changes across gates and remotes. Maintain ecosystem convergence without a central coordinator.

A single pattern cannot handle all three. A system that only senses never creates. A system that only creates never propagates. A system that only syncs never understands what it is syncing.

---

## The Neural API as Integrator

The Neural API is the layer that composes the triad:

- It receives quorumSignal observations (what primals are available, what capabilities exist)
- It dispatches rootPulse actions (create provenance DAG, sign, store, commit)
- It triggers waterFall sync (cascade changes to remotes, resolve divergence)

The integration is not hardcoded. The Neural API executes TOML-defined capability
graphs — the same graph execution model used for products and deploy graphs. The
triad patterns are compositions, not services.

---

## Mass-Energy-Information

The triad also maps to a physical model:

| Concept | Physical Analog | What It Carries |
|---------|----------------|----------------|
| Spores (artifacts) | Mass | The thing itself — code, data, provenance |
| waterFall (propagation) | Energy | The force that moves artifacts across boundaries |
| Braids (intent) | Information | The meaning — why this change, for whom, toward what goal |

Spores have mass (they take up space, require storage, have weight in the mesh).
waterFall is energy (it does work, moving spores from where they were created to
where they are needed). Braids carry information (semantic attribution, intent
signals, impulse potentials that ride the sync heartbeat).

---

## Implementation Status

| Component | Status |
|-----------|--------|
| quorumSignal sensing | {{ entity(name="biomeos") }} primal discovery + health probes (**live**) |
| rootPulse provenance trio | {{ entity(name="rhizocrypt") }} + {{ entity(name="loamspine") }} + {{ entity(name="sweetgrass") }} (**live**, 2,308+ tests) |
| rootPulse composition graphs | 5 graphs defined (commit, branch, merge, diff, federate) |
| rootPulse CLI frontend | Not yet built |
| waterFall temporal cascade | `membrane temporal.cascade` (**live** since Wave 66) |
| waterFall impulse potential | Designed — structured messages riding sync heartbeat |
| Neural API graph execution | Phases 1-2 complete, Phase 3 partial |
| Neural API PathwayLearner | Exists but unwired |
| Neural API atomic signal dispatch | 32 composition graphs defined (Phase 3.5A) |

---

## Primals Involved

The triad does not introduce new primals. It coordinates existing ones:

| Primal | Role in Triad |
|--------|-------------|
| {{ entity(name="biomeos") }} | Composition engine — executes all three patterns |
| {{ entity(name="rhizocrypt") }} | DAG storage — the ever-branching present/future |
| {{ entity(name="loamspine") }} | Linear storage — the immutable past |
| {{ entity(name="sweetgrass") }} | Attribution — semantic contribution tracking |
| {{ entity(name="nestgate") }} | Content-addressed storage — artifacts at rest |
| {{ entity(name="beardog") }} | Signing — cryptographic attestation |
| {{ entity(name="songbird") }} | Federation — cross-gate transport |

---

*The coordination triad is the nervous system of the ecosystem. quorumSignal senses.
rootPulse acts. waterFall maintains homeostasis. The Neural API integrates them into
a coherent whole. No new code required — only new compositions of existing primals.*
