+++
title = "FAO-56 Chapter 8 — Daily Soil Water Balance Scheduling"
description = "Rendered from 004-fao56-water-balance.ipynb"
date = 2026-06-06
weight = 50

[extra]
domain = "Lab"
rendered_from = "004-fao56-water-balance.ipynb"
+++

<!-- Auto-generated from 004-fao56-water-balance.ipynb by spore-validate render-notebooks -->

# FAO-56 Chapter 8 — Daily Soil Water Balance Scheduling

**Citation:** Allen, R.G., Pereira, L.S., Raes, D., Smith, M. (1998). *FAO Irrigation and Drainage Paper 56,* Chapter 8.

**AirSpring linkage:** primal `science.water_balance`; Rust binary `validate_water_balance`.

This notebook aligns with `control/water_balance/fao56_water_balance.py` and the digitized benchmark in `control/water_balance/benchmark_water_balance.json`.


## Theory

FAO-56 expresses root-zone soil water deficit (depletion) $D_\mathrm{r,i}$ relative to maximum storage in the rooting depth. Chapter 8 gives the scheduling mass balance:

$$D_{\mathrm{r},i} = D_{\mathrm{r},i-1} - (P - \mathrm{RO})_i - I_i - \mathrm{CR}_i + ET_{\mathrm{c,adj},i} + DP_i$$

With $ET_{\mathrm{c,adj}} = K_s \, K_\mathrm{c} \, ET_0$, total available water $T_\mathrm{AW} = 1000(\theta_\mathrm{fc}-\theta_\mathrm{wp})Z_\mathrm{r}$ (mm), and readily available water $RAW = p \, T_\mathrm{AW}$, the stress coefficient is

$$K_s = \begin{cases}
1 & D_\mathrm{r} \le RAW\\
\frac{T_\mathrm{AW} - D_\mathrm{r}}{T_\mathrm{AW} - RAW} & D_\mathrm{r} > RAW
\end{cases}$$

(with $K_s \in [0,1]$ in practice).

This reproduction follows the simplifying defaults in the control script ($\mathrm{RO}=\mathrm{CR}=0$, deep percolation when $D_\mathrm{r}$ would become negative).


```python
import importlib.util
import json
from pathlib import Path

import matplotlib.pyplot as plt
import numpy as np

plt.rcParams["figure.figsize"] = (10, 4)
plt.rcParams["axes.grid"] = True

NOTEBOOK_DIR = Path.cwd().resolve()
BENCH_PATH = (NOTEBOOK_DIR / "../../control/water_balance/benchmark_water_balance.json").resolve()
CONTROL_PATH = (NOTEBOOK_DIR / "../../control/water_balance/fao56_water_balance.py").resolve()

spec = importlib.util.spec_from_file_location("wb", CONTROL_PATH)
wb = importlib.util.module_from_spec(spec)
spec.loader.exec_module(wb)

with open(BENCH_PATH, encoding="utf-8") as f:
    bench_wb = json.load(f)

tol_mb = bench_wb["mass_balance_test"]["tolerance"]
sl = bench_wb["soil_parameters"]["sandy_loam"]
corn = bench_wb["crop_parameters"]["corn"]

print("Benchmark:", BENCH_PATH)
print("Tolerance (mass balance):", tol_mb)
```

```python
# Core numerical core is in fao56_water_balance.py — functions used below:


def recap_api():
    return {
        "TAW(mm) example": wb.total_available_water(sl["theta_fc"], sl["theta_wp"], corn["root_depth_m"]),
        "RAW(mm) example": wb.readily_available_water(
            wb.total_available_water(sl["theta_fc"], sl["theta_wp"], corn["root_depth_m"]),
            corn["depletion_fraction_p"],
        ),
        "Ks at Dr=0": wb.stress_coefficient(0, 90, 49.5),
        "daily_water_balance_step keys": wb.daily_water_balance_step(10, P=0, I=0, ET0=5.0, Kc=1.2, Ks=1.0, TAW=90).keys(),
        "simulate_season keys": list(wb.simulate_season(np.array([5.0]), np.array([0.0]), Kc=1.2,
            theta_fc=0.18, theta_wp=0.08, root_depth_m=0.9, p=0.55).keys()),
    }


for k, v in recap_api().items():
    print(k, ":", v)
```

```python
# Structural validation against benchmark fields + mass balance closures

scenario = bench_wb["michigan_summer_scenario"]
rng = np.random.default_rng(42)

n_days = 90
et0 = rng.normal(scenario["et0_mean_mm_day"], scenario["et0_std_mm_day"], n_days)
et0 = np.maximum(et0, 0.5)
rain_days = rng.random(n_days) < scenario["precip_prob"]
precip = np.zeros(n_days)
precip[rain_days] = rng.exponential(scenario["precip_depth_when_rain_mm"], np.sum(rain_days))

result = wb.simulate_season(
    et0,
    precip,
    Kc=scenario["kc"],
    theta_fc=sl["theta_fc"],
    theta_wp=sl["theta_wp"],
    root_depth_m=corn["root_depth_m"],
    p=corn["depletion_fraction_p"],
    irrigation_trigger=True,
    irrig_depth_mm=25.0,
)

mb_error = wb.mass_balance_check(result)
assert mb_error <= tol_mb, mb_error

et_lo, et_hi = scenario["expected_seasonal_et_range_mm"]
assert et_lo <= result["total_et"] <= et_hi
assert result["irrig_events"] > 0

dry = wb.simulate_season(
    np.full(30, 5.0),
    np.zeros(30),
    Kc=1.2,
    theta_fc=0.18,
    theta_wp=0.08,
    root_depth_m=0.90,
    p=0.55,
    irrigation_trigger=False,
)
assert wb.mass_balance_check(dry) <= tol_mb
assert dry["Ks"][-1] < dry["Ks"][0]

print("PASS: Michigan summer + dry-down mass balance and scenario checks")
print("  Mass balance error (MI):", mb_error)
print("  Seasonal ET (mm):", float(result["total_et"]))
```

```python
C_GREEN, C_RED, C_BLUE = "#2ecc71", "#e74c3c", "#3498db"

fig, ax1 = plt.subplots()
ax2 = ax1.twinx()
ax1.plot(result["Dr"], color=C_RED, label="Dr (depletion, mm)")
ax1.set_ylabel("Depletion Dr (mm)", color=C_RED)
ax2.plot(result["Ks"], color=C_GREEN, label="Ks")
ax2.set_ylabel("Stress Ks (-)", color=C_GREEN)
ax1.set_xlabel("Day of season")
ax1.set_title("Michigan summer scenario — depletion and stress")

ax3 = ax1.twinx()
ax3.spines["right"].set_position(("axes", 1.12))
ax3.plot(result["ETc"], color=C_BLUE, alpha=0.85, label="ETc_adj (mm/d)")
ax3.set_ylabel("ETc_adj (mm/d)", color=C_BLUE)

lines, labels = [], []
for ax in (ax1, ax2, ax3):
    L, lab = ax.get_legend_handles_labels()
    lines += L
    labels += lab
ax1.legend(lines, labels, loc="upper left", frameon=True)
plt.tight_layout()
plt.show()
```

## Summary

We reproduced the FAO-56 Chapter 8 daily water balance (depletion update, $K_s$, adjusted $ET_\mathrm{c}$, deep percolation handling) and checked mass balance closure at the benchmark tolerance. The Michigan summer scenario produces physically plausible seasonal $ET_\mathrm{c}$, irrigation events, declining then partially recovered $K_\mathrm{s}$, and root-zone deficit dynamics consistent with the digitized expectation ranges in `benchmark_water_balance.json`.


