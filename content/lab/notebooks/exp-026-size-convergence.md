+++
title = "Experiment 026 — System-Size Convergence"
description = "Rendered from exp-026-size-convergence.ipynb"
date = 2026-06-12
weight = 50

[extra]
domain = "Lab"
rendered_from = "exp-026-size-convergence.ipynb"
+++

<!-- Auto-generated from exp-026-size-convergence.ipynb by spore-validate render-notebooks -->

# Experiment 026 — System-Size Convergence

Validates the analysis methodology for finite-size extrapolation of WDM
transport coefficients. Uses synthetic D(N) = D∞ + α/N^(1/d) data with
noise to test linear regression extrapolation and convergence detection.
References:
Yeh & Hummer (2004) J. Phys. Chem. B 108, 15873
Dünweg & Kremer (1993) J. Chem. Phys. 99, 6983

**Domain**: Numerical Methods
**Faculty**: WDM
**Reference**: WDM system-size convergence analysis

**Data source**: `control/size_convergence/size_convergence.py` + `benchmark_*.json`

---

*This notebook is the publication-grade Python baseline for Experiment 026. The identical computations are validated in Rust (see `validate_*` binary) and delegated to barraCuda for GPU acceleration.*


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
benchmark_path = CONTROL / 'size_convergence' / 'benchmark_size_convergence.json'
with open(benchmark_path) as f:
    benchmark = json.load(f)

PASS_COLOR = '#2ecc71'
FAIL_COLOR = '#e74c3c'
INFO_COLOR = '#3498db'

print(f'Loaded benchmark: benchmark_size_convergence.json')
print(f'Provenance: {benchmark.get("_provenance", {})}')
```

## Validation


### Initialization


```python
print("groundSpring Exp 026: System-size Convergence for WDM Transport")
print("  Finite-size extrapolation D(N) = D∞ + α/N^(1/d)")

model = benchmark["model"]
exp = benchmark["expected_results"]

d_inf_true = float(model["d_inf_true"])
alpha_true = float(model["alpha_true"])
d_dim = float(model["d_dim"])
system_sizes = np.array(model["system_sizes"], dtype=np.float64)
noise_std = float(model["noise_std"])
n_replicas = int(model["n_replicas"])
seed = int(model["seed"])
threshold_pct = float(model["convergence_threshold_pct"])
threshold = threshold_pct / 100.0

rng = np.random.default_rng(seed)

# Generate synthetic D(N) data: D(N) = d_inf_true + alpha_true/N^(1/d) + noise
# Shape: (n_sizes, n_replicas)
exponent = 1.0 / d_dim
signal = d_inf_true + alpha_true / np.power(system_sizes, exponent)
noise = rng.normal(0, noise_std, size=(len(system_sizes), n_replicas))
d_values = signal[:, np.newaxis] + noise
```

### Replica means at each N


```python
d_mean = np.mean(d_values, axis=1)

# Linear regression on (1/N^(1/d), D_mean): D = D_inf + alpha * x, x = 1/N^(1/d)
x = 1.0 / np.power(system_sizes, exponent)
# Manual least-squares: slope = ss_xy/ss_xx, intercept = y_mean - slope * x_mean
x_mean = float(np.mean(x))
y_mean = float(np.mean(d_mean))
ss_xy = float(np.sum((x - x_mean) * (d_mean - y_mean)))
ss_xx = float(np.sum((x - x_mean) ** 2))
ss_yy = float(np.sum((d_mean - y_mean) ** 2))
alpha_fit = ss_xy / ss_xx if ss_xx > 0 else 0.0
d_inf_fit = y_mean - alpha_fit * x_mean
r_squared = (ss_xy**2) / (ss_xx * ss_yy) if ss_yy > 0 else 1.0
```

### Extrapolation relative error


```python
extrapolation_rel_err = abs(d_inf_fit - d_inf_true) / d_inf_true if d_inf_true != 0 else 0.0

# Find convergence point: smallest N where |D(N) - D_inf| / D_inf < threshold
convergence_n = None
for idx, n in enumerate(system_sizes):
    rel_err = abs(d_mean[idx] - d_inf_fit) / d_inf_fit if d_inf_fit != 0 else float("inf")
    if rel_err < threshold:
        convergence_n = float(n)
        break
```

### Mean D at largest N


```python
mean_at_largest_n = float(d_mean[-1])

# Residual std: std of (D_mean - fitted) at each N
fitted = d_inf_fit + alpha_fit * x
residuals = d_mean - fitted
residual_std = float(np.std(residuals))

print(f"\n  D∞ (fitted): {d_inf_fit:.6f}, α (fitted): {alpha_fit:.6f}, R²: {r_squared:.6f}")
print(f"  Convergence at N: {convergence_n}")
print(f"  Mean D at largest N: {mean_at_largest_n:.6f}, residual std: {residual_std:.6f}")
```

### Validation Checks


```python
# Check 1: Extrapolated D_inf within tolerance of true value
check_range(
    "Extrapolated D∞ within tolerance of true",
    d_inf_fit,
    d_inf_true - exp["d_inf_tolerance"],
    d_inf_true + exp["d_inf_tolerance"],
)

# Check 2: Fitted alpha in expected range
alpha_lo, alpha_hi = exp["alpha_range"]
check_range("Fitted α in expected range", alpha_fit, alpha_lo, alpha_hi)

# Check 3: R² above minimum (good fit)
check_min("R² above minimum", r_squared, exp["r_squared_min"])

# Check 4: Extrapolation relative error bounded
check_max(
    "Extrapolation relative error bounded",
    extrapolation_rel_err,
    exp["extrapolation_relative_error_max"],
)

# Check 5: Convergence achieved by N_max
convergence_n_max = float(exp["convergence_n_max"])
convergence_ok = convergence_n is not None and convergence_n <= convergence_n_max
check_true(
    "Convergence achieved by N_max",
    convergence_ok,
)

# Check 6: Mean D at largest N in expected range
mean_lo, mean_hi = exp["mean_at_largest_n_range"]
check_range("Mean D at largest N in expected range", mean_at_largest_n, mean_lo, mean_hi)

# Check 7: Residual std bounded
check_max("Residual std bounded", residual_std, exp["residual_std_max"])

# Results: {pass_count()}/{total_count()} checks passed
print_summary("Exp 026: System-size Convergence for WDM Transport")
```

## Visualization


```python
# Publication-grade summary chart for Exp 026
fig, ax = plt.subplots(figsize=(8, 4))

p, f_count, t = pass_count(), fail_count(), total_count()
ax.barh(['Pass', 'Fail'], [p, f_count], color=[PASS_COLOR, FAIL_COLOR])
ax.set_xlim(0, max(t * 1.15, 1))
ax.set_title('Exp 026: System-Size Convergence — Validation Results')
ax.set_xlabel('Check Count')
for i, v in enumerate([p, f_count]):
    if v > 0:
        ax.text(v + 0.3, i, str(v), va='center', fontweight='bold')

plt.tight_layout()
plt.savefig(f'/tmp/groundspring_exp026.png', dpi=150, bbox_inches='tight')
plt.show()
print(f'\nResult: {p}/{t} PASS, {f_count}/{t} FAIL')
```

## Provenance & Summary

| Field | Value |
|-------|-------|
| Experiment | 026 — System-Size Convergence |
| Domain | Numerical Methods |
| Reference | WDM system-size convergence analysis |
| Faculty | WDM |
| Python baseline | `control/size_convergence/size_convergence.py` |
| Benchmark JSON | `control/size_convergence/benchmark_size_convergence.json` |
| Rust validator | `validate_*` binary (exit-code protocol) |
| Rust speedup | See benchmark comparison notebook |
| License | AGPL-3.0-or-later |

**Provenance chain**: Python baseline → Rust validation → barraCuda GPU → metalForge cross-substrate → primal IPC composition

See [primals.eco](https://primals.eco) for rendered lab notebooks.


