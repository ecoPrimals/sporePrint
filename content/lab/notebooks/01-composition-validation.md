+++
title = "Composition Validation — hotSpring"
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
<h1 id="Composition-Validation-%E2%80%94-hotSpring">Composition Validation — hotSpring<a class="anchor-link" href="#Composition-Validation-%E2%80%94-hotSpring">¶</a></h1><p>hotSpring validates computational physics (lattice QCD, nuclear structure, plasma)
on consumer GPU hardware via the ecoPrimal NUCLEUS composition. This notebook shows
the deploy graph topology, guideStone Level 6 validation, and capability-based
primal routing.</p>
<p><strong>Data sources:</strong> <code>composition_validation.json</code>, <code>test_suite_report.json</code></p>
<p><strong>Reproduce:</strong> <code>cargo test --lib</code> in <code>barracuda/</code>, then <code>scripts/validate-primal-proof.sh</code></p>
<hr/>
<p><em>For other springs:</em> Replace QCD domain content with your science. Keep the guideStone
property structure and atomic type hierarchy — they're universal across all springs.</p>
</div>
</div>
</div>
<div class="cell border-box-sizing code_cell rendered">
<div class="output_wrapper">
<div class="output">
<div class="output_area">
<div class="output_subarea output_stream output_stdout output_text">
<pre>Spring: hotSpring v0.6.32
guideStone Level: 5
Deploy graph: 11 nodes, 9 required primals
Tests: 1036 passed, 6 ignored
Bare guideStone: 30/30 checks
</pre>
</div>
</div>
</div>
</div>
</div>
<div class="cell border-box-sizing text_cell rendered"><div class="inner_cell">
<div class="text_cell_render border-box-sizing rendered_html">
<h2 id="guideStone-Level-6-%E2%80%94-Five-Properties">guideStone Level 6 — Five Properties<a class="anchor-link" href="#guideStone-Level-6-%E2%80%94-Five-Properties">¶</a></h2><p>The <code>hotspring_guidestone</code> binary validates 5 guideStone properties in bare mode
(no primals needed) and adds NUCLEUS IPC parity checks when primals are deployed.
Property 3 (Self-Verifying) uses BLAKE3 checksums for 15 validation-critical source files.</p>
</div>
</div>
</div>
<div class="cell border-box-sizing code_cell rendered">
</div>
<div class="cell border-box-sizing text_cell rendered"><div class="inner_cell">
<div class="text_cell_render border-box-sizing rendered_html">
<h2 id="Capability-Based-Routing">Capability-Based Routing<a class="anchor-link" href="#Capability-Based-Routing">¶</a></h2><p>hotSpring routes to primals by <strong>capability domain</strong> (<code>by_domain("compute")</code>), not
by hardcoded process names. All requirements derive from <code>niche::DEPENDENCIES</code> —
a single source of truth. Named accessors are deprecated.</p>
</div>
</div>
</div>
<div class="cell border-box-sizing code_cell rendered">
</div>
<div class="cell border-box-sizing text_cell rendered"><div class="inner_cell">
<div class="text_cell_render border-box-sizing rendered_html">
<h2 id="Test-Suite-by-Physics-Domain">Test Suite by Physics Domain<a class="anchor-link" href="#Test-Suite-by-Physics-Domain">¶</a></h2><p>596 (default) / 1,045 (barracuda-local) library tests organized by physics domain — from nuclear structure (SEMF, HFB)
through lattice QCD (HMC, RHMC, gradient flow) to GPU compute validation.</p>
</div>
</div>
</div>
<div class="cell border-box-sizing code_cell rendered">
</div>
<div class="cell border-box-sizing text_cell rendered"><div class="inner_cell">
<div class="text_cell_render border-box-sizing rendered_html">
<h2 id="Validation-Summary">Validation Summary<a class="anchor-link" href="#Validation-Summary">¶</a></h2><table>
<thead>
<tr>
<th>Component</th>
<th>Status</th>
<th>Detail</th>
</tr>
</thead>
<tbody>
<tr>
<td>guideStone Level 6</td>
<td><strong>30/30 PASS</strong></td>
<td>5 properties certified, BLAKE3 P3, 3 SKIP (liveness)</td>
</tr>
<tr>
<td>Library tests</td>
<td><strong>596/596 PASS</strong> (default) / <strong>1,045</strong> (barracuda-local)</td>
<td>6 GPU-heavy ignored (upstream barraCuda CI)</td>
</tr>
<tr>
<td>Validation suites</td>
<td><strong>65/65 PASS</strong></td>
<td>167 <code>validate_*</code> binaries + <code>hotspring_guidestone</code></td>
</tr>
<tr>
<td>NUCLEUS routing</td>
<td><strong>by_domain()</strong></td>
<td>Capability-based from <code>niche::DEPENDENCIES</code></td>
</tr>
<tr>
<td>Deploy graph</td>
<td><strong>11 nodes</strong></td>
<td>9 required + 1 optional + hotspring_unibin</td>
</tr>
</tbody>
</table>
<hr/>
<p><strong>Provenance:</strong> All data from <code>experiments/results/</code> committed JSON artifacts.<br/>
<strong>Reproduce:</strong> <code>cargo test --lib</code> in <code>barracuda/</code>, <code>scripts/validate-primal-proof.sh</code> from repo root.<br/>
<strong>Source:</strong> <a href="https://github.com/syntheticChemistry/hotSpring">hotSpring on GitHub</a> · <a href="https://primals.eco/lab/springs/hotspring/">primals.eco</a></p>
</div>
</div>
</div>
