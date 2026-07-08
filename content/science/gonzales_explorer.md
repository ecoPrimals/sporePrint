+++
title = "Gonzales Interactive Explorer"
description = "Interactive exploration of canine atopic dermatitis and Anderson localization in immunological signaling. All data computed by guideStone (29/29 PASS)."
date = 2026-04-06
weight = 100

[extra]
domain = "Human Health"
paper_number = "E"

[taxonomies]
springs = ["wetspring"]
primals = ["barracuda", "petaltongue"]
+++

Interactive exploration of the Gonzales dermatitis science
([Paper 12](@/science/12_immunological_anderson.md)) and the
Anderson localization framework applied to immunological signaling.

Data is computed by the `wetspring-gonzales-guideStone` binary from
validated Rust math (**29/29 checks, exit 0**). When the HPC is online,
data streams live from wetSpring via the science facade — parameters are
adjustable and every result carries cryptographic provenance. Click any
data point to trace its lineage.

> **Note:** The legacy Plotly-based Gonzales explorer (inline CSS/JS and
> `static/gonzales/` assets) has been removed from this page. Interactive
> visualization is evolving into petalTongue's `visualization.render.graph`
> and `viz.serve` IPC methods. The science domains, provenance, and
> reproducibility documentation below remain current.

---

## Science Domains

| Domain | Source | IPC Method |
|--------|--------|------------|
| IC50 Dose-Response | Gonzales 2014 | `science.gonzales.dose_response` |
| PK Decay | Fleck/Gonzales 2021 | `science.gonzales.pk_decay` |
| Tissue Geometry | Paper 12 (Exp273-279) | `science.gonzales.tissue_lattice` |
| Hormesis | Paper 14 | `science.anderson.hormesis` |
| Cross-Species | Paper 12 extension | `science.anderson.cross_species` |

## Provenance

### Static (guideStone)

```bash
cargo run --release --features json \
  --bin wetspring_gonzales_guidestone \
  -- --export-scenarios data/
```

{{ entity(name="guidestone") }} validation: **29/29 checks passed** (exit 0).
Source: {{ entity(name="wetspring") }}.

### Live (science facade)

When the HPC is online, `lab.primals.eco` serves live results from the
`wetspring_science_facade` Axum binary through a Dark Forest-gated
cloudflared tunnel. Every response carries:

- **Tier 1** — guideStone version, wetSpring commit, BLAKE3 content hash
- **Tier 2** — rhizoCrypt DAG session, loamSpine ledger commit, sweetGrass braid ID
- **Tier 3** — W3C PROV-O export, Merkle inclusion proof, verify link

Click any data point to open the lineage panel and trace the full chain.

## Validation Chain

The **Validation** tab shows the full paper-to-code-to-primal proof:

1. **Published Paper** — Peer-reviewed source with DOI
2. **Python Baseline** — healthSpring experiment reproducing published values
3. **Rust Validation** — `validate_gonzales_ic50_s79` (35 checks)
4. **guideStone** — Domain scenario validation (29 checks)
5. **NUCLEUS Composition** — Live computation with provenance trio wrapping

Every value is traceable from the original paper table through Python, Rust,
and the full primal ecosystem. The system is a living artifact: shortcomings
found here inform wateringHole and primalSpring evolution.

## Access Tiers

| Tier | Token | Capabilities |
|------|-------|-------------|
| Public | None | Static JSON fallback, health endpoint |
| Visitor | Dark Forest | Read-only live science, Tier 1 provenance |
| Collaborator | Elevated + vault consent | Parameter exploration, Tier 2/3 provenance, data export |
| Owner | Family seed holder | Full system access, vault admin, data ingestion |

## Reproducibility

Every data point on this page carries a **reproduction envelope** — click
any point, then press "Reproduce this result" to see exact commands:

1. **Fetch** the pinned primal versions via `plasmidBin/fetch.sh`
2. **Deploy** the NUCLEUS graph with `biomeos deploy`
3. **Recompute** the same IPC call with identical parameters
4. **Verify** the BLAKE3 content hash matches the original

The reproduction manifest (`reproduction_manifest.toml`) pins every primal
version needed by the science pipeline. Combined with the deploy graph
(`wetspring_science_nucleus.toml`), anyone with a commodity machine can
recreate the full computation environment.

Each result is also structured as a **Novel Ferment Transcript (NFT) vertex**
— a DAG node in the gAIa knowledge commons. The vertex records the method,
parameters, result hash, and agent chain (primal, paper authors, hardware).
The transcript's value comes from its verifiable history and attribution chain,
not artificial scarcity.

## Contributing (Ionic Bonding)

External researchers can interact with the science pipeline via **ionic bonds**
— contract-scoped, provenance-wrapped data exchanges across trust boundaries:

| Bond Type | Scope | Trust Model |
|-----------|-------|-------------|
| Covalent  | LAN mesh (same family) | GeneticLineage — implicit full trust |
| Ionic     | External via cloudflared | Contractual — capability-scoped, auditable |

To establish an ionic bond:

1. Fetch the system composition: `GET /api/v1/system/composition`
2. Review available capabilities and bonding metadata
3. Negotiate a contract (capability scope, duration, attribution)
4. All interactions are provenance-wrapped and NFT vertex-recorded

**Note:** Ionic contract negotiation is scaffolded but not yet automated —
this is owned by primalSpring Track 4 (`BondingConstraint + BondingPolicy`).
Contact the ecosystem maintainers for manual ionic bond setup.

---

*primals.eco Full NUCLEUS — live data from wetSpring via Dark Forest-gated
cloudflared tunnel, static fallback from guideStone, progressive provenance
from the trio. Tower authenticates, Node computes, Nest stores.
[Architecture](@/architecture/DEPLOYMENT_MODEL.md).*
