+++
title = "Stewart (1977) Yield Response to Water Stress"
description = "Rendered from 008-yield-response-stewart.ipynb"
date = 2026-07-18
weight = 50

[extra]
domain = "Lab"
rendered_from = "008-yield-response-stewart.ipynb"
+++

<!-- Auto-generated from 008-yield-response-stewart.ipynb by spore-validate render-notebooks -->

# Stewart (1977) Yield Response to Water Stress

**Citation:** Stewart, J.I. et al. (1977); documented in Allen et al. (1998) *FAO-56,* Chapter 10.

**AirSpring linkage:** primal `science.yield_response`; Rust binary `validate_yield_response`.

Sources: `control/yield_response/yield_response.py` + `benchmark_yield_response.json`.

The proportional deficit model relates actual to maximum yield through the seasonal water deficit and crop-specific sensitivity $K_\mathrm{y}$.


## Theory

Following Stewart's single-stage form (FAO-56 Chapter 10 presentation):

$$\left(1 - \frac{Y_\mathrm{a}}{Y_\mathrm{m}}\right) = K_\mathrm{y}\left(1 - \frac{ET_\mathrm{a}}{ET_\mathrm{m}}\right)$$

Equivalently, as implemented in airSpring controls:

$$\frac{Y_\mathrm{a}}{Y_\mathrm{m}} = 1 - K_\mathrm{y}\left(1 - \frac{ET_\mathrm{a}}{ET_\mathrm{c}}\right)$$

Multi-stage formulations multiply stage factors (FAO-56 Eq.\ 90). Water-use efficiency aggregates yield per unit actual evaporation:
$\mathrm{WUE} = Y / ET_\mathrm{a}$ using the mm–ha volumetric conventions in `yield_response.py`.


```python
import importlib.util
import json
from pathlib import Path

import matplotlib.pyplot as plt
import numpy as np

NOTEBOOK_DIR = Path.cwd().resolve()
BENCH_PATH = (NOTEBOOK_DIR / "../../control/yield_response/benchmark_yield_response.json").resolve()
CONTROL_PATH = (NOTEBOOK_DIR / "../../control/yield_response/yield_response.py").resolve()

spec = importlib.util.spec_from_file_location("yr", CONTROL_PATH)
yr = importlib.util.module_from_spec(spec)
spec.loader.exec_module(yr)

with open(BENCH_PATH, encoding="utf-8") as f:
    yb = json.load(f)

print("Loaded benchmark:", BENCH_PATH)
```

```python
# Core functions exercised (see yield_response.py)

ky_corn = yb["ky_values"]["corn"]["ky_total"]

grid = np.linspace(0, 1, 121)
yield_curve = np.array([yr.yield_ratio_single(ky_corn, r) for r in grid])

wue_demo = yr.water_use_efficiency(12000, 500)
print("Corn Ky (table):", ky_corn, "| WUE demo (kg/m3):", wue_demo)

ms = yr.yield_ratio_multistage([0.40, 1.50, 0.50, 0.20], [0.9, 0.9, 0.9, 0.9])
assert abs(ms - 0.7597) <= 1e-3
print("Multi-stage corn uniform deficit ratio:", ms)
```

```python
passed = []

for tc in yb["validation_checks"]["single_stage_analytical"]["test_cases"]:
    c = yr.yield_ratio_single(tc["ky"], tc["eta_etc"])
    assert abs(c - tc["expected_ratio"]) <= tc["tolerance"]
    passed.append(tc["label"])

for tc in yb["validation_checks"]["multi_stage_analytical"]["test_cases"]:
    c = yr.yield_ratio_multistage(tc["stages_ky"], tc["stages_eta_etc"])
    assert abs(c - tc["expected_ratio"]) <= tc["tolerance"]

for tc in yb["validation_checks"]["water_use_efficiency"]["test_cases"]:
    c = yr.water_use_efficiency(tc["yield_kg_ha"], tc["eta_mm"])
    assert abs(c - tc["expected_wue_kg_m3"]) <= tc["tolerance"]

scenario = yb["validation_checks"]["scheduling_comparison"]["scenario"]
strategies = yb["validation_checks"]["scheduling_comparison"]["strategies"]

np.random.seed(42)
season_days = scenario["season_days"]
et0_daily = np.maximum(0.5, np.random.normal(scenario["et0_mean_mm_day"], 1.5, season_days))
precip_mean = strategies["no_irrigation"]["precip_mean_mm_day"]
precip_prob = strategies["no_irrigation"]["precip_prob"]
rain_days = np.random.random(season_days) < precip_prob
precip_daily = np.where(
    rain_days,
    np.random.exponential(precip_mean / precip_prob, season_days),
    0.0,
)

results = {}
for name, strat in strategies.items():
    thresh_frac = strat.get("irrigation_threshold_frac", None)
    irrig_depth = strat.get("irrigation_depth_mm", 25.0) if thresh_frac else 0.0
    r = yr.simulate_season_with_yield(
        season_days=season_days,
        et0_daily=et0_daily,
        precip_daily=precip_daily,
        kc=1.2,
        ky_total=scenario["ky_total"],
        theta_fc=scenario["theta_fc"],
        theta_wp=scenario["theta_wp"],
        root_depth_m=scenario["root_depth_m"],
        p=scenario["p"],
        irrigation_threshold_frac=thresh_frac,
        irrigation_depth_mm=irrig_depth,
    )
    results[name] = r
    yr_lo, yr_hi = strat["expected_yield_ratio_range"]
    sd_lo, sd_hi = strat["expected_stress_days_range"]
    yr_clamped = r["yield_ratio_clamped"]
    assert yr_lo <= yr_clamped <= yr_hi
    assert sd_lo <= r["stress_days"] <= sd_hi

assert results["threshold_mad"]["yield_ratio_clamped"] > results["no_irrigation"]["yield_ratio_clamped"]
assert results["threshold_mad"]["stress_days"] < results["no_irrigation"]["stress_days"]

print("PASS analytical + scheduling envelope checks,", len(passed), "single-stage labels")
```

```python
C_GREEN, C_RED, C_BLUE = "#2ecc71", "#e74c3c", "#3498db"

fig, ax = plt.subplots(figsize=(7, 4))
ax.plot(grid, yield_curve, color=C_BLUE, lw=2, label="Ya/Ym vs ETa/Etc")

ax.axhline(1.0, color=C_GREEN, ls=":", alpha=0.8, label="No yield loss")
stress_x = np.array([0.9, 0.75, 0.50])
stress_y = np.array([yr.yield_ratio_single(ky_corn, x) for x in stress_x])
ax.scatter(stress_x, stress_y, color=C_RED, s=52, zorder=3, label="Example deficits")

ax.set_xlabel("ET$_a$/ET$_c$ (-)")
ax.set_ylabel("$Y_a / Y_{m}$ (-)")
ax.set_title(f"Single-stage Stewart curve (Ky={ky_corn}, corn)")
ax.legend()
plt.tight_layout()
plt.show()
```

## Summary

The Stewart proportional-deficit formulation and multi-stage extension were checked against analytic expectations in `benchmark_yield_response.json`, including WUE scalings. A stochastic Michigan-style scheduling trio (rain-fed vs threshold irrigation) verifies that MAD-style irrigation lifts clamped yield ratio and trims stress-days relative to rain-fed realizations seeded identically across strategies—precisely mirroring how `yield_response.py` gates cross-language validation (`validate_yield_response`).


