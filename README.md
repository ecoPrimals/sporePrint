# sporePrint

The public-facing science site for [ecoPrimals](https://github.com/ecoPrimals) —
sovereign scientific computing.

**Live site:** [primals.eco](https://primals.eco)
**Sovereign primary:** golgiBody-ext VPS (137.184.197.151) via Caddy + Let's Encrypt
**Trailing shadow:** GitHub Pages (extracellular mirror, will be archived after NS cutover)

## Deployment Architecture

```
Gate pushes to Forgejo → K-Derm relay chain → golgiBody-ext
  1. Forgejo post-receive hook fires
  2. peptidoglycan relays to golgiBody-ext
  3. ext-github-push.sh pushes to GitHub (trailing shadow)
  4. sporeprint-rebuild.sh pulls + zola build (sovereign primary)
  5. Caddy serves from public/ (auto-TLS via Let's Encrypt)
  
Backup: systemd timer rebuilds every 15 minutes
```

## Stack

- **[Zola](https://www.getzola.org/) 0.22.1** — Rust static site generator (single binary, zero deps)
- **`spore-validate`** — Rust validation crate (registry, content, metrics, links, notebooks)
- **Markdown + TOML front matter** — human-readable, AI-parseable content
- **Custom theme** — Catppuccin Mocha/Latte, system fonts, dark/light, zero external deps
- **Caddy** — TLS termination + file serving on golgiBody-ext
- **Knot DNS** — Sovereign DNSSEC (ns1/ns2.primals.eco) with CAA for Let's Encrypt

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

## Auto-Refresh

### Sovereign (primary)
```
source repo push → Forgejo → relay chain → golgiBody-ext
  → sporeprint-rebuild.sh pulls from Forgejo + zola build
  → Caddy serves updated public/ (zero downtime)
```

### GitHub (trailing shadow — will be archived)
```
source repo push → notify-sporeprint.yml → repository_dispatch
  → sporePrint auto-refresh.yml
    → clone source, run spore-validate refresh --write
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

## Evolution Roadmap

### Wave 66+ — Sovereign Self-Hosting
- [x] VPS rebuild pipeline (Forgejo → zola build → Caddy)
- [x] systemd timer (15-min fallback rebuild)
- [x] Sovereign DNS records (primals.eco → golgiBody-ext)
- [x] Caddy TLS config with Let's Encrypt
- [ ] DNS registrar NS cutover to ns1/ns2.primals.eco
- [ ] Archive GitHub Pages deploy workflow to fossilRecord

### Wave 67+ — Provenance Trio Data System
- [ ] BLAKE3 content addressing: every published page gets a content hash
- [ ] rhizoCrypt DAG: site state tracked as content-addressed DAG
- [ ] loamSpine ledger: page publish events appended to immutable history
- [ ] sweetGrass attribution: PROV-O braids for every content source
- [ ] `spore-validate provenance` subcommand: verify content chain
- [ ] liveSpore.json: real-time content manifest with hashes + provenance

### Wave 68+ — Live Science Surface
- [ ] petalTongue renders live dashboards from primal APIs
- [ ] NestGate serves content directly (replace file_server)
- [ ] Forgejo webhook triggers sovereign CI rebuild (no GitHub Actions)

## License

- Code: [AGPL-3.0-or-later](https://www.gnu.org/licenses/agpl-3.0.html)
- Documents: [CC-BY-SA 4.0](https://creativecommons.org/licenses/by-sa/4.0/)
- Combined: scyBorg triple-copyleft (AGPL-3.0 + CC-BY-SA-4.0 + ORC provenance)
