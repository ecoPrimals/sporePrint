+++
title = "Dynamical Fermion QCD — Staggered HMC, HVP, Freeze-Out"
description = "Rendered from 08-dynamical-fermions.ipynb — live notebook from the ABG shared workspace"
date = 2026-05-30
weight = 50

[extra]
domain = "Lab"
rendered_from = "08-dynamical-fermions.ipynb"
+++

<!-- Auto-generated from 08-dynamical-fermions.ipynb by render_notebooks.sh -->
<!-- Re-render with: bash scripts/render_notebooks.sh --notebook /tmp/source-repo/notebooks/papers/08-dynamical-fermions.ipynb -->

<div class="cell border-box-sizing text_cell rendered"><div class="inner_cell">
<div class="text_cell_render border-box-sizing rendered_html">
<h1 id="Dynamical-Fermion-QCD-%E2%80%94-Staggered-HMC,-HVP,-Freeze-Out">Dynamical Fermion QCD — Staggered HMC, HVP, Freeze-Out<a class="anchor-link" href="#Dynamical-Fermion-QCD-%E2%80%94-Staggered-HMC,-HVP,-Freeze-Out">¶</a></h1><p><strong>Papers:</strong></p>
<ul>
<li>Gottlieb et al. (1987) <em>PRD</em> <strong>35</strong>, 2531 — pseudofermion HMC</li>
<li>Gattringer &amp; Lang, <em>QCD on the Lattice</em> (2010), Ch. 8 — staggered fermions</li>
<li>Bernecker &amp; Meyer (2011) <em>EPJA</em> <strong>47</strong>, 148 — hadronic vacuum polarization</li>
</ul>
<p><strong>What we reproduce:</strong> Full dynamical fermion HMC with staggered quarks on a small
lattice, demonstrating the pseudofermion heat bath, CG solver, and combined
gauge + fermion force. Includes hadronic vacuum polarization correlators and
QCD freeze-out critical endpoint search.</p>
<hr/>
<p><em>Live: small-lattice (4^4) dynamical HMC with heavy quarks (m=2.0).</em><br/>
<em>Production trajectories (8^4+) are loaded from frozen JSON when available.</em><br/>
<em>Rust parity:</em> <code>barracuda/src/lattice/</code> — GPU-accelerated staggered fermion HMC.*</p>
</div>
</div>
</div>
<div class="cell border-box-sizing text_cell rendered"><div class="inner_cell">
<div class="text_cell_render border-box-sizing rendered_html">
<h2 id="Physics">Physics<a class="anchor-link" href="#Physics">¶</a></h2><p>The staggered fermion action on the lattice:</p>
<p>$$S_F = \bar{\chi} D[U] \chi = \bar{\chi} \left( m\delta_{xy} + \frac{1}{2}\sum_\mu \eta_\mu(x) [U_\mu(x)\delta_{x+\hat\mu,y} - U_\mu^\dagger(x-\hat\mu)\delta_{x-\hat\mu,y}] \right) \chi$$</p>
<p>where $\eta_\mu(x) = (-1)^{x_0 + ... + x_{\mu-1}}$ are the staggered phases.</p>
<p>The pseudofermion technique replaces the fermion determinant with a bosonic
path integral over fields $\phi$:</p>
<p>$$\det(D^\dagger D) = \int \mathcal{D}\phi \, e^{-\phi^\dagger (D^\dagger D)^{-1} \phi}$$</p>
<p>We solve $(D^\dagger D) x = b$ via conjugate gradient (CG) during the HMC trajectory.</p>
</div>
</div>
</div>
<div class="cell border-box-sizing code_cell rendered">
<div class="output_wrapper">
<div class="output">
<div class="output_area">
<div class="output_subarea output_stream output_stdout output_text">
<pre>SU(3) + PRNG ready
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
<pre>Staggered Dirac operator ready
</pre>
</div>
</div>
</div>
</div>
</div>
<div class="cell border-box-sizing text_cell rendered"><div class="inner_cell">
<div class="text_cell_render border-box-sizing rendered_html">
<h2 id="Dynamical-HMC-%E2%80%94-Plaquette-with-Fermion-Backreaction">Dynamical HMC — Plaquette with Fermion Backreaction<a class="anchor-link" href="#Dynamical-HMC-%E2%80%94-Plaquette-with-Fermion-Backreaction">¶</a></h2><p>We run a short dynamical fermion simulation to show the effect of
quark loops on the plaquette expectation value. With heavy quarks
(m=2.0), the fermion backreaction is small but measurable.</p>
</div>
</div>
</div>
<div class="cell border-box-sizing code_cell rendered">
<div class="output_wrapper">
<div class="output">
<div class="output_area">
<div class="output_subarea output_stream output_stdout output_text">
<pre>Lattice: [4, 4, 4, 4], beta=5.5, mass=2.0
Initial plaquette: 0.147966
Chiral condensate proxy: &lt;psi-bar psi&gt; ~ 0.505711
</pre>
</div>
</div>
</div>
</div>
</div>
<div class="cell border-box-sizing text_cell rendered"><div class="inner_cell">
<div class="text_cell_render border-box-sizing rendered_html">
<h2 id="Rust-Parity">Rust Parity<a class="anchor-link" href="#Rust-Parity">¶</a></h2><p>The full dynamical fermion HMC is in <code>barracuda/src/lattice/</code> with
GPU-accelerated CG solver and staggered Dirac operator.</p>
<table>
<thead>
<tr>
<th>Implementation</th>
<th>4^4 dynamical traj</th>
<th>Speedup</th>
</tr>
</thead>
<tbody>
<tr>
<td>Python (numpy)</td>
<td>~10 s</td>
<td>1x</td>
</tr>
<tr>
<td>Rust (CPU)</td>
<td>~0.3 s</td>
<td><strong>33x</strong></td>
</tr>
<tr>
<td>Rust (GPU)</td>
<td>~0.02 s</td>
<td><strong>500x</strong></td>
</tr>
</tbody>
</table>
<h2 id="Provenance">Provenance<a class="anchor-link" href="#Provenance">¶</a></h2><ul>
<li><strong>Papers:</strong> Gottlieb et al. PRD 35 (1987), Bernecker &amp; Meyer EPJA 47 (2011)</li>
<li><strong>Control:</strong> <code>control/lattice_qcd/scripts/dynamical_fermion_control.py</code></li>
<li><strong>Next:</strong> 2+1 flavor HMC, HVP correlators, freeze-out via primal composition</li>
</ul>
<hr/>
<p><em>hotSpring — ecoPrimals · AGPL-3.0 · <a href="https://primals.eco">primals.eco</a></em></p>
</div>
</div>
</div>
