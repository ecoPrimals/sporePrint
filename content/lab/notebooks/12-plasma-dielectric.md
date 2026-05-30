+++
title = "Plasma Dielectric Functions — BGK/Mermin + Kinetic-Fluid Coupling"
description = "Rendered from 12-plasma-dielectric.ipynb — live notebook from the ABG shared workspace"
date = 2026-05-30
weight = 50

[extra]
domain = "Lab"
rendered_from = "12-plasma-dielectric.ipynb"
+++

<!-- Auto-generated from 12-plasma-dielectric.ipynb by render_notebooks.sh -->
<!-- Re-render with: bash scripts/render_notebooks.sh --notebook /tmp/source-repo/notebooks/papers/12-plasma-dielectric.ipynb -->

<div class="cell border-box-sizing text_cell rendered"><div class="inner_cell">
<div class="text_cell_render border-box-sizing rendered_html">
<h1 id="Plasma-Dielectric-Functions-%E2%80%94-BGK/Mermin-+-Kinetic-Fluid-Coupling">Plasma Dielectric Functions — BGK/Mermin + Kinetic-Fluid Coupling<a class="anchor-link" href="#Plasma-Dielectric-Functions-%E2%80%94-BGK/Mermin-+-Kinetic-Fluid-Coupling">¶</a></h1><p><strong>Papers:</strong></p>
<ul>
<li>Chuna &amp; Murillo, <em>Phys. Rev. E</em> <strong>111</strong>, 035206 (2024), arXiv:2405.07871 — Completed Mermin</li>
<li>Haack, Murillo, Sagert &amp; Chuna, <em>J. Comput. Phys.</em> (2024), DOI:10.1016/j.jcp.2024.112908 — Kinetic-fluid</li>
<li>Mermin, <em>Phys. Rev. B</em> <strong>1</strong>, 2362 (1970) — Original Mermin function</li>
</ul>
<p><strong>What we reproduce:</strong> The completed Mermin dielectric function with number + momentum
conservation, Vlasov susceptibility, dynamic structure factor $S(k,\omega)$,
f-sum rule validation, and multi-species BGK kinetic-fluid relaxation.</p>
<hr/>
<p><em>All compute runs live — analytical dielectric functions + BGK relaxation in pure numpy.</em><br/>
<em>Rust parity:</em> <code>barracuda/src/physics/dielectric.rs</code>, <code>barracuda/src/physics/kinetic_fluid.rs</code></p>
</div>
</div>
</div>
<div class="cell border-box-sizing text_cell rendered"><div class="inner_cell">
<div class="text_cell_render border-box-sizing rendered_html">
<h2 id="Physics-%E2%80%94-Dielectric-Response">Physics — Dielectric Response<a class="anchor-link" href="#Physics-%E2%80%94-Dielectric-Response">¶</a></h2><p>The <strong>Vlasov</strong> (collisionless) dielectric function for a classical plasma:</p>
<p>$$\varepsilon_V(k, \omega) = 1 + \frac{k_D^2}{k^2} W\left(\frac{\omega}{\sqrt{2} k v_{\text{th}}}\right)$$</p>
<p>where $W(z) = 1 + z Z(z)$ and $Z(z)$ is the plasma dispersion function.</p>
<p>The <strong>completed Mermin</strong> (Chuna &amp; Murillo 2024) conserves both particle number AND momentum:</p>
<p>$$\varepsilon_{CM}(k, \omega) = 1 + \frac{(\omega + i\nu)}{\omega} \frac{\varepsilon_V(k, \omega+i\nu) - 1}{1 + \frac{i\nu}{\omega} R(1 - G_p)}$$</p>
<p>where $G_p = R \cdot \omega(\omega + i\nu) / (k^2 v_{\text{th}}^2)$ is the momentum correction.</p>
</div>
</div>
</div>
<div class="cell border-box-sizing code_cell rendered">
<div class="output_wrapper">
<div class="output">
<div class="output_area">
<div class="output_subarea output_stream output_stdout output_text">
<pre>Dielectric functions defined
</pre>
</div>
</div>
</div>
</div>
</div>
<div class="cell border-box-sizing text_cell rendered"><div class="inner_cell">
<div class="text_cell_render border-box-sizing rendered_html">
<h2 id="Loss-Function-and-Dynamic-Structure-Factor">Loss Function and Dynamic Structure Factor<a class="anchor-link" href="#Loss-Function-and-Dynamic-Structure-Factor">¶</a></h2>
</div>
</div>
</div>
<div class="cell border-box-sizing code_cell rendered">
</div>
<div class="cell border-box-sizing text_cell rendered"><div class="inner_cell">
<div class="text_cell_render border-box-sizing rendered_html">
<h2 id="Sum-Rule-Validation-and-Static-Limits">Sum Rule Validation and Static Limits<a class="anchor-link" href="#Sum-Rule-Validation-and-Static-Limits">¶</a></h2>
</div>
</div>
</div>
<div class="cell border-box-sizing code_cell rendered">
<div class="output_wrapper">
<div class="output">
<div class="output_area">
<div class="output_subarea output_stream output_stdout output_text">
<pre>f-sum rule: integral = -6.6430, expected = -19.7392, error = 66.3%
  computed in 604 ms

Debye screening: eps(k,0) = 3.5985, expected = 3.5985
High-freq limit: eps(k,100wp) = 0.999967 + 0.000000i (expect ~1)
</pre>
</div>
</div>
</div>
</div>
</div>
<div class="cell border-box-sizing text_cell rendered"><div class="inner_cell">
<div class="text_cell_render border-box-sizing rendered_html">
<h2 id="Kinetic-Fluid-Coupling-%E2%80%94-Multi-Species-BGK-Relaxation">Kinetic-Fluid Coupling — Multi-Species BGK Relaxation<a class="anchor-link" href="#Kinetic-Fluid-Coupling-%E2%80%94-Multi-Species-BGK-Relaxation">¶</a></h2><p>Phase 1 of the kinetic-fluid framework: homogeneous multi-species BGK
relaxation with conservation-preserving target Maxwellians (Haack et al. 2017, 2024).</p>
</div>
</div>
</div>
<div class="cell border-box-sizing code_cell rendered">
<div class="output_wrapper">
<div class="output">
<div class="output_area">
<div class="output_subarea output_stream output_stdout output_text">
<pre>Final: u1=0.2026, u2=-0.1757 (should converge)
Final: T1=3.1130, T2=2.9718 (should converge)
</pre>
</div>
</div>
</div>
</div>
</div>
<div class="cell border-box-sizing text_cell rendered"><div class="inner_cell">
<div class="text_cell_render border-box-sizing rendered_html">
<h2 id="Rust-Parity">Rust Parity<a class="anchor-link" href="#Rust-Parity">¶</a></h2><p>Both the dielectric functions and kinetic-fluid relaxation are implemented in Rust:</p>
<ul>
<li><code>barracuda/src/physics/dielectric.rs</code>: Vlasov, Mermin, completed Mermin</li>
<li><code>barracuda/src/physics/kinetic_fluid.rs</code>: Multi-species BGK, Sod shock tube</li>
</ul>
<h2 id="Provenance">Provenance<a class="anchor-link" href="#Provenance">¶</a></h2><ul>
<li><strong>Papers:</strong> Chuna &amp; Murillo PRE 111 (2024), Haack et al. JCP (2024), Mermin PRB 1 (1970)</li>
<li><strong>Control:</strong> <code>control/bgk_dielectric/scripts/bgk_dielectric_control.py</code>, <code>control/kinetic_fluid/scripts/kinetic_fluid_control.py</code></li>
<li><strong>Next:</strong> GPU-accelerated DSF computation via primal composition</li>
</ul>
<hr/>
<p><em>hotSpring — ecoPrimals · AGPL-3.0 · <a href="https://primals.eco">primals.eco</a></em></p>
</div>
</div>
</div>
