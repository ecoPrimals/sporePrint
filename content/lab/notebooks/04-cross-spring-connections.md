+++
title = "Cross-Spring Connections — hotSpring"
description = "Rendered from 04-cross-spring-connections.ipynb"
date = 2026-06-13
weight = 50

[extra]
domain = "Lab"
rendered_from = "04-cross-spring-connections.ipynb"
+++

<!-- Auto-generated from 04-cross-spring-connections.ipynb by spore-validate render-notebooks -->

# Cross-Spring Connections — hotSpring

hotSpring consumes 10 primals (9 required + 1 optional) for computational physics.
This notebook maps the primal consumption matrix, ecosystem data flows, and patterns
hotSpring has handed back to the ecosystem — capability-based discovery, convergence
tick models, DAG memoization, and scientific provenance braids.

**Data sources:** `cross_spring_matrix.json`

**Reproduce:** See `docs/PRIMAL_GAPS.md` for gap registry, `tools/hotspring_composition.sh`
for Phase 46 composition patterns.

---

*For other springs:* Map your own primal consumption against the ecosystem matrix.
Document patterns you discover that benefit sibling springs.

```python
import json
from pathlib import Path
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
import numpy as np

RESULTS = Path('..') / 'experiments' / 'results'

def load(name):
    with open(RESULTS / name) as f:
        return json.load(f)

matrix = load('cross_spring_matrix.json')

primals = matrix['primals_consumed']
print(f"Primals consumed: {len(primals)}")
print(f"Patterns handed back: {len(matrix['patterns_handed_back'])}")
print(f"Ecosystem flows: {len(matrix['ecosystem_flows'])}")
for name, info in primals.items():
    critical = '***' if info['critical'] else ''
    print(f"  {name} ({info['domain']}): {info['usage']}{critical}")
```

## Primal Consumption by Domain

hotSpring's NUCLEUS composition requires 9 primals across distinct capability
domains. The **compute triangle** (barraCuda + toadStool + coralReef) is unique
to hotSpring's GPU-heavy physics workloads.

```python
C_PASS = '#2ecc71'
C_INFO = '#3498db'
C_GPU  = '#9b59b6'

domain_colors = {
    'crypto': '#e74c3c', 'discovery': '#f39c12', 'compute': '#9b59b6',
    'math': '#3498db', 'shader': '#1abc9c', 'storage': '#2ecc71',
    'dag': '#e67e22', 'ledger': '#34495e', 'provenance': '#16a085',
    'inference': '#95a5a6'
}

p_names = list(primals.keys())
p_domains = [primals[p]['domain'] for p in p_names]
colors = [domain_colors.get(d, '#95a5a6') for d in p_domains]
critical = [1.0 if primals[p]['critical'] else 0.6 for p in p_names]

fig, axes = plt.subplots(1, 2, figsize=(14, 6))

bars = axes[0].barh(p_names, [1]*len(p_names), color=colors, alpha=critical)
for i, p in enumerate(p_names):
    label = f"{primals[p]['domain']}"
    axes[0].text(0.05, i, label, va='center', fontsize=9, fontweight='bold', color='white')
axes[0].set_title(f'{len(primals)} Primals Consumed (9 required + 1 optional)')
axes[0].set_xticks([])
axes[0].invert_yaxis()

# Contribution patterns
patterns = matrix['primal_contribution_patterns']
pat_names = [k.replace('_', ' ').title() for k in patterns]
pat_counts = [len(patterns[k]['primals']) for k in patterns]
axes[1].bar(pat_names, pat_counts, color=[C_GPU, C_PASS, '#e74c3c'])
axes[1].set_ylabel('Primals Involved')
axes[1].set_title('Unique Contribution Patterns')
for i, (name, p) in enumerate(patterns.items()):
    axes[1].text(i, pat_counts[i] + 0.1, ', '.join(p['primals']),
                 ha='center', fontsize=7, style='italic')

fig.suptitle('hotSpring NUCLEUS Composition — Primal Dependency Map', fontsize=13, fontweight='bold')
plt.tight_layout()
plt.savefig('/tmp/hotspring_04_primals.png', dpi=150, bbox_inches='tight')
plt.show()
```

## Patterns Handed Back to Ecosystem

hotSpring's physics-driven exploration surfaces patterns that benefit the entire
ecosystem — from composition library improvements to primal API recommendations.

```python
handed_back = matrix['patterns_handed_back']

fig, ax = plt.subplots(figsize=(12, 5))

pattern_names = [p['pattern'][:50] + '...' if len(p['pattern']) > 50 else p['pattern'] for p in handed_back]
targets = [p['for'].split(' — ')[0] for p in handed_back]

target_colors = {
    'primalSpring': C_PASS,
    'nucleus_composition_lib.sh': C_INFO,
    'sweetGrass': '#16a085',
    'barraCuda': C_GPU
}
colors = [target_colors.get(t, '#95a5a6') for t in targets]

ax.barh(pattern_names, range(len(handed_back), 0, -1), color=colors)
for i, p in enumerate(handed_back):
    ax.text(0.3, i, f'→ {p["for"][:60]}', va='center', fontsize=7, color='white')

ax.set_title(f'{len(handed_back)} Patterns Handed Back to Ecosystem')
ax.set_xticks([])
ax.invert_yaxis()

plt.tight_layout()
plt.savefig('/tmp/hotspring_04_handback.png', dpi=150, bbox_inches='tight')
plt.show()
```

## Ecosystem Data Flows

hotSpring is a **consumer** of primalSpring composition patterns and a **producer**
of capability-based discovery patterns, sovereign GPU recipes, and compute stress
feedback.

```python
flows = matrix['ecosystem_flows']

fig, ax = plt.subplots(figsize=(10, 4))

flow_labels = [f"{f['from']} → {f['to']}" for f in flows]
flow_types = [f['type'].replace('_', ' ').title() for f in flows]
flow_colors = [C_PASS if f['from'] == 'hotSpring' else C_INFO for f in flows]

ax.barh(flow_labels, [1]*len(flows), color=flow_colors)
for i, f in enumerate(flows):
    ax.text(0.05, i, f['desc'][:70], va='center', fontsize=7, color='white', fontweight='bold')

ax.set_title('Ecosystem Data Flows')
ax.set_xticks([])
ax.invert_yaxis()

from matplotlib.patches import Patch
legend = [Patch(facecolor=C_PASS, label='hotSpring produces'),
          Patch(facecolor=C_INFO, label='hotSpring consumes')]
ax.legend(handles=legend, loc='lower right', fontsize=8)

plt.tight_layout()
plt.savefig('/tmp/hotspring_04_flows.png', dpi=150, bbox_inches='tight')
plt.show()
```

## Validation Summary

| Connection | Detail |
|------------|--------|
| Primals consumed | **10** (9 required + Squirrel optional) |
| Unique patterns | **Compute triangle**, scientific provenance, sovereign GPU |
| Patterns handed back | **5** (capability discovery, convergence tick, DAG memo, braid schema, API feedback) |
| Ecosystem flows | **6** (gap tracking, sovereign pipeline, compute stress, composition patterns, sporePrint) |

---

**Provenance:** All data from `experiments/results/cross_spring_matrix.json`.  
**Gaps:** `docs/PRIMAL_GAPS.md` — 8 active gaps for upstream primal teams.  
**Source:** [hotSpring on GitHub](https://github.com/syntheticChemistry/hotSpring) · [primals.eco](https://primals.eco/lab/springs/hotspring/)

