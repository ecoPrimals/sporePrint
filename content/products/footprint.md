+++
title = "footPrint — GIS Home Planner"
description = "Sovereign GIS home planning tool — the first protoKarya protist. Static SPA live at primals.eco/footprint/, with 10 GIS upstream sources via drawbridge proxy."
date = 2026-07-15

[taxonomies]
primals = ["songbird", "nestgate", "petaltongue"]
springs = ["groundspring"]
trails = ["first-visit"]

[extra]
maturity = "scaffold"

[[extra.companions]]
url = "/products/nf-case-study/"
title = "NF Case Study"
relation = "pairs_with"
label = "Multi-product composition pattern footPrint shares"

[[extra.companions]]
url = "/products/tideglass/"
title = "tideGlass"
relation = "pairs_with"
label = "Sister protist — GPS platform for field science"

[[extra.companions]]
url = "/architecture/composition-patterns/"
title = "Composition Patterns"
relation = "architecture"
label = "How footPrint consumes primals via drawbridge routing"

[[extra.companions]]
url = "/architecture/creative-surface/"
title = "Creative Surface"
relation = "architecture"
label = "The organizational model footPrint emerges from"
+++

{{ maturity(level="scaffold") }} Static SPA live. GIS proxy operational (10 upstream hosts). Server composition not yet deployed.

---

## What It Is

A sovereign GIS home planning tool. Load satellite imagery, draw property
boundaries, place structures, calculate areas and distances, save projects —
all running on your hardware with no cloud account, no API key, no data
harvesting. Your property plans stay on your machine.

footPrint is the first **protist** — a composition that lives in the
[protoKarya](https://github.com/protoKarya) organization, consuming
ecoPrimals infrastructure via drawbridge routing rather than source coupling.

---

## Live Surfaces

| Surface | URL | Status |
|---------|-----|--------|
| Static SPA | [primals.eco/footprint/](https://primals.eco/footprint/) | **Live** |
| GIS proxy (10 upstream hosts) | via Caddy drawbridge | **Live** |
| Express backend (CRUD, agent WS) | — | Not deployed |

The static SPA provides the full mapping UI. The GIS proxy routes tile
requests through {{ entity(name="songbird") }}'s drawbridge to 10 upstream
sources (USGS, FEMA, OpenStreetMap, Esri, NOAA, and others) — the user's
browser never contacts these services directly.

---

## RustScript — Zero-Dependency Type Safety

footPrint's client-side code uses **RustScript**: 12 zero-dependency TypeScript
modules that bring Rust-style safety patterns to the browser. No npm packages,
no bundler plugins, no runtime dependencies — pure type-level guarantees.

| Module | What it provides |
|--------|-----------------|
| `Result<T, E>` | Rust-style error handling — no thrown exceptions |
| `Option<T>` | Explicit nullable handling — no undefined surprises |
| `match()` | Exhaustive pattern matching on discriminated unions |
| `Vec<T>` | Bounds-checked array operations |
| `HashMap<K, V>` | Type-safe key-value with `.get()` returning `Option` |

58 tests validate the RustScript modules. The extraction to
`@protoKarya/rustscript` as an npm package is planned — making these patterns
available to any TypeScript project.

---

## Architecture

```
Browser (SPA)
  │
  ├── Map UI (Leaflet + custom layers)
  ├── Drawing tools (polygon, line, point)
  ├── Project storage (localStorage, future: NestGate CAS)
  │
  └── Tile requests
        │
        ▼
      Caddy (drawbridge proxy)
        │
        ├── USGS National Map
        ├── FEMA flood zones
        ├── OpenStreetMap
        ├── Esri imagery
        ├── NOAA weather
        └── 5 additional GIS sources
```

### Evolution Path

The current architecture is a static SPA with a proxy layer. The composition
evolution wires footPrint into the full primal stack:

| Step | What | Owner |
|------|------|-------|
| Wire `PROXY_PATH` → {{ entity(name="songbird") }} drawbridge | Drawbridge-managed routing | songBird team |
| Wire `PROJECTS_PATH` → {{ entity(name="nestgate") }} CAS | Content-addressed project storage | nestGate team |
| Wire `WS_PATH` → agent bridge | AI-assisted planning via {{ entity(name="squirrel") }} | petalTongue team |
| Deploy composition on sporeGate | Full server-side composition | sporeGate team |
| Create `footprint_composition.toml` | TOML deploy graph manifest | overwatch |

When complete, footPrint becomes a full composition: the SPA talks to
a Rust backend, projects are content-addressed in {{ entity(name="nestgate") }},
and an AI agent can help with property planning via the WebSocket bridge.

---

## Validation

| Scenario | What it proves | Status |
|----------|---------------|--------|
| `protokarya-composition-routing` | Capability routing for footPrint dependencies | Green |
| `fp-api-proxy` | Drawbridge port 7780, {{ entity(name="songbird") }} ownership | Green |
| `footprint-drawbridge-live` | E2E: upstream GIS → drawbridge → NestGate CAS | Missing |

---

## Lineage

footPrint emerged from {{ entity(name="groundspring") }}'s geospatial validation
work. The GIS capabilities proven in spring mathematics (coordinate transforms,
projection handling, spatial indexing) became the foundation for a user-facing
tool. The pattern: springs validate the science, protists make it usable.

---

*footPrint is not a product for sale. It is a demonstration that sovereign
infrastructure can serve everyday needs — plan your garden, map your property,
understand your land. No account required. No data leaves your machine.*
