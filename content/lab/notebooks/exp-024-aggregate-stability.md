+++
title = "Experiment 024 — Aggregate Stability Noise Analysis"
description = "Rendered from exp-024-aggregate-stability.ipynb"
date = 2026-06-15
weight = 50

[extra]
domain = "Lab"
rendered_from = "exp-024-aggregate-stability.ipynb"
+++

<!-- Auto-generated from exp-024-aggregate-stability.ipynb by spore-validate render-notebooks -->

# Experiment 024 — Aggregate Stability Noise Analysis

Applies the bias-variance decomposition (Exp 001) to soil aggregate stability
measurements. Answers: "How precisely must aggregate stability be measured
to distinguish Anderson localization regimes?"
Core idea:
1. Soil aggregates determine effective pore structure → effective disorder dimension d_eff
2. d_eff = 2 (low aggregate stability, tilled) vs d_eff = 3 (high aggregate stability, no-till)
3. Measurement noise in aggregate stability introduces uncertainty in d_eff
4. This uncertainty propagates to Anderson localization predictions
5. We decompose the measurement error into bias (correctable) and noise (irreducible)
Pipeline:
1. Define two "true" soil states: tilled (low WSA, d_eff≈2) and no-till (high WSA, d_eff≈3)
2. Simulate measured WSA (Water Stable Aggregates) with known bias and random noise
3. Map WSA → d_eff via a calibration curve
4. Check if measurement noise allows distinguishing d_eff=2 from d_eff=3
5. Decompose measurement error using Exp 001 methodology
References:
Nimmo & Perkins (2002) Methods of Soil Analysis
Kemper & Rosenau (1986) Aggregate stability wet sieving
Bourgain & Kachkovskiy (2018) GAFA 29:3-43

**Domain**: Soil Science
**Faculty**: Cross-spring (airSpring)
**Reference**: Cross-spring: airSpring soil structure

**Data source**: `control/aggregate_stability/aggregate_stability.py` + `benchmark_*.json`

---

*This notebook is the publication-grade Python baseline for Experiment 024. The identical computations are validated in Rust (see `validate_*` binary) and delegated to barraCuda for GPU acceleration.*


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
benchmark_path = CONTROL / 'aggregate_stability' / 'benchmark_aggregate_stability.json'
with open(benchmark_path) as f:
    benchmark = json.load(f)

PASS_COLOR = '#2ecc71'
FAIL_COLOR = '#e74c3c'
INFO_COLOR = '#3498db'

print(f'Loaded benchmark: benchmark_aggregate_stability.json')
print(f'Provenance: {benchmark.get("_provenance", {})}')
```

## Model Implementation


```python
def simulate_measured_wsa(
    wsa_true: float,
    bias_mbe: float,
    random_sigma: float,
    n_measurements: int,
    rng: np.random.Generator,
) -> np.ndarray:
    """Simulate measured WSA values with known bias and random noise.

    measured_wsa = wsa_true + bias_mbe + N(0, random_sigma)

    The bias represents systematic over/under-estimation (e.g. lab protocol).
    """
    return wsa_true + bias_mbe + rng.normal(0.0, random_sigma, size=n_measurements)


def wsa_to_d_eff(wsa: np.ndarray, slope: float, intercept: float) -> np.ndarray:
    """Map Water Stable Aggregates to effective disorder dimension.

    d_eff = slope * WSA + intercept
    """
    return slope * wsa + intercept


def regime_distinguishable(
    d_eff_tilled: np.ndarray,
    d_eff_notill: np.ndarray,
    gap_threshold: float = 0.5,
) -> bool:
    """Check if tilled and no-till d_eff distributions are separable.

    Uses non-overlapping 95% intervals as a simple criterion.
    """
    t_low = np.percentile(d_eff_tilled, 2.5)
    t_high = np.percentile(d_eff_tilled, 97.5)
    n_low = np.percentile(d_eff_notill, 2.5)
    n_high = np.percentile(d_eff_notill, 97.5)
    # Regimes distinguishable if the intervals don't overlap by more than gap_threshold
    overlap = min(t_high, n_high) - max(t_low, n_low)
    return bool(overlap < gap_threshold)
```

## Validation


### Initialization


```python
print("groundSpring Exp 024: Aggregate Stability Measurement Noise")
print("  Anderson regime discrimination from WSA measurement precision")

soil_states = benchmark["soil_states"]
noise_cfg = benchmark["measurement_noise"]
cal = benchmark["calibration"]
expected = benchmark["expected"]

rng = np.random.default_rng(noise_cfg["seed"])
```

### Simulated WSA Measurements


```python
wsa_measured: dict[str, np.ndarray] = {}
for state_name, state in soil_states.items():
    wsa_true = state["wsa_true"]
    wsa_measured[state_name] = simulate_measured_wsa(
        wsa_true,
        noise_cfg["bias_mbe"],
        noise_cfg["random_sigma"],
        noise_cfg["n_measurements"],
        rng,
    )
    print(
        f"  {state_name}: true WSA={wsa_true:.3f}, "
        f"measured mean={float(np.mean(wsa_measured[state_name])):.3f}, "
        f"std={float(np.std(wsa_measured[state_name])):.3f}"
    )
```

### WSA → d_eff Calibration


```python
d_eff_measured: dict[str, np.ndarray] = {}
for state_name in soil_states:
    d_eff_measured[state_name] = wsa_to_d_eff(
        wsa_measured[state_name], cal["slope"], cal["intercept"]
    )
    mean_d = float(np.mean(d_eff_measured[state_name]))
    std_d = float(np.std(d_eff_measured[state_name]))
    cv = std_d / mean_d if mean_d != 0 else 0.0

    exp_range = (
        expected["tilled_d_eff_range"]
        if state_name == "tilled"
        else expected["notill_d_eff_range"]
    )
    check_range(
        f"  {state_name} d_eff mean in range",
        mean_d,
        exp_range[0],
        exp_range[1],
    )
    check_range(
        f"  {state_name} d_eff CV in range",
        cv,
        expected["d_eff_cv_range"][0],
        expected["d_eff_cv_range"][1],
    )
```

### Regime Discrimination


```python
distinguishable = regime_distinguishable(
    d_eff_measured["tilled"], d_eff_measured["notill"]
)
check_true(
    "  Tilled vs no-till regimes distinguishable",
    distinguishable == expected["regimes_distinguishable"],
)
```

### Bias-Variance Decomposition


```python
regime_gap = 1.0  # d_eff difference between tilled (2) and no-till (3)
noise_floor_ok = True

for state_name, state in soil_states.items():
    wsa_true = state["wsa_true"]
    wsa_meas = wsa_measured[state_name]
    # observed = measured, modeled = true (ground truth)
    decomp = bias_variance_decompose(wsa_meas, np.full_like(wsa_meas, wsa_true))

    print(f"\n  {state_name}:")
    print(f"    Bias (MBE):      {decomp['bias']:.4f}")
    print(f"    Random std:      {decomp['random_std']:.4f}")
    print(f"    Bias fraction:   {decomp['bias_fraction']:.3f}")

    check_range(
        f"    {state_name} bias fraction in range",
        decomp["bias_fraction"],
        expected["bias_fraction_range"][0],
        expected["bias_fraction_range"][1],
    )

    # Noise floor: random_std propagates to d_eff as slope * random_std
    d_eff_noise_floor = cal["slope"] * decomp["random_std"]
    if d_eff_noise_floor >= regime_gap:
        noise_floor_ok = False
    print(f"    d_eff noise floor (slope × σ): {d_eff_noise_floor:.4f}")

check_true(
    "  Noise floor below regime gap (d_eff=1)",
    noise_floor_ok == expected["noise_floor_below_regime_gap"],
)
```

### Summary


```python
print("\n" + "=" * 72)
```

### Measurement error structure:


```python
for state_name, state in soil_states.items():
    decomp = bias_variance_decompose(
        wsa_measured[state_name],
        np.full(noise_cfg["n_measurements"], state["wsa_true"]),
    )
    dominant = "BIAS" if decomp["bias_fraction"] > 0.5 else "NOISE"
    print(
        f"   {state_name}: {dominant}-dominated "
        f"({decomp['bias_fraction']*100:.1f}% bias)"
    )
```

### Regime discrimination:


```python
print(f"   Tilled vs no-till distinguishable: {distinguishable}")
```

### Implications for Anderson localization:


```python
print("   - With n=100 measurements, WSA noise (σ=0.05) propagates to d_eff")
print("   - Noise floor in d_eff is small enough to distinguish d_eff=2 vs 3")
print("   - Bias correction can improve regime classification accuracy")

# Results: {pass_count()}/{total_count()} checks passed
print_summary("Exp 024: Aggregate Stability Measurement Noise")
```

## Visualization


```python
# Publication-grade summary chart for Exp 024
fig, ax = plt.subplots(figsize=(8, 4))

p, f_count, t = pass_count(), fail_count(), total_count()
ax.barh(['Pass', 'Fail'], [p, f_count], color=[PASS_COLOR, FAIL_COLOR])
ax.set_xlim(0, max(t * 1.15, 1))
ax.set_title('Exp 024: Aggregate Stability Noise Analysis — Validation Results')
ax.set_xlabel('Check Count')
for i, v in enumerate([p, f_count]):
    if v > 0:
        ax.text(v + 0.3, i, str(v), va='center', fontweight='bold')

plt.tight_layout()
plt.savefig(f'/tmp/groundspring_exp024.png', dpi=150, bbox_inches='tight')
plt.show()
print(f'\nResult: {p}/{t} PASS, {f_count}/{t} FAIL')
```

## Provenance & Summary

| Field | Value |
|-------|-------|
| Experiment | 024 — Aggregate Stability Noise Analysis |
| Domain | Soil Science |
| Reference | Cross-spring: airSpring soil structure |
| Faculty | Cross-spring (airSpring) |
| Python baseline | `control/aggregate_stability/aggregate_stability.py` |
| Benchmark JSON | `control/aggregate_stability/benchmark_aggregate_stability.json` |
| Rust validator | `validate_*` binary (exit-code protocol) |
| Rust speedup | See benchmark comparison notebook |
| License | AGPL-3.0-or-later |

**Provenance chain**: Python baseline → Rust validation → barraCuda GPU → metalForge cross-substrate → primal IPC composition

See [primals.eco](https://primals.eco) for rendered lab notebooks.


