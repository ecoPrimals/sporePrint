+++
title = "Anaerobic Biogas Kinetics & Community Diversity — Track 6"
description = "Rendered from liao-anaerobic-biogas.ipynb"
date = 2026-06-13
weight = 50

[extra]
domain = "Lab"
rendered_from = "liao-anaerobic-biogas.ipynb"
+++

<!-- Auto-generated from liao-anaerobic-biogas.ipynb by spore-validate render-notebooks -->

# Anaerobic Biogas Kinetics & Community Diversity — Track 6

This notebook derives four classical **biogas / growth-rate** formulations used in anaerobic digestion literature and closes with **microbial diversity** summaries for reactor communities (Zhong-style indices). Companion automation: **`scripts/python_anaerobic_biogas_baseline.py`** writes frozen numerics consumed by **`validate_anaerobic_biogas`** (and **`validate_barracuda_cpu_v27`** domains D69–D70) in **`barracuda/`**.

| # | Paper | Model |
|---|-------|-------|
| 1 | Yang et al. 2016 (Adv Microbiol 6:879-897) | Modified Gompertz |
| 2 | Chen et al. 2016 (Biomass Bioenergy 85:84-93) | First-order kinetics |
| 3 | Rojas-Sossa et al. 2017 (Bioresour Technol 245:714-723) | Monod growth |
| 4 | Rojas-Sossa et al. 2019 (Biomass Bioenergy 127:105263) | Haldane substrate inhibition |
| 5 | Zhong et al. 2016 (Biotechnol Biofuels 9:253) | Diversity indices |

---

**Script**: `scripts/python_anaerobic_biogas_baseline.py`. **Rust binary**: `validate_anaerobic_biogas` (canonical frozen JSON matcher in CI).

```python
import json
import math
import os
import socket
import struct
from pathlib import Path

import matplotlib.pyplot as plt
import numpy as np

RESULTS = Path('..') / '..' / 'experiments' / 'results'


def load(name):
    with open(RESULTS / name) as f:
        return json.load(f)


TIER = 'frozen'
IPC_SOCKET = os.environ.get('WETSPRING_IPC_SOCKET')


def ipc_call(method, params=None):
    sock = socket.socket(socket.AF_UNIX, socket.SOCK_STREAM)
    sock.connect(IPC_SOCKET)
    req = json.dumps(
        {'jsonrpc': '2.0', 'method': method, 'params': params or {}, 'id': 1}
    )
    payload = req.encode()
    sock.sendall(struct.pack('<I', len(payload)) + payload)
    length = struct.unpack('<I', sock.recv(4))[0]
    data = sock.recv(length)
    sock.close()
    return json.loads(data)['result']


if IPC_SOCKET and os.path.exists(IPC_SOCKET):
    try:
        ipc_call('health.check')
        TIER = 'live_ipc'
        print(f'Tier 2 ACTIVE — live IPC via {IPC_SOCKET}')
    except Exception:
        print('Tier 2 socket found but not responding — using frozen data')
else:
    print('Tier 1 — frozen data (no IPC socket)')
```

## a) Modified Gompertz biogas production (Yang et al. 2016)

Cumulative production $H(t)$ (modified Gompertz):

$$ H(t) = P \times \exp\!\left(-\exp\Big(\frac{R_m e}{P}(\lambda - t) + 1\Big)\right)$$

with $P$: ultimate yield, $R_m$: max production rate parameter, $\lambda$: lag phase (days). Pedagogical parameters: $P=350$ mL/gVS, $R_m=25$ mL/gVS/d, $\lambda=2.5$ d — curve shown for $t \in [0, 30]$ d.

```python
import math
def gompertz(t, P, Rm, lambda_):
    inner = (Rm * math.e / P) * (lambda_ - t) + 1
    return P * math.exp(-math.exp(inner))
```

```python
def gompertz(t, P, Rm, lambda_):
    inner = (Rm * math.e / P) * (lambda_ - t) + 1
    return P * math.exp(-math.exp(inner))


P_y, Rm_y, lam_y = 350.0, 25.0, 2.5
t_ped = np.linspace(0, 30, 400)
H_ped = np.array([gompertz(float(ti), P_y, Rm_y, lam_y) for ti in t_ped])

fig, ax = plt.subplots(figsize=(8, 4.8))
ax.plot(t_ped, H_ped, color='#2980b9', linewidth=2.5, label='H(t) modified Gompertz')
ax.axhline(P_y, color='#7f8c8d', linestyle='--', linewidth=1.2, label=f'Asymptote P = {P_y:g}')
ax.axvline(lam_y, color='#c0392b', linestyle=':', linewidth=2, alpha=0.85, label=f'Lag λ = {lam_y:g} d')
imax = int(np.argmax(np.gradient(H_ped)))
ax.scatter([t_ped[imax]], [H_ped[imax]], color='#27ae60', zorder=5, s=60, label='Max slope region (Rm-linked)')
ax.annotate(
    rf'$R_m$={Rm_y:g}',
    xy=(t_ped[imax], H_ped[imax]),
    xytext=(t_ped[imax] + 6, H_ped[imax] + 35),
    arrowprops=dict(arrowstyle='->', color='#27ae60'),
    fontsize=10,
)
ax.set_xlabel('Time t (days)')
ax.set_ylabel('H(t) (mL/gVS)')
ax.set_title('Yang-style modified Gompertz biogas (pedagogical parameters)')
ax.legend(loc='lower right')
ax.grid(alpha=0.3)
plt.tight_layout()
plt.show()
```

## b) First-order kinetics (Chen et al. 2016)

$$ B(t) = B_{\\max}\bigl(1 - e^{-kt}\bigr) $$

Parameters: $B_{\\max}=320$ mL/gVS, $k=0.08$ d⁻¹.

```python
def first_order(t, B_max, k):
    return B_max * (1 - math.exp(-k * t))
```

## c) Monod growth (Rojas-Sossa et al. 2017)

$$ \mu = \mu_{\\max}\, \frac{S}{K_s + S} $$

Scan $S \in [0, 5000]$ mg/L with $\mu_{\\max}=0.35$ d⁻¹, $K_s=500$ mg/L.

```python
def monod(S, mu_max, Ks):
    return mu_max * S / (Ks + S)
```

## d) Haldane inhibition (Rojas-Sossa et al. 2019)

$$ \mu = \mu_{\\max}\, \frac{S}{K_s + S + S^2/K_i} $$

Parameters: $\mu_{\\max}=0.35$, $K_s=500$, $K_i=3000$ mg/L — exhibits a **substrate inhibition peak** at finite $S$.

```python
def haldane(S, mu_max, Ks, Ki):
    return mu_max * S / (Ks + S + S**2 / Ki)
```

```python
def first_order(t, B_max, k):
    return B_max * (1 - math.exp(-k * t))


def monod(S, mu_max, Ks):
    if S <= 0:
        return 0.0
    return mu_max * S / (Ks + S)


def haldane(S, mu_max, Ks, Ki):
    if S <= 0:
        return 0.0
    return mu_max * S / (Ks + S + S**2 / Ki)


B_max, k_chen = 320.0, 0.08
G_ped = np.array([gompertz(float(ti), P_y, Rm_y, lam_y) for ti in t_ped])
B_ped = np.array([first_order(float(ti), B_max, k_chen) for ti in t_ped])

mu_ped_max, Ks_ped, Ki_ped = 0.35, 500.0, 3000.0
S_ped = np.linspace(0.0, 5000.0, 500)
mu_m = np.array([monod(float(S), mu_ped_max, Ks_ped) for S in S_ped])
mu_h = np.array([haldane(float(S), mu_ped_max, Ks_ped, Ki_ped) for S in S_ped])
S_opt_analytic = math.sqrt(Ks_ped * Ki_ped)

fig2, axes = plt.subplots(2, 2, figsize=(11, 9))

ax = axes[0, 0]
ax.plot(t_ped, G_ped, color='#2980b9', label='Gompertz H(t)', linewidth=2)
ax.set_xlabel('t (days)')
ax.set_ylabel('H(t) (mL/gVS)')
ax.set_title('Modified Gompertz — cumulative biogas')
ax.legend(); ax.grid(alpha=0.3)

ax = axes[0, 1]
ax.plot(t_ped, B_ped, color='#e67e22', label=r'First-order B(t)', linewidth=2)
ax.set_xlabel('t (days)')
ax.set_ylabel('B(t) (mL/gVS)')
ax.set_title('First-order — cumulative biogas')
ax.legend(); ax.grid(alpha=0.3)

ax = axes[1, 0]
ax.plot(S_ped[S_ped > 0], mu_m[S_ped > 0], label='Monod', color='#16a085', linewidth=2)
ax.set_xlim(0, 5000)
ax.set_xlabel('Substrate S (mg/L)')
ax.set_ylabel(r'Growth rate $\mu$ (day$^{-1}$)')
ax.set_title('Monod — no inhibition')
ax.legend(); ax.grid(alpha=0.3)

ax = axes[1, 1]
ax.plot(S_ped[S_ped > 0], mu_h[S_ped > 0], label='Haldane', color='#c0392b', linewidth=2)
ax.scatter([S_opt_analytic], [haldane(S_opt_analytic, mu_ped_max, Ks_ped, Ki_ped)], s=85, marker='o', edgecolors='k', facecolors='#f1c40f', zorder=6, label=rf'$S^{{*}}$={S_opt_analytic:.0f}')
ax.set_xlim(0, 5000)
ax.set_xlabel('Substrate S (mg/L)')
ax.set_ylabel(r'Growth rate $\mu$ (day$^{-1}$)')
ax.set_title('Haldane — inhibition peak')
ax.legend(fontsize=8); ax.grid(alpha=0.3)

plt.suptitle('Four kinetic models — 2×2 paper panels (Tracks 6, papers 1–4)', fontsize=13, fontweight='bold')
plt.tight_layout()
plt.show()
```

## e) Diversity indices for anaerobic communities (Zhong et al. 2016)

**Shannon**: $H' = - \sum_i p_i \ln p_i$. **Simpson**: $D = 1 - \sum_i p_i^2$.

Synthetic **relative-abundance style** OTU/read counts illustrate three digester regimes: startup (uneven pioneers), stable ( richer, more balanced ), inhibited (dominance collapse of diversity).

```python
def shannon(counts):
    arr = np.array(counts, dtype=float)
    arr = arr[arr > 0]
    total = arr.sum()
    if total <= 0:
        return 0.0
    p = arr / total
    return float(-np.sum(p * np.log(p)))


def simpson(counts):
    arr = np.array(counts, dtype=float)
    arr = arr[arr > 0]
    total = arr.sum()
    if total <= 0:
        return 0.0
    p = arr / total
    return float(1.0 - np.sum(p**2))


stages = {
    'Startup': [62.0, 18.0, 9.0, 6.0, 3.0, 2.0],
    'Stable': [21.0, 19.0, 17.5, 16.0, 14.5, 12.0],
    'Inhibited': [78.0, 10.0, 5.0, 3.5, 2.2, 1.3],
}

H_vals = [shannon(stages[s]) for s in stages]
D_vals = [simpson(stages[s]) for s in stages]
labs = list(stages.keys())

x = np.arange(len(labs))
w = 0.35
fig, ax = plt.subplots(figsize=(7.5, 4.2))
ax.bar(x - w / 2, H_vals, w, label="Shannon H'", color='#3498db')
ax.bar(x + w / 2, D_vals, w, label='Simpson D', color='#9b59b6')
ax.set_xticks(x)
ax.set_xticklabels(labs)
ax.set_ylabel('Index value')
ax.set_title('Synthetic digester microbiomes — startup vs stable vs inhibited')
ax.legend()
ax.grid(alpha=0.3, axis='y')
plt.tight_layout()
plt.show()

for i, nm in enumerate(labs):
    print(f"  {nm:10s}: H'={H_vals[i]:.4f}, Simpson D={D_vals[i]:.4f}")
```

## Rust parity — frozen `biogas_kinetics_baseline.json`

`experiments/results/track6_anaerobic/biogas_kinetics_baseline.json` is emitted by **`python_anaerobic_biogas_baseline.py`**. Rows below **recompute** with the fixture’s parameters (**note**: canonical Gompertz uses $\lambda{=}3$ d for case 1, while the pedagogical curve above uses $\lambda{=}2.5$).

```python
fro = load('track6_anaerobic/biogas_kinetics_baseline.json')


def assert_close(a, b, rtol=1e-9):
    assert abs(a - b) <= rtol * max(1.0, abs(b)), (a, b)


g1 = fro['gompertz']['case1_yang_manure']
for ti, Hi in zip(g1['t'], g1['H']):
    assert_close(
        gompertz(float(ti), float(g1['P']), float(g1['Rm']), float(g1['lambda'])), Hi
    )

g2 = fro['gompertz']['case2_corn_stover']
for ti, Hi in zip(g2['t'], g2['H']):
    assert_close(
        gompertz(float(ti), float(g2['P']), float(g2['Rm']), float(g2['lambda'])), Hi
    )

fo = fro['first_order']
for ti, Bi in zip(fo['t'], fo['B']):
    assert_close(first_order(float(ti), float(fo['B_max']), float(fo['k'])), Bi)

mn = fro['monod']
for Si, mui in zip(mn['S'], mn['mu']):
    assert_close(monod(float(Si), float(mn['mu_max']), float(mn['Ks'])), mui)

hd = fro['haldane']
for Si, mui in zip(hd['S'], hd['mu']):
    assert_close(
        haldane(float(Si), float(hd['mu_max']), float(hd['Ks']), float(hd['Ki'])),
        mui,
    )

aer_ct = fro['diversity']['aerobic']['counts']
ana_ct = fro['diversity']['anaerobic']['counts']
assert_close(shannon(aer_ct), fro['diversity']['aerobic']['shannon'])
assert_close(simpson(aer_ct), fro['diversity']['aerobic']['simpson'])
assert_close(shannon(ana_ct), fro['diversity']['anaerobic']['shannon'], rtol=1e-6)
assert_close(simpson(ana_ct), fro['diversity']['anaerobic']['simpson'], rtol=1e-6)

print('Rust parity bookkeeping: PASSED recomputation checks vs frozen baseline')
print(
    "  Frozen metadata command:",
    fro.get('metadata', {}).get('command', '(no metadata.command)'),
)
print(
    "  Script SHA (metadata):",
    str(fro.get('metadata', {}).get('script_sha256', '(n/a)'))[:16],
    'git:', fro.get('metadata', {}).get('git_commit', 'n/a'),
)
```

## Tier 2 — IPC parity (`science.biogas_kinetics`, guarded)

When **`WETSPRING_IPC_SOCKET`** points at a responding daemon, optionally mirror the consolidated Track‑6 kinetic bundle; failures fall back silently to Tier 1 JSON.

```python
if TIER == 'live_ipc':
    try:
        live = ipc_call(
            'science.biogas_kinetics',
            {
                'artifact': 'track6/biogas_kinetics',
                'gompertz_cases': True,
                'first_order': {'B_max': 320.0, 'k': 0.08},
                'monod': {'mu_max': 0.4, 'Ks': 200},
                'haldane': {'mu_max': 0.4, 'Ks': 200, 'Ki': 3000},
            },
        )
        print('science.biogas_kinetics IPC response:', type(live).__name__, str(live)[:500])
    except Exception as exc:
        print('science.biogas_kinetics IPC failed:', exc)
else:
    print('Skipping IPC — Tier 1 frozen mode (set WETSPRING_IPC_SOCKET for Tier 2).')
```

## Summary — validation anchor, provenance, evolution

| Check | Fixture / reference | Comparator |
|-------|---------------------|--------------|
| Gompertz (2 feed cases) | `track6_anaerobic/biogas_kinetics_baseline.json` | Matches `gompertz_*` |
| First-order B(t) | same file | $\pm$ float ULP |
| Monod / Haldane grids | same file | $\pm$ float ULP |
| Diversity aerobic vs anaerobic | frozen counts + Shannon/Simpson | Matches script `python_anaerobic_biogas_baseline.py` |
| Anderson disorder hook | frozen `anderson_w` | **Rust** validates $W$-mapping consistency |

**Provenance**: Regenerate artifacts with **`python3 scripts/python_anaerobic_biogas_baseline.py`** → **`experiments/results/track6_anaerobic/biogas_kinetics_baseline.json`**. **Rust validation** via **`wetspring validate --scenario barracuda_cpu_v27`** (domains D69–D70).

**Evolution**  
**Tier 1 — frozen**: this notebook runs everywhere with pathlib-derived **`RESULTS`**. **Tier 2**: guarded **`ipc_call('science.biogas_kinetics', …)`**. **Tier 3**: deterministic CI binaries with provenance hashes in **`barracuda/`**.

