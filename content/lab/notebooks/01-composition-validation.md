+++
title = "01 — Composition Validation"
description = "Rendered from 01-composition-validation.ipynb"
date = 2026-06-16
weight = 50

[extra]
domain = "Lab"
rendered_from = "01-composition-validation.ipynb"
+++

<!-- Auto-generated from 01-composition-validation.ipynb by spore-validate render-notebooks -->

# 01 — Composition Validation

**neuralSpring sporePrint** | Session S188 | May 2026

Deploy graph structure, bond types, capability profiles, and discovery tiers.

**Data sources:** `validation-state.json`, `cross-spring-matrix.json`

**For other springs:** Replace capability lists with your spring's niche surface.
Replace deploy graph node counts and fragment lists with your own
`graphs/<spring>_deploy.toml` data.

```python
import json
from pathlib import Path
import matplotlib.pyplot as plt
import matplotlib.patches as mpatches

RESULTS = Path('..') / 'experiments' / 'results'

with open(RESULTS / 'validation-state.json') as f:
    vs = json.load(f)

with open(RESULTS / 'cross-spring-matrix.json') as f:
    cs = json.load(f)

print(f"neuralSpring v{vs['version']} — Session {vs['session']}")
```

## Capability Surface

neuralSpring advertises 30 capabilities across 9 domains.
All are registered in `niche.rs`, `config.rs`, `capability_registry.toml`,
and MCP tool definitions.

```python
caps = vs['capabilities']
domains = {k: v for k, v in caps.items() if k != 'total'}

PASS = '#2ecc71'
INFO = '#3498db'

fig, ax = plt.subplots(figsize=(10, 5))
bars = ax.barh(list(domains.keys()), list(domains.values()), color=INFO)
ax.set_xlabel('Capabilities')
ax.set_title(f'neuralSpring Capability Surface ({caps["total"]} total)')
for bar, val in zip(bars, domains.values()):
    ax.text(bar.get_width() + 0.2, bar.get_y() + bar.get_height()/2,
            str(val), va='center', fontweight='bold')
plt.tight_layout()
plt.show()
```

## Deploy Graph Structure

The deploy graph (`neuralspring_deploy.toml`) defines 14 nodes across
3 fragments: `tower_atomic`, `node_atomic`, `meta_tier`.

```python
dg = vs['deploy_graph']
proto = cs['proto_nucleate']

fig, axes = plt.subplots(1, 2, figsize=(12, 4))

# Deploy graph summary
labels = ['Nodes', 'Capabilities', 'Fragments']
values = [dg['nodes'], dg['capabilities_provided'], len(dg['fragments'])]
colors = [INFO, PASS, '#9b59b6']
axes[0].bar(labels, values, color=colors)
axes[0].set_title('Deploy Graph')
for i, v in enumerate(values):
    axes[0].text(i, v + 0.3, str(v), ha='center', fontweight='bold')

# Proto-nucleate dependencies
deps = proto['depends_on']
axes[1].barh(deps, [1]*len(deps), color=PASS)
axes[1].set_title(f'Proto-Nucleate depends_on ({len(deps)} primals)')
axes[1].set_xlim(0, 1.5)
axes[1].set_xlabel('Required')

plt.tight_layout()
plt.show()

print(f"Bond type: {dg['bond_type']}")
print(f"Trust model: {dg['trust_model']}")
print(f"Fragments: {', '.join(dg['fragments'])}")
```

## Discovery Tiers

The validation chain progresses through 5 tiers, from Python baseline
through guideStone certification.

```python
gs = vs['guidestone']

tiers = [
    ('Tier 1: Python baseline', 397, 'complete'),
    ('Tier 2: Rust CPU proof', vs['tests']['total_workspace'], 'complete'),
    ('Tier 3: GPU/WGSL parity', vs['tests']['rust_gpu_checks'], 'complete'),
    ('Tier 4: Primal IPC', 6, 'wip'),
    ('Tier 5: guideStone', 29, f"Level {gs['level']}")
]

fig, ax = plt.subplots(figsize=(10, 4))
tier_names = [t[0] for t in tiers]
tier_checks = [t[1] for t in tiers]
tier_colors = [PASS if t[2] == 'complete' else '#f39c12' for t in tiers]

bars = ax.barh(tier_names, tier_checks, color=tier_colors)
ax.set_xlabel('Validation Checks')
ax.set_title('Validation Discovery Tiers')
ax.set_xscale('log')

legend_elements = [
    mpatches.Patch(color=PASS, label='Complete'),
    mpatches.Patch(color='#f39c12', label='WIP / Partial')
]
ax.legend(handles=legend_elements, loc='lower right')

plt.tight_layout()
plt.show()
```

## guideStone Readiness

neuralSpring's guideStone is at **Level 3** — bare ALL PASS (29/29 checks,
P1-P5 certified). Levels 4-5 pending live NUCLEUS deployment.

```python
levels = [
    ('L1: Validation exists', True),
    ('L2: Properties documented', True),
    ('L3: Bare guideStone (29/29)', True),
    ('L4: NUCLEUS guideStone', False),
    ('L5: Certified (cross-substrate)', False)
]

fig, ax = plt.subplots(figsize=(8, 3))
colors = [PASS if done else '#e74c3c' for _, done in levels]
ax.barh([l[0] for l in levels], [1]*len(levels), color=colors)
ax.set_xlim(0, 1.2)
ax.set_title(f'guideStone Readiness — Level {gs["level"]}')

legend_elements = [
    mpatches.Patch(color=PASS, label='DONE'),
    mpatches.Patch(color='#e74c3c', label='PENDING')
]
ax.legend(handles=legend_elements)

plt.tight_layout()
plt.show()

print(f"Properties certified: {', '.join(gs['properties_certified'])}")
```

## Summary

| Metric | Value |
|--------|-------|
| Capabilities | 30 (9 domains) |
| Deploy graph nodes | 14 |
| Bond type | Metallic |
| Trust model | InternalNucleus |
| Proto-nucleate deps | 6 primals |
| Validation capabilities | 7 |
| guideStone level | 3 (29/29 bare) |
| Properties certified | P1-P5 |

**Provenance:** [primals.eco](https://primals.eco) |
neuralSpring Session S188 | May 2026

