+++
title = "03 — Ecosystem Evidence"
description = "Rendered from 03-ecosystem-evidence.ipynb"
date = 2026-06-11
weight = 50

[extra]
domain = "Lab"
rendered_from = "03-ecosystem-evidence.ipynb"
+++

<!-- Auto-generated from 03-ecosystem-evidence.ipynb by spore-validate render-notebooks -->

# 03 — Ecosystem Evidence

**neuralSpring sporePrint** | Session S188 | May 2026

134 experiments across 11 domains, gap resolution timeline,
and security posture evolution.

**Data sources:** `experiment-catalog.json`, `gap-status.json`, `security-posture.json`

**For other springs:** Replace experiment catalog and gap data with your
own. Security posture structure is shared across all springs.

```python
import json
from pathlib import Path
import matplotlib.pyplot as plt
import matplotlib.patches as mpatches

RESULTS = Path('..') / 'experiments' / 'results'

with open(RESULTS / 'experiment-catalog.json') as f:
    ec = json.load(f)

with open(RESULTS / 'gap-status.json') as f:
    gs = json.load(f)

with open(RESULTS / 'security-posture.json') as f:
    sp = json.load(f)

PASS = '#2ecc71'
FAIL = '#e74c3c'
INFO = '#3498db'

print(f"neuralSpring — {ec['total_experiments']} experiments, {len(ec['domains'])} domains")
```

## Experiment Timeline

134 experiments organized into 8 milestone bands, from foundation
Python baselines through guideStone Level 3 maturity.

```python
milestones = ec['milestone_experiments']

fig, ax = plt.subplots(figsize=(12, 5))
labels = [m['id'] for m in milestones]
scopes = [m['scope'] for m in milestones]

# Extract approximate experiment counts from ranges
counts = [10, 17, 23, 30, 20, 20, 10, 4]
colors_list = [INFO, PASS, '#f39c12', '#9b59b6', '#e67e22', '#1abc9c', '#34495e', PASS]

bars = ax.barh(labels[::-1], counts[::-1], color=colors_list[::-1])
ax.set_xlabel('Experiments')
ax.set_title(f'Experiment Timeline ({ec["total_experiments"]} total)')

for i, (bar, scope) in enumerate(zip(bars, scopes[::-1])):
    ax.text(bar.get_width() + 0.5, bar.get_y() + bar.get_height()/2,
            scope[:50], va='center', fontsize=8)

plt.tight_layout()
plt.show()
```

## Faculty Contributions

27 peer-reviewed papers across 6 faculties provide the scientific
foundation for neuralSpring's validation chain.

```python
faculties = ec['faculties']

fig, axes = plt.subplots(1, 2, figsize=(12, 5))

# Papers per faculty
fnames = [f['name'] for f in faculties]
fpapers = [len(f['papers']) for f in faculties]
axes[0].barh(fnames[::-1], fpapers[::-1], color=INFO)
axes[0].set_xlabel('Papers')
axes[0].set_title('Papers per Faculty')
for i, (bar_val, f) in enumerate(zip(fpapers[::-1], faculties[::-1])):
    axes[0].text(bar_val + 0.1, i, f['institution'], va='center', fontsize=8)

# Checks per faculty
fchecks = [f['checks'] for f in faculties]
axes[1].barh(fnames[::-1], fchecks[::-1], color=PASS)
axes[1].set_xlabel('Validation Checks')
axes[1].set_title('Checks per Faculty')
for i, v in enumerate(fchecks[::-1]):
    axes[1].text(v + 0.5, i, str(v), va='center', fontweight='bold')

plt.tight_layout()
plt.show()
```

## Gap Resolution

14 main gaps tracked in `PRIMAL_GAPS.md`, with 13 historically resolved
gaps in the appendix and 5 composition evolution items implemented.

```python
summary = gs['summary']

status_counts = {
    'resolved': summary['resolved_main'],
    'implemented': summary['implemented'],
    'wip': summary['wip'],
    'open': summary['open'],
    'deferred': summary['deferred'],
    'tracking': summary['tracking'],
    'explored': summary['explored'],
    'partial': summary['partial']
}

status_colors = {
    'resolved': PASS, 'implemented': PASS,
    'wip': '#f39c12', 'open': FAIL,
    'deferred': '#95a5a6', 'tracking': INFO,
    'explored': '#9b59b6', 'partial': '#e67e22'
}

fig, axes = plt.subplots(1, 2, figsize=(12, 4))

# Main gaps by status
labels = list(status_counts.keys())
vals = list(status_counts.values())
cols = [status_colors[s] for s in labels]
axes[0].bar(labels, vals, color=cols)
axes[0].set_title(f'Main Gaps by Status ({summary["total_main_gaps"]} total)')
axes[0].tick_params(axis='x', rotation=45)
for i, v in enumerate(vals):
    if v > 0:
        axes[0].text(i, v + 0.1, str(v), ha='center', fontweight='bold')

# Historical resolution
hist = ['Resolved (appendix)', 'Composition evolution', 'Main resolved']
hist_vals = [summary['resolved_appendix'], summary['composition_evolution'], summary['resolved_main']]
axes[1].bar(hist, hist_vals, color=[PASS, '#1abc9c', PASS])
axes[1].set_title('Resolved Gap History')
for i, v in enumerate(hist_vals):
    axes[1].text(i, v + 0.2, str(v), ha='center', fontweight='bold')

plt.tight_layout()
plt.show()
```

## Security Posture Timeline

The security posture has evolved from basic Rust safety through
BTSP mandatory encryption and BLAKE3 checksum verification.

```python
security_milestones = [
    ('forbid(unsafe_code)', True),
    ('cargo-deny enforcement', True),
    ('Zero #[allow()]', True),
    ('BLAKE3 checksums (15 files)', True),
    ('BTSP 13/13 default', True),
    ('Stadial deny bans (8 crates)', True),
    ('SPDX headers (all .rs)', True),
    ('Pure Rust supply chain', True),
    ('BTSP session establishment', False),
    ('Level 4 NUCLEUS certified', False)
]

fig, ax = plt.subplots(figsize=(10, 4))
names = [m[0] for m in security_milestones]
colors = [PASS if m[1] else FAIL for m in security_milestones]
ax.barh(names[::-1], [1]*len(names), color=colors[::-1])
ax.set_xlim(0, 1.3)
ax.set_title('Security Posture Evolution')

legend_elements = [
    mpatches.Patch(color=PASS, label='Complete'),
    mpatches.Patch(color=FAIL, label='Pending')
]
ax.legend(handles=legend_elements, loc='lower right')

plt.tight_layout()
plt.show()
```

## Summary

| Metric | Value |
|--------|-------|
| Experiments | 134 across 11 domains |
| Papers reproduced | 27 (6 faculties) |
| Main gaps | 14 (2 resolved, 2 wip, 5 open) |
| Resolved gaps (appendix) | 13 |
| Composition evolution | 5 implemented |
| BTSP | 13/13 mandatory |
| Unsafe code | 0 (forbid workspace-wide) |
| Supply chain | Pure Rust |
| BLAKE3 checksums | 15 files |

**Provenance:** [primals.eco](https://primals.eco) |
neuralSpring Session S188 | May 2026

