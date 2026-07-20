+++
title = "Building-Scale Thermal Sovereignty"
description = "Solar → compute → heat → hot water → food. The full thermal loop at 464K SF — how GPU exhaust heats a building, grows food, and serves a community."
date = 2026-07-20

[taxonomies]
primals = ["barracuda", "coralreef", "toadstool"]
springs = ["airspring"]
trails = ["first-visit"]

[extra]

[[extra.companions]]
url = "/vision/lansing_scuffle/"
title = "The Lansing Scuffle"
relation = "pairs_with"
label = "The campus where this thermal architecture deploys"

[[extra.companions]]
url = "/architecture/sovereign-hpc-evolution/"
title = "Sovereign HPC Evolution"
relation = "architecture"
label = "The compute infrastructure that generates the heat"
+++

## Two Energies

A building has two energy systems: electricity (the nervous system) and heat
(the circulatory system). Conventional data centers treat heat as waste — an
expensive problem solved by chillers and cooling towers. Thermal sovereignty
treats heat as a resource. There is no "waste heat." There is only energy in
different forms, and every joule is used twice.

The [Lansing Scuffle](@/vision/lansing_scuffle.md) campus makes this concrete.
The building — 464,281 SF, 8 MW transformer capacity, ~600 tons of existing
cooling, 14-foot-7-inch ceilings — already has the infrastructure for both
energy systems. The thermal sovereignty loop connects them.

---

## The Loop

```
┌─────────────────────────────────────────────────────────────────┐
│                        ROOFTOP (100K SF)                        │
│   Solar panels → DC electricity → GPU compute (3rd floor)      │
│   Greenhouses ← heat ← sand batteries ← GPU exhaust           │
│   Weather stations + airSpring sensors                         │
└─────────────────────────────────────────────────────────────────┘
                              ↓ electricity    ↑ heat
┌─────────────────────────────────────────────────────────────────┐
│                     3RD FLOOR — CYTOPLASM                       │
│   GPU racks → glycol heat capture loops                        │
│   Sovereign compute: {{ entity(name="barracuda") }},           │
│   {{ entity(name="coralreef") }}, {{ entity(name="toadstool") }}│
└─────────────────────────────────────────────────────────────────┘
                              ↓ heat (glycol)
┌─────────────────────────────────────────────────────────────────┐
│                 NORTH WAREHOUSE — EXTRACELLULAR                 │
│   Sand thermal batteries (14'7" ceilings, industrial loading)  │
│   Heat stored at low cost, dispatched seasonally               │
└─────────────────────────────────────────────────────────────────┘
                              ↓ heat (dispatched)
          ┌───────────────────┼───────────────────┐
          ↓                   ↓                   ↓
┌──────────────────┐ ┌──────────────────┐ ┌──────────────────┐
│   HOT WATER      │ │   GREENHOUSES    │ │   BUILDING HVAC  │
│   Community      │ │   Year-round     │ │   Winter heating  │
│   station, 24/7  │ │   food production│ │   offset         │
│   No credentials │ │   GPU-warmed     │ │   Sand-backed    │
└──────────────────┘ └──────────────────┘ └──────────────────┘
```

---

## Solar Capacity

The building's roof area is approximately 100,000 usable square feet for solar
installation. At typical panel density, this supports a significant DC
generation capacity — enough to power dozens of GPU nodes directly from
rooftop generation during peak sun hours, with grid power as baseline.

Michigan (USDA zones 5b/6a) has a solar profile with strong summers and weak
winters. The seasonal strategy:

| Season | Solar | Compute | Heat Dispatch |
|--------|-------|---------|--------------|
| Summer | Peak generation | Maximum GPU throughput | Charge sand batteries |
| Shoulder | Moderate | Steady state | Sand → greenhouses, building preheat |
| Winter | Minimal | Grid-powered | Sand → hot water, greenhouse, building heat |

The campus is grid-connected, not grid-independent. Solar reduces operating
cost and provides sovereignty during grid instability, but the 8 MW
transformer capacity is the primary power source.

---

## GPU Heat Capture

GPU racks produce heat as a byproduct of computation. In a conventional data
center, this heat is rejected to the atmosphere via cooling towers. In the
Scuffle, heat is captured at the rack via glycol cooling loops and routed to
the thermal storage system.

The existing third-floor rooms each have dedicated HVAC units and electrical
service. The same per-room isolation that allowed cannabis cultivation —
individual climate control, individual power, individual access — enables
per-room thermal capture. Each room is a thermal cell.

The compute dispatched by {{ entity(name="toadstool") }} is thermal-aware:
workloads can be scheduled to rooms where heat demand is highest, making the
GPU racks responsive to the building's thermal needs, not just computational ones.

---

## Sand Thermal Batteries

The north warehouse — single-story, 14-foot-7-inch ceilings, industrial floor
loading rated for heavy equipment — is built for thermal mass storage. Sand
thermal batteries store heat captured from GPU exhaust for later dispatch.

Sand as a storage medium:

- **Does not degrade** over charge/discharge cycles
- **Does not catch fire** — no thermal runaway risk
- **Requires no battery management system** — passive storage
- **Orders of magnitude cheaper** than electrical storage per kWh
- **Stores heat for days to weeks** depending on insulation

The warehouse bays provide the volume, the floor loading supports the mass,
and the ceiling height allows proper insulation layering. What looks like
unused industrial space is actually the building's circulatory reservoir.

---

## Heat Dispatch

Stored thermal energy is dispatched to three endpoints:

### Hot Water — Community Station

GPU-heated water at 40–50°C, available 24 hours a day to anyone who walks in.
No credentials, no identity check, no means testing. A person who can wash
their hands in warm water has dignity. This is not a luxury — it is the
architecture of the humanitarian zone.

### Rooftop Greenhouses

GPU exhaust heat extends the growing season to year-round in Michigan's climate.
The rooftop greenhouses are simultaneously:

- **Food production** — vegetables and herbs for the community kitchen
- **Instrumented science** — {{ entity(name="airspring") }} sensor grids
  (temperature, humidity, CO₂, soil moisture, light) feeding data through
  {{ entity(name="nestgate") }} CAS into spring analysis pipelines
- **Visible solarpunk** — gardens on the roof, visible from Cedar Street
  and the rail corridor

### Building HVAC Offset

In winter, sand-stored heat supplements the building's HVAC system, reducing
natural gas consumption. The same GPU computation that runs scientific
simulations during the day heats the building at night. Every joule used twice.

---

## Seasonal Strategy

The thermal system dispatches differently by season:

**Winter (November–March):** GPU heat is the primary heating source. Sand
batteries charged during compute peaks are discharged overnight for building
heating. Hot water station demand is highest. Greenhouses rely entirely on
GPU thermal input.

**Shoulder (April–May, September–October):** Solar begins contributing.
Sand batteries are gradually charged for winter. Greenhouses transition
between GPU heat and ambient temperature. Building HVAC demand drops.

**Summer (June–August):** Peak solar generation. Maximum GPU throughput
funded by rooftop generation. Sand batteries charge deeply. Excess heat
is managed via existing cooling infrastructure. Hot water demand drops
(ambient temperature handles basic needs).

---

## From House to Building

The thermal sovereignty concepts deployed at house scale — GPU heat recovery
to domestic hot water, compute workload scheduling aligned with heating demand,
solar offset of grid consumption — are the same concepts at building scale.
The organism is the same. The habitat is larger.

| Concept | House Scale | Building Scale |
|---------|------------|---------------|
| Heat source | 5–7 GPUs | 50–100+ GPUs |
| Storage | Domestic hot water tank | Sand thermal batteries |
| Distribution | Household radiators | Building HVAC + community station |
| Food | Backyard garden | Rooftop greenhouses (10,000+ SF) |
| Sensors | {{ entity(name="airspring") }} — 3–5 nodes | {{ entity(name="airspring") }} — 50+ nodes |
| Network | Residential mesh | Industrial mesh supernode |

The scaling is not linear — building-scale thermal storage, community-scale
hot water, and year-round greenhouse production are capabilities that only
emerge at industrial scale. But the architecture, the K-Derm zone model, and
the primal composition are identical.

---

*The building is just a larger cell.*
