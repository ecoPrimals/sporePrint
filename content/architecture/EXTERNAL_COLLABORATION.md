+++
title = "External Collaboration Model"
description = "How sovereign infrastructure enables external science production — the gen5 collaborator gate pattern."
date = 2026-05-31
weight = 18

[extra]
domain = "Architecture"
maturity = "architectural"
+++

## The gen5 Pattern

gen5 asks: does someone else's science come out the other end? The external
collaboration model makes this architecturally possible — collaborators use
the same infrastructure patterns as internal gates, scoped to their domain.

## Sovereignty Enables Collaboration

Sovereignty is not isolation. The ecosystem's self-hosted infrastructure
(Forgejo, WaterFall sync, sovereign DNS) provides the substrate for external
collaboration without vendor lock-in:

- Collaborators sync through the same pipelines as internal gates
- Data stays on sovereign infrastructure (no cloud vendor ingestion)
- Provenance is tracked end-to-end (every computational step attributable)
- The collaborator owns their output (pseudoSpore as delivery format)

## The Collaborator Gate Model

External collaborators get a **gate profile** in the ecosystem manifest,
scoped to repos relevant to their domain:

```toml
[gates.gonzales_nf]
repos = [
    "wateringHole",
    "helixVision", "initioChem", "blueFish",
    "wetSpring", "hotSpring",
    "projectFOUNDATION",
]
```

They pull only what they need. They never see NUCLEUS internals, unrelated
springs, or infrastructure repos. The WaterFall pipeline handles scoping
automatically.

## What the Collaborator Brings

1. **A biological question** the ecosystem hasn't answered
2. **Domain data access** (NF Data Portal, LTEE datasets, analytical standards)
3. **Domain expertise** (signaling biology, microbial evolution, analytical chemistry)
4. **Institutional authority** (PI status for grants, publication lead)

## What the Ecosystem Provides

1. **Validated computation** — {{ total_stat(stat="validation_checks") }} checks, {{ total_stat(stat="papers_reproduced") }} papers reproduced across 8 domains
2. **Multi-product composition** — orchestrated products for the collaborator's question
3. **GPU compute** at zero cost to the institution
4. **Self-verifying artifacts** — pseudoSpore packaging with full provenance
5. **AI-accelerated coordination** — metadata extraction, cross-checking, assembly

## What Comes Out

1. **A pseudoSpore** — self-verifying data package the collaborator owns
2. **Grant preliminary data** — foundation-ready computational evidence
3. **New spring validation targets** — the ecosystem grows from collaborator science
4. **A reproducibility record** — every step provenance-tracked

## Multi-Product Composition (gen5 novelty)

gen4 products each composed primals independently. gen5 demands products
compose with each other — driven by the biological question, not internal design.

Example — neurofibromatosis data mining requires:
- **helixVision** for gene expression mining from NF Data Portal
- **healthSpring** for drug repurposing scoring against NF targets
- **initioChem** for conformational dynamics of inhibitor binding
- **coralForge** for structural variant impact prediction

No single product answers the question. The answer emerges from their
composition — orchestrated by the science itself.

## The Spore Cycle

The collaboration completes a biological cycle:

```
Ecosystem validates published science (springs)
  → Products compose validated computation
  → Collaborator produces new science
  → New science → new validation targets for springs
  → Springs evolve from external demand
  → Ecosystem is stronger than before
```

## Spring–Literature Connections

| Spring Domain | Published Literature Anchor | Products | Validation Basis |
|---------------|----------------------------|----------|------------------|
| healthSpring / NF | Neurofibromatosis & immunology literature | helixVision + healthSpring + initioChem | Reproduced publications + spring checks |
| initioChem / enzymes | CAZyme conformational FEL literature | initioChem | Published FEL methods |
| blueFish / analytical | PFAS detection & mass spectrometry literature | blueFish | Published spectral pipelines |
| lithoSpore / evolution | Long-term evolution experiment literature | lithoSpore | Published LTEE models |

## Scientific Challenges

Beyond individual spring domains, the ecosystem participates in structured
benchmarks hosted by scientific foundations (Synapse/DREAM challenges):

- **Docker-based submission** maps to pseudoSpore pattern
- **Foundation-sponsored evaluation** builds credibility with funders
- **Domain-expert scoring** validates products under external criteria
- **Challenge results** reveal gaps internal testing never surfaces

This is gen5 validation at population scale — not one collaborator's
question, but the entire field's benchmarks.
