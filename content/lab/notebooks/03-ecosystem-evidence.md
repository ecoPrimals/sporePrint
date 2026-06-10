+++
title = "Ecosystem Evidence — airSpring"
description = "Rendered from 03-ecosystem-evidence.ipynb"
date = 2026-06-10
weight = 50

[extra]
domain = "Lab"
rendered_from = "03-ecosystem-evidence.ipynb"
+++

<!-- Auto-generated from 03-ecosystem-evidence.ipynb by spore-validate render-notebooks -->

# Ecosystem Evidence — airSpring

87 experiments validating precision agriculture and irrigation science.
1,284 Python baselines → 1,364 Rust tests → 91 validation binaries.
60 named tolerances with full provenance tracking.

**Data sources**: `experiment_catalog.json`, `test_suite_report.json`, `security_convergence.json`

**Reproduce**: `cargo test --lib && cargo test --tests --all-features`

**For other springs**: Replace experiment categories with your domain areas.
The pattern of categorized experiments with check counts and named tolerances
applies universally.

```python
import json
from pathlib import Path

import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt

RESULTS = Path('..') / 'experiments' / 'results'

def load(name):
    with open(RESULTS / name) as f:
        return json.load(f)

catalog = load('experiment_catalog.json')
tests = load('test_suite_report.json')
security = load('security_convergence.json')

print(f"Total experiments: {catalog['total_experiments']}")
print(f"  Complete: {catalog['status_breakdown']['complete']}")
print(f"  Active: {catalog['status_breakdown']['active']}")
print(f"Categories: {len(catalog['categories'])}")
print(f"Tolerances: {tests['tolerances']['total_named']} named, {tests['tolerances']['submodules']} submodules")
```

## Experiment Distribution by Category

```python
categories = catalog['categories']
cat_names = [c['name'] for c in categories]
cat_counts = [len(c['experiments']) for c in categories]
cat_checks = [c['total_checks'] for c in categories]

fig, (ax1, ax2) = plt.subplots(1, 2, figsize=(14, 6))

ax1.barh(cat_names, cat_counts, color='#3498db', edgecolor='white')
ax1.set_xlabel('Experiments')
ax1.set_title(f'Experiments by Category ({catalog["total_experiments"]} total)')
for i, v in enumerate(cat_counts):
    ax1.text(v + 0.1, i, str(v), va='center', fontsize=9)

ax2.barh(cat_names, cat_checks, color='#2ecc71', edgecolor='white')
ax2.set_xlabel('Validation Checks')
ax2.set_title('Validation Checks by Category')
for i, v in enumerate(cat_checks):
    ax2.text(v + 5, i, str(v), va='center', fontsize=9)

plt.tight_layout()
plt.savefig('/tmp/airspring_03_categories.png', dpi=150)
plt.show()
```

## Test Suite Composition

```python
test_cats = tests['categories']
labels = [c['name'] for c in test_cats]
counts = [c['count'] for c in test_cats]
colors = ['#2ecc71', '#3498db', '#9b59b6', '#e74c3c', '#f39c12', '#1abc9c', '#34495e']

fig, ax = plt.subplots(figsize=(8, 8))
wedges, texts, autotexts = ax.pie(counts, labels=labels, colors=colors[:len(labels)],
                                   autopct='%1.0f%%', startangle=90, pctdistance=0.85)
for text in texts:
    text.set_fontsize(8)
for autotext in autotexts:
    autotext.set_fontsize(7)
total = sum(counts)
ax.set_title(f'Test Suite: {total:,} total checks')
plt.tight_layout()
plt.savefig('/tmp/airspring_03_tests.png', dpi=150)
plt.show()
```

## Quality Gates & Safety

```python
safety = security['rust_safety']
deps = security['dependency_security']
validation = security['validation_integrity']

gates = [
    ('forbid(unsafe_code)', safety['forbid_unsafe_code']),
    ('deny(cast_*)', safety['deny_cast_lints']),
    ('deny(unwrap_used)', safety['deny_clippy_unwrap']),
    ('warn(missing_docs)', safety['warn_missing_docs']),
    ('zero #[allow()]', safety['zero_allow_attributes']),
    ('#[expect(reason)]', safety['expect_with_reason']),
    ('cargo-deny clean', deps['cargo_deny_clean']),
    ('zero C deps', deps['c_dependencies'] == 0),
    ('ecoBin compliant', deps['ecobin_compliant']),
    ('zero-panic (91 bins)', validation['zero_panic_binaries'] == 91),
    ('determinism contract', validation['determinism_contract']),
    (f'{validation["named_tolerances"]} named tolerances', True),
]

fig, ax = plt.subplots(figsize=(8, 5))
gate_names = [g[0] for g in gates]
gate_pass = [1 if g[1] else 0 for g in gates]
gate_colors = ['#2ecc71' if g[1] else '#e74c3c' for g in gates]
ax.barh(gate_names, gate_pass, color=gate_colors, edgecolor='white')
ax.set_xlim(0, 1.5)
ax.set_xticks([])
for i, (name, passed) in enumerate(gates):
    ax.text(1.05, i, 'PASS' if passed else 'FAIL', va='center',
            color='#2ecc71' if passed else '#e74c3c', fontweight='bold', fontsize=9)
ax.set_title('Quality Gates')
plt.tight_layout()
plt.savefig('/tmp/airspring_03_gates.png', dpi=150)
plt.show()
```

## Summary

| Metric | Value |
|--------|-------|
| Experiments | 87 (86 complete, 1 active) |
| Python baselines | 1,284 checks |
| Rust tests | 1,364 (986 lib + 316 integration + 62 forge) |
| Validation binaries | 91 (all zero-panic) |
| Line coverage | 90.56% (gated at 90%) |
| Named tolerances | 60 in 5 submodules (Python mirror) |
| Quality gates | 12/12 PASS |
| Provenance baselines | 63 registered |

**Provenance**: airSpring v0.10.0 · AGPL-3.0-or-later · [primals.eco](https://primals.eco)

