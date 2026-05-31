# Rust Tooling for sporePrint

`spore-validate` is the pure Rust validation, metrics, and content pipeline
for sporePrint. It lives at `crates/spore-validate/` and enforces
`#![forbid(unsafe_code)]` at the crate root.

## Current State (Wave 63 — May 2026)

- **12 modules**, 80 tests, 90.3% line coverage (llvm-cov)
- Zero warnings for `clippy::pedantic` + `clippy::nursery`
- Zero `#[allow()]` in production code
- All files under 470 LOC (limit: 1000)
- Pure Rust dependencies only (ecoBin compliant)
- `thiserror`-based typed error hierarchy (`Result` propagation, no `process::exit`)

## Subcommands

| Command | Purpose |
|---------|---------|
| `validate` | Registry fields, totals sums, taxonomy tags, content lint |
| `validate --check` | + shortcode integrity + internal link validation |
| `validate --strict` | Promote warnings to errors |
| `validate --verbose` | Full entity report (all fields consumed) |
| `refresh <repos_root>` | Cross-repo metric drift detection |
| `refresh --write` | Auto-update config.toml |
| `fetch-refresh` | Clone upstream → refresh in one step |
| `render-notebooks <dirs>` | Jupyter .ipynb → Zola markdown |
| `render-notebooks --discover` | Auto-find notebooks via .gate walk |
| `check-links` | Validate all @/ internal links |

## Module Architecture

```
crates/spore-validate/src/
├── main.rs       — CLI (clap derive), orchestration, ExitCode
├── error.rs      — thiserror Error enum + Diagnostic enum
├── model.rs      — Typed entity model, Zola config parser
├── registry.rs   — Per-kind field validation
├── totals.rs     — Aggregate sum verification
├── content.rs    — Front matter taxonomy + shortcode checks
├── refresh.rs    — Cross-repo metric comparison + write-back
├── fetch.rs      — VcsBackend trait, GitBackend, MockBackend, Source model
├── notebook.rs   — Jupyter .ipynb JSON → Zola markdown
├── links.rs      — Internal @/ link validation
├── report.rs     — Entity/totals report generation (consumes all fields)
└── time.rs       — Pure Rust UTC date (shared, no external commands)
```

## Key Design Decisions

### Trait-based VCS (`fetch.rs`)

```rust
pub trait VcsBackend {
    fn clone_repo(&self, url: &str, target: &Path) -> Result<(), Error>;
    fn pull_repo(&self, target: &Path) -> Result<(), Error>;
    fn is_repo(&self, target: &Path) -> bool;
}
```

- `GitBackend`: production (shells out to `git`)
- `MockBackend`: testing (in-memory, no I/O) — enables 75%+ coverage on fetch
- Future: Forgejo API backend, temporal sync backend

### Capability-based Discovery

- `Source.origin`: explicit clone URL (supports Forgejo SSH, not just GitHub)
- `Source.private`: gated by `SPOREPRINT_REFRESH_PAT` env var
- `discover_springs_root()`: walks up filesystem looking for `.gate` file
- No hardcoded forge assumptions — primal code has self-knowledge only

### Error Propagation

```rust
#[derive(Debug, thiserror::Error)]
pub enum Error {
    Io { path, source },
    TomlParse(#[from] ...),
    Config(String),
    ValidationFailed { error_count, warning_count },
    Git(String),
}
```

All fallible operations return `Result<T, Error>`. The `main()` function
returns `ExitCode`, mapping `Err(e)` to a single `eprintln!` + FAILURE.

## Dependencies (7 total, all pure Rust)

| Crate | Purpose |
|-------|---------|
| `clap` | CLI argument parsing (derive) |
| `regex` | Taxonomy + link pattern matching |
| `serde` + `serde_json` | Entity registry + notebook deserialization |
| `toml` + `toml_edit` | Config parsing + in-place write-back |
| `thiserror` | Typed error derivation |
| `walkdir` | Directory traversal |

Dev-only: `tempfile` (test fixtures).

## Evolution Targets

- [ ] pseudoSpore gallery: read lithoSpore `registry.toml`, generate gallery markdown
- [ ] projectFOUNDATION ingestion: replace GitHub Actions dispatch with direct consumption
- [ ] Temporal sync trigger: detect upstream push → local rebuild on flockGate
- [ ] Absorb `refresh-metrics.sh` entirely (currently wraps `fetch-refresh`)
