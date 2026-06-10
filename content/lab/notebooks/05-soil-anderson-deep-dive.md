+++
title = "Soil Anderson Deep Dive — Track 4 Domain Exemplar"
description = "Rendered from 05-soil-anderson-deep-dive.ipynb"
date = 2026-06-10
weight = 50

[extra]
domain = "Lab"
rendered_from = "05-soil-anderson-deep-dive.ipynb"
+++

<!-- Auto-generated from 05-soil-anderson-deep-dive.ipynb by spore-validate render-notebooks -->

# Soil Anderson Deep Dive — Track 4 Domain Exemplar

Anderson localization — originally a condensed matter physics concept
(waves in disordered media) — applied to soil microbial ecology.
The key insight: quorum sensing (QS) in soil pore networks is
structurally isomorphic to electron transport in disordered lattices.

This notebook loads 9 Track 4 soil experiments, each validated against
a published field study, and visualizes the Anderson-QS connection.

**Data sources**: `experiments/results/170_*` through `178_*`

---

*For other springs: this is what a domain deep-dive looks like. Pick your
most compelling cross-domain connection and build a notebook around it.*

```python
import json
from pathlib import Path

import matplotlib
# matplotlib backend set by environment
import matplotlib.pyplot as plt
import numpy as np

RESULTS = Path('..') / 'experiments' / 'results'

def load_soil(dirname):
    d = RESULTS / dirname
    if d.exists():
        jsons = list(d.glob('*.json'))
        if jsons:
            with open(jsons[0]) as f:
                return json.load(f)
    return None

soil_170 = load_soil('170_soil_qs_pore_geometry')
soil_171 = load_soil('171_soil_pore_diversity')
soil_172 = load_soil('172_soil_distance_colonization')
soil_173 = load_soil('173_notill_brandt_farm')
soil_174 = load_soil('174_notill_meta_analysis')
soil_175 = load_soil('175_notill_longterm_tillage')
soil_176 = load_soil('176_soil_biofilm_aggregate')
soil_177 = load_soil('177_soil_structure_function')
soil_178 = load_soil('178_tillage_microbiome')

loaded = sum(1 for d in [soil_170, soil_171, soil_172, soil_173, soil_174,
                          soil_175, soil_176, soil_177, soil_178] if d is not None)
print(f'Loaded {loaded}/9 soil experiment baselines')
```

## Experiment 170: QS Probability vs Pore Geometry

The Anderson connection: soil pore size controls effective disorder W.
Small pores (clay) → high W → localized (no QS). Large pores (sand) → low W → extended (QS active).

Martinez et al. 2023 provided the pore geometry framework.

```python
if soil_170 and 'pore_mapping' in soil_170:
    pores = soil_170['pore_mapping']
    pore_sizes = [p['pore_um'] for p in pores]
    qs_probs = [p['qs_probability'] for p in pores]
    w_values = [p['effective_w'] for p in pores]
    connectivity = [p['connectivity'] for p in pores]

    fig, axes = plt.subplots(1, 3, figsize=(15, 5))

    # QS probability vs pore size
    ax = axes[0]
    ax.plot(pore_sizes, qs_probs, 'o-', color='#2ecc71', linewidth=2, markersize=8)
    ax.set_xlabel('Pore size (\u00b5m)')
    ax.set_ylabel('QS probability')
    ax.set_title('Quorum Sensing vs Pore Size')
    ax.axhline(y=0.5, color='red', linestyle='--', alpha=0.5, label='Critical threshold')
    ax.legend()
    ax.grid(True, alpha=0.3)

    # Effective disorder W vs pore size
    ax = axes[1]
    ax.plot(pore_sizes, w_values, 's-', color='#e74c3c', linewidth=2, markersize=8)
    ax.set_xlabel('Pore size (\u00b5m)')
    ax.set_ylabel('Effective disorder W')
    ax.set_title('Anderson Disorder Parameter')
    ax.axhline(y=16.5, color='blue', linestyle='--', alpha=0.5, label='W_c (critical)')
    ax.legend()
    ax.grid(True, alpha=0.3)

    # Connectivity vs pore size
    ax = axes[2]
    ax.plot(pore_sizes, connectivity, 'D-', color='#3498db', linewidth=2, markersize=8)
    ax.set_xlabel('Pore size (\u00b5m)')
    ax.set_ylabel('Connectivity')
    ax.set_title('Network Connectivity')
    ax.grid(True, alpha=0.3)

    plt.suptitle('Exp 170: Pore Geometry \u2192 Anderson Disorder \u2192 QS Probability',
                 fontsize=14, fontweight='bold')
    plt.tight_layout()
    plt.savefig('/tmp/wetspring_05_pore_qs.png', dpi=150, bbox_inches='tight')
    plt.show()
else:
    print('Exp 170 data not available')
```

## Chemotaxis as Disorder Reduction

Bacterial chemotaxis reduces the effective disorder by 15%,
shifting the Anderson transition. The benefit is maximal near
the critical disorder W_c = 16.5.

```python
if soil_170 and 'chemotaxis' in soil_170:
    chem = soil_170['chemotaxis']
    sweep = chem['disorder_sweep']

    w_vals = [s['w'] for s in sweep]
    p_no = [s['p_no_chemotaxis'] for s in sweep]
    p_with = [s['p_with_chemotaxis'] for s in sweep]
    benefit = [s['benefit'] for s in sweep]

    fig, axes = plt.subplots(1, 2, figsize=(12, 5))

    ax = axes[0]
    ax.plot(w_vals, p_no, 'o-', color='#e74c3c', label='Without chemotaxis', linewidth=2)
    ax.plot(w_vals, p_with, 's-', color='#2ecc71', label='With chemotaxis', linewidth=2)
    ax.set_xlabel('Disorder W')
    ax.set_ylabel('QS probability')
    ax.set_title('Chemotaxis Shifts the Anderson Transition')
    ax.axvline(x=16.5, color='blue', linestyle='--', alpha=0.5, label='W_c')
    ax.legend()
    ax.grid(True, alpha=0.3)

    ax = axes[1]
    ax.bar(w_vals, benefit, width=2, color='#9b59b6', alpha=0.8)
    ax.set_xlabel('Disorder W')
    ax.set_ylabel('Benefit (\u0394 QS probability)')
    ax.set_title('Chemotaxis Benefit — Maximal Near W_c')
    ax.grid(True, alpha=0.3)

    plt.suptitle(f'Chemotaxis Reduces Effective Disorder by {chem["reduction_pct"]}%',
                 fontsize=14, fontweight='bold')
    plt.tight_layout()
    plt.savefig('/tmp/wetspring_05_chemotaxis.png', dpi=150, bbox_inches='tight')
    plt.show()
else:
    print('Chemotaxis data not available')
```

## Integrated Soil Types

The model applied to real soil types: sandy loam, clay, no-till
aggregates, and tilled soil. Anderson localization predicts QS
activity directly from pore geometry.

```python
if soil_170 and 'integrated' in soil_170:
    soils = soil_170['integrated']

    fig, ax = plt.subplots(figsize=(10, 6))

    names = [s['name'] for s in soils]
    qs_probs = [s['qs_probability'] for s in soils]
    coop = [s['coop_survival'] for s in soils]
    colors = ['#2ecc71' if s['qs_active'] else '#e74c3c' for s in soils]

    x = np.arange(len(names))
    width = 0.35

    bars1 = ax.bar(x - width/2, qs_probs, width, label='QS probability', color=colors, alpha=0.8)
    bars2 = ax.bar(x + width/2, coop, width, label='Cooperative survival', color='#3498db', alpha=0.6)

    ax.set_ylabel('Probability')
    ax.set_title('Soil Type \u2192 QS Activity \u2192 Cooperative Survival')
    ax.set_xticks(x)
    ax.set_xticklabels(names, rotation=15, ha='right', fontsize=9)
    ax.legend()
    ax.axhline(y=0.5, color='gray', linestyle=':', alpha=0.5)
    ax.grid(True, alpha=0.2, axis='y')

    plt.tight_layout()
    plt.savefig('/tmp/wetspring_05_soil_types.png', dpi=150, bbox_inches='tight')
    plt.show()

    print('Soil type predictions:')
    for s in soils:
        status = 'QS ACTIVE' if s['qs_active'] else 'QS SUPPRESSED'
        print(f'  {s["name"]:<40s} W={s["effective_w"]:<6.1f} '
              f'P(QS)={s["qs_probability"]:.4f}  {status}')
else:
    print('Integrated soil data not available')
```

## Track 4 Experiment Overview

9 experiments, each validated against a specific published paper.

```python
experiments = [
    ('170', 'Soil QS Pore Geometry', 'Martinez 2023', soil_170),
    ('171', 'Soil Pore Diversity', 'Feng 2024', soil_171),
    ('172', 'Distance Colonization', 'Mukherjee 2024', soil_172),
    ('173', 'No-Till Brandt Farm', 'Islam 2014', soil_173),
    ('174', 'No-Till Meta-Analysis', 'Zuber 2016', soil_174),
    ('175', 'Long-Term Tillage', 'Liang 2015', soil_175),
    ('176', 'Biofilm Aggregate', 'Tecon 2017', soil_176),
    ('177', 'Structure-Function', 'Rabot 2018', soil_177),
    ('178', 'Tillage Microbiome', 'Wang 2025', soil_178),
]

print(f'{"Exp":>4s}  {"Title":<30s} {"Paper":<18s} {"Data Keys"}')
print('=' * 80)
for exp_id, title, paper, data in experiments:
    if data:
        keys = [k for k in data.keys() if k != 'math_verification']
        print(f'{exp_id:>4s}  {title:<30s} {paper:<18s} {len(keys)} sections')
    else:
        print(f'{exp_id:>4s}  {title:<30s} {paper:<18s} NOT LOADED')

print(f'\nAll {loaded} experiments have frozen Python baselines.')
print('Each has a corresponding Rust validator in barracuda/src/bin/.')
```

## The Anderson Isomorphism

```
Physics (hotSpring)              Biology (wetSpring)
──────────────────              ──────────────────────
Electron in lattice        ↔    Bacterium in soil pore
Disorder W                 ↔    Pore geometry variation
Critical W_c = 16.5       ↔    Pore size ~50\u00b5m threshold
Extended state (conducts)  ↔    QS active (cooperates)
Localized state (insulates)↔    QS suppressed (isolated)
Anderson transition        ↔    Tillage disruption
Chemotaxis                 ↔    Disorder reduction (\u039412-15%)
```

This is the key scientific insight: the same mathematics that
describes electron transport in disordered solids predicts
microbial cooperation in soil pore networks. The Rust
implementation validates both domains using shared barraCuda
primitives (`erf_f64`, eigenvalue solvers, spectral methods).

hotSpring validates the physics. wetSpring validates the biology.
groundSpring provides the uncertainty budget. The composition
proves they're the same math.

---

**baseCamp Paper 06**: No-till Anderson

**Source**: [syntheticChemistry/wetSpring](https://github.com/syntheticChemistry/wetSpring) |
**Hub**: [primals.eco/lab/springs/wetspring](https://primals.eco/lab/springs/wetspring/)

