# sporePrint specs/

Internal specifications and evolution plans for sporePrint. This directory
is **not** part of the published site — Zola only processes `content/`,
so everything here is invisible to the build.

## Purpose

When an agent or human pulls sporePrint to evolve the site, `specs/`
provides the context, constraints, and roadmap needed to make good
decisions without reading every content file first.

## Contents

| File | What it is |
|------|------------|
| `CONTEXT.md` | AI-ingestible project context (<150 lines). Read this first. |
| `CONTENT_MAP.md` | What exists in `content/`, how sections relate, what's stale |
| `EVOLUTION_QUEUE.md` | Planned changes, ordered by priority |

## Convention

- Specs describe *intent and constraints*, not prose for the site
- Keep files short — these are working documents, not publications
- Use wateringHole standards language where applicable
- When a spec is implemented, move the relevant items to `CHANGELOG.md`
