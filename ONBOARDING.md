# Onboarding a New Repo to sporePrint Auto-Refresh

When a new primal, spring, or product is created, follow these steps to
connect it to the auto-refresh pipeline so primals.eco metrics stay current.

## Pipeline Architecture

**Primary (Sovereign CI — Wave 120+):** Gate pushes to Forgejo → golgi quorum
timer → cascade propagates → sporePrint rebuilds via Sovereign CI on sporeGate.

**Shadow (GitHub Actions — trailing, will be archived):** GitHub repo push →
`notify-sporeprint.yml` → sporePrint `auto-refresh.yml` → metric refresh →
deploy to GitHub Pages.

## Prerequisites

- GitHub repo exists (in `ecoPrimals/`, `syntheticChemistry/`, or `sporeGarden/`)
- You have a PAT with `repo` scope (or the existing `SPOREPRINT_DISPATCH_TOKEN`)
- The repo contains Rust code (for metrics: LOC, tests, files, crates)

## Steps

### 1. Add to `sources.toml`

Add an entry mapping the entity ID to its GitHub repo:

```toml
[sources.newprimal]
repo = "ecoPrimals/newPrimal"
type = "primal"        # or "spring" or "product"
# private = true       # add for private repos
```

### 2. Add to `config.toml` entity registry

Add a `[extra.entity_registry.<id>]` entry. Metrics fields (`loc`, `tests`,
`files`, `crates`) can be omitted — auto-refresh will populate them on first
run.

```toml
[extra.entity_registry.newprimal]
display = "newPrimal"
emoji = "🆕"
kind = "primal"        # or "spring" or "product"
description = "Short description of what this does"
domain = "Domain category"
repo = "ecoPrimals/newPrimal"
# tier = "foundation"  # primals only: foundation, post-nucleus, meta, tooling
```

### 3. Install the workflow

Copy the template from `plasmidBin/templates/notify-sporeprint.yml` to the
new repo:

```bash
mkdir -p /path/to/newPrimal/.github/workflows

# For primals:
sed 's/"type": "spring"/"type": "primal"/' \
  plasmidBin/templates/notify-sporeprint.yml \
  > /path/to/newPrimal/.github/workflows/notify-sporeprint.yml

# For springs: copy as-is (template default is "spring")
# For products: sed 's/"type": "spring"/"type": "product"/'
```

If the repo uses `master` instead of `main`, the template already triggers
on both (`branches: [main, master]`).

### 4. Set the secret

```bash
gh secret set SPOREPRINT_DISPATCH_TOKEN \
  --repo org/newPrimal \
  --body "$(gh secret get SPOREPRINT_DISPATCH_TOKEN --repo ecoPrimals/sporePrint 2>/dev/null || echo 'YOUR_PAT_HERE')"
```

Or set the same PAT used by all other repos.

### 5. Commit and push

```bash
cd /path/to/newPrimal
git add .github/workflows/notify-sporeprint.yml
git commit -m "Add notify-sporeprint.yml for auto-refresh on primals.eco"
git push origin main
```

The push triggers `notify-sporeprint.yml` → sporePrint `auto-refresh.yml` →
clones the repo → `spore-validate refresh --write` → populates metrics in
`config.toml` → `deploy.yml` rebuilds primals.eco.

### 6. Commit sporePrint changes

Push the `sources.toml` and `config.toml` changes to sporePrint:

```bash
cd infra/sporePrint
git add sources.toml config.toml
git commit -m "onboard: add newPrimal to auto-refresh"
git push origin main
```

## Verification

After the first push from the new repo, check that auto-refresh ran:

```bash
# Check GitHub Actions on sporePrint
gh run list --repo ecoPrimals/sporePrint --workflow auto-refresh.yml --limit 3

# Or manually trigger a refresh
gh workflow run auto-refresh.yml --repo ecoPrimals/sporePrint \
  -f source=newprimal
```

## What Gets Updated

| Field | Source | Auto-updated? |
|-------|--------|--------------|
| `loc` / `loc_display` | Rust source line count | Yes |
| `tests` / `tests_display` | `#[test]` annotation count | Yes |
| `files` | `.rs` file count | Yes |
| `crates` | `Cargo.toml` count | Yes |
| `[extra.totals]` | Aggregate across all entities | Yes |
| `display`, `emoji`, `description` | Manual | No |
| `capabilities` | Manual | No |
