+++
title = "Composition Patterns"
description = "How primals compose into products — PrimalBridge, deploy graphs, graceful degradation, and the gen4 architecture that makes primals invisible."
date = 2026-05-31
weight = 12

[extra]
foundation = true
domain = "Architecture"
maturity = "architectural"
+++

## The Composition Layer

gen4 introduced the pattern that makes primals invisible to end users. A
product (esotericWebb, initioChem, helixVision) consumes primals as
infrastructure through a composition layer — the user interacts with the
product, never the underlying primals.

## PrimalBridge

The `PrimalBridge` is the runtime composition interface. A product declares
which primal capabilities it needs, and the bridge handles discovery,
connection, retry, and graceful degradation.

Key properties:
- **Discovery-based**: Products don't hardcode primal addresses — they discover available capabilities at runtime
- **Graceful degradation**: If a primal is unavailable, the bridge provides sensible defaults or reduced functionality
- **Transport-agnostic**: UDS (local), TCP (network), or future transports — the bridge abstracts all of them
- **Retry + circuit breaker**: Transient failures don't crash the product

## Deploy Graphs

Products define their primal dependencies in TOML deploy graphs:

```toml
[[primals]]
name = "rhizoCrypt"
capabilities = ["dag.create", "dag.append", "dag.verify"]
required = true

[[primals]]
name = "petalTongue"
capabilities = ["render.markdown", "render.template"]
required = false
fallback = "local_renderer"

[composition]
ordering = "topological"
```

The PrimalLauncher reads deploy graphs, topologically sorts dependencies,
spawns binaries from plasmidBin, polls for TCP readiness, and injects
connections into the PrimalBridge.

## Graceful Degradation

Not all primals are required. Products define three tiers:

| Tier | Behavior | Example |
|------|----------|---------|
| Required | Product fails to start without this primal | rhizoCrypt for provenance |
| Enhanced | Product works but with reduced capability | petalTongue for rich rendering |
| Optional | Product ignores absence entirely | barraCuda for GPU acceleration |

This means a product can run on a minimal machine (no GPU, limited primals)
and still function — it just doesn't have GPU acceleration or rich rendering.

## TCP JSON-RPC 2.0

All primal communication uses TCP JSON-RPC 2.0:

- Platform-agnostic (works on GrapheneOS, across network boundaries)
- Standardized method/params/result format
- Enables federation (primals on different nodes communicate naturally)
- No shared memory, no unsafe FFI, no tight coupling

## The Invisible Infrastructure Principle

In a well-composed gen4 product:

1. The user never types a primal name
2. The user never sees a capability graph
3. The user never configures a deploy graph (that's developer-facing)
4. Failures degrade gracefully, not catastrophically
5. The product "just works" — the primals are the plumbing

This is the measure of composition success: **the primals disappear.**

## guideStone Verification Class

guideStone ensures that composition doesn't sacrifice correctness.
A guideStone artifact is self-contained, self-verifying, and
self-benchmarking:

**Five properties:**
1. **Self-contained**: Carries all binaries, data, and configs
2. **Self-verifying**: Validates its own physics against published papers
3. **Self-benchmarking**: Measures the machine it lands on
4. **Cross-substrate**: Works on Ubuntu, Alpine, aarch64, GPU/CPU
5. **Provenance-tracked**: Every output has a BLAKE3-anchored derivation chain

The first guideStone (hotSpring v0.7.0) validates 59/59 physics checks
across 5 substrates with bit-identical cross-platform observables.

## Pattern Summary

```
Developer writes product code
  → Defines deploy graph (TOML)
  → PrimalLauncher spawns dependencies
  → PrimalBridge connects capabilities
  → User interacts with product (primals invisible)
  → guideStone verifies output correctness
  → pseudoSpore packages results with provenance
```

## Infrastructure Compositions — Fractal Deployment (Wave 134c)

Product composition (above) describes how **products** consume primals. Infrastructure
composition describes how **gates** deploy primals. Both follow the same principle:
declare what you need, the system handles the rest.

The ecosystem defines five **infrastructure composition profiles** in
`ecosystem_manifest.toml [compositions]`. Each is a replicable fractal pattern
deployable on any hardware:

| Profile | Description | Scale |
|---------|-------------|-------|
| **full** | All 13+ primals, build-capable, full mesh | Server (128GB+ RAM) |
| **thin-relay** | Depot + relay + sporePrint. No source repos. | VPS ($5/mo) |
| **tower** | Minimal secure mesh entry | Any device |
| **compute** | GPU/HPC workloads | GPU server |
| **nest** | Cold storage + CAS | Storage node |

### The Thin Relay Pattern

The **thin-relay** composition is the fractal building block for sovereign
infrastructure. It requires no Rust toolchain and no primal source repos:

```
thin-relay gate:
  ├── songBird (mesh relay + drawbridge)
  ├── nestGate (sporePrint website hosting)
  ├── membrane (cascade CLI + auto-fetch)
  └── wateringHole (only repo tracked)
```

**Deploy anywhere**: VPS nodes, HPC sites, edge locations, university mirrors.
A thin relay receives ecobins via `mesh.subscribe` and serves them via Caddy TLS.
sporePrint runs on nestGate within the thin relay, making the website available
from any sovereign relay point.

### Product + Infrastructure: Two Layers, One Pattern

```
Product composition (user-facing):
  Product → deploy graph → PrimalBridge → primals → invisible infrastructure

Infrastructure composition (operator-facing):
  Gate → ecosystem_manifest → composition profile → ecobins → sovereign deployment
```

Both are declarative. Both degrade gracefully. Both compose from the same
primal building blocks. The difference is audience: developers write deploy
graphs, operators select composition profiles.

The composition layer makes the ecosystem usable by people who neither
know nor care about sovereign infrastructure. The science is correct
because the infrastructure was validated in gen3. The user experience is
clean because the composition layer was built in gen4.
