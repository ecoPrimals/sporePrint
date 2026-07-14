# spore-validate

Typed validation tooling for [sporePrint](https://primals.eco) — the public-facing science site for the ecoPrimals sovereign computing ecosystem.

## What it does

- **Entity registry validation** — checks that every `{{ entity(name="...") }}` shortcode in content resolves to a registered entity in `config.toml`
- **Content integrity** — generates `content-manifest.toml` with BLAKE3 hashes per page
- **Metric sync** — validates that aggregate stats (`[extra.totals]`) match computed sums from entity registry
- **Provenance extraction** — parses front matter for companions, trails, and voice metadata

## Usage

```bash
# Validate all content against entity registry
spore-validate check

# Generate content manifest with BLAKE3 hashes
spore-validate manifest

# Refresh entity metrics from registry
spore-validate refresh
```

## Part of ecoPrimals

This crate is part of the [ecoPrimals](https://github.com/ecoPrimals) sovereign scientific computing ecosystem. Pure Rust, zero C dependencies, AGPL-3.0-or-later.

## License

AGPL-3.0-or-later
