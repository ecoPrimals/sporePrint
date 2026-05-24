+++
title = "Abelian Higgs Model — (1+1)D Lattice Field Theory"
description = "Rendered from 09-abelian-higgs.ipynb — live notebook from the ABG shared workspace"
date = 2026-05-24
weight = 50

[extra]
domain = "Lab"
rendered_from = "09-abelian-higgs.ipynb"
+++

<!-- Auto-generated from 09-abelian-higgs.ipynb by render_notebooks.sh -->
<!-- Re-render with: bash scripts/render_notebooks.sh --notebook /tmp/source-repo/notebooks/papers/09-abelian-higgs.ipynb -->

<div class="cell border-box-sizing text_cell rendered"><div class="inner_cell">
<div class="text_cell_render border-box-sizing rendered_html">
<h1 id="Abelian-Higgs-Model-%E2%80%94-(1+1)D-Lattice-Field-Theory">Abelian Higgs Model — (1+1)D Lattice Field Theory<a class="anchor-link" href="#Abelian-Higgs-Model-%E2%80%94-(1+1)D-Lattice-Field-Theory">¶</a></h1><p><strong>Paper:</strong> Bazavov et al., <em>Phys. Rev. D</em> <strong>92</strong>, 076003 (2015)<br/>
<strong>What we reproduce:</strong> U(1) gauge + complex scalar Higgs on a (1+1)D lattice using HMC.
We scan coupling parameters ($\beta_{\text{pl}}$, $\kappa$, $\lambda$) to explore
the Higgs mechanism, confinement, and symmetry breaking in the simplest gauge-Higgs system.</p>
<hr/>
<p><em>This notebook runs live HMC on small (8x8) lattices in Python. All compute executes in-notebook.</em><br/>
<em>Rust parity:</em> <code>barracuda/src/lattice/abelian_higgs.rs</code> — GPU-accelerated via WGSL.*<br/>
<em>Algorithm-identical: same LCG PRNG, same leapfrog, same Metropolis accept/reject.</em></p>
</div>
</div>
</div>
<div class="cell border-box-sizing text_cell rendered"><div class="inner_cell">
<div class="text_cell_render border-box-sizing rendered_html">
<h2 id="Physics">Physics<a class="anchor-link" href="#Physics">¶</a></h2><p>The Abelian Higgs action on a (1+1)D lattice:</p>
<p>$$S = S_{\text{gauge}} + S_{\text{Higgs}}$$</p>
<p>$$S_{\text{gauge}} = \beta_{\text{pl}} \sum_x \left(1 - \cos\theta_p(x)\right)$$</p>
<p>$$S_{\text{Higgs}} = \sum_x \left[|\phi(x)|^2 + \lambda(|\phi(x)|^2 - 1)^2 - 2\kappa \sum_\mu \text{Re}(\phi^* U_\mu \phi')\right]$$</p>
<p>where $U_\mu = e^{i\theta_\mu}$ are U(1) link variables and $\phi$ is a complex scalar.</p>
<p>Key phases:</p>
<ul>
<li><strong>Higgs phase</strong> (large $\kappa$): $\langle|\phi|^2\rangle \gg 1$, gauge symmetry "broken"</li>
<li><strong>Confined phase</strong> (small $\kappa$): $\langle|\phi|^2\rangle \approx 1$, confining strings</li>
<li><strong>Coulomb phase</strong> (large $\beta_{\text{pl}}$): plaquette $\to 1$, weak coupling</li>
</ul>
</div>
</div>
</div>
<div class="cell border-box-sizing code_cell rendered">
<div class="output_wrapper">
<div class="output">
<div class="output_area">
<div class="output_subarea output_stream output_stdout output_text">
<pre>LCG PRNG ready (deterministic, matches Rust)
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
<pre>U(1) Higgs lattice + HMC ready
</pre>
</div>
</div>
</div>
</div>
</div>
<div class="cell border-box-sizing text_cell rendered"><div class="inner_cell">
<div class="text_cell_render border-box-sizing rendered_html">
<h2 id="Phase-Scan-%E2%80%94-Higgs,-Confined,-Coulomb">Phase Scan — Higgs, Confined, Coulomb<a class="anchor-link" href="#Phase-Scan-%E2%80%94-Higgs,-Confined,-Coulomb">¶</a></h2><p>We run HMC across different parameter regimes to identify the physical phases.</p>
</div>
</div>
</div>
<div class="cell border-box-sizing code_cell rendered">
<div class="output_wrapper">
<div class="output">
<div class="output_area">
<div class="output_subarea output_stream output_stderr output_text">
<pre>/tmp/ipykernel_2832/4200604355.py:22: RuntimeWarning: overflow encountered in scalar multiply
  self.state = self.state * LCG_MUL + LCG_INC
/tmp/ipykernel_2832/4200604355.py:22: RuntimeWarning: overflow encountered in scalar add
  self.state = self.state * LCG_MUL + LCG_INC
</pre>
</div>
</div>
<div class="output_area">
<div class="output_subarea output_stream output_stdout output_text">
<pre>  Weak coupling         plaq=0.906987  |phi^2|=0.8742  acc=84%  1.1s
</pre>
</div>
</div>
<div class="output_area">
<div class="output_subarea output_stream output_stdout output_text">
<pre>  Strong coupling       plaq=0.245105  |phi^2|=0.8912  acc=90%  1.1s
</pre>
</div>
</div>
<div class="output_area">
<div class="output_subarea output_stream output_stdout output_text">
<pre>  Higgs condensed       plaq=0.926801  |phi^2|=4.4227  acc=92%  1.1s
</pre>
</div>
</div>
<div class="output_area">
<div class="output_subarea output_stream output_stdout output_text">
<pre>  Confined              plaq=0.431564  |phi^2|=0.7977  acc=96%  1.1s
</pre>
</div>
</div>
<div class="output_area">
<div class="output_subarea output_stream output_stdout output_text">
<pre>  Large lambda          plaq=0.693292  |phi^2|=0.9972  acc=86%  1.1s
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
<h2 id="Rust-Parity">Rust Parity<a class="anchor-link" href="#Rust-Parity">¶</a></h2><p>The same U(1) Higgs HMC is implemented in <code>barracuda/src/lattice/abelian_higgs.rs</code>.
Rust validation confirms bit-for-bit agreement with the Python baseline for
deterministic LCG-seeded configurations.</p>
<table>
<thead>
<tr>
<th>Implementation</th>
<th>8x8 trajectory</th>
<th>Speedup</th>
</tr>
</thead>
<tbody>
<tr>
<td>Python (numpy)</td>
<td>~0.5 s</td>
<td>1x</td>
</tr>
<tr>
<td>Rust (CPU)</td>
<td>~0.02 s</td>
<td><strong>25x</strong></td>
</tr>
</tbody>
</table>
<h2 id="Provenance">Provenance<a class="anchor-link" href="#Provenance">¶</a></h2><ul>
<li><strong>Paper:</strong> Bazavov et al., PRD <strong>92</strong>, 076003 (2015)</li>
<li><strong>Validation:</strong> <code>barracuda/src/lattice/abelian_higgs.rs</code>, <code>validate_abelian_higgs</code></li>
<li><strong>Control:</strong> <code>control/abelian_higgs/scripts/abelian_higgs_hmc.py</code></li>
<li><strong>Next:</strong> Extend to (2+1)D, finite-density via chemical potential $\mu$</li>
</ul>
<hr/>
<p><em>hotSpring — ecoPrimals · AGPL-3.0 · <a href="https://primals.eco">primals.eco</a></em></p>
</div>
</div>
</div>
