+++
title = "Chapter 7: Experimental Methodology"
description = "The spring framework: Python control to Rust to GPU phased validation across five springs and eight scientific domains."
weight = 7
date = 2026-07-09
+++

{{ maturity(level="planned") }}

## The Spring Framework

A **spring** is a public AGPL-3.0 repository that reproduces published peer-reviewed science through a phased protocol:

| Phase | Runtime | Purpose |
|-------|---------|---------|
| 0 | Python | Control — establish baseline with reference implementations |
| 1 | Rust | Port — reproduce Python results in pure Rust |
| 2 | GPU | Promote — accelerate via BarraCuda WGSL shaders |
| 3+ | Extensions | Extend — new science beyond the original papers |

Multi-domain validation is required to show the platform generalizes beyond any single field. Every check is an automated pass/fail assertion with defined tolerances.

---

**See also:**

- [How to Start a Spring](@/methodology/HOW_TO_START_A_SPRING.md) — the operational playbook
- [Spring Catalog](@/architecture/SPRING_CATALOG.md) — complete spring inventory
- Chapters [8](@/thesis/08_results_hotspring.md)–[12](@/thesis/12_results_neuralspring.md) — per-spring results

---

*Full content transplant pending. Source: `whitePaper/gen3/thesis/07_experimental_methodology.md`*
