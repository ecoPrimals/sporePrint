+++
title = "Experiment 016 — Rare Biosphere Signal Detection"
description = "Rendered from exp-016-rare-biosphere.ipynb"
date = 2026-06-09
weight = 50

[extra]
domain = "Lab"
rendered_from = "exp-016-rare-biosphere.ipynb"
+++

<!-- Auto-generated from exp-016-rare-biosphere.ipynb by spore-validate render-notebooks -->

# Experiment 016 — Rare Biosphere Signal Detection

At what sequencing depth can we reliably distinguish rare biological
lineages from sequencing artifacts?  This extends Exp 004 (genus
saturation at 5 000 reads) to the rare end of the abundance distribution
using Chao1 richness estimation and analytical detection power.

**Method:**
- Synthetic community with 50 species across 5 abundance tiers
- Multinomial sampling simulates sequencing at various depths
- Chao1 non-parametric richness estimator (Chao 1984)
- Detection power: P(detect) = 1 - (1 - p)^D
- Detection threshold: D* = ceil(ln(0.05) / ln(1-p))
- Abundance-occupancy relationship across replicate samples

**Reference:**
Anderson, Sogin, Baross (2015) FEMS Microbiol Ecol 91:fiv016
Chao (1984) Scand J Stat 11:265-270
Sogin et al. (2006) PNAS 103:12115-12120

**Cross-spring: wetSpring (microbial diversity), Exp 004 (sequencing depth).**

**Domain**: Genomics
**Faculty**: R. Anderson (Carleton)
**Reference**: R. Anderson — deep subsurface microbiology

**Data source**: `control/rare_biosphere/rare_biosphere.py` + `benchmark_*.json`

---

*This notebook is the publication-grade Python baseline for Experiment 016. The identical computations are validated in Rust (see `validate_*` binary) and delegated to barraCuda for GPU acceleration.*


```python
import json
import math
import sys
from pathlib import Path

import numpy as np
from scipy.stats import spearmanr
import matplotlib.pyplot as plt

# Wire path to groundSpring control/ for common utilities
CONTROL = Path('..') / '..' / 'control'
sys.path.insert(0, str(CONTROL))
from common import *  # noqa: F403 — validation harness

# Load benchmark data
benchmark_path = CONTROL / 'rare_biosphere' / 'benchmark_rare_biosphere.json'
with open(benchmark_path) as f:
    benchmark = json.load(f)

PASS_COLOR = '#2ecc71'
FAIL_COLOR = '#e74c3c'
INFO_COLOR = '#3498db'

print(f'Loaded benchmark: benchmark_rare_biosphere.json')
print(f'Provenance: {benchmark.get("_provenance", {})}')
```

## Validation


### Initialization


```python
model = benchmark["model"]
pred = benchmark["analytical_predictions"]
exp = benchmark["expected_results"]

community = np.array(model["community"])
n_species = model["n_species"]
depths = model["depths"]
n_reps = model["n_replicates"]
base_seed = model["base_seed"]
tiers = model["tier_boundaries"]

print("groundSpring Exp 016: Rare Biosphere Signal Detection")
print(f"  Community: {n_species} species, 5 abundance tiers")
print(f"  Depths: {depths}")
print(f"  Replicates: {n_reps}")
print("  Reference: Anderson, Sogin, Baross (2015) FEMS")
```

### Chao1 Richness Estimation


```python
rng = np.random.default_rng(base_seed)

chao1_by_depth = {}
sobs_by_depth = {}
for depth in depths:
    chao1_vals = []
    sobs_vals = []
    for _ in range(n_reps):
        seed = int(rng.integers(0, 2**32))
        rep_rng = np.random.default_rng(seed)
        counts = rep_rng.multinomial(depth, community)
        chao1_vals.append(chao1(counts))
        sobs_vals.append(int(np.sum(counts > 0)))
    mean_chao1 = float(np.mean(chao1_vals))
    mean_sobs = float(np.mean(sobs_vals))
    chao1_by_depth[depth] = mean_chao1
    sobs_by_depth[depth] = mean_sobs
    print(f"  D={depth:6d}: S_obs={mean_sobs:.1f}, Chao1={mean_chao1:.1f} (true={n_species})")

check_range(
    "Chao1 at D=50000 ≈ true richness",
    chao1_by_depth[50000],
    exp["chao1_at_depth_50000_range"][0],
    exp["chao1_at_depth_50000_range"][1],
)

check_true(
    "Chao1 > S_obs at low depth (D=100)",
    chao1_by_depth[100] > sobs_by_depth[100],
)

check_true(
    "All species detected at D=50000",
    sobs_by_depth[50000] >= exp["sobs_at_depth_50000"] - 0.5,
)
```

### Detection Power by Tier


```python
dom_lo, dom_hi = tiers["dominant"]
vr_lo, vr_hi = tiers["very_rare"]

dom_rate = tier_detection_rate(
    community, dom_lo, dom_hi, 100, n_reps, base_seed + 1000,
)
vr_rate_100 = tier_detection_rate(
    community, vr_lo, vr_hi, 100, n_reps, base_seed + 2000,
)
vr_rate_5000 = tier_detection_rate(
    community, vr_lo, vr_hi, 5000, n_reps, base_seed + 3000,
)

p_dom = detection_power(0.06, 100)
p_vr_100 = detection_power(0.003, 100)
p_vr_5000 = detection_power(0.003, 5000)

print(f"  Dominant  at D=100:  rate={dom_rate:.3f} (theory ≥ {p_dom:.3f})")
print(f"  Very rare at D=100:  rate={vr_rate_100:.3f} (theory ≈ {p_vr_100:.3f})")
print(f"  Very rare at D=5000: rate={vr_rate_5000:.3f} (theory ≈ {p_vr_5000:.3f})")

check_min(
    "Dominant detected at D=100",
    dom_rate,
    exp["detection_rate_dominant_at_100_min"],
)
check_max(
    "Very rare rarely detected at D=100",
    vr_rate_100,
    exp["detection_rate_very_rare_at_100_max"],
)
check_min(
    "Very rare detected at D=5000",
    vr_rate_5000,
    exp["detection_rate_very_rare_at_5000_min"],
)
```

### Analytical Detection Thresholds


```python
for label, p_val, expected_key in [
    ("very_rare (p=0.003)", 0.003, "detection_threshold_very_rare_p003"),
    ("rare (p=0.004)", 0.004, "detection_threshold_rare_p004"),
    ("moderate (p=0.008)", 0.008, "detection_threshold_moderate_p008"),
    ("common (p=0.030)", 0.030, "detection_threshold_common_p030"),
]:
    computed = detection_threshold(p_val, 0.95)
    expected = pred[expected_key]
    print(f"  {label}: D*={computed} (expected {expected})")

check_true(
    "Detection threshold monotonically decreases with abundance",
    (
        detection_threshold(0.003) > detection_threshold(0.004)
        > detection_threshold(0.008) > detection_threshold(0.030)
    ),
)
```

### Abundance-Occupancy Relationship


```python
n_samples = model["n_samples_occupancy"]
occ_depth = model["occupancy_depth"]
occ_rng = np.random.default_rng(base_seed + 50000)

detection_counts = np.zeros(n_species)
for _ in range(n_samples):
    seed = int(occ_rng.integers(0, 2**32))
    rep_rng = np.random.default_rng(seed)
    counts = rep_rng.multinomial(occ_depth, community)
    detection_counts += (counts > 0).astype(float)
occupancy = detection_counts / n_samples

dom_occ = float(np.mean(occupancy[tiers["dominant"][0]:tiers["dominant"][1]]))
vr_occ = float(np.mean(occupancy[tiers["very_rare"][0]:tiers["very_rare"][1]]))
print(f"  Dominant mean occupancy:   {dom_occ:.3f}")
print(f"  Very rare mean occupancy:  {vr_occ:.3f}")

from scipy.stats import spearmanr
rho, _ = spearmanr(community, occupancy)
print(f"  Spearman(abundance, occupancy) = {rho:.3f}")

check_true("Occupancy positively correlated with abundance", bool(rho > 0.5))
```

### Singleton Fraction vs Depth


```python
sing_rng = np.random.default_rng(base_seed + 60000)
singleton_fracs = {}
for depth in depths:
    frac_sum = 0.0
    for _ in range(n_reps):
        seed = int(sing_rng.integers(0, 2**32))
        rep_rng = np.random.default_rng(seed)
        counts = rep_rng.multinomial(depth, community)
        s_obs = int(np.sum(counts > 0))
        f1 = int(np.sum(counts == 1))
        frac_sum += f1 / s_obs if s_obs > 0 else 0.0
    singleton_fracs[depth] = frac_sum / n_reps

for depth in depths:
    print(f"  D={depth:6d}: singleton fraction = {singleton_fracs[depth]:.3f}")

check_true(
    "Singleton fraction decreases with depth",
    singleton_fracs[depths[0]] > singleton_fracs[depths[-1]],
)
```

### Determinism


```python
det_rng1 = np.random.default_rng(99999)
det_rng2 = np.random.default_rng(99999)
c1 = det_rng1.multinomial(1000, community)
c2 = det_rng2.multinomial(1000, community)
check_true("Multinomial deterministic (same seed)", np.array_equal(c1, c2))

chao1_a = chao1(c1)
chao1_b = chao1(c2)
check_true("Chao1 deterministic", chao1_a == chao1_b)
```

### Key Findings


```python
print(f"\n{'=' * 72}")
```

## Visualization


```python
# Publication-grade summary chart for Exp 016
fig, ax = plt.subplots(figsize=(8, 4))

p, f_count, t = pass_count(), fail_count(), total_count()
ax.barh(['Pass', 'Fail'], [p, f_count], color=[PASS_COLOR, FAIL_COLOR])
ax.set_xlim(0, max(t * 1.15, 1))
ax.set_title('Exp 016: Rare Biosphere Signal Detection — Validation Results')
ax.set_xlabel('Check Count')
for i, v in enumerate([p, f_count]):
    if v > 0:
        ax.text(v + 0.3, i, str(v), va='center', fontweight='bold')

plt.tight_layout()
plt.savefig(f'/tmp/groundspring_exp016.png', dpi=150, bbox_inches='tight')
plt.show()
print(f'\nResult: {p}/{t} PASS, {f_count}/{t} FAIL')
```

## Provenance & Summary

| Field | Value |
|-------|-------|
| Experiment | 016 — Rare Biosphere Signal Detection |
| Domain | Genomics |
| Reference | R. Anderson — deep subsurface microbiology |
| Faculty | R. Anderson (Carleton) |
| Python baseline | `control/rare_biosphere/rare_biosphere.py` |
| Benchmark JSON | `control/rare_biosphere/benchmark_rare_biosphere.json` |
| Rust validator | `validate_*` binary (exit-code protocol) |
| Rust speedup | See benchmark comparison notebook |
| License | AGPL-3.0-or-later |

**Provenance chain**: Python baseline → Rust validation → barraCuda GPU → metalForge cross-substrate → primal IPC composition

See [primals.eco](https://primals.eco) for rendered lab notebooks.


