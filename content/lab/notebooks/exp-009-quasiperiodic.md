+++
title = "Experiment 009 — Almost-Mathieu Quasiperiodic Localization"
description = "Rendered from exp-009-quasiperiodic.ipynb"
date = 2026-06-14
weight = 50

[extra]
domain = "Lab"
rendered_from = "exp-009-quasiperiodic.ipynb"
+++

<!-- Auto-generated from exp-009-quasiperiodic.ipynb by spore-validate render-notebooks -->

# Experiment 009 — Almost-Mathieu Quasiperiodic Localization

Numerically verifies the Aubry-André metal-insulator transition in the
Almost-Mathieu operator to answer:
1. Does structured (quasiperiodic) noise produce the same localization
as random disorder?
2. Can we predict the EXACT transition point (λ=2)?
3. Does Herman's formula γ = ln(λ/2) hold for λ > 2?
4. Do level statistics distinguish extended vs localized phases?

**Method:**
- 1D Almost-Mathieu: H ψ(n) = ψ(n+1) + ψ(n-1) + 2λ cos(2παn + θ) ψ(n)
- α = golden ratio (maximally irrational → no resonances)
- Lyapunov exponent via transfer matrix (same as Exp 008)
- Level spacing ratio for finite-size eigenvalue statistics

**Reference:**
Aubry & André (1980) Ann Israel Phys Soc 3:133
Herman (1983) Commentarii Math Helv 58:453
Jitomirskaya & Kachkovskiy (2018) JEMS 21:777-795
Bourgain & Kachkovskiy (2018) GAFA 29:3-43

**Cross-spring with hotSpring: spectral theory, Hofstadter butterfly.**

**Domain**: Condensed Matter
**Faculty**: Kachkovskiy (MSU)
**Reference**: Aubry-André model; Jitomirskaya (1999)

**Data source**: `control/quasiperiodic/quasiperiodic_localization.py` + `benchmark_*.json`

---

*This notebook is the publication-grade Python baseline for Experiment 009. The identical computations are validated in Rust (see `validate_*` binary) and delegated to barraCuda for GPU acceleration.*


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
benchmark_path = CONTROL / 'quasiperiodic' / 'benchmark_quasiperiodic.json'
with open(benchmark_path) as f:
    benchmark = json.load(f)

PASS_COLOR = '#2ecc71'
FAIL_COLOR = '#e74c3c'
INFO_COLOR = '#3498db'

print(f'Loaded benchmark: benchmark_quasiperiodic.json')
print(f'Provenance: {benchmark.get("_provenance", {})}')
```

## Model Implementation


```python
def almost_mathieu_potential(
    n: int, coupling: float, alpha: float, theta: float
) -> np.ndarray:
    """Generate the quasiperiodic potential V(i) = λ cos(2παi + θ).

    Standard convention: coupling λ multiplies cos directly, so the
    Aubry-André transition is at λ=2 and Herman's formula gives
    γ = ln(λ/2) for λ > 2.

    Unlike Anderson's random disorder, this potential is fully deterministic
    but incommensurate with the lattice when α is irrational.
    """
    indices = np.arange(n, dtype=np.float64)
    return np.asarray(coupling * np.cos(2.0 * np.pi * alpha * indices + theta))


def lyapunov_exponent(potential: np.ndarray, energy: float) -> float:
    """Compute Lyapunov exponent via transfer matrix method.

    Identical to Exp 008 (Anderson): T_n = [[E - V(n), -1], [1, 0]].
    """
    n = len(potential)
    if n == 0:
        return 0.0

    log_growth = 0.0
    vec = np.array([1.0, 0.0])

    for i in range(n):
        new_0 = (energy - potential[i]) * vec[0] - vec[1]
        new_1 = vec[0]
        vec[0] = new_0
        vec[1] = new_1

        norm = math.sqrt(vec[0] ** 2 + vec[1] ** 2)
        if norm > 0:
            log_growth += math.log(norm)
            vec[0] /= norm
            vec[1] /= norm

    return log_growth / n


def level_spacing_ratio(eigenvalues: np.ndarray) -> float:
    """Mean level spacing ratio <r> for a sorted eigenvalue sequence.

    r_n = min(δ_n, δ_{n+1}) / max(δ_n, δ_{n+1})
    <r> ≈ 0.5307 for GOE (extended), ≈ 0.3863 for Poisson (localized).
    """
    es = np.sort(eigenvalues)
    gaps = np.diff(es)
    if len(gaps) < 2:
        return 0.0
    ratios = []
    for i in range(len(gaps) - 1):
        small = min(gaps[i], gaps[i + 1])
        large = max(gaps[i], gaps[i + 1])
        if large > 0:
            ratios.append(small / large)
    return float(np.mean(ratios)) if ratios else 0.0


def build_hamiltonian(n: int, coupling: float, alpha: float, theta: float) -> np.ndarray:
    """Build the n×n Almost-Mathieu Hamiltonian matrix (dense)."""
    h = np.zeros((n, n))
    pot = almost_mathieu_potential(n, coupling, alpha, theta)
    for i in range(n):
        h[i, i] = pot[i]
        if i + 1 < n:
            h[i, i + 1] = 1.0
            h[i + 1, i] = 1.0
    return h
```

## Validation


### Initialization


```python
model = benchmark["model"]
pred = benchmark["analytical_predictions"]
exp = benchmark["expected_results"]

n_sites = model["n_sites"]
energy = model["energy"]
alpha = model["alpha"]
theta = model["theta"]
couplings = model["coupling_strengths"]
n_eig = model["n_eigenvalues"]

print("groundSpring Exp 009: Quasiperiodic Localization (Almost-Mathieu)")
print(f"  Model: 1D Almost-Mathieu, {n_sites} sites, α = golden ratio")
print("  Cross-spring: hotSpring (spectral theory, Hofstadter butterfly)")
```

### Clean System (λ=0


```python
pot_clean = almost_mathieu_potential(n_sites, 0.0, alpha, theta)
gamma_clean = lyapunov_exponent(pot_clean, energy)
print(f"  Lyapunov exponent (λ=0): {gamma_clean:.6f}")

check_approx(
    "Clean system γ ≈ 0",
    gamma_clean, pred["clean_lyapunov"], exp["clean_lyapunov_tol"],
)
```

### Coupling Sweep


```python
gammas = {}
for lam in couplings:
    pot = almost_mathieu_potential(n_sites, lam, alpha, theta)
    g = lyapunov_exponent(pot, energy)
    gammas[lam] = g
    print(f"  λ={lam:.1f}: γ={g:.6f}")

check_max(
    "Extended regime (λ=1) γ < threshold",
    gammas[1.0], exp["extended_regime_lyapunov_max"],
)

check_approx(
    "Herman's formula at λ=3: γ ≈ ln(3/2)",
    gammas[3.0], pred["herman_lambda_3"], exp["herman_tol_lambda_3"],
)

check_approx(
    "Herman's formula at λ=4: γ ≈ ln(2)",
    gammas[4.0], pred["herman_lambda_4"], exp["herman_tol_lambda_4"],
)
```

### Critical Point (λ=2


```python
print(f"  Lyapunov at critical coupling (λ=2): {gammas[2.0]:.6f}")
check_approx(
    "Critical point γ ≈ 0 (Aubry-André)",
    gammas[2.0], 0.0, exp["critical_lyapunov_tol"],
)
```

### Monotonicity


```python
above_critical = [lam for lam in couplings if lam >= 2.0]
above_gammas = [gammas[lam] for lam in above_critical]

check_true(
    "γ monotonically increasing for λ ≥ 2",
    all(
        above_gammas[i] <= above_gammas[i + 1]
        for i in range(len(above_gammas) - 1)
    ),
)
```

### Level spacing ratio (finite-size eigenvalue statistics


```python
# The Almost-Mathieu model is deterministic (not a random ensemble),
# so we average over many θ values to recover ensemble statistics.
# We also restrict to the bulk (middle 50%) of eigenvalues to avoid
# edge effects.
```

### Level Spacing Statistics


```python
n_theta = 50
thetas = np.linspace(0, 2 * np.pi, n_theta, endpoint=False)

ratios_ext = []
ratios_loc = []
for th in thetas:
    h_ext = build_hamiltonian(n_eig, 1.0, alpha, th)
    eigs_ext = np.sort(np.linalg.eigvalsh(h_ext))
    n_bulk = len(eigs_ext)
    lo, hi = n_bulk // 4, 3 * n_bulk // 4
    bulk = eigs_ext[lo:hi]
    gaps = np.diff(bulk)
    for j in range(len(gaps) - 1):
        s, b = min(gaps[j], gaps[j + 1]), max(gaps[j], gaps[j + 1])
        if b > 0:
            ratios_ext.append(s / b)

    h_loc = build_hamiltonian(n_eig, 4.0, alpha, th)
    eigs_loc = np.sort(np.linalg.eigvalsh(h_loc))
    bulk_loc = eigs_loc[lo:hi]
    gaps_loc = np.diff(bulk_loc)
    for j in range(len(gaps_loc) - 1):
        s, b = min(gaps_loc[j], gaps_loc[j + 1]), max(gaps_loc[j], gaps_loc[j + 1])
        if b > 0:
            ratios_loc.append(s / b)

r_ext = float(np.mean(ratios_ext)) if ratios_ext else 0.0
r_loc = float(np.mean(ratios_loc)) if ratios_loc else 0.0

print(f"  λ=1 (extended): <r> = {r_ext:.4f}  (GOE ≈ {pred['goe_r']})")
print(f"  λ=4 (localized): <r> = {r_loc:.4f}  (Poisson ≈ {pred['poisson_r']})")

check_range(
    "Level spacing λ=1 ~ GOE",
    r_ext,
    exp["level_spacing_extended_range"][0],
    exp["level_spacing_extended_range"][1],
)

check_range(
    "Level spacing λ=4 ~ Poisson",
    r_loc,
    exp["level_spacing_localized_range"][0],
    exp["level_spacing_localized_range"][1],
)
```

### Key Findings


```python
print(f"\n{'=' * 72}")
```

### Structured noise (quasiperiodic) shows a SHARP transition vs


```python
print("   Anderson's gradual onset — the key groundSpring insight for")
print("   periodic environmental signals (tides, seasons, diurnal cycles)")

# Results: {pass_count()}/{total_count()} checks passed
print_summary("Exp 009: Quasiperiodic Localization")
```

## Visualization


```python
# Publication-grade summary chart for Exp 009
fig, ax = plt.subplots(figsize=(8, 4))

p, f_count, t = pass_count(), fail_count(), total_count()
ax.barh(['Pass', 'Fail'], [p, f_count], color=[PASS_COLOR, FAIL_COLOR])
ax.set_xlim(0, max(t * 1.15, 1))
ax.set_title('Exp 009: Almost-Mathieu Quasiperiodic Localization — Validation Results')
ax.set_xlabel('Check Count')
for i, v in enumerate([p, f_count]):
    if v > 0:
        ax.text(v + 0.3, i, str(v), va='center', fontweight='bold')

plt.tight_layout()
plt.savefig(f'/tmp/groundspring_exp009.png', dpi=150, bbox_inches='tight')
plt.show()
print(f'\nResult: {p}/{t} PASS, {f_count}/{t} FAIL')
```

## Provenance & Summary

| Field | Value |
|-------|-------|
| Experiment | 009 — Almost-Mathieu Quasiperiodic Localization |
| Domain | Condensed Matter |
| Reference | Aubry-André model; Jitomirskaya (1999) |
| Faculty | Kachkovskiy (MSU) |
| Python baseline | `control/quasiperiodic/quasiperiodic_localization.py` |
| Benchmark JSON | `control/quasiperiodic/benchmark_quasiperiodic.json` |
| Rust validator | `validate_*` binary (exit-code protocol) |
| Rust speedup | See benchmark comparison notebook |
| License | AGPL-3.0-or-later |

**Provenance chain**: Python baseline → Rust validation → barraCuda GPU → metalForge cross-substrate → primal IPC composition

See [primals.eco](https://primals.eco) for rendered lab notebooks.


