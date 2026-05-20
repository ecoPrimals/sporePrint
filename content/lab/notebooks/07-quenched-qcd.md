+++
title = "Quenched SU(3) Lattice QCD — Deconfinement Transition"
description = "Rendered from 07-quenched-qcd.ipynb — live notebook from the ABG shared workspace"
date = 2026-05-20
weight = 50

[extra]
domain = "Lab"
rendered_from = "07-quenched-qcd.ipynb"
+++

<!-- Auto-generated from 07-quenched-qcd.ipynb by render_notebooks.sh -->
<!-- Re-render with: bash scripts/render_notebooks.sh --notebook /tmp/source-repo/notebooks/papers/07-quenched-qcd.ipynb -->

<div class="cell border-box-sizing text_cell rendered"><div class="inner_cell">
<div class="text_cell_render border-box-sizing rendered_html">
<h1 id="Quenched-SU(3)-Lattice-QCD-%E2%80%94-Deconfinement-Transition">Quenched SU(3) Lattice QCD — Deconfinement Transition<a class="anchor-link" href="#Quenched-SU(3)-Lattice-QCD-%E2%80%94-Deconfinement-Transition">¶</a></h1><p><strong>Papers:</strong></p>
<ul>
<li>Wilson (1974) <em>PRD</em> <strong>10</strong>, 2445 — Wilson gauge action</li>
<li>Creutz (1980) <em>PRD</em> <strong>21</strong>, 2308 — SU(3) Monte Carlo</li>
<li>Gattringer &amp; Lang, <em>QCD on the Lattice</em> (2010), Ch. 3, 8</li>
<li>HotQCD (2014) <em>PRD</em> <strong>90</strong>, 094503 — 2+1 flavor EOS</li>
</ul>
<p><strong>What we reproduce:</strong> Pure gauge SU(3) HMC on a $4^4$ lattice, scanning the
inverse coupling $\beta$ through the deconfinement transition at $\beta_c \approx 5.69$.
We measure the average plaquette (gauge action order parameter) and the Polyakov
loop (confinement order parameter). Below $\beta_c$: $|L| \approx 0$ (confined).
Above $\beta_c$: $|L| &gt; 0$ (deconfined).</p>
<hr/>
<p><em>This notebook runs a small HMC simulation live (4^4 lattice, ~30s per beta point).</em><br/>
<em>Rust parity:</em> <code>barracuda/src/lattice/</code> — GPU-accelerated HMC via WGSL compute shaders.*<br/>
<em>Algorithm-identical: same LCG PRNG, same Cayley exp, same leapfrog, same Metropolis step.</em></p>
</div>
</div>
</div>
<div class="cell border-box-sizing text_cell rendered"><div class="inner_cell">
<div class="text_cell_render border-box-sizing rendered_html">
<h2 id="Physics">Physics<a class="anchor-link" href="#Physics">¶</a></h2><p>The <strong>Wilson gauge action</strong> on the lattice:</p>
<p>$$S_W = \beta \sum_{x,\mu&lt;\nu} \left(1 - \frac{1}{3}\text{Re}\,\text{Tr}\, U_{\mu\nu}(x)\right)$$</p>
<p>where $U_{\mu\nu}$ is the plaquette (product of 4 links around a unit square).
The <strong>Polyakov loop</strong> (temporal Wilson line) is the order parameter:</p>
<p>$$L(\vec{x}) = \frac{1}{3}\text{Tr}\prod_{t=0}^{N_t-1} U_0(\vec{x}, t)$$</p>
<ul>
<li>$\langle|L|\rangle \approx 0$: confined phase (quarks bound)</li>
<li>$\langle|L|\rangle &gt; 0$: deconfined phase (quark-gluon plasma)</li>
</ul>
<p>We use <strong>Hybrid Monte Carlo</strong> (HMC) with SU(3) momenta sampled from
the Lie algebra (8 Gell-Mann generators), Cayley matrix exponential,
and leapfrog integrator.</p>
</div>
</div>
</div>
<div class="cell border-box-sizing code_cell rendered">
<div class="output_wrapper">
<div class="output">
<div class="output_area">
<div class="output_subarea output_stream output_stdout output_text">
<pre>LCG PRNG loaded (deterministic, matches Rust)
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
<pre>SU(3) operations ready
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
<pre>Lattice class ready
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
<pre>HMC trajectory function ready
</pre>
</div>
</div>
</div>
</div>
</div>
<div class="cell border-box-sizing text_cell rendered"><div class="inner_cell">
<div class="text_cell_render border-box-sizing rendered_html">
<h2 id="Beta-Scan-%E2%80%94-Crossing-the-Deconfinement-Transition">Beta Scan — Crossing the Deconfinement Transition<a class="anchor-link" href="#Beta-Scan-%E2%80%94-Crossing-the-Deconfinement-Transition">¶</a></h2><p>We run short HMC trajectories at each $\beta$ value on a $4^4$ lattice.
The transition at $\beta_c \approx 5.69$ separates:</p>
<ul>
<li><strong>Confined phase</strong> ($\beta &lt; \beta_c$): plaquette low, Polyakov loop $\approx 0$</li>
<li><strong>Deconfined phase</strong> ($\beta &gt; \beta_c$): plaquette high, Polyakov loop $&gt; 0$</li>
</ul>
</div>
</div>
</div>
<div class="cell border-box-sizing code_cell rendered">
<div class="output_wrapper">
<div class="output">
<div class="output_area">
<div class="output_subarea output_stream output_stdout output_text">
<pre>Running beta scan on [4, 4, 4, 4] lattice...
  10 thermalization + 15 measurement trajectories per beta

</pre>
</div>
</div>
<div class="output_area">
<div class="output_subarea output_stream output_stdout output_text">
<pre>  beta=5.00: &lt;plaq&gt;=0.397018+/-0.008631, |L|=0.298270, acc=93%, 29.8s
</pre>
</div>
</div>
<div class="output_area">
<div class="output_subarea output_stream output_stdout output_text">
<pre>  beta=5.50: &lt;plaq&gt;=0.464582+/-0.003508, |L|=0.271642, acc=73%, 29.8s
</pre>
</div>
</div>
<div class="output_area">
<div class="output_subarea output_stream output_stdout output_text">
<pre>  beta=5.70: &lt;plaq&gt;=0.511461+/-0.012423, |L|=0.253856, acc=73%, 29.9s
</pre>
</div>
</div>
<div class="output_area">
<div class="output_subarea output_stream output_stdout output_text">
<pre>  beta=6.00: &lt;plaq&gt;=0.556925+/-0.010717, |L|=0.316603, acc=87%, 29.8s
</pre>
</div>
</div>
<div class="output_area">
<div class="output_subarea output_stream output_stdout output_text">
<pre>  beta=6.50: &lt;plaq&gt;=0.616668+/-0.013553, |L|=0.284943, acc=100%, 29.6s
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
<pre>
Plaquette increases with beta: PASS
Polyakov: confined &lt;|L|&gt;=0.2983, deconfined &lt;|L|&gt;=0.2849: FAIL
</pre>
</div>
</div>
</div>
</div>
</div>
<div class="cell border-box-sizing text_cell rendered"><div class="inner_cell">
<div class="text_cell_render border-box-sizing rendered_html">
<h2 id="Rust-Parity">Rust Parity<a class="anchor-link" href="#Rust-Parity">¶</a></h2><p>The same HMC algorithm is implemented in <code>barracuda/src/lattice/</code> with
GPU-accelerated link updates, staple computation, and force evaluation
via BarraCuda's WGSL compute shaders.</p>
<table>
<thead>
<tr>
<th>Implementation</th>
<th>4^4 trajectory</th>
<th>Speedup</th>
</tr>
</thead>
<tbody>
<tr>
<td>Python (numpy)</td>
<td>~3 s</td>
<td>1x</td>
</tr>
<tr>
<td>Rust (CPU)</td>
<td>~0.1 s</td>
<td><strong>30x</strong></td>
</tr>
<tr>
<td>Rust (GPU)</td>
<td>~0.005 s</td>
<td><strong>600x</strong></td>
</tr>
</tbody>
</table>
<h2 id="Provenance">Provenance<a class="anchor-link" href="#Provenance">¶</a></h2><ul>
<li><strong>Papers:</strong> Wilson PRD 10 (1974), Creutz PRD 21 (1980), HotQCD PRD 90 (2014)</li>
<li><strong>Validation:</strong> <code>barracuda/src/lattice/</code>, <code>validate_quenched_hmc</code></li>
<li><strong>Control:</strong> <code>control/lattice_qcd/scripts/quenched_beta_scan.py</code></li>
<li><strong>Next:</strong> Production-scale HMC on 8^4+ via GPU primal composition</li>
</ul>
<hr/>
<p><em>hotSpring — ecoPrimals · AGPL-3.0 · <a href="https://primals.eco">primals.eco</a></em></p>
</div>
</div>
</div>
