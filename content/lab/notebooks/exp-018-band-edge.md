+++
title = "Experiment 018 — Band Edge Structure"
description = "Rendered from exp-018-band-edge.ipynb"
date = 2026-06-01
weight = 50

[extra]
domain = "Lab"
rendered_from = "exp-018-band-edge.ipynb"
+++

<!-- Auto-generated from exp-018-band-edge.ipynb by spore-validate render-notebooks -->

# Experiment 018 — Band Edge Structure

Where do propagating waves transition to evanescent in periodic
structures?  Band edges are the mathematical boundary between
"signal gets through" and "noise kills it."

**Method:**
- 1D tight-binding chain with periodic potential V_n of period p
- Transfer matrix: T(E) = prod_n [[(E-V_n)/t, -1], [1, 0]]
- Bands where |Tr(T)/2| <= 1, gaps where |Tr(T)/2| > 1
- Band edges at |Tr(T)/2| = 1
- Finite system eigenvalues via tridiagonal diagonalization
- Gap width proportional to potential contrast |V1-V2|

**Reference:**
Filonov & Kachkovskiy (2018) Acta Math 221:59-80
Anderson (1958) Phys Rev 109:1492-1505

**Cross-spring: hotSpring (spectral), Exp 008 (Anderson), Exp 012 (transport).**

**Domain**: Condensed Matter
**Faculty**: Kachkovskiy (MSU)
**Reference**: Kachkovskiy — spectral gap detection

**Data source**: `control/band_edge/band_edge.py` + `benchmark_*.json`

---

*This notebook is the publication-grade Python baseline for Experiment 018. The identical computations are validated in Rust (see `validate_*` binary) and delegated to barraCuda for GPU acceleration.*


```python
import json
import math
import sys
from pathlib import Path

import numpy as np
import matplotlib.pyplot as plt

# Wire path to groundSpring control/ for common utilities
CONTROL = Path('..') / '..' / 'control'
sys.path.insert(0, str(CONTROL))
from common import *  # noqa: F403 — validation harness

# Load benchmark data
benchmark_path = CONTROL / 'band_edge' / 'benchmark_band_edge.json'
with open(benchmark_path) as f:
    benchmark = json.load(f)

PASS_COLOR = '#2ecc71'
FAIL_COLOR = '#e74c3c'
INFO_COLOR = '#3498db'

print(f'Loaded benchmark: benchmark_band_edge.json')
print(f'Provenance: {benchmark.get("_provenance", {})}')
```

## Validation


### Initialization


```python
model = benchmark["model"]
pred = benchmark["analytical_predictions"]
exp = benchmark["expected_results"]

t_hop = model["hopping"]
pot_2 = model["period_2_potential"]
pot_3 = model["period_3_potential"]
n_scan = model["n_energy_scan"]
e_range = tuple(model["energy_range"])
n_periods = model["n_periods_finite"]

print("groundSpring Exp 018: Band Edge Structure (Filonov-Kachkovskiy 2018)")
print(f"  Period-2 potential: {pot_2}, hopping: {t_hop}")
print(f"  Period-3 potential: {pot_3}")
print(f"  Energy scan: {e_range} with {n_scan} points")
print("  Cross-spring: hotSpring, Exp 008, Exp 012")
```

### Free Lattice (V=0


```python
free_edges = find_band_edges([0.0], t_hop, e_range, n_scan)
n_free_bands = count_bands([0.0], t_hop, e_range, n_scan)
print(f"  Free lattice edges: {[f'{e:.3f}' for e in free_edges]}")
print(f"  Number of bands: {n_free_bands}")

expected_edges = pred["free_band_edges"]
check_true("Free lattice has 2 band edges", len(free_edges) == 2)
if len(free_edges) == 2:
    check_range(
        "Lower band edge ≈ -2t",
        free_edges[0],
        expected_edges[0] - 0.05,
        expected_edges[0] + 0.05,
    )
    check_range(
        "Upper band edge ≈ +2t",
        free_edges[1],
        expected_edges[1] - 0.05,
        expected_edges[1] + 0.05,
    )
```

### Period-2 Gap Opening


```python
p2_edges = find_band_edges(pot_2, t_hop, e_range, n_scan)
p2_bands = count_bands(pot_2, t_hop, e_range, n_scan)
print(f"  Period-2 edges: {[f'{e:.3f}' for e in p2_edges]}")
print(f"  Number of bands: {p2_bands}")

check_true("Period-2 opens a gap (4 edges)", len(p2_edges) == 4)
check_true("Period-2 has 2 bands", p2_bands == 2)

if len(p2_edges) == 4:
    gap_lo = p2_edges[1]
    gap_hi = p2_edges[2]
    gap_width = gap_hi - gap_lo
    expected_gap = pred["period_2_gap_width"]
    print(f"  Gap: [{gap_lo:.3f}, {gap_hi:.3f}], width = {gap_width:.3f} (expected {expected_gap})")
    check_range(
        "Gap width ≈ |V1-V2|",
        gap_width,
        expected_gap - exp["gap_width_tolerance"],
        expected_gap + exp["gap_width_tolerance"],
    )
```

### Period-3 Band Count


```python
p3_bands = count_bands(pot_3, t_hop, e_range, n_scan)
p3_edges = find_band_edges(pot_3, t_hop, e_range, n_scan)
print(f"  Period-3 edges: {[f'{e:.3f}' for e in p3_edges]}")
print(f"  Number of bands: {p3_bands} (expected {pred['n_bands_period_3']})")

check_true(
    f"Period-3 has {pred['n_bands_period_3']} bands",
    p3_bands == pred["n_bands_period_3"],
)
```

### Gap Width vs Potential Contrast


```python
gap_widths = []
for dv in model["period_2_gap_widths_to_test"]:
    pot = [dv / 2.0, -dv / 2.0]
    edges = find_band_edges(pot, t_hop, e_range, n_scan)
    if len(edges) >= 4:
        gw = edges[2] - edges[1]
    elif len(edges) == 2:
        gw = 0.0
    else:
        gw = 0.0
    gap_widths.append(gw)
    print(f"  ΔV={dv:.1f}: gap width = {gw:.3f}")

monotone = all(
    gap_widths[i] <= gap_widths[i + 1] + 0.01
    for i in range(len(gap_widths) - 1)
)
check_true("Gap width increases with ΔV", monotone)
```

### Finite System Eigenvalues


```python
h_mat = build_periodic_hamiltonian(pot_2, t_hop, n_periods)
eigenvalues = np.sort(np.linalg.eigvalsh(h_mat))

in_gap = 0
for ev in eigenvalues:
    ht = transfer_matrix_trace(ev, pot_2, t_hop)
    if abs(ht) > 1.05:
        in_gap += 1

n_total = len(eigenvalues)
frac_in_band = (n_total - in_gap) / n_total
print(f"  {n_total} eigenvalues, {in_gap} in gap region, {frac_in_band:.1%} in bands")

check_true(
    "Eigenvalues mostly within bands (≥95%)",
    frac_in_band >= 0.95,
)
```

### Determinism


```python
t1 = transfer_matrix_trace(0.5, pot_2, t_hop)
t2 = transfer_matrix_trace(0.5, pot_2, t_hop)
check_true("Transfer matrix deterministic", t1 == t2)
```

### Key findings


```python
print(f"\n{'=' * 72}")
```

### Key Findings


```python
print(f"\n1. Free lattice: single band [{expected_edges[0]}, {expected_edges[1]}]")
print(f"2. Period-2 (V=[{pot_2[0]},{pot_2[1]}]): gap width = {pred['period_2_gap_width']} = |V1-V2|")
print(f"3. Period-3: {p3_bands} bands (number of bands = period)")
```

### Gap width proportional to potential contrast ΔV


```python
print(f"5. Finite system: {frac_in_band:.1%} of eigenvalues within transfer-matrix bands")
print()
print("  Filonov & Kachkovskiy (2018) proved that band edges of periodic")
print("  elliptic operators have definite structure. This experiment shows")
print("  the 1D tight-binding analog: band gaps separate propagating from")
print("  evanescent regimes, with gap width controlled by potential contrast.")

# Results: {pass_count()}/{total_count()} checks passed
print_summary("Exp 018: Band Edge Structure")
```

## Visualization


```python
# Publication-grade summary chart for Exp 018
fig, ax = plt.subplots(figsize=(8, 4))

p, f_count, t = pass_count(), fail_count(), total_count()
ax.barh(['Pass', 'Fail'], [p, f_count], color=[PASS_COLOR, FAIL_COLOR])
ax.set_xlim(0, max(t * 1.15, 1))
ax.set_title('Exp 018: Band Edge Structure — Validation Results')
ax.set_xlabel('Check Count')
for i, v in enumerate([p, f_count]):
    if v > 0:
        ax.text(v + 0.3, i, str(v), va='center', fontweight='bold')

plt.tight_layout()
plt.savefig(f'/tmp/groundspring_exp018.png', dpi=150, bbox_inches='tight')
plt.show()
print(f'\nResult: {p}/{t} PASS, {f_count}/{t} FAIL')
```

## Provenance & Summary

| Field | Value |
|-------|-------|
| Experiment | 018 — Band Edge Structure |
| Domain | Condensed Matter |
| Reference | Kachkovskiy — spectral gap detection |
| Faculty | Kachkovskiy (MSU) |
| Python baseline | `control/band_edge/band_edge.py` |
| Benchmark JSON | `control/band_edge/benchmark_band_edge.json` |
| Rust validator | `validate_*` binary (exit-code protocol) |
| Rust speedup | See benchmark comparison notebook |
| License | AGPL-3.0-or-later |

**Provenance chain**: Python baseline → Rust validation → barraCuda GPU → metalForge cross-substrate → primal IPC composition

See [primals.eco](https://primals.eco) for rendered lab notebooks.


