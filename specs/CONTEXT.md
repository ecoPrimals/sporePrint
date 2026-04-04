# sporePrint — Project Context

Read this first. Everything an agent needs to make good decisions about sporePrint.

## What This Is

sporePrint is the public-facing website for the ecoPrimals sovereign scientific computing ecosystem. Hosted at **primals.eco** via GitHub Pages, built with **Zola** (Rust static site generator).

**sporePrint is human-facing.** wateringHole is the dev-facing shared context repo. sporePrint explains what the ecosystem IS, what it does, and how to verify it. It is not a technical reference manual — it is a compass.

## Current State (April 2026)

- **55 content pages** across 7 sections + landing page
- **2 taxonomies**: `primals` (14 terms), `springs` (8 terms) — build-validated typed tags
- **Entity registry** in `config.toml` — canonical display names, emojis, kinds for all entities
- **1 shortcode**: `entity.html` — renders `{{ entity(name="beardog") }}` as linked emoji+name
- **Site tree sidebar** — collapsible section-level navigation with current-page highlighting
- **Card-based landing page** — stats ribbon, audience cards, org cards, explore cards (no tables)
- **Full-text search** — Zola's built-in elasticlunr, indexed at build time

## Repository Structure

```
sporePrint/
├── config.toml              # Zola config + taxonomies + entity_registry
├── content/                 # All site content (Markdown + TOML front matter)
│   ├── _index.md            # Landing page (minimal — cards are in index.html template)
│   ├── architecture/        # 8 pages: catalogs, inventory, NUCLEUS, deployment, timeline
│   ├── audience/            # 5 pages: role-based entry points
│   ├── guidestone/          # guideStone verification class
│   ├── methodology/         # 5 pages: constrained evolution, K-Nome, spring guide
│   ├── products/            # 3 pages: esotericWebb, helixVision, blueFish
│   ├── science/             # 28 pages: 25 baseCamp papers + 3 reference docs
│   └── technical/           # 6 pages: hardware, GPU pipeline, grants, teaching
├── templates/
│   ├── base.html            # Layout: nav, site tree sidebar, footer, search
│   ├── index.html           # Landing page: hero + cards (stats, audience, orgs, explore)
│   ├── page.html            # Single page: breadcrumbs, TOC sidebar, content
│   ├── section.html         # Section listing: card grid of child pages
│   ├── science_section.html # Science section: groups papers by [extra] domain
│   ├── taxonomy_list.html   # All terms in a taxonomy (e.g., /primals/)
│   ├── taxonomy_single.html # All pages for one term (e.g., /primals/beardog/)
│   └── shortcodes/
│       └── entity.html      # {{ entity(name="beardog") }} → linked emoji+name
├── static/
│   ├── css/main.css         # Catppuccin Mocha/Latte, card layouts, site tree
│   ├── CNAME                # primals.eco
│   └── search.css
├── specs/                   # THIS DIRECTORY — internal, not built
├── .github/workflows/       # deploy.yml — zola build + zola check + GitHub Pages
└── CHANGELOG.md
```

## Key Technical Facts

- **Zola 0.22.1** — TOML front matter, Tera templates, strict mode
- **Catppuccin** color palette (Mocha dark / Latte light, auto via `prefers-color-scheme`)
- **zola check** runs as CI gate — all internal and external links validated before deploy
- **zola build** generates taxonomy pages automatically from front matter tags
- **No JavaScript** beyond Zola's built-in elasticlunr search and mobile nav toggle
- **Inline SVG favicon** — no external assets
- **`minify_html = true`** — output is minified

## Three Organizations

| Org | Role | URL |
|-----|------|-----|
| ecoPrimals | 17 primals + tooling (infrastructure) | github.com/ecoPrimals |
| syntheticChemistry | 8 springs (science validation) | github.com/syntheticChemistry |
| sporeGarden | Products (esotericWebb, helixVision, blueFish) | github.com/sporeGarden |

## Content Principles

1. **Tufte-esque**: every element justifies its space — no decorative filler
2. **Executable claims**: if we state a number, the reader can run a binary to verify it
3. **Agentic-friendly**: TOML front matter + structured Markdown = machine-parseable
4. **PII-minimal**: no personal names beyond published researchers, no locations, no employment history
5. **Replication, not endorsement**: researchers are listed as published work being reproduced, not collaborators or endorsers (see `CONTENT_VOICE.md`)
6. **Metrics over grades**: lines of code, test count, coverage percentage — not subjective letter grades

## Dependencies on Other Repos

| Repo | What sporePrint gets from it |
|------|------------------------------|
| wateringHole | GLOSSARY, PRIMAL_EMOJI_STANDARD, PUBLIC_SURFACE_STANDARD, LINK_INTEGRITY_STANDARD |
| whitePaper | gen3/ primal narratives, gen4/ guideStone architecture, baseCamp paper content |
| plasmidBin | Deployment model facts, metadata.toml format, current inventory counts |
| petalTongue | SPOREPRINT_CONTENT_DELIVERY_SPECIFICATION (future: how petalTongue consumes this site) |

## What Needs Periodic Refresh

- Check counts in landing page stats ribbon and spring/primal catalogs
- Squirrel version and test counts (fast-moving)
- plasmidBin inventory (new primals get metadata.toml entries)
- Science paper descriptions (new experiments get added to springs)
- Entity registry in config.toml (new primals/springs/products)
- Taxonomy tags in front matter when new content references new entities

## Nav Bar Structure

7 items: Science · Architecture · guideStone · Products | Primals · Springs · GitHub

Audience, Methodology, and Technical are accessible via the site tree sidebar. Architecture highlights when browsing Methodology or Technical paths.
