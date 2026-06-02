+++
title = "Quenched SU(3) Lattice QCD — Deconfinement Transition"
description = "Rendered from 07-quenched-qcd.ipynb"
date = 2026-06-02
weight = 50

[extra]
domain = "Lab"
rendered_from = "07-quenched-qcd.ipynb"
+++

<!-- Auto-generated from 07-quenched-qcd.ipynb by spore-validate render-notebooks -->

# Quenched SU(3) Lattice QCD — Deconfinement Transition

**Papers:**  
- Wilson (1974) *PRD* **10**, 2445 — Wilson gauge action  
- Creutz (1980) *PRD* **21**, 2308 — SU(3) Monte Carlo  
- Gattringer & Lang, *QCD on the Lattice* (2010), Ch. 3, 8  
- HotQCD (2014) *PRD* **90**, 094503 — 2+1 flavor EOS  

**What we reproduce:** Pure gauge SU(3) HMC on a $4^4$ lattice, scanning the
inverse coupling $\beta$ through the deconfinement transition at $\beta_c \approx 5.69$.
We measure the average plaquette (gauge action order parameter) and the Polyakov
loop (confinement order parameter). Below $\beta_c$: $|L| \approx 0$ (confined).
Above $\beta_c$: $|L| > 0$ (deconfined).

---

*This notebook runs a small HMC simulation live (4^4 lattice, ~30s per beta point).*  
*Rust parity:* `barracuda/src/lattice/` — GPU-accelerated HMC via WGSL compute shaders.*  
*Algorithm-identical: same LCG PRNG, same Cayley exp, same leapfrog, same Metropolis step.*

## Physics

The **Wilson gauge action** on the lattice:

$$S_W = \beta \sum_{x,\mu<\nu} \left(1 - \frac{1}{3}\text{Re}\,\text{Tr}\, U_{\mu\nu}(x)\right)$$

where $U_{\mu\nu}$ is the plaquette (product of 4 links around a unit square).
The **Polyakov loop** (temporal Wilson line) is the order parameter:

$$L(\vec{x}) = \frac{1}{3}\text{Tr}\prod_{t=0}^{N_t-1} U_0(\vec{x}, t)$$

- $\langle|L|\rangle \approx 0$: confined phase (quarks bound)
- $\langle|L|\rangle > 0$: deconfined phase (quark-gluon plasma)

We use **Hybrid Monte Carlo** (HMC) with SU(3) momenta sampled from
the Lie algebra (8 Gell-Mann generators), Cayley matrix exponential,
and leapfrog integrator.

```python
import numpy as np
import time
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt

C_PASS = '#2ecc71'
C_FAIL = '#e74c3c'
C_INFO = '#3498db'
C_RUST = '#1abc9c'
C_PYTHON = '#f39c12'
C_GPU = '#9b59b6'

# LCG PRNG — matches Rust constants.rs exactly
LCG_A = 6_364_136_223_846_793_005
LCG_C = 1_442_695_040_888_963_407
LCG_MOD = 1 << 64
LCG_53_DIV = float(1 << 53)
LATTICE_DIVISION_GUARD = 1e-15

def lcg_step(seed):
    return (seed * LCG_A + LCG_C) % LCG_MOD

def lcg_uniform(seed):
    seed = lcg_step(seed)
    return seed, float(seed >> 11) / LCG_53_DIV

def lcg_gaussian(seed):
    seed, u1 = lcg_uniform(seed)
    seed, u2 = lcg_uniform(seed)
    r = np.sqrt(-2.0 * np.log(max(u1, LATTICE_DIVISION_GUARD)))
    theta = 2.0 * np.pi * u2
    return seed, float(r * np.cos(theta))

print("LCG PRNG loaded (deterministic, matches Rust)")
```

```python
def su3_identity():
    return np.eye(3, dtype=np.complex128)

def su3_random_near_identity(seed, epsilon):
    m = np.zeros((3, 3), dtype=np.complex128)
    for i in range(3):
        for j in range(3):
            seed, re = lcg_uniform(seed)
            seed, im = lcg_uniform(seed)
            m[i, j] = complex((re - 0.5) * epsilon, (im - 0.5) * epsilon)
    m = np.eye(3, dtype=np.complex128) + m
    u0 = m[0] / np.linalg.norm(m[0])
    u1 = m[1] - np.dot(np.conj(u0), m[1]) * u0
    u1 = u1 / np.linalg.norm(u1)
    u2 = np.cross(np.conj(u0), np.conj(u1))
    return seed, np.array([u0, u1, u2])

def su3_random_algebra(seed):
    scale = 1.0 / np.sqrt(2.0)
    def rg():
        nonlocal seed
        seed, g = lcg_gaussian(seed)
        return scale * g
    h = np.zeros((3, 3), dtype=np.complex128)
    a3, a8 = rg(), rg()
    sqrt3 = np.sqrt(3.0)
    h[0, 0] = complex(a3 + a8 / sqrt3, 0.0)
    h[1, 1] = complex(-a3 + a8 / sqrt3, 0.0)
    h[2, 2] = complex(-2.0 * a8 / sqrt3, 0.0)
    for i, j in [(0, 1), (0, 2), (1, 2)]:
        re, im = rg(), rg()
        h[i, j] = complex(re, im)
        h[j, i] = complex(re, -im)
    return seed, 1j * h

def exp_su3_cayley(p, dt):
    half = 0.5 * dt * p
    plus = np.eye(3, dtype=np.complex128) + half
    minus = np.eye(3, dtype=np.complex128) - half
    result = plus @ np.linalg.inv(minus)
    u0 = result[0] / np.linalg.norm(result[0])
    u1 = result[1] - np.dot(np.conj(u0), result[1]) * u0
    u1 = u1 / np.linalg.norm(u1)
    u2 = np.cross(np.conj(u0), np.conj(u1))
    return np.array([u0, u1, u2])

print("SU(3) operations ready")
```

```python
class Lattice:
    def __init__(self, dims, beta):
        self.dims = list(dims)
        self.beta = beta
        self.volume = dims[0] * dims[1] * dims[2] * dims[3]
        self.links = None

    @staticmethod
    def hot_start(dims, beta, seed):
        lat = Lattice(dims, beta)
        rng = int(seed)
        lat.links = []
        for _ in range(lat.volume * 4):
            rng, u = su3_random_near_identity(rng, 1.5)
            lat.links.append(u)
        return lat

    def site_index(self, x):
        return x[0] + self.dims[0] * (x[1] + self.dims[1] * (x[2] + self.dims[2] * x[3]))

    def site_coords(self, idx):
        x0 = idx % self.dims[0]
        rem = idx // self.dims[0]
        x1 = rem % self.dims[1]
        rem2 = rem // self.dims[1]
        x2 = rem2 % self.dims[2]
        x3 = rem2 // self.dims[2]
        return [x0, x1, x2, x3]

    def neighbor(self, x, mu, forward):
        y = list(x)
        y[mu] = (x[mu] + (1 if forward else -1)) % self.dims[mu]
        return y

    def link(self, x, mu):
        return self.links[self.site_index(x) * 4 + mu]

    def set_link(self, x, mu, u):
        self.links[self.site_index(x) * 4 + mu] = u

    def plaquette(self, x, mu, nu):
        x_mu = self.neighbor(x, mu, True)
        x_nu = self.neighbor(x, nu, True)
        return self.link(x, mu) @ self.link(x_mu, nu) @ self.link(x_nu, mu).conj().T @ self.link(x, nu).conj().T

    def average_plaquette(self):
        total, count = 0.0, 0
        for idx in range(self.volume):
            x = self.site_coords(idx)
            for mu in range(4):
                for nu in range(mu + 1, 4):
                    total += np.trace(self.plaquette(x, mu, nu)).real / 3.0
                    count += 1
        return total / count

    def staple(self, x, mu):
        s = np.zeros((3, 3), dtype=np.complex128)
        x_mu = self.neighbor(x, mu, True)
        for nu in range(4):
            if nu == mu: continue
            x_nu = self.neighbor(x, nu, True)
            x_mu_bnu = self.neighbor(x_mu, nu, False)
            x_bnu = self.neighbor(x, nu, False)
            s += self.link(x_mu, nu) @ self.link(x_nu, mu).conj().T @ self.link(x, nu).conj().T
            s += self.link(x_mu_bnu, nu).conj().T @ self.link(x_bnu, mu).conj().T @ self.link(x_bnu, nu)
        return s

    def wilson_action(self):
        total = 0.0
        for idx in range(self.volume):
            x = self.site_coords(idx)
            for mu in range(4):
                for nu in range(mu + 1, 4):
                    total += 1.0 - np.trace(self.plaquette(x, mu, nu)).real / 3.0
        return self.beta * total

    def gauge_force(self, x, mu):
        w = self.link(x, mu) @ self.staple(x, mu)
        diff = 0.5 * (w - w.conj().T)
        tr = np.trace(diff)
        for i in range(3): diff[i, i] -= tr / 3.0
        return -self.beta / 3.0 * diff

    def polyakov_loop(self, x_spatial):
        prod = su3_identity()
        for t in range(self.dims[3]):
            prod = prod @ self.link([x_spatial[0], x_spatial[1], x_spatial[2], t], 3)
        return np.trace(prod) / 3.0

    def average_polyakov_loop(self):
        ns = self.dims[:3]
        total = sum(abs(self.polyakov_loop([ix, iy, iz]))
                    for ix in range(ns[0]) for iy in range(ns[1]) for iz in range(ns[2]))
        return total / (ns[0] * ns[1] * ns[2])

print("Lattice class ready")
```

```python
def hmc_trajectory(lattice, seed, n_md_steps, dt):
    old_links = [u.copy() for u in lattice.links]
    action_before = lattice.wilson_action()
    momenta = []
    for _ in range(lattice.volume * 4):
        seed, p = su3_random_algebra(seed)
        momenta.append(p)
    ke_before = sum(-0.5 * np.trace(p @ p).real for p in momenta)
    h_old = action_before + ke_before

    # Leapfrog
    half_dt = 0.5 * dt
    for idx in range(lattice.volume):
        x = lattice.site_coords(idx)
        for mu in range(4):
            momenta[idx * 4 + mu] += half_dt * lattice.gauge_force(x, mu)
    for step in range(n_md_steps):
        for idx in range(lattice.volume):
            x = lattice.site_coords(idx)
            for mu in range(4):
                lattice.set_link(x, mu, exp_su3_cayley(momenta[idx * 4 + mu], dt) @ lattice.link(x, mu))
        p_dt = dt if step < n_md_steps - 1 else half_dt
        for idx in range(lattice.volume):
            x = lattice.site_coords(idx)
            for mu in range(4):
                momenta[idx * 4 + mu] += p_dt * lattice.gauge_force(x, mu)

    action_after = lattice.wilson_action()
    ke_after = sum(-0.5 * np.trace(p @ p).real for p in momenta)
    delta_h = (action_after + ke_after) - h_old

    if delta_h <= 0.0:
        accept = True
    else:
        seed, r = lcg_uniform(seed)
        accept = r < np.exp(-delta_h)
    if not accept:
        lattice.links = old_links

    return seed, lattice.average_plaquette(), delta_h, accept

print("HMC trajectory function ready")
```

## Beta Scan — Crossing the Deconfinement Transition

We run short HMC trajectories at each $\beta$ value on a $4^4$ lattice.
The transition at $\beta_c \approx 5.69$ separates:
- **Confined phase** ($\beta < \beta_c$): plaquette low, Polyakov loop $\approx 0$
- **Deconfined phase** ($\beta > \beta_c$): plaquette high, Polyakov loop $> 0$

```python
dims = [4, 4, 4, 4]
beta_values = [5.0, 5.5, 5.7, 6.0, 6.5]
n_therm = 10
n_traj = 15
n_md_steps = 10
dt = 0.05
seed_base = 42

results = []
print(f"Running beta scan on {dims} lattice...")
print(f"  {n_therm} thermalization + {n_traj} measurement trajectories per beta")
print()

for i, beta in enumerate(beta_values):
    seed = seed_base + i
    lat = Lattice.hot_start(dims, beta, seed_base + i)
    plaquettes = []
    accepted = 0
    t0 = time.perf_counter()

    for traj in range(n_therm + n_traj):
        seed, plaq, dh, acc = hmc_trajectory(lat, seed, n_md_steps, dt)
        if traj >= n_therm:
            plaquettes.append(plaq)
            if acc: accepted += 1

    poly = lat.average_polyakov_loop()
    wall_s = time.perf_counter() - t0
    mean_plaq = np.mean(plaquettes)
    std_plaq = np.std(plaquettes, ddof=1) if len(plaquettes) > 1 else 0.0
    acc_rate = accepted / n_traj

    results.append({
        'beta': beta, 'mean_plaquette': mean_plaq, 'std_plaquette': std_plaq,
        'polyakov_loop': poly, 'acceptance_rate': acc_rate, 'wall_time_s': wall_s
    })
    print(f"  beta={beta:.2f}: <plaq>={mean_plaq:.6f}+/-{std_plaq:.6f}, |L|={poly:.6f}, "
          f"acc={acc_rate:.0%}, {wall_s:.1f}s")
```

```python
fig, axes = plt.subplots(1, 3, figsize=(18, 5))

betas = [r['beta'] for r in results]
plaqs = [r['mean_plaquette'] for r in results]
plaqs_err = [r['std_plaquette'] for r in results]
polys = [r['polyakov_loop'] for r in results]
accs = [r['acceptance_rate'] for r in results]

# Plaquette vs beta
axes[0].errorbar(betas, plaqs, yerr=plaqs_err, fmt='o-', color=C_INFO, markersize=8,
                 capsize=4, linewidth=2)
axes[0].axvline(x=5.69, color='gray', ls=':', alpha=0.5, label='$\\beta_c \\approx 5.69$')
axes[0].set_xlabel('$\\beta$')
axes[0].set_ylabel('$\\langle P \\rangle$')
axes[0].set_title('Average Plaquette')
axes[0].legend()

# Polyakov loop vs beta
axes[1].plot(betas, polys, 's-', color=C_FAIL, markersize=8, linewidth=2)
axes[1].axvline(x=5.69, color='gray', ls=':', alpha=0.5, label='$\\beta_c \\approx 5.69$')
axes[1].set_xlabel('$\\beta$')
axes[1].set_ylabel('$\\langle|L|\\rangle$')
axes[1].set_title('Polyakov Loop (confinement order parameter)')
axes[1].legend()

# Acceptance rate
axes[2].bar(range(len(betas)), [a*100 for a in accs], color=C_PASS, alpha=0.7)
axes[2].set_xticks(range(len(betas)))
axes[2].set_xticklabels([f'{b:.1f}' for b in betas])
axes[2].set_xlabel('$\\beta$')
axes[2].set_ylabel('Acceptance %')
axes[2].set_title('HMC Acceptance Rate')
axes[2].axhline(y=50, color='gray', ls='--', alpha=0.3)

fig.suptitle(f'Quenched SU(3) on $4^4$ — {n_traj} trajectories per $\\beta$', fontsize=14, fontweight='bold')
plt.tight_layout()
plt.savefig('/tmp/hotspring_paper_07_beta_scan.png', dpi=150, bbox_inches='tight')
plt.show()

# Validation checks
monotonic = all(results[i+1]['mean_plaquette'] >= results[i]['mean_plaquette'] - 0.01
                for i in range(len(results) - 1))
print(f"\nPlaquette increases with beta: {'PASS' if monotonic else 'FAIL'}")

if len([r for r in results if r['beta'] < 5.5]) > 0 and len([r for r in results if r['beta'] > 6.0]) > 0:
    conf = np.mean([r['polyakov_loop'] for r in results if r['beta'] < 5.5])
    deconf = np.mean([r['polyakov_loop'] for r in results if r['beta'] > 6.0])
    print(f"Polyakov: confined <|L|>={conf:.4f}, deconfined <|L|>={deconf:.4f}: {'PASS' if deconf > conf else 'FAIL'}")
```

## Rust Parity

The same HMC algorithm is implemented in `barracuda/src/lattice/` with
GPU-accelerated link updates, staple computation, and force evaluation
via BarraCuda's WGSL compute shaders.

| Implementation | 4^4 trajectory | Speedup |
|---------------|---------------|--------|
| Python (numpy) | ~3 s | 1x |
| Rust (CPU) | ~0.1 s | **30x** |
| Rust (GPU) | ~0.005 s | **600x** |

## Provenance

- **Papers:** Wilson PRD 10 (1974), Creutz PRD 21 (1980), HotQCD PRD 90 (2014)
- **Validation:** `barracuda/src/lattice/`, `validate_quenched_hmc`
- **Control:** `control/lattice_qcd/scripts/quenched_beta_scan.py`
- **Next:** Production-scale HMC on 8^4+ via GPU primal composition

---
*hotSpring — ecoPrimals · AGPL-3.0 · [primals.eco](https://primals.eco)*

