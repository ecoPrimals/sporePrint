# sporePrint Taxonomy & Registry Standard

How build-validated entity references and data-driven metrics work in sporePrint.

## Overview

sporePrint uses a **single source of truth** pattern: all entity data (names, emojis, LOC, tests, files, crates, domains) lives in `config.toml`'s entity registry. Content pages reference this data via shortcodes and taxonomy tags. A pre-build validation script enforces required fields. The result: update `config.toml` once after a tokei run, and the numbers propagate to every page that references them.

## Taxonomies

Two taxonomies are defined in `config.toml`:

| Taxonomy | Terms | Generated pages |
|----------|-------|-----------------|
| `primals` | 15 (beardog, songbird, nestgate, ...) | `/primals/`, `/primals/{name}/` |
| `springs` | 8 (hotspring, airspring, wetspring, ...) | `/springs/`, `/springs/{name}/` |

A `products` taxonomy was intentionally NOT created — it collides with the `products/` content section URL. Products are a content section, not a taxonomy.

## Naming Convention

Taxonomy term values in front matter are **lowercase, no spaces, no camelCase**:

```toml
[taxonomies]
primals = ["beardog", "barracuda", "biomeos"]
springs = ["hotspring", "wetspring"]
```

These map to display names via the entity registry:
- `beardog` → `🐻🐕 BearDog`
- `barracuda` → `🐟⚡ barraCuda`
- `biomeos` → `🌿🖥️ biomeOS`

## Entity Registry Schema

`config.toml` contains `[extra.entity_registry.{key}]` entries. Required fields depend on `kind`. As of June 2026: **66 entities** across 7 kinds.

### All entities (required)

| Field | Type | Example |
|-------|------|---------|
| `display` | string | `"BearDog"` |
| `emoji` | string | `"🐻🐕"` |
| `kind` | string | See kind table below |

### Kind-specific required fields

| Kind | Required fields | What it is |
|------|----------------|------------|
| `primal` | `domain`, `loc`, `loc_display`, `tests`, `tests_display`, `files`, `crates`, `repo`, `tier` | A self-contained Rust binary providing domain primitives |
| `spring` | `domain`, `loc`, `loc_display`, `tests`, `tests_display`, `files`, `crates`, `repo` | A validation environment composing primals against published science |
| `product` | `domain` | A sporeGarden user-facing application |
| `composition` | `description` | A named primal interaction pattern (Tower Atomic, NUCLEUS, etc.) |
| `concept` | `description` | A named standard, methodology, or pattern (guideStone, BYOB, scyBorg, etc.) |
| `infra` | `description` | An ecosystem infrastructure repository (wateringHole, plasmidBin, etc.) |
| `org` | `description` | A GitHub organization (ecoPrimals, syntheticChemistry, sporeGarden) |

### Optional fields (any kind)

| Field | Type | Purpose |
|-------|------|---------|
| `description` | string | One-line definition — renders as tooltip on hover |
| `page` | string | Link target for the entity shortcode (absolute path or URL) |
| `repo` | string | GitHub org/repo path |

### Link resolution in the `entity` shortcode

1. `primal` or `spring` → auto-links to taxonomy page `/{kind}s/{key}/`
2. Any kind with `page` field → links to that page
3. Any kind without `page` → renders as styled text with tooltip (no link)

### Primal tier values

| Tier | Meaning |
|------|---------|
| `foundation` | Production-ready NUCLEUS primals (BearDog, Songbird, NestGate, ToadStool, Squirrel, biomeOS, coralReef, barraCuda) |
| `post-nucleus` | Capabilities that compose after NUCLEUS (petalTongue, rhizoCrypt, sweetGrass, loamSpine, skunkBat) |
| `meta` | Build-time tooling, not a runtime service (sourDough) |
| `tooling` | Additional utilities publishing soon (bingoCube, agentReagents, benchScale) |

### Aggregate Totals

`[extra.totals]` stores precomputed aggregates:

```toml
[extra.totals]
primal_loc = 2595725
primal_loc_display = "2,595,725"
total_loc_display = "3.2M"
total_tests_display = "107K+"
measured_date = "2026-04-04"
```

The validation script checks that `primal_loc` equals the sum of all primal `loc` values, etc.

## Shortcodes

### `entity` — linked name with emoji

```markdown
{{ entity(name="beardog") }}
```
Renders: `🐻🐕 BearDog` (linked to `/primals/beardog/`)

### `entity_metrics` — standard metrics line

```markdown
{{ entity_metrics(name="beardog") }}
```
Renders: **Lines**: 402,770 Rust (2,075 files, 60 crates, 16,848 tests)

Use this in catalogs instead of hardcoding numbers. When `config.toml` is updated, every catalog entry updates automatically.

### `entity_stat` — single metric value

```markdown
{{ entity_stat(name="beardog", stat="loc_display") }}
```
Renders: `402,770`

Use for inline references to a specific metric in prose.

### `total_stat` — aggregate metric value

```markdown
{{ total_stat(stat="total_loc_display") }}
```
Renders: `3.2M`

Use for ecosystem-wide statistics.

## Adding a New Entity

1. **Add to entity registry** in `config.toml` with ALL required fields for its kind:
   ```toml
   [extra.entity_registry.newprimal]
   display = "newPrimal"
   emoji = "🦊🔬"
   kind = "primal"
   domain = "Some Domain"
   loc = 12345
   loc_display = "12,345"
   tests = 678
   tests_display = "678"
   files = 90
   crates = 3
   repo = "ecoPrimals/newPrimal"
   tier = "foundation"
   ```

2. **Update `[extra.totals]`** — recompute aggregates

3. **Update wateringHole** — add the emoji mapping to `PRIMAL_EMOJI_STANDARD.md`

4. **Tag relevant pages** — add the term to `[taxonomies]` in front matter

5. **Run validation** — `cargo run -p spore-validate -- validate --check`

6. **Build** — `zola build` will generate taxonomy pages automatically

## Update Procedure (Refreshing Metrics)

When codebase metrics change:

1. Run `spore-validate fetch-refresh --write` (clones/pulls all sources, compares metrics, writes drifts)
2. Or manually: `spore-validate refresh <repos_root> --write`
3. `zola build` — all shortcoded pages pick up new numbers automatically

## Validation: `spore-validate validate`

Runs in CI before `zola build` (`crates/spore-validate/`). Checks:

| Check | Severity |
|-------|----------|
| Every entity has required base fields (`display`, `emoji`, `kind`) | Error |
| `kind` is one of the 7 valid values | Error |
| Every entity has required fields for its `kind` | Error |
| `tier` values are valid (primals only) | Error |
| Aggregate totals match sum of individual entries | Error |
| Taxonomy tags in content reference valid registry keys | Error |
| Taxonomy tag kind matches the taxonomy it's in (primal in `primals`, etc.) | Warning |
| Primal/spring registry keys not referenced by any content page | Warning |

Non-taxonomy kinds (composition, concept, infra, org) are not checked for content references — they don't use Zola taxonomies.

## What Zola Additionally Validates

- Internal links (`@/path/to/page.md`) validated by `zola check`
- Taxonomy terms validated against defined taxonomies

## What Nothing Currently Validates

- Whether prose mentions "BearDog" but doesn't tag `primals = ["beardog"]`
- Whether the entity registry emoji matches wateringHole's PRIMAL_EMOJI_STANDARD
- Whether `loc` numbers in the registry have drifted from actual repo counts

These are future evolution targets (see `EVOLUTION_QUEUE.md`).
