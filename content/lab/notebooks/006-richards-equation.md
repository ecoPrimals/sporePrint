+++
title = "1D Richards Equation with van Genuchten–Mualem Hydraulics"
description = "Rendered from 006-richards-equation.ipynb"
date = 2026-06-15
weight = 50

[extra]
domain = "Lab"
rendered_from = "006-richards-equation.ipynb"
+++

<!-- Auto-generated from 006-richards-equation.ipynb by spore-validate render-notebooks -->

# 1D Richards Equation with van Genuchten–Mualem Hydraulics

**Citations:** Richards LA (1931) *Physics* **1**:318–333; van Genuchten MT (1980) *SSSA J* **44**:892–898.

**Primal:** `science.richards_1d` · **Rust:** `validate_richards`

**Baseline:** `control/richards/richards_1d.py` · **Benchmark:** `control/richards/benchmark_richards.json`


## Theory

**Richards equation** (1D, vertical, $z$ positive downward):

$$\frac{\partial \theta}{\partial t} = \frac{\partial}{\partial z}\left[ K(h)\left(\frac{\partial h}{\partial z} + 1\right) \right]$$

**van Genuchten retention** (1980, Eq. 1), $h < 0$:

$$\theta(h) = \theta_r + \frac{\theta_s - \theta_r}{\left[1 + (\alpha |h|)^n\right]^m}, \quad m = 1 - \frac{1}{n}$$

**Mualem–van Genuchten conductivity** (1980, Eq. 9):

$$K(h) = K_s \sqrt{S_e}\,\left[1 - \left(1 - S_e^{1/m}\right)^m\right]^2$$

**Method of lines:** finite-difference fluxes in $z$, state $h$ at cell centers; $\partial h/\partial t = (\partial\theta/\partial t)/C(h)$ with $C = \mathrm{d}\theta/\mathrm{d}h$; time integration with `scipy.integrate.solve_ivp` (see baseline for BCs: Dirichlet top for infiltration; zero flux top + free drainage bottom for drainage helper).


```python
import json
import warnings
from pathlib import Path

import matplotlib.pyplot as plt
import numpy as np
from scipy.integrate import solve_ivp

warnings.filterwarnings("ignore", category=RuntimeWarning, module="scipy")

REPO = Path('/home/eastgate/Development/ecoPrimals/springs/airSpring').resolve()
BENCH = REPO / "control/richards/benchmark_richards.json"

C_GREEN, C_RED, C_BLUE = "#2ecc71", "#e74c3c", "#3498db"
plt.rcParams.update({"figure.figsize": (8, 4.5), "axes.grid": True})
```

```python
# Core hydraulics + MOL solver (from control/richards/richards_1d.py)


def van_genuchten_theta(h, theta_r, theta_s, alpha, n):
    if h >= 0:
        return theta_s
    h_safe = min(abs(h), 1e4)
    m = 1.0 - 1.0 / n
    x = (alpha * h_safe) ** n
    x = min(x, 1e10)
    se = 1.0 / (1.0 + x) ** m
    theta = theta_r + (theta_s - theta_r) * se
    return float(np.clip(theta, theta_r, theta_s))


def van_genuchten_K(h, Ks, theta_r, theta_s, alpha, n):
    if h >= 0:
        return Ks
    if h < -1e4:
        return 0.0
    m = 1.0 - 1.0 / n
    theta = van_genuchten_theta(h, theta_r, theta_s, alpha, n)
    se = (theta - theta_r) / (theta_s - theta_r)
    if se <= 0:
        return 0.0
    if se >= 1:
        return Ks
    term = 1.0 - se ** (1.0 / m)
    if term <= 0:
        return Ks
    kr = np.sqrt(se) * (1.0 - term**m) ** 2
    return float(Ks * np.clip(kr, 0.0, 1.0))


def dtheta_dh(h, theta_r, theta_s, alpha, n):
    if h >= 0:
        return 1e-6
    h_safe = max(abs(h), 0.1)
    h_safe = min(h_safe, 1e4)
    m = 1.0 - 1.0 / n
    x = (alpha * h_safe) ** n
    x = min(x, 1e10)
    denom = (1.0 + x) ** (m + 1)
    if denom <= 0 or not np.isfinite(denom):
        return 1e-6
    dse_dh = m * n * (alpha**n) * (h_safe ** (n - 1)) / denom
    result = (theta_s - theta_r) * dse_dh
    return float(np.clip(result, 1e-10, 1e2))


def _richards_rhs(t, h_vec, params):
    dz = params["dz"]
    n = params["n_nodes"]
    theta_r = params["theta_r"]
    theta_s = params["theta_s"]
    alpha = params["alpha"]
    n_vg = params["n_vg"]
    Ks = params["Ks_cm_day"]
    h = np.clip(np.asarray(h_vec).flatten(), -1e3, 50.0)
    K = np.array(
        [van_genuchten_K(h[i], Ks, theta_r, theta_s, alpha, n_vg) for i in range(n)]
    )
    C = np.array(
        [dtheta_dh(h[i], theta_r, theta_s, alpha, n_vg) for i in range(n)]
    )
    q = np.zeros(n + 1)
    h_top = params.get("h_top", 0.0)
    K_top = van_genuchten_K(h_top, Ks, theta_r, theta_s, alpha, n_vg)
    q[0] = K_top * ((h_top - h[0]) / (0.5 * dz) + 1.0)
    for i in range(n - 1):
        K_mid = 0.5 * (K[i] + K[i + 1])
        q[i + 1] = K_mid * ((h[i + 1] - h[i]) / dz + 1.0)
    q[n] = K[n - 1]
    dtheta_dt = (q[:-1] - q[1:]) / dz
    C_safe = np.maximum(C, 1e-10)
    dh_dt = np.where(np.isfinite(dtheta_dt / C_safe), dtheta_dt / C_safe, 0.0)
    return dh_dt


def solve_richards_1d(params, h_initial, h_top, duration_hours, n_nodes=50, t_eval=None):
    duration_days = duration_hours / 24.0
    dz = params["column_depth_cm"] / n_nodes
    params = dict(params)
    params["dz"] = dz
    params["n_nodes"] = n_nodes
    params["h_top"] = h_top
    h0 = np.full(n_nodes, h_initial)
    t_span = (0.0, duration_days)
    if t_eval is None:
        n_t = min(100, max(1, int(duration_hours) + 1))
        t_eval = np.linspace(0, duration_days, n_t)
    else:
        t_eval = np.asarray(t_eval) / 24.0
    sol = solve_ivp(
        _richards_rhs,
        t_span,
        h0,
        method="LSODA",
        t_eval=t_eval,
        args=(params,),
        atol=1e-4,
        rtol=1e-2,
        max_step=min(duration_days / 3, 0.002),
    )
    if not sol.success:
        raise RuntimeError(sol.message)
    t_days = sol.t
    t_hours = t_days * 24.0
    h = sol.y
    theta = np.zeros_like(h)
    for i in range(h.shape[0]):
        for j in range(h.shape[1]):
            theta[i, j] = van_genuchten_theta(
                h[i, j], params["theta_r"], params["theta_s"], params["alpha"], params["n_vg"]
            )
    z = np.linspace(dz / 2, params["column_depth_cm"] - dz / 2, n_nodes)
    return {"t": t_hours, "h": h, "theta": theta, "z": z, "params": params, "dz": dz}
```

```python
with open(BENCH) as f:
    bench = json.load(f)

soils = bench["soil_types"]
checks = bench["validation_checks"]

# Retention tests
for tc in checks["van_genuchten_retention"]["test_cases"]:
    s = soils[tc["soil"]]
    th = van_genuchten_theta(tc["h_cm"], s["theta_r"], s["theta_s"], s["alpha"], s["n_vg"])
    assert abs(th - tc["expected_theta"]) <= tc["tolerance"], (tc, th)

# K tests
for tc in checks["hydraulic_conductivity"]["test_cases"]:
    s = soils[tc["soil"]]
    K = van_genuchten_K(tc["h_cm"], s["Ks_cm_day"], s["theta_r"], s["theta_s"], s["alpha"], s["n_vg"])
    if "expected_K_ratio" in tc:
        assert abs(K / s["Ks_cm_day"] - tc["expected_K_ratio"]) <= tc["tolerance"]
    else:
        r = K / s["Ks_cm_day"]
        lo, hi = tc["expected_K_ratio_range"]
        assert lo <= r <= hi, r

# Infiltration sand
cfg = checks["infiltration_sand"]
s = soils["sand"]
params = {
    "theta_r": s["theta_r"],
    "theta_s": s["theta_s"],
    "alpha": s["alpha"],
    "n_vg": s["n_vg"],
    "Ks_cm_day": s["Ks_cm_day"],
    "column_depth_cm": cfg["column_depth_cm"],
}
sol = solve_richards_1d(
    params,
    h_initial=cfg["initial_h_cm"],
    h_top=cfg["top_h_cm"],
    duration_hours=cfg["duration_hours"],
    n_nodes=25,
)
assert sol["theta"][0, -1] >= cfg["checks"][1]["min_theta"]

print("benchmark_richards.json: retention, K, and sand infiltration checks passed.")
```

```python
# Visualization: retention curves for benchmark soils
h_grid = np.linspace(-200, 0, 200)
fig, ax = plt.subplots()
for soil_name, color in zip(soils.keys(), [C_GREEN, C_BLUE, C_RED]):
    s = soils[soil_name]
    th = [van_genuchten_theta(h, s["theta_r"], s["theta_s"], s["alpha"], s["n_vg"]) for h in h_grid]
    ax.plot(h_grid, th, label=soil_name.replace("_", " "), color=color, lw=2)
ax.set_xlabel("Pressure head $h$ (cm)")
ax.set_ylabel(r"$\theta(h)$")
ax.set_title("van Genuchten retention (Carsel & Parrish–class parameters in benchmark)")
ax.legend()
plt.tight_layout()
plt.show()

# Final moisture profile: sand infiltration
z = sol["z"]
th_prof = sol["theta"][:, -1]
fig2, ax2 = plt.subplots()
ax2.plot(th_prof, z, color=C_GREEN, lw=2)
ax2.set_xlabel(r"$\theta$")
ax2.set_ylabel("Depth $z$ (cm)")
ax2.invert_yaxis()
ax2.set_title("Sand column: final moisture profile after infiltration run")
plt.tight_layout()
plt.show()
```

## Summary

- **Governing equation:** 1D Richards with VG–Mualem; **solver:** method of lines + `solve_ivp`.  
- **Benchmark:** `benchmark_richards.json` supplies soil parameters (Carsel & Parrish, 1988) and analytical spot checks for $\theta(h)$ and $K(h)$, plus infiltration QA.  
- **Provenance:** See `_provenance` in JSON and module docstring in `richards_1d.py`.  
- **Note:** Drainage scenarios use `solve_richards_1d_drainage` in the full baseline; this notebook validates core hydraulics and infiltration.


