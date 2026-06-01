+++
title = "Experiment 021 — Spectral Reconstruction"
description = "Rendered from exp-021-spectral-recon.ipynb"
date = 2026-06-01
weight = 50

[extra]
domain = "Lab"
rendered_from = "exp-021-spectral-recon.ipynb"
+++

<!-- Auto-generated from exp-021-spectral-recon.ipynb by spore-validate render-notebooks -->

# Experiment 021 — Spectral Reconstruction

Tikhonov-regularized reconstruction of a spectral function from a noisy
Euclidean correlator.  Validates kernel matrix construction, forward model,
Cholesky-based normal equation solve, peak recovery under regularization,
and the bias-variance trade-off as lambda varies.

**Cross-spring: extends Exp 005 (seismic inversion) and Exp 020 (chi-squared**
fitting) with a continuous inverse problem requiring regularization.

**Domain**: Lattice Qcd
**Faculty**: Bazavov (MSU)
**Reference**: Bazavov — Tikhonov regularization of lattice QCD correlators

**Data source**: `control/spectral_recon/spectral_recon.py` + `benchmark_*.json`

---

*This notebook is the publication-grade Python baseline for Experiment 021. The identical computations are validated in Rust (see `validate_*` binary) and delegated to barraCuda for GPU acceleration.*


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
benchmark_path = CONTROL / 'spectral_recon' / 'benchmark_spectral_recon.json'
with open(benchmark_path) as f:
    benchmark = json.load(f)

PASS_COLOR = '#2ecc71'
FAIL_COLOR = '#e74c3c'
INFO_COLOR = '#3498db'

print(f'Loaded benchmark: benchmark_spectral_recon.json')
print(f'Provenance: {benchmark.get("_provenance", {})}')
```

## Validation


### Initialization


```python
with open(bench_path) as f:

sf = benchmark["spectral_function"]
grid = benchmark["grid"]
noise_cfg = benchmark["noise"]
reg = benchmark["regularization"]
exp = benchmark["expected_results"]

n_tau = grid["n_tau"]
n_omega = grid["n_omega"]
tau = np.linspace(0, grid["tau_max"], n_tau, endpoint=False) + grid["tau_max"] / n_tau
omega = np.linspace(0, grid["omega_max"], n_omega, endpoint=False) + grid["omega_max"] / n_omega

rho_true = gaussian_peak(omega, sf["omega_center"], sf["omega_width"], sf["amplitude"])
```

### Kernel and forward model


```python
kernel = build_kernel(tau, omega)
g_exact = forward_correlator(kernel, rho_true)
g_recon = kernel @ tikhonov_solve(kernel, g_exact, 0.0)
rmse_noiseless = np.sqrt(np.mean((g_exact - g_recon) ** 2))
check_max(
    "Noiseless forward RMSE",
    rmse_noiseless,
    exp["forward_rmse_noiseless_max"],
)
```

### Cholesky residual check


```python
rho_noiseless = tikhonov_solve(kernel, g_exact, 1e-12)
residual = np.max(np.abs(kernel @ rho_noiseless - g_exact))
check_max("Cholesky max residual", residual, exp["cholesky_residual_max"])
```

### Noisy reconstruction at optimal lambda


```python
rng = np.random.default_rng(noise_cfg["seed"])
noise = rng.normal(0, noise_cfg["correlator_noise_std"], n_tau)
g_noisy = g_exact + noise
lam_opt = reg["optimal_lambda"]
rho_recon = tikhonov_solve(kernel, g_noisy, lam_opt)
peak_idx = np.argmax(rho_recon)
peak_omega = omega[peak_idx]
check_max(
    "Peak location error",
    abs(peak_omega - sf["omega_center"]),
    exp["peak_location_tol"],
)
check_true("Peak value positive", rho_recon[peak_idx] > 0)
```

### Regularization trade-off


```python
lambdas = reg["lambda_values"]
rmses = []
for lam in lambdas:
    rho_l = tikhonov_solve(kernel, g_noisy, lam)
    rmse_l = np.sqrt(np.mean((rho_l - rho_true) ** 2))
    rmses.append(rmse_l)

small_rmse = rmses[0]
large_rmse = rmses[-1]
opt_rmse = rmses[2]
check_true(
    "Small lambda amplifies noise (higher RMSE than optimal)",
    small_rmse >= opt_rmse * 0.5,
)
check_true(
    "Large lambda over-smooths (higher RMSE than optimal)",
    large_rmse >= opt_rmse * 0.5,
)
check_range(
    "Optimal lambda RMSE in range",
    opt_rmse,
    exp["optimal_lambda_rmse_range"][0],
    exp["optimal_lambda_rmse_range"][1],
)
```

### Determinism


```python
rng2 = np.random.default_rng(noise_cfg["seed"])
noise2 = rng2.normal(0, noise_cfg["correlator_noise_std"], n_tau)
g_noisy2 = g_exact + noise2
rho2 = tikhonov_solve(kernel, g_noisy2, lam_opt)
check_true("Reconstruction deterministic", np.array_equal(rho_recon, rho2))

# Results: {pass_count()}/{total_count()} checks passed
print_summary("Exp 021: Spectral Function Reconstruction")
```

## Visualization


```python
# Publication-grade summary chart for Exp 021
fig, ax = plt.subplots(figsize=(8, 4))

p, f_count, t = pass_count(), fail_count(), total_count()
ax.barh(['Pass', 'Fail'], [p, f_count], color=[PASS_COLOR, FAIL_COLOR])
ax.set_xlim(0, max(t * 1.15, 1))
ax.set_title('Exp 021: Spectral Reconstruction — Validation Results')
ax.set_xlabel('Check Count')
for i, v in enumerate([p, f_count]):
    if v > 0:
        ax.text(v + 0.3, i, str(v), va='center', fontweight='bold')

plt.tight_layout()
plt.savefig(f'/tmp/groundspring_exp021.png', dpi=150, bbox_inches='tight')
plt.show()
print(f'\nResult: {p}/{t} PASS, {f_count}/{t} FAIL')
```

## Provenance & Summary

| Field | Value |
|-------|-------|
| Experiment | 021 — Spectral Reconstruction |
| Domain | Lattice Qcd |
| Reference | Bazavov — Tikhonov regularization of lattice QCD correlators |
| Faculty | Bazavov (MSU) |
| Python baseline | `control/spectral_recon/spectral_recon.py` |
| Benchmark JSON | `control/spectral_recon/benchmark_spectral_recon.json` |
| Rust validator | `validate_*` binary (exit-code protocol) |
| Rust speedup | See benchmark comparison notebook |
| License | AGPL-3.0-or-later |

**Provenance chain**: Python baseline → Rust validation → barraCuda GPU → metalForge cross-substrate → primal IPC composition

See [primals.eco](https://primals.eco) for rendered lab notebooks.


