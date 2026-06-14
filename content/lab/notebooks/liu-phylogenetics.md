+++
title = "Computational Phylogenetics — Liu Lab (MSU CMSE)"
description = "Rendered from liu-phylogenetics.ipynb"
date = 2026-06-14
weight = 50

[extra]
domain = "Lab"
rendered_from = "liu-phylogenetics.ipynb"
+++

<!-- Auto-generated from liu-phylogenetics.ipynb by spore-validate render-notebooks -->

# Computational Phylogenetics — Liu Lab (MSU CMSE)

This notebook walks through the **sequence → alignment → likelihood → tree distance → reconstruction**
pipeline that grounds comparative genomics validation in wetSpring. It emphasizes five **core models** inline below;
the broader Track **1b** surface is **twelve** Python baseline scripts in `scripts/` (each with a matching `validate_*` binary).
Topics span hidden Markov models for
phylogenomic heterogeneity (PhyloNet-HMM / Liu 2014), pairwise **local alignment** with affine gaps
(Smith–Waterman), **Felsenstein pruning** under Jukes–Cantor, **Robinson–Foulds** tree comparison,
and **neighbor-joining** guide trees (SATé / Liu 2009).

| # | Topic | DOI | Model |
|---|-------|-----|-------|
| 1 | Liu et al. 2014 (PhyloNet-HMM framing) | [10.1371/journal.pcbi.1003649](https://doi.org/10.1371/journal.pcbi.1003649) | 3-state emission HMM: forward + Viterbi |
| 2 | Smith & Waterman 1981 | [10.1016/0022-2836(81)90087-5](https://doi.org/10.1016/0022-2836(81)90087-5) | Affine-gap local alignment |
| 3 | Felsenstein 1981 | [10.1007/BF01734359](https://doi.org/10.1007/BF01734359) | Pruning + JC69 transition probabilities |
| 4 | Robinson–Foulds | *Consensus usage in phylogenetics* | Unrooted bipartition distance |
| 5 | Liu et al. 2009 (SATé) | [10.1126/science.1171243](https://doi.org/10.1126/science.1171243) | Neighbor-joining from distances |

**Rust validation (Track 1b, 12 scripts)**:
`validate_newick_parse` (019), `validate_rf_distance` (021), `validate_hmm` (026),
`validate_alignment` (028), `validate_felsenstein` (029), `validate_bootstrap` (031),
`validate_placement` (032), `validate_neighbor_joining` (033), `validate_reconciliation` (034),
`validate_phynetpy_rf` (036), `validate_phylohmm` (037), `validate_sate_pipeline` (038).

---

*For other springs: swap the organismal story for your domain but keep this publishable scaffold —
math spec → pure-Python replicate → matplotlib figures → frozen JSON under `experiments/results/` →
optional Tier-2 IPC.*


```python
import os, json, struct, socket, math
from pathlib import Path
import numpy as np
from scipy.integrate import odeint

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

def hill(x, k, n):
    """Hill activation: x^n / (k^n + x^n)."""
    return x**n / (k**n + x**n) if x > 0 else 0.0

def hill_repress(x, k, n):
    """Hill repression: k^n / (k^n + x^n)."""
    return k**n / (k**n + x**n) if x > 0 else 1.0
```

## 1. Liu 2014 — 3-state HMM (forward + Viterbi)

Hidden states \(q_t \in \{0,1,2\}\) emit symbols \(o_t \in \{0,1,2\}\) through row-stochastic **B**.
Markov transitions use **A**; initial distribution \(\pi\).

**Forward** (dynamic programming):
\(\alpha_t(j) = P(o_1,\ldots,o_t, q_t=j) = b_j(o_t) \sum_i \alpha_{t-1}(i)\, a_{ij}\), with
\(\alpha_1(j) = \pi_j\, b_j(o_1)\).

**Viterbi** (max-product):
\(\delta_t(j) = b_j(o_t) \max_i \delta_{t-1}(i)\, a_{ij}\), backtrace for \(\arg\max\) path.

Below: fixed \(\pi\), **A**, **B**, and the observation trace from this notebook specification; visualize
the forward lattice and decode the MAP path.


```python
pi = np.array([0.6, 0.3, 0.1], dtype=float)
A = np.array([[0.7, 0.2, 0.1], [0.1, 0.6, 0.3], [0.2, 0.3, 0.5]], dtype=float)
B = np.array([[0.5, 0.3, 0.2], [0.1, 0.4, 0.5], [0.3, 0.3, 0.4]], dtype=float)
obs = np.array([0, 1, 0, 2, 1, 0, 1, 2, 0, 1], dtype=int)
n_states = 3
T = len(obs)

alpha = np.zeros((T, n_states))
alpha[0] = pi * B[:, obs[0]]
for t in range(1, T):
    for j in range(n_states):
        alpha[t, j] = B[j, obs[t]] * np.sum(alpha[t - 1] * A[:, j])
log_lik = math.log(alpha[-1].sum())

delta = np.zeros((T, n_states))
psi = np.zeros((T, n_states), dtype=int)
delta[0] = np.log(pi + 1e-300) + np.log(B[:, obs[0]] + 1e-300)
for t in range(1, T):
    for j in range(n_states):
        scores = delta[t - 1] + np.log(A[:, j] + 1e-300)
        psi[t, j] = int(np.argmax(scores))
        delta[t, j] = np.log(B[j, obs[t]] + 1e-300) + scores[psi[t, j]]
path = [0] * T
path[-1] = int(np.argmax(delta[-1]))
viterbi_logp = float(delta[-1, path[-1]])
for t in range(T - 1, 0, -1):
    path[t - 1] = psi[t, path[t]]

fig, axes = plt.subplots(1, 2, figsize=(14, 4))
ax = axes[0]
im = ax.imshow(np.log(alpha.T + 1e-300), aspect='auto', cmap='viridis')
ax.set_xlabel('time t'); ax.set_ylabel('state j')
ax.set_title('log forward lattice log α_t(j)')
plt.colorbar(im, ax=ax)
ax = axes[1]
ax.step(np.arange(T), path, where='mid', color=INFO_COLOR, linewidth=2)
ax.scatter(np.arange(T), path, color=FAIL_COLOR, zorder=3)
ax.set_xlabel('t'); ax.set_ylabel('Viterbi state'); ax.set_yticks([0, 1, 2])
ax.set_title('Viterbi path'); ax.grid(alpha=0.3)
plt.suptitle('Liu 2014 HMM — forward likelihood + Viterbi decoding', fontweight='bold')
plt.tight_layout(); plt.show()

print(f'log P(obs) = {log_lik:.6f}')
print(f'Viterbi path = {path}')
print(f'Viterbi log prob (max joint) = {viterbi_logp:.6f}')
```

## 2. Smith & Waterman 1981 — affine-gap local alignment

Local alignment score with **match** \(=2\), **mismatch** \(=-1\), **gap open** \(=-3\),
**gap extend** \(=-1\) (Gotoh recurrence; scores floored at 0 for locality, as in wetSpring validation).


```python
def smith_waterman(query, target, match=2, mismatch=-1, gap_open=-3, gap_extend=-1):
    m, n = len(query), len(target)
    if m == 0 or n == 0:
        return {"score": 0, "aligned_query": "", "aligned_target": "", "query_start": 0, "target_start": 0}
    H = [[0] * (n + 1) for _ in range(m + 1)]
    E = [[-(10**9)] * (n + 1) for _ in range(m + 1)]
    F = [[-(10**9)] * (n + 1) for _ in range(m + 1)]
    best_score, best_i, best_j = 0, 0, 0
    for i in range(1, m + 1):
        for j in range(1, n + 1):
            s = match if query[i - 1].upper() == target[j - 1].upper() else mismatch
            E[i][j] = max(H[i][j - 1] + gap_open + gap_extend, E[i][j - 1] + gap_extend)
            F[i][j] = max(H[i - 1][j] + gap_open + gap_extend, F[i - 1][j] + gap_extend)
            H[i][j] = max(0, H[i - 1][j - 1] + s, E[i][j], F[i][j])
            if H[i][j] > best_score:
                best_score, best_i, best_j = H[i][j], i, j
    aq, at = [], []
    i, j = best_i, best_j
    while i > 0 and j > 0 and H[i][j] > 0:
        s = match if query[i - 1].upper() == target[j - 1].upper() else mismatch
        if H[i][j] == H[i - 1][j - 1] + s:
            aq.append(query[i - 1]); at.append(target[j - 1]); i -= 1; j -= 1
        elif H[i][j] == F[i][j]:
            aq.append(query[i - 1]); at.append('-'); i -= 1
        else:
            aq.append('-'); at.append(target[j - 1]); j -= 1
    return {
        "score": best_score,
        "aligned_query": ''.join(reversed(aq)),
        "aligned_target": ''.join(reversed(at)),
        "query_start": i,
        "target_start": j,
    }

cases = {
    "identical": ("ACGTACGT", "ACGTACGT"),
    "mismatch": ("ACGT", "ACTT"),
    "gap": ("ACGTACGT", "ACGACGT"),
    "local": ("XXXACGTACGTXXX", "ACGTACGT"),
    "16s_fragment": (
        "GATCCTGGCTCAGGATGAACGCTGGCGGCGTGCCTAATAC",
        "GATCCTGGCTCAGAATGAACGCTGGCGGCATGCCTAATAC",
    ),
}
scores = {k: smith_waterman(*v)["score"] for k, v in cases.items()}
fig, ax = plt.subplots(figsize=(11, 4))
labs = list(scores.keys())
vals = [scores[k] for k in labs]
cols = [INFO_COLOR if i % 2 == 0 else PASS_COLOR for i in range(len(labs))]
bars = ax.bar(labs, vals, color=cols)
ax.set_ylabel('SW score'); ax.set_title('Smith–Waterman — scenario scores')
for b, v in zip(bars, vals):
    ax.text(b.get_x() + b.get_width() / 2, v + 0.5, str(v), ha='center', fontsize=9)
ax.grid(alpha=0.3, axis='y')
plt.tight_layout(); plt.show()
for k in labs:
    r = smith_waterman(*cases[k])
    print(f'{k:12s} score={r["score"]:3d} q@{r["query_start"]} t@{r["target_start"]}')
```

## 3. Felsenstein 1981 — pruning under Jukes–Cantor

For rate \(\mu\) and edge length \(t\), JC69 gives
\(P_{ii}(t)=\tfrac{1}{4}+\tfrac{3}{4}\exp(-4\mu t/3)\) and
\(P_{ij}(t)=\tfrac{1}{4}(1-\exp(-4\mu t/3))\) for \(i\neq j\).

**Likelihood**: post-order **pruning** with partials at each node; multiply root partials by \(\pi_k=\tfrac14\).

Tree (Newick):
`((A:0.1, C:0.1):0.2, (G:0.1, T:0.1):0.2):0` with leaf strings **ACGT**, **ACTT**, **GCGT**, **GCAT**.


```python
N_STATES = 4

def jc69_prob(fr, to, branch_len, mu=1.0):
    e = math.exp(-4 * mu * branch_len / 3)
    return (0.25 + 0.75 * e) if fr == to else 0.25 * (1 - e)

def transition_matrix(branch_len, mu=1.0):
    return [[jc69_prob(i, j, branch_len, mu) for j in range(N_STATES)] for i in range(N_STATES)]

def encode_dna(seq):
    m = {'A': 0, 'C': 1, 'G': 2, 'T': 3}
    return [m[c.upper()] for c in seq]

def leaf_partial(state):
    p = [0.0] * N_STATES
    p[state] = 1.0
    return p

def pruning(tree, mu=1.0):
    if tree["type"] == "leaf":
        return [[*leaf_partial(s)] for s in tree["states"]]
    left_p = pruning(tree["left"], mu)
    right_p = pruning(tree["right"], mu)
    trans_l = transition_matrix(tree["left_branch"], mu)
    trans_r = transition_matrix(tree["right_branch"], mu)
    partials = []
    for lp, rp in zip(left_p, right_p):
        result = [0.0] * N_STATES
        for s in range(N_STATES):
            ls = sum(trans_l[s][x] * lp[x] for x in range(N_STATES))
            rs = sum(trans_r[s][x] * rp[x] for x in range(N_STATES))
            result[s] = ls * rs
        partials.append(result)
    return partials

def site_log_likelihoods(tree, mu=1.0):
    partials = pruning(tree, mu)
    pi = 0.25
    return [math.log(pi * sum(p[s] for s in range(N_STATES))) for p in partials]

fel_tree = {
    "type": "internal",
    "left": {
        "type": "internal",
        "left": {"type": "leaf", "name": "A", "states": encode_dna("ACGT")},
        "right": {"type": "leaf", "name": "C", "states": encode_dna("ACTT")},
        "left_branch": 0.1, "right_branch": 0.1,
    },
    "right": {
        "type": "internal",
        "left": {"type": "leaf", "name": "G", "states": encode_dna("GCGT")},
        "right": {"type": "leaf", "name": "T", "states": encode_dna("GCAT")},
        "left_branch": 0.1, "right_branch": 0.1,
    },
    "left_branch": 0.2, "right_branch": 0.2,
}

sll = site_log_likelihoods(fel_tree, 1.0)
ll = sum(sll)

fig, ax = plt.subplots(figsize=(10, 4))
ax.bar(np.arange(1, len(sll) + 1), sll, color=INFO_COLOR, edgecolor='white')
ax.set_xlabel('site'); ax.set_ylabel('log site likelihood')
ax.set_title('Felsenstein pruning — per-site log-likelihoods (JC69, μ=1)')
ax.grid(alpha=0.3, axis='y')
plt.tight_layout(); plt.show()
print(f'total log L = {ll:.6f}')
print('site log L:', [round(x, 4) for x in sll])
```

## 4. Robinson–Foulds distance

Summarize each unrooted **fully resolved** tree by its **non-trivial** bipartitions — for four taxa
only the central split matters (alternatives **AB|CD**, **AC|BD**, **AD|BC**). The Robinson–Foulds metric
counts edges that must appear in exactly one reconstruction (equivalently, split symmetric difference).

Example (Exp021 `single_nni_4leaf`):

`T1 = ((A:0.1,B:0.2):0.3,(C:0.3,D:0.4):0.5);`

`T2 = ((A:0.1,C:0.3):0.3,(B:0.2,D:0.4):0.5);`

Here **T1** encodes **AB|CD**; **T2** encodes **AC|BD**; \(\mathrm{RF}(T_1,T_2)=2\).


```python
def canon_split(side, labels):
    side = frozenset(side)
    other = frozenset(labels) - side
    if not side or not other:
        return None
    a, b = side, other
    if len(a) > len(b) or (len(a) == len(b) and min(a) > min(b)):
        a, b = b, a
    return frozenset(a), frozenset(b)

labels4 = {'A', 'B', 'C', 'D'}
S1 = {canon_split({'A', 'B'}, labels4)}
S2 = {canon_split({'A', 'C'}, labels4)}
rf = len(S1.symmetric_difference(S2))

newick1 = '((A:0.1,B:0.2):0.3,(C:0.3,D:0.4):0.5);'
newick2 = '((A:0.1,C:0.3):0.3,(B:0.2,D:0.4):0.5);'

fig, axes = plt.subplots(1, 2, figsize=(14, 3.5))
for ax, nw, title in zip(axes, [newick1, newick2], ['T1', 'T2']):
    ax.axis('off')
    ax.set_title(title, fontweight='bold', loc='left')
    ax.text(0.02, 0.55, nw, family='monospace', fontsize=10, va='center')
plt.suptitle('Robinson–Foulds quartet example (Newick)', fontweight='bold')
plt.tight_layout(); plt.show()

s1, s2 = list(S1)[0], list(S2)[0]
print(f'T1 nontrivial split (canonical): {sorted(s1[0])}|{sorted(s1[1])}')
print(f'T2 nontrivial split (canonical): {sorted(s2[0])}|{sorted(s2[1])}')
print(f'RF distance = {rf} (symmetric split difference; Exp021 NNI case)')
```

## 5. Neighbor-Joining — Liu 2009 (SATé primitive)

Neighbor-Joining minimizes the **neighbor criterion** \(Q\) (Saitou & Nei) and iteratively merges taxa —
the guide tree backbone of SATé (Liu 2009) when fed a pairwise divergence matrix \(d_{ij}\).

Below: the **\(5\times5\)** JC distance matrix from `liu2009_neighbor_joining.py` (same synthetic contigs).
We plot the matrix then emit a Newick string from a pure NumPy/Python NJ port.


```python
def neighbor_joining(dist_matrix, labels):
    n_orig = len(dist_matrix)
    D = [row[:] for row in dist_matrix]
    active = list(range(n_orig))
    node_labels = list(labels)
    next_node = n_orig
    while len(active) > 2:
        r = len(active)
        row_sums = {}
        for i in active:
            row_sums[i] = sum(D[i][j] for j in active if j != i)
        best_q = float('inf'); best_i, best_j = active[0], active[1]
        for ai, i in enumerate(active):
            for j in active[ai + 1 :]:
                q_val = (r - 2) * D[i][j] - row_sums[i] - row_sums[j]
                if q_val < best_q:
                    best_q = q_val; best_i, best_j = i, j
        if r > 2:
            delta = (row_sums[best_i] - row_sums[best_j]) / (r - 2)
        else:
            delta = 0.0
        li = max(0.0, 0.5 * (D[best_i][best_j] + delta))
        lj = max(0.0, 0.5 * (D[best_i][best_j] - delta))
        new_label = f'({node_labels[best_i]}:{li:.6f},{node_labels[best_j]}:{lj:.6f})'
        node_labels.append(new_label)
        new_idx = next_node; next_node += 1
        while len(D) <= new_idx:
            D.append([0.0] * len(D[0]))
        for row in D:
            while len(row) <= new_idx:
                row.append(0.0)
        for k in active:
            if k not in (best_i, best_j):
                d_new = 0.5 * (D[best_i][k] + D[best_j][k] - D[best_i][best_j])
                D[new_idx][k] = d_new
                D[k][new_idx] = d_new
        D[new_idx][new_idx] = 0.0
        active.remove(best_i); active.remove(best_j); active.append(new_idx)

    i, j = active
    final_d = D[i][j]
    return f'({node_labels[i]}:{final_d / 2:.6f},{node_labels[j]}:{final_d / 2:.6f});'

def jukes_cantor_distance(seq1, seq2):
    diffs = sum(1 for a, b in zip(seq1, seq2) if a != b)
    p = diffs / len(seq1)
    if p >= 0.75:
        return 10.0
    return -0.75 * math.log(1.0 - 4.0 * p / 3.0)

seqs = {
    'S1': 'ACGTACGTACGT',
    'S2': 'ACGTACGTACTT',
    'S3': 'ACTTACTTACTT',
    'S4': 'TGCATGCATGCA',
    'S5': 'TGCATGCATGCC',
}
labels_5 = list(seqs.keys())
n = len(labels_5)
D5 = [[0.0] * n for _ in range(n)]
for i in range(n):
    for j in range(i + 1, n):
        d = jukes_cantor_distance(seqs[labels_5[i]], seqs[labels_5[j]])
        D5[i][j] = d; D5[j][i] = d

fig, axes = plt.subplots(1, 2, figsize=(13, 4.5))
ax = axes[0]
im = ax.imshow(D5, cmap='viridis')
ax.set_xticks(range(n)); ax.set_yticks(range(n))
ax.set_xticklabels(labels_5); ax.set_yticklabels(labels_5)
ax.set_title('5×5 Jukes–Cantor distance matrix')
plt.colorbar(im, ax=ax)
ax = axes[1]
ax.axis('off')
nw = neighbor_joining(D5, labels_5)
ax.text(0.02, 0.5, 'NJ Newick:\n' + nw, family='monospace', fontsize=9, va='center')
ax.set_title('Neighbor-joining tree')
plt.suptitle('Liu 2009 SATé — NJ from distances', fontweight='bold')
plt.tight_layout(); plt.show()
print(nw)
```

## Rust parity — frozen baselines

Load JSON recorded under `experiments/results/`. Canonical paths in this repository:
`026_hmm/`, `028_alignment/`, `029_felsenstein/`, `021_rf_baseline/` (aliases in the loader below map the
names from the wetSpring experiment index).


```python
def resolve_result(*candidates):
    for c in candidates:
        p = RESULTS / c
        if p.exists():
            return p, c
    raise FileNotFoundError('None found: ' + ', '.join(candidates))

p_hmm, _ = resolve_result(
    '026_hmm/liu2014_hmm_python_baseline.json',
)
p_sw, _ = resolve_result(
    '028_smith_waterman/sw_python_baseline.json',
    '028_alignment/smith_waterman_python_baseline.json',
)
p_fel, _ = resolve_result(
    '029_pruning/felsenstein_python_baseline.json',
    '029_felsenstein/felsenstein_python_baseline.json',
)
p_rf, _ = resolve_result(
    '021_rf_distance/rf_distance_python_baseline.json',
    '021_rf_baseline/rf_python_baseline.json',
)

frozen_hmm = json.load(open(p_hmm))
frozen_sw = json.load(open(p_sw))
frozen_fel = json.load(open(p_fel))
frozen_rf = json.load(open(p_rf))

print('Frozen baseline paths resolved:')
print(f'  HMM         → {p_hmm.relative_to(RESULTS)}')
print(f'  Smith–Waterman → {p_sw.relative_to(RESULTS)}  (requested `028_smith_waterman/…` falls back here)')
print(f'  Felsenstein → {p_fel.relative_to(RESULTS)}')
print(f'  RF distance → {p_rf.relative_to(RESULTS)}')
print()

print('Frozen check / case counts:')
print(f'  liu2014_hmm scenarios: {len(frozen_hmm)} top-level buckets')
genomic = frozen_hmm.get('genomic_3state', {})
if genomic:
    print(f'    genomic_3state: logL={genomic.get("log_likelihood")}, Viterbi len={len(genomic.get("viterbi_path", []))}')

print(f'  smith_waterman cases: {len(frozen_sw)}')
print(f'  felsenstein cases: {len(frozen_fel)}')
print(f'  RF baseline: PASS {frozen_rf.get("total_pass")} / CASES {frozen_rf.get("total_cases")}')
```

## Tier 2 IPC parity — `science.alignment`

When `WETSPRING_IPC_SOCKET` is live, call the Rust Smith–Waterman handler and compare to the frozen
`identical` scenario (Exp028). Parameters use `seq_a` / `seq_b` per the IPC schema.


```python
if TIER == 'live_ipc':
    live = ipc_call('science.alignment', {
        'seq_a': 'ACGTACGT',
        'seq_b': 'ACGTACGT',
        'match_score': 2,
        'mismatch_penalty': -1,
        'gap_open': -3,
        'gap_extend': -1,
    })
    # reload SW path for frozen identical score
    p_sw, _ = resolve_result(
        '028_smith_waterman/sw_python_baseline.json',
        '028_alignment/smith_waterman_python_baseline.json',
    )
    frozen_sw = json.load(open(p_sw))
    exp = frozen_sw['identical']['score']
    assert live['score'] == exp, (live['score'], exp)
    print(f'Tier 2 parity: science.alignment MATCH (score={live["score"]})')
else:
    print('Tier 2 IPC skipped (Tier 1 frozen mode).')
```

## Validation summary

| # | Python script | Exp | Rust binary |
|---|---------------|-----|-------------|
| 1 | `newick_parse_baseline.py` | 019 | `validate_newick_parse` |
| 2 | `rf_distance_baseline.py` | 021 | `validate_rf_distance` |
| 3 | `liu2014_hmm_baseline.py` | 026 | `validate_hmm` |
| 4 | `smith_waterman_baseline.py` | 028 | `validate_alignment` |
| 5 | `felsenstein_pruning_baseline.py` | 029 | `validate_felsenstein` |
| 6 | `wang2021_rawr_bootstrap.py` | 031 | `validate_bootstrap` |
| 7 | `alamin2024_placement.py` | 032 | `validate_placement` |
| 8 | `liu2009_neighbor_joining.py` | 033 | `validate_neighbor_joining` |
| 9 | `zheng2023_dtl_reconciliation.py` | 034 | `validate_reconciliation` |
| 10 | `phynetpy_rf_baseline.py` | 036 | `validate_phynetpy_rf` |
| 11 | `phylohmm_introgression_baseline.py` | 037 | `validate_phylohmm` |
| 12 | `sate_alignment_baseline.py` | 038 | `validate_sate_pipeline` |

**Provenance**: Parameters and seeds follow the published references above; wetSpring stores Python
baselines as JSON under `experiments/results/` with matching Rust harnesses in `barracuda/`.

**Evolution**: **Tier 1** — this notebook + frozen baselines. **Tier 2** — live IPC (`science.alignment`
shown here; other methods map to additional handlers). **Tier 3** — primal composition with provenance
sessions wrapping each experiment slice.


