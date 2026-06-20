+++
title = "Benchmark Comparison — wetSpring"
description = "Rendered from 02-benchmark-comparison.ipynb"
date = 2026-06-20
weight = 50

[extra]
domain = "Lab"
rendered_from = "02-benchmark-comparison.ipynb"
+++

<!-- Auto-generated from 02-benchmark-comparison.ipynb by spore-validate render-notebooks -->

# Benchmark Comparison — wetSpring

wetSpring validates computational correctness across 23 scientific domains,
each with Python baseline timing and Rust implementation. This notebook
visualizes the 23-domain benchmark, the full 16S pipeline comparison
(Rust vs Galaxy/QIIME2), and energy efficiency estimates.

**Data sources**: `experiments/results/benchmark_timing.json`

**Reproduce**: `wetspring validate --scenario 23_domain_timing` in the wetSpring repository.

---

*For other springs: replace the domain list with your own benchmark data.
Keep the Rust vs Python comparison pattern — it demonstrates the value
of primal composition over script-based pipelines.*

```python
import os, json, struct, socket, time
from pathlib import Path

RESULTS = Path('..') / 'experiments' / 'results'

def load(name):
    with open(RESULTS / name) as f:
        return json.load(f)

TIER = 'frozen'
IPC_SOCKET = os.environ.get('WETSPRING_IPC_SOCKET')

def ipc_call(method, params=None):
    """JSON-RPC call to barracuda IPC — active in Tier 2."""
    sock = socket.socket(socket.AF_UNIX, socket.SOCK_STREAM)
    sock.connect(IPC_SOCKET)
    req = json.dumps({'jsonrpc': '2.0', 'method': method, 'params': params or {}, 'id': 1})
    payload = req.encode()
    sock.sendall(struct.pack('<I', len(payload)) + payload)
    length = struct.unpack('<I', sock.recv(4))[0]
    data = sock.recv(length)
    sock.close()
    return json.loads(data)['result']

if IPC_SOCKET and os.path.exists(IPC_SOCKET):
    try:
        ipc_call('health.check')
        TIER = 'live_ipc'
        print(f'Tier 2 ACTIVE — live IPC via {IPC_SOCKET}')
    except Exception:
        print('Tier 2 socket found but not responding — using frozen data')
else:
    print(f'Tier 1 — frozen data (no IPC socket)')

bench = load('benchmark_timing.json')
print(f'Hardware: {bench["hardware"]["cpu"]}, {bench["hardware"]["gpu"]}')
print(f'Domains benchmarked: {len(bench["domain_benchmark_23"]["domains"])}')
print(f'Total Python time: {bench["domain_benchmark_23"]["total_us"]:.0f} µs')
```

## 23-Domain Python Baseline Timing

Each domain represents a distinct scientific algorithm implemented in both
Python (baseline) and Rust (barracuda). The Python times establish the
reference point; Rust times are measured via validation binaries.

```python
import matplotlib
import matplotlib.pyplot as plt
import numpy as np

domains = bench['domain_benchmark_23']['domains']
d_keys = sorted(domains.keys())
d_labels = [domains[k]['label'] for k in d_keys]
d_times = [domains[k]['python_us'] for k in d_keys]
d_cats = [domains[k]['category'] for k in d_keys]

cat_colors = {
    'ecology': '#2ecc71', 'bioinformatics': '#3498db',
    'chemistry': '#e74c3c', 'integrated': '#f39c12'
}
colors = [cat_colors.get(c, '#95a5a6') for c in d_cats]

fig, ax = plt.subplots(figsize=(14, 8))
bars = ax.barh(range(len(d_labels)), [np.log10(max(t, 1)) for t in d_times], color=colors)
ax.set_yticks(range(len(d_labels)))
ax.set_yticklabels(d_labels, fontsize=8)
ax.set_xlabel('log10(Python µs)')
ax.set_title(f'23-Domain Python Baseline — {bench["domain_benchmark_23"]["total_us"]:.0f} µs total')

for bar, val in zip(bars, d_times):
    if val > 1000:
        label = f'{val/1000:.0f} ms'
    else:
        label = f'{val:.1f} µs'
    ax.text(bar.get_width() + 0.05, bar.get_y() + bar.get_height()/2,
            label, va='center', fontsize=7)

from matplotlib.patches import Patch
legend_elements = [Patch(facecolor=c, label=n.title()) for n, c in cat_colors.items()]
ax.legend(handles=legend_elements, loc='lower right', fontsize=9)

plt.tight_layout()
plt.savefig('/tmp/wetspring_02_domains.png', dpi=150, bbox_inches='tight')
plt.show()
```

## Rust vs Python Speedup

Key domains where Rust measurements are available show consistent
speedups from 2.6x (BLAKE3 hash) to 41x (Smith-Waterman alignment).

```python
rvp = bench['rust_vs_python_estimates']
compare_keys = [k for k in rvp if k != 'note']
labels = [k.replace('_', ' ').title() for k in compare_keys]
speedups = [float(rvp[k]['speedup'].replace('x', '')) for k in compare_keys]

fig, axes = plt.subplots(1, 2, figsize=(14, 5))

# Speedup bars
ax = axes[0]
bars = ax.barh(labels, speedups, color='#3498db')
ax.set_xlabel('Speedup (x)')
ax.set_title('Rust vs Python — Key Domains')
for bar, val in zip(bars, speedups):
    ax.text(bar.get_width() + 0.3, bar.get_y() + bar.get_height()/2,
            f'{val:.0f}x', va='center', fontsize=10, fontweight='bold')

# Energy comparison
energy = bench['energy_estimate']
ax = axes[1]
e_labels = ['Rust', 'Python (est.)']
e_vals = [energy['rust_full_suite_kwh'], energy['python_equivalent_estimate_kwh']]
e_colors = ['#2ecc71', '#e74c3c']
bars = ax.bar(e_labels, e_vals, color=e_colors)
ax.set_ylabel('Energy (kWh)')
ax.set_title(f'Energy Efficiency — {energy["ratio"]}')
for bar, val in zip(bars, e_vals):
    ax.text(bar.get_x() + bar.get_width()/2, bar.get_height() + 0.1,
            f'{val:.2f} kWh', ha='center', va='bottom', fontsize=11)

plt.suptitle('wetSpring: Rust Primal Composition vs Python Baselines',
             fontsize=13, fontweight='bold')
plt.tight_layout()
plt.savefig('/tmp/wetspring_02_speedup.png', dpi=150, bbox_inches='tight')
plt.show()

# Tier 2: live benchmark comparison
if TIER == 'live_ipc':
    counts = [1, 10, 100, 1000, 10000]
    t0 = time.perf_counter()
    result = ipc_call('science.diversity', {'counts': counts})
    elapsed_us = (time.perf_counter() - t0) * 1e6
    python_us = bench['rust_vs_python_estimates']['diversity_calc']['python_us']
    print(f'Tier 2 live: science.diversity took {elapsed_us:.1f} µs '
          f'(Python baseline: {python_us} µs, speedup: {python_us/elapsed_us:.1f}x)')
```

## Pipeline Benchmark — Rust vs Galaxy/QIIME2

Full 16S metagenomics pipeline: FASTQ parse → quality filter → DADA2 denoise
→ chimera detection → taxonomy → diversity. Rust runs the complete pipeline
locally on CPU; Galaxy uses cloud-optimized DADA2.

```python
pipe = bench['pipeline_benchmark']

fig, ax = plt.subplots(figsize=(10, 5))
p_labels = ['Rust CPU\n(local)', 'Galaxy/QIIME2\n(cloud)']
p_per_sample = [pipe['rust']['per_sample_s'], pipe['galaxy']['per_sample_s']]
p_energy = [pipe['rust']['energy_kwh'], pipe['galaxy']['energy_kwh']]

x = [0, 1]
bar1 = ax.bar([i - 0.2 for i in x], p_per_sample, 0.35,
              label='Per-sample (s)', color='#3498db')
ax2 = ax.twinx()
bar2 = ax2.bar([i + 0.2 for i in x], p_energy, 0.35,
               label='Energy (kWh)', color='#f39c12', alpha=0.7)

ax.set_xticks(x)
ax.set_xticklabels(p_labels)
ax.set_ylabel('Per-sample time (s)', color='#3498db')
ax2.set_ylabel('Energy (kWh)', color='#f39c12')
ax.set_title(f'16S Pipeline — {pipe["rust"]["samples"]} samples, '
             f'{pipe["rust"]["total_reads"]:,} reads')

for bar, val in zip(bar1, p_per_sample):
    ax.text(bar.get_x() + bar.get_width()/2, bar.get_height() + 10,
            f'{val:.1f}s', ha='center', fontsize=10)
for bar, val in zip(bar2, p_energy):
    ax2.text(bar.get_x() + bar.get_width()/2, bar.get_height() + 0.02,
             f'{val:.3f}', ha='center', fontsize=10)

fig.legend(loc='upper right', bbox_to_anchor=(0.95, 0.88))
plt.tight_layout()
plt.savefig('/tmp/wetspring_02_pipeline.png', dpi=150, bbox_inches='tight')
plt.show()

print(f'Note: {pipe["speedup_notes"]}')
```

## Validation Summary

| Benchmark | Result |
|-----------|--------|
| 23-domain Python baselines | All timed, Rust parity confirmed |
| Rust vs Python speedup | 2.6x – 41x across key domains |
| Energy efficiency | 7x more efficient (Rust vs Python equivalent) |
| 16S pipeline (22 samples) | Rust: 1,565s/sample, Galaxy: 9.6s/sample |

Galaxy's cloud-optimized pipeline wins on small batches; Rust wins on
large-scale local processing with full chimera detection and provenance.

---

**Provenance**: Benchmark data frozen from `experiments/results/059_23_domain_benchmark/`
and `experiments/results/015_pipeline_benchmark/`. BLAKE3 hashes track drift.

**Evolution**: Tier 2 adds live IPC timing to compare against frozen baselines.
Tier 3 adds provenance-wrapped benchmark sessions.

**Source**: [ecoPrimals/wetSpring](https://github.com/ecoPrimals/wetSpring)

