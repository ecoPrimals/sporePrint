+++
title = "Ecosystem Evidence — airSpring"
description = "Rendered from 03-ecosystem-evidence.ipynb"
date = 2026-05-31
weight = 50

[extra]
domain = "Lab"
rendered_from = "03-ecosystem-evidence.ipynb"
+++

<!-- Auto-generated from 03-ecosystem-evidence.ipynb by render_notebooks.sh -->
<!-- Preferred: spore-validate render-notebooks (pure Rust) -->

<div class="cell border-box-sizing text_cell rendered"><div class="inner_cell">
<div class="text_cell_render border-box-sizing rendered_html">
<h1 id="Ecosystem-Evidence-%E2%80%94-airSpring">Ecosystem Evidence — airSpring<a class="anchor-link" href="#Ecosystem-Evidence-%E2%80%94-airSpring">¶</a></h1><p>87 experiments validating precision agriculture and irrigation science.
1,284 Python baselines → 1,364 Rust tests → 91 validation binaries.
60 named tolerances with full provenance tracking.</p>
<p><strong>Data sources</strong>: <code>experiment_catalog.json</code>, <code>test_suite_report.json</code>, <code>security_convergence.json</code></p>
<p><strong>Reproduce</strong>: <code>cargo test --lib &amp;&amp; cargo test --tests --all-features</code></p>
<p><strong>For other springs</strong>: Replace experiment categories with your domain areas.
The pattern of categorized experiments with check counts and named tolerances
applies universally.</p>
</div>
</div>
</div>
<div class="cell border-box-sizing code_cell rendered">
<div class="output_wrapper">
<div class="output">
<div class="output_area">
<div class="output_subarea output_stream output_stdout output_text">
<pre>Total experiments: 87
  Complete: 86
  Active: 1
Categories: 12
Tolerances: 60 named, 5 submodules
</pre>
</div>
</div>
</div>
</div>
</div>
<div class="cell border-box-sizing text_cell rendered"><div class="inner_cell">
<div class="text_cell_render border-box-sizing rendered_html">
<h2 id="Experiment-Distribution-by-Category">Experiment Distribution by Category<a class="anchor-link" href="#Experiment-Distribution-by-Category">¶</a></h2>
</div>
</div>
</div>
<div class="cell border-box-sizing code_cell rendered">
</div>
<div class="cell border-box-sizing text_cell rendered"><div class="inner_cell">
<div class="text_cell_render border-box-sizing rendered_html">
<h2 id="Test-Suite-Composition">Test Suite Composition<a class="anchor-link" href="#Test-Suite-Composition">¶</a></h2>
</div>
</div>
</div>
<div class="cell border-box-sizing code_cell rendered">
</div>
<div class="cell border-box-sizing text_cell rendered"><div class="inner_cell">
<div class="text_cell_render border-box-sizing rendered_html">
<h2 id="Quality-Gates-&amp;-Safety">Quality Gates &amp; Safety<a class="anchor-link" href="#Quality-Gates-&amp;-Safety">¶</a></h2>
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
<td>Experiments</td>
<td>87 (86 complete, 1 active)</td>
</tr>
<tr>
<td>Python baselines</td>
<td>1,284 checks</td>
</tr>
<tr>
<td>Rust tests</td>
<td>1,364 (986 lib + 316 integration + 62 forge)</td>
</tr>
<tr>
<td>Validation binaries</td>
<td>91 (all zero-panic)</td>
</tr>
<tr>
<td>Line coverage</td>
<td>90.56% (gated at 90%)</td>
</tr>
<tr>
<td>Named tolerances</td>
<td>60 in 5 submodules (Python mirror)</td>
</tr>
<tr>
<td>Quality gates</td>
<td>12/12 PASS</td>
</tr>
<tr>
<td>Provenance baselines</td>
<td>63 registered</td>
</tr>
</tbody>
</table>
<p><strong>Provenance</strong>: airSpring v0.10.0 · AGPL-3.0-or-later · <a href="https://primals.eco">primals.eco</a></p>
</div>
</div>
</div>
