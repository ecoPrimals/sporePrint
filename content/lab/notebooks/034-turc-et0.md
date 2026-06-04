+++
title = "Turc (1961) Temperature-Radiation ET₀"
description = "Rendered from 034-turc-et0.ipynb"
date = 2026-06-04
weight = 50

[extra]
domain = "Lab"
rendered_from = "034-turc-et0.ipynb"
+++

<!-- Auto-generated from 034-turc-et0.ipynb by spore-validate render-notebooks -->

# Turc (1961) Temperature-Radiation ET₀

**Experiment 034** — Python control baseline aligned with primal `science.et0_turc` and Rust `validate_turc`.


## Theory

Turc estimates ET₀ from mean daily temperature and solar radiation, with a **humidity correction** when mean relative humidity is below 50%.

For $RH \geq 50\%$:
$$\mathrm{ET}_0 = 0.013 \, \frac{T}{T+15} \, (23.8846\, R_s + 50)$$

For $RH < 50\%$, multiply by $\bigl(1 + (50 - RH)/70\bigr)$.

Here $T$ is mean temperature (°C), $R_s$ is solar radiation (MJ m⁻² day⁻¹), and 23.8846 converts MJ m⁻² day⁻¹ to cal cm⁻² day⁻¹.

**Citation:** Turc L (1961) *Annales Agronomiques* 12:13–49.


```python
import json
import math
import sys
from pathlib import Path

def _find_repo_root() -> Path:
    p = Path.cwd().resolve()
    for _ in range(8):
        if (p / "control" / "turc").is_dir():
            return p
        p = p.parent
    return Path('/home/eastgate/Development/ecoPrimals/springs/airSpring')

REPO = _find_repo_root()
BENCHMARK_PATH = REPO / "control" / "turc" / "benchmark_turc.json"

with open(BENCHMARK_PATH, encoding="utf-8") as f:
    benchmark = json.load(f)

print("Benchmark:", BENCHMARK_PATH)
```

```python
MJ_TO_CAL_CM2 = 23.8846


def turc_et0(tmean, rs_mj, rh):
    """Turc (1961) ET₀ (mm/day)."""
    if tmean + 15.0 == 0.0:
        return 0.0
    t_factor = tmean / (tmean + 15.0)
    if t_factor < 0.0:
        return 0.0
    rs_cal = MJ_TO_CAL_CM2 * rs_mj + 50.0
    et0 = 0.013 * t_factor * rs_cal
    if rh < 50.0:
        et0 *= 1.0 + (50.0 - rh) / 70.0
    return max(0.0, et0)
```

```python
def validate_analytical_high_rh(benchmark):
    checks = benchmark["validation_checks"]["analytical_high_rh"]["test_cases"]
    passed = 0
    for tc in checks:
        computed = turc_et0(tc["tmean"], tc["rs_mj"], tc["rh"])
        expected = tc["expected_et0"]
        tol = tc["tolerance"]
        ok = abs(computed - expected) <= tol
        status = "PASS" if ok else "FAIL"
        print("  [%s] T=%s, Rs=%s, RH=%s%% → ET₀=%.3f (expected %s, tol %s)" % (
            status, tc["tmean"], tc["rs_mj"], tc["rh"], computed, expected, tol))
        if ok:
            passed += 1
    return passed, len(checks)


def validate_analytical_low_rh(benchmark):
    checks = benchmark["validation_checks"]["analytical_low_rh"]["test_cases"]
    passed = 0
    for tc in checks:
        computed = turc_et0(tc["tmean"], tc["rs_mj"], tc["rh"])
        expected = tc["expected_et0"]
        tol = tc["tolerance"]
        ok = abs(computed - expected) <= tol
        status = "PASS" if ok else "FAIL"
        print("  [%s] T=%s, Rs=%s, RH=%s%% → ET₀=%.3f (expected %s, tol %s)" % (
            status, tc["tmean"], tc["rs_mj"], tc["rh"], computed, expected, tol))
        if ok:
            passed += 1
    return passed, len(checks)


def validate_humidity_boundary(benchmark):
    checks = benchmark["validation_checks"]["humidity_boundary"]["test_cases"]
    passed = 0
    for tc in checks:
        tmean = tc["tmean"]
        rs_mj = tc["rs_mj"]
        tol = tc["tolerance"]
        et0_at_50 = turc_et0(tmean, rs_mj, 50.0)
        et0_at_49 = turc_et0(tmean, rs_mj, 49.99)
        diff = abs(et0_at_50 - et0_at_49)
        ok = diff < tol
        status = "PASS" if ok else "FAIL"
        print("  [%s] RH=50: %.4f, RH=49.99: %.4f, diff=%.6f (tol %s)" % (
            status, et0_at_50, et0_at_49, diff, tol))
        if ok:
            passed += 1
    return passed, len(checks)


def validate_edge_cases(benchmark):
    checks = benchmark["validation_checks"]["edge_cases"]["test_cases"]
    passed = 0
    for tc in checks:
        computed = turc_et0(tc["tmean"], tc["rs_mj"], tc["rh"])
        check = tc["check"]
        if check == "positive":
            ok = computed > 0.0
        elif check == "non_negative":
            ok = computed >= 0.0
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
        label = tc["label"]
        if "base_rs" in tc:
            low = turc_et0(tc["tmean"], tc["base_rs"], tc["rh"])
            high = turc_et0(tc["tmean"], tc["high_rs"], tc["rh"])
        elif "base_t" in tc:
            low = turc_et0(tc["base_t"], tc["rs_mj"], tc["rh"])
            high = turc_et0(tc["high_t"], tc["rs_mj"], tc["rh"])
        else:
            low = turc_et0(tc["tmean"], tc["rs_mj"], tc["base_rh"])
            high = turc_et0(tc["tmean"], tc["rs_mj"], tc["low_rh"])
        ok = high > low
        status = "PASS" if ok else "FAIL"
        print("  [%s] %s: %.3f < %.3f" % (status, label, low, high))
        if ok:
            passed += 1
    return passed, len(checks)


def validate_pyet_cross():
    try:
        import pyet
        import pandas as pd
    except ImportError:
        print("  [SKIP] pyet not installed — skipping cross-validation")
        return 0, 0

    conditions = [
        (20.0, 15.0, 70.0),
        (30.0, 25.0, 55.0),
        (10.0, 8.0, 80.0),
        (30.0, 25.0, 40.0),
        (25.0, 20.0, 20.0),
    ]
    tol = 0.1
    passed = 0
    for tmean, rs, rh in conditions:
        our = turc_et0(tmean, rs, rh)
        try:
            pyet_val = float(pyet.turc(
                pd.Series([tmean]), pd.Series([rs]), pd.Series([rh])
            ).iloc[0])
        except Exception as e:
            print("  [SKIP] pyet.turc failed:", e)
            continue
        diff = abs(our - pyet_val)
        ok = diff <= tol
        status = "PASS" if ok else "FAIL"
        print("  [%s] T=%s, Rs=%s, RH=%s: ours=%.3f, pyet=%.3f, diff=%.4f" % (
            status, tmean, rs, rh, our, pyet_val, diff))
        if ok:
            passed += 1
    return passed, len(conditions)


total_passed = 0
total_checks = 0

print("\n── Analytical (RH ≥ 50%) ──")
p, t = validate_analytical_high_rh(benchmark)
total_passed += p
total_checks += t

print("\n── Analytical (RH < 50%) ──")
p, t = validate_analytical_low_rh(benchmark)
total_passed += p
total_checks += t

print("\n── Humidity Boundary (RH=50%) ──")
p, t = validate_humidity_boundary(benchmark)
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
p, t = validate_pyet_cross()
total_passed += p
total_checks += t

print("\n=== Turc ET₀: %s/%s PASS ===" % (total_passed, total_checks))
if total_passed != total_checks:
    sys.exit(1)
```

```python
import matplotlib.pyplot as plt
import numpy as np

# Compare high-RH analytical cases
cases = benchmark["validation_checks"]["analytical_high_rh"]["test_cases"]
xs = np.arange(len(cases))
comp = [turc_et0(c["tmean"], c["rs_mj"], c["rh"]) for c in cases]
exp = [c["expected_et0"] for c in cases]

rh_vals = np.linspace(10, 90, 50)
et0_rh = [turc_et0(28.0, 22.0, float(rh)) for rh in rh_vals]

fig, axes = plt.subplots(1, 2, figsize=(11, 4.2))

ax = axes[0]
w = 0.35
ax.bar(xs - w / 2, comp, w, label="Computed", color="#2ecc71", edgecolor="white")
ax.bar(xs + w / 2, exp, w, label="Benchmark", color="#3498db", edgecolor="white")
ax.set_xticks(xs)
ax.set_xticklabels([str(i + 1) for i in range(len(cases))])
ax.set_title("Turc analytical (RH ≥ 50%)")
ax.set_xlabel("Test case")
ax.set_ylabel("ET₀ (mm day⁻¹)")
ax.legend()

ax2 = axes[1]
ax2.plot(rh_vals, et0_rh, color="#e74c3c", lw=2, label="T=28°C, Rs=22 MJ m⁻² d⁻¹")
ax2.axvline(50, color="0.4", ls="--", lw=1, label="RH = 50%")
ax2.set_xlabel("Relative humidity (%)")
ax2.set_ylabel("ET₀ (mm day⁻¹)")
ax2.set_title("Humidity correction branch")
ax2.legend()
ax2.grid(True, alpha=0.25)

plt.tight_layout()
plt.show()
```

## Summary and provenance

- **Primal:** `science.et0_turc`
- **Rust:** `validate_turc`
- **Control script:** `control/turc/turc_et0.py`
- **Benchmark:** `control/turc/benchmark_turc.json`

Turc coefficients and humidity split follow the project's Python baseline. Optional `pyet` checks require that package.


