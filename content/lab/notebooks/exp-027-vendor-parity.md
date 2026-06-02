+++
title = "Experiment 027 — GPU Vendor Parity"
description = "Rendered from exp-027-vendor-parity.ipynb"
date = 2026-06-02
weight = 50

[extra]
domain = "Lab"
rendered_from = "exp-027-vendor-parity.ipynb"
+++

<!-- Auto-generated from exp-027-vendor-parity.ipynb by spore-validate render-notebooks -->

# Experiment 027 — GPU Vendor Parity

Validates the methodology for verifying GPU vendor parity — that different
GPU implementations produce statistically indistinguishable WDM transport
coefficient results.
Two simulated vendors compute diffusion D from synthetic VACFs:
- Vendor A: Green-Kubo integration of VACF with seed_a noise
- Vendor B: Same VACF + tiny epsilon perturbation (FP implementation differences)

**Domain**: Gpu Validation
**Faculty**: WDM
**Reference**: WDM cross-vendor GPU comparison

**Data source**: `control/vendor_parity/vendor_parity.py` + `benchmark_*.json`

---

*This notebook is the publication-grade Python baseline for Experiment 027. The identical computations are validated in Rust (see `validate_*` binary) and delegated to barraCuda for GPU acceleration.*


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
benchmark_path = CONTROL / 'vendor_parity' / 'benchmark_vendor_parity.json'
with open(benchmark_path) as f:
    benchmark = json.load(f)

PASS_COLOR = '#2ecc71'
FAIL_COLOR = '#e74c3c'
INFO_COLOR = '#3498db'

print(f'Loaded benchmark: benchmark_vendor_parity.json')
print(f'Provenance: {benchmark.get("_provenance", {})}')
```

## Validation


### Initialization


```python
print("groundSpring Exp 027: GPU Vendor Parity for WDM Observables")
print("  Green-Kubo transport coefficient parity across simulated vendors")

model = benchmark["model"]
exp = benchmark["expected_results"]

c0 = float(model["c0"])
dt = float(model["dt"])
d_dim = float(model["d_dim"])
n_steps = int(model["n_steps"])
n_observables = int(model["n_observables"])
tau_min = float(model["tau_min"])
tau_max = float(model["tau_max"])
noise_amplitude = float(model["noise_amplitude"])
epsilon = float(model["epsilon"])
seed_a = int(model["seed_a"])
seed_b = int(model["seed_b"])

rng_a = np.random.default_rng(seed_a)
rng_b = np.random.default_rng(seed_b)

d_a_list: list[float] = []
d_b_list: list[float] = []

denom = max(n_observables - 1, 1)
for i in range(n_observables):
    tau = tau_min + (tau_max - tau_min) * i / denom
    vacf_a = synthetic_vacf_noisy(c0, tau, n_steps, dt, noise_amplitude, rng_a)
    integral_a = green_kubo_integrate(vacf_a, dt)
    d_a = integral_a / d_dim

    vendor_b_vacf = vacf_a + epsilon * rng_b.standard_normal(len(vacf_a))
    integral_b = green_kubo_integrate(vendor_b_vacf, dt)
    d_b = integral_b / d_dim

    d_a_list.append(d_a)
    d_b_list.append(d_b)

d_a_arr = np.array(d_a_list)
d_b_arr = np.array(d_b_list)

# Relative differences: |D_A - D_B| / |D_A|
d_a_safe = np.where(np.abs(d_a_arr) > 1e-20, d_a_arr, 1e-20)
rel_diffs = np.abs(d_a_arr - d_b_arr) / np.abs(d_a_safe)
max_rel_diff = float(np.max(rel_diffs))
mean_rel_diff = float(np.mean(rel_diffs))
```

### Pearson correlation between D_A and D_B


```python
if np.std(d_a_arr) > 0 and np.std(d_b_arr) > 0:
    correlation = float(np.corrcoef(d_a_arr, d_b_arr)[0, 1])
else:
    correlation = 1.0
```

### Bias-variance decomposition of D_B - D_A


```python
diff = d_b_arr - d_a_arr
mbe = float(np.mean(diff))
rmse = float(np.sqrt(np.mean(diff**2)))
decomp = decompose_error(mbe, rmse)
bias_fraction = decomp["bias_fraction"]
```

### Max absolute difference


```python
max_abs_diff = float(np.max(np.abs(diff)))

# All within tolerance (relative diff < max_relative_difference for each)
tol = exp["max_relative_difference"]
all_within = bool(np.all(rel_diffs <= tol))

# Chi-squared per DOF: sum((D_A - D_B)^2 / max(D_A^2, 1e-20)) / n_observables
chi2_terms = (d_a_arr - d_b_arr) ** 2 / np.maximum(d_a_arr**2, 1e-20)
chi2_per_dof = float(np.sum(chi2_terms) / n_observables)

print(f"\n  Max relative diff: {max_rel_diff:.2e}, mean: {mean_rel_diff:.2e}")
print(f"  Vendor correlation: {correlation:.8f}")
print(f"  Bias fraction: {bias_fraction:.6f}, max abs diff: {max_abs_diff:.2e}")
print(f"  Chi² per DOF: {chi2_per_dof:.6f}, all within tol: {all_within}")
```

### Validation Checks


```python
check_max("Max relative difference bounded", max_rel_diff, exp["max_relative_difference"])
check_max("Mean relative difference bounded", mean_rel_diff, exp["mean_relative_difference_max"])
check_min("Vendor correlation above minimum", correlation, exp["vendor_correlation_min"])
check_max("Bias fraction below maximum", bias_fraction, exp["bias_fraction_max"])
check_max("Max absolute difference bounded", max_abs_diff, exp["max_absolute_difference"])
check_true("All observables within tolerance", all_within)
check_max("Chi-squared per DOF bounded", chi2_per_dof, exp["chi2_per_dof_max"])

# Results: {pass_count()}/{total_count()} checks passed
print_summary("Exp 027: GPU Vendor Parity for WDM Observables")
```

## Visualization


```python
# Publication-grade summary chart for Exp 027
fig, ax = plt.subplots(figsize=(8, 4))

p, f_count, t = pass_count(), fail_count(), total_count()
ax.barh(['Pass', 'Fail'], [p, f_count], color=[PASS_COLOR, FAIL_COLOR])
ax.set_xlim(0, max(t * 1.15, 1))
ax.set_title('Exp 027: GPU Vendor Parity — Validation Results')
ax.set_xlabel('Check Count')
for i, v in enumerate([p, f_count]):
    if v > 0:
        ax.text(v + 0.3, i, str(v), va='center', fontweight='bold')

plt.tight_layout()
plt.savefig(f'/tmp/groundspring_exp027.png', dpi=150, bbox_inches='tight')
plt.show()
print(f'\nResult: {p}/{t} PASS, {f_count}/{t} FAIL')
```

## Provenance & Summary

| Field | Value |
|-------|-------|
| Experiment | 027 — GPU Vendor Parity |
| Domain | Gpu Validation |
| Reference | WDM cross-vendor GPU comparison |
| Faculty | WDM |
| Python baseline | `control/vendor_parity/vendor_parity.py` |
| Benchmark JSON | `control/vendor_parity/benchmark_vendor_parity.json` |
| Rust validator | `validate_*` binary (exit-code protocol) |
| Rust speedup | See benchmark comparison notebook |
| License | AGPL-3.0-or-later |

**Provenance chain**: Python baseline → Rust validation → barraCuda GPU → metalForge cross-substrate → primal IPC composition

See [primals.eco](https://primals.eco) for rendered lab notebooks.


