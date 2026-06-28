+++
title = "Experiment 014 — Drift vs Selection in Microbial Populations"
description = "Rendered from exp-014-drift-selection.ipynb"
date = 2026-06-28
weight = 50

[extra]
domain = "Lab"
rendered_from = "exp-014-drift-selection.ipynb"
+++

<!-- Auto-generated from exp-014-drift-selection.ipynb by spore-validate render-notebooks -->

# Experiment 014 — Drift vs Selection in Microbial Populations

Wright-Fisher simulation testing when stochastic drift dominates over
deterministic selection in finite populations, to answer:
1. At what population size does selection (signal) overcome drift (noise)?
2. Does the N*s > 1 threshold correctly predict the regime?
3. How does diversity decay under pure drift vs selection?
4. Is the fixation probability consistent with Kimura's formula?

**Method:**
- Wright-Fisher model: N diploid individuals, binomial sampling each gen
- Allele A has fitness 1+s, allele a has fitness 1
- Track fixation probability over many trials
- Compare to Kimura (1968) analytical predictions
- Neutral diversity: multi-species WF under pure drift

**Reference:**
Anderson (2022) mBio 13:e00354-22 — drift dominates in low-biomass habitats
Kimura (1968) Nature 217:624-626 — neutral theory of molecular evolution
Wright (1931) Genetics 16:97-159 — Wright-Fisher model

**Cross-spring: wetSpring (microbial diversity), Exp 004 (sequencing depth).**

**Domain**: Population Genetics
**Faculty**: R. Anderson (Carleton)
**Reference**: Wright-Fisher + Moran models

**Data source**: `control/drift_selection/drift_selection.py` + `benchmark_*.json`

---

*This notebook is the publication-grade Python baseline for Experiment 014. The identical computations are validated in Rust (see `validate_*` binary) and delegated to barraCuda for GPU acceleration.*


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
benchmark_path = CONTROL / 'drift_selection' / 'benchmark_drift_selection.json'
with open(benchmark_path) as f:
    benchmark = json.load(f)

PASS_COLOR = '#2ecc71'
FAIL_COLOR = '#e74c3c'
INFO_COLOR = '#3498db'

print(f'Loaded benchmark: benchmark_drift_selection.json')
print(f'Provenance: {benchmark.get("_provenance", {})}')
```

## Validation


### Initialization


```python
model = benchmark["model"]
pred = benchmark["analytical_predictions"]
exp = benchmark["expected_results"]

pop_sizes = model["population_sizes"]
s_coeff = model["selection_coefficient"]
p0 = model["initial_frequency"]
n_trials = model["n_trials"]
base_seed = model["base_seed"]

print("groundSpring Exp 014: Drift vs Selection (R. Anderson 2022)")
print(f"  Wright-Fisher model: s={s_coeff}, p₀={p0}, {n_trials} trials")
print(f"  Population sizes: {pop_sizes}")
print("  Cross-spring: wetSpring (microbial diversity)")
```

### Neutral Fixation (s=0


```python
n_neutral = 100
neutral_fixes = sum(
    wright_fisher_fixation(n_neutral, 0.0, p0, base_seed + i)
    for i in range(n_trials)
)
neutral_fix_rate = neutral_fixes / n_trials
print(f"  N={n_neutral}, s=0: fixation rate = {neutral_fix_rate:.3f} (expected ~{p0})")

check_range(
    "Neutral fixation ≈ p₀",
    neutral_fix_rate,
    exp["neutral_fixation_range"][0],
    exp["neutral_fixation_range"][1],
)
```

### Selection Across Population Sizes


```python
fix_rates = {}
for n_pop in pop_sizes:
    fixes = sum(
        wright_fisher_fixation(n_pop, s_coeff, p0, base_seed + 10000 + n_pop * 1000 + i)
        for i in range(n_trials)
    )
    fix_rate = fixes / n_trials
    kimura_pred = kimura_fixation_prob(n_pop, s_coeff, p0)
    ns_product = n_pop * s_coeff
    regime = "DRIFT" if ns_product < 1.0 else "SELECTION"
    fix_rates[n_pop] = fix_rate
    print(f"  N={n_pop:4d}, N×s={ns_product:5.2f} ({regime:9s}): "
          f"P_fix={fix_rate:.3f} (Kimura={kimura_pred:.3f})")

# Drift regime: fixation ≈ neutral
drift_fix = fix_rates[pop_sizes[0]]
check_range(
    f"Drift regime (N={pop_sizes[0]}) near neutral",
    drift_fix,
    pred["fixation_prob_neutral_p0_half"] - exp["drift_regime_fixation_near_neutral_tol"],
    pred["fixation_prob_neutral_p0_half"] + exp["drift_regime_fixation_near_neutral_tol"],
)

# Selection regime: fixation > neutral
sel_fix = fix_rates[pop_sizes[-1]]
check_true(
    f"Selection regime (N={pop_sizes[-1]}) > 60%",
    sel_fix >= exp["strong_selection_fixation_min"],
)

# Monotonicity: fixation increases with N (for s > 0)
rates_ordered = [fix_rates[n] for n in pop_sizes]
check_true(
    "Fixation generally increases with N",
    rates_ordered[-1] > rates_ordered[0],
)
```

### Kimura Formula Accuracy


```python
for n_pop in pop_sizes:
    kimura = kimura_fixation_prob(n_pop, s_coeff, p0)
    observed = fix_rates[n_pop]
    diff = abs(observed - kimura)
    status = "OK" if diff < 0.10 else "WARN"
    print(f"  N={n_pop:4d}: observed={observed:.3f}, Kimura={kimura:.3f}, "
          f"diff={diff:.3f} [{status}]")
```

### Neutral Diversity Decay


```python
n_sp = model["n_species_neutral"]
n_gen = model["n_generations_diversity"]

diversities_small = neutral_diversity_trajectory(n_sp, 50, n_gen, base_seed + 90000)
diversities_large = neutral_diversity_trajectory(n_sp, 500, n_gen, base_seed + 91000)

h0_small = diversities_small[0]
h_end_small = diversities_small[-1]
h0_large = diversities_large[0]
h_end_large = diversities_large[-1]

print(f"  N=50:  H(0)={h0_small:.4f} → H({n_gen})={h_end_small:.4f}")
print(f"  N=500: H(0)={h0_large:.4f} → H({n_gen})={h_end_large:.4f}")

check_true(
    "Diversity declines under drift (N=50)",
    h_end_small < h0_small,
)
check_true(
    "Small pop loses diversity faster",
    h_end_small < h_end_large,
)
```

### Determinism


```python
r1 = wright_fisher_fixation(100, 0.01, 0.5, 99999)
r2 = wright_fisher_fixation(100, 0.01, 0.5, 99999)
check_true("WF deterministic (same seed)", r1 == r2)
```

### Key Findings


```python
print(f"\n{'=' * 72}")
```

## Visualization


```python
# Publication-grade summary chart for Exp 014
fig, ax = plt.subplots(figsize=(8, 4))

p, f_count, t = pass_count(), fail_count(), total_count()
ax.barh(['Pass', 'Fail'], [p, f_count], color=[PASS_COLOR, FAIL_COLOR])
ax.set_xlim(0, max(t * 1.15, 1))
ax.set_title('Exp 014: Drift vs Selection in Microbial Populations — Validation Results')
ax.set_xlabel('Check Count')
for i, v in enumerate([p, f_count]):
    if v > 0:
        ax.text(v + 0.3, i, str(v), va='center', fontweight='bold')

plt.tight_layout()
plt.savefig(f'/tmp/groundspring_exp014.png', dpi=150, bbox_inches='tight')
plt.show()
print(f'\nResult: {p}/{t} PASS, {f_count}/{t} FAIL')
```

## Provenance & Summary

| Field | Value |
|-------|-------|
| Experiment | 014 — Drift vs Selection in Microbial Populations |
| Domain | Population Genetics |
| Reference | Wright-Fisher + Moran models |
| Faculty | R. Anderson (Carleton) |
| Python baseline | `control/drift_selection/drift_selection.py` |
| Benchmark JSON | `control/drift_selection/benchmark_drift_selection.json` |
| Rust validator | `validate_*` binary (exit-code protocol) |
| Rust speedup | See benchmark comparison notebook |
| License | AGPL-3.0-or-later |

**Provenance chain**: Python baseline → Rust validation → barraCuda GPU → metalForge cross-substrate → primal IPC composition

See [primals.eco](https://primals.eco) for rendered lab notebooks.


