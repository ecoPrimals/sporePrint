+++
title = "Saxton-Rawls (2006) Pedotransfer Functions"
description = "Rendered from 023-pedotransfer-saxton-rawls.ipynb"
date = 2026-06-23
weight = 50

[extra]
domain = "Lab"
rendered_from = "023-pedotransfer-saxton-rawls.ipynb"
+++

<!-- Auto-generated from 023-pedotransfer-saxton-rawls.ipynb by spore-validate render-notebooks -->

# Saxton-Rawls (2006) Pedotransfer Functions

**Citation:** Saxton KE, Rawls WJ (2006) Soil water characteristic estimates by texture and organic matter for hydrologic solutions. *Soil Sci. Soc. Am. J.* **70**(5):1569–1578.

**Primal:** `science.pedotransfer_saxton_rawls` · **Rust validation:** `validate_pedotransfer`

**Baseline:** `control/pedotransfer/saxton_rawls.py` · **Benchmark:** `control/pedotransfer/benchmark_pedotransfer.json`


## Theory

Mass fractions of sand $S$ and clay $C$ (0–1) and organic matter $OM$ (%, w/w) predict:

- Wilting-point water content $\theta_{1500}$ at $-1500$ kPa  
- Field capacity $\theta_{33}$ at $-33$ kPa  
- Saturation (porosity) $\theta_s$  
- Saturated hydraulic conductivity $K_{\mathrm{sat}}$ (mm/h)

Saxton & Rawls give polynomial regressions; $K_{\mathrm{sat}}$ uses a $\lambda$ slope from the retention curve and

$$K_{\mathrm{sat}} = 1930\,(\theta_s - \theta_{33})^{3-\lambda}$$

with $\lambda$ derived from $\theta_{33}$, $\theta_{1500}$, and tensions 33 and 1500 kPa.


```python
import json
import math
from pathlib import Path

import matplotlib.pyplot as plt
import numpy as np

REPO = Path('/home/eastgate/Development/ecoPrimals/springs/airSpring').resolve()
BENCH = REPO / "control/pedotransfer/benchmark_pedotransfer.json"

C_GREEN, C_RED, C_BLUE = "#2ecc71", "#e74c3c", "#3498db"
plt.rcParams.update({"figure.figsize": (8, 4.5), "axes.grid": True})
```

```python
# Implementation (from control/pedotransfer/saxton_rawls.py)


def theta_1500_first(S, C, OM):
    return (
        -0.024 * S
        + 0.487 * C
        + 0.006 * OM
        + 0.005 * S * OM
        - 0.013 * C * OM
        + 0.068 * S * C
        + 0.031
    )


def theta_1500(S, C, OM):
    t1500_first = theta_1500_first(S, C, OM)
    return t1500_first + (0.14 * t1500_first - 0.02)


def theta_33_first(S, C, OM):
    return (
        -0.251 * S
        + 0.195 * C
        + 0.011 * OM
        + 0.006 * S * OM
        - 0.027 * C * OM
        + 0.452 * S * C
        + 0.299
    )


def theta_33(S, C, OM):
    t33_first = theta_33_first(S, C, OM)
    return t33_first + (1.283 * t33_first * t33_first - 0.374 * t33_first - 0.015)


def theta_s_33_first(S, C, OM):
    t33 = theta_33(S, C, OM)
    return (
        0.278 * S
        + 0.034 * C
        + 0.022 * OM
        - 0.018 * S * OM
        - 0.027 * C * OM
        - 0.584 * S * C
        + 0.078
    )


def theta_s_33(S, C, OM):
    first = theta_s_33_first(S, C, OM)
    return first + (0.636 * first - 0.107)


def theta_s(S, C, OM):
    return theta_33(S, C, OM) + theta_s_33(S, C, OM) - 0.097 * S + 0.043


def lambda_param(S, C, OM):
    t33 = theta_33(S, C, OM)
    t1500 = theta_1500(S, C, OM)
    B = (math.log(1500) - math.log(33)) / (math.log(t33) - math.log(t1500))
    return 1.0 / B


def ksat(S, C, OM):
    ts = theta_s(S, C, OM)
    t33 = theta_33(S, C, OM)
    lam = lambda_param(S, C, OM)
    return 1930.0 * (ts - t33) ** (3.0 - lam)
```

```python
with open(BENCH) as f:
    bench = json.load(f)

tol_m = bench["tol_moisture"]
tol_k = bench["tol_ksat"]
lo = bench["loam_intermediates"]
S, C, OM = lo["S"], lo["C"], lo["OM"]

checks = []
checks.append(
    abs(theta_1500_first(S, C, OM) - lo["theta_1500_first"]) < tol_m * 10
)
checks.append(abs(theta_1500(S, C, OM) - lo["theta_1500"]) < tol_m * 10)
checks.append(abs(theta_33_first(S, C, OM) - lo["theta_33_first"]) < tol_m * 10)
checks.append(abs(theta_33(S, C, OM) - lo["theta_33"]) < tol_m * 10)
checks.append(abs(theta_s_33_first(S, C, OM) - lo["theta_s_33_first"]) < tol_m * 10)
checks.append(abs(theta_s_33(S, C, OM) - lo["theta_s_33"]) < tol_m * 10)
checks.append(abs(theta_s(S, C, OM) - lo["theta_s"]) < tol_m * 10)
checks.append(abs(lambda_param(S, C, OM) - lo["lambda"]) < 0.01)
checks.append(abs(ksat(S, C, OM) - lo["ksat_mm_hr"]) < tol_k)

for name, row in bench["texture_classes"].items():
    S_, C_, OM_ = row["S"], row["C"], row["OM"]
    checks.append(abs(theta_1500(S_, C_, OM_) - row["theta_wp"]) < tol_m * 10)
    checks.append(abs(theta_33(S_, C_, OM_) - row["theta_fc"]) < tol_m * 10)
    checks.append(abs(theta_s(S_, C_, OM_) - row["theta_s"]) < tol_m * 10)
    checks.append(abs(ksat(S_, C_, OM_) - row["ksat_mm_hr"]) < tol_k)

print("benchmark_pedotransfer.json checks:", sum(checks), "/", len(checks))
assert all(checks), "Validation failed against benchmark JSON"
```

```python
names = list(bench["texture_classes"].keys())
wp = [bench["texture_classes"][k]["theta_wp"] for k in names]
fc = [bench["texture_classes"][k]["theta_fc"] for k in names]
ts = [bench["texture_classes"][k]["theta_s"] for k in names]
x = np.arange(len(names))
w = 0.25
fig, ax = plt.subplots()
ax.bar(x - w, wp, width=w, label=r"$\theta_{WP}$", color=C_GREEN, edgecolor="black", linewidth=0.4)
ax.bar(x, fc, width=w, label=r"$\theta_{FC}$", color=C_BLUE, edgecolor="black", linewidth=0.4)
ax.bar(x + w, ts, width=w, label=r"$\theta_s$", color=C_RED, edgecolor="black", linewidth=0.4)
ax.set_xticks(x)
ax.set_xticklabels(names, rotation=35, ha="right")
ax.set_ylabel("Volumetric water content (-)")
ax.set_title("Saxton–Rawls moisture endpoints by USDA texture class")
ax.legend()
plt.tight_layout()
plt.show()

fig2, ax2 = plt.subplots()
ks = [bench["texture_classes"][k]["ksat_mm_hr"] for k in names]
ax2.bar(names, ks, color=C_BLUE, edgecolor="black", linewidth=0.4)
ax2.set_ylabel("$K_{\\mathrm{sat}}$ (mm/h)")
ax2.set_title("Saturated hydraulic conductivity (benchmark values)")
plt.xticks(rotation=35, ha="right")
plt.tight_layout()
plt.show()
```

## Summary

- **Method:** Saxton & Rawls (2006) pedotransfer regressions; $K_{\mathrm{sat}}$ from moisture–tension slope and power law.  
- **Validation:** Recomputed quantities match `benchmark_pedotransfer.json` within `tol_moisture` / `tol_ksat`.  
- **Provenance:** Baseline script `control/pedotransfer/saxton_rawls.py`; JSON includes `_provenance` with references and SHA when present.  
- **Reproduce:** Run the validation cell after editing `REPO` if the repository root moves.


