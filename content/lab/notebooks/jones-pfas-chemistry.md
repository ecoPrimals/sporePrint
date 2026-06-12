+++
title = "PFAS Detection & Environmental Chemistry — Jones Lab (MSU BMB)"
description = "Rendered from jones-pfas-chemistry.ipynb"
date = 2026-06-12
weight = 50

[extra]
domain = "Lab"
rendered_from = "jones-pfas-chemistry.ipynb"
+++

<!-- Auto-generated from jones-pfas-chemistry.ipynb by spore-validate render-notebooks -->

# PFAS Detection & Environmental Chemistry — Jones Lab (MSU BMB)

**Track 2**, **six scripts.** PFOS and PFOA are *forever chemicals* — persistent per- and polyfluoroalkyl substances that accumulate in water and organisms. WetSpring detects them via **liquid chromatography–mass spectrometry (LC‑MS)** feature patterns and **machine-learning classifiers**, turning raw spectral and tabular readings into reproducible PFAS-positive vs negative decisions aligned with EPA-style regulatory thresholds.

**EPA interim health advisory**: **4.0 ppt combined PFOA + PFOS** (used here as an illustrative cutoff for labeling *exceedances*).

**Narrative**: PFAS propagate through groundwater and biosphere; tandem **mass spectra + cosine similarity**, **tabular ML**, and **chromatogram peak picking** jointly triage suspects before expert review.

| # | Script | Experiment | Model |
|---|--------|-----------|-------|
| 1 | `pfas_tree_export.py` | Exp008 | sklearn Decision Tree export |
| 2 | `exp008_pfas_ml_baseline.py` | Exp008 / Exp041 | RF + GBM PFAS classification |
| 3 | `epa_pfas_ml_baseline.py` | Exp041 | EPA PFAS ML validation |
| 4 | `massbank_spectral_baseline.py` | Exp042 | MassBank spectral cosine similarity |
| 5 | `spectral_match_baseline.py` | Exp124 | NPU spectral triage |
| 6 | `generate_peak_baselines.py` | Exp010 | scipy peak detection |

---

*For other springs: swap in your instrument export format and threshold policy; keep the pattern — **freeze JSON baselines**, run **Rust validators**, and escalate from **frozen Tier 1** through **IPC Tier 2** to **hardware Tier 3** when GPUs or remote services are wired.*

```python
import json
import math
import os
import struct
import socket
from pathlib import Path

import matplotlib.pyplot as plt
import numpy as np
from scipy import signal

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
    print(f'Tier 1 — frozen data (no IPC socket)')
```

## a) PFAS ML classification (decision stump)

A single-level **decision stump** minimizes **Gini impurity** at each split candidate. Synthetic **200 × 6** feature matrix mimics spreadsheet-style monitoring fields: **`total_pfas`**, **`pfas_count`**, **`max_single`**, **`pfca_ratio`**, **`latitude`**, **`longitude`**. Binary label **1** iff **`total_pfas` > 4.0 ppt** (EPA-style exceedance).

```python
def gini_impurity(labels):
    n = len(labels)
    if n == 0:
        return 0.0
    counts = {}
    for l in labels:
        counts[l] = counts.get(l, 0) + 1
    return 1.0 - sum((c / n) ** 2 for c in counts.values())


def best_split(features, labels, feat_idx):
    n = len(labels)
    vals = sorted(zip([f[feat_idx] for f in features], labels))
    best_gini, best_thresh = float('inf'), vals[0][0]
    for k in range(1, n):
        if vals[k][0] == vals[k - 1][0]:
            continue
        left = [v[1] for v in vals[:k]]
        right = [v[1] for v in vals[k:]]
        g = (len(left) / n) * gini_impurity(left) + (len(right) / n) * gini_impurity(right)
        if g < best_gini:
            best_gini = g
            best_thresh = (vals[k - 1][0] + vals[k][0]) / 2
    return best_thresh, best_gini


def majority(labels):
    return 1 if sum(labels) > len(labels) / 2 else 0


def train_stump(features, labels):
    base = gini_impurity(labels)
    best = (float('inf'), 0, labels[0] and 0.0)
    n_feat = len(features[0])
    ginis = []
    for j in range(n_feat):
        thresh, g = best_split(features, labels, j)
        ginis.append((base - g, j, thresh, g))
        if g < best[0]:
            best = (g, j, thresh)
    _, best_j, best_t = best
    left_y = [y for f, y in zip(features, labels) if f[best_j] <= best_t]
    right_y = [y for f, y in zip(features, labels) if f[best_j] > best_t]
    left_pred = majority(left_y) if left_y else 0
    right_pred = majority(right_y) if right_y else 0

    def predict_row(f):
        return right_pred if f[best_j] > best_t else left_pred

    acc = sum(int(predict_row(f) == y) for f, y in zip(features, labels)) / len(labels)
    return {
        'feat_idx': best_j,
        'threshold': best_t,
        'left_pred': left_pred,
        'right_pred': right_pred,
        'accuracy': acc,
        'per_feature_impurity_reduction': [
            {'feature': FEAT_NAMES[j], 'reduction': base - ginis[j][3]} for j in range(n_feat)
        ],
    }


FEAT_NAMES = [
    'total_pfas',
    'pfas_count',
    'max_single',
    'pfca_ratio',
    'latitude',
    'longitude',
]

rng = np.random.default_rng(7)
n_samp = 200
X = []
Y = []
for _ in range(n_samp):
    total = float(rng.uniform(0.05, 22.0))
    pfas_count = int(rng.integers(1, 14))
    max_single = float(rng.uniform(0.01, total + rng.uniform(-0.5, 2.0)))
    max_single = max(0.01, min(max_single, total * 1.05))
    pfca_ratio = float(rng.uniform(0.15, 4.8))
    lat = float(rng.uniform(41.69, 48.31))
    lon = float(rng.uniform(-90.42, -82.42))
    X.append([total, pfas_count, max_single, pfca_ratio, lat, lon])
    Y.append(int(total > 4.0))

stump = train_stump(X, Y)
jstar = stump['feat_idx']
print(
    f"Best stump: feature={FEAT_NAMES[jstar]} (index {jstar}), "
    f"threshold={stump['threshold']:.6f}, training accuracy={stump['accuracy']:.4f}"
)
print(f"  leaf predictions: left (≤ thresh)={stump['left_pred']}, right (> thresh)={stump['right_pred']}")

imp = sorted(stump['per_feature_impurity_reduction'], key=lambda d: -d['reduction'])
fig, axes = plt.subplots(1, 2, figsize=(12, 4))
names = [d['feature'] for d in imp]
vals = [d['reduction'] for d in imp]
colors = ['#c0392b' if d['feature'] == FEAT_NAMES[jstar] else '#2980b9' for d in imp]
axes[0].barh(names[::-1], vals[::-1], color=colors[::-1])
axes[0].set_xlabel('Impurity reduction (parent Gini − weighted child Gini)')
axes[0].set_title('Feature importance (stump search)')

x0 = np.array([f[jstar] for f in X])
axes[1].scatter(
    x0,
    [f[2] for f in X],
    c=['#e74c3c' if y else '#2ecc71' for y in Y],
    alpha=0.65,
    edgecolors='k',
    linewidths=0.3,
)
axes[1].axvline(stump['threshold'], color='#8e44ad', linestyle='--', linewidth=2, label='stump threshold')
axes[1].set_xlabel(FEAT_NAMES[jstar])
axes[1].set_ylabel('max_single')
axes[1].set_title('Decision boundary (2D projection)')
axes[1].legend()
plt.tight_layout()
plt.show()
```

## b) MassBank-style spectral matching

**Cosine similarity** with **0.5 Da m/z tolerance**: for each peak in spectrum A, pick the nearest unused peak in spectrum B within tolerance and accumulate matched intensities (`scripts/massbank_spectral_baseline.py`). Frozen Exp042 artifacts use caffeine as the unrelated comparator; below we visualize **four** spectra including an **unrelated** toy mixture for intuition.

```python
def cosine_similarity(
    mz_a, int_a, mz_b, int_b, tolerance_da=0.5):
    """Greedy tolerance matching then cosine on matched intensities (Exp042)."""
    matched_a = []
    matched_b = []
    used_b = set()
    for i, mz_ai in enumerate(mz_a):
        best_j = -1
        best_diff = tolerance_da + 1
        for j, mz_bj in enumerate(mz_b):
            if j in used_b:
                continue
            diff = abs(mz_ai - mz_bj)
            if diff < best_diff:
                best_diff = diff
                best_j = j
        if best_j >= 0 and best_diff <= tolerance_da:
            matched_a.append(int_a[i])
            matched_b.append(int_b[best_j])
            used_b.add(best_j)

    if not matched_a:
        return 0.0
    dot = sum(a * b for a, b in zip(matched_a, matched_b))
    norm_a = math.sqrt(sum(a ** 2 for a in matched_a))
    norm_b = math.sqrt(sum(b ** 2 for b in matched_b))
    if norm_a == 0 or norm_b == 0:
        return 0.0
    return dot / (norm_a * norm_b)


# Synthetic spectra from notebook specification
PFOS_MZ = [80, 99, 119, 169, 219, 269, 319, 369, 419, 499]
PFOS_I = [30, 100, 45, 80, 55, 70, 40, 25, 15, 60]
PFOS_SHIFT_MZ = [80.12, 99.02, 119.08, 169.04, 219.0, 268.88, 319.15, 369.0, 419.02, 499.0]
PFOS_SHIFT_I = [29, 98, 46, 79, 54, 71, 39, 24, 16, 59]
PFOA_MZ = [69, 119, 169, 219, 269, 319, 369, 413]
PFOA_I = [100, 35, 65, 50, 45, 30, 20, 55]
UNREL_MZ = [50, 77, 105, 133, 161]
UNREL_I = [60, 100, 40, 20, 10]

specs = [
    ('PFOS', PFOS_MZ, PFOS_I),
    ('PFOS shifted', PFOS_SHIFT_MZ, PFOS_SHIFT_I),
    ('PFOA', PFOA_MZ, PFOA_I),
    ('Unrelated', UNREL_MZ, UNREL_I),
]
tol = 0.5
nsp = len(specs)
mat = np.zeros((nsp, nsp))
for i in range(nsp):
    for j in range(nsp):
        mat[i, j] = cosine_similarity(
            specs[i][1],
            specs[i][2],
            specs[j][1],
            specs[j][2],
            tol,
        )

fig, ax = plt.subplots(figsize=(7, 5.5))
im = ax.imshow(mat, cmap='magma', vmin=0, vmax=1, aspect='auto')
ax.set_xticks(range(nsp))
ax.set_yticks(range(nsp))
labels = [s[0] for s in specs]
ax.set_xticklabels(labels, rotation=25, ha='right')
ax.set_yticklabels(labels)
for i in range(nsp):
    for j in range(nsp):
        ax.text(j, i, f'{mat[i, j]:.3f}', ha='center', va='center', color='w', fontsize=9)
fig.colorbar(im, ax=ax, label='cosine similarity')
ax.set_title(f'Pairwise spectral cosine (±{tol} Da)')
plt.tight_layout()
plt.show()
```

## c) Peak detection (chromatogram)

**Local maxima** with **minimum prominence** — the same family of rules exercised in **Exp010** (`scipy.signal.find_peaks`). Synthetic signal: sum of three Gaussians plus light noise.

```python
def gaussian(x, mu, sigma, amp):
    return amp * np.exp(-0.5 * ((x - mu) / sigma) ** 2)


x = np.linspace(0, 200, 800)
y = (
    gaussian(x, 45, 4.0, 1200)
    + gaussian(x, 102, 5.5, 900)
    + gaussian(x, 168, 6.0, 750)
    + np.random.default_rng(91).normal(0, 25, size=x.shape)
)

prominence = 80.0
peaks, props = signal.find_peaks(y, prominence=prominence, distance=15)

fig, ax = plt.subplots(figsize=(11, 4))
ax.plot(x, y, color='#34495e', lw=1.2, label='synthetic LC trace')
ax.plot(x[peaks], y[peaks], 'rv', markersize=10, label=f'peaks (prom≥{prominence})')
for pk in peaks:
    ax.axvline(x[pk], color='#e67e22', ls=':', alpha=0.5)
ax.set_xlabel('Retention time index')
ax.set_ylabel('Intensity (AU)')
ax.set_title('Peak detection — Gaussian mixture chromatogram')
ax.legend()
ax.grid(alpha=0.25)
plt.tight_layout()
plt.show()
print(f'Detected peaks at indices {peaks.tolist()} (count={len(peaks)})')
```

## Frozen baselines — Rust validator parity bookkeeping

Load immutable **Python baseline JSON** from `experiments/results/` paths used by **`barracuda` validation binaries**. Counts summarize how many **structured checks** each artifact encodes (models, accuracy rows, spectral pairs, peak scenarios).

```python
exp008 = load('008_pfas_ml/exp008_python_baseline.json')
tree_ex = load('008_pfas_ml/decision_tree_exported.json')
epa041 = load('041_epa_pfas_ml/python_baseline.json')
mb042 = load('042_massbank_spectral/python_baseline.json')
sp124 = load('124_npu_spectral_triage/spectral_match_python_baseline.json')
pk010 = load('010_peak_baselines/scipy_baselines.json')

acc = exp008.get('acceptance_criteria', {})
n_model_checks = 0
for mname, mrec in exp008.get('models', {}).items():
    f1_ok = mrec.get('f1_score', 0) >= acc.get('f1_target', 0)
    auc_ok = mrec.get('auc_roc', 0) >= acc.get('auc_target', 0)
    n_model_checks += int(f1_ok) + int(auc_ok)

mat042 = mb042.get('results', {}).get('pairwise_matrix', [])
n_pair_cells = sum(len(row) for row in mat042) if mat042 else 0
n_peak_cases = len(pk010.get('cases', []))
n_peak_idx_checks = sum(len(c.get('indices', [])) for c in pk010.get('cases', []))

print('Frozen baseline check / structure counts (JSON):')
print(
    f"  Exp008 ML ({exp008.get('experiment')}): {n_model_checks}/"
    f"{2 * len(exp008.get('models', {}))} acceptance checks (F1 + AUC per model)"
)
print(
    f"  Exp008 tree export: {tree_ex.get('n_nodes')} nodes frozen, "
    f"{tree_ex.get('n_features')} features"
)
print(
    f"  Exp041 stump: {epa041.get('n_correct', 0)}/"
    f"{epa041.get('n_samples', '?')} labeled samples consistent, accuracy="
    f"{epa041.get('accuracy')}"
)
print(
    f"  Exp042 spectral matrix: {n_pair_cells} cosine cells frozen "
    f"(±{mb042.get('tolerance_da')} Da tolerance)"
)
print(
    f"  Exp124 triage queries: recall={sp124.get('triage', {}).get('recall')}, "
    f"top-1 hits={sp124.get('triage', {}).get('top1_correct')}/"
    f"{sp124.get('n_queries', '?')}"
)
print(
    f"  Exp010 peak fixtures: {n_peak_cases} scenarios, "
    f"{n_peak_idx_checks} expected peak index checks total"
)
```

## Tier 2 — live `science.pfas_classify` IPC

When `WETSPRING_IPC_SOCKET` points at a healthy daemon, repeat the regulatory-style call path used in deployment.

```python
if TIER == 'live_ipc':
    try:
        out = ipc_call(
            'science.pfas_classify',
            {
                'total_pfas_ng_per_L': 6.8,
                'pfos_ng_per_L': 3.9,
                'pfoa_ng_per_L': 2.1,
                'features': [
                    {'name': 'latitude', 'value': 42.7312},
                    {'name': 'longitude', 'value': -84.4805},
                ],
            },
        )
        print('Tier 2 science.pfas_classify:', json.dumps(out, indent=2)[:1200])
    except Exception as e:
        print('science.pfas_classify IPC unavailable:', repr(e))
else:
    print('Skipping live PFAS classify (Tier 1 frozen — set WETSPRING_IPC_SOCKET for Tier 2).')
```

## Summary — validation anchors & evolution

**Validation anchors (tabular)**  

| Anchor | Artifact | Highlights |
|--------|----------|------------|
| Exp008 RF / GBM | `008_pfas_ml/exp008_python_baseline.json` | High AUC/F1 vs acceptance gates; top features PFOS, PFOA, totals |
| Exp008 export | `008_pfas_ml/decision_tree_exported.json` | Full sklearn tree structure for auditor replay |
| Exp041 EPA-style stump | `041_epa_pfas_ml/python_baseline.json` | 5-feature stump + exact sample accuracy |
| Exp042 cosine | `042_massbank_spectral/python_baseline.json` | Tolerance-matched cosine matrix + four reference spectra |
| Exp124 triage | `124_npu_spectral_triage/spectral_match_python_baseline.json` | Library-scale recall / top-1 statistics |
| Exp010 peaks | `010_peak_baselines/scipy_baselines.json` | Frozen `find_peaks` indices across synthetic fixtures |

**Provenance**: JSON under `experiments/results/` is generated by the six Track 2 scripts and consumed by Rust validators in `barracuda` (e.g. `validate_massbank_spectral`). This notebook’s inline math is **pedagogical**; numerics for publication should cite the frozen files verbatim.

**Evolution**  

- **Tier 1 — frozen**: offline replay from JSON (default in CI / air-gapped labs).  
- **Tier 2 — IPC**: `science.pfas_classify` and related handlers when `WETSPRING_IPC_SOCKET` is live.  
- **Tier 3 — accelerated / fleet**: GPU spectral kernels and cross-spring orchestration (see experiment docs for MassBank GPU scale and metalForge hooks).

*Together, MS-based detection and ML trees operationalize “forever chemical” surveillance with regulatory-aware thresholds and audit trails.*

