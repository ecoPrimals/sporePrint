+++
title = "Ecosystem Evidence — groundSpring"
description = "Rendered from 03-ecosystem-evidence.ipynb"
date = 2026-06-10
weight = 50

[extra]
domain = "Lab"
rendered_from = "03-ecosystem-evidence.ipynb"
+++

<!-- Auto-generated from 03-ecosystem-evidence.ipynb by spore-validate render-notebooks -->

# Ecosystem Evidence — groundSpring

35 experiments across 10 scientific domains, each proving that Python
baselines can be faithfully ported to sovereign Rust+GPU compute.
This notebook visualizes the experiment catalog, domain distribution,
gap resolution timeline, and security posture.

**Data sources**: `experiments/results/experiment_catalog.json`, `security_gaps.json`

---

*For other springs*: Replace with your experiment catalog and gap registry.
The domain breakdown and gap resolution pattern adapts to any spring.

```python
import json
import matplotlib
import matplotlib.pyplot as plt
from pathlib import Path

RESULTS = Path('..') / 'experiments' / 'results'
PASS = '#2ecc71'
FAIL = '#e74c3c'
INFO = '#3498db'
WARN = '#f39c12'

def load(name):
    with open(RESULTS / name) as f:
        return json.load(f)

catalog = load('experiment_catalog.json')
gaps = load('security_gaps.json')

print(f"Total experiments: {catalog['total_experiments']}")
print(f"Validation checks: {catalog['total_validation_checks']} ({catalog['core_checks']} core + {catalog['nucleus_checks']} NUCLEUS)")
print(f"Math parity: {catalog['math_parity_proven']}")
print(f"Domains: {len(catalog['domains'])}")
print(f"Gaps: {gaps['gaps']['active']}/{gaps['gaps']['total']} active, {gaps['gaps']['resolved']} resolved")
```

## Experiment Distribution by Domain

```python
domains = catalog['domains']
domain_names = list(domains.keys())
domain_counts = [domains[d]['count'] for d in domain_names]
domain_labels = [d.replace('_', ' ').title() for d in domain_names]

fig, (ax1, ax2) = plt.subplots(1, 2, figsize=(14, 6))

colors = plt.cm.Set3([i / len(domain_names) for i in range(len(domain_names))])
wedges, texts, autotexts = ax1.pie(domain_counts, labels=domain_labels,
    autopct='%1.0f%%', colors=colors, startangle=90)
ax1.set_title(f'{catalog["total_experiments"]} Experiments Across {len(domains)} Domains')

ax2.barh(domain_labels[::-1], domain_counts[::-1], color=INFO)
ax2.set_xlabel('Experiment Count')
ax2.set_title('Experiments per Domain')

plt.tight_layout()
plt.savefig('/tmp/groundspring_03_domains.png', dpi=150, bbox_inches='tight')
plt.show()
```

## Rust vs Python Parity Across All Experiments

```python
exps = catalog['experiments']
has_speedup = [e for e in exps if e.get('speedup') and 'x' in str(e['speedup'])]
names = [f"{e['id']}: {e['title'][:25]}" for e in has_speedup]
speeds = [float(str(e['speedup']).replace('x', '')) for e in has_speedup]

fig, ax = plt.subplots(figsize=(12, 8))
colors = [PASS if s >= 10 else INFO if s >= 5 else '#95a5a6' for s in speeds]
ax.barh(names[::-1], speeds[::-1], color=colors[::-1])
ax.axvline(x=1.0, color=FAIL, linestyle='--', alpha=0.5, label='Python baseline')
ax.set_xlabel('Speedup (×)')
ax.set_title(f'Rust vs Python: {len(has_speedup)} Experiments with Measured Speedups')
ax.legend()

plt.tight_layout()
plt.savefig('/tmp/groundspring_03_parity.png', dpi=150, bbox_inches='tight')
plt.show()
```

## Gap Resolution & Security Posture

```python
gap_data = gaps['gaps']
labels = ['Resolved', 'Active (low)', 'Blocked upstream']
counts = [
    gap_data['resolved'],
    gap_data['active'] - gap_data['blocked_upstream'],
    gap_data['blocked_upstream']
]
colors_g = [PASS, WARN, '#95a5a6']

sec = gaps['security_posture']
sec_labels = ['unsafe blocks', 'allow attrs', 'TODO/FIXME', 'prod mocks', 'hardcoded addrs', 'sys crates']
sec_values = [sec['unsafe_blocks'], sec['allow_attributes'], sec['todo_fixme'],
              sec['production_mocks'], sec['hardcoded_addresses'], sec['direct_sys_crates']]

fig, (ax1, ax2) = plt.subplots(1, 2, figsize=(12, 4))

ax1.bar(labels, counts, color=colors_g)
ax1.set_title(f'Gap Registry: {gap_data["total"]} Total')
ax1.set_ylabel('Count')

bar_colors = [PASS if v == 0 else FAIL for v in sec_values]
ax2.barh(sec_labels, sec_values, color=bar_colors)
ax2.set_title('Security Posture — All Zero')
ax2.set_xlabel('Count')
ax2.set_xlim(-0.5, 1)

plt.tight_layout()
plt.savefig('/tmp/groundspring_03_gaps.png', dpi=150, bbox_inches='tight')
plt.show()
```

## Validation Summary

| Metric | Value |
|--------|-------|
| Experiments | 35 across 10 domains |
| Validation checks | 395/395 (340 core + 55 NUCLEUS) |
| Math parity | 29/29 proven |
| Gaps | 4 resolved, 5 active (low), 2 blocked upstream |
| Security | Zero unsafe, zero mocks, zero hardcoded addresses |
| Tolerance tiers | 13 library + 5 epsilon + 25 validation-specific |

**Provenance**: All data from `groundSpring V143 (May 16, 2026)).
See [Spring Catalog](https://primals.eco/architecture/spring-catalog/) on primals.eco.

