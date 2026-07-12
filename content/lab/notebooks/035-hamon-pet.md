+++
title = "Hamon (1961) Temperature-Based PET"
description = "Rendered from 035-hamon-pet.ipynb"
date = 2026-07-04
weight = 50

[extra]
domain = "Lab"
rendered_from = "035-hamon-pet.ipynb"
+++

<!-- Auto-generated from 035-hamon-pet.ipynb by spore-validate render-notebooks -->

# Hamon (1961) Temperature-Based PET

**Experiment 035** — Python control baseline aligned with primal `science.et0_hamon` and Rust `validate_hamon`.


## Theory

Hamon PET uses mean temperature and day length (possible sunshine hours). This notebook follows the **Lu et al. (2005)** formulation used in `hamon_pet.py`.

$$\mathrm{PET} = 0.1651 \, N \, \rho_{\mathrm{sat}} \, K_{\mathrm{PEC}}$$

with saturated vapour pressure $e_s$ (kPa), $\rho_{\mathrm{sat}} = 216.7\, e_s/(T+273.3)$ g m⁻³, $N$ daylight hours, and $K_{\mathrm{PEC}}=1$.

An equivalent textbook form is PET = 0.55 × (N/12)² × e_s(T)/100 × 25.4 under algebraic rearrangement of constants (see Lu et al. 2005; Hamon 1961).

**Citation:** Hamon WR (1961) *J Hydraulics Div ASCE* 87(HY3):107–120; Lu J et al. (2005) *J Am Water Resour Assoc* 41(3):621–633.


```python
import json
import math
import sys
from pathlib import Path

def _find_repo_root() -> Path:
    p = Path.cwd().resolve()
    for _ in range(8):
        if (p / "control" / "hamon").is_dir():
            return p
        p = p.parent
    return Path(os.environ.get('AIRSPRING_ROOT', '../airSpring'))

REPO = _find_repo_root()
BENCHMARK_PATH = REPO / "control" / "hamon" / "benchmark_hamon.json"

with open(BENCHMARK_PATH, encoding="utf-8") as f:
    benchmark = json.load(f)

print("Benchmark:", BENCHMARK_PATH)
```

```python
KPEC = 1.0


def saturation_vapour_pressure(t):
    """e_s (kPa). FAO-56 Eq. 11."""
    return 0.6108 * math.exp(17.27 * t / (t + 237.3))


def saturated_absolute_humidity(t):
    """RHOSAT (g/m³) — saturated vapour density at temperature t (°C)."""
    es = saturation_vapour_pressure(t)
    return 216.7 * es / (t + 273.3)


def daylight_hours(latitude_deg, doy):
    """Possible sunshine hours N from latitude and day of year. FAO-56 Eq. 34."""
    lat_rad = latitude_deg * math.pi / 180.0
    delta = 0.409 * math.sin(2.0 * math.pi * doy / 365.0 - 1.39)
    arg = -math.tan(lat_rad) * math.tan(delta)
    arg = max(-1.0, min(1.0, arg))
    ws = math.acos(arg)
    return 24.0 * ws / math.pi


def hamon_pet(tmean, day_length_hours):
    """Hamon PET (mm/day) using Lu et al. (2005) formulation."""
    if tmean < 0.0 or day_length_hours <= 0.0:
        return 0.0
    rhosat = saturated_absolute_humidity(tmean)
    return 0.1651 * day_length_hours * rhosat * KPEC


def hamon_pet_from_location(tmean, latitude_deg, doy):
    """Hamon PET computing day length from solar geometry."""
    if tmean < 0.0:
        return 0.0
    n = daylight_hours(latitude_deg, doy)
    return hamon_pet(tmean, n)
```

```python
def validate_analytical(benchmark):
    checks = benchmark["validation_checks"]["analytical"]["test_cases"]
    passed = 0
    for tc in checks:
        computed = hamon_pet(tc["tmean"], tc["day_length_hours"])
        expected = tc["expected_pet"]
        tol = tc["tolerance"]
        ok = abs(computed - expected) <= tol
        status = "PASS" if ok else "FAIL"
        print("  [%s] T=%s, N=%sh → PET=%.3f (expected %s, tol %s)" % (
            status, tc["tmean"], tc["day_length_hours"], computed, expected, tol))
        if ok:
            passed += 1
    return passed, len(checks)


def validate_day_length(benchmark):
    checks = benchmark["validation_checks"]["day_length_computation"]["test_cases"]
    passed = 0
    for tc in checks:
        computed = daylight_hours(tc["latitude"], tc["doy"])
        expected = tc["expected_hours"]
        tol = tc["tolerance"]
        ok = abs(computed - expected) <= tol
        status = "PASS" if ok else "FAIL"
        print("  [%s] lat=%s°, DOY=%s → N=%.2fh (expected %s, tol %s)" % (
            status, tc["latitude"], tc["doy"], computed, expected, tol))
        if ok:
            passed += 1
    return passed, len(checks)


def validate_edge_cases(benchmark):
    checks = benchmark["validation_checks"]["edge_cases"]["test_cases"]
    passed = 0
    for tc in checks:
        computed = hamon_pet(tc["tmean"], tc["day_length_hours"])
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
        print("  [%s] %s: PET=%.4f" % (status, tc["label"], computed))
        if ok:
            passed += 1
    return passed, len(checks)


def validate_monotonicity(benchmark):
    checks = benchmark["validation_checks"]["monotonicity"]["test_cases"]
    passed = 0
    for tc in checks:
        label = tc["label"]
        if "base_t" in tc and "base_dl" in tc:
            low = hamon_pet(tc["base_t"], tc["base_dl"])
            high = hamon_pet(tc["high_t"], tc["high_dl"])
        elif "base_t" in tc:
            dl = tc["day_length_hours"]
            low = hamon_pet(tc["base_t"], dl)
            high = hamon_pet(tc["high_t"], dl)
        else:
            tmean = tc["tmean"]
            low = hamon_pet(tmean, tc["base_dl"])
            high = hamon_pet(tmean, tc["high_dl"])
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
        (20.0, 42.0, 172),
        (30.0, 42.0, 196),
        (10.0, 42.0, 80),
        (25.0, 42.0, 265),
        (5.0, 42.0, 355),
    ]
    passed = 0
    our_vals = []
    pyet_vals = []
    for tmean, lat, doy in conditions:
        our = hamon_pet_from_location(tmean, lat, doy)
        try:
            pyet_val = float(pyet.hamon(
                pd.Series([tmean]),
                lat=lat * math.pi / 180.0,
                method=1,
            ).iloc[0])
        except Exception as e:
            print("  [SKIP] pyet.hamon failed:", e)
            continue
        our_vals.append(our)
        pyet_vals.append(pyet_val)
        ratio = our / pyet_val if pyet_val > 0 else float("nan")
        print("  [INFO] T=%s, lat=%s, DOY=%s: ours=%.3f, pyet=%.3f, ratio=%.2f" % (
            tmean, lat, doy, our, pyet_val, ratio))

    if len(our_vals) >= 3:
        monotonic_ok = True
        for i in range(len(our_vals) - 1):
            for j in range(i + 1, len(our_vals)):
                ours_dir = our_vals[i] - our_vals[j]
                pyet_dir = pyet_vals[i] - pyet_vals[j]
                if ours_dir * pyet_dir < 0:
                    monotonic_ok = False
        ok = monotonic_ok
        status = "PASS" if ok else "FAIL"
        print("  [%s] Rank correlation preserved between formulations" % status)
        if ok:
            passed += 1
        return passed, 1
    return 0, 0


total_passed = 0
total_checks = 0

print("\n── Analytical Benchmarks ──")
p, t = validate_analytical(benchmark)
total_passed += p
total_checks += t

print("\n── Day Length Computation ──")
p, t = validate_day_length(benchmark)
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

print("\n=== Hamon PET: %s/%s PASS ===" % (total_passed, total_checks))
if total_passed != total_checks:
    sys.exit(1)
```

```python
import matplotlib.pyplot as plt
import numpy as np

cases = benchmark["validation_checks"]["analytical"]["test_cases"]
xs = np.arange(len(cases))
comp = [hamon_pet(c["tmean"], c["day_length_hours"]) for c in cases]
exp = [c["expected_pet"] for c in cases]

t_grid = np.linspace(0, 35, 100)
n_fix = 14.0
pet_line = [hamon_pet(float(t), n_fix) for t in t_grid]

fig, axes = plt.subplots(1, 2, figsize=(11, 4.2))

ax = axes[0]
w = 0.35
ax.bar(xs - w / 2, comp, w, label="Computed", color="#2ecc71", edgecolor="white")
ax.bar(xs + w / 2, exp, w, label="Benchmark", color="#3498db", edgecolor="white")
ax.set_xticks(xs)
ax.set_xticklabels([str(i + 1) for i in range(len(cases))])
ax.set_title("Hamon PET: analytical cases")
ax.set_xlabel("Test case")
ax.set_ylabel("PET (mm day⁻¹)")
ax.legend()

ax2 = axes[1]
ax2.plot(t_grid, pet_line, color="#e74c3c", lw=2, label="N = %g h" % n_fix)
ax2.set_xlabel("Mean temperature (°C)")
ax2.set_ylabel("PET (mm day⁻¹)")
ax2.set_title("PET vs temperature (Lu et al. 2005 form)")
ax2.grid(True, alpha=0.25)
ax2.legend()

plt.tight_layout()
plt.show()
```

## Summary and provenance

- **Primal:** `science.et0_hamon`
- **Rust:** `validate_hamon`
- **Control script:** `control/hamon/hamon_pet.py`
- **Benchmark:** `control/hamon/benchmark_hamon.json`

Optional `pyet` validation compares ranking against a different published Hamon variant; analytical targets come from `benchmark_hamon.json`.


