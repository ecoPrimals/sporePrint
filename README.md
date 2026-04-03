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
templates/           # Tera HTML templates
static/              # CSS, CNAME, static assets
  css/main.css       # Custom theme (dark mode, responsive, accessible)
config.toml          # Zola site configuration
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
