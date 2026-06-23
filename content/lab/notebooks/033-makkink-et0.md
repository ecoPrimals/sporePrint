+++
title = "Makkink (1957) Radiation-Based ET₀"
description = "Rendered from 033-makkink-et0.ipynb"
date = 2026-06-23
weight = 50

[extra]
domain = "Lab"
rendered_from = "033-makkink-et0.ipynb"
+++

<!-- Auto-generated from 033-makkink-et0.ipynb by spore-validate render-notebooks -->

# Makkink (1957) Radiation-Based ET₀

**Experiment 033** — Python control baseline aligned with primal `science.et0_makkink` and Rust `validate_makkink`.


## Theory

The Makkink method estimates reference evapotranspiration ET₀ from solar radiation and air temperature only (no wind or humidity). Coefficients follow **de Bruin (1987)** as commonly applied in the Netherlands (KNMI) and Northern Europe.

$$\mathrm{ET}_0 = C_1 \frac{\Delta}{\Delta + \gamma} \frac{R_s}{\lambda} + C_2$$

with $C_1 = 0.61$, $C_2 = -0.12$, $R_s$ incoming solar radiation (MJ m⁻² day⁻¹), $\lambda = 2.45$ MJ kg⁻¹, $\Delta$ the slope of the saturation vapour pressure curve, and $\gamma$ the psychrometric constant (both in kPa °C⁻¹).

**Citation:** Makkink GF (1957) *J Inst Water Eng* 11:277–288; de Bruin HAR (1987) *From Penman to Makkink*, TNO, The Hague.


```python
import json
import math
import sys
from pathlib import Path

def _find_repo_root() -> Path:
    p = Path.cwd().resolve()
    for _ in range(8):
        if (p / "control" / "makkink").is_dir():
            return p
        p = p.parent
    return Path('/home/eastgate/Development/ecoPrimals/springs/airSpring')

REPO = _find_repo_root()
BENCHMARK_PATH = REPO / "control" / "makkink" / "benchmark_makkink.json"

with open(BENCHMARK_PATH, encoding="utf-8") as f:
    benchmark = json.load(f)

print("Benchmark:", BENCHMARK_PATH)
print("Keys:", list(benchmark.keys()))
```

```python
C1 = 0.61
C2 = -0.12
LAMBDA = 2.45


def saturation_vapour_pressure(t):
    """e_s (kPa) at temperature t (°C). FAO-56 Eq. 11."""
    return 0.6108 * math.exp(17.27 * t / (t + 237.3))


def vapour_pressure_slope(t):
    """Δ (kPa/°C) — slope of sat. vapour pressure curve. FAO-56 Eq. 13."""
    es = saturation_vapour_pressure(t)
    return 4098.0 * es / (t + 237.3) ** 2


def atmospheric_pressure(elevation_m):
    """Atmospheric pressure (kPa) from elevation. FAO-56 Eq. 7."""
    return 101.3 * ((293.0 - 0.0065 * elevation_m) / 293.0) ** 5.26


def psychrometric_constant(pressure_kpa):
    """γ (kPa/°C). FAO-56 Eq. 8."""
    return 0.665e-3 * pressure_kpa


def makkink_et0(tmean, rs_mj, elevation_m):
    """Makkink ET₀ (mm/day) with de Bruin (1987) coefficients."""
    p = atmospheric_pressure(elevation_m)
    gamma = psychrometric_constant(p)
    delta = vapour_pressure_slope(tmean)
    et0 = C1 * (delta / (delta + gamma)) * (rs_mj / LAMBDA) + C2
    return max(0.0, et0)
```

```python
def validate_analytical(benchmark):
    checks = benchmark["validation_checks"]["analytical"]["test_cases"]
    passed = 0
    for tc in checks:
        computed = makkink_et0(tc["tmean"], tc["rs_mj"], tc["elevation_m"])
        expected = tc["expected_et0"]
        tol = tc["tolerance"]
        ok = abs(computed - expected) <= tol
        status = "PASS" if ok else "FAIL"
        print("  [%s] T=%s, Rs=%s, z=%s → ET₀=%.3f (expected %s, tol %s)" % (
            status, tc["tmean"], tc["rs_mj"], tc["elevation_m"],
            computed, expected, tol))
        if ok:
            passed += 1
    return passed, len(checks)


def validate_pm_cross(benchmark):
    checks = benchmark["validation_checks"]["pm_cross_comparison"]["test_cases"]
    passed = 0
    for tc in checks:
        computed = makkink_et0(tc["tmean"], tc["rs_mj"], tc["elevation_m"])
        ratio = computed / tc["approx_pm_et0"] if tc["approx_pm_et0"] > 0 else 0.0
        ok = tc["min_ratio"] <= ratio <= tc["max_ratio"]
        status = "PASS" if ok else "FAIL"
        print("  [%s] %s: Makkink=%.2f / PM≈%s = %.3f (range [%s, %s])" % (
            status, tc["label"], computed, tc["approx_pm_et0"],
            ratio, tc["min_ratio"], tc["max_ratio"]))
        if ok:
            passed += 1
    return passed, len(checks)


def validate_edge_cases(benchmark):
    checks = benchmark["validation_checks"]["edge_cases"]["test_cases"]
    passed = 0
    for tc in checks:
        computed = makkink_et0(tc["tmean"], tc["rs_mj"], tc["elevation_m"])
        check = tc["check"]
        if check == "non_negative":
            ok = computed >= 0.0
        elif check == "positive":
            ok = computed > 0.0
        elif check == "zero":
            ok = computed == 0.0
        else:
            ok = False
        status = "PASS" if ok else "FAIL"
        print("  [%s] %s: ET₀=%.4f" % (status, tc["label"], computed))
        if ok:
            passed += 1
    return passed, len(checks)


def validate_monotonicity(benchmark):
    checks = benchmark["validation_checks"]["monotonicity"]["test_cases"]
    passed = 0
    for tc in checks:
        if "base_rs" in tc:
            low = makkink_et0(tc["tmean"], tc["base_rs"], tc["elevation_m"])
            high = makkink_et0(tc["tmean"], tc["high_rs"], tc["elevation_m"])
        else:
            low = makkink_et0(tc["base_t"], tc["rs_mj"], tc["elevation_m"])
            high = makkink_et0(tc["high_t"], tc["rs_mj"], tc["elevation_m"])
        ok = high > low
        status = "PASS" if ok else "FAIL"
        print("  [%s] %s: %.3f < %.3f" % (status, tc["label"], low, high))
        if ok:
            passed += 1
    return passed, len(checks)


def validate_pyet_cross(benchmark):
    try:
        import pyet
        import pandas as pd
    except ImportError:
        print("  [SKIP] pyet not installed — skipping cross-validation")
        return 0, 0

    section = benchmark["validation_checks"]["pyet_cross_validation"]
    tol = section["tolerance"]
    conditions = section["test_conditions"]
    passed = 0
    for tc in conditions:
        our_et0 = makkink_et0(tc["tmean"], tc["rs_mj"], tc["elevation_m"])
        tmean_s = pd.Series([tc["tmean"]])
        rs_s = pd.Series([tc["rs_mj"]])
        try:
            pyet_val = float(
                pyet.makkink(tmean_s, rs_s, elevation=tc["elevation_m"]).iloc[0]
            )
        except Exception as e:
            print("  [SKIP] pyet.makkink failed:", e)
            continue
        diff = abs(our_et0 - pyet_val)
        ok = diff <= tol
        status = "PASS" if ok else "FAIL"
        print("  [%s] T=%s, Rs=%s: ours=%.3f, pyet=%.3f, diff=%.4f (tol %s)" % (
            status, tc["tmean"], tc["rs_mj"], our_et0, pyet_val, diff, tol))
        if ok:
            passed += 1
    return passed, len(conditions)


total_passed = 0
total_checks = 0

print("\n── Analytical Benchmarks ──")
p, t = validate_analytical(benchmark)
total_passed += p
total_checks += t

print("\n── PM Cross-Comparison ──")
p, t = validate_pm_cross(benchmark)
total_passed += p
total_checks += t

print("\n── Edge Cases ──")
p, t = validate_edge_cases(benchmark)
total_passed += p
total_checks += t

print("\n── Monotonicity ──")
p, t = validate_monotonicity(benchmark)
total_passed += p
total_checks += t

print("\n── pyet Cross-Validation ──")
p, t = validate_pyet_cross(benchmark)
total_passed += p
total_checks += t

print("\n=== Makkink ET₀: %s/%s PASS ===" % (total_passed, total_checks))
if total_passed != total_checks:
    sys.exit(1)
```

```python
import matplotlib.pyplot as plt
import numpy as np

# Analytical suite: computed vs expected
cases = benchmark["validation_checks"]["analytical"]["test_cases"]
xs = np.arange(len(cases))
computed = [makkink_et0(c["tmean"], c["rs_mj"], c["elevation_m"]) for c in cases]
expected = [c["expected_et0"] for c in cases]

fig, axes = plt.subplots(1, 2, figsize=(11, 4.2))

ax = axes[0]
width = 0.35
ax.bar(xs - width / 2, computed, width, label="Computed", color="#2ecc71", edgecolor="white")
ax.bar(xs + width / 2, expected, width, label="Benchmark", color="#3498db", edgecolor="white")
ax.set_xticks(xs)
ax.set_xticklabels([str(i + 1) for i in range(len(cases))])
ax.set_xlabel("Test case")
ax.set_ylabel("ET₀ (mm day⁻¹)")
ax.set_title("Makkink: benchmark analytical cases")
ax.legend()

ax2 = axes[1]
t_fix, z_fix = 20.0, 100.0
rs_grid = np.linspace(0, 30, 100)
et0_line = [makkink_et0(t_fix, float(r), z_fix) for r in rs_grid]
ax2.plot(rs_grid, et0_line, color="#e74c3c", lw=2, label="T=%g°C, z=%gm" % (t_fix, z_fix))
ax2.set_xlabel("Solar radiation $R_s$ (MJ m⁻² day⁻¹)")
ax2.set_ylabel("ET₀ (mm day⁻¹)")
ax2.set_title("Sensitivity to $R_s$ (de Bruin coefficients)")
ax2.legend()
ax2.grid(True, alpha=0.25)

plt.tight_layout()
plt.show()
```

## Summary and provenance

- **Primal:** `science.et0_makkink`
- **Rust:** `validate_makkink`
- **Control script:** `control/makkink/makkink_et0.py`
- **Benchmark:** `control/makkink/benchmark_makkink.json`
- **Equation:** ET₀ = 0.61 × (Δ/(Δ+γ)) × R_s/λ − 0.12 (same as script; mm day⁻¹).

Metadata in the benchmark file includes `_provenance` (baseline commit, command, references). Re-run the control script at the recorded commit to regenerate expected numerical targets.


