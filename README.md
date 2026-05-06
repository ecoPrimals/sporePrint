# sporePrint

The public-facing site for [ecoPrimals](https://github.com/ecoPrimals) —
sovereign scientific computing.

**Live site:** [primals.eco](https://primals.eco)

## Stack

- **[Zola](https://www.getzola.org/)** — Rust static site generator (single binary, zero deps)
- **Markdown + TOML front matter** — human-readable, AI-parseable content
- **Custom theme** — no third-party dependencies
- **GitHub Actions** → GitHub Pages deployment

## Local Development

```bash
# Install Zola (https://www.getzola.org/documentation/getting-started/installation/)
# Or: download binary from https://github.com/getzola/zola/releases

zola serve           # http://127.0.0.1:1111
zola build           # output to public/
zola check           # verify links
```

## Structure

```
content/             # Markdown content with TOML front matter
  _index.md          # Landing page
  audience/          # Audience-specific guides
  science/           # baseCamp papers (25+)
  architecture/      # Ecosystem architecture docs
  methodology/       # Constrained evolution, K-NOME, playbooks
  technical/         # Hardware, grants, pipelines
  guidestone/        # guideStone verification class + deployment artifact
  lab/               # Live validation results from projectNUCLEUS
templates/           # Tera HTML templates
static/              # CSS, CNAME, static assets
  css/main.css       # Custom theme (dark mode, responsive, accessible)
config.toml          # Zola site configuration + entity registry (metrics)
sources.toml         # GitHub repo map for auto-refresh
scripts/
  refresh-metrics.sh # Clone upstream, run spore-validate refresh --write
  render_notebooks.sh # Convert Jupyter notebooks to lab pages
crates/
  spore-validate/    # Typed validation: registry, content, metric sync
```

## Auto-Refresh (CI)

Primal and spring repos notify sporePrint on push to main via
`repository_dispatch`. sporePrint's `auto-refresh.yml` workflow clones the
source repo, runs `spore-validate refresh --write`, and commits updated
metrics to `config.toml`. The existing `deploy.yml` picks up the push and
rebuilds the site.

```
source repo push → notify-sporeprint.yml → repository_dispatch
  → sporePrint auto-refresh.yml
    → clone source, run spore-validate refresh --write
    → commit config.toml if changed
    → deploy.yml → zola build → GitHub Pages
```

**Metrics (Tier 1)**: LOC, tests, files, crates — auto-committed directly.
**Content (Tier 2)**: Lab pages from `sporeprint/` dirs in source repos —
created as PRs for review.

### Setup for new repos

1. Copy `templates/notify-sporeprint.yml` (from plasmidBin) to
   `.github/workflows/notify-sporeprint.yml` in the source repo
2. Add `SPOREPRINT_DISPATCH_TOKEN` secret to the source repo
3. Add the source to `sources.toml` in sporePrint

### Manual refresh

```bash
# Refresh all sources (requires local checkouts)
bash scripts/refresh-metrics.sh all

# Refresh single source
bash scripts/refresh-metrics.sh wetspring
```

## Adding Content

Every page is a Markdown file with TOML front matter:

```markdown
+++
title = "Page Title"
description = "Short description for listings and search"
date = 2026-04-03
+++

Your content here...
```

Section indexes use `_index.md` with `template = "section.html"`.

## License

- Documents: [CC-BY-SA 4.0](https://creativecommons.org/licenses/by-sa/4.0/)
- Code references: [AGPL-3.0-or-later](https://www.gnu.org/licenses/agpl-3.0.html)
