+++
title = "The Lansing Scuffle"
description = "A 464K SF wartime factory becoming a solarpunk sovereign campus — data center, wet lab, community services, and rooftop gardens in one building."
date = 2026-07-20

[taxonomies]
primals = ["nestgate", "songbird", "toadstool", "beardog"]
springs = ["airspring", "groundspring", "wetspring"]
trails = ["first-visit"]

[extra]

[[extra.companions]]
url = "/vision/thermal_sovereignty_building/"
title = "Building-Scale Thermal Sovereignty"
relation = "pairs_with"
label = "The energy loop that makes the campus self-sustaining"

[[extra.companions]]
url = "/products/footprint/"
title = "footPrint"
relation = "pairs_with"
label = "GeoJSON model of the building's parcel and K-Derm zones"

[[extra.companions]]
url = "/outreach/consulting/"
title = "Sovereign Consulting"
relation = "pairs_with"
label = "How the expertise sustains the infrastructure"
+++

## The Building

1305 South Cedar Street, Lansing, Michigan. A wartime factory built in 1941,
sitting on 12 acres. 464,281 square feet across a three-story south section
and a single-story north warehouse with 14-foot-7-inch ceilings. Five loading
docks. A rail spur. 8 megawatts of transformer capacity. Approximately 600
tons of cooling. Currently vacant.

The building was constructed for John Bean fire truck manufacturing, evolved
through FMC industrial production, housed artists and light manufacturing,
and most recently served as a cannabis cultivation facility — a use case that
solved the power-density and per-room electrical isolation problem before
sovereign compute was even a concept. Each third-floor room has 400A at 480V
and its own 35-ton HVAC unit. The infrastructure is already there.

---

## The Shuffle and the Scuffle

Lansing already has a venue called the **Lansing Shuffle** — a riverfront
entertainment district in the former Christman building. The Shuffle monetizes
desirability: a restored building on the Grand River, food trucks, craft beer,
Instagram-ready aesthetics.

The Scuffle is the inversion. It creates value from what others overlook:
a vacant factory next to train tracks, 8 megawatts of power that no one is
using, 14-foot ceilings that are too tall for offices and too industrial for
retail. The cross-traffic is not the point. The silicon, the science, the
community service — that is the point.

---

## K-Derm Zones

The building follows the same **K-Derm membrane model** that structures every
ecoPrimals deployment — from a single-board computer to a six-gate home mesh.
At 464K SF, the zones have physical floors:

| Floor | Zone | Function |
|-------|------|----------|
| 3rd (south) | Cytoplasm | Sovereign compute — GPU racks, primal services, mesh backbone |
| 2nd (south) | Periplasm | Science — wet lab, dry lab, instrumentation, maker spaces |
| 1st (south) | Outer membrane | Community — hot water station, WiFi, warming center, event space |
| North warehouse | Extracellular | Thermal storage — sand batteries, greenhouses, loading, staging |
| Roof | Membrane surface | Solar panels, rooftop gardens, mesh antennas, weather stations |

The humanitarian zone on the first floor is maximally permeable by design.
Hot water, phone charging, WiFi, and warmth flow outward without authentication.
This is not a security gap — it is the architecture.

---

## Thermal Sovereignty

The building's energy loop uses every joule twice:

```
Solar (100K SF roof) → Electricity → GPU compute
                                        ↓
                                   GPU heat (glycol loops)
                                        ↓
                              Sand thermal batteries (warehouse bays)
                                        ↓
                    ┌───────────────────┼───────────────────┐
                    ↓                   ↓                   ↓
            Hot water station    Greenhouse heating    Building HVAC
            (community, 24/7)    (year-round food)    (winter offset)
```

The GPU racks are not the building's problem — they are its furnace. In a
Michigan winter, the same computation that trains models and runs simulations
provides the heat that warms the building, grows food in rooftop greenhouses,
and supplies hot water to a community station open 24 hours a day.

Sand thermal batteries store heat at a fraction of the cost of electrical
storage. Sand does not degrade. Sand does not catch fire. Sand does not need
battery management systems. The north warehouse bays, with 14-foot ceilings
and industrial floor loading, are built for exactly this kind of mass storage.

For the full thermal architecture, see
[Building-Scale Thermal Sovereignty](@/vision/thermal_sovereignty_building.md).

---

## The Beachhead

The campus does not start at 464K SF. It starts with one room.

The third-floor rooms are self-contained: individual HVAC, individual electrical
service, individual access. A single room with pre-installed 400A/480V power
and 35-ton cooling is the entry point. Run a handful of GPU nodes, prove the
thermal loop works at room scale, establish the mesh backbone, and grow from
there.

Phase 0 is always residential. The existing house-scale deployment — the same
WireGuard mesh, the same {{ entity(name="songbird") }} federation, the same
13 primals — continues to operate regardless of what happens at the building.
If the building fails, the ecosystem continues from the houses. If the building
succeeds, the houses become residential nodes in the campus mesh.

---

## Humanitarian Anchor

A scuffle is a small, scrappy fight — not a battle, not a war. The Good
Samaritan scuffled. He stopped, bandaged, paid the innkeeper, and moved on.
He didn't build an institution. He prepared a room.

The building prepares many rooms:

- **Hot water** — GPU-heated, available 24/7, no credentials required
- **Phone charging** — USB stations, no data collection
- **Open WiFi** — mesh-backed, no DNS logging, no identity harvesting
- **Warming center** — sand-battery heated, daytime hours in winter
- **Community kitchen** — rooftop garden produce, shared preparation space
- **Sovereign identity** — {{ entity(name="beardog") }} biometric enrollment
  for people who have no government ID. No second-class identities

**The Fledge** — Lansing's existing social enterprise incubator and sanctuary
organization — is a natural partner. The Fledge operates from small spaces
that constrain their programs. The building solves the single-room problem.
Community is structural, not decorative.

---

## Science and Makers

The second floor houses the science:

- **BSL-1 wet lab** — bench space, fume hoods, autoclave, -80°C freezer.
  Every sample that touches the bench enters the {{ entity(name="nestgate") }}
  provenance chain via {{ entity(name="loamspine") }}
- **Dry lab / instrumentation** — microscopy, spectroscopy, analytical equipment
  connected to spring pipelines
- **Maker spaces** — shared fabrication, prototyping, and hardware development
- **{{ entity(name="airspring") }} sensor grid** — ESP32 + capacitive moisture,
  temperature, humidity, CO₂, and light sensors monitoring rooftop gardens and
  lab environments

The spring-to-bench pipeline: data flows from the instrument through
{{ entity(name="nestgate") }} CAS to the appropriate spring, with provenance
tracked by {{ entity(name="loamspine") }}, integrity verified by
{{ entity(name="rhizocrypt") }}, and attribution recorded by
{{ entity(name="sweetgrass") }}.

---

## Network Supernode

The building's three-story industrial roof becomes a mesh supernode, replacing
the current residential antenna with commercial-grade sector coverage. Same
K-Derm membrane model, same WireGuard overlay, same {{ entity(name="songbird") }}
federation — but with commercial fiber, 10G+ internal backbone, and a rooftop
position that covers the Old Everett neighborhood.

Open WiFi in the humanitarian zone uses the mesh backbone. No authentication,
no harvesting — funded by the compute infrastructure above it.

---

## The footPrint Model

The building's parcel boundary, building footprint, and K-Derm zone layout
are modeled in [footPrint](@/products/footprint.md) as a GeoJSON project
(`projects/lansing-scuffle.json`). The same sovereign GIS tool that plans
home gardens and property layouts serves as the spatial documentation for
a 464K SF campus — the organism is the same, the habitat is larger.

---

## Timeline

| Phase | Period | What Happens |
|-------|--------|-------------|
| Document | Year 0 (now) | Model the building, prove thermal at house scale, build community |
| Contact | Year 1 | Beachhead lease, first room operational, mesh backbone live |
| Expand | Year 2–3 | Second suite, wet lab, solar pilot, rooftop garden pilot |
| Operate | Year 3–4 | Multiple tenants, full thermal loop, community services active |
| Campus | Year 4–5 | Building acquisition, full sovereign campus |

The timeline is a plan, not a promise. Each phase proves the next one is
viable before committing to it. Grants accelerate, not enable — the
beachhead operates on metabolic cost alone if necessary.

---

## What This Is Not

This is not a data center project. It is not a community center project. It
is not a wet lab project. It is not a real estate investment. It is the same
primal composition model that runs at house scale — K-Derm zones, WireGuard
mesh, {{ entity(name="songbird") }} federation, spring mathematics — applied
to a building whose industrial infrastructure makes it possible.

Gardens on the roof, not just antennas. Food alongside fiber. Warmth alongside
compute. Science alongside service.

Solarpunk, not cyberpunk.

---

*Every constraint that survived became load-bearing architecture.*
