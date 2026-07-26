+++
title = "Thornthwaite (1948) Monthly Evapotranspiration"
description = "Estimates Thornthwaite (1948) monthly evapotranspiration from temperature data using heat index and day-length corrections for water balance studies."
date = 2026-07-18
weight = 50

[extra]
domain = "Lab"
rendered_from = "021-thornthwaite-et0.ipynb"
+++

<!-- Auto-generated from 021-thornthwaite-et0.ipynb by spore-validate render-notebooks -->

# Thornthwaite (1948) Monthly Evapotranspiration

**Citation:** Thornthwaite, C.W. (1948). *Geographical Review*, 38(1), 55-94.

**Abstract.** Thornthwaite’s monthly PET index uses an annual heat index and empirical exponent, adjusted for daylight and month length. This notebook mirrors `thornthwaite_et0.py`, validates against `benchmark_thornthwaite.json`, and compares seasonal structure to Hargreaves on the same illustrative stations.

**For other springs:** Primal capability `science.thornthwaite`; Rust binary `validate_thornthwaite`.


## Theory

$$I = \sum_i \left(\frac{T_i}{5}\right)^{1.514}\quad\text{(for } T_i > 0\text{)}$$

$$a = 6.75\times10^{-7} I^3 - 7.71\times10^{-5} I^2 + 1.792\times10^{-2} I + 0.49239$$

$$\mathrm{PET}_i = 16\left(\frac{10 T_i}{I}\right)^a$$

$$\mathrm{PET}_{\mathrm{adj}} = \mathrm{PET}_i \cdot \frac{N_i}{12}\cdot\frac{d_i}{30}$$


```python
import json
from pathlib import Path

import matplotlib.pyplot as plt
import numpy as np

BENCH_TW = (Path.cwd() / "../../control/thornthwaite/benchmark_thornthwaite.json").resolve()
with open(BENCH_TW, encoding="utf-8") as f:
    bench_tw = json.load(f)
print("Loaded:", BENCH_TW)
```

```python
import math

# From control/thornthwaite/thornthwaite_et0.py

DAYS_IN_MONTH = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]


def monthly_heat_index_term(tmean_c):
    if tmean_c <= 0.0:
        return 0.0
    return (tmean_c / 5.0) ** 1.514


def annual_heat_index(monthly_temps):
    return sum(monthly_heat_index_term(t) for t in monthly_temps)


def thornthwaite_exponent(heat_index):
    i = heat_index
    return 6.75e-7 * i**3 - 7.71e-5 * i**2 + 1.792e-2 * i + 0.49239


def unadjusted_monthly_et0(tmean_c, heat_index, exponent_a):
    if tmean_c <= 0.0 or heat_index <= 0.0:
        return 0.0
    if tmean_c >= 26.5:
        return -415.85 + 32.24 * tmean_c - 0.43 * tmean_c**2
    return 16.0 * (10.0 * tmean_c / heat_index) ** exponent_a


def daylight_hours(latitude_deg, day_of_year):
    lat_rad = math.radians(latitude_deg)
    decl = 0.4093 * math.sin(2.0 * math.pi / 365.0 * day_of_year - 1.405)
    arg = -math.tan(lat_rad) * math.tan(decl)
    arg = max(-1.0, min(1.0, arg))
    ws = math.acos(arg)
    return 24.0 / math.pi * ws


def mean_daylight_hours_for_month(latitude_deg, month_index):
    doy_start = sum(DAYS_IN_MONTH[:month_index]) + 1
    days = DAYS_IN_MONTH[month_index]
    total = sum(daylight_hours(latitude_deg, doy_start + d) for d in range(days))
    return total / days


def thornthwaite_monthly_et0(monthly_temps, latitude_deg):
    hi = annual_heat_index(monthly_temps)
    if hi <= 0.0:
        return [0.0] * 12
    a = thornthwaite_exponent(hi)
    et0_monthly = []
    for m in range(12):
        pet_unadj = unadjusted_monthly_et0(monthly_temps[m], hi, a)
        n_hours = mean_daylight_hours_for_month(latitude_deg, m)
        d = DAYS_IN_MONTH[m]
        pet_adj = pet_unadj * (n_hours / 12.0) * (d / 30.0)
        et0_monthly.append(max(0.0, pet_adj))
    return et0_monthly


def hargreaves_monthly_et0(monthly_tmin, monthly_tmax, latitude_deg):
    monthly_et0 = []
    for m in range(12):
        doy_start = sum(DAYS_IN_MONTH[:m]) + 1
        days = DAYS_IN_MONTH[m]
        total = 0.0
        for d in range(days):
            doy = doy_start + d
            lat_rad = math.radians(latitude_deg)
            dr = 1.0 + 0.033 * math.cos(2.0 * math.pi / 365.0 * doy)
            decl = 0.4093 * math.sin(2.0 * math.pi / 365.0 * doy - 1.405)
            ws = math.acos(max(-1.0, min(1.0, -math.tan(lat_rad) * math.tan(decl))))
            ra = (24.0 * 60.0 / math.pi) * 0.0820 * dr * (
                ws * math.sin(lat_rad) * math.sin(decl)
                + math.cos(lat_rad) * math.cos(decl) * math.sin(ws)
            )
            ra_mm = ra * 0.408
            tmax = monthly_tmax[m]
            tmin = monthly_tmin[m]
            tmean = (tmax + tmin) / 2.0
            delta_t = max(0.0, tmax - tmin)
            et0_day = 0.0023 * ra_mm * (tmean + 17.8) * math.sqrt(delta_t)
            total += max(0.0, et0_day)
        monthly_et0.append(total)
    return monthly_et0
```

```python
PASS_COL, FAIL_COL, INFO_COL = "#2ecc71", "#e74c3c", "#3498db"

passed = failed = 0


def ck(name, condition, detail=""):
    global passed, failed
    if condition:
        passed += 1
        print(f"  [PASS] {name}")
    else:
        failed += 1
        print(f"  [FAIL] {name}: {detail}")


a = bench_tw["analytical"]
ck(
    "heat_index_25C",
    abs(monthly_heat_index_term(25.0) - a["heat_index_25C"]) < 1e-4,
)
ck(
    "heat_index_uniform_25C",
    abs(annual_heat_index([25.0] * 12) - a["heat_index_annual_uniform_25C"]) < 0.01,
)
hi_u = annual_heat_index([25.0] * 12)
ck(
    "exponent_uniform_25C",
    abs(thornthwaite_exponent(hi_u) - a["exponent_uniform_25C"]) < 1e-4,
)
ck(
    "unadjusted_et0_25C",
    abs(
        unadjusted_monthly_et0(25.0, hi_u, thornthwaite_exponent(hi_u))
        - a["unadjusted_et0_25C"]
    )
    < 0.02,
)


def cmp_monthly(site_key):
    blk = bench_tw[site_key]
    lat = blk["latitude"]
    monthly = blk["monthly_tmean_c"]
    tol = blk["tol"]
    comp = thornthwaite_monthly_et0(monthly, lat)
    exp = blk["monthly_et0_mm"]
    ok_all = True
    for i, (c, e) in enumerate(zip(comp, exp)):
        if abs(c - e) > tol:
            ok_all = False
            print(f"  [FAIL] {site_key} month {i+1}: {c:.4f} vs {e:.4f} (tol={tol})")
    if ok_all:
        global passed
        passed += 1
        print(f"  [PASS] {site_key} monthly ET₀ within tolerance")
    else:
        global failed
        failed += 1


print("Monthly patterns vs benchmark:")
cmp_monthly("east_lansing")
cmp_monthly("wooster")

ann_el = sum(
    thornthwaite_monthly_et0(bench_tw["east_lansing"]["monthly_tmean_c"],
                             bench_tw["east_lansing"]["latitude"])
)
ann_el_e = bench_tw["east_lansing"]["annual_et0_mm"]
ck("east_lansing_annual", abs(ann_el - ann_el_e) <= bench_tw["east_lansing"]["tol"], f"diff={abs(ann_el-ann_el_e)}")

ann_wo = sum(
    thornthwaite_monthly_et0(bench_tw["wooster"]["monthly_tmean_c"],
                             bench_tw["wooster"]["latitude"])
)
ann_wo_e = bench_tw["wooster"]["annual_et0_mm"]
ck("wooster_annual", abs(ann_wo - ann_wo_e) <= bench_tw["wooster"]["tol"])

# Monotonicity: uniform warming increases annual total
lat_m = bench_tw["monotonicity_latitude"]
prev = -1.0
mono_ok = True
for t in bench_tw["monotonicity_temps"]:
    annual = sum(thornthwaite_monthly_et0([t] * 12, lat_m))
    if annual <= prev:
        mono_ok = False
    prev = annual
ck("temp_monotonicity_uniform", mono_ok)

# Edge: all frozen
fz = bench_tw["edge_cases"]["all_frozen"]
ck(
    "all_frozen",
    sum(thornthwaite_monthly_et0(fz["monthly_tmean_c"], 45.0)) == fz["expected_annual"],
)

# Tropical range
tr = bench_tw["edge_cases"]["tropical_uniform"]
trop_sum = sum(thornthwaite_monthly_et0(tr["monthly_tmean_c"], tr["latitude"]))
lo, hi = tr["annual_range"]
ck("tropical_annual_range", lo <= trop_sum <= hi, f"annual={trop_sum}")

# TH vs Hargreaves ratio (East Lansing)
el = bench_tw["east_lansing"]
el_tmean = el["monthly_tmean_c"]
el_tmax = [t + 6.0 for t in el_tmean]
el_tmin = [t - 6.0 for t in el_tmean]
th_m = thornthwaite_monthly_et0(el_tmean, el["latitude"])
hg_m = hargreaves_monthly_et0(el_tmin, el_tmax, el["latitude"])
th_g = sum(th_m[4:9])
hg_g = sum(hg_m[4:9])
ratio = th_g / hg_g if hg_g > 0 else 0
rr = bench_tw["thresholds"]["th_hg_ratio_range"]
ck("th_vs_hg_growing", rr[0] < ratio < rr[1], f"ratio={ratio:.3f}")

total = passed + failed
print("\n" + "=" * 60)
print(f"Checks reported: {passed} pass, {failed} fail (aggregated)")
print("=" * 60)
validation_ok_tw = failed == 0
```

```python
PASS_COL, FAIL_COL, INFO_COL = "#2ecc71", "#e74c3c", "#3498db"

fig, axes = plt.subplots(1, 3, figsize=(14, 4.2))

# 1) Monthly profiles
el = bench_tw["east_lansing"]
wo = bench_tw["wooster"]
th_el = thornthwaite_monthly_et0(el["monthly_tmean_c"], el["latitude"])
th_wo = thornthwaite_monthly_et0(wo["monthly_tmean_c"], wo["latitude"])
exp_el = el["monthly_et0_mm"]
exp_wo = wo["monthly_et0_mm"]
months = np.arange(1, 13)
ax = axes[0]
w = 0.35
ax.bar(months - w / 2, th_el, width=w, label="East Lansing (computed)", color=INFO_COL, alpha=0.85)
ax.bar(months + w / 2, th_wo, width=w, label="Wooster (computed)", color="#34495e", alpha=0.8)
ax.set_xticks(months)
ax.set_xlabel("Month")
ax.set_ylabel("ET₀ (mm/month)")
ax.set_title("Monthly Thornthwaite ET₀ profiles")
ax.legend(fontsize=7)

# mark benchmark agreement
for i in range(12):
    ok_el = abs(th_el[i] - exp_el[i]) <= el["tol"]
    ok_wo = abs(th_wo[i] - exp_wo[i]) <= wo["tol"]
    if not ok_el:
        ax.text(i + 1, th_el[i] + 8, "×", ha="center", color=FAIL_COL)
    if not ok_wo:
        ax.text(i + 1, th_wo[i] + 8, "×", ha="center", color=FAIL_COL)

# 2) T vs heat-index contribution
temps_curve = np.linspace(-5, 35, 100)
terms = [monthly_heat_index_term(float(t)) for t in temps_curve]
ax2 = axes[1]
ax2.plot(temps_curve, terms, color=INFO_COL, lw=2)
ax2.axvline(0, color="#7f8c8d", ls="--", lw=0.8)
ax2.set_xlabel("Monthly mean temperature (°C)")
ax2.set_ylabel(r"$(T/5)^{1.514}$ contribution")
ax2.set_title("Heat-index term vs temperature")

# 3) Thornthwaite vs Hargreaves monthly (East Lansing)
el_tmean = el["monthly_tmean_c"]
el_tmax = [t + 6.0 for t in el_tmean]
el_tmin = [t - 6.0 for t in el_tmean]
hg = hargreaves_monthly_et0(el_tmin, el_tmax, el["latitude"])
ax3 = axes[2]
ax3.plot(months, th_el, "o-", color=INFO_COL, label="Thornthwaite")
ax3.plot(months, hg, "s--", color="#e67e22", label="Hargreaves (proxy Tmin/Tmax)")
ax3.set_xlabel("Month")
ax3.set_ylabel("mm/month")
ax3.set_title("Cross-check: Thornthwaite vs Hargreaves (East Lansing)")
ax3.legend(fontsize=8)

plt.tight_layout()
plt.show()
```

## Summary

| Item | Value |
|------|-------|
| Primal capability | `science.thornthwaite` |
| Rust validator | `validate_thornthwaite` |
| Python baseline | [`control/thornthwaite/thornthwaite_et0.py`](../../control/thornthwaite/thornthwaite_et0.py) |
| Benchmark JSON | [`control/thornthwaite/benchmark_thornthwaite.json`](../../control/thornthwaite/benchmark_thornthwaite.json) |

**Provenance.** Monthly means for East Lansing and Wooster are documented in the control script and `_provenance` in the benchmark JSON.

**Future: Tier 2 primal IPC.** Monthly vectors and aggregated checks map cleanly to attestations under `science.thornthwaite`.


