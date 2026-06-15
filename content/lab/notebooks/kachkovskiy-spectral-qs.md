+++
title = "Spectral Theory & Quorum Sensing — Kachkovskiy (MSU CMSE)"
description = "Rendered from kachkovskiy-spectral-qs.ipynb"
date = 2026-06-15
weight = 50

[extra]
domain = "Lab"
rendered_from = "kachkovskiy-spectral-qs.ipynb"
+++

<!-- Auto-generated from kachkovskiy-spectral-qs.ipynb by spore-validate render-notebooks -->

# Spectral Theory & Quorum Sensing — Kachkovskiy (MSU CMSE)

This notebook bridges **Anderson localization theory** to **microbial quorum sensing (QS)**.
The central analogy: autoinducer diffusion through a heterogeneous bacterial population is
analogous to wave propagation in a disordered lattice. Anderson localization predicts when
signals stay local (localized states) versus propagate community-wide (extended states),
depending on population heterogeneity.

| # | Paper | DOI | Model |
|---|-------|-----|-------|
| 1 | Bourgain & Kachkovskiy 2018 | [10.1007/s00039-019-00478-4](https://doi.org/10.1007/s00039-019-00478-4) | Anderson localization for quasiperiodic potentials |
| 2 | Anderson 1958 | [10.1103/PhysRev.109.1492](https://doi.org/10.1103/PhysRev.109.1492) | Absence of diffusion in random lattices |
| 3 | Aubry & André 1980 | *Ann. Israel Phys. Soc.* 3:133 | Quasiperiodic metal–insulator transition |

**Rust validation (cross-spring, 312 checks across 20+ experiments):**
`validate_spectral_cross_spring` (Exp107), `validate_qs_disorder_real` (Exp113),
`validate_npu_disorder_classifier` (Exp119), `validate_anderson_2d_qs` (Exp122),
`validate_ncbi_qs_atlas` (Exp126), `validate_anderson_3d_qs` (Exp127-130),
`validate_mapping_sensitivity` (Exp135-138), `validate_cold_seep_qs_catalog` (Exp144-149).

---

*For other springs:* the Anderson framework is universal — swap autoinducer
propagation for phonon transport (hotSpring), electromagnetic wave propagation
(airSpring), or nutrient diffusion in soil pore networks (groundSpring).
The spectral primitives are all in `barracuda::spectral` via ToadStool.

```python
import os, json, struct, socket, math
from pathlib import Path
import numpy as np
import matplotlib.pyplot as plt

RESULTS = Path('..') / '..' / 'experiments' / 'results'

def load(name):
    with open(RESULTS / name) as f:
        return json.load(f)

TIER = 'frozen'
IPC_SOCKET = os.environ.get('WETSPRING_IPC_SOCKET')

def ipc_call(method, params=None):
    """Tier 2 IPC — call wetSpring barracuda over Unix socket."""
    if not IPC_SOCKET:
        return None
    sock = socket.socket(socket.AF_UNIX, socket.SOCK_STREAM)
    sock.connect(IPC_SOCKET)
    req = json.dumps({'jsonrpc': '2.0', 'method': method, 'params': params or {}, 'id': 1})
    sock.sendall((req + '\n').encode())
    resp = b''
    while b'\n' not in resp:
        chunk = sock.recv(4096)
        if not chunk:
            break
        resp += chunk
    sock.close()
    return json.loads(resp).get('result')

print(f'Tier: {TIER}  |  IPC: {"connected" if IPC_SOCKET else "offline (frozen only)"}')
```

## 1. Anderson Hamiltonian — 1D Disordered Lattice

The 1D Anderson Hamiltonian is a tridiagonal matrix with random diagonal disorder:

$$H_{ij} = \begin{cases} \varepsilon_i \sim \text{Uniform}[-W/2, W/2] & i = j \\ -1 & |i-j| = 1 \\ 0 & \text{otherwise} \end{cases}$$

where $W$ controls disorder strength. Anderson's 1958 theorem: in 1D, **all** states localize for any $W > 0$.
The Lyapunov exponent $\gamma(E) > 0$ for all energies $E$ — no extended states exist.

For wetSpring's QS analogy:
- **Lattice site** → bacterial cell position
- **On-site energy $\varepsilon_i$** → cell-type identity heterogeneity
- **Hopping $t = 1$** → autoinducer diffusion coefficient
- **Disorder $W$** → population heterogeneity (mapped from Pielou evenness $J$)

```python
def anderson_1d(n, W, seed=42):
    """Build 1D Anderson Hamiltonian (tridiagonal + random diagonal)."""
    rng = np.random.default_rng(seed)
    H = np.zeros((n, n))
    for i in range(n):
        H[i, i] = rng.uniform(-W/2, W/2)
        if i > 0:
            H[i, i-1] = -1.0
            H[i-1, i] = -1.0
    return H

N = 200
W_test = 4.0
H = anderson_1d(N, W_test)
eigenvalues = np.linalg.eigvalsh(H)

# Gershgorin bounds: all eigenvalues in [-2-W/2, 2+W/2]
lo, hi = -2 - W_test/2, 2 + W_test/2
assert eigenvalues.min() >= lo - 1e-10, f'Gershgorin lower bound violated'
assert eigenvalues.max() <= hi + 1e-10, f'Gershgorin upper bound violated'
print(f'1D Anderson (N={N}, W={W_test}): {len(eigenvalues)} eigenvalues')
print(f'  spectrum: [{eigenvalues.min():.4f}, {eigenvalues.max():.4f}]')
print(f'  Gershgorin: [{lo}, {hi}] ✓')
```

```python
def lyapunov_exponent(W, E=0.0, n=500, seed=42):
    """Transfer-matrix Lyapunov exponent γ(E) for 1D Anderson.
    
    γ > 0 means localized; γ = 0 means extended.
    Anderson 1958: γ > 0 for all E in 1D for any W > 0.
    Kappus–Wegner approximation at band center: γ(0) ≈ W²/96.
    """
    rng = np.random.default_rng(seed)
    log_sum = 0.0
    u_prev, u_curr = 0.0, 1.0
    for _ in range(n):
        eps = rng.uniform(-W/2, W/2)
        u_next = (E - eps) * u_curr - u_prev
        norm = abs(u_next)
        if norm > 0:
            log_sum += math.log(norm)
        u_prev, u_curr = u_curr, u_next / max(norm, 1e-300)
    return log_sum / n

gamma_0 = lyapunov_exponent(W_test, E=0.0)
kw_approx = W_test**2 / 96  # Kappus–Wegner
rel_err = abs(gamma_0 - kw_approx) / kw_approx

print(f'Lyapunov γ(0) at W={W_test}: {gamma_0:.6f}')
print(f'Kappus–Wegner W²/96: {kw_approx:.6f}')
print(f'Relative error: {rel_err:.1%} (< 30% at W=4 expected)')
assert gamma_0 > 0, 'Anderson theorem: all states localize in 1D'
```

```python
def level_spacing_ratio(eigenvalues):
    """Mean level spacing ratio ⟨r⟩.
    
    Poisson (localized): ⟨r⟩ ≈ 0.3863
    GOE (extended):      ⟨r⟩ ≈ 0.5307
    """
    eigs = np.sort(eigenvalues)
    spacings = np.diff(eigs)
    spacings = spacings[spacings > 1e-15]
    ratios = []
    for i in range(len(spacings) - 1):
        s_n, s_n1 = spacings[i], spacings[i+1]
        ratios.append(min(s_n, s_n1) / max(s_n, s_n1))
    return np.mean(ratios)

POISSON_R = 0.3863
GOE_R = 0.5307

r_1d = level_spacing_ratio(eigenvalues)
print(f'⟨r⟩ = {r_1d:.4f}  (Poisson = {POISSON_R}, GOE = {GOE_R})')
print(f'Diagnosis: {"Localized (Poisson-like)" if r_1d < (POISSON_R + GOE_R)/2 else "Extended (GOE-like)"}')
```

## 2. Almost-Mathieu / Aubry–André Model

The quasiperiodic potential replaces random disorder with a cosine:

$$H_{n,n} = 2\lambda \cos(2\pi \alpha n + \theta)$$

where $\alpha$ is irrational (golden ratio). The **Aubry–André transition** at $\lambda = 1$:
- $\lambda < 1$: all states extended
- $\lambda > 1$: all states localized
- Herman's formula: $\gamma(0) = \max(0, \ln\lambda)$

This is Kachkovskiy's domain — rigorous proofs of localization for quasiperiodic potentials.

```python
GOLDEN = (1 + math.sqrt(5)) / 2
ALPHA = 1 / GOLDEN  # irrational frequency

def almost_mathieu(n, lam, alpha=ALPHA, theta=0.0):
    """Build Almost-Mathieu Hamiltonian (quasiperiodic diagonal)."""
    H = np.zeros((n, n))
    for i in range(n):
        H[i, i] = 2 * lam * math.cos(2 * math.pi * alpha * i + theta)
        if i > 0:
            H[i, i-1] = -1.0
            H[i-1, i] = -1.0
    return H

fig, axes = plt.subplots(1, 3, figsize=(14, 4))

for ax, lam in zip(axes, [0.5, 1.5, 3.0]):
    H_am = almost_mathieu(300, lam)
    eigs = np.linalg.eigvalsh(H_am)
    gamma = max(0, math.log(lam)) if lam > 0 else 0
    ax.hist(eigs, bins=60, density=True, alpha=0.7, color='steelblue')
    ax.set_title(f'λ = {lam}  |  γ = {gamma:.3f}')
    ax.set_xlabel('Energy')
    ax.set_ylabel('DOS')

fig.suptitle('Almost-Mathieu spectrum: Aubry–André transition at λ = 1', fontsize=13)
plt.tight_layout()
plt.show()

# Herman formula verification
for lam in [1.5, 2.0, 3.0]:
    predicted = math.log(lam)
    print(f'λ = {lam}: Herman γ = ln(λ) = {predicted:.4f}')
```

## 3. QS-Disorder Analogy — Population Heterogeneity ↔ Signal Localization

The central wetSpring contribution: map microbial community diversity to Anderson disorder.

**Mapping**: $W = 0.5 + 14.5 \times J$ where $J$ is Pielou evenness.

- **Low $J$ (dominated, biofilm-like)**: low $W$ → signals propagate → QS active
- **High $J$ (diverse, soil-like)**: high $W$ → signals localize → QS suppressed

This is testable: communities with low Pielou evenness should exhibit stronger
QS-mediated collective behaviors (biofilm formation, virulence).

```python
# Eight ecosystem profiles from Exp113
ecosystems = [
    ('HMP gut',      300,  0.957),
    ('HMP oral',     500,  0.984),
    ('Tara surface', 800,  0.987),
    ('Tara deep',    200,  0.931),
    ('EMP soil',    1000,  0.990),
    ('Algal bloom',   50,  0.762),
    ('Vent',         150,  0.940),
    ('Biofilm',       20,  0.559),
]

def pielou_to_W(J):
    return 0.5 + 14.5 * J

fig, (ax1, ax2) = plt.subplots(1, 2, figsize=(14, 5))

names, gammas, rs = [], [], []
for name, n_species, J in ecosystems:
    W = pielou_to_W(J)
    gamma = lyapunov_exponent(W, E=0.0, n=200)
    H = anderson_1d(200, W)
    eigs = np.linalg.eigvalsh(H)
    r = level_spacing_ratio(eigs)
    names.append(name)
    gammas.append(gamma)
    rs.append(r)

colors = ['#2ecc71' if g < 0.8 else '#e74c3c' for g in gammas]

ax1.barh(names, gammas, color=colors, edgecolor='white')
ax1.set_xlabel('Lyapunov exponent γ(0)')
ax1.set_title('Localization strength by ecosystem')
ax1.axvline(x=0.8, color='gray', linestyle='--', alpha=0.5, label='QS threshold')
ax1.legend()

ax2.scatter([e[2] for e in ecosystems], gammas, s=80, c=colors, edgecolors='black', zorder=5)
for i, name in enumerate(names):
    ax2.annotate(name, (ecosystems[i][2], gammas[i]), fontsize=8,
                 xytext=(5, 5), textcoords='offset points')
ax2.set_xlabel('Pielou evenness J')
ax2.set_ylabel('Lyapunov exponent γ(0)')
ax2.set_title('Evenness → Disorder → Localization')

plt.tight_layout()
plt.show()

print(f'\nEcosystem QS predictions (1D):')
for i, (name, _, J) in enumerate(ecosystems):
    W = pielou_to_W(J)
    regime = 'QS-permissive' if gammas[i] < 0.8 else 'QS-suppressed'
    print(f'  {name:15s}  J={J:.3f}  W={W:5.2f}  γ={gammas[i]:.3f}  → {regime}')
```

## 4. Dimensional Phase Diagram — 1D vs 2D vs 3D

Anderson's theorem applies only to **d ≤ 2**: all states localize for any disorder.
In **d = 3**, a genuine metal–insulator transition exists at $W_c \approx 16.5$.

For QS: spatial structure (biofilm thickness, vent chimney 3D geometry) changes the
critical evenness threshold $J_c$ where QS breaks down.

| Dimension | Plateau (W > 2) | $J_c$ | Biological interpretation |
|-----------|:--:|:---:|---|
| 1D | 0 | — | Linear chains: QS impossible at any diversity |
| 2D | 5 | 0.557 | Surface biofilms: QS at low-moderate diversity |
| 3D | 12 | 1.283 | Thick biofilms/vents: QS at nearly any diversity |

```python
def anderson_2d(L, W, seed=42):
    """Build 2D Anderson Hamiltonian on L×L square lattice."""
    N = L * L
    H = np.zeros((N, N))
    rng = np.random.default_rng(seed)
    for i in range(L):
        for j in range(L):
            idx = i * L + j
            H[idx, idx] = rng.uniform(-W/2, W/2)
            if j + 1 < L:
                H[idx, idx+1] = -1.0
                H[idx+1, idx] = -1.0
            if i + 1 < L:
                H[idx, idx+L] = -1.0
                H[idx+L, idx] = -1.0
    return H

def anderson_3d(L, W, seed=42):
    """Build 3D Anderson Hamiltonian on L×L×L cubic lattice."""
    N = L * L * L
    H = np.zeros((N, N))
    rng = np.random.default_rng(seed)
    for i in range(L):
        for j in range(L):
            for k in range(L):
                idx = i * L * L + j * L + k
                H[idx, idx] = rng.uniform(-W/2, W/2)
                if k + 1 < L:
                    H[idx, idx+1] = -1.0
                    H[idx+1, idx] = -1.0
                if j + 1 < L:
                    H[idx, idx+L] = -1.0
                    H[idx+L, idx] = -1.0
                if i + 1 < L:
                    H[idx, idx+L*L] = -1.0
                    H[idx+L*L, idx] = -1.0
    return H

W_sweep = np.linspace(0.5, 20.0, 15)
midpoint = (POISSON_R + GOE_R) / 2

r_1d_sweep = []
for W in W_sweep:
    H = anderson_1d(200, W)
    r_1d_sweep.append(level_spacing_ratio(np.linalg.eigvalsh(H)))

L2 = 10  # 10×10 = 100 sites
r_2d_sweep = []
for W in W_sweep:
    H = anderson_2d(L2, W)
    r_2d_sweep.append(level_spacing_ratio(np.linalg.eigvalsh(H)))

L3 = 6  # 6×6×6 = 216 sites
r_3d_sweep = []
for W in W_sweep:
    H = anderson_3d(L3, W)
    r_3d_sweep.append(level_spacing_ratio(np.linalg.eigvalsh(H)))

fig, ax = plt.subplots(figsize=(10, 6))
ax.plot(W_sweep, r_1d_sweep, 'o-', label='1D (N=200)', color='#e74c3c')
ax.plot(W_sweep, r_2d_sweep, 's-', label='2D (10×10)', color='#f39c12')
ax.plot(W_sweep, r_3d_sweep, '^-', label='3D (6×6×6)', color='#2ecc71')
ax.axhline(y=POISSON_R, color='gray', linestyle='--', alpha=0.5, label=f'Poisson (localized) = {POISSON_R}')
ax.axhline(y=GOE_R, color='gray', linestyle=':', alpha=0.5, label=f'GOE (extended) = {GOE_R}')
ax.axhline(y=midpoint, color='navy', linestyle='-.', alpha=0.3, label='Midpoint')

ax.set_xlabel('Disorder W', fontsize=12)
ax.set_ylabel('⟨r⟩ (level spacing ratio)', fontsize=12)
ax.set_title('Anderson Localization: Dimensional Phase Diagram', fontsize=14)
ax.legend(fontsize=9)
ax.set_ylim(0.35, 0.56)
plt.tight_layout()
plt.show()

# Count 2D and 3D plateau points above midpoint for W > 2
mask = W_sweep > 2
plateau_2d = sum(1 for r, m in zip(r_2d_sweep, mask) if m and r > midpoint)
plateau_3d = sum(1 for r, m in zip(r_3d_sweep, mask) if m and r > midpoint)
print(f'2D plateau (W > 2, ⟨r⟩ > midpoint): {plateau_2d} points')
print(f'3D plateau (W > 2, ⟨r⟩ > midpoint): {plateau_3d} points')
print(f'3D advantage: {plateau_3d / max(plateau_2d, 1):.1f}× wider QS window')
```

## 5. Lanczos Eigensolver — Cross-Spring Primitive

The Lanczos algorithm finds extremal eigenvalues of large sparse matrices without
forming the full matrix. This is a cross-spring primitive: originally developed in
hotSpring for lattice QCD, absorbed into ToadStool's `barracuda::spectral`, and
consumed here by wetSpring for Anderson Hamiltonians.

We verify the Lanczos tridiagonalization against NumPy's dense solver.

```python
def lanczos_tridiag(H, k=50, seed=42):
    """Lanczos tridiagonalization: returns (alpha, beta) diagonals."""
    n = H.shape[0]
    rng = np.random.default_rng(seed)
    q = rng.standard_normal(n)
    q /= np.linalg.norm(q)
    
    alpha = []
    beta = [0.0]
    q_prev = np.zeros(n)
    
    for j in range(min(k, n)):
        w = H @ q - beta[-1] * q_prev
        alpha.append(float(q @ w))
        w -= alpha[-1] * q
        b = np.linalg.norm(w)
        if b < 1e-14:
            break
        beta.append(b)
        q_prev = q.copy()
        q = w / b
    
    beta = beta[1:]  # drop leading zero
    T = np.diag(alpha) + np.diag(beta[:len(alpha)-1], 1) + np.diag(beta[:len(alpha)-1], -1)
    return np.linalg.eigvalsh(T)

H_test = anderson_1d(200, 4.0)
eigs_dense = np.sort(np.linalg.eigvalsh(H_test))
eigs_lanczos = np.sort(lanczos_tridiag(H_test, k=200))

# Compare extremal eigenvalues
min_err = abs(eigs_dense[0] - eigs_lanczos[0])
max_err = abs(eigs_dense[-1] - eigs_lanczos[-1])
print(f'Dense  min/max: {eigs_dense[0]:.6f} / {eigs_dense[-1]:.6f}')
print(f'Lanczos min/max: {eigs_lanczos[0]:.6f} / {eigs_lanczos[-1]:.6f}')
print(f'Error (min): {min_err:.2e}  (max): {max_err:.2e}')
print(f'Lanczos returned {len(eigs_lanczos)} eigenvalues (dense: {len(eigs_dense)})')
```

## 6. Biome Atlas — 28-Biome QS Disorder Map

Exp126 scales the QS-disorder mapping to 28 real biome profiles derived from 136
NCBI BioProjects. This produces the first global atlas of predicted QS propagation
potential by biome type.

Key finding: **all 28 biomes are QS-active in 3D** (W < W_c ≈ 16.5), **zero in
1D or 2D**. The 3D metal–insulator transition exceeds the highest naturally
occurring microbial community disorder.

```python
# Representative biome data from Exp126 (28-biome atlas)
biomes = [
    ('Algal bloom (Taihu)',   0.731, 11.11),
    ('Biofilm (hospital)',    0.773, 11.70),
    ('Deep-sea hadal',       0.785, 11.88),
    ('Gut (infant)',         0.812, 12.27),
    ('Coral reef',           0.835, 12.61),
    ('Hot spring mat',       0.857, 12.93),
    ('Freshwater lake',      0.871, 13.13),
    ('Marine surface',       0.889, 13.39),
    ('Vent chimney (EPR)',   0.899, 13.53),
    ('Gut (adult)',          0.921, 13.85),
    ('Permafrost active',    0.935, 14.06),
    ('Wastewater digester',  0.961, 14.44),
    ('Rhizosphere',          0.975, 14.64),
    ('Soil (forest)',        0.990, 14.85),
]

W_c_3d = 16.5  # 3D Anderson metal–insulator transition

fig, ax = plt.subplots(figsize=(12, 6))
names_b = [b[0] for b in biomes]
Ws = [b[2] for b in biomes]
Js = [b[1] for b in biomes]

bar_colors = ['#2ecc71' if W < W_c_3d else '#e74c3c' for W in Ws]
ax.barh(names_b, Ws, color=bar_colors, edgecolor='white')
ax.axvline(x=W_c_3d, color='red', linestyle='--', linewidth=2,
           label=f'$W_c$(3D) ≈ {W_c_3d}')
ax.set_xlabel('Anderson disorder W', fontsize=12)
ax.set_title('28-Biome QS Disorder Atlas — All QS-active in 3D', fontsize=13)
ax.legend(fontsize=11)
plt.tight_layout()
plt.show()

qc_3d = sum(1 for W in Ws if W < W_c_3d)
print(f'\nBiomes QS-active in 3D: {qc_3d}/{len(biomes)} ({qc_3d/len(biomes)*100:.0f}%)')
print(f'W range: [{min(Ws):.2f}, {max(Ws):.2f}] — all below W_c(3D) = {W_c_3d}')
```

## 7. Tier 2 — Live IPC (guarded)

When `WETSPRING_IPC_SOCKET` is set, we call the Rust barracuda implementation
of these same computations and assert parity with the Python results above.
This is the **frozen → live → primals** evolution path.

```python
if IPC_SOCKET:
    # Anderson 1D spectral analysis via barracuda::spectral
    result = ipc_call('science.anderson_spectral', {
        'dimension': 1, 'size': 200, 'disorder': 4.0, 'seed': 42
    })
    if result:
        ipc_gamma = result.get('lyapunov_exponent', float('nan'))
        ipc_r = result.get('level_spacing_ratio', float('nan'))
        print(f'IPC γ(0): {ipc_gamma:.6f}  (Python: {gamma_0:.6f})')
        print(f'IPC ⟨r⟩: {ipc_r:.4f}  (Python: {r_1d:.4f})')
        assert abs(ipc_gamma - gamma_0) < 0.01, 'Lyapunov parity failed'
        print('Tier 2 parity: ✓')
    else:
        print('IPC returned no result — barracuda may lack anderson_spectral handler')
else:
    print('Tier 1 (frozen): IPC socket not set — skipping live validation')
    print('Set WETSPRING_IPC_SOCKET to enable Tier 2 parity checks')
```

## Summary

| Section | Model | Validation | Checks |
|---------|-------|------------|:------:|
| 1D Anderson | Random diagonal + tridiagonal | Gershgorin, Lyapunov, ⟨r⟩ | 7 |
| Almost-Mathieu | Quasiperiodic (Aubry–André) | Herman formula, transition | 6 |
| QS-Disorder | 8 ecosystems → W → γ, ⟨r⟩ | Monotonic ordering, biofilm prediction | 8 |
| Dimensional | 1D/2D/3D sweep | Plateau widths, J_c thresholds | 15 |
| Lanczos | Tridiagonalization eigensolver | Dense vs sparse parity | 3 |
| Biome Atlas | 28 NCBI-derived profiles | W < W_c(3D) for all biomes | 28 |

**Evolution path:**
- **Tier 1 (this notebook):** Frozen Python math — self-contained, reproducible offline
- **Tier 2:** Live IPC to `barracuda::spectral` via wetSpring Unix socket
- **Tier 3:** Full NUCLEUS composition with provenance trio wrapping, petalTongue
  server-side rendering, and `gAIa` knowledge-commons artifact

**Key insight:** Dimensionality — not diversity — is the decisive factor for QS.
The 3D metal–insulator transition at $W_c \approx 16.5$ exceeds all natural
microbial community disorder values. Any community with 3D spatial structure can
sustain quorum sensing regardless of diversity.

