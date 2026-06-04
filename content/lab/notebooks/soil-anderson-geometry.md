+++
title = "Soil Quorum Sensing & Anderson Geometry — Track 4"
description = "Rendered from soil-anderson-geometry.ipynb"
date = 2026-06-04
weight = 50

[extra]
domain = "Lab"
rendered_from = "soil-anderson-geometry.ipynb"
+++

<!-- Auto-generated from soil-anderson-geometry.ipynb by spore-validate render-notebooks -->

# Soil Quorum Sensing & Anderson Geometry — Track 4

| # | Paper | Experiment | Model |
|---|-------|-----------|-------|
| 1 | Martínez-García et al. 2023 | Exp170 | Pore geometry → Anderson disorder |
| 2 | Feng et al. 2024 | Exp171 | Pore-scale diversity |
| 3 | Mukherjee et al. 2024 | Exp172 | Distance colonization |
| 4 | Islam et al. 2014 | Exp173 | Brandt farm no-till |
| 5 | Zuber et al. 2016 | Exp174 | No-till meta-analysis |
| 6 | Liang et al. 2015 | Exp175 | 31-year tillage factorial |
| 7 | Tecon et al. 2017 | Exp176 | Biofilm in aggregates |
| 8 | Rabot et al. 2018 | Exp177 | Structure-function |
| 9 | Wang et al. 2025 | Exp178 | Tillage microbiome |

**Narrative:** How soil pore geometry controls bacterial QS and community assembly, bridging Anderson localization physics with microbial ecology.


```python
import os, json, struct, socket
from pathlib import Path
import numpy as np
import matplotlib
import matplotlib.pyplot as plt
from scipy.stats import norm

RESULTS = Path('..') / '..' / 'experiments' / 'results'

def load(name):
    with open(RESULTS / name) as f:
        return json.load(f)

TIER = 'frozen'
IPC_SOCKET = os.environ.get('WETSPRING_IPC_SOCKET')

def ipc_call(method, params=None):
    sock = socket.socket(socket.AF_UNIX, socket.SOCK_STREAM)
    sock.connect(IPC_SOCKET)
    req = json.dumps({'jsonrpc': '2.0', 'method': method, 'params': params or {}, 'id': 1})
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

PASS_COLOR = '#2ecc71'
FAIL_COLOR = '#e74c3c'
INFO_COLOR = '#3498db'
ACCENT = '#9b59b6'
```

## Martínez-García 2023 — Pore Geometry → Anderson Disorder

Map characteristic pore size to a disorder parameter $W$ (Anderson-type effective disorder in the pore network) and to the probability of QS activation, using a smooth link through connectivity. The critical disorder $W_{C,3D}$ anchors the logistic tail (via a Gaussian CDF proxy).

```python
W_C_3D = 16.5  # critical disorder (3D Anderson-type proxy)

def pore_to_anderson(pore_um):
    connectivity = min((pore_um / 75.0)**2, 1.0)
    effective_w = 25.0 * (1.0 - connectivity)
    qs_prob = norm.cdf((W_C_3D - effective_w) / 3.0)
    return connectivity, effective_w, qs_prob

pore_scan = np.array([4, 10, 30, 50, 100, 150], dtype=float)
rows = [pore_to_anderson(p) for p in pore_scan]
conn = np.array([r[0] for r in rows])
w_eff = np.array([r[1] for r in rows])
qs_p = np.array([r[2] for r in rows])

fig, axes = plt.subplots(1, 2, figsize=(12, 4.5))
ax = axes[0]
ax.plot(pore_scan, qs_p, 'o-', color=INFO_COLOR, linewidth=2, markersize=8)
ax.set_xlabel('Pore size (µm)')
ax.set_ylabel('QS activation probability')
ax.set_title('Pore size vs QS probability')
ax.grid(alpha=0.3)

ax = axes[1]
ax.plot(pore_scan, w_eff, 's-', color=FAIL_COLOR, linewidth=2, markersize=8)
ax.axhline(W_C_3D, color=ACCENT, linestyle='--', label=f'$W_{{C,3D}}$ = {W_C_3D}')
ax.set_xlabel('Pore size (µm)')
ax.set_ylabel('Effective disorder W')
ax.set_title('Pore size vs disorder W')
ax.legend()
ax.grid(alpha=0.3)

plt.suptitle('Martínez-García et al. 2023 — pore network → Anderson disorder → QS', fontsize=12, fontweight='bold')
plt.tight_layout()
plt.show()
```

## Chemotaxis benefit under disorder

Chemotaxis tightens spatial coupling and lowers effective disorder. We model a **15%** reduction in $W$ when chemotaxis is active (Exp170 frozen baseline uses the same reduction). Benefit is measured as the gain in QS activation probability.

```python
def chemotaxis_benefit(w):
    p_no = norm.cdf((W_C_3D - w) / 3.0)
    p_yes = norm.cdf((W_C_3D - w * 0.85) / 3.0)
    return p_yes - p_no

W_grid = np.linspace(0.0, 30.0, 200)
benefit = np.array([chemotaxis_benefit(w) for w in W_grid])

fig, ax = plt.subplots(figsize=(10, 4.5))
ax.plot(W_grid, benefit, color=PASS_COLOR, linewidth=2)
ax.set_xlabel('Disorder W')
ax.set_ylabel('ΔQS probability (chemotaxis on − off)')
ax.set_title('Chemotaxis benefit vs disorder')
ax.grid(alpha=0.3)
plt.tight_layout()
plt.show()
```

## Tillage effects on diversity

**Islam 2014, Zuber 2016, Liang 2015** contrast no-till with conventional tillage. Below, **synthetic** Poisson-drawn communities target published ranges for Shannon diversity; Chao1 uses singleton/doubleton excess; Simpson reports $1-\sum_i p_i^2$.

```python
rng = np.random.default_rng(7)

def synthetic_metrics(mean_shannon, n_taxa, evenness_hint):
    # Multinomial-style draws skewed by Dirichlet concentration to mimic published ranges (Islam/Shannon anchors; pore diversity paper Simpson scale).
    conc = np.exp(mean_shannon / max(np.log(max(n_taxa, 4)), 0.001))
    alpha = rng.uniform(conc * evenness_hint, conc / evenness_hint, size=n_taxa)
    counts = rng.poisson(alpha * 200)
    counts = counts[counts > 0]
    if len(counts) < 12:
        return synthetic_metrics(mean_shannon, n_taxa, evenness_hint * 1.05)
    N = counts.sum()
    ps = counts / N
    shannon = -np.sum(ps * np.log(ps))
    simp_dom = np.sum(ps ** 2)
    simp_div = 1.0 - simp_dom
    s_obs = len(ps)
    f1 = np.sum(counts == 1)
    f2 = np.sum(counts == 2)
    chao1 = s_obs + (f1 * f1) / (2 * max(f2, 1)) if f2 > 0 else s_obs + f1 * (f1 - 1) / 2.0
    return shannon, simp_div, chao1

# Targets from merged literature ranges (Islam 2014 Shannon; Liang factorial richness swings; pore-diversity Simpson scale)
profiles = (
    ('No-till synthetic', 5.42, 220, 1.06),
    ('Conventional synthetic', 4.72, 148, 0.94),
)

n_boot = 500
means = {}
for label, h_tar, nt, ev in profiles:
    sh_vals, sd_vals, c_vals = [], [], []
    for _ in range(n_boot):
        h, sd, cc = synthetic_metrics(h_tar, nt, ev)
        sh_vals.append(h); sd_vals.append(sd); c_vals.append(cc)
    means[label] = {
        'shannon': np.mean(sh_vals),
        'simpson_1_lambda': np.mean(sd_vals),
        'chao1': np.mean(c_vals),
    }

labels_arr = np.arange(len(means))
fig, axes = plt.subplots(1, 3, figsize=(13, 4))
metric_keys = [('shannon', 'Shannon H'), ('simpson_1_lambda', 'Simpson (1 − Σ$p_i^2$)'), ('chao1', 'Chao1')]
colors = [INFO_COLOR, FAIL_COLOR]
for ax, (key, ttl) in zip(axes, metric_keys):
    vals = [means[p[0]][key] for p in profiles]
    ax.bar(labels_arr, vals, color=colors)
    ax.set_xticks(labels_arr)
    ax.set_xticklabels([p[0] for p in profiles], rotation=12, ha='right', fontsize=8)
    ax.set_title(ttl)
    ax.grid(alpha=0.3, axis='y')
plt.suptitle('No-till vs conventional — synthetic diversity (published-range anchors)', fontsize=12, fontweight='bold')
plt.tight_layout()
plt.show()

isl = load('173_notill_brandt_farm/islam2014_python_baseline.json')
print('Islam2014 frozen Shannon — NT %.3f, tilled %.3f' % (
    isl['diversity']['notill_mean_shannon'], isl['diversity']['tilled_mean_shannon']))
```

## Tecon 2017 — biofilm fraction in aggregates

Simple surface-limited uptake: biofilm occupies outer shells; the **fraction of aggregate volume occupied by biofilm** rises with surface-to-volume ratio (smaller aggregates have higher $S/V$).

```python
d_mm = np.array([0.25, 0.5, 1.0, 2.0, 5.0])
sv = 6.0 / d_mm  # mm^-1 for sphere
kappa = 0.08     # scales film thickness × S/V into fractional volume (illustrative)
biofilm_frac = 1.0 - np.exp(-kappa * sv)

fig, ax = plt.subplots(figsize=(8.5, 4.5))
ax.plot(d_mm, biofilm_frac, 'o-', color=ACCENT, linewidth=2, markersize=9)
ax.set_xlabel('Aggregate diameter (mm)')
ax.set_ylabel('Biofilm volume fraction')
ax.set_title('Aggregate size vs modeled biofilm fraction (Tecon 2017 scaffold)')
ax.grid(alpha=0.3)
plt.tight_layout()
plt.show()
```

## Rust parity — frozen Track 4 baselines

Load JSON under `experiments/results/` produced by Rust-validated pipelines (Tier 1). Inline `pore_to_anderson` matches `martinez2023_python_baseline.json`; chemotaxis sweep matches the embedded disorder points.

```python
baselines = {
    'Exp170': '170_soil_qs_pore_geometry/martinez2023_python_baseline.json',
    'Exp171': '171_soil_pore_diversity/feng2024_python_baseline.json',
    'Exp172': '172_soil_distance_colonization/mukherjee2024_python_baseline.json',
    'Exp173': '173_notill_brandt_farm/islam2014_python_baseline.json',
    'Exp174': '174_notill_meta_analysis/zuber2016_python_baseline.json',
    'Exp175': '175_notill_longterm_tillage/liang2015_python_baseline.json',
    'Exp176': '176_soil_biofilm_aggregate/tecon2017_python_baseline.json',
    'Exp177': '177_soil_structure_function/rabot2018_python_baseline.json',
    'Exp178': '178_tillage_microbiome/wang2025_python_baseline.json',
}

loaded = {}
for exp, rel in baselines.items():
    p = RESULTS / rel
    loaded[exp] = load(rel) if p.exists() else None
    ok = loaded[exp] is not None
    print(f'{exp}: {"OK" if ok else "MISSING"} {rel}')

fr = loaded['Exp170']
for row in fr['pore_mapping']:
    pu = row['pore_um']
    c, w, q = pore_to_anderson(pu)
    np.testing.assert_allclose(c, row['connectivity'], rtol=1e-9, atol=1e-9)
    np.testing.assert_allclose(w, row['effective_w'], rtol=1e-9, atol=1e-9)
    np.testing.assert_allclose(q, row['qs_probability'], rtol=1e-9, atol=1e-10)
print('Exp170 pore mapping parity: PASS')

for pt in fr['chemotaxis']['disorder_sweep']:
    wb = pt['w']
    b_obs = pt['benefit']
    np.testing.assert_allclose(chemotaxis_benefit(wb), b_obs, rtol=1e-9, atol=1e-9)
print('Exp170 chemotaxis sweep parity: PASS')

print('\nRust binaries: validate_soil_qs_pore_geometry (170), validate_soil_pore_diversity (171), '
      'validate_soil_distance_colonization (172), validate_notill_brandt_farm (173), validate_notill_meta_analysis (174), '
      'validate_notill_longterm_tillage (175), validate_soil_biofilm_aggregate (176), validate_soil_structure_function (177), '
      'validate_tillage_microbiome_2025 (178)')
```

## Tier 2 — live IPC (`science.soil_geometry`)

Guarded JSON-RPC hook. When `WETSPRING_IPC_SOCKET` is live, calls the barracuda method with probe parameters; Tier 1 leaves this cell as documentation-only.

```python
if TIER == 'live_ipc':
    try:
        live_geom = ipc_call('science.soil_geometry', {'pore_um': 50.0, 'w_c_3d': W_C_3D, 'chemotaxis_reduction': 0.15})
        print('science.soil_geometry live response:', live_geom)
        if isinstance(live_geom, dict) and 'qs_probability' in live_geom:
            ref = next(r for r in load('170_soil_qs_pore_geometry/martinez2023_python_baseline.json')['pore_mapping'] if abs(r['pore_um'] - 50.0) < 0.01)
            if abs(live_geom['qs_probability'] - ref['qs_probability']) > 1e-6:
                print('WARN: live qs_probability differs from frozen at 50 µm')
            else:
                print('Tier 2 parity probe: QS probability matches frozen @ 50 µm')
    except Exception as e:
        print('science.soil_geometry unavailable or schema mismatch:', e)
else:
    print('Tier 2 idle — Tier 1 frozen tier (no live soil_geometry assertion).')
```

## Summary

| Experiment | Topic | Frozen baseline | Rust binary |
|-----------|-------|-----------------|-------------|
| Exp170 | Pore geometry → Anderson | `170_soil_qs_pore_geometry/` | `validate_soil_qs_pore_geometry` |
| Exp171 | Pore-scale diversity | `171_soil_pore_diversity/` | `validate_soil_pore_diversity` |
| Exp172 | Distance colonization | `172_soil_distance_colonization/` | `validate_soil_distance_colonization` |
| Exp173 | Brandt farm no-till | `173_notill_brandt_farm/` | `validate_notill_brandt_farm` |
| Exp174 | No-till meta-analysis | `174_notill_meta_analysis/` | `validate_notill_meta_analysis` |
| Exp175 | 31-year tillage factorial | `175_notill_longterm_tillage/` | `validate_notill_longterm_tillage` |
| Exp176 | Aggregate biofilm | `176_soil_biofilm_aggregate/` | `validate_soil_biofilm_aggregate` |
| Exp177 | Structure–function | `177_soil_structure_function/` | `validate_soil_structure_function` |
| Exp178 | Tillage microbiome 2025 | `178_tillage_microbiome/` | `validate_tillage_microbiome_2025` |

**Provenance.** Parameters and frozen JSON originate from cited papers and Track 4 validation scripts (`barracuda` binaries above). Paths resolve from `notebooks/papers/` via `RESULTS = Path('..') / '..' / 'experiments' / 'results'`.

**Evolution path.** Tier 1 (notebook + scipy + frozen JSON assertions) → Tier 2 (`science.soil_geometry` live parity when IPC is configured) → Tier 3 bundled provenance sessions and publication artifacts.

