+++
title = "Primal Composition Patterns — wetSpring"
description = "Rendered from 05-primal-composition-patterns.ipynb"
date = 2026-06-28
weight = 50

[extra]
domain = "Lab"
rendered_from = "05-primal-composition-patterns.ipynb"
+++

<!-- Auto-generated from 05-primal-composition-patterns.ipynb by spore-validate render-notebooks -->

# Primal Composition Patterns — wetSpring

wetSpring is the first spring to implement **pure primal composition** with
zero sovereign HTTP fallbacks. This notebook documents the patterns that
emerged: three-tier fetch routing, structured gap reports, provenance
session lifecycle, and capability wire names. These patterns are the
template for all future springs.

**Data sources**: `experiments/results/primal_composition.json`

**Reproduce**: `wetspring validate --scenario gonzales_provenance_chain` in wetSpring.

---

*For other springs: this notebook is the reference for how to integrate
with the primal ecosystem. Copy the fetch routing, gap report, and
provenance patterns. When you find new friction, add it to wateringHole.*

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

comp = load('primal_composition.json')
print(f'Composition model: {comp["composition_model"]}')
print(f'Sovereign fallbacks: {comp["sovereign_http_fallbacks"]}')
print(f'Gap reports: {comp["gap_reports_enabled"]}')
print(f'Deploy graph: {comp["deploy_graph"]["name"]} v{comp["deploy_graph"]["version"]}')
print(f'Graph nodes: {comp["deploy_graph"]["total_nodes"]}')
```

## Three-Tier Fetch Routing

External data (ChEMBL, PubChem) routes through a strict three-tier
hierarchy. Springs never make direct HTTP calls — NestGate handles TLS.
If primals are unavailable, a structured gap report is returned instead
of a fallback.

```python
import matplotlib
import matplotlib.pyplot as plt

tiers = comp['fetch_routing']['tiers']

fig, ax = plt.subplots(figsize=(14, 4))
tier_colors = ['#2ecc71', '#3498db', '#e74c3c']

for i, tier in enumerate(tiers):
    x = i * 4.5
    rect = plt.Rectangle((x, 0.3), 3.8, 2.4, facecolor=tier_colors[i],
                          alpha=0.25, edgecolor=tier_colors[i], linewidth=2)
    ax.add_patch(rect)
    ax.text(x + 1.9, 2.1, f'Tier {tier["tier"]}', ha='center', va='center',
            fontsize=12, fontweight='bold', color=tier_colors[i])
    ax.text(x + 1.9, 1.5, tier['name'].replace('_', ' '), ha='center',
            va='center', fontsize=9)
    ax.text(x + 1.9, 0.8, tier['description'], ha='center', va='center',
            fontsize=7, color='#555', wrap=True)
    if i < len(tiers) - 1:
        ax.annotate('', xy=(x + 4.2, 1.5), xytext=(x + 3.8, 1.5),
                    arrowprops=dict(arrowstyle='->', color='#555', lw=2))
        ax.text(x + 4.0, 1.8, 'fallback', ha='center', fontsize=7, color='#999')

ax.set_xlim(-0.5, 14)
ax.set_ylim(0, 3)
ax.axis('off')
ax.set_title('Three-Tier Fetch Routing — No Sovereign HTTP',
             fontsize=13, fontweight='bold', pad=15)
plt.tight_layout()
plt.savefig('/tmp/wetspring_05_routing.png', dpi=150, bbox_inches='tight')
plt.show()
```

## Deploy Graph Phases

The NUCLEUS deploy graph has 14 nodes across 8 phases, from orchestration
(biomeOS) through the Tower → Nest → Node → Trio → Meta → Science → Facade
pipeline.

```python
phases = comp['deploy_graph']['phases']
phase_names = [p['name'].replace('_', ' ').title() for p in phases]
phase_nodes = [len(p['nodes']) for p in phases]
phase_details = [', '.join(p['nodes']) for p in phases]

fig, ax = plt.subplots(figsize=(14, 5))
colors = ['#1abc9c', '#e74c3c', '#3498db', '#9b59b6',
          '#f39c12', '#2ecc71', '#e67e22', '#34495e']
bars = ax.barh(range(len(phases)), phase_nodes, color=colors[:len(phases)])
ax.set_yticks(range(len(phases)))
ax.set_yticklabels(phase_names, fontsize=9)
ax.set_xlabel('Nodes in phase')
ax.set_title(f'NUCLEUS Deploy Graph — {comp["deploy_graph"]["total_nodes"]} nodes, '
             f'{len(phases)} phases')

for bar, val, detail in zip(bars, phase_nodes, phase_details):
    ax.text(bar.get_width() + 0.1, bar.get_y() + bar.get_height()/2,
            detail, va='center', fontsize=8, color='#555')

plt.tight_layout()
plt.savefig('/tmp/wetspring_05_graph.png', dpi=150, bbox_inches='tight')
plt.show()
```

## Provenance Session Lifecycle

Every data and compute operation in wetSpring is wrapped in a provenance
session: `session.create` → `event.append` (N times) → `session.commit`
→ `braid.create`. The wire names follow the canonical `{DOMAIN}.{OPERATION}`
pattern established by primalSpring V082.

```python
trio = comp['provenance_trio']
lifecycle = trio['session_lifecycle']
wire_names = trio['wire_names']

fig, ax = plt.subplots(figsize=(16, 4))
step_colors = ['#2ecc71', '#3498db', '#f39c12', '#9b59b6']

for i, (step, color) in enumerate(zip(lifecycle, step_colors)):
    x = i * 3.8
    rect = plt.Rectangle((x, 0.5), 3.2, 2, facecolor=color,
                          alpha=0.25, edgecolor=color, linewidth=2)
    ax.add_patch(rect)
    ax.text(x + 1.6, 1.9, step, ha='center', va='center',
            fontsize=10, fontweight='bold')
    if i < len(lifecycle) - 1:
        ax.annotate('', xy=(x + 3.5, 1.5), xytext=(x + 3.2, 1.5),
                    arrowprops=dict(arrowstyle='->', color='#555', lw=2))

# Wire name details below
for i, (key, info) in enumerate(wire_names.items()):
    x = i * 3.8
    ax.text(x + 1.6, 0.8, f'{info["domain"]}.{info["operation"]}',
            ha='center', va='center', fontsize=8, color='#555')
    ax.text(x + 1.6, 0.4, info['primal'], ha='center', va='center',
            fontsize=7, color='#999')

ax.set_xlim(-0.5, 16)
ax.set_ylim(0, 3)
ax.axis('off')
ax.set_title(f'Provenance Session Lifecycle — {trio["witness_wire"]}',
             fontsize=13, fontweight='bold', pad=15)
plt.tight_layout()
plt.savefig('/tmp/wetspring_05_provenance.png', dpi=150, bbox_inches='tight')
plt.show()
```

## Evolution Status

What's code-complete, what needs primals running (deployment gaps),
and what code debt remains.

```python
status = comp['evolution_status']

fig, axes = plt.subplots(1, 3, figsize=(18, 5))

categories = [
    ('Code Complete', status['code_complete'], '#2ecc71'),
    ('Deployment Gaps', status['deployment_gaps'], '#e74c3c'),
    ('Code Debt', status['code_debt'], '#f39c12')
]

for ax, (title, items, color) in zip(axes, categories):
    y_pos = range(len(items))
    ax.barh(y_pos, [1]*len(items), color=color, alpha=0.3)
    ax.set_yticks(list(y_pos))
    short_labels = [item[:40] + ('...' if len(item) > 40 else '') for item in items]
    ax.set_yticklabels(short_labels, fontsize=7)
    ax.set_xlim(0, 1.2)
    ax.set_xticks([])
    ax.set_title(f'{title} ({len(items)})', fontsize=11, fontweight='bold', color=color)

# Tier 2: live gap report demo
if TIER == 'live_ipc':
    result = ipc_call('data.fetch.chembl', {'chembl_id': 'CHEMBL2103874'})
    if result.get('gap_report'):
        gap_primals = result['missing_primals']
        axes[1].set_title(f'Deployment Gaps ({len(gap_primals)} live)',
                          fontsize=11, fontweight='bold', color='#e74c3c')
        print(f'Tier 2: Live gap report — {gap_primals}')
    else:
        print(f'Tier 2: ChEMBL fetch succeeded via primal composition!')

plt.suptitle('wetSpring Evolution Status — Pure Primal Composition',
             fontsize=13, fontweight='bold')
plt.tight_layout()
plt.savefig('/tmp/wetspring_05_evolution.png', dpi=150, bbox_inches='tight')
plt.show()
```

## Gap Report Structure

When a required primal is unavailable, wetSpring returns a structured
JSON gap report instead of falling back to sovereign HTTP or disk.
This surfaces missing capabilities cleanly for primalSpring handoff.

```python
gap = comp['gap_report_structure']
print('Gap Report Fields:')
for field in gap['fields']:
    print(f'  - {field}')
print(f'\nAction pattern: {gap["action_pattern"]}')
print(f'\nExample gap report:')
print(json.dumps(gap['example'], indent=2))
```

## Tier 3 Vision — gAIa Artifact Evolution

The final form of these notebooks is not static HTML — it is a **live
gAIa artifact** where:

1. **biomeOS routes** every computation through `capability.call`, so
   the notebook is a composition consumer, not an implementation.

2. **Provenance trio wraps** every cell execution: `session.create` before
   the cell, `event.append` for each intermediate result, `session.commit`
   after the cell, `braid.create` to link the notebook run into the
   global provenance graph.

3. **petalTongue renders** visualizations server-side using its grammar
   of graphics engine (manim-style), replacing matplotlib with
   `GrammarExpr` specifications that are resolution-independent.

4. **Click any data point** and the provenance chain lets you trace back
   to the source paper, the external API call (with BLAKE3 hash), the
   Rust computation, and the primal composition that produced it.

5. **Reproduction envelope** — the provenance metadata embeds enough
   context to recreate the entire computation environment locally via
   plasmidBin, allowing anyone to fork, modify, and contribute back
   through ionic contracts.

This is the path from static notebook to living gAIa commons artifact.
wetSpring's Tier 1 (frozen) and Tier 2 (live IPC) are stepping stones
on this evolution.

---

**Source**: [ecoPrimals/wetSpring](https://github.com/ecoPrimals/wetSpring)

## Validation Summary

| Pattern | Status |
|---------|--------|
| Pure primal composition | Code-complete |
| Three-tier fetch routing | Code-complete |
| Structured gap reports | Code-complete |
| Provenance session wrapping | Code-complete |
| Wire name alignment (V082) | Code-complete |
| BLAKE3 content hashing | Code-complete |
| Deploy graph v4.0 (14 nodes) | Code-complete |
| biomeOS domain routing (v2.92) | Resolved upstream |
| Deployment gaps (6 primals) | Awaiting primal deployment |

---

**Provenance**: Composition patterns documented in V138 handoff.
Wire names aligned per primalSpring V082. Deploy graph validated.

**Evolution**: Tier 1 → Tier 2 (live IPC parity) → Tier 3 (gAIa artifact).

**Source**: [ecoPrimals/wetSpring](https://github.com/ecoPrimals/wetSpring)

