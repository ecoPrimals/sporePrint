+++
title = "Ecosystem Visualization"
description = "Interactive entity graph explorer for the ecoPrimals ecosystem"
weight = 2
[extra]
foundation = true
entity_id = "ecosystem-visualization"
primal_emoji = "🌐🔭"
maturity = "architectural"
+++

# Ecosystem Visualization

The **Entity Graph Explorer** renders all 66 entities in the ecoPrimals ecosystem
as a force-directed graph. Nodes are colored by kind (primal, spring, product,
composition, concept, infrastructure, organization) and sized by connectivity.

## Entity Graph Explorer

<div id="viz-entity-graph" class="viz-container">
{{ viz_embed(src="/viz/entity-graph", caption="Entity graph: 66 entities with typed relationships across 15 primals and 9 springs") }}
</div>

### Reading the Graph

- **Green nodes** — Primals (core capabilities)
- **Blue nodes** — Springs (lifecycle management)
- **Pink nodes** — Products (user-facing applications)
- **Yellow nodes** — Compositions (atomic combinations)
- **Purple nodes** — Concepts (patterns and philosophies)
- **Teal nodes** — Infrastructure (tooling, repos)
- **Peach nodes** — Organizations

### Interactions (when WASM is available)

- **Scroll** to zoom in/out
- **Click and drag** to pan the view
- **Click a node** to highlight it and its connections
- **Double-click** to reset the viewport

### Accessibility

This visualization is available in multiple modalities:
- **SVG** (visual, default) — scalable vector rendering
- **Description** — full textual description for screen readers
- **Scene JSON** — raw scene graph for programmatic access

Access alternative modalities via query parameters:
`/viz/entity-graph?format=description`

## Data Source

The graph is built from `static/graph/entity-graph.json`, which is
Merkle-certified as part of sporePrint's guideStone system. The JSON
contains 66 nodes and 126 edges representing relationships like
"composes into", "validated by", "dispatches", "stores for", etc.

<script type="module" src="/js/viz-hydrate.js"></script>
