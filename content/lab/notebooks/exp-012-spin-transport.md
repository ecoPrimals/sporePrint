+++
title = "Experiment 012 — Spin Chain Transport"
description = "Rendered from exp-012-spin-transport.ipynb"
date = 2026-06-22
weight = 50

[extra]
domain = "Lab"
rendered_from = "exp-012-spin-transport.ipynb"
+++

<!-- Auto-generated from exp-012-spin-transport.ipynb by spore-validate render-notebooks -->

# Experiment 012 — Spin Chain Transport

Wavepacket dynamics in the 1D Almost-Mathieu (quasiperiodic) tight-binding
model to answer:
1. Does a wavepacket injected at one site spread ballistically (β≈1)
in the extended phase (λ<2)?
2. Does the wavepacket remain localized (β≈0) in the localized phase (λ>2)?
3. What happens at the critical point (λ=2)?
4. Does the Lyapunov exponent correctly predict the transport regime?

**Method:**
- Build tridiagonal Hamiltonian: H_{ij} = δ_{i,j±1} + λ cos(2παi+θ) δ_{ij}
- Eigendecompose: H = U Λ U^T
- Time-evolve: ψ_j(t) = Σ_k U_{j,k} U_{n₀,k} exp(-i E_k t)
- Compute MSD: σ²(t) = Σ_j (j - n₀)² |ψ_j(t)|²
- Extract transport exponent: fit log σ(t) = β log(t) + const

**Reference:**
Kachkovskiy (2016) Comm Math Phys 345:659-673
Jitomirskaya & Kachkovskiy (2018) JEMS 21:777-795

**Cross-spring: extends Exp 009 (quasiperiodic localization) with dynamics.**

**Domain**: Condensed Matter
**Faculty**: Kachkovskiy / Gonzales
**Reference**: Anderson localization in 1D spin chains

**Data source**: `control/spin_transport/spin_chain_transport.py` + `benchmark_*.json`

---

*This notebook is the publication-grade Python baseline for Experiment 012. The identical computations are validated in Rust (see `validate_*` binary) and delegated to barraCuda for GPU acceleration.*


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
benchmark_path = CONTROL / 'spin_transport' / 'benchmark_spin_transport.json'
with open(benchmark_path) as f:
    benchmark = json.load(f)

PASS_COLOR = '#2ecc71'
FAIL_COLOR = '#e74c3c'
INFO_COLOR = '#3498db'

print(f'Loaded benchmark: benchmark_spin_transport.json')
print(f'Provenance: {benchmark.get("_provenance", {})}')
```

## Validation


### Initialization


```python
model = benchmark["model"]
exp = benchmark["expected_results"]

n_sites = model["n_sites"]
alpha = model["alpha"]
theta = model["theta"]
init_site = model["init_site"]
couplings = model["coupling_strengths"]
times = np.array(model["times"])
lyap_n = model["lyapunov_n_sites"]
lyap_e = model["lyapunov_energy"]

print("groundSpring Exp 012: Spin Chain Transport (Kachkovskiy 2016)")
print(f"  Model: 1D Almost-Mathieu, {n_sites} sites, α = golden ratio")
print(f"  Wavepacket: δ_{{{init_site}}}, times: {list(times)}")
print("  Cross-spring: hotSpring (spectral), wetSpring (porous transport)")

betas = {}
final_msds = {}

for lam in couplings:
```

### Coupling λ = {lam:.1f}


```python
h = build_hamiltonian(n_sites, lam, alpha, theta)
eigenvalues, eigenvectors = np.linalg.eigh(h)

msds_at_t = []
for t in times:
    msd, norm = wavepacket_msd(eigenvalues, eigenvectors, init_site, t)
    msds_at_t.append(msd)
    if t == times[0]:
        check_approx(
            f"Normalization λ={lam:.1f} t={t:.0f}",
            norm, 1.0, exp["normalization_tolerance"],
        )
    if t == times[-1]:
        norm_check = norm
        check_approx(
            f"Normalization λ={lam:.1f} t={t:.0f}",
            norm_check, 1.0, exp["normalization_tolerance"],
        )

msds_arr = np.array(msds_at_t)
beta = transport_exponent(times, msds_arr)
betas[lam] = beta
final_msds[lam] = msds_at_t[-1]

sigma_final = math.sqrt(msds_at_t[-1]) if msds_at_t[-1] > 0 else 0
print(f"  MSD(t={times[-1]:.0f}) = {msds_at_t[-1]:.4f}, σ = {sigma_final:.4f}")
print(f"  Transport exponent β = {beta:.4f}")
```

### Validation: Transport Exponents


```python
check_range(
    "Ballistic transport β (λ=0.5)",
    betas[0.5],
    exp["ballistic_beta_range"][0],
    exp["ballistic_beta_range"][1],
)

check_range(
    "Ballistic transport β (λ=1.0)",
    betas[1.0],
    exp["ballistic_beta_range"][0],
    exp["ballistic_beta_range"][1],
)
```

### Localized transport (λ > 2


```python
check_max(
    "Localized transport β (λ=4.0)",
    betas[4.0],
    exp["localized_beta_max"],
)

check_max(
    "Localized MSD bounded (λ=4.0)",
    final_msds[4.0],
    exp["msd_localized_bounded_max"],
)
```

### Critical point (λ = 2


```python
check_range(
    "Critical transport β (λ=2.0)",
    betas[2.0],
    exp["critical_beta_range"][0],
    exp["critical_beta_range"][1],
)
```

### Lyapunov Cross-Check


```python
pot_ext = np.array([
    1.0 * math.cos(2.0 * math.pi * alpha * i + theta)
    for i in range(lyap_n)
])
gamma_ext = lyapunov_exponent(pot_ext, lyap_e)
print(f"  Lyapunov γ (λ=1.0): {gamma_ext:.6f}")

check_max("Lyapunov extended (λ=1.0) γ ≈ 0", gamma_ext, exp["lyapunov_extended_max"])

pot_loc = np.array([
    4.0 * math.cos(2.0 * math.pi * alpha * i + theta)
    for i in range(lyap_n)
])
gamma_loc = lyapunov_exponent(pot_loc, lyap_e)
print(f"  Lyapunov γ (λ=4.0): {gamma_loc:.6f}")

check_true(
    "Lyapunov localized (λ=4.0) γ > threshold",
    gamma_loc > exp["lyapunov_localized_min"],
)
```

### Monotonicity


```python
check_true(
    "β decreases with increasing λ",
    all(
        betas[couplings[i]] >= betas[couplings[i + 1]] - 0.15
        for i in range(len(couplings) - 1)
    ),
)
```

### Key Findings


```python
print(f"\n{'=' * 72}")
```

## Visualization


```python
# Publication-grade summary chart for Exp 012
fig, ax = plt.subplots(figsize=(8, 4))

p, f_count, t = pass_count(), fail_count(), total_count()
ax.barh(['Pass', 'Fail'], [p, f_count], color=[PASS_COLOR, FAIL_COLOR])
ax.set_xlim(0, max(t * 1.15, 1))
ax.set_title('Exp 012: Spin Chain Transport — Validation Results')
ax.set_xlabel('Check Count')
for i, v in enumerate([p, f_count]):
    if v > 0:
        ax.text(v + 0.3, i, str(v), va='center', fontweight='bold')

plt.tight_layout()
plt.savefig(f'/tmp/groundspring_exp012.png', dpi=150, bbox_inches='tight')
plt.show()
print(f'\nResult: {p}/{t} PASS, {f_count}/{t} FAIL')
```

## Provenance & Summary

| Field | Value |
|-------|-------|
| Experiment | 012 — Spin Chain Transport |
| Domain | Condensed Matter |
| Reference | Anderson localization in 1D spin chains |
| Faculty | Kachkovskiy / Gonzales |
| Python baseline | `control/spin_transport/spin_chain_transport.py` |
| Benchmark JSON | `control/spin_transport/benchmark_spin_transport.json` |
| Rust validator | `validate_*` binary (exit-code protocol) |
| Rust speedup | See benchmark comparison notebook |
| License | AGPL-3.0-or-later |

**Provenance chain**: Python baseline → Rust validation → barraCuda GPU → metalForge cross-substrate → primal IPC composition

See [primals.eco](https://primals.eco) for rendered lab notebooks.


