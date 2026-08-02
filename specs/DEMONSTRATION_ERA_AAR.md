# sporePrint Demonstration Era — AAR + Dependency Map

**Date**: August 1, 2026 | **Wave**: post-155n | **From**: sporePrint team
**Status**: Phase 1 COMPLETE, Phase 2 content layer COMPLETE, Phase 3 content COMPLETE

---

## WHAT WE DID

| Change | Files | Impact |
|--------|-------|--------|
| Nav triage: Lab \| Science \| Architecture \| Products \| Get Started | templates/base.html, templates/section.html, config.toml | 79 pages out of main nav |
| Foundation flag system (`extra.foundation = true`) | 79 content pages, section.html filter, _content.scss | URLs preserved, SEO intact |
| 7 VALIDATED badges on baseCamp papers | 7 science pages | Papers 10, 14, 21, 24, 7, 17, 28 |
| 6 architecture pages → `maturity = "live"` | SOVEREIGN_CI, MESH_TOPOLOGY, GENERATIONAL_ARC, ECOSYSTEM_ARCHITECTURE, KDERM, neural_api | Live evidence in descriptions |
| Homepage rewrite | templates/index.html, content/_index.md | "NUCLEUS Is Running", live system cards |
| Getting Started section | content/getting-started/_index.md | Stack table, boot order, mesh enrollment |
| Hype cleanup | 20 files | 353x → topology. 3.24 TFLOPS → 2,130 matmul/sec measured. Rust-vs-Python qualified |
| Auto-publish verified | golgi-ext systemd timer | 15-min Forgejo pull → zola build confirmed working |

## WHAT WE NEED FROM OTHER TEAMS

### biomeOS — G22 single-process merge (steps 3-5)

**What we need**: Stable biomeOS process model to document. Currently biomeOS runs
api + neural-api as separate concerns; steps 3-5 merge them. When complete:
- Update architecture pages with single-process model
- Document the unified socket namespace (`/run/membrane/`)
- Update Getting Started boot order if sequence changes

**Blocks**: Getting Started accuracy, architecture page freshness

### petalTongue — G19 Node Atomics rendering

**What we need**: Live rendering pipeline so sporePrint can serve dynamic dashboards.
spore-validate already has the IPC modules (`pt-render`, `pt-viz`, `pt-status`).
When petalTongue can render from primal APIs:
- Gate Status page: live health from biomeOS neuralAPI
- GPU Compute page: live matmul/SVD/FFT from barraCuda
- Provenance Dashboard: live 7/7 chain from Nest Atomic

**Blocks**: Phase 2 live dashboard pages (currently static content only)

### cellMembrane — J12 sub-builder IPC wire

**What we need**: Windows genomeBin builds automated (not manual).
When J12 lands:
- Update Sovereign CI page with Windows sub-builder pipeline
- Update depot binary counts if new targets added

**Blocks**: Depot documentation accuracy

### tideGlass + Nest Atomic — pseudoSpore pipeline

**What we need**: First real pseudoSpore artifact (NF data through CAS pipeline).
When available:
- pseudoSpore gallery page with download link
- Getting Started "grab a pseudoSpore" section

**Blocks**: Phase 3 pseudoSpore download page

### eastGate — southGate NUCLEUS launch

**What we need**: External deployment proof (southGate is off WireGuard).
When bonding validates:
- Getting Started "southGate story" as external on-ramp
- Update Living Systems gate table

**Blocks**: Phase 3 on-ramp narrative

---

## RESOLVED SINCE INITIAL AAR (Aug 1 PM)

### golgi auto-publish — FIXED
Three compounding bugs resolved by sporeGate team:
1. Worktree ownership (`git:git` vs `root:root`)
2. Missing `--force` on `zola build`
3. SSH config pointing at wrong golgi IP

sporePrint now deploys correctly to both inner and outer membrane via
the Forgejo push → timer → zola build → Caddy serve pipeline.

### P2 plaquette divergence — ROOT-CAUSED + VALIDATED
GPU PRNG polyfill bias caused plaquette values to diverge from CPU reference.
`cpu_mom` workaround deployed on strandGate. Production data now flowing.
arXiv Section 3.2 (plaquette measurements) is UNBLOCKED.

### arXiv draft — ALL DATA SECTIONS COMPLETE (Aug 2)
hotSpring team filled all 5 remaining sections with production data:
- **Section 3.2**: Plaquette values at β=2.3 (4⁴, 8⁴). |Δ|/σ < 1 vs CPU. COMPLETE.
- **Section 3.3**: DF64 precision — ~9 significant digits for accumulated observables. COMPLETE.
- **Section 3.4**: AMD RX 6950 XT benchmarks. 190× speedup at 8⁴. Cross-GPU agreement 3.1e-9. COMPLETE.
- **Section 3.5**: Autocorrelation τ_int = 1.63 (4⁴), 3.37 (8⁴). COMPLETE.
- **Section 4.2**: Three-path validation methodology (A/B/C comparison). COMPLETE.

Paper status: COMPLETE. Zero [TODO] markers remaining.
Next: markdown → LaTeX (REVTeX4-2) → arXiv hep-lat submission.

---

## WHAT WE CAN DO NOW (no dependencies)

### 1. spore-validate refresh (metric sync)
Run `spore-validate refresh` against local repos to detect metric drift.
biomeOS is at v4.56 (8,570 tests), squirrel at 7,138. Sync registry.

### 2. Content manifest + certification regeneration
After all these content changes, regenerate:
- `spore-validate provenance --write` (content-manifest.toml)
- `spore-validate certify --emit` (certification manifest)

### 3. llms.txt update
Current llms.txt references old structure. Update to reflect demonstration era.

### 4. Static placeholder pages for future dashboards
Create stub pages at `/lab/gate-status/`, `/lab/gpu-compute-live/`,
`/lab/provenance-dashboard/` with current data. These become dynamic
when petalTongue wires in (G19). Better to have static truth than nothing.

### 5. Foundation section template polish
The `<details>` collapsible in section.html works but could use better
grouping (by domain: architecture, methodology, products, technical).

### 6. Accessibility (EVOLUTION_QUEUE P2)
- Mobile experience testing
- Pa11y integration
- Keyboard-only navigation test

### 7. README + llms.txt sync
Both need to reflect 79 foundation pages, 4 NUCLEUS gates, hype cleanup.

---

## PAGE COUNT AUDIT

| Target (blurb) | Actual | Notes |
|-----------------|--------|-------|
| ~120-150 in main nav | 190 | Lab (132) is core value, can't shrink |
| ~180 in foundation/backstory | 115 (79 foundation + 36 backstory) | Backstory pages don't have foundation flag (they have their own nav section) |
| Total | 313 | Matches. 23 sections. |

The gap vs target is Lab (132 pages). These are notebooks, validation summaries,
and spring evidence — the core scientific output. They stay.

---

*sporePrint team scope: content, templates, spore-validate, static site.
We don't touch primal code. We document what's running.*
