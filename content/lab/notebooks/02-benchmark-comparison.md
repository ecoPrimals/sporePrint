+++
title = "Benchmark Comparison — hotSpring"
description = "Rendered from 02-benchmark-comparison.ipynb — live notebook from the ABG shared workspace"
date = 2026-05-25
weight = 50

[extra]
domain = "Lab"
rendered_from = "02-benchmark-comparison.ipynb"
+++

<!-- Auto-generated from 02-benchmark-comparison.ipynb by render_notebooks.sh -->
<!-- Re-render with: bash scripts/render_notebooks.sh --notebook /tmp/source-repo/notebooks/02-benchmark-comparison.ipynb -->

<div class="cell border-box-sizing text_cell rendered"><div class="inner_cell">
<div class="text_cell_render border-box-sizing rendered_html">
<h1 id="Benchmark-Comparison-%E2%80%94-hotSpring">Benchmark Comparison — hotSpring<a class="anchor-link" href="#Benchmark-Comparison-%E2%80%94-hotSpring">¶</a></h1><p>hotSpring's three-tier validation architecture (Python → Rust → NUCLEUS) produces
direct performance comparisons at every tier. This notebook visualizes Rust vs Python
speedups, GPU vs CPU acceleration, and the DF64 emulated double-precision breakthrough.</p>
<p><strong>Data sources:</strong> <code>benchmark_timing.json</code></p>
<p><strong>Reproduce:</strong> Individual benchmarks via <code>cargo bench</code> or <code>cargo run --release --bin &lt;benchmark&gt;</code></p>
<hr/>
<p><em>For other springs:</em> Replace physics benchmarks with your domain. The Rust vs Python
comparison pattern applies to any spring that migrated from scripting to compiled Rust.</p>
</div>
</div>
</div>
<div class="cell border-box-sizing code_cell rendered">
<div class="output_wrapper">
<div class="output">
<div class="output_area">
<div class="output_subarea output_stream output_stdout output_text">
<pre>Hardware: AMD Ryzen 9 7950X, NVIDIA RTX 4070 (12 GB)
Clean build: 85.2s
Test suite: 461.2s
Total science cost: $0.30
</pre>
</div>
</div>
</div>
</div>
</div>
<div class="cell border-box-sizing text_cell rendered"><div class="inner_cell">
<div class="text_cell_render border-box-sizing rendered_html">
<h2 id="Rust-vs-Python-%E2%80%94-Direct-Paper-Reproduction-Comparisons">Rust vs Python — Direct Paper Reproduction Comparisons<a class="anchor-link" href="#Rust-vs-Python-%E2%80%94-Direct-Paper-Reproduction-Comparisons">¶</a></h2><p>Every published paper reproduction in hotSpring was first implemented in Python
(Phase A baselines), then in Rust (Phase B-E), producing direct timing comparisons
on identical algorithms and datasets.</p>
</div>
</div>
</div>
<div class="cell border-box-sizing code_cell rendered">
<div class="output_wrapper">
<div class="output">
<div class="output_area">
<div class="output_subarea output_stream output_stderr output_text">
<pre>/tmp/ipykernel_2861/1195676791.py:34: UserWarning: set_ticklabels() should only be used with a fixed number of ticks, i.e. after set_ticks() or using a FixedLocator.
  axes[1].set_xticklabels(names, rotation=25, ha='right', fontsize=8)
</pre>
</div>
</div>
</div>
</div>
</div>
<div class="cell border-box-sizing text_cell rendered"><div class="inner_cell">
<div class="text_cell_render border-box-sizing rendered_html">
<h2 id="GPU-vs-CPU-%E2%80%94-Physics-Domain-Benchmarks">GPU vs CPU — Physics Domain Benchmarks<a class="anchor-link" href="#GPU-vs-CPU-%E2%80%94-Physics-Domain-Benchmarks">¶</a></h2><p>Consumer GPU hardware (RTX 4070) delivers 40-70x acceleration over CPU for
physics-heavy workloads. The key enabler is DF64 (emulated double precision
on FP32 cores), which delivers 3.24 TFLOPS — 5.6x over native FP64.</p>
</div>
</div>
</div>
<div class="cell border-box-sizing code_cell rendered">
<div class="output_wrapper">
<div class="output">
<div class="output_area">
<div class="output_subarea output_stream output_stderr output_text">
<pre>/tmp/ipykernel_2861/2554584232.py:31: UserWarning: set_ticklabels() should only be used with a fixed number of ticks, i.e. after set_ticks() or using a FixedLocator.
  axes[1].set_xticklabels(gb_names, rotation=25, ha='right', fontsize=8)
</pre>
</div>
</div>
</div>
</div>
</div>
<div class="cell border-box-sizing text_cell rendered"><div class="inner_cell">
<div class="text_cell_render border-box-sizing rendered_html">
<h2 id="Energy-and-Cost-Efficiency">Energy and Cost Efficiency<a class="anchor-link" href="#Energy-and-Cost-Efficiency">¶</a></h2><p>All 181 experiments ran on consumer hardware for a total science cost of $0.30.
Rust is 8.8x more energy-efficient than equivalent Python implementations.</p>
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
<th>Benchmark</th>
<th>Result</th>
</tr>
</thead>
<tbody>
<tr>
<td>Rust vs Python</td>
<td><strong>44.8x</strong> (SEMF), <strong>2274x</strong> (screening), <strong>283x</strong> (eigenvalue)</td>
</tr>
<tr>
<td>GPU vs CPU</td>
<td><strong>71.8x</strong> (HFB), <strong>54.4x</strong> (HMC), <strong>44.3x</strong> (gradient flow)</td>
</tr>
<tr>
<td>DF64 throughput</td>
<td><strong>3.24 TFLOPS</strong> (5.6x over native FP64)</td>
</tr>
<tr>
<td>Total science cost</td>
<td><strong>$0.30</strong> for 181 experiments</td>
</tr>
<tr>
<td>Energy efficiency</td>
<td><strong>8.8x</strong> more efficient than Python</td>
</tr>
</tbody>
</table>
<hr/>
<p><strong>Provenance:</strong> All benchmarks from <code>experiments/results/benchmark_timing.json</code>.<br/>
<strong>Hardware:</strong> AMD Ryzen 9 7950X, NVIDIA RTX 4070, Pop!_OS 22.04.<br/>
<strong>Source:</strong> <a href="https://github.com/syntheticChemistry/hotSpring">hotSpring on GitHub</a> · <a href="https://primals.eco/lab/springs/hotspring/">primals.eco</a></p>
</div>
</div>
</div>
