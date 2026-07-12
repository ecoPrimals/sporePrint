+++
title = "Blaney–Criddle (1950) Temperature-Based PET"
description = "Rendered from 049-blaney-criddle-et0.ipynb"
date = 2026-07-04
weight = 50

[extra]
domain = "Lab"
rendered_from = "049-blaney-criddle-et0.ipynb"
+++

<!-- Auto-generated from 049-blaney-criddle-et0.ipynb by spore-validate render-notebooks -->

# Blaney–Criddle (1950) Temperature-Based PET

**Experiment 049** — Python control baseline aligned with primal `science.et0_blaney_criddle` and Rust `validate_blaney_criddle`.


## Theory

The original Blaney–Criddle relation estimates consumptive use / PET from mean monthly temperature and the **mean daily fraction of annual daytime hours** $p$ (latitude and month).

$$\mathrm{ET}_0 = p \, (0.46\, T + 8.13) \quad [\mathrm{mm\,day}^{-1}]$$

with $T$ mean monthly temperature (°C) and $p$ as a 0–1 fraction (SCS-TP-96; FAO-24 Table 18 for tabulated $p$). FAO-24 humidity and wind corrections are **not** applied here — this matches the western-US irrigation form in the control script.

**Citation:** Blaney HF, Criddle WD (1950) *Determining water requirements in irrigated areas from climatological and irrigation data*. USDA-SCS Technical Paper 96.


```python
import json
import math
import sys
from pathlib import Path

def _find_repo_root() -> Path:
    p = Path.cwd().resolve()
    for _ in range(8):
        if (p / "control" / "blaney_criddle").is_dir():
            return p
        p = p.parent
    return Path(os.environ.get('AIRSPRING_ROOT', '../airSpring'))

REPO = _find_repo_root()
BENCHMARK_PATH = REPO / "control" / "blaney_criddle" / "benchmark_blaney_criddle.json"

with open(BENCHMARK_PATH, encoding="utf-8") as f:
    bench = json.load(f)

print("Benchmark:", BENCHMARK_PATH)
```

```python
def check(name: str, observed: float, expected: float, tol: float = 0.02) -> bool:
    ok = abs(observed - expected) <= tol
    status = "PASS" if ok else "FAIL"
    print("  [%s] %s: observed=%.4f, expected=%.4f, tol=%s" % (status, name, observed, expected, tol))
    return ok


def blaney_criddle_et0(tmean_c: float, p: float) -> float:
    """Original Blaney-Criddle (1950) ET₀ estimate (mm/day).

    Args:
        tmean_c: Mean monthly temperature (°C).
        p: Mean daily percentage of annual daytime hours (0-1 fraction).

    Returns:
        ET₀ in mm/day (clamped to >= 0).
    """
    return max(0.0, p * (0.46 * tmean_c + 8.13))


def daylight_hours(latitude_rad: float, doy: int) -> float:
    """Compute daylight hours N from latitude and day-of-year (FAO-56 Eq. 34)."""
    dr = 1.0 + 0.033 * math.cos(2.0 * math.pi * doy / 365.0)  # noqa: F841
    delta = 0.4093 * math.sin(2.0 * math.pi * doy / 365.0 - 1.405)
    ws = math.acos(max(-1.0, min(1.0, -math.tan(latitude_rad) * math.tan(delta))))
    return (24.0 / math.pi) * ws


def daylight_fraction(latitude_deg: float, doy: int) -> float:
    """Compute Blaney-Criddle p factor from latitude and DOY."""
    lat_rad = math.radians(latitude_deg)
    N = daylight_hours(lat_rad, doy)
    return N / 43.80


def blaney_criddle_from_location(tmean_c: float, latitude_deg: float, doy: int) -> float:
    """Blaney-Criddle with daylight computed from latitude + DOY."""
    p = daylight_fraction(latitude_deg, doy)
    return blaney_criddle_et0(tmean_c, p)


def amc_cn_dry(cn_ii: float) -> float:
    """AMC-I (dry) curve number from AMC-II."""
    return 4.2 * cn_ii / (10.0 - 0.058 * cn_ii)


def amc_cn_wet(cn_ii: float) -> float:
    """AMC-III (wet) curve number from AMC-II."""
    return 23.0 * cn_ii / (10.0 + 0.13 * cn_ii)
```

```python
PASS_COUNT = 0
FAIL_COUNT = 0


def run_check(name: str, observed: float, expected: float, tol: float = 0.02) -> None:
    global PASS_COUNT, FAIL_COUNT
    ok = abs(observed - expected) <= tol
    status = "PASS" if ok else "FAIL"
    if ok:
        PASS_COUNT += 1
    else:
        FAIL_COUNT += 1
    print("  [%s] %s: observed=%.4f, expected=%.4f, tol=%s" % (status, name, observed, expected, tol))


print("\n── Analytical Benchmarks (from JSON) ──")
for case in bench["analytical_benchmarks"]:
    inputs = case["inputs"]
    expected = case["expected_et0_mm_day"]
    tol = case["tolerance"]
    computed = blaney_criddle_et0(inputs["tmean_c"], inputs["p"])
    run_check(case["name"], computed, expected, tol)

print("\n── Daylight fraction from location ──")
p_summer = daylight_fraction(40.0, 172)
p_winter = daylight_fraction(40.0, 356)
p_equator = daylight_fraction(0.0, 172)
run_check("p_summer_40N", p_summer, 0.333, 0.015)
run_check("p_winter_40N", p_winter, 0.222, 0.015)
run_check("p_equator", p_equator, 0.274, 0.005)

print("\n── Monotonicity ──")
temps = [-10, 0, 10, 20, 30, 40]
et0_temp = [blaney_criddle_et0(t, 0.274) for t in temps]
temp_mono = all(et0_temp[i] <= et0_temp[i + 1] for i in range(len(et0_temp) - 1))
run_check("temperature_monotonic", float(temp_mono), 1.0, 0.0)

ps = [0.199, 0.222, 0.274, 0.333, 0.366]
et0_p = [blaney_criddle_et0(25.0, p) for p in ps]
p_mono = all(et0_p[i] <= et0_p[i + 1] for i in range(len(et0_p) - 1))
run_check("daylight_monotonic", float(p_mono), 1.0, 0.0)
run_check("summer_gt_winter_p", float(p_summer > p_winter), 1.0, 0.0)

print("\n── Cross-method spot (Michigan July) ──")
bc_michigan = blaney_criddle_from_location(22.0, 42.7, 195)
run_check("bc_michigan_july_range_low", float(bc_michigan > 5.0), 1.0, 0.0)
run_check("bc_michigan_july_range_high", float(bc_michigan < 7.5), 1.0, 0.0)
print("    BC ET₀ = %.2f mm/day (Michigan July, T=22°C)" % bc_michigan)

print("\n── Non-negative constraint ──")
run_check("non_negative_at_minus20", blaney_criddle_et0(-20.0, 0.199), 0.0, 0.01)
run_check("non_negative_at_minus30", blaney_criddle_et0(-30.0, 0.199), 0.0, 0.01)

total = PASS_COUNT + FAIL_COUNT
print("\n=== Blaney-Criddle: %s/%s PASS, %s FAIL ===" % (PASS_COUNT, total, FAIL_COUNT))
if FAIL_COUNT:
    sys.exit(1)
```

```python
import matplotlib.pyplot as plt
import numpy as np

cases = bench["analytical_benchmarks"]
xs = np.arange(len(cases))
comp = [blaney_criddle_et0(c["inputs"]["tmean_c"], c["inputs"]["p"]) for c in cases]
exp = [c["expected_et0_mm_day"] for c in cases]

t_ax = np.linspace(-5, 35, 200)
for p_frac, clr, lbl in [(0.222, "#3498db", "p=0.222 (40°N Jan)"),
                         (0.274, "#2ecc71", "p=0.274 (equator)"),
                         (0.333, "#e74c3c", "p=0.333 (40°N Jul)")]:
    line = [blaney_criddle_et0(float(t), p_frac) for t in t_ax]
    plt.plot(t_ax, line, lw=2, color=clr, label=lbl)

plt.xlabel("Mean monthly temperature (°C)")
plt.ylabel("ET₀ (mm day⁻¹)")
plt.title("Blaney–Criddle families vs temperature")
plt.grid(True, alpha=0.25)
plt.legend()
plt.tight_layout()
plt.show()

fig, ax = plt.subplots(figsize=(6.8, 4.2))
w = 0.35
ax.bar(xs - w / 2, comp, w, label="Computed", color="#2ecc71", edgecolor="white")
ax.bar(xs + w / 2, exp, w, label="Benchmark JSON", color="#3498db", edgecolor="white")
ax.set_xticks(xs)
ax.set_xticklabels([c["name"] for c in cases], rotation=35, ha="right")
ax.set_ylabel("ET₀ (mm day⁻¹)")
ax.set_title("Analytical cases vs benchmark")
ax.legend()
plt.tight_layout()
plt.show()
```

## Summary and provenance

- **Primal:** `science.et0_blaney_criddle`
- **Rust:** `validate_blaney_criddle`
- **Control script:** `control/blaney_criddle/blaney_criddle_et0.py`
- **Benchmark:** `control/blaney_criddle/benchmark_blaney_criddle.json`

The benchmark file's `_provenance` block records baseline commit, command, and expected pass count. Monthly $p$ tables in the JSON follow FAO-24 Table 18.


