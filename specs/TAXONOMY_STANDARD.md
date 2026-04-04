# sporePrint Taxonomy Standard

How build-validated entity references work in sporePrint.

## Overview

Zola taxonomies provide compile-time validation of entity references. Every content page declares which primals and springs it references via `[taxonomies]` in its TOML front matter. Zola validates these at build time and auto-generates cross-reference index pages.

## Taxonomies

Two taxonomies are defined in `config.toml`:

| Taxonomy | Terms | Generated pages |
|----------|-------|-----------------|
| `primals` | 14 (beardog, songbird, nestgate, ...) | `/primals/`, `/primals/{name}/` |
| `springs` | 8 (hotspring, airspring, wetspring, ...) | `/springs/`, `/springs/{name}/` |

A `products` taxonomy was intentionally NOT created — it collides with the `products/` content section URL. Products are a content section, not a taxonomy.

## Naming Convention

Taxonomy term values in front matter are **lowercase, no spaces, no camelCase**:

```toml
[taxonomies]
primals = ["beardog", "barracuda", "biomeos"]
springs = ["hotspring", "wetspring"]
```

These map to display names via the entity registry in `config.toml`:
- `beardog` → `🐻🐕 BearDog`
- `barracuda` → `🐟⚡ barraCuda`
- `biomeos` → `🌿🖥️ biomeOS`

## Entity Registry

`config.toml` contains `[extra.entity_registry.{key}]` entries for every entity:

```toml
[extra.entity_registry.beardog]
display = "BearDog"
emoji = "🐻🐕"
kind = "primal"
```

- `display` — canonical display name (camelCase as the ecosystem uses it)
- `emoji` — 2-emoji pair from wateringHole/PRIMAL_EMOJI_STANDARD.md
- `kind` — `"primal"`, `"spring"`, or `"product"`

Taxonomy templates and the `entity` shortcode look up this registry for rendering.

## Adding a New Entity

When a new primal, spring, or product is created:

1. **Add to entity registry** in `config.toml`:
   ```toml
   [extra.entity_registry.newprimal]
   display = "newPrimal"
   emoji = "🦊🔬"
   kind = "primal"
   ```

2. **Update wateringHole** — add the emoji mapping to `PRIMAL_EMOJI_STANDARD.md`

3. **Tag relevant pages** — add the term to `[taxonomies]` in front matter of pages that reference it:
   ```toml
   [taxonomies]
   primals = ["beardog", "newprimal"]
   ```

4. **Build** — `zola build` will generate `/primals/newprimal/` automatically

## Shortcode: `entity`

Use in markdown content:

```markdown
The cryptographic operations are handled by {{ entity(name="beardog") }}.
```

Renders as: `🐻🐕 BearDog` (linked to `/primals/beardog/`).

If `name` doesn't match any registry entry, renders as italic text with no link (visual warning that something is wrong).

The shortcode is defined in `templates/shortcodes/entity.html`.

## What the Build Validates

- Front matter `[taxonomies]` values must match a defined taxonomy in `config.toml`
- Internal links (`@/path/to/page.md`) are validated by `zola check`
- The entity registry is NOT validated against taxonomy terms — a registry entry without any page tagging it is silently ignored (it just won't generate a taxonomy page)

## What the Build Does NOT Validate

- Whether a page's prose mentions "BearDog" but doesn't tag `primals = ["beardog"]`
- Whether a taxonomy term is spelled consistently across pages
- Whether the entity registry emoji matches wateringHole's standard
- Semantic consistency of descriptions across pages

These are future evolution targets for a custom Rust lint tool (see `EVOLUTION_QUEUE.md`).
