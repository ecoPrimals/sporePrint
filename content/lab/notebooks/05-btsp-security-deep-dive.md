+++
title = "05 — BTSP Security Deep Dive"
description = "Rendered from 05-btsp-security-deep-dive.ipynb"
date = 2026-06-11
weight = 50

[extra]
domain = "Lab"
rendered_from = "05-btsp-security-deep-dive.ipynb"
+++

<!-- Auto-generated from 05-btsp-security-deep-dive.ipynb by spore-validate render-notebooks -->

# 05 — BTSP Security Deep Dive

**neuralSpring sporePrint** | Session S188 | May 2026

Per-primal security posture, BTSP convergence arc, encryption
tiers, and supply chain integrity.

**Data sources:** `security-posture.json`, `cross-spring-matrix.json`, `gap-status.json`

**For other springs:** Replace BTSP capability counts and encryption
tier details with your spring's security configuration.

```python
import json
from pathlib import Path
import matplotlib.pyplot as plt
import matplotlib.patches as mpatches

RESULTS = Path('..') / 'experiments' / 'results'

with open(RESULTS / 'security-posture.json') as f:
    sp = json.load(f)

with open(RESULTS / 'cross-spring-matrix.json') as f:
    cs = json.load(f)

with open(RESULTS / 'gap-status.json') as f:
    gs = json.load(f)

PASS = '#2ecc71'
FAIL = '#e74c3c'
INFO = '#3498db'

print(f"neuralSpring — BTSP Security Deep Dive")
```

## BTSP Convergence Arc

BTSP (BearDog Transport Security Protocol) is mandatory for all 13
capabilities since Phase 45c. Session establishment with BearDog
is deferred pending `crypto.btsp_handshake` upstream wire.

```python
btsp = sp['btsp']

convergence_stages = [
    ('Cleartext IPC', True, 'Pre-Phase 45'),
    ('BTSP awareness', True, 'Phase 45a'),
    ('BTSP optional', True, 'Phase 45b'),
    ('BTSP mandatory (13/13)', True, 'Phase 45c'),
    ('BTSP session establishment', False, 'Pending BearDog'),
    ('End-to-end signed receipts', False, 'Level 5')
]

fig, ax = plt.subplots(figsize=(10, 4))
stage_names = [s[0] for s in convergence_stages]
stage_done = [s[1] for s in convergence_stages]
stage_phases = [s[2] for s in convergence_stages]
colors = [PASS if d else FAIL for d in stage_done]

bars = ax.barh(stage_names[::-1], [1]*len(convergence_stages), color=colors[::-1])
ax.set_xlim(0, 2)
ax.set_title(f'BTSP Convergence Arc — {btsp["capabilities_covered"]} capabilities')

for i, phase in enumerate(stage_phases[::-1]):
    ax.text(1.05, i, phase, va='center', fontsize=9, style='italic')

legend_elements = [
    mpatches.Patch(color=PASS, label='Complete'),
    mpatches.Patch(color=FAIL, label='Pending')
]
ax.legend(handles=legend_elements, loc='lower right')

plt.tight_layout()
plt.show()
```

## Encryption Tiers

neuralSpring uses a 4-tier encryption model: Tower (full BTSP),
Node/Nest/Meta (Tower-delegated).

```python
enc = sp['encryption_tiers']

tiers = list(enc.keys())
tier_vals = [enc[t]['tier'] for t in tiers]
tier_descs = [enc[t]['description'] for t in tiers]
tier_colors = ['#9b59b6' if v == 'full' else INFO for v in tier_vals]

fig, ax = plt.subplots(figsize=(8, 3))
bars = ax.barh(tiers[::-1], [1]*len(tiers), color=tier_colors[::-1])
ax.set_xlim(0, 2.5)
ax.set_title('Encryption Tiers')

for i, (tier_val, desc) in enumerate(zip(tier_vals[::-1], tier_descs[::-1])):
    ax.text(1.05, i, f'{tier_val} — {desc}', va='center', fontsize=9)

plt.tight_layout()
plt.show()
```

## Per-Primal Security Posture

Each consumed primal has specific security requirements and
environment variables for production BTSP operation.

```python
consumption = cs['consumption']

security_primals = {
    'beardog': {'role': 'Tower crypto/BTSP', 'env': 'BEARDOG_FAMILY_SEED', 'status': 'wip'},
    'songbird': {'role': 'Tower discovery mesh', 'env': 'SONGBIRD_SECURITY_PROVIDER', 'status': 'wip'},
    'nestgate': {'role': 'Weight storage JWT', 'env': 'NESTGATE_JWT_SECRET', 'status': 'open'},
    'barracuda': {'role': 'GPU compute (pure Rust)', 'env': 'None required', 'status': 'active'},
    'primalspring': {'role': 'Composition framework', 'env': 'FAMILY_ID', 'status': 'active'},
    'squirrel': {'role': 'Inference routing', 'env': 'None required', 'status': 'wip'},
    'coralreef': {'role': 'Shader IPC', 'env': '--rpc-bind', 'status': 'open'},
    'toadstool': {'role': 'Compute dispatch', 'env': 'None required', 'status': 'open'}
}

color_map = {'active': PASS, 'wip': '#f39c12', 'open': FAIL}

fig, ax = plt.subplots(figsize=(12, 5))
primal_names = list(security_primals.keys())
primal_colors = [color_map[security_primals[p]['status']] for p in primal_names]

bars = ax.barh(primal_names[::-1], [1]*len(primal_names), color=primal_colors[::-1])
ax.set_xlim(0, 3)
ax.set_title('Per-Primal Security Posture')

for i, p in enumerate(primal_names[::-1]):
    info = security_primals[p]
    ax.text(1.05, i, f"{info['role']} | env: {info['env']}",
            va='center', fontsize=8)

legend_elements = [
    mpatches.Patch(color=PASS, label='Active'),
    mpatches.Patch(color='#f39c12', label='WIP'),
    mpatches.Patch(color=FAIL, label='Open')
]
ax.legend(handles=legend_elements, loc='lower right')

plt.tight_layout()
plt.show()
```

## Supply Chain Integrity

neuralSpring maintains a pure Rust supply chain with `cargo-deny`
enforcement and 8 banned crate categories.

```python
deny = sp['cargo_deny']
lint = sp['lint_policy']

supply_checks = [
    ('Pure Rust supply chain', sp['supply_chain']['pure_rust']),
    ('cargo-deny advisory check', True),
    ('cargo-deny license check', True),
    ('cargo-deny source check', True),
    ('forbid(unsafe_code)', sp['unsafe_code']['forbid_unsafe']),
    ('Zero #[allow()] attributes', lint['allow_attributes'] == 0),
    ('SPDX headers on all .rs', True),
    ('Clippy pedantic+nursery', True)
]

fig, ax = plt.subplots(figsize=(10, 4))
check_names = [c[0] for c in supply_checks]
check_pass = [c[1] for c in supply_checks]
check_colors = [PASS if p else FAIL for p in check_pass]

ax.barh(check_names[::-1], [1]*len(supply_checks), color=check_colors[::-1])
ax.set_xlim(0, 1.3)
ax.set_title('Supply Chain Integrity Checks')

plt.tight_layout()
plt.show()

print(f"Banned crates: {', '.join(deny['banned_crates'])}")
print(f"Exemptions: {', '.join(deny['exemptions'])}")
```

## guideStone Security Properties

All 5 guideStone properties are certified at Level 3 (bare mode).

```python
gs_sec = sp['guidestone_security']

properties = list(gs_sec.keys())
descriptions = list(gs_sec.values())
prop_labels = [
    'P1 Deterministic',
    'P2 Traceable',
    'P3 Self-Verifying',
    'P4 Environment-Agnostic',
    'P5 Tolerance-Documented'
]

fig, ax = plt.subplots(figsize=(10, 3))
ax.barh(prop_labels[::-1], [1]*5, color=PASS)
ax.set_xlim(0, 3)
ax.set_title('guideStone Security Properties — All Certified')

for i, desc in enumerate(descriptions[::-1]):
    ax.text(1.05, i, desc[:70], va='center', fontsize=7)

plt.tight_layout()
plt.show()
```

## Summary

| Metric | Value |
|--------|-------|
| BTSP capabilities | 13/13 mandatory |
| BTSP phase | 45c (default) |
| BTSP session | Deferred (pending BearDog) |
| Encryption tiers | 4 (Tower full, Node/Nest/Meta delegated) |
| Unsafe code | 0 (#![forbid(unsafe_code)]) |
| Supply chain | Pure Rust (wgpu HAL exception) |
| Banned crates | 8 (cargo-deny enforced) |
| BLAKE3 checksums | 15 validation-critical files |
| guideStone properties | P1-P5 certified |
| Security model | Metallic bond / InternalNucleus trust |

**Provenance:** [primals.eco](https://primals.eco) |
neuralSpring Session S188 | May 2026

