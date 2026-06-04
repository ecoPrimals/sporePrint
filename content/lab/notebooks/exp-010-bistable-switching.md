+++
title = "Experiment 010 — Bistable Phenotypic Switching"
description = "Rendered from exp-010-bistable-switching.ipynb"
date = 2026-06-04
weight = 50

[extra]
domain = "Lab"
rendered_from = "exp-010-bistable-switching.ipynb"
+++

<!-- Auto-generated from exp-010-bistable-switching.ipynb by spore-validate render-notebooks -->

# Experiment 010 — Bistable Phenotypic Switching

Models bistable switching in V. cholerae c-di-GMP circuit to answer:
1. Does positive feedback on DGC production create bistability?
2. Can different initial conditions converge to different attractors?
3. When does stochastic noise push a cell across the threshold?

**Method:**
- 5-variable ODE: [cell, AI, HapR, c-di-GMP, biofilm]
- RK4 integration with parameters from barracuda::BistableOde
- Positive feedback: alpha_fb * Hill(bio, K_fb, n_fb) → DGC
- Two initial conditions (low-cdg motile, high-cdg sessile)

**Reference:**
Fernandez, Waters et al. (2020) PNAS 117:26058-26068
Hammer & Bassler (2007) Mol Microbiol 64:547-558

**Cross-spring with wetSpring: biological ODE, QS signaling.**

**Domain**: Biochemistry
**Faculty**: Waters Lab (MSU)
**Reference**: Fernandez, Waters et al. (2020) PNAS 117:26058

**Data source**: `control/bistable_switching/bistable_switching.py` + `benchmark_*.json`

---

*This notebook is the publication-grade Python baseline for Experiment 010. The identical computations are validated in Rust (see `validate_*` binary) and delegated to barraCuda for GPU acceleration.*


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
benchmark_path = CONTROL / 'bistable_switching' / 'benchmark_bistable.json'
with open(benchmark_path) as f:
    benchmark = json.load(f)

PASS_COLOR = '#2ecc71'
FAIL_COLOR = '#e74c3c'
INFO_COLOR = '#3498db'

print(f'Loaded benchmark: benchmark_bistable.json')
print(f'Provenance: {benchmark.get("_provenance", {})}')
```

## Model Implementation


```python
def hill(x: float, k: float, n: float) -> float:
    if x <= 0:
        return 0.0
    xn = x ** n
    return float(xn / (k ** n + xn))
```

```python
def bistable_derivative(state: list[float], params: dict) -> list[float]:
    """Compute derivative for the 5-variable bistable ODE.

    State: [cell, ai, hapr, cdg, bio]
    Matches barracuda::BistableOde::cpu_derivative exactly.
    """
    cell = max(state[0], 0.0)
    ai = max(state[1], 0.0)
    hapr = max(state[2], 0.0)
    cdg = max(state[3], 0.0)
    bio = max(state[4], 0.0)

    p = params

    d_cell = p["mu_max"] * cell * (1.0 - cell / p["k_cap"]) - p["death_rate"] * cell
    d_ai = p["k_ai_prod"] * cell - p["d_ai"] * ai
    d_hapr = p["k_hapr_max"] * hill(ai, p["k_hapr_ai"], p["n_hapr"]) - p["d_hapr"] * hapr

    basal_dgc = p["k_dgc_basal"] * max(1.0 - p["k_dgc_rep"] * hapr, 0.0)
    feedback_dgc = p["alpha_fb"] * hill(bio, p["k_fb"], p["n_fb"])
    pde_rate = p["k_pde_basal"] + p["k_pde_act"] * hapr
    d_cdg = basal_dgc + feedback_dgc - pde_rate * cdg - p["d_cdg"] * cdg

    bio_promote = p["k_bio_max"] * hill(cdg, p["k_bio_cdg"], p["n_bio"])
    d_bio = bio_promote * (1.0 - bio) - p["d_bio"] * bio

    return [d_cell, d_ai, d_hapr, d_cdg, d_bio]


def rk4_step(state: list[float], params: dict, dt: float) -> list[float]:
    """Single RK4 step."""
    k1 = bistable_derivative(state, params)
    s1 = [s + 0.5 * dt * k for s, k in zip(state, k1, strict=True)]
    k2 = bistable_derivative(s1, params)
    s2 = [s + 0.5 * dt * k for s, k in zip(state, k2, strict=True)]
    k3 = bistable_derivative(s2, params)
    s3 = [s + dt * k for s, k in zip(state, k3, strict=True)]
    k4 = bistable_derivative(s3, params)
    return [
        s + dt / 6.0 * (a + 2 * b + 2 * c + d)
        for s, a, b, c, d in zip(state, k1, k2, k3, k4, strict=True)
    ]


def integrate(state0: list[float], params: dict, dt: float, t_final: float) -> list[list[float]]:
    """Integrate the ODE from state0 to t_final."""
    n_steps = int(t_final / dt)
    trajectory = [list(state0)]
    state = list(state0)
    for _ in range(n_steps):
        state = rk4_step(state, params, dt)
        trajectory.append(state)
    return trajectory
```

```python
def stochastic_integrate(
    state0: list[float],
    params: dict,
    dt: float,
    t_final: float,
    noise_level: float,
    rng: np.random.Generator,
) -> list[list[float]]:
    """Euler-Maruyama with additive Gaussian noise on c-di-GMP."""
    n_steps = int(t_final / dt)
    trajectory = [list(state0)]
    state = list(state0)
    sqrt_dt = math.sqrt(dt)
    for _ in range(n_steps):
        deriv = bistable_derivative(state, params)
        for i in range(5):
            state[i] += dt * deriv[i]
        state[3] += noise_level * sqrt_dt * rng.standard_normal()
        state[3] = max(state[3], 0.0)
        for i in range(5):
            state[i] = max(state[i], 0.0)
        trajectory.append(list(state))
    return trajectory
```

## Validation


### Initialization


```python
model = benchmark["model"]
exp = benchmark["expected_results"]

params = model["parameters"]
dt = model["dt"]
t_final = model["t_final"]
ic_low = model["initial_low_cdg"]
ic_high = model["initial_high_cdg"]

print("groundSpring Exp 010: Bistable Phenotypic Switching")
print(f"  Model: 5-variable ODE, dt={dt}, t_final={t_final}")
print("  Cross-spring: wetSpring (biological ODE, QS signaling)")
```

### Deterministic Bistability


```python
traj_low = integrate(ic_low, params, dt, t_final)
traj_high = integrate(ic_high, params, dt, t_final)

final_low = traj_low[-1]
final_high = traj_high[-1]

print(f"  Low IC final:  cell={final_low[0]:.3f}, cdg={final_low[3]:.3f}, bio={final_low[4]:.3f}")
print(f"  High IC final: cell={final_high[0]:.3f}, cdg={final_high[3]:.3f}, bio={final_high[4]:.3f}")

check_approx(
    "Cell reaches carrying capacity (low IC)",
    final_low[0],
    benchmark["analytical_predictions"]["cell_steady_state"],
    exp["cell_reaches_capacity_tol"],
)

check_approx(
    "Cell reaches carrying capacity (high IC)",
    final_high[0],
    benchmark["analytical_predictions"]["cell_steady_state"],
    exp["cell_reaches_capacity_tol"],
)
```

### Attractor Separation


```python
check_max(
    "Low IC → low c-di-GMP attractor",
    final_low[3], exp["low_cdg_attractor_max"],
)
check_min(
    "High IC → high c-di-GMP attractor",
    final_high[3], exp["high_cdg_attractor_min"],
)
check_max(
    "Low IC → low biofilm",
    final_low[4], exp["biofilm_low_attractor_max"],
)
check_min(
    "High IC → high biofilm",
    final_high[4], exp["biofilm_high_attractor_min"],
)
```

### Monostable Control


```python
mono_params = dict(params)
mono_params["alpha_fb"] = 0.0

traj_mono_low = integrate(ic_low, mono_params, dt, t_final)
traj_mono_high = integrate(ic_high, mono_params, dt, t_final)

mono_low = traj_mono_low[-1]
mono_high = traj_mono_high[-1]

print(f"  Monostable low IC:  cdg={mono_low[3]:.3f}, bio={mono_low[4]:.3f}")
print(f"  Monostable high IC: cdg={mono_high[3]:.3f}, bio={mono_high[4]:.3f}")

cdg_diff = abs(mono_low[3] - mono_high[3])
check_max(
    "Monostable: both ICs converge to same c-di-GMP",
    cdg_diff, exp["monostable_attractors_agree_tol"],
)
```

### Determinism


```python
traj_a = integrate(ic_low, params, dt, t_final)
check_true(
    "Deterministic trajectories agree",
    all(abs(a - b) < 1e-10 for a, b in zip(traj_low[-1], traj_a[-1], strict=True)),
)
```

### Stochastic Switching


```python
rng = np.random.default_rng(42)
n_trials = 50
threshold = (final_low[3] + final_high[3]) / 2
crossings = 0
for _trial in range(n_trials):
    straj = stochastic_integrate(
        list(ic_low), params, dt, t_final, 0.5, rng,
    )
    cdg_final = straj[-1][3]
    if cdg_final > threshold:
        crossings += 1

switching_rate = crossings / n_trials
print(f"  Switching rate: {crossings}/{n_trials} = {switching_rate:.3f}")

check_range(
    "Stochastic switching rate in expected range",
    switching_rate,
    exp["stochastic_switching_rate_range"][0],
    exp["stochastic_switching_rate_range"][1],
)
```

### Low Noise Agreement


```python
rng2 = np.random.default_rng(99)
straj_low_noise = stochastic_integrate(
    list(ic_low), params, dt, t_final, 0.01, rng2,
)
cdg_det = final_low[3]
cdg_stoch = straj_low_noise[-1][3]
print(f"  Deterministic cdg: {cdg_det:.3f}, stochastic (σ=0.01): {cdg_stoch:.3f}")

check_max(
    "Low noise c-di-GMP agrees with deterministic",
    abs(cdg_det - cdg_stoch), exp["low_noise_agreement_tol"],
)
```

### Key Findings


```python
print(f"\n{'=' * 72}")
```

### This is Anderson localization for biology: noise determines which


```python
print("   phenotypic state a bistable cell occupies")

# Results: {pass_count()}/{total_count()} checks passed
print_summary("Exp 010: Bistable Switching")
```

## Visualization


```python
# Publication-grade summary chart for Exp 010
fig, ax = plt.subplots(figsize=(8, 4))

p, f_count, t = pass_count(), fail_count(), total_count()
ax.barh(['Pass', 'Fail'], [p, f_count], color=[PASS_COLOR, FAIL_COLOR])
ax.set_xlim(0, max(t * 1.15, 1))
ax.set_title('Exp 010: Bistable Phenotypic Switching — Validation Results')
ax.set_xlabel('Check Count')
for i, v in enumerate([p, f_count]):
    if v > 0:
        ax.text(v + 0.3, i, str(v), va='center', fontweight='bold')

plt.tight_layout()
plt.savefig(f'/tmp/groundspring_exp010.png', dpi=150, bbox_inches='tight')
plt.show()
print(f'\nResult: {p}/{t} PASS, {f_count}/{t} FAIL')
```

## Provenance & Summary

| Field | Value |
|-------|-------|
| Experiment | 010 — Bistable Phenotypic Switching |
| Domain | Biochemistry |
| Reference | Fernandez, Waters et al. (2020) PNAS 117:26058 |
| Faculty | Waters Lab (MSU) |
| Python baseline | `control/bistable_switching/bistable_switching.py` |
| Benchmark JSON | `control/bistable_switching/benchmark_bistable.json` |
| Rust validator | `validate_*` binary (exit-code protocol) |
| Rust speedup | See benchmark comparison notebook |
| License | AGPL-3.0-or-later |

**Provenance chain**: Python baseline → Rust validation → barraCuda GPU → metalForge cross-substrate → primal IPC composition

See [primals.eco](https://primals.eco) for rendered lab notebooks.


