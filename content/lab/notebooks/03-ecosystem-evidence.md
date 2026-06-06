+++
title = "Ecosystem Evidence — primalSpring"
description = "Rendered from 03-ecosystem-evidence.ipynb"
date = 2026-06-06
weight = 50

[extra]
domain = "Lab"
rendered_from = "03-ecosystem-evidence.ipynb"
+++

<!-- Auto-generated from 03-ecosystem-evidence.ipynb by spore-validate render-notebooks -->

# Ecosystem Evidence — primalSpring

primalSpring's 89 experiments form the evidence base for NUCLEUS composition
validation. This notebook maps the experiment catalog across categories,
timeline, and gap resolution — showing how primalSpring systematically
validated every coordination pattern in the ecosystem.

Unlike domain springs (wetSpring validates 16S pipelines, hotSpring validates
QCD lattices), primalSpring validates the **composition layer itself** —
the deploy graphs, bond types, IPC routing, and security model that every
other spring depends on.

**Data sources**: `experiments/results/experiment_catalog.json`, `security_convergence.json`

**Reproduce**: Individual experiments run via `cargo run --release -p <experiment_crate>`

---

*For other springs: replace experiment categories with your domain research areas.
Map your experiments to the papers/evidence they validate.*

```python
import json
from pathlib import Path

import matplotlib
import matplotlib.pyplot as plt

RESULTS = Path('..') / 'experiments' / 'results'

def load(path):
    with open(RESULTS / path) as f:
        return json.load(f)

catalog = load('experiment_catalog.json')
security = load('security_convergence.json')

print(f'Total experiments: {catalog["total_experiments"]}')
print(f'Categories: {len(catalog["categories"])}')
for name, cat in catalog['categories'].items():
    print(f'  {name}: {cat["count"]} experiments — {cat["focus"]}')
```

## Experiment Distribution by Category

89 experiments across 20 tracks, from atomic tower compositions
to frontier work (MCP tools, agentic loops, micro desktop shells).

```python
cats = catalog['categories']
cat_names = [n.replace('_', ' ').title() for n in cats.keys()]
cat_counts = [cats[c]['count'] for c in cats.keys()]

fig, axes = plt.subplots(1, 2, figsize=(16, 6))

palette = plt.cm.Set3(range(len(cat_names)))

ax = axes[0]
wedges, texts, autotexts = ax.pie(
    cat_counts, labels=None, autopct='%1.0f%%',
    colors=palette, startangle=90, pctdistance=0.85)
for t in autotexts:
    t.set_fontsize(7)
ax.legend(cat_names, loc='center left', bbox_to_anchor=(1, 0.5), fontsize=7)
ax.set_title(f'{catalog["total_experiments"]} Experiments — Category Distribution')

ax = axes[1]
bars = ax.barh(cat_names[::-1], cat_counts[::-1], color=palette[::-1])
ax.set_xlabel('Experiments')
ax.set_title('Experiments per Category')
for bar, val in zip(bars, cat_counts[::-1]):
    ax.text(bar.get_width() + 0.2, bar.get_y() + bar.get_height()/2,
            str(val), va='center', fontsize=9)

plt.suptitle('primalSpring Experiment Catalog',
             fontsize=13, fontweight='bold')
plt.tight_layout()
plt.savefig('/tmp/primalspring_03_catalog.png', dpi=150, bbox_inches='tight')
plt.show()
```

## Experiment Growth Timeline

Experiment creation accelerated through Phase 55–59 as BTSP Phase 3
convergence and security gap resolution drove new validation needs.

```python
timeline = catalog['timeline']
periods = ['Phase 40–50\n(Feb–Mar)', 'Phase 50–55\n(Mar–Apr)', 'Phase 55–59\n(Apr–May)']
added = [timeline['phase_40_50']['experiments_added'],
         timeline['phase_50_55']['experiments_added'],
         timeline['phase_55_59']['experiments_added']]
cumulative = [added[0], added[0]+added[1], added[0]+added[1]+added[2]]

fig, axes = plt.subplots(1, 2, figsize=(12, 5))

ax = axes[0]
bars = ax.bar(periods, added, color=['#3498db', '#2ecc71', '#e67e22'])
ax.set_ylabel('Experiments added')
ax.set_title('New Experiments per Phase Window')
for bar, val in zip(bars, added):
    ax.text(bar.get_x() + bar.get_width()/2, bar.get_height() + 0.5,
            str(val), ha='center', fontsize=12, fontweight='bold')

ax = axes[1]
ax.plot(periods, cumulative, 'o-', color='#2c3e50', linewidth=2, markersize=8)
ax.fill_between(range(len(periods)), cumulative, alpha=0.2, color='#3498db')
ax.set_ylabel('Cumulative experiments')
ax.set_title('Experiment Growth')
for i, val in enumerate(cumulative):
    ax.text(i, val + 1.5, str(val), ha='center', fontsize=11, fontweight='bold')

plt.suptitle('primalSpring: Experiment Evolution',
             fontsize=13, fontweight='bold')
plt.tight_layout()
plt.savefig('/tmp/primalspring_03_timeline.png', dpi=150, bbox_inches='tight')
plt.show()
```

## Security Gap Resolution

PG-55 through PG-59 were identified during projectNUCLEUS Phase 2a audit.
Each gap was tracked, validated by specific experiments, and resolved by
upstream primal teams.

```python
gaps = security['gaps']
gap_ids = list(gaps.keys())
gap_titles = [gaps[g]['title'] for g in gap_ids]
gap_sevs = [gaps[g]['severity'] for g in gap_ids]
gap_exps = catalog['gap_resolution_experiments']

sev_colors = {'HIGH': '#e74c3c', 'MEDIUM': '#f39c12', 'LOW': '#3498db'}

print(f'{"Gap":<8s} {"Severity":<8s} {"Status":<10s} {"Title"}')
print('-' * 70)
for gid in gap_ids:
    g = gaps[gid]
    exps = ', '.join(gap_exps.get(gid, []))
    print(f'{gid:<8s} {g["severity"]:<8s} {g["status"]:<10s} {g["title"]}')
    print(f'         Resolution: {g["resolution"]}')
    print(f'         Experiments: {exps}')
    print()
```

```python
# Security convergence timeline
timeline = security['security_timeline']
dates = [e['date'] for e in timeline]
primals = [e['primals'] for e in timeline]
events = [e['event'].split(' — ')[0] for e in timeline]

fig, ax = plt.subplots(figsize=(12, 5))
ax.plot(range(len(dates)), primals, 'o-', color='#2ecc71', linewidth=2, markersize=10)
ax.fill_between(range(len(dates)), primals, alpha=0.2, color='#2ecc71')
ax.set_xticks(range(len(dates)))
ax.set_xticklabels([f'{d}\n{e}' for d, e in zip(dates, events)], fontsize=7, rotation=15)
ax.set_ylabel('Primals at Phase 3')
ax.set_ylim(-1, 15)
ax.set_title('BTSP Security Convergence Timeline')
ax.axhline(y=13, color='#e74c3c', linestyle='--', alpha=0.5, label='Target: 13/13')
ax.legend()
for i, (d, p) in enumerate(zip(dates, primals)):
    ax.text(i, p + 0.5, f'{p}/13', ha='center', fontsize=9, fontweight='bold')

plt.tight_layout()
plt.savefig('/tmp/primalspring_03_security.png', dpi=150, bbox_inches='tight')
plt.show()
```

## Validation Summary

| Metric | Value |
|--------|-------|
| Total experiments | 85 |
| Categories | 15 |
| Security gaps tracked | 5 (PG-55 – PG-59) |
| Security gaps resolved | 5/5 |
| BTSP Phase 3 coverage | 13/13 primals |
| Bind default localhost | 13/13 primals |

---

**Provenance**: All results are content-addressed via BLAKE3 hashes,
tracked in rhizoCrypt DAG sessions, committed to the loamSpine ledger,
and witnessed with ed25519 signatures via sweetGrass braid.

**Reproduce**: See [primals.eco/lab/reproduce](https://primals.eco/lab/reproduce/)

**Source**: [ecoPrimals/primalSpring](https://github.com/ecoPrimals/primalSpring)

