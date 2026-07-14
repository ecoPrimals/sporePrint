+++
title = "Ecosystem Economics"
description = "The flywheel, enzymatic bounties, sunCloud metabolic economics, and loam certificates — guiding concepts for how the ecosystem sustains itself through attribution rather than artificial scarcity."
date = 2026-05-12
weight = 42

[extra]
domain = "Architecture"
maturity = "architectural"

[[extra.companions]]
url = "/architecture/sovereign-transaction-membrane/"
title = "Sovereign Transaction Membrane"
relation = "pairs_with"
label = "Value flows through the membrane"

[[extra.companions]]
url = "/architecture/golden-cage/"
title = "The Golden Cage"
relation = "extends"
label = "Economic independence from cage services"

[[extra.companions]]
url = "/philosophy/fossil-lineage/"
title = "Fossil Lineage"
relation = "narrative_version"
label = "Origin values that shaped the economic model"
+++

{{ maturity(level="architectural") }} The economic model described here is a **guiding design** — the direction, not the destination. The flywheel is turning (ABG is producing science on live infrastructure). sunCloud and enzymatic economics are design targets being built toward. Nothing here is financial advice or a promise. It's architecture.

---

## The Current Reality

One person pays the electricity. One person bought the hardware. One person mentors the AI sessions that produce all the code. The metabolic cost is real and currently unsubsidized:

| Cost | Monthly | Annual |
|------|---------|--------|
| Electricity (multiple gates) | ~$150 | ~$1,800 |
| Internet (JupyterHub, data, DNS) | ~$85 | ~$1,020 |
| Hardware depreciation (~$15K / 5yr) | ~$250 | ~$3,000 |
| **Total metabolic** | **~$485** | **~$5,820** |

The bar is low because the architecture is sovereign — no cloud bills, no subscription fees, no platform cuts. But it's not zero.

---

## The Flywheel — Where You Come In

The flywheel is how the ecosystem sustains itself before products generate revenue. It is not a detour from the economic model — it is the first expression of it.

```
You offer compute / infrastructure / expertise / money
    ↓
Community produces science on it (ABG, springs, foundation)
    ↓
Science proves the ecosystem works (publications, validated results)
    ↓
People see value and contribute (money, hardware, time, science)
    ↓
Contributions grow the infrastructure (more nodes, more storage)
    ↓
More infrastructure → more science → more proof → more contributions
    ↓
(repeat — each turn adds mass)
```

**The first turn is happening now.** ABG is producing computational chemistry on live sovereign infrastructure at `lab.primals.eco`. Gonzales is mapping NF drug discovery targets. Jones shaped blueFish's analytical chemistry. Each is a turn of the wheel.

### What the Flywheel Accepts

**Money** — direct support for metabolic costs:

| Amount | What It Covers |
|--------|---------------|
| $10/month | Electricity for one gate for ~2 weeks |
| $50/month | Full metabolic cost of a dedicated compute node |
| $200 | 1 TB NVMe drive for fast scratch storage |
| $500 | 4 TB HDD for cold dataset storage |

**Hardware** — physical contributions to the mesh:

| Contribution | Effect |
|-------------|--------|
| Spare GPU (3060, 3070, 3090, etc.) | Adds VRAM to the compute pool |
| Old tower (any generation) | Becomes a new gate in the mesh |
| NPU (Akida, Coral, etc.) | Adds neuromorphic substrate |

**Effort** — the slow burn:

| Contribution | Attribution |
|-------------|-------------|
| Science (validate a paper in a spring) | {{ entity(name="sweetgrass") }} braid: researcher attribution |
| Code (write or audit a spring binary) | AGPL attribution + sweetGrass |
| Outreach (connect a faculty member) | sweetGrass: bridge attribution |
| Sysadmin (help configure a gate) | sweetGrass: infrastructure attribution |

Every contribution — money, hardware, time — gets a {{ entity(name="loamspine") }} certificate. The certificate ferments: as the hardware powers more science and the money enables more infrastructure, its `enabled` record grows. A $200 NVMe donation's certificate might eventually read:

```
enabled:
  ├── 47 ABG validation runs
  ├── 3 publications under scyBorg
  ├── 12,000 BLAKE3-hashed datasets
  └── foundation sediment layers 12–58
```

### The Preferred Flywheel Turn: Run Your Own Mesh

The most valuable contribution isn't money. It's **running your own NUCLEUS gate** — proving the sovereign mesh works by being a node in it. A recycled tower, a NUC, whatever fits: deploy NUCLEUS, connect via {{ entity(name="songbird") }} federation, and your hardware becomes part of the sovereign compute fabric.

This is the invitation: *I'd rather you run your own mesh to prove mine.*

---

## sunCloud — Metabolic Economics (Design Target)

sunCloud is the long-term economic engine. The flywheel is the kickstart. They are the same system at different scales — the flywheel becomes the wheel in the sky.

**Status: guiding architecture. Not yet implemented as software.**

sunCloud is value distribution along {{ entity(name="sweetgrass") }} braids. It is not a payment processor. It is not cryptocurrency. It is the thermodynamic requirement that sustains the organism.

### The Core Contract

> **You forgo personal, exclusionary ownership of any discovery made using the commons. In return, you receive a permanent, verifiable, and proportional share of all future value that discovery ever generates.**

This replaces finite, rivalrous "ownership" with infinite, non-rivalrous "attribution." sweetGrass provides the unimpeachable record. sunCloud provides the mechanism to translate that record into economic reality.

### The Metabolic Mandate

| Category | Rate | Purpose |
|----------|------|---------|
| Infrastructure | 3-7% | Electricity, hardware amortization, VPS |
| Science | 2-5% | Spring validation, data processing |
| Products | 0% | Products compose primals — no additional cost |
| Ecosystem | 1-3% | Coordination, wateringHole, sporePrint |

These rates are not taxes. They are thermodynamic requirements — the minimum energy the organism needs to maintain homeostasis.

### Worked Example: Content Pack ($5)

| Recipient | Share | Amount |
|-----------|-------|--------|
| Creator | 65% | $3.25 |
| Derived-from attribution | 10% | $0.50 |
| Infrastructure | 10% | $0.50 |
| Science springs | 5% | $0.25 |
| Ecosystem coordination | 2% | $0.10 |
| Treasury (future investment) | 8% | $0.40 |

### The Anti-Platform Principle

Products keep their revenue. The platform sustains itself through metabolic rates — not through platform rent, not through data harvesting, not through lock-in.

| Platform | Cut |
|----------|-----|
| Apple App Store | 30% platform rent |
| Steam | 30% platform rent |
| sunCloud | 0% product tax, 3-7% infrastructure metabolic rate |

---

## Enzymatic Economics (Design Target)

**Status: conceptual. Not yet implemented. This section describes a design direction.**

Enzymes lower activation energy — they make reactions possible that thermodynamics allows but kinetics forbids. The same principle applies to research funding:

### Catalytic Bounties

Traditional grants fund predetermined outcomes. Enzymatic bounties fund **conditions** and let the ecosystem determine outcomes:

- **Prize Bounties**: Well-defined problems. An entity posts a reward paid out via radiating attribution to whoever verifiably solves it. Post-reward for a specific result.

- **Catalytic Bounties (Co-Investment)**: Ambitious, long-term goals requiring upfront investment. An external organization co-invests seed capital. In return, they're written into the foundational sweetGrass attribution chain for all resulting work — a perpetual, proportional beneficiary alongside the researchers and the commons.

### The Metabolic Regulator

The square-cube law applies to network economics:

- **Network Cost (squared)**: infrastructure grows with the network's size
- **Network Value (cubed)**: discoveries, correlations, and emergent capabilities grow faster than costs

sunCloud acts as the metabolic regulator — ensuring the explosive "cubed" growth in value nourishes the sustainable "squared" growth in cost. The treasury is not a bank to be hoarded but a heart that pumps value back into the network.

When the treasury accumulates beyond operational needs, it discharges through:
- **Seeding new gates**: hardware for NUCLEUS deployments in resource-constrained regions
- **Funding education**: scholarships and grants for scientists and developers
- **Infrastructure development**: new tools, networks, and data capture methods

---

## Loam Certificates

{{ entity(name="loamspine") }} certificates transform from infrastructure into user-facing products — ownership, credentials, and chain-of-custody.

### Certificate Types

| Type | What It Certifies | Example |
|------|------------------|---------|
| **Game key** | Ownership of a game/content | Verifiable, lendable, sovereign |
| **Collectible** | History of a game object (Novel Ferment Transcript) | The history IS the value |
| **Creator credential** | Provenance of creative work | Living portfolio, not static resume |
| **Ruleset certificate** | Inspectable game/AI constraints | Players can verify the rules |
| **Chain-of-custody** | Provenance of a physical/digital object | Science samples, field data |

### Lending Protocol

Loam certificates support temporary access without ownership transfer. You can lend a game to a friend. The certificate records the loan. When the loan expires, access reverts. No DRM server required — the certificate itself enforces the terms.

| Model | Ownership |
|-------|-----------|
| Steam license | You do not own the game |
| Loam certificate | You own it, can lend it, can verify its history |

---

## Memory-Bound Value

The economic model is **memory-bound**: value comes from what the ecosystem remembers (validated results, provenance chains, attribution DAGs), not from artificial scarcity (licenses, subscriptions, usage metering).

A journal paper behind a paywall creates artificial scarcity. A sovereign publication with {{ entity(name="guidestone") }} verification creates memory-bound value — the value is in the proof, the provenance, and the attribution, which grow richer over time as more people verify and extend the work.

---

## The Transition

| | Flywheel (now) | sunCloud (design target) |
|---|---|---|
| **Value source** | Donations, hardware, sweat equity | Product revenue (games, tools, science) |
| **Distribution** | Direct to metabolic costs + hardware | sweetGrass braids → attributed splits |
| **Attribution** | Loam Certificates for contributions | Loam Certificates for all value flow |
| **Metabolic** | 100% goes to survival | 5–15% metabolic, rest to creators |
| **Treasury** | Not yet (all funds = metabolic) | Accumulates from metabolic mandate |
| **Governance** | Architect decides (bootstrapping) | Contribution-weighted ecosystem |

Early contributors don't get diluted. They get compounded. Their braids are in the earliest sediment layers — the deepest geology. When the ecosystem is mature, the bedrock still carries their names.

---

*The ecosystem's economics follow its biology: metabolic rates sustain the organism, attribution tracks contribution, and value accrues through memory rather than scarcity. The flywheel turns because people see value. The value compounds because the flywheel turns. The geology accumulates. The commons grows.*
