+++
title = "primalSpring — Composition Parity, Deploy Graphs, NUCLEUS Validation"
description = "Meta-spring — validates that primals compose correctly. 666 tests, 85 experiments, 389 registered capabilities, 13 deploy graphs."
date = 2026-05-08
weight = 1

[taxonomies]
primals = ["biomeos", "barracuda", "toadstool", "nestgate", "beardog", "songbird", "squirrel", "rhizocrypt", "loamspine", "sweetgrass", "petaltongue", "skunkbat", "coralreef"]
springs = ["primalspring"]
+++

## Domain

Primal composition and deploy graphs — validates that the 13 primals compose into working NUCLEUS deployments.

**Repository**: [syntheticChemistry/primalSpring](https://github.com/syntheticChemistry/primalSpring)

## The Composition Story

primalSpring is the meta-spring. It doesn't do science itself — it validates that the primals compose correctly, that deploy graphs produce working compositions, and that cross-gate bonding works as documented. If primalSpring passes, the composition model works. If it fails, the error is in the wiring.

## Headline Results

- **666 tests** passing (618 pass + 48 ignored), 0 failed
- **85 experiments** across 15 categories (tower atomic through frontier)
- **13 deploy graphs** validated (74 total nodes, 5 bond types)
- **389 registered capability methods** in `capability_registry.toml`
- **13/13 primals** adopt JH-0 (MethodGate capability check)
- **5 composition patterns** validated: Sequential, Parallel, ConditionalDag, Pipeline, Continuous
- **exp094** validates full NUCLEUS parity: Tower + Node + Nest + Cross-Atomic pipeline

## Validation Phases

| Phase | Key Result |
|-------|------------|
| Tower Atomic | BearDog + Songbird compose — `crypto.hash` + `discovery.resolve` match baselines |
| Node Atomic | barraCuda + coralReef + ToadStool — `stats.mean` via IPC matches Python |
| Nest Atomic | NestGate + provenance trio — `storage.store` + `storage.retrieve` round-trip integrity |
| Cross-Atomic Pipeline | Hash → store → retrieve → verify across all three atomics |
| Multi-User Hardening | JH-0 through JH-5 — MethodGate, ionic tokens, resource envelopes, composition hot-reload, log aggregation |

## What primalSpring Validates

| Capability | Method |
|-----------|--------|
| Deploy graphs | Topologically sorted, capability-addressed, bonded composition |
| Cross-gate bonding | Primal → primal IPC wiring verified against capability registry |
| BYOB composition | Packaged binary artifacts compose into emergent behavior |
| 5-tier discovery | UDS → mDNS → BirdSong → STUN → Dark Forest hierarchy |
| BTSP Phase 3 | All primals bind `127.0.0.1`, AEAD on wire |

## Notebooks (5)

| # | Notebook | Focus |
|---|----------|-------|
| 01 | [Composition Validation](/lab/notebooks/01-composition-validation/) | Deploy graphs, bond types, profiles, discovery tiers |
| 02 | [Benchmark Comparison](/lab/notebooks/02-benchmark-comparison/) | Rust vs Python timing, energy, guidestone phases |
| 03 | [Ecosystem Evidence](/lab/notebooks/03-ecosystem-evidence/) | 85 experiments, gap resolution, security timeline |
| 04 | [Cross-Spring Connections](/lab/notebooks/04-cross-spring-connections/) | Primal consumption matrix, ecosystem flows, sporePrint readiness |
| 05 | [BTSP Security Deep Dive](/lab/notebooks/05-btsp-security-deep-dive/) | Per-primal posture, convergence arc, discovery hierarchy |

## See Also

- [Validation Summary](/lab/primalspring-validation-summary/) — full status with metrics
- [NUCLEUS Architecture](/architecture/nucleus-architecture/) — how primals compose
- [Deployment Model](/architecture/deployment-model/) — plasmidBin and BYOB composition
