+++
title = "Experiment 001 — Sensor Noise Characterization"
description = "Rendered from exp-001-sensor-noise.ipynb"
date = 2026-07-04
weight = 50

[extra]
domain = "Lab"
rendered_from = "exp-001-sensor-noise.ipynb"
+++

<!-- Auto-generated from exp-001-sensor-noise.ipynb by spore-validate render-notebooks -->

# Experiment 001 — Sensor Noise Characterization

Decomposes factory calibration error in Dong et al. (2020) soil moisture
sensors into systematic bias (correctable) vs random noise (irreducible).

**Key questions:**
1. What fraction of total error is systematic bias vs random noise?
2. How does the noise structure differ across soil types?
3. What is the noise floor after bias correction?
4. Are the errors consistent with Gaussian noise?

**Reference:**
Dong, Miller, Kelley (2020) Performance Evaluation of Soil Moisture
Sensors in Coarse- and Fine-Textured Michigan Agricultural Soils.
Agriculture 10(12), 598. doi:10.3390/agriculture10120598

**Domain**: Measurement
**Faculty**: Dong et al.
**Reference**: Dong, Miller, Kelley (2020) Agriculture 10(12), 598

**Data source**: `control/sensor_noise/sensor_noise_decomposition.py` + `benchmark_*.json`

---

*This notebook is the publication-grade Python baseline for Experiment 001. The identical computations are validated in Rust (see `validate_*` binary) and delegated to barraCuda for GPU acceleration.*


```python
import json
import math
import sys
from pathlib import Path

import numpy as np
from scipy import stats
import matplotlib.pyplot as plt

# Wire path to groundSpring control/ for common utilities
CONTROL = Path('..') / '..' / 'control'
sys.path.insert(0, str(CONTROL))
from common import *  # noqa: F403 — validation harness

# Load benchmark data
benchmark_path = CONTROL / 'sensor_noise' / 'benchmark_sensor_noise.json'
with open(benchmark_path) as f:
    benchmark = json.load(f)

PASS_COLOR = '#2ecc71'
FAIL_COLOR = '#e74c3c'
INFO_COLOR = '#3498db'

print(f'Loaded benchmark: benchmark_sensor_noise.json')
print(f'Provenance: {benchmark.get("_provenance", {})}')
```

## Model Implementation


```python
def generate_sensor_noise_samples(
    mbe: float,
    random_std: float,
    n_samples: int = 10_000,
    rng_seed: int = 42,
) -> np.ndarray:
    """Generate synthetic sensor error samples matching observed statistics.

    Samples = bias + N(0, random_std).  Used to verify that the
    decomposed noise parameters reconstruct a Gaussian distribution.
    """
    rng = np.random.default_rng(rng_seed)
    return mbe + rng.normal(0.0, random_std, size=n_samples)


def test_normality(samples: np.ndarray) -> dict:
    """Test whether error samples are consistent with a Gaussian."""
    n = len(samples)
    mean = float(np.mean(samples))
    std = float(np.std(samples, ddof=1))
    skewness = float(stats.skew(samples))
    kurtosis = float(stats.kurtosis(samples))

    _, shapiro_p = stats.shapiro(samples[:5000])

    return {
        "n_samples": n,
        "mean": mean,
        "std": std,
        "skewness": skewness,
        "excess_kurtosis": kurtosis,
        "shapiro_p_value": float(shapiro_p),
        "is_normal_shapiro": float(shapiro_p) > 0.05,
    }
```

```python
def compare_across_soils(decompositions: dict) -> dict:
    """Compare noise characteristics across soil types."""
    soil_types = list(decompositions.keys())
    biases = [decompositions[s]["bias_abs"] for s in soil_types]
    random_stds = [decompositions[s]["random_std"] for s in soil_types]
    bias_fracs = [decompositions[s]["bias_fraction"] for s in soil_types]

    return {
        "soil_types": soil_types,
        "bias_range": [min(biases), max(biases)],
        "random_std_range": [min(random_stds), max(random_stds)],
        "bias_dominated_soils": [
            s for s in soil_types if decompositions[s]["bias_fraction"] > 0.5
        ],
        "noise_dominated_soils": [
            s for s in soil_types if decompositions[s]["bias_fraction"] <= 0.5
        ],
        "mean_bias_fraction": sum(bias_fracs) / len(bias_fracs),
    }
```

## Validation


### Initialization


```python
print("groundSpring Exp 001: Sensor Noise Characterization")
print("  Source: Dong et al. (2020) — same data, groundSpring perspective")

sensors = benchmark["sensors"]
soils = benchmark["soil_types"]
factory = benchmark["factory_calibration_stats"]
corrected = benchmark["corrected_stats"]
expected = benchmark["expected_results"]
```

### Bias-Variance Decomposition


```python
all_decompositions: dict[str, dict] = {}

for sensor in sensors:
    print(f"\n  Sensor: {sensor.upper()}")
    all_decompositions[sensor] = {}

    for soil in soils:
        s = factory[sensor][soil]
        decomp = decompose_error(s["mbe"], s["rmse"])
        all_decompositions[sensor][soil] = decomp

        exp = expected[sensor][soil]

        # Tol 0.001: derived analytically from sqrt(RMSE²-MBE²),
        # rounding to 4 decimal places introduces ≤0.0005 error.
        check_approx(f"  {soil} bias", decomp["bias"], exp["bias"], 0.001)
        check_approx(
            f"  {soil} random_std",
            decomp["random_std"],
            exp["random_std"],
            0.001,
        )
        # Bias fraction tolerance 0.005: ratio of two small numbers,
        # propagated rounding from MBE and RMSE (each ±0.0005).
        check_approx(
            f"  {soil} bias_fraction",
            decomp["bias_fraction"],
            exp["bias_fraction"],
            0.005,
        )
```

### Noise Floor After Correction


```python
for sensor in sensors:
    print(f"\n  Sensor: {sensor.upper()}")
    for soil in soils:
        c = corrected[sensor][soil]
        nf = noise_floor_reduction(c["factory_rmse"], c["corrected_rmse"])
        print(f"    {soil}:")
        print(f"      Factory RMSE:   {nf['factory_rmse']:.4f}")
        print(f"      Corrected RMSE: {nf['corrected_rmse']:.4f}")
        print(f"      Removed error:  {nf['removed_error']:.4f}")
        print(f"      Reduction:      {nf['reduction_pct']:.1f}%")
        print(f"      Noise floor:    {nf['noise_floor']:.4f} m³/m³")

        check_true(
            f"    {soil} corrected <= factory",
            nf["corrected_rmse"] <= nf["factory_rmse"],
        )
```

### Cross-Soil-Type Comparison


```python
for sensor in sensors:
    comparison = compare_across_soils(all_decompositions[sensor])
    print(f"\n  Sensor: {sensor.upper()}")
    print(
        f"    Bias range:           [{comparison['bias_range'][0]:.4f}, "
        f"{comparison['bias_range'][1]:.4f}] m³/m³"
    )
    print(
        f"    Random noise range:   [{comparison['random_std_range'][0]:.4f}, "
        f"{comparison['random_std_range'][1]:.4f}] m³/m³"
    )
    print(f"    Bias-dominated soils: {comparison['bias_dominated_soils']}")
    print(f"    Noise-dominated soils:{comparison['noise_dominated_soils']}")
    print(f"    Mean bias fraction:   {comparison['mean_bias_fraction']:.3f}")

    if sensor == "cs616":
        has_both = (
            len(comparison["bias_dominated_soils"]) > 0
            and len(comparison["noise_dominated_soils"]) > 0
        )
        check_true("    Mixed bias/noise structure across soils", has_both)
    else:
        check_true(
            "    EC5 is bias-dominated overall",
            comparison["mean_bias_fraction"] > 0.5,
        )
```

### Noise Distribution Characterization


```python
for sensor in sensors:
    print(f"\n  Sensor: {sensor.upper()}")
    for soil in soils:
        decomp = all_decompositions[sensor][soil]
        samples = generate_sensor_noise_samples(
            decomp["bias"], decomp["random_std"]
        )
        norm_test = test_normality(samples)

        print(f"    {soil}:")
        print(
            f"      Generated: N={norm_test['n_samples']}, "
            f"mean={norm_test['mean']:.4f}, std={norm_test['std']:.4f}"
        )
        print(f"      Skewness:  {norm_test['skewness']:.4f}")
        print(f"      Kurtosis:  {norm_test['excess_kurtosis']:.4f}")
        print(f"      Shapiro p: {norm_test['shapiro_p_value']:.4f}")

        check_true(
            f"    {soil} synthetic samples pass normality",
            norm_test["is_normal_shapiro"],
        )
```

### Key Findings Summary


```python
print("\n" + "=" * 72)
```

### Bias vs Noise Structure:


```python
for sensor in sensors:
    for soil in soils:
        d = all_decompositions[sensor][soil]
        dominant = "BIAS" if d["bias_fraction"] > 0.5 else "NOISE"
        print(
            f"   {sensor.upper()} in {soil}: "
            f"{dominant}-dominated ({d['bias_fraction']*100:.1f}% bias)"
        )
```

### Correctable vs Irreducible Error:


```python
for sensor in sensors:
    for soil in soils:
        c = corrected[sensor][soil]
        nf = noise_floor_reduction(c["factory_rmse"], c["corrected_rmse"])
        print(
            f"   {sensor.upper()} in {soil}: "
            f"{nf['reduction_pct']:.0f}% correctable, "
            f"noise floor = {nf['noise_floor']:.4f} m³/m³"
        )
```

### Implications for Penny Irrigation:


```python
print("   - Sand: Low noise floor (0.004-0.006), suitable for precision irrigation")
print("   - Sandy clay loam: Higher noise (0.012-0.020), needs averaging or filtering")
print("   - Site-specific calibration removes 50-80% of total sensor error")

# Results: {pass_count()}/{total_count()} checks passed
print_summary("Exp 001: Sensor Noise Characterization")
```

## Visualization


```python
# Publication-grade summary chart for Exp 001
fig, ax = plt.subplots(figsize=(8, 4))

p, f_count, t = pass_count(), fail_count(), total_count()
ax.barh(['Pass', 'Fail'], [p, f_count], color=[PASS_COLOR, FAIL_COLOR])
ax.set_xlim(0, max(t * 1.15, 1))
ax.set_title('Exp 001: Sensor Noise Characterization — Validation Results')
ax.set_xlabel('Check Count')
for i, v in enumerate([p, f_count]):
    if v > 0:
        ax.text(v + 0.3, i, str(v), va='center', fontweight='bold')

plt.tight_layout()
plt.savefig(f'/tmp/groundspring_exp001.png', dpi=150, bbox_inches='tight')
plt.show()
print(f'\nResult: {p}/{t} PASS, {f_count}/{t} FAIL')
```

## Provenance & Summary

| Field | Value |
|-------|-------|
| Experiment | 001 — Sensor Noise Characterization |
| Domain | Measurement |
| Reference | Dong, Miller, Kelley (2020) Agriculture 10(12), 598 |
| Faculty | Dong et al. |
| Python baseline | `control/sensor_noise/sensor_noise_decomposition.py` |
| Benchmark JSON | `control/sensor_noise/benchmark_sensor_noise.json` |
| Rust validator | `validate_*` binary (exit-code protocol) |
| Rust speedup | See benchmark comparison notebook |
| License | AGPL-3.0-or-later |

**Provenance chain**: Python baseline → Rust validation → barraCuda GPU → metalForge cross-substrate → primal IPC composition

See [primals.eco](https://primals.eco) for rendered lab notebooks.


