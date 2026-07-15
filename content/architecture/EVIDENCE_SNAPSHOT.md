+++
title = "Evidence Snapshot"
description = "Canonical metrics for the ecoPrimals ecosystem — single source of truth for all counts, definitions, and measurement methodology. Every other page should agree with this one."
date = 2026-07-07
weight = 1

[taxonomies]
primals = ["barracuda", "beardog", "nestgate", "songbird", "squirrel", "sweetgrass", "toadstool"]
springs = ["primalspring"]
trails = ["grant-ready", "evidence-chain"]

[extra]
domain = "Architecture"
maturity = "implemented"
+++

## Purpose

This page defines every metric used across sporePrint. When a number appears
elsewhere on the site, it should either pull from this registry via shortcodes
or state the measurement date explicitly. If a page conflicts with these numbers,
this page is correct and the other page is stale.

**Measured**: {{ total_stat(stat="measured_date") }} — via `spore-validate refresh`
(tokei line counts + `cargo test` pass counts from source repos)

---

## Ecosystem Scale

| Metric | Value | Definition |
|--------|-------|------------|
| **Total Rust LOC** | {{ total_stat(stat="total_loc_display") }} | Lines of Rust counted by tokei across all primal + spring repos |
| Primal Rust LOC | {{ total_stat(stat="primal_loc_display") }} | Infrastructure code ({{ total_stat(stat="total_primals") }} primals) |
| Spring Rust LOC | {{ total_stat(stat="spring_loc_display") }} | Science validation code ({{ total_stat(stat="total_springs") }} springs) |
| **Total test functions** | {{ total_stat(stat="total_tests_display") }} | `cargo test` unit + integration tests across all repos |
| Primal tests | {{ total_stat(stat="primal_tests_display") }} | Infrastructure test functions |
| Spring tests | {{ total_stat(stat="spring_tests_display") }} | Science validation test functions |
| **WGSL shaders** | {{ total_stat(stat="wgsl_files") }} files, {{ total_stat(stat="wgsl_lines_display") }} lines | Vendor-agnostic GPU compute (WebGPU) |
| **Validation checks** | {{ total_stat(stat="validation_checks") }} | Quantitative science assertions with explicit numerical tolerance |
| **Papers reproduced** | {{ total_stat(stat="papers_reproduced") }} | External peer-reviewed publications whose results are reproduced in Rust |
| **baseCamp papers** | {{ total_stat(stat="basecamp_papers") }} | ecoPrimals' own executable manuscripts/studies |
| **Primals** | {{ total_stat(stat="total_primals") }} | Sovereign infrastructure binaries (Rust, statically linked) |
| **Springs** | {{ total_stat(stat="total_springs") }} | Domain-specific science validation environments |
| **Content pages** | {{ total_stat(stat="content_pages") }} | Pages on this site (sporePrint) |

---

## What These Numbers Mean

### Test functions vs. validation checks

**Test functions** are standard Rust `#[test]` functions counted by `cargo test`.
They include unit tests, integration tests, property tests, and fuzz harnesses.
The number {{ total_stat(stat="total_tests_display") }} is the sum of all `cargo test`
passes across all repos.

**Validation checks** are a subset: the {{ total_stat(stat="validation_checks") }}
quantitative science assertions that compare computed results against published
values with explicit numerical tolerances. These are the "does the science
reproduce?" checks. Every validation check is also a test function, but not
every test function is a validation check.

### Papers reproduced vs. baseCamp papers

**Papers reproduced** ({{ total_stat(stat="papers_reproduced") }}) are external,
peer-reviewed publications from journals (Nature, Science, PNAS, etc.) whose
key results are reproduced in Rust with explicit tolerance comparisons. The
count includes papers across all {{ total_stat(stat="total_springs") }} springs.

**baseCamp papers** ({{ total_stat(stat="basecamp_papers") }}) are ecoPrimals' own
executable manuscripts — each is a narrative with embedded `cargo test` results
that a reader can reproduce. These are in the [Science](@/science/_index.md) section.

### Primals vs. springs vs. products

**Primals** are infrastructure: the binaries that form the mesh ({{ entity(name="songbird") }}
for routing, {{ entity(name="beardog") }} for identity, {{ entity(name="nestgate") }}
for storage, etc.). There are {{ total_stat(stat="total_primals") }}.

**Springs** are science validation environments: domain-specific test suites that
reproduce published results. There are {{ total_stat(stat="total_springs") }}
(7 science domains + 1 meta-spring for ecosystem validation).

**Products** are compositions of primals aimed at specific use cases
({{ entity(name="helixvision") }}, {{ entity(name="bluefish") }},
{{ entity(name="esotericwebb") }}). Products have their own maturity levels.

---

## Measurement Methodology

All metrics come from source code, not estimates:

1. **LOC**: `tokei` run on each repo's `src/` and `crates/` directories
2. **Tests**: `cargo test` pass counts from CI or local runs
3. **WGSL**: `tokei` on `*.wgsl` files in {{ entity(name="barracuda") }}
4. **Validation checks**: counted from `validate_*` and `exp_*` test binaries
5. **Papers**: counted from spring validation summaries (each paper has a named test)

The `spore-validate refresh` command automates this: it clones all repos,
runs tokei, and compares against the registry. Drift beyond 5% triggers a warning.

---

## Maturity Levels

Claims across this site carry maturity labels:

{{ maturity(level="implemented") }} — Code exists and tests pass.

{{ maturity(level="reproduced") }} — Matches an external published result with explicit tolerance.

{{ maturity(level="certified") }} — Portable guideStone artifact exists and is verifiable.

{{ maturity(level="architectural") }} — Design exists and is partially implemented; not fully validated.

{{ maturity(level="planned") }} — Roadmap item. No implementation yet.

{{ maturity(level="unaudited") }} — Claim is not externally reviewed (security, compliance, regulatory).

When you see a claim without a maturity badge, assume {{ maturity(level="implemented") }}
for code claims and {{ maturity(level="unaudited") }} for compliance/security claims.

---

## Historical Notes

Pages dated **March 2026** reflect the ecosystem state at that time (~3.2M LOC,
~107K tests, 7 springs, 14 primals). Those pages are historical snapshots.
The current numbers are on this page.

Pages that use `{{ "{{" }} total_stat(...) {{ "}}" }}` or
`{{ "{{" }} entity_metrics(...) {{ "}}" }}` shortcodes pull from the live
registry and are always current.

---

## Per-Entity Metrics

For individual primal and spring metrics (LOC, tests, files, crates), see:

- [Primal Catalog](@/architecture/PRIMAL_CATALOG.md) — all {{ total_stat(stat="total_primals") }} primals with live metrics
- [Spring Catalog](@/architecture/SPRING_CATALOG.md) — all {{ total_stat(stat="total_springs") }} springs with live metrics
- Taxonomy pages — [/primals/](/primals/) and [/springs/](/springs/) — auto-generated from registry

---

## Verify It

```bash
git clone https://github.com/ecoPrimals/sporePrint.git && cd sporePrint
cargo run --manifest-path crates/spore-validate/Cargo.toml -- validate --check --verbose
cargo run --manifest-path crates/spore-validate/Cargo.toml -- certify
```

The `certify` command computes a BLAKE3 Merkle root of the entity graph.
Compare it against the [published manifest](/certification/manifest.json).
If they match, the registry is internally consistent.
