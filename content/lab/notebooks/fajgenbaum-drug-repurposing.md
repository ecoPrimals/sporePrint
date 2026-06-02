+++
title = "Drug Repurposing & Knowledge Graphs — Fajgenbaum Lab (Track 3)"
description = "Rendered from fajgenbaum-drug-repurposing.ipynb"
date = 2026-06-02
weight = 50

[extra]
domain = "Lab"
rendered_from = "fajgenbaum-drug-repurposing.ipynb"
+++

<!-- Auto-generated from fajgenbaum-drug-repurposing.ipynb by spore-validate render-notebooks -->

# Drug Repurposing & Knowledge Graphs — Fajgenbaum Lab (Track 3)

This notebook walks **Track 3** from NMF-based drug repositioning (repoDB-scale
methodology, synthetic dimensions here) through pathway-specific scoring
(iMCD / sirolimus, JCI 2019) to biomedical knowledge-graph embeddings
(ROBOKOP-style TransE). Each section ties visible NumPy computations to the
in-repo Python control scripts and optional Rust validation binaries.

| # | Paper | Script | Model |
|---|-------|--------|-------|
| 1 | Gao et al. 2020 / Yang et al. 2020 | `nmf_drug_disease_pipeline.py` | NMF drug-disease matrix factorization |
| 2 | Fajgenbaum et al. JCI 2019 | `fajgenbaum_pathway_scoring.py` (Exp157) | Pathway activation scoring (PI3K/AKT/mTOR) |
| 3 | ROBOKOP (Paper 43) | `transe_knowledge_graph.py` (Exp161) | TransE knowledge graph embedding |

**Narrative:** Start from nonnegative factorization of a sparse drug–disease matrix
(repoDB-motivated pipelines), narrow to pharmacophenomic **pathway activation**
and drug–pathway alignment (mTOR ↔ sirolimus), then broaden to **TransE**
embeddings over a synthetic biomedical KG (drugs, diseases, genes, pathways).

**Rust binaries (wetSpring checkout):** `validate_nmf_drug_repurposing`,
`validate_fajgenbaum_pathway`, `validate_knowledge_graph_embedding`
(e.g. `wetspring validate --scenario nmf_drug_repurposing`).

---
*Publishing note:* Figures and metrics here are reproducible Tier-1 numerics;
link frozen JSON under `experiments/results/` when exporting baselines from the validators.


```python
import os, json, struct, socket
from pathlib import Path
import numpy as np

RESULTS = Path('..') / '..' / 'experiments' / 'results'

def load(rel_path):
    with open(RESULTS / rel_path) as f:
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

import matplotlib
import matplotlib.pyplot as plt

PASS_COLOR = '#2ecc71'
FAIL_COLOR = '#e74c3c'
INFO_COLOR = '#3498db'
```

## NMF drug–disease matrix factorization (Lee & Seung)

**Nonnegative matrix factorization** approximates a nonnegative association matrix
\(V \in \mathbb{R}_+^{n \times m}\) as \(V \approx W H\) with
\(W \in \mathbb{R}_+^{n \times k}\), \(H \in \mathbb{R}_+^{k \times m}\), using
multiplicative updates (Gao/Yang-style repoDB pipelines use the same family of
models at full scale).

For this notebook we use a **synthetic** \(50 \times 30\) matrix with clear
block structure (**10 drug clusters** × **3 disease groups**) and **`k = 8`**
latent factors—small enough to run interactively while mirroring sparse
repoDB-shaped problems.


```python
def nmf_multiplicative(V, k, n_iter=200, seed=42):
    rng = np.random.RandomState(seed)
    n, m = V.shape
    W = rng.uniform(0.01, 1, (n, k))
    H = rng.uniform(0.01, 1, (k, m))
    eps = 1e-10
    for _ in range(n_iter):
        H *= (W.T @ V) / (W.T @ W @ H + eps)
        W *= (V @ H.T) / (W @ H @ H.T + eps)
    return W, H


def nmf_with_error_curve(V, k, n_iter=200, seed=42):
    rng = np.random.RandomState(seed)
    n, m = V.shape
    W = rng.uniform(0.01, 1, (n, k))
    H = rng.uniform(0.01, 1, (k, m))
    eps = 1e-10
    errors = []
    for _ in range(n_iter):
        H *= (W.T @ V) / (W.T @ W @ H + eps)
        W *= (V @ H.T) / (W @ H @ H.T + eps)
        V_hat = W @ H
        errors.append(np.linalg.norm(V - V_hat, 'fro'))
    return W, H, np.array(errors), V_hat


def build_block_V(n_drugs=50, n_dis=30, n_drug_clusters=10, n_dis_groups=3, seed=0):
    assert n_drugs % n_drug_clusters == 0 and n_dis % n_dis_groups == 0
    rng = np.random.RandomState(seed)
    V = np.zeros((n_drugs, n_dis), dtype=float)
    dz, sz = n_drugs // n_drug_clusters, n_dis // n_dis_groups
    for c in range(n_drug_clusters):
        g = c % n_dis_groups
        sub = rng.uniform(0.55, 1.0, (dz, sz))
        V[c * dz : (c + 1) * dz, g * sz : (g + 1) * sz] = sub
    V += 0.04 * rng.random(V.shape)
    return V


RNG = np.random.RandomState(1)
V = build_block_V()
k = 8
W, H, err_curve, V_hat_train = nmf_with_error_curve(V, k=k, n_iter=200)
W2, H2 = nmf_multiplicative(V, k=k, n_iter=200)
assert np.allclose(W @ H, W2 @ H2, rtol=2e-2, atol=2e-2)
V_hat = W @ H
frob = np.linalg.norm(V - V_hat, 'fro')
rel = frob / (np.linalg.norm(V, 'fro') + 1e-12)
print(f'Frobenius reconstruction error ||V-WH||_F = {frob:.4f} (relative {rel:.3f})')

# Top predicted associations where V_ij is near zero but reconstruction is high
mask = V < 0.15
scores = np.where(mask, V_hat, -np.inf)
flat = scores.ravel()
top_idx = np.argsort(-flat)[:8]
pairs = [(int(i // V.shape[1]), int(i % V.shape[1])) for i in top_idx]
print('Top sparse-cell predictions (drug_idx, disease_idx, V_hat):')
for di, dj in pairs:
    print(f'  ({di:2d}, {dj:2d})  V={V[di,dj]:.3f}  V_hat={V_hat[di,dj]:.3f}')

fig, axes = plt.subplots(1, 2, figsize=(10, 3.8))
for ax, M, title in zip(axes, [V, V_hat], ['Original V', 'Reconstructed V_hat']):
    im = ax.imshow(M, aspect='auto', cmap='magma', vmin=0, vmax=max(V.max(), V_hat.max()))
    ax.set_title(title)
    ax.set_xlabel('disease index')
    ax.set_ylabel('drug index')
    plt.colorbar(im, ax=ax, fraction=0.046)
plt.tight_layout()
plt.show()

plt.figure(figsize=(7, 3.5))
plt.plot(err_curve, color=INFO_COLOR)
plt.xlabel('iteration')
plt.ylabel(r'$\|V - \hat V\|_F$')
plt.title('NMF multiplicative updates — reconstruction error curve')
plt.grid(alpha=0.3)
plt.tight_layout()
plt.show()
```

## Fajgenbaum pathway scoring (JCI 2019)

Published **pathway activation scores** (proteomics) drive a simple
**drug–pathway match**: each drug is mapped to its primary target pathway; the
top pathway is **PI3K/AKT/mTOR**, consistent with **sirolimus** (mTOR inhibitor)
as the mechanistic match in IL-6-blockade-refractory iMCD.

**Selectivity** is illustrated with **cosine similarity** between each drug’s
pathway indicator (weighted by activation on that pathway) and the global
activation profile over pathways.


```python
PATHWAYS = {
    'PI3K/AKT/mTOR': 0.92, 'JAK/STAT3': 0.85, 'NF-κB': 0.78,
    'MAPK/ERK': 0.65, 'VEGF': 0.72, 'IL-6/gp130': 0.88,
}
DRUGS = {
    'sirolimus': 'PI3K/AKT/mTOR', 'everolimus': 'PI3K/AKT/mTOR',
    'tocilizumab': 'IL-6/gp130', 'siltuximab': 'IL-6/gp130',
    'ruxolitinib': 'JAK/STAT3', 'bevacizumab': 'VEGF',
}

path_keys = list(PATHWAYS.keys())
p_index = {p: i for i, p in enumerate(path_keys)}
P = np.array([PATHWAYS[p] for p in path_keys])

top_pathway = max(PATHWAYS, key=PATHWAYS.get)
top_score = PATHWAYS[top_pathway]
print(f'Top pathway: {top_pathway} ({top_score:.2f})')

# Drug vs pathway numeric matrix (match when drug targets pathway)
names = list(DRUGS.keys())
M = np.zeros((len(names), len(path_keys)))
for i, d in enumerate(names):
    targ = DRUGS[d]
    j = p_index[targ]
    M[i, j] = PATHWAYS[targ]

activation_vec = P.copy()


def cosine(a, b):
    return float(np.dot(a, b) / (np.linalg.norm(a) * np.linalg.norm(b) + 1e-12))


print('Drug selectivity vs full activation spectrum:')
spec_scores = []
for d in names:
    v = np.zeros_like(P)
    v[p_index[DRUGS[d]]] = PATHWAYS[DRUGS[d]]
    s = cosine(v, activation_vec)
    spec_scores.append((d, s))
for d, s in sorted(spec_scores, key=lambda x: -x[1]):
    print(f'  {d:14s}  cos(drug profile, activation) = {s:.3f}')

fig, axes = plt.subplots(1, 2, figsize=(10, 3.8))
axes[0].barh(path_keys[::-1], [PATHWAYS[p] for p in path_keys[::-1]], color=PASS_COLOR)
axes[0].set_xlabel('activation score')
axes[0].set_title('Pathway activation (JCI 2019 panel)')
axes[1].imshow(M, aspect='auto', cmap='Blues', vmin=0, vmax=1)
axes[1].set_xticks(range(len(path_keys)))
axes[1].set_xticklabels(path_keys, rotation=45, ha='right')
axes[1].set_yticks(range(len(names)))
axes[1].set_yticklabels(names)
axes[1].set_title('Drug–pathway match (target-pathway weight)')
plt.tight_layout()
plt.show()
```

## TransE knowledge-graph embedding

**TransE:** \( \mathbf{h} + \mathbf{r} \approx \mathbf{t} \).
Energy \(\|\mathbf{h} + \mathbf{r} - \mathbf{t}\|\); conventionally **score**
\(= -\|\mathbf{h} + \mathbf{r} - \mathbf{t}\|\)\/ margin ranking on corrupted triples.

We train on a **synthetic biomedical KG**: cluster-structured triples for
`treats`, `targets`, `associated_with`, and `part_of`, then report **mean rank**
(lower is better, filtered) and **Hits@10** on a held-out triple set.


```python
EMBED_DIM = 16
N_ENTITIES = 50
N_RELATIONS = 4
N_EPOCHS = 50
MARGIN = 1.0
LR = 0.05
SEED_TR = 7

rng_tr = np.random.RandomState(SEED_TR)


def l2_normalize_rows(E):
    n = np.linalg.norm(E, axis=1, keepdims=True) + 1e-12
    return E / n


def transe_energy(E, R, h, r, t):
    x = E[h] + R[r] - E[t]
    return np.linalg.norm(x)


def generate_cluster_triples(n_ent=50, n_rel=4, seed=0):
    rnd = np.random.RandomState(seed)
    n_clusters = 5
    per = n_ent // n_clusters  # 10 entities per cluster
    triples = []
    for c in range(n_clusters):
        base = c * per
        # types within slice: drugs [0..2], diseases [3..5], genes [6..8], pathways [9]
        offsets = {'dr': 0, 'di': 3, 'g': 6, 'p': 9}
        for i in range(3):
            d = base + offsets['dr'] + i
            s = base + offsets['di'] + i % 3
            g = base + offsets['g'] + i % 3
            ptw = base + offsets['p']
            triples.append((d, 0, s))   # treats
            triples.append((d, 1, g))   # targets
            triples.append((g, 2, s))   # gene associated_with disease
            triples.append((g, 3, ptw))  # gene part_of pathway
        extras = rnd.randint(base, base + per, size=(8, 3))
        for h, rr, tail in extras:
            triples.append((int(h % n_ent), int(rr % n_rel), int(tail % n_ent)))
    seen = set()
    uniq = []
    for h, r_, t in triples:
        if (h, r_, t) in seen:
            continue
        if max(h, t) >= n_ent or h < 0 or t < 0:
            continue
        seen.add((h, r_, t))
        uniq.append((h, r_, t))
    return uniq


all_triples = generate_cluster_triples(N_ENTITIES, N_RELATIONS)
rng_tr.shuffle(all_triples)
n_test = max(1, len(all_triples) // 5)
test_set = set(all_triples[:n_test])
train = [t for t in all_triples if t not in test_set]
print(f'Triples: {len(train)} train, {len(test_set)} test')


def entity_type(e):
    slot = e % 10
    if slot <= 2:
        return 'drug'
    if slot <= 5:
        return 'disease'
    if slot <= 8:
        return 'gene'
    return 'pathway'


E = rng_tr.normal(0, 0.1, (N_ENTITIES, EMBED_DIM))
R = rng_tr.normal(0, 0.1, (N_RELATIONS, EMBED_DIM))
E = l2_normalize_rows(E)

train_idx = np.arange(N_ENTITIES)


def corrupt_tail(h, r_, t):
    cand = int(rng_tr.choice(train_idx))
    if cand == t:
        cand = (cand + 1) % N_ENTITIES
    return h, r_, cand


for epoch in range(N_EPOCHS):
    rng_tr.shuffle(train)
    loss_acc = 0.0
    for h, r_, t in train:
        _, _, tn = corrupt_tail(h, r_, t)
        pos = transe_energy(E, R, h, r_, t)
        neg = transe_energy(E, R, h, r_, tn)
        diff = pos - neg
        if diff + MARGIN > 0:
            grad_coef = LR
            # subgradients for pos triple
            x = E[h] + R[r_] - E[t]
            nx = np.linalg.norm(x) + 1e-12
            gx = x / nx
            E[h] -= grad_coef * gx
            R[r_] -= grad_coef * gx
            E[t] += grad_coef * gx
            # neg triple (same relation, corrupted tail)
            xn = E[h] + R[r_] - E[tn]
            nn = np.linalg.norm(xn) + 1e-12
            gxn = xn / nn
            E[h] += grad_coef * gxn
            R[r_] += grad_coef * gxn
            E[tn] -= grad_coef * gxn
            loss_acc += diff + MARGIN
    E = l2_normalize_rows(E)
    if epoch == 0 or epoch == N_EPOCHS - 1:
        print(f'epoch {epoch+1:3d}  hinge ~ {loss_acc / max(1,len(train)):.4f}')

# Filtered evaluation: rank true tail among all entities


def filtered_mean_rank_hits(test_triples, train_set):
    ranks = []
    hits = 0
    for h, r_, t in test_triples:
        energies = np.array([transe_energy(E, R, h, r_, j) for j in range(N_ENTITIES)])
        order = np.argsort(energies)
        rank = 1
        for j in order:
            if j == t:
                break
            if (h, r_, j) in train_set:
                continue
            rank += 1
        ranks.append(rank)
        if rank <= 10:
            hits += 1
    return float(np.mean(ranks)), hits / len(test_triples)


train_set = set(train)
mr, h10 = filtered_mean_rank_hits(list(test_set), train_set)
print(f'Mean rank (filtered, tail prediction): {mr:.2f}')
print(f'Hits@10: {h10:.2%}')

# 2D PCA coloring
X = E - E.mean(axis=0)
_, _, Vt = np.linalg.svd(X, full_matrices=False)
XY = X @ Vt.T[:, :2]
colors = {'drug': '#e74c3c', 'disease': '#3498db', 'gene': '#2ecc71', 'pathway': '#9b59b6'}
plt.figure(figsize=(6.5, 5))
for lab, col in colors.items():
    idx = [i for i in range(N_ENTITIES) if entity_type(i) == lab]
    plt.scatter(XY[idx, 0], XY[idx, 1], c=col, s=36, label=lab, alpha=0.85, edgecolors='k', linewidths=0.3)
plt.legend()
plt.title('Entity embeddings (2D PCA) — synthetic biomedical KG')
plt.xlabel('PC1')
plt.ylabel('PC2')
plt.tight_layout()
plt.show()
```

## Rust parity — frozen baselines (`experiments/results`)

Track 3 validator outputs are expected under experiment-numbered directories
alongside other wetSpring baselines. This cell loads any present JSON without
failing the notebook when snapshots have not yet been exported from CI.


```python
TRACK3_BASELINES = [
    ('Exp157 pathway / validate_fajgenbaum_pathway', Path('157_fajgenbaum_pathway') / 'python_baseline.json'),
    ('Exp159 NMF / validate_nmf_drug_repurposing', Path('159_nmf_drug_repurposing') / 'python_baseline.json'),
    ('Exp161 KG / validate_knowledge_graph_embedding', Path('161_knowledge_graph_embedding') / 'python_baseline.json'),
]

rows = []
for label, rel in TRACK3_BASELINES:
    p = RESULTS / rel
    if p.is_file():
        try:
            data = load(rel)
            status = data.get('status', data.get('validation', 'OK'))
            keys = sorted(data.keys())
            snippet = ', '.join(keys[:6]) + ('…' if len(keys) > 6 else '')
            print(f'[{label}] loaded {p.name}: status={status!r}; keys=[{snippet}]')
            rows.append((label, str(p.relative_to(RESULTS)), 'loaded', status))
        except Exception as e:
            print(f'[{label}] read error: {e}')
            rows.append((label, str(p), 'error', str(e)))
    else:
        print(f'[{label}] missing file: {p}')
        rows.append((label, str(p), 'missing', '—'))

catalog = RESULTS / 'experiment_catalog.json'
if catalog.is_file():
    ec = load(Path('experiment_catalog.json'))
    t3 = ec.get('tracks', {}).get('track3_bioinformatics', {})
    print('Catalog track3 summary:', t3.get('description', '')[:120], '…')
else:
    print('No experiment_catalog.json — skip catalog cross-check.')
```

## Tier 2 parity — guarded `science.nmf_predict`

When `WETSPRING_IPC_SOCKET` is live, we probe the IPC surface for NMF-style
inference. The method name and payload mirror the drug-repurposing track
contract; Tier 1 runs skip this cell body.


```python
if TIER == 'live_ipc':
    try:
        ipc_nmf = ipc_call('science.nmf_predict', {
            'matrix_shape': [50, 30],
            'rank': 8,
            'seed': 42,
            'max_iter': 50,
        })
        print('science.nmf_predict:', ipc_nmf)
    except Exception as ex:
        print('science.nmf_predict failed:', ex)
else:
    print("Tier 1 frozen tier — ipc_call('science.nmf_predict', ...) skipped.")
```

## Summary — validation tiers and provenance

| Stage | Artifact | Role |
|-------|----------|------|
| **Tier 1** | This notebook + JSON under `experiments/results/*/python_baseline.json` | Frozen, publishable Python numerics |
| **Rust validators** | `validate_nmf_drug_repurposing`, `validate_fajgenbaum_pathway`, `validate_knowledge_graph_embedding` | Structural parity / ranking checks (`barracuda` binaries) |
| **Tier 2** | `ipc_call('science.nmf_predict', …)` guarded on `health.check` | Live nucleus / IPC contract (optional) |
| **Tier 3** | Provenance sessions (`provenance.*` RPCs) | Full composition with hashes (not exercised here) |

**Provenance:** Scripts `scripts/nmf_drug_disease_pipeline.py`, `scripts/fajgenbaum_pathway_scoring.py`, `scripts/transe_knowledge_graph.py` are the Python controls referenced in the title table; this notebook condenses their math for teaching.

**Evolution:** **Tier 1** — standalone NumPy + catalog JSON + optional baseline files. **Tier 2** — IPC `science.nmf_predict` when the socket is healthy. **Tier 3** — wrap production runs in provenance + repoDB / ROBOKOP ingest as in the white-paper Track 3 completion notes.


