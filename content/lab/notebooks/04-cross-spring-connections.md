+++
title = "04 — Cross-Spring Connections"
description = "Rendered from 04-cross-spring-connections.ipynb"
date = 2026-06-07
weight = 50

[extra]
domain = "Lab"
rendered_from = "04-cross-spring-connections.ipynb"
+++

<!-- Auto-generated from 04-cross-spring-connections.ipynb by spore-validate render-notebooks -->

# 04 — Cross-Spring Connections

**neuralSpring sporePrint** | Session S188 | May 2026

Primal consumption matrix, ecosystem flows, proto-nucleate
dependencies, and integration status.

**Data sources:** `cross-spring-matrix.json`, `validation-state.json`

**For other springs:** Replace the consumption matrix with your own
primal dependencies. The proto-nucleate structure is spring-specific.

```python
import json
from pathlib import Path
import matplotlib.pyplot as plt
import matplotlib.patches as mpatches

RESULTS = Path('..') / 'experiments' / 'results'

with open(RESULTS / 'cross-spring-matrix.json') as f:
    cs = json.load(f)

with open(RESULTS / 'validation-state.json') as f:
    vs = json.load(f)

PASS = '#2ecc71'
FAIL = '#e74c3c'
INFO = '#3498db'

print(f"neuralSpring v{vs['version']} — Primal consumption matrix")
```

## Primal Consumption Matrix

neuralSpring consumes 8 primals across 4 integration tiers:
upstream (compile-time), lateral (IPC), tower (security), and storage.

```python
consumption = cs['consumption']

status_map = {
    'barracuda': 'active',
    'primalspring': 'active',
    'coralreef': 'open',
    'toadstool': 'open',
    'beardog': 'wip',
    'songbird': 'wip',
    'squirrel': 'wip',
    'nestgate': 'open'
}

color_map = {'active': PASS, 'wip': '#f39c12', 'open': FAIL}

primals = list(consumption.keys())
roles = [consumption[p]['role'] for p in primals]
statuses = [status_map[p] for p in primals]
colors = [color_map[s] for s in statuses]

fig, ax = plt.subplots(figsize=(12, 5))
bars = ax.barh(primals[::-1], [1]*len(primals), color=colors[::-1])
ax.set_xlim(0, 2.5)
ax.set_title('Primal Consumption Matrix')

for i, (p, role) in enumerate(zip(primals[::-1], roles[::-1])):
    ax.text(1.05, i, role, va='center', fontsize=9)

legend_elements = [
    mpatches.Patch(color=PASS, label='Active'),
    mpatches.Patch(color='#f39c12', label='WIP'),
    mpatches.Patch(color=FAIL, label='Open')
]
ax.legend(handles=legend_elements, loc='lower right')

plt.tight_layout()
plt.show()
```

## Ecosystem Flow

neuralSpring sits in the middle of the ecosystem — consuming
barraCuda GPU ops upstream and routing through lateral primals
for composition.

```python
flow = cs['production_flow']

fig, ax = plt.subplots(figsize=(10, 5))

tiers = list(flow.keys())
tier_labels = ['Upstream\n(compile-time)', 'Lateral\n(IPC)', 'Tower\n(security)', 'Storage\n(deploy)']
tier_counts = [len(flow[t]) for t in tiers]
tier_primals = [', '.join([p.split(' (')[0] for p in flow[t]]) for t in tiers]
tier_colors = [PASS, INFO, '#9b59b6', '#f39c12']

bars = ax.bar(tier_labels, tier_counts, color=tier_colors)
ax.set_ylabel('Primals')
ax.set_title('Ecosystem Flow Tiers')

for i, (bar, label) in enumerate(zip(bars, tier_primals)):
    ax.text(i, bar.get_height() + 0.1, label,
            ha='center', fontsize=8, style='italic')

plt.tight_layout()
plt.show()
```

## Proto-Nucleate Dependencies

The proto-nucleate graph defines 7 validation capabilities
across 6 primal dependencies and 3 fragments.

```python
proto = cs['proto_nucleate']

fig, axes = plt.subplots(1, 2, figsize=(12, 4))

# Validation capabilities
caps = proto['validation_capabilities']
cap_colors = []
for c in caps:
    if c.startswith('tensor') or c.startswith('stats'):
        cap_colors.append(INFO)
    elif c.startswith('compute'):
        cap_colors.append('#f39c12')
    elif c.startswith('crypto'):
        cap_colors.append('#9b59b6')
    else:
        cap_colors.append(PASS)

axes[0].barh(caps[::-1], [1]*len(caps), color=cap_colors[::-1])
axes[0].set_xlim(0, 1.5)
axes[0].set_title(f'Validation Capabilities ({len(caps)})')

# Dependencies
deps = proto['depends_on']
axes[1].barh(deps[::-1], [1]*len(deps), color=PASS)
axes[1].set_xlim(0, 1.5)
axes[1].set_title(f'Proto-Nucleate Dependencies ({len(deps)})')

plt.tight_layout()
plt.show()

print(f"Fragments: {', '.join(proto['fragments'])}")
```

## barraCuda Usage Depth

barraCuda is the deepest dependency — 806+ WGSL shaders,
128+ files importing, ~97% GPU coverage.

```python
bc = consumption['barracuda']

usage_domains = list(bc['usage'].keys())
usage_counts = [len(bc['usage'][d]) for d in usage_domains]

fig, ax = plt.subplots(figsize=(10, 4))
bars = ax.barh(usage_domains[::-1], usage_counts[::-1], color=INFO)
ax.set_xlabel('Methods used')
ax.set_title(f'barraCuda Usage by Domain ({bc["version"]})')

for bar, val in zip(bars, usage_counts[::-1]):
    ax.text(bar.get_width() + 0.1, bar.get_y() + bar.get_height()/2,
            str(val), va='center', fontweight='bold')

plt.tight_layout()
plt.show()

print(f"IPC surface: {bc['ipc_surface']}")
print(f"GPU coverage: {bc['gpu_coverage']}")
```

## Summary

| Metric | Value |
|--------|-------|
| Primals consumed | 8 |
| Active integrations | 2 (barraCuda, primalSpring) |
| WIP integrations | 3 (BearDog, Songbird, Squirrel) |
| Open integrations | 3 (coralReef, toadStool, NestGate) |
| Proto-nucleate capabilities | 7 |
| Proto-nucleate dependencies | 6 |
| barraCuda version | v0.3.12 |
| barraCuda WGSL shaders | 806+ |
| barraCuda IPC gaps | 18 |

**Provenance:** [primals.eco](https://primals.eco) |
neuralSpring Session S188 | May 2026

