+++
title = "BTSP Security Deep Dive — primalSpring"
description = "Rendered from 05-btsp-security-deep-dive.ipynb — primalSpring composition validation notebook"
date = 2026-05-08
weight = 50

[extra]
domain = "Lab"
rendered_from = "05-btsp-security-deep-dive.ipynb"
+++

<div class="cell border-box-sizing text_cell rendered"><div class="inner_cell">
<div class="text_cell_render border-box-sizing rendered_html">
<h1 id="BTSP-Security-Deep-Dive-%E2%80%94-primalSpring">BTSP Security Deep Dive — primalSpring<a class="anchor-link" href="#BTSP-Security-Deep-Dive-%E2%80%94-primalSpring">¶</a></h1><p>The most compelling cross-domain insight from primalSpring: the BTSP
(BearDog Transport Security Protocol) convergence story. From plaintext
JSON-RPC to full ChaCha20-Poly1305 AEAD across all 13 primals, with
5 security gaps identified, tracked, and resolved.</p>
<p>This notebook provides the definitive view of ecosystem security posture —
per-primal bind address defaults, encryption state, gap resolution timeline,
and the discovery tier hierarchy that enables primals to find each other
without exposing network surfaces.</p>
<p><strong>Data sources</strong>: <code>experiments/results/security_convergence.json</code>, <code>composition_validation.json</code></p>
<p><strong>Reproduce</strong>: <code>cargo test -p primalspring -- btsp</code> and review <code>docs/PRIMAL_GAPS.md</code></p>
<hr/>
<p><em>For other springs: your security story is about how your domain data
flows through BTSP-protected channels. Show which primals you call and
confirm they're all Phase 3.</em></p>
</div>
</div>
</div>
<div class="cell border-box-sizing code_cell rendered">
</div>
<div class="cell border-box-sizing text_cell rendered"><div class="inner_cell">
<div class="text_cell_render border-box-sizing rendered_html">
<h2 id="Per-Primal-Security-Posture">Per-Primal Security Posture<a class="anchor-link" href="#Per-Primal-Security-Posture">¶</a></h2><p>Every primal's BTSP phase, bind address default, and bind flag.
All 13 are at Phase 3 with <code>127.0.0.1</code> default — no primal listens
on <code>0.0.0.0</code> by default.</p>
</div>
</div>
</div>
<div class="cell border-box-sizing code_cell rendered">
</div>
<div class="cell border-box-sizing code_cell rendered">
</div>
<div class="cell border-box-sizing text_cell rendered"><div class="inner_cell">
<div class="text_cell_render border-box-sizing rendered_html">
<h2 id="Gap-Resolution-Timeline">Gap Resolution Timeline<a class="anchor-link" href="#Gap-Resolution-Timeline">¶</a></h2><p>Five security gaps (PG-55 through PG-59) were identified during the
projectNUCLEUS Phase 2a audit. All were resolved within 3 weeks.</p>
</div>
</div>
</div>
<div class="cell border-box-sizing code_cell rendered">
</div>
<div class="cell border-box-sizing text_cell rendered"><div class="inner_cell">
<div class="text_cell_render border-box-sizing rendered_html">
<h2 id="BTSP-Convergence-Arc">BTSP Convergence Arc<a class="anchor-link" href="#BTSP-Convergence-Arc">¶</a></h2><p>The security protocol evolved through 3 phases over 7 weeks. Phase 2a
audit findings (PG-55–59) drove the final push to full convergence.</p>
</div>
</div>
</div>
<div class="cell border-box-sizing code_cell rendered">
</div>
<div class="cell border-box-sizing text_cell rendered"><div class="inner_cell">
<div class="text_cell_render border-box-sizing rendered_html">
<h2 id="Discovery-Hierarchy-Deep-Dive">Discovery Hierarchy Deep Dive<a class="anchor-link" href="#Discovery-Hierarchy-Deep-Dive">¶</a></h2><p>The 5-tier discovery escalation prevents primals from broadcasting
their presence unnecessarily. Tier 1 (static config) is the most
restrictive; Tier 5 (relay federation) enables cross-network discovery.</p>
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
<th>Security Metric</th>
<th>Value</th>
</tr>
</thead>
<tbody>
<tr>
<td>BTSP Phase</td>
<td>3 (ChaCha20-Poly1305 AEAD)</td>
</tr>
<tr>
<td>Primals at Phase 3</td>
<td>13/13 (100%)</td>
</tr>
<tr>
<td>Bind default</td>
<td>127.0.0.1 (all 13)</td>
</tr>
<tr>
<td>Security gaps tracked</td>
<td>5 (PG-55 – PG-59)</td>
</tr>
<tr>
<td>Security gaps open</td>
<td>0</td>
</tr>
<tr>
<td>Discovery tiers</td>
<td>5 (static → relay)</td>
</tr>
<tr>
<td>Convergence time</td>
<td>7 weeks (plaintext → AEAD)</td>
</tr>
</tbody>
</table>
<p>The ecoPrimals ecosystem has achieved full security convergence.
Every primal defaults to localhost binding, uses AEAD encryption
for all IPC, and supports tiered discovery escalation.</p>
<hr/>
<p><strong>Provenance</strong>: All results are content-addressed via BLAKE3 hashes,
tracked in rhizoCrypt DAG sessions, committed to the loamSpine ledger,
and witnessed with ed25519 signatures via sweetGrass braid.</p>
<p><strong>Reproduce</strong>: See <a href="https://primals.eco/lab/reproduce/">primals.eco/lab/reproduce</a></p>
<p><strong>Source</strong>: <a href="https://github.com/ecoPrimals/primalSpring">ecoPrimals/primalSpring</a></p>
</div>
</div>
</div>
