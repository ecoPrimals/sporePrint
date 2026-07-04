+++
title = "Green-Ampt (1911) Infiltration Model"
description = "Rendered from 051-green-ampt-infiltration.ipynb"
date = 2026-07-04
weight = 50

[extra]
domain = "Lab"
rendered_from = "051-green-ampt-infiltration.ipynb"
+++

<!-- Auto-generated from 051-green-ampt-infiltration.ipynb by spore-validate render-notebooks -->

# Green-Ampt (1911) Infiltration Model

**Citations:** Green WH, Ampt GA (1911) *J. Agric. Sci.* **4**:1–24; Rawls WJ, Brakensiek DL, Miller N (1983) *J. Hydraul. Eng.* **109**:62–70.

**Primal:** `science.green_ampt_infiltration` · **Rust:** `validate_green_ampt`

**Baseline:** `control/green_ampt/green_ampt_infiltration.py` · **Benchmark:** `control/green_ampt/benchmark_green_ampt.json`


## Theory

Cumulative infiltration $F(t)$ satisfies the implicit Green–Ampt equation:

$$F = K_s t + \psi \Delta\theta \ln\left(1 + \frac{F}{\psi \Delta\theta}\right)$$

Infiltration rate from $F$:

$$f = K_s \left(1 + \frac{\psi \Delta\theta}{F}\right)$$

Time to ponding under constant intensity $i$ (when $i > K_s$):

$$t_p = \frac{K_s \psi \Delta\theta}{i\,(i - K_s)}$$


```python
import json
import math
from pathlib import Path

import matplotlib.pyplot as plt
import numpy as np

REPO = Path('/home/eastgate/Development/ecoPrimals/springs/airSpring').resolve()
BENCH = REPO / "control/green_ampt/benchmark_green_ampt.json"

C_GREEN, C_RED, C_BLUE = "#2ecc71", "#e74c3c", "#3498db"
plt.rcParams.update({"figure.figsize": (8, 4.5), "axes.grid": True})
```

```python
def green_ampt_cumulative(ks, psi, delta_theta, t_hr, max_iter=100, tol=1e-8):
    if t_hr <= 0:
        return 0.0
    psi_dt = psi * delta_theta
    f_guess = ks * t_hr + math.sqrt(2.0 * ks * psi_dt * t_hr)
    for _ in range(max_iter):
        if f_guess <= 0:
            f_guess = ks * t_hr * 0.01
        g = f_guess - ks * t_hr - psi_dt * math.log(1.0 + f_guess / psi_dt)
        dg = 1.0 - psi_dt / (psi_dt + f_guess)
        if abs(dg) < 1e-15:
            break
        f_new = f_guess - g / dg
        if f_new < 0:
            f_new = f_guess * 0.5
        if abs(f_new - f_guess) < tol:
            f_guess = f_new
            break
        f_guess = f_new
    return max(0.0, f_guess)


def green_ampt_rate(ks, psi, delta_theta, f_cum):
    if f_cum <= 0:
        return float("inf")
    return ks * (1.0 + psi * delta_theta / f_cum)


def ponding_time(ks, psi, delta_theta, rain_intensity):
    if rain_intensity <= ks:
        return float("inf")
    return ks * psi * delta_theta / (rain_intensity * (rain_intensity - ks))
```

```python
with open(BENCH) as f:
    bench = json.load(f)

for case in bench["analytical_benchmarks"]:
    inp = case["inputs"]
    ks, psi, dt = inp["Ks_cm_hr"], inp["psi_cm"], inp["delta_theta"]
    if "t_hr" in inp:
        t = inp["t_hr"]
        if "expected_F_cm" in case and not case.get("expected_f_infinite", False):
            F = green_ampt_cumulative(ks, psi, dt, t)
            assert abs(F - case["expected_F_cm"]) <= case["tolerance"], (case["name"], F)
        if "expected_f_cm_hr" in case:
            F = green_ampt_cumulative(ks, psi, dt, t)
            fr = green_ampt_rate(ks, psi, dt, F)
            assert abs(fr - case["expected_f_cm_hr"]) <= case["tolerance"], (case["name"], fr)
        if case.get("expected_f_approaches_Ks", False):
            F = green_ampt_cumulative(ks, psi, dt, t)
            r = green_ampt_rate(ks, psi, dt, F) / ks
            assert abs(r - 1.0) <= case.get("tolerance_ratio", 0.05)
    if "rain_intensity_cm_hr" in inp:
        tp = ponding_time(ks, psi, dt, inp["rain_intensity_cm_hr"])
        assert abs(tp - case["expected_tp_hr"]) <= case["tolerance"]

print("benchmark_green_ampt.json analytical cases passed.")
```

```python
soils = bench["soil_parameters"]
# skip description key
soil_names = [k for k, v in soils.items() if isinstance(v, dict)]
Ks = [soils[k]["Ks_cm_hr"] for k in soil_names]

tvec = np.linspace(0.01, 8.0, 100)
fig, ax = plt.subplots()
for name, color in [("sand", C_GREEN), ("sandy_loam", C_BLUE), ("clay", C_RED)]:
    p = soils[name]
    F = [green_ampt_cumulative(p["Ks_cm_hr"], p["psi_cm"], 0.35, t) for t in tvec]
    ax.plot(tvec, F, lw=2, color=color, label=name)
ax.set_xlabel("Time (hr)")
ax.set_ylabel("Cumulative $F$ (cm)")
ax.set_title("Green–Ampt cumulative infiltration (Δθ ≈ 0.35, illustrative)")
ax.legend()
plt.tight_layout()
plt.show()

fig2, ax2 = plt.subplots()
ax2.bar(soil_names, Ks, color=C_BLUE, edgecolor="black", linewidth=0.4)
ax2.set_ylabel("$K_s$ (cm/hr)")
ax2.set_title("Saturated conductivity (Rawls et al. 1983 table in benchmark)")
plt.xticks(rotation=30, ha="right")
plt.tight_layout()
plt.show()
```

## Summary

- **Numerics:** Newton–Raphson on the implicit $F$ equation matches benchmark tolerances.  
- **Parameters:** Soil catalog from Rawls et al. (1983) embedded in JSON.  
- **Provenance:** See `_provenance` in `benchmark_green_ampt.json`.


