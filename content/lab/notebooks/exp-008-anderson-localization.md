+++
title = "Experiment 008 — Anderson Localization"
description = "Rendered from exp-008-anderson-localization.ipynb"
date = 2026-06-11
weight = 50

[extra]
domain = "Lab"
rendered_from = "exp-008-anderson-localization.ipynb"
+++

<!-- Auto-generated from exp-008-anderson-localization.ipynb by spore-validate render-notebooks -->

# Experiment 008 — Anderson Localization

Numerically verifies Anderson localization in the 1D tight-binding model
to answer:
1. Does the Lyapunov exponent vanish for a clean (W=0) system?
2. Is it positive for ANY disorder W > 0?
3. Does the localization length follow Thouless scaling (xi ~ C/W^2)?
4. Does stronger disorder give shorter localization length?

**Method:**
- 1D Anderson model: H psi(n) = psi(n+1) + psi(n-1) + V(n) psi(n)
- V(n) uniform in [-W/2, W/2] (disorder strength W)
- Lyapunov exponent via transfer matrix method (Furstenberg theorem)
- Averaging over many disorder realizations for convergence

**Reference:**
Anderson (1958) Phys Rev 109:1492
Bourgain & Kachkovskiy (2018) GAFA 29:3-43
Thouless (1972) J Phys C 5:77

**Cross-spring with hotSpring: spectral theory, lattice physics.**

**Domain**: Condensed Matter
**Faculty**: Kachkovskiy (MSU)
**Reference**: Anderson (1958) Phys Rev 109:1492; Bourgain & Kachkovskiy (2018)

**Data source**: `control/anderson_localization/anderson_localization.py` + `benchmark_*.json`

---

*This notebook is the publication-grade Python baseline for Experiment 008. The identical computations are validated in Rust (see `validate_*` binary) and delegated to barraCuda for GPU acceleration.*


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
benchmark_path = CONTROL / 'anderson_localization' / 'benchmark_anderson_localization.json'
with open(benchmark_path) as f:
    benchmark = json.load(f)

PASS_COLOR = '#2ecc71'
FAIL_COLOR = '#e74c3c'
INFO_COLOR = '#3498db'

print(f'Loaded benchmark: benchmark_anderson_localization.json')
print(f'Provenance: {benchmark.get("_provenance", {})}')
```

## Model Implementation


```python
def anderson_potential(n: int, disorder: float, rng: np.random.Generator) -> np.ndarray:
    """Generate random potential V(n) uniform in [-W/2, W/2]."""
    if disorder <= 0:
        return np.zeros(n)
    return rng.uniform(-disorder / 2, disorder / 2, size=n)


def lyapunov_exponent(potential: np.ndarray, energy: float) -> float:
    """Compute Lyapunov exponent via transfer matrix method.

    For the 1D Anderson model at energy E, the transfer matrix at site n is:
      T_n = [[E - V(n), -1], [1, 0]]

    The Lyapunov exponent gamma = lim (1/N) ln ||T_N ... T_1||.
    We use the numerically stable QR-based method.
    """
    n = len(potential)
    if n == 0:
        return 0.0

    log_growth = 0.0
    vec = np.array([1.0, 0.0])

    for i in range(n):
        new_0 = (energy - potential[i]) * vec[0] - vec[1]
        new_1 = vec[0]
        vec[0] = new_0
        vec[1] = new_1

        norm = math.sqrt(vec[0] ** 2 + vec[1] ** 2)
        if norm > 0:
            log_growth += math.log(norm)
            vec[0] /= norm
            vec[1] /= norm

    return log_growth / n


def lyapunov_averaged(
    n_sites: int,
    disorder: float,
    energy: float,
    n_realizations: int,
    base_seed: int,
) -> float:
    """Average Lyapunov exponent over many disorder realizations."""
    total = 0.0
    for i in range(n_realizations):
        rng = np.random.default_rng(base_seed + i)
        pot = anderson_potential(n_sites, disorder, rng)
        total += lyapunov_exponent(pot, energy)
    return total / n_realizations


def localization_length(gamma: float) -> float:
    """Localization length xi = 1 / gamma."""
    if gamma <= 0:
        return float("inf")
    return 1.0 / gamma
```

## Validation


### Initialization


```python
model = benchmark["model"]
pred = benchmark["analytical_predictions"]
exp = benchmark["expected_results"]

n_sites = model["n_sites"]
n_real = model["n_realizations"]
energy = model["energy"]
disorders = model["disorder_strengths"]

print("groundSpring Exp 008: Anderson Localization")
print(f"  Model: 1D tight-binding, {n_sites} sites, {n_real} realizations")
print("  Cross-spring: hotSpring (spectral theory, lattice physics)")
```

### Clean System (W=0


```python
gamma_clean = lyapunov_averaged(n_sites, 0.0, energy, 1, 42)
print(f"  Lyapunov exponent (W=0): {gamma_clean:.6f}")

check_approx(
    "Clean system γ ≈ 0",
    gamma_clean, pred["clean_lyapunov"], exp["clean_lyapunov_tol"],
)
```

### Disorder Sweep


```python
gammas = {}
for w in disorders:
    g = lyapunov_averaged(n_sites, w, energy, n_real, 42)
    gammas[w] = g
    xi = localization_length(g)
    xi_str = f"{xi:.1f}" if xi < 1e6 else "∞"
    print(f"  W={w:.1f}: γ={g:.6f}, ξ={xi_str}")

nonzero_disorders = [w for w in disorders if w > 0]
nonzero_gammas = [gammas[w] for w in nonzero_disorders]

check_true(
    "All disordered states have γ > 0",
    all(g > 0 for g in nonzero_gammas),
)

check_true(
    "γ increases monotonically with W",
    all(
        nonzero_gammas[i] <= nonzero_gammas[i + 1]
        for i in range(len(nonzero_gammas) - 1)
    ),
)

check_true(
    "Strong disorder (W=8) γ > 0.3",
    gammas[8.0] > exp["strong_disorder_lyapunov_min"],
)
```

### Thouless Scaling


```python
w_test = 1.0
gamma_test = gammas[w_test]
xi_test = localization_length(gamma_test)
thouless_ratio = xi_test * (w_test ** 2)

print(f"  At W={w_test}: ξ={xi_test:.1f}, C = ξ×W² = {thouless_ratio:.1f}")
print(f"  Expected C ≈ {pred['thouless_coefficient']:.0f} (Thouless/Derrida-Gardner)")

check_range(
    "Thouless coefficient C = ξ·W²",
    thouless_ratio,
    exp["thouless_ratio_range"][0],
    exp["thouless_ratio_range"][1],
)
```

### Localization Length vs Disorder


```python
xi_values = [localization_length(gammas[w]) for w in nonzero_disorders]
for w, xi in zip(nonzero_disorders, xi_values, strict=True):
    xi_str = f"{xi:.1f}" if xi < 1e6 else "∞"
    print(f"  W={w:.1f}: ξ={xi_str}")

check_true(
    "ξ decreases with increasing W",
    all(
        xi_values[i] >= xi_values[i + 1]
        for i in range(len(xi_values) - 1)
    ),
)
```

### Determinism


```python
rng_a = np.random.default_rng(12345)
rng_b = np.random.default_rng(12345)
pot_a = anderson_potential(1000, 2.0, rng_a)
pot_b = anderson_potential(1000, 2.0, rng_b)
check_true("Potential deterministic", np.array_equal(pot_a, pot_b))

g_a = lyapunov_exponent(pot_a, 0.0)
g_b = lyapunov_exponent(pot_b, 0.0)
check_true("Lyapunov deterministic", g_a == g_b)
```

### Key Findings


```python
print(f"\n{'=' * 72}")
```

### ANY disorder localizes: all γ > 0 for W > 0 (Anderson 1958


```python
print(f"3. Thouless scaling: ξ ≈ {thouless_ratio:.0f}/W² at band center")
print(f"4. Strong disorder (W=8): ξ ≈ {localization_length(gammas[8.0]):.1f} sites")
```

### This is the mathematical foundation of groundSpring


```python
print("   noise (disorder) traps signal (wave) in ALL dimensions for 1D")

# Results: {pass_count()}/{total_count()} checks passed
print_summary("Exp 008: Anderson Localization")
```

## Visualization


```python
# Publication-grade summary chart for Exp 008
fig, ax = plt.subplots(figsize=(8, 4))

p, f_count, t = pass_count(), fail_count(), total_count()
ax.barh(['Pass', 'Fail'], [p, f_count], color=[PASS_COLOR, FAIL_COLOR])
ax.set_xlim(0, max(t * 1.15, 1))
ax.set_title('Exp 008: Anderson Localization — Validation Results')
ax.set_xlabel('Check Count')
for i, v in enumerate([p, f_count]):
    if v > 0:
        ax.text(v + 0.3, i, str(v), va='center', fontweight='bold')

plt.tight_layout()
plt.savefig(f'/tmp/groundspring_exp008.png', dpi=150, bbox_inches='tight')
plt.show()
print(f'\nResult: {p}/{t} PASS, {f_count}/{t} FAIL')
```

## Provenance & Summary

| Field | Value |
|-------|-------|
| Experiment | 008 — Anderson Localization |
| Domain | Condensed Matter |
| Reference | Anderson (1958) Phys Rev 109:1492; Bourgain & Kachkovskiy (2018) |
| Faculty | Kachkovskiy (MSU) |
| Python baseline | `control/anderson_localization/anderson_localization.py` |
| Benchmark JSON | `control/anderson_localization/benchmark_anderson_localization.json` |
| Rust validator | `validate_*` binary (exit-code protocol) |
| Rust speedup | See benchmark comparison notebook |
| License | AGPL-3.0-or-later |

**Provenance chain**: Python baseline → Rust validation → barraCuda GPU → metalForge cross-substrate → primal IPC composition

See [primals.eco](https://primals.eco) for rendered lab notebooks.


