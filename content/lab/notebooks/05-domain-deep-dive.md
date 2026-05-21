+++
title = "Domain Deep Dive — airSpring"
description = "Rendered from 05-domain-deep-dive.ipynb — live notebook from the ABG shared workspace"
date = 2026-05-21
weight = 50

[extra]
domain = "Lab"
rendered_from = "05-domain-deep-dive.ipynb"
+++

<!-- Auto-generated from 05-domain-deep-dive.ipynb by render_notebooks.sh -->
<!-- Re-render with: bash scripts/render_notebooks.sh --notebook /tmp/source-repo/notebooks/05-domain-deep-dive.ipynb -->

<div class="cell border-box-sizing text_cell rendered"><div class="inner_cell">
<div class="text_cell_render border-box-sizing rendered_html">
<h1 id="Domain-Deep-Dive-%E2%80%94-airSpring">Domain Deep Dive — airSpring<a class="anchor-link" href="#Domain-Deep-Dive-%E2%80%94-airSpring">¶</a></h1><p>Michigan Crop Water Atlas (100 stations × 80 years), seasonal GPU pipeline,
and the path to Penny Irrigation — sovereign compute on consumer hardware.</p>
<p><strong>Data sources</strong>: <code>experiment_catalog.json</code>, <code>benchmark_timing.json</code>, <code>composition_validation.json</code></p>
<p><strong>Reproduce</strong>: <code>cargo run --release --bin validate_atlas</code> (1354/1354)</p>
<p><strong>For other springs</strong>: This notebook covers the domain-specific "crown jewel"
experiment. Replace with your flagship validation story. The frozen data pattern
captures the result without requiring live hardware.</p>
</div>
</div>
</div>
<div class="cell border-box-sizing code_cell rendered">
<div class="output_wrapper">
<div class="output">
<div class="output_area">
<div class="output_subarea output_stream output_stdout output_text">
<pre>Michigan Crop Water Atlas: 1,354 checks (active)
Atlas R²: N/A
Seasonal pipeline speedup: 125.0× (Python 250.0µs → Rust 2.0µs)
Atlas-scale throughput: 10,000,000 ET₀/s
</pre>
</div>
</div>
</div>
</div>
</div>
<div class="cell border-box-sizing text_cell rendered"><div class="inner_cell">
<div class="text_cell_render border-box-sizing rendered_html">
<h2 id="Seasonal-Pipeline:-ET%E2%82%80-%E2%86%92-Kc-%E2%86%92-Water-Balance-%E2%86%92-Yield">Seasonal Pipeline: ET₀ → Kc → Water Balance → Yield<a class="anchor-link" href="#Seasonal-Pipeline:-ET%E2%82%80-%E2%86%92-Kc-%E2%86%92-Water-Balance-%E2%86%92-Yield">¶</a></h2><p>The seasonal pipeline chains four stages: evapotranspiration (FAO-56 PM) →
crop coefficient (dual Kc with cover crops) → water balance (FAO-56 Ch 8) →
yield response (Stewart 1977). Each stage can run on CPU or GPU.</p>
</div>
</div>
</div>
<div class="cell border-box-sizing code_cell rendered">
</div>
<div class="cell border-box-sizing text_cell rendered"><div class="inner_cell">
<div class="text_cell_render border-box-sizing rendered_html">
<h2 id="ET%E2%82%80-Method-Comparison-(8-methods)">ET₀ Method Comparison (8 methods)<a class="anchor-link" href="#ET%E2%82%80-Method-Comparison-(8-methods)">¶</a></h2><p>airSpring validates 8 evapotranspiration methods against peer-reviewed baselines.</p>
</div>
</div>
</div>
<div class="cell border-box-sizing code_cell rendered">
</div>
<div class="cell border-box-sizing text_cell rendered"><div class="inner_cell">
<div class="text_cell_render border-box-sizing rendered_html">
<h2 id="Key-Experiments">Key Experiments<a class="anchor-link" href="#Key-Experiments">¶</a></h2>
</div>
</div>
</div>
<div class="cell border-box-sizing code_cell rendered">
<div class="output_wrapper">
<div class="output">
<div class="output_area">
<div class="output_subarea output_stream output_stdout output_text">
<pre>  ID  Status      Checks  Name
------------------------------------------------------------
   1  complete        95  FAO-56 PM ET₀
   5  complete    15,300  Real Data Pipeline (100 MI stations) (R²=0.967)
  18  active       1,354  Michigan Crop Water Atlas
  55  complete        78  GPU Live Dispatch
  72  complete        46  Pure GPU End-to-End
  76  complete        60  NUCLEUS Mesh Routing
  84  complete        21  CPU/GPU Parity (21/21 modules)
</pre>
</div>
</div>
</div>
</div>
</div>
<div class="cell border-box-sizing text_cell rendered"><div class="inner_cell">
<div class="text_cell_render border-box-sizing rendered_html">
<h2 id="The-Path-to-Penny-Irrigation">The Path to Penny Irrigation<a class="anchor-link" href="#The-Path-to-Penny-Irrigation">¶</a></h2><p>Penny Irrigation is the Garden-level product vision: sovereign irrigation
scheduling on consumer hardware ($600 GPU + $99 NPU). The pipeline:</p>
<pre><code>Open-Meteo weather → FAO-56 PM ET₀ → Dual Kc (cover crops) →
    Water balance → Yield prediction → Scheduling recommendation
</code></pre>
<p>All stages validated through 87 experiments. GPU pipeline delivers 6.8M
field-days/s on consumer hardware (RTX 4070 + AKD1000). The full NUCLEUS
composition deploys via biomeOS from pre-built plasmidBin binaries.</p>
<p><strong>Current state</strong>: Science validated (L2), primal composition readiness (L0→L1).
Next: guideStone scaffold, then Tier 2 IPC validation against live NUCLEUS.</p>
</div>
</div>
</div>
<div class="cell border-box-sizing text_cell rendered"><div class="inner_cell">
<div class="text_cell_render border-box-sizing rendered_html">
<h2 id="Summary">Summary<a class="anchor-link" href="#Summary">¶</a></h2><table>
<thead>
<tr>
<th>Metric</th>
<th>Value</th>
</tr>
</thead>
<tbody>
<tr>
<td>Michigan Atlas</td>
<td>100 stations × 80 years, R²=0.967</td>
</tr>
<tr>
<td>Atlas checks</td>
<td>1,354 (active experiment)</td>
</tr>
<tr>
<td>Seasonal pipeline</td>
<td>125× Python→Rust speedup</td>
</tr>
<tr>
<td>ET₀ methods</td>
<td>8 validated (FAO-56, HG, PT, TW, MK, TC, HM, BC)</td>
</tr>
<tr>
<td>Throughput</td>
<td>10M ET₀/s, 6.8M field-days/s</td>
</tr>
<tr>
<td>Papers reproduced</td>
<td>60 (Dong, Allen, FAO-56, van Genuchten, Stewart, ...)</td>
</tr>
<tr>
<td>Penny hardware</td>
<td>RTX 4070 ($600) + AKD1000 ($99) + i9</td>
</tr>
</tbody>
</table>
<p><strong>Provenance</strong>: airSpring v0.10.0 · MSU BAE (Dong lab) · <a href="https://primals.eco">primals.eco</a></p>
</div>
</div>
</div>
