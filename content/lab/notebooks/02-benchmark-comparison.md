+++
title = "Benchmark Comparison — primalSpring"
description = "Rendered from 02-benchmark-comparison.ipynb"
date = 2026-06-01
weight = 50

[extra]
domain = "Lab"
rendered_from = "02-benchmark-comparison.ipynb"
+++

<!-- Auto-generated from 02-benchmark-comparison.ipynb by spore-validate render-notebooks -->

# Benchmark Comparison — primalSpring

Performance benchmarks for primalSpring's Rust validation infrastructure
versus hypothetical Python equivalents. Covers compilation, test suite
execution, validation binary phases, IPC handshake timing, and energy
efficiency estimates.

primalSpring validates compositions — its performance characteristics
are dominated by graph parsing, BTSP cryptographic handshakes, checksum
verification (BLAKE3), and IPC method dispatch.

**Data sources**: `experiments/results/benchmark_timing.json`

**Reproduce**: `cargo test --workspace` and `cargo run --release --bin primalspring_unibin -- validate`

---

*For other springs: replace benchmark data with your domain-specific
timing results. The Rust vs Python comparison pattern is universal.*

```python
import json
from pathlib import Path

import matplotlib
import matplotlib.pyplot as plt
import numpy as np

RESULTS = Path('..') / 'experiments' / 'results'

def load(path):
    with open(RESULTS / path) as f:
        return json.load(f)

bench = load('benchmark_timing.json')

hw = bench['hardware']
print(f'Hardware: {hw["cpu"]}, {hw["ram_gb"]}GB RAM, {hw["gpu"]}')
print(f'OS: {hw["os"]}')
print(f'Clean build: {bench["compilation"]["clean_build_release_s"]}s')
print(f'Test suite: {bench["test_suite_timing"]["total_s"]}s')
```

## Rust vs Python — Operation Timing

Key operations compared: graph parsing, BTSP handshake, BLAKE3 checksum
verification, and IPC method calls. Python estimates use equivalent
JSON-RPC client logic with `hashlib` for checksums.

```python
rvp = bench['rust_vs_hypothetical_python']
ops = ['Graph\nParsing', 'BTSP\nHandshake', 'Checksum\nVerification', 'IPC\nMethod Call']
rust_times = [rvp['graph_parsing']['rust_ms'],
              rvp['btsp_handshake']['rust_ms'],
              rvp['checksum_verification']['rust_blake3_ms'],
              rvp['ipc_method_call']['rust_ms']]
python_times = [rvp['graph_parsing']['python_estimate_ms'],
                rvp['btsp_handshake']['python_estimate_ms'],
                rvp['checksum_verification']['python_hashlib_sha256_ms'],
                rvp['ipc_method_call']['python_estimate_ms']]
speedups = [rvp['graph_parsing']['speedup'],
            rvp['btsp_handshake']['speedup'],
            rvp['checksum_verification']['speedup'],
            rvp['ipc_method_call']['speedup']]

fig, axes = plt.subplots(1, 2, figsize=(14, 5))

x = np.arange(len(ops))
width = 0.35
ax = axes[0]
bars_r = ax.bar(x - width/2, rust_times, width, label='Rust', color='#e67e22')
bars_p = ax.bar(x + width/2, python_times, width, label='Python (est.)', color='#3498db')
ax.set_xticks(x)
ax.set_xticklabels(ops, fontsize=9)
ax.set_ylabel('Time (ms)')
ax.set_title('Operation Timing — Rust vs Python')
ax.legend()
ax.set_yscale('log')

ax = axes[1]
speedup_vals = [float(s.replace('x', '')) for s in speedups]
colors = ['#2ecc71' if s >= 10 else '#f39c12' if s >= 5 else '#3498db' for s in speedup_vals]
bars = ax.barh(ops, speedup_vals, color=colors)
ax.set_xlabel('Speedup (x)')
ax.set_title('Rust Speedup over Python')
for bar, val, label in zip(bars, speedup_vals, speedups):
    ax.text(bar.get_width() + 0.3, bar.get_y() + bar.get_height()/2,
            label, va='center', fontsize=11, fontweight='bold')

plt.suptitle('primalSpring: Rust vs Python Performance',
             fontsize=13, fontweight='bold')
plt.tight_layout()
plt.savefig('/tmp/primalspring_02_rust_vs_python.png', dpi=150, bbox_inches='tight')
plt.show()
```

## Validation Binary Phases

`primalspring certify` (UniBin organelle, formerly guidestone) runs 4 validation phases:
- **P1 Compile**: Workspace compiles with zero warnings
- **P2 Structural**: Module presence, public API checks
- **P3 Checksum**: BLAKE3 manifest verification of tracked files
- **P4 Semantic**: Cross-module consistency, deploy graph coherence

```python
gs = bench['validation_benchmarks']['primalspring_certify']
phases = list(gs['phases'].keys())
times = [gs['phases'][p]['time_ms'] for p in phases]
checks = [gs['phases'][p]['checks'] for p in phases]
labels = [p.replace('_', ' ') for p in phases]

fig, axes = plt.subplots(1, 2, figsize=(12, 5))

colors = ['#3498db', '#2ecc71', '#e67e22', '#9b59b6']
ax = axes[0]
bars = ax.bar(labels, times, color=colors)
ax.set_ylabel('Time (ms)')
ax.set_title(f'Certify Phase Timing — {gs["total_ms"]}ms total')
for bar, val in zip(bars, times):
    ax.text(bar.get_x() + bar.get_width()/2, bar.get_height() + 5,
            f'{val}ms', ha='center', fontsize=10)

ax = axes[1]
bars = ax.bar(labels, checks, color=colors)
ax.set_ylabel('Checks')
ax.set_title(f'Certify Checks per Phase — {gs["total_checks"]} total')
for bar, val in zip(bars, checks):
    ax.text(bar.get_x() + bar.get_width()/2, bar.get_height() + 0.3,
            str(val), ha='center', fontsize=10)

plt.suptitle('primalspring certify: 4-Phase Validation (UniBin)',
             fontsize=13, fontweight='bold')
plt.tight_layout()
plt.savefig('/tmp/primalspring_02_certify.png', dpi=150, bbox_inches='tight')
plt.show()
```

## Energy Efficiency

CPU energy measurements (RAPL) during full test suite execution.
Rust's zero-overhead validation completes in a fraction of the energy
that an equivalent Python implementation would require.

```python
energy = bench['energy_estimate']

fig, ax = plt.subplots(figsize=(8, 4))
langs = ['Rust', 'Python (est.)']
wh = [energy['rust_test_suite_wh'], energy['python_equivalent_estimate_wh']]
bars = ax.bar(langs, wh, color=['#e67e22', '#3498db'], width=0.5)
ax.set_ylabel('Watt-hours')
ax.set_title(f'Test Suite Energy — {energy["ratio"]}')
for bar, val in zip(bars, wh):
    ax.text(bar.get_x() + bar.get_width()/2, bar.get_height() + 0.05,
            f'{val} Wh', ha='center', fontsize=12, fontweight='bold')

plt.tight_layout()
plt.savefig('/tmp/primalspring_02_energy.png', dpi=150, bbox_inches='tight')
plt.show()
```

## Validation Summary

| Metric | Value |
|--------|-------|
| Clean release build | 48.3s |
| Incremental build | 2.1s |
| Full test suite | 32.1s (613 tests) |
| Guidestone validation | 755ms (46 checks) |
| Graph parse (13 graphs) | 2.1ms |
| Rust/Python speedup | 2.6x – 21x |
| Energy efficiency | 9x more efficient |

---

**Provenance**: All results are content-addressed via BLAKE3 hashes,
tracked in rhizoCrypt DAG sessions, committed to the loamSpine ledger,
and witnessed with ed25519 signatures via sweetGrass braid.

**Reproduce**: See [primals.eco/lab/reproduce](https://primals.eco/lab/reproduce/)

**Source**: [ecoPrimals/primalSpring](https://github.com/ecoPrimals/primalSpring)

