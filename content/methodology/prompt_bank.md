+++
title = "The Prompt Bank"
description = "The working vocabulary of K-NOME in practice — real prompts from ecosystem development, not polished templates."
date = 2026-05-20
weight = 21

[taxonomies]
trails = ["methodology"]

[extra]
domain = "Methodology"
maturity = "implemented"

[[extra.companions]]
url = "/methodology/conversation-constraint/"
title = "Conversation Constraint"
relation = "extended_by"
label = "The structural constraint these prompts operate within"

[[extra.companions]]
url = "/methodology/massively-parallel-mentoring/"
title = "Massively Parallel Mentoring"
relation = "pairs_with"
label = "How these prompts run across parallel sessions"
+++

## What This Is

Real prompts from ~6 months of K-NOME development — the gardener's shorthand,
not polished templates. These are the phrases that move the ecosystem forward,
organized by intent.

---

## 1. Comprehensive Audit

The audit prompt is the starting point for any session:

> *"Review this crate for deep debt, evolution gaps, hardcoded values,
> unsafe code that could be safe, mocks in production, and overstep
> cleanup. Proceed with fixes."*

Variations target specific concerns:

> *"Analyze external dependencies and evolve to Rust."*
> *"Large files (>800L) should be refactored smart rather than just split."*

---

## 2. Constraint Evolution

Prompts that apply specific constraints:

> *"Evolve this to accept a branch parameter instead of hardcoding main."*
> *"Replace .expect() with Result-based error handling in production code."*
> *"Centralize this magic number as a named constant."*
> *"Make this agnostic — primal code only has self-knowledge."*

---

## 3. Cross-System Coordination

Moving patterns between springs and primals:

> *"Apply the same pattern from hotSpring to wetSpring."*
> *"Propagate this fix across all crates that use the same idiom."*
> *"Cascade from VPS and review."*

---

## 4. Deep Debt

Targeted debt elimination:

> *"Proceed to execute on all remaining deep debt and evolution gaps."*
> *"Mocks should be isolated to testing; any in production should be
> evolved to complete implementations."*
> *"Hardcoding should be evolved to agnostic and capability-based."*

---

## 5. Handoff and Documentation

Generating documentation and coordination artifacts:

> *"Update root documentation — CONTEXT.md, EVOLUTION_QUEUE.md,
> CHANGELOG, test counts."*
> *"Write a wateringHole handoff for this wave."*

---

## 6. The Proceed Prompt

The simplest and most powerful K-NOME prompt:

> *"Proceed."*

This prompt trusts the AI to continue the current work without additional
direction. It works because the conversation context carries the intent —
the AI knows what was audited, what was fixed, what remains.

The proceed prompt is the mentoring pattern in miniature: the gardener
has set the direction, the AI tends the growth, and "proceed" means
"keep going, I trust the trajectory."

---

## Pattern Observations

- **Near-duplicates reflect reuse** — the same audit prompt appears in
  many springs because the same patterns recur
- **Constraint language evolves** — early prompts say "fix this"; mature
  prompts say "evolve this to modern idiomatic Rust"
- **Garden language is natural** — "tend," "propagate," "fossil," "prune"
  emerge organically from the biological framing
- **Geological language scales up** — "stadial gate," "interstadial,"
  "glacial goal" describe ecosystem-level K-NOME

---

*The prompt bank is not a template library. It is a vocabulary — a living
record of how one gardener talks to the garden. Your vocabulary will be
different. The constraint is the same: the conversation is the interface.*
