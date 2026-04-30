# Rust Tooling for sporePrint

`spore-validate` is the typed Rust replacement for the Python validation script.
It lives at `crates/spore-validate/` and provides three modes of operation:

## `spore-validate validate` (default)

Replaces `scripts/validate_registry.py` with typed data structures.

**Checks performed:**

1. Every entity has required base fields (`display`, `emoji`, `kind`)
2. `kind` is a compile-time enum — invalid values fail deserialization
3. Required fields per kind enforced by explicit validation (primals need
   `loc`, `tests`, `tier`, etc.)
4. `Tier` is a typed enum (`Foundation | PostNucleus | Meta | Tooling`) —
   typos are caught at parse time
5. Aggregate totals (`[extra.totals]`) match sums of individual entries
6. Taxonomy tags in content front matter reference valid registry keys
7. Taxonomy tag kind matches the taxonomy it appears in

All checks the Python script performed, plus compile-time schema guarantees.

### `--check` flag

When passed, also scans all Markdown prose for `{{ entity(name="xxx") }}`
Tera shortcodes and verifies each `xxx` exists in the entity registry.

### `--strict` flag

Promotes warnings to errors.

## `spore-validate refresh <repos_root>`

Cross-repo metric sync. Given the path to the ecoPrimals checkout root
(containing `primals/`, `springs/`, `infra/`), this command:

1. Discovers each entity's local repo from the `repo` field
2. Counts Rust LOC (non-blank, non-comment lines in `.rs` files)
3. Counts tests (`#[test]`, `#[tokio::test]` annotations)
4. Counts `.rs` files and Cargo.toml crates
5. Compares actual vs registered metrics
6. Reports drift with percentage change

This replaces the manual "pull repos, run tokei, update config.toml" workflow.

## Typed Data Model

The entity registry schema is encoded as Rust types:

```rust
enum EntityKind { Primal, Spring, Product, Composition, Concept, Infra, Org }
enum Tier { Foundation, PostNucleus, Meta, Tooling }

struct Entity {
    display: String,
    emoji: String,
    kind: EntityKind,
    // Kind-specific fields are Option — validated per-kind at runtime
    // so all errors can be collected before failing.
    loc: Option<u64>,
    tier: Option<Tier>,
    // ...
}
```

The Rust type system enforces the schema at compile time. If someone adds a
new `kind` without updating the enum, the code won't compile. If someone adds
a primal without `loc`, deserialization succeeds but validation fails with a
clear, collected error message.

## Why Rust, Not Just Better Python

The entity registry is already a type system — it has kinds, required fields
per kind, tier enums, and validated totals. The Python script checked these
rules at runtime with string comparisons. Rust encodes them as types:

- `Tier::Foundation` vs `"foundation"` — the compiler catches typos
- `Entity { loc: Option<u64> }` vs `entry.get("loc", 0)` — missing fields
  are explicitly validated, not silently defaulted
- `EntityKind` enum vs `VALID_KINDS` set — new kinds require updating the
  enum, which forces updating all match arms

## CI Integration

The GitHub Actions workflow (`deploy.yml`) now builds and runs
`spore-validate` instead of the Python script. The Rust toolchain is
pre-installed on `ubuntu-latest`; a cargo cache keeps subsequent CI runs fast.

## Crate Structure

```
crates/spore-validate/
├── Cargo.toml
├── Cargo.lock
└── src/
    ├── main.rs       — CLI (clap), orchestration, reporting
    ├── model.rs      — Typed entity model, Zola config parser
    ├── registry.rs   — Per-kind field validation
    ├── totals.rs     — Aggregate sum verification
    ├── content.rs    — Front matter taxonomy + entity shortcode checks
    └── refresh.rs    — Cross-repo metric comparison
```

11 tests covering model deserialization, registry validation, totals
verification, front matter extraction, and line counting.
