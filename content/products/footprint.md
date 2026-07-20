+++
title = "footPrint — GIS Home Planner"
description = "Sovereign GIS home planning tool — the first protoKarya protist. LIVE at footprint.primals.eco with 10 GIS upstream sources via songBird drawbridge proxy."
date = 2026-07-15

[taxonomies]
primals = ["songbird", "nestgate", "petaltongue"]
springs = ["groundspring"]
trails = ["first-visit"]

[extra]
maturity = "live"

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

{{ maturity(level="live") }} **LIVE** at [footprint.primals.eco](https://footprint.primals.eco) (200, 216ms WAN). Code complete: {{ entity_stat(name="footprint", stat="tests_display") }} tests, responsive design, accessibility, CSP + security headers.

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
| Static SPA | [footprint.primals.eco](https://footprint.primals.eco) | **Live** (200) |
| GIS proxy (10 upstream hosts) | via Caddy drawbridge | **Live** |
| WebSocket bridge | `/ws` → petalTongue:8080 | **Live** (Wave 150g) |
| CAS backend | nestGate `PROJECTS_PATH` | **Wired** (consumer verify pending) |

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
  ├── Project storage (localStorage + NestGate CAS wired)
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

| Step | What | Status |
|------|------|--------|
| ~~Wire `PROXY_PATH` → {{ entity(name="songbird") }} drawbridge~~ | Drawbridge-managed routing | **Done** (Wave 148b) |
| ~~Responsive design + accessibility~~ | Breakpoints, ARIA, focus traps, mobile drawer | **Done** (Wave 150c) |
| ~~Known locations + E2E tutorial~~ | 5 verification locations per Live Frontend Standard | **Done** (Wave 149b) |
| ~~Fix Caddy routing + CSP~~ | Route `footprint.primals.eco` → sporeGate:8090, CSP for tile domains | **Done** (Wave 150e) |
| ~~Wire `PROJECTS_PATH` → {{ entity(name="nestgate") }} CAS~~ | Content-addressed project storage | **Done** (Wave 150e) |
| ~~Wire `WS_PATH` → {{ entity(name="petaltongue") }} bridge~~ | WebSocket JSON-RPC on `/ws` :8080 | **Done** (Wave 150g) |
| Verify CAS consumer wiring | footPrint client → nestGate | Pending (footPrint team) |
| Create `footprint_composition.toml` | TOML deploy graph manifest | Open |

footPrint is now a full composition: the SPA serves from Express on
sporeGate, projects are content-addressed via {{ entity(name="nestgate") }},
and the {{ entity(name="petaltongue") }} WebSocket bridge enables real-time
agent communication. Client-side wiring for CAS and WS is the remaining
integration step.

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

## Known Locations

The footPrint project system includes modeled locations:

- **Lansing Scuffle** (`projects/lansing-scuffle.json`) — parcel boundary,
  building footprint, and K-Derm zone polygons for the 464K SF campus at
  1305 S Cedar St. See [The Lansing Scuffle](@/vision/lansing_scuffle.md)
  for the campus vision

---

*footPrint is not a product for sale. It is a demonstration that sovereign
infrastructure can serve everyday needs — plan your garden, map your property,
understand your land. No account required. No data leaves your machine.*
