+++
title = "Benchmark Comparison — airSpring"
description = "Benchmarks Rust implementations against Python and R reference code for soil water balance, ET₀, and infiltration model numerical parity."
date = 2026-07-18
weight = 50

[extra]
domain = "Lab"
rendered_from = "02-benchmark-comparison.ipynb"
+++

<!-- Auto-generated from 02-benchmark-comparison.ipynb by spore-validate render-notebooks -->

# Benchmark Comparison — airSpring

Python vs Rust vs GPU performance for 24 ecological algorithms.
14.3× geometric mean speedup, 21/21 CPU-GPU parity, 13,000× at atlas scale.

**Data sources**: `benchmark_timing.json`

**Reproduce**: `cargo run --release --bin bench_cpu_vs_python`

**For other springs**: Replace the algorithm list with your domain methods.
The frozen JSON pattern lets you capture timing without re-running benchmarks.

```python
import json
from pathlib import Path

import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
import math

RESULTS = Path('..') / 'experiments' / 'results'

def load(name):
    with open(RESULTS / name) as f:
        return json.load(f)

bench = load('benchmark_timing.json')
rvp = bench['rust_vs_python']

print(f"Geometric mean speedup: {rvp['geometric_mean_speedup']}×")
print(f"Algorithms tested: {rvp['algorithms_tested']}, parity: {rvp['parity_confirmed']}/{rvp['algorithms_tested']}")
print(f"CPU-GPU parity modules: {rvp['cpu_gpu_parity_modules']}")
print(f"Atlas scale: {bench['atlas_scale']['throughput_et0_per_sec']:,} ET₀/s")
```

## Rust vs Python Speedup by Algorithm

```python
algos = bench['algorithms']
names = [a['name'].replace('_', ' ') for a in algos]
speedups = [a['speedup'] for a in algos]

fig, ax = plt.subplots(figsize=(12, 8))
colors = ['#e74c3c' if s > 50 else '#2ecc71' for s in speedups]
bars = ax.barh(names, speedups, color=colors, edgecolor='white')
ax.axvline(x=rvp['geometric_mean_speedup'], color='#3498db', linestyle='--',
           label=f'Geometric mean: {rvp["geometric_mean_speedup"]}×')
ax.set_xlabel('Speedup (×)')
ax.set_title(f'Rust vs Python: {rvp["algorithms_tested"]} algorithms, '
             f'{rvp["geometric_mean_speedup"]}× geometric mean')
ax.legend()
for bar, val in zip(bars, speedups):
    ax.text(bar.get_width() + 0.5, bar.get_y() + bar.get_height()/2,
            f'{val}×', va='center', fontsize=8)
plt.tight_layout()
plt.savefig('/tmp/airspring_02_speedup.png', dpi=150)
plt.show()
```

## Python vs Rust vs GPU Timing

```python
gpu_algos = [a for a in algos if a['gpu_us'] is not None]
gnames = [a['name'].replace('_', ' ') for a in gpu_algos]

fig, ax = plt.subplots(figsize=(12, 8))
x = range(len(gnames))
width = 0.25

py_times = [a['python_us'] for a in gpu_algos]
rs_times = [a['rust_us'] for a in gpu_algos]
gpu_times = [a['gpu_us'] for a in gpu_algos]

ax.barh([i - width for i in x], py_times, width, label='Python', color='#e74c3c', alpha=0.8)
ax.barh(list(x), rs_times, width, label='Rust CPU', color='#3498db', alpha=0.8)
ax.barh([i + width for i in x], gpu_times, width, label='Rust GPU', color='#2ecc71', alpha=0.8)
ax.set_yticks(list(x))
ax.set_yticklabels(gnames, fontsize=8)
ax.set_xlabel('Time (µs)')
ax.set_title('Python vs Rust CPU vs GPU (µs per call)')
ax.set_xscale('log')
ax.legend()
plt.tight_layout()
plt.savefig('/tmp/airspring_02_three_way.png', dpi=150)
plt.show()
```

## GPU Tier Distribution

```python
tiers = bench['gpu_tiers']
labels = ['Tier A upstream (batched)', 'Dedicated GPU', 'CPU-only']
values = [tiers['upstream_batched_ops'], tiers['dedicated_gpu_modules'], tiers['cpu_only_modules']]
colors = ['#2ecc71', '#3498db', '#e74c3c']

fig, ax = plt.subplots(figsize=(6, 6))
wedges, texts, autotexts = ax.pie(values, labels=labels, colors=colors,
                                   autopct='%1.0f%%', startangle=90)
ax.set_title(f'GPU Module Distribution ({tiers["tier_a_upstream"]} Tier A total)')
plt.tight_layout()
plt.savefig('/tmp/airspring_02_gpu_tiers.png', dpi=150)
plt.show()
```

## Summary

| Metric | Value |
|--------|-------|
| Geometric mean speedup | 14.3× (Rust vs Python) |
| Algorithms validated | 24/24 parity confirmed |
| CPU-GPU parity | 21/21 modules |
| Atlas-scale throughput | 10M ET₀/s, 6.8M field-days/s |
| GPU tier distribution | 20 upstream batched + 5 dedicated + 3 CPU-only |
| Seasonal pipeline | 125× speedup (250 µs Python → 2.0 µs Rust) |
| Hardware | i9-12900K, RTX 4070, TITAN V, AKD1000 NPU |

**Provenance**: airSpring v0.10.0 · bench_cpu_vs_python · [primals.eco](https://primals.eco)

