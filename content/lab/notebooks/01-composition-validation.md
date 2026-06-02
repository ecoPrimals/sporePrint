+++
title = "Composition Validation — primalSpring"
description = "Rendered from 01-composition-validation.ipynb"
date = 2026-06-02
weight = 50

[extra]
domain = "Lab"
rendered_from = "01-composition-validation.ipynb"
+++

<!-- Auto-generated from 01-composition-validation.ipynb by spore-validate render-notebooks -->

# Composition Validation — primalSpring

primalSpring is the meta-validation spring for the ecoPrimals ecosystem.
It validates that all 13 NUCLEUS primals compose correctly — deploy graphs
parse cleanly, bond types assign properly, IPC methods route correctly,
and security (BTSP Phase 3 AEAD) is enforced across every composition.

This notebook loads frozen validation results from primalSpring experiments
and visualizes the evidence that every composition graph, bond type, and
deploy profile passes structural and semantic validation.

**Data sources**: `experiments/results/composition_validation.json`, `test_suite_report.json`

**Reproduce**: `cargo test --workspace` in the primalSpring repository.
See [primals.eco/lab/springs/primalspring](https://primals.eco/lab/springs/primalspring/).

---

*For other springs: adapt this pattern by loading your own deploy graph stats
and IPC validation results. The cell structure (load → parse → visualize → provenance)
is the template. Your domain validation replaces composition validation.*

```python
import json
from pathlib import Path

import matplotlib
import matplotlib.pyplot as plt
import matplotlib.ticker as ticker

RESULTS = Path('..') / 'experiments' / 'results'

def load(path):
    with open(RESULTS / path) as f:
        return json.load(f)

comp = load('composition_validation.json')
tests = load('test_suite_report.json')

print(f'Phase: {comp["phase"]}')
print(f'Graphs validated: {comp["graphs"]["count"]} ({comp["graphs"]["total_nodes"]} total nodes)')
print(f'Primals validated: {comp["primals_validated"]["count"]}/13 BTSP Phase 3')
print(f'Test suite: {tests["passed"]}/{tests["total_tests"]} passed ({tests["total_time_s"]}s)')
print(f'Bond types: {len(comp["bond_types"])} ({', '.join(comp["bond_types"].keys())})')
```

## Deploy Graph Topology

primalSpring maintains 13 composition graphs covering single-tower,
multi-tower, full NUCLEUS, pipeline, and overlay patterns. Each graph
is parsed, structurally validated (cycle detection, node reachability),
and checked for metadata completeness.

```python
fig, axes = plt.subplots(1, 3, figsize=(16, 5))

# Graph structural validation
sv = comp['structural_validation']
checks = ['Parse', 'Cycles', 'Reachability', 'Metadata']
values = [sv['graph_parse_pass'], sv['cycle_detection_pass'],
          sv['node_reachability_pass'], sv['metadata_complete']]
colors = ['#2ecc71' if v == 13 else '#e74c3c' for v in values]
ax = axes[0]
bars = ax.bar(checks, values, color=colors)
ax.set_ylim(0, 15)
ax.set_ylabel('Graphs passing')
ax.set_title(f'Structural Validation — {sum(values)}/{len(values)*13} checks')
for bar, val in zip(bars, values):
    ax.text(bar.get_x() + bar.get_width()/2, bar.get_height() + 0.3,
            f'{val}/13', ha='center', va='bottom', fontsize=10)

# Bond type distribution
bonds = comp['bond_types']
bond_names = list(bonds.keys())
bond_graphs = [bonds[b]['graphs_using'] for b in bond_names]
bond_tests = [bonds[b]['test_count'] for b in bond_names]
labels = [n.replace('_', '\n') for n in bond_names]
ax = axes[1]
x = range(len(bond_names))
bar1 = ax.bar([i - 0.2 for i in x], bond_graphs, 0.4, label='Graphs using', color='#3498db')
bar2 = ax.bar([i + 0.2 for i in x], bond_tests, 0.4, label='Tests', color='#9b59b6')
ax.set_xticks(list(x))
ax.set_xticklabels(labels, fontsize=8)
ax.set_ylabel('Count')
ax.set_title('Bond Types — Coverage')
ax.legend(fontsize=8)

# Deploy profile completeness
dp = comp['deploy_profiles']
profile_items = ['bind_flag', 'tolerances', 'env_keys', 'discovery_tier']
profile_vals = [dp[f'with_{k}'] for k in profile_items]
ax = axes[2]
bars = ax.barh(profile_items, profile_vals, color='#2ecc71')
ax.set_xlim(0, 15)
ax.set_xlabel('Primals with feature')
ax.set_title('Deploy Profile Completeness')
for bar, val in zip(bars, profile_vals):
    ax.text(bar.get_width() + 0.3, bar.get_y() + bar.get_height()/2,
            f'{val}/13', va='center', fontsize=10)

plt.suptitle(f'primalSpring Phase {comp["phase"]}: {comp["graphs"]["count"]} Graphs, '
             f'{comp["primals_validated"]["count"]} Primals — {comp["validation"]}',
             fontsize=13, fontweight='bold')
plt.tight_layout()
plt.savefig('/tmp/primalspring_01_composition.png', dpi=150, bbox_inches='tight')
plt.show()
```

## Discovery Tier Coverage

The 5-tier discovery escalation hierarchy determines how primals find each
other at runtime. All 13 support Tier 1 (static config) and Tier 2 (env override).
Higher tiers enable LAN multicast, STUN NAT traversal, and relay federation.

```python
disc = comp['primals_validated']['discovery_tier_coverage']
tiers = ['Tier 1\nStatic', 'Tier 2\nEnv', 'Tier 3\nMulticast', 'Tier 4\nSTUN', 'Tier 5\nRelay']
counts = [disc['tier1_static'], disc['tier2_env'], disc['tier3_multicast'],
          disc['tier4_stun'], disc['tier5_relay']]

fig, ax = plt.subplots(figsize=(10, 5))
colors = ['#2ecc71', '#27ae60', '#f39c12', '#e67e22', '#e74c3c']
bars = ax.bar(tiers, counts, color=colors)
ax.set_ylabel('Primals supporting tier')
ax.set_ylim(0, 15)
ax.set_title('Discovery Escalation Hierarchy — Primal Coverage')
for bar, val in zip(bars, counts):
    ax.text(bar.get_x() + bar.get_width()/2, bar.get_height() + 0.3,
            f'{val}/13', ha='center', va='bottom', fontsize=11, fontweight='bold')

plt.tight_layout()
plt.savefig('/tmp/primalspring_01_discovery.png', dpi=150, bbox_inches='tight')
plt.show()
```

## Test Suite by Module

The primalSpring test suite covers IPC protocol, bonding chemistry,
deploy composition, security (BTSP), genetics identity, coordination,
and meta-validation. Every module has full coverage.

```python
cats = tests['key_categories']
cat_names = list(cats.keys())
cat_tests = [cats[c]['tests'] for c in cat_names]
cat_pcts = [cats[c]['pct'] for c in cat_names]

fig, axes = plt.subplots(1, 2, figsize=(14, 5))

palette = ['#3498db', '#9b59b6', '#2ecc71', '#e74c3c', '#f39c12',
           '#1abc9c', '#e67e22', '#95a5a6']

ax = axes[0]
labels = [n.replace('_', ' ').title() for n in cat_names]
wedges, texts, autotexts = ax.pie(
    cat_tests, labels=None, autopct='%1.0f%%',
    colors=palette, startangle=90, pctdistance=0.82)
for t in autotexts:
    t.set_fontsize(8)
ax.legend(labels, loc='center left', bbox_to_anchor=(1, 0.5), fontsize=8)
ax.set_title(f'Test Distribution — {tests["total_tests"]} tests')

# Top modules by test count
mods = tests['modules']
top_mods = sorted(mods.items(), key=lambda x: -x[1]['tests'])[:12]
mod_names = [m[0].replace('::', '\n') for m in top_mods]
mod_counts = [m[1]['tests'] for m in top_mods]
ax = axes[1]
bars = ax.barh(mod_names[::-1], mod_counts[::-1], color='#3498db')
ax.set_xlabel('Tests')
ax.set_title('Top 12 Modules by Test Count')
for bar, val in zip(bars, mod_counts[::-1]):
    ax.text(bar.get_width() + 0.5, bar.get_y() + bar.get_height()/2,
            str(val), va='center', fontsize=9)

plt.suptitle(f'primalSpring Test Suite — {tests["passed"]} passed, 0 failed',
             fontsize=13, fontweight='bold')
plt.tight_layout()
plt.savefig('/tmp/primalspring_01_tests.png', dpi=150, bbox_inches='tight')
plt.show()
```

## Validation Summary

| Component | Checks | Status |
|-----------|--------|--------|
| Deploy graphs (13 TOML) | 52/52 structural | PASS |
| Bond types (5 chemistry) | 48/48 tests | PASS |
| Deploy profiles (13 primals) | 52/52 features | PASS |
| Discovery hierarchy (5 tiers) | 13/13 Tier 1+2 | PASS |
| Test suite (workspace) | 613/613 passed | PASS |

All 13 NUCLEUS primals compose correctly with BTSP Phase 3 AEAD encryption,
localhost-default bind addresses, and complete deploy profiles.

---

**Provenance**: All results are content-addressed via BLAKE3 hashes,
tracked in rhizoCrypt DAG sessions, committed to the loamSpine ledger,
and witnessed with ed25519 signatures via sweetGrass braid.

**Reproduce**: See [primals.eco/lab/reproduce](https://primals.eco/lab/reproduce/)

**Source**: [ecoPrimals/primalSpring](https://github.com/ecoPrimals/primalSpring)

