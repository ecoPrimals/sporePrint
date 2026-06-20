+++
title = "Experiment Evidence — hotSpring"
description = "Rendered from 03-experiment-evidence.ipynb"
date = 2026-06-20
weight = 50

[extra]
domain = "Lab"
rendered_from = "03-experiment-evidence.ipynb"
+++

<!-- Auto-generated from 03-experiment-evidence.ipynb by spore-validate render-notebooks -->

# Experiment Evidence — hotSpring

hotSpring has accumulated 181 experiments across 12 physics categories,
reproducing 22 published papers and building a science ladder from quenched QCD
through sovereign GPU compute to NUCLEUS composition validation.

**Data sources:** `experiment_catalog.json`, `security_convergence.json`

**Reproduce:** See `EXPERIMENT_INDEX.md` for per-experiment reproduction commands.

---

*For other springs:* Replace the physics categories with your domain categories.
The experiment catalog structure and science ladder pattern are reusable.

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

catalog = load('experiment_catalog.json')
security = load('security_convergence.json')

print(f"Total experiments: {catalog['total_experiments']}")
print(f"Categories: {len(catalog['categories'])}")
print(f"Papers reproduced: {catalog['papers_reproduced']}")
print(f"Total science cost: {catalog['total_science_cost']}")
print(f"Science ladder: {len(catalog['science_ladder'])} milestones")
```

## Experiment Categories

181 experiments organized by physics domain — from molecular dynamics (Phase A)
through lattice QCD production (Phase H) to sovereign GPU and multi-GPU hardware
validation (Phase K).

```python
C_PASS = '#2ecc71'
C_INFO = '#3498db'
C_GPU  = '#9b59b6'

cats = catalog['categories']
cat_names = [k.replace('_', ' ').title() for k in cats]
cat_counts = [cats[k]['count'] for k in cats]

palette = ['#2ecc71', '#3498db', '#9b59b6', '#1abc9c', '#f39c12',
           '#e67e22', '#e74c3c', '#16a085', '#2c3e50', '#8e44ad',
           '#d35400', '#27ae60']

fig, axes = plt.subplots(1, 2, figsize=(14, 6))

# Bar chart by category
y = np.arange(len(cat_names))
axes[0].barh(y, cat_counts, color=palette[:len(cat_names)])
axes[0].set_yticks(y)
axes[0].set_yticklabels(cat_names, fontsize=8)
axes[0].set_xlabel('Experiments')
axes[0].set_title(f'{catalog["total_experiments"]} Experiments Across {len(cats)} Categories')
axes[0].invert_yaxis()

# Timeline
timeline = catalog['timeline']
phases = list(timeline.keys())
added = [timeline[p]['experiments_added'] for p in phases]
phase_labels = [timeline[p]['period'] for p in phases]
axes[1].bar(phase_labels, added, color=[C_PASS, C_INFO, C_GPU, '#f39c12'])
axes[1].set_ylabel('Experiments Added')
axes[1].set_title('Experiment Growth Timeline')
for i, v in enumerate(added):
    axes[1].text(i, v + 1, str(v), ha='center', fontweight='bold')

fig.suptitle(f'hotSpring: {catalog["total_experiments"]} Experiments, {catalog["papers_reproduced"]} Papers Reproduced', fontsize=13, fontweight='bold')
plt.tight_layout()
plt.savefig('/tmp/hotspring_03_experiments.png', dpi=150, bbox_inches='tight')
plt.show()
```

## Science Ladder

The science ladder traces hotSpring's evolution from basic quenched QCD through
increasingly sophisticated physics to the current state: NUCLEUS composition
validation with sovereign GPU compute on multiple hardware generations.

```python
ladder = catalog['science_ladder']

fig, ax = plt.subplots(figsize=(10, 8))

y = np.arange(len(ladder))
colors = []
for i, step in enumerate(ladder):
    if 'QCD' in step or 'N_f' in step or 'Chuna' in step:
        colors.append('#3498db')
    elif 'GPU' in step or 'Sovereign' in step or 'Firmware' in step or 'Dispatch' in step:
        colors.append('#9b59b6')
    elif 'NUCLEUS' in step or 'Composition' in step or 'guideStone' in step or 'Primal' in step or 'Phase' in step:
        colors.append('#2ecc71')
    elif 'K80' in step or 'Ember' in step or 'Debt' in step:
        colors.append('#f39c12')
    else:
        colors.append('#1abc9c')

ax.barh(y, [1]*len(ladder), color=colors, height=0.7)
for i, step in enumerate(ladder):
    ax.text(0.05, i, step, va='center', fontsize=8, fontweight='bold', color='white')

ax.set_yticks([])
ax.set_xticks([])
ax.set_title(f'Science Ladder: {len(ladder)} Milestones')
ax.invert_yaxis()

from matplotlib.patches import Patch
legend_elements = [
    Patch(facecolor='#3498db', label='Lattice QCD'),
    Patch(facecolor='#9b59b6', label='Sovereign GPU'),
    Patch(facecolor='#2ecc71', label='NUCLEUS Composition'),
    Patch(facecolor='#f39c12', label='Hardware Validation'),
    Patch(facecolor='#1abc9c', label='Physics Infrastructure')
]
ax.legend(handles=legend_elements, loc='lower right', fontsize=8)

plt.tight_layout()
plt.savefig('/tmp/hotspring_03_ladder.png', dpi=150, bbox_inches='tight')
plt.show()
```

## Evolution Timeline

Test count growth tracks the science ladder — each milestone adds coverage.

```python
timeline = security['evolution_timeline']
dates = [t['date'] for t in timeline]
test_counts = [t['tests'] for t in timeline]
events = [t['event'].split(' — ')[1] if ' — ' in t['event'] else t['event'] for t in timeline]

fig, ax = plt.subplots(figsize=(12, 5))
ax.plot(dates, test_counts, 'o-', color=C_PASS, linewidth=2, markersize=8)
for i, (d, t, e) in enumerate(zip(dates, test_counts, events)):
    ax.annotate(f'{t}\n{e}', (d, t), textcoords='offset points',
                xytext=(0, 15), ha='center', fontsize=7, rotation=20)

ax.set_xlabel('Date')
ax.set_ylabel('Library Tests')
ax.set_title('Test Count Evolution: 120 → 993')
plt.xticks(rotation=30, ha='right')
plt.tight_layout()
plt.savefig('/tmp/hotspring_03_evolution.png', dpi=150, bbox_inches='tight')
plt.show()
```

## Validation Summary

| Metric | Value |
|--------|-------|
| Total experiments | **181** |
| Categories | **12** (MD, nuclear EOS L1-L3, lattice QCD, spectral, plasma, GPU, sovereign, composition, firmware, multi-GPU) |
| Papers reproduced | **22** |
| Science ladder | **22 milestones** (quenched QCD → ember gate) |
| Test growth | **120 → 993** (Jan → May 2026) |
| Total science cost | **$0.30** |

---

**Provenance:** All data from `experiments/results/experiment_catalog.json`.  
**Full index:** `EXPERIMENT_INDEX.md` in repo root.  
**Source:** [hotSpring on GitHub](https://github.com/syntheticChemistry/hotSpring) · [primals.eco](https://primals.eco/lab/springs/hotspring/)

