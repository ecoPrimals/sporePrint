+++
title = "ET₀ Sensitivity Analysis (One-at-a-Time)"
description = "Rendered from 017-et0-sensitivity-analysis.ipynb"
date = 2026-06-15
weight = 50

[extra]
domain = "Lab"
rendered_from = "017-et0-sensitivity-analysis.ipynb"
+++

<!-- Auto-generated from 017-et0-sensitivity-analysis.ipynb by spore-validate render-notebooks -->

# ET₀ Sensitivity Analysis (One-at-a-Time)

**Citation:** Gong L et al. (2006) *Agricultural Water Management* **86**:188–198.

**Primal:** N/A · **Rust:** `validate_sensitivity`

**Baseline:** `control/sensitivity/et0_sensitivity.py` · **Benchmark:** `control/sensitivity/benchmark_sensitivity.json`


## Theory

**One-at-a-time (OAT)** sensitivity: for each input $x_i$, perturb $\pm 10\%$ about the baseline, holding others fixed.  

**Sensitivity index:** $\partial \mathrm{ET}_0 / \partial x_i \approx (\mathrm{ET}_0^+ - \mathrm{ET}_0^-) / (2\Delta x_i)$.  

**Elasticity:** fractional response in $\mathrm{ET}_0$ per fractional change in $x_i$.  

The baseline meteorology follows **FAO-56 Example 18** (Uccle, Belgium) as encoded in the benchmark.


```python
import json
import math
from pathlib import Path

import matplotlib.pyplot as plt
import numpy as np

REPO = Path('/home/eastgate/Development/ecoPrimals/springs/airSpring').resolve()
BENCH = REPO / "control/sensitivity/benchmark_sensitivity.json"

C_GREEN, C_RED, C_BLUE = "#2ecc71", "#e74c3c", "#3498db"
plt.rcParams.update({"figure.figsize": (8, 4.5), "axes.grid": True})
```

```python
def saturation_vapour_pressure(t_c):
    return 0.6108 * math.exp(17.27 * t_c / (t_c + 237.3))


def slope_vapour_pressure_curve(t_c):
    es = saturation_vapour_pressure(t_c)
    return 4098.0 * es / (t_c + 237.3) ** 2


def atmospheric_pressure(altitude_m):
    return 101.3 * ((293.0 - 0.0065 * altitude_m) / 293.0) ** 5.26


def psychrometric_constant(pressure_kpa):
    return 0.000665 * pressure_kpa


def actual_vapour_pressure_rh(tmax_c, tmin_c, rhmax, rhmin):
    e_tmin = saturation_vapour_pressure(tmin_c)
    e_tmax = saturation_vapour_pressure(tmax_c)
    return (e_tmin * (rhmax / 100.0) + e_tmax * (rhmin / 100.0)) / 2.0


def solar_declination(doy):
    return 0.409 * math.sin(2.0 * math.pi / 365.0 * doy - 1.39)


def inverse_relative_distance(doy):
    return 1.0 + 0.033 * math.cos(2.0 * math.pi / 365.0 * doy)


def sunset_hour_angle(lat_rad, dec_rad):
    arg = max(-1.0, min(1.0, -math.tan(lat_rad) * math.tan(dec_rad)))
    return math.acos(arg)


def extraterrestrial_radiation(lat_deg, doy):
    gsc = 0.0820
    phi = math.radians(lat_deg)
    dr = inverse_relative_distance(doy)
    delta = solar_declination(doy)
    ws = sunset_hour_angle(phi, delta)
    return (24.0 * 60.0 / math.pi) * gsc * dr * (
        ws * math.sin(phi) * math.sin(delta) + math.cos(phi) * math.cos(delta) * math.sin(ws)
    )


def clear_sky_radiation(alt_m, ra):
    return (0.75 + 2e-5 * alt_m) * ra


def net_shortwave(rs, albedo=0.23):
    return (1.0 - albedo) * rs


def net_longwave(tmax_c, tmin_c, ea_kpa, rs_over_rso):
    sigma = 4.903e-9
    tmax_k4 = (tmax_c + 273.16) ** 4
    tmin_k4 = (tmin_c + 273.16) ** 4
    avg_k4 = (tmax_k4 + tmin_k4) / 2.0
    hf = 0.34 - 0.14 * math.sqrt(ea_kpa)
    cf = 1.35 * rs_over_rso - 0.35
    return sigma * avg_k4 * hf * cf


def fao56_pm(rn, g, tmean_c, u2, vpd_kpa, delta, gamma):
    num = 0.408 * delta * (rn - g) + gamma * (900.0 / (tmean_c + 273.0)) * u2 * vpd_kpa
    den = delta + gamma * (1.0 + 0.34 * u2)
    return num / den


def compute_et0(params):
    tmin = params["tmin_c"]
    tmax = params["tmax_c"]
    tmean = (tmin + tmax) / 2.0
    rh_min = params["rh_min_pct"]
    rh_max = params["rh_max_pct"]
    u2 = params["wind_2m_m_s"]
    rs = params["solar_rad_mj_m2_day"]
    elev = params["elevation_m"]
    lat = params["latitude_deg"]
    doy = params["day_of_year"]
    p = atmospheric_pressure(elev)
    gamma = psychrometric_constant(p)
    delta = slope_vapour_pressure_curve(tmean)
    es = (saturation_vapour_pressure(tmax) + saturation_vapour_pressure(tmin)) / 2.0
    ea = actual_vapour_pressure_rh(tmax, tmin, rh_max, rh_min)
    vpd = es - ea
    ra = extraterrestrial_radiation(lat, doy)
    rso = clear_sky_radiation(elev, ra)
    rns = net_shortwave(rs)
    rs_rso = min(rs / rso, 1.0) if rso > 0 else 1.0
    rnl = net_longwave(tmax, tmin, ea, rs_rso)
    rn = rns - rnl
    g = 0.0
    return fao56_pm(rn, g, tmean, u2, vpd, delta, gamma)


def oat_sensitivity(baseline_params, var_name, pct=10.0):
    et0_base = compute_et0(baseline_params)
    x_base = baseline_params[var_name]
    dx = abs(x_base) * pct / 100.0
    params_plus = {**baseline_params, var_name: x_base + dx}
    params_minus = {**baseline_params, var_name: x_base - dx}
    et0_plus = compute_et0(params_plus)
    et0_minus = compute_et0(params_minus)
    sensitivity = (et0_plus - et0_minus) / (2.0 * dx) if dx > 0 else 0.0
    elasticity = ((et0_plus - et0_minus) / et0_base) / (2.0 * pct / 100.0) if et0_base > 0 else 0.0
    return et0_base, et0_plus, et0_minus, sensitivity, elasticity


def full_sensitivity_analysis(params, variables, pct=10.0):
    results = []
    for var in variables:
        name = var["name"]
        et0_base, et0_plus, et0_minus, sens, elast = oat_sensitivity(params, name, pct)
        results.append(
            {
                "name": name,
                "label": var["label"],
                "sensitivity": sens,
                "abs_sensitivity": abs(sens),
                "elasticity": elast,
                "et0_plus": et0_plus,
                "et0_minus": et0_minus,
            }
        )
    results.sort(key=lambda r: r["abs_sensitivity"], reverse=True)
    return results
```

```python
with open(BENCH) as f:
    benchmark = json.load(f)

bc = benchmark["baseline_conditions"]
baseline_params = {
    "tmin_c": bc["tmin_c"],
    "tmax_c": bc["tmax_c"],
    "rh_min_pct": bc["rh_min_pct"],
    "rh_max_pct": bc["rh_max_pct"],
    "wind_2m_m_s": bc["wind_2m_m_s"],
    "solar_rad_mj_m2_day": bc["solar_rad_mj_m2_day"],
    "elevation_m": bc["elevation_m"],
    "latitude_deg": bc["latitude_deg"],
    "day_of_year": bc["day_of_year"],
}

et0 = compute_et0(baseline_params)
vc = benchmark["validation_checks"]["baseline_et0"]
assert abs(et0 - vc["expected"]) <= vc["tolerance"], et0

variables = benchmark["variables"]
pct = benchmark["perturbation_pct"]
results = full_sensitivity_analysis(baseline_params, variables, pct)

mc = benchmark["validation_checks"]["monotonicity"]
for r in results:
    if r["name"] in mc["positive_sensitivity"]:
        assert r["sensitivity"] > 0, r
    elif r["name"] in mc["negative_sensitivity"]:
        assert r["sensitivity"] < 0, r

ec = benchmark["validation_checks"]["elasticity_bounds"]
for r in results:
    assert ec["min_elasticity"] <= r["elasticity"] <= ec["max_elasticity"]

rc = benchmark["validation_checks"]["sensitivity_ranking"]
top_two = [r["name"] for r in results[:2]]
assert any(n in rc["top_two_include"] for n in top_two), top_two

for site in benchmark["validation_checks"]["multi_site_consistency"]["sites"]:
    sp = {
        "tmin_c": site["tmin_c"],
        "tmax_c": site["tmax_c"],
        "rh_min_pct": site["rh_min_pct"],
        "rh_max_pct": site["rh_max_pct"],
        "wind_2m_m_s": site["wind_2m_m_s"],
        "solar_rad_mj_m2_day": site["solar_rad_mj_m2_day"],
        "elevation_m": site["elevation_m"],
        "latitude_deg": site["latitude_deg"],
        "day_of_year": site["day_of_year"],
    }
    rr = full_sensitivity_analysis(sp, variables, pct)
    top3 = [x["name"] for x in rr[:3]]
    assert "solar_rad_mj_m2_day" in top3, (site["name"], top3)

print("benchmark_sensitivity.json: baseline ET0, monotonicity, elasticity, ranking, multi-site OK.")
```

```python
labels = [r["label"] for r in results]
vals = [r["abs_sensitivity"] for r in results]
colors = [C_GREEN, C_BLUE, C_RED, C_GREEN, C_BLUE, C_RED][: len(labels)]
fig, ax = plt.subplots()
ax.barh(labels[::-1], vals[::-1], color=colors[::-1], edgecolor="black", linewidth=0.4)
ax.set_xlabel(r"$|\partial ET_0 / \partial x|$ (mm day$^{-1}$ per unit $x$)")
ax.set_title("OAT sensitivity ranking (FAO-56 Example 18 baseline)")
plt.tight_layout()
plt.show()
```

## Summary

- **Baseline:** FAO-56 Example 18 encoded in `benchmark_sensitivity.json`; computed $\mathrm{ET}_0 \approx 3.88$ mm/day within tolerance.  
- **OAT:** ±10% perturbations reproduce expected monotonic signs, elasticity bounds, and literature-consistent dominance of radiation / humidity in the ranking.  
- **Provenance:** JSON `_provenance` references Allen (FAO-56), Gong et al., and related studies.


