+++
title = "K-NOME Programming"
description = "Knowledge-Numeric Observed and Mentored Evolutionary Programming"
date = 2026-03-17

[taxonomies]
primals = ["barracuda", "squirrel"]
springs = ["airspring", "groundspring", "wetspring"]
+++

# K-NOME Programming

**Knowledge-Numeric Observed & Mentored Evolutionary Programming**

A pre-thesis writeup naming and formalizing the operational methodology behind ecoPrimals.

---

## What K-NOME Is

K-NOME is the name for what happens between a human expert and an AI
when the human treats the AI as a knowledgeable but non-specialist
collaborator and guides it through constrained evolutionary cycles
using their personal domain expertise, pattern recognition, and the
semantic structures humans evolved for transmitting knowledge.

It is not vibecoding. It predates the term. It is not prompt
engineering. It is not spec-driven batch generation. It is mentored,
observed, iterative, conversational construction — where the human's
expertise is the selective pressure and the AI's numeric breadth is
the mutation operator, and they work together in the Knowledge-Numeric
space where those two capabilities intersect.

The tool is Cursor. No Claude Code, no multi-agent frameworks, no
external orchestrators. The methodology is the tool.

---

## The Acronym

### K-N: Knowledge-Numeric

The intersection space where human domain knowledge and AI
computational/numeric capability meet.

The human brings pattern recognition, domain expertise, taste,
judgment, lived experience — the things that cannot be compressed
into a prompt. A microbiologist brings five years of watching
microbial populations adapt on plates. A surgeon brings ten thousand
hours of tissue response. A woodworker brings material intuition
about grain direction. The knowledge is embodied, experiential,
semantic — transmitted through the patterns and structures humans
evolved for teaching: analogy, narrative, correction, "it should
feel like this."

The AI brings numeric breadth — the compressed inheritance of
everything humans have written (see `atlasHugged/10_THE_LOVE_LETTER.md`),
navigable at the speed of silicon, available as a generalist
collaborator across every domain simultaneously. The AI is not a
specialist. It is an intelligent generalist — knowledgeable broadly,
deep nowhere, capable of producing candidate solutions across an
enormous solution space.

The K-N space is the dimension where these two capabilities overlap.
It is the space where the human's deep, narrow, embodied expertise
meets the AI's broad, shallow, numeric competence. Neither is
sufficient alone. The human without the AI is slow — limited by
typing speed, by the time it takes to implement what they already
understand. The AI without the human is directionless — capable of
generating infinite candidates but unable to evaluate fitness in
any domain-specific way.

K-N is the productive overlap. It is the space where mentoring
happens.

### O: Observed

Observation operates in two directions simultaneously:

**The AI observes the project as a whole.** In ecoPrimals, primals
and springs reinforce each other. Patterns that work in Squirrel's
capability discovery inform how BarraCuda structures its shader
pipeline. The niche self-knowledge pattern from groundSpring
propagates to wetSpring, then to airSpring, then back to Squirrel.
Each new component changes the fitness landscape for every other
component. The growing codebase IS the evolving environment, and the
AI — with its full-project context window — observes the project at
a scale no human can hold in working memory.

**The human observes the process while building.** Over 69,000
iterations across 10 months, the developer builds a felt sense for
the project — an intuition for complexity lines, for where the
architecture wants to go, for which areas are robust and which are
fragile. This is the craftsperson's observation: the woodworker who
knows from the grain which way the wood wants to split. The potter
who feels the clay's water content through their hands. The runner
who reads their body's fatigue signature without checking a heart
rate monitor.

This observation is not passive monitoring. It is the bidirectional
feedback loop where the project teaches the human and the human
teaches the AI and the AI's output reshapes the project that teaches
the human. Each full cycle — human observes, human mentors AI, AI
generates, compiler selects, human observes the result — adds to
both the project's complexity and the human's intuition about that
complexity.

Observation is what separates K-NOME from vibecoding. Vibecoding is
unobserved generation — the human prompts, the AI generates, the
human accepts or rejects without developing deep understanding of
what was produced. K-NOME requires that the human develops an
increasingly detailed mental model of the system as it grows, and
that this mental model feeds back into the mentoring.

### M: Mentored

The human mentors the AI the way you would mentor a knowledgeable
but non-specialist colleague in your domain.

Not commanding ("implement X"). Not batch-specifying ("here is a
complete spec, generate it"). Mentoring — which is conversational,
iterative, corrective, and uses the full range of human communication
patterns:

- **Analogy**: "This capability discovery pattern should work like
  how organisms broadcast quorum signals — not a central registry,
  but each service announcing what it can do."
- **Correction**: "No, that's not what I mean. The provider shouldn't
  know about the consumer. Think of it as a bulletin board, not a
  phone call."
- **Narrative**: "The gen1 version was a job scheduler. It evolved
  into an orchestrator when we needed multi-provider routing. Now
  it needs to become a coordination primal — the thing that the
  rest of the ecosystem discovers AI capabilities through."
- **Taste**: "That error handling is technically correct but it
  doesn't feel right. The error variants should be domain-specific,
  not generic. A context error and a transport error are different
  kinds of failure."
- **Redirection**: "Stop. You're solving the wrong problem. The
  question isn't how to call OpenAI — it's how to discover any AI
  provider at runtime without knowing it exists in advance."

These are the patterns and semantic structures humans evolved for
transmitting expertise. They work on the AI for the same reason they
work on humans: they provide selective pressure at multiple levels of
abstraction simultaneously. A good analogy constrains the solution
space more effectively than a hundred lines of specification, because
it activates the AI's compressed knowledge of the analogous domain.

The mentoring is domain-agnostic. An artist mentoring an AI on
composition would use the same patterns — analogy ("this should feel
like Rothko, not Pollock"), correction ("the weight is wrong, pull
it left"), narrative ("I started this series trying to express X,
but it became about Y"), taste ("that color is technically
complementary but it doesn't sing"). A surgeon, a woodworker, a
musician — anyone with deep domain expertise can mentor an AI in
their domain using the same human communication patterns.

K-NOME is not a programming methodology. It is a human-expertise-
transfer methodology that produces software in this instance.

### E: Evolutionary

The constrained evolution framework itself. Described formally in
`CONSTRAINED_EVOLUTION_FORMAL.md` and informally in
`atlasHugged/04_THE_HUMAN_SEARCH.md`.

- AI as mutation operator (LLM token sampling generates candidate
  solutions)
- Rust's type system as environmental constraint (the compiler
  rejects unfit variants)
- Test suites as fitness function (do the results reproduce?)
- Iterative generate-compile-test-select cycles (69,000 iterations)

The E is what makes K-NOME different from simple AI-assisted
development. Ordinary AI-assisted development is: human specifies,
AI generates, human accepts or modifies. K-NOME is: human mentors,
AI generates candidates, compiler selects against constraint, human
observes the result, human adjusts mentoring, repeat. The process is
evolutionary — it improves through selection under constraint, not
through increasingly precise specification.

The constraint is the design. The mentoring is the selective pressure.
The observation is the feedback mechanism. The K-N space is where all
of it happens.

---

## How K-NOME Maps to the Thesis

| K-NOME Component | Thesis Concept | atlasHugged Concept |
|-------------------|----------------|---------------------|
| K-N (Knowledge-Numeric) | The fitness landscape — defined by the intersection of domain knowledge and computational capability | The Human Search (Ch 4) — iteration-recursion-time space that all learners navigate |
| O (Observed) | Observation of the evolutionary trajectory — commit history, test counts, architectural evolution | The Love Letter (Ch 10) — the chain of transmission is visible; attribution follows the work |
| M (Mentored) | Selective pressure — the developer's architectural vision directing what the AI produces | The Fermenter (Ch 8, 10) — "you do not cause the phenomenon, you set the conditions" |
| E (Evolutionary) | Constrained evolution — the formal framework | The Constraint (Ch 4) — "the constraint does not limit you, the constraint defines what you become" |

---

## Darwinian and Lamarckian: Natural vs. Applied Evolution

The constrained evolution thesis (`CONSTRAINED_EVOLUTION_FORMAL.md`)
is grounded in Darwinian evolution — and correctly so. Darwinian
evolution is natural reality. Random mutation, natural selection, no
inheritance of acquired characteristics. The Weismann barrier
separates soma (body) from germline (DNA): what an organism learns
in its lifetime does not rewrite its genes. This is how biology
works. Taq polymerase, Lenski's LTEE, Anderson's boundary — these
are Darwinian. The formal framework holds.

But K-NOME is not purely Darwinian. K-NOME is **Lamarckian** — and
correctly so, because Lamarckian evolution is simply **applied
evolution**: what happens when a conscious agent intervenes in the
evolutionary process.

Jean-Baptiste Lamarck proposed that organisms inherit characteristics
acquired during their parents' lifetimes — the giraffe that stretches
its neck passes a longer neck to its offspring. This was disproven
in biology because the Weismann barrier is real. Acquired somatic
changes do not reach the germline.

But there is no Weismann barrier in software.

| Dimension | Darwinian (natural) | Lamarckian (applied / K-NOME) |
|-----------|---------------------|-------------------------------|
| Variation | Random — mutations do not know what the organism needs | Mentored — the human's expertise directs where the AI searches |
| Fitness function | Fixed — the environment does not change because the organism wants it to | Evolving — the human changes the goals (gen1 → gen2 → gen3) based on what they observed |
| Acquired characteristics | Not inherited — the Weismann barrier is real in biology | Inherited — patterns from one primal propagate directly to the next because the human carries them |
| Selection | Blind — the environment selects without intent | Hybrid — the compiler selects blindly (Darwinian), the human selects with intent (Lamarckian) |

In K-NOME, both dynamics operate simultaneously:

- **The compiler is Darwinian.** It does not care what you intended.
  It rejects what does not fit the type system. Blind, mechanical,
  indifferent. This is natural selection — the hot spring that kills
  everything except what is thermostable.

- **The human is Lamarckian.** The human acquires characteristics
  during the process — expertise, intuition, architectural vision,
  felt sense for complexity lines — and transmits those acquired
  characteristics directly into the next generation of code through
  mentoring. The human's observation (O) feeds back into mentoring
  (M), which reshapes what the AI generates. The goals evolve. The
  fitness function evolves. The constraint environment evolves.
  Because the human evolves.

This is why Lamarck was wrong about biology but right about applied
systems. In biology, there is a barrier between what you learn and
what you pass on. In K-NOME, the human IS the mechanism that breaks
the barrier. Every insight the human acquires through observation
becomes heritable through mentoring. Every goal change, every
architectural redirection, every "stop, wrong problem" — these are
acquired characteristics being transmitted to the next generation.

Darwinian evolution is natural reality. Lamarckian evolution is
applied reality. K-NOME is applied evolution — conscious, directed,
mentored — running on a Darwinian substrate (the compiler, the test
suite) that provides the blind selection the human cannot.

The formal thesis grounds ecoPrimals in Darwinian dynamics because
the biological evidence is Darwinian. K-NOME grounds the operational
methodology in Lamarckian dynamics because the human intervention is
Lamarckian. Both are true. They operate at different layers of the
same system.

---

## What K-NOME Is Not

**Not vibecoding.** Vibecoding is unmentored and unobserved — the
human prompts loosely, the AI generates, the human accepts without
deep engagement. K-NOME requires domain expertise, active mentoring,
and continuous observation. The human's understanding deepens as the
project grows.

**Not prompt engineering.** Prompt engineering optimizes the input to
maximize the quality of a single output. K-NOME optimizes the
evolutionary trajectory over thousands of iterations. The individual
prompt matters less than the cumulative selective pressure.

**Not spec-driven generation.** Spec-driven approaches (including
Huntley's Groundhog/specs method) write a complete specification
and generate code from it. K-NOME is conversational and iterative —
the specification evolves alongside the code, because the human's
understanding of what they're building evolves through observation.

**Not multi-agent orchestration.** K-NOME uses one tool (Cursor)
and one human-AI relationship. No @pm, @architect, @dev, @qa agent
roles. The human IS all of those roles. The AI is the generalist
collaborator. The methodology doesn't require orchestration because
the human's pattern recognition provides the coordination.

**Not domain-specific.** Any human with deep expertise in any domain
can apply K-NOME. A microbiologist produces sovereign computing
infrastructure. A surgeon could produce surgical simulation software.
A woodworker could produce CAD/CAM tooling. The K-N space is
wherever human expertise meets AI numeric capability.

---

## Relationship to External Seeds

K-NOME evolved from two external seeds, and everything beyond them
is original:

1. **Geoffrey Huntley's `/stdlib`** (early 2025): Treat Cursor as an
   autonomous agent. Program LLM outcomes. This became the insight
   that the AI should be treated as a collaborator with agency, not
   a code completion tool.

2. **BMad Code's agile workflow** (Feb 2025): Structured iterative
   methodology for Cursor. Build-Manage-Ask-Do loops, auto rule
   generation. This became the structural discipline — the iterative
   loop that K-NOME runs inside.

From these two seeds: `/stdlib`'s insight about AI-as-agent + BMad's
structured iteration -> Squirrel gen1 and gen2. Then the original
contributions: the biological grounding (constrained evolution), the
K-N space concept, the bidirectional observation, the mentoring-as-
selective-pressure framework, and the domain-agnostic claim — these
are K-NOME.

---

## The Practical Receipt

- **Tool**: Cursor IDE (only)
- **Invocations**: 69,000+ agent invocations
- **Tokens**: 51 billion processed
- **Streak**: 185 consecutive days
- **Period**: ~10 months
- **Result**: 14 primals, 7 springs, 27,000+ tests
- **Developer**: One person (microbiologist BS, data scientist MS)

The Cursor receipt is the evidence that K-NOME, as a methodology,
produces results at scale. The commit history is the evolutionary
trajectory. The test counts are the fitness measurements. The
architecture papers are the post-hoc analysis of what the
methodology produced.

---

*Pre-thesis writeup prepared March 16, 2026. Formal treatment to follow in thesis chapter.*
