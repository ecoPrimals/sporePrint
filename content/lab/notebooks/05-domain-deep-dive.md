+++
title = "Domain Deep Dive — airSpring"
description = "Rendered from 05-domain-deep-dive.ipynb"
date = 2026-06-22
weight = 50

[extra]
domain = "Lab"
rendered_from = "05-domain-deep-dive.ipynb"
+++

<!-- Auto-generated from 05-domain-deep-dive.ipynb by spore-validate render-notebooks -->

# Domain Deep Dive — airSpring

Michigan Crop Water Atlas (100 stations × 80 years), seasonal GPU pipeline,
and the path to Penny Irrigation — sovereign compute on consumer hardware.

**Data sources**: `experiment_catalog.json`, `benchmark_timing.json`, `composition_validation.json`

**Reproduce**: `cargo run --release --bin validate_atlas` (1354/1354)

**For other springs**: This notebook covers the domain-specific "crown jewel"
experiment. Replace with your flagship validation story. The frozen data pattern
captures the result without requiring live hardware.

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

catalog = load('experiment_catalog.json')
bench = load('benchmark_timing.json')
comp = load('composition_validation.json')

atlas = next(e for e in catalog['key_experiments'] if e['id'] == 18)
pipeline = next(a for a in bench['algorithms'] if a['name'] == 'seasonal_pipeline')

print(f"Michigan Crop Water Atlas: {atlas['checks']:,} checks ({atlas['status']})")
print(f"Atlas R²: {atlas.get('note', 'N/A')}")
print(f"Seasonal pipeline speedup: {pipeline['speedup']}× (Python {pipeline['python_us']}µs → Rust {pipeline['rust_us']}µs)")
print(f"Atlas-scale throughput: {bench['atlas_scale']['throughput_et0_per_sec']:,} ET₀/s")
```

## Seasonal Pipeline: ET₀ → Kc → Water Balance → Yield

The seasonal pipeline chains four stages: evapotranspiration (FAO-56 PM) →
crop coefficient (dual Kc with cover crops) → water balance (FAO-56 Ch 8) →
yield response (Stewart 1977). Each stage can run on CPU or GPU.

```python
pipeline_methods = [
    ('ET₀ (FAO-56 PM)', 'fao56_pm_et0'),
    ('Dual Kc', 'dual_kc_daily'),
    ('Water Balance', 'water_balance_step'),
    ('Yield Response', 'yield_response_stewart'),
]

algos = {a['name']: a for a in bench['algorithms']}
stages = [algos[m] for _, m in pipeline_methods]
stage_names = [n for n, _ in pipeline_methods]

fig, ax = plt.subplots(figsize=(10, 5))
x = range(len(stage_names))
width = 0.25

py = [s['python_us'] for s in stages]
rs = [s['rust_us'] for s in stages]
gpu = [s['gpu_us'] for s in stages]

ax.bar([i - width for i in x], py, width, label='Python', color='#e74c3c', alpha=0.8)
ax.bar(list(x), rs, width, label='Rust CPU', color='#3498db', alpha=0.8)
ax.bar([i + width for i in x], gpu, width, label='GPU', color='#2ecc71', alpha=0.8)
ax.set_xticks(list(x))
ax.set_xticklabels(stage_names)
ax.set_ylabel('µs per call')
ax.set_title('Seasonal Pipeline: 4-Stage Timing (Python vs Rust vs GPU)')
ax.legend()
plt.tight_layout()
plt.savefig('/tmp/airspring_05_pipeline.png', dpi=150)
plt.show()
```

## ET₀ Method Comparison (8 methods)

airSpring validates 8 evapotranspiration methods against peer-reviewed baselines.

```python
et0_methods = [
    'fao56_pm_et0', 'hargreaves_et0', 'priestley_taylor',
    'thornthwaite', 'makkink_et0', 'turc_et0', 'hamon_et0', 'blaney_criddle_et0'
]
et0_data = [algos[m] for m in et0_methods]
et0_names = [a['name'].replace('_et0', '').replace('_', ' ').title() for a in et0_data]
et0_speedups = [a['speedup'] for a in et0_data]

fig, ax = plt.subplots(figsize=(10, 5))
bars = ax.bar(et0_names, et0_speedups, color='#2ecc71', edgecolor='white')
ax.set_ylabel('Speedup (×)')
ax.set_title('8 ET₀ Methods: Rust vs Python Speedup')
for bar, val in zip(bars, et0_speedups):
    ax.text(bar.get_x() + bar.get_width()/2, bar.get_height() + 0.1,
            f'{val}×', ha='center', fontsize=9)
plt.xticks(rotation=30, ha='right')
plt.tight_layout()
plt.savefig('/tmp/airspring_05_et0_methods.png', dpi=150)
plt.show()
```

## Key Experiments

```python
key_exps = catalog['key_experiments']
print(f"{'ID':>4s}  {'Status':8s}  {'Checks':>8s}  Name")
print('-' * 60)
for e in key_exps:
    note = f" ({e['note']})" if 'note' in e else ''
    print(f"{e['id']:4d}  {e['status']:8s}  {e['checks']:8,d}  {e['name']}{note}")
```

## The Path to Penny Irrigation

Penny Irrigation is the Garden-level product vision: sovereign irrigation
scheduling on consumer hardware ($600 GPU + $99 NPU). The pipeline:

```
Open-Meteo weather → FAO-56 PM ET₀ → Dual Kc (cover crops) →
    Water balance → Yield prediction → Scheduling recommendation
```

All stages validated through 87 experiments. GPU pipeline delivers 6.8M
field-days/s on consumer hardware (RTX 4070 + AKD1000). The full NUCLEUS
composition deploys via biomeOS from pre-built plasmidBin binaries.

**Current state**: Science validated (L2), primal composition readiness (L0→L1).
Next: guideStone scaffold, then Tier 2 IPC validation against live NUCLEUS.

## Summary

| Metric | Value |
|--------|-------|
| Michigan Atlas | 100 stations × 80 years, R²=0.967 |
| Atlas checks | 1,354 (active experiment) |
| Seasonal pipeline | 125× Python→Rust speedup |
| ET₀ methods | 8 validated (FAO-56, HG, PT, TW, MK, TC, HM, BC) |
| Throughput | 10M ET₀/s, 6.8M field-days/s |
| Papers reproduced | 60 (Dong, Allen, FAO-56, van Genuchten, Stewart, ...) |
| Penny hardware | RTX 4070 ($600) + AKD1000 ($99) + i9 |

**Provenance**: airSpring v0.10.0 · MSU BAE (Dong lab) · [primals.eco](https://primals.eco)

