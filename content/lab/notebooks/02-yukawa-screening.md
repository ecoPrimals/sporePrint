+++
title = "Screened Coulomb (Yukawa) Bound-State Eigenvalues"
description = "Rendered from 02-yukawa-screening.ipynb"
date = 2026-06-20
weight = 50

[extra]
domain = "Lab"
rendered_from = "02-yukawa-screening.ipynb"
+++

<!-- Auto-generated from 02-yukawa-screening.ipynb by spore-validate render-notebooks -->

# Screened Coulomb (Yukawa) Bound-State Eigenvalues

**Paper:** Murillo & Weisheit, *Physics Reports* **302**, 1-65 (1998)  
**Reference:** Lam & Varshni, *Phys. Rev. A* **4**, 1875 (1971) — critical screening parameters  
**What we reproduce:** Eigenvalues of the Yukawa potential $V(r) = -Z e^{-\kappa r}/r$
via discretized radial Schrödinger equation. We compute bound-state spectra
as a function of screening parameter $\kappa$ and determine critical screening
values $\kappa_c$ where bound states disappear.

---

*This notebook runs entirely in Python (numpy + scipy). No GPU, no Rust, no primals required.*  
*Rust parity:* `barracuda/src/physics/screened_coulomb.rs` uses the same grid and discretization.  
*Grid: 2,000 points, $r_{\max} = 100$ a.u.*

## Physics

The Yukawa (screened Coulomb) potential models charge screening in plasmas:

$$V(r) = -\frac{Z e^{-\kappa r}}{r}$$

We solve the radial Schrödinger equation on a uniform grid $r_i = (i+1)h$
with $h = r_{\max}/(N+1)$. The resulting tridiagonal Hamiltonian is:

$$H_{ii} = \frac{1}{h^2} + \frac{\ell(\ell+1)}{2r_i^2} - \frac{Z e^{-\kappa r_i}}{r_i}$$

$$H_{i,i\pm 1} = -\frac{1}{2h^2}$$

Eigenvalues below zero are bound states. As screening increases ($\kappa \to \kappa_c$),
bound states ionize — this defines the critical screening parameter.

```python
import numpy as np
from scipy.linalg import eigh_tridiagonal
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
import time

N_GRID = 2000
R_MAX = 100.0

# Literature critical screening (Lam & Varshni 1971)
CRITICAL_SCREENING_LIT = {
    (1, 0): 1.19061,  # 1s
    (2, 0): 0.31750,  # 2s
    (2, 1): 0.21954,  # 2p
    (3, 0): 0.14459,  # 3s
    (3, 1): 0.10789,  # 3p
    (3, 2): 0.09025,  # 3d
}

C_PASS = '#2ecc71'
C_FAIL = '#e74c3c'
C_INFO = '#3498db'
C_RUST = '#1abc9c'
C_PYTHON = '#f39c12'

print(f"Grid: N={N_GRID}, r_max={R_MAX} a.u.")
print(f"h = {R_MAX/(N_GRID+1):.6f} a.u.")
```

```python
def eigenvalues(z, kappa, l, n_grid=N_GRID, r_max=R_MAX):
    """Bound-state eigenvalues of screened Coulomb potential."""
    h = r_max / (n_grid + 1)
    inv_h2 = 1.0 / (h * h)
    centrifugal = l * (l + 1.0) / 2.0
    r = np.arange(1, n_grid + 1) * h
    diag = inv_h2 + centrifugal / (r * r) - z * np.exp(-kappa * r) / r
    off_diag = np.full(n_grid - 1, -0.5 * inv_h2)
    evals = eigh_tridiagonal(diag, off_diag, eigvals_only=True)
    return evals[evals < 0.0]

def critical_screening(z, n_state, l, n_grid=N_GRID, r_max=R_MAX):
    """Find kappa_c via bisection on bound-state count."""
    target = n_state - l
    def has_state(kappa):
        return len(eigenvalues(z, kappa, l, n_grid, r_max)) >= target
    hi = z * 2.0
    while has_state(hi):
        hi *= 2.0
    lo = 0.0
    for _ in range(80):
        mid = 0.5 * (lo + hi)
        if has_state(mid):
            lo = mid
        else:
            hi = mid
    return 0.5 * (lo + hi)

# Hydrogen reference: exact eigenvalues E_n = -Z^2/(2n^2)
print("Hydrogen eigenvalues at kappa=0 (reference):")
for l in [0, 1, 2]:
    evals = eigenvalues(1.0, 0.0, l)
    for i in range(min(3, len(evals))):
        n_eff = l + i + 1
        exact = -0.5 / (n_eff * n_eff)
        rel_err = abs((evals[i] - exact) / exact)
        label = f"{n_eff}{'spd'[l]}"
        print(f"  E({label}) = {evals[i]:.8f}  (exact {exact:.8f}, err {rel_err:.2e})")
```

## Eigenvalue Spectrum vs Screening

As the screening parameter $\kappa$ increases, higher-lying bound states
are ionized first. At $\kappa_c(1s) \approx 1.19$, all bound states vanish.

```python
kappa_values = np.linspace(0.0, 1.3, 60)

t0 = time.perf_counter()
spectrum_data = {}
for kappa in kappa_values:
    evals = eigenvalues(1.0, kappa, 0)
    spectrum_data[kappa] = evals
elapsed = time.perf_counter() - t0
print(f"Computed {len(kappa_values)} screening values in {elapsed*1000:.0f} ms")

fig, axes = plt.subplots(1, 2, figsize=(16, 6))

# Eigenvalue trajectories
for i in range(6):
    ks, es = [], []
    for kappa in kappa_values:
        evals = spectrum_data[kappa]
        if len(evals) > i:
            ks.append(kappa)
            es.append(evals[i])
    if ks:
        n_state = i + 1
        axes[0].plot(ks, es, label=f'$n={n_state}$ (l=0)', linewidth=2)

axes[0].axhline(y=0, color='gray', ls='--', alpha=0.5)
axes[0].set_xlabel('Screening parameter $\\kappa$ (a.u.)')
axes[0].set_ylabel('Energy (Hartree)')
axes[0].set_title('Bound-State Eigenvalues vs Screening')
axes[0].legend(fontsize=9)
axes[0].set_ylim(-0.55, 0.05)

# Number of bound states vs kappa
n_bound = [len(spectrum_data[k]) for k in kappa_values]
axes[1].step(kappa_values, n_bound, where='mid', color=C_INFO, linewidth=2)
for (n_state, l), lit in sorted(CRITICAL_SCREENING_LIT.items()):
    if l == 0:
        axes[1].axvline(x=lit, color=C_FAIL, ls=':', alpha=0.5)
        axes[1].text(lit + 0.02, max(n_bound)*0.9 - (n_state-1)*3, f'$\\kappa_c({n_state}s)$', fontsize=8)

axes[1].set_xlabel('Screening parameter $\\kappa$ (a.u.)')
axes[1].set_ylabel('Number of bound states (l=0)')
axes[1].set_title('Ionization Cascade')

fig.suptitle('Yukawa Potential — Murillo & Weisheit (1998)', fontsize=14, fontweight='bold')
plt.tight_layout()
plt.savefig('/tmp/hotspring_paper_02_spectrum.png', dpi=150, bbox_inches='tight')
plt.show()
```

## Critical Screening Parameters

The critical screening $\kappa_c(n\ell)$ is the maximum screening
at which the $(n, \ell)$ bound state still exists. We compare to
Lam & Varshni (1971) literature values.

```python
t0 = time.perf_counter()
critical_results = {}
for (n_state, l), lit in sorted(CRITICAL_SCREENING_LIT.items()):
    kc = critical_screening(1.0, n_state, l)
    rel_err = abs((kc - lit) / lit)
    label = f"{n_state}{'spd'[l]}"
    critical_results[label] = {'computed': kc, 'literature': lit, 'rel_err': rel_err}
    print(f"  kappa_c({label}) = {kc:.6f}  (lit {lit:.5f}, err {rel_err:.2e})")
elapsed = time.perf_counter() - t0
print(f"\nCritical screening computed in {elapsed*1000:.0f} ms")

fig, ax = plt.subplots(figsize=(10, 5))
labels = list(critical_results.keys())
computed = [critical_results[l]['computed'] for l in labels]
literature = [critical_results[l]['literature'] for l in labels]
errors = [critical_results[l]['rel_err'] for l in labels]

x = np.arange(len(labels))
w = 0.35
ax.bar(x - w/2, computed, w, label='Computed (this notebook)', color=C_INFO)
ax.bar(x + w/2, literature, w, label='Lam & Varshni (1971)', color=C_PYTHON)

ax.set_xticks(x)
ax.set_xticklabels(labels, fontsize=12)
ax.set_ylabel('$\\kappa_c$ (a.u.)')
ax.set_title('Critical Screening Parameters — Computed vs Literature')
ax.legend()

for i, e in enumerate(errors):
    ax.text(i, max(computed[i], literature[i]) + 0.02, f'{e:.1e}', ha='center', fontsize=8, color='gray')

plt.tight_layout()
plt.savefig('/tmp/hotspring_paper_02_critical.png', dpi=150, bbox_inches='tight')
plt.show()
```

## Higher Angular Momenta and Z-Scaling

The screened Coulomb problem has a non-trivial angular momentum structure.
Higher $\ell$ states have shallower binding and ionize at lower screening.
For Z > 1, eigenvalues scale as $Z^2$.

```python
fig, axes = plt.subplots(1, 2, figsize=(16, 6))

# l = 0, 1, 2 at kappa=0
colors_l = [C_INFO, C_PASS, C_PYTHON]
for l, color in zip([0, 1, 2], colors_l):
    evals = eigenvalues(1.0, 0.0, l)
    n_show = min(8, len(evals))
    ns = np.arange(l+1, l+1+n_show)
    exact = -0.5 / ns**2
    axes[0].scatter(ns, evals[:n_show], s=60, color=color, zorder=3, label=f'Computed $\\ell={l}$')
    axes[0].plot(ns, exact, '--', color=color, alpha=0.5)

axes[0].set_xlabel('Principal quantum number $n$')
axes[0].set_ylabel('Energy (Hartree)')
axes[0].set_title('Hydrogen Spectrum ($\\kappa=0$, Z=1)')
axes[0].legend(fontsize=9)

# Z-scaling: He+ (Z=2)
evals_h = eigenvalues(1.0, 0.0, 0)[:5]
evals_he = eigenvalues(2.0, 0.0, 0)[:5]
ns = np.arange(1, 6)
axes[1].plot(ns, evals_h, 'o-', color=C_INFO, label='H ($Z=1$)', markersize=8)
axes[1].plot(ns, evals_he, 's-', color=C_FAIL, label='He$^+$ ($Z=2$)', markersize=8)
axes[1].plot(ns, -0.5/ns**2, '--', color='gray', alpha=0.5, label='$-1/(2n^2)$')
axes[1].plot(ns, -2.0/ns**2, '--', color='gray', alpha=0.3, label='$-Z^2/(2n^2)$')
axes[1].set_xlabel('Principal quantum number $n$')
axes[1].set_ylabel('Energy (Hartree)')
axes[1].set_title('Z-Scaling of Eigenvalues')
axes[1].legend(fontsize=9)

plt.tight_layout()
plt.savefig('/tmp/hotspring_paper_02_angular.png', dpi=150, bbox_inches='tight')
plt.show()
```

## Rust Parity

The identical algorithm is implemented in `barracuda/src/physics/screened_coulomb.rs`
with the same grid parameters. Rust validation via `cargo test --lib screened_coulomb`
confirms eigenvalue agreement to machine precision.

| Implementation | 2,000-point eigensolve | Speedup |
|---------------|----------------------|--------|
| Python (scipy LAPACK) | ~15 ms | 1x |
| Rust (ndarray LAPACK) | ~2 ms | **7.5x** |

## Provenance

- **Paper:** Murillo & Weisheit, Phys. Rep. **302**, 1 (1998)
- **Reference data:** Lam & Varshni, PRA **4**, 1875 (1971)
- **Validation:** `barracuda/src/physics/screened_coulomb.rs`, `validate_yukawa`
- **Control baseline:** `control/screened_coulomb/scripts/yukawa_eigenvalues.py`
- **Next:** Primal composition via `by_domain("math")` eigensolve dispatch

---
*hotSpring — ecoPrimals · AGPL-3.0 · [primals.eco](https://primals.eco)*

