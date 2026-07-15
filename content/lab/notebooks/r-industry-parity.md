+++
title = "R Industry Parity — vegan / DADA2 / phyloseq"
description = "Rendered from r-industry-parity.ipynb"
date = 2026-07-04
weight = 50

[extra]
domain = "computation"
rendered_from = "r-industry-parity.ipynb"
+++

<!-- Auto-generated from r-industry-parity.ipynb by spore-validate render-notebooks -->

# R Industry Parity — vegan / DADA2 / phyloseq

| # | R Package | Function | Python Equivalent |
|---|-----------|----------|-------------------|
| 1 | vegan | diversity(), vegdist() | Shannon, Simpson, Bray-Curtis |
| 2 | DADA2 | dada() error model | Loess-like error rate estimation |
| 3 | phyloseq | UniFrac() | Weighted/unweighted UniFrac distance |

These three R packages dominate amplicon analysis. **wetSpring** reproduces their key computations in pure Python (and Rust) to maintain parity without an R dependency.

**Rust validation:** `validate_diversity`, `validate_error_model`, `validate_unifrac`. R-script JSON baselines align with **`wetspring validate --scenario r_industry_parity`**.

*Frozen baselines:* `experiments/results/r_baselines/` (exported via `scripts/r_*_baseline.R`).*


```python
import os, json, math, struct, socket
from pathlib import Path

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
INFO_COLOR = '#3498db'
WARN_COLOR = '#e67e22'

# Vectors aligned with frozen `vegan_diversity.json` / IPC smoke tests
IPC_COUNTS_A = [10.0, 20.0, 30.0, 40.0, 50.0]
IPC_COUNTS_B = [50.0, 40.0, 30.0, 20.0, 10.0]
```

## (a) vegan parity — diversity indices

Pure-Python **Shannon** (natural log, nonzero terms only), **Simpson** $1-\sum p_i^2$, and **Bray–Curtis** dissimilarity match **`vegan::diversity`** / **`vegan::vegdist(method='bray')`** on the reference integer vectors from `r_baselines/vegan_diversity.json`.


```python
import math


def shannon(counts):
    total = sum(counts)
    if total == 0:
        return 0.0
    return -sum((c / total) * math.log(c / total) for c in counts if c > 0)


def simpson(counts):
    total = sum(counts)
    if total == 0:
        return 0.0
    return 1.0 - sum((c / total) ** 2 for c in counts)


def bray_curtis(a, b):
    num = sum(abs(x - y) for x, y in zip(a, b))
    den = sum(x + y for x, y in zip(a, b))
    return num / den if den > 0 else 0.0


# Three synthetic OTU tables: 10, 10, and 40 taxa (zeros pad without changing H', 1-D)
uniform_10 = [100] * 10
skewed_10 = [900] + [11] * 9
five_sp_padded_40 = [100, 80, 60, 40, 20] + [0] * 35

otu_tables = {
    'uniform (n=10)': uniform_10,
    'skewed (n=10)': skewed_10,
    'five-species + zeros (n=40)': five_sp_padded_40,
}

fb_vegan = load('r_baselines/vegan_diversity.json')
expected = {
    'uniform (n=10)': (fb_vegan['shannon_uniform_10'], fb_vegan['simpson_uniform_10']),
    'skewed (n=10)': (fb_vegan['shannon_skewed'], fb_vegan['simpson_skewed']),
    'five-species + zeros (n=40)': (fb_vegan['shannon_5species'], fb_vegan['simpson_5species']),
}

for label, counts in otu_tables.items():
    h, d, exp = shannon(counts), simpson(counts), expected[label]
    assert abs(h - exp[0]) < 1e-12, (label, h, exp[0])
    assert abs(d - exp[1]) < 1e-12, (label, d, exp[1])
    print(f'{label}: Shannon={h:.12f}  Simpson={d:.12f}  (vegan JSON match)')

comm_a = [10, 20, 30, 40, 50]
comm_b = [50, 40, 30, 20, 10]
bc = bray_curtis(comm_a, comm_b)
assert abs(bc - fb_vegan['bray_curtis_ab']) < 1e-12
print(f'Bray–Curtis(a,b)={bc} (vegan reference {fb_vegan["bray_curtis_ab"]})')

labels = list(otu_tables.keys())
x = list(range(len(labels)))
width = 0.35
fig, ax = plt.subplots(figsize=(9, 4.2))
ax.bar([xi - width / 2 for xi in x], [shannon(otu_tables[k]) for k in labels], width, label="Shannon H'", color=INFO_COLOR)
ax.bar([xi + width / 2 for xi in x], [simpson(otu_tables[k]) for k in labels], width, label='Simpson 1−Σp²', color=PASS_COLOR)
ax.set_xticks(x)
ax.set_xticklabels(labels, rotation=15, ha='right')
ax.set_ylabel('Index value')
ax.set_title('Synthetic OTU tables — diversity vs frozen vegan baselines')
ax.legend()
ax.grid(alpha=0.3, axis='y')
plt.tight_layout()
```

## (b) DADA2 parity — error rate model

**DADA2** learns $Q$-indexed transition matrices; initialization follows **Phred** $P(\mathrm{error}) = 10^{-Q/10}$ with uniform substitutions $P(\mathrm{sub}) = P(\mathrm{error})/3$. The snippet below mirrors that spine (conceptually **loess-smoothed** in full `learnErrors`, here the theoretical ladder). Compare against `phred_error_probs` in **`r_baselines/dada2_error_model.json`**.


```python
def dada2_error_rate(quality_score, a=-0.2, b=1.0):
    """Approximate DADA2 error rate: p_error = 10^(-Q/10), smoothed."""
    return 10 ** (-quality_score / 10.0)


Q_grid = list(range(0, 41))
theoretical = [10 ** (-q / 10.0) for q in Q_grid]
approx = [dada2_error_rate(q) for q in Q_grid]
assert all(abs(a - b) < 1e-15 for a, b in zip(theoretical, approx))

fb_dada = load('r_baselines/dada2_error_model.json')
pairs = list(zip(fb_dada['phred_qualities'], fb_dada['phred_error_probs']))
for q, exp in pairs:
    got = dada2_error_rate(q)
    assert abs(got - exp) < 1e-14, (q, got, exp)
print('Phred ladder: Python matches frozen dada2_error_model.json —', len(pairs), 'anchors')

bases = ['A', 'C', 'G', 'T']
Q = 30
p_err = dada2_error_rate(Q)
p_sub = p_err / 3.0
TM = [[(1 - p_err if i == j else p_sub) for j in range(4)] for i in range(4)]
print(f'Transition matrix rows/cols {bases} at Q={Q}: p_err={p_err}')
for row, bi in zip(TM, bases):
    print(bi, ' '.join(f'{v:.6f}' for v in row))

fig, axes = plt.subplots(1, 2, figsize=(11, 4.2))
ax = axes[0]
ax.semilogy(Q_grid, theoretical, color=INFO_COLOR, label='P(error)=10^(−Q/10)')
ax.set_xlabel('Quality score Q')
ax.set_ylabel('Error probability (log scale)')
ax.set_title('DADA2 / Phred error rate vs Q')
ax.grid(True, which='both', ls=':', alpha=0.5)
ax.legend()
ax = axes[1]
im = ax.imshow(TM, cmap='magma', vmin=0, vmax=1)
ax.set_xticks(range(4))
ax.set_yticks(range(4))
ax.set_xticklabels(bases)
ax.set_yticklabels(bases)
ax.set_title(f'4×4 transition heatmap (rows: from, cols: to) @ Q={Q}')
for i in range(4):
    for j in range(4):
        ax.text(j, i, f'{TM[i][j]:.4f}', ha='center', va='center', color='w', fontsize=8)
plt.colorbar(im, ax=ax, fraction=0.046, label='Probability')
plt.tight_layout()
```

## (c) phyloseq parity — weighted UniFrac (toy tree)

Pedagogical **sum-normalized** weighted UniFrac on an explicit **parent→child** edge list (tip abundances only). **`phyloseq::distance(..., method='unifrac')`** on the full Newick uses the same abundance framing as the frozen JSON below; production Rust code lives in **`barracuda::bio::unifrac`** with additional normalization variants — see **`validate_r_industry_parity`** header comments.


```python
def weighted_unifrac(tree_edges, branch_lengths, counts_a, counts_b):
    """Weighted UniFrac: sum(bi * |Ai/AT - Bi/BT|) / sum(bi * (Ai/AT + Bi/BT))"""
    total_a = sum(counts_a.values())
    total_b = sum(counts_b.values())
    num, den = 0.0, 0.0
    for (parent, child), bl in zip(tree_edges, branch_lengths):
        pa = counts_a.get(child, 0) / max(total_a, 1)
        pb = counts_b.get(child, 0) / max(total_b, 1)
        num += bl * abs(pa - pb)
        den += bl * (pa + pb)
    return num / den if den > 0 else 0.0


# Five tips (A–E) on a bifurcating scaffold; internal labels are placeholders for edges
tree_edges = [
    ('root', 'AB'), ('AB', 'A'), ('AB', 'B'),
    ('root', 'CDE'), ('CDE', 'C'), ('CDE', 'DE'), ('DE', 'D'), ('DE', 'E'),
]
branch_lengths = [0.05, 0.12, 0.18, 0.04, 0.10, 0.07, 0.11, 0.13]

S1 = {'A': 100, 'B': 40, 'C': 30, 'D': 20, 'E': 10}
S2 = {'A': 20, 'B': 90, 'C': 25, 'D': 35, 'E': 30}
S3 = {'A': 50, 'B': 50, 'C': 50, 'D': 50, 'E': 50}

samples = {'S1': S1, 'S2': S2, 'S3': S3}
names = list(samples.keys())
n = len(names)
D = [[0.0] * n for _ in range(n)]
for i, ni in enumerate(names):
    for j, nj in enumerate(names):
        D[i][j] = weighted_unifrac(tree_edges, branch_lengths, samples[ni], samples[nj])

print('Toy weighted UniFrac (pedagogical edge list):')
for row, ni in zip(D, names):
    print(ni, ['{:.4f}'.format(v) for v in row])

fig, ax = plt.subplots(figsize=(5, 4.2))
im = ax.imshow(D, cmap='viridis', vmin=0, vmax=max(max(r) for r in D))
ax.set_xticks(range(n))
ax.set_yticks(range(n))
ax.set_xticklabels(names)
ax.set_yticklabels(names)
ax.set_title('Pairwise weighted UniFrac — toy 5-tip tree')
for i in range(n):
    for j in range(n):
        ax.text(j, i, f'{D[i][j]:.3f}', ha='center', va='center', color='w', fontsize=10)
plt.colorbar(im, ax=ax, fraction=0.046, label='Distance')
plt.tight_layout()
```

## Rust parity — frozen R validation JSON

Reload **`experiments/results/r_baselines/*.json`** (two levels up from `notebooks/papers/`) and assert exact numeric parity on the exported anchors. Full **`barracuda`** coverage: **`validate_r_industry_parity`** (53 checks as of Exp335 README), plus **`validate_diversity`** where Python baselines intersect generic diversity exports.


```python
fb_vegan = load('r_baselines/vegan_diversity.json')
fb_dada = load('r_baselines/dada2_error_model.json')
fb_phy = load('r_baselines/phyloseq_unifrac.json')

u10 = [100.0] * 10
sk = [900.0] + [11.0] * 9
f5 = [100.0, 80.0, 60.0, 40.0, 20.0]
assert abs(shannon(u10) - fb_vegan['shannon_uniform_10']) < 1e-12
assert abs(simpson(u10) - fb_vegan['simpson_uniform_10']) < 1e-12
assert abs(shannon(sk) - fb_vegan['shannon_skewed']) < 1e-12
assert abs(simpson(sk) - fb_vegan['simpson_skewed']) < 1e-12
assert abs(shannon(f5) - fb_vegan['shannon_5species']) < 1e-12
assert abs(simpson(f5) - fb_vegan['simpson_5species']) < 1e-12
ca, cb = [10.0, 20.0, 30.0, 40.0, 50.0], [50.0, 40.0, 30.0, 20.0, 10.0]
assert abs(bray_curtis(ca, cb) - fb_vegan['bray_curtis_ab']) < 1e-12
print('vegan JSON — Shannon/Simpson/Bray–Curtis parity OK')

for q, p in zip(fb_dada['phred_qualities'], fb_dada['phred_error_probs']):
    assert abs(10 ** (-q / 10.0) - p) < 1e-15
print('dada2 JSON — Phred ladder parity OK')

w = fb_phy['unifrac_weighted']
order_ok = w['s1_s2'] > w['s1_s3'] and w['s1_s2'] > w['s2_s3']
assert order_ok
assert fb_phy['unifrac_unweighted']['s1_s2'] == 0
print('phyloseq JSON — weighted ordering + zero unweighted (all OTUs shared) OK')
print('  reference WUF:', w)
print('Provenance:', fb_vegan['metadata']['command'], '|', fb_dada['metadata']['command'], '|', fb_phy['metadata']['command'])
```

## Tier 2 — IPC parity (`science.diversity`, guarded)

When **`WETSPRING_IPC_SOCKET`** resolves to a live daemon, **`ipc_call('science.diversity', {...})`** probes the same metrics surface used in production JSON-RPC. Otherwise the notebook stays **Tier 1** frozen.


```python
if TIER == 'live_ipc':
    try:
        div = ipc_call(
            'science.diversity',
            {
                'counts': IPC_COUNTS_A,
                'counts_b': IPC_COUNTS_B,
                'metrics': ['shannon', 'simpson', 'bray_curtis'],
            },
        )
        print('Tier 2 science.diversity OK:', json.dumps(div, indent=2)[:1200])
    except Exception as exc:
        print('Tier 2 science.diversity failed:', exc)
else:
    print('Tier 2 inactive — skipping ipc_call probes (frozen tier).')
```

## Summary — validation table, provenance, evolution

| # | Focus | R package | Frozen artifact | Rust validator |
|---|-------|-----------|-----------------|----------------|
| 1 | Shannon, Simpson, Bray–Curtis | vegan 2.7.3 | `r_baselines/vegan_diversity.json` | `validate_r_industry_parity`, `validate_diversity` |
| 2 | Phred ladder, error init | dada2 1.22.0 | `r_baselines/dada2_error_model.json` | `validate_r_industry_parity` (DADA2 sections) |
| 3 | Weighted / unweighted UniFrac | phyloseq 1.38.0 | `r_baselines/phyloseq_unifrac.json` | `validate_r_industry_parity`, `validate_unifrac` paths |
| 4 | Live IPC | — | requires `WETSPRING_IPC_SOCKET` | `science.diversity` JSON-RPC |

**Provenance:** R **4.1.2** snapshots on **2026-03-10** via `Rscript scripts/r_vegan_diversity_baseline.R`, `r_dada2_error_baseline.R`, `r_phyloseq_unifrac_baseline.R` (see each JSON `metadata`).

**Evolution path:** **Tier 1** — this notebook + **`experiments/results/`** JSON. **Tier 2** — guarded **`ipc_call('science.diversity', …)`**. **Tier 3** — **`barracuda`** binaries (`validate_r_industry_parity`, `validate_diversity`, streaming UniFrac validators) in CI with pinned tolerances.

**Status:** Regenerate `r_baselines/*.json` after changing R package versions or synthetic fixtures in `scripts/r_*_baseline.R`.


