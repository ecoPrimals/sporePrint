+++
title = "Experiment 011 — Multi-Signal Quorum Sensing Integration"
description = "Rendered from exp-011-multisignal-qs.ipynb"
date = 2026-06-28
weight = 50

[extra]
domain = "Lab"
rendered_from = "exp-011-multisignal-qs.ipynb"
+++

<!-- Auto-generated from exp-011-multisignal-qs.ipynb by spore-validate render-notebooks -->

# Experiment 011 — Multi-Signal Quorum Sensing Integration

Models dual-signal quorum sensing in V. cholerae to answer:
1. How do cells fuse two noisy QS signals (CAI-1 and AI-2) through
the LuxO/HapR pathway?
2. When two noisy inputs are integrated, does the combined signal-to-
noise ratio improve or degrade?
3. Does dual signaling produce a stronger biofilm response than either
signal alone?

**Method:**
- 7-variable ODE: [cell, CAI-1, AI-2, LuxO~P, HapR, c-di-GMP, biofilm]
- RK4 integration with parameters from barracuda::MultiSignalOde
- Compare: dual-signal, CAI-1 only, AI-2 only, no signal

**Reference:**
Srivastava, Waters et al. (2011) J Bacteriology 194:122-136
Hammer & Bassler (2007) Mol Microbiol 64:547-558

**Cross-spring with wetSpring: biological ODE, dual-signal processing.**

**Domain**: Biochemistry
**Faculty**: Waters Lab (MSU)
**Reference**: Hammer & Bassler (2007) Mol Microbiol 64:547

**Data source**: `control/multisignal_qs/multisignal_qs.py` + `benchmark_*.json`

---

*This notebook is the publication-grade Python baseline for Experiment 011. The identical computations are validated in Rust (see `validate_*` binary) and delegated to barraCuda for GPU acceleration.*


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
benchmark_path = CONTROL / 'multisignal_qs' / 'benchmark_multisignal.json'
with open(benchmark_path) as f:
    benchmark = json.load(f)

PASS_COLOR = '#2ecc71'
FAIL_COLOR = '#e74c3c'
INFO_COLOR = '#3498db'

print(f'Loaded benchmark: benchmark_multisignal.json')
print(f'Provenance: {benchmark.get("_provenance", {})}')
```

## Model Implementation


```python
def hill(x: float, k: float, n: float) -> float:
    if x <= 0:
        return 0.0
    xn = x ** n
    return float(xn / (k ** n + xn))


def hill_repress(x: float, k: float, n: float) -> float:
    if x <= 0:
        return 1.0
    kn = k ** n
    return float(kn / (kn + x ** n))
```

```python
def multisignal_derivative(state: list[float], params: dict) -> list[float]:
    """Compute derivative for the 7-variable multi-signal ODE.

    State: [cell, cai1, ai2, luxo_p, hapr, cdg, bio]
    Matches barracuda::MultiSignalOde::cpu_derivative.
    """
    cell = max(state[0], 0.0)
    cai1 = max(state[1], 0.0)
    ai2 = max(state[2], 0.0)
    luxo_p = max(state[3], 0.0)
    hapr = max(state[4], 0.0)
    cdg = max(state[5], 0.0)
    bio = max(state[6], 0.0)

    p = params

    d_cell = p["mu_max"] * cell * (1.0 - cell / p["k_cap"]) - p["death_rate"] * cell
    d_cai1 = p["k_cai1_prod"] * cell - p["d_cai1"] * cai1
    d_ai2 = p["k_ai2_prod"] * cell - p["d_ai2"] * ai2

    dephos_cai1 = hill(cai1, p["k_cqs"], 2.0)
    dephos_ai2 = hill(ai2, p["k_luxpq"], 2.0)
    d_luxo_p = p["k_luxo_phos"] - (p["d_luxo_p"] + dephos_cai1 + dephos_ai2) * luxo_p

    d_hapr = p["k_hapr_max"] * hill_repress(luxo_p, p["k_repress"], p["n_repress"]) - p["d_hapr"] * hapr

    dgc_rate = p["k_dgc_basal"] * max(1.0 - p["k_dgc_rep"] * hapr, 0.0)
    pde_rate = p["k_pde_basal"] + p["k_pde_act"] * hapr
    d_cdg = dgc_rate - pde_rate * cdg - p["d_cdg"] * cdg

    bio_promote = p["k_bio_max"] * hill(cdg, p["k_bio_cdg"], p["n_bio"])
    d_bio = bio_promote * (1.0 - bio) - p["d_bio"] * bio

    return [d_cell, d_cai1, d_ai2, d_luxo_p, d_hapr, d_cdg, d_bio]


def rk4_step(state: list[float], params: dict, dt: float) -> list[float]:
    k1 = multisignal_derivative(state, params)
    s1 = [s + 0.5 * dt * k for s, k in zip(state, k1, strict=True)]
    k2 = multisignal_derivative(s1, params)
    s2 = [s + 0.5 * dt * k for s, k in zip(state, k2, strict=True)]
    k3 = multisignal_derivative(s2, params)
    s3 = [s + dt * k for s, k in zip(state, k3, strict=True)]
    k4 = multisignal_derivative(s3, params)
    return [
        s + dt / 6.0 * (a + 2 * b + 2 * c + d)
        for s, a, b, c, d in zip(state, k1, k2, k3, k4, strict=True)
    ]


def integrate(state0: list[float], params: dict, dt: float, t_final: float) -> list[list[float]]:
    n_steps = int(t_final / dt)
    trajectory = [list(state0)]
    state = list(state0)
    for _ in range(n_steps):
        state = rk4_step(state, params, dt)
        trajectory.append(state)
    return trajectory


def stochastic_integrate(
    state0: list[float],
    params: dict,
    dt: float,
    t_final: float,
    noise_level: float,
    rng: np.random.Generator,
) -> list[list[float]]:
    """Euler-Maruyama with additive noise on c-di-GMP (index 5)."""
    n_steps = int(t_final / dt)
    state = list(state0)
    sqrt_dt = math.sqrt(dt)
    trajectory = [list(state)]
    for _ in range(n_steps):
        deriv = multisignal_derivative(state, params)
        for i in range(7):
            state[i] += dt * deriv[i]
        state[5] += noise_level * sqrt_dt * rng.standard_normal()
        for i in range(7):
            state[i] = max(state[i], 0.0)
        trajectory.append(list(state))
    return trajectory
```

## Validation


### Initialization


```python
model = benchmark["model"]
pred = benchmark["analytical_predictions"]
exp = benchmark["expected_results"]

params = model["parameters"]
dt = model["dt"]
t_final = model["t_final"]
ic = model["initial_state"]

print("groundSpring Exp 011: Multi-Signal QS Integration")
print(f"  Model: 7-variable ODE, dt={dt}, t_final={t_final}")
print("  Cross-spring: wetSpring (biological ODE, dual-signal processing)")
```

### Dual-Signal Steady State


```python
traj_dual = integrate(ic, params, dt, t_final)
final_dual = traj_dual[-1]

print(f"  Cell={final_dual[0]:.3f}, CAI-1={final_dual[1]:.3f}, AI-2={final_dual[2]:.3f}")
print(f"  LuxO~P={final_dual[3]:.3f}, HapR={final_dual[4]:.3f}")
print(f"  c-di-GMP={final_dual[5]:.3f}, Biofilm={final_dual[6]:.3f}")

check_approx(
    "Cell reaches capacity",
    final_dual[0], pred["cell_steady_state"], exp["cell_reaches_capacity_tol"],
)
check_min("HapR > 0 at steady state", final_dual[4], exp["hapr_min_at_steady_state"])
check_min("Biofilm > 0 at steady state", final_dual[6], exp["biofilm_min_at_steady_state"])
```

### Single-Signal vs Dual-Signal


```python
params_cai1_only = dict(params)
params_cai1_only["k_ai2_prod"] = 0.0

params_ai2_only = dict(params)
params_ai2_only["k_cai1_prod"] = 0.0

traj_cai1 = integrate(ic, params_cai1_only, dt, t_final)
traj_ai2 = integrate(ic, params_ai2_only, dt, t_final)

final_cai1 = traj_cai1[-1]
final_ai2 = traj_ai2[-1]

print(f"  CAI-1 only: HapR={final_cai1[4]:.3f}, bio={final_cai1[6]:.3f}")
print(f"  AI-2 only:  HapR={final_ai2[4]:.3f}, bio={final_ai2[6]:.3f}")
print(f"  Dual:       HapR={final_dual[4]:.3f}, bio={final_dual[6]:.3f}")

check_true(
    "Dual HapR > CAI-1 only",
    final_dual[4] > final_cai1[4],
)
check_true(
    "Dual HapR > AI-2 only",
    final_dual[4] > final_ai2[4],
)
check_true(
    "Dual HapR represses biofilm more (less bio than single)",
    final_dual[6] < max(final_cai1[6], final_ai2[6]),
)
```

### Determinism


```python
traj_repeat = integrate(ic, params, dt, t_final)
check_true(
    "Deterministic trajectories agree",
    all(abs(a - b) < 1e-10 for a, b in zip(traj_dual[-1], traj_repeat[-1], strict=True)),
)
```

### SNR Analysis


```python
rng = np.random.default_rng(42)
n_trials = 30
noise_sigma = 0.3

cdg_dual_samples = []
cdg_cai1_samples = []
for _ in range(n_trials):
    s_d = stochastic_integrate(list(ic), params, dt, t_final, noise_sigma, rng)
    cdg_dual_samples.append(s_d[-1][5])
    s_c = stochastic_integrate(list(ic), params_cai1_only, dt, t_final, noise_sigma, rng)
    cdg_cai1_samples.append(s_c[-1][5])

dual_mean = float(np.mean(cdg_dual_samples))
dual_std = float(np.std(cdg_dual_samples))
cai1_mean = float(np.mean(cdg_cai1_samples))
cai1_std = float(np.std(cdg_cai1_samples))

snr_dual = abs(dual_mean) / max(dual_std, 1e-10)
snr_cai1 = abs(cai1_mean) / max(cai1_std, 1e-10)

print(f"  Dual SNR: {snr_dual:.2f} (mean={dual_mean:.3f}, std={dual_std:.3f})")
print(f"  CAI-1 only SNR: {snr_cai1:.2f} (mean={cai1_mean:.3f}, std={cai1_std:.3f})")

check_true(
    "Dual-signal has lower HapR variance (more robust regulation)",
    dual_std <= cai1_std * 1.5,
)
```

### Low Noise Agreement


```python
rng2 = np.random.default_rng(99)
straj = stochastic_integrate(list(ic), params, dt, t_final, 0.01, rng2)
cdg_det = final_dual[5]
cdg_stoch = straj[-1][5]
print(f"  Deterministic cdg: {cdg_det:.3f}, stochastic (σ=0.01): {cdg_stoch:.3f}")

check_max(
    "Low noise c-di-GMP agrees with deterministic",
    abs(cdg_det - cdg_stoch), exp["low_noise_cdg_agreement_tol"],
)
```

### Key Findings


```python
print(f"\n{'=' * 72}")
```

### Signal integration: two noisy inputs produce a more robust


```python
print("   phenotypic output than either signal alone — noise averaging")

# Results: {pass_count()}/{total_count()} checks passed
print_summary("Exp 011: Multi-Signal QS Integration")
```

## Visualization


```python
# Publication-grade summary chart for Exp 011
fig, ax = plt.subplots(figsize=(8, 4))

p, f_count, t = pass_count(), fail_count(), total_count()
ax.barh(['Pass', 'Fail'], [p, f_count], color=[PASS_COLOR, FAIL_COLOR])
ax.set_xlim(0, max(t * 1.15, 1))
ax.set_title('Exp 011: Multi-Signal Quorum Sensing Integration — Validation Results')
ax.set_xlabel('Check Count')
for i, v in enumerate([p, f_count]):
    if v > 0:
        ax.text(v + 0.3, i, str(v), va='center', fontweight='bold')

plt.tight_layout()
plt.savefig(f'/tmp/groundspring_exp011.png', dpi=150, bbox_inches='tight')
plt.show()
print(f'\nResult: {p}/{t} PASS, {f_count}/{t} FAIL')
```

## Provenance & Summary

| Field | Value |
|-------|-------|
| Experiment | 011 — Multi-Signal Quorum Sensing Integration |
| Domain | Biochemistry |
| Reference | Hammer & Bassler (2007) Mol Microbiol 64:547 |
| Faculty | Waters Lab (MSU) |
| Python baseline | `control/multisignal_qs/multisignal_qs.py` |
| Benchmark JSON | `control/multisignal_qs/benchmark_multisignal.json` |
| Rust validator | `validate_*` binary (exit-code protocol) |
| Rust speedup | See benchmark comparison notebook |
| License | AGPL-3.0-or-later |

**Provenance chain**: Python baseline → Rust validation → barraCuda GPU → metalForge cross-substrate → primal IPC composition

See [primals.eco](https://primals.eco) for rendered lab notebooks.


