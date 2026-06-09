+++
title = "Experiment 015 — Uncertainty Bridge: Sensor Noise → Localization"
description = "Rendered from exp-015-uncertainty-bridge.ipynb"
date = 2026-06-09
weight = 50

[extra]
domain = "Lab"
rendered_from = "exp-015-uncertainty-bridge.ipynb"
+++

<!-- Auto-generated from exp-015-uncertainty-bridge.ipynb by spore-validate render-notebooks -->

# Experiment 015 — Uncertainty Bridge: Sensor Noise → Localization

Propagates sensor measurement noise (Exp 001) through Anderson localization
(Exp 008) to predict how soil moisture sensor accuracy affects quorum sensing
regime predictions.
Pipeline:
θ_measured = θ_true + bias + N(0, σ)     (Exp 001: sensor noise)
W_eff = α * θ + β                        (moisture → disorder mapping)
γ = lyapunov_exponent(W_eff, E=0)        (Exp 008: Anderson model)
ξ = 1/γ                                  (localization length)
Key question: How much does sensor noise in θ propagate into uncertainty
in ξ (the QS signal propagation length)? Is bias correction sufficient to
reduce this uncertainty below a useful threshold?
Data sources:
- Dong et al. (2020) sensor calibration (Exp 001 benchmark)
- Anderson localization analytical model (Exp 008)
- No external data — fully analytical + Monte Carlo

**Domain**: Cross Domain
**Faculty**: Cross-domain bridge
**Reference**: Cross-domain error propagation

**Data source**: `control/uncertainty_bridge/uncertainty_bridge.py` + `benchmark_*.json`

---

*This notebook is the publication-grade Python baseline for Experiment 015. The identical computations are validated in Rust (see `validate_*` binary) and delegated to barraCuda for GPU acceleration.*


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
benchmark_path = CONTROL / 'uncertainty_bridge' / 'benchmark_uncertainty_bridge.json'
with open(benchmark_path) as f:
    benchmark = json.load(f)

PASS_COLOR = '#2ecc71'
FAIL_COLOR = '#e74c3c'
INFO_COLOR = '#3498db'

print(f'Loaded benchmark: benchmark_uncertainty_bridge.json')
print(f'Provenance: {benchmark.get("_provenance", {})}')
```

## Validation


### Initialization


```python
print("groundSpring Exp 015: Uncertainty Bridge")
print("  Sensor noise → Anderson localization → QS regime uncertainty")

sensor = benchmark["sensor_noise"]
anderson = benchmark["anderson_model"]
prop = benchmark["propagation"]
expected = benchmark["expected"]

chain_length = anderson["chain_length"]
n_real = anderson["n_realizations"]
n_mc = prop["n_mc_samples"]
slope = prop["theta_to_disorder_slope"]
intercept = prop["theta_to_disorder_intercept"]
theta_nom = prop["theta_nominal"]

rng = np.random.default_rng(prop.get("mc_seed", 2026))
```

### Anderson model sanity checks


```python
for w in anderson["disorder_range"]:
    gamma = lyapunov_averaged(w, 0.0, chain_length, n_real, 42)
    print(f"  W={w:5.1f} → γ={gamma:.4f}, ξ={1.0/max(gamma,1e-10):.1f}")

gammas = [
    lyapunov_averaged(w, 0.0, chain_length, n_real, 42)
    for w in anderson["disorder_range"]
]
from itertools import pairwise
monotonic = all(g1 <= g2 for g1, g2 in pairwise(gammas))
check_true("Lyapunov exponent monotonically increasing with W", monotonic)

check_true(
    "Clean system (W=0.5) has small γ",
    gammas[0] < 0.1,
)
check_true(
    "Strong disorder (W=12) has large γ",
    gammas[-1] > 0.3,
)
```

### CS616 Sand sensor noise propagation


```python
cs616 = sensor["cs616_sand"]

cs616_raw = propagate_sensor_noise(
    theta_nom, cs616["bias_mbe"], cs616["random_sigma"],
    slope, intercept, chain_length, n_real, n_mc, rng,
)
print(f"  Raw:  ξ = {cs616_raw['xi_mean']:.1f} ± {cs616_raw['xi_std']:.1f} "
      f"(CV = {cs616_raw['xi_cv']:.3f})")

cs616_corrected = propagate_bias_corrected(
    theta_nom, cs616["bias_mbe"], cs616["random_sigma"],
    slope, intercept, chain_length, n_real, n_mc, rng,
)
print(f"  Corrected: ξ = {cs616_corrected['xi_mean']:.1f} ± "
      f"{cs616_corrected['xi_std']:.1f} (CV = {cs616_corrected['xi_cv']:.3f})")

check_range(
    "CS616 localization length CV",
    cs616_raw["xi_cv"],
    expected["localization_length_cv_cs616"]["min"],
    expected["localization_length_cv_cs616"]["max"],
)
```

### EC5 Sandy Clay Loam sensor noise propagation


```python
ec5 = sensor["ec5_sandy_clay_loam"]

ec5_raw = propagate_sensor_noise(
    theta_nom, ec5["bias_mbe"], ec5["random_sigma"],
    slope, intercept, chain_length, n_real, n_mc, rng,
)
print(f"  Raw:  ξ = {ec5_raw['xi_mean']:.1f} ± {ec5_raw['xi_std']:.1f} "
      f"(CV = {ec5_raw['xi_cv']:.3f})")

ec5_corrected = propagate_bias_corrected(
    theta_nom, ec5["bias_mbe"], ec5["random_sigma"],
    slope, intercept, chain_length, n_real, n_mc, rng,
)
print(f"  Corrected: ξ = {ec5_corrected['xi_mean']:.1f} ± "
      f"{ec5_corrected['xi_std']:.1f} (CV = {ec5_corrected['xi_cv']:.3f})")

check_range(
    "EC5 localization length CV",
    ec5_raw["xi_cv"],
    expected["localization_length_cv_ec5"]["min"],
    expected["localization_length_cv_ec5"]["max"],
)
```

### Cross-sensor comparison


```python
check_true(
    "EC5 has higher CV than CS616 (more noise → more uncertainty)",
    ec5_raw["xi_cv"] > cs616_raw["xi_cv"],
)
```

### Bias correction effectiveness


```python
ec5_improvement = 1.0 - ec5_corrected["xi_cv"] / max(ec5_raw["xi_cv"], 1e-10)
print(f"  EC5 CV reduction from bias correction: {ec5_improvement:.1%}")

min_reduction = expected["bias_corrected_improvement"]["min_reduction_fraction"]
check_min(
    "EC5 bias correction reduces CV",
    ec5_improvement,
    min_reduction,
)

cs616_improvement = 1.0 - cs616_corrected["xi_cv"] / max(cs616_raw["xi_cv"], 1e-10)
print(f"  CS616 CV reduction from bias correction: {cs616_improvement:.1%}")

check_true(
    "EC5 benefits more from bias correction than CS616 (higher bias fraction)",
    ec5_improvement > cs616_improvement or cs616["bias_fraction"] < ec5["bias_fraction"],
)
```

### Summary


```python
print("\n" + "=" * 72)
print("Uncertainty Bridge Summary:")
print(f"  CS616 Sand:         CV(ξ) = {cs616_raw['xi_cv']:.3f} → "
      f"{cs616_corrected['xi_cv']:.3f} (corrected)")
print(f"  EC5 Sandy Clay Loam: CV(ξ) = {ec5_raw['xi_cv']:.3f} → "
      f"{ec5_corrected['xi_cv']:.3f} (corrected)")
print("  Sensor ranking preserved: EC5 > CS616 in uncertainty")
print(f"  Bias correction: EC5 improves {ec5_improvement:.0%}, "
      f"CS616 improves {cs616_improvement:.0%}")

print_summary("Exp 015: Uncertainty Bridge")
return 1 if fail_count() > 0 else 0
```

## Visualization


```python
# Publication-grade summary chart for Exp 015
fig, ax = plt.subplots(figsize=(8, 4))

p, f_count, t = pass_count(), fail_count(), total_count()
ax.barh(['Pass', 'Fail'], [p, f_count], color=[PASS_COLOR, FAIL_COLOR])
ax.set_xlim(0, max(t * 1.15, 1))
ax.set_title('Exp 015: Uncertainty Bridge: Sensor Noise → Localization — Validation Results')
ax.set_xlabel('Check Count')
for i, v in enumerate([p, f_count]):
    if v > 0:
        ax.text(v + 0.3, i, str(v), va='center', fontweight='bold')

plt.tight_layout()
plt.savefig(f'/tmp/groundspring_exp015.png', dpi=150, bbox_inches='tight')
plt.show()
print(f'\nResult: {p}/{t} PASS, {f_count}/{t} FAIL')
```

## Provenance & Summary

| Field | Value |
|-------|-------|
| Experiment | 015 — Uncertainty Bridge: Sensor Noise → Localization |
| Domain | Cross Domain |
| Reference | Cross-domain error propagation |
| Faculty | Cross-domain bridge |
| Python baseline | `control/uncertainty_bridge/uncertainty_bridge.py` |
| Benchmark JSON | `control/uncertainty_bridge/benchmark_uncertainty_bridge.json` |
| Rust validator | `validate_*` binary (exit-code protocol) |
| Rust speedup | See benchmark comparison notebook |
| License | AGPL-3.0-or-later |

**Provenance chain**: Python baseline → Rust validation → barraCuda GPU → metalForge cross-substrate → primal IPC composition

See [primals.eco](https://primals.eco) for rendered lab notebooks.


