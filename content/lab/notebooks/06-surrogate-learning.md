+++
title = "Surrogate Learning — Directed Sampling for Nuclear EOS"
description = "Rendered from 06-surrogate-learning.ipynb"
date = 2026-06-04
weight = 50

[extra]
domain = "Lab"
rendered_from = "06-surrogate-learning.ipynb"
+++

<!-- Auto-generated from 06-surrogate-learning.ipynb by spore-validate render-notebooks -->

# Surrogate Learning — Directed Sampling for Nuclear EOS

**Paper:** Diaw, McKerns, Sagert, Stanton, Murillo, *Nature Machine Intelligence* (2024)  
**What we reproduce:** Optimizer-driven directed sampling for training accurate surrogates
of complex simulation codes. We demonstrate the key insight: instead of random or grid
sampling, use optimization (mystic) to select training points that maximize surrogate
accuracy. Applied to Skyrme nuclear EOS, this achieves 10x fewer evaluations vs random.

---

*Live: demonstrate the sampling strategy comparison on a 2D test function.*  
*The full nuclear EOS workflow requires Zenodo data + mystic optimizer.*  
*Rust parity:* `barracuda/src/nuclear_eos.rs` — the physics that surrogates approximate.*

## Physics Background

Nuclear EOS codes (Skyrme HF/HFB) are expensive: seconds to minutes per nucleus.
Training a surrogate requires evaluating the code at many parameter combinations.

**Random sampling** wastes evaluations in flat regions.  
**Grid sampling** scales exponentially with dimension.  
**Directed sampling** (Diaw et al.) uses optimization to select points
that maximize information gain — concentrating evaluations where the
function varies most.

The Skyrme EDF has 10 parameters ($t_0, t_1, t_2, t_3, x_0, x_1, x_2, x_3, \alpha, W_0$).
A surrogate trained with 100 directed samples can match one trained with 1,000 random samples.

```python
import numpy as np
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
import time

C_PASS = '#2ecc71'
C_FAIL = '#e74c3c'
C_INFO = '#3498db'
C_RUST = '#1abc9c'
C_PYTHON = '#f39c12'
C_GPU = '#9b59b6'

# 2D test function: Rastrigin-like (cheap proxy for expensive EOS)
def test_function(x, y):
    return (x**2 + y**2 - 10*(np.cos(2*np.pi*x) + np.cos(2*np.pi*y)) + 20)

# Ground truth on fine grid
x_fine = np.linspace(-3, 3, 200)
y_fine = np.linspace(-3, 3, 200)
X, Y = np.meshgrid(x_fine, y_fine)
Z_true = test_function(X, Y)

print(f"Test function: modified Rastrigin on [-3, 3]^2")
print(f"Ground truth: {X.shape[0]}x{X.shape[1]} = {X.size} points")
```

```python
from scipy.interpolate import RBFInterpolator

def surrogate_error(x_train, y_train, z_train, X_test, Y_test, Z_test):
    """Train RBF surrogate and compute RMSE on test grid."""
    points = np.column_stack([x_train, y_train])
    rbf = RBFInterpolator(points, z_train, kernel='thin_plate_spline')
    test_pts = np.column_stack([X_test.ravel(), Y_test.ravel()])
    Z_pred = rbf(test_pts).reshape(X_test.shape)
    rmse = np.sqrt(np.mean((Z_pred - Z_test)**2))
    return rmse, Z_pred

n_samples_list = [20, 50, 100, 200]
rng = np.random.default_rng(42)

random_errors = []
directed_errors = []

for n_s in n_samples_list:
    # Random sampling
    xr = rng.uniform(-3, 3, n_s)
    yr = rng.uniform(-3, 3, n_s)
    zr = test_function(xr, yr)
    rmse_r, _ = surrogate_error(xr, yr, zr, X, Y, Z_true)
    random_errors.append(rmse_r)

    # Directed sampling: concentrate near function extrema
    # (simplified version of optimizer-driven selection)
    n_coarse = n_s // 3
    n_refine = n_s - n_coarse
    xc = rng.uniform(-3, 3, n_coarse)
    yc = rng.uniform(-3, 3, n_coarse)
    zc = test_function(xc, yc)

    # Find high-gradient regions from coarse samples
    grad_x = np.gradient(zc)
    high_grad_idx = np.argsort(np.abs(grad_x))[-n_coarse//2:]

    # Refine near high-gradient points
    xd = list(xc)
    yd = list(yc)
    for _ in range(n_refine):
        idx = rng.choice(high_grad_idx)
        xd.append(xc[idx] + rng.normal(0, 0.3))
        yd.append(yc[idx] + rng.normal(0, 0.3))
    xd = np.clip(xd, -3, 3)
    yd = np.clip(yd, -3, 3)
    zd = test_function(np.array(xd), np.array(yd))
    rmse_d, _ = surrogate_error(np.array(xd), np.array(yd), zd, X, Y, Z_true)
    directed_errors.append(rmse_d)

    print(f"  N={n_s:>4d}: Random RMSE={rmse_r:.3f}, Directed RMSE={rmse_d:.3f}, "
          f"improvement={rmse_r/max(rmse_d,1e-10):.1f}x")
```

```python
fig, axes = plt.subplots(1, 3, figsize=(18, 5))

# Ground truth
im = axes[0].contourf(X, Y, Z_true, levels=30, cmap='viridis')
axes[0].set_xlabel('$x$')
axes[0].set_ylabel('$y$')
axes[0].set_title('Ground Truth (Rastrigin-like)')
plt.colorbar(im, ax=axes[0])

# Random vs directed samples (N=100)
n_demo = 100
xr = rng.uniform(-3, 3, n_demo)
yr = rng.uniform(-3, 3, n_demo)
axes[1].contourf(X, Y, Z_true, levels=20, cmap='viridis', alpha=0.3)
axes[1].scatter(xr, yr, s=15, color=C_PYTHON, label='Random', alpha=0.7)
axes[1].set_xlabel('$x$')
axes[1].set_ylabel('$y$')
axes[1].set_title('Random Sampling (N=100)')
axes[1].legend()

# Convergence comparison
axes[2].semilogy(n_samples_list, random_errors, 'o-', color=C_PYTHON, markersize=8,
                 linewidth=2, label='Random')
axes[2].semilogy(n_samples_list, directed_errors, 's-', color=C_RUST, markersize=8,
                 linewidth=2, label='Directed')
axes[2].set_xlabel('Number of samples')
axes[2].set_ylabel('RMSE')
axes[2].set_title('Surrogate Accuracy vs Sample Count')
axes[2].legend()
axes[2].grid(True, alpha=0.3)

fig.suptitle('Surrogate Learning — Diaw et al. (2024)', fontsize=14, fontweight='bold')
plt.tight_layout()
plt.savefig('/tmp/hotspring_paper_06_surrogate.png', dpi=150, bbox_inches='tight')
plt.show()
```

## Connection to Nuclear EOS

In the full workflow (requires Zenodo data + Code Ocean capsule):
1. The "expensive function" is the Skyrme HFB solver (seconds per nucleus)
2. The optimizer (mystic) selects Skyrme parameter combinations
3. The surrogate maps parameters → binding energies in microseconds
4. This enables parameter sweeps that would take weeks via direct computation

The 1,990 novel predictions from SEMF (notebook 01) serve as the baseline
that surrogates must reproduce.

## Rust Parity

The underlying nuclear physics is in `barracuda/src/nuclear_eos.rs`.
GPU-accelerated SEMF evaluation processes 2,042 nuclei in 0.8 ms.

## Provenance

- **Paper:** Diaw et al., Nature Machine Intelligence (2024)
- **Data:** Zenodo directed sampling datasets
- **Control:** `control/surrogate/scripts/run_reproduction.py`
- **Next:** Full 10D surrogate via primal composition + cluster dispatch

---
*hotSpring — ecoPrimals · AGPL-3.0 · [primals.eco](https://primals.eco)*

