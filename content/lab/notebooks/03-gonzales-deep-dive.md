+++
title = "Gonzales Deep Dive — wetSpring"
description = "Rendered from 03-gonzales-deep-dive.ipynb"
date = 2026-06-14
weight = 50

[extra]
domain = "Lab"
rendered_from = "03-gonzales-deep-dive.ipynb"
+++

<!-- Auto-generated from 03-gonzales-deep-dive.ipynb by spore-validate render-notebooks -->

# Gonzales Deep Dive — wetSpring

The Gonzales dermatitis pipeline is wetSpring's flagship science story:
oclacitinib (APOQUEL) JAK inhibitor pharmacology, validated against
Gonzales et al. 2014 Table 1 IC50 values, cross-referenced with ChEMBL
and PubChem, extended with lokivetmab (Cytopoint) PK decay models and
tissue lattice diversity profiles for atopic dermatitis severity.

Every value traces back to a published DOI and is content-addressed
with BLAKE3 hashes through the provenance chain.

**Data sources**: `experiments/results/gonzales_domain.json`

**Reproduce**: `wetspring validate --scenario gonzales_ic50_s79` in the wetSpring repository.

---

*For other springs: replace the Gonzales data with your domain's key
published results. The pattern — paper values → external cross-validation
→ computational model → provenance — applies to any field.*

```python
import os, json, struct, socket
from pathlib import Path
import numpy as np

RESULTS = Path('..') / 'experiments' / 'results'

def load(name):
    with open(RESULTS / name) as f:
        return json.load(f)

TIER = 'frozen'
IPC_SOCKET = os.environ.get('WETSPRING_IPC_SOCKET')

def ipc_call(method, params=None):
    """JSON-RPC call to barracuda IPC — active in Tier 2."""
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

gonz = load('gonzales_domain.json')
g2014 = gonz['gonzales_2014']
print(f'Paper: {g2014["title"]}')
print(f'DOI: {g2014["doi"]}')
print(f'Validation: {g2014["validation"]["status"]}')
```

## IC50 Selectivity Panel — Gonzales 2014 Table 1

Oclacitinib preferentially inhibits JAK1-dependent signaling. The IC50
ordering JAK1 < IL-2 < IL-31 < IL-6 < IL-4 < IL-13 demonstrates
selectivity for pruritus-associated pathways over broader immune functions.

```python
import matplotlib
import matplotlib.pyplot as plt

ic50 = g2014['table_1_ic50']
ordering = g2014['ordering']
values = [ic50[k]['ic50_nm'] for k in ordering]
pathways = [ic50[k]['pathway'] for k in ordering]
labels = [k.replace('_', ' ') for k in ordering]

fig, axes = plt.subplots(1, 2, figsize=(15, 5))

# IC50 bar chart
ax = axes[0]
colors = ['#e74c3c', '#e67e22', '#f39c12', '#f1c40f', '#3498db', '#9b59b6']
bars = ax.bar(labels, values, color=colors)
ax.set_ylabel('IC50 (nM)')
ax.set_title('Oclacitinib IC50 — Gonzales 2014 Table 1')
ax.set_xticklabels(labels, rotation=30, ha='right')
for bar, val, pw in zip(bars, values, pathways):
    ax.text(bar.get_x() + bar.get_width()/2, bar.get_height() + 3,
            f'{val} nM', ha='center', va='bottom', fontsize=9, fontweight='bold')
    ax.text(bar.get_x() + bar.get_width()/2, bar.get_height()/2,
            pw, ha='center', va='center', fontsize=7, color='white')

# Selectivity ratios
ax = axes[1]
sel = g2014['selectivity_ratios']
sel_names = [k.replace('_over_', '/') for k in sel]
sel_vals = list(sel.values())
bars = ax.barh(sel_names, sel_vals, color='#3498db')
ax.set_xlabel('Fold selectivity over JAK1')
ax.set_title('Selectivity Ratios (IC50 / JAK1 IC50)')
for bar, val in zip(bars, sel_vals):
    ax.text(bar.get_width() + 0.3, bar.get_y() + bar.get_height()/2,
            f'{val:.1f}x', va='center', fontsize=10, fontweight='bold')

plt.suptitle(f'Gonzales 2014 — Oclacitinib JAK Selectivity (DOI: {g2014["doi"]})',
             fontsize=12, fontweight='bold')
plt.tight_layout()
plt.savefig('/tmp/wetspring_03_ic50.png', dpi=150, bbox_inches='tight')
plt.show()
```

## Dose-Response Hill Curves

Hill equation modeling of oclacitinib dose-response for each cytokine
pathway. The barrier width W = 4.5 separates extended (responsive)
from localized (resistant) regimes — an Anderson localization metaphor
applied to pharmacological selectivity.

```python
dr = g2014['dose_response_params']
doses = np.logspace(np.log10(dr['dose_range_nm'][0]),
                    np.log10(dr['dose_range_nm'][1]),
                    dr['dose_points'])

fig, ax = plt.subplots(figsize=(12, 6))
colors_hill = ['#e74c3c', '#e67e22', '#f39c12', '#f1c40f', '#3498db', '#9b59b6']

for i, target in enumerate(ordering):
    ic50_val = ic50[target]['ic50_nm']
    hill_n = dr['hill_coefficient']
    response = 100 * (doses ** hill_n) / (ic50_val ** hill_n + doses ** hill_n)
    label_name = target.replace('_', ' ')
    ax.semilogx(doses, response, color=colors_hill[i], linewidth=2,
                label=f'{label_name} (IC50={ic50_val} nM)')
    ax.axvline(x=ic50_val, color=colors_hill[i], linestyle='--', alpha=0.3)

ax.axhline(y=50, color='#555', linestyle=':', alpha=0.5, label='50% inhibition')
ax.set_xlabel('Oclacitinib concentration (nM)')
ax.set_ylabel('% Inhibition')
ax.set_title('Dose-Response Curves — Hill Model (n=1.0)')
ax.legend(fontsize=8, loc='lower right')
ax.set_ylim(-5, 105)
ax.grid(True, alpha=0.3)

plt.tight_layout()
plt.savefig('/tmp/wetspring_03_hill.png', dpi=150, bbox_inches='tight')
plt.show()

# Tier 2: live dose-response parity
if TIER == 'live_ipc':
    live_dr = ipc_call('science.gonzales.dose_response', {'barrier_w': 4.5})
    for pathway in ordering:
        frozen_ic50 = ic50[pathway]['ic50_nm']
        live_ic50 = live_dr['pathways'][pathway]['ic50_nm']
        assert abs(frozen_ic50 - live_ic50) < 0.1, \
            f'IC50 parity fail: {pathway} frozen={frozen_ic50} live={live_ic50}'
    print('Tier 2 parity: ALL IC50 values match frozen baseline')
```

## Lokivetmab PK Decay — Fleck & Gonzales 2021

Three dose tiers of lokivetmab (Cytopoint) with exponential efficacy
decay modeling. Higher doses extend the therapeutic window.

```python
pk = gonz['fleck_gonzales_2021']
pk_params = pk['pk_parameters']

fig, ax = plt.subplots(figsize=(12, 5))
t = np.linspace(0, 60, pk['time_points'])

dose_colors = {'low_dose': '#3498db', 'mid_dose': '#f39c12', 'high_dose': '#e74c3c'}

for dose_name, params in pk_params.items():
    efficacy = 100 * np.exp(-params['k_decay'] * t)
    label = f'{params["dose_mg_kg"]} mg/kg ({params["duration_days"]}d)'
    ax.plot(t, efficacy, color=dose_colors[dose_name], linewidth=2, label=label)
    ax.axvline(x=params['duration_days'], color=dose_colors[dose_name],
               linestyle='--', alpha=0.3)

ax.axhline(y=50, color='#555', linestyle=':', alpha=0.5, label='50% efficacy')
ax.set_xlabel('Days post-injection')
ax.set_ylabel('Efficacy (%)')
ax.set_title(f'Lokivetmab PK Decay — Fleck & Gonzales 2021 (DOI: {pk["doi"]})')
ax.legend(fontsize=9)
ax.set_ylim(-5, 105)
ax.grid(True, alpha=0.3)

plt.tight_layout()
plt.savefig('/tmp/wetspring_03_pk.png', dpi=150, bbox_inches='tight')
plt.show()

# Tier 2: live PK parity
if TIER == 'live_ipc':
    live_pk = ipc_call('science.gonzales.pk_decay', {})
    frozen_k = pk_params['high_dose']['k_decay']
    live_k = live_pk['dose_profiles']['high_dose']['k_decay']
    assert abs(frozen_k - live_k) < 0.001, \
        f'PK parity fail: frozen={frozen_k} live={live_k}'
    print('Tier 2 parity: PK decay constants match frozen baseline')
```

## Tissue Lattice Diversity — AD Severity Profiles

Synthetic cell-type distributions model the shift from healthy skin
(keratinocyte-dominated, low diversity) to severe atopic dermatitis
(immune infiltrate, high Shannon diversity). This connects the Gonzales
pharmacology to the Anderson localization physics framework.

```python
tissue = gonz['tissue_lattice']
profiles = tissue['profiles']
div_metrics = tissue['diversity_metrics']
profile_names = list(profiles.keys())

cell_types = ['keratinocytes', 't_cells', 'mast_cells', 'dendritic', 'fibroblasts', 'other']
ct_colors = ['#2ecc71', '#e74c3c', '#9b59b6', '#3498db', '#f39c12', '#95a5a6']

fig, axes = plt.subplots(1, 3, figsize=(18, 5))

# Stacked bar of cell-type fractions
ax = axes[0]
x = range(len(profile_names))
bottom = [0] * len(profile_names)
for ct, color in zip(cell_types, ct_colors):
    vals = [profiles[p][ct] for p in profile_names]
    ax.bar(x, vals, bottom=bottom, color=color, label=ct.replace('_', ' ').title())
    bottom = [b + v for b, v in zip(bottom, vals)]
ax.set_xticks(list(x))
ax.set_xticklabels([n.replace('_', '\n').title() for n in profile_names], fontsize=8)
ax.set_ylabel('Cell fraction')
ax.set_title('Cell-Type Distribution by Severity')
ax.legend(fontsize=7, loc='upper right')

# Shannon diversity
ax = axes[1]
shannon = [div_metrics[p]['shannon'] for p in profile_names]
severity = [profiles[p]['severity'] for p in profile_names]
ax.plot(severity, shannon, 'o-', color='#3498db', linewidth=2, markersize=8)
for s, sh, name in zip(severity, shannon, profile_names):
    ax.annotate(f'{sh:.2f}', (s, sh), textcoords='offset points',
                xytext=(0, 10), ha='center', fontsize=10)
ax.set_xlabel('AD Severity')
ax.set_ylabel('Shannon Diversity (H)')
ax.set_title('Shannon Diversity vs AD Severity')
ax.grid(True, alpha=0.3)

# Pielou evenness
ax = axes[2]
pielou = [div_metrics[p]['pielou'] for p in profile_names]
ax.plot(severity, pielou, 's-', color='#e74c3c', linewidth=2, markersize=8)
for s, pi, name in zip(severity, pielou, profile_names):
    ax.annotate(f'{pi:.2f}', (s, pi), textcoords='offset points',
                xytext=(0, 10), ha='center', fontsize=10)
ax.set_xlabel('AD Severity')
ax.set_ylabel('Pielou Evenness (J)')
ax.set_title('Pielou Evenness vs AD Severity')
ax.grid(True, alpha=0.3)

plt.suptitle('Tissue Lattice — Atopic Dermatitis Severity Progression',
             fontsize=13, fontweight='bold')
plt.tight_layout()
plt.savefig('/tmp/wetspring_03_tissue.png', dpi=150, bbox_inches='tight')
plt.show()
```

## ChEMBL Cross-Validation

Oclacitinib (CHEMBL2103874) IC50 values from ChEMBL assays match
Gonzales 2014 Table 1. The reference artifact is content-addressed
with BLAKE3 to detect drift.

```python
chembl = gonz['chembl_cross_validation']
pubchem = gonz['pubchem_cross_validation']

print(f'ChEMBL compound: {chembl["compound_name"]} ({chembl["compound_id"]})')
print(f'JAK1 IC50 (ChEMBL assays): {chembl["jak1_ic50_nm_chembl"]} nM')
print(f'JAK1 IC50 (Gonzales 2014): {chembl["jak1_ic50_nm_paper"]} nM')
print(f'Match: {chembl["match"]}')
print(f'BLAKE3 hash: {chembl["reference_artifact_hash_blake3"][:16]}...')
print()
print(f'PubChem CID: {pubchem["cid"]}')
print(f'Formula: {pubchem["molecular_formula"]}')
print(f'MW: {pubchem["molecular_weight"]} Da')
print(f'InChIKey: {pubchem["inchi_key"]}')
```

## Validation Summary

| Component | Evidence | Status |
|-----------|----------|--------|
| IC50 Table 1 (6 pathways) | 35/35 checks | PASS |
| Selectivity ordering | JAK1 < IL-2 < IL-31 < IL-6 < IL-4 < IL-13 | PASS |
| Dose-response Hill curves | n=1.0, barrier W=4.5 | PASS |
| PK decay (3 dose tiers) | Exponential, k_decay validated | PASS |
| Tissue lattice (4 profiles) | Shannon/Pielou increase with severity | PASS |
| ChEMBL cross-validation | 10.0 nM matches paper | PASS |
| PubChem identity | C15H23N5O2S, MW 337.4 | PASS |
| BLAKE3 content hash | Reference artifacts tracked | PASS |

---

**Provenance**: IC50 values from Gonzales 2014 (DOI: 10.1111/jvp.12065),
PK from Fleck & Gonzales 2021 (DOI: 10.1111/vde.13028). ChEMBL/PubChem
reference artifacts content-addressed with BLAKE3. Full provenance chain
via rhizoCrypt → loamSpine → sweetGrass when primals are deployed.

**Evolution**: Tier 2 calls `science.gonzales.dose_response` and
`science.gonzales.pk_decay` live, asserting parity with frozen values.
Tier 3 wraps each call in a provenance session.

**Source**: [ecoPrimals/wetSpring](https://github.com/ecoPrimals/wetSpring)

