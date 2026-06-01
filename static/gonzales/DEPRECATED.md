# gonzales/ — DEPRECATED (Jelly String)

**Status:** Vestigial. Superseded by petalTongue viz system (Wave 68+).

## What this was

A client-side Plotly.js explorer for Gonzales experiment data (dermatitis,
hormesis, tissue geometry, cross-species). It loaded JSON datasets and rendered
interactive charts in the browser.

## Why it's deprecated

- petalTongue now provides server-rendered SVG visualizations with WASM
  progressive enhancement (see `/viz/*` routes)
- The grammar of graphics engine in petalTongue replaces Plotly for all
  chart needs
- Client-side-only rendering excludes blind/non-visual users; petalTongue
  serves description modality natively

## Timeline

- **Wave 68** (current): Marked deprecated, still served for historical links
- **Wave 70**: Data migrated to entity-graph or NestGate CAS
- **Wave 72**: Files removed; any remaining links redirect to petalTongue equivalents

## Migration path

The JSON data files will be absorbed into the entity registry or served via
NestGate CAS. Interactive exploration moves to petalTongue's grammar of
graphics with `?format=scene-json` hydration.
