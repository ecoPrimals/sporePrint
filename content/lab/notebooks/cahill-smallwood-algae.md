+++
title = "Algal Pond Ecology & Bloom Surveillance — Cahill/Smallwood (Sandia)"
description = "Rendered from cahill-smallwood-algae.ipynb"
date = 2026-06-28
weight = 50

[extra]
domain = "Lab"
rendered_from = "cahill-smallwood-algae.ipynb"
+++

<!-- Auto-generated from cahill-smallwood-algae.ipynb by spore-validate render-notebooks -->

# Algal Pond Ecology & Bloom Surveillance — Cahill/Smallwood (Sandia)

This notebook summarizes **raceway pond** microbiome surveillance patterns aligned with Sandia-led algal biotechnology work: longitudinal **16S-derived diversity** under disturbance (phage biocontrol proxy) and **bloom-era anomaly detection** on a surrogate time series.

| Paper | Focus | Key accession |
|-------|-------|---------------|
| **Cahill et al.** | *Nannochloropsis* raceway pond, phage biocontrol monitoring (PRJNA382322; 128 samples over ~4 months) | longitudinal diversity + crash surrogate |
| **Smallwood et al.** | Bloom-event surveillance & early-warning framing on metagenomic / amplicon time series | rolling statistics & anomaly flags |

**Scripts (Python baselines):** `algae_timeseries_baseline.py` (Exp039), `bloom_surveillance_baseline.py` (Exp040), `validate_public_16s_python.py` (public 16S proxy controls).

**Rust binaries:** `validate_algae_timeseries`, `validate_bloom_surveillance` (plus `validate_algae_16s` for real FASTQ benchmarks — see docs).

---

*For other springs: keep the scaffold — pure-Python diversity trajectories → matplotlib diagnostics → frozen JSON under `experiments/results/` → optional Tier-2 `science.diversity` IPC. Swap taxa counts for your longitudinal feature matrix.*

```python
import os, json, struct, socket
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
FAIL_COLOR = '#e74c3c'
INFO_COLOR = '#3498db'
```

## 1. Algal pond time-series — Exp039 (Cahill proxy)

Simulate longitudinal **relative-abundance–like** pseudocounts for **60** pond days (**15** synthetic taxa), **seed&nbsp;=&nbsp;42**, with an engineered **crash** at **t&nbsp;=&nbsp;40** (half suppressed, half boosted). Computes **Shannon** \(H\) per timepoint and **Bray–Curtis** dissimilarity between consecutive samples.

> Full Exp039 reproducibility uses **`n_taxa=20`** in `scripts/algae_timeseries_baseline.py`; the pedagogical slice below follows the **15-taxa** spec for this notebook while Rust parity reads the canonical frozen artifact.

```python
import math, random


def shannon(counts):
    total = sum(counts)
    if total <= 0:
        return 0.0
    return -sum((c / total) * math.log(c / total) for c in counts if c > 0)


def bray_curtis(a, b):
    num = sum(abs(x - y) for x, y in zip(a, b))
    den = sum(x + y for x, y in zip(a, b))
    return num / den if den > 0 else 0.0


def simulate_timeseries(n_timepoints, n_taxa, seed, crash_at=-1):
    rng = random.Random(seed)
    base = [rng.uniform(10, 100) for _ in range(n_taxa)]
    series = []
    for t in range(n_timepoints):
        sample = []
        for i in range(n_taxa):
            drift = rng.gauss(0, 5)
            seasonal = 10 * math.sin(2 * math.pi * t / 30)
            val = base[i] + drift + seasonal
            if crash_at >= 0 and t == crash_at:
                val *= 0.1 if i < n_taxa // 2 else 2.0
            sample.append(max(0.1, val))
        series.append(sample)
    return series


N_TIME = 60
N_TAXA = 15
SEED = 42
CRASH_T = 40
series = simulate_timeseries(N_TIME, N_TAXA, SEED, crash_at=CRASH_T)

shannon_vals = [shannon(row) for row in series]
bc_steps = []
for i in range(len(series) - 1):
    bc_steps.append(bray_curtis(series[i], series[i + 1]))
bc_t = list(range(1, len(series)))  # dissimilarity between t-1 and t

# For Tier 2 IPC (run this section before Tier 2 cell)
IPC_COUNTS_A = [float(x) for x in series[0]]
IPC_COUNTS_B = [float(x) for x in series[1]]

fig, axes = plt.subplots(2, 1, figsize=(11, 7), sharex=True)
t_axis = range(N_TIME)
axes[0].plot(t_axis, shannon_vals, color=INFO_COLOR, lw=2, label='Shannon H')
axes[0].axvline(CRASH_T, color=FAIL_COLOR, ls='--', lw=2, alpha=0.85, label=f'crash t={CRASH_T}')
axes[0].set_ylabel('Shannon diversity')
axes[0].set_title('Longitudinal Shannon diversity — raceway pond surrogate (60 d, 15 taxa)')
axes[0].legend(loc='upper right')
axes[0].grid(alpha=0.3)

axes[1].plot(bc_t, bc_steps, color=PASS_COLOR, lw=2, marker='.', ms=5, label='Bray-Curtis (consecutive)')
axes[1].axvline(CRASH_T, color=FAIL_COLOR, ls='--', lw=1.5, alpha=0.7)
axes[1].set_xlabel('Timepoint t (paired BC is vs t−1)')
axes[1].set_ylabel('Bray–Curtis')
axes[1].set_title('Beta turnover between consecutive pseudosamples')
axes[1].legend()
axes[1].grid(alpha=0.3)
plt.tight_layout()
plt.show()

print(f"Shannon at crash: {shannon_vals[CRASH_T]:.4f} | BC step entering crash (t={CRASH_T}): {bc_steps[CRASH_T - 1]:.4f}")
```

## 2. Bloom surveillance — rolling Z-score (Smallwood proxy)

Apply a **moving-window Z-score** to the Shannon trajectory (**window&nbsp;=&nbsp;7**). Flag timepoints where **|Z| &gt; 2** as **anomalies** — a minimal early-warning caricature complementary to **`bloom_surveillance_baseline.py`** (Exp040, multi-phase bloom/recovery).

```python
def rolling_zscore(values, window):
    zscores = [0.0] * len(values)
    for i in range(window, len(values)):
        w = values[i - window : i]
        mu = sum(w) / len(w)
        std = (sum((x - mu) ** 2 for x in w) / len(w)) ** 0.5
        zscores[i] = (values[i] - mu) / std if std > 0 else 0
    return zscores


WIN = 7
z_sh = rolling_zscore(shannon_vals, WIN)
anomaly_idx = [i for i in range(len(z_sh)) if abs(z_sh[i]) > 2.0]

fig, ax1 = plt.subplots(figsize=(11, 4.8))
ax1.plot(range(N_TIME), shannon_vals, color=INFO_COLOR, lw=2, label='Shannon H')
ax1.scatter(anomaly_idx, [shannon_vals[i] for i in anomaly_idx], color=FAIL_COLOR, s=55, zorder=5, label='|Z|>2 anomaly')
ax1.axvline(CRASH_T, color=FAIL_COLOR, ls='--', lw=1.3, alpha=0.65)
ax1.set_xlabel('Timepoint t')
ax1.set_ylabel('Shannon H', color=INFO_COLOR)
ax1.tick_params(axis='y', labelcolor=INFO_COLOR)
ax1.grid(alpha=0.3)

ax2 = ax1.twinx()
ax2.plot(range(N_TIME), z_sh, color=PASS_COLOR, lw=1.5, ls=':', alpha=0.9, label=f'Rolling Z (w={WIN})')
ax2.axhline(2, color='#95a5a6', lw=1, ls='-')
ax2.axhline(-2, color='#95a5a6', lw=1, ls='-')
ax2.set_ylabel('Z-score', color=PASS_COLOR)
ax2.tick_params(axis='y', labelcolor=PASS_COLOR)

lines_a, labs_a = ax1.get_legend_handles_labels()
lines_b, labs_b = ax2.get_legend_handles_labels()
ax1.legend(lines_a + lines_b, labs_a + labs_b, loc='upper right')
ax1.set_title('Shannon trajectory with rolling-window Z-score overlay — anomaly thresholds ±2')
plt.tight_layout()
plt.show()

print(f'Flagged anomalies (|Z|>2) at t = {anomaly_idx}')
```

## Rust parity — frozen Exp039 / Exp040 (+ public 16S control JSON)

`wetspring validate --scenario algae_timeseries` and `wetspring validate --scenario bloom_surveillance` consume the **`python_baseline.json`** fixtures written by **`scripts/`** into **`experiments/results/`**. `validate_public_16s_python.py` freezes **`python_16s_baselines.json`**.

```python
fb_algae = load('039_algae_timeseries/python_baseline.json')
fb_bloom = load('040_bloom_surveillance/python_baseline.json')
fb_16s = load('python_16s_controls/python_16s_baselines.json')

print('Frozen baselines (wetSpring RESULTS → two levels up from this notebook dir)')
print(
    " 039 algae timeseries —",
    fb_algae['experiment'],
    '| proxy:',
    fb_algae['proxy_for'],
    '| anomaly_detected:',
    fb_algae['anomaly_detected'],
)
print(
    "     Shannon@crash",
    fb_algae['crash'].get('shannon_at_crash'),
    '| BC@crash',
    fb_algae['crash'].get('bc_at_crash'),
)
print(
    " 040 bloom surveillance —",
    fb_bloom['experiment'],
    '| bloom_detected:',
    fb_bloom['bloom_detected'],
    '| bloom_window:',
    fb_bloom['bloom_window'],
)
k = next(iter(fb_16s))
print(" 16S controls — sample key", k, "| Shannon", fb_16s[k]['diversity']['shannon'])
```

## Tier 2 — IPC parity (`science.diversity`, guarded)

`science.diversity` accepts JSON **`counts`** and optional **`counts_b`** plus **`metrics`**. Invoked **only when** `WETSPRING_IPC_SOCKET` points to a live responder (see **`health.check`** in the Tier cell).

```python
if TIER == 'live_ipc':
    try:
        div = ipc_call(
            'science.diversity',
            {
                'counts': IPC_COUNTS_A,
                'counts_b': IPC_COUNTS_B,
                'metrics': ['shannon', 'simpson', 'chao1'],
            },
        )
        print('Tier 2 science.diversity OK:', json.dumps(div, indent=2)[:900])
    except Exception as exc:
        print('Tier 2 science.diversity failed:', exc)
else:
    print('Skipping IPC — Tier 1 frozen mode')
```

## Summary — validation footprint & provenance

| # | Notebook section | Proxied paper strand | Fixture / artifact | Rust binary |
|---|------------------|----------------------|--------------------|-------------|
| 1 | Shannon + Bray–Curtis crash | Cahill (#13 / PRJNA382322) | `039_algae_timeseries/python_baseline.json` | `wetspring validate --scenario algae_timeseries` |
| 2 | Rolling Z anomalies | Smallwood bloom surveillance narrative | `040_bloom_surveillance/python_baseline.json` | `wetspring validate --scenario bloom_surveillance` |
| 3 | Real public 16S QC hook | Supplementary `scripts/validate_public_16s_python.py` | `python_16s_controls/python_16s_baselines.json` | `wetspring validate --scenario algae_16s` |
| 4 | Parity IPC | — | Requires `WETSPRING_IPC_SOCKET` | `bio::diversity` surface via JSON-RPC |

**Provenance:** Inline functions mirror **`scripts/algae_timeseries_baseline.py`** (Shannon / Bray-Curtis / rolling Z). Frozen JSON is emitted by **`algae_timeseries_baseline.py`**, **`bloom_surveillance_baseline.py`**, and **`validate_public_16s_python.py`**.

**Evolution path:** **Tier 1** — standalone notebook + **`experiments/results/`** JSON artifacts. **Tier 2** — guarded **`ipc_call('science.diversity', …)`** against a running wetSpring IPC daemon. **Tier 3** — production runs wrapped in **`barracuda`** validation binaries with reproducible hashing in CI.

**Status:** Scripts + Rust validators are exercised in-repo; regenerate baselines after changing RNG or taxa cardinality in **`scripts/`**.

