+++
title = "Deep-Sea Hydrothermal Vent Ecology — R. Anderson Lab (Carleton)"
description = "Rendered from anderson-deep-sea.ipynb"
date = 2026-06-10
weight = 50

[extra]
domain = "Lab"
rendered_from = "anderson-deep-sea.ipynb"
+++

<!-- Auto-generated from anderson-deep-sea.ipynb by spore-validate render-notebooks -->

# Deep-Sea Hydrothermal Vent Ecology — R. Anderson Lab (Carleton)

papers:
| # | Paper | DOI | Model |
|---|-------|-----|-------|
| 1 | Anderson et al. 2015 | 10.1093/femsec/fiu016 | Rare biosphere diversity |
| 2 | Anderson et al. 2014 | 10.1371/journal.pone.0109696 | Viral metagenomics + dN/dS |
| 3 | Mateos et al. 2023 | (deep-sea sulfur phylogenomics) | Sulfur phylogenomics |
| 4 | Boden et al. 2024 | (phosphorus phylogenomics) | Phosphorus cycling |
| 5 | Anderson et al. 2017 | (population genomics) | ANI/SNP analysis |
| 6 | Moulana et al. 2020 | (pangenomics) | Sulfurovum pangenomics |

**Rust binaries**: `validate_rare_biosphere` (Exp051), `validate_viral_metagenomics` (Exp052), `validate_sulfur_phylogenomics` (Exp053), `validate_phosphorus_phylogenomics` (Exp054), `validate_population_genomics` (Exp055), `validate_pangenomics` (Exp056).

*For other springs: swap in your domain-specific abundance matrices, sequences, or gene-presence matrices; keep the RESULTS-relative paths and parity pattern.*


```python
import os, json, math, struct, socket
from pathlib import Path

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

import matplotlib
import matplotlib.pyplot as plt

PASS_COLOR = '#2ecc71'
FAIL_COLOR = '#e74c3c'
INFO_COLOR = '#3498db'
```

## Anderson 2015 — Rare biosphere diversity

Synthetic vent communities (**Piccard**, **Von Damm**, **background seawater**) illustrate how dominance vs evenness shifts Shannon, Simpson, Chao1, and pairwise Bray–Curtis dissimilarity.


```python
# Pure-Python diversity (no scipy / numpy required)


def shannon(counts):
    # Shannon H' = -sum p_i ln(p_i) on nonzero reads
    total = sum(counts)
    if total <= 0:
        return 0.0
    h = 0.0
    for c in counts:
        if c > 0:
            p = c / total
            h -= p * math.log(p)
    return h


def simpson(counts):
    # Simpson 1 - sum(p_i^2)
    total = sum(counts)
    if total <= 0:
        return 0.0
    s = 0.0
    for c in counts:
        p = c / total
        s += p * p
    return 1.0 - s


def chao1(counts):
    # Chao1 richness from abundance vector (singleton / doubleton formula)
    observed = sum(1 for c in counts if c > 0)
    singletons = sum(1 for c in counts if c == 1)
    doubletons = sum(1 for c in counts if c == 2)
    if doubletons == 0:
        if singletons > 0:
            return observed + singletons * (singletons - 1) / 2.0
        return float(observed)
    return observed + singletons ** 2 / (2.0 * doubletons)


def bray_curtis(a, b):
    # Bray-Curtis dissimilarity; pads shorter vector with zeros
    max_len = max(len(a), len(b))
    a_ext = list(a) + [0] * (max_len - len(a))
    b_ext = list(b) + [0] * (max_len - len(b))
    num = sum(abs(a_ext[i] - b_ext[i]) for i in range(max_len))
    den = sum(a_ext[i] + b_ext[i] for i in range(max_len))
    return num / den if den else 0.0


piccard = [500, 300, 100, 50, 20, 10, 5, 5, 5, 5]

von_damm = [
    200, 180, 150, 120, 100, 80, 60, 40, 20, 10,
    8, 5, 5, 3, 2, 1, 1, 1, 1, 1,
]

# Uniform-ish 50 OTUs + long tail of singletons / rare taxa
background = [14] * 10 + [10] * 10 + [7] * 15 + [4] * 5 + [2, 2, 1, 1, 1, 1, 1, 1, 1, 1]

sites = {
    'Piccard': piccard,
    'Von Damm': von_damm,
    'Background': background,
}

rows = []
for name, cts in sites.items():
    rows.append((name, shannon(cts), simpson(cts), chao1(cts)))

labels = [r[0] for r in rows]
metrics = [('Shannon H′', [r[1] for r in rows]),
           ('Simpson 1−Σp²', [r[2] for r in rows]),
           ('Chao1 Ŝ', [r[3] for r in rows])]

x = range(len(labels))
width = 0.25

fig, axes = plt.subplots(1, 2, figsize=(12, 4.5))

ax = axes[0]
for i, (ttl, vals) in enumerate(metrics):
    ax.bar([xi + i * width for xi in x], vals, width, label=ttl,
           color=[PASS_COLOR, INFO_COLOR, FAIL_COLOR][i])
ax.set_xticks([xi + width for xi in x])
ax.set_xticklabels(labels, rotation=12, ha='right')
ax.set_ylabel('Index value')
ax.set_title('Synthetic vent communities — diversity indices')
ax.legend(fontsize=8)
ax.grid(alpha=0.3, axis='y')

order = list(sites.keys())
n = len(order)
dist = [[0.0] * n for _ in range(n)]
for i, si in enumerate(order):
    for j, sj in enumerate(order):
        dist[i][j] = bray_curtis(sites[si], sites[sj])

ax = axes[1]
im = ax.imshow(dist, cmap='YlOrRd', aspect='auto', vmin=0, vmax=max(max(row) for row in dist))
ax.set_xticks(range(n))
ax.set_yticks(range(n))
ax.set_xticklabels(order, rotation=20, ha='right')
ax.set_yticklabels(order)
ax.set_title('Bray–Curtis dissimilarity')
for i in range(n):
    for j in range(n):
        ax.text(j, i, f'{dist[i][j]:.2f}', ha='center', va='center', fontsize=9, color='black')
plt.colorbar(im, ax=ax, fraction=0.046, pad=0.04, label='Bray–Curtis')

plt.tight_layout()
plt.show()
```

## Anderson 2014 — Viral metagenomics / Nei–Gojobori dN/dS

Standard codon table; per-codon **synonymous vs nonsynonymous site** counts averaged over single-nucleotide neighbors; substitutions classified along averaged pathways for multi-hit codons. **Jukes–Cantor** correction yields *d<sub>S</sub>* and *d<sub>N</sub>*; ω = *d<sub>N</sub>*/*d<sub>S</sub>*.

Interpret ω: **&lt;1** purifying selection, **&gt;1** positive selection, **≈1** neutral.


```python
CODON_TABLE = {
    "TTT": "F", "TTC": "F", "TTA": "L", "TTG": "L",
    "CTT": "L", "CTC": "L", "CTA": "L", "CTG": "L",
    "ATT": "I", "ATC": "I", "ATA": "I", "ATG": "M",
    "GTT": "V", "GTC": "V", "GTA": "V", "GTG": "V",
    "TCT": "S", "TCC": "S", "TCA": "S", "TCG": "S",
    "CCT": "P", "CCC": "P", "CCA": "P", "CCG": "P",
    "ACT": "T", "ACC": "T", "ACA": "T", "ACG": "T",
    "GCT": "A", "GCC": "A", "GCA": "A", "GCG": "A",
    "TAT": "Y", "TAC": "Y", "TAA": "*", "TAG": "*",
    "CAT": "H", "CAC": "H", "CAA": "Q", "CAG": "Q",
    "AAT": "N", "AAC": "N", "AAA": "K", "AAG": "K",
    "GAT": "D", "GAC": "D", "GAA": "E", "GAG": "E",
    "TGT": "C", "TGC": "C", "TGA": "*", "TGG": "W",
    "CGT": "R", "CGC": "R", "CGA": "R", "CGG": "R",
    "AGT": "S", "AGC": "S", "AGA": "R", "AGG": "R",
    "GGT": "G", "GGC": "G", "GGA": "G", "GGG": "G",
}
BASES = ['A', 'C', 'G', 'T']


def translate(codon):
    return CODON_TABLE.get(codon.upper())


def codon_sites(codon):
    codon = codon.upper()
    orig_aa = translate(codon)
    if orig_aa is None or orig_aa == '*':
        return 0.0, 0.0
    syn_sites = 0.0
    for pos in range(3):
        syn_changes = 0
        total_changes = 0
        for base in BASES:
            if base == codon[pos]:
                continue
            mutant = ''.join((codon[:pos] + base + codon[pos + 1:]))
            total_changes += 1
            new_aa = translate(mutant)
            if new_aa not in (None, '*') and new_aa == orig_aa:
                syn_changes += 1
        syn_sites += syn_changes / total_changes if total_changes else 0.0
    return syn_sites, 3.0 - syn_sites


def pathway_diffs(c1, c2, order):
    cur = list(c1)
    syn = non = 0.0
    for pos in order:
        aa_b = translate(''.join(cur))
        cur[pos] = c2[pos]
        aa_a = translate(''.join(cur))
        if aa_b not in (None, '*') and aa_a not in (None, '*') and aa_b == aa_a:
            syn += 1.0
        else:
            non += 1.0
    return syn, non


def permutations(items):
    if len(items) <= 1:
        return [list(items)]
    out = []
    for i, it in enumerate(items):
        rest = items[:i] + items[i + 1:]
        for perm in permutations(rest):
            out.append([it] + perm)
    return out


def count_codon_diffs(c1, c2):
    diff_pos = [i for i in range(3) if c1[i] != c2[i]]
    n = len(diff_pos)
    if n == 0:
        return 0.0, 0.0
    if n == 1:
        a1, a2 = translate(c1), translate(c2)
        if a1 not in (None, '*') and a2 not in (None, '*') and a1 == a2:
            return 1.0, 0.0
        return 0.0, 1.0
    tsyn = tnon = 0.0
    for perm in permutations(diff_pos):
        s, n_ = pathway_diffs(list(c1), list(c2), perm)
        tsyn += s
        tnon += n_
    cnt = math.factorial(n)
    return tsyn / cnt, tnon / cnt


def jukes_cantor(p):
    if p <= 0.0:
        return 0.0
    arg = 1.0 - 4.0 * p / 3.0
    if arg <= 0.0:
        return float('inf')
    return -0.75 * math.log(arg)


def pairwise_dnds(seq1, seq2):
    assert len(seq1) == len(seq2) and len(seq1) % 3 == 0
    ts_sites = tn_sites = sd = nd = 0.0
    for i in range(0, len(seq1), 3):
        c1, c2 = seq1[i:i + 3], seq2[i:i + 3]
        if '-' in c1 + c2 or '.' in c1 + c2:
            continue
        s1_syn, s1_ns = codon_sites(c1)
        s2_syn, s2_ns = codon_sites(c2)
        ts_sites += (s1_syn + s2_syn) / 2
        tn_sites += (s1_ns + s2_ns) / 2
        s_diff, n_diff = count_codon_diffs(c1, c2)
        sd += s_diff
        nd += n_diff
    ps = sd / ts_sites if ts_sites else 0.0
    pn = nd / tn_sites if tn_sites else 0.0
    ds, dn = jukes_cantor(ps), jukes_cantor(pn)
    omega = (dn / ds) if ds > 1e-10 else None
    return {'dn': dn, 'ds': ds, 'omega': omega, 'label': ''}


pairs = [
    ('Synonymous', 'TTT', 'TTC'),
    ('Nonsynonymous', 'TTT', 'TCT'),
    ('Mixed', 'TTTATGCCC', 'TTCATGCCA'),
]

vals = []
for label, sa, sb in pairs:
    r = pairwise_dnds(sa, sb)
    r['label'] = label
    vals.append(r)
    om = '∞' if r['omega'] is None else f"{r['omega']:.4f}"
    print(f"{label}: dN={r['dn']:.4f}, dS={r['ds']:.4f}, ω={om}")

lbls = [v['label'] for v in vals]
fig, ax = plt.subplots(figsize=(7.5, 3.8))
x = range(len(lbls))
ax.bar([i - 0.18 for i in x], [v['ds'] if math.isfinite(v['ds']) else 0 for v in vals],
       width=0.35, label='dS', color=INFO_COLOR)
_dns = []
for v in vals:
    d = v['dn']
    _dns.append(d if math.isfinite(d) else 0)
ax.bar([i + 0.18 for i in x], _dns, width=0.35, label='dN', color=FAIL_COLOR)
ax.set_xticks(list(x))
ax.set_xticklabels(lbls)
ax.set_ylabel('Jukes–Cantor distance')
ax.set_title('Nei–Gojobori pairwise test codons')
ax.legend()
ax.grid(alpha=0.3, axis='y')

# ω text overlay
for i, v in enumerate(vals):
    if v['omega'] is None:
        t = 'ω: n/a'
    elif not math.isfinite(v['omega']):
        t = 'ω: ∞'
    else:
        t = f"ω={v['omega']:.3f}"
    ax.text(i, max(v['dn'], v['ds'] if math.isfinite(v['ds']) else 0) * 1.08, t, ha='center', fontsize=9)

plt.tight_layout()
plt.show()
```

## Mateos 2023 & Boden 2024 — Sulfur and phosphorus phylogenomics

Molecular-clock priors, node-age constraints, and lightweight reconciliation sanity checks are exercised in **`validate_sulfur_phylogenomics`** (Exp053) and **`validate_phosphorus_phylogenomics`** (Exp054); see frozen JSON summaries in the parity cell below.


## Anderson 2017 — Population genomics (ANI)

Average nucleotide identity (ANI) between aligned fragments: identities / aligned (non-gap, non-`N`) positions.


```python
def pairwise_ani(seq1, seq2):
    la, ident = 0, 0
    for a, b in zip(seq1.upper(), seq2.upper()):
        if a in '-.N' or b in '-.N':
            continue
        la += 1
        if a == b:
            ident += 1
    return ident / la if la else 0.0


# Short synthetic haplotypes (barcode-style)
hap_a = 'ATGATGATGATGATGATGATGATGATGATGATGATGATGATGATGATGATG'
hap_b = ''.join(('C' if i in (0, 10) else hap_a[i]) for i in range(len(hap_a)))
hap_c = ''.join(('C' if i % 10 == 0 and hap_a[i] != 'C' else ('G' if hap_a[i] == 'C' else hap_a[i]))
                for i in range(len(hap_a)))
hap_d = hap_a[:-10] + 'CCCCCCCCCC'

strains = [hap_a, hap_b, hap_c, hap_d]
nst = len(strains)
mat = [[0.0] * nst for _ in range(nst)]
for i in range(nst):
    for j in range(nst):
        mat[i][j] = pairwise_ani(strains[i], strains[j])

labs = ['type A', 'A+2 SNP', 'A sporadic', 'truncated']

fig, ax = plt.subplots(figsize=(5, 4.5))
im = ax.imshow(mat, cmap='GnBu', vmin=0.8, vmax=1.0, aspect='auto')
ax.set_xticks(range(nst))
ax.set_yticks(range(nst))
ax.set_xticklabels(labs, rotation=18, ha='right')
ax.set_yticklabels(labs)
for i in range(nst):
    for j in range(nst):
        ax.text(j, i, f'{mat[i][j]:.3f}', ha='center', va='center', fontsize=9)
ax.set_title('Pairwise ANI (synthetic haplotypes)')
plt.colorbar(im, ax=ax, fraction=0.046, pad=0.04, label='ANI')
plt.tight_layout()
plt.show()
```

## Moulana 2020 — Pangenomics (core / shell / cloud)

Gene families scored by fraction of genomes present: **core** (100%), **shell** (50–99%), **cloud** (&lt;50%, present in ≥1 genome). This matches intuition from *Sulfurovum* pangenome gain/loss studies.


```python
# Rows: gene families; columns: genomes (1 = present)
import random

rng = random.Random(42)
G, F = 6, 18
presence = []
for fi in range(F):
    frac = rng.random()
    if frac < 0.25:
        # core — all strains
        row = [1] * G
    elif frac < 0.55:
        # shell — 50–99%
        k = rng.randint(max(3, G // 2), G - 1)
        idx = rng.sample(range(G), k)
        row = [1 if i in idx else 0 for i in range(G)]
    else:
        # cloud — <50% but ≥1
        k = rng.randint(1, max(1, G // 2 - 1))
        idx = rng.sample(range(G), k)
        row = [1 if i in idx else 0 for i in range(G)]
    presence.append(row)

counts = {'core': 0, 'shell': 0, 'cloud': 0}
for row in presence:
    f = sum(row) / G
    if f >= 1.0:
        counts['core'] += 1
    elif f >= 0.5:
        counts['shell'] += 1
    else:
        counts['cloud'] += 1

print('Families:', F, '| per category:', counts)

fig, ax = plt.subplots(figsize=(6, 3.8))
cats = ['core\n(100%)', 'shell\n(50–99%)', 'cloud\n(<50%)']
nums = [counts['core'], counts['shell'], counts['cloud']]
bars = ax.bar(cats, nums, color=[PASS_COLOR, INFO_COLOR, FAIL_COLOR])
ax.set_ylabel('Gene families')
ax.set_title('Random presence/absence pangenome partition (demo)')
for b, n in zip(bars, nums):
    ax.text(b.get_x() + b.get_width() / 2, b.get_height() + 0.05, str(n),
            ha='center', fontsize=10)
plt.tight_layout()
plt.show()
```

## Rust parity — frozen Exp051–056 baselines

Load canonical JSON emitted by `scripts/anderson*.py` and related generators; Rust binaries diff against these fixtures in CI.


```python
fb51 = load('051_rare_biosphere/anderson2015_python_baseline.json')
fb52 = load('052_viral_metagenomics/anderson2014_python_baseline.json')
fb53 = load('053_sulfur_phylogenomics/mateos2023_python_baseline.json')
fb54 = load('054_phosphorus_phylogenomics/boden2024_python_baseline.json')
fb55 = load('055_population_genomics/anderson2017_python_baseline.json')
fb56 = load('056_pangenomics/moulana2020_python_baseline.json')

print('Frozen baselines (wetSpring RESULTS)')
print(f" 051 rare biosphere — Piccard Shannon={fb51['piccard']['shannon']:.4f}, Von Damm Chao1={fb51['von_damm']['chao1']:.2f}")
print(f" 052 viral meta — synonymous ω={fb52['dnds_synonymous']['omega']} (frozen uses multi-codon cases)")
print(f" 053 sulfur — root_age={fb53['root_age']}, rf_congruent={fb53['rf_congruent']}")
print(f" 054 phosphorus — n_nodes={fb54['n_nodes']}, tree_height={fb54['tree_height']}")
print(f" 055 pop gen — ANI same-species={fb55['ani_same_species']['ani']:.4f}")
print(f" 056 pangenomics — core={fb56['pangenome']['core']}, accessory={fb56['pangenome']['accessory']}")
```

## Tier 2 — IPC parity (guarded)

`science.diversity` accepts JSON **`counts`** (and optional **`counts_b`** for Bray–Curtis). It is invoked only when **`WETSPRING_IPC_SOCKET`** is set and responds to **`health.check`**.


```python
if TIER == 'live_ipc':
    try:
        div = ipc_call('science.diversity', {
            'counts': [float(x) for x in piccard],
            'counts_b': [float(x) for x in von_damm[: len(piccard)]]
            + [0.0] * max(0, len(piccard) - len(von_damm)),
            'metrics': ['shannon', 'simpson', 'chao1'],
        })
        print('Tier 2 science.diversity OK:', json.dumps(div, indent=2)[:800])
    except Exception as exc:
        print('Tier 2 science.diversity failed:', exc)
else:
    print('Skipping IPC — Tier 1 frozen mode')
```

## Summary

| # | Paper | Model | Rust binary | Status |
|---|-------|-------|-------------|--------|
| 1 | Anderson 2015 | Rare biosphere diversity | `validate_rare_biosphere` | PASS |
| 2 | Anderson 2014 | Viral Shannon + NG dN/dS | `validate_viral_metagenomics` | PASS |
| 3 | Mateos 2023 | Sulfur clock / phylogenomics | `validate_sulfur_phylogenomics` | PASS |
| 4 | Boden 2024 | Phosphorus clock | `validate_phosphorus_phylogenomics` | PASS |
| 5 | Anderson 2017 | ANI / SNP primitives | `validate_population_genomics` | PASS |
| 6 | Moulana 2020 | Pangenome + enrichment hooks | `validate_pangenomics` | PASS |

**Provenance:** Diversity and dN/dS implementations mirror `scripts/anderson2015_rare_biosphere.py` and `scripts/anderson2014_viral_metagenomics.py`; ANI aligns with `scripts/anderson2017_population_genomics.py`. Frozen baselines live under **`experiments/results/`**.

**Evolution:** **Tier 1** — standalone Python + frozen JSON. **Tier 2** — guarded `ipc_call('science.diversity', …)`. **Tier 3** — wrap runs in provenance sessions (Hashes, BLAKE3) as in barracuda validation harness.


