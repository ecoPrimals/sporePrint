+++
title = "Benchmark Comparison — hotSpring"
description = "Rendered from 02-benchmark-comparison.ipynb"
date = 2026-06-06
weight = 50

[extra]
domain = "Lab"
rendered_from = "02-benchmark-comparison.ipynb"
+++

<!-- Auto-generated from 02-benchmark-comparison.ipynb by spore-validate render-notebooks -->

# Benchmark Comparison — hotSpring

hotSpring's three-tier validation architecture (Python → Rust → NUCLEUS) produces
direct performance comparisons at every tier. This notebook visualizes Rust vs Python
speedups, GPU vs CPU acceleration, and the DF64 emulated double-precision breakthrough.

**Data sources:** `benchmark_timing.json`

**Reproduce:** Individual benchmarks via `cargo bench` or `cargo run --release --bin <benchmark>`

---

*For other springs:* Replace physics benchmarks with your domain. The Rust vs Python
comparison pattern applies to any spring that migrated from scripting to compiled Rust.

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

bench = load('benchmark_timing.json')

print(f"Hardware: {bench['hardware']['cpu']}, {bench['hardware']['gpu_primary']}")
print(f"Clean build: {bench['compilation']['clean_build_release_s']}s")
print(f"Test suite: {bench['test_suite_timing']['total_s']}s")
print(f"Total science cost: {bench['cost_estimate']['total_science_cost']}")
```

## Rust vs Python — Direct Paper Reproduction Comparisons

Every published paper reproduction in hotSpring was first implemented in Python
(Phase A baselines), then in Rust (Phase B-E), producing direct timing comparisons
on identical algorithms and datasets.

```python
C_RUST = '#1abc9c'
C_PYTHON = '#f39c12'
C_GPU = '#9b59b6'
C_PASS = '#2ecc71'
C_INFO = '#3498db'

rvp = bench['rust_vs_python']
comparisons = {k: v for k, v in rvp.items() if k != 'note'}

names = [k.replace('_', ' ').title() for k in comparisons]
rust_ms = [comparisons[k]['rust_ms'] for k in comparisons]
python_ms = [comparisons[k]['python_ms'] for k in comparisons]
speedups = [comparisons[k]['speedup'] for k in comparisons]

fig, axes = plt.subplots(1, 2, figsize=(14, 5))

x = np.arange(len(names))
w = 0.35
axes[0].bar(x - w/2, python_ms, w, label='Python', color=C_PYTHON)
axes[0].bar(x + w/2, rust_ms, w, label='Rust', color=C_RUST)
axes[0].set_yscale('log')
axes[0].set_ylabel('Time (ms, log scale)')
axes[0].set_xticks(x)
axes[0].set_xticklabels(names, rotation=25, ha='right', fontsize=8)
axes[0].legend()
axes[0].set_title('Python vs Rust — Absolute Timing')

speedup_vals = [float(s.replace('x', '')) for s in speedups]
bars = axes[1].bar(names, speedup_vals, color=C_PASS)
for bar, s in zip(bars, speedups):
    axes[1].text(bar.get_x() + bar.get_width()/2, bar.get_height() + 20,
                 s, ha='center', fontsize=9, fontweight='bold')
axes[1].set_ylabel('Speedup (x)')
axes[1].set_xticklabels(names, rotation=25, ha='right', fontsize=8)
axes[1].set_title('Rust Speedup over Python')

fig.suptitle('Three-Tier Validation: Python Baselines → Rust Parity', fontsize=13, fontweight='bold')
plt.tight_layout()
plt.savefig('/tmp/hotspring_02_rust_vs_python.png', dpi=150, bbox_inches='tight')
plt.show()
```

## GPU vs CPU — Physics Domain Benchmarks

Consumer GPU hardware (RTX 4070) delivers 40-70x acceleration over CPU for
physics-heavy workloads. The key enabler is DF64 (emulated double precision
on FP32 cores), which delivers 3.24 TFLOPS — 5.6x over native FP64.

```python
domain = bench['domain_benchmarks']
gpu_benchmarks = {}
for k, v in domain.items():
    if 'gpu_ms' in v and 'cpu_ms' in v:
        gpu_benchmarks[k] = v

fig, axes = plt.subplots(1, 2, figsize=(14, 5))

gb_names = [k.replace('_', ' ').title() for k in gpu_benchmarks]
gb_gpu = [gpu_benchmarks[k]['gpu_ms'] for k in gpu_benchmarks]
gb_cpu = [gpu_benchmarks[k]['cpu_ms'] for k in gpu_benchmarks]
gb_speedup = [gpu_benchmarks[k]['speedup'] for k in gpu_benchmarks]

x = np.arange(len(gb_names))
w = 0.35
axes[0].bar(x - w/2, gb_cpu, w, label='CPU', color='#95a5a6')
axes[0].bar(x + w/2, gb_gpu, w, label='GPU (RTX 4070)', color=C_GPU)
axes[0].set_yscale('log')
axes[0].set_ylabel('Time (ms, log scale)')
axes[0].set_xticks(x)
axes[0].set_xticklabels(gb_names, rotation=25, ha='right', fontsize=8)
axes[0].legend()
axes[0].set_title('CPU vs GPU — Physics Workloads')

speedup_vals = [float(s.replace('x', '')) for s in gb_speedup]
bars = axes[1].bar(gb_names, speedup_vals, color=C_GPU)
for bar, s in zip(bars, gb_speedup):
    axes[1].text(bar.get_x() + bar.get_width()/2, bar.get_height() + 1,
                 s, ha='center', fontsize=9, fontweight='bold')
axes[1].set_ylabel('GPU Speedup (x)')
axes[1].set_xticklabels(gb_names, rotation=25, ha='right', fontsize=8)
axes[1].set_title('GPU Acceleration Factor')

fig.suptitle(f'GPU Compute: DF64 delivers {domain["df64_throughput"]["tflops"]} TFLOPS on FP32 cores', fontsize=13, fontweight='bold')
plt.tight_layout()
plt.savefig('/tmp/hotspring_02_gpu_vs_cpu.png', dpi=150, bbox_inches='tight')
plt.show()
```

## Energy and Cost Efficiency

All 181 experiments ran on consumer hardware for a total science cost of $0.30.
Rust is 8.8x more energy-efficient than equivalent Python implementations.

```python
fig, axes = plt.subplots(1, 2, figsize=(12, 4))

energy = bench['energy_estimate']
axes[0].bar(['Rust', 'Python\n(equivalent)'],
            [energy['rust_full_suite_wh'], energy['python_equivalent_estimate_wh']],
            color=[C_RUST, C_PYTHON])
axes[0].set_ylabel('Energy (Wh)')
axes[0].set_title(f'Energy: {energy["ratio"]}')

cost = bench['cost_estimate']
axes[1].bar(['Total Science\nCost'], [0.30], color=C_PASS, width=0.4)
axes[1].set_ylabel('USD')
axes[1].set_title(f'181 experiments, {cost["total_science_cost"]} total')
axes[1].set_ylim(0, 0.5)

fig.suptitle('Consumer Hardware Science: $0.30 for 181 Experiments', fontsize=13, fontweight='bold')
plt.tight_layout()
plt.savefig('/tmp/hotspring_02_energy.png', dpi=150, bbox_inches='tight')
plt.show()
```

## Validation Summary

| Benchmark | Result |
|-----------|--------|
| Rust vs Python | **44.8x** (SEMF), **2274x** (screening), **283x** (eigenvalue) |
| GPU vs CPU | **71.8x** (HFB), **54.4x** (HMC), **44.3x** (gradient flow) |
| DF64 throughput | **3.24 TFLOPS** (5.6x over native FP64) |
| Total science cost | **$0.30** for 181 experiments |
| Energy efficiency | **8.8x** more efficient than Python |

---

**Provenance:** All benchmarks from `experiments/results/benchmark_timing.json`.  
**Hardware:** AMD Ryzen 9 7950X, NVIDIA RTX 4070, Pop!_OS 22.04.  
**Source:** [hotSpring on GitHub](https://github.com/syntheticChemistry/hotSpring) · [primals.eco](https://primals.eco/lab/springs/hotspring/)

