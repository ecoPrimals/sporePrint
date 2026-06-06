+++
title = "Cross-Spring Connections — primalSpring"
description = "Rendered from 04-cross-spring-connections.ipynb"
date = 2026-06-06
weight = 50

[extra]
domain = "Lab"
rendered_from = "04-cross-spring-connections.ipynb"
+++

<!-- Auto-generated from 04-cross-spring-connections.ipynb by spore-validate render-notebooks -->

# Cross-Spring Connections — primalSpring

primalSpring occupies a unique position in the ecoPrimals ecosystem:
it validates the composition layer that connects all 13 primals to all
8 springs. This notebook visualizes the consumption matrix — which springs
use which primals, how primalSpring validates each connection, and the
ecosystem data flows that make the whole system cohere.

**Data sources**: `experiments/results/cross_spring_matrix.json`

**Reproduce**: Review `graphs/*.toml` and `experiments/exp024_cross_spring_ecology/`

---

*For other springs: map your own primal consumption and cross-spring
data flows. Show which primals you depend on and what you validate.*

```python
import json
from pathlib import Path

import matplotlib
import matplotlib.pyplot as plt
import numpy as np

RESULTS = Path('..') / 'experiments' / 'results'

def load(path):
    with open(RESULTS / path) as f:
        return json.load(f)

matrix = load('cross_spring_matrix.json')

print(f'Role: {matrix["role"]}')
print(f'Springs tracked: {len(matrix["springs"])}')
print(f'Primals in matrix: {len(matrix["primal_consumption_matrix"])}')
print(f'Ecosystem flows: {len(matrix["ecosystem_flows"])}')
```

## Primal Consumption Heatmap

Which primals does each spring consume? The matrix reveals the backbone
primals (BearDog, ToadStool — used by 7 springs each) versus specialized
primals (Songbird, skunkBat — domain-specific consumers).

```python
springs = list(matrix['springs'].keys())
primals = list(matrix['primal_consumption_matrix'].keys())

heat = np.zeros((len(springs), len(primals)))
for i, s in enumerate(springs):
    consumed = matrix['springs'][s]['primals_consumed']
    for j, p in enumerate(primals):
        if p in consumed:
            heat[i, j] = 1

fig, ax = plt.subplots(figsize=(14, 6))
im = ax.imshow(heat, cmap='YlGn', aspect='auto')
ax.set_xticks(range(len(primals)))
ax.set_xticklabels(primals, rotation=45, ha='right', fontsize=9)
ax.set_yticks(range(len(springs)))
ax.set_yticklabels(springs, fontsize=9)
ax.set_title('Spring × Primal Consumption Matrix')

for i in range(len(springs)):
    for j in range(len(primals)):
        if heat[i, j] == 1:
            ax.text(j, i, '●', ha='center', va='center', fontsize=14, color='#27ae60')

totals = heat.sum(axis=0).astype(int)
for j, t in enumerate(totals):
    ax.text(j, len(springs) + 0.1, f'{t}', ha='center', va='top',
            fontsize=9, fontweight='bold', color='#2c3e50')

plt.tight_layout()
plt.savefig('/tmp/primalspring_04_heatmap.png', dpi=150, bbox_inches='tight')
plt.show()
```

## Primal Usage Ranking

BearDog (witness signing) and ToadStool (workload dispatch) are universal —
every spring needs identity and scheduling. Storage primals (rhizoCrypt,
loamSpine, sweetGrass) form the next tier. Compute and UI primals serve
specific domains.

```python
pcm = matrix['primal_consumption_matrix']
sorted_primals = sorted(pcm.items(), key=lambda x: -x[1]['springs_using'])

names = [p[0] for p in sorted_primals]
counts = [p[1]['springs_using'] for p in sorted_primals]
roles = [p[1]['role'] for p in sorted_primals]

fig, ax = plt.subplots(figsize=(12, 6))
colors = ['#2ecc71' if c >= 5 else '#f39c12' if c >= 3 else '#3498db' for c in counts]
bars = ax.barh(names[::-1], counts[::-1], color=colors[::-1])
ax.set_xlabel('Springs consuming this primal')
ax.set_title('Primal Usage Across Springs')
for bar, val, role in zip(bars, counts[::-1], roles[::-1]):
    ax.text(bar.get_width() + 0.15, bar.get_y() + bar.get_height()/2,
            f'{val} — {role}', va='center', fontsize=8)

plt.tight_layout()
plt.savefig('/tmp/primalspring_04_primal_rank.png', dpi=150, bbox_inches='tight')
plt.show()
```

## sporePrint Readiness

Each spring's progress toward live notebook publishing. wetSpring is the
exemplar at Tier 2 (content + notebooks). primalSpring is now joining
at Tier 2. Other springs are wired at Tier 1 (metrics auto-commit).

```python
sp_springs = matrix['springs']
sp_names = list(sp_springs.keys())
sp_tiers = [sp_springs[s]['sporePrint_tier'] for s in sp_names]
sp_nbs = [sp_springs[s]['notebooks_live'] for s in sp_names]

fig, axes = plt.subplots(1, 2, figsize=(12, 5))

tier_colors = {0: '#e74c3c', 1: '#f39c12', 2: '#2ecc71', 3: '#3498db'}
colors = [tier_colors[t] for t in sp_tiers]

ax = axes[0]
bars = ax.barh(sp_names[::-1], sp_tiers[::-1], color=colors[::-1])
ax.set_xlabel('sporePrint Tier')
ax.set_title('Spring sporePrint Readiness')
ax.set_xlim(0, 3.5)
for bar, val in zip(bars, sp_tiers[::-1]):
    ax.text(bar.get_width() + 0.05, bar.get_y() + bar.get_height()/2,
            f'Tier {val}', va='center', fontsize=9)

ax = axes[1]
bars = ax.barh(sp_names[::-1], sp_nbs[::-1], color=colors[::-1])
ax.set_xlabel('Live notebooks')
ax.set_title('Notebooks Live on primals.eco')
for bar, val in zip(bars, sp_nbs[::-1]):
    if val > 0:
        ax.text(bar.get_width() + 0.1, bar.get_y() + bar.get_height()/2,
                str(val), va='center', fontsize=10, fontweight='bold')

plt.suptitle('sporePrint: Spring Ecosystem Status',
             fontsize=13, fontweight='bold')
plt.tight_layout()
plt.savefig('/tmp/primalspring_04_sporeprint.png', dpi=150, bbox_inches='tight')
plt.show()
```

## Ecosystem Flows

Data and validation evidence flow between springs, primals, and
infrastructure. primalSpring sits at the center as the composition validator.

```python
flows = matrix['ecosystem_flows']
print(f'{"From":<20s} {"To":<20s} {"Type":<25s} Description')
print('-' * 90)
for f in flows:
    print(f'{f["from"]:<20s} {f["to"]:<20s} {f["type"]:<25s} {f["desc"]}')
```

## Validation Summary

| Metric | Value |
|--------|-------|
| Springs tracked | 7 (+ primalSpring itself) |
| Primals in matrix | 13 |
| Universal primals (5+ springs) | BearDog, ToadStool |
| Ecosystem flows | 5 directional |
| sporePrint Tier 2 springs | 2 (wetSpring, primalSpring) |

---

**Provenance**: All results are content-addressed via BLAKE3 hashes,
tracked in rhizoCrypt DAG sessions, committed to the loamSpine ledger,
and witnessed with ed25519 signatures via sweetGrass braid.

**Reproduce**: See [primals.eco/lab/reproduce](https://primals.eco/lab/reproduce/)

**Source**: [ecoPrimals/primalSpring](https://github.com/ecoPrimals/primalSpring)

