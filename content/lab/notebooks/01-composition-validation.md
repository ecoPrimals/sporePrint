+++
title = "Composition Validation — primalSpring"
description = "Rendered from 01-composition-validation.ipynb — primalSpring composition validation notebook"
date = 2026-05-08
weight = 50

[extra]
domain = "Lab"
rendered_from = "01-composition-validation.ipynb"
+++

<div class="cell border-box-sizing text_cell rendered"><div class="inner_cell">
<div class="text_cell_render border-box-sizing rendered_html">
<h1 id="Composition-Validation-%E2%80%94-primalSpring">Composition Validation — primalSpring<a class="anchor-link" href="#Composition-Validation-%E2%80%94-primalSpring">¶</a></h1><p>primalSpring is the meta-validation spring for the ecoPrimals ecosystem.
It validates that all 13 NUCLEUS primals compose correctly — deploy graphs
parse cleanly, bond types assign properly, IPC methods route correctly,
and security (BTSP Phase 3 AEAD) is enforced across every composition.</p>
<p>This notebook loads frozen validation results from primalSpring experiments
and visualizes the evidence that every composition graph, bond type, and
deploy profile passes structural and semantic validation.</p>
<p><strong>Data sources</strong>: <code>experiments/results/composition_validation.json</code>, <code>test_suite_report.json</code></p>
<p><strong>Reproduce</strong>: <code>cargo test --workspace</code> in the primalSpring repository.
See <a href="https://primals.eco/lab/springs/primalspring/">primals.eco/lab/springs/primalspring</a>.</p>
<hr/>
<p><em>For other springs: adapt this pattern by loading your own deploy graph stats
and IPC validation results. The cell structure (load → parse → visualize → provenance)
is the template. Your domain validation replaces composition validation.</em></p>
</div>
</div>
</div>
<div class="cell border-box-sizing code_cell rendered">
</div>
<div class="cell border-box-sizing text_cell rendered"><div class="inner_cell">
<div class="text_cell_render border-box-sizing rendered_html">
<h2 id="Deploy-Graph-Topology">Deploy Graph Topology<a class="anchor-link" href="#Deploy-Graph-Topology">¶</a></h2><p>primalSpring maintains 13 composition graphs covering single-tower,
multi-tower, full NUCLEUS, pipeline, and overlay patterns. Each graph
is parsed, structurally validated (cycle detection, node reachability),
and checked for metadata completeness.</p>
</div>
</div>
</div>
<div class="cell border-box-sizing code_cell rendered">
</div>
<div class="cell border-box-sizing text_cell rendered"><div class="inner_cell">
<div class="text_cell_render border-box-sizing rendered_html">
<h2 id="Discovery-Tier-Coverage">Discovery Tier Coverage<a class="anchor-link" href="#Discovery-Tier-Coverage">¶</a></h2><p>The 5-tier discovery escalation hierarchy determines how primals find each
other at runtime. All 13 support Tier 1 (static config) and Tier 2 (env override).
Higher tiers enable LAN multicast, STUN NAT traversal, and relay federation.</p>
</div>
</div>
</div>
<div class="cell border-box-sizing code_cell rendered">
</div>
<div class="cell border-box-sizing text_cell rendered"><div class="inner_cell">
<div class="text_cell_render border-box-sizing rendered_html">
<h2 id="Test-Suite-by-Module">Test Suite by Module<a class="anchor-link" href="#Test-Suite-by-Module">¶</a></h2><p>The primalSpring test suite covers IPC protocol, bonding chemistry,
deploy composition, security (BTSP), genetics identity, coordination,
and meta-validation. Every module has full coverage.</p>
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
<th>Component</th>
<th>Checks</th>
<th>Status</th>
</tr>
</thead>
<tbody>
<tr>
<td>Deploy graphs (13 TOML)</td>
<td>52/52 structural</td>
<td>PASS</td>
</tr>
<tr>
<td>Bond types (5 chemistry)</td>
<td>48/48 tests</td>
<td>PASS</td>
</tr>
<tr>
<td>Deploy profiles (13 primals)</td>
<td>52/52 features</td>
<td>PASS</td>
</tr>
<tr>
<td>Discovery hierarchy (5 tiers)</td>
<td>13/13 Tier 1+2</td>
<td>PASS</td>
</tr>
<tr>
<td>Test suite (workspace)</td>
<td>613/613 passed</td>
<td>PASS</td>
</tr>
</tbody>
</table>
<p>All 13 NUCLEUS primals compose correctly with BTSP Phase 3 AEAD encryption,
localhost-default bind addresses, and complete deploy profiles.</p>
<hr/>
<p><strong>Provenance</strong>: All results are content-addressed via BLAKE3 hashes,
tracked in rhizoCrypt DAG sessions, committed to the loamSpine ledger,
and witnessed with ed25519 signatures via sweetGrass braid.</p>
<p><strong>Reproduce</strong>: See <a href="https://primals.eco/lab/reproduce/">primals.eco/lab/reproduce</a></p>
<p><strong>Source</strong>: <a href="https://github.com/ecoPrimals/primalSpring">ecoPrimals/primalSpring</a></p>
</div>
</div>
</div>
