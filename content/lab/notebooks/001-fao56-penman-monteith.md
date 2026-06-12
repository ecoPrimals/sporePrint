+++
title = "FAO-56 Penman-Monteith Reference Evapotranspiration (ET₀)"
description = "Rendered from 001-fao56-penman-monteith.ipynb"
date = 2026-06-11
weight = 50

[extra]
domain = "Lab"
rendered_from = "001-fao56-penman-monteith.ipynb"
+++

<!-- Auto-generated from 001-fao56-penman-monteith.ipynb by spore-validate render-notebooks -->

# FAO-56 Penman-Monteith Reference Evapotranspiration (ET₀)

**Citation:** Allen, R.G., Pereira, L.S., Raes, D., Smith, M. (1998). *FAO Irrigation and Drainage Paper 56.* https://www.fao.org/4/X0490E/x0490e00.htm

**Abstract.** This notebook reproduces the FAO-56 Penman–Monteith reference evapotranspiration (ET₀) core algebra: saturation vapour pressure, the slope of the saturation curve, and the combined radiation–aerodynamic ET₀ equation. Worked examples (Bangkok, Uccle, Lyon) and tabulated vapour-pressure checks are validated against digitized FAO-56 benchmarks in the airSpring control suite.

**For other springs:** Primal capability `science.et0_fao56`; Rust binary `validate_et0`. Results here should match `control/fao56/penman_monteith.py` when the same benchmark JSON is used.


## Theory

FAO-56 Eq. 6 (mm day⁻¹):

$$\mathrm{ET}_0 = \frac{0.408\,\Delta(R_n - G) + \gamma\,\frac{900}{T+273}\,u_2\,(e_s - e_a)}{\Delta + \gamma(1 + 0.34\,u_2)}$$

Saturation vapour pressure (Eq. 11):

$$e^\circ(T) = 0.6108\,\exp\left(\frac{17.27\,T}{T + 237.3}\right)$$

Slope of the curve (Eq. 13):

$$\Delta = \frac{4098\,e^\circ(T)}{(T + 237.3)^2}$$


```python
import json
import math
from pathlib import Path

import matplotlib.pyplot as plt
import numpy as np

NOTEBOOK_DIR = Path.cwd()
BENCH_PATH = NOTEBOOK_DIR / "../../control/fao56/benchmark_fao56.json"
BENCH_PATH = BENCH_PATH.resolve()

with open(BENCH_PATH, encoding="utf-8") as f:
    bench = json.load(f)

print("Loaded:", BENCH_PATH)
```

```python
# Core equations from control/fao56/penman_monteith.py (subset)


def saturation_vapour_pressure(t_c: float) -> float:
    """FAO-56 Eq. 11: e°(T) = 0.6108 exp(17.27 T / (T + 237.3))"""
    return 0.6108 * math.exp(17.27 * t_c / (t_c + 237.3))


def slope_vapour_pressure_curve(t_c: float) -> float:
    """FAO-56 Eq. 13: Δ = 4098 * e°(T) / (T + 237.3)²"""
    es = saturation_vapour_pressure(t_c)
    return 4098.0 * es / (t_c + 237.3) ** 2


def atmospheric_pressure(altitude_m: float) -> float:
    """FAO-56 Eq. 7"""
    return 101.3 * ((293.0 - 0.0065 * altitude_m) / 293.0) ** 5.26


def psychrometric_constant(pressure_kpa: float) -> float:
    """FAO-56 Eq. 8"""
    return 0.000665 * pressure_kpa


def fao56_penman_monteith(rn: float, G: float, tmean_c: float,
                          u2: float, vpd_kpa: float,
                          delta: float, gamma: float) -> float:
    """FAO-56 Eq. 6: ET₀ (mm/day)"""
    numerator = (0.408 * delta * (rn - G) +
                 gamma * (900.0 / (tmean_c + 273.0)) * u2 * vpd_kpa)
    denominator = delta + gamma * (1.0 + 0.34 * u2)
    return numerator / denominator


def solar_declination(day_of_year: int) -> float:
    return 0.409 * math.sin(2.0 * math.pi / 365.0 * day_of_year - 1.39)


def inverse_relative_distance(day_of_year: int) -> float:
    return 1.0 + 0.033 * math.cos(2.0 * math.pi / 365.0 * day_of_year)


def sunset_hour_angle(latitude_rad: float, declination_rad: float) -> float:
    arg = -math.tan(latitude_rad) * math.tan(declination_rad)
    arg = max(-1.0, min(1.0, arg))
    return math.acos(arg)


def extraterrestrial_radiation(latitude_deg: float, day_of_year: int) -> float:
    gsc = 0.0820
    phi = math.radians(latitude_deg)
    dr = inverse_relative_distance(day_of_year)
    delta = solar_declination(day_of_year)
    ws = sunset_hour_angle(phi, delta)
    return (24.0 * 60.0 / math.pi) * gsc * dr * (
        ws * math.sin(phi) * math.sin(delta) +
        math.cos(phi) * math.cos(delta) * math.sin(ws)
    )


def daylight_hours(latitude_deg: float, day_of_year: int) -> float:
    phi = math.radians(latitude_deg)
    delta = solar_declination(day_of_year)
    ws = sunset_hour_angle(phi, delta)
    return 24.0 / math.pi * ws


def solar_radiation_from_sunshine(n: float, N: float, Ra: float) -> float:
    return (0.25 + 0.50 * n / N) * Ra


def clear_sky_radiation(altitude_m: float, Ra: float) -> float:
    return (0.75 + 2e-5 * altitude_m) * Ra


def net_shortwave_radiation(Rs: float, albedo: float = 0.23) -> float:
    return (1.0 - albedo) * Rs


def net_longwave_radiation(tmax_c: float, tmin_c: float,
                           ea_kpa: float, Rs_over_Rso: float) -> float:
    sigma = 4.903e-9
    tmax_k4 = (tmax_c + 273.16) ** 4
    tmin_k4 = (tmin_c + 273.16) ** 4
    avg_k4 = (tmax_k4 + tmin_k4) / 2.0
    humidity_factor = 0.34 - 0.14 * math.sqrt(ea_kpa)
    cloudiness_factor = 1.35 * Rs_over_Rso - 0.35
    return sigma * avg_k4 * humidity_factor * cloudiness_factor


def soil_heat_flux_monthly(t_month: float, t_month_prev: float) -> float:
    return 0.14 * (t_month - t_month_prev)


def wind_speed_at_2m(uz: float, z: float) -> float:
    return uz * 4.87 / math.log(67.8 * z - 5.42)


def mean_saturation_vapour_pressure(tmax_c: float, tmin_c: float) -> float:
    return (saturation_vapour_pressure(tmax_c) +
            saturation_vapour_pressure(tmin_c)) / 2.0


def actual_vapour_pressure_rh(tmax_c: float, tmin_c: float,
                               rhmax: float, rhmin: float) -> float:
    e_tmin = saturation_vapour_pressure(tmin_c)
    e_tmax = saturation_vapour_pressure(tmax_c)
    return (e_tmin * (rhmax / 100.0) + e_tmax * (rhmin / 100.0)) / 2.0


def solar_radiation_from_temp(tmax_c: float, tmin_c: float,
                               Ra: float, krs: float = 0.16) -> float:
    return krs * math.sqrt(tmax_c - tmin_c) * Ra


def compute_example_17_bangkok(inputs: dict) -> dict:
    tmax = inputs["tmax_c"]
    tmin = inputs["tmin_c"]
    ea = inputs["ea_kpa"]
    u2 = inputs["u2_m_s"]
    n = inputs["sunshine_hours"]
    lat = inputs["latitude_deg_n"]
    alt = inputs["altitude_m"]
    doy = inputs["day_of_year"]
    t_month = inputs["t_month_c"]
    t_prev = inputs["t_month_prev_c"]
    tmean = (tmax + tmin) / 2.0
    delta = slope_vapour_pressure_curve(tmean)
    P = atmospheric_pressure(alt)
    gamma = psychrometric_constant(P)
    es = mean_saturation_vapour_pressure(tmax, tmin)
    vpd = es - ea
    Ra = extraterrestrial_radiation(lat, doy)
    N = daylight_hours(lat, doy)
    Rs = solar_radiation_from_sunshine(n, N, Ra)
    Rso = clear_sky_radiation(alt, Ra)
    Rns = net_shortwave_radiation(Rs)
    Rnl = net_longwave_radiation(tmax, tmin, ea, Rs / Rso)
    Rn = Rns - Rnl
    G = soil_heat_flux_monthly(t_month, t_prev)
    et0 = fao56_penman_monteith(Rn, G, tmean, u2, vpd, delta, gamma)
    return {
        "tmean_c": tmean,
        "delta_kpa_per_c": delta,
        "pressure_kpa": P,
        "gamma_kpa_per_c": gamma,
        "es_kpa": es,
        "vpd_kpa": vpd,
        "ra_mj_m2_day": Ra,
        "daylight_hours": N,
        "rs_mj_m2_day": Rs,
        "rso_mj_m2_day": Rso,
        "rns_mj_m2_day": Rns,
        "rnl_mj_m2_day": Rnl,
        "rn_mj_m2_day": Rn,
        "G_mj_m2_day": G,
        "et0_mm_day": et0,
    }


def compute_example_18_uccle(inputs: dict) -> dict:
    tmax = inputs["tmax_c"]
    tmin = inputs["tmin_c"]
    rhmax = inputs["rhmax_pct"]
    rhmin = inputs["rhmin_pct"]
    wind_10m_kmh = inputs["wind_speed_10m_km_h"]
    n = inputs["sunshine_hours"]
    lat = inputs["latitude_deg_n"]
    alt = inputs["altitude_m"]
    doy = inputs["day_of_year"]
    tmean = (tmax + tmin) / 2.0
    uz_ms = wind_10m_kmh / 3.6
    u2 = wind_speed_at_2m(uz_ms, 10.0)
    delta = slope_vapour_pressure_curve(tmean)
    P = atmospheric_pressure(alt)
    gamma = psychrometric_constant(P)
    es = mean_saturation_vapour_pressure(tmax, tmin)
    ea = actual_vapour_pressure_rh(tmax, tmin, rhmax, rhmin)
    vpd = es - ea
    Ra = extraterrestrial_radiation(lat, doy)
    N = daylight_hours(lat, doy)
    Rs = solar_radiation_from_sunshine(n, N, Ra)
    Rso = clear_sky_radiation(alt, Ra)
    Rns = net_shortwave_radiation(Rs)
    Rnl = net_longwave_radiation(tmax, tmin, ea, Rs / Rso)
    Rn = Rns - Rnl
    G = 0.0
    et0 = fao56_penman_monteith(Rn, G, tmean, u2, vpd, delta, gamma)
    return {
        "tmean_c": tmean,
        "u2_m_s": u2,
        "delta_kpa_per_c": delta,
        "pressure_kpa": P,
        "gamma_kpa_per_c": gamma,
        "es_kpa": es,
        "ea_kpa": ea,
        "vpd_kpa": vpd,
        "ra_mj_m2_day": Ra,
        "daylight_hours": N,
        "rs_mj_m2_day": Rs,
        "rso_mj_m2_day": Rso,
        "rns_mj_m2_day": Rns,
        "rnl_mj_m2_day": Rnl,
        "rn_mj_m2_day": Rn,
        "G_mj_m2_day": G,
        "et0_mm_day": et0,
    }


def compute_example_20_lyon(inputs: dict) -> dict:
    tmax = inputs["tmax_c"]
    tmin = inputs["tmin_c"]
    lat = inputs["latitude_deg_n"]
    alt = inputs["altitude_m"]
    doy = inputs["day_of_year"]
    u2 = inputs["u2_m_s_estimated"]
    tmean = (tmax + tmin) / 2.0
    delta = slope_vapour_pressure_curve(tmean)
    P = atmospheric_pressure(alt)
    gamma = psychrometric_constant(P)
    ea = saturation_vapour_pressure(tmin)
    es = mean_saturation_vapour_pressure(tmax, tmin)
    vpd = es - ea
    Ra = extraterrestrial_radiation(lat, doy)
    Rs = solar_radiation_from_temp(tmax, tmin, Ra, krs=0.16)
    Rso = clear_sky_radiation(alt, Ra)
    Rns = net_shortwave_radiation(Rs)
    Rnl = net_longwave_radiation(tmax, tmin, ea, Rs / Rso)
    Rn = Rns - Rnl
    G = 0.0
    et0 = fao56_penman_monteith(Rn, G, tmean, u2, vpd, delta, gamma)
    return {
        "tmean_c": tmean,
        "delta_kpa_per_c": delta,
        "pressure_kpa": P,
        "gamma_kpa_per_c": gamma,
        "ea_kpa": ea,
        "es_kpa": es,
        "vpd_kpa": vpd,
        "ra_mj_m2_day": Ra,
        "rs_mj_m2_day": Rs,
        "rso_mj_m2_day": Rso,
        "rns_mj_m2_day": Rns,
        "rnl_mj_m2_day": Rnl,
        "rn_mj_m2_day": Rn,
        "et0_mm_day": et0,
    }
```

```python
def check(label: str, computed: float, expected: float, tol: float) -> bool:
    diff = abs(computed - expected)
    ok = diff <= tol
    status = "PASS" if ok else "FAIL"
    print(f"  [{status}] {label}: {computed:.4f} (expected {expected:.4f}, tol {tol:.4f}, diff {diff:.4f})")
    return ok


def validate_component_tables(benchmark: dict) -> tuple:
    passed = failed = 0
    print("\n=== Saturation Vapour Pressure (FAO-56 Table 2.3) ===")
    es_table = benchmark["saturation_vapour_pressure_table"]
    es_tol = es_table["tolerance_kpa"]
    for row in es_table["data"]:
        c = saturation_vapour_pressure(row["temp_c"])
        if check(f"e°({row['temp_c']:.0f}°C)", c, row["es_kpa"], es_tol):
            passed += 1
        else:
            failed += 1
    print("\n=== Slope Vapour Pressure Curve (FAO-56 Table 2.4) ===")
    delta_table = benchmark["slope_vapour_pressure_table"]
    delta_tol = delta_table["tolerance_kpa_per_c"]
    for row in delta_table["data"]:
        c = slope_vapour_pressure_curve(row["temp_c"])
        if check(f"Δ({row['temp_c']:.0f}°C)", c, row["delta_kpa_per_c"], delta_tol):
            passed += 1
        else:
            failed += 1
    return passed, failed


def validate_example(name: str, compute_fn, example_data: dict) -> tuple:
    passed = failed = 0
    print(f"\n=== {name} ===")
    result = compute_fn(example_data["inputs"])
    expected = example_data["intermediates"]
    intermediate_tol = {
        "tmean_c": 0.1,
        "delta_kpa_per_c": 0.005,
        "gamma_kpa_per_c": 0.002,
        "es_kpa": 0.02,
        "vpd_kpa": 0.02,
        "ea_kpa": 0.02,
        "ra_mj_m2_day": 0.5,
        "rs_mj_m2_day": 0.3,
        "rso_mj_m2_day": 0.3,
        "rns_mj_m2_day": 0.3,
        "rnl_mj_m2_day": 0.3,
        "rn_mj_m2_day": 0.5,
        "u2_m_s": 0.01,
        "pressure_kpa": 0.2,
        "daylight_hours": 0.2,
    }
    for key, tol in intermediate_tol.items():
        if key in result and key in expected:
            if check(key, result[key], expected[key], tol):
                passed += 1
            else:
                failed += 1
    et0_exp = example_data["expected_et0_mm_day"]
    et0_tol = example_data["tolerance_mm_day"]
    if check("ET₀ (mm/day)", result["et0_mm_day"], et0_exp, et0_tol):
        passed += 1
    else:
        failed += 1
    return passed, failed


total_p = total_f = 0
p, f = validate_component_tables(bench)
total_p += p
total_f += f
for name, fn, key in [
    ("Example 17: Bangkok", compute_example_17_bangkok, "example_17_bangkok_monthly"),
    ("Example 18: Uccle", compute_example_18_uccle, "example_18_uccle_daily"),
    ("Example 20: Lyon", compute_example_20_lyon, "example_20_lyon_missing_data"),
]:
    p, f = validate_example(name, fn, bench[key])
    total_p += p
    total_f += f
total = total_p + total_f
print("\n" + "=" * 60)
print(f"TOTAL: {total_p}/{total} PASS, {total_f}/{total} FAIL")
print("=" * 60)
validation_ok_fao56 = total_f == 0
```

```python
PASS_COL, FAIL_COL, INFO_COL = "#2ecc71", "#e74c3c", "#3498db"

# 1) Table 2.3 computed vs benchmark
tbl = bench["saturation_vapour_pressure_table"]
temps = [r["temp_c"] for r in tbl["data"]]
comp = [saturation_vapour_pressure(r["temp_c"]) for r in tbl["data"]]
exp = [r["es_kpa"] for r in tbl["data"]]
tol = tbl["tolerance_kpa"]
bar_ok = [abs(c - e) <= tol for c, e in zip(comp, exp)]

fig, axes = plt.subplots(1, 3, figsize=(14, 4.5))

ax = axes[0]
x = np.arange(len(temps))
w = 0.35
ax.bar(x - w/2, comp, width=w, label="Computed", color=INFO_COL, alpha=0.85)
ax.bar(x + w/2, exp, width=w, label="FAO-56 Table 2.3", color="#34495e", alpha=0.75)
for i, ok in enumerate(bar_ok):
    ax.text(i, max(comp[i], exp[i]) + 0.15, "OK" if ok else "X",
            ha="center", fontsize=8, color=(PASS_COL if ok else FAIL_COL))
ax.set_xticks(x)
ax.set_xticklabels([f"{t}°C" for t in temps], rotation=45, ha="right")
ax.set_ylabel("Saturation vapour pressure (kPa)")
ax.set_title("Table 2.3: saturation vapour pressure")
ax.legend(fontsize=8)

# 2) ET₀ scatter: computed vs expected (3 examples)
examples = [
    ("Bangkok (Ex. 17)", compute_example_17_bangkok, "example_17_bangkok_monthly"),
    ("Uccle (Ex. 18)", compute_example_18_uccle, "example_18_uccle_daily"),
    ("Lyon (Ex. 20)", compute_example_20_lyon, "example_20_lyon_missing_data"),
]
comps = []
exps = []
labels = []
tols = []
for lab, fn, k in examples:
    r = fn(bench[k]["inputs"])
    comps.append(r["et0_mm_day"])
    exps.append(bench[k]["expected_et0_mm_day"])
    labels.append(lab)
    tols.append(bench[k]["tolerance_mm_day"])

ax2 = axes[1]
lims = [min(comps + exps) - 1, max(comps + exps) + 1]
ax2.plot(lims, lims, color=INFO_COL, lw=1, label="1:1")
for lab, c, e, tol in zip(labels, comps, exps, tols):
    ok = abs(c - e) <= tol
    ax2.scatter([e], [c], s=80, color=(PASS_COL if ok else FAIL_COL), zorder=3, edgecolors="black")
ax2.set_xlabel("Expected ET₀ (mm/day)")
ax2.set_ylabel("Computed ET₀ (mm/day)")
ax2.set_title("ET₀ examples: computed vs expected")
ax2.set_aspect("equal", adjustable="datalim")

# 3) Example 17 intermediates
exp17 = bench["example_17_bangkok_monthly"]
res17 = compute_example_17_bangkok(exp17["inputs"])
exp_i = exp17["intermediates"]
keys_plot = ["tmean_c", "delta_kpa_per_c", "gamma_kpa_per_c", "es_kpa", "vpd_kpa",
             "rn_mj_m2_day", "G_mj_m2_day"]
cvals = [res17[k] for k in keys_plot]
evals = [exp_i[k] for k in keys_plot]
tol_map = {
    "tmean_c": 0.1, "delta_kpa_per_c": 0.005, "gamma_kpa_per_c": 0.002,
    "es_kpa": 0.02, "vpd_kpa": 0.02, "rn_mj_m2_day": 0.5, "G_mj_m2_day": 0.3,
}
ax3 = axes[2]
xx = np.arange(len(keys_plot))
wp = 0.35
bars_c = ax3.bar(xx - wp/2, cvals, width=wp, label="Computed", color=INFO_COL, alpha=0.85)
bars_e = ax3.bar(xx + wp/2, evals, width=wp, label="Expected", color="#95a5a6", alpha=0.85)
for i, k in enumerate(keys_plot):
    ok = abs(cvals[i] - evals[i]) <= tol_map.get(k, 0.5)
    ax3.text(i, max(cvals[i], evals[i]) * 1.03, "✓" if ok else "✗",
             ha="center", fontsize=9, color=(PASS_COL if ok else FAIL_COL))
ax3.set_xticks(xx)
ax3.set_xticklabels(keys_plot, rotation=35, ha="right", fontsize=7)
ax3.set_title("Example 17 intermediates")
ax3.legend(fontsize=7)

plt.tight_layout()
plt.show()
```

## Summary

| Item | Value |
|------|-------|
| Primal capability | `science.et0_fao56` |
| Rust validator | `validate_et0` |
| Python baseline | [`control/fao56/penman_monteith.py`](../../control/fao56/penman_monteith.py) |
| Benchmark JSON | [`control/fao56/benchmark_fao56.json`](../../control/fao56/benchmark_fao56.json) |

**Provenance.** Values are digitized from FAO-56 Chapter 4 examples and tables; see `_provenance` in the benchmark JSON.

**Future: Tier 2 primal IPC.** Wire these checks into the spring runtime so regressions surface as structured capability attestations alongside `validate_et0`.


