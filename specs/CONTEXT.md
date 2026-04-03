# sporePrint — Project Context

## What This Is

sporePrint is the public-facing website for the ecoPrimals sovereign
scientific computing ecosystem. It is hosted at **primals.eco** via
GitHub Pages and built with **Zola** (Rust static site generator).

## Repository Structure

```
sporePrint/
├── config.toml          # Zola config (base_url, search, highlighting)
├── content/             # All site content (Markdown + TOML front matter)
│   ├── _index.md        # Landing page
│   ├── architecture/    # Primal catalog, spring catalog, deployment model, ecosystem arch
│   ├── audience/        # Role-based entry points (faculty, students, builders, compliance)
│   ├── guidestone/      # guideStone verification class documentation
│   ├── methodology/     # How-to guides, formal methods, knowledge commons
│   ├── science/         # 25 baseCamp papers organized by domain
│   └── technical/       # Hardware, GPU pipeline, grants, teaching
├── templates/           # Tera templates (base, index, section, page, science_section)
├── static/              # CSS (Catppuccin Mocha/Latte), CNAME, search.css
├── specs/               # THIS DIRECTORY — internal specs, not built
├── .github/workflows/   # deploy.yml — Zola build + zola check + GitHub Pages
└── CHANGELOG.md
```

## Key Technical Facts

- **Zola 0.22.1** — TOML front matter, Tera templates, strict mode
- **Catppuccin** color palette (Mocha dark / Latte light, auto via prefers-color-scheme)
- **zola check** runs as CI gate — all internal and external links validated before deploy
- **50 pages, 6 sections** as of March 2026
- Custom `science_section.html` template groups papers by `[extra] domain` metadata
- Inline SVG favicon, semantic emoji pairs per PRIMAL_EMOJI_STANDARD
- No JavaScript beyond Zola's built-in search

## Three Organizations

| Org | Role | URL |
|-----|------|-----|
| ecoPrimals | 14 primals (infrastructure) | github.com/ecoPrimals |
| syntheticChemistry | 8 springs (science validation) | github.com/syntheticChemistry |
| sporeGarden | Products (esotericWebb, helixVision) | github.com/sporeGarden |

## Content Principles

- **Tufte-esque**: every element justifies its space
- **Executable claims**: if we say a number, the reader can run a binary to verify it
- **Agentic-friendly**: TOML front matter + structured Markdown = machine-parseable
- **PII-minimal**: no personal names beyond published faculty, no locations, no employment history
- **Emoji pairs**: canonical 2-emoji mapping per PRIMAL_EMOJI_STANDARD.md in wateringHole

## Dependencies on Other Repos

| Repo | What sporePrint gets from it |
|------|------------------------------|
| wateringHole | GLOSSARY, PRIMAL_EMOJI_STANDARD, PUBLIC_SURFACE_STANDARD, LINK_INTEGRITY_STANDARD |
| whitePaper | baseCamp papers (source content for science/ section), gen4 guideStone architecture |
| plasmidBin | Deployment model facts, metadata.toml format, current inventory counts |
| petalTongue | SPOREPRINT_CONTENT_DELIVERY_SPECIFICATION (how petalTongue consumes this site) |

## What Needs Periodic Refresh

- Check counts in `_index.md` and spring/primal catalogs (drift as springs evolve)
- Squirrel version and test counts (fast-moving)
- plasmidBin inventory (new primals get metadata.toml entries)
- Science paper descriptions (new experiments get added to springs)
- Faculty table (new collaborations)
