+++
title = "Self-Certifying Publication: sporePrint as guideStone"
description = "Every claim on this site is backed by executable verification. Clone the repo, run one command, compare the hash."
weight = 15

[extra]
foundation = true
domain = "Architecture"
maturity = "implemented"
+++

## The Principle

guideStone's five verification properties — deterministic, reference-traceable, self-verifying, environment-agnostic, and tolerance-documented — are not limited to physics computation. They apply equally to *information publication*.

sporePrint publishes verifiable claims: entity counts, code metrics, relationship graphs. These are not just text on a page — they are *certified* at build time, and any reader can independently reproduce the certification.

## The Five Properties Applied

### 1. Deterministic Output

Same `config.toml` + same content = same certification manifest. The manifest includes entity counts, graph Merkle root, and content page totals. Identical inputs always produce identical outputs.

### 2. Reference-Traceable

Every metric traces to a source. The `repo` field in each entity points to the actual codebase; `loc` and `tests` values are measured by `spore-validate refresh` from those repos. No number floats without a source.

### 3. Self-Verifying

The site publishes a `certification-manifest.json` containing a BLAKE3 Merkle root of the entity graph. The Merkle root is computed over sorted, deterministic edge representations — proving the graph is exactly what was declared.

### 4. Environment-Agnostic

`spore-validate` is pure Rust with zero C dependencies. It compiles to a static binary on any platform. Verification requires only the repo clone and Rust toolchain — no external services, no network calls, no platform-specific tools.

### 5. Tolerance-Documented

Metrics drift — repositories grow daily. Rather than hiding this reality, the manifest explicitly declares drift tolerance: `"5%/30d"` means metrics are expected to vary by up to 5% within 30 days of measurement. Re-certification happens on each deploy.

## Verify This Site

Any reader can verify sporePrint's published claims:

```bash
# Clone the repository
git clone https://github.com/ecoPrimals/sporePrint.git
cd sporePrint

# Build the validator
cargo build --release --manifest-path crates/spore-validate/Cargo.toml

# Run certification (validates against published manifest)
./crates/spore-validate/target/release/spore-validate certify

# Or generate a fresh manifest and compare hashes
./crates/spore-validate/target/release/spore-validate certify --emit
```

If the `graph_merkle` in your locally generated manifest matches the one at [/certification/manifest.json](/certification/manifest.json), the entity graph is exactly as published. No trust required — only verification.

## The Manifest

The certification manifest records:

| Field | Meaning |
|-------|---------|
| `version` | Manifest schema version |
| `generated` | UTC timestamp of generation |
| `entity_count` | Total entities in the registry |
| `primal_count` | Core primals (organisms) |
| `spring_count` | Springs (compositions) |
| `edge_count` | Total typed relationships |
| `graph_merkle` | BLAKE3 hash of sorted entity graph edges |
| `content_pages` | Markdown pages (excluding section indices) |
| `total_loc` | Lines of code across all tracked repos |
| `total_tests` | Test count across all tracked repos |
| `validation_errors` | Errors at certification time (must be 0) |
| `measured_date` | Date metrics were last measured |
| `drift_tolerance` | Declared acceptable drift window |

## Why BLAKE3?

BLAKE3 is already part of the ecoPrimals ecosystem (used by BearDog for content addressing). It is:
- Pure Rust (no C FFI)
- Extremely fast (single-threaded, no SIMD required)
- Cryptographically secure
- Deterministic across all platforms

The Merkle root is computed by sorting all edges as `source:target:relation` strings, then feeding them sequentially to the BLAKE3 hasher. This ensures order-independence while maintaining cryptographic binding.

## Relationship to the Knowledge Topology

Self-certification and the [typed entity graph](/architecture/renvois-knowledge-topology/) are complementary:

- The **entity graph** declares what connections *exist* between ideas
- The **certification manifest** *proves* those connections haven't been tampered with

Together they complete the arc from Diderot's renvois to a fully verifiable knowledge topology: connections that are typed, bidirectional, provenance-tracked, and cryptographically certified.
