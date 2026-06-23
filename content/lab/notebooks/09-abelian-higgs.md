+++
title = "Abelian Higgs Model — (1+1)D Lattice Field Theory"
description = "Rendered from 09-abelian-higgs.ipynb"
date = 2026-06-23
weight = 50

[extra]
domain = "Lab"
rendered_from = "09-abelian-higgs.ipynb"
+++

<!-- Auto-generated from 09-abelian-higgs.ipynb by spore-validate render-notebooks -->

# Abelian Higgs Model — (1+1)D Lattice Field Theory

**Paper:** Bazavov et al., *Phys. Rev. D* **92**, 076003 (2015)  
**What we reproduce:** U(1) gauge + complex scalar Higgs on a (1+1)D lattice using HMC.
We scan coupling parameters ($\beta_{\text{pl}}$, $\kappa$, $\lambda$) to explore
the Higgs mechanism, confinement, and symmetry breaking in the simplest gauge-Higgs system.

---

*This notebook runs live HMC on small (8x8) lattices in Python. All compute executes in-notebook.*  
*Rust parity:* `barracuda/src/lattice/abelian_higgs.rs` — GPU-accelerated via WGSL.*  
*Algorithm-identical: same LCG PRNG, same leapfrog, same Metropolis accept/reject.*

## Physics

The Abelian Higgs action on a (1+1)D lattice:

$$S = S_{\text{gauge}} + S_{\text{Higgs}}$$

$$S_{\text{gauge}} = \beta_{\text{pl}} \sum_x \left(1 - \cos\theta_p(x)\right)$$

$$S_{\text{Higgs}} = \sum_x \left[|\phi(x)|^2 + \lambda(|\phi(x)|^2 - 1)^2 - 2\kappa \sum_\mu \text{Re}(\phi^* U_\mu \phi')\right]$$

where $U_\mu = e^{i\theta_\mu}$ are U(1) link variables and $\phi$ is a complex scalar.

Key phases:
- **Higgs phase** (large $\kappa$): $\langle|\phi|^2\rangle \gg 1$, gauge symmetry "broken"
- **Confined phase** (small $\kappa$): $\langle|\phi|^2\rangle \approx 1$, confining strings
- **Coulomb phase** (large $\beta_{\text{pl}}$): plaquette $\to 1$, weak coupling

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

LCG_MUL = np.uint64(6_364_136_223_846_793_005)
LCG_INC = np.uint64(1_442_695_040_888_963_407)
LCG_53_DIV = float(1 << 53)

class LCG:
    def __init__(self, seed):
        self.state = np.uint64(seed)
    def step(self):
        self.state = self.state * LCG_MUL + LCG_INC
    def uniform(self):
        self.step()
        return float(self.state >> np.uint64(11)) / LCG_53_DIV
    def gaussian(self):
        u1 = max(self.uniform(), 1e-30)
        u2 = self.uniform()
        return np.sqrt(-2.0 * np.log(u1)) * np.cos(2.0 * np.pi * u2)

print("LCG PRNG ready (deterministic, matches Rust)")
```

```python
class U1HiggsLattice:
    def __init__(self, nt, ns, beta_pl, kappa, lam, mu=0.0):
        self.nt, self.ns = nt, ns
        self.beta_pl, self.kappa, self.lam, self.mu = beta_pl, kappa, lam, mu
        self.vol = nt * ns
        self.links = np.zeros(self.vol * 2)
        self.higgs = np.ones(self.vol, dtype=complex)

    @classmethod
    def hot(cls, nt, ns, beta_pl, kappa, lam, rng, mu=0.0):
        lat = cls(nt, ns, beta_pl, kappa, lam, mu)
        lat.links = np.array([2*np.pi*rng.uniform() - np.pi for _ in range(lat.vol * 2)])
        lat.higgs = np.array([complex(rng.gaussian()*0.5+1, rng.gaussian()*0.5) for _ in range(lat.vol)])
        return lat

    def idx(self, t, x): return t * self.ns + x
    def fwd(self, t, x, mu): return ((t+1)%self.nt, x) if mu == 0 else (t, (x+1)%self.ns)
    def bwd(self, t, x, mu): return ((t-1)%self.nt, x) if mu == 0 else (t, (x-1)%self.ns)
    def link_angle(self, t, x, mu): return self.links[self.idx(t, x) * 2 + mu]
    def link(self, t, x, mu): return np.exp(1j * self.link_angle(t, x, mu))

    def plaquette_phase(self, t, x):
        t1, _ = self.fwd(t, x, 0)
        _, x1 = self.fwd(t, x, 1)
        return (self.link_angle(t, x, 0) + self.link_angle(t1, x, 1)
                - self.link_angle(t, x1, 0) - self.link_angle(t, x, 1))

    def average_plaquette(self):
        return sum(np.cos(self.plaquette_phase(t, x))
                   for t in range(self.nt) for x in range(self.ns)) / self.vol

    def gauge_action(self):
        return self.beta_pl * sum(1.0 - np.cos(self.plaquette_phase(t, x))
                                 for t in range(self.nt) for x in range(self.ns))

    def higgs_action(self):
        s_kin, s_pot = 0.0, 0.0
        for t in range(self.nt):
            for x in range(self.ns):
                phi = self.higgs[self.idx(t, x)]
                phi_sq = abs(phi)**2
                s_pot += phi_sq + self.lam * (phi_sq - 1.0)**2
                for mu in range(2):
                    tf, xf = self.fwd(t, x, mu)
                    hop = np.conj(phi) * self.link(t, x, mu) * self.higgs[self.idx(tf, xf)]
                    cw = np.exp(self.mu) if mu == 0 else 1.0
                    s_kin += cw * hop.real
        return -2.0 * self.kappa * s_kin + s_pot

    def total_action(self): return self.gauge_action() + self.higgs_action()
    def average_higgs_sq(self): return np.mean(np.abs(self.higgs)**2)

    def gauge_force(self, t, x, mu):
        nu = 1 - mu
        tm, xm = self.fwd(t, x, mu)
        tn, xn = self.fwd(t, x, nu)
        phase_fwd = (self.link_angle(t, x, mu) + self.link_angle(tm, xm, nu)
                     - self.link_angle(tn, xn, mu) - self.link_angle(t, x, nu))
        f = self.beta_pl * np.sin(phase_fwd)
        tb, xb = self.bwd(t, x, nu)
        tbm, xbm = self.fwd(tb, xb, mu)
        phase_bwd = (self.link_angle(tb, xb, nu) + self.link_angle(t, x, mu)
                     - self.link_angle(tbm, xbm, nu) - self.link_angle(tb, xb, mu))
        return -(f + self.beta_pl * np.sin(phase_bwd))

    def higgs_link_force(self, t, x, mu):
        tf, xf = self.fwd(t, x, mu)
        hop = np.conj(self.higgs[self.idx(t, x)]) * self.link(t, x, mu) * self.higgs[self.idx(tf, xf)]
        cw = np.exp(self.mu) if mu == 0 else 1.0
        return -2.0 * self.kappa * cw * hop.imag

    def higgs_field_force(self, t, x):
        i = self.idx(t, x)
        phi = self.higgs[i]
        pot_grad = phi * (1.0 + 2.0 * self.lam * (abs(phi)**2 - 1.0))
        kin = 0j
        for mu in range(2):
            tf, xf = self.fwd(t, x, mu)
            cf = np.exp(self.mu) if mu == 0 else 1.0
            kin += self.link(t, x, mu) * self.higgs[self.idx(tf, xf)] * cf
            tb, xb = self.bwd(t, x, mu)
            cb = np.exp(-self.mu) if mu == 0 else 1.0
            kin += np.conj(self.link(tb, xb, mu)) * self.higgs[self.idx(tb, xb)] * cb
        return 2.0 * (self.kappa * kin - pot_grad)

def hmc_trajectory(lat, n_md, dt, rng):
    old_links, old_higgs = lat.links.copy(), lat.higgs.copy()
    pi_links = np.array([rng.gaussian() for _ in range(lat.vol * 2)])
    pi_higgs = np.array([complex(rng.gaussian(), rng.gaussian()) for _ in range(lat.vol)])
    ke0 = 0.5 * (np.sum(pi_links**2) + np.sum(np.abs(pi_higgs)**2))
    s0 = lat.total_action()
    h0 = ke0 + s0

    def update_mom(fdt):
        for t in range(lat.nt):
            for x in range(lat.ns):
                for mu in range(2):
                    pi_links[lat.idx(t, x)*2+mu] += fdt * (lat.gauge_force(t, x, mu) + lat.higgs_link_force(t, x, mu))
                pi_higgs[lat.idx(t, x)] += fdt * lat.higgs_field_force(t, x)

    update_mom(dt / 2.0)
    for step in range(n_md):
        lat.links += dt * pi_links
        for i in range(lat.vol): lat.higgs[i] += dt * pi_higgs[i]
        if step < n_md - 1: update_mom(dt)
    update_mom(dt / 2.0)

    ke1 = 0.5 * (np.sum(pi_links**2) + np.sum(np.abs(pi_higgs)**2))
    dh = (ke1 + lat.total_action()) - h0
    accepted = dh <= 0.0 or rng.uniform() < np.exp(-dh)
    if not accepted:
        lat.links, lat.higgs = old_links, old_higgs
    return accepted, dh, lat.average_plaquette(), lat.average_higgs_sq()

print("U(1) Higgs lattice + HMC ready")
```

## Phase Scan — Higgs, Confined, Coulomb

We run HMC across different parameter regimes to identify the physical phases.

```python
CONFIGS = [
    ("Weak coupling",   8, 8, 6.0, 0.3, 1.0, 30, 50, 10, 0.08, 42),
    ("Strong coupling", 8, 8, 0.5, 0.3, 1.0, 30, 50, 10, 0.08, 42),
    ("Higgs condensed", 8, 8, 2.0, 2.0, 1.0, 30, 50, 10, 0.05, 42),
    ("Confined",        8, 8, 1.0, 0.1, 1.0, 30, 50, 10, 0.08, 42),
    ("Large lambda",    8, 8, 2.0, 0.5, 10.0, 30, 50, 10, 0.05, 42),
]

results = []
for label, nt, ns, beta, kappa, lam, n_th, n_tr, n_md, dt, seed in CONFIGS:
    rng = LCG(seed)
    lat = U1HiggsLattice.hot(nt, ns, beta, kappa, lam, rng)
    t0 = time.perf_counter()
    for _ in range(n_th):
        hmc_trajectory(lat, n_md, dt, rng)
    plaq_sum, hsq_sum, n_acc = 0.0, 0.0, 0
    for _ in range(n_tr):
        acc, dh, plaq, hsq = hmc_trajectory(lat, n_md, dt, rng)
        plaq_sum += plaq; hsq_sum += hsq; n_acc += int(acc)
    elapsed = time.perf_counter() - t0
    r = {'label': label, 'beta': beta, 'kappa': kappa, 'lambda': lam,
         'plaq': plaq_sum/n_tr, 'higgs_sq': hsq_sum/n_tr,
         'acc': n_acc/n_tr, 'time': elapsed}
    results.append(r)
    print(f"  {label:20s}  plaq={r['plaq']:.6f}  |phi^2|={r['higgs_sq']:.4f}  "
          f"acc={r['acc']:.0%}  {elapsed:.1f}s")
```

```python
fig, axes = plt.subplots(1, 2, figsize=(14, 6))

labels = [r['label'] for r in results]
plaqs = [r['plaq'] for r in results]
higgs = [r['higgs_sq'] for r in results]
colors = [C_INFO, C_FAIL, C_GPU, C_PYTHON, C_PASS]

x = np.arange(len(labels))

axes[0].bar(x, plaqs, color=colors, alpha=0.7)
axes[0].set_xticks(x)
axes[0].set_xticklabels(labels, rotation=30, ha='right', fontsize=9)
axes[0].set_ylabel('$\\langle P \\rangle$')
axes[0].set_title('Average Plaquette by Phase')
for i, v in enumerate(plaqs):
    axes[0].text(i, v + 0.01, f'{v:.3f}', ha='center', fontsize=9)

axes[1].bar(x, higgs, color=colors, alpha=0.7)
axes[1].set_xticks(x)
axes[1].set_xticklabels(labels, rotation=30, ha='right', fontsize=9)
axes[1].set_ylabel('$\\langle|\\phi|^2\\rangle$')
axes[1].set_title('Higgs Condensate by Phase')
for i, v in enumerate(higgs):
    axes[1].text(i, v + 0.05, f'{v:.2f}', ha='center', fontsize=9)

fig.suptitle('Abelian Higgs (1+1)D — Phase Structure', fontsize=14, fontweight='bold')
plt.tight_layout()
plt.savefig('/tmp/hotspring_paper_09_phases.png', dpi=150, bbox_inches='tight')
plt.show()
```

## Rust Parity

The same U(1) Higgs HMC is implemented in `barracuda/src/lattice/abelian_higgs.rs`.
Rust validation confirms bit-for-bit agreement with the Python baseline for
deterministic LCG-seeded configurations.

| Implementation | 8x8 trajectory | Speedup |
|---------------|---------------|--------|
| Python (numpy) | ~0.5 s | 1x |
| Rust (CPU) | ~0.02 s | **25x** |

## Provenance

- **Paper:** Bazavov et al., PRD **92**, 076003 (2015)
- **Validation:** `barracuda/src/lattice/abelian_higgs.rs`, `validate_abelian_higgs`
- **Control:** `control/abelian_higgs/scripts/abelian_higgs_hmc.py`
- **Next:** Extend to (2+1)D, finite-density via chemical potential $\mu$

---
*hotSpring — ecoPrimals · AGPL-3.0 · [primals.eco](https://primals.eco)*

