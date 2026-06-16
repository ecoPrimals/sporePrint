+++
title = "Priestley-Taylor (1972) Radiation-Based ET₀"
description = "Rendered from 019-priestley-taylor-et0.ipynb"
date = 2026-06-16
weight = 50

[extra]
domain = "Lab"
rendered_from = "019-priestley-taylor-et0.ipynb"
+++

<!-- Auto-generated from 019-priestley-taylor-et0.ipynb by spore-validate render-notebooks -->

# Priestley-Taylor (1972) Radiation-Based ET₀

**Citation:** Priestley CHB, Taylor RJ (1972). *Monthly Weather Review* 100(2): 81-92.

**Abstract.** Priestley–Taylor estimates reference evapotranspiration from available energy and temperature-dependent thermodynamic factors, using α = 1.26 for well-watered surfaces. This notebook implements the radiation pathway, cross-checks analytical cases and FAO-56 Example 18 (Uccle) against `benchmark_priestley_taylor.json`, and visualizes gradients and sensitivities used in ET₀ intercomparisons.

**For other springs:** Primal capability `science.et0_priestley_taylor`; Rust binary `validate_priestley_taylor`.


## Theory

$$\mathrm{ET}_{0,\mathrm{PT}} = \alpha \,\frac{\Delta}{\Delta + \gamma}\,\frac{R_n - G}{\lambda},\quad \alpha = 1.26$$

Reference implementation (FAO-style mm day⁻¹ conversion):

$$\mathrm{ET}_{0,\mathrm{PT}} = \max\left(0,\;\alpha \cdot 0.408 \cdot \frac{\Delta}{\Delta + \gamma}\cdot (R_n - G)\right)$$


```python
import json
from pathlib import Path

import matplotlib.pyplot as plt
import numpy as np

BENCH_PATH = (Path.cwd() / "../../control/priestley_taylor/benchmark_priestley_taylor.json").resolve()
with open(BENCH_PATH, encoding="utf-8") as f:
    bench_pt = json.load(f)
print("Loaded:", BENCH_PATH)
```

```python
import math

# From control/priestley_taylor/priestley_taylor_et0.py

ALPHA_PT = 1.26


def saturation_vapour_pressure(temp_c):
    return 0.6108 * math.exp(17.27 * temp_c / (temp_c + 237.3))


def vapour_pressure_slope(temp_c):
    return 4098.0 * saturation_vapour_pressure(temp_c) / (temp_c + 237.3) ** 2


def atmospheric_pressure(elevation_m):
    return 101.3 * ((293.0 - 0.0065 * elevation_m) / 293.0) ** 5.26


def psychrometric_constant(pressure_kpa):
    return 0.665e-3 * pressure_kpa


def extraterrestrial_radiation(lat_rad, doy):
    gsc = 0.0820
    dr = 1.0 + 0.033 * math.cos(2.0 * math.pi * doy / 365.0)
    delta = 0.409 * math.sin(2.0 * math.pi * doy / 365.0 - 1.39)
    ws = math.acos(max(-1.0, min(1.0, -math.tan(lat_rad) * math.tan(delta))))
    return (24.0 * 60.0 / math.pi) * gsc * dr * (
        ws * math.sin(lat_rad) * math.sin(delta)
        + math.cos(lat_rad) * math.cos(delta) * math.sin(ws)
    )


def clear_sky_radiation(elevation_m, ra):
    return (0.75 + 2.0e-5 * elevation_m) * ra


def net_shortwave_radiation(rs, albedo=0.23):
    return (1.0 - albedo) * rs


def net_longwave_radiation(tmin, tmax, ea, rs, rso):
    sigma = 4.903e-9
    tk_min = tmin + 273.16
    tk_max = tmax + 273.16
    avg_tk4 = (tk_max**4 + tk_min**4) / 2.0
    humidity_factor = 0.34 - 0.14 * math.sqrt(ea)
    cloudiness = (
        max(0.05, 1.35 * min(rs / rso, 1.0) - 0.35) if rso > 0 else 0.05
    )
    return sigma * avg_tk4 * humidity_factor * cloudiness


def priestley_taylor_et0(rn, g, tmean_c, elevation_m):
    pressure = atmospheric_pressure(elevation_m)
    gamma = psychrometric_constant(pressure)
    delta = vapour_pressure_slope(tmean_c)
    return max(0.0, ALPHA_PT * 0.408 * (delta / (delta + gamma)) * (rn - g))


def penman_monteith_et0(rn, g, tmean_c, u2, vpd, elevation_m):
    pressure = atmospheric_pressure(elevation_m)
    gamma = psychrometric_constant(pressure)
    delta = vapour_pressure_slope(tmean_c)
    num = 0.408 * delta * (rn - g) + gamma * (900.0 / (tmean_c + 273.0)) * u2 * vpd
    den = delta + gamma * (1.0 + 0.34 * u2)
    return max(0.0, num / den)


def daily_et0_both(tmin, tmax, tmean, solar_rad, wind_2m, ea, elev, lat_deg, doy):
    lat_rad = math.radians(lat_deg)
    ra = extraterrestrial_radiation(lat_rad, doy)
    rso = clear_sky_radiation(elev, ra)
    rns = net_shortwave_radiation(solar_rad)
    rnl = net_longwave_radiation(tmin, tmax, ea, solar_rad, rso)
    rn = rns - rnl
    g = 0.0
    es = (saturation_vapour_pressure(tmin) + saturation_vapour_pressure(tmax)) / 2.0
    vpd = es - ea
    pt = priestley_taylor_et0(rn, g, tmean, elev)
    pm = penman_monteith_et0(rn, g, tmean, wind_2m, vpd, elev)
    return {
        "pt_et0": round(pt, 6),
        "pm_et0": round(pm, 6),
        "rn": round(rn, 6),
        "pt_pm_ratio": round(pt / pm, 6) if pm > 0 else None,
    }
```

```python
PASS_COL, FAIL_COL, INFO_COL = "#2ecc71", "#e74c3c", "#3498db"

passed = failed = 0


def check(name, computed, expected, tol):
    global passed, failed
    diff = abs(computed - expected)
    ok = diff <= tol
    if ok:
        passed += 1
        print(f"  [PASS] {name} = {computed:.6f} (expected {expected:.6f})")
    else:
        failed += 1
        print(f"  [FAIL] {name} = {computed:.6f} (expected {expected:.6f}, diff={diff:.6f})")
    return ok


print("§1 — Analytical properties")
for test in bench_pt["analytical_tests"]:
    result = priestley_taylor_et0(
        test["rn"], test["g"], test["tmean_c"], test["elevation_m"]
    )
    check(test["name"], result, test["expected_pt_et0"], test["tolerance"])

print("\n§2 — FAO-56 Example 18 (Uccle)")
uccle = bench_pt["fao56_example_18"]
result = daily_et0_both(**uccle["inputs"])
check("PT ET₀ (Uccle)", result["pt_et0"], uccle["expected"]["pt_et0"], uccle["tolerance_pt"])
check("PM ET₀ (Uccle)", result["pm_et0"], uccle["expected"]["pm_et0"], uccle["tolerance_pm"])
ratio = result["pt_pm_ratio"]
lo, hi = uccle["expected"]["pt_pm_ratio_range"]
if lo <= ratio <= hi:
    passed += 1
    print(f"  [PASS] PT/PM ratio = {ratio:.4f} (range [{lo}, {hi}])")
else:
    failed += 1
    print(f"  [FAIL] PT/PM ratio = {ratio:.4f} (expected [{lo}, {hi}])")

print("\n§3 — Climate gradient (humid → arid)")
prev_ratio = None
for case in bench_pt["climate_gradient"]["cases"]:
    result = daily_et0_both(**case["inputs"])
    check(f"PT ET₀ ({case['name']})", result["pt_et0"], case["expected_pt_et0"], case["tolerance"])
    if prev_ratio is not None and result["pt_pm_ratio"] is not None:
        if result["pt_pm_ratio"] > prev_ratio:
            failed += 1
            print("  [FAIL] PT/PM should decrease humid→arid")
        else:
            passed += 1
            print(f"  [PASS] PT/PM decreasing: {prev_ratio:.4f} → {result['pt_pm_ratio']:.4f}")
    prev_ratio = result["pt_pm_ratio"]

print("\n§4 — Monotonicity (PT vs Rn)")
prev_pt = None
for test in bench_pt["monotonicity_tests"]["increasing_rn"]:
    result = priestley_taylor_et0(test["rn"], 0.0, test["tmean_c"], test["elevation_m"])
    check(f"PT at Rn={test['rn']}", result, test["expected_pt_et0"], test["tolerance"])
    if prev_pt is not None:
        if result > prev_pt:
            passed += 1
            print(f"  [PASS] PT increasing: {prev_pt:.4f} → {result:.4f}")
        else:
            failed += 1
            print("  [FAIL] PT should increase with Rn")
    prev_pt = result

print("\n§5 — Temperature sensitivity")
prev_pt = None
for test in bench_pt["temperature_sensitivity"]["increasing_temp"]:
    result = priestley_taylor_et0(test["rn"], 0.0, test["tmean_c"], test["elevation_m"])
    check(f"PT at T={test['tmean_c']}°C", result, test["expected_pt_et0"], test["tolerance"])
    if prev_pt is not None:
        if result > prev_pt:
            passed += 1
            print(f"  [PASS] PT increasing with T: {prev_pt:.4f} → {result:.4f}")
        else:
            failed += 1
            print("  [FAIL] PT should increase with T")
    prev_pt = result

total = passed + failed
print("\n" + "=" * 60)
print(f"TOTAL: {passed}/{total} PASS, {failed} FAIL")
print("=" * 60)
validation_ok_pt = failed == 0
```

```python
PASS_COL, FAIL_COL, INFO_COL = "#2ecc71", "#e74c3c", "#3498db"

# 1) PT vs PM — climate gradient
cases = bench_pt["climate_gradient"]["cases"]
names = [c["name"].replace("_", " ") for c in cases]
pt_vals, pm_vals = [], []
for c in cases:
    r = daily_et0_both(**c["inputs"])
    pt_vals.append(r["pt_et0"])
    pm_vals.append(r["pm_et0"])

x = np.arange(len(names))
w = 0.35
fig, axes = plt.subplots(1, 3, figsize=(14, 4.2))

ax = axes[0]
ax.bar(x - w / 2, pt_vals, w, label="Priestley–Taylor", color=INFO_COL, alpha=0.9)
ax.bar(x + w / 2, pm_vals, w, label="Penman–Monteith", color="#34495e", alpha=0.8)
ax.set_xticks(x)
ax.set_xticklabels(names, rotation=15, ha="right")
ax.set_ylabel("ET₀ (mm/day)")
ax.set_title("Climate gradient: PT vs PM")
ax.legend(fontsize=8)

# 2) PT vs Rn (monotonicity)
mono = bench_pt["monotonicity_tests"]["increasing_rn"]
rn_axis = [m["rn"] for m in mono]
pt_axis = [
    priestley_taylor_et0(m["rn"], 0.0, m["tmean_c"], m["elevation_m"]) for m in mono
]
exp_axis = [m["expected_pt_et0"] for m in mono]
tols = [m["tolerance"] for m in mono]
ax2 = axes[1]
ax2.plot(rn_axis, pt_axis, "o-", color=INFO_COL, label="computed")
ax2.scatter(
    rn_axis,
    exp_axis,
    s=55,
    c=[
        PASS_COL if abs(a - b) <= t else FAIL_COL
        for a, b, t in zip(pt_axis, exp_axis, tols)
    ],
    edgecolors="k",
    zorder=5,
    label="benchmark expected",
)
ax2.set_xlabel("Net radiation Rn (MJ m⁻² day⁻¹)")
ax2.set_ylabel("PT ET₀ (mm/day)")
ax2.set_title("Monotonicity vs radiation")
ax2.legend(fontsize=8)

# 3) Temperature sensitivity
ts = bench_pt["temperature_sensitivity"]["increasing_temp"]
temps = [t["tmean_c"] for t in ts]
pt_t = [
    priestley_taylor_et0(t["rn"], 0.0, t["tmean_c"], t["elevation_m"]) for t in ts
]
ax3 = axes[2]
ax3.plot(temps, pt_t, "s-", color=INFO_COL, lw=2, markersize=6)
for t, computed, row in zip(temps, pt_t, ts):
    ok = abs(computed - row["expected_pt_et0"]) <= row["tolerance"]
    ax3.scatter(
        [t],
        [computed],
        color=PASS_COL if ok else FAIL_COL,
        s=70,
        zorder=5,
        edgecolors="black",
    )
ax3.set_xlabel("Mean air temperature (°C)")
ax3.set_ylabel("PT ET₀ (mm/day)")
ax3.set_title("Temperature sensitivity (fixed Rn)")

plt.tight_layout()
plt.show()
```

## Summary

| Item | Value |
|------|-------|
| Primal capability | `science.et0_priestley_taylor` |
| Rust validator | `validate_priestley_taylor` |
| Python baseline | [`control/priestley_taylor/priestley_taylor_et0.py`](../../control/priestley_taylor/priestley_taylor_et0.py) |
| Benchmark JSON | [`control/priestley_taylor/benchmark_priestley_taylor.json`](../../control/priestley_taylor/benchmark_priestley_taylor.json) |

**Provenance.** See `_provenance` in the benchmark JSON (baseline commit and tolerances).

**Future: Tier 2 primal IPC.** Expose analytical, cross-method, and climate-gradient checks as attestations keyed to `science.et0_priestley_taylor`.


