+++
title = "Stanton-Murillo Transport Coefficients — Yukawa OCP"
description = "Rendered from 05-stanton-murillo-transport.ipynb"
date = 2026-07-04
weight = 50

[extra]
domain = "physics"
rendered_from = "05-stanton-murillo-transport.ipynb"
+++

<!-- Auto-generated from 05-stanton-murillo-transport.ipynb by spore-validate render-notebooks -->

# Stanton-Murillo Transport Coefficients — Yukawa OCP

**Papers:**  
- Stanton & Murillo, *PRE* **91**, 033104 (2015) — ionic transport model  
- Daligault, *PRE* **86**, 047401 (2012) — D* analytical fit  

**What we reproduce:** The Daligault analytical model for the reduced
self-diffusion coefficient $D^*(\Gamma, \kappa)$ of a Yukawa one-component
plasma, covering the full range from weakly coupled gas ($\Gamma \ll 1$)
to strongly coupled liquid/crystal ($\Gamma \gg 100$). This model interpolates
between Landau-Spitzer kinetic theory and the caging/Einstein frequency regime.

---

*All compute runs live — the analytical model is fast (<1 ms).*  
*Production MD transport grids are loaded from frozen JSON when available.*  
*Rust parity:* `barracuda/src/physics/transport.rs`

## Physics

The reduced self-diffusion coefficient $D^* = D/(a_{\text{ws}}^2 \omega_p)$ interpolates:

$$D^*(\Gamma, \kappa) = D^*_w(\Gamma, \kappa) \cdot f(\Gamma, \kappa) + D^*_s(\Gamma, \kappa) \cdot [1 - f(\Gamma, \kappa)]$$

**Weak coupling** (Landau-Spitzer):
$$D^*_w = \frac{3\sqrt{\pi}}{4} \frac{1}{\Gamma^{5/2} \ln\Lambda}$$

**Strong coupling** (caging):
$$D^*_s = A(\kappa) \cdot \Gamma^{-\alpha(\kappa)}$$

**Crossover:**
$$f(\Gamma, \kappa) = \frac{1}{1 + (\Gamma/\Gamma_x(\kappa))^2}$$

```python
import numpy as np
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt

C_PASS = '#2ecc71'
C_FAIL = '#e74c3c'
C_INFO = '#3498db'
C_RUST = '#1abc9c'
C_PYTHON = '#f39c12'
C_GPU = '#9b59b6'

def coulomb_log(gamma, kappa):
    gamma_eff = gamma * np.exp(-kappa)
    if gamma_eff < 0.1:
        return max(np.log(1.0/gamma_eff), 1.0)
    return max(np.log(1.0 + 1.0/gamma_eff), 0.1)

def d_star_weak(gamma, kappa):
    cl = coulomb_log(gamma, kappa)
    return 3*np.sqrt(np.pi)/4 / (gamma**2.5 * cl)

def d_star_strong(gamma, kappa):
    a = 0.0094 + 0.018*kappa - 0.0025*kappa**2
    alpha = 1.09 + 0.12*kappa - 0.019*kappa**2
    return a * gamma**(-alpha)

def crossover(gamma, kappa):
    gamma_x = 10.0*np.exp(0.5*kappa)
    return 1.0/(1.0 + (gamma/gamma_x)**2)

def d_star(gamma, kappa):
    f = crossover(gamma, kappa)
    return d_star_weak(gamma, kappa)*f + d_star_strong(gamma, kappa)*(1-f)

print("Daligault transport model loaded")
```

```python
gammas = np.logspace(-1, 3, 300)
kappas = [0.0, 0.5, 1.0, 2.0, 3.0]
colors_k = [C_INFO, C_PASS, C_PYTHON, C_GPU, C_FAIL]

fig, axes = plt.subplots(1, 3, figsize=(18, 5))

# D* vs Gamma for different kappa
for kap, color in zip(kappas, colors_k):
    d_vals = [d_star(g, kap) for g in gammas]
    axes[0].loglog(gammas, d_vals, color=color, linewidth=2, label=f'$\\kappa={kap}$')

axes[0].set_xlabel('$\\Gamma$')
axes[0].set_ylabel('$D^*$')
axes[0].set_title('Self-Diffusion Coefficient')
axes[0].legend(fontsize=9)
axes[0].grid(True, alpha=0.3)

# Weak vs strong contributions
kap_demo = 1.0
dw = [d_star_weak(g, kap_demo) for g in gammas]
ds = [d_star_strong(g, kap_demo) for g in gammas]
dt = [d_star(g, kap_demo) for g in gammas]
axes[1].loglog(gammas, dw, '--', color=C_PYTHON, label='$D^*_w$ (weak)', linewidth=1.5)
axes[1].loglog(gammas, ds, '--', color=C_GPU, label='$D^*_s$ (strong)', linewidth=1.5)
axes[1].loglog(gammas, dt, '-', color=C_INFO, label='$D^*$ (full)', linewidth=2.5)
axes[1].set_xlabel('$\\Gamma$')
axes[1].set_ylabel('$D^*$')
axes[1].set_title(f'Weak/Strong Decomposition ($\\kappa={kap_demo}$)')
axes[1].legend(fontsize=9)
axes[1].grid(True, alpha=0.3)

# Crossover function
f_vals = [crossover(g, kap_demo) for g in gammas]
axes[2].semilogx(gammas, f_vals, color=C_INFO, linewidth=2)
axes[2].axhline(y=0.5, color='gray', ls='--', alpha=0.5)
gamma_x = 10*np.exp(0.5*kap_demo)
axes[2].axvline(x=gamma_x, color=C_FAIL, ls=':', label=f'$\\Gamma_x={gamma_x:.1f}$')
axes[2].set_xlabel('$\\Gamma$')
axes[2].set_ylabel('$f(\\Gamma, \\kappa)$')
axes[2].set_title('Crossover Function')
axes[2].legend()

fig.suptitle('Stanton-Murillo / Daligault Transport Model', fontsize=14, fontweight='bold')
plt.tight_layout()
plt.savefig('/tmp/hotspring_paper_05_transport.png', dpi=150, bbox_inches='tight')
plt.show()
```

## Screening Dependence — 2D Map

```python
g_grid = np.logspace(-1, 3, 100)
k_grid = np.linspace(0, 4, 80)
G, K = np.meshgrid(g_grid, k_grid)
D_map = np.zeros_like(G)
for i in range(G.shape[0]):
    for j in range(G.shape[1]):
        D_map[i, j] = np.log10(max(d_star(G[i,j], K[i,j]), 1e-10))

fig, ax = plt.subplots(figsize=(10, 6))
im = ax.pcolormesh(g_grid, k_grid, D_map, cmap='viridis', shading='auto')
ax.set_xscale('log')
ax.set_xlabel('$\\Gamma$')
ax.set_ylabel('$\\kappa$')
ax.set_title('$\\log_{10} D^*(\\Gamma, \\kappa)$ — Transport Landscape')
plt.colorbar(im, label='$\\log_{10} D^*$')
plt.tight_layout()
plt.savefig('/tmp/hotspring_paper_05_landscape.png', dpi=150, bbox_inches='tight')
plt.show()
```

## Rust Parity

The Daligault model and MD-based transport are in `barracuda/src/physics/transport.rs`.
Production MD sweeps (N=500, 50+ $\Gamma$ values) are validated against
the analytical fit.

## Provenance

- **Papers:** Stanton & Murillo PRE 91 (2015), Daligault PRE 86 (2012)
- **Control:** `control/sarkas/simulations/transport-study/scripts/daligault_fit.py`
- **Validation:** `barracuda/src/physics/transport.rs`
- **Next:** Full transport tensor via GPU MD + Green-Kubo

---
*hotSpring — ecoPrimals · AGPL-3.0 · [primals.eco](https://primals.eco)*

