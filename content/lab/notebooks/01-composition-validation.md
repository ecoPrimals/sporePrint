+++
title = "Composition Validation — airSpring"
description = "Rendered from 01-composition-validation.ipynb — live notebook from the ABG shared workspace"
date = 2026-05-21
weight = 50

[extra]
domain = "Lab"
rendered_from = "01-composition-validation.ipynb"
+++

<!-- Auto-generated from 01-composition-validation.ipynb by render_notebooks.sh -->
<!-- Re-render with: bash scripts/render_notebooks.sh --notebook /tmp/source-repo/notebooks/01-composition-validation.ipynb -->

<div class="cell border-box-sizing text_cell rendered"><div class="inner_cell">
<div class="text_cell_render border-box-sizing rendered_html">
<h1 id="Composition-Validation-%E2%80%94-airSpring">Composition Validation — airSpring<a class="anchor-link" href="#Composition-Validation-%E2%80%94-airSpring">¶</a></h1><p>airSpring is the ecological sciences validation spring in the ecoPrimals ecosystem.
It validates precision agriculture, irrigation science, and environmental systems
through 44 IPC capabilities across 87 experiments.</p>
<p><strong>Data sources</strong>: <code>composition_validation.json</code>, <code>test_suite_report.json</code></p>
<p><strong>Reproduce</strong>: <code>cargo run --release --bin validate_biome_graph</code> (35/35 PASS)</p>
<p><strong>For other springs</strong>: Replace capability categories and deploy graph names with your
domain. The pattern of niche.rs as canonical source → all deploy surfaces derive from
it eliminates drift.</p>
</div>
</div>
</div>
<div class="cell border-box-sizing code_cell rendered">
<div class="output_wrapper">
<div class="output">
<div class="output_area">
<div class="output_subarea output_stream output_stdout output_text">
<pre>Capabilities: 44 total, 44/44 routable
Deploy graphs: 4
Gaps: 9 open / 2 resolved
guideStone level: 0 → 1
MCP tools: 10
Tests: 1364 Rust + 1284 Python
</pre>
</div>
</div>
</div>
</div>
</div>
<div class="cell border-box-sizing text_cell rendered"><div class="inner_cell">
<div class="text_cell_render border-box-sizing rendered_html">
<h2 id="Capability-Distribution">Capability Distribution<a class="anchor-link" href="#Capability-Distribution">¶</a></h2><p>airSpring exposes 44 IPC capabilities organized by domain. The <code>niche.rs</code> module
is the single source of truth — deploy TOMLs and cell graphs derive from it.</p>
</div>
</div>
</div>
<div class="cell border-box-sizing code_cell rendered">
</div>
<div class="cell border-box-sizing text_cell rendered"><div class="inner_cell">
<div class="text_cell_render border-box-sizing rendered_html">
<h2 id="Deploy-Graph-Topology">Deploy Graph Topology<a class="anchor-link" href="#Deploy-Graph-Topology">¶</a></h2><p>airSpring defines 4 biomeOS deploy graphs for different composition patterns.</p>
</div>
</div>
</div>
<div class="cell border-box-sizing code_cell rendered">
</div>
<div class="cell border-box-sizing text_cell rendered"><div class="inner_cell">
<div class="text_cell_render border-box-sizing rendered_html">
<h2 id="Primal-Composition-&amp;-Gap-Status">Primal Composition &amp; Gap Status<a class="anchor-link" href="#Primal-Composition-&amp;-Gap-Status">¶</a></h2><p>airSpring's NUCLEUS composition wires 5 primals via IPC directly;
7 remain graph-level only (handled by biomeOS deployment).</p>
</div>
</div>
</div>
<div class="cell border-box-sizing code_cell rendered">
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
<td>IPC Capabilities</td>
<td>44/44 routable</td>
</tr>
<tr>
<td>Deploy Graphs</td>
<td>4 validated offline</td>
</tr>
<tr>
<td>Primals IPC-wired</td>
<td>5 (toadStool, barraCuda, biomeOS, NestGate, Squirrel)</td>
</tr>
<tr>
<td>Primals graph-level</td>
<td>7 (petalTongue, coralReef, BearDog, Songbird, rhizoCrypt, loamSpine, sweetGrass)</td>
</tr>
<tr>
<td>MCP Tools</td>
<td>10 (Squirrel-discoverable)</td>
</tr>
<tr>
<td>guideStone Level</td>
<td>0 → 1 (blocked on primalSpring dependency)</td>
</tr>
<tr>
<td>Open Gaps</td>
<td>9 (AG-001 through AG-011)</td>
</tr>
</tbody>
</table>
<p><strong>Provenance</strong>: airSpring v0.10.0 · AGPL-3.0-or-later · <a href="https://primals.eco">primals.eco</a></p>
</div>
</div>
</div>
