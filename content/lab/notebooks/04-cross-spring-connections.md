+++
title = "Cross-Spring Connections — wetSpring"
description = "Rendered from 04-cross-spring-connections.ipynb"
date = 2026-06-12
weight = 50

[extra]
domain = "Lab"
rendered_from = "04-cross-spring-connections.ipynb"
+++

<!-- Auto-generated from 04-cross-spring-connections.ipynb by spore-validate render-notebooks -->

# Cross-Spring Connections — wetSpring

wetSpring consumes 10 primals and exchanges data with 4 other springs.
This notebook visualizes the primal consumption surface, the cross-spring
data exchange patterns, and sporePrint readiness across the ecosystem.

**Data sources**: `experiments/results/cross_spring_matrix.json`, `primal_composition.json`

**Reproduce**: See the deploy graph at `graphs/wetspring_science_nucleus.toml`.

---

*For other springs: document which primals you consume, which springs
you exchange data with, and where your deployment gaps are. This makes
the ecosystem dependency graph explicit.*

```python
import os, json, struct, socket
from pathlib import Path

RESULTS = Path('..') / 'experiments' / 'results'

def load(name):
    with open(RESULTS / name) as f:
        return json.load(f)

TIER = 'frozen'
IPC_SOCKET = os.environ.get('WETSPRING_IPC_SOCKET')

def ipc_call(method, params=None):
    """JSON-RPC call to barracuda IPC — active in Tier 2."""
    sock = socket.socket(socket.AF_UNIX, socket.SOCK_STREAM)
    sock.connect(IPC_SOCKET)
    req = json.dumps({'jsonrpc': '2.0', 'method': method, 'params': params or {}, 'id': 1})
    payload = req.encode()
    sock.sendall(struct.pack('<I', len(payload)) + payload)
    length = struct.unpack('<I', sock.recv(4))[0]
    data = sock.recv(length)
    sock.close()
    return json.loads(data)['result']

if IPC_SOCKET and os.path.exists(IPC_SOCKET):
    try:
        ipc_call('health.check')
        TIER = 'live_ipc'
        print(f'Tier 2 ACTIVE — live IPC via {IPC_SOCKET}')
    except Exception:
        print('Tier 2 socket found but not responding — using frozen data')
else:
    print(f'Tier 1 — frozen data (no IPC socket)')

matrix = load('cross_spring_matrix.json')
comp = load('primal_composition.json')

primals = matrix['primal_consumption']
print(f'Primals consumed: {len(primals)}')
print(f'Path dependencies: {len(matrix["path_dependencies"])}')
print(f'Cross-spring exchanges: {len(matrix["cross_spring_data_exchange"])}')
```

## Primal Consumption Surface

wetSpring consumes 10 primals across 4 roles: security (Tower),
storage (Nest), compute (Node), provenance (Trio), AI, and visualization.

```python
import matplotlib
import matplotlib.pyplot as plt
import numpy as np

p_names = list(primals.keys())
p_caps = [len(primals[p]['capabilities_used']) for p in p_names]
p_status = [primals[p]['status'] for p in p_names]

status_colors = {
    'deployment_gap': '#e74c3c',
    'optional': '#3498db'
}
colors = [status_colors.get(s, '#95a5a6') for s in p_status]

fig, axes = plt.subplots(1, 2, figsize=(16, 6))

# Capability count per primal
ax = axes[0]
bars = ax.barh(p_names, p_caps, color=colors)
ax.set_xlabel('Capabilities consumed')
ax.set_title('Primal Consumption — Capabilities Used')
for bar, val, status in zip(bars, p_caps, p_status):
    label = f'{val} ({status.replace("_", " ")})'
    ax.text(bar.get_width() + 0.1, bar.get_y() + bar.get_height()/2,
            label, va='center', fontsize=8)

# Status summary
ax = axes[1]
status_counts = {}
for s in p_status:
    status_counts[s] = status_counts.get(s, 0) + 1
s_labels = [k.replace('_', ' ').title() for k in status_counts]
s_vals = list(status_counts.values())
s_colors = [status_colors.get(k, '#95a5a6') for k in status_counts]
wedges, texts, autotexts = ax.pie(s_vals, labels=s_labels, autopct='%1.0f%%',
                                   colors=s_colors, startangle=90)
ax.set_title(f'Primal Status — {len(p_names)} primals consumed')

plt.suptitle('wetSpring Primal Consumption Surface',
             fontsize=13, fontweight='bold')
plt.tight_layout()
plt.savefig('/tmp/wetspring_04_consumption.png', dpi=150, bbox_inches='tight')
plt.show()
```

## Cross-Spring Data Exchange

wetSpring exchanges data with 4 springs: healthSpring (Python baselines),
hotSpring (physics models), coralSpring (PFAS patterns), and primalSpring
(gap reports and composition patterns).

```python
exchanges = matrix['cross_spring_data_exchange']

fig, ax = plt.subplots(figsize=(14, 6))

spring_names = list(exchanges.keys())
directions = [exchanges[s]['direction'] for s in spring_names]
purposes = [exchanges[s]['purpose'] for s in spring_names]
data_desc = [exchanges[s]['data'] for s in spring_names]

dir_colors = {'inbound': '#3498db', 'outbound': '#e74c3c', 'bidirectional': '#2ecc71'}
colors = [dir_colors[d] for d in directions]

y_pos = range(len(spring_names))
bars = ax.barh(y_pos, [1]*len(spring_names), color=colors, alpha=0.6)
ax.set_yticks(list(y_pos))
ax.set_yticklabels(spring_names, fontsize=11)
ax.set_xlim(0, 3)
ax.set_xticks([])

for i, (spring, direction, data, purpose) in enumerate(
        zip(spring_names, directions, data_desc, purposes)):
    arrow = {'inbound': '←', 'outbound': '→', 'bidirectional': '↔'}[direction]
    ax.text(1.1, i, f'{arrow}  {data}', va='center', fontsize=9)
    ax.text(1.1, i - 0.25, purpose, va='center', fontsize=7, color='#777')

from matplotlib.patches import Patch
legend_elements = [Patch(facecolor=c, label=n.title(), alpha=0.6)
                   for n, c in dir_colors.items()]
ax.legend(handles=legend_elements, loc='upper right', fontsize=9)
ax.set_title('Cross-Spring Data Exchange — wetSpring')

plt.tight_layout()
plt.savefig('/tmp/wetspring_04_exchange.png', dpi=150, bbox_inches='tight')
plt.show()
```

## sporePrint Readiness

Which springs have shipped their sporePrint notebooks?

```python
readiness = matrix['sporePrint_readiness']
springs = list(readiness.keys())
ready = [readiness[s] for s in springs]

fig, ax = plt.subplots(figsize=(10, 4))
colors = ['#2ecc71' if r else '#e74c3c' for r in ready]
bars = ax.barh(springs, [1]*len(springs), color=colors)
ax.set_xlim(0, 1.5)
ax.set_xticks([])
for bar, r in zip(bars, ready):
    label = 'READY' if r else 'PENDING'
    ax.text(1.05, bar.get_y() + bar.get_height()/2,
            label, va='center', fontsize=10, fontweight='bold',
            color='#2ecc71' if r else '#e74c3c')

total_ready = sum(ready)
ax.set_title(f'sporePrint Readiness — {total_ready}/{len(springs)} springs ready')

# Tier 2: check live composition health
if TIER == 'live_ipc':
    health = ipc_call('composition.science_health', {})
    live_primals = health.get('available', [])
    frozen_primals = [p for p, info in primals.items()
                      if info['status'] != 'optional']
    print(f'Tier 2: {len(live_primals)} primals live vs '
          f'{len(frozen_primals)} required (deployment gaps)')

plt.tight_layout()
plt.savefig('/tmp/wetspring_04_readiness.png', dpi=150, bbox_inches='tight')
plt.show()
```

## Validation Summary

| Connection | Direction | Status |
|-----------|-----------|--------|
| healthSpring (Python baselines) | Inbound | Active |
| hotSpring (physics models) | Bidirectional | Active |
| coralSpring (PFAS patterns) | Outbound | Active |
| primalSpring (gap reports) | Bidirectional | Active |
| 10 primals consumed | 6 deployment gaps | Documented |
| sporePrint readiness | 2/8 springs | In progress |

---

**Provenance**: Cross-spring matrix derived from `barracuda/Cargo.toml`
path dependencies and `graphs/wetspring_science_nucleus.toml` capability
declarations.

**Evolution**: Tier 2 calls `composition.science_health` to compare live
primal availability against the documented consumption surface.

**Source**: [ecoPrimals/wetSpring](https://github.com/ecoPrimals/wetSpring)

