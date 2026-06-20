+++
title = "Python vs Rust vs GPU — Performance Evidence"
description = "Rendered from 02-benchmark-python-vs-rust.ipynb"
date = 2026-06-20
weight = 50

[extra]
domain = "Lab"
rendered_from = "02-benchmark-python-vs-rust.ipynb"
+++

<!-- Auto-generated from 02-benchmark-python-vs-rust.ipynb by spore-validate render-notebooks -->

# Python vs Rust vs GPU — Performance Evidence

Benchmark data from three tiers of the wetSpring pipeline:

1. **Python (numpy/scipy)** — industry-standard baseline
2. **Rust (sovereign CPU)** — wetSpring barracuda crate
3. **GPU (barraCuda WGSL)** — consumer RTX via ToadStool dispatch

All benchmarks run on **ironGate** (i9-14900K, 96 GB DDR5, RTX 4070 / RTX 3090).

**Data sources**: `benchmarks/results/python_baseline_latest.json`,
`experiments/results/015_pipeline_benchmark/`,
`experiments/results/016_gpu_pipeline_parity/`

---

*For other springs: load your own benchmark JSONs. The Python baseline script
(`scripts/python_baseline.py`) generates the same JSON schema for any domain.*

```python
import json
from pathlib import Path

import matplotlib
# matplotlib backend set by environment
import matplotlib.pyplot as plt
import numpy as np

RESULTS = Path('..') / 'experiments' / 'results'
BENCH = Path('..') / 'benchmarks' / 'results'

def load(path):
    with open(path) as f:
        return json.load(f)

baseline = load(BENCH / 'python_baseline_latest.json')
pipeline = load(RESULTS / '015_pipeline_benchmark' / 'benchmark_results.json')
gpu_parity = load(RESULTS / '016_gpu_pipeline_parity' / 'gpu_parity_results.json')

hw = baseline['hardware']
print(f'Hardware: {hw["cpu_model"]}')
print(f'  CPU cores: {hw["cpu_cores"]}, RAM: {hw["ram_total_mb"]:,} MB')
print(f'  GPU: {hw.get("gpu_name", "N/A")}, VRAM: {hw.get("gpu_vram_mb", 0):,} MB')
print(f'  OS kernel: {hw["os_kernel"]}')
print(f'\nBenchmark timestamp: {baseline["timestamp"]}')
print(f'Python phases: {len(baseline["phases"])}')
```

## Python Baseline Timings

Per-operation timings from Python/NumPy/SciPy across diversity metrics,
distance matrices, and ordination at varying input sizes.

```python
phases = baseline['phases']

# Group by operation type
groups = {}
for p in phases:
    op = p['phase'].rsplit(' N=', 1)[0] if ' N=' in p['phase'] else p['phase'].rsplit(' ', 1)[0]
    groups.setdefault(op, []).append(p)

fig, axes = plt.subplots(2, 2, figsize=(14, 10))

# Shannon entropy scaling
shannon = [p for p in phases if p['phase'].startswith('Shannon entropy')]
if shannon:
    ax = axes[0, 0]
    sizes = [int(p['phase'].split('N=')[1]) for p in shannon]
    times = [p['per_eval_us'] for p in shannon]
    ax.loglog(sizes, times, 'o-', color='#2ecc71', linewidth=2, markersize=8)
    ax.set_xlabel('Input size (N)')
    ax.set_ylabel('Time per eval (\u00b5s)')
    ax.set_title('Shannon Entropy — Python Scaling')
    ax.grid(True, alpha=0.3)

# Bray-Curtis scaling
bray = [p for p in phases if p['phase'].startswith('Bray-Curtis')]
if bray:
    ax = axes[0, 1]
    labels = [p['phase'].split(' N=')[0].replace('Bray-Curtis ', '') for p in bray]
    times = [p['per_eval_us'] for p in bray]
    ax.bar(labels, times, color='#e74c3c')
    ax.set_ylabel('Time per eval (\u00b5s)')
    ax.set_title('Bray-Curtis Distance — Python')
    ax.set_yscale('log')

# Cosine similarity scaling
cosine = [p for p in phases if p['phase'].startswith('Cosine')]
if cosine:
    ax = axes[1, 0]
    labels = [p['phase'].split(' N=')[0].replace('Cosine ', '') for p in cosine]
    times = [p['per_eval_us'] for p in cosine]
    ax.bar(labels, times, color='#3498db')
    ax.set_ylabel('Time per eval (\u00b5s)')
    ax.set_title('Cosine Similarity — Python')
    ax.set_yscale('log')

# Memory usage across all phases
ax = axes[1, 1]
mem = [p['peak_rss_mb'] for p in phases]
ax.plot(range(len(mem)), mem, 'o-', color='#9b59b6', markersize=4)
ax.set_xlabel('Phase index')
ax.set_ylabel('Peak RSS (MB)')
ax.set_title('Memory Usage Across Phases')
ax.grid(True, alpha=0.3)

plt.suptitle('Python/NumPy/SciPy Baseline Performance', fontsize=14, fontweight='bold')
plt.tight_layout()
plt.savefig('/tmp/wetspring_02_python_baseline.png', dpi=150, bbox_inches='tight')
plt.show()
```

## Rust vs Galaxy Pipeline

Full 16S pipeline comparison: sovereign Rust vs Galaxy/QIIME2.
22 samples, 3.9M reads through the complete pipeline.

```python
rust = pipeline['rust']
galaxy_data = pipeline['galaxy']

print(f'Pipeline: {pipeline["benchmark"]}')
print(f'Date: {pipeline["date"]}')
print(f'Hardware: {pipeline["hardware"]}')
print()

print('Rust Pipeline:')
print(f'  Samples: {rust["samples"]}')
print(f'  Total reads: {rust["total_reads"]:,}')
print(f'  ASVs: {rust["total_asvs"]:,}')
print(f'  Wall time: {rust["wall_total_ms"]/1000:.1f}s')
print(f'  Energy: {rust["energy_kwh"]:.6f} kWh')
print()

print('Galaxy/QIIME2 Pipeline:')
for exp_key in ['exp001', 'exp002']:
    if exp_key in galaxy_data:
        exp = galaxy_data[exp_key]
        print(f'  {exp_key}: {exp["samples"]} samples, {exp["reads"]:,} reads, {exp["total_s"]}s')
print(f'  Per sample: {galaxy_data["per_sample_s"]}s')
print(f'  Energy: {galaxy_data["energy_kwh"]:.6f} kWh')

fig, axes = plt.subplots(1, 3, figsize=(15, 5))

# Per-stage timing
ax = axes[0]
stages = ['FASTQ Parse', 'QC Filter', 'Dereplic.', 'DADA2', 'Chimera', 'Taxonomy', 'Diversity']
rust_times = [rust['fastq_parse_ms'], rust['quality_filter_ms'], rust['dereplication_ms'],
              rust['dada2_denoise_ms'], rust['chimera_detect_ms'], rust['taxonomy_classify_ms'],
              rust['diversity_calc_ms']]
rust_times_s = [t/1000 for t in rust_times]
bars = ax.barh(stages, rust_times_s, color='#e67e22')
ax.set_xlabel('Time (seconds)')
ax.set_title('Rust Pipeline — Per Stage')
ax.set_xscale('log')

# Energy comparison
ax = axes[1]
ax.bar(['Rust', 'Galaxy'], [rust['energy_kwh'], galaxy_data['energy_kwh']],
       color=['#e67e22', '#3498db'])
ax.set_ylabel('Energy (kWh)')
ax.set_title('Energy Consumption')

# Throughput
ax = axes[2]
ax.bar(['Rust\n(per sample)', 'Galaxy\n(per sample)'],
       [rust['per_sample_s'], galaxy_data['per_sample_s']],
       color=['#e67e22', '#3498db'])
ax.set_ylabel('Seconds per sample')
ax.set_title('Throughput Comparison')

plt.suptitle('Rust vs Galaxy/QIIME2 — Full Pipeline', fontsize=14, fontweight='bold')
plt.tight_layout()
plt.savefig('/tmp/wetspring_02_rust_vs_galaxy.png', dpi=150, bbox_inches='tight')
plt.show()
```

## GPU Acceleration

CPU vs GPU parity on the 16S math pipeline.
The GPU path delegates to barraCuda via ToadStool — zero local WGSL.
**1,077x speedup** for spectral cosine matching at production scale.

```python
print(f'Experiment: {gpu_parity["experiment"]}')
print(f'Tolerance: {gpu_parity["tolerance"]}')
print(f'Samples: {gpu_parity["samples_processed"]}')
print()
print(f'CPU total: {gpu_parity["cpu_total_ms"]:.1f} ms')
print(f'GPU total: {gpu_parity["gpu_total_ms"]:.1f} ms')
print(f'Speedup:   {gpu_parity["speedup"]:.2f}x')
print()
print('Note: This is the pipeline-level speedup. Individual operations')
print('like spectral cosine matching show 1,077x on larger datasets.')

fig, ax = plt.subplots(figsize=(8, 5))
bars = ax.bar(['CPU (Rust)', 'GPU (barraCuda)'],
              [gpu_parity['cpu_total_ms'], gpu_parity['gpu_total_ms']],
              color=['#e67e22', '#2ecc71'])
ax.set_ylabel('Total time (ms)')
ax.set_title(f'CPU vs GPU — {gpu_parity["speedup"]:.1f}x Pipeline Speedup\n'
             f'({gpu_parity["samples_processed"]} samples, tolerance={gpu_parity["tolerance"]})')
for bar, val in zip(bars, [gpu_parity['cpu_total_ms'], gpu_parity['gpu_total_ms']]):
    ax.text(bar.get_x() + bar.get_width()/2, bar.get_height() + 50,
            f'{val:.0f} ms', ha='center', va='bottom', fontsize=12)
plt.tight_layout()
plt.savefig('/tmp/wetspring_02_gpu_speedup.png', dpi=150, bbox_inches='tight')
plt.show()
```

## Summary

| Tier | Substrate | Pipeline Time | Energy | Parity |
|------|-----------|---------------|--------|--------|
| Python | numpy/scipy | baseline | baseline | reference |
| Rust CPU | wetSpring barracuda | varies by stage | measured | machine epsilon |
| GPU | barraCuda WGSL via ToadStool | 2.19x pipeline, 1,077x spectral | lower | tolerance 1e-6 |

The three-tier validation pattern (Python baseline -> Rust parity -> GPU acceleration)
was pioneered in wetSpring and adopted across all 8 springs.

---

**Source**: [syntheticChemistry/wetSpring](https://github.com/syntheticChemistry/wetSpring) |
**Live results**: [primals.eco/lab](https://primals.eco/lab/)

