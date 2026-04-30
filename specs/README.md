# sporePrint specs/

Internal specifications and standards for sporePrint. This directory is **not** part of the published site — Zola only processes `content/`, so everything here is invisible to the build.

## Purpose

When an agent or human pulls sporePrint to evolve the site, `specs/` provides the context, constraints, and roadmap needed to make good decisions without reading every content file first.

**Read order for a new agent:**

1. `CONTEXT.md` — what sporePrint is, current state, structure, dependencies
2. `CONTENT_VOICE.md` — how to write: narrative standards, replication framing, metric conventions
3. `TAXONOMY_STANDARD.md` — how entity references and build-time validation work
4. `TEMPLATE_GUIDE.md` — how templates, layouts, and sections work
5. `CONTENT_MAP.md` — what exists, what may be stale, cross-section dependencies
6. `EVOLUTION_QUEUE.md` — what needs doing next

## Contents

| File | What it is | When to read |
|------|------------|-------------|
| `CONTEXT.md` | Project context — structure, state, dependencies | Always first |
| `CONTENT_VOICE.md` | Writing standards — voice, framing, metrics, PII rules | Before editing content |
| `TAXONOMY_STANDARD.md` | Taxonomy system — naming, registry, adding entities | Before adding entities or pages |
| `TEMPLATE_GUIDE.md` | Template architecture — layouts, sections, CSS | Before editing templates |
| `CONTENT_MAP.md` | What's in content/, staleness risks, dependencies | Before planning changes |
| `EVOLUTION_QUEUE.md` | Prioritized work queue | Before starting work |
| `RUST_TOOLING_VISION.md` | Rust validation tooling — `spore-validate` crate docs | When working on validation or metrics |

## Conventions

- Specs describe **intent and constraints**, not prose for the site
- Keep files short — working documents, not publications
- Use wateringHole standards language where applicable
- When a spec item is implemented, move it to the Resolved section in `EVOLUTION_QUEUE.md`
- When content structure changes, update `CONTENT_MAP.md`
- When technical infrastructure changes, update `CONTEXT.md`
