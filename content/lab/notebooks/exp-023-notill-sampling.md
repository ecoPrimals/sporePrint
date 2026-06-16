+++
title = "Experiment 023 — No-Till vs Tilled 16S Sampling"
description = "Rendered from exp-023-notill-sampling.ipynb"
date = 2026-06-16
weight = 50

[extra]
domain = "Lab"
rendered_from = "exp-023-notill-sampling.ipynb"
+++

<!-- Auto-generated from exp-023-notill-sampling.ipynb by spore-validate render-notebooks -->

# Experiment 023 — No-Till vs Tilled 16S Sampling

Extends Exp 004 (sequencing noise / rarefaction) to compare sampling strategies
for no-till (high diversity) vs tilled (low diversity) soil microbiome communities.
Answers:
1. Does the saturation depth differ between soil management regimes?
2. Does aggregate stability affect effective sampling?
3. What is the minimum depth to reliably distinguish the two communities?

**Method:**
- Generate two synthetic communities: no-till (150 genera, high evenness) and
tilled (100 genera, lower evenness / more dominant species)
- Run rarefaction curves for both at multiple depths
- Compare Shannon diversity convergence depths
- Compare Chao1 richness estimation accuracy at various depths
- Determine minimum depth to reliably distinguish the two communities

**Cross-spring: wetSpring (16S microbiome pipeline).**

**Domain**: Soil Science
**Faculty**: Cross-spring (wetSpring)
**Reference**: Cross-spring: wetSpring 16S pipeline

**Data source**: `control/notill_sampling/notill_sampling.py` + `benchmark_*.json`

---

*This notebook is the publication-grade Python baseline for Experiment 023. The identical computations are validated in Rust (see `validate_*` binary) and delegated to barraCuda for GPU acceleration.*


```python
import json
import math
import sys
from pathlib import Path

import numpy as np
import matplotlib.pyplot as plt

# Wire path to groundSpring control/ for common utilities
CONTROL = Path('..') / '..' / 'control'
sys.path.insert(0, str(CONTROL))
from common import *  # noqa: F403 — validation harness

# Load benchmark data
benchmark_path = CONTROL / 'notill_sampling' / 'benchmark_notill_sampling.json'
with open(benchmark_path) as f:
    benchmark = json.load(f)

PASS_COLOR = '#2ecc71'
FAIL_COLOR = '#e74c3c'
INFO_COLOR = '#3498db'

print(f'Loaded benchmark: benchmark_notill_sampling.json')
print(f'Provenance: {benchmark.get("_provenance", {})}')
```

## Model Implementation


```python
def generate_community(
    n_genera: int,
    log_normal_mu: float,
    log_normal_sigma: float,
    seed: int = 42,
) -> dict:
    """Generate a synthetic community from log-normal abundance distribution.

    Uses log-normal to produce realistic rank-abundance curves.
    Higher sigma -> more uneven (dominant species); lower sigma -> higher evenness.
    """
    rng = np.random.default_rng(seed)
    raw = rng.lognormal(mean=log_normal_mu, sigma=log_normal_sigma, size=n_genera)
    raw = np.maximum(raw, 1e-12)  # avoid zeros
    abundances = raw / raw.sum()
    return {
        "n_genera": n_genera,
        "true_abundances": abundances,
    }
```

```python
def compute_shannon(counts: np.ndarray) -> float:
    """Shannon diversity index H' = -Sigma(p_i ln p_i)."""
    total = counts.sum()
    if total == 0:
        return 0.0
    proportions = counts[counts > 0] / total
    return float(-np.sum(proportions * np.log(proportions)))


def chao1(counts: np.ndarray) -> float:
    """Chao1 non-parametric richness estimator.

    S_chao1 = S_obs + f1^2 / (2*f2)
    When f2=0 and f1>0: S_obs + f1*(f1-1)/2  (bias-corrected, Chao 1984).
    """
    s_obs = int(np.sum(counts > 0))
    f1 = int(np.sum(counts == 1))
    f2 = int(np.sum(counts == 2))

    if f2 > 0:
        return s_obs + f1**2 / (2 * f2)
    if f1 > 0:
        return s_obs + f1 * (f1 - 1) / 2
    return float(s_obs)
```

```python
def rarefaction_at_depth(
    community: dict,
    depth: int,
    n_replicates: int,
    base_seed: int,
) -> dict:
    """Run rarefaction at a specific sequencing depth."""
    rng = np.random.default_rng(base_seed + depth)
    true_abund = community["true_abundances"]

    shannon_values: list[float] = []
    chao1_values: list[float] = []

    for _ in range(n_replicates):
        counts = rng.multinomial(depth, true_abund)
        shannon_values.append(compute_shannon(counts))
        chao1_values.append(chao1(counts))

    return {
        "depth": depth,
        "shannon_mean": float(np.mean(shannon_values)),
        "shannon_std": float(np.std(shannon_values)),
        "chao1_mean": float(np.mean(chao1_values)),
        "chao1_std": float(np.std(chao1_values)),
    }
```

```python
def find_saturation_depth(
    rarefaction_results: list[dict],
    true_shannon: float,
    threshold_pct: float = 5.0,
) -> int:
    """Find depth where Shannon stabilizes within threshold_pct of true."""
    for result in rarefaction_results:
        obs_h = result["shannon_mean"]
        if true_shannon > 0:
            pct_diff = abs(obs_h - true_shannon) / true_shannon * 100
            if pct_diff <= threshold_pct:
                return int(result["depth"])
    return -1
```

## Validation


### Initialization


```python
print("groundSpring Exp 023: No-Till vs Tilled 16S Sampling Design")
print("  Cross-spring: wetSpring (16S microbiome pipeline)")

communities_config = benchmark["communities"]
rarefaction_config = benchmark["rarefaction"]
expected = benchmark["expected"]

depths = rarefaction_config["depths"]
n_replicates = rarefaction_config["n_replicates"]
base_seed = rarefaction_config["seed"]
```

### Synthetic Communities


```python
notill_cfg = communities_config["notill"]
tilled_cfg = communities_config["tilled"]

if "abundances" in notill_cfg:
    notill = {"n_genera": notill_cfg["n_genera"], "true_abundances": np.array(notill_cfg["abundances"])}
else:
    notill = generate_community(
        notill_cfg["n_genera"],
        notill_cfg["log_normal_mu"],
        notill_cfg["log_normal_sigma"],
        seed=base_seed,
    )
if "abundances" in tilled_cfg:
    tilled = {"n_genera": tilled_cfg["n_genera"], "true_abundances": np.array(tilled_cfg["abundances"])}
else:
    tilled = generate_community(
        tilled_cfg["n_genera"],
        tilled_cfg["log_normal_mu"],
        tilled_cfg["log_normal_sigma"],
        seed=base_seed + 1000,
    )
```

### True Shannon from high-resolution counts


```python
notill_true_shannon = compute_shannon(
    (notill["true_abundances"] * 1e8).astype(np.int64)
)
tilled_true_shannon = compute_shannon(
    (tilled["true_abundances"] * 1e8).astype(np.int64)
)

print(f"  No-till: {notill['n_genera']} genera, true Shannon = {notill_true_shannon:.4f}")
print(f"  Tilled:  {tilled['n_genera']} genera, true Shannon = {tilled_true_shannon:.4f}")
```

### Rarefaction Analysis


```python
notill_results = []
tilled_results = []

for depth in depths:
    nr = rarefaction_at_depth(notill, depth, n_replicates, base_seed)
    tr = rarefaction_at_depth(tilled, depth, n_replicates, base_seed)
    notill_results.append(nr)
    tilled_results.append(tr)

    print(f"\n  Depth {depth:>6d} reads:")
    print(
        f"    No-till Shannon: {nr['shannon_mean']:.4f} +/- {nr['shannon_std']:.4f}, "
        f"Chao1: {nr['chao1_mean']:.1f} +/- {nr['chao1_std']:.1f}"
    )
    print(
        f"    Tilled  Shannon: {tr['shannon_mean']:.4f} +/- {tr['shannon_std']:.4f}, "
        f"Chao1: {tr['chao1_mean']:.1f} +/- {tr['chao1_std']:.1f}"
    )
```

### Validate Expected Patterns


```python
depth_to_notill = {r["depth"]: r for r in notill_results}
depth_to_tilled = {r["depth"]: r for r in tilled_results}

high_depth = max(depths)
notill_shannon_high = depth_to_notill[high_depth]["shannon_mean"]
tilled_shannon_high = depth_to_tilled[high_depth]["shannon_mean"]

check_true(
    "No-till has higher diversity than tilled at high depth",
    notill_shannon_high > tilled_shannon_high,
)

check_range(
    "No-till Shannon at high depth",
    notill_shannon_high,
    expected["notill_shannon_range"][0],
    expected["notill_shannon_range"][1],
)

check_range(
    "Tilled Shannon at high depth",
    tilled_shannon_high,
    expected["tilled_shannon_range"][0],
    expected["tilled_shannon_range"][1],
)

notill_chao1_high = depth_to_notill[high_depth]["chao1_mean"]
tilled_chao1_high = depth_to_tilled[high_depth]["chao1_mean"]
check_true(
    "No-till Chao1 higher than tilled at high depth",
    notill_chao1_high > tilled_chao1_high,
)

if 1000 in depth_to_notill:
    notill_shannon_1k = depth_to_notill[1000]["shannon_mean"]
    tilled_shannon_1k = depth_to_tilled[1000]["shannon_mean"]
    check_true(
        "Communities distinguishable at 1000 reads (no-till Shannon > tilled)",
        notill_shannon_1k > tilled_shannon_1k,
    )

sat_notill = find_saturation_depth(notill_results, notill_true_shannon)
sat_tilled = find_saturation_depth(tilled_results, tilled_true_shannon)

print("\n  Saturation depth (5% convergence):")
print(f"    No-till: {sat_notill} reads")
print(f"    Tilled:  {sat_tilled} reads")

check_range(
    "No-till saturation depth in expected range",
    sat_notill,
    expected["saturation_depth_notill_range"][0],
    expected["saturation_depth_notill_range"][1],
)

check_range(
    "Tilled saturation depth in expected range",
    sat_tilled,
    expected["saturation_depth_tilled_range"][0],
    expected["saturation_depth_tilled_range"][1],
)
```

### Key Findings


```python
print(f"\n{'=' * 72}")
```

### Distinguishable at 1000 reads: yes


```python
print(f"4. No-till Chao1 ({notill_chao1_high:.1f}) > Tilled Chao1 ({tilled_chao1_high:.1f})")

# Results: {pass_count()}/{total_count()} checks passed
print_summary("Exp 023: No-Till vs Tilled Sampling Design")
```

## Visualization


```python
# Publication-grade summary chart for Exp 023
fig, ax = plt.subplots(figsize=(8, 4))

p, f_count, t = pass_count(), fail_count(), total_count()
ax.barh(['Pass', 'Fail'], [p, f_count], color=[PASS_COLOR, FAIL_COLOR])
ax.set_xlim(0, max(t * 1.15, 1))
ax.set_title('Exp 023: No-Till vs Tilled 16S Sampling — Validation Results')
ax.set_xlabel('Check Count')
for i, v in enumerate([p, f_count]):
    if v > 0:
        ax.text(v + 0.3, i, str(v), va='center', fontweight='bold')

plt.tight_layout()
plt.savefig(f'/tmp/groundspring_exp023.png', dpi=150, bbox_inches='tight')
plt.show()
print(f'\nResult: {p}/{t} PASS, {f_count}/{t} FAIL')
```

## Provenance & Summary

| Field | Value |
|-------|-------|
| Experiment | 023 — No-Till vs Tilled 16S Sampling |
| Domain | Soil Science |
| Reference | Cross-spring: wetSpring 16S pipeline |
| Faculty | Cross-spring (wetSpring) |
| Python baseline | `control/notill_sampling/notill_sampling.py` |
| Benchmark JSON | `control/notill_sampling/benchmark_notill_sampling.json` |
| Rust validator | `validate_*` binary (exit-code protocol) |
| Rust speedup | See benchmark comparison notebook |
| License | AGPL-3.0-or-later |

**Provenance chain**: Python baseline → Rust validation → barraCuda GPU → metalForge cross-substrate → primal IPC composition

See [primals.eco](https://primals.eco) for rendered lab notebooks.


