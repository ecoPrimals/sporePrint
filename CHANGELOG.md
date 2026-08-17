# sporePrint Changelog

All notable changes to this whitepaper repository are documented here.
Format: `[version] — date — description`

---

## [3.35.0] — 2026-08-17 — QCD Production Complete + Site Refresh

**32⁴ SU(3) PRODUCTION COMPLETE — 45/45 cross-vendor configs, literature ~0.3%,
cross-GPU Δ=0.19% at β=6.20. Normalization RESOLVED (gauge-group mismatch, not bug).
arXiv 41/42, reviewer send blocked on primals.eco. Lattice capacity 73⁴ dual GPU.
Site-wide staleness sweep: 10 pages updated to Wave 157k reality.**

### Changed

- **pseudoSpore QCD** (`hotspring-qcd-sun.md`) — 32⁴ production data, 45/45 configs,
  cross-vendor 0.19%, lattice capacity 73⁴, normalization resolution, streaming encoder
- **arXiv draft** (`hotspring-qcd-sun-paper.md`) — SU(N) rung table, 32⁴ data,
  resolved known issues (normalization, β-scan, statistics, 16⁴ validation)
- **Audit trail** (`hotspring-qcd-sun-audit.md`) — Phases 9-12: gauge group resolution,
  32⁴ production campaign (57 configs), full silicon activation (45/45), current status
- **pseudoSpore catalog** (`pseudospore/_index.md`) — 3.21→3.3 TB, updated QCD status
- **Data Braids** (`data/_index.md`) — 3.21→3.3 TB, added date
- **GPU Compute Live** — 16⁴/32⁴ benchmarks, streaming encoder, SU(3) production data
- **Living Systems** — full rewrite: 12 gates, 6 NUCLEUS, 4-arch depot, Wave 157k backlog
- **Mesh Topology** — full rewrite: 12 gates, 6 NUCLEUS, graftGate/iosGate/steamGate, Wave 157k
- **Sovereign CI** — Wave 155n→157k, 3/3 builders enmeshed, 4-arch depot, NanoWire retired
- **llms.txt** — 32⁴ complete, arXiv status, lattice capacity, 12 gates × 6 OS families
- **config.toml** — measured_date 2026-08-17

### Metrics

- QCD: **32⁴ production COMPLETE** — 45/45 configs, 0.3% literature, 0.19% cross-vendor
- arXiv: **41/42** — physics done, reviewer send blocked on primals.eco
- Normalization: **RESOLVED** — SU(3) vs SU(2) literature mismatch, not a bug
- Lattice capacity: **73⁴ dual GPU** (121× more sites)
- Streaming encoder: GPU utilization **43%→85-95%**
- Pages refreshed: **10** (QCD + infrastructure + data volumes)

---

## [3.34.0] — 2026-08-16 — Enmeshment + Ingestion (Wave 157k)

**12 GATES ONLINE. 0/0/0. bonsai-bt FORKED — DECIDE layer meta-primal ingesting
(exp125 23/24). rootPulse 6/6 graphs REGISTERED (item #10 CLOSED). Titan V Tier 1
CONFIRMED (4 measurement bugs fixed). graftGate FULL NUCLEUS (Darwin, 16/16 depot).
NanoWire Tier 1 RETIRED. 227 files fossilized (1,513 total). Pipeline + provenance CONVERGED.**

### Changed

- **Homepage** — hero: 12 gates, 0/0/0, bonsai-bt DECIDE layer, rootPulse 6/6, Titan V, graftGate
- **Gate status** — full rewrite: 12-gate fleet table (biomeGate/grapheneGate/iosGate/steamGate),
  bonsai-bt ingestion section (exp125 5 trees, EcoAction architecture), rootPulse 6/6 section,
  Titan V Tier 1 section, depot table (4 arches including aarch64-apple-darwin 16/16),
  NanoWire SSH retirement, active code teams, downstream patterns
- **llms.txt** — 12 gates, 0/0/0, bonsai-bt, rootPulse 6/6, Titan V, depot status
- **config.toml** — gates_online=12, measured_date 2026-08-16
- **All specs** — CONTEXT, EVOLUTION_QUEUE, CONTENT_MAP current at Wave 157k

### Metrics

- Gates: **12 ONLINE** (was 11). graftGate FULL NUCLEUS (Darwin). biomeGate Tower+Node.
- P0 / P1 / P2: **0 / 0 / 0** (was 0/0/1)
- bonsai-bt: FORKED, exp125 23/24, Phase 0 ingesting
- rootPulse: 6/6 graphs REGISTERED (item #10 CLOSED)
- biomeGate: Titan V Tier 1 CONFIRMED (4 bugs fixed)
- Depot: x86_64-musl 13/13, aarch64-musl 15/15, aarch64-darwin 16/16, Windows 0/13 STALE
- NanoWire: Tier 1 RETIRED (3/3 builders enmeshed)
- Fossilized: 227 files (1,513 total records)
- Pages: 338 across 25 sections

---

## [3.33.0] — 2026-08-11 — Pandemic Responds: G72 Tier 1 Complete (Wave 157i)

**G72 TIER 1 COMPLETE: 9/9 teams responded, ~114 crates shed fleet-wide. toadStool
tokio 118→65 files. P2 braid.verify CLOSED (0/0/1). Gossip injection 6/16 LIVE
(barraCuda 19 events). hotSpring pseudoSpore E2E pipeline shipped (pure Rust).
graftGate M4 arrived — GLACIAL→ACTIVE.**

### Changed

- **Homepage** — hero-desc updated: G72 Tier 1 complete, gossip 6/16, pseudoSpore E2E, graftGate M4
- **Gate status** — full rewrite: G72 Tier 1 table (9/9 teams with deps shed / impact),
  gossip injection 6/16 (barraCuda 19 events, esotericWebb 2, songBird 1), science
  pipeline section (hotSpring pseudoSpore E2E), graftGate section (M4 Mac Mini,
  aarch64-apple-darwin), immediate post-pandemic work table, updated primal dashboard
- **llms.txt** — G72 Tier 1 complete, gossip 6/16, pseudoSpore E2E, graftGate
- **config.toml** — measured_date updated to 2026-08-11
- **All specs** — CONTEXT, EVOLUTION_QUEUE, CONTENT_MAP current at Wave 157i

### Metrics

- Pages: 338 across 25 sections
- P0: ZERO | P1: ZERO | P2: 1 (petalTongue port). ~~braid.verify~~ **CLOSED**.
- Tests: ~150,000+ across 16 primals + 9 springs
- G72 Tier 1: 9/9 teams DONE. ~114 crates shed fleet-wide.
- Gossip injection: 6/16 primals LIVE (was 3/16)
- Gossip mesh: 4-gate (sporeGate, eastGate, strandGate, westGate)
- Science: hotSpring pseudoSpore E2E shipped (pure Rust)
- graftGate: M4 Mac Mini arrived, GLACIAL→ACTIVE

---

## [3.32.0] — 2026-08-10 — Stadial Shift + 4-Gate Gossip Mesh (Wave 157g)

**STADIAL → INTERSTADIAL. 4-gate gossip mesh LIVE. G72 Dependency Pandemic (664
Cargo.toml audited, 3-tier excision plan). Primals shed vestigial deps. sourDough CI
shipped. biome.yaml manifest CONVERGED. ~150K+ tests. Zero P0. Zero P1.**

### Changed

- **Homepage** — hero-desc updated: stadial shift, 4-gate gossip mesh, G72, ~150K+ tests
- **Gate status** — full rewrite: 4-gate gossip mesh table, G72 dep pandemic section, gossip
  injection 3/16 primals, three-pillar architecture, stadial shift narrative, updated primal
  dashboard (coralReef 3,963, cellMembrane 1,353, sweetGrass braid.verify)
- **llms.txt** — stadial shift posture, gossip mesh, G72, ~150K+ tests
- **config.toml** — test totals updated (~150K+), measured_date 2026-08-10-PM
- **All specs** — CONTEXT, EVOLUTION_QUEUE, CONTENT_MAP current at Wave 157g

### Metrics

- Pages: 338 across 25 sections
- P0: ZERO | P1: ZERO | P2: 2
- Tests: ~150,000+ (was 116,930+)
- Gossip: 4-gate mesh (sporeGate, eastGate, strandGate, westGate)
- Gossip injection: 3/16 primals LIVE
- WASM: 38/48 (79%)
- G72: 664 Cargo.toml audited

---

## [3.31.0] — 2026-08-10 — spore-validate Deep Debt Evolution

**Systematic deep debt resolution: hardcoded ecosystem knowledge evolved to
capability-based runtime discovery. Forgejo-first forge default. Tower probes
externally overridable. Stale test totals and hero-sub data fixed.**

### Changed

- **discovery.rs** — removed `WELL_KNOWN_PEERS` hardcoded 7-primal array; peer
  discovery is now pure runtime via socket dir scan + `SPOREPRINT_EXTRA_PEERS` env var
- **discovery.rs** — added `env_var_for_slug()` public helper; all scattered
  `"PETALTONGUE_SOCKET"` / `"NESTGATE_SOCKET"` strings in dispatch.rs, commands.rs,
  petaltongue.rs replaced with derived env vars
- **cas_push.rs** — replaced hardcoded `"sporePrint"` source identity with
  `discovery::SELF.primal_id`
- **paths.rs** — `DEFAULT_FORGE_URL` changed from `github.com` to `git.primals.eco`
  (Forgejo sovereign primary); GitHub access requires explicit env override
- **tower.rs** — added `--probes <file.toml>` CLI flag for external probe definitions;
  embedded `default_tower_probes.toml` documented as G69 baseline
- **nucleus.rs** — cross-repo test paths gated behind `#[ignore]` (monorepo layout)
- **templates/index.html** — hero-sub `3.21 TB` replaced with Tera template variable
  `config.extra.totals.data_volume`; added `data_volume` key to config.toml
- **config.toml** — fixed stale test totals (primal 105,242, spring 11,688, total 116,930+)
- **fetch.rs** — tests updated to use `DEFAULT_FORGE_URL` constant instead of
  hardcoded GitHub URLs

### Metrics

- Unit tests: 251 passed, 2 ignored (monorepo), 0 failed
- Integration tests: 29 passed, 6 ignored (server), 0 failed
- Zero new dependencies added

---

## [3.30.0] — 2026-08-09 — Depot Unified + G69 Lineage Spec (Wave 157d)

**ZERO P0. Depot unified + pruned (60 binaries, 4 arches). G69 depot lineage spec.
Mesh-native build (blueGate primary). Neural API unblocked (13,910 caps). barraCuda
Silicon Fold ABSORBED. coralReef 18/18 IPC. cellMembrane depot.prune. 145K+ tests.**

### Changed

- **Homepage** — 145K+ tests, zero P0, depot unified, mesh-native build, Neural API unblocked
- **Gate status** — full rewrite: infrastructure phase complete table, 6/6 fleet with
  updated caps/status (sporeGate 15/15, strandGate 13/13), depot unified + pruned
  section (G69 lineage, 4-arch table), mesh-native build system, Neural API unblocked
  (13,910 caps), self-audit summary (13 primals), Phases 4+5 upgraded to UNBLOCKED
- **Primal dashboard** — updated test counts (barraCuda 5,025, coralReef 3,702,
  cellMembrane 1,347), Silicon Fold ABSORBED, P0-C IN DEPOT
- **llms.txt** — zero P0, depot unified, G69, mesh-native, 13,910 caps
- **All specs** — EVOLUTION_QUEUE, CONTEXT, CONTENT_MAP current

### Metrics

- Pages: 338 across 25 sections
- P0: ZERO (was 3 → 1 → 0)
- Tests: ~145,000+ (was ~135,000+)
- Caps: 13,910 (was 1,987 pre-vertebrate)
- Depot: 60 binaries, 4 arches, BLAKE3SUMS
- arXiv: 41/42

---

## [3.29.0] — 2026-08-09 — Vertebrate Evolution Complete, Depot Rebuild (Wave 157a)

**12/16 primals self-audited — zero phantom APIs. P0-B RESOLVED (nestGate content.ingest
was shipped, stale depot). P0-A code-fixed (bearDog health guard 766951004). songBird
CanonicalTransport shipped. swarmVine 39→124 tests. sourDough rpc-surface audit tool.
sporeGate rebuilding depot — gates pull from golgi, no self-builds.**

### Changed

- **Homepage** — 12/16 self-audited, zero phantom APIs, depot rebuild in progress
- **Gate status** — full rewrite: P0-B resolved, P0-A code-fixed/depot-stale,
  deployment discipline section (postPrimordial), 12-row self-audit table,
  depot rebuild table (7 primals with key commits), updated primal dashboard
  with self-audit annotations and updated test counts
- **llms.txt** — 12/16 audited, P0 status, CanonicalTransport, rpc-surface tool
- **All specs** — EVOLUTION_QUEUE, CONTEXT, CONTENT_MAP current

### Metrics

- Pages: 338 across 25 sections
- Self-audits: 12/16 complete (zero phantom methods)
- P0: 1 code-open (biomeOS FD), 2 depot-stale (bearDog, nestGate)
- swarmVine: 39→124 tests (82% coverage)
- arXiv: 41/42

---

## [3.28.0] — 2026-08-09 — Vertebrate Evolution, 3 P0s OPEN (Wave 157a)

**Vertebrate evolution phase. westGate 7-session retrospective exposed 3 P0s: bearDog
sign stub (spine commits unsigned), nestGate API mismatch (content.ingest doesn't exist),
biomeOS FD leak (capability.call unusable). Mesh code-complete, production-blocked.**

### Changed

- **Homepage** — 16 primals, 3.3 TB, vertebrate evolution + 3 P0s status
- **Gate status** — full rewrite: P0 section (P0-A/B/C with evidence + fixes),
  vertebrate evolution table (6 primals with evolution tasks), mesh status
  (code-complete, production-blocked), updated westGate to 3.3 TB / 989K files,
  Phase 4+5 upgraded to PRODUCTION-BLOCKED, toadStool S371
- **Primal health dashboard** — P0 annotations on bearDog, nestGate, biomeOS.
  songBird 24 MB FIXED. loamSpine commits deferred.
- **llms.txt** — 16 primals, 3 P0s, mesh status, vertebrate evolution
- **All specs** — EVOLUTION_QUEUE, CONTEXT, CONTENT_MAP current

### Metrics

- Pages: 338 across 25 sections
- P0: 3 OPEN (was ZERO)
- Primals: 16 (N-series 90/91)
- westGate: 989K files, 153 datasets, 3.3 TB
- Mesh: code-complete, production-blocked
- arXiv: 41/42

---

## [3.27.0] — 2026-08-08 — 6/6 Gates Redeployed, NG-05 CLOSED, SU(N) Relabel (Wave 157a)

**6/6 NUCLEUS gates redeployed. NG-05 CLOSED (westGate CAS federation: 26 capabilities,
2.5 TB). QCD pseudoSpore PACKAGED. SU(2)→SU(N) relabel DONE. cellMembrane plasmid.fetch
FIXED. toadStool S370: WASM compute.**

### Changed

- **SU(2)→SU(N) relabel** — 3 pages renamed (`hotspring-qcd-su2` → `hotspring-qcd-sun`),
  10 files updated. Titles, descriptions, scope reflect SU(N) for N=2→8.
  SU(3) COMPLETE, SU(4) running. Rung 2 marked COMPLETE in paper.
- **Homepage** — 6/6 redeployed, NG-05 CLOSED, QCD pseudoSpore PACKAGED
- **Gate status** — full rewrite: 6/6 fleet table with RSS/capabilities, NG-05
  section (26 capabilities, 2.5 TB CAS), toadStool S370 WASM, cellMembrane
  plasmid.fetch FIXED, Phase 5 upgraded to UNBLOCKED
- **hotSpring QCD** — pseudoSpore PACKAGED (was NOT YET), arXiv blockers 3/6 resolved,
  relabel marked DONE
- **llms.txt** — 6/6, NG-05, QCD packaged, sovereign deploy path
- **All specs** — EVOLUTION_QUEUE, CONTEXT, CONTENT_MAP current

### Metrics

- Pages: 338 across 25 sections
- Gates redeployed: 6/6
- NG-05: CLOSED (26 capabilities)
- QCD pseudoSpore: PACKAGED
- arXiv: 41/42 (validate.sh + freeze/sign remain)

---

## [3.26.1] — 2026-08-08 — Gate Redeploy + Trust Surfaces LIVE (Wave 157a)

**3/6 NUCLEUS gates redeployed. Trust surfaces LIVE on nestgate.io. strandGate DIVERGED.
SU(3) campaign COMPLETE, SU(4) running. Cascade auto-push operational.**

### Changed

- **Homepage** — 3/6 redeployed, trust surfaces LIVE, pseudoSpore bundles served
- **Gate status** — redeploy table (3 DONE, 1 DIVERGED, 2 PENDING), strandGate
  divergence section, trust surface routes table, cascade auto-push, blueGate Windows
  issues (3 P3/P4), SSH discipline per-gate compliance
- **hotSpring QCD** — SU(3) COMPLETE (36 configs), SU(4) running, pseudoSpore routes
  LIVE, arXiv blockers 2/5 partially resolved
- **llms.txt** — 3/6 redeployed, trust surfaces, strandGate diverged
- **All specs** — EVOLUTION_QUEUE, CONTEXT, CONTENT_MAP at redeploy status
- **wateringHole** — overwatch handoff updated, sporePrint blurb rewritten

---

## [3.26.0] — 2026-08-08 — G68 Convergence + SSH Key Discipline (Wave 157a)

**G68 COMPLETE — 16/16 prod-clean, 205→0 violations. SSH key discipline enforced.
Depot current on golgi (Musl 17/17, Windows 15/15). arXiv 41/42 science-complete.**

### Changed

- **Homepage** — G68 COMPLETE, SSH discipline, arXiv 41/42, depot current
- **Gate status** — full rewrite: G68 audit table, depot table, SSH discipline section,
  fleet shows redeploy status, primal dashboard updated (toadStool S369, cellMembrane 1,327,
  sweetGrass capability.call SHIPPED, nestgate.io 10/12 sections)
- **hotSpring QCD** — arXiv 41/42 science-complete, trust surface blocks documented,
  SU(2)→SU(N) relabel note, pseudoSpore bundle status (NOT YET PACKAGED)
- **llms.txt** — G68, SSH discipline, arXiv 41/42, depot metrics
- **EVOLUTION_QUEUE** — Wave 157a current state
- **CONTEXT** — Wave 157a header, G68 convergence

### Metrics

- Pages: 338 across 25 sections
- Tests (sporePrint): 283 (spore-validate)
- Tests (ecosystem): 135,000+
- G68: 16/16 prod-clean
- Depot: Musl 17/17, Windows 15/15

---

## [3.25.0] — 2026-08-05 — Data Flow Activation Era (Wave 156d)

**G18 signal dispatch LIVE. Phase 1 cell boot SUCCEEDED. footPrint Phase 2 DEPLOYED.
3.21 TB / 153 datasets. Convoy at 145/s. 16⁴ dual-GPU data COMPLETE. All 6 NUCLEUS gates v4.57+.**

### Added

- **Phase execution status** — gate-status page tracks 5 phases with current status
- **Fleet table** — all 6 NUCLEUS gates with NUCLEUS version and status
- **Convoy provenance section** — 145/s (460× total), convergence sweep data
- **Live sites table** — sporePrint, footPrint, nestgate.io, esotericWebb with status
- **Overwatch handoff** (`OVERWATCH_WAVE156D_DATA_FLOW_HANDOFF.md`) — gate team assignments

### Changed

- **Homepage** — 3.21 TB, G18 dispatch LIVE, Phase 1 SUCCEEDED, convoy 145/s
- **Gate status** — full rewrite: phase execution, fleet table, ironGate 12.7 TB CAS
- **Data index + pseudoSpore** — 3.21 TB / 153 datasets (was 519 GB / 130)
- **hotSpring QCD** — 16⁴ dual-GPU COMPLETE, 42-item reviewer rubric, config cache
- **llms.txt** — Data Flow Activation Era, corrected counts, live sites
- **EVOLUTION_QUEUE** — Wave 156d current state, demonstration era collapsed
- **CONTEXT** — Data Flow Activation Era header
- **Primal dashboard** — rhizoCrypt 1,791 (G63), sweetGrass 1,636, tideGlass 214,
  nestGate content.query SHIPPED, cellMembrane harvest scheduler, esotericWebb V31b 484
- **README** — 3.21 TB, 135K+, remaining milestones updated

### Metrics

- Pages: 338 across 25 sections
- Tests (sporePrint): 283 (spore-validate)
- Tests (ecosystem): 135,000+
- NUCLEUS gates: 11 online, 6 at v4.57+

---

## [3.24.0] — 2026-08-03 — Data Braids Visibility, Demonstration Era (Waves 155n → 155q/156b)

**Data Braids catalog with inline provenance. Ecosystem metrics refresh.
11 gates, 519 GB data, 121K+ tests. G19 proven on ironGate.**

### Added

- **Data Braids inline braids** — W3C PROV-O JSON-LD attestation on every
  dataset across 13 domain pages (22 braids total, up from 1)
- **Transplant page** (`/data/transplant/`) — pseudoSpore/lithoSpore paths
  for PIs carrying data + provenance to their own hardware
- **"Data" in main nav** — between pseudoSpore and Lab
- **3 stub pages upgraded** — cancer-genomics, disease-ontology, genomic-reference
  now full domain pages with frontmatter, "What's Possible", and braids
- **pseudoSpore section** — lead nav item with data catalog, QCD paper,
  verification guide, computation audit trail
- **arXiv preprint page** — Rung 1 SU(2) Lattice Gauge Theory paper
  (PREPRINT UNDER REFINEMENT — plaquette normalization blocker)
- **Gate status page** — 11-gate role taxonomy with NUCLEUS health dashboard

### Changed

- **Homepage** — 11 gates, 519 GB, 121K+ tests, G19 milestone
- **Data catalog** — synced to 519 GB / 130+ datasets / 17+ domains
- **pseudoSpore catalog** — fixed stale 38.2 GB → 519 GB references
- **llms.txt** — added Data Braids + transplant entries
- **config.toml** — `total_tests = 121000+`, `gates_online = 11`,
  `data_volume_gb = 519`, `data_datasets = 130`

### Metrics

- Pages: 338 across 25 sections
- Tests (sporePrint): 283 (spore-validate)
- Tests (ecosystem): 121,000+
- NUCLEUS gates: 11 online

---

## [3.23.0] — 2026-07-14 — Branch-Agnostic Fetch, WCAG Figure/Table, Constant Sweep (Wave 138b)

**Fetch evolution: configurable branch per source. Accessibility: figure/figcaption
for all visualizations, table scope/caption for screen readers. Constants centralized.**

### Changed

- **`ForgeArchiveBackend` branch-agnostic** — `Source.branch` field (default: `main`)
  threaded through `VcsBackend` trait. Both `GitBackend` (via `--branch`) and
  `ForgeArchiveBackend` (archive URL) now respect per-source branch configuration.
  Eliminates hardcoded `main` in archive URL construction.
- **WCAG: `viz_embed` → `<figure>/<figcaption>`** — shortcode output now uses
  semantic HTML5 `<figure>` with optional `caption` parameter. All 6 viz pages
  updated with descriptive captions for screen readers.
- **WCAG: table `scope`/`<caption>`** — capability table in `taxonomy_single.html`
  now has `scope="col"` on headers, `scope="row"` on category cells, and a
  visually-hidden `<caption>` for screen readers.
- **`.sr-only` CSS utility** — standard screen-reader-only class added to
  `_layout.scss` for hidden-but-accessible content.
- **Notebook constants centralized** — `NOTEBOOK_DEFAULT_WEIGHT` (50) and
  `NOTEBOOK_DEFAULT_DOMAIN` ("Lab") extracted to `paths.rs` from inline literals.

### Metrics

- Tests: 287 (249 unit + 29 integration + 3 refresh_write + 6 parity ignored)
- Pages: 270 across 18 sections
- Clippy: 0 warnings (pedantic + nursery)
- `validate --strict`: 0 errors, 0 warnings

## [3.22.0] — 2026-07-14 — Rust Evolution: Expect→Result, IPC Constants, Test Dedup (Wave 138b)

**Crate-level deep debt: eliminate production panic paths, centralize magic
numbers, deduplicate test infrastructure, remove duplicate tests.**

### Changed

- **`tower.rs` `.expect()` → `Result`** — embedded TOML parse now returns
  `Result<ProbeList, String>` through `probe_tower_status`, surfacing errors
  via `Error::Config` instead of panicking at runtime.
- **`ipc::JSONRPC_METHOD_NOT_FOUND`** — centralized `-32601` constant. Replaced
  inline magic numbers in `tower.rs` and `petaltongue.rs` with the shared constant.
- **`ipc::mock::MockStream`** — shared NDJSON mock stream extracted from
  duplicate implementations in `ipc.rs` and `petaltongue.rs` tests. Single
  `pub(crate)` module used by both test suites.
- **Duplicate tests removed** — `today_utc_is_valid_format` test deduplicated
  (was in `time.rs`, `refresh.rs`, and `notebook.rs`; canonical copy retained
  in `time.rs`).
- **`biomeos-validation-summary.md`** — added missing `weight` field to
  resolve Zola build warning in weight-sorted `lab` section.

### Metrics

- Tests: 287 (249 unit + 29 integration + 3 refresh_write + 6 parity ignored)
- Pages: 270 across 18 sections
- Clippy: 0 warnings (pedantic + nursery)
- `validate --strict`: 0 errors, 0 warnings

---

## [3.21.0] — 2026-07-13 — Deep Debt Sweep: Dead Code, Alt Text, TBD Cleanup (Wave 137b)

**Systematic deep debt elimination: dead CSS/code removal, WCAG alt text
for notebook charts, TBD content resolution, deprecated script retirement.**

### Changed

- **Dead CSS removed** — `.badge-status-unknown` (unused), `.landing-footer`
  (dead; landing uses `base.html` footer) pruned from `_badges.scss` and
  `_landing.scss`.
- **`css_class()` dead code removed** — `MaturityLevel::css_class()` method
  and its test deleted from `model.rs` (zero callers in production; trivially
  restorable when petalTongue needs it). Sole `#[allow(dead_code)]` eliminated.
- **Deprecated `validate_parity.sh` deleted** — superseded by Rust parity
  tests (`cargo test --test parity -- --ignored`).
- **WCAG: 12 notebook images** — all "No description has been provided"
  placeholder alt texts replaced with descriptive chart summaries across
  5 groundSpring notebook files.
- **TBD markers resolved** — RTX 5090 fp64:fp32 ratio set to 1:64 in
  sovereign compute hardware page; fieldGate/biomeGate status clarified
  in mesh topology; lattice QCD product naming markers softened.

### Metrics

- Tests: 289 (251 unit + 29 integration + 3 refresh_write + 6 parity ignored)
- Pages: 270 across 18 sections
- Clippy: 0 warnings (pedantic + nursery)
- `validate --strict`: 0 errors, 0 warnings

---

## [3.20.0] — 2026-07-13 — Agent Content Parity: Detect and Prevent Silent Substitution (Wave 137b)

**External AI agent reported receiving `llms.txt` for all URLs instead of
requested pages. Root cause: agent fetch tool following `<link rel="alternate"
type="text/plain">`. Infrastructure confirmed clean — no UA-sniffing.**

### Added

- **`validate_agent_parity.sh`** — dogfood test fetching 11 URLs with both
  browser and bot UAs, asserting title + canonical parity. Catches any
  future UA-based content substitution.
- **`llms.txt` self-identification** — canonical URL + explicit warning header
  so agents can detect when they've received the overview instead of a
  specific page. Makes wrongness detectable.
- **Slug provenance fix** — `70_papers_one_stack.md` slug overridden to
  `175-papers-one-stack` to match its "175+ Papers" title.
- **AAR** — `AGENT_PARITY_AAR_137b.md` documenting root cause, investigation,
  and lessons learned.

---

## [3.19.0] — 2026-07-12 — Deep Debt: Scaffold Maturity, Path Scrub, Metric Sync (Wave 137b)

**Resolve validator warnings, remove PII-adjacent hardcoded paths from rendered
notebooks, add the `scaffold` maturity level, and sync all page counts and
metrics across the codebase.**

### Added

- **`Scaffold` maturity level** — new enum variant in `MaturityLevel`, Tera
  shortcode, and SCSS badges. Clears 11 validator warnings from outreach pages.

### Fixed

- **Notebook path leak** — 11 rendered notebook `.md` files contained hardcoded
  `/home/eastgate/Development/ecoPrimals/springs/airSpring` paths. Replaced with
  `os.environ.get('AIRSPRING_ROOT', '../airSpring')`. Zero hardcoded local paths
  remain in content.
- **`sample.md` test fixture** excluded from published site via `draft = true`.
- **Page count drift** — config.toml (`259` → `270`), README, specs/CONTEXT,
  specs/CONTENT_MAP, specs/RUST_TOOLING_VISION, llms.txt, site-index all synced
  to 270 published pages / 18 sections.
- **`last_push` date** — config.toml updated from `2026-06-19` to `2026-07-12`.
- **Test count** — corrected from 284 to 290 (includes 6 parity ignored).
- **Certification manifest** + **entity graph** regenerated from current state.
- **64 entity metrics** refreshed from upstream repos via `spore-validate refresh`.

---

## [3.18.0] — 2026-07-12 — AI Progressive Richness: Cross-Domain Topology (Wave 137b)

**Give AI agents typed cross-domain navigation so philosophy, thesis, science,
and code docs reinforce each other instead of acting as isolated gravity wells.
28 key pages carry companion metadata linking narrative ↔ formal ↔ experimental
registers.**

### Added

- **Per-page JSON-LD** on all 272 pages — `Article` or `ScholarlyArticle` with
  typed author (attsi for philosophy/story, ecoPrimals for thesis/science/technical),
  license, and `isPartOf` linking back to the site.
- **`relatedLink` in JSON-LD** on 28 companion-seeded pages — typed cross-domain
  links discoverable by any agent that parses structured data.
- **Companion panel** (`<nav class="companions">`) — visible HTML block at the
  bottom of pages with companions. Shows relation type badge, linked title, and
  one-line description. Serves both humans and AI agents.
- **Companion front matter convention** — `[[extra.companions]]` TOML arrays with
  `url`, `title`, `relation`, and `label` fields. Six relation types: `formal_version`,
  `narrative_version`, `pairs_with`, `validates`/`validated_by`, `extends`/`extended_by`,
  `evidence_for`.
- **Content Topology section in `llms.txt`** — describes the four-register
  reinforcement model and richness levels 0-4 so AI agents understand the
  cross-domain structure in one fetch.
- **Companion panel CSS** in `_pages.scss` — responsive layout with relation badges
  and companion labels.

### Pages Seeded

- Philosophy: 9 essays (the_human_search, the_city_of_omelas, i_own_nothing,
  the_knowledge_numeric, discovery_is_local, sovereign_science, the_orthogonal_synthesis,
  the_mobility_edge, the_temptation_of_kingdoms)
- Story: 3 essays (i_dont_know_rust, the_sovereign_lab, 70_papers_one_stack)
- Thesis: 7 chapters (01, 03, 05, 06, 08, 10, 14)
- Science: 5 papers (01, 02, 03, 07, 10)
- Architecture: 1 (SOVEREIGN_DEPLOYMENT)
- Methodology: 3 (K_NOME_PROGRAMMING, CONSTRAINED_EVOLUTION_FORMAL, SCYBORG_LICENSING)

### Richness Level Architecture

- Level 0 (Billboard): robots.txt, sitemap.xml, meta tags — unchanged
- Level 1 (Overview): llms.txt + /site-index/ — unchanged
- Level 2 (Catalog): Section indexes with hasPart JSON-LD — unchanged
- Level 3 (Topology): **NEW** — Per-page JSON-LD with typed relatedLink companions
- Level 4 (Reinforcement): **NEW** — Visible companion panels + cross-domain summaries

---

## [3.17.0] — 2026-07-12 — AI Accessibility: Fetch Budget Optimization (Wave 137a)

**Minimize fetches needed for AI agents to comprehend the full site. Solves
agent-side fetch quota exhaustion (observed: Claude web_fetch hitting per-domain
session limits after ~50 requests across 289 pages).**

### Added

- **`/llms.txt`** — structured plain-text site overview for AI agents. One fetch
  gives: site description, section map with page counts, key concept glossary,
  identity model, live metrics, and machine-readable endpoint URLs.
- **`/site-index/`** — auto-generated page listing every page on the site with
  title, description, and URL. Template-driven (`site_index.html`), updates
  automatically with every build. One fetch = complete catalog.
- **`<link rel="alternate" type="text/plain">` for `/llms.txt`** in `base.html`
  `<head>` — HTML metadata discovery for agents that parse link elements.
- **`robots.txt`** updated with comments pointing to `/llms.txt` and `/site-index/`.

### Context

An external Claude agent reviewing the site exhausted its per-domain fetch quota
after ~50 pages, leaving 240+ pages unseen. The agent correctly diagnosed this as
its own tool's limitation, not server-side rate limiting. Our response: treat
agent-side constraints as accessibility constraints. A screen reader with a limited
buffer is not the screen reader's fault to fix — it's ours. Two new endpoints let
any fetch-constrained agent get comprehensive site understanding in 2 requests
instead of 289.

---

## [3.16.0] — 2026-07-11 — AI Accessibility: Table-to-List Evolution (Wave 137a)

**Convert all 23 navigational tables across 9 section indexes to ordered/unordered lists.
Add `hasPart` JSON-LD to all sections with child pages. Resolves AI fetch-tool accessibility bug.**

### Changed

- **9 section `_index.md` files**: All navigational tables (tables whose primary content was links
  to child pages) converted to ordered or unordered lists. Markdown tables are stripped of links
  by many AI fetch-to-text tools; lists survive extraction. Data-only tables (outreach Two Voices,
  Community) preserved.
  - `architecture/_index.md` — 16 entries
  - `audience/_index.md` — 5 entries
  - `lab/_index.md` — 21 entries (6 springs + 15 notebooks)
  - `methodology/_index.md` — 9 entries
  - `outreach/_index.md` — 11 entries (3 phase-1 + 5 phase-2 + 3 partnerships)
  - `science/_index.md` — 28 entries across 6 domains
  - `story/_index.md` — 6 entries (3 stories + 3 philosophy pairings)
  - `technical/_index.md` — 6 entries
  - `thesis/_index.md` — 20 entries (16 chapters + 4 back matter)
- **`templates/section.html`**: Generic `CollectionPage` JSON-LD `hasPart` fallback added for all
  sections not already covered by philosophy/thesis/story-specific blocks.
- **`templates/science_section.html`**: `CollectionPage` JSON-LD with `ScholarlyArticle` `hasPart`
  added (33 articles).

### Context

External AI agent (Claude via `web_fetch`) reported essay tables rendering header-only on
`/philosophy/` and `/story/`. Root cause: fetch-to-text tools strip links from markdown-rendered
`<table>` elements. Tables converted to `<ol>`/`<ul>` in Wave 136b for philosophy/story; this
commit extends the fix to all 9 section indexes. JSON-LD `hasPart` ensures machine-readable
child-page discovery regardless of HTML rendering.

### Metrics

- tables with links remaining: 0 (across all section indexes)
- sections with `hasPart` JSON-LD: 12 (all sections with child pages)
- sitemap entries: 314

---

## [3.15.0] — 2026-07-11 — Cast Safety, Identity Model, License Enforcement, Doc Sync (Wave 136b)

**Evolve unsafe casts to idiomatic Rust. Codify identity model. Three-layer license enforcement. Root doc sync.**

### Changed

- **`depot.rs`**: `#[allow(cast_sign_loss)]` → `u64::try_from()` with typed error for negative TOML sizes.
- **`cas_push.rs`**: `PushResult.errors` evolved from `u64` to `usize` (natural count type).
- **`commands.rs`**: `cast_possible_truncation` allow eliminated (errors now `usize`).
- **Production `#[allow]` count**: 13 → 11.
- **Root docs synced**: README (289 pages, 17 sections), CONTEXT.md (11,012L), RUST_TOOLING_VISION.md
  (11 justified allows, 680L max), CONTENT_MAP.md (all section counts updated from 205→289),
  EVOLUTION_QUEUE.md (`prefers-contrast`/`forced-colors` marked complete).
- **wateringHole handoff**: `WAVE136B_DEEP_DEBT_BLURB.md` filed with quality gates + upstream gaps.

### Metrics

- tests: 284 (252 unit + 29 integration + 3 refresh_write, 6 parity ignored)
- modules: 34 (11,012L)
- content pages: 289
- clippy: 0 warnings
- production #[allow]: 11

---

## [3.14.0] — 2026-07-10 — Deep Debt: Module Split, A11y Contrast, Test Dedup (Wave 136b)

**Structural hardening: module splits, contrast media queries, test harness deduplication.**

### Added

- **`nucleus_probe.rs`** — extracted live socket probing (health + riboCipher acceptance) from
  `nucleus.rs` (811L → 670L + 142L). Smart split by domain: types/parse/validate stay,
  probing moves out.
- **`prefers-contrast: more`** CSS media query — high-contrast tokens for light and dark modes.
  Addresses last a11y validation warning (was WCAG AAA stretch goal, now passes).
- **`forced-colors: active`** CSS support — Windows High Contrast mode border fallbacks for
  badges, entity refs, and certification lines.
- **5 new `DiagnosticCollector` tests**: `into_result_ok`, `into_result_err`, `diagnostics_slice`,
  `extend`, `default` — removes `#[allow(dead_code)]` from test-only collector.
- **`tests/common/mod.rs`** — shared test harness helpers (`sporeprint_root`, `binary_path`,
  `ensure_built`) deduplicated from integration, parity, and refresh_write test files.

### Changed

- **Removed 4 stale `#[allow(dead_code)]`** from NUCLEUS struct fields (`role`, `parallel_after`,
  `node_id`, `peers`) — these are consumed by `structural_warnings()` since v3.13.0.
- **Removed `#[allow(dead_code)]`** from `DiagnosticCollector` impl — all methods now exercised.
- **Module count**: 33 → 34. **Test count**: 279 → 284 (252 unit + 29 integration + 3 refresh_write).
- **Max file size**: 620L → 670L (nucleus.rs, down from 811L pre-split).

---

## [3.13.0] — 2026-07-10 — NUCLEUS Structural Validation + Schema-Behavior Alignment (Wave 136a)

**Resolved NUCLEUS schema-behavior drift. All parsed fields now validated.**

### Added

- **NUCLEUS structural validation**: `NucleusProfile::structural_warnings()` validates internal
  consistency without live socket probing — launch order references, parallel_after coherence,
  mesh federation config, health critical references, min_healthy bounds, known deployment roles.
- **`ValidationResult::structural_warnings`**: Structural diagnostics now surfaced alongside
  live probe results in `nucleus` subcommand output.
- **7 new tests**: `structural_warnings_clean_profile`, `_unknown_role`, `_launch_order_references_undeclared`,
  `_parallel_after_not_in_order`, `_federation_without_node_id`, `_critical_references_undeclared`,
  `_min_healthy_exceeds_total`.

### Changed

- **Removed 4 `#[allow(dead_code)]` annotations** from `ProfileMeta::role`, `LaunchConfig::parallel_after`,
  `MeshConfig::node_id`, `MeshConfig::peers` — all fields now consumed by structural validation.
- **Known roles**: `canary`, `production`, `development`, `relay`, `compute`.

### Metrics

- tests: 279 (247 unit + 29 integration + 3 refresh_write, 6 parity ignored)
- clippy: 0 warnings
- `#[allow(dead_code)]` in production: 1 remaining (`MaturityLevel::css_class` — P2 petalTongue target)

## [3.12.0] — 2026-07-10 — Accessibility Validation Suite + CI Integration (Wave 134f)

**Automated accessibility validation wired into CI pipeline.**

### Added

- **`scripts/validate_a11y.sh`**: 7-phase accessibility validation suite covering HTML5 structural
  validity, ARIA landmarks, heading hierarchy, image alt text, skip link, meta accessibility
  (lang, viewport, reduced-motion), and search ARIA combobox. Filters Zola syntax-highlighting
  CSS false positives. Targets WCAG 2.2 AAA.
- **CI integration**: `deploy.yml` now runs `validate_a11y.sh` in the `check` job before build.

### Fixed

- **Canonical URL on 404 page**: Zola was entity-escaping `://` in `config.base_url` fallback.
  Added `| safe` filter to `base.html` canonical link.

### Metrics

- a11y suite: 13 pass, 0 fail, 1 warning (prefers-contrast — AAA stretch goal)
- HTML5 structural validation: 0 errors across 301 pages

## [3.11.0] — 2026-07-10 — Deep Debt: Constant Centralization + Dependency Inversion (Wave 134f)

**Deduplicated riboCipher, centralized env vars, corrected dependency direction, removed dead code.**

### Changed

- **riboCipher constants consolidated in `ipc.rs`**: `RIBOCIPHER_CLEAR`, `RIBOCIPHER_PROTO_NDJSON`,
  `RIBOCIPHER_MITO_CLEAR` now defined once. `cas_push.rs` and `nucleus.rs` import from `ipc`.
  `ribocipher_enabled()` and `send_ribocipher_signal()` moved to `ipc.rs`.
- **`ReadWrite` trait moved from `cas_push` to `ipc`**: Corrects dependency inversion — `ipc`
  defines the transport trait, `cas_push` re-exports for backward compatibility.
- **10 env var names centralized in `paths.rs`**: `ENV_FORGE_URL`, `ENV_RIBOCIPHER`,
  `ENV_REFRESH_PAT`, `ENV_NOTEBOOK_OUTPUT`, `ENV_TRANSPORT_ENDPOINT`, `ENV_BIOMEOS_SOCKET_DIR`,
  `ENV_BIOMEOS_SYSTEMD_DIR`, `ENV_XDG_RUNTIME`, `ENV_PLASMIDBIN_CHECKSUMS`.
  All scattered `std::env::var("STRING")` calls now reference these constants.
- **Default forge URL** (`https://github.com`) moved to `paths::DEFAULT_FORGE_URL`.
- **Tower fallback removed**: `fallback_tower_probes()` deleted — `default_tower_probes.toml`
  is the sole source of truth, parsed with `expect()` since it's compile-time embedded.
- **`VizResult::format` dead field removed**: Populated but never read. `VizFormat` enum retained
  (used as parameter to `viz()`).
- **`MaturityLevel::css_class` annotation clarified**: References EVOLUTION_QUEUE P2 petalTongue target.

### Metrics

- clippy: 0 warnings (pedantic + nursery)
- tests: 240 pass, 6 ignored (parity — requires live petalTongue)
- fmt: clean

## [3.10.0] — 2026-07-09 — Deep Debt Sprint: Module Splits + Transplants + Metric Evolution (Wave 134e)

**Module architecture overhaul, 7 content transplants, hardcoded metrics evolved, structured error handling.**

### Added

- **7 content transplants** from gen3: barracuda compute gaps (364L), hotSpring Phase B evidence
  (207L), Murillo reproduction plan (385L), neuromorphic benchmark (327L), primal composition
  methodology (355L), heterogeneous fabric economics (348L), scyBorg exception protocol (294L).
- **3 new Rust modules**: `cli.rs` (Clap types), `dispatch.rs` (command routing),
  `commands_validate.rs`, `commands_provenance.rs`, `commands_discover.rs` (focused command handlers).
- **`default_tower_probes.toml`**: Tower probe configuration embedded via `include_str!`,
  parsed with `LazyLock`. `DefaultProbe` struct with `slug` + `methods`.
- **Thesis snapshot banner**: `_index.md` notes gen3 metrics are historical, links to Evidence Snapshot.
- **Front matter completed**: department resolved (CMSE), acknowledgments written, dedication written.
- **3 thesis TODOs resolved**: ch16 LTEE sequencing, NUCLEUS scaling, NK landscape — replaced
  brackets with substantive prose.

### Changed

- **`commands.rs` split** (785L → 4 modules): validate (189L), provenance (119L), discover (95L),
  dispatch layer (422L). Smart refactor, not mechanical split.
- **`main.rs` split** (687L → 3 modules): cli.rs (217L), dispatch.rs (410L), main.rs (66L).
- **Max file size**: 699L → 616L (content.rs). 33 modules total (was 28).
- **Tower probes**: `DEFAULT_TOWER_PROBES` constant → data-driven TOML with `LazyLock` parse.
  Fallback hardcoded minimum if parsing fails (impossible for embedded data).
- **CAS push errors**: `PushFileOutcome::Error` gains `String` payload; callers print structured messages.
- **Fetch errors**: `fetch_sources` returns `Result<Vec<FetchOutcome>, Error>` (was silent `Vec::new()`).
- **VIZ_OUTPUT_DIR**: centralized in `paths.rs` (was hardcoded in dispatch.rs).
- **Regex init**: all 5 static `LazyLock<Regex>` now use `.expect("static regex")` consistently.
- **Dead code cleanup**: `DiagnosticCollector` scoped to `#[cfg(test)]`; `discover_and_connect()` removed;
  `VizResult.format` annotated.
- **Clippy**: `pub(crate)` in private modules → `pub`; `.or()` → `.or_else()`.
- **5 broken Spring Catalog URLs** fixed in lab validation summaries.
- **8 pages**: hardcoded `27,000+ tests`, `14 primals`, `7 springs` → `total_stat` shortcodes.
- **`sharing_the_pen.md`**: 4 gen3/ path references → Zola `@/` internal links.
- **`the_knowledge_numeric.md`**: description updated (removed stale "14 primals and 7 springs").

### Metrics

- Content pages: 252 → 259
- Rust modules: 28 → 33
- Tests: 272 (unchanged — pure structural refactor)
- Max file size: 616L (content.rs)
- Total Rust LOC: 10,718
- Clippy: zero warnings (pedantic + nursery)

---

## [3.9.0] — 2026-07-09 — Thesis Scaffold + lithoSpore Product + Philosophy Subtabs (Wave 134c)

**Thesis section scaffolded, lithoSpore product page, philosophy sidebar grouped, cross-references linked.**

### Added

- **Thesis section** (`content/thesis/`): 16-chapter PhD dissertation scaffold mirroring
  gen3/thesis/ structure. Each chapter has frontmatter, abstract, maturity badge, and
  cross-links to existing sporePrint pages. Section index with thesis statement,
  six-part TOC, audience reading paths, and lineage tracing.
- **lithoSpore product page** (`content/products/lithoSpore.md`): spore taxonomy table
  (coldSpore→liveSpore→pseudoSpore→lithoSpore), three operating modes, three-tier
  validation, pseudoSpore lifecycle (emit/ingest/promote), deployment vision.
- **Philosophy sidebar subtabs**: atlasHugged essays grouped under The Stories (01–03),
  The Framework (04–08), The Synthesis (09–12), and Reference (bibliography).
- **Section page grouping**: philosophy section card listing also groups by subsection.
- **44 cross-references linked**: Inter-essay references (Document NN, Chapter N)
  converted to proper Zola internal links across 6 essays.

### Changed

- **Products `_index.md`**: lithoSpore entry expanded from one-line bullet to description
  with link to product page.
- **Sidebar tree**: thesis section added with Foundations/Theory/System/Validation/
  Analysis/Synthesis/Back Matter grouping.
- **Methodology `_index.md`**: added thesis section to reading table.
- **Philosophy `_index.md`**: added thesis cross-link in Discussion section.
- **pseudoSpore gallery `_index.md`**: added cross-link to lithoSpore product page.
- **Content pages**: 245 → 249 (18 thesis files + 1 product page).

---

## [3.8.0] — 2026-07-08 — Philosophy Complete + Content Enrichment (Wave 134b)

**All 12 atlasHugged essays live, bibliography page, cross-links, number sync.**

### Added

- **6 philosophy essays** transplanted: The City of Omelas (01), The Orthogonal
  Synthesis (02), The New City (03), The Loaves and the Fishes (05), The Many
  Rooms (09), The Love Letter (11). Philosophy section now has the complete
  narrative foundation beneath the existing conceptual essays.
- **Bibliography page**: `content/philosophy/bibliography.md` — full academic
  citations from CITATIONS.md (sacred texts, philosophy, science, literature).
- **Story ↔ Philosophy cross-links**: "Read More" sections in all 3 story essays
  linking to paired philosophy essays.
- **Philosophy ↔ Story cross-links**: Story section index maps each builder
  narrative to its philosophical counterpart.

### Changed

- **Philosophy `_index.md`**: Complete 12-essay table with sequence numbering,
  reading order guidance, bibliography link.
- **Philosophy essay weights**: Updated to match atlasHugged numbering (1–12).
- **Landing page "Try It"**: Replaced `wetSpring` (broken cold-clone due to path
  deps) with `groundSpring` (works cold from fresh clone).
- **Number sync**: Story essays now use `{{ total_stat() }}` shortcodes for
  validation checks and primal counts. Hardcoded "12,510" → "20,695+", "14" → "15".
- **Content pages**: 238 → 245 (6 essays + 1 bibliography).

## [3.7.0] — 2026-07-08 — Phase 1+2 Idiomatic Rust + petalTongue Integration (Wave 134)

**Codebase evolution sprint — shared abstractions, function decomposition, build-time petalTongue wiring.**

### Added

- **Shared content walker** (`paths.rs`): `walk_markdown_files` and `walk_content_files`
  iterators eliminate 4 duplicate `WalkDir` patterns across content, links, provenance, certify.
- **IPC connect helper** (`ipc.rs`): `connect_uds` consolidates 3 duplicate UDS connection
  setups from nucleus and tower modules.
- **`DiagnosticCollector`** (`error.rs`): typed accumulator with `error()`, `warning()`,
  `promote_warnings()`, `into_result()` — bridge for gradual `Vec<Diagnostic>` migration.
- **`MaturityLevel` enum** (`model.rs`): 6 typed levels (Implemented, Reproduced, Certified,
  Architectural, Planned, Unaudited) with `css_class()`, `label()`, `from_str_loose()`.
  Build-time validation via `validate_maturity_levels` in `--check` mode.
- **`build-viz` subcommand**: scans content for `viz_embed` shortcodes, calls petalTongue
  IPC to generate SVGs at build time, writes to `static/viz/`. Graceful fallback when
  petalTongue is offline.
- **`scan_viz_embeds`** (`commands.rs`): regex scanner for viz_embed shortcode names.
- **12 new tests**: DiagnosticCollector (2), MaturityLevel roundtrip/css/display/unknown/
  case-insensitive (5), maturity validation (2), viz scanner (3).

### Changed

- **`#[must_use]`** added to ~15 pure functions across error, model, paths, ipc, discovery,
  content, links modules.
- **`Cow<str>` evolution**: `normalize_key` returns `Cow<'_, str>` (zero-alloc fast path);
  `systemd_socket_dir` returns `Cow<'static, str>`.
- **Function decomposition**: `commands::validate` split into `validate_registry` +
  `validate_content`; `http::request_raw` into `parse_url` + `read_response` +
  `HttpResponse`; `cas_push::push_single_file` into `encode_file_payload` + RPC send.
- **`tower.rs`**: `match` → `let...else` for clippy pedantic compliance.
- **`petaltongue.rs`**: module docs corrected (removed stale `content.render` reference).
- **Root docs refreshed**: README, CONTEXT, EVOLUTION_QUEUE, RUST_TOOLING_VISION updated
  to 272 tests, 239 pages, 6 shortcodes, Wave 134 metrics.

### Removed

- **`section_count.html`** shortcode — unused, deleted.
- **`gonzales_explorer.md`** dead code: ~550 lines of inline CSS + Plotly JS referencing
  nonexistent `static/gonzales/` removed. Scientific content preserved with petalTongue
  evolution note.

## [3.6.0] — 2026-07-08 — Content Transplant (atlasHugged + Story + Methodology)

**Wave 133d content transplant — fills the largest content gaps on primals.eco.**

### Added

- **Philosophy section**: 6 atlasHugged essays transplanted from whitePaper/gen3:
  The Human Search, The Temptation of Kingdoms, The Mobility Edge, Discovery Is
  Local, I Own Nothing, The Knowledge-Numeric. Philosophy section now has real
  content instead of "Coming" promises.
- **Story section**: New `content/story/` with 3 builder narrative essays:
  I Don't Know Rust, The Sovereign Lab, 70 Papers One Stack. Nav bar and sidebar
  updated with Story link.
- **Methodology depth**: Sharing the Pen (`content/methodology/sharing_the_pen.md`)
  transplanted from whitePaper/gen4/knome — K-NOME methodology sharing argument.

### Changed

- **Philosophy `_index.md`**: "Coming: atlasHugged Essays" replaced with linked
  table of 6 real essays.
- **Nav bar**: Story link added between Lab and divider.
- **Sidebar tree**: Story and Philosophy sections now expand to show child pages.
- **Integration test fix**: `cas_push_requires_manifest_or_generate` assertion
  updated to match lowercase error message from transport unification refactor.
- Content page count: 228 → 238.

### Removed

- "attsi" references stripped from all public-facing story essays per transplant
  boundary rules.

---

## [3.5.0] — 2026-07-07 — Transport Unification + Catalog Metric Evolution

**Unified transport resolution, centralized timeouts, deduplicated discovery,
catalog metric evolution.**

### Changed

- **Unified transport resolution**: `petalTongue` commands (`pt-render`, `pt-viz`)
  now honor `TRANSPORT_ENDPOINT` env var, matching NestGate's injection pattern.
  New `discovery::resolve_primal_endpoint()` replaces per-primal `discover_socket()`
  wrappers (CLI → TRANSPORT_ENDPOINT → socket discovery).
- **Centralized timeout constants**: `PROBE_TIMEOUT` (3s), `TRANSPORT_CONNECT_TIMEOUT`
  (15s), `TRANSPORT_IO_TIMEOUT` (30s) moved to `paths.rs` — eliminates 4 duplicated
  constant definitions across `nucleus.rs`, `tower.rs`, `cas_push.rs`, `http.rs`.
- **Catalog metric evolution**: Removed 11 hardcoded `**Tests**:` lines from
  PRIMAL_CATALOG.md — `entity_metrics` shortcode renders live registry values.
  Remaining narrative test counts use `entity_stat` shortcodes.
- **Import consolidation**: Mid-file `use` statements in `fetch.rs`, `refresh.rs`,
  `notebook.rs` moved to top of file (idiomatic Rust module layout).

### Removed

- `cas_push::discover_socket()` — replaced by `discovery::resolve_primal_endpoint()`
- `petaltongue::discover_socket()` — replaced by `discovery::resolve_primal_endpoint()`

### Metrics

- 260 tests (228 unit + 29 integration + 3 refresh) — up from 258
- 28 modules, 10,112 lines
- All files under 800 lines (max: commands.rs at 699)
- Zero clippy warnings (pedantic + nursery)
- Zero `unwrap()` / `expect()` in production code
- 227 content pages, 190 internal links verified

---

## [3.4.0] — 2026-07-07 — Content Enrichment: Forensic Consistency Sprint

**Evidence Snapshot, maturity badges, metric unification, claim calibration,
accessibility improvements, contact page evolution.**

### Added

- **Evidence Snapshot page** (`architecture/EVIDENCE_SNAPSHOT.md`): Single canonical
  source of truth for all ecosystem metrics, definitions, and measurement methodology.
  Every metric uses `total_stat` shortcodes — always current.
- **Maturity badge shortcode** (`maturity.html`): 6 levels — Implemented, Reproduced,
  Certified, Architecture-ready, Planned, Unaudited. Used across products and science.
- **Config aliases**: `total_primals`, `total_springs`, `content_pages` added to
  `[extra.totals]` — fixes broken `total_primals` shortcode references.
- **Historical snapshot banners**: All 12 March 2026 docs (audience/, technical/,
  methodology/) now carry blockquote banners linking to Evidence Snapshot with
  current metrics via shortcodes.

### Changed

- **Product pages calibrated**: helixVision, lattice QCD, blueFish, esotericWebb,
  initioChem — all now carry maturity badges and calibrated claims. helixVision
  description updated: "AlphaFold-quality" → target, with primitive-level parity
  demonstrated and full-pipeline benchmarks pending.
- **Metric unification**: Replaced hardcoded numbers with `total_stat` shortcodes
  in lab/_index, _index, science/_index, ECOSYSTEM_INVENTORY, SPRING_CATALOG,
  SOVEREIGN_PRIOR_ART_CATALOG, KNOME_EVOLUTION, glossary, contact, sitemap,
  architecture/_index.
- **Clone commands fixed**: reproduce.md now uses HTTPS URLs first
  (`https://github.com/...`) instead of SSH (`git@github.com:...`).
- **Contact page softened**: Organization section rewritten — institution-friendly
  while maintaining professional boundaries. Metrics now via shortcodes.
- **Sitemap tree collapsed**: `<details open>` → `<details>` — content appears
  first for screen readers, SEO, and text-mode readers.
- **OpenGraph metadata**: `og:site_name` updated from "sporePrint — ecoPrimals"
  to "ecoPrimals — Sovereign Scientific Computing" for better discoverability.
- **Spring/domain count fixes**: "8 scientific domains" → "6 research domains"
  in science/_index description (matches actual section count).
  "7 springs" → "8 springs" in COMPOSITION_PIPELINE, CROSS_SPRING_EVIDENCE_MAP.

### Fixed

- Broken `total_primals` shortcode: config had `primal_count` but content used
  `total_primals` — added alias.
- 12+ pages with conflicting LOC counts (3.2M vs 3.46M), test counts (27K vs
  107K vs 113K), validation checks (12K vs 15K vs 20K), WGSL counts (628 vs 806
  vs 952), and spring counts (7 vs 8) — all now either unified via shortcodes or
  marked as historical snapshots.

---

## [3.3.0] — 2026-07-06 — Deep Debt Sprint + Sovereign AAR

**Smart refactoring, profile-driven probes, env-overridable socket dirs,
mock isolation, path dedup, content pages, static SVG diagrams, sovereign
deployment AAR.**

### Added

- **`nucleus_display.rs`**: Display/reporting extracted from `nucleus.rs` —
  `print_result`, `format_probe_info`, `count_by_contract`, etc. (385L new module)
- **Profile-driven Tower probes**: `PrimalEntry.probe_methods` in NUCLEUS TOML
  profiles overrides the default hardcoded probe table. `probe_tower_status()`
  now accepts `Option<&NucleusProfile>`.
- **`BIOMEOS_SYSTEMD_SOCKET_DIR` env override**: `discovery::systemd_socket_dir()`
  checks env before falling back to `/run/membrane`.
- **Content pages**: contact, living systems, sovereign CI, compute-access rewrite.
- **Static SVG diagrams**: `gate-mesh.svg`, `ci-pipeline.svg`.
- **`viz_embed` shortcode**: static SVG fallback rendering (graceful degradation).
- **4 new unit tests**: `build_probe_targets` (3), `systemd_socket_dir` (1).

### Changed

- **`nucleus.rs`**: 930L → 565L (display concern extracted).
- **`tower.rs`**: `TOWER_PROBES` → `DEFAULT_TOWER_PROBES` (fallback only).
- **`fetch.rs`**: `MockBackend` moved into `mod tests` (was module-scope `#[cfg(test)]`).
- **`main.rs`**: `"static/graph/entity-graph.json"` → `paths::ENTITY_GRAPH_JSON`.
- **`certify.rs`**: `"content"` → `crate::paths::CONTENT_DIR`.

### Sovereign AAR

- **5 divergences identified** in `SPOREPRINT_SOVEREIGN_DEPLOY_AAR_133a.md`:
  GitHub Pages primary, dual-push, no NUCLEUS on VPS, cascade doesn't rebuild
  Zola, deploy.yml still load-bearing.
- **VPS NUCLEUS deployment handoff** filed for golgi (4 binaries, 70MB, fits 10GB VPS).

### Metrics

- **258 total tests** (226 unit + 29 integration + 3 refresh_write)
- **28 modules**, 10,109 lines — all under 800L
- **226 content pages**

---

## [3.2.1] — 2026-07-05 — Coverage Sprint

**34 new tests across 5 modules — main, nucleus, commands, petaltongue, content.
All modules now have unit tests. Mock-stream IPC testing for petalTongue client.**

### Added

- **`main.rs` unit tests (7)**: `discover_springs_root` (3 paths), `load_entity_graph_for_render`
  (valid/missing/invalid JSON), `dispatch_standalone` (None paths).
- **`nucleus.rs` format tests (9)**: `format_probe_info` (full/partial/none),
  `format_probe_error` (message/no-health-method/none), `count_by_contract`,
  `has_ribo_result`, `ValidationResult::passed`.
- **`commands.rs` drift tests (5)**: `drift_pct` (positive/negative/zero/new/large).
- **`petaltongue.rs` mock-stream IPC tests (7)**: `health_check`, `render_graph`
  (success + error), `viz` (success + error), `probe_method` (success/method-not-found/other-error).
  Added `from_stream` test constructor.
- **`content.rs` check_integrity tests (4)**: valid shortcodes, unknown entity detection,
  name normalization, `entity_metrics` shortcode parsing.

### Fixed

- **`ipc.rs` / `petaltongue.rs`**: `MockStream` constructors use `Self` (pedantic
  clippy `use_self` lint).

### Metrics

- **254 total tests** (222 unit + 29 integration + 3 refresh_write)
- All modules now have `#[cfg(test)]` — 0 untested modules

---

## [3.2.0] — 2026-07-04 — Living Topology + Dep Evolution

**Mesh topology content page, dependency evolution (toml 1.x / TOML spec 1.1),
version alignment, and stale doc reconciliation.**

### Added

- **`content/architecture/MESH_TOPOLOGY.md`**: Living topology page with
  `viz_embed(src="/viz/gate-mesh?live=true")` — documents gate mesh architecture,
  capability routing, enrollment flow, and live health color mapping. Prepared
  for petalTongue `LiveMeshState` wire-up (Work Item 4 of living topology handoff).

### Changed

- **`toml` 0.8 → 1.x**: TOML spec 1.1 support. Zero breaking changes for our usage.
- **Crate version**: 0.3.0 → 0.3.1 (aligns with IPC consolidation release).
- **`specs/CONTEXT.md`**: Wave 128 → Wave 132d, version reference corrected.
- **`wateringHole/sporePrint/CONTENT_GUIDE.md`**: Replaced stale
  `scripts/render_notebooks.sh` reference with `cargo run -- render-notebooks`
  (script retired Wave 69).

---

## [3.1.0] — 2026-07-04 — Deep Debt Resolution + IPC Consolidation

**Deep debt cleanup and evolution across all 27 modules. Zero-copy idioms,
shared IPC module, static capability declarations, supply chain security,
and coverage boost. All quality gates pass. Zero mocks in production.**

### Architecture

- **Shared `ipc.rs` module**: Centralizes NDJSON JSON-RPC 2.0 client logic
  (previously duplicated across `cas_push`, `nucleus`, `petaltongue`, `tower`).
  Enforces response ID correlation (JSON-RPC §5) and `health.liveness` fallback
  for legacy primals.
- **Static capability declarations**: `discovery.rs` capabilities evolved from
  `Vec<String>` heap allocations to `&'static [&'static str]` slices compiled
  into `.rodata`. Zero runtime allocation for self-knowledge.
- **Zero-copy HTTP**: Body extraction via `Vec::split_off()` instead of `.to_vec()`.
  Scoped raw buffer drops in CAS push before JSON encoding.
- **`OnceLock` for env reads**: Forge URL cached for process lifetime.

### Added

- `deny.toml` — `cargo-deny` supply chain security (all deps pure Rust, no
  advisories, SPDX-compliant licenses, crates.io-only sources)
- `#![warn(missing_docs)]` — documentation lint active (guards future lib extraction)
- 14 new tests: mock-stream IPC roundtrip, certify emit/validate, refresh
  write_updates, count_metrics isolation, stored manifest deserialization
- SPDX license headers on all 15 templates + 7 SCSS files

### Changed

- `tower.rs`: inline JSON-RPC → `ipc::send_rpc` + `ipc::is_method_not_found`
- `nucleus.rs`: `probe_socket_health` → `ipc::probe_health` with `health.liveness`
- `petaltongue.rs`: inline `send_rpc` → `ipc::send_rpc` delegation
- `cas_push.rs`: scoped `contents` drop before JSON payload construction
- `fetch.rs`: `default_forge_url() -> String` → `-> &'static str` via `OnceLock`
- `http.rs`: header parsing before `split_off` to avoid borrow conflicts
- Integration test isolation: notebook rendering outputs to tempdir

### Removed

- `scripts/refresh-metrics.sh` — retired Wave 69, fossil record in git history

### Metrics

- Tests: 206 → **220** (188 unit + 29 integration + 3 refresh_write)
- Coverage: 60.77% → **64.87%** (ipc: 97%, certify: 71%, refresh: 78%)
- `cargo deny check` — clean (zero advisories, zero yanked)
- Zero TODO/FIXME/HACK in source
- Zero hardcoded IPs/ports in production code
- All `#[allow()]` justified (16 total)
- All files under 800 lines

---

## [3.0.0] — 2026-06-01 — Sovereign Self-Hosting + Provenance Data System

**sporePrint is now sovereign-primary. VPS serves the site via Caddy + Let's Encrypt.
GitHub Pages becomes the trailing extracellular shadow. BLAKE3 content addressing
enables provenance trio integration.**

### Sovereign Deployment
- VPS auto-rebuild pipeline: Forgejo push → relay chain → `sporeprint-rebuild.sh`
- systemd timer (15-min fallback) on golgiBody-ext
- Caddy TLS config with domain routing + Let's Encrypt CAA
- Sovereign DNS A records for primals.eco → golgiBody-ext
- GitHub Pages marked as trailing shadow in deploy.yml

### Provenance
- `provenance` subcommand: BLAKE3 content addressing for all 218 pages
- `content-manifest.toml`: deterministic root hash, per-page hashes + titles
- `--verify`: validate current content against manifest (integrity check)
- `--diff`: show new/changed/removed pages since last manifest
- `--write`: persist manifest for version-controlled provenance tracking

### Config
- `deploy_locations` now lists `golgiBody-ext` as primary
- `shadow_status` → `sovereign-primary`
- `sovereign_url` and `sovereign_rebuild` fields added

---

## [2.0.0] — 2026-05-31 — Deep Debt Resolution + Sovereign Evolution

**Complete code quality overhaul. trait-based architecture, 90%+ coverage,
zero dead code, capability-based discovery, shared utilities, and new
subcommands. sporePrint is now spring-grade quality.**

### Architecture

- **Trait-based VCS**: `VcsBackend` trait with `GitBackend` (production) and
  `MockBackend` (testing) — enables full test coverage without network I/O
- **Shared `time.rs`**: Pure Rust UTC date utility, deduplicated from 3 modules
- **`report.rs`**: Entity registry summarization — consumes all model fields,
  eliminates crate-level `dead_code` lint allowance
- **`links.rs`**: Internal link validation (absorbs external tools)
- **`error.rs`**: `thiserror`-based typed error hierarchy with `Diagnostic` enum
- **CSS semantic split**: monolith → `base.css` (tokens) + `main.css` (components)
- **Capability-based discovery**: `.gate` workspace walk, configurable origins

### Added

- `check-links` subcommand — validates 149 internal `@/` links across 207 files
- `render-notebooks --discover` — auto-discovers notebooks from ecosystem workspace
- `validate --verbose` — full entity report with all fields + totals display
- Private repo gating via `SPOREPRINT_REFRESH_PAT` environment variable
- `FetchOutcome` enum with structured results (replaces string messages)
- SPDX license headers on all Rust source files
- `static/css/base.css` — design system tokens extracted from monolith
- `static/gonzales/js/config.js` — capability-based API endpoint discovery

### Changed

- **Error handling**: `process::exit` → `thiserror` + `Result` propagation + `ExitCode`
- **Crate root**: `#![forbid(unsafe_code)]` enforced
- **Linting**: zero warnings for clippy pedantic + nursery (no `#[allow()]` in production)
- **`dead_code`**: removed crate-level allowance; all fields now consumed via `report.rs`
- **`fetch.rs`**: trait-based with `Source.clone_url()`, `Source.kind`, private filtering
- **`explorer.js`**: 1097L → 533L (config extracted to `config.js` at 140L)
- **`render_notebooks.sh`**: hardcoded paths → `.gate` file discovery
- All external `date` command calls replaced with pure Rust `time::today_utc()`
- `Diagnostic::message()` used in production output (not just tests)

### Removed

- 1,162 tracked build artifacts (`crates/spore-validate/target/`) — never should
  have been committed. `.gitignore` now catches `target/` at any depth.
- Crate-level `[lints.rust] dead_code = "allow"` — all fields truly consumed
- Duplicate date computation (3 copies → 1 in `time.rs`)
- TOML escape error in notebook front matter that broke Zola build

### Metrics

- Test coverage: 32.6% → **90.3%** (llvm-cov)
- Tests: 11 → **80** (50 unit + 12 integration + 3 refresh-write + 15 link/time)
- Modules: 6 → **12** (+ error, fetch rewrite, links, report, time, notebook rewrite)
- Max file size: 466L (all well under 1000L limit)
- Release build: 5.56s clean (lean deps, pure Rust)
- Binary size: 4.4MB (12 modules, 7 deps)
- `zola build`: 736ms, zero errors

---

## [1.1.0] — 2026-04-30 — Fully Rust Toolchain

**The Python validation script is replaced by `spore-validate`, a typed Rust
binary. The sporePrint pipeline is now 100% Rust — from validation to
generation to deployment.**

### Added

- **`crates/spore-validate/`** — Rust crate replacing `scripts/validate_registry.py`
  - Typed entity model: `EntityKind` enum (7 kinds), `Tier` enum (4 tiers),
    `Entity` struct with per-kind field validation
  - `validate` subcommand (default): registry field checks, totals verification,
    content taxonomy cross-references — full parity with Python script
  - `--check` flag: scans 2,488 entity shortcodes in prose, validates all
    resolve to registry keys
  - `--strict` flag: promotes warnings to errors
  - `refresh` subcommand: cross-repo metric comparison — discovers repos in
    `primals/`, `springs/`, `infra/` directories, counts Rust LOC, tests,
    files, and crates, reports drift with percentage change
  - 11 unit tests covering model deserialization, validation logic, totals
    verification, front matter extraction, and line counting
- **`content/philosophy/_index.md`** — atlasHugged integration stub explaining
  the "why" of ecoPrimals (AGPL-3.0, attribution, sovereignty)
- **Science pages 26–27**: neuromorphic sovereign driver (rustChip), nature
  preserve applied NPU science

### Changed

- **CI pipeline** (`deploy.yml`): `python3 scripts/validate_registry.py` →
  `cargo build --release` + `spore-validate validate --check` with cargo cache
- **rustChip entity** in `config.toml`: updated to 23,733 LOC, 367 tests,
  118 files; description includes glowplug, science demos, HW/SW separation
- **Landing page**: "25 baseCamp Papers" → "27 baseCamp Papers"
- **Specs updated**: `CONTEXT.md`, `EVOLUTION_QUEUE.md`, `CONTENT_MAP.md`,
  `RUST_TOOLING_VISION.md` all reflect the implemented tooling

### Metrics

- Release binary: 94ms runtime (vs 146ms Python)
- 60 entities validated, 2,488 shortcodes scanned
- 24 repos scannable via `refresh`, 87 metric drifts detected across ecosystem
- 11 Rust tests, 0 Python dependency

---

## [1.0.0] — 2026-04-03 — Zola: Rust to the Very Edge

**The site becomes sovereign infrastructure. Zola (Rust static site
generator) replaces raw GitHub Markdown rendering. Full content refresh.
guideStone section. Missing papers added. Search enabled.**

### Architecture

- **Zola 0.22.1** — single Rust binary, zero runtime dependencies
- Custom theme: dark mode (prefers-color-scheme), responsive sidebar,
  accessible (skip links, ARIA, focus outlines, semantic HTML)
- Built-in search via Elasticlunr (no external JS frameworks)
- GitHub Actions CI/CD: build with Zola, deploy to GitHub Pages
- CNAME moved to `static/CNAME` for Zola's build pipeline

### Content Migration

- All Markdown files migrated from flat directories to `content/` with
  TOML front matter (title, description, date, extras)
- Section indexes (`_index.md`) for audience, science, architecture,
  methodology, technical, guidestone
- Landing page (`content/_index.md`) replaces root README as site content
- New repo README for developer documentation

### New Content

- **`content/guidestone/_index.md`** — The verification class: five properties,
  first deployment artifact (hotSpring-guideStone-v0.7.0), self-leveling
  benchmark, onboarding pattern, cross-substrate validation (5 substrates,
  40/40 bit-identical), metrological analogy
- **Missing science papers added:**
  - Paper 14: Sovereign Compute Hardware (131+ experiments, deep debt evolution)
  - Paper 23: Mass-Energy-Information Equivalence (unifying hypothesis)
  - Paper 24: All-Silicon Science (sovereign GPU pipeline, both vendors)
  - Paper 24b: Esoteric Webb Composition Patterns (gen4 creative infrastructure)
  - Paper 25: Self-Tuning Simulation (physics-validated parameter discovery)

### Content Refresh

- Landing page updated: guideStone verification path, pre-built artifact
  verification option (no Rust required), gen4 references, updated ecosystem
  diagram with guideStone layer, physicist/collaborator audience path
- "Last Updated" date bumped to April 3, 2026
- guideStone callout in ecosystem glance section
- Foundation papers table updated with guideStone certification references

### Design Principles

- **Rust to the edge**: Zola generates the site. No Ruby, no Node, no Python.
- **Markdown-first**: All content stays in `.md` with TOML front matter.
  Agents and humans read and edit identically.
- **Self-contained**: No CDN, no Google Fonts, no analytics scripts.
- **Accessible**: Semantic HTML, ARIA landmarks, skip links, keyboard nav.
- **No external theme dependency**: Custom templates and CSS.

### Document count at v1.0.0: 49 pages, 6 sections

---

## [0.1.0] — 2026-03-17 — Initial Scaffold

**First spore print.**

### Added

**Audience docs (from publicRelease/):**
- `audience/FOR_FACULTY_AND_PIS.md` — what ecoPrimals replaces in a lab
- `audience/FOR_STUDENTS_AND_CORE_FACILITIES.md` — setup guide, 16S walkthrough
- `audience/FOR_HARDWARE_BUILDERS_AND_HOBBYISTS.md` — f64 Vulkan discovery, Games@Home
- `audience/FOR_COMPLIANCE_AND_INSTITUTIONAL_REVIEW.md` — FDA/ISO/HIPAA/GDPR mapping
- `audience/CAPABILITY_PARITY_BRIEF.md` — domain-by-domain comparison vs proprietary tools

**Technical docs (from publicRelease/):**
- `technical/DRUG_DISCOVERY_PIPELINE.md` — Anderson-augmented MATRIX, 329/329 checks
- `technical/MSU_ASSET_ACCELERATION.md` — Genomics Core, ICER, ADDRC, MSDS integration
- `technical/GRANT_TECHNICAL_APPENDIX.md` — NIH/NSF/USDA/DOE validation evidence
- `technical/KNOME_TEACHING_BRIEF.md` — K-Nome as pedagogy for real science

**Methodology (from whitePaper/gen3/):**
- `methodology/CONSTRAINED_EVOLUTION_FORMAL.md` — the core methodology paper
- `methodology/K_NOME_PROGRAMMING.md` — K-Nome operational framework
- `methodology/P_NP_ENZYME_THESIS.md` — P≠NP enzyme argument

**Architecture (from whitePaper/gen3/):**
- `architecture/ECOSYSTEM_ARCHITECTURE.md` — UniBin/ecoBin/genomeBin, NUCLEUS, Neural API
- `architecture/PRIMAL_CATALOG.md` — all 14 primals with capabilities and test counts
- `architecture/SPRING_CATALOG.md` — all 7 springs with checks, papers, cross-spring flow
- `architecture/SOVEREIGN_PRIOR_ART_CATALOG.md` — lysogeny prior art record

**Science (from whitePaper/gen3/baseCamp/):**
- `science/README.md` — baseCamp index with reading order by discipline
- 21 baseCamp papers (Papers 01–22, all available)

**Root:**
- `README.md` — whitepaper index, 4-audience guide, 5-minute verification
- `LICENSE` — CC-BY-SA 4.0 (docs) + AGPL-3.0 (code)
- `CHANGELOG.md` — this file

**Spring checks at scaffold time:**

| Spring | Checks |
|--------|:------:|
| wetSpring | 5,707+ |
| airSpring | 3,123+ |
| neuralSpring | 4,500+ |
| hotSpring | 664+ |
| groundSpring | 535+ |
| healthSpring | 474+ |
| ludoSpring | 1,692+ |
| **Total** | **16,695+** |

---

## [0.2.0] — 2026-03-17 — Data, analysis, and polish

### Added

- `technical/HARDWARE_COST_ANALYSIS.md` — The f64 Vulkan discovery ($0.044/run,
  9.9× DF64 uplift, $15K basement vs ICER/cloud cost analysis, NUCLEUS scaling
  math, GPU DF64 TFLOPS table for consumer cards)
- `architecture/EVOLUTION_TIMELINE.md` — Day-by-day 27-day sprint record.
  Velocity analysis (checks/day per sprint), what the timeline proves about
  K-Nome methodology, the infrastructure that made velocity possible
- `science/CROSS_SPRING_EVIDENCE_MAP.md` — The Anderson thread across 5 domains,
  paper-by-paper cross-spring dependency tables, convergent predictions where
  independent springs agree on the same number, the groundSpring anomaly,
  open questions awaiting wet-lab validation

### Updated

- `README.md` — Added "The Five Numbers" table ($0.044, 9.9×, 27 days, 20,695+,
  175+), new Hardware/Cost reading path, references to 3 new docs, improved
  document map with annotations

### Document count at v0.2.0: 47 files

---

## [0.3.0] — 2026-03-17 — Spring profiling, sovereign pipeline, knowledge commons

### Added

- `technical/SOVEREIGN_GPU_PIPELINE_PROFILE.md` — Complete vendor replacement
  story: what's already replaced (CUDA runtime, cuBLAS, nvcc, nvidia.ko →
  toadStool, BarraCuda, coralReef, coral-glowplug), current performance vs
  Kokkos (27×→3.7× gap closure), 3/6/12-month roadmap, proprietary cost table,
  full sovereign stack diagram. 27,169+ combined tests across 4 primals.
- `science/STRUCTURE_PREDICTION_ROADMAP.md` — coralForge: sovereign AlphaFold-
  quality structure prediction. The isomorphism proof (6 universal primitives),
  current status (154/154 checks), performance targets (~3 min/sequence on
  consumer GPU), LTEE at scale ($1K vs $83K cloud), drug docking and enzyme
  engineering extensions.
- `methodology/KNOWLEDGE_COMMONS_TARGETS.md` — What others can pick up with
  existing primals: 9 Tier 1 domains ready now (antibiotic resistance,
  wastewater surveillance, marine ecology, veterinary PK/PD, climate crops,
  materials science, educational games, fermentation, environmental tox).
  The three-lock guarantee (AGPL + ORC + CC-BY-SA). Velocity projection
  (75,000+ checks by March 2027). Why it can't be taken back.

### Updated

- `README.md` — Added sovereign GPU pipeline reading path, structure prediction
  in science section, knowledge commons in "what others can build" section,
  document map annotations for all new files.

### Document count at v0.3.0: 50 files

---

## [0.4.0] — 2026-03-17 — How to start a spring: the operational playbook

### Added

- `methodology/HOW_TO_START_A_SPRING.md` — The complete operational playbook
  for anyone starting their own spring. Carries the core K-Nome insight: LLMs
  work because of the data, the data is human language, every human is already
  an expert practitioner. You don't need to know how to code — you need to know
  how to talk. Includes: Phase 0→1→2→3+ protocol, the conversation patterns
  (analogy, correction, narrative, taste, redirection), a concrete worked
  example (fermentation spring, week by week), cost model ($150 GPU +
  electricity), honest constraints, and getting-started commands. Links to
  KNOWLEDGE_COMMONS_TARGETS.md for what domains are ready now.

### Updated

- `README.md` — Added HOW_TO_START_A_SPRING to methodology reading path,
  document map, and "What Others Can Build" section.

### Document count at v0.4.0: 51 files

---

## Roadmap

### [2.1.0] — pseudoSpore Gallery + Sovereign Deploy
- pseudoSpore gallery pages (`/lab/spores/{name}/`) with lithoSpore registry
- DNS cutover: primals.eco → golgiBody-ext (137.184.197.151)
- peptidoglycan build pipeline → golgiBody-ext Caddy
- GitHub Pages becomes extracellular shadow

### Future — petalTongue Integration
- gonzales JS files (JELLY STRING) absorbed by server-rendered SVG + WASM
- Conversational navigation of site content via petalTongue
- Audio narration from Markdown source

### Future — projectFOUNDATION Ingestion
- Replace GitHub Actions dispatch with Foundation-driven content publishing
- Temporal sync-driven rebuilds on flockGate (WAN shadow)
