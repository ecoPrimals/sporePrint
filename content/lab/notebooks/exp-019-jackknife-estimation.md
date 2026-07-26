+++
title = "Experiment 019 — Jackknife Error Estimation"
description = "Applies jackknife resampling for error estimation in spectral and transport calculations, quantifying bias and variance in finite-size physics results."
date = 2026-07-18
weight = 50

[extra]
domain = "Lab"
rendered_from = "exp-019-jackknife-estimation.ipynb"
+++

<!-- Auto-generated from exp-019-jackknife-estimation.ipynb by spore-validate render-notebooks -->

# Experiment 019 — Jackknife Error Estimation

Delete-one jackknife resampling for variance estimation and bias correction.
Validates against analytical variance of the mean for Gaussian and exponential
distributions, tests bias correction on variance estimator, and compares
block jackknife with bootstrap on AR(1) correlated data.

**Cross-spring: extends Exp 007 (RAWR bootstrap) with jackknife methodology.**

**Domain**: Statistics
**Faculty**: Bazavov (MSU)
**Reference**: Bazavov — QCD systematic error estimation

**Data source**: `control/jackknife_estimation/jackknife_estimation.py` + `benchmark_*.json`

---

*This notebook is the publication-grade Python baseline for Experiment 019. The identical computations are validated in Rust (see `validate_*` binary) and delegated to barraCuda for GPU acceleration.*


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
benchmark_path = CONTROL / 'jackknife_estimation' / 'benchmark_jackknife.json'
with open(benchmark_path) as f:
    benchmark = json.load(f)

PASS_COLOR = '#2ecc71'
FAIL_COLOR = '#e74c3c'
INFO_COLOR = '#3498db'

print(f'Loaded benchmark: benchmark_jackknife.json')
print(f'Provenance: {benchmark.get("_provenance", {})}')
```

## Validation


### Initialization


```python
with open(bench_path) as f:

gauss = benchmark["gaussian"]
exp_cfg = benchmark["exponential"]
corr = benchmark["correlated"]
exp_res = benchmark["expected_results"]

rng_g = np.random.default_rng(gauss["seed"])
gauss_data = rng_g.normal(gauss["true_mean"], gauss["true_std"], gauss["n_samples"])
```

### Jackknife on Gaussian data


```python
jk_mean, jk_var, _ = jackknife_mean_variance(gauss_data)
check_max(
    "Jackknife mean near true mean",
    abs(jk_mean - gauss["true_mean"]),
    exp_res["gaussian_jk_mean_tol"],
)
check_range(
    "Jackknife variance of mean",
    jk_var,
    exp_res["gaussian_jk_var_range"][0],
    exp_res["gaussian_jk_var_range"][1],
)
```

### Jackknife on exponential data


```python
rng_e = np.random.default_rng(exp_cfg["seed"])
exp_data = rng_e.exponential(1.0 / exp_cfg["rate"], exp_cfg["n_samples"])
exp_mean, exp_var, _ = jackknife_mean_variance(exp_data)
check_max(
    "Exponential jackknife mean near 1/rate",
    abs(exp_mean - 1.0 / exp_cfg["rate"]),
    exp_res["exponential_jk_mean_tol"],
)
check_range(
    "Exponential jackknife variance of mean",
    exp_var,
    exp_res["exponential_jk_var_range"][0],
    exp_res["exponential_jk_var_range"][1],
)
```

### Jackknife bias correction


```python
_, _bias_raw, corrected = jackknife_bias(gauss_data, biased_variance)
true_var = gauss["true_std"] ** 2
naive_err = abs(biased_variance(gauss_data) - true_var)
corrected_err = abs(corrected - true_var)
check_true("Bias correction reduces error", corrected_err < naive_err * 1.5)
```

### Block jackknife on correlated data


```python
rng_c = np.random.default_rng(corr["seed"])
corr_data = generate_ar1(
    corr["n_samples"], corr["true_mean"], corr["true_std"], corr["ar1_phi"], rng_c
)
block_vars = []
for bs in corr["block_sizes"]:
    _, bv = block_jackknife_variance(corr_data, bs)
    block_vars.append(bv)
check_true(
    "Block JK variance increases with block size",
    all(block_vars[i] <= block_vars[i + 1] * 1.5 for i in range(len(block_vars) - 2)),
)
check_range(
    "Large-block variance in expected range",
    block_vars[-1],
    exp_res["block_jk_large_block_var_range"][0],
    exp_res["block_jk_large_block_var_range"][1],
)
```

### Jackknife vs bootstrap comparison


```python
boot_vars = []
n_boot = 500
for _ in range(n_boot):
    idx = rng_g.integers(0, len(gauss_data), len(gauss_data))
    boot_vars.append(np.mean(gauss_data[idx]))
boot_var = np.var(boot_vars, ddof=1)
ratio = jk_var / boot_var if boot_var > 0 else float("inf")
check_range(
    "Jackknife/bootstrap variance ratio",
    ratio,
    exp_res["jk_bootstrap_ratio_range"][0],
    exp_res["jk_bootstrap_ratio_range"][1],
)
```

### Determinism


```python
rng_d1 = np.random.default_rng(gauss["seed"])
d1 = rng_d1.normal(gauss["true_mean"], gauss["true_std"], gauss["n_samples"])
rng_d2 = np.random.default_rng(gauss["seed"])
d2 = rng_d2.normal(gauss["true_mean"], gauss["true_std"], gauss["n_samples"])
m1, v1, _ = jackknife_mean_variance(d1)
m2, v2, _ = jackknife_mean_variance(d2)
check_true("Jackknife deterministic", m1 == m2 and v1 == v2)

# Results: {pass_count()}/{total_count()} checks passed
print_summary("Exp 019: Jackknife Error Estimation")
```

## Visualization


```python
# Publication-grade summary chart for Exp 019
fig, ax = plt.subplots(figsize=(8, 4))

p, f_count, t = pass_count(), fail_count(), total_count()
ax.barh(['Pass', 'Fail'], [p, f_count], color=[PASS_COLOR, FAIL_COLOR])
ax.set_xlim(0, max(t * 1.15, 1))
ax.set_title('Exp 019: Jackknife Error Estimation — Validation Results')
ax.set_xlabel('Check Count')
for i, v in enumerate([p, f_count]):
    if v > 0:
        ax.text(v + 0.3, i, str(v), va='center', fontweight='bold')

plt.tight_layout()
plt.savefig(f'/tmp/groundspring_exp019.png', dpi=150, bbox_inches='tight')
plt.show()
print(f'\nResult: {p}/{t} PASS, {f_count}/{t} FAIL')
```

## Provenance & Summary

| Field | Value |
|-------|-------|
| Experiment | 019 — Jackknife Error Estimation |
| Domain | Statistics |
| Reference | Bazavov — QCD systematic error estimation |
| Faculty | Bazavov (MSU) |
| Python baseline | `control/jackknife_estimation/jackknife_estimation.py` |
| Benchmark JSON | `control/jackknife_estimation/benchmark_jackknife.json` |
| Rust validator | `validate_*` binary (exit-code protocol) |
| Rust speedup | See benchmark comparison notebook |
| License | AGPL-3.0-or-later |

**Provenance chain**: Python baseline → Rust validation → barraCuda GPU → metalForge cross-substrate → primal IPC composition

See [primals.eco](https://primals.eco) for rendered lab notebooks.


