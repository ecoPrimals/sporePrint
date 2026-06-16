+++
title = "Experiment 017 — Quasispecies Error Threshold"
description = "Rendered from exp-017-quasispecies-threshold.ipynb"
date = 2026-06-16
weight = 50

[extra]
domain = "Lab"
rendered_from = "exp-017-quasispecies-threshold.ipynb"
+++

<!-- Auto-generated from exp-017-quasispecies-threshold.ipynb by spore-validate render-notebooks -->

# Experiment 017 — Quasispecies Error Threshold

At what mutation rate does noise (copying errors) destroy signal
(heritable information)?  Eigen's error threshold (1971) defines
the boundary: below it, the master sequence maintains a stable
subpopulation; above it, population randomizes to uniform noise.
This is the most fundamental formulation of groundSpring's central
question applied to self-replicating systems.

**Method:**
- Single-peak fitness landscape: master fitness sigma, mutant fitness 1
- Wright-Fisher selection + independent per-base mutation
- Track master sequence frequency across generations
- Compare to analytical: x_m = (sigma*Q - 1)/(sigma - 1), Q = (1-mu)^L
- Error threshold: mu_c = 1 - sigma^(-1/L)

**Reference:**
Dolson et al. (2023) J R Soc Interface 20(208)
Eigen (1971) Naturwiss 58:465-523
Kimura (1968) Nature 217:624-626

**Cross-spring: wetSpring (microbial evolution), Exp 014 (drift).**

**Domain**: Evolutionary Biology
**Faculty**: Dolson (MSU)
**Reference**: Eigen (1971) error catastrophe; Dolson

**Data source**: `control/quasispecies_threshold/quasispecies_threshold.py` + `benchmark_*.json`

---

*This notebook is the publication-grade Python baseline for Experiment 017. The identical computations are validated in Rust (see `validate_*` binary) and delegated to barraCuda for GPU acceleration.*


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
benchmark_path = CONTROL / 'quasispecies_threshold' / 'benchmark_quasispecies.json'
with open(benchmark_path) as f:
    benchmark = json.load(f)

PASS_COLOR = '#2ecc71'
FAIL_COLOR = '#e74c3c'
INFO_COLOR = '#3498db'

print(f'Loaded benchmark: benchmark_quasispecies.json')
print(f'Provenance: {benchmark.get("_provenance", {})}')
```

## Validation


### Initialization


```python
model = benchmark["model"]
exp = benchmark["expected_results"]

pop_size = model["population_size"]
genome_length = model["genome_length"]
sigma = model["master_fitness"]
mutation_rates = model["mutation_rates"]
n_gen = model["n_generations"]
base_seed = model["base_seed"]

mu_c = error_threshold(sigma, genome_length)

print("groundSpring Exp 017: Quasispecies Error Threshold (Dolson 2023)")
print(f"  N={pop_size}, L={genome_length}, σ={sigma}")
print(f"  Analytical error threshold: μ_c = {mu_c:.5f}")
print(f"  Mutation rates tested: {mutation_rates}")
print("  Cross-spring: wetSpring (microbial evolution), Exp 014")
```

### Analytical Master Frequency


```python
for mu in mutation_rates:
    x_m = master_frequency_analytical(sigma, mu, genome_length)
    regime = "BELOW" if mu < mu_c else "ABOVE"
    print(f"  μ={mu:.3f} ({regime:5s} threshold): x_m = {x_m:.4f}")

check_range(
    "Error threshold matches analytical",
    mu_c,
    exp["error_threshold_observed_range"][0],
    exp["error_threshold_observed_range"][1],
)
```

### Below Threshold (Signal Survives


```python
mu_below = mutation_rates[1]
assert mu_below < mu_c, f"mu_below={mu_below} should be < mu_c={mu_c}"

freqs_below = quasispecies_simulation(
    pop_size, genome_length, sigma, mu_below, n_gen, base_seed,
)
steady_state = float(np.mean(freqs_below[n_gen // 2 :]))
x_m_theory = master_frequency_analytical(sigma, mu_below, genome_length)

print(f"  μ={mu_below}: steady-state x_m = {steady_state:.4f} (theory {x_m_theory:.4f})")

check_min(
    "Master survives below threshold",
    steady_state,
    exp["master_freq_below_threshold_min"],
)
```

### Above Threshold (Signal Destroyed


```python
mu_above = mutation_rates[5]
assert mu_above > mu_c, f"mu_above={mu_above} should be > mu_c={mu_c}"

freqs_above = quasispecies_simulation(
    pop_size, genome_length, sigma, mu_above, n_gen, base_seed + 1000,
)
steady_above = float(np.mean(freqs_above[n_gen // 2 :]))
x_m_above_theory = master_frequency_analytical(sigma, mu_above, genome_length)

print(f"  μ={mu_above}: steady-state x_m = {steady_above:.4f} (theory {x_m_above_theory:.4f})")

check_max(
    "Master lost above threshold",
    steady_above,
    exp["master_freq_above_threshold_max"],
)
```

### Mutation Rate Sweep


```python
steady_states = {}
mean_fitnesses = {}
for i, mu in enumerate(mutation_rates):
    freqs = quasispecies_simulation(
        pop_size, genome_length, sigma, mu, n_gen, base_seed + 5000 + i * 100,
    )
    ss = float(np.mean(freqs[n_gen // 2 :]))
    mf = sigma * ss + 1.0 * (1.0 - ss)
    steady_states[mu] = ss
    mean_fitnesses[mu] = mf
    regime = "SIGNAL" if mu < mu_c else "NOISE"
    print(f"  μ={mu:.3f}: x_m={ss:.4f}, fitness={mf:.3f} [{regime}]")

below_rates = [mu for mu in mutation_rates if mu < mu_c]
above_rates = [mu for mu in mutation_rates if mu > mu_c]

if below_rates and above_rates:
    below_fitness = mean_fitnesses[below_rates[-1]]
    above_fitness = mean_fitnesses[above_rates[0]]
    check_true(
        "Mean fitness drops at threshold",
        below_fitness > above_fitness,
    )
```

### Master Frequency Monotonically Decreases


```python
ss_list = [steady_states[mu] for mu in mutation_rates]
decreasing = all(
    ss_list[i] >= ss_list[i + 1] - 0.05
    for i in range(len(ss_list) - 1)
)
check_true("Master frequency decreases with μ", decreasing)
```

### Determinism


```python
f1 = quasispecies_simulation(pop_size, genome_length, sigma, 0.01, 100, 99999)
f2 = quasispecies_simulation(pop_size, genome_length, sigma, 0.01, 100, 99999)
check_true("Quasispecies deterministic", f1 == f2)
```

### Key findings


```python
print(f"\n{'=' * 72}")
```

### Key Findings


```python
print(f"\n1. Error threshold: μ_c = {mu_c:.5f} (Eigen 1971)")
print(f"2. Below threshold (μ={mu_below}): master x_m = {steady_state:.4f} (signal survives)")
print(f"3. Above threshold (μ={mu_above}): master x_m = {steady_above:.4f} (noise wins)")
print(f"4. Mean fitness drops from {mean_fitnesses.get(below_rates[-1], 0):.3f} to "
      f"{mean_fitnesses.get(above_rates[0], 0):.3f} at threshold")
print()
print("  Dolson et al. (2023) asked: where does signal begin in a system")
print("  that starts as pure noise? Eigen's error threshold provides the")
print("  answer — below μ_c, information self-organizes; above, noise wins.")

# Results: {pass_count()}/{total_count()} checks passed
print_summary("Exp 017: Quasispecies Error Threshold")
```

## Visualization


```python
# Publication-grade summary chart for Exp 017
fig, ax = plt.subplots(figsize=(8, 4))

p, f_count, t = pass_count(), fail_count(), total_count()
ax.barh(['Pass', 'Fail'], [p, f_count], color=[PASS_COLOR, FAIL_COLOR])
ax.set_xlim(0, max(t * 1.15, 1))
ax.set_title('Exp 017: Quasispecies Error Threshold — Validation Results')
ax.set_xlabel('Check Count')
for i, v in enumerate([p, f_count]):
    if v > 0:
        ax.text(v + 0.3, i, str(v), va='center', fontweight='bold')

plt.tight_layout()
plt.savefig(f'/tmp/groundspring_exp017.png', dpi=150, bbox_inches='tight')
plt.show()
print(f'\nResult: {p}/{t} PASS, {f_count}/{t} FAIL')
```

## Provenance & Summary

| Field | Value |
|-------|-------|
| Experiment | 017 — Quasispecies Error Threshold |
| Domain | Evolutionary Biology |
| Reference | Eigen (1971) error catastrophe; Dolson |
| Faculty | Dolson (MSU) |
| Python baseline | `control/quasispecies_threshold/quasispecies_threshold.py` |
| Benchmark JSON | `control/quasispecies_threshold/benchmark_quasispecies.json` |
| Rust validator | `validate_*` binary (exit-code protocol) |
| Rust speedup | See benchmark comparison notebook |
| License | AGPL-3.0-or-later |

**Provenance chain**: Python baseline → Rust validation → barraCuda GPU → metalForge cross-substrate → primal IPC composition

See [primals.eco](https://primals.eco) for rendered lab notebooks.


