+++
title = "Hargreaves-Samani (1985) Temperature-Based ET₀"
description = "Rendered from 031-hargreaves-samani-et0.ipynb"
date = 2026-06-16
weight = 50

[extra]
domain = "Lab"
rendered_from = "031-hargreaves-samani-et0.ipynb"
+++

<!-- Auto-generated from 031-hargreaves-samani-et0.ipynb by spore-validate render-notebooks -->

# Hargreaves-Samani (1985) Temperature-Based ET₀

**Citation:** Hargreaves GH, Samani ZA (1985). *Applied Eng Agric* 1(2):96-99.

**Abstract.** Hargreaves–Samani expresses daily reference ET₀ using only extraterrestrial radiation and the diurnal temperature range. Here we replicate `hargreaves_samani.py`, validate analytical and RA cases plus FAO-56 city cross-checks against `benchmark_hargreaves.json`, and visualize PM comparison and numerical stress tests.

**For other springs:** Primal capability `science.et0_hargreaves`; Rust binary `validate_hargreaves`.


## Theory

$$\mathrm{ET}_0 = 0.0023\,(T_{\mathrm{mean}} + 17.8)\,(T_{\max} - T_{\min})^{0.5}\,R_a$$

with $R_a$ as extraterrestrial radiation; the control script reports $R_a$ in mm day⁻¹ equivalent (FAO-56 Eq. 21, divided by 2.45).


```python
import json
from pathlib import Path

import matplotlib.pyplot as plt
import numpy as np

BENCH_HG = (Path.cwd() / "../../control/hargreaves/benchmark_hargreaves.json").resolve()
with open(BENCH_HG, encoding="utf-8") as f:
    bench_hg = json.load(f)
print("Loaded:", BENCH_HG)
```

```python
import math

# From control/hargreaves/hargreaves_samani.py


def hargreaves_et0(tmin, tmax, ra_mm_day):
    tmean = (tmin + tmax) / 2.0
    dt = max(tmax - tmin, 0.0)
    return max(0.0, 0.0023 * (tmean + 17.8) * math.sqrt(dt) * ra_mm_day)


def extraterrestrial_radiation_ra(lat_deg, doy):
    lat_rad = lat_deg * math.pi / 180.0
    dr = 1.0 + 0.033 * math.cos(2.0 * math.pi * doy / 365.0)
    delta = 0.409 * math.sin(2.0 * math.pi * doy / 365.0 - 1.39)
    ws = math.acos(-math.tan(lat_rad) * math.tan(delta))
    gsc = 0.0820
    ra_mj = (24.0 * 60.0 / math.pi) * gsc * dr * (
        ws * math.sin(lat_rad) * math.sin(delta)
        + math.cos(lat_rad) * math.cos(delta) * math.sin(ws)
    )
    return ra_mj / 2.45
```

```python
PASS_COL, FAIL_COL, INFO_COL = "#2ecc71", "#e74c3c", "#3498db"

passed = failed = 0
vc = bench_hg["validation_checks"]


def check_line(name, ok):
    global passed, failed
    print(f"  [{'PASS' if ok else 'FAIL'}] {name}")
    if ok:
        passed += 1
    else:
        failed += 1


print("Analytical")
for tc in vc["analytical"]["test_cases"]:
    c = hargreaves_et0(tc["tmin"], tc["tmax"], tc["ra_mm_day"])
    ok = abs(c - tc["expected_et0"]) <= tc["tolerance"]
    check_line(f"analytical T={tc['tmin']}/{tc['tmax']}", ok)

print("\nRa computation")
for tc in vc["ra_computation"]["test_cases"]:
    c = extraterrestrial_radiation_ra(tc["latitude"], tc["doy"])
    ok = abs(c - tc["expected_ra_mm"]) <= tc["tolerance"]
    check_line(f"Ra lat={tc['latitude']} DOY={tc['doy']}", ok)

print("\nFAO56 cities (HG vs PM ratio band)")
for tc in vc["fao56_cross_comparison"]["test_cases"]:
    ra_mm = extraterrestrial_radiation_ra(tc["latitude"], tc["doy"])
    hg_et0 = hargreaves_et0(tc["tmin"], tc["tmax"], ra_mm)
    pm_et0 = tc["fao56_pm_et0"]
    ratio_diff = abs(hg_et0 / pm_et0 - 1.0) if pm_et0 > 0 else 999
    ok = ratio_diff <= tc["max_ratio_diff"]
    check_line(tc["city"], ok)

print("\nEdge cases")
for tc in vc["edge_cases"]["test_cases"]:
    c = hargreaves_et0(tc["tmin"], tc["tmax"], tc["ra_mm_day"])
    ct = tc["check"]
    if ct == "zero":
        ok = c == 0.0
    elif ct == "positive":
        ok = c > 0.0
    elif ct == "ge":
        ok = c >= tc["bound"]
    else:
        ok = False
    check_line(tc["label"], ok)

print("\nMonotonicity")
for tc in vc["monotonicity"]["test_cases"]:
    b = tc["base"]
    inc = tc["increased"]
    ra = tc["ra_mm_day"]
    e0 = hargreaves_et0(b["tmin"], b["tmax"], ra)
    e1 = hargreaves_et0(inc["tmin"], inc["tmax"], ra)
    check_line(tc["label"], e1 > e0)

total = passed + failed
print("\n" + "=" * 60)
print(f"TOTAL checks: {passed}/{total} PASS, {failed} FAIL")
print("=" * 60)
validation_ok_hg = failed == 0
```

```python
PASS_COL, FAIL_COL, INFO_COL = "#2ecc71", "#e74c3c", "#3498db"

fig, axes = plt.subplots(1, 3, figsize=(14, 4.2))

vc = bench_hg["validation_checks"]

# 1) HG vs PM — reference cities
cities_tc = vc["fao56_cross_comparison"]["test_cases"]
names = [c["city"].split()[0] for c in cities_tc]
hg_vals, pm_vals, ok_flags = [], [], []
for c in cities_tc:
    ra_mm = extraterrestrial_radiation_ra(c["latitude"], c["doy"])
    hg = hargreaves_et0(c["tmin"], c["tmax"], ra_mm)
    pm = c["fao56_pm_et0"]
    hg_vals.append(hg)
    pm_vals.append(pm)
    rd = abs(hg / pm - 1.0) if pm > 0 else 999
    ok_flags.append(rd <= c["max_ratio_diff"])

x = np.arange(len(names))
w = 0.35
ax = axes[0]
ax.bar(x - w / 2, hg_vals, w, label="Hargreaves", color=INFO_COL)
ax.bar(x + w / 2, pm_vals, w, label="FAO-56 PM ref.", color="#34495e", alpha=0.85)
for i, ok in enumerate(ok_flags):
    ax.text(i, max(hg_vals[i], pm_vals[i]) + 0.2, "✓" if ok else "✗", ha="center",
            color=PASS_COL if ok else FAIL_COL, fontsize=11)
ax.set_xticks(x)
ax.set_xticklabels(names, rotation=12, ha="right")
ax.set_ylabel("ET₀ (mm/day)")
ax.set_title("Hargreaves vs PM (reference cities)")
ax.legend(fontsize=8)

# 2) Analytical scatter: computed vs expected
ana = vc["analytical"]["test_cases"]
comp_a = [hargreaves_et0(t["tmin"], t["tmax"], t["ra_mm_day"]) for t in ana]
exp_a = [t["expected_et0"] for t in ana]
tol_a = [t["tolerance"] for t in ana]
ax2 = axes[1]
lim_lo = min(comp_a + exp_a) - 0.5
lim_hi = max(comp_a + exp_a) + 0.5
ax2.plot([lim_lo, lim_hi], [lim_lo, lim_hi], color=INFO_COL, lw=1)
for c, e, t in zip(comp_a, exp_a, tol_a):
    ok = abs(c - e) <= t
    ax2.scatter(e, c, color=PASS_COL if ok else FAIL_COL, edgecolors="k", s=60)
ax2.set_xlabel("Benchmark expected ET₀")
ax2.set_ylabel("Computed ET₀")
ax2.set_title("Analytical validation")
ax2.set_aspect("equal", adjustable="datalim")

# 3) Monotonicity: base vs perturbed pairs
mono = vc["monotonicity"]["test_cases"]
labels_m = []
deltas = []
colors = []
for tc in mono:
    b = tc["base"]
    inc = tc["increased"]
    ra = tc["ra_mm_day"]
    e0 = hargreaves_et0(b["tmin"], b["tmax"], ra)
    e1 = hargreaves_et0(inc["tmin"], inc["tmax"], ra)
    ok = e1 > e0
    short = tc["label"][:28] + "…" if len(tc["label"]) > 28 else tc["label"]
    labels_m.append(short)
    deltas.append(e1 - e0)
    colors.append(PASS_COL if ok else FAIL_COL)

ax3 = axes[2]
ypos = np.arange(len(labels_m))
ax3.barh(ypos, deltas, color=colors, alpha=0.85)
ax3.set_yticks(ypos)
ax3.set_yticklabels(labels_m, fontsize=7)
ax3.axvline(0, color="#7f8c8d", lw=0.8)
ax3.set_xlabel("Δ ET₀ (perturbed − base)")
ax3.set_title("Monotonicity uplift")

plt.tight_layout()
plt.show()
```

## Summary

| Item | Value |
|------|-------|
| Primal capability | `science.et0_hargreaves` |
| Rust validator | `validate_hargreaves` |
| Python baseline | [`control/hargreaves/hargreaves_samani.py`](../../control/hargreaves/hargreaves_samani.py) |
| Benchmark JSON | [`control/hargreaves/benchmark_hargreaves.json`](../../control/hargreaves/benchmark_hargreaves.json) |

**Provenance.** `_provenance` in the benchmark describes tolerances (including generous HG-vs-PM ratio bands for regional PM references).

**Future: Tier 2 primal IPC.** Promote checklist sections (`analytical`, `ra_computation`, `fao56_cross_comparison`, ...) to structured IPC attestations for `science.et0_hargreaves`.


