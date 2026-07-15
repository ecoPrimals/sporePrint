+++
title = "Composition Validation — groundSpring"
description = "Rendered from 01-composition-validation.ipynb"
date = 2026-07-10
weight = 50

[extra]
domain = "computation"
rendered_from = "01-composition-validation.ipynb"
+++

<!-- Auto-generated from 01-composition-validation.ipynb by spore-validate render-notebooks -->

# Composition Validation — groundSpring

groundSpring validates measurement noise characterization across 12 scientific
domains. This notebook examines the NUCLEUS composition: 7 deploy graphs,
20 IPC methods, guideStone Level 4, and the verb reconciliation
that aligned all graph definitions to actual IPC contracts.

**Data sources**: `experiments/results/composition_validation.json`, `test_suite_report.json`

**Reproduce**: `cargo run --bin groundspring_unibin -- certify`

---

*For other springs*: Replace `composition_validation.json` with your own deploy
graph analysis. The cell structure (title → load → charts → summary) stays the same.

```python
import json
import matplotlib
import matplotlib.pyplot as plt
import matplotlib.ticker as ticker
from pathlib import Path

RESULTS = Path('..') / 'experiments' / 'results'
PASS = '#2ecc71'
FAIL = '#e74c3c'
INFO = '#3498db'
WARN = '#f39c12'

def load(name):
    with open(RESULTS / name) as f:
        return json.load(f)

comp = load('composition_validation.json')
tests = load('test_suite_report.json')

print(f"groundSpring {comp['version']} — guideStone Level {comp['guidestone_level']}")
print(f"Deploy graphs: {comp['graphs']['count']} ({comp['graphs']['total_nodes']} nodes)")
print(f"Capabilities provided: {comp['capabilities_provided']['count']}")
print(f"Capabilities consumed: {comp['capabilities_consumed']['count']}")
print(f"Tests: {tests['total_tests']} passed, {tests['total_failed']} failed")
```

## Deploy Graphs & Capabilities

```python
fig, axes = plt.subplots(1, 3, figsize=(16, 5))

# Panel 1: Deploy graphs
graph_names = [g.replace('groundspring_', '') for g in comp['graphs']['names']]
colors = [PASS] * len(graph_names)
axes[0].barh(graph_names, [1] * len(graph_names), color=colors)
axes[0].set_title(f"{comp['graphs']['count']} Deploy Graphs — ALL VALID")
axes[0].set_xlim(0, 1.2)
axes[0].set_xlabel('Validated')

# Panel 2: Capabilities provided (grouped by prefix)
caps = comp['capabilities_provided']['names']
cap_short = [c.replace('measurement.', '') for c in caps]
axes[1].barh(cap_short, range(len(cap_short), 0, -1), color=INFO)
axes[1].set_title(f"{len(caps)} Measurement Capabilities")
axes[1].set_xlabel('Index')

# Panel 3: guideStone properties
gs = comp['guidestone']
bare = gs['bare_properties']
nucleus = gs['nucleus_additive_checks']
labels = [p.replace('_', ' ').title() for p in bare] + [c.replace('_', ' ').title() for c in nucleus]
colors_gs = [PASS] * len(bare) + [WARN] * len(nucleus)
axes[2].barh(labels, [1] * len(labels), color=colors_gs)
axes[2].set_title(f"guideStone L{gs['level']}: {len(bare)} Bare + {len(nucleus)} NUCLEUS")
axes[2].set_xlim(0, 1.2)

plt.tight_layout()
plt.savefig('/tmp/groundspring_01_composition.png', dpi=150, bbox_inches='tight')
plt.show()
```

## Deploy Graph Verb Reconciliation

V124 fixed 4 verb mismatches between deploy graphs and actual IPC contracts.
Springs should validate that every `capability` field in their TOML graphs
corresponds to an actual method in their dispatch table.

```python
fixes = comp['deploy_graph_verbs_reconciled']['fixes']
print(f"Verb fixes applied: {len(fixes)}")
print()
for fix in fixes:
    print(f"  {fix['old']:35s} → {fix['new']}")
```

## Test Suite by Module

```python
modules = tests['modules']
sorted_mods = sorted(modules.items(), key=lambda x: x[1], reverse=True)[:15]
names = [m[0] for m in sorted_mods]
counts = [m[1] for m in sorted_mods]

fig, ax = plt.subplots(figsize=(10, 6))
bars = ax.barh(names[::-1], counts[::-1], color=INFO)
ax.set_xlabel('Test Count')
ax.set_title(f'Top 15 Modules by Test Count (total: {tests["total_tests"]})')
for bar, count in zip(bars, counts[::-1]):
    ax.text(bar.get_width() + 1, bar.get_y() + bar.get_height()/2,
            str(count), va='center', fontsize=9)

plt.tight_layout()
plt.savefig('/tmp/groundspring_01_tests.png', dpi=150, bbox_inches='tight')
plt.show()
```

## Validation Summary

| Metric | Value |
|--------|-------|
| Deploy graphs | 7 validated (incl. nest_sync) |
| Capabilities provided | 20 IPC methods + 3 signals |
| Capabilities consumed | 14 (7 primals) |
| Validation scenarios | 11 (9 Tier 1, 2 Tier 2) |
| guideStone Level | 4 (IPC-first, Tier 4) |
| Verb mismatches fixed | 4 (all reconciled) |
| Tests | 1,123 passed, 0 failed |
| Clippy warnings | 0 (pedantic + nursery) |
| Unsafe blocks | 0 (`#![forbid(unsafe_code)]`) |

**Provenance**: All data from groundSpring V146 (May 25, 2026).
See [Spring Catalog](https://primals.eco/architecture/spring-catalog/) on primals.eco.

