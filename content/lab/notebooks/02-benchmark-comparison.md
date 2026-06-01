+++
title = "02 — Benchmark Comparison"
description = "Rendered from 02-benchmark-comparison.ipynb"
date = 2026-06-01
weight = 50

[extra]
domain = "Lab"
rendered_from = "02-benchmark-comparison.ipynb"
+++

<!-- Auto-generated from 02-benchmark-comparison.ipynb by spore-validate render-notebooks -->

# 02 — Benchmark Comparison

**neuralSpring sporePrint** | Session S188 | May 2026

Rust vs Python timing across 11 domains, GPU acceleration,
multi-GPU parity, and guideStone validation phases.

**Data sources:** `benchmark-data.json`, `validation-state.json`

**For other springs:** Replace domain speedup data with your own
benchmark results. Adjust GPU coverage to match your compute profile.

```python
import json
from pathlib import Path
import matplotlib.pyplot as plt
import matplotlib.patches as mpatches
import numpy as np

RESULTS = Path('..') / 'experiments' / 'results'

with open(RESULTS / 'benchmark-data.json') as f:
    bm = json.load(f)

with open(RESULTS / 'validation-state.json') as f:
    vs = json.load(f)

PASS = '#2ecc71'
FAIL = '#e74c3c'
INFO = '#3498db'

print(f"neuralSpring v{vs['version']} — Session {vs['session']}")
```

## Rust vs Python — Per-Domain Speedups

Pure Rust achieves an **83.6x geometric mean** speedup over Python
across 11 scientific domains, with the fastest (multi-objective
optimization) reaching **1104x**.

```python
rvp = bm['rust_vs_python']
speedups = rvp['domain_speedups']

domains = [s['domain'] for s in speedups]
values = [s['speedup'] for s in speedups]

fig, ax = plt.subplots(figsize=(12, 6))
bars = ax.barh(domains[::-1], values[::-1], color=INFO)
ax.set_xlabel('Speedup (x)')
ax.set_title(f'Rust vs Python Speedups — {rvp["geomean_speedup"]}x geomean')
ax.set_xscale('log')
ax.axvline(x=rvp['geomean_speedup'], color=PASS, linestyle='--',
           linewidth=2, label=f'Geomean: {rvp["geomean_speedup"]}x')

for bar, val in zip(bars, values[::-1]):
    ax.text(bar.get_width() * 1.1, bar.get_y() + bar.get_height()/2,
            f'{val:.0f}x', va='center', fontsize=9)

ax.legend(loc='lower right', fontsize=11)
plt.tight_layout()
plt.show()
```

## CPU-Python Cross-Language Parity

All 39 parity checks pass at 1e-10 tolerance — Rust CPU produces
numerically identical results to the Python baselines.

```python
parity = rvp['cpu_python_parity']

fig, ax = plt.subplots(figsize=(5, 3))
ax.bar(['PASS', 'FAIL'],
       [parity['pass'], parity['total'] - parity['pass']],
       color=[PASS, FAIL])
ax.set_title(f'CPU↔Python Parity ({parity["pass"]}/{parity["total"]})')
ax.set_ylabel('Checks')
ax.text(0, parity['pass'] + 0.5, str(parity['pass']),
        ha='center', fontweight='bold', fontsize=14)
plt.tight_layout()
plt.show()

print(f"Tolerance: {parity['tolerance']}")
```

## GPU Performance

GPU acceleration via barraCuda + WGSL achieves up to **104x** vs Python,
with ~97% of production operations promoted to GPU.

```python
gpu = bm['gpu_performance']
mgpu = bm['multi_gpu']

fig, axes = plt.subplots(1, 3, figsize=(14, 4))

# GPU coverage pie
cov = gpu['gpu_coverage_percent']
axes[0].pie([cov, 100-cov], labels=[f'GPU ({cov}%)', f'CPU ({100-cov}%)'],
            colors=[PASS, '#95a5a6'], autopct='%1.0f%%', startangle=90)
axes[0].set_title('GPU Coverage')

# GPU metrics
metrics = ['Max speedup\nvs Python', 'Crossover\nlatency (ms)', 'Dispatch\noverhead (x)']
vals = [gpu['max_speedup_vs_python'], gpu['gpu_crossover_latency_ms'], 1.04]
axes[1].bar(metrics, vals, color=[INFO, '#f39c12', PASS])
axes[1].set_title('GPU Performance Metrics')
for i, v in enumerate(vals):
    axes[1].text(i, v + max(vals)*0.02, f'{v}', ha='center', fontweight='bold')

# Multi-GPU parity
titan = mgpu['titan_v_checks']
axes[2].bar(['PASS', 'FAIL'],
            [titan['pass'], titan['total'] - titan['pass']],
            color=[PASS, FAIL])
axes[2].set_title(f'Multi-GPU Parity ({titan["pass"]}/{titan["total"]})')
axes[2].text(0, titan['pass'] + 5, f'{titan["pass"]}',
             ha='center', fontweight='bold', fontsize=14)

plt.tight_layout()
plt.show()

print(f"Devices: {', '.join(mgpu['devices_tested'])}")
print(f"Parity: {mgpu['parity']}")
```

## guideStone Validation Phases

The guideStone binary validates in 4 phases: bare properties,
discovery + liveness, domain science parity, and additive NUCLEUS.

```python
phases = bm['guidestone_phases']

fig, ax = plt.subplots(figsize=(10, 3))
phase_names = list(phases.keys())
phase_labels = [
    'Phase 1: Bare Properties',
    'Phase 2: Discovery + Liveness',
    'Phase 3: Domain Parity',
    'Phase 4: Additive NUCLEUS'
]
phase_status = [True, True, True, False]
colors = [PASS if s else '#f39c12' for s in phase_status]

ax.barh(phase_labels[::-1], [1]*4, color=colors[::-1])
ax.set_xlim(0, 1.3)
ax.set_title('guideStone Validation Phases')

for i, (label, desc) in enumerate(zip(phase_labels[::-1],
                                       list(phases.values())[::-1])):
    ax.text(1.05, i, desc[:60] + '...' if len(desc) > 60 else desc,
            va='center', fontsize=7)

plt.tight_layout()
plt.show()
```

## Isomorphic Primitives

Six primitives compose all domain architectures — from transformers
and protein folding to evolutionary computation and spectral analysis.

```python
prims = bm['isomorphic_primitives']

fig, ax = plt.subplots(figsize=(10, 3))
names = [p['primitive'] for p in prims]
ax.barh(names[::-1], [1]*len(prims), color=INFO)
for i, p in enumerate(prims[::-1]):
    ax.text(1.05, i, p['role'], va='center', fontsize=9)
ax.set_xlim(0, 2.5)
ax.set_title('Isomorphic Computational Primitives')
plt.tight_layout()
plt.show()
```

## Summary

| Metric | Value |
|--------|-------|
| Rust vs Python geomean | 83.6x (11 domains) |
| Fastest speedup | 1104x (multi-objective) |
| CPU↔Python parity | 39/39 PASS (1e-10) |
| GPU max speedup | 104x (transformer medium) |
| GPU coverage | ~97% |
| Dispatch overhead | ≤1.04x (9/10 ops) |
| Fused pipeline | 46-78x over per-op |
| Multi-GPU parity | 384/384 bit-identical |
| guideStone | Level 3 (29/29 bare) |

**Provenance:** [primals.eco](https://primals.eco) |
neuralSpring Session S188 | May 2026

