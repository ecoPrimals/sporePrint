+++
title = "Stanton-Murillo Transport Coefficients — Yukawa OCP"
description = "Rendered from 05-stanton-murillo-transport.ipynb — live notebook from the ABG shared workspace"
date = 2026-05-27
weight = 50

[extra]
domain = "Lab"
rendered_from = "05-stanton-murillo-transport.ipynb"
+++

<!-- Auto-generated from 05-stanton-murillo-transport.ipynb by render_notebooks.sh -->
<!-- Re-render with: bash scripts/render_notebooks.sh --notebook /tmp/source-repo/notebooks/papers/05-stanton-murillo-transport.ipynb -->

<div class="cell border-box-sizing text_cell rendered"><div class="inner_cell">
<div class="text_cell_render border-box-sizing rendered_html">
<h1 id="Stanton-Murillo-Transport-Coefficients-%E2%80%94-Yukawa-OCP">Stanton-Murillo Transport Coefficients — Yukawa OCP<a class="anchor-link" href="#Stanton-Murillo-Transport-Coefficients-%E2%80%94-Yukawa-OCP">¶</a></h1><p><strong>Papers:</strong></p>
<ul>
<li>Stanton &amp; Murillo, <em>PRE</em> <strong>91</strong>, 033104 (2015) — ionic transport model</li>
<li>Daligault, <em>PRE</em> <strong>86</strong>, 047401 (2012) — D* analytical fit</li>
</ul>
<p><strong>What we reproduce:</strong> The Daligault analytical model for the reduced
self-diffusion coefficient $D^*(\Gamma, \kappa)$ of a Yukawa one-component
plasma, covering the full range from weakly coupled gas ($\Gamma \ll 1$)
to strongly coupled liquid/crystal ($\Gamma \gg 100$). This model interpolates
between Landau-Spitzer kinetic theory and the caging/Einstein frequency regime.</p>
<hr/>
<p><em>All compute runs live — the analytical model is fast (&lt;1 ms).</em><br/>
<em>Production MD transport grids are loaded from frozen JSON when available.</em><br/>
<em>Rust parity:</em> <code>barracuda/src/physics/transport.rs</code></p>
</div>
</div>
</div>
<div class="cell border-box-sizing text_cell rendered"><div class="inner_cell">
<div class="text_cell_render border-box-sizing rendered_html">
<h2 id="Physics">Physics<a class="anchor-link" href="#Physics">¶</a></h2><p>The reduced self-diffusion coefficient $D^* = D/(a_{\text{ws}}^2 \omega_p)$ interpolates:</p>
<p>$$D^*(\Gamma, \kappa) = D^*_w(\Gamma, \kappa) \cdot f(\Gamma, \kappa) + D^*_s(\Gamma, \kappa) \cdot [1 - f(\Gamma, \kappa)]$$</p>
<p><strong>Weak coupling</strong> (Landau-Spitzer):
$$D^*_w = \frac{3\sqrt{\pi}}{4} \frac{1}{\Gamma^{5/2} \ln\Lambda}$$</p>
<p><strong>Strong coupling</strong> (caging):
$$D^*_s = A(\kappa) \cdot \Gamma^{-\alpha(\kappa)}$$</p>
<p><strong>Crossover:</strong>
$$f(\Gamma, \kappa) = \frac{1}{1 + (\Gamma/\Gamma_x(\kappa))^2}$$</p>
</div>
</div>
</div>
<div class="cell border-box-sizing code_cell rendered">
<div class="output_wrapper">
<div class="output">
<div class="output_area">
<div class="output_subarea output_stream output_stdout output_text">
<pre>Daligault transport model loaded
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
<h2 id="Screening-Dependence-%E2%80%94-2D-Map">Screening Dependence — 2D Map<a class="anchor-link" href="#Screening-Dependence-%E2%80%94-2D-Map">¶</a></h2>
</div>
</div>
</div>
<div class="cell border-box-sizing code_cell rendered">
</div>
<div class="cell border-box-sizing text_cell rendered"><div class="inner_cell">
<div class="text_cell_render border-box-sizing rendered_html">
<h2 id="Rust-Parity">Rust Parity<a class="anchor-link" href="#Rust-Parity">¶</a></h2><p>The Daligault model and MD-based transport are in <code>barracuda/src/physics/transport.rs</code>.
Production MD sweeps (N=500, 50+ $\Gamma$ values) are validated against
the analytical fit.</p>
<h2 id="Provenance">Provenance<a class="anchor-link" href="#Provenance">¶</a></h2><ul>
<li><strong>Papers:</strong> Stanton &amp; Murillo PRE 91 (2015), Daligault PRE 86 (2012)</li>
<li><strong>Control:</strong> <code>control/sarkas/simulations/transport-study/scripts/daligault_fit.py</code></li>
<li><strong>Validation:</strong> <code>barracuda/src/physics/transport.rs</code></li>
<li><strong>Next:</strong> Full transport tensor via GPU MD + Green-Kubo</li>
</ul>
<hr/>
<p><em>hotSpring — ecoPrimals · AGPL-3.0 · <a href="https://primals.eco">primals.eco</a></em></p>
</div>
</div>
</div>
