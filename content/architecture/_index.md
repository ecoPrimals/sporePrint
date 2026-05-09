+++
title = "🏗️ Architecture"
description = "What was built — the ecosystem architecture, NUCLEUS composition model, primal catalog, spring catalog, ecosystem inventory, deployment model, and evolution timeline."
sort_by = "title"
template = "section.html"
+++

Small, purpose-built Rust programs (primals) compose into larger systems (atomics), which compose into sovereign compute substrates (NUCLEUS), which run science. The architecture follows chemistry, not enterprise software.

```
Springs (8 domains)              → validate the science
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

---

## Where to start

| I want to... | Read this |
|---|---|
| Understand the whole system | [Ecosystem Architecture](ECOSYSTEM_ARCHITECTURE.md) — primals, springs, composition, Neural API |
| See how primals compose on a machine | [NUCLEUS Architecture](NUCLEUS_ARCHITECTURE.md) — atomics ladder, deploy graphs, lifecycle |
| Browse every primal | [Primal Catalog](PRIMAL_CATALOG.md) — 15 primals with metrics, primitives, and tiers |
| Browse every spring | [Spring Catalog](SPRING_CATALOG.md) — 8 springs with checks and papers reproduced |
| Deploy it myself | [Deployment Model](DEPLOYMENT_MODEL.md) — {{ entity(name="plasmidbin") }} binary distribution |
| See how it was built | [Evolution Timeline](EVOLUTION_TIMELINE.md) — the 27-day sprint narrative |
| Find every repo | [Ecosystem Inventory](ECOSYSTEM_INVENTORY.md) — complete map across all organizations |
| Check prior art and licensing | [Sovereign Prior Art](SOVEREIGN_PRIOR_ART_CATALOG.md) — AGPL analysis per primal |
| See how springs feed products | [Composition Pipeline](COMPOSITION_PIPELINE.md) — springs → compositions → products → foundation |
