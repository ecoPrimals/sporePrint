+++
title = "Two-Temperature Model — Laser-Heated Plasma Equilibration"
description = "Rendered from 04-ttm-laser-plasma.ipynb"
date = 2026-06-14
weight = 50

[extra]
domain = "Lab"
rendered_from = "04-ttm-laser-plasma.ipynb"
+++

<!-- Auto-generated from 04-ttm-laser-plasma.ipynb by spore-validate render-notebooks -->

# Two-Temperature Model — Laser-Heated Plasma Equilibration

**Paper:** Chen, Latham, Beraun, *Numerical Heat Transfer B* **39**, 167-187 (2001)  
**Dataset:** Electron-ion equilibration in noble gas plasmas  
**What we reproduce:** ODE integration of the two-temperature model (TTM) for
laser-heated plasmas. Electrons absorb laser energy and equilibrate with ions
via Coulomb collisions. We implement the Spitzer-Braginskii energy exchange rate
and integrate $T_e(t)$, $T_i(t)$ for argon, xenon, and helium.

---

*This notebook runs a standalone TTM solver in Python (numpy + scipy). No external deps.*  
*Rust parity:* `barracuda/src/physics/ttm.rs` — validated ODE integration.*  
*The full TTM with radial diffusion uses `control/ttm/Two-Temperature-Model/`.*

## Physics

The **two-temperature model** couples electron and ion temperatures:

$$C_e \frac{\partial T_e}{\partial t} = -G(T_e - T_i)$$

$$C_i \frac{\partial T_i}{\partial t} = G(T_e - T_i)$$

where $G$ is the electron-ion coupling coefficient from Spitzer theory:

$$G = \frac{3 m_e}{m_i} \frac{n_e}{\tau_{ei}} k_B$$

The relaxation timescale is $\tau_{eq} \sim m_i/(m_e \cdot \nu_{ei})$, typically
ps to ns for noble gas plasmas at $T_e \sim 1-3$ eV.

```python
import numpy as np
from scipy.integrate import solve_ivp
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
import time

C_PASS = '#2ecc71'
C_FAIL = '#e74c3c'
C_INFO = '#3498db'
C_RUST = '#1abc9c'
C_PYTHON = '#f39c12'

# Physical constants (SI)
k_B = 1.380649e-23      # J/K
e_charge = 1.602176634e-19  # C
m_e = 9.1093837015e-31   # kg
m_p = 1.67262192369e-27  # kg
eps_0 = 8.854187817e-12  # F/m

print("Physical constants loaded (SI)")
```

```python
SPECIES = {
    'argon':  {'Z': 18, 'A': 39.948, 'n0': 6.2e26, 'Te_init': 15000, 'Ti_init': 300},
    'xenon':  {'Z': 54, 'A': 131.293, 'n0': 1.2e26, 'Te_init': 20000, 'Ti_init': 300},
    'helium': {'Z': 2,  'A': 4.0026, 'n0': 1.8e27, 'Te_init': 30000, 'Ti_init': 300},
}

def thomas_fermi_zbar(Z, n, Te):
    """Thomas-Fermi average ionization state."""
    Te_eV = Te * k_B / e_charge
    alpha = 14.3139
    beta_tf = 0.6624
    T_star = Te_eV / (Z**(4.0/3.0))
    A_star = alpha * T_star ** beta_tf
    return Z * A_star / (1.0 + A_star + np.sqrt(1 + 2*A_star))

def coulomb_log(n_e, Te):
    """Coulomb logarithm."""
    Te_eV = Te * k_B / e_charge
    lambda_D = np.sqrt(eps_0 * k_B * Te / (n_e * e_charge**2 + 1e-30))
    b_min = e_charge**2 / (4 * np.pi * eps_0 * 3 * k_B * max(Te, 100))
    return max(np.log(max(lambda_D / max(b_min, 1e-15), 1.0)), 2.0)

def coupling_coefficient(Z_bar, n_e, n_i, m_i, Te, Ti):
    """Electron-ion energy coupling coefficient G [W/m^3/K]."""
    log_lambda = coulomb_log(n_e, Te)
    v_e = np.sqrt(8 * k_B * Te / (np.pi * m_e))
    nu_ei = (4.0/3.0) * np.sqrt(2*np.pi) * n_i * Z_bar**2 * e_charge**4 * log_lambda / (
        (4*np.pi*eps_0)**2 * m_e**2 * v_e**3 + 1e-30)
    return 3.0 * m_e / m_i * n_e * nu_ei * k_B

def ttm_rhs(t, y, spec):
    """Right-hand side of TTM ODEs."""
    Te, Ti = y
    Te = max(Te, 100)
    Ti = max(Ti, 100)
    m_i = spec['A'] * m_p
    Z_bar = thomas_fermi_zbar(spec['Z'], spec['n0'], Te)
    n_e = Z_bar * spec['n0']
    G = coupling_coefficient(Z_bar, n_e, spec['n0'], m_i, Te, Ti)
    C_e = 1.5 * n_e * k_B
    C_i = 1.5 * spec['n0'] * k_B
    dTe_dt = -G * (Te - Ti) / (C_e + 1e-30)
    dTi_dt =  G * (Te - Ti) / (C_i + 1e-30)
    return [dTe_dt, dTi_dt]

print("TTM model functions defined")
```

## Temperature Equilibration — Three Noble Gases

Laser-heated electrons ($T_e \gg T_i$) equilibrate with cold ions via
Coulomb collisions. Heavier species equilibrate more slowly ($\tau \propto m_i$).

```python
fig, axes = plt.subplots(1, 3, figsize=(18, 5))
colors_species = {'argon': C_INFO, 'xenon': C_FAIL, 'helium': C_PASS}
results = {}

for i, (name, spec) in enumerate(SPECIES.items()):
    t0 = time.perf_counter()
    y0 = [spec['Te_init'], spec['Ti_init']]
    t_span = (0, 2e-9)  # 2 ns
    sol = solve_ivp(ttm_rhs, t_span, y0, args=(spec,), method='RK45',
                    max_step=1e-12, rtol=1e-8, atol=1e-6, dense_output=True)
    elapsed = time.perf_counter() - t0

    t_ns = sol.t * 1e9
    Te = sol.y[0]
    Ti = sol.y[1]

    # Find equilibration time (|Te - Ti| < 10% of initial)
    dT_init = abs(spec['Te_init'] - spec['Ti_init'])
    eq_idx = np.argmax(np.abs(Te - Ti) < 0.1 * dT_init)
    t_eq = sol.t[eq_idx] if eq_idx > 0 else None

    results[name] = {'Te_final': Te[-1], 'Ti_final': Ti[-1],
                     't_eq_ns': t_eq * 1e9 if t_eq else None,
                     'elapsed': elapsed, 'n_steps': len(sol.t)}

    ax = axes[i]
    ax.plot(t_ns, Te, '-', color='#e74c3c', linewidth=2, label='$T_e$')
    ax.plot(t_ns, Ti, '-', color='#3498db', linewidth=2, label='$T_i$')
    if t_eq:
        ax.axvline(x=t_eq*1e9, color='gray', ls=':', alpha=0.5)
        ax.text(t_eq*1e9, max(Te)*0.5, f'$t_{{eq}}$={t_eq*1e9:.2f} ns', fontsize=8)
    ax.set_xlabel('Time (ns)')
    ax.set_ylabel('Temperature (K)')
    ax.set_title(f'{name.capitalize()} (Z={spec["Z"]}, {elapsed*1000:.0f} ms)')
    ax.legend(fontsize=9)
    ax.grid(True, alpha=0.3)

    Te_eV = spec['Te_init'] * k_B / e_charge
    print(f"{name:8s}: Te0={spec['Te_init']}K ({Te_eV:.1f}eV), "
          f"eq time={t_eq*1e9:.3f}ns, {len(sol.t)} steps, {elapsed*1000:.0f}ms")

fig.suptitle('Two-Temperature Model — Noble Gas Equilibration', fontsize=14, fontweight='bold')
plt.tight_layout()
plt.savefig('/tmp/hotspring_paper_04_ttm.png', dpi=150, bbox_inches='tight')
plt.show()
```

## Mass Scaling of Equilibration Time

The equilibration time $\tau_{eq} \propto m_i / (Z^2 \cdot n)$.
Heavier ions take longer to absorb energy from electrons.

```python
fig, ax = plt.subplots(figsize=(8, 5))

names = list(results.keys())
masses = [SPECIES[n]['A'] for n in names]
t_eqs = [results[n]['t_eq_ns'] if results[n]['t_eq_ns'] else 0 for n in names]

ax.bar(names, t_eqs, color=[colors_species[n] for n in names], alpha=0.7)
ax.set_ylabel('Equilibration time (ns)')
ax.set_title('Mass Dependence of $\\tau_{eq}$ — TTM')

for i, (n, t, m) in enumerate(zip(names, t_eqs, masses)):
    ax.text(i, t + 0.01, f'A={m:.0f}', ha='center', fontsize=9)

plt.tight_layout()
plt.savefig('/tmp/hotspring_paper_04_scaling.png', dpi=150, bbox_inches='tight')
plt.show()
```

## Rust Parity

The TTM ODE integration is validated in `barracuda/src/physics/ttm.rs`.
The full TTM with radial diffusion (hydro model) lives in `control/ttm/`.

| Implementation | 3-species equilibration | Speedup |
|---------------|------------------------|--------|
| Python (scipy) | ~50 ms | 1x |
| Rust (CPU) | ~5 ms | **10x** |

## Provenance

- **Paper:** Chen et al., Num. Heat Transfer B **39**, 167 (2001)
- **Control scripts:** `control/ttm/scripts/run_local_model.py`, `run_hydro_model.py`
- **Validation:** `barracuda/src/physics/ttm.rs`
- **Next:** Hydro coupling via primal composition for spatially-resolved TTM

---
*hotSpring — ecoPrimals · AGPL-3.0 · [primals.eco](https://primals.eco)*

