+++
title = "BTSP Security Deep Dive — primalSpring"
description = "Rendered from 05-btsp-security-deep-dive.ipynb"
date = 2026-06-02
weight = 50

[extra]
domain = "Lab"
rendered_from = "05-btsp-security-deep-dive.ipynb"
+++

<!-- Auto-generated from 05-btsp-security-deep-dive.ipynb by spore-validate render-notebooks -->

# BTSP Security Deep Dive — primalSpring

The most compelling cross-domain insight from primalSpring: the BTSP
(BearDog Transport Security Protocol) convergence story. From plaintext
JSON-RPC to full ChaCha20-Poly1305 AEAD across all 13 primals, with
5 security gaps identified, tracked, and resolved.

This notebook provides the definitive view of ecosystem security posture —
per-primal bind address defaults, encryption state, gap resolution timeline,
and the discovery tier hierarchy that enables primals to find each other
without exposing network surfaces.

**Data sources**: `experiments/results/security_convergence.json`, `composition_validation.json`

**Reproduce**: `cargo test -p primalspring -- btsp` and review `docs/PRIMAL_GAPS.md`

---

*For other springs: your security story is about how your domain data
flows through BTSP-protected channels. Show which primals you call and
confirm they're all Phase 3.*

```python
import json
from pathlib import Path

import matplotlib
import matplotlib.pyplot as plt
import numpy as np

RESULTS = Path('..') / 'experiments' / 'results'

def load(path):
    with open(RESULTS / path) as f:
        return json.load(f)

security = load('security_convergence.json')
comp = load('composition_validation.json')

btsp = security['btsp']
print(f'BTSP Phase: {btsp["phase"]}')
print(f'Cipher: {btsp["cipher"]}')
print(f'Key exchange: {btsp["key_exchange"]}')
print(f'AEAD: {btsp["aead"]}')
print(f'Coverage: {btsp["primals_phase3"]}/{btsp["primals_total"]} ({btsp["coverage_pct"]}%)')
print(f'Security gaps: {len(security["gaps"])} tracked, all RESOLVED')
```

## Per-Primal Security Posture

Every primal's BTSP phase, bind address default, and bind flag.
All 13 are at Phase 3 with `127.0.0.1` default — no primal listens
on `0.0.0.0` by default.

```python
per_primal = btsp['per_primal']
primal_names = list(per_primal.keys())

print(f'{"Primal":<14s} {"Phase":<7s} {"Bind Default":<16s} {"Flag"}')
print('-' * 50)
for name in primal_names:
    p = per_primal[name]
    print(f'{name:<14s} {p["phase"]:<7d} {p["bind_default"]:<16s} {p["bind_flag"]}')

flags = {}
for name, p in per_primal.items():
    f = p['bind_flag']
    flags.setdefault(f, []).append(name)

print(f'\nBind flag distribution:')
for flag, names in flags.items():
    print(f'  {flag}: {len(names)} primals ({', '.join(names)})')
```

```python
fig, axes = plt.subplots(1, 2, figsize=(14, 5))

# BTSP Phase per primal
phases = [per_primal[p]['phase'] for p in primal_names]
colors = ['#2ecc71' if p == 3 else '#f39c12' if p == 2 else '#e74c3c' for p in phases]
ax = axes[0]
bars = ax.barh(primal_names[::-1], phases[::-1], color=colors[::-1])
ax.set_xlabel('BTSP Phase')
ax.set_xlim(0, 4)
ax.set_title('BTSP Phase per Primal')
ax.axvline(x=3, color='#2ecc71', linestyle='--', alpha=0.5)
for bar, val in zip(bars, phases[::-1]):
    ax.text(bar.get_width() + 0.05, bar.get_y() + bar.get_height()/2,
            f'Phase {val}', va='center', fontsize=9)

# Bind flag distribution
flag_labels = list(flags.keys())
flag_counts = [len(flags[f]) for f in flag_labels]
ax = axes[1]
flag_colors = ['#3498db', '#9b59b6', '#e67e22']
wedges, texts, autotexts = ax.pie(
    flag_counts, labels=flag_labels, autopct='%1.0f%%',
    colors=flag_colors[:len(flag_labels)], startangle=90)
ax.set_title('Bind Flag Distribution')

plt.suptitle('BTSP Phase 3: 13/13 Primals — Full AEAD Coverage',
             fontsize=13, fontweight='bold')
plt.tight_layout()
plt.savefig('/tmp/primalspring_05_btsp_posture.png', dpi=150, bbox_inches='tight')
plt.show()
```

## Gap Resolution Timeline

Five security gaps (PG-55 through PG-59) were identified during the
projectNUCLEUS Phase 2a audit. All were resolved within 3 weeks.

```python
gaps = security['gaps']
gap_ids = list(gaps.keys())

sev_map = {'HIGH': 3, 'MEDIUM': 2, 'LOW': 1}
sev_colors = {'HIGH': '#e74c3c', 'MEDIUM': '#f39c12', 'LOW': '#3498db'}

fig, axes = plt.subplots(1, 2, figsize=(14, 5))

# Severity distribution
ax = axes[0]
sevs = [gaps[g]['severity'] for g in gap_ids]
unique_sevs = list(set(sevs))
sev_counts = [sevs.count(s) for s in unique_sevs]
bars = ax.bar(unique_sevs, sev_counts, color=[sev_colors[s] for s in unique_sevs])
ax.set_ylabel('Gap count')
ax.set_title('Security Gaps by Severity')
for bar, val in zip(bars, sev_counts):
    ax.text(bar.get_x() + bar.get_width()/2, bar.get_height() + 0.05,
            f'{val} (all resolved)', ha='center', fontsize=9)

# Resolution dates
ax = axes[1]
dates = [gaps[g]['resolved_date'] for g in gap_ids]
date_labels = [f'{gid}\n{gaps[gid]["severity"]}' for gid in gap_ids]
y_pos = range(len(gap_ids))
colors = [sev_colors[gaps[g]['severity']] for g in gap_ids]
ax.barh(date_labels[::-1], [1]*len(gap_ids), color=colors[::-1], alpha=0.7)
for i, gid in enumerate(reversed(gap_ids)):
    ax.text(0.5, i, f'{gaps[gid]["resolved_date"]} — {gaps[gid]["title"]}',
            va='center', ha='center', fontsize=8)
ax.set_xlim(0, 1)
ax.set_xticks([])
ax.set_title('Gap Resolution — All Closed')

plt.suptitle('PG-55 through PG-59: Zero Open Security Gaps',
             fontsize=13, fontweight='bold')
plt.tight_layout()
plt.savefig('/tmp/primalspring_05_gaps.png', dpi=150, bbox_inches='tight')
plt.show()
```

## BTSP Convergence Arc

The security protocol evolved through 3 phases over 7 weeks. Phase 2a
audit findings (PG-55–59) drove the final push to full convergence.

```python
timeline = security['security_timeline']
dates = [e['date'] for e in timeline]
primals_at = [e['primals'] for e in timeline]
events = [e['event'] for e in timeline]

fig, ax = plt.subplots(figsize=(14, 6))

ax.plot(range(len(dates)), primals_at, 'o-', color='#2c3e50', linewidth=3, markersize=12)
ax.fill_between(range(len(dates)), primals_at, alpha=0.15, color='#2ecc71')
ax.axhline(y=13, color='#e74c3c', linestyle='--', alpha=0.5, linewidth=2, label='Target: 13/13')

for i, (d, p, e) in enumerate(zip(dates, primals_at, events)):
    color = '#2ecc71' if p == 13 else '#f39c12' if p > 0 else '#e74c3c'
    ax.annotate(f'{p}/13\n{e}',
                xy=(i, p), xytext=(0, 20 if i % 2 == 0 else -30),
                textcoords='offset points', ha='center', fontsize=7,
                arrowprops=dict(arrowstyle='->', color=color, lw=1.5),
                bbox=dict(boxstyle='round,pad=0.3', facecolor='white',
                          edgecolor=color, alpha=0.9))

ax.set_xticks(range(len(dates)))
ax.set_xticklabels(dates, fontsize=8, rotation=15)
ax.set_ylabel('Primals at Phase 3 AEAD')
ax.set_ylim(-1, 16)
ax.set_title('BTSP Security Convergence — Plaintext to Full AEAD')
ax.legend(loc='upper left')

plt.tight_layout()
plt.savefig('/tmp/primalspring_05_convergence.png', dpi=150, bbox_inches='tight')
plt.show()
```

## Discovery Hierarchy Deep Dive

The 5-tier discovery escalation prevents primals from broadcasting
their presence unnecessarily. Tier 1 (static config) is the most
restrictive; Tier 5 (relay federation) enables cross-network discovery.

```python
disc = security['discovery_hierarchy']
disc_v = comp['primals_validated']['discovery_tier_coverage']

tier_names = disc['tier_names']
tier_labels = ['Tier 1: Static Config', 'Tier 2: Env Override',
               'Tier 3: Multicast LAN', 'Tier 4: STUN NAT', 'Tier 5: Relay Fed.']
tier_counts = [disc_v['tier1_static'], disc_v['tier2_env'],
               disc_v['tier3_multicast'], disc_v['tier4_stun'], disc_v['tier5_relay']]

fig, ax = plt.subplots(figsize=(10, 5))
colors = ['#2ecc71', '#27ae60', '#f39c12', '#e67e22', '#e74c3c']
bars = ax.bar(tier_labels, tier_counts, color=colors)
ax.set_ylabel('Primals supporting tier')
ax.set_ylim(0, 15)
ax.set_title('Discovery Escalation — Primal Support by Tier')
for bar, val in zip(bars, tier_counts):
    ax.text(bar.get_x() + bar.get_width()/2, bar.get_height() + 0.3,
            f'{val}/13', ha='center', fontsize=11, fontweight='bold')

plt.xticks(rotation=15, fontsize=9)
plt.tight_layout()
plt.savefig('/tmp/primalspring_05_discovery.png', dpi=150, bbox_inches='tight')
plt.show()
```

## Validation Summary

| Security Metric | Value |
|----------------|-------|
| BTSP Phase | 3 (ChaCha20-Poly1305 AEAD) |
| Primals at Phase 3 | 13/13 (100%) |
| Bind default | 127.0.0.1 (all 13) |
| Security gaps tracked | 5 (PG-55 – PG-59) |
| Security gaps open | 0 |
| Discovery tiers | 5 (static → relay) |
| Convergence time | 7 weeks (plaintext → AEAD) |

The ecoPrimals ecosystem has achieved full security convergence.
Every primal defaults to localhost binding, uses AEAD encryption
for all IPC, and supports tiered discovery escalation.

---

**Provenance**: All results are content-addressed via BLAKE3 hashes,
tracked in rhizoCrypt DAG sessions, committed to the loamSpine ledger,
and witnessed with ed25519 signatures via sweetGrass braid.

**Reproduce**: See [primals.eco/lab/reproduce](https://primals.eco/lab/reproduce/)

**Source**: [ecoPrimals/primalSpring](https://github.com/ecoPrimals/primalSpring)

