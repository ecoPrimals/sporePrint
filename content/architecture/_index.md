+++
title = "🏗️ Architecture"
description = "What was built — the ecosystem architecture, NUCLEUS composition model, primal catalog, spring catalog, ecosystem inventory, deployment model, and evolution timeline."
sort_by = "title"
template = "section.html"
+++

The architecture of {{ entity(name="ecoprimals") }} follows the structure of
chemistry: small, purpose-built programs ({{ entity(name="unibin") }} primals)
compose into molecules (atomics), which compose into organisms (products),
which reproduce science (springs). Everything is open, everything is
verifiable, everything runs on your hardware.

---

## The Composition Stack

```
Springs (7 science + 1 meta)     → validate the science
  ↓ produce kernels, experiments, evidence
Primals (15 Rust binaries)       → provide the capabilities
  ↓ compose via deploy graphs
Atomics (Tower / Node / Nest)    → minimum viable compositions
  ↓ assemble into
NUCLEUS (full composition)       → sovereign compute substrate
  ↓ runs
Products (helixVision, etc.)     → emergent tools for real work
  ↓ proves patterns for
foundation                       → institutional adoption
```

Springs produce the science. Primals provide the infrastructure. Products
emerge from composition. {{ entity(name="primalspring") }} validates that the
composition layer itself works — the meta-spring that tests the substrate
every other spring runs on.

## Documents

- [Ecosystem Architecture](ECOSYSTEM_ARCHITECTURE.md) — the full model: primals, springs, composition, Neural API
- [NUCLEUS Architecture](NUCLEUS_ARCHITECTURE.md) — atomics, deploy graphs, substrate lifecycle
- [Primal Catalog](PRIMAL_CATALOG.md) — all 15 primals with metrics, primitives, and tiers
- [Spring Catalog](SPRING_CATALOG.md) — all 8 springs with checks, papers reproduced, and data flow
- [Deployment Model](DEPLOYMENT_MODEL.md) — {{ entity(name="plasmidbin") }} harvest/fetch, binary distribution
- [Evolution Timeline](EVOLUTION_TIMELINE.md) — the 27-day sprint narrative
- [Ecosystem Inventory](ECOSYSTEM_INVENTORY.md) — complete repository map across all organizations
- [Sovereign Prior Art](SOVEREIGN_PRIOR_ART_CATALOG.md) — AGPL prior-art analysis per primal
- [Composition Pipeline](COMPOSITION_PIPELINE.md) — how springs feed into compositions, products, and foundation
