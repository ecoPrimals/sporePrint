+++
title = "Measurement Science Deep Dive — groundSpring"
description = "Rendered from 05-measurement-science-deep-dive.ipynb"
date = 2026-06-23
weight = 50

[extra]
domain = "Lab"
rendered_from = "05-measurement-science-deep-dive.ipynb"
+++

<!-- Auto-generated from 05-measurement-science-deep-dive.ipynb by spore-validate render-notebooks -->

# Measurement Science Deep Dive — groundSpring

The gap between what models predict and what instruments measure.
This notebook explores groundSpring's core domain: noise decomposition
(bias vs variance), Anderson localization (signal propagation through
disordered media), and the tolerance architecture that makes every
comparison traceable to a mathematical bound.

**Data sources**: `experiments/results/experiment_catalog.json`, `benchmark_timing.json`, `security_gaps.json`

---

*For other springs*: This is your domain-specific showcase. Replace the
science narrative with your most compelling discovery story.

```python
import json
import matplotlib
import matplotlib.pyplot as plt
import numpy as np
from pathlib import Path

RESULTS = Path('..') / 'experiments' / 'results'
PASS = '#2ecc71'
FAIL = '#e74c3c'
INFO = '#3498db'
WARN = '#f39c12'

def load(name):
    with open(RESULTS / name) as f:
        return json.load(f)

catalog = load('experiment_catalog.json')
gaps = load('security_gaps.json')

print('groundSpring — The Dirty Differences')
print('"How do things actually look, and why is it different from what we expected?"')
print()
print(f"Domains: {len(catalog['domains'])}")
print(f"Tolerance tiers: {gaps['tolerance_system']['library_tiers']} library + {gaps['tolerance_system']['epsilon_guards']} epsilon + {gaps['tolerance_system']['validation_specific']} validation")
print(f"Upstream contract pins: {gaps['tolerance_system']['upstream_contract_pins']}")
```

## The Five Pillars of Measurement Science

1. **Signal vs Noise** — Sensor drift, calibration error, environmental interference
2. **Inverse Problems** — From observations back to causes
3. **Sensing Systems** — How instruments distort reality
4. **Temporal Dynamics** — How systems drift over time
5. **Spatial Propagation** — How signals travel through media

```python
pillars = {
    'Signal vs Noise': ['001', '002', '003', '004', '015', '024'],
    'Inverse Problems': ['005', '019', '020', '021'],
    'Sensing Systems': ['006', '010', '011', '028'],
    'Temporal Dynamics': ['014', '016', '017', '033'],
    'Spatial Propagation': ['008', '009', '012', '018']
}

fig, ax = plt.subplots(figsize=(10, 5))
pillar_names = list(pillars.keys())
pillar_counts = [len(v) for v in pillars.values()]
colors = [INFO, PASS, WARN, '#9b59b6', '#e67e22']

bars = ax.barh(pillar_names[::-1], pillar_counts[::-1], color=colors[::-1])
ax.set_xlabel('Experiments')
ax.set_title('The Five Pillars of Measurement Science')
for bar, count in zip(bars, pillar_counts[::-1]):
    ax.text(bar.get_width() + 0.1, bar.get_y() + bar.get_height()/2,
            str(count), va='center', fontweight='bold')

plt.tight_layout()
plt.savefig('/tmp/groundspring_05_pillars.png', dpi=150, bbox_inches='tight')
plt.show()
```

## Tolerance Architecture

Every floating-point comparison in groundSpring uses a named constant
with documented provenance. 13 library tiers from `DETERMINISM` (1e-15)
to `EQUILIBRIUM` (0.1), 5 epsilon guards, 6 upstream contract pins
binding to barraCuda v0.3.12 API behavior.

```python
tol_tiers = [
    ('DETERMINISM', 1e-15, 'Bitwise reproducibility'),
    ('STRICT', 1e-14, 'Compensated sum'),
    ('EXACT', 1e-12, 'Summation-only paths'),
    ('ANALYTICAL', 1e-10, 'One transcendental'),
    ('INTEGRATION', 1e-8, 'ODE RK4 accumulation'),
    ('CDF_APPROX', 1e-6, 'CDF/erf approximation'),
    ('ROUNDTRIP', 1e-5, 'CDF-PPF round-trip'),
    ('RECONSTRUCTION', 1e-4, 'Tikhonov RMSE'),
    ('LITERATURE', 1e-3, '3-4 sig figs'),
    ('DECOMPOSITION', 5e-3, 'Bias-variance fractions'),
    ('STOCHASTIC', 1e-2, 'CLT O(1/sqrt(N))'),
    ('NORM_2PCT', 2e-2, 'Integral conservation'),
    ('EQUILIBRIUM', 1e-1, 'Physical precision'),
]

names = [t[0] for t in tol_tiers]
values = [t[1] for t in tol_tiers]
descs = [t[2] for t in tol_tiers]

fig, ax = plt.subplots(figsize=(12, 6))
y_pos = range(len(names))
bars = ax.barh(y_pos, [np.log10(v) for v in values], color=INFO)
ax.set_yticks(y_pos)
ax.set_yticklabels([f'{n} ({d})' for n, d in zip(names, descs)], fontsize=8)
ax.set_xlabel('log10(tolerance)')
ax.set_title('13-Tier Tolerance Architecture')
ax.invert_yaxis()

for i, (bar, v) in enumerate(zip(bars, values)):
    ax.text(bar.get_width() - 0.3, bar.get_y() + bar.get_height()/2,
            f'{v:.0e}', va='center', fontsize=8, color='white', fontweight='bold')

plt.tight_layout()
plt.savefig('/tmp/groundspring_05_tolerances.png', dpi=150, bbox_inches='tight')
plt.show()
```

## Anderson Localization: The Flagship Domain

Anderson localization — how disorder causes wave functions to
exponentially localize — threads through 8 experiments across
4 scientific domains: pure math (Exp 008, 009, 012, 018),
uncertainty bridging (Exp 015, 022), immunology (Exp 033),
and hardware (Exp 028, NPU classification).

```python
anderson_exps = [
    ('008: Anderson 1D', '8/8', '29.9x', 'Lyapunov exponents'),
    ('009: Almost-Mathieu', '8/8', '49.5x', 'Aubry-Andre transition'),
    ('012: Spin Chain', '18/18', '12.3x', 'Wavepacket MSD, transport'),
    ('015: Uncertainty Bridge', '8/8', '14.1x', 'Sensor noise → xi'),
    ('018: Band Edge', '10/10', '22.4x', 'Spectral gap detection'),
    ('022: ET0-Anderson', '7/7', '8.9x', 'FAO-56 → localization'),
    ('028: NPU Anderson', '9/9', 'N/A', 'BrainChip classification'),
    ('033: Tissue Anderson', '29/29', 'analytical', 'Immunological signaling'),
]

print('Anderson Localization Thread: 8 experiments, 4 domains')
print()
for exp, checks, speedup, note in anderson_exps:
    print(f'  {exp:30s}  {checks:8s}  {speedup:12s}  {note}')
```

## Validation Summary

| Metric | Value |
|--------|-------|
| Scientific domains | 10 |
| Five pillars mapped | Signal, Inverse, Sensing, Temporal, Spatial |
| Tolerance tiers | 13 library + 5 epsilon + 6 upstream pins |
| Anderson thread | 8 experiments across 4 domains |
| Faculty connections | 7 researchers (MSU, Carleton) |
| Bare float literals | 0 (all named with provenance) |

**Provenance**: All data from `groundSpring V143 (May 16, 2026)).
See [Spring Catalog](https://primals.eco/architecture/spring-catalog/) on primals.eco.

