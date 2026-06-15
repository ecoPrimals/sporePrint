+++
title = "SCS Curve Number Runoff Method (USDA 1972)"
description = "Rendered from 050-scs-curve-number.ipynb"
date = 2026-06-15
weight = 50

[extra]
domain = "Lab"
rendered_from = "050-scs-curve-number.ipynb"
+++

<!-- Auto-generated from 050-scs-curve-number.ipynb by spore-validate render-notebooks -->

# SCS Curve Number Runoff Method (USDA 1972)

**Citation:** USDA–SCS (1972) *National Engineering Handbook*, Section 4: Hydrology.

**Primal:** `science.scs_cn_runoff` · **Rust:** `validate_scs_cn`

**Baseline:** `control/scs_curve_number/scs_curve_number.py` · **Benchmark:** `control/scs_curve_number/benchmark_scs_cn.json`


## Theory

Direct runoff depth $Q$ (mm) from rainfall $P$ (mm):

$$Q = \frac{(P - I_a)^2}{P - I_a + S} \quad \text{when } P > I_a, \quad \text{else } Q = 0$$

Potential maximum retention:  
$$S = \frac{25400}{\mathrm{CN}} - 254 \quad \text{(mm)}$$

Initial abstraction (standard): $I_a = 0.2\, S$. (Updated analyses sometimes use $\lambda = 0.05$.)


```python
import json
from pathlib import Path

import matplotlib.pyplot as plt
import numpy as np

REPO = Path('/home/eastgate/Development/ecoPrimals/springs/airSpring').resolve()
BENCH = REPO / "control/scs_curve_number/benchmark_scs_cn.json"

C_GREEN, C_RED, C_BLUE = "#2ecc71", "#e74c3c", "#3498db"
plt.rcParams.update({"figure.figsize": (8, 4.5), "axes.grid": True})
```

```python
def potential_retention(cn: float) -> float:
    if cn <= 0:
        return float("inf")
    return (25400.0 / cn) - 254.0


def initial_abstraction(s_mm: float, ia_ratio: float = 0.2) -> float:
    return ia_ratio * s_mm


def scs_cn_runoff(precip_mm: float, cn: float, ia_ratio: float = 0.2) -> float:
    if precip_mm <= 0 or cn <= 0:
        return 0.0
    s = potential_retention(cn)
    ia = initial_abstraction(s, ia_ratio)
    if precip_mm <= ia:
        return 0.0
    pe = precip_mm - ia
    return (pe * pe) / (pe + s)


def amc_cn_dry(cn_ii: float) -> float:
    return cn_ii / (2.281 - 0.01281 * cn_ii)


def amc_cn_wet(cn_ii: float) -> float:
    return cn_ii / (0.4036 + 0.0059 * cn_ii)
```

```python
with open(BENCH) as f:
    bench = json.load(f)

for case in bench["analytical_benchmarks"]:
    inp = case["inputs"]
    q = scs_cn_runoff(inp["precip_mm"], inp["cn"], inp.get("ia_ratio", 0.2))
    assert abs(q - case["expected_Q_mm"]) <= case["tolerance"], (case["name"], q)
    if "S_mm" in case:
        assert abs(potential_retention(inp["cn"]) - case["S_mm"]) <= 0.01

for tc in bench["amc_adjustment"]["test_cases"]:
    assert abs(amc_cn_dry(tc["cn_ii"]) - tc["expected_cn_i"]) <= tc["tolerance"]
    assert abs(amc_cn_wet(tc["cn_ii"]) - tc["expected_cn_iii"]) <= tc["tolerance"]

cns = [30, 50, 65, 75, 85, 90, 95, 98]
assert all(scs_cn_runoff(50.0, cns[i]) <= scs_cn_runoff(50.0, cns[i + 1]) for i in range(len(cns) - 1))
ps = [0, 10, 20, 30, 50, 75, 100, 150]
assert all(scs_cn_runoff(ps[i], 75) <= scs_cn_runoff(ps[i + 1], 75) for i in range(len(ps) - 1))
assert scs_cn_runoff(50.0, 75, 0.05) > scs_cn_runoff(50.0, 75, 0.2)

print("benchmark_scs_cn.json analytical + AMC + monotonicity checks passed.")
```

```python
P = np.linspace(0, 120, 200)
fig, ax = plt.subplots()
for cn, color, label in [(50, C_GREEN, "CN=50"), (75, C_BLUE, "CN=75"), (90, C_RED, "CN=90")]:
    Q = [scs_cn_runoff(p, cn) for p in P]
    ax.plot(P, Q, color=color, lw=2, label=label)
ax.set_xlabel("Precipitation $P$ (mm)")
ax.set_ylabel("Runoff $Q$ (mm)")
ax.set_title("SCS curve-number runoff ($I_a=0.2S$)")
ax.legend()
plt.tight_layout()
plt.show()
```

## Summary

- **Equations:** Standard NEH Chapter 4 formulation; optional AMC-I/III adjustments per Hawkins (1985) as in benchmark.  
- **Validation:** All analytical cases and CN ordering checks match `benchmark_scs_cn.json`.  
- **Provenance:** JSON `_provenance` cites TR-55 and review literature.


