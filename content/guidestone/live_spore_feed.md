+++
title = "Live Spore Feed"
description = "Automated provenance feed from guideStone deployment artifacts. Each liveSpore.json records every machine that ran a validated artifact, building a reproducibility chain across substrates."
date = 2026-05-15
weight = 3

[taxonomies]
primals = ["nestgate", "sweetgrass", "loamspine"]
springs = ["hotspring", "groundspring", "wetspring"]
+++

## What This Is

The Live Spore Feed is the automated pipeline that ingests
`liveSpore.json` provenance records from {{ entity(name="guidestone") }}
deployment artifacts and publishes them to primals.eco.

Each time a {{ entity(name="guidestone") }} artifact runs on a new machine,
it appends a provenance record: hostname hash, architecture, GPU, check
counts, wall time. This data feeds into {{ entity(name="sweetgrass") }}
attribution chains and {{ entity(name="loamspine") }} permanence ledgers.

---

## Data Endpoint

The latest `liveSpore.json` is served as raw JSON:

```
https://sporeprint.primals.eco/lab/guidestone/liveSpore.json
```

This endpoint updates automatically when upstream guideStone repos push
new validation runs.

---

## Pipeline

The feed follows sporePrint's standard dispatch pipeline with a
guideStone-specific extension:

1. A guideStone repo pushes `liveSpore.json` after validation runs complete.
2. The repo's `notify-sporeprint.yml` dispatches a `source-updated` event
   with `type: "guidestone"` and `content: "true"`.
3. sporePrint's `auto-refresh.yml` content job clones the source and
   copies `liveSpore.json` to `static/lab/guidestone/`.
4. If `litho verify` is available in CI, it runs as a defense-in-depth
   check (non-blocking — the producing repo is the verification authority).
5. A PR is created. On merge, `deploy.yml` publishes to primals.eco.

### Active GuideStone Feeds

| Artifact | Repository | Modules | Checks |
|----------|-----------|---------|--------|
| hotSpring-guideStone | syntheticChemistry/hotSpring | 3 physics papers | 59/59 |
| {{ entity(name="lithospore") }} | sporeGarden/lithoSpore | 7 LTEE modules | 75/75 |

---

## Schema

Each `liveSpore.json` contains an array of run records:

```json
{
  "runs": [
    {
      "timestamp": "2026-03-15T14:22:00Z",
      "hostname_hash": "a3b8...",
      "arch": "x86_64",
      "gpu": "NVIDIA RTX 3090",
      "checks_passed": 59,
      "checks_total": 59,
      "wall_time_seconds": 47.2
    }
  ]
}
```

Fields are documented in [Deployment Artifacts](@/guidestone/deployment_artifacts.md).

---

## For GuideStone Producers

To wire your guideStone repo into this feed:

1. **Add `notify-sporeprint.yml`** to `.github/workflows/` (template in
   `plasmidBin/templates/notify-sporeprint.yml`). Set `content: "true"`
   and `type: "guidestone"` in the payload.

2. **Ensure `liveSpore.json` is at repo root** — the pipeline looks for
   it at `/tmp/source-repo/liveSpore.json` after clone.

3. **Optionally add `sporeprint/` directory** with Zola-compatible markdown
   pages that will be imported as lab content alongside the JSON feed.
