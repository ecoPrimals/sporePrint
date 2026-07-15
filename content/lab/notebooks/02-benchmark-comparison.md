+++
title = "Benchmark Comparison — groundSpring"
description = "Rendered from 02-benchmark-comparison.ipynb"
date = 2026-07-10
weight = 50

[extra]
domain = "computation"
rendered_from = "02-benchmark-comparison.ipynb"
+++

<!-- Auto-generated from 02-benchmark-comparison.ipynb by spore-validate render-notebooks -->

# Benchmark Comparison — groundSpring

Rust vs Python performance across 29 benchmarked experiments (001–029 + 035), plus
three-mode benchmark data (default → barraCuda CPU → barraCuda GPU)
and the 110-delegation inventory breakdown by barraCuda module.

**Data sources**: `experiments/results/benchmark_timing.json`, `experiment_catalog.json`

---

*For other springs*: Replace benchmark data with your own timing JSONs.
Keep the Rust-vs-Python comparison pattern and delegation breakdown.

```python
import json
import matplotlib
import matplotlib.pyplot as plt
from pathlib import Path

RESULTS = Path('..') / 'experiments' / 'results'
PASS = '#2ecc71'
FAIL = '#e74c3c'
INFO = '#3498db'

def load(name):
    with open(RESULTS / name) as f:
        return json.load(f)

bench = load('benchmark_timing.json')
catalog = load('experiment_catalog.json')

print(f"Overall Rust vs Python speedup: {bench['rust_vs_python']['overall_speedup']}")
print(f"Excluding LAPACK-bound: {bench['rust_vs_python']['excl_lapack_speedup']}")
print(f"barraCuda delegations: {bench['barracuda_delegations']['total']} ({bench['barracuda_delegations']['cpu']} CPU + {bench['barracuda_delegations']['gpu']} GPU)")
```

## Top Speedups: Rust vs Python

```python
highlights = bench['rust_vs_python']['highlights']
names = [h['experiment'] for h in highlights]
speedups = [float(h['speedup'].replace('x', '')) for h in highlights]

fig, ax = plt.subplots(figsize=(10, 5))
bars = ax.barh(names[::-1], speedups[::-1], color=PASS)
ax.set_xlabel('Speedup (×)')
ax.set_title('Top 6 Rust vs Python Speedups')
for bar, s in zip(bars, speedups[::-1]):
    ax.text(bar.get_width() + 0.5, bar.get_y() + bar.get_height()/2,
            f'{s:.1f}×', va='center', fontsize=10, fontweight='bold')
ax.axvline(x=1.0, color='gray', linestyle='--', alpha=0.5)

plt.tight_layout()
plt.savefig('/tmp/groundspring_02_speedups.png', dpi=150, bbox_inches='tight')
plt.show()
```

## Three-Mode Benchmark (Default → CPU → GPU)

```python
modes = bench['three_mode_benchmark']
labels = ['Default Features', 'barraCuda CPU', 'barraCuda GPU']
times = [modes['default_features']['time_s'], modes['barracuda_cpu']['time_s'], modes['barracuda_gpu']['time_s']]
test_counts = [modes['default_features']['tests'], modes['barracuda_cpu']['tests'], modes['barracuda_gpu']['tests']]
colors = ['#95a5a6', INFO, PASS]

fig, (ax1, ax2) = plt.subplots(1, 2, figsize=(12, 4))

ax1.bar(labels, times, color=colors)
ax1.set_ylabel('Time (seconds)')
ax1.set_title('Suite Runtime by Mode')
for i, (t, tc) in enumerate(zip(times, test_counts)):
    ax1.text(i, t + 0.3, f'{t:.1f}s\n({tc} tests)', ha='center', fontsize=9)

ax2.bar(labels, test_counts, color=colors)
ax2.set_ylabel('Test Count')
ax2.set_title('Tests Available per Mode')

plt.tight_layout()
plt.savefig('/tmp/groundspring_02_three_mode.png', dpi=150, bbox_inches='tight')
plt.show()
```

## barraCuda Delegation Breakdown

```python
mods = bench['barracuda_delegations']['modules']
mod_names = list(mods.keys())
cpu_counts = [mods[m]['cpu'] for m in mod_names]
gpu_counts = [mods[m]['gpu'] for m in mod_names]

fig, ax = plt.subplots(figsize=(10, 5))
y = range(len(mod_names))
ax.barh(y, cpu_counts, color=INFO, label='CPU')
ax.barh(y, gpu_counts, left=cpu_counts, color=PASS, label='GPU')
ax.set_yticks(y)
ax.set_yticklabels(mod_names)
ax.set_xlabel('Delegations')
ax.set_title(f'110 barraCuda Delegations (67 CPU + 43 GPU)')
ax.legend()

plt.tight_layout()
plt.savefig('/tmp/groundspring_02_delegations.png', dpi=150, bbox_inches='tight')
plt.show()
```

## Validation Summary

| Metric | Value |
|--------|-------|
| Rust vs Python (overall) | 5.1× faster |
| Rust vs Python (excl LAPACK) | 11.6× faster |
| Peak speedup | 49.5× (Almost-Mathieu Sturm) |
| Three-mode GPU speedup | 2.2× vs default |
| barraCuda delegations | 110 (67 CPU + 43 GPU) |
| metalForge checks | 140 across 5 substrates |

**Provenance**: All benchmarks from `groundSpring V143 (May 16, 2026)).
See [Spring Catalog](https://primals.eco/architecture/spring-catalog/) on primals.eco.

