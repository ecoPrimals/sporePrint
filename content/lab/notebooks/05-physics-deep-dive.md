+++
title = "Physics Deep Dive — hotSpring"
description = "Rendered from 05-physics-deep-dive.ipynb"
date = 2026-05-31
weight = 50

[extra]
domain = "Lab"
rendered_from = "05-physics-deep-dive.ipynb"
+++

<!-- Auto-generated from 05-physics-deep-dive.ipynb by render_notebooks.sh -->
<!-- Preferred: spore-validate render-notebooks (pure Rust) -->

<div class="cell border-box-sizing text_cell rendered"><div class="inner_cell">
<div class="text_cell_render border-box-sizing rendered_html">
<h1 id="Physics-Deep-Dive-%E2%80%94-hotSpring">Physics Deep Dive — hotSpring<a class="anchor-link" href="#Physics-Deep-Dive-%E2%80%94-hotSpring">¶</a></h1><p>hotSpring's most compelling domain contribution: first-principles nuclear structure
and lattice QCD on consumer GPU hardware. This notebook dives into the physics
validation arc, sovereign GPU pipeline, and the guideStone security posture.</p>
<p><strong>Data sources:</strong> <code>security_convergence.json</code>, <code>benchmark_timing.json</code>, <code>test_suite_report.json</code></p>
<p><strong>Reproduce:</strong> <code>cargo test --lib</code> in <code>barracuda/</code>, individual <code>validate_*</code> binaries.</p>
<hr/>
<p><em>For other springs:</em> This is hotSpring's unique domain notebook. Replace with your
most compelling discovery — the one that justifies your spring's existence in the ecosystem.</p>
</div>
</div>
</div>
<div class="cell border-box-sizing code_cell rendered">
<div class="output_wrapper">
<div class="output">
<div class="output_area">
<div class="output_subarea output_stream output_stdout output_text">
<pre>guideStone Level: 5
Bare checks: 30/30
BTSP Phase 3: 13/13
Gaps: 32 resolved / 8 active / 43 total
</pre>
</div>
</div>
</div>
</div>
</div>
<div class="cell border-box-sizing text_cell rendered"><div class="inner_cell">
<div class="text_cell_render border-box-sizing rendered_html">
<h2 id="Three-Tier-Validation-Arc">Three-Tier Validation Arc<a class="anchor-link" href="#Three-Tier-Validation-Arc">¶</a></h2><p>hotSpring's validation architecture stacks three tiers:</p>
<ol>
<li><strong>Python baselines</strong> — Published paper reproductions (Phase A-E)</li>
<li><strong>Rust validation</strong> — Same algorithms, GPU-accelerated (<code>cargo test --lib</code>)</li>
<li><strong>NUCLEUS IPC</strong> — Primal composition validates IPC matches direct Rust</li>
</ol>
<p>Each tier must agree before a science claim is trusted in production.</p>
</div>
</div>
</div>
<div class="cell border-box-sizing code_cell rendered">
</div>
<div class="cell border-box-sizing text_cell rendered"><div class="inner_cell">
<div class="text_cell_render border-box-sizing rendered_html">
<h2 id="Sovereign-GPU-Pipeline">Sovereign GPU Pipeline<a class="anchor-link" href="#Sovereign-GPU-Pipeline">¶</a></h2><p>hotSpring's sovereign GPU pipeline replaces nouveau with a pure Rust path:
VFIO → SovereignInit (8 stages) → native SASS/GFX compilation → dispatch.
Validated across 3 GPU generations: K80 (Kepler/SM35), Titan V (Volta/SM70),
RTX 5060 (Blackwell/SM120).</p>
</div>
</div>
</div>
<div class="cell border-box-sizing code_cell rendered">
</div>
<div class="cell border-box-sizing text_cell rendered"><div class="inner_cell">
<div class="text_cell_render border-box-sizing rendered_html">
<h2 id="guideStone-Security-Posture">guideStone Security Posture<a class="anchor-link" href="#guideStone-Security-Posture">¶</a></h2><p>hotSpring's guideStone Level 5 certification validates 5 properties.
As a NUCLEUS consumer, BTSP Phase 3 security is inherited from the
13 primals it composes with.</p>
</div>
</div>
</div>
<div class="cell border-box-sizing code_cell rendered">
</div>
<div class="cell border-box-sizing text_cell rendered"><div class="inner_cell">
<div class="text_cell_render border-box-sizing rendered_html">
<h2 id="Code-Safety-Assessment">Code Safety Assessment<a class="anchor-link" href="#Code-Safety-Assessment">¶</a></h2><p>hotSpring enforces strict code safety at the library level while allowing
controlled unsafe in feature-gated GPU binaries.</p>
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
<th>Domain</th>
<th>Highlight</th>
</tr>
</thead>
<tbody>
<tr>
<td>Nuclear EOS</td>
<td><strong>2,042 AME2020 nuclei</strong>, L1-L3 on single RTX 4070, 1,990 novel predictions</td>
</tr>
<tr>
<td>Lattice QCD</td>
<td><strong>SU(3) HMC/RHMC</strong>, gradient flow, beta-scan deconfinement at β=5.69</td>
</tr>
<tr>
<td>Sovereign GPU</td>
<td><strong>3 generations</strong> (Kepler/Volta/Blackwell), 8-stage SovereignInit, zero nouveau</td>
</tr>
<tr>
<td>DF64</td>
<td><strong>3.24 TFLOPS</strong> emulated double precision on FP32 cores</td>
</tr>
<tr>
<td>guideStone</td>
<td><strong>Level 5</strong>, 30/30 bare, BLAKE3 P3, 5/5 properties</td>
</tr>
<tr>
<td>Code safety</td>
<td><strong>forbid(unsafe)</strong> in lib, deny.toml, zero dyn dispatch, zero #[allow]</td>
</tr>
<tr>
<td>PRIMAL_GAPS</td>
<td><strong>32/43 resolved</strong>, 8 active, 3 blocked upstream</td>
</tr>
</tbody>
</table>
<hr/>
<p><strong>Provenance:</strong> All data from <code>experiments/results/</code> committed JSON artifacts.<br/>
<strong>Papers:</strong> 22 published papers reproduced — see <code>specs/PAPER_REVIEW_QUEUE.md</code>.<br/>
<strong>Source:</strong> <a href="https://github.com/syntheticChemistry/hotSpring">hotSpring on GitHub</a> · <a href="https://primals.eco/lab/springs/hotspring/">primals.eco</a></p>
</div>
</div>
</div>
