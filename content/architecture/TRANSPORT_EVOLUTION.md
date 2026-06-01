+++
title = "Transport Evolution"
description = "From nanowire SSH scripts to quorum-sensing coordination — the evolutionary path of inter-node communication."
date = 2026-05-31
weight = 17

[extra]
domain = "Architecture"
+++

## Current State: Nanowire

The K-Derm relay chain currently uses **nanowire** transport: point-to-point
SSH connections between specific nodes. Each node knows the next node's address,
holds its SSH key, and issues explicit commands.

```
Gate ──SSH──→ golgiBody ──SSH──→ peptidoglycan ──SSH──→ golgiBody-ext ──SSH──→ GitHub
```

This is the biological equivalent of type IV pili — conductive protein filaments
that Gram-negative bacteria use for direct cell-to-cell electron transfer.

### Where Nanowire Is Correct

Nanowire remains the correct pattern for metallic bond interactions:

| Interaction | Why Nanowire |
|-------------|-------------|
| Gate SSH to Forgejo | Authentication requires direct connection |
| Fleet key operations | Admin commands need explicit targeting |
| Build triggers | peptidoglycan needs deterministic invocation |

### Where Nanowire Is Limiting

For population-level coordination, nanowire creates fragility:

- **Topology changes require rewiring**: Adding a node means editing scripts
- **Synchronous blocking**: Each hop waits for the previous to complete
- **Single path**: No redundancy if any node is unreachable
- **Hardcoded knowledge**: Each script knows the next node's address

## Target State: Quorum Sensing

The target architecture is **quorum sensing** — a diffusion-based coordination
model where nodes detect environmental signals and respond independently.

In biology, quorum sensing uses **autoinducers**: small signaling molecules
that accumulate in the environment. When concentration exceeds a threshold,
all nearby cells activate coordinated behavior — without any cell directly
commanding another.

### The Impulse System as Proto-Quorum

The ecosystem already has the foundation: the **impulse/potential** system.
Impulses are TOML files committed to `wateringHole/impulses/active/`. Any
gate can fire an impulse; any gate can sense pending impulses.

```
Gate fires impulse → commits TOML to wateringHole
  → temporal sync distributes to all gates
  → each gate's membrane binary senses the impulse
  → gates with matching capabilities respond
```

This is quorum sensing: the impulse diffuses through the periplasm (wateringHole
repo), and nodes that can respond, do respond. No direct addressing needed.

### The Evolution Path

| Phase | Transport | Coordination | Status |
|-------|-----------|-------------|--------|
| 1 (current) | Nanowire (SSH scripts) | Explicit relay chain | LIVE |
| 2 | Nanowire + impulse sensing | Hybrid: relay + async signals | In progress |
| 3 | Quorum sensing | Impulse diffusion drives all coordination | Target |

### Phase 2: Hybrid (Current Wave)

The relay chain handles synchronous operations (git push propagation).
The impulse system handles asynchronous coordination (wave assignments,
gate capabilities, build triggers). Both coexist — nanowire for the
critical path, quorum sensing for orchestration.

### Phase 3: Full Quorum (Future)

In the target state, even the relay chain becomes quorum-driven:

1. Gate pushes to Forgejo (covalent bond — always nanowire)
2. Forgejo fires an impulse: "new content available for relay"
3. peptidoglycan senses the impulse, pulls autonomously
4. golgiBody-ext senses "content staged for shipping," pushes to GitHub

Each node acts on environmental signals, not direct commands. The topology
can change (add/remove nodes) without rewiring — new nodes simply begin
sensing impulses and responding to those matching their capabilities.

## Songbird: The Federation Transport

For inter-gate coordination beyond the VPS membrane, **Songbird** provides
the mesh federation transport:

- TURN relay on golgiBody-ext for NAT traversal
- Peer-to-peer connections between gates
- Capability discovery across the mesh
- WAN-resilient with automatic reconnection

Songbird is the organism's long-range communication — not point-to-point
nanowire but broadcast signaling across the entire gate mesh.

## Why This Matters

The transport evolution directly enables:

1. **Scalability**: New gates join by sensing impulses, not by being wired in
2. **Resilience**: No single relay failure blocks coordination
3. **Autonomy**: Each node makes its own decisions based on environmental signals
4. **Evolution**: The coordination protocol can evolve without infrastructure changes
