+++
title = "Soil Moisture Sensor Calibration (Dong et al., 2020)"
description = "Rendered from 002-soil-sensor-calibration.ipynb"
date = 2026-06-04
weight = 50

[extra]
domain = "Lab"
rendered_from = "002-soil-sensor-calibration.ipynb"
+++

<!-- Auto-generated from 002-soil-sensor-calibration.ipynb by spore-validate render-notebooks -->

# Soil Moisture Sensor Calibration (Dong et al.\ 2020)

**Citation:** Dong, Y., Miller, W.\ M., Kelley, K.\ M.\ (2020). *Agriculture* **10**(12):598. doi:10.3390/agriculture10120598

**AirSpring linkage:** primal `science.sensor_calibration`; Rust binary `validate_soil_sensors`.

Source baseline: `control/soil_sensors/calibration_dong2020.py`; benchmark digits: `control/soil_sensors/benchmark_dong2020.json`.


## Theory

Industry calibrations often begin from apparent permittivity $\varepsilon$ (FDR/TDR proxy). The Topp et al.\ (1980) cubic maps dielectric permittivity to volumetric water content $\theta$ (m$^3$ m$^{-3}$):

$$\theta(\varepsilon) = -5.3\times 10^{-2} + 2.92\times 10^{-2}\,\varepsilon - 5.5\times 10^{-4}\,\varepsilon^2 + 4.3\times 10^{-6}\,\varepsilon^3$$

Dong et al.\ (2020) evaluate RMSE, index-of-agreement, and bias between factory-calibrated probes and volumetric gravimetrics, then refine fits with linear/non-linear correction models (their Table 4), matching the scaffolding in `calibration_dong2020.py`.


```python
import importlib.util
import json
from pathlib import Path

import matplotlib.pyplot as plt
import numpy as np

NOTEBOOK_DIR = Path.cwd().resolve()
BENCH_PATH = (NOTEBOOK_DIR / "../../control/soil_sensors/benchmark_dong2020.json").resolve()
CONTROL_PATH = (NOTEBOOK_DIR / "../../control/soil_sensors/calibration_dong2020.py").resolve()

spec = importlib.util.spec_from_file_location("cal", CONTROL_PATH)
cal = importlib.util.module_from_spec(spec)
spec.loader.exec_module(cal)

with open(BENCH_PATH, encoding="utf-8") as f:
    dg = json.load(f)

criteria = dg["statistical_formulas"]["criteria"]
print("Loaded:", BENCH_PATH)
print("MBE thresh ±", criteria["mbe_threshold"], "| RMSE thresh <", criteria["rmse_threshold"])
```

```python
# Topp + statistics mirrored in calibration_dong2020.py

epsilons = np.linspace(3, 40, 200)
theta_line = np.array([cal.topp_equation(float(e)) for e in epsilons])

coeff = dg["topp_equation"]["coefficients"]
print(" coeffs :", coeff)

pairs = dg["table_3_factory_calibration"]

def flat_rmse(sensor, soil_key):
    return pairs[sensor][soil_key]["rmse"]

print(" Example CS616/sand RMSE:", flat_rmse("cs616", "sand"))
```

```python
topp_block = dg["topp_equation"]
_tol = topp_block["tolerance"]

for pt in topp_block["published_points"]:
    th = cal.topp_equation(pt["epsilon"])
    assert abs(th - pt["theta_expected"]) <= _tol

meas = np.array([0.10, 0.15, 0.20, 0.25, 0.30])
pred = meas.copy()

assert abs(cal.compute_rmse(meas, pred)) < 1e-12
assert abs(cal.compute_ia(meas, pred) - 1.0) < 1e-12
assert abs(cal.compute_mbe(meas, pred)) < 1e-12

mbe_thresh = dg["statistical_formulas"]["criteria"]["mbe_threshold"]

for sensor_name, soils in dg["table_3_factory_calibration"].items():
    if sensor_name.startswith("_"):
        continue
    for soil_name, stats in soils.items():
        mbe_ok = abs(stats["mbe"]) <= mbe_thresh
        rmse_ok = stats["rmse"] < dg["statistical_formulas"]["criteria"]["rmse_threshold"]

        # Paper logic encoded in calibration_dong2020.py tests
        if sensor_name == "cs616" and soil_name == "sand":
            assert mbe_ok and rmse_ok
        else:
            assert (not mbe_ok) or (not rmse_ok)

field = dg["field_validation_rmse"]

for sensor_name, soils in field.items():
    if sensor_name.startswith("_"):
        continue
    for soil_name, data in soils.items():
        assert data["corrected"] < data["factory"]

print("PASS: Topp table, statistics identities, criteria rules, RMSE reductions")
```

```python
C_GREEN, C_RED, C_BLUE = "#2ecc71", "#e74c3c", "#3498db"

eps_pub = np.array([pt["epsilon"] for pt in topp_block["published_points"]])
theta_pub_meas = np.array([pt["theta_expected"] for pt in topp_block["published_points"]])
theta_pub_pred = np.array([cal.topp_equation(e) for e in eps_pub])

fig, ax = plt.subplots(figsize=(7, 4))

ax.plot(epsilons, theta_line, color=C_BLUE, label="Topp curve (dense grid)")
ax.scatter(eps_pub, theta_pub_meas, color=C_RED, s=60, zorder=3, label="Published θ (benchmark)")
ax.scatter(eps_pub, theta_pub_pred, facecolors="none", edgecolors=C_GREEN, s=80, lw=2, label="Notebook eval")

for lo, hi, col in [(0.10, 0.30, "#cccccc")]:
    ax.axhspan(lo, hi, color=col, alpha=0.08)

ax.set_xlabel("Dielectric ε (-)")
ax.set_ylabel("Volumetric water content θ (-)")
ax.set_title("Topp (1980) equation vs Dong benchmark anchors")
ax.legend(ncol=2, fontsize="small")
plt.tight_layout()
plt.show()
```

## Summary

We bound the Dong et al.\ digitized QA around the Topp dielectric calibration, analytic statistics checks (perfect/bias RMSE behaviors), categorical pass/fail gating mirrored from Table 3, and monotonic corrections where field-derived RMSE always improves—precisely aligning with how `benchmark_dong2020.json` informs `validate_soil_sensors` regressions alongside `calibration_dong2020.py`.


