+++
title = "Standardized Precipitation Index (SPI)"
description = "Rendered from 081-spi-drought-index.ipynb"
date = 2026-06-23
weight = 50

[extra]
domain = "Lab"
rendered_from = "081-spi-drought-index.ipynb"
+++

<!-- Auto-generated from 081-spi-drought-index.ipynb by spore-validate render-notebooks -->

# Standardized Precipitation Index (SPI)

**Citations:** McKee TB et al. (1993); Edwards DC, McKee TB (1997); WMO (2012) SPI User Guide.

**Primal:** `science.spi_drought_index` · **Rust:** `validate_drought_index`

**Baseline:** `control/drought_index/drought_index_spi.py` · **Benchmark:** `control/drought_index/benchmark_drought_index.json`


## Theory

For scale $k$ months, accumulate precipitation, fit a **gamma** distribution to positive amounts (MLE via Thom’s approximation), account for zero-rain mass $q$, map through the gamma CDF, then apply the **inverse normal** to obtain standard-normal SPI.

**WMO-style categories** (illustrative): extremely wet (SPI $\ge 2$) … extremely dry (SPI $\le -2$).


```python
import json
import math
from pathlib import Path

import matplotlib.pyplot as plt
import numpy as np

REPO = Path('/home/eastgate/Development/ecoPrimals/springs/airSpring').resolve()
BENCH = REPO / "control/drought_index/benchmark_drought_index.json"

C_GREEN, C_RED, C_BLUE = "#2ecc71", "#e74c3c", "#3498db"
plt.rcParams.update({"figure.figsize": (9, 4.5), "axes.grid": True})
```

```python
def gamma_mle_fit(data):
    positive = [x for x in data if x > 0]
    n = len(positive)
    if n < 3:
        return None
    mean_val = sum(positive) / n
    log_mean = sum(math.log(x) for x in positive) / n
    A = math.log(mean_val) - log_mean
    if A <= 0:
        return None
    alpha = (1.0 / (4.0 * A)) * (1.0 + math.sqrt(1.0 + 4.0 * A / 3.0))
    beta = mean_val / alpha
    return (alpha, beta)


def gamma_series(a, x):
    ap = a
    total = 1.0 / a
    delta = total
    for _ in range(200):
        ap += 1
        delta *= x / ap
        total += delta
        if abs(delta) < abs(total) * 1e-14:
            break
    return total * math.exp(-x + a * math.log(x) - math.lgamma(a))


def gamma_cf(a, x):
    b = x + 1 - a
    c = 1e30
    d = 1.0 / b
    h = d
    for i in range(1, 200):
        an = -i * (i - a)
        b += 2
        d = an * d + b
        if abs(d) < 1e-30:
            d = 1e-30
        c = b + an / c
        if abs(c) < 1e-30:
            c = 1e-30
        d = 1.0 / d
        delta = d * c
        h *= delta
        if abs(delta - 1.0) < 1e-14:
            break
    return math.exp(-x + a * math.log(x) - math.lgamma(a)) * h


def regularized_gamma_p(a, x):
    if x <= 0:
        return 0.0
    if x < a + 1:
        return gamma_series(a, x)
    return 1.0 - gamma_cf(a, x)


def gamma_cdf(x, alpha, beta):
    if x <= 0:
        return 0.0
    return regularized_gamma_p(alpha, x / beta)


def norm_ppf(p):
    if p <= 0:
        return -8.0
    if p >= 1:
        return 8.0
    if p == 0.5:
        return 0.0
    if p < 0.5:
        t = math.sqrt(-2.0 * math.log(p))
    else:
        t = math.sqrt(-2.0 * math.log(1.0 - p))
    c0, c1, c2 = 2.515517, 0.802853, 0.010328
    d1, d2, d3 = 1.432788, 0.189269, 0.001308
    x = t - (c0 + c1 * t + c2 * t * t) / (1.0 + d1 * t + d2 * t * t + d3 * t * t * t)
    return -x if p < 0.5 else x


def compute_spi(monthly_precip, scale=1):
    n = len(monthly_precip)
    spi = [float("nan")] * n
    accum = []
    for i in range(n):
        if i < scale - 1:
            accum.append(float("nan"))
        else:
            accum.append(sum(monthly_precip[i - scale + 1 : i + 1]))
    valid = [x for x in accum if not math.isnan(x)]
    if len(valid) < 3:
        return spi
    fit = gamma_mle_fit(valid)
    if fit is None:
        return spi
    alpha, beta = fit
    q = sum(1 for x in valid if x == 0) / len(valid)
    for i in range(n):
        if math.isnan(accum[i]):
            continue
        if accum[i] == 0:
            prob = q
        else:
            prob = q + (1.0 - q) * gamma_cdf(accum[i], alpha, beta)
        prob = max(1e-10, min(1.0 - 1e-10, prob))
        spi[i] = norm_ppf(prob)
    return spi


def classify_spi(spi_value):
    if math.isnan(spi_value):
        return "insufficient_data"
    if spi_value >= 2.0:
        return "extremely_wet"
    if spi_value >= 1.5:
        return "very_wet"
    if spi_value >= 1.0:
        return "moderately_wet"
    if spi_value > -1.0:
        return "near_normal"
    if spi_value > -1.5:
        return "moderately_dry"
    if spi_value > -2.0:
        return "severely_dry"
    return "extremely_dry"

```

```python
with open(BENCH) as f:
    bench = json.load(f)

precip = bench["monthly_precip_mm"]
known = bench["gamma_fit_known"]["data"]
fit = gamma_mle_fit(known)
assert fit is not None
assert abs(fit[0] - bench["gamma_fit_known"]["alpha"]) < 1e-6
assert abs(fit[1] - bench["gamma_fit_known"]["beta"]) < 1e-6

spi1 = compute_spi(precip, 1)
# bench stores null for NaN in JSON — compare non-NaN indices
b1 = bench["spi1"]["values"]
for i, (a, b) in enumerate(zip(spi1, b1)):
    if b is None:
        assert math.isnan(a)
    else:
        assert abs(a - b) < 1e-9, i

spi3 = compute_spi(precip, 3)
for a, b in zip(spi3, bench["spi3"]["values"]):
    if b is None:
        assert math.isnan(a)
    else:
        assert abs(a - b) < 1e-9

assert bench["spi1"]["n_valid"] == len([x for x in spi1 if not math.isnan(x)])
classes = [classify_spi(v) for v in spi1 if not math.isnan(v)]
from collections import Counter

c = Counter(classes)
for k, v in bench["classification_counts"].items():
    assert c[k] == v

print("benchmark_drought_index.json SPI + gamma fit agreement passed.")
```

```python
fig, ax = plt.subplots()
ax.plot(range(1, len(precip) + 1), precip, color=C_BLUE, lw=1.2, label="Monthly $P$")
ax.set_xlabel("Month index")
ax.set_ylabel("mm")
ax.set_title("Benchmark monthly precipitation")
ax2 = ax.twinx()
ax2.plot(range(1, len(spi1) + 1), spi1, color=C_RED, lw=1.2, alpha=0.85, label="SPI-1")
ax.legend(loc="upper left")
ax2.legend(loc="upper right")
plt.tight_layout()
plt.show()

fig2, ax3 = plt.subplots()
months = np.arange(1, len(spi1) + 1)
ax3.plot(months, spi1, color=C_GREEN, label="SPI-1")
ax3.plot(months, spi3, color=C_BLUE, alpha=0.8, label="SPI-3")
ax3.axhline(0, color="gray", lw=1)
ax3.set_xlabel("Month")
ax3.set_ylabel("SPI")
ax3.set_title("SPI scales (benchmark series)")
ax3.legend()
plt.tight_layout()
plt.show()
```

## Summary

- **Algorithm:** Gamma MLE + zero handling + normal quantile transform, matching the control script.  
- **Benchmark:** `benchmark_drought_index.json` stores the 60-month synthetic series, fitted gamma parameters, SPI vectors, and classification counts.  
- **Provenance:** Deterministic generator seed and references in JSON `_provenance`.


