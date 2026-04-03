+++
title = "Esoteric Webb — Primal Composition as Creative Infrastructure"
description = "Game Design x Provenance — primal composition patterns for creative tools"
date = 2026-03-30

[extra]
paper_number = 24
+++

<!-- SPDX-License-Identifier: CC-BY-SA-4.0 -->
# Sub-Thesis 24: Esoteric Webb — Primal Composition as Creative Infrastructure

**Date:** March 29, 2026
**Status:** V6, 342 tests, ~91% coverage, 41 Rust files (~13.5k LOC), no spring dependencies
**Identity:** sporeGarden composition for deployment — composes primals directly via biomeOS graph deployments
**Foundation:** ecoPrimals gen4 composition layer; primalSpring IPC patterns; wateringHole standards

## Abstract

We demonstrate that a complex creative product (a Disco Elysium-inspired CRPG
with DAG-traced narrative, deep NPCs, emergent ability interactions, and
meaningful bounded endings) can be built entirely from composed primal services
without importing any primal or spring Rust crate. All coordination occurs at
runtime via JSON-RPC IPC over TCP/UDS. This establishes the gen3→gen4
boundary: infrastructure becomes invisible inside a product someone actually
uses.

The key architectural contribution is a **6-phase graceful degradation
pipeline** where every player action flows through AI narration → NPC
dialogue → flow evaluation → scene push → provenance lifecycle, with each
phase degrading silently when its backing primal is unavailable. Gameplay is
never blocked by missing infrastructure.

## Relationship to the Constrained Evolution Thesis

The constrained evolution thesis (gen3) proved that science computes correctly
across 7 springs with 12k+ validation checks. Esoteric Webb asks the gen4
question: **can people who didn't build the primals compose them into tools
they care about?**

This mirrors biological evolution: once molecular machinery (primals) is
reliable, organisms (compositions) can use it without understanding its
internal mechanisms. The composition surface must be:

1. **Discovery-based** — primals found by capability, not hardcoded address
2. **Degradation-tolerant** — missing capabilities reduce fidelity but never
   break function
3. **Semantically classified** — errors carry operational meaning
   (retriable vs fatal) across IPC boundaries
4. **Zero coupling** — no shared Rust crates, no compile-time dependencies

## Key Findings

### 1. Semantic Error Classification Enables Resilient Composition

Flat error enums (`Io(String)`) lose operational meaning at IPC boundaries.
By classifying errors semantically (`ConnectionRefused`, `Timeout`,
`MethodNotFound`, `ApplicationError`), circuit breakers and retry policies
can make intelligent decisions without string parsing. This pattern emerged
independently in primalSpring and was absorbed by Webb, confirming it as an
ecosystem-wide need.

### 2. Single Source of Truth Eliminates Coordination Debt

Every system that maintains its own list of known primals accumulates
coordination debt as the ecosystem grows. A canonical names module reduces
the cost of adding new primals from "update N files" to "add one line."

### 3. Smart Refactoring Preserves Cohesion Under Size Constraints

When a module exceeds quality gates (1000 lines), naive splitting scatters
related logic. Identifying *semantic boundaries* (data types, pipeline logic,
core state management) produces modules that are independently comprehensible
while sharing state via `pub(crate)` visibility.

V5.1 extended this pattern: `content/mod.rs` (967 LOC) extracted data model
types to `content/types.rs`; `bridge.rs` (943 LOC) converted to a directory
module with `bridge/mod.rs` (core) and `bridge/domains.rs` (domain
delegations). The bridge split is particularly instructive — child modules in
Rust can access parent private methods, so domain delegations call generic
helpers without visibility changes.

### 4. Coverage Gates Drive Targeted Testing

Setting `--fail-under-lines 90` as a CI gate forced identification of
untested *behavior paths* rather than inflating test counts with trivial
assertions. The most impactful coverage gains came from testing:
- Content validation edge cases (missing NPC, empty compound predicates)
- Topological sort edge cases (diamond dependencies, missing deps, cycles)
- Metadata ingestion edge cases (missing fields, bad TOML)
- Protocol handling (TCP listener parse errors, empty lines)

### 5. Edition 2024 Constraints Shape Design

Rust 2024's `unsafe` classification of `std::env::set_var()` means
env-var-dependent code paths cannot be directly unit-tested under
`#![forbid(unsafe_code)]`. This forces design toward pure-function
alternatives that accept configuration as parameters rather than reading
the environment directly.

## Architecture

```
Springs (science)  →  produce  →  primals (genomeBin/ecoBin)
                                         ↓
                                 plasmidBin/ (deployment surface)
                                         ↓
                    Webb discovers via 5-tier capability probe
                           (env vars → metadata → filesystem → Songbird)
                                         ↓
                    PrimalBridge composes 7 domains with 19 methods
                           (retry + circuit breaker per domain)
                    Local science (flow, engagement, DDA) — no IPC needed
                                         ↓
                    6-phase enrichment per player action:
                      narrate → dialogue → flow → scene → DAG → close
                                         ↓
                    Each phase degrades silently → gameplay never blocked
```

## Metrics

| Metric | Value |
|--------|-------|
| Tests | 342 (323 unit + 18 E2E + 1 validation) |
| Coverage | ~91% lines |
| Rust files | 41 |
| LOC | ~13,500 |
| Bridge methods | 19 (all domains, all degrading) |
| Primal domains | 7 (ai, viz, dag, lineage, compute, storage, provenance) |
| Local science | flow, engagement, DDA (absorbed from spring patterns) |
| Unsafe code | `#![forbid(unsafe_code)]` |
| C dependencies | 0 |
| Quality gates | 6/6 (fmt, clippy, test, doc, deny, coverage) |

### 6. Compositions Absorb and Evolve — They Don't Depend

V6 proved that a consumer can absorb patterns from springs (game science
algorithms) as local implementations, then compose the rest directly from
primals. The gap between what primal compositions can do today and what a
self-composed creative engine needs is itself the evolution signal. sporeGarden
projects are **compositions for deployment**: they deploy primal compositions
via biomeOS graph deployments and discover gaps through use-case exercise —
they do not depend on spring source code at runtime.

This establishes a clear ecosystem layering:
- **Primals**: independent binaries with IPC capabilities
- **Springs**: science workspaces that produce and evolve primals
- **Compositions** (gardens): deploy primals, compose them, discover gaps
- **biomeOS**: routes capabilities, orchestrates graphs, bridges naming

When a composition's use case doesn't work, it's a spring validation gap.
When a spring validation gap is found, it discovers primal debt. The
compositions are the final stage — focused on the use case, not the
validation.

## Implications for Ecosystem Evolution

1. **Composition surfaces need semantic errors** — any primal producer should
   classify errors with `is_retriable()` / `is_recoverable()` methods.
2. **Discovery standards need enforcement** — capability-based discovery
   works at tiers 1-4 (filesystem) but tier-5 (Songbird) remains untested.
3. **plasmidBin needs CI** — the gap between "primals exist" and "primals
   are deployed" is the primary blocker for gen4 adoption.
4. **Creative surfaces validate infrastructure** — Webb found and logged 15+
   evolution gaps that fed back to 6 primal teams, proving that consumers
   are the best auditors of producer capabilities.
5. **`#[expect]` over `#[allow]`** — lint suppressions with mandatory reasons
   and automatic dead-suppression detection. During V5.1 migration, several
   `#[allow]` were found dead (lints already resolved by prior refactoring).
6. **Directory modules for growing impl blocks** — Rust child modules can
   access parent private methods, enabling `bridge/domains.rs` to call generic
   call helpers without widening visibility. New primal domains add lines only
   to `domains.rs`, never growing core infrastructure.
