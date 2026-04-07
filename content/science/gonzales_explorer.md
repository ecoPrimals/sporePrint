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

<div id="explorer-tabs" class="explorer-tabs"></div>
<div id="explorer-content" class="explorer-content">
  <noscript>
    <p>This interactive explorer requires JavaScript to render charts.
    The underlying data is available as JSON in the
    <a href="https://github.com/ecoPrimals/sporePrint">sporePrint repository</a>.</p>
  </noscript>
  <p class="loading">Loading explorer…</p>
</div>

<style>
.explorer-tabs {
  display: flex;
  flex-wrap: wrap;
  gap: 0.5rem;
  margin: 1.5rem 0;
  padding: 0;
}

.tab-btn {
  background: var(--bg-secondary, #24283b);
  color: var(--fg-secondary, #a9b1d6);
  border: 1px solid var(--border, #414868);
  border-radius: 6px;
  padding: 0.5rem 1rem;
  cursor: pointer;
  font-size: 0.9rem;
  font-family: inherit;
  transition: background 0.15s, color 0.15s, border-color 0.15s;
}

.tab-btn:hover {
  background: var(--bg-hover, #363b54);
  color: var(--fg-primary, #c0caf5);
}

.tab-btn.active {
  background: #7aa2f7;
  color: #1a1b26;
  border-color: #7aa2f7;
  font-weight: 600;
}

.explorer-content {
  min-height: 400px;
}

.node-section {
  margin: 1.5rem 0;
  padding: 1rem 1.25rem;
  background: var(--bg-secondary, #24283b);
  border: 1px solid var(--border, #414868);
  border-radius: 8px;
}

.node-section h3 {
  color: #7aa2f7;
  margin: 0 0 1rem 0;
  font-size: 1.1rem;
}

.chart {
  width: 100%;
  min-height: 350px;
  margin: 0.75rem 0;
}

.gauge-row {
  display: flex;
  flex-wrap: wrap;
  gap: 1rem;
}

.gauge {
  flex: 1;
  min-width: 200px;
  min-height: 250px;
}

.ranges {
  margin-top: 0.75rem;
  display: flex;
  flex-wrap: wrap;
  gap: 0.5rem;
}

.range-badge {
  display: inline-block;
  padding: 0.25rem 0.75rem;
  border-radius: 1rem;
  font-size: 0.8rem;
  font-weight: 500;
}

.range-normal  { background: rgba(158,206,106,0.15); color: #9ece6a; border: 1px solid rgba(158,206,106,0.3); }
.range-warning { background: rgba(224,175,104,0.15); color: #e0af68; border: 1px solid rgba(224,175,104,0.3); }
.range-critical{ background: rgba(247,118,142,0.15); color: #f7768e; border: 1px solid rgba(247,118,142,0.3); }

.scenario-desc {
  color: var(--fg-secondary, #a9b1d6);
  font-style: italic;
  margin-bottom: 1rem;
}

.loading {
  color: var(--fg-muted, #565f89);
  text-align: center;
  padding: 3rem;
}

.error {
  color: #f7768e;
  text-align: center;
  padding: 2rem;
}

.scenario-header {
  display: flex;
  align-items: center;
  gap: 1rem;
  flex-wrap: wrap;
}

.source-badge {
  display: inline-flex;
  align-items: center;
  padding: 0.2rem 0.6rem;
  border-radius: 1rem;
  font-size: 0.75rem;
  font-weight: 600;
  letter-spacing: 0.03em;
  white-space: nowrap;
}
.source-badge.live {
  background: rgba(158,206,106,0.15);
  color: #9ece6a;
  border: 1px solid rgba(158,206,106,0.3);
}
.source-badge.cached {
  background: rgba(122,162,247,0.12);
  color: #7aa2f7;
  border: 1px solid rgba(122,162,247,0.3);
}

.api-status {
  display: inline-block;
  padding: 0.35rem 0.75rem;
  border-radius: 1rem;
  font-size: 0.8rem;
  font-weight: 600;
  margin-left: auto;
}
.api-status.live {
  background: rgba(158,206,106,0.12);
  color: #9ece6a;
}

.slider-panel {
  background: var(--bg-secondary, #24283b);
  border: 1px solid var(--border, #414868);
  border-radius: 8px;
  padding: 0.75rem 1rem;
  margin: 0.75rem 0 0 0;
}
.slider-title {
  color: #7aa2f7;
  font-size: 0.85rem;
  font-weight: 600;
  margin-bottom: 0.5rem;
}
.slider-row {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  margin: 0.35rem 0;
}
.slider-label {
  color: var(--fg-secondary, #a9b1d6);
  font-size: 0.8rem;
  min-width: 9rem;
}
.slider-input {
  flex: 1;
  accent-color: #7aa2f7;
}
.slider-value {
  color: #c0caf5;
  font-size: 0.8rem;
  min-width: 3.5rem;
  text-align: right;
  font-variant-numeric: tabular-nums;
}

.provenance-panel {
  background: var(--bg-secondary, #24283b);
  border: 1px solid var(--border, #414868);
  border-radius: 8px;
  margin: 0.75rem 0;
  padding: 0;
}
.provenance-panel summary {
  cursor: pointer;
  padding: 0.5rem 1rem;
  color: var(--fg-secondary, #a9b1d6);
  font-size: 0.85rem;
  display: flex;
  align-items: center;
  gap: 0.5rem;
}
.provenance-content {
  padding: 0.5rem 1rem 0.75rem;
  border-top: 1px solid var(--border, #414868);
}
.tier-badge {
  background: rgba(187,154,247,0.15);
  color: #bb9af7;
  border: 1px solid rgba(187,154,247,0.3);
  padding: 0.1rem 0.5rem;
  border-radius: 0.8rem;
  font-size: 0.7rem;
  font-weight: 600;
}
.prov-row {
  display: flex;
  gap: 0.75rem;
  padding: 0.15rem 0;
  font-size: 0.8rem;
}
.prov-label {
  color: var(--fg-muted, #565f89);
  min-width: 6rem;
}
.prov-value {
  color: var(--fg-primary, #c0caf5);
  font-family: 'JetBrains Mono', 'Fira Code', monospace;
  word-break: break-all;
}
.verify-btn {
  background: rgba(122,162,247,0.15);
  color: #7aa2f7;
  border: 1px solid rgba(122,162,247,0.3);
  border-radius: 6px;
  padding: 0.35rem 0.75rem;
  cursor: pointer;
  font-size: 0.8rem;
  margin-top: 0.5rem;
}
.verify-btn:hover {
  background: rgba(122,162,247,0.25);
}

.lineage-popup {
  display: none;
  position: fixed;
  bottom: 1rem;
  right: 1rem;
  width: 340px;
  max-height: 70vh;
  overflow-y: auto;
  background: #1a1b26;
  border: 1px solid #414868;
  border-radius: 10px;
  box-shadow: 0 8px 32px rgba(0,0,0,0.4);
  z-index: 1000;
  padding: 0;
}
.lineage-header {
  background: rgba(122,162,247,0.12);
  color: #7aa2f7;
  font-weight: 600;
  padding: 0.6rem 1rem;
  border-radius: 10px 10px 0 0;
  font-size: 0.9rem;
}
.lineage-body {
  padding: 0.75rem 1rem;
}
.lineage-row {
  padding: 0.2rem 0;
  font-size: 0.8rem;
  color: #c0caf5;
}
.lineage-row code {
  font-size: 0.75rem;
  background: rgba(122,162,247,0.1);
  padding: 0.1rem 0.3rem;
  border-radius: 3px;
}
.lineage-divider {
  border: none;
  border-top: 1px solid #292e42;
  margin: 0.5rem 0;
}
.lineage-verify {
  color: #7aa2f7;
  font-size: 0.8rem;
  text-decoration: none;
}
.lineage-verify:hover {
  text-decoration: underline;
}
.lineage-close {
  position: absolute;
  top: 0.4rem;
  right: 0.5rem;
  background: none;
  border: none;
  color: #565f89;
  cursor: pointer;
  font-size: 1rem;
  padding: 0.2rem;
}
.lineage-close:hover {
  color: #f7768e;
}

.svg-render {
  width: 100%;
  min-height: 300px;
  display: flex;
  justify-content: center;
  margin: 0.75rem 0;
}
.svg-render svg {
  max-width: 100%;
  height: auto;
}

.renderer-toggle {
  margin-left: 0.5rem;
  font-size: 0.8rem;
  border-style: dashed;
}

.validation-tab {
  border-color: #bb9af7 !important;
  color: #bb9af7 !important;
}
.validation-tab.active {
  background: #bb9af7 !important;
  color: #1a1b26 !important;
}

.validation-stage {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  padding: 0.6rem 0.75rem;
  margin: 0.35rem 0;
  background: rgba(255,255,255,0.02);
  border-radius: 6px;
  border: 1px solid var(--border, #414868);
}
.validation-icon {
  font-size: 1.3rem;
  flex-shrink: 0;
}
.validation-info {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 0.15rem;
}
.validation-info strong {
  color: var(--fg-primary, #c0caf5);
  font-size: 0.85rem;
}
.validation-info .prov-value {
  font-size: 0.75rem;
}
.validation-status {
  padding: 0.15rem 0.6rem;
  border-radius: 1rem;
  font-size: 0.7rem;
  font-weight: 600;
  white-space: nowrap;
}
.validation-status.verified {
  background: rgba(158,206,106,0.15);
  color: #9ece6a;
}
.validation-status.live {
  background: rgba(122,162,247,0.15);
  color: #7aa2f7;
}
.validation-status.pending, .validation-status.pending_hash {
  background: rgba(224,175,104,0.15);
  color: #e0af68;
}
.validation-status.offline, .validation-status.unknown {
  background: rgba(86,95,137,0.2);
  color: #565f89;
}

.access-tier-badge {
  display: inline-block;
  padding: 0.25rem 0.65rem;
  border-radius: 1rem;
  font-size: 0.75rem;
  font-weight: 600;
  border: 1px solid;
  margin-left: 0.5rem;
}

.system-status-panel {
  background: var(--bg-secondary, #24283b);
  border: 1px solid var(--border, #414868);
  border-radius: 8px;
  padding: 0.75rem 1rem;
  margin: 0.75rem 0;
}
.status-header {
  color: #7aa2f7;
  font-size: 0.85rem;
  font-weight: 600;
  margin-bottom: 0.5rem;
}
.status-ok { color: #9ece6a; }
.status-degraded { color: #e0af68; }
.status-connected { color: #9ece6a; }
.status-unreachable { color: #f7768e; }

.reproduce-btn {
  width: 100%;
  background: rgba(158,206,106,0.12);
  color: #9ece6a;
  border: 1px solid rgba(158,206,106,0.3);
  border-radius: 6px;
  padding: 0.5rem 0.75rem;
  cursor: pointer;
  font-size: 0.8rem;
  font-weight: 600;
  margin-top: 0.25rem;
  transition: background 0.15s;
}
.reproduce-btn:hover {
  background: rgba(158,206,106,0.22);
}

.reproduce-panel {
  display: none;
  position: fixed;
  bottom: 1rem;
  left: 1rem;
  width: 400px;
  max-height: 80vh;
  overflow-y: auto;
  background: #1a1b26;
  border: 1px solid #414868;
  border-radius: 10px;
  box-shadow: 0 8px 32px rgba(0,0,0,0.4);
  z-index: 1001;
  padding: 0;
}

.reproduce-step {
  display: flex;
  gap: 0.75rem;
  padding: 0.5rem 0;
  align-items: flex-start;
}
.reproduce-step-num {
  width: 1.6rem;
  height: 1.6rem;
  border-radius: 50%;
  background: rgba(122,162,247,0.15);
  color: #7aa2f7;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 0.75rem;
  font-weight: 700;
  flex-shrink: 0;
}
.reproduce-step-body {
  display: flex;
  flex-direction: column;
  gap: 0.25rem;
  flex: 1;
  min-width: 0;
}
.reproduce-step-body strong {
  color: #c0caf5;
  font-size: 0.8rem;
}
.reproduce-cmd {
  display: block;
  background: rgba(0,0,0,0.3);
  color: #9ece6a;
  padding: 0.35rem 0.5rem;
  border-radius: 4px;
  font-size: 0.7rem;
  font-family: 'JetBrains Mono', 'Fira Code', monospace;
  word-break: break-all;
  white-space: pre-wrap;
}

.reproduce-nft {
  padding: 0.5rem 0;
}
.reproduce-nft strong {
  color: #bb9af7;
  font-size: 0.8rem;
}
.reproduce-nft-note {
  color: #565f89;
  font-size: 0.7rem;
  font-style: italic;
  margin-top: 0.25rem;
}

.reproduce-links {
  padding: 0.5rem 0 0.25rem;
}
.reproduce-links a {
  font-size: 0.8rem;
}

.trace-source {
  padding: 0.25rem 0 0.5rem;
  border-left: 2px solid #7aa2f7;
  padding-left: 0.75rem;
  margin-bottom: 0.5rem;
}
.doi-link {
  color: #7aa2f7;
  text-decoration: none;
  font-size: 0.8rem;
}
.doi-link:hover {
  text-decoration: underline;
}
.chain-steps {
  display: flex;
  flex-direction: column;
  gap: 0.15rem;
  margin-top: 0.4rem;
}
.chain-step {
  font-size: 0.7rem;
  padding: 0.15rem 0.4rem;
  border-radius: 3px;
  display: inline-block;
}
.chain-done {
  background: rgba(158,206,106,0.15);
  color: #9ece6a;
}
.chain-done::before {
  content: '\2713 ';
}
.chain-pending {
  background: rgba(224,175,104,0.12);
  color: #e0af68;
}
.chain-pending::before {
  content: '\25CB ';
}
</style>

<script src="https://cdn.plot.ly/plotly-2.35.2.min.js" charset="utf-8"></script>
<script src="/gonzales/js/explorer.js"></script>

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
