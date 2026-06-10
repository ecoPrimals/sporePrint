+++
title = "Cross-Spring Connections — airSpring"
description = "Rendered from 04-cross-spring-connections.ipynb"
date = 2026-06-10
weight = 50

[extra]
domain = "Lab"
rendered_from = "04-cross-spring-connections.ipynb"
+++

<!-- Auto-generated from 04-cross-spring-connections.ipynb by spore-validate render-notebooks -->

# Cross-Spring Connections — airSpring

barraCuda integration (25 Tier A GPU modules), cross-spring shader evolution
(767+ WGSL shaders), and primal consumption matrix across the ecosystem.

**Data sources**: `cross_spring_matrix.json`, `composition_validation.json`

**Reproduce**: `cargo run --release --bin bench_cross_spring_evolution` (146/146)

**For other springs**: Replace shader families and primal consumption with your
domain's ecosystem connections. The cross-spring matrix pattern shows how
springs give and receive capabilities.

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

matrix = load('cross_spring_matrix.json')
comp = load('composition_validation.json')

bc = matrix['barracuda_integration']
print(f"barraCuda {bc['version']} (wgpu {bc['wgpu_version']})")
print(f"Ecosystem shaders: {bc['total_shaders_ecosystem']}+")
print(f"Tier A GPU modules: {bc['tier_a_gpu_modules']}")
print(f"Upstream batched ops: {bc['upstream_batched_ops']}")
print(f"local_dispatch retired: {bc['local_dispatch_retired']}")
print(f"Cross-spring checks: {matrix['cross_spring_checks']['total']}")
```

## Cross-Spring Shader Families

airSpring consumes shaders from 4 sibling springs and contributed 3 upstream fixes.

```python
families = matrix['cross_spring_shader_families']
springs = [f['spring'] for f in families]
shader_counts = [f['shaders'] for f in families]

fig, ax = plt.subplots(figsize=(8, 4))
colors = ['#e74c3c', '#3498db', '#9b59b6', '#f39c12']
bars = ax.bar(springs, shader_counts, color=colors, edgecolor='white')
ax.set_ylabel('Shaders')
ax.set_title(f'Cross-Spring Shader Families ({bc["total_shaders_ecosystem"]}+ ecosystem total)')
for bar, count in zip(bars, shader_counts):
    if count > 0:
        ax.text(bar.get_x() + bar.get_width()/2, bar.get_height() + 0.5,
                str(count), ha='center', fontsize=10)
plt.tight_layout()
plt.savefig('/tmp/airspring_04_shaders.png', dpi=150)
plt.show()

print('\nWhat airSpring uses from each spring:')
for f in families:
    if f['shaders'] > 0:
        print(f"  {f['spring']}: {f['airspring_uses']}")

print('\nWhat airSpring contributed upstream:')
for f in families:
    if f['airspring_contributed']:
        print(f"  {f['spring']}: {f['airspring_contributed']}")
```

## Primal Consumption Matrix

```python
consumption = matrix['primal_consumption']
primals = list(consumption.keys())
wired = [1 if consumption[p]['wired'] else 0 for p in primals]
cap_counts = [len(consumption[p]['capabilities_used']) for p in primals]

fig, (ax1, ax2) = plt.subplots(1, 2, figsize=(12, 5))

wire_colors = ['#2ecc71' if w else '#e74c3c' for w in wired]
ax1.barh(primals, wired, color=wire_colors, edgecolor='white')
ax1.set_xlim(0, 1.5)
ax1.set_xticks([])
for i, (p, w) in enumerate(zip(primals, wired)):
    label = 'IPC wired' if w else consumption[p].get('note', 'not wired')
    ax1.text(1.05, i, label, va='center', fontsize=8)
ax1.set_title('Primal IPC Status')

ax2.barh(primals, cap_counts, color='#3498db', edgecolor='white')
ax2.set_xlabel('Capabilities Used')
ax2.set_title('Capabilities Consumed per Primal')
for i, v in enumerate(cap_counts):
    ax2.text(v + 0.1, i, str(v), va='center', fontsize=9)

plt.tight_layout()
plt.savefig('/tmp/airspring_04_consumption.png', dpi=150)
plt.show()
```

## Hardware Validation Matrix

```python
hw = matrix['hardware_validated']
print('Validated Hardware:')
for h in hw:
    details = h.get('features', h.get('api', h.get('device', '')))
    print(f"  {h['component']:6s} | {h['model']:30s} | {details}")
```

## Summary

| Metric | Value |
|--------|-------|
| barraCuda version | 0.3.7 (wgpu 28) |
| Ecosystem shaders | 767+ WGSL (f64 canonical) |
| Tier A GPU modules | 25 (20 upstream batched, 5 dedicated) |
| Cross-spring checks | 211 (146 evolution + 32 provenance + 33 cross-validation) |
| Upstream contributions | 8 ops + 3 bug fixes |
| Primals IPC-wired | 5 / 9 core |
| Hardware substrates | 4 (CPU, GPU×2, NPU) |
| local_dispatch | Retired (Write→Absorb→Lean complete) |

**Provenance**: airSpring v0.10.0 · barraCuda 0.3.7 · [primals.eco](https://primals.eco)

