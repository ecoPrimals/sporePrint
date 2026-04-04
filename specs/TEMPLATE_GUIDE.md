# sporePrint Template Guide

How templates, layouts, and content rendering work.

## Template Architecture

```
base.html
├── index.html              (landing page — extends base)
├── page.html               (single content page — extends base)
├── section.html            (section listing — extends base)
├── science_section.html    (science section — extends base, groups by domain)
├── taxonomy_list.html      (all terms in a taxonomy — extends base)
└── taxonomy_single.html    (all pages for a term — extends base)

shortcodes/
├── entity.html             (inline entity name + emoji link)
├── entity_metrics.html     (standard Lines: X Rust (Y files, Z crates, W tests) line)
├── entity_stat.html        (single metric from registry)
└── total_stat.html         (aggregate metric from [extra.totals])

scripts/
└── validate_registry.py    (pre-build validation, runs in CI)
```

## `base.html` — The Layout Shell

Everything extends `base.html`. It provides:

1. **`<head>`** — meta, CSS, favicon, search CSS, atom feed
2. **Nav bar** — 7 items: Science, Architecture, guideStone, Products | Primals, Springs, GitHub
3. **Site tree sidebar** — collapsible `<details>` showing all 7 sections + taxonomy indexes
   - Expands the current section to show all its child pages
   - Highlights the current page
   - On mobile (≤960px): collapses to top, `<details>` closed by default
4. **`<main>`** — the `{% block content %}` that children fill
5. **Footer** — license, tagline
6. **Scripts** — search (elasticlunr), mobile nav toggle

The site tree uses `get_section(path="...")` for each section — these are hardcoded. When adding a new section, add a `get_section` block to `base.html`.

## `index.html` — Landing Page

Hybrid template: renders `{{ section.content | safe }}` (from `_index.md`) inside structured HTML sections.

Content in `_index.md`:
- Try It (code blocks)
- Why sporePrint (prose)

Content in the template:
- Hero header
- Stats ribbon (6 cards — hardcoded numbers, update when counts change)
- Audience cards (6 links — hardcoded, update if audience pages change)
- Organization cards (3 — stable)
- Explore cards (6 quick links — update if key pages move)
- Landing footer (license)

**When updating stats**: edit the template HTML directly — the numbers are in `index.html`, not `_index.md`.

## `page.html` — Content Pages

Standard single-page layout:
- Breadcrumbs (auto-generated from `page.ancestors`)
- Title + description
- Per-page TOC sidebar (from markdown headings, sticky, right side on desktop)
- Content body

The TOC appears only if the page has headings (`{% if page.toc %}`).

## `section.html` — Section Listings

Renders section header + card grid of all child pages. Each card shows title + description.

## `science_section.html` — Domain-Grouped Papers

Groups science pages by `[extra] domain` value:
- Microbiology and Ecology, Physics and Materials, Agriculture and Field Science
- Human Health, Game Science and Systems, Economics and Provenance
- Reference (pages without a domain)

Domain list and icons are hardcoded in the template. To add a new domain: add it to the `domains` and `domain_icons` arrays in `science_section.html`.

## `taxonomy_list.html` / `taxonomy_single.html`

- **List**: shows all terms with emoji + display name + page count (card grid)
- **Single**: shows all pages referencing a term (card grid)

Both look up `config.extra.entity_registry[key]` for emoji/display name. Dynamic key access happens inside `{% block content %}` (Tera requires this — `{% set %}` outside blocks doesn't work in child templates).

## Adding a New Content Page

1. Create `content/{section}/MY_PAGE.md` with TOML front matter:
   ```toml
   +++
   title = "My Page Title"
   description = "One-line description for card listings and meta tags"
   date = 2026-04-01

   [taxonomies]
   primals = ["beardog", "barracuda"]
   springs = ["hotspring"]
   +++
   ```

2. Write markdown content below the closing `+++`

3. The page automatically appears in:
   - The section's card listing
   - The site tree sidebar (when its section is active)
   - Taxonomy pages for tagged entities
   - Full-text search index

## Adding a New Section

1. Create `content/newsection/_index.md`:
   ```toml
   +++
   title = "📌 New Section"
   description = "What this section covers"
   sort_by = "title"
   template = "section.html"
   +++
   ```

2. Add a `get_section` block to `base.html`'s site tree (copy an existing one, change the path)

3. Optionally add to the nav bar (if it's a primary navigation target)

## CSS Structure (`static/css/main.css`)

| Section | What it styles |
|---------|---------------|
| `:root` variables | Catppuccin Latte (light) colors |
| `@media (prefers-color-scheme: dark)` | Catppuccin Mocha (dark) colors |
| `.site-nav`, `.nav-*` | Top navigation bar |
| `.site-layout`, `.site-tree` | Flex layout with sidebar |
| `.landing`, `.hero`, `.stats-ribbon` | Landing page components |
| `.audience-*`, `.org-*`, `.explore-*` | Landing page card grids |
| `.content` | Typography (headings, links, code, tables, blockquotes) |
| `.page-layout`, `.toc` | Page layout with per-page TOC |
| `.page-listing` | Section card grid |
| `.breadcrumbs` | Breadcrumb navigation |
| `.entity-ref` | Shortcode-generated entity links |
| `.site-footer` | Footer |

All responsive breakpoints: `960px` (sidebar collapse), `768px` (nav collapse).
