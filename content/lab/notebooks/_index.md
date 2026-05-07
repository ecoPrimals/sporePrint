+++
title = "Public Notebooks"
description = "Interactive Jupyter notebooks that visualize baseCamp science — frozen experiment data, benchmark comparisons, paper reproductions. Run them yourself or review the rendered evidence."
sort_by = "weight"
template = "section.html"
+++

Public-facing notebooks from the spring baseCamp science. Each notebook
loads frozen experiment data (JSON artifacts committed to the spring
repositories) and visualizes the validation evidence.

**No live primals required** — these notebooks work with frozen data.
When the composition is running, the same notebooks can dispatch live
workloads via ToadStool (see the Tier 2 stubs in each notebook).

## How to Run

1. Clone the spring repository (e.g. [wetSpring](https://github.com/syntheticChemistry/wetSpring))
2. `cd notebooks/`
3. `jupyter lab` (or use JupyterHub on ironGate)
4. Run all cells — data loads from `../experiments/results/`

## Rendering Pipeline

These pages are rendered from `.ipynb` files via `render_notebooks.sh`.
The script extracts HTML from executed notebooks and wraps them in
Zola front matter for static site generation.

To re-render: `bash scripts/render_notebooks.sh --all`
