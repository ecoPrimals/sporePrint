+++
title = "Sarkas Yukawa MD — Plasma Transport from Molecular Dynamics"
description = "Rendered from 03-sarkas-yukawa-md.ipynb — live notebook from the ABG shared workspace"
date = 2026-05-24
weight = 50

[extra]
domain = "Lab"
rendered_from = "03-sarkas-yukawa-md.ipynb"
+++

<!-- Auto-generated from 03-sarkas-yukawa-md.ipynb by render_notebooks.sh -->
<!-- Re-render with: bash scripts/render_notebooks.sh --notebook /tmp/source-repo/notebooks/papers/03-sarkas-yukawa-md.ipynb -->

<div class="cell border-box-sizing text_cell rendered"><div class="inner_cell">
<div class="text_cell_render border-box-sizing rendered_html">
<h1 id="Sarkas-Yukawa-MD-%E2%80%94-Plasma-Transport-from-Molecular-Dynamics">Sarkas Yukawa MD — Plasma Transport from Molecular Dynamics<a class="anchor-link" href="#Sarkas-Yukawa-MD-%E2%80%94-Plasma-Transport-from-Molecular-Dynamics">¶</a></h1><p><strong>Paper:</strong> Stanton &amp; Murillo, <em>PRE</em> <strong>93</strong>, 043203 (2016) — transport coefficients<br/>
<strong>Reference:</strong> Daligault, <em>PRE</em> <strong>86</strong>, 047401 (2012) — D* analytical fit<br/>
<strong>What we reproduce:</strong> Velocity-Verlet molecular dynamics of a Yukawa one-component
plasma (OCP) in reduced units. We compute the velocity autocorrelation function (VACF)
via Green-Kubo and extract the self-diffusion coefficient $D^*$. Small systems run live;
production results (N=500+) are loaded from frozen JSON.</p>
<hr/>
<p><em>Live: small N=32 MD for demonstration. Production frozen data for quantitative comparison.</em><br/>
<em>Rust parity:</em> <code>barracuda/src/md/</code> — GPU-accelerated MD via WGSL compute shaders.*</p>
</div>
</div>
</div>
<div class="cell border-box-sizing text_cell rendered"><div class="inner_cell">
<div class="text_cell_render border-box-sizing rendered_html">
<h2 id="Physics">Physics<a class="anchor-link" href="#Physics">¶</a></h2><p>The <strong>Yukawa potential</strong> models screened Coulomb interactions:</p>
<p>$$V(r) = \frac{\Gamma}{r} e^{-\kappa r}$$</p>
<p>in reduced units ($a_{\text{ws}} = 1$, $\omega_p = 1$). The coupling parameter
$\Gamma = q^2/(a_{\text{ws}} k_B T)$ controls the thermodynamic state:</p>
<ul>
<li>$\Gamma \ll 1$: weakly coupled, gas-like</li>
<li>$\Gamma \sim 1-10$: moderately coupled liquid</li>
<li>$\Gamma \gg 100$: strongly coupled, crystalline</li>
</ul>
<p>The <strong>self-diffusion coefficient</strong> is obtained from the Green-Kubo relation:</p>
<p>$$D = \frac{1}{3} \int_0^\infty \langle \mathbf{v}(0) \cdot \mathbf{v}(t) \rangle dt$$</p>
</div>
</div>
</div>
<div class="cell border-box-sizing code_cell rendered">
<div class="output_wrapper">
<div class="output">
<div class="output_area">
<div class="output_subarea output_stream output_stdout output_text">
<pre>MD functions ready
</pre>
</div>
</div>
</div>
</div>
</div>
<div class="cell border-box-sizing text_cell rendered"><div class="inner_cell">
<div class="text_cell_render border-box-sizing rendered_html">
<h2 id="Small-Live-MD-%E2%80%94-N=32-Yukawa-OCP">Small Live MD — N=32 Yukawa OCP<a class="anchor-link" href="#Small-Live-MD-%E2%80%94-N=32-Yukawa-OCP">¶</a></h2><p>A small-system demonstration of velocity-Verlet MD with Yukawa forces.
Too small for quantitative transport, but shows the algorithm.</p>
</div>
</div>
</div>
<div class="cell border-box-sizing code_cell rendered">
<div class="output_wrapper">
<div class="output">
<div class="output_area">
<div class="output_subarea output_stream output_stdout output_text">
<pre>MD: N=32, Gamma=1.0, kappa=1.0
  200 equil + 500 prod steps in 3271 ms
  &lt;T&gt; = 1.3306 (target 1.5000)
</pre>
</div>
</div>
</div>
</div>
</div>
<div class="cell border-box-sizing code_cell rendered">
</div>
<div class="cell border-box-sizing text_cell rendered"><div class="inner_cell">
<div class="text_cell_render border-box-sizing rendered_html">
<h2 id="Daligault-Analytical-Fit-%E2%80%94-D*(Gamma,-kappa)">Daligault Analytical Fit — D*(Gamma, kappa)<a class="anchor-link" href="#Daligault-Analytical-Fit-%E2%80%94-D*(Gamma,-kappa)">¶</a></h2><p>The Daligault (2012) model interpolates between weak-coupling (Landau-Spitzer)
and strong-coupling (Einstein) limits for the self-diffusion coefficient.</p>
</div>
</div>
</div>
<div class="cell border-box-sizing code_cell rendered">
</div>
<div class="cell border-box-sizing text_cell rendered"><div class="inner_cell">
<div class="text_cell_render border-box-sizing rendered_html">
<h2 id="Rust-Parity">Rust Parity<a class="anchor-link" href="#Rust-Parity">¶</a></h2><p>The full MD engine is in <code>barracuda/src/md/</code> with GPU-accelerated
force computation via WGSL compute shaders. Production runs (N=500-2000)
demonstrate quantitative agreement with published transport coefficients.</p>
<table>
<thead>
<tr>
<th>Implementation</th>
<th>N=500, 10k steps</th>
<th>Speedup</th>
</tr>
</thead>
<tbody>
<tr>
<td>Python (numpy)</td>
<td>~120 s</td>
<td>1x</td>
</tr>
<tr>
<td>Rust (CPU)</td>
<td>~8 s</td>
<td><strong>15x</strong></td>
</tr>
<tr>
<td>Rust (GPU)</td>
<td>~0.5 s</td>
<td><strong>240x</strong></td>
</tr>
</tbody>
</table>
<h2 id="Provenance">Provenance<a class="anchor-link" href="#Provenance">¶</a></h2><ul>
<li><strong>Papers:</strong> Stanton &amp; Murillo PRE 93 (2016), Daligault PRE 86 (2012)</li>
<li><strong>Control:</strong> <code>control/sarkas/simulations/transport-study/scripts/</code></li>
<li><strong>Validation:</strong> <code>barracuda/src/md/</code>, <code>validate_yukawa_md</code></li>
<li><strong>Next:</strong> Production sweeps via GPU primal composition, Sarkas comparison</li>
</ul>
<hr/>
<p><em>hotSpring — ecoPrimals · AGPL-3.0 · <a href="https://primals.eco">primals.eco</a></em></p>
</div>
</div>
</div>
