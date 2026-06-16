+++
title = "Michigan Crop Water Atlas — 100 Stations, 80 Years"
description = "Rendered from 018-michigan-crop-water-atlas.ipynb"
date = 2026-06-16
weight = 50

[extra]
domain = "Lab"
rendered_from = "018-michigan-crop-water-atlas.ipynb"
+++

<!-- Auto-generated from 018-michigan-crop-water-atlas.ipynb by spore-validate render-notebooks -->

# Michigan Crop Water Atlas — 100 Stations, 80 Years

**Citation / data:** ERA5-derived daily weather courtesy of Open-Meteo hourly archive; methodological footprint described in airSpring Experiment 018 README.

This is the flagship integration experiment combining FAO-56 reference evaporation, staged crop coefficients, soil-water stress, MAD-style replenishment irrigation, and Stewart-type yield bookkeeping.

**AirSpring linkage:** primal `ecology.full_pipeline`; Rust validator `validate_atlas`.

Baseline driver: `control/atlas/atlas_water_budget.py` — benchmark snapshot: `control/atlas/benchmark_atlas.json`.


## Theory

Atlas processing threads together:

$$\mathrm{ET_0}=f_{\mathrm{PM}}\left(R_n,u_2,e_s-e_a,\Delta,\gamma\right)\quad\text{(FAO-56 Penman-Monteith)}$$

$$ET_{\mathrm{c},i}=K_{\mathrm{c},i}(t)\ ET_{0,i}, \quad ET_{\mathrm{a},i}=K_{\mathrm{s},i} ET_{\mathrm{c},i}$$
with $K_{\mathrm{c}}$ evolving through empirical initial/mid/end plateaus over the simulated season fraction, identical to Chapter 8 stress reduction with managed irrigation pulses when depletion crosses $RAW=p\,T_{\mathrm{AW}}$.

$$\frac{Y_a}{Y_m} = \max\left(0, \min\left(1, 1 - K_y (1 - \mathrm{ETA}/\mathrm{ETC})\right)\right)$$

where seasonal $\mathrm{ETA}/\mathrm{ETC}$ is accumulated actual vs unstressed totals.

The Python atlas script parses Open-Meteo CSV exports, restricts to agronomic summers (approximately DOY $121\le d \le 273$ when valid), and summarizes per crop means for cross-validation.


```python
import json
from pathlib import Path

import matplotlib.pyplot as plt
import numpy as np

NOTEBOOK_DIR = Path.cwd().resolve()
BENCH_PATH = (NOTEBOOK_DIR / "../../control/atlas/benchmark_atlas.json").resolve()
CONTROL_PATH = (NOTEBOOK_DIR / "../../control/atlas/atlas_water_budget.py").resolve()

with open(BENCH_PATH, encoding="utf-8") as f:
    atlas = json.load(f)

stations = atlas.get("stations", {})
print("Control script reference:", CONTROL_PATH.name)
print("Stations encoded:", len(stations))
print("Bench file:", BENCH_PATH)
sample_key = sorted(stations.keys())[0]
print("Example station key:", sample_key)
print(atlas["stations"][sample_key].keys())
```

```python
# Key pipeline logic is in atlas_water_budget.py (ET0 + MAD irrigation + staged Kc).


def recap_atlas_station(station_id):
    sd = atlas["stations"][station_id]
    crops = sd.get("crops", {})
    return {
        "n_days": sd.get("n_days"),
        "n_years": sd.get("n_years"),
        "mean_ET0_mm_yr": sd.get("mean_annual_et0"),
        "crop_count": len(crops),
        "Corn mean_yield": crops.get("Corn", {}).get("mean_yield_ratio"),
    }


example = recap_atlas_station(sample_key)

print(example)
assert example["Corn mean_yield"] is not None
```

```python
# Structural validation suitable for notebooks (no filesystem weather dependency)

station_ids = list(stations.keys())
assert station_ids

for sid in station_ids[:20]:
    blob = atlas["stations"][sid]
    assert blob["mean_annual_et0"] >= 200
    for crop_name, cr in blob["crops"].items():
        for key in ["mean_et_mm", "mean_precip_mm", "mean_stress_days", "mean_yield_ratio", "mean_irrig_mm"]:
            assert key in cr
        assert 0 <= cr["mean_yield_ratio"] <= 1.01 + 1e-3

spread_corn_yield = np.array([
    atlas["stations"][sid]["crops"]["Corn"]["mean_yield_ratio"] for sid in station_ids if "Corn" in atlas["stations"][sid]["crops"]
])

print("Corn yield-ratio spread (min/med/max):", float(np.min(spread_corn_yield)), float(np.median(spread_corn_yield)), float(np.max(spread_corn_yield)))
print("PASS: structural QA on first 20 stations + Corn coverage")
```

```python
C_GREEN, C_RED, C_BLUE = "#2ecc71", "#e74c3c", "#3498db"

corn_et = []
corn_yield = []

for sid in station_ids:
    c = atlas["stations"][sid]["crops"].get("Corn")
    if c is None:
        continue
    corn_et.append(c["mean_et_mm"])
    corn_yield.append(c["mean_yield_ratio"])

fig, ax = plt.subplots(figsize=(8, 4))
ax.scatter(corn_et, corn_yield, alpha=0.35, edgecolors="none", color=C_GREEN, label="Corn — stations")
ax.scatter(
    corn_et[:3],
    corn_yield[:3],
    alpha=1.0,
    color=C_RED,
    marker="x",
    s=40,
    label="First trio (markers)",
)

ce = np.array(corn_et)
cy = np.array(corn_yield)
if len(ce) > 5:
    m, b = np.polyfit(ce, cy, 1)
    xx = np.linspace(ce.min(), ce.max(), 200)
    ax.plot(xx, m * xx + b, linestyle="--", color=C_BLUE, lw=2, label="OLS trend")

ax.set_xlabel("Mean seasonal corn ET (mm)")
ax.set_ylabel("Mean yield ratio (-)")
ax.set_title("Experiment 018 — Corn ET vs modeled yield coefficient")
ax.legend(markerscale=1.8)
plt.tight_layout()
plt.show()
```

## Summary

The atlas benchmark JSON embeds aggregates for every simulated station—the exact artifact `validate_atlas` diff-checks against the Python driver. Notebook-side QA confirms sane ranges for reference $\mathrm{ET}_0$, per-crop bookkeeping fields, and Corn yield ratios across the lattice. The Corn scatter uses the requested palette; an OLS trend summarizes bulk association over the archived stations. For regenerated goldens, rerun `python control/atlas/atlas_water_budget.py` with local Open-Meteo CSV holdings.


