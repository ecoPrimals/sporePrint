# Rust Tooling Vision for sporePrint

The Python validation script (`scripts/validate_registry.py`) is the only
non-Rust tool in the sporePrint pipeline. It works, but it represents a
direction we want to evolve: from Python scripts doing ad-hoc validation to
Rust-powered typed tooling that makes the dense data systems of ecoPrimals
parseable, verifiable, and human-friendly.

## What the Python Script Does Today

`validate_registry.py` (172 lines) performs 7 checks:

1. Every entity has required base fields (`display`, `emoji`, `kind`)
2. `kind` is one of 7 valid values
3. Every entity has required fields for its `kind` (e.g., primals need `loc`, `tests`, `tier`)
4. `tier` values are from the valid set
5. Aggregate totals (`[extra.totals]`) match sums of individual entries
6. Taxonomy tags in content front matter reference valid registry keys
7. Taxonomy tag kind matches the taxonomy it's in

It runs in CI before `zola build`. Exit 0 = clean.

## What a Rust Replacement Would Provide

### Phase 1: `spore-validate` — direct replacement

A Rust binary (`spore-validate`) that does everything the Python script does,
but with typed data structures:

```rust
#[derive(Deserialize)]
struct EntityRegistry {
    #[serde(flatten)]
    entities: HashMap<String, Entity>,
}

enum Entity {
    Primal(PrimalEntity),
    Spring(SpringEntity),
    Product(ProductEntity),
    Composition(CompositionEntity),
    Concept(ConceptEntity),
    Infra(InfraEntity),
    Org(OrgEntity),
}

struct PrimalEntity {
    display: String,
    emoji: String,
    domain: String,
    loc: u64,
    loc_display: String,
    tests: u64,
    tests_display: String,
    files: u32,
    crates: u32,
    repo: String,
    tier: Tier,
    // ...
}

enum Tier {
    Foundation,
    PostNucleus,
    Meta,
    Tooling,
}
```

The Rust type system enforces the schema at compile time. If someone adds a
new `kind` without updating the enum, the code won't compile. If someone adds
a primal without `loc`, deserialization fails with a clear error message.

**Benefit over Python**: schema errors are caught at build time in the tooling
crate, not at CI runtime. The validator binary is a single executable with no
Python dependency.

### Phase 2: `spore-refresh` — cross-repo metric sync

A Rust tool that:

1. Walks all primal/spring repos (from `repo` fields in the registry)
2. Runs `tokei` (or uses the `tokei` crate directly) to count LOC
3. Runs `cargo test --workspace` to count tests
4. Compares actual vs registered metrics
5. Optionally updates `config.toml` with fresh numbers

This replaces the manual "pull repos, run tokei, update config.toml" workflow
described in `TAXONOMY_STANDARD.md`.

### Phase 3: `spore-check` — content integrity

A Rust tool that:

1. Parses all Markdown front matter (TOML)
2. Checks that entity names in prose match taxonomy tags
3. Validates cross-references between sections
4. Detects stale metrics (config.toml vs repo reality)
5. Detects missing content (whitePaper baseCamp papers without sporePrint equivalents)

This is the "grammar compiler" described in `EVOLUTION_QUEUE.md` P1 — but
implemented in Rust with typed parsing.

## Why Rust, Not Just Better Python

The entity registry is already a type system — it has kinds, required fields per
kind, tier enums, and validated totals. The Python script checks these rules
at runtime with string comparisons. Rust encodes them as types:

- `Tier::Foundation` vs `"foundation"` — the compiler catches typos
- `PrimalEntity { loc: u64 }` vs `entry.get("loc", 0)` — missing fields fail
  to deserialize, not silently default
- `Entity` enum vs `VALID_KINDS` set — new kinds require updating the enum,
  which forces updating all match arms

This is the same philosophy that drives ecoPrimals: the constraint (Rust's type
system) reveals the structure. The Python script works. The Rust version would
make the structure inspectable, composable, and impossible to subtly break.

## Implementation Notes

- The crate would live at `crates/spore-validate/` within sporePrint (or as a
  standalone tool)
- It would use `toml` + `serde` for config parsing, `pulldown-cmark` for
  Markdown parsing, `tokei` crate for LOC counting
- The binary could be distributed via plasmidBin for ecosystem-wide use
- The same typed entity structures could feed a Rust-powered content generator
  (replacing Tera shortcodes with compile-time content injection)

## Timeline

This is exploratory. No code yet. The vision is documented so that future
sessions can pick up where this one left off. When the Python script becomes
a bottleneck (more entity kinds, more cross-repo checks, more content integrity
rules), the Rust replacement will be ready to build.
