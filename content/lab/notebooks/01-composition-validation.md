+++
title = "Composition Validation — airSpring"
description = "Rendered from 01-composition-validation.ipynb"
date = 2026-07-18
weight = 50

[extra]
domain = "Lab"
rendered_from = "01-composition-validation.ipynb"
+++

<!-- Auto-generated from 01-composition-validation.ipynb by spore-validate render-notebooks -->

# Composition Validation — airSpring

airSpring is the ecological sciences validation spring in the ecoPrimals ecosystem.
It validates precision agriculture, irrigation science, and environmental systems
through 44 IPC capabilities across 87 experiments.

**Data sources**: `composition_validation.json`, `test_suite_report.json`

**Reproduce**: `cargo run --release --bin validate_biome_graph` (35/35 PASS)

**For other springs**: Replace capability categories and deploy graph names with your
domain. The pattern of niche.rs as canonical source → all deploy surfaces derive from
it eliminates drift.

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

comp = load('composition_validation.json')
tests = load('test_suite_report.json')

caps = comp['primal_capabilities']
print(f"Capabilities: {caps['total']} total, {caps['routable']}/{caps['total']} routable")
print(f"Deploy graphs: {comp['deploy_graphs']['total']}")
print(f"Gaps: {comp['gaps']['open']} open / {comp['gaps']['resolved']} resolved")
print(f"guideStone level: {comp['guidestone']['current_level']} → {comp['guidestone']['target_level']}")
print(f"MCP tools: {comp['mcp_tools']['total']}")
print(f"Tests: {tests['summary']['total_rust_tests']} Rust + {tests['summary']['total_python_checks']} Python")
```

## Capability Distribution

airSpring exposes 44 IPC capabilities organized by domain. The `niche.rs` module
is the single source of truth — deploy TOMLs and cell graphs derive from it.

```python
categories = comp['capability_categories']
cat_sizes = {k: len(v) for k, v in categories.items()}

fig, ax = plt.subplots(figsize=(10, 6))
bars = ax.barh(list(cat_sizes.keys()), list(cat_sizes.values()),
               color='#2ecc71', edgecolor='white')
ax.set_xlabel('Capabilities')
ax.set_title(f'airSpring: {caps["total"]} IPC Capabilities by Category')
for bar, val in zip(bars, cat_sizes.values()):
    ax.text(bar.get_width() + 0.1, bar.get_y() + bar.get_height()/2,
            str(val), va='center', fontsize=9)
plt.tight_layout()
plt.savefig('/tmp/airspring_01_capabilities.png', dpi=150)
plt.show()
```

## Deploy Graph Topology

airSpring defines 4 biomeOS deploy graphs for different composition patterns.

```python
graphs = comp['deploy_graphs']['graphs']
names = [g['name'] for g in graphs]
nodes = [g['nodes'] for g in graphs]

fig, ax = plt.subplots(figsize=(8, 4))
colors = ['#3498db'] * len(graphs)
ax.bar(names, nodes, color=colors, edgecolor='white')
ax.set_ylabel('Nodes')
ax.set_title(f'Deploy Graphs ({comp["deploy_graphs"]["total"]} total)')
plt.xticks(rotation=30, ha='right', fontsize=8)
plt.tight_layout()
plt.savefig('/tmp/airspring_01_graphs.png', dpi=150)
plt.show()
```

## Primal Composition & Gap Status

airSpring's NUCLEUS composition wires 5 primals via IPC directly;
7 remain graph-level only (handled by biomeOS deployment).

```python
composition = comp['primal_composition']
ipc_wired = composition['ipc_wired']
graph_only = composition['graph_level_only']

fig, ax = plt.subplots(figsize=(8, 5))
all_primals = ipc_wired + graph_only
status_colors = ['#2ecc71' if p in ipc_wired else '#e74c3c' for p in all_primals]
ax.barh(all_primals, [1]*len(all_primals), color=status_colors, edgecolor='white')
ax.set_xlim(0, 1.5)
ax.set_xticks([])
for i, p in enumerate(all_primals):
    label = 'IPC wired' if p in ipc_wired else 'graph-level'
    ax.text(1.05, i, label, va='center', fontsize=9)
ax.set_title(f'Primal Composition: {len(ipc_wired)} IPC / {len(graph_only)} graph-level')
plt.tight_layout()
plt.savefig('/tmp/airspring_01_primals.png', dpi=150)
plt.show()
```

## Summary

| Metric | Value |
|--------|-------|
| IPC Capabilities | 44/44 routable |
| Deploy Graphs | 4 validated offline |
| Primals IPC-wired | 5 (toadStool, barraCuda, biomeOS, NestGate, Squirrel) |
| Primals graph-level | 7 (petalTongue, coralReef, BearDog, Songbird, rhizoCrypt, loamSpine, sweetGrass) |
| MCP Tools | 10 (Squirrel-discoverable) |
| guideStone Level | 0 → 1 (blocked on primalSpring dependency) |
| Open Gaps | 9 (AG-001 through AG-011) |

**Provenance**: airSpring v0.10.0 · AGPL-3.0-or-later · [primals.eco](https://primals.eco)

