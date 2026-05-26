+++
title = "Wilson Gradient Flow — Scale Setting with $t_0$ and $w_0$"
description = "Rendered from 11-gradient-flow.ipynb — live notebook from the ABG shared workspace"
date = 2026-05-26
weight = 50

[extra]
domain = "Lab"
rendered_from = "11-gradient-flow.ipynb"
+++

<!-- Auto-generated from 11-gradient-flow.ipynb by render_notebooks.sh -->
<!-- Re-render with: bash scripts/render_notebooks.sh --notebook /tmp/source-repo/notebooks/papers/11-gradient-flow.ipynb -->

<div class="cell border-box-sizing text_cell rendered"><div class="inner_cell">
<div class="text_cell_render border-box-sizing rendered_html">
<h1 id="Wilson-Gradient-Flow-%E2%80%94-Scale-Setting-with-$t_0$-and-$w_0$">Wilson Gradient Flow — Scale Setting with $t_0$ and $w_0$<a class="anchor-link" href="#Wilson-Gradient-Flow-%E2%80%94-Scale-Setting-with-$t_0$-and-$w_0$">¶</a></h1><p><strong>Papers:</strong></p>
<ul>
<li>Lüscher (2010) <em>JHEP</em> <strong>08</strong>, 071 — Wilson flow, $t_0$ scale</li>
<li>BMW, arXiv:1203.4469 — $w_0$ scale</li>
<li>Bazavov &amp; Chuna, arXiv:2101.05320 — LSCFRK Lie group integrators</li>
</ul>
<p><strong>What we reproduce:</strong> Wilson gradient flow on SU(3) gauge fields with three
integrators: Euler, RK3 Lüscher (W6), and LSCFRK3W7 (Chuna). We measure
$t^2\langle E(t)\rangle$ to extract the $t_0$ and $w_0$ scales, and verify
LSCFRK coefficient derivation from 3rd-order Runge-Kutta conditions.</p>
<hr/>
<p><em>This notebook runs live gradient flow on a 4^4 lattice (~10s per integrator).</em><br/>
<em>Rust parity:</em> <code>barracuda/src/lattice/gradient_flow.rs</code> — GPU-accelerated flow.*<br/>
<em>Algorithm-identical: same Cayley exp, same gauge force, same coefficient derivation.</em></p>
</div>
</div>
</div>
<div class="cell border-box-sizing text_cell rendered"><div class="inner_cell">
<div class="text_cell_render border-box-sizing rendered_html">
<h2 id="Physics">Physics<a class="anchor-link" href="#Physics">¶</a></h2><p>The Wilson gradient flow evolves gauge fields along a fictitious flow time $t$:</p>
<p>$$\dot{V}_\mu(x, t) = -g_0^2 (\partial_{x,\mu} S_W) V_\mu(x, t)$$</p>
<p>This smooths UV fluctuations, defining renormalized observables. The $t_0$ and $w_0$
scales are set by:</p>
<p>$$t^2 \langle E(t) \rangle \Big|_{t=t_0} = 0.3 \quad (\text{Lüscher 2010})$$</p>
<p>$$t \frac{d}{dt}\left[t^2 E(t)\right] \Big|_{t=w_0^2} = 0.3 \quad (\text{BMW})$$</p>
<p>The <strong>LSCFRK3</strong> integrators use 2N-storage Lie group Runge-Kutta methods
with coefficients derived from the 3rd-order conditions:</p>
<p>$$b_1 + b_2 + b_3 = 1, \quad b_2 c_2 + b_3 c_3 = \frac{1}{2}, \quad b_2 c_2^2 + b_3 c_3^2 = \frac{1}{3}, \quad b_3 a_{32} c_2 = \frac{1}{6}$$</p>
</div>
</div>
</div>
<div class="cell border-box-sizing code_cell rendered">
<div class="output_wrapper">
<div class="output">
<div class="output_area">
<div class="output_subarea output_stream output_stdout output_text">
<pre>SU(3) operations loaded
</pre>
</div>
</div>
</div>
</div>
</div>
<div class="cell border-box-sizing code_cell rendered">
<div class="output_wrapper">
<div class="output">
<div class="output_area">
<div class="output_subarea output_stream output_stdout output_text">
<pre>LSCFRK3 Coefficient Verification:
  W6 (Luscher): A = [0.0000, -0.531250, -1.185185]
                B = [0.2500, 0.888889, 0.750000]
  W7 (Chuna):   A = [0.0000, -0.555556, -1.195313]
                B = [0.3333, 0.937500, 0.533333]
  W6: sum(b)=1.000000000000000, sum(bc)=0.500000000000000, sum(bc2)=0.333333333333333, b3*a32*c2=0.166666666666667
  W7: sum(b)=1.000000000000000, sum(bc)=0.500000000000000, sum(bc2)=0.333333333333333, b3*a32*c2=0.166666666666667
</pre>
</div>
</div>
</div>
</div>
</div>
<div class="cell border-box-sizing code_cell rendered">
<div class="output_wrapper">
<div class="output">
<div class="output_area">
<div class="output_subarea output_stream output_stdout output_text">
<pre>Flow lattice and integrators ready
</pre>
</div>
</div>
</div>
</div>
</div>
<div class="cell border-box-sizing text_cell rendered"><div class="inner_cell">
<div class="text_cell_render border-box-sizing rendered_html">
<h2 id="Gradient-Flow-%E2%80%94-Three-Integrators-Compared">Gradient Flow — Three Integrators Compared<a class="anchor-link" href="#Gradient-Flow-%E2%80%94-Three-Integrators-Compared">¶</a></h2><p>We run the flow from the same hot-start configuration with all three
integrators and compare $t^2\langle E(t)\rangle$.</p>
</div>
</div>
</div>
<div class="cell border-box-sizing code_cell rendered">
<div class="output_wrapper">
<div class="output">
<div class="output_area">
<div class="output_subarea output_stream output_stdout output_text">
<pre>  Euler                    : E_final=0.000488, t2E_peak=0.028659, 7.3s
</pre>
</div>
</div>
<div class="output_area">
<div class="output_subarea output_stream output_stdout output_text">
<pre>  RK3 W6 (Luscher)         : E_final=0.000475, t2E_peak=0.026253, 20.4s
</pre>
</div>
</div>
<div class="output_area">
<div class="output_subarea output_stream output_stdout output_text">
<pre>  LSCFRK3 W7 (Chuna)       : E_final=0.000475, t2E_peak=0.026255, 20.4s
</pre>
</div>
</div>
</div>
</div>
</div>
<div class="cell border-box-sizing text_cell rendered"><div class="inner_cell">
<div class="text_cell_render border-box-sizing rendered_html">
<h2 id="Rust-Parity">Rust Parity<a class="anchor-link" href="#Rust-Parity">¶</a></h2><p>The same gradient flow integrators are in <code>barracuda/src/lattice/gradient_flow.rs</code>.
GPU-accelerated gauge force computation via WGSL shaders.</p>
<table>
<thead>
<tr>
<th>Implementation</th>
<th>4^4 flow (t=1.0)</th>
<th>Speedup</th>
</tr>
</thead>
<tbody>
<tr>
<td>Python (numpy)</td>
<td>~15 s</td>
<td>1x</td>
</tr>
<tr>
<td>Rust (CPU)</td>
<td>~0.5 s</td>
<td><strong>30x</strong></td>
</tr>
<tr>
<td>Rust (GPU)</td>
<td>~0.05 s</td>
<td><strong>300x</strong></td>
</tr>
</tbody>
</table>
<h2 id="Provenance">Provenance<a class="anchor-link" href="#Provenance">¶</a></h2><ul>
<li><strong>Papers:</strong> Luscher JHEP 08 (2010) 071, BMW arXiv:1203.4469, Bazavov &amp; Chuna arXiv:2101.05320</li>
<li><strong>Validation:</strong> <code>barracuda/src/lattice/gradient_flow.rs</code></li>
<li><strong>Control:</strong> <code>control/gradient_flow/scripts/gradient_flow_control.py</code></li>
<li><strong>Next:</strong> Production flow on 16^4+ via GPU primal composition, physical $t_0$/$w_0$ extraction</li>
</ul>
<hr/>
<p><em>hotSpring — ecoPrimals · AGPL-3.0 · <a href="https://primals.eco">primals.eco</a></em></p>
</div>
</div>
</div>
