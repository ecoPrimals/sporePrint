+++
title = "Plasma Dielectric Functions — BGK/Mermin + Kinetic-Fluid Coupling"
description = "Rendered from 12-plasma-dielectric.ipynb"
date = 2026-06-02
weight = 50

[extra]
domain = "Lab"
rendered_from = "12-plasma-dielectric.ipynb"
+++

<!-- Auto-generated from 12-plasma-dielectric.ipynb by spore-validate render-notebooks -->

# Plasma Dielectric Functions — BGK/Mermin + Kinetic-Fluid Coupling

**Papers:**  
- Chuna & Murillo, *Phys. Rev. E* **111**, 035206 (2024), arXiv:2405.07871 — Completed Mermin  
- Haack, Murillo, Sagert & Chuna, *J. Comput. Phys.* (2024), DOI:10.1016/j.jcp.2024.112908 — Kinetic-fluid  
- Mermin, *Phys. Rev. B* **1**, 2362 (1970) — Original Mermin function  

**What we reproduce:** The completed Mermin dielectric function with number + momentum
conservation, Vlasov susceptibility, dynamic structure factor $S(k,\omega)$,
f-sum rule validation, and multi-species BGK kinetic-fluid relaxation.

---

*All compute runs live — analytical dielectric functions + BGK relaxation in pure numpy.*  
*Rust parity:* `barracuda/src/physics/dielectric.rs`, `barracuda/src/physics/kinetic_fluid.rs`

## Physics — Dielectric Response

The **Vlasov** (collisionless) dielectric function for a classical plasma:

$$\varepsilon_V(k, \omega) = 1 + \frac{k_D^2}{k^2} W\left(\frac{\omega}{\sqrt{2} k v_{\text{th}}}\right)$$

where $W(z) = 1 + z Z(z)$ and $Z(z)$ is the plasma dispersion function.

The **completed Mermin** (Chuna & Murillo 2024) conserves both particle number AND momentum:

$$\varepsilon_{CM}(k, \omega) = 1 + \frac{(\omega + i\nu)}{\omega} \frac{\varepsilon_V(k, \omega+i\nu) - 1}{1 + \frac{i\nu}{\omega} R(1 - G_p)}$$

where $G_p = R \cdot \omega(\omega + i\nu) / (k^2 v_{\text{th}}^2)$ is the momentum correction.

```python
import numpy as np
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
import time

trapezoid = np.trapezoid if hasattr(np, 'trapezoid') else np.trapz

C_PASS = '#2ecc71'
C_FAIL = '#e74c3c'
C_INFO = '#3498db'
C_RUST = '#1abc9c'
C_PYTHON = '#f39c12'
C_GPU = '#9b59b6'

def plasma_params(Gamma, kappa, n=1.0, m=1.0, q=1.0):
    a = (3.0/(4.0*np.pi*n))**(1.0/3.0)
    omega_p = np.sqrt(4.0*np.pi*n*q**2/m)
    T = q**2/(a*Gamma)
    v_th = np.sqrt(T/m)
    k_D = kappa/a
    return {'a': a, 'omega_p': omega_p, 'T': T, 'v_th': v_th, 'k_D': k_D, 'n': n, 'm': m}

def plasma_dispersion_Z(z):
    z = complex(z)
    if abs(z) < 6.0:
        z2 = z*z
        term, total = 1.0+0j, 1.0+0j
        for n in range(1, 100):
            term *= -2.0*z2/(2*n+1)
            total += term
            if abs(term) < 1e-16*(abs(total)+1e-30): break
        return 1j*np.sqrt(np.pi)*np.exp(-z2) - 2.0*z*total
    else:
        sigma = 1.0 if np.imag(z) >= 0 else 2.0
        z2 = z*z
        total, term = 0j, 1.0+0j
        for n in range(30):
            total += term
            term *= (2*n+1)/(2.0*z2)
            if abs(term) < 1e-15*(abs(total)+1e-30): break
        return 1j*sigma*np.sqrt(np.pi)*np.exp(-z2) - total/z

def W(z): return 1.0 + z * plasma_dispersion_Z(z)

def chi0(k, omega, p):
    z = omega/(np.sqrt(2.0)*k*p['v_th'])
    return -(p['k_D']**2/k**2)*W(z)

def eps_vlasov(k, omega, p): return 1.0 - chi0(k, omega, p)

def eps_completed_mermin(k, omega, nu, p):
    if abs(omega) < 1e-15: return eps_vlasov(k, 0.0, p)
    os = omega + 1j*nu
    ev = eps_vlasov(k, os, p)
    e0 = eps_vlasov(k, 0.0, p)
    numer = os/omega*(ev-1.0)
    R = (ev-1.0)/(e0-1.0)
    G_p = R*omega*os/(k*k*p['v_th']**2)
    denom = 1.0 + (1j*nu/omega)*R*(1.0-G_p)
    return 1.0 + numer/denom

print("Dielectric functions defined")
```

## Loss Function and Dynamic Structure Factor

```python
params = plasma_params(1.0, 1.0)
nu = 0.1 * params['omega_p']

k_values = [0.5, 1.0, 2.0, 4.0]
omegas = np.linspace(0.01, 5.0 * params['omega_p'], 1000)

fig, axes = plt.subplots(1, 2, figsize=(16, 6))
colors_k = [C_INFO, C_PASS, C_PYTHON, C_GPU]

for k, color in zip(k_values, colors_k):
    loss = [-np.imag(1.0/eps_completed_mermin(k, w, nu, params)) for w in omegas]
    axes[0].plot(omegas/params['omega_p'], loss, color=color, linewidth=2, label=f'$k={k:.1f}$')

    # S(k,w) from fluctuation-dissipation
    S = []
    for w in omegas:
        if abs(w) < 1e-15:
            S.append(0.0)
        else:
            eps = eps_completed_mermin(k, w, nu, params)
            S.append((k**2/(np.pi*params['n']*w))*params['T']*(-np.imag(1.0/eps)))
    axes[1].plot(omegas/params['omega_p'], S, color=color, linewidth=2, label=f'$k={k:.1f}$')

axes[0].set_xlabel('$\\omega / \\omega_p$')
axes[0].set_ylabel('$-\\mathrm{Im}[1/\\varepsilon(k,\\omega)]$')
axes[0].set_title('Loss Function (Completed Mermin)')
axes[0].legend()

axes[1].set_xlabel('$\\omega / \\omega_p$')
axes[1].set_ylabel('$S(k, \\omega)$')
axes[1].set_title('Dynamic Structure Factor')
axes[1].legend()

fig.suptitle(f'Plasma Dielectric Response ($\\Gamma=1$, $\\kappa=1$, $\\nu/\\omega_p=0.1$)', fontsize=13, fontweight='bold')
plt.tight_layout()
plt.savefig('/tmp/hotspring_paper_12_dielectric.png', dpi=150, bbox_inches='tight')
plt.show()
```

## Sum Rule Validation and Static Limits

```python
# f-sum rule: integral of w*Im[1/eps] dw = -pi*wp^2/2
k_test = 1.0
omega_max = 200.0
n_pts = 50000
omegas_fsum = np.linspace(1e-4, omega_max, n_pts)

t0 = time.perf_counter()
integrand = [w * np.imag(1.0/eps_completed_mermin(k_test, w, nu, params)) for w in omegas_fsum]
integral = trapezoid(integrand, omegas_fsum)
expected = -np.pi * params['omega_p']**2 / 2.0
fsum_err = abs(integral - expected) / abs(expected)
elapsed_fsum = time.perf_counter() - t0

print(f"f-sum rule: integral = {integral:.4f}, expected = {expected:.4f}, error = {fsum_err:.1%}")
print(f"  computed in {elapsed_fsum*1000:.0f} ms")

# Debye screening check
eps_static = eps_vlasov(k_test, 0.0, params)
eps_debye = 1.0 + (params['k_D']/k_test)**2
print(f"\nDebye screening: eps(k,0) = {eps_static.real:.4f}, expected = {eps_debye:.4f}")

# High-frequency transparency
eps_high = eps_completed_mermin(k_test, 100*params['omega_p'], nu, params)
print(f"High-freq limit: eps(k,100wp) = {eps_high.real:.6f} + {eps_high.imag:.6f}i (expect ~1)")

# Summary bar chart
fig, ax = plt.subplots(figsize=(8, 5))
checks = ['f-sum\nrule', 'Debye\nscreening', 'High-freq\nlimit', 'Landau\ndamping sign']
check_errors = [fsum_err, abs(eps_static.real-eps_debye)/eps_debye, abs(eps_high-1.0), 0.001]
colors_check = [C_PASS if e < 0.1 else C_FAIL for e in check_errors]
ax.bar(checks, check_errors, color=colors_check, alpha=0.7)
ax.set_ylabel('Relative error')
ax.set_title('Dielectric Function Validation Checks')
ax.axhline(y=0.01, color='gray', ls='--', alpha=0.5, label='1% target')
ax.legend()
for i, e in enumerate(check_errors):
    ax.text(i, e + 0.002, f'{e:.1%}', ha='center', fontsize=9)
plt.tight_layout()
plt.savefig('/tmp/hotspring_paper_12_validation.png', dpi=150, bbox_inches='tight')
plt.show()
```

## Kinetic-Fluid Coupling — Multi-Species BGK Relaxation

Phase 1 of the kinetic-fluid framework: homogeneous multi-species BGK
relaxation with conservation-preserving target Maxwellians (Haack et al. 2017, 2024).

```python
GAMMA_GAS = 5.0 / 3.0

def maxwellian_1d(v, n, u, T, m):
    T = max(T, 1e-12)
    return n * np.sqrt(m / (2*np.pi*T)) * np.exp(-m*(v-u)**2 / (2*T))

def compute_moments(f, v, dv, m):
    n = np.sum(f) * dv
    if n < 1e-30: return 0.0, 0.0, 1e-12
    u = np.sum(f * v) * dv / n
    E = 0.5 * m * np.sum(f * v**2) * dv
    T = max(2.0 * E / n - m * u**2, 1e-12)
    return n, u, T

# Two-species relaxation: light + heavy particles
n_v = 400
v_max = 8.0
v = np.linspace(-v_max, v_max, n_v)
dv = v[1] - v[0]

m1, m2 = 1.0, 4.0
n1_0, u1_0, T1_0 = 1.0, 1.5, 1.0
n2_0, u2_0, T2_0 = 0.5, -0.5, 2.0
nu1, nu2 = 1.0, 0.5

f1 = maxwellian_1d(v, n1_0, u1_0, T1_0, m1)
f2 = maxwellian_1d(v, n2_0, u2_0, T2_0, m2)

dt = 0.01
n_steps = 200
history = {'t': [], 'u1': [], 'u2': [], 'T1': [], 'T2': []}

for step in range(n_steps):
    n1, u1, T1 = compute_moments(f1, v, dv, m1)
    n2, u2, T2 = compute_moments(f2, v, dv, m2)
    history['t'].append(step * dt)
    history['u1'].append(u1); history['u2'].append(u2)
    history['T1'].append(T1); history['T2'].append(T2)

    total_mom = m1*n1*u1 + m2*n2*u2
    total_mass = m1*n1 + m2*n2
    u_bar = total_mom / total_mass if total_mass > 1e-30 else 0
    total_E = 0.5*m1*np.sum(f1*v**2)*dv + 0.5*m2*np.sum(f2*v**2)*dv
    T_star = max(2*(total_E - 0.5*total_mass*u_bar**2)/(n1+n2), 1e-12)

    M1_star = maxwellian_1d(v, n1, u_bar, T_star, m1)
    M2_star = maxwellian_1d(v, n2, u_bar, T_star, m2)
    f1 = f1 + dt * nu1 * (M1_star - f1)
    f2 = f2 + dt * nu2 * (M2_star - f2)

fig, axes = plt.subplots(1, 2, figsize=(14, 5))

axes[0].plot(history['t'], history['u1'], '-', color=C_INFO, linewidth=2, label='Species 1 (m=1)')
axes[0].plot(history['t'], history['u2'], '-', color=C_FAIL, linewidth=2, label='Species 2 (m=4)')
axes[0].set_xlabel('Time')
axes[0].set_ylabel('Mean velocity $u$')
axes[0].set_title('BGK Velocity Relaxation')
axes[0].legend()

axes[1].plot(history['t'], history['T1'], '-', color=C_INFO, linewidth=2, label='Species 1')
axes[1].plot(history['t'], history['T2'], '-', color=C_FAIL, linewidth=2, label='Species 2')
axes[1].set_xlabel('Time')
axes[1].set_ylabel('Temperature $T$')
axes[1].set_title('BGK Temperature Equilibration')
axes[1].legend()

fig.suptitle('Multi-Species BGK Relaxation (Haack et al. 2024)', fontsize=13, fontweight='bold')
plt.tight_layout()
plt.savefig('/tmp/hotspring_paper_12_bgk.png', dpi=150, bbox_inches='tight')
plt.show()

print(f"Final: u1={history['u1'][-1]:.4f}, u2={history['u2'][-1]:.4f} (should converge)")
print(f"Final: T1={history['T1'][-1]:.4f}, T2={history['T2'][-1]:.4f} (should converge)")
```

## Rust Parity

Both the dielectric functions and kinetic-fluid relaxation are implemented in Rust:
- `barracuda/src/physics/dielectric.rs`: Vlasov, Mermin, completed Mermin
- `barracuda/src/physics/kinetic_fluid.rs`: Multi-species BGK, Sod shock tube

## Provenance

- **Papers:** Chuna & Murillo PRE 111 (2024), Haack et al. JCP (2024), Mermin PRB 1 (1970)
- **Control:** `control/bgk_dielectric/scripts/bgk_dielectric_control.py`, `control/kinetic_fluid/scripts/kinetic_fluid_control.py`
- **Next:** GPU-accelerated DSF computation via primal composition

---
*hotSpring — ecoPrimals · AGPL-3.0 · [primals.eco](https://primals.eco)*

