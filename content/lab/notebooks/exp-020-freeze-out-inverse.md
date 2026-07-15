+++
title = "Experiment 020 — Freeze-Out Inverse Problem"
description = "Rendered from exp-020-freeze-out-inverse.ipynb"
date = 2026-07-10
weight = 50

[extra]
domain = "physics"
rendered_from = "exp-020-freeze-out-inverse.ipynb"
+++

<!-- Auto-generated from exp-020-freeze-out-inverse.ipynb by spore-validate render-notebooks -->

# Experiment 020 — Freeze-Out Inverse Problem

Chi-squared fitting inverse problem: recover freeze-out curve parameters
(T0, kappa2) from noisy observations of T_f(mu_B) using 2D grid search.
Validates polynomial forward model, chi-squared statistic, grid-search
recovery, and noise degradation of precision.

**Cross-spring: extends Exp 005 (seismic grid-search inversion) with**
chi-squared fitting on polynomial models.

**Domain**: Lattice Qcd
**Faculty**: Bazavov (MSU)
**Reference**: Bazavov — QCD freeze-out temperature

**Data source**: `control/freeze_out_inverse/freeze_out_inverse.py` + `benchmark_*.json`

---

*This notebook is the publication-grade Python baseline for Experiment 020. The identical computations are validated in Rust (see `validate_*` binary) and delegated to barraCuda for GPU acceleration.*


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
benchmark_path = CONTROL / 'freeze_out_inverse' / 'benchmark_freeze_out.json'
with open(benchmark_path) as f:
    benchmark = json.load(f)

PASS_COLOR = '#2ecc71'
FAIL_COLOR = '#e74c3c'
INFO_COLOR = '#3498db'

print(f'Loaded benchmark: benchmark_freeze_out.json')
print(f'Provenance: {benchmark.get("_provenance", {})}')
```

## Validation


### Initialization


```python
with open(bench_path) as f:

model = benchmark["model"]
grid = benchmark["grid"]
exp = benchmark["expected_results"]

true_t0 = model["true_t0"]
true_k2 = model["true_kappa2"]
mu_b = np.array(model["mu_b_values"])
noise_std = model["noise_std"]
seed = model["seed"]
n_rep = model["n_replicates"]
```

### Forward model correctness


```python
t_at_0 = freeze_out_curve(true_t0, true_k2, 0.0)
check_true("T_f(0) = T0", abs(t_at_0 - true_t0) < 1e-12)

t_at_400 = freeze_out_curve(true_t0, true_k2, 400.0)
expected_400 = true_t0 * (1.0 - true_k2 * (400.0 / true_t0) ** 2)
check_true("T_f(400) matches formula", abs(t_at_400 - expected_400) < 1e-12)
```

### Chi-squared at truth


```python
rng = np.random.default_rng(seed)
true_curve = np.array([freeze_out_curve(true_t0, true_k2, m) for m in mu_b])
noise = rng.normal(0, noise_std, len(mu_b))
obs = true_curve + noise
chi2_truth = chi_squared(obs, true_curve, noise_std)
n_dof = len(mu_b) - 2
chi2_per_dof = chi2_truth / n_dof
check_max("Chi2/dof at truth reasonable", chi2_per_dof, exp["chi2_per_dof_max"])
```

### Grid search recovery (single realization


```python
fit_t0, fit_k2, _fit_chi2 = grid_search_2d(
    obs,
    mu_b,
    noise_std,
    grid["t0_range"],
    grid["t0_step"],
    grid["kappa2_range"],
    grid["kappa2_step"],
)
check_max("T0 recovery error", abs(fit_t0 - true_t0), exp["t0_recovery_tol"])
check_max("kappa2 recovery error", abs(fit_k2 - true_k2), exp["kappa2_recovery_tol"])
```

### Replicate coverage


```python
coverage = 0
for i in range(n_rep):
    rng_i = np.random.default_rng(seed + i + 1)
    noise_i = rng_i.normal(0, noise_std, len(mu_b))
    obs_i = true_curve + noise_i
    t0_i, k2_i, _ = grid_search_2d(
        obs_i,
        mu_b,
        noise_std,
        grid["t0_range"],
        grid["t0_step"],
        grid["kappa2_range"],
        grid["kappa2_step"],
    )
    if abs(t0_i - true_t0) <= exp["t0_recovery_tol"] and abs(k2_i - true_k2) <= exp["kappa2_recovery_tol"]:
        coverage += 1
frac = coverage / n_rep
check_min("Replicate coverage", frac, exp["replicate_coverage_min"])
```

### Noise degrades precision


```python
rng_low = np.random.default_rng(seed + 999)
obs_low_noise = true_curve + rng_low.normal(0, noise_std * 0.1, len(mu_b))
t0_low, k2_low, _ = grid_search_2d(
    obs_low_noise,
    mu_b,
    noise_std * 0.1,
    grid["t0_range"],
    grid["t0_step"],
    grid["kappa2_range"],
    grid["kappa2_step"],
)
err_high_noise = abs(fit_t0 - true_t0) + abs(fit_k2 - true_k2)
err_low_noise = abs(t0_low - true_t0) + abs(k2_low - true_k2)
check_true("Lower noise improves recovery", err_low_noise <= err_high_noise + 0.5)
```

### Determinism


```python
rng_d1 = np.random.default_rng(seed)
obs_d1 = true_curve + rng_d1.normal(0, noise_std, len(mu_b))
rng_d2 = np.random.default_rng(seed)
obs_d2 = true_curve + rng_d2.normal(0, noise_std, len(mu_b))
check_true("Observations deterministic", np.array_equal(obs_d1, obs_d2))

# Results: {pass_count()}/{total_count()} checks passed
print_summary("Exp 020: Freeze-Out Inverse Problem")
```

## Visualization


```python
# Publication-grade summary chart for Exp 020
fig, ax = plt.subplots(figsize=(8, 4))

p, f_count, t = pass_count(), fail_count(), total_count()
ax.barh(['Pass', 'Fail'], [p, f_count], color=[PASS_COLOR, FAIL_COLOR])
ax.set_xlim(0, max(t * 1.15, 1))
ax.set_title('Exp 020: Freeze-Out Inverse Problem — Validation Results')
ax.set_xlabel('Check Count')
for i, v in enumerate([p, f_count]):
    if v > 0:
        ax.text(v + 0.3, i, str(v), va='center', fontweight='bold')

plt.tight_layout()
plt.savefig(f'/tmp/groundspring_exp020.png', dpi=150, bbox_inches='tight')
plt.show()
print(f'\nResult: {p}/{t} PASS, {f_count}/{t} FAIL')
```

## Provenance & Summary

| Field | Value |
|-------|-------|
| Experiment | 020 — Freeze-Out Inverse Problem |
| Domain | Lattice Qcd |
| Reference | Bazavov — QCD freeze-out temperature |
| Faculty | Bazavov (MSU) |
| Python baseline | `control/freeze_out_inverse/freeze_out_inverse.py` |
| Benchmark JSON | `control/freeze_out_inverse/benchmark_freeze_out.json` |
| Rust validator | `validate_*` binary (exit-code protocol) |
| Rust speedup | See benchmark comparison notebook |
| License | AGPL-3.0-or-later |

**Provenance chain**: Python baseline → Rust validation → barraCuda GPU → metalForge cross-substrate → primal IPC composition

See [primals.eco](https://primals.eco) for rendered lab notebooks.


