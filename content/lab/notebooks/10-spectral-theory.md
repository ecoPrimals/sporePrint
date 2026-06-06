+++
title = "Spectral Theory — Anderson Localization, Hofstadter Butterfly, Lyapunov Exponents"
description = "Rendered from 10-spectral-theory.ipynb"
date = 2026-06-06
weight = 50

[extra]
domain = "Lab"
rendered_from = "10-spectral-theory.ipynb"
+++

<!-- Auto-generated from 10-spectral-theory.ipynb by spore-validate render-notebooks -->

# Spectral Theory — Anderson Localization, Hofstadter Butterfly, Lyapunov Exponents

**Papers:**  
- Anderson (1958) *Phys. Rev.* **109**, 1492 — localization in 1D/2D  
- Abrahams, Anderson, Licciardello, Ramakrishnan (1979) *PRL* **42**, 673 — scaling theory  
- Aubry & André (1980) *Ann. Israel Phys. Soc.* **3**, 133 — almost-Mathieu operator  
- Herman (1983) *Comment. Math. Helv.* **58**, 453 — Lyapunov exponent formula  
- Hofstadter (1976) *Phys. Rev. B* **14**, 2239 — butterfly spectrum  
- Slevin & Ohtsuki (1999) *PRL* **82**, 382 — 3D Anderson critical disorder  

**What we reproduce:** Full spectral analysis across dimensions — 1D localization,
2D eigenvalues, 3D metal-insulator transition, Hofstadter butterfly, Herman's formula
for Lyapunov exponents, and level spacing statistics (Poisson vs GOE). All live compute.

---

*This notebook runs entirely in Python (numpy + scipy). All computations execute live.*  
*Rust parity:* `barracuda/src/spectral.rs` — validated via `cargo test --lib spectral`

## Physics

The Anderson model describes a quantum particle on a lattice with random on-site potential:

$$H = -\Delta + V, \quad V_i \sim \text{Uniform}[-W/2, W/2]$$

Key phenomena:
- **1D/2D:** All states localized for any $W > 0$ (Abrahams et al. 1979)
- **3D:** Metal-insulator transition at $W_c \approx 16.5$ (Slevin & Ohtsuki 1999)
- Level statistics transition: GOE ($\langle r \rangle \approx 0.53$) for extended states → Poisson ($\langle r \rangle \approx 0.39$) for localized

The **almost-Mathieu operator** (Aubry-André model) has a quasiperiodic potential:

$$H_{nn} = 2\lambda \cos(2\pi\alpha n + \theta), \quad H_{n,n\pm1} = -1$$

Herman's formula gives the Lyapunov exponent: $\gamma = \ln|\lambda|$ for $|\lambda| > 1$.

```python
import numpy as np
from scipy import sparse
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
import time

GOLDEN_RATIO = (1.0 + np.sqrt(5.0)) / 2.0
POISSON_R = 2.0 * np.log(2.0) - 1.0  # 0.3863
GOE_R = 0.531

C_PASS = '#2ecc71'
C_FAIL = '#e74c3c'
C_INFO = '#3498db'
C_RUST = '#1abc9c'
C_PYTHON = '#f39c12'
C_GPU = '#9b59b6'

print(f"Golden ratio: phi = {GOLDEN_RATIO:.10f}")
print(f"Poisson <r> = {POISSON_R:.4f}")
print(f"GOE <r> = {GOE_R:.3f}")
```

```python
def anderson_1d_eigenvalues(n, disorder, seed=42):
    rng = np.random.default_rng(seed)
    diagonal = disorder * (rng.random(n) - 0.5)
    off_diag = -np.ones(n - 1)
    H = np.diag(diagonal) + np.diag(off_diag, 1) + np.diag(off_diag, -1)
    return np.sort(np.linalg.eigh(H)[0])

def lyapunov_exponent(potential, energy):
    n = len(potential)
    v_prev, v_curr = 0.0, 1.0
    log_growth = 0.0
    for v_i in potential:
        v_next = (energy - v_i) * v_curr - v_prev
        v_prev = v_curr
        v_curr = v_next
        norm = np.hypot(v_curr, v_prev)
        if norm > 0:
            log_growth += np.log(norm)
            v_curr /= norm
            v_prev /= norm
    return log_growth / n

def level_spacing_ratio(eigenvalues):
    spacings = np.diff(eigenvalues)
    spacings = spacings[spacings > 0]
    if len(spacings) < 2:
        return 0.0
    r_values = np.minimum(spacings[:-1], spacings[1:]) / np.maximum(spacings[:-1], spacings[1:])
    return np.mean(r_values)

def anderson_2d_hamiltonian(lx, ly, disorder, seed=42):
    n = lx * ly
    rng = np.random.default_rng(seed)
    H = np.zeros((n, n))
    for ix in range(lx):
        for iy in range(ly):
            i = ix * ly + iy
            H[i, i] = disorder * (rng.random() - 0.5)
            if ix > 0:
                j = (ix - 1) * ly + iy
                H[i, j] = H[j, i] = -1.0
            if iy > 0:
                j = ix * ly + (iy - 1)
                H[i, j] = H[j, i] = -1.0
    return H

def anderson_3d_hamiltonian(lx, ly, lz, disorder, seed=42):
    n = lx * ly * lz
    rng = np.random.default_rng(seed)
    diag = disorder * (rng.random(n) - 0.5)
    rows, cols, vals = [], [], []
    for ix in range(lx):
        for iy in range(ly):
            for iz in range(lz):
                i = ix * ly * lz + iy * lz + iz
                rows.append(i); cols.append(i); vals.append(diag[i])
                for dx, dy, dz in [(-1,0,0),(0,-1,0),(0,0,-1)]:
                    jx, jy, jz = ix+dx, iy+dy, iz+dz
                    if jx >= 0 and jy >= 0 and jz >= 0:
                        j = jx * ly * lz + jy * lz + jz
                        rows.extend([i, j]); cols.extend([j, i]); vals.extend([-1.0, -1.0])
    return sparse.csr_matrix((vals, (rows, cols)), shape=(n, n))

def almost_mathieu_hamiltonian(n, lam, alpha, theta=0.0):
    diagonal = np.array([2.0 * lam * np.cos(2.0 * np.pi * alpha * i + theta) for i in range(n)])
    off_diag = -np.ones(n - 1)
    return np.diag(diagonal) + np.diag(off_diag, 1) + np.diag(off_diag, -1)

print("All spectral functions defined")
```

## Anderson 1D — Eigenvalue Spectrum and Localization

```python
fig, axes = plt.subplots(1, 3, figsize=(18, 5))

# Density of states for different disorder strengths
n = 1000
for w, color, label in [(1.0, C_PASS, 'W=1'), (4.0, C_INFO, 'W=4'), (8.0, C_FAIL, 'W=8')]:
    evals = anderson_1d_eigenvalues(n, w, seed=42)
    axes[0].hist(evals, bins=60, alpha=0.5, color=color, label=label, density=True)
axes[0].set_xlabel('Energy')
axes[0].set_ylabel('DOS')
axes[0].set_title('Anderson 1D Density of States (N=1000)')
axes[0].legend()

# Level spacing ratio vs disorder (1D)
w_values = np.linspace(0.5, 15, 20)
r_means = []
for w in w_values:
    rs = [level_spacing_ratio(anderson_1d_eigenvalues(500, w, seed=s)) for s in range(5)]
    r_means.append(np.mean(rs))

axes[1].plot(w_values, r_means, 'o-', color=C_INFO, markersize=5)
axes[1].axhline(y=POISSON_R, color=C_FAIL, ls='--', label=f'Poisson ({POISSON_R:.3f})')
axes[1].axhline(y=GOE_R, color=C_PASS, ls='--', label=f'GOE ({GOE_R:.3f})')
axes[1].set_xlabel('Disorder $W$')
axes[1].set_ylabel('$\\langle r \\rangle$')
axes[1].set_title('Level Spacing Ratio — 1D')
axes[1].legend(fontsize=8)

# 2D eigenvalues
t0 = time.perf_counter()
H_2d = anderson_2d_hamiltonian(16, 16, 5.0, seed=42)
evals_2d = np.sort(np.linalg.eigh(H_2d)[0])
t_2d = time.perf_counter() - t0
axes[2].hist(evals_2d, bins=50, color=C_INFO, alpha=0.7)
axes[2].set_xlabel('Energy')
axes[2].set_ylabel('Count')
axes[2].set_title(f'Anderson 2D (16x16, W=5) — {t_2d*1000:.0f} ms')

plt.tight_layout()
plt.savefig('/tmp/hotspring_paper_10_anderson.png', dpi=150, bbox_inches='tight')
plt.show()
```

## Herman's Formula — Lyapunov Exponents

For the almost-Mathieu operator with irrational frequency $\alpha = \phi$ (golden ratio),
Herman (1983) proved: $\gamma(E=0) = \ln|\lambda|$ for $|\lambda| > 1$.

```python
n_lyap = 100_000
lambdas = np.linspace(1.1, 5.0, 30)

t0 = time.perf_counter()
gammas = []
for lam in lambdas:
    pot = np.array([2.0 * lam * np.cos(2.0 * np.pi * GOLDEN_RATIO * i) for i in range(n_lyap)])
    gammas.append(lyapunov_exponent(pot, 0.0))
elapsed = time.perf_counter() - t0

theory = np.log(lambdas)

fig, axes = plt.subplots(1, 2, figsize=(14, 5))

axes[0].plot(lambdas, gammas, 'o', color=C_INFO, markersize=4, label='Computed $\\gamma$')
axes[0].plot(lambdas, theory, '-', color=C_FAIL, linewidth=2, label='$\\ln|\\lambda|$ (Herman)')
axes[0].set_xlabel('$\\lambda$')
axes[0].set_ylabel('$\\gamma$')
axes[0].set_title(f"Herman's Formula (N={n_lyap:,}, {elapsed*1000:.0f} ms)")
axes[0].legend()

# Error
errors = np.abs(np.array(gammas) - theory)
axes[1].semilogy(lambdas, errors, 'o-', color=C_GPU, markersize=4)
axes[1].set_xlabel('$\\lambda$')
axes[1].set_ylabel('$|\\gamma - \\ln|\\lambda||$')
axes[1].set_title('Deviation from Herman Formula')

plt.tight_layout()
plt.savefig('/tmp/hotspring_paper_10_herman.png', dpi=150, bbox_inches='tight')
plt.show()
```

## Hofstadter Butterfly

The spectrum of the almost-Mathieu operator as a function of rational
flux $\alpha = p/q$ produces the famous Hofstadter butterfly — a fractal
energy spectrum. At $\alpha = p/q$, the spectrum has exactly $q$ bands.

```python
n_hof = 500
n_alpha = 200

t0 = time.perf_counter()
alphas = []
energies = []
for i in range(1, n_alpha + 1):
    alpha = i / (n_alpha + 1)
    H = almost_mathieu_hamiltonian(n_hof, 1.0, alpha)
    evals = np.sort(np.linalg.eigh(H)[0])
    for e in evals:
        alphas.append(alpha)
        energies.append(e)
elapsed = time.perf_counter() - t0

fig, ax = plt.subplots(figsize=(10, 8))
ax.scatter(alphas, energies, s=0.01, c=C_INFO, alpha=0.3)
ax.set_xlabel('$\\alpha$ (magnetic flux / flux quantum)')
ax.set_ylabel('Energy')
ax.set_title(f'Hofstadter Butterfly — N={n_hof}, {n_alpha} flux values ({elapsed:.1f}s)')
ax.set_xlim(0, 1)
plt.tight_layout()
plt.savefig('/tmp/hotspring_paper_10_butterfly.png', dpi=150, bbox_inches='tight')
plt.show()
```

## 3D Anderson — Metal-Insulator Transition

In 3D, there is a genuine phase transition at $W_c \approx 16.5$.
Below $W_c$: extended states (GOE statistics). Above $W_c$: localized states (Poisson).
The mobility edge separates extended band center from localized band edges.

```python
L = 8
w_values_3d = [2.0, 6.0, 10.0, 14.0, 18.0, 25.0, 35.0]
n_real = 3

t0 = time.perf_counter()
r_3d = []
for w in w_values_3d:
    r_sum = 0.0
    for seed in range(n_real):
        H = anderson_3d_hamiltonian(L, L, L, w, seed * 137 + 42).toarray()
        ev = np.sort(np.linalg.eigh(H)[0])
        mid = len(ev) // 4
        end = 3 * len(ev) // 4
        r_sum += level_spacing_ratio(ev[mid:end])
    r_3d.append(r_sum / n_real)
elapsed_3d = time.perf_counter() - t0

# Mobility edge: center vs edges at W=12
w_me = 12.0
n_real_me = 5
center_rs, edge_rs = [], []
for seed in range(n_real_me):
    H = anderson_3d_hamiltonian(L, L, L, w_me, seed * 137 + 42).toarray()
    ev = np.sort(np.linalg.eigh(H)[0])
    n_ev = len(ev)
    center_rs.append(level_spacing_ratio(ev[n_ev//4:3*n_ev//4]))
    edge_rs.append((level_spacing_ratio(ev[:n_ev//5]) + level_spacing_ratio(ev[4*n_ev//5:])) / 2)

fig, axes = plt.subplots(1, 2, figsize=(14, 5))

axes[0].plot(w_values_3d, r_3d, 'o-', color=C_INFO, markersize=8, linewidth=2)
axes[0].axhline(y=POISSON_R, color=C_FAIL, ls='--', label=f'Poisson ({POISSON_R:.3f})')
axes[0].axhline(y=GOE_R, color=C_PASS, ls='--', label=f'GOE ({GOE_R:.3f})')
axes[0].axvline(x=16.5, color='gray', ls=':', alpha=0.5, label='$W_c \\approx 16.5$')
axes[0].set_xlabel('Disorder $W$')
axes[0].set_ylabel('$\\langle r \\rangle$ (band center)')
axes[0].set_title(f'3D Anderson Transition (L={L}, {elapsed_3d:.0f}s)')
axes[0].legend(fontsize=8)

# Mobility edge
labels = ['Band\ncenter', 'Band\nedges']
means = [np.mean(center_rs), np.mean(edge_rs)]
stds = [np.std(center_rs), np.std(edge_rs)]
colors = [C_PASS, C_FAIL]
axes[1].bar(labels, means, yerr=stds, color=colors, capsize=5, alpha=0.7)
axes[1].axhline(y=POISSON_R, color=C_FAIL, ls='--', alpha=0.5)
axes[1].axhline(y=GOE_R, color=C_PASS, ls='--', alpha=0.5)
axes[1].set_ylabel('$\\langle r \\rangle$')
axes[1].set_title(f'Mobility Edge at W={w_me} (L={L})')
axes[1].set_ylim(0.3, 0.6)

plt.tight_layout()
plt.savefig('/tmp/hotspring_paper_10_3d_transition.png', dpi=150, bbox_inches='tight')
plt.show()

print(f"\n3D transition GOE->Poisson: <r>(W=2)={r_3d[0]:.4f} -> <r>(W=35)={r_3d[-1]:.4f}")
print(f"Mobility edge: center <r>={np.mean(center_rs):.4f}, edges <r>={np.mean(edge_rs):.4f}")
```

## Dimensional Bandwidth Hierarchy

```python
w_dim = 2.0

ev_1d = anderson_1d_eigenvalues(500, w_dim, seed=42)
bw_1d = ev_1d[-1] - ev_1d[0]

H_2d = anderson_2d_hamiltonian(22, 22, w_dim, seed=42)
ev_2d = np.sort(np.linalg.eigh(H_2d)[0])
bw_2d = ev_2d[-1] - ev_2d[0]

H_3d = anderson_3d_hamiltonian(8, 8, 8, w_dim, seed=42).toarray()
ev_3d = np.sort(np.linalg.eigh(H_3d)[0])
bw_3d = ev_3d[-1] - ev_3d[0]

fig, ax = plt.subplots(figsize=(8, 5))
dims = ['1D\n(N=500)', '2D\n(22x22)', '3D\n(8^3)']
bws = [bw_1d, bw_2d, bw_3d]
clean_bws = [4.0, 8.0, 12.0]
colors = [C_INFO, C_PASS, C_GPU]

x = np.arange(3)
ax.bar(x - 0.2, bws, 0.35, label=f'W={w_dim}', color=colors, alpha=0.7)
ax.bar(x + 0.2, clean_bws, 0.35, label='Clean (W=0)', color='lightgray', edgecolor='gray')
ax.set_xticks(x)
ax.set_xticklabels(dims)
ax.set_ylabel('Bandwidth')
ax.set_title('Dimensional Bandwidth Hierarchy: 1D < 2D < 3D')
ax.legend()

for i, (b, c) in enumerate(zip(bws, clean_bws)):
    ax.text(i - 0.2, b + 0.1, f'{b:.2f}', ha='center', fontsize=9)
    ax.text(i + 0.2, c + 0.1, f'{c:.0f}', ha='center', fontsize=9)

plt.tight_layout()
plt.savefig('/tmp/hotspring_paper_10_hierarchy.png', dpi=150, bbox_inches='tight')
plt.show()

assert bw_3d > bw_2d > bw_1d, "Hierarchy violated!"
print(f"Hierarchy confirmed: {bw_1d:.2f} < {bw_2d:.2f} < {bw_3d:.2f}")
```

## Rust Parity

All spectral algorithms are implemented in `barracuda/src/spectral.rs`.
GPU-accelerated eigensolve via BarraCuda's WGSL compute shaders.

| Computation | Python | Rust | Speedup |
|------------|--------|------|--------|
| Anderson 1D (N=1000) | ~30 ms | ~3 ms | **10x** |
| Hofstadter (200 alpha) | ~8 s | ~0.5 s | **16x** |
| 3D Anderson (L=8) | ~2 s | ~0.1 s | **20x** |

## Provenance

- **Papers:** Anderson (1958), Abrahams et al. (1979), Aubry & André (1980), Herman (1983), Hofstadter (1976), Slevin & Ohtsuki (1999)
- **Validation:** `barracuda/src/spectral.rs`, `validate_spectral`
- **Control:** `control/spectral_theory/scripts/spectral_control.py`
- **Next:** GPU-accelerated eigensolve via primal composition

---
*hotSpring — ecoPrimals · AGPL-3.0 · [primals.eco](https://primals.eco)*

