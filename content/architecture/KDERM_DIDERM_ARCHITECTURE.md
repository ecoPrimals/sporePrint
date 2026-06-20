+++
title = "K-Derm Diderm Architecture"
description = "The Gram-negative cell envelope model applied to sovereign infrastructure — inner membrane, peptidoglycan, outer membrane, and bond-mediated communication."
date = 2026-05-31
weight = 15

[extra]
domain = "Architecture"
+++

## Overview

The ecoPrimals deployment architecture follows the Gram-negative bacterial
cell envelope: a **diderm** (double-membrane) system with a structural
peptidoglycan layer between inner and outer membranes. This maps directly
to the three-VPS infrastructure topology that hosts sovereign services.

The model is not metaphorical — it drives real architecture decisions about
which services run where, which bonds mediate communication, and which
direction information flows.

## Interactive Topology

<div id="viz-kderm" data-viz-src="/viz/kderm-topology?format=scene-json" class="viz-container">
{{ viz_embed(src="/viz/kderm-topology") }}
</div>

<script type="module" src="/js/viz-hydrate.js"></script>

## The Biological Pattern

Gram-negative bacteria have:

- **Inner (cytoplasmic) membrane**: Selective barrier, active transport, energy production
- **Peptidoglycan**: Structural integrity layer, thin but rigid
- **Outer membrane**: Asymmetric lipid bilayer, porins for passive diffusion
- **Periplasm**: Space between membranes, signal transduction, protein folding

The ecoPrimals infrastructure maps each layer to physical nodes with defined
roles:

## Physical Layer Mapping

| K-Derm Layer | Physical Node | Role | Bond Types |
|-------------|---------------|------|------------|
| Cytoplasm | LAN gates (eastGate, ironGate, etc.) | Full NUCLEUS, development, UDS IPC | Covalent |
| Plasma membrane | Gate firewall (UFW/nftables) | Mediates all exits from cytoplasm | Covalent, Metallic |
| Inner membrane (cis) | golgi VPS | Forgejo sovereign store, WG hub, relay | Covalent, Metallic |
| Periplasm | sporeGate (LAN) | Build authority (Sovereign CI), depot origin | Metallic |
| Outer membrane (trans) | golgi VPS (Caddy) | TLS termination, sporePrint, WAN depot | Ionic, Weak |
| Extracellular | GitHub, public internet | Trailing mirrors, CDN, CI | Weak |

## Bond-Mediated Communication

Every boundary crossing uses a specific bond type, enforcing the biological
pattern that information degrades as it moves outward:

```
Cytoplasm ←─[covalent: UDS IPC, family seed]──→ Plasma membrane
Plasma    ←─[covalent/metallic: SSH, Tower]──→ Inner membrane (golgi)
Inner     ←─[metallic: SSH over WG]──→ Periplasm (sporeGate)
Periplasm ←─[ionic: rsync, Caddy]──→ Outer membrane (golgi Caddy)
Outer     ←─[weak: public read-only]──→ Extracellular
```

Key constraint: **the outer membrane cannot reach inward.** GitHub (extracellular)
cannot push to golgi. golgi Caddy cannot SSH to sporeGate. Information
flows outward through the relay chain; inward communication requires bond-appropriate
authentication at each boundary.

## Golgi cis/trans Model

The biological Golgi apparatus processes and ships. The cis face receives; the
trans face ships outward. In the ecoPrimals architecture, golgi serves both
roles on a single VPS — cis (Forgejo, WG hub) and trans (Caddy, depot):

- **golgi (cis/inner)**: Receives pushes from gates, stores in Forgejo, fires Sovereign CI hooks
- **golgi (trans/outer)**: Ships to the public — hosts sporePrint, serves depot, pushes to GitHub
- **sporeGate (periplasm)**: Build authority — compiles binaries, rsyncs to golgi trans face

## Channel Proteins

Each boundary has specific mediators (channel proteins) that control what
crosses:

| Boundary | Channel | Mechanism |
|----------|---------|-----------|
| Cytoplasm → Inner | Aquaporin | SSH covalent over WG (registered keys) |
| Inner → Periplasm | Aquaporin | SSH metallic over WG (fleet keys) |
| Periplasm → Outer | Gated ion | rsync, BLAKE3-verified depot push |
| Outer → Extracellular | Passive diffusion | Public read-only (Caddy HTTPS) |

## The Sovereign CI Chain (Live)

As of Wave 120, the K-Derm relay chain uses Sovereign CI:

```
Gate pushes to Forgejo (covalent bond)
  → golgi post-receive hook fires (sovereign-ci-trigger.sh)
  → SSH to sporeGate over WG (metallic bond)
  → sporeGate: cargo build → rsync depot to golgi (ionic)
  → golgi: site rebuild + GitHub push (weak bond)
```

End-to-end propagation: **~3-8 seconds** from gate push to GitHub appearance.
Build time: ~2-5 min incremental, ~24 min full (sporeGate NUC hardware).

## Why This Matters

The diderm model provides:

1. **Security through topology**: The outer membrane is intentionally minimal. Attack surface is one Caddy server with no inward reach.
2. **Sovereignty through architecture**: Forgejo on the inner membrane means source-of-truth is always sovereign. GitHub is an extracellular trailing mirror.
3. **Resilience through separation**: Losing golgiBody-ext does not affect development. Losing GitHub does not affect the ecosystem. Only the inner membrane is critical.
4. **Evolution through layers**: New gates join at the cytoplasm level. New VPS services attach at appropriate membrane layers. The topology scales without redesign.

## WAN Validation (flockGate)

flockGate validates that the diderm model works across WAN, not just LAN:

- Push latency (WAN → inner membrane): 1.2-1.4s
- Relay propagation (inner → extracellular): ~3-6s
- Total gate-to-GitHub over WAN: ~5-8s

The same architecture that works on the LAN mesh works identically for a
remote gate on the other side of the state, connected only via public internet.
