+++
title = "Composition Validation — hotSpring"
description = "Rendered from 01-composition-validation.ipynb"
date = 2026-06-12
weight = 50

[extra]
domain = "Lab"
rendered_from = "01-composition-validation.ipynb"
+++

<!-- Auto-generated from 01-composition-validation.ipynb by spore-validate render-notebooks -->

# Composition Validation — hotSpring

hotSpring validates computational physics (lattice QCD, nuclear structure, plasma)
on consumer GPU hardware via the ecoPrimal NUCLEUS composition. This notebook shows
the deploy graph topology, guideStone Level 6 validation, and capability-based
primal routing.

**Data sources:** `composition_validation.json`, `test_suite_report.json`

**Reproduce:** `cargo test --lib` in `barracuda/`, then `scripts/validate-primal-proof.sh`

---

*For other springs:* Replace QCD domain content with your science. Keep the guideStone
property structure and atomic type hierarchy — they're universal across all springs.

```python
import json
from pathlib import Path
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
import matplotlib.ticker as ticker

RESULTS = Path('..') / 'experiments' / 'results'

def load(name):
    with open(RESULTS / name) as f:
        return json.load(f)

comp = load('composition_validation.json')
tests = load('test_suite_report.json')

print(f"Spring: {comp['spring']} v{comp['version']}")
print(f"guideStone Level: {comp['guidestone_level']}")
print(f"Deploy graph: {comp['deploy_graph']['total_nodes']} nodes, {comp['deploy_graph']['primals_required']} required primals")
print(f"Tests: {tests['total_tests']} passed, {tests['ignored']} ignored")
print(f"Bare guideStone: {comp['guidestone_bare']['passed']}/{comp['guidestone_bare']['total_checks']} checks")
```

## guideStone Level 6 — Five Properties

The `hotspring_guidestone` binary validates 5 guideStone properties in bare mode
(no primals needed) and adds NUCLEUS IPC parity checks when primals are deployed.
Property 3 (Self-Verifying) uses BLAKE3 checksums for 15 validation-critical source files.

```python
C_PASS = '#2ecc71'
C_FAIL = '#e74c3c'
C_INFO = '#3498db'
C_GPU  = '#9b59b6'

props = comp['guidestone_bare']['properties']
names = [k.replace('_', ' ') for k in props]
checks = [props[k]['checks'] for k in props]
colors = [C_PASS if props[k]['status'] == 'PASS' else C_FAIL for k in props]

fig, axes = plt.subplots(1, 2, figsize=(14, 5))

# guideStone property checks
axes[0].barh(names, checks, color=colors)
axes[0].set_xlabel('Checks')
axes[0].set_title(f'guideStone Bare: {comp["guidestone_bare"]["passed"]}/{comp["guidestone_bare"]["total_checks"]} PASS')
axes[0].invert_yaxis()

# Atomic type test coverage
atomics = comp['atomic_types']
at_names = list(atomics.keys())
at_tests = [atomics[k]['tests'] for k in at_names]
axes[1].bar(at_names, at_tests, color=[C_INFO, C_INFO, C_GPU, C_PASS])
axes[1].set_ylabel('Tests')
axes[1].set_title(f'Atomic Composition Tests ({sum(at_tests)} total)')

fig.suptitle(f'hotSpring v{comp["version"]} — guideStone Level {comp["guidestone_level"]} CERTIFIED', fontsize=14, fontweight='bold')
plt.tight_layout()
plt.savefig('/tmp/hotspring_01_composition.png', dpi=150, bbox_inches='tight')
plt.show()
```

## Capability-Based Routing

hotSpring routes to primals by **capability domain** (`by_domain("compute")`), not
by hardcoded process names. All requirements derive from `niche::DEPENDENCIES` —
a single source of truth. Named accessors are deprecated.

```python
fig, ax = plt.subplots(figsize=(10, 6))

primals = comp['primals_validated']['required']
p_names = [p['name'] for p in primals]
p_domains = [p['domain'] for p in primals]

domain_colors = {
    'crypto': '#e74c3c', 'discovery': '#f39c12', 'compute': '#9b59b6',
    'math': '#3498db', 'shader': '#1abc9c', 'storage': '#2ecc71',
    'dag': '#e67e22', 'ledger': '#34495e', 'provenance': '#16a085'
}
colors = [domain_colors.get(d, '#95a5a6') for d in p_domains]

bars = ax.barh(p_names, range(len(p_names), 0, -1), color=colors)
for i, p in enumerate(primals):
    ax.text(0.5, i, f'{p["domain"]} — {p["role"]}', va='center', fontsize=8, color='white', fontweight='bold')

ax.set_xlabel('Required Primals (ordered by composition dependency)')
ax.set_title(f'NUCLEUS Composition: {len(primals)} Required Primals, by_domain() Routing')
ax.set_xlim(0, len(primals) + 1)
ax.invert_yaxis()

cap = comp['capability_routing']
ax.text(0.02, 0.02, f'{cap["local_capabilities"]} local + {cap["routed_capabilities"]} routed capabilities',
        transform=ax.transAxes, fontsize=9, color='gray')

plt.tight_layout()
plt.savefig('/tmp/hotspring_01_routing.png', dpi=150, bbox_inches='tight')
plt.show()
```

## Test Suite by Physics Domain

596 (default) / 1,045 (barracuda-local) library tests organized by physics domain — from nuclear structure (SEMF, HFB)
through lattice QCD (HMC, RHMC, gradient flow) to GPU compute validation.

```python
cats = tests['key_categories']
cat_names = [k.replace('_', ' ').title() for k in cats]
cat_tests = [cats[k]['tests'] for k in cats]
cat_pcts = [cats[k]['pct'] for k in cats]

fig, axes = plt.subplots(1, 2, figsize=(14, 6))

# Pie chart
pie_colors = [C_PASS, C_INFO, C_GPU, '#1abc9c', '#f39c12', '#e67e22', '#e74c3c', '#95a5a6']
axes[0].pie(cat_tests, labels=cat_names, colors=pie_colors[:len(cat_names)],
            autopct='%1.0f%%', startangle=90, textprops={'fontsize': 8})
axes[0].set_title(f'{tests["total_tests"]} Library Tests by Domain')

# Module breakdown (top 12)
mods = tests['modules']
sorted_mods = sorted(mods.items(), key=lambda x: x[1]['tests'], reverse=True)[:12]
mod_names = [m[0] for m in sorted_mods]
mod_tests = [m[1]['tests'] for m in sorted_mods]
axes[1].barh(mod_names, mod_tests, color=C_INFO)
axes[1].set_xlabel('Tests')
axes[1].set_title('Top 12 Modules by Test Count')
axes[1].invert_yaxis()

fig.suptitle(f'hotSpring Test Suite: {tests["total_tests"]}/{tests["total_tests"]} PASS, {tests["ignored"]} ignored', fontsize=13, fontweight='bold')
plt.tight_layout()
plt.savefig('/tmp/hotspring_01_tests.png', dpi=150, bbox_inches='tight')
plt.show()
```

## Validation Summary

| Component | Status | Detail |
|-----------|--------|--------|
| guideStone Level 6 | **30/30 PASS** | 5 properties certified, BLAKE3 P3, 3 SKIP (liveness) |
| Library tests | **596/596 PASS** (default) / **1,045** (barracuda-local) | 6 GPU-heavy ignored (upstream barraCuda CI) |
| Validation suites | **65/65 PASS** | 167 `validate_*` binaries + `hotspring_guidestone` |
| NUCLEUS routing | **by_domain()** | Capability-based from `niche::DEPENDENCIES` |
| Deploy graph | **11 nodes** | 9 required + 1 optional + hotspring_unibin |

---

**Provenance:** All data from `experiments/results/` committed JSON artifacts.  
**Reproduce:** `cargo test --lib` in `barracuda/`, `scripts/validate-primal-proof.sh` from repo root.  
**Source:** [hotSpring on GitHub](https://github.com/syntheticChemistry/hotSpring) · [primals.eco](https://primals.eco/lab/springs/hotspring/)

