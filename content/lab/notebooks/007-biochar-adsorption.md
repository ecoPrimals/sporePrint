+++
title = "Biochar Phosphorus Adsorption Isotherms (Kumari et al. 2025)"
description = "Rendered from 007-biochar-adsorption.ipynb"
date = 2026-06-02
weight = 50

[extra]
domain = "Lab"
rendered_from = "007-biochar-adsorption.ipynb"
+++

<!-- Auto-generated from 007-biochar-adsorption.ipynb by spore-validate render-notebooks -->

# Biochar Phosphorus Adsorption Isotherms (Kumari et al. 2025)

**Citation:** Kumari S, Dong Y, Safferman S (2025) Phosphorus adsorption and recovery from waste streams using biochar. *Appl. Water Sci.* **15**(7).

**Primal:** N/A (equilibrium isotherms)

**Baseline:** `control/biochar/biochar_isotherms.py` · **Benchmark:** `control/biochar/benchmark_biochar.json`


## Theory

Equilibrium absorbed concentration $q_e$ (mg/g) vs. equilibrium concentration $C_e$ (mg/L):

**Langmuir:**  
$$q_e = \frac{q_m K_L C_e}{1 + K_L C_e}$$

**Freundlich:**  
$$q_e = K_F\, C_e^{1/n}$$

Separation factor (Langmuir): $R_L = 1/(1 + K_L C_0)$; favorable adsorption often has $0 < R_L < 1$.


```python
import json
from pathlib import Path

import matplotlib.pyplot as plt
import numpy as np
from scipy.optimize import curve_fit

REPO = Path('/home/eastgate/Development/ecoPrimals/springs/airSpring').resolve()
BENCH = REPO / "control/biochar/benchmark_biochar.json"

C_GREEN, C_RED, C_BLUE = "#2ecc71", "#e74c3c", "#3498db"
plt.rcParams.update({"figure.figsize": (8, 4.5), "axes.grid": True})
```

```python
def langmuir(Ce, qmax, KL):
    return qmax * KL * Ce / (1.0 + KL * Ce)


def freundlich(Ce, KF, n):
    Ce_safe = np.maximum(Ce, 1e-10)
    return KF * np.power(Ce_safe, 1.0 / n)


def compute_r2(measured, predicted):
    ss_res = np.sum((measured - predicted) ** 2)
    ss_tot = np.sum((measured - np.mean(measured)) ** 2)
    if ss_tot == 0:
        return 1.0
    return 1.0 - ss_res / ss_tot


def separation_factor_RL(KL, C0):
    return 1.0 / (1.0 + KL * C0)


def fit_langmuir(Ce, qe):
    p0 = [np.max(qe) * 1.2, 0.1]
    popt, _ = curve_fit(langmuir, Ce, qe, p0=p0, bounds=(0, np.inf))
    qmax, KL = popt[0], popt[1]
    pred = langmuir(Ce, qmax, KL)
    return qmax, KL, pred, compute_r2(qe, pred)


def fit_freundlich(Ce, qe):
    p0 = [1.0, 2.0]
    popt, _ = curve_fit(freundlich, Ce, qe, p0=p0, bounds=([1e-10, 0.1], np.inf))
    KF, n = popt[0], popt[1]
    pred = freundlich(Ce, KF, n)
    return KF, n, pred, compute_r2(qe, pred)
```

```python
with open(BENCH) as f:
    bench = json.load(f)

validation = bench["validation_checks"]
datasets = bench["isotherm_data"]["datasets"]

results = {}
for ds_name, ds in datasets.items():
    Ce = np.array(ds["Ce"], dtype=float)
    qe = np.array(ds["qe"], dtype=float)
    qmax, KL, pred_l, r2l = fit_langmuir(Ce, qe)
    KF, n, pred_f, r2f = fit_freundlich(Ce, qe)
    results[ds_name] = dict(
        qmax=qmax, KL=KL, r2l=r2l, KF=KF, n=n, r2f=r2f, Ce=Ce, qe=qe, pred_l=pred_l, pred_f=pred_f
    )


def run_langmuir_checks():
    for c in validation["langmuir_fit"]["checks"]:
        cid = c["id"]
        if cid == "wood_qmax_range":
            q = results["wood_biochar_500C"]["qmax"]
            assert c["min"] <= q <= c["max"], q
        elif cid == "wood_KL_positive":
            assert results["wood_biochar_500C"]["KL"] > 0
        elif cid == "wood_r2":
            assert results["wood_biochar_500C"]["r2l"] >= c["min_r2"]
        elif cid == "sugar_qmax_range":
            q = results["sugar_beet_biochar"]["qmax"]
            assert c["min"] <= q <= c["max"], q
        elif cid == "sugar_r2":
            assert results["sugar_beet_biochar"]["r2l"] >= c["min_r2"]


def run_freundlich_checks():
    for c in validation["freundlich_fit"]["checks"]:
        cid = c["id"]
        if cid == "wood_KF_positive":
            assert results["wood_biochar_500C"]["KF"] > 0
        elif cid == "wood_n_favorable":
            assert results["wood_biochar_500C"]["n"] >= c["min_n"]
        elif cid == "wood_r2":
            assert results["wood_biochar_500C"]["r2f"] >= c["min_r2"]
        elif cid == "sugar_KF_positive":
            assert results["sugar_beet_biochar"]["KF"] > 0
        elif cid == "sugar_n_range":
            n = results["sugar_beet_biochar"]["n"]
            assert c["min"] <= n <= c["max"]


run_langmuir_checks()
run_freundlich_checks()

# Model comparison + RL
wood = results["wood_biochar_500C"]
assert wood["r2l"] >= wood["r2f"]
for ds_name, r in results.items():
    assert r["qmax"] > 0 and r["KL"] > 0 and r["KF"] > 0 and r["n"] > 0
    assert abs(np.mean(r["qe"] - r["pred_l"])) < 0.5
    assert abs(np.mean(r["qe"] - r["pred_f"])) < 0.5
    RL = separation_factor_RL(r["KL"], 100.0)
    assert 0 < RL < 1

print("benchmark_biochar.json validation checks passed.")
```

```python
fig, axes = plt.subplots(1, 2, figsize=(10, 4))
for ax, (name, color) in zip(
    axes,
    [
        ("wood_biochar_500C", C_GREEN),
        ("sugar_beet_biochar", C_BLUE),
    ],
):
    r = results[name]
    Ce_fine = np.linspace(r["Ce"].min(), r["Ce"].max(), 100)
    ax.scatter(r["Ce"], r["qe"], color=C_RED, s=40, label="Data", zorder=3)
    ax.plot(Ce_fine, langmuir(Ce_fine, r["qmax"], r["KL"]), color=color, lw=2, label="Langmuir")
    ax.plot(Ce_fine, freundlich(Ce_fine, r["KF"], r["n"]), color=color, lw=2, ls="--", label="Freundlich")
    ax.set_xlabel("$C_e$ (mg/L)")
    ax.set_ylabel("$q_e$ (mg/g)")
    ax.set_title(name.replace("_", " "))
    ax.legend()
plt.tight_layout()
plt.show()
```

## Summary

- **Models:** Langmuir and Freundlich with nonlinear least squares (`scipy.optimize.curve_fit`).  
- **Data & QA:** Digitized representative points and thresholds in `benchmark_biochar.json` (R², $q_m$ ranges, $R_L$).  
- **Provenance:** JSON `_provenance` lists Kumari et al. (2025) and classical isotherm references.


