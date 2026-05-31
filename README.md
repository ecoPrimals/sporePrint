# sporePrint

The public-facing science site for [ecoPrimals](https://github.com/ecoPrimals) —
sovereign scientific computing.

**Live site:** [primals.eco](https://primals.eco)
**Sovereign host:** golgiBody-ext VPS via Caddy (DNS cutover pending)

## Stack

- **[Zola](https://www.getzola.org/)** — Rust static site generator (single binary, zero deps)
- **`spore-validate`** — Rust validation crate (registry, content, metrics, links, notebooks)
- **Markdown + TOML front matter** — human-readable, AI-parseable content
- **Custom theme** — system fonts, dark/light mode, zero external dependencies
- **Deployment:** GitHub Pages (extracellular shadow) + golgiBody-ext VPS (sovereign target)

## Local Development

```bash
# Prerequisites: Zola, Rust toolchain

zola serve             # http://127.0.0.1:1111
zola build             # output to public/

# Validation (Rust — typed, pedantic, 90%+ coverage)
cd crates/spore-validate
cargo build --release
cargo run -- validate                    # registry + totals + taxonomies
cargo run -- validate --check --verbose  # + shortcode scan + entity report
cargo run -- check-links                 # internal link integrity
cargo run -- render-notebooks --discover # Jupyter → Zola markdown
cargo run -- fetch-refresh --write       # clone upstream, update metrics
```

## Structure

```
sporePrint/
├── config.toml          # Zola config + entity_registry (63 entities) + totals
├── sources.toml         # Upstream repo map (GitHub + Forgejo origins)
├── content/             # 207 Markdown pages with TOML front matter
│   ├── science/         # 27 baseCamp companion papers
│   ├── architecture/    # Ecosystem architecture docs
│   ├── lab/             # Spring validation summaries + rendered notebooks
│   ├── products/        # blueFish, esotericWebb, helixVision, lattice QCD
│   ├── guidestone/      # GuideStone verification class
│   ├── audience/        # PI, student, builder, compliance guides
│   ├── methodology/     # Constrained evolution, K-NOME, playbooks
│   └── technical/       # Hardware, grants, pipelines
├── templates/           # Tera HTML templates (base, page, section, taxonomy)
├── static/
│   ├── css/base.css     # Design tokens (Catppuccin Mocha/Latte)
│   ├── css/main.css     # Component styles
│   └── gonzales/        # Interactive science explorer (JELLY STRING → petalTongue)
├── crates/
│   └── spore-validate/  # Rust crate: typed validation, 12 modules, 80 tests
├── scripts/
│   ├── refresh-metrics.sh    # JELLY STRING — wraps spore-validate fetch-refresh
│   └── render_notebooks.sh   # JELLY STRING — vestigial, absorbed by Rust
├── specs/               # Internal standards (not built)
└── .github/workflows/   # deploy.yml, auto-refresh.yml
```

## spore-validate

Pure Rust validation binary — `#![forbid(unsafe_code)]`, clippy pedantic+nursery
zero warnings, 90.3% test coverage (llvm-cov).

| Subcommand | Purpose |
|---|---|
| `validate` | Registry field checks, totals sums, taxonomy tags, content lint |
| `validate --check` | + shortcode scan + internal link validation |
| `validate --verbose` | + full entity report with all fields |
| `refresh <repos_root>` | Cross-repo metric drift detection |
| `refresh --write` | Auto-update config.toml with current metrics |
| `fetch-refresh` | Clone upstream repos → refresh in one step |
| `render-notebooks` | Jupyter .ipynb → Zola markdown (pure JSON parse) |
| `render-notebooks --discover` | Auto-find notebooks via .gate workspace walk |
| `check-links` | Validate all @/ internal links (207 files, 149 links) |

## Auto-Refresh (CI)

```
source repo push → notify-sporeprint.yml → repository_dispatch
  → sporePrint auto-refresh.yml
    → clone source, run spore-validate refresh --write
    → commit config.toml if changed
    → deploy.yml → zola build → GitHub Pages
```

## Adding Content

Every page is a Markdown file with TOML front matter:

```markdown
+++
title = "Page Title"
description = "Short description for listings and search"
date = 2026-05-31
+++

Your content here...
```

## License

- Code: [AGPL-3.0-or-later](https://www.gnu.org/licenses/agpl-3.0.html)
- Documents: [CC-BY-SA 4.0](https://creativecommons.org/licenses/by-sa/4.0/)
- Combined: scyBorg triple-copyleft (AGPL-3.0 + CC-BY-SA-4.0 + ORC provenance)
