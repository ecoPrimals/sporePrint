+++
title = "Experiment 006 — Signal Specificity in Quorum Sensing"
description = "Rendered from exp-006-signal-specificity.ipynb"
date = 2026-06-14
weight = 50

[extra]
domain = "Lab"
rendered_from = "exp-006-signal-specificity.ipynb"
+++

<!-- Auto-generated from exp-006-signal-specificity.ipynb by spore-validate render-notebooks -->

# Experiment 006 — Signal Specificity in Quorum Sensing

Models c-di-GMP signaling in Vibrio cholerae to answer:
1. What is the steady-state c-di-GMP level for a given enzyme network?
2. How does activating one DGC change the steady state?
3. What is the signal-to-noise ratio of activation?
4. How does SNR scale with activation fold-change?

**Method:**
- Gillespie SSA of a birth-death process: DGCs synthesize c-di-GMP,
PDEs degrade it.  One "signal" DGC is activated (rate × alpha).
- Analytical steady state validates the stochastic mean.
- SNR = (mean_activated - mean_basal) / std_basal

**Reference:**
Massie et al. (2012) PNAS 109:12746-51
Gillespie (1977) J Phys Chem 81:2340-2361

**Cross-spring with wetSpring: biological signal vs noise decomposition.**

**Domain**: Biochemistry
**Faculty**: Waters Lab (MSU)
**Reference**: Massie et al. (2012) PNAS, Gillespie SSA

**Data source**: `control/signal_specificity/signal_specificity.py` + `benchmark_*.json`

---

*This notebook is the publication-grade Python baseline for Experiment 006. The identical computations are validated in Rust (see `validate_*` binary) and delegated to barraCuda for GPU acceleration.*


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
benchmark_path = CONTROL / 'signal_specificity' / 'benchmark_signal_specificity.json'
with open(benchmark_path) as f:
    benchmark = json.load(f)

PASS_COLOR = '#2ecc71'
FAIL_COLOR = '#e74c3c'
INFO_COLOR = '#3498db'

print(f'Loaded benchmark: benchmark_signal_specificity.json')
print(f'Provenance: {benchmark.get("_provenance", {})}')
```

## Model Implementation


```python
def gillespie_birth_death(
    synthesis_rates: list[float],
    degradation_rate: float,
    initial: int,
    t_max: float,
    rng: np.random.Generator,
) -> tuple[np.ndarray, np.ndarray]:
    """Run Gillespie SSA for a birth-death process.

    Each DGC synthesizes at its own rate (zero-order).
    Degradation is first-order: rate = degradation_rate × current_count.

    Returns (times, states) arrays.
    """
    total_syn = sum(synthesis_rates)

    times = [0.0]
    states = [initial]
    t = 0.0
    s = initial

    while t < t_max:
        deg_rate = degradation_rate * s
        total_rate = total_syn + deg_rate

        if total_rate <= 0:
            break

        dt = rng.exponential(1.0 / total_rate)
        t += dt
        if t > t_max:
            break

        if rng.random() < total_syn / total_rate:
            s += 1
        else:
            s = max(0, s - 1)

        times.append(t)
        states.append(s)

    return np.array(times), np.array(states)


def steady_state_mean(total_synthesis: float, degradation_rate: float) -> float:
    """Analytical steady state: S* = total_syn / k_deg."""
    if degradation_rate <= 0:
        return 0.0
    return total_synthesis / degradation_rate


def time_averaged_mean(
    times: np.ndarray, states: np.ndarray, t_start: float
) -> float:
    """Compute time-weighted average of state after burn-in."""
    mask = times >= t_start
    t_trimmed = times[mask]
    s_trimmed = states[mask]

    if len(t_trimmed) < 2:
        return float(s_trimmed[0]) if len(s_trimmed) > 0 else 0.0

    dt = np.diff(t_trimmed)
    weighted = s_trimmed[:-1].astype(float) * dt
    return float(weighted.sum() / dt.sum())


def time_averaged_variance(
    times: np.ndarray, states: np.ndarray, t_start: float, mean: float
) -> float:
    """Compute time-weighted variance of state after burn-in."""
    mask = times >= t_start
    t_trimmed = times[mask]
    s_trimmed = states[mask]

    if len(t_trimmed) < 2:
        return 0.0

    dt = np.diff(t_trimmed)
    sq_dev = (s_trimmed[:-1].astype(float) - mean) ** 2
    return float((sq_dev * dt).sum() / dt.sum())
```

```python
def compute_snr(
    mean_activated: float, mean_basal: float, std_basal: float
) -> float:
    """Signal-to-noise ratio: (signal - baseline) / noise."""
    if std_basal <= 0:
        return 0.0
    return (mean_activated - mean_basal) / std_basal
```

## Validation


### Initialization


```python
net = benchmark["enzyme_network"]
sim = benchmark["simulation"]
pred = benchmark["analytical_predictions"]
exp = benchmark["expected_results"]

n_dgc = net["n_dgc"]
n_pde = net["n_pde"]
k_syn = net["k_syn_per_dgc"]
k_deg = net["k_deg_per_pde"]
total_deg = n_pde * k_deg

print("groundSpring Exp 006: Enzymatic Signal Specificity (c-di-GMP)")
print(f"  Enzyme network: {n_dgc} DGCs, {n_pde} PDEs")
print("  Cross-spring: wetSpring (biological signal-noise decomposition)")
```

### Analytical Steady State


```python
total_syn_basal = n_dgc * k_syn
ss_mean = steady_state_mean(total_syn_basal, total_deg)
ss_std = math.sqrt(ss_mean)

print(f"  Total synthesis rate: {total_syn_basal:.1f} s⁻¹")
print(f"  Total degradation rate constant: {total_deg:.1f} s⁻¹")
print(f"  Analytical S*: {ss_mean:.3f} molecules")
print(f"  Analytical σ: {ss_std:.3f} (Poisson)")

check_approx("Analytical mean", ss_mean, pred["steady_state_mean"], 0.01)
check_approx("Analytical std", ss_std, pred["steady_state_std"], 0.01)
```

### Gillespie SSA Basal Steady State


```python
basal_rates = [k_syn] * n_dgc
t_max = sim["t_max"]
t_burnin = sim["t_burnin"]

basal_means = []
basal_vars = []
n_reps = sim["n_replicates"]

for i in range(n_reps):
    rep_rng = np.random.default_rng(sim["seed"] + i)
    times, states = gillespie_birth_death(
        basal_rates, total_deg, int(ss_mean), t_max, rep_rng,
    )
    m = time_averaged_mean(times, states, t_burnin)
    v = time_averaged_variance(times, states, t_burnin, m)
    basal_means.append(m)
    basal_vars.append(v)

ensemble_mean = float(np.mean(basal_means))
ensemble_std = float(np.std(basal_means))
mean_variance = float(np.mean(basal_vars))

print(f"  Gillespie mean (N={n_reps}): {ensemble_mean:.3f} ± {ensemble_std:.3f}")
print(f"  Mean time-avg variance: {mean_variance:.3f}")

check_approx(
    "Gillespie mean matches analytical",
    ensemble_mean, ss_mean, exp["steady_state_mean_tol"],
)
check_approx(
    "Gillespie variance ~ Poisson",
    mean_variance, ss_mean, exp["steady_state_std_tol"] ** 2,
)

basal_ensemble_std = float(np.sqrt(np.mean(basal_vars)))
```

### Activated States & Response Ratios


```python
activated_means = {}
for alpha in net["activation_ratios"]:
    activated_rates = [k_syn] * n_dgc
    activated_rates[0] = k_syn * alpha

    act_means = []
    for i in range(n_reps):
        rep_rng = np.random.default_rng(sim["seed"] + 10000 + alpha * 1000 + i)
        times, states = gillespie_birth_death(
            activated_rates, total_deg, int(ss_mean), t_max, rep_rng,
        )
        m = time_averaged_mean(times, states, t_burnin)
        act_means.append(m)

    act_ensemble_mean = float(np.mean(act_means))
    activated_means[alpha] = act_ensemble_mean

    expected_total_syn = (n_dgc - 1) * k_syn + k_syn * alpha
    expected_mean = steady_state_mean(expected_total_syn, total_deg)
    response_ratio = act_ensemble_mean / ensemble_mean

    print(f"\n  α={alpha}: mean={act_ensemble_mean:.3f}, "
          f"expected={expected_mean:.3f}, ratio={response_ratio:.3f}")

rr_10 = activated_means[10] / ensemble_mean
rr_20 = activated_means[20] / ensemble_mean

check_range(
    "Response ratio α=10",
    rr_10,
    exp["response_ratio_alpha_10_range"][0],
    exp["response_ratio_alpha_10_range"][1],
)
check_range(
    "Response ratio α=20",
    rr_20,
    exp["response_ratio_alpha_20_range"][0],
    exp["response_ratio_alpha_20_range"][1],
)
```

### Signal-to-Noise Ratio


```python
snr_values = {}
for alpha in net["activation_ratios"]:
    snr = compute_snr(activated_means[alpha], ensemble_mean, basal_ensemble_std)
    snr_values[alpha] = snr
    print(f"  SNR(α={alpha}): {snr:.3f}")

check_range(
    "SNR α=10 in expected range",
    snr_values[10],
    exp["snr_alpha_10_range"][0],
    exp["snr_alpha_10_range"][1],
)
check_range(
    "SNR α=20 in expected range",
    snr_values[20],
    exp["snr_alpha_20_range"][0],
    exp["snr_alpha_20_range"][1],
)

snr_list = [snr_values[a] for a in sorted(snr_values.keys())]
check_true(
    "SNR monotonically increases with α",
    all(snr_list[i] <= snr_list[i + 1] for i in range(len(snr_list) - 1)),
)
check_true("SNR(α=2) > 0", snr_values[2] > 0)
```

### Determinism


```python
rng_a = np.random.default_rng(12345)
rng_b = np.random.default_rng(12345)
_, s_a = gillespie_birth_death(basal_rates, total_deg, 18, 50.0, rng_a)
_, s_b = gillespie_birth_death(basal_rates, total_deg, 18, 50.0, rng_b)
check_true("Gillespie deterministic (same seed)", np.array_equal(s_a, s_b))

rng_c = np.random.default_rng(99999)
_, s_c = gillespie_birth_death(basal_rates, total_deg, 18, 50.0, rng_c)
check_true("Gillespie differs (different seed)", not np.array_equal(s_a, s_c))
```

### Key Findings


```python
print(f"\n{'=' * 72}")
```

### Specificity requires α >> N_dgc for SNR >> 1


```python
# Results: {pass_count()}/{total_count()} checks passed
print_summary("Exp 006: Enzymatic Signal Specificity")
```

## Visualization


```python
# Publication-grade summary chart for Exp 006
fig, ax = plt.subplots(figsize=(8, 4))

p, f_count, t = pass_count(), fail_count(), total_count()
ax.barh(['Pass', 'Fail'], [p, f_count], color=[PASS_COLOR, FAIL_COLOR])
ax.set_xlim(0, max(t * 1.15, 1))
ax.set_title('Exp 006: Signal Specificity in Quorum Sensing — Validation Results')
ax.set_xlabel('Check Count')
for i, v in enumerate([p, f_count]):
    if v > 0:
        ax.text(v + 0.3, i, str(v), va='center', fontweight='bold')

plt.tight_layout()
plt.savefig(f'/tmp/groundspring_exp006.png', dpi=150, bbox_inches='tight')
plt.show()
print(f'\nResult: {p}/{t} PASS, {f_count}/{t} FAIL')
```

## Provenance & Summary

| Field | Value |
|-------|-------|
| Experiment | 006 — Signal Specificity in Quorum Sensing |
| Domain | Biochemistry |
| Reference | Massie et al. (2012) PNAS, Gillespie SSA |
| Faculty | Waters Lab (MSU) |
| Python baseline | `control/signal_specificity/signal_specificity.py` |
| Benchmark JSON | `control/signal_specificity/benchmark_signal_specificity.json` |
| Rust validator | `validate_*` binary (exit-code protocol) |
| Rust speedup | See benchmark comparison notebook |
| License | AGPL-3.0-or-later |

**Provenance chain**: Python baseline → Rust validation → barraCuda GPU → metalForge cross-substrate → primal IPC composition

See [primals.eco](https://primals.eco) for rendered lab notebooks.


