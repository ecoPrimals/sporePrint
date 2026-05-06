+++
title = "🧬 Methodology"
description = "How it was built — constrained evolution, K-NOME programming, and the operational playbook."
sort_by = "title"
template = "section.html"
+++

The methodology is biological: evolve under constraint, validate
against published science, compose from small parts, track everything.

**Constrained evolution** is the core principle. Remove external dependencies
(no CUDA, no cloud, no vendor toolchains) and the system is forced to evolve
genuine capabilities. Every constraint that seems limiting becomes an
innovation pressure — eliminating CUDA produced Vulkan GPU compute that works
on any vendor. Eliminating cloud produced a sovereign infrastructure that runs
on commodity hardware.

**K-NOME** (Knowledge–Numeric, Observed, Mentored, Evolution) is the
operational model for AI-assisted development. The AI is a collaborator
under human constraint, not an autonomous agent. Every generation of code
is tested against the previous generation and against published results.

- [Constrained Evolution — Formal](CONSTRAINED_EVOLUTION_FORMAL.md) — the theoretical framework
- [K-NOME Programming](K_NOME_PROGRAMMING.md) — operational AI-assisted development
- [How to Start a Spring](HOW_TO_START_A_SPRING.md) — the phased playbook: Python → Rust → GPU → composition
- [Knowledge Commons Targets](KNOWLEDGE_COMMONS_TARGETS.md) — what public data + cheap hardware unlocks
- [scyBorg Licensing](SCYBORG_LICENSING.md) — AGPL + ORC + CC-BY-SA triple license rationale
- [P vs NP and the Enzyme Thesis](P_NP_ENZYME_THESIS.md) — generation/verification asymmetry in computation
