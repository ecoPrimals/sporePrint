+++
title = "Experiment 028 — NPU Anderson Classification"
description = "Rendered from exp-028-npu-anderson.ipynb"
date = 2026-06-01
weight = 50

[extra]
domain = "Lab"
rendered_from = "exp-028-npu-anderson.ipynb"
+++

<!-- Auto-generated from exp-028-npu-anderson.ipynb by spore-validate render-notebooks -->

# Experiment 028 — NPU Anderson Classification

Classifies Anderson localization regimes (Localized / Critical / Extended)
using int8-quantized features (W, E, L) and a simple centroid classifier.
This baseline establishes ground truth for the Rust + AKD1000 NPU path.
Validation checks:
1. CPU regime classification matches expected labels
2. Quantization round-trip error within tolerance
3. Training produces a 3×3 weight matrix
4. CPU classifier accuracy is 100% on training set
5. All three regime classes are covered
6. Extended regime detected for weak disorder
7. Localized regime detected for strong disorder

**Domain**: Neuromorphic
**Faculty**: metalForge (NPU)
**Reference**: BrainChip AKD1000 Anderson classification

**Data source**: `control/npu_anderson/npu_anderson.py` + `benchmark_*.json`

---

*This notebook is the publication-grade Python baseline for Experiment 028. The identical computations are validated in Rust (see `validate_*` binary) and delegated to barraCuda for GPU acceleration.*


```python
import json
import math
import sys
from pathlib import Path

import numpy as np
import matplotlib.pyplot as plt

# Wire path to groundSpring control/ for common utilities
CONTROL = Path('..') / '..' / 'control'
sys.path.insert(0, str(CONTROL))
from common import *  # noqa: F403 — validation harness

# Load benchmark data
benchmark_path = CONTROL / 'npu_anderson' / 'benchmark_npu_anderson.json'
with open(benchmark_path) as f:
    benchmark = json.load(f)

PASS_COLOR = '#2ecc71'
FAIL_COLOR = '#e74c3c'
INFO_COLOR = '#3498db'

print(f'Loaded benchmark: benchmark_npu_anderson.json')
print(f'Provenance: {benchmark.get("_provenance", {})}')
```

## Validation


### Initialization


```python
bench = load_benchmark()
model = bench["model"]
expected = bench["expected_results"]

n_sites = model["n_sites"]
energy = model["energy"]
derrida_c = model["derrida_gardner_C"]
disorders = model["disorders"]
expected_regimes = expected["cpu_regimes"]


print("groundSpring Exp 028: NPU Anderson Regime Classification")

# Check 1: CPU regime classification
cpu_regimes = [classify_regime(w, energy, n_sites, derrida_c) for w in disorders]
check_true(
    f"CPU regimes match expected: {cpu_regimes}",
    cpu_regimes == expected_regimes,
)

# Check 2: Quantization round-trip error
max_err = 0.0
for w in disorders:
    features = quantize_features(w, energy, n_sites, model)
    w_deq = dequantize_i8(features[0], *model["quantization"]["W_range"])
    err = abs(w - w_deq) / max(abs(w), 1e-10)
    max_err = max(max_err, err)
tol = expected["quantization_roundtrip_max_error"]
check_approx("Quantization roundtrip max error", max_err, 0.0, tol)

# Check 3: Training produces 3x3 weight matrix
rng = np.random.default_rng(model.get("seed", 42))
n_train = model["n_training_disorders"]
w_min, w_max = model["training_W_min"], model["training_W_max"]
train_disorders = rng.uniform(w_min, w_max, n_train)
weights = train_centroid_classifier(train_disorders, n_sites, model, derrida_c)
check_true(
    f"Classifier weight matrix shape: {weights.shape}",
    weights.shape == (3, 3),
)

# Check 4: CPU classifier accuracy on training data
correct = 0
for w in train_disorders:
    features = quantize_features(w, energy, n_sites, model)
    pred_idx = classify_with_weights(features, weights)
    true_label = classify_regime(w, energy, n_sites, derrida_c)
    pred_label = ["Localized", "Critical", "Extended"][pred_idx]
    if pred_label == true_label:
        correct += 1
accuracy = correct / len(train_disorders)
check_true(
    f"CPU accuracy >= {expected['cpu_accuracy_min']:.0%}: {accuracy:.2%}",
    accuracy >= expected["cpu_accuracy_min"],
)

# Check 5: All three regime classes covered
unique_regimes = set(cpu_regimes)
check_true(
    f"Regime coverage >= {expected['regime_coverage_min']} classes",
    len(unique_regimes) >= expected["regime_coverage_min"],
)

# Check 6: Extended detected for weak disorder
check_true(
    "Extended for W=0.1",
    classify_regime(0.1, energy, n_sites, derrida_c) == "Extended",
)

# Check 7: Localized detected for strong disorder
check_true(
    "Localized for W=10",
    classify_regime(10.0, energy, n_sites, derrida_c) == "Localized",
)

# Results: {pass_count()}/{total_count()} checks passed
print_summary("Exp 028: NPU Anderson Regime Classification")
```

## Visualization


```python
# Publication-grade summary chart for Exp 028
fig, ax = plt.subplots(figsize=(8, 4))

p, f_count, t = pass_count(), fail_count(), total_count()
ax.barh(['Pass', 'Fail'], [p, f_count], color=[PASS_COLOR, FAIL_COLOR])
ax.set_xlim(0, max(t * 1.15, 1))
ax.set_title('Exp 028: NPU Anderson Classification — Validation Results')
ax.set_xlabel('Check Count')
for i, v in enumerate([p, f_count]):
    if v > 0:
        ax.text(v + 0.3, i, str(v), va='center', fontweight='bold')

plt.tight_layout()
plt.savefig(f'/tmp/groundspring_exp028.png', dpi=150, bbox_inches='tight')
plt.show()
print(f'\nResult: {p}/{t} PASS, {f_count}/{t} FAIL')
```

## Provenance & Summary

| Field | Value |
|-------|-------|
| Experiment | 028 — NPU Anderson Classification |
| Domain | Neuromorphic |
| Reference | BrainChip AKD1000 Anderson classification |
| Faculty | metalForge (NPU) |
| Python baseline | `control/npu_anderson/npu_anderson.py` |
| Benchmark JSON | `control/npu_anderson/benchmark_npu_anderson.json` |
| Rust validator | `validate_*` binary (exit-code protocol) |
| Rust speedup | See benchmark comparison notebook |
| License | AGPL-3.0-or-later |

**Provenance chain**: Python baseline → Rust validation → barraCuda GPU → metalForge cross-substrate → primal IPC composition

See [primals.eco](https://primals.eco) for rendered lab notebooks.


