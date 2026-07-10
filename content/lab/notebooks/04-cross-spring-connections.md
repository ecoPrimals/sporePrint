+++
title = "Cross-Spring Connections — groundSpring"
description = "Rendered from 04-cross-spring-connections.ipynb"
date = 2026-07-10
weight = 50

[extra]
domain = "Lab"
rendered_from = "04-cross-spring-connections.ipynb"
+++

<!-- Auto-generated from 04-cross-spring-connections.ipynb by spore-validate render-notebooks -->

# Cross-Spring Connections — groundSpring

groundSpring consumes 5 primals and contributes uncertainty budgets to
every baseCamp paper. This notebook maps the primal consumption matrix,
cross-spring data flows, and ecosystem patterns pioneered by groundSpring.

**Data sources**: `experiments/results/cross_spring_matrix.json`

---

*For other springs*: Replace with your own primal consumption and
cross-spring flow data. The matrix visualization pattern stays the same.

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

matrix = load('cross_spring_matrix.json')

consumed = matrix['primals_consumed']
not_consumed = matrix['primals_not_yet_consumed']
flows = matrix['cross_spring_flows']

print(f"Primals consumed: {len(consumed)}")
print(f"Primals not yet consumed: {len(not_consumed)}")
print(f"Cross-spring flows: {len(flows)}")
print(f"Patterns pioneered: {len(matrix['ecosystem_contribution']['patterns_pioneered'])}")
```

## Primal Consumption Matrix

```python
fig, (ax1, ax2) = plt.subplots(1, 2, figsize=(14, 5))

# Consumed primals with capability counts
primal_names = list(consumed.keys())
cap_counts = [len(consumed[p]['capabilities_used']) for p in primal_names]
statuses = [consumed[p]['status'] for p in primal_names]
colors = [PASS if s == 'validated' else WARN for s in statuses]

ax1.barh(primal_names, cap_counts, color=colors)
ax1.set_xlabel('Capabilities Used')
ax1.set_title(f'{len(consumed)} Primals Consumed')
for i, (name, count) in enumerate(zip(primal_names, cap_counts)):
    role = consumed[name]['role']
    ax1.text(count + 0.2, i, f'({role})', va='center', fontsize=8, color='gray')

# Not-consumed primals
nc_names = list(not_consumed.keys())
nc_reasons = [not_consumed[n]['reason'][:40] for n in nc_names]
ax2.barh(nc_names, [1]*len(nc_names), color='#95a5a6')
ax2.set_title(f'{len(nc_names)} Not Yet Consumed')
for i, reason in enumerate(nc_reasons):
    ax2.text(0.05, i, reason, va='center', fontsize=7)
ax2.set_xlim(0, 1.2)

plt.tight_layout()
plt.savefig('/tmp/groundspring_04_primals.png', dpi=150, bbox_inches='tight')
plt.show()
```

## Cross-Spring Data Flows

```python
print('Cross-Spring Flows:')
print()
for flow in flows:
    direction = f"{flow['from']:15s} → {flow['to']:15s}"
    exps = ', '.join(flow['experiments'])
    print(f"  {direction}  [{flow['capability'][:40]}]  (Exp {exps})")
```

## Patterns Pioneered by groundSpring

```python
patterns = matrix['ecosystem_contribution']['patterns_pioneered']

fig, ax = plt.subplots(figsize=(10, 4))
ax.barh(range(len(patterns)), [1]*len(patterns), color=PASS)
ax.set_yticks(range(len(patterns)))
ax.set_yticklabels(patterns)
ax.set_title(f'{len(patterns)} Patterns Pioneered for Ecosystem Adoption')
ax.set_xlim(0, 1.2)
ax.set_xlabel('Adopted')

plt.tight_layout()
plt.savefig('/tmp/groundspring_04_patterns.png', dpi=150, bbox_inches='tight')
plt.show()
```

## Validation Summary

| Metric | Value |
|--------|-------|
| Primals consumed | 5 (beardog, songbird, toadstool, nestgate, barracuda) |
| Primals not consumed | 7 (low priority or implicit) |
| Cross-spring flows | 7 bidirectional connections |
| Patterns pioneered | 7 for ecosystem-wide adoption |
| barraCuda delegations | 110 (67 CPU + 43 GPU) |

**Provenance**: All data from `groundSpring V143 (May 16, 2026)).
See [Spring Catalog](https://primals.eco/architecture/spring-catalog/) on primals.eco.

