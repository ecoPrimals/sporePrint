# Rust Tooling for sporePrint

`spore-validate` is the pure Rust validation, certification, and content pipeline
for sporePrint. It lives at `crates/spore-validate/` and enforces
`#![forbid(unsafe_code)]` at the crate root.

## Current State (Wave 68 — June 2026)

- **14 modules**, 89+ tests, 90.3% line coverage (llvm-cov)
- Zero warnings for `clippy::pedantic` + `clippy::nursery`
- Zero `#[allow()]` in production code
- All files under 600 LOC (limit: 800, target: 500)
- Pure Rust dependencies only (ecoBin compliant) + `blake3` for certification
- `thiserror`-based typed error hierarchy (`Result` propagation, no `process::exit`)
- Structured `Diagnostic` with `Severity` (Error/Warning) and `promote_to_error()`
- `LazyLock<Regex>` for all regex compilation (zero runtime panic paths)
- Parity integration tests: 6 tests validate petalTongue vs Zola output

## Subcommands

| Command | Purpose |
|---------|---------|
| `validate` | Registry fields, totals sums, taxonomy tags, content lint |
| `validate --check` | + shortcode integrity + internal link validation |
| `validate --strict` | Promote warnings to errors |
| `validate --verbose` | Full entity report (all fields consumed) |
| `graph [--emit]` | Build typed entity graph (renvois de choses), emit JSON |
| `certify [--emit]` | guideStone certification — BLAKE3 Merkle, manifest emit/validate |
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
├── error.rs      — thiserror Error enum + Diagnostic struct (Severity)
├── model.rs      — Typed entity model, Edge/EdgeRelation, Zola config parser
├── registry.rs   — Per-kind field validation
├── totals.rs     — Aggregate sum verification
├── content.rs    — Front matter taxonomy + shortcode checks
├── graph.rs      — Typed entity graph: build, validate, emit JSON
├── certify.rs    — guideStone: BLAKE3 Merkle root, manifest emit/validate
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

### Typed Entity Graph (`graph.rs` + `model.rs`)

```rust
pub struct Edge {
    pub target: String,
    pub relation: EdgeRelation,
    pub weight: Option<u8>,
}
```

14 `EdgeRelation` variants (ComposesInto, ValidatedBy, AnalogousTo, etc.) with
`inverse()` for automatic bidirectional graph construction. Implements the
Diderot → Bush → Nelson lineage of non-linguistic knowledge connections.

### guideStone Certification (`certify.rs`)

```rust
pub struct CertificationManifest {
    pub version: &'static str,
    pub generated: String,
    pub entity_count: usize,
    // ... counts, graph_merkle (BLAKE3), drift_tolerance
}
```

Computes a deterministic BLAKE3 Merkle root over sorted edge representations.
Same graph = same hash, regardless of iteration order.

### Capability-based Discovery

- `Source.origin`: explicit clone URL (supports Forgejo SSH, not just GitHub)
- `Source.private`: gated by `SPOREPRINT_REFRESH_PAT` env var
- `SPOREPRINT_FORGE_URL`: configurable forge (no GitHub assumption)
- `EntityKind::taxonomy_pairs()`: dynamic taxonomy discovery (no hardcoded names)
- `find_repo()`: filesystem walk (no hardcoded repo list)
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

## Dependencies (8 total, all pure Rust)

| Crate | Purpose |
|-------|---------|
| `blake3` | BLAKE3 hashing for certification Merkle root |
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
- [x] ~~petalTongue WASM: replace gonzales JS explorer with sovereign Rust/WASM~~ — Wave 67: petalTongue now has full document rendering pipeline (TOML front-matter → DocumentNode → multi-modal output)
- [x] Absorb `render_notebooks.sh` — superseded by `render-notebooks` subcommand
- [x] Entity graph with typed edges — `graph` subcommand (Wave 66)
- [x] guideStone certification — `certify` subcommand (Wave 66)
- [x] Capability-based discovery — no hardcoded repo lists or forge URLs

## Pure-Primal Rendering Path (Wave 67)

sporePrint content can now be rendered without Zola via the petalTongue pipeline:

```
content/*.md → content_render::parse_document()
  → DocumentNode tree (petal-tongue-scene::document)
  → document_compiler::compile_to_html() / compile_to_description()
  → ModalityOutput (HTML, accessible text, braille, audio)
```

The Nest Atomic composition (`sporeprint_composition.toml` in projectNUCLEUS)
orchestrates: NestGate (CAS) → petalTongue (render) → multi-modal output.

Zola remains as the validation oracle — `spore-validate certify` verifies
both rendering paths produce equivalent certification manifests.

### Local Validation (content-direct backend)

For local development and pre-deployment validation, petalTongue supports a
`content-direct` backend that bypasses NestGate and reads markdown directly from disk:

```bash
petaltongue web --backend content-direct --docroot ./content --port 8080
```

This loads the entity registry (66 entities from `config.toml`), builds the
navigation tree (11 sections from `_index.md` front matter), and serves all
pages through the DocumentNode pipeline with full entity shortcode resolution.

Parity is validated via `scripts/validate_parity.sh` (22 structural checks
against Zola reference output: content serving, entity resolution, modality
support, static assets, and heading-level structural comparison).
