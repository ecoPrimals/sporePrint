+++
title = "JAK/STAT Pharmacology — Gonzales Lab"
description = "Rendered from gonzales-jak-pharmacology.ipynb"
date = 2026-06-20
weight = 50

[extra]
domain = "Lab"
rendered_from = "gonzales-jak-pharmacology.ipynb"
+++

<!-- Auto-generated from gonzales-jak-pharmacology.ipynb by spore-validate render-notebooks -->

# JAK/STAT Pharmacology — Gonzales Lab

This notebook walks through Gonzales-lab dermatology pharmacology spanning
oclacitinib JAK inhibition, anti-pruritic PK scaffolding, IL-31–driven itch
signaling, and tissue compartment diversity—all with visible NumPy/Python
computations tied to reproducible Tier-1 artifacts and optional Tier-2 IPC.

| # | Paper | DOI | Model |
|---|-------|-----|-------|
| 1 | Gonzales et al. 2014 | [10.1016/j.clinthera.2014.01.014](https://doi.org/10.1016/j.clinthera.2014.01.014) | IC50 dose-response (Hill equation) |
| 2 | Fleck & Gonzales 2021 | [10.1016/j.tvjl.2020.105603](https://doi.org/10.1016/j.tvjl.2020.105603) | PK exponential decay |
| 3 | Gonzales et al. 2013 | [10.1111/vde.12013](https://doi.org/10.1111/vde.12013) | IL-31/pruritus pathway |

**Rust validation:** `gonzales_ic50_s79`, `gonzales_pk_s79`, `gonzales_provenance_chain`. Run from the wetSpring checkout via UniBin, e.g. `wetspring validate --scenario gonzales_ic50_s79`.

---

*For other springs:* swap the cytokine/JAK axes and dosing units for your
domain while keeping the same workflow—explicit paper parameters,
Python-visible math, `experiments/results` baselines, then guarded IPC probes.

```python
import os, json, struct, socket
from pathlib import Path
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
    print(f'Tier 1 — frozen data (no IPC socket)')

import matplotlib
import matplotlib.pyplot as plt

PASS_COLOR = '#2ecc71'
FAIL_COLOR = '#e74c3c'
INFO_COLOR = '#3498db'
```

## Gonzales 2014 — IC50 dose-response (Hill equation)

**4-parameter logistic (inhibition framing):**

$$\mathrm{Response} = \mathrm{Bottom} + \frac{\mathrm{Top}-\mathrm{Bottom}}{1 + (\mathrm{IC50}/[\mathrm{Drug}])^n}$$

Published IC50 values for oclacitinib **enzyme-level** JAK inhibition (Table 1, clinic therapeutics review / vet pharmacology literature) drive the sweep below. The wetSpring frozen domain JSON instead carries the **cytokine pathway** Table-1 panel used by `science.gonzales.dose_response`—we reconcile both in the Rust parity cell.

**Selectivity (this notebook panel):** JAK3/JAK1 = $310\times$, TYK2/JAK1 = $220\times$.

```python
# Oclacitinib IC50 values (nM) from Gonzales 2014 Table 1
targets = {
    'JAK1': {'ic50_nM': 10.0, 'hill_n': 1.0},
    'JAK2': {'ic50_nM': 18.0, 'hill_n': 1.0},
    'JAK3': {'ic50_nM': 3100.0, 'hill_n': 1.0},
    'TYK2': {'ic50_nM': 2200.0, 'hill_n': 1.0},
}

def hill_dose_response(conc_nM, ic50, hill_n, top=100, bottom=0):
    if conc_nM <= 0:
        return top
    return bottom + (top - bottom) / (1 + (ic50 / conc_nM)**hill_n)

dose_points = np.array([0.1, 1.0, 10.0, 100.0, 1000.0, 10000.0], dtype=float)

responses = {}
for tgt, cfg in targets.items():
    responses[tgt] = np.array([
        hill_dose_response(c, cfg['ic50_nM'], cfg['hill_n']) for c in dose_points
    ])

sel_jak3_vs_jak1 = targets['JAK3']['ic50_nM'] / targets['JAK1']['ic50_nM']
sel_tyk2_vs_jak1 = targets['TYK2']['ic50_nM'] / targets['JAK1']['ic50_nM']
print(f'Selectivity JAK3/JAK1 = {sel_jak3_vs_jak1:.0f}×')
print(f'Selectivity TYK2/JAK1 = {sel_tyk2_vs_jak1:.0f}×')

fig, axes = plt.subplots(1, 2, figsize=(14, 5))
palette = {'JAK1': '#e74c3c', 'JAK2': '#3498db', 'JAK3': '#f39c12', 'TYK2': '#9b59b6'}

ax = axes[0]
for tgt, cfg in targets.items():
    ax.semilogx(dose_points, responses[tgt], '-o',
                label=f"{tgt} (IC50={cfg['ic50_nM']:.1f} nM)", color=palette[tgt], linewidth=2)
ax.set_xlabel('Oclacitinib concentration (nM)')
ax.set_ylabel('Response (%)')
ax.set_title('Log dose vs Hill response — 4 JAK family targets')
ax.axhline(50, color='#555', linestyle=':', alpha=0.5)
ax.legend(fontsize=8)
ax.grid(True, alpha=0.3)

ax = axes[1]
ratios = [sel_jak3_vs_jak1, sel_tyk2_vs_jak1]
labels = ['JAK3 / JAK1', 'TYK2 / JAK1']
bars = ax.bar(labels, ratios, color=[palette['JAK3'], palette['TYK2']])
ax.set_ylabel('IC50 fold-change vs JAK1')
ax.set_title('Selectivity ratios (higher = more selective for JAK1)')
for bar, r in zip(bars, ratios):
    ax.text(bar.get_x() + bar.get_width() / 2, bar.get_height() + 5,
            f'{r:.0f}×', ha='center', va='bottom', fontweight='bold')
ax.grid(True, alpha=0.3, axis='y')

plt.suptitle('Gonzales 2014 — oclacitinib JAK IC50 sweep', fontsize=13, fontweight='bold')
plt.tight_layout()
plt.savefig('/tmp/gonzales_ic50_notebook.png', dpi=150, bbox_inches='tight')
plt.show()
```

## Fleck & Gonzales 2021 — PK exponential decay

**One-compartment IV bolus**

$$C(t) = \frac{Dose}{V_d} e^{-k_{el} t},\quad k_{el} = \frac{\ln 2}{t_{1/2}}$$

Concentrations are reported in ng/mL using the dosing scaffold below.
The shaded band marks an **illustrative** therapeutic window; replace the
bounds with clinically validated thresholds for your manuscript.

```python
import math

pk_params = {
    'low_dose': {'dose_mg': 0.4, 'vd_L_per_kg': 2.9, 'half_life_hr': 4.1, 'weight_kg': 10.0},
    'high_dose': {'dose_mg': 0.6, 'vd_L_per_kg': 2.9, 'half_life_hr': 4.1, 'weight_kg': 10.0},
}

def pk_decay(t_hr, dose_mg, vd_L_per_kg, half_life_hr, weight_kg):
    import math
    k_el = math.log(2) / half_life_hr
    vd = vd_L_per_kg * weight_kg
    c0 = (dose_mg * 1e6) / (vd * 1000)  # ng/mL
    return c0 * math.exp(-k_el * t_hr)

t_hr = np.linspace(0.0, 24.0, 200)
curves = {}
for name, p in pk_params.items():
    curves[name] = np.array([pk_decay(t, **{k: p[k] for k in
        ('dose_mg', 'vd_L_per_kg', 'half_life_hr', 'weight_kg')}) for t in t_hr])

c_lo, c_hi = 15.0, 120.0  # illustrative ng/mL window

fig, ax = plt.subplots(figsize=(11, 5))
ax.axhspan(c_lo, c_hi, color=PASS_COLOR, alpha=0.15, label='Therapeutic band (example)')
ax.plot(t_hr, curves['low_dose'], color='#3498db', linewidth=2, label=f"Low dose ({pk_params['low_dose']['dose_mg']} mg)")
ax.plot(t_hr, curves['high_dose'], color='#e74c3c', linewidth=2, label=f"High dose ({pk_params['high_dose']['dose_mg']} mg)")
ax.set_xlabel('Time (h)')
ax.set_ylabel('Plasma concentration (ng/mL)')
ax.set_title('One-compartment PK decay — Fleck & Gonzales 2021 scaffolding')
ax.legend()
ax.grid(True, alpha=0.3)
plt.tight_layout()
plt.savefig('/tmp/gonzales_pk_notebook.png', dpi=150, bbox_inches='tight')
plt.show()

# Report Cmax and times crossing therapeutic band
for nm, ys in curves.items():
    print(f"{nm}: C0 ≈ {ys[0]:.1f} ng/mL, C24 ≈ {ys[-1]:.2f} ng/mL")
_ = math
```

## Gonzales 2013 — IL-31 pruritus pathway

IL-31 engages the heterodimeric receptor **IL-31RA/OSMR**, recruiting **JAK1/JAK2** and phosphorylating **STAT3/STAT5**. The activation matrix below encodes illustrative relative strengths for notebook visualization—not a calibrated fit.

```python
pathway = {
    'IL-31': {'receptor': 'IL-31RA/OSMR', 'jak': 'JAK1/JAK2', 'stat': 'STAT3/STAT5'},
}
```

```python
pathway = {
    'IL-31': {'receptor': 'IL-31RA/OSMR', 'jak': 'JAK1/JAK2', 'stat': 'STAT3/STAT5'},
}

# Illustrative phosphorylation scores (0–1) for heatmap rows
row_labels = ['IL-31 (cytokine)', 'IL-31RA', 'OSMR', 'JAK1', 'JAK2', 'STAT3', 'STAT5']
scores = np.array([
    [1.00],
    [0.95],
    [0.94],
    [0.92],
    [0.91],
    [0.89],
    [0.74],
])
_ = pathway  # keep dict available for prose exports

fig, ax = plt.subplots(figsize=(3.5, 6))
im = ax.imshow(scores, aspect='auto', cmap='OrRd', vmin=0, vmax=1)
ax.set_xticks([0])
ax.set_xticklabels(['Cascade activation'])
ax.set_yticks(np.arange(len(row_labels)))
ax.set_yticklabels(row_labels)
ax.set_title('IL-31 → JAK/STAT activation heatmap (illustrative)')
cb = plt.colorbar(im, ax=ax, fraction=0.046, pad=0.04)
cb.set_label('Relative score')
plt.tight_layout()
plt.savefig('/tmp/gonzales_il31_pathway_notebook.png', dpi=150, bbox_inches='tight')
plt.show()
```

## Tissue lattice diversity

Shannon diversity $H^{\prime} = -\sum_i p_i \ln p_i$ across **four tissue compartments**
(plasma, skin, liver, kidney) using representative immune/parenchyma mixtures.
These vectors are pedagogical—for AD severity lattice metrics see `science.gonzales.tissue_lattice` / `gonzales_domain.json`.

```python
def shannon_evenness(counts):
    counts = np.asarray(counts, dtype=float)
    total = counts.sum()
    if total <= 0:
        return 0.0
    p = counts / total
    p = p[p > 0]
    return float(-(p * np.log(p)).sum())

# Four shared "species" axes: lymphoid, myeloid, parenchyma, stromal
species = ['lymphoid', 'myeloid', 'parenchyma', 'stromal']
mixtures = {
    'plasma': [55, 25, 5, 15],
    'skin': [15, 20, 50, 15],
    'liver': [8, 12, 70, 10],
    'kidney': [10, 15, 60, 15],
}

compartments = list(mixtures.keys())
H_vals = np.array([shannon_evenness(mixtures[c]) for c in compartments])

fig, ax = plt.subplots(figsize=(9, 4))
colors = ['#3498db', '#e67e22', '#2ecc71', '#9b59b6']
bars = ax.bar(compartments, H_vals, color=colors)
ax.set_ylabel("Shannon H'")
ax.set_title('Tissue compartment diversity (illustrative mixtures)')
for bar, h in zip(bars, H_vals):
    ax.text(bar.get_x() + bar.get_width() / 2, bar.get_height() + 0.02,
            f"{h:.2f}", ha='center', va='bottom', fontsize=10)
ax.grid(True, alpha=0.3, axis='y')
plt.tight_layout()
plt.savefig('/tmp/gonzales_tissue_diversity_notebook.png', dpi=150, bbox_inches='tight')
plt.show()

print('Species mixture (counts):')
for comp, vec in mixtures.items():
    print(f'  {comp:>6}: {dict(zip(species, vec))}')
```

## Rust parity — frozen baselines

Load `experiments/results/gonzales_domain.json` (and any optional `280_gonzales_ic50` snapshot) to cross-check Table-1 IC50 values against this notebook's `targets` dict.

```python
domain_path = RESULTS / 'gonzales_domain.json'
with open(domain_path) as f:
    dom = json.load(f)

ic50_tbl = dom['gonzales_2014']['table_1_ic50']
print('Frozen cytokine/pathway IC50 panel (wetSpring Tier-1):')
for key, row in ic50_tbl.items():
    print(f"  {key:14s}  IC50 = {row['ic50_nm']:.2f} nM   ({row['pathway']})")

frozen_jak1 = ic50_tbl['JAK1_enzyme']['ic50_nm']
nb_jak1 = targets['JAK1']['ic50_nM']
jak1_ok = abs(frozen_jak1 - nb_jak1) < 1e-9
print()
print(f'Notebook JAK1 IC50: {nb_jak1} nM')
print(f'Frozen JAK1_enzyme: {frozen_jak1} nM  → match: {jak1_ok}')

print()
print('Notebook-only JAK2/JAK3/TYK2 enzyme IC50s are not serialized in gonzales_domain.json;')
print('they intentionally complement the cytokine-centric IPC handler for pedagogy.')

alt_ic50_dir = RESULTS / '280_gonzales_ic50'
if alt_ic50_dir.is_dir():
    print()
    print(f'Optional snapshot {alt_ic50_dir}:')
    for p in sorted(alt_ic50_dir.glob('*.json')):
        print(f'  {p.name}')
else:
    print()
    print('No experiments/results/280_gonzales_ic50 directory in working tree (optional).')

_ = jak1_ok
```

## Tier 2 — IPC parity (guarded)

When `WETSPRING_IPC_SOCKET` points at a live barracuda IPC server, call the
published Gonzales handlers. `science.gonzales.pk_decay` models **lokivetmab efficacy vs. days** (Fleck et al. framing in-repo), which differs from the one-compartment ng/mL scaffold above—both are kept side-by-side intentionally.

```python
if TIER == 'live_ipc':
    try:
        dom_ipc = load('gonzales_domain.json')
        ic50_ipc = dom_ipc['gonzales_2014']['table_1_ic50']
        fro_map = {}
        for key, val in ic50_ipc.items():
            pname = 'JAK1' if key == 'JAK1_enzyme' else key
            fro_map[pname] = val['ic50_nm']
        dr_live = ipc_call('science.gonzales.dose_response', {
            'n_points': 50,
            'dose_max': 500.0,
            'hill_n': 1.0,
        })
        live_map = {c['pathway']: c['ic50_nm'] for c in dr_live['curves']}
        print('IPC science.gonzales.dose_response IC50 (nM):')
        for k in sorted(live_map):
            print(f'  {k:6s}  {live_map[k]:.2f}')
        for pathway, ic in live_map.items():
            if pathway in fro_map and abs(fro_map[pathway] - ic) > 1e-3:
                print(f'WARN: {pathway} mismatch frozen={fro_map[pathway]} live={ic}')
        pk_live = ipc_call('science.gonzales.pk_decay', {
            'n_points': 100,
            't_max_days': 56.0,
        })
        print('\nIPC science.gonzales.pk_decay:', 'k_decay =', pk_live.get('k_decay'))
        print('  dose_profiles:', len(pk_live.get('dose_profiles', [])))
        print('Tier 2 IPC calls completed without transport errors.')
    except Exception as exc:
        print('Tier 2 IPC call failed:', exc)
else:
    print('Tier 2 inactive — skipping ipc_call probes (frozen tier).')
```

## Summary

| Check | Artifact / command | Role |
|-------|-------------------|------|
| IC50 Table-1 parity | `validate_gonzales_ic50_s79` | 35-check Hill + ordering vs. Gonzales 2014 |
| PK scaffolding | `validate_gonzales_pk` (`validate_gonzales_pk_s79`) | Exponential decay + duration tiering |
| Provenance chain | `validate_gonzales_provenance_chain` | End-to-end BLAKE3 / IPC envelope closure |

**Papers covered:** (1) Gonzales et al. 2014 — JAK-selective IC50 Hill surfaces; (2) Fleck & Gonzales 2021 — anti-pruritic PK decay family; (3) Gonzales et al. 2013 — IL-31 receptor/JAK/STAT itch axis context.

**Provenance:** Tier-1 parameters live under `experiments/results/gonzales_domain.json` with optional experiment subfolders (e.g. `280_gonzales_ic50/`). Notebook math is intentionally open so reviewers can diff against frozen JSON and Rust validators.

**Evolution path:** Tier 1 (this notebook, local NumPy) → Tier 2 (`WETSPRING_IPC_SOCKET`, `science.gonzales.*`) → Tier 3 (provenance sessions + composition nodes) matching the broader wetSpring nucleus pattern.

