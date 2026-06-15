+++
title = "FAO-56 Chapter 7 — Dual Crop Coefficient ($K_{cb} + K_e$)"
description = "Rendered from 009-fao56-dual-kc.ipynb"
date = 2026-06-15
weight = 50

[extra]
domain = "Lab"
rendered_from = "009-fao56-dual-kc.ipynb"
+++

<!-- Auto-generated from 009-fao56-dual-kc.ipynb by spore-validate render-notebooks -->

# FAO-56 Chapter 7 — Dual Crop Coefficient ($K_{cb} + K_e$)

**Citation:** Allen, R.G., Pereira, L.S., Raes, D., Smith, M. (1998). *FAO Irrigation and Drainage Paper 56,* Chapter 7.

**AirSpring linkage:** primal `science.dual_kc`; Rust binary `validate_dual_kc`.

Source: `control/dual_kc/dual_crop_coefficient.py`; benchmark: `control/dual_kc/benchmark_dual_kc.json`.


## Theory

Separate basal transpiration ($K_{cb}$) from soil evaporation ($K_e$). FAO-56 Eq.\ 69:

$$ET_\mathrm{c} = (K_{cb} \, K_\mathrm{s} + K_e) \, ET_0$$

Supporting relations include $K_{\mathrm{c,max}}$ limits, total evaporable water $TEW$, evaporation reduction $K_\mathrm{r}$, and the daily evaporating-layer balance linking $K_e$, $K_{\mathrm{c,max}}$, and wetted fraction $f_\mathrm{ew}$ (Eqs.\ 71--77 in Allen et al., 1998).


```python
import importlib.util
import json
from pathlib import Path

import matplotlib.pyplot as plt
import numpy as np

NOTEBOOK_DIR = Path.cwd().resolve()
BENCH_PATH = (NOTEBOOK_DIR / "../../control/dual_kc/benchmark_dual_kc.json").resolve()
CONTROL_PATH = (NOTEBOOK_DIR / "../../control/dual_kc/dual_crop_coefficient.py").resolve()

spec = importlib.util.spec_from_file_location("dkc", CONTROL_PATH)
dkc = importlib.util.module_from_spec(spec)
spec.loader.exec_module(dkc)

with open(BENCH_PATH, encoding="utf-8") as f:
    dk_bench = json.load(f)

print("Benchmark:", BENCH_PATH)
```

```python
# Dual Kc building blocks mirror dual_crop_coefficient.py

summaries = []

for tc in dk_bench["equations"]["eq_69"]["test_cases"]:
    y = dkc.etc_dual(tc["kcb"], tc["ks"], tc["ke"], tc["et0"])
    summaries.append((tc["label"], y, tc["expected_etc"]))

for label, y, exp in summaries:
    assert abs(y - exp) <= 1e-6

kcmax_demo = [dkc.kc_max(2.0, 45.0, 2.0, 1.15), dkc.total_evaporable_water(0.23, 0.1, 0.1)]
bare = dk_bench["validation_scenarios"]["bare_soil_drydown"]

sim = dkc.simulate_dual_kc(
    et0_daily=bare["et0_daily"],
    precip_daily=bare["precip_daily"],
    kcb=bare["kcb"],
    kc_max_val=bare["kc_max"],
    few=bare["few"],
    tew=bare["tew"],
    rew=bare["rew"],
)
print("Eq.69 checks OK;", "Kc_max std climate:", round(kcmax_demo[0], 4))
print("TEW sandy_loam ze=0.1:", kcmax_demo[1])
print("Bare soil simulation total ETc:", sim["total_etc"])
assert sim["kr"][0] == 1.0
assert sim["total_etc"] > 0
assert sim["final_de"] <= bare["tew"]
```

```python
# Targeted validators from benchmark JSON sections

_tol = dk_bench["equations"]["eq_69"]["tolerance"]

def check_eq_cases():
    for tc in dk_bench["equations"]["eq_69"]["test_cases"]:
        v = dkc.etc_dual(tc["kcb"], tc["ks"], tc["ke"], tc["et0"])
        assert abs(v - tc["expected_etc"]) <= _tol
    for tc in dk_bench["equations"]["eq_71_kc_max"]["test_cases"]:
        v = dkc.kc_max(tc["u2"], tc["rh_min"], tc["h"], tc["kcb"])
        assert abs(v - tc["expected_kc_max"]) <= 1e-4
    for tc in dk_bench["equations"]["eq_73_tew"]["test_cases"]:
        v = dkc.total_evaporable_water(tc["theta_fc"], tc["theta_wp"], tc["ze_m"])
        assert abs(v - tc["expected_tew"]) <= _tol
    for tc in dk_bench["equations"]["eq_72_kr"]["test_cases"]:
        v = dkc.evaporation_reduction(tc["tew"], tc["rew"], tc["de"])
        assert abs(v - tc["expected_kr"]) <= _tol

check_eq_cases()

kcb_tab = dk_bench["table_17_kcb"]["crops"]
kc_tab = dk_bench["table_12_kc_single"]["crops"]
for crop_name, row in kcb_tab.items():
    if crop_name in kc_tab:
        diff_mid = kc_tab[crop_name]["kc_mid"] - row["kcb_mid"]
        assert 0.0 <= diff_mid <= 0.20

soils19 = dk_bench["table_19_rew"]["soils"]
for sname, s in soils19.items():
    TEW_mm = dkc.total_evaporable_water(s["theta_fc"], s["theta_wp"], 0.10)
    assert TEW_mm > s["rew_mm"]

corn_sc = dk_bench["validation_scenarios"]["corn_mid_season"]
sim_corn = dkc.simulate_dual_kc(
    et0_daily=corn_sc["et0_daily"],
    precip_daily=corn_sc["precip_daily"],
    kcb=corn_sc["kcb"],
    kc_max_val=corn_sc["kc_max"],
    few=corn_sc["few"],
    tew=corn_sc["tew"],
    rew=corn_sc["rew"],
)
for i, (etc_val, et0_val) in enumerate(zip(sim_corn["etc"], corn_sc["et0_daily"])):
    ratio = etc_val / et0_val if et0_val > 0 else 0
    assert abs(ratio - corn_sc["kcb"]) < 0.10

print("PASS: equations, table consistency TEW>REW, and corn mid-season ratio checks")
```

```python
C_GREEN, C_RED, C_BLUE = "#2ecc71", "#e74c3c", "#3498db"

x = np.arange(len(sim["de"]))
fig, ax1 = plt.subplots()
ax2 = ax1.twinx()

ax1.bar(x - 0.2, sim["de"], width=0.35, color=C_RED, label="Depletion De (mm)")
ax1.bar(x + 0.2, np.array(sim["ke"]) * 10, width=0.35, color=C_BLUE, alpha=0.75, label="Ke ×10 (-)")
ax1.set_xlabel("Day")
ax2.plot(x, sim["kr"], color=C_GREEN, marker="o", linewidth=2, label="Kr (-)")
ax1.set_ylabel("Bars: De | Ke scaled", color=C_RED)
ax2.set_ylabel("Kr (-)", color=C_GREEN)
plt.title("Bare-soil drydown — evaporation-layer state (FAO-56 dual Kc)")
fig.legend(loc="upper right", bbox_to_anchor=(0.92, 0.92))
plt.tight_layout()
plt.show()
```

## Summary

The notebook loads `benchmark_dual_kc.json` alongside `dual_crop_coefficient.py`, validates Eq.\ 69/72/73 test vectors, confirms $TEW > REW$ for Table 19 textures, exercises the bare-soil integration scenario ($K_\mathrm{r}$ decline, bounded $D_\mathrm{e}$), and shows mid-season corn where $ET_\mathrm{c}/ET_0 \approx K_{cb}$ because $f_\mathrm{ew}$ is small. This matches how the Rust harness cross-checks the Python baseline under `validate_dual_kc`.


