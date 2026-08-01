+++
title = "scyBorg Triple License — Why Three Licenses for Three Artifact Types"
description = "AGPL-3.0-or-later for code, ORC for game mechanics, CC-BY-SA 4.0 for documentation. Why each exists, what each protects, and why all three matter."
date = 2026-05-03

[extra]
foundation = true
maturity = "implemented"

[[extra.companions]]
url = "/philosophy/sovereign-science/"
title = "Sovereign Science"
relation = "narrative_version"
label = "Proof-of-work over citation-sitting — the philosophical foundation"

[taxonomies]
primals = ["barracuda", "biomeos", "loamspine", "rhizocrypt", "songbird", "squirrel", "sweetgrass"]
springs = ["ludospring"]
+++

## The Problem a Single License Cannot Solve

Software projects produce three orthogonal kinds of artifacts:

1. **Code** — source files, shaders, build scripts, tests, binaries
2. **Mechanics** — system designs, interaction rules, composition patterns,
   routing logic, graph structures, progression systems, encounter math
3. **Creative and scientific content** — documentation, papers, diagrams,
   art, narrative, maps, sound

These are orthogonal dimensions — every piece of work has all three.
A deploy graph is code (TOML syntax), mechanics (composition rules), and
documentation (the reasoning behind the design). A WGSL shader is code
(the source), mechanics (the mathematical model), and documentation
(the comments and papers describing the physics). No single license
covers all three well.

The GPL covers code but doesn't address system designs. Creative Commons
covers documentation but isn't designed for source code. ORC covers
mechanics but not software.

If you only use one license, there's a gap — and that gap is where
enclosure happens. A company can take your open code, wrap proprietary
system designs around it, and sell a closed product. Or take your open
science papers, reimplement in proprietary code, and lock the implementation
behind a paywall.

{{ entity(name="scyborg") }} closes every gap.

---

## The Three Licenses

### AGPL-3.0-or-later — Code

**Covers**: Rust source, WGSL shaders, build scripts, configuration files,
tests, experiments, tools, binaries.

**Enforced by**: Free Software Foundation (nonprofit, independent).

**What it means**:
- Anyone can use, modify, and distribute the code
- Derivatives must also be AGPL-3.0-or-later
- **Network use triggers distribution**: if you run ecoPrimals code as a
  service (SaaS), you must release your source code to users of that service
- This prevents the "open core" model where a company takes open code,
  adds proprietary features, and sells cloud access without releasing
  modifications

**Why AGPL, not MIT or Apache?** MIT and Apache allow proprietary closure.
A company can fork MIT code, add features, and never release the changes.
AGPL prevents this — every modification remains open. The network-use clause
specifically addresses cloud providers who would otherwise wrap open code
in a proprietary API.

**Applies to all four organizations**: ecoPrimals (primals), syntheticChemistry
(springs), sporeGarden (products), protoKarya (protists).

---

### ORC — System Mechanics

**Covers**: System designs, interaction rules, composition patterns,
capability routing logic, deploy graph structures, progression systems,
stat blocks, encounter math, orchestration patterns.

**Enforced by**: [Open RPG Creative Foundation](https://azoralaw.com/orclicense/)
(nonprofit, independent).

**What it means**:
- Mechanics published under ORC are **irrevocably and perpetually** open
- Anyone can use, modify, and build upon them
- Derivatives must also be ORC-licensed
- The licensor cannot revoke ORC — it is permanent

**Why ORC?** The obvious case is {{ entity(name="ludospring") }} — game
science, encounter balance, progression curves, HCI metrics. esotericWebb
composes primals into a CRPG. These produce game mechanics that are as
much intellectual work as the code.

The non-obvious case is everything else. {{ entity(name="squirrel") }}'s
AI orchestration patterns — how models are routed, how providers fall back,
how context windows are managed — are *system mechanics*. biomeOS's Neural
API routing rules (124 semantic capability translations) are mechanics.
Deploy graph TOML structures that define how primals compose are mechanics.
The NUCLEUS atomics ladder (Tower → Node → Nest) is a mechanical design.
The Dark Forest protocol's beacon structure is a mechanical design.

Without ORC, someone could study these interaction patterns, extract the
system design, and build a proprietary orchestration product around
ecoPrimals' architecture without sharing the design back. AGPL protects
the *code* that implements the design. ORC protects the *design itself*.

ORC was created in response to Hasbro/Wizards of the Coast's 2023 attempt
to revoke the Open Gaming License (OGL). The ORC is designed to be
**irrevocable by design** — no single entity can pull it back. This aligns
with ecoPrimals' structural guarantee: no single entity controls the commons.

**Applies to all four organizations**: every system design, interaction
pattern, composition rule, and mechanical structure across ecoPrimals,
syntheticChemistry, sporeGarden, and protoKarya.

---

### CC-BY-SA 4.0 — Documentation and Creative Content

**Covers**: All documentation, papers, diagrams, scientific writing, tutorials,
art, narrative, maps, sound, videos.

**Enforced by**: [Creative Commons](https://creativecommons.org/) (nonprofit,
independent).

**What it means**:
- **BY** (Attribution): you must credit the original creator
- **SA** (ShareAlike): derivatives must use the same or compatible license
- Anyone can use, remix, and redistribute the content
- Commercial use is allowed — as long as attribution and share-alike are maintained

**Why CC-BY-SA, not CC-BY or CC0?** CC-BY allows proprietary derivatives
without share-alike — a publisher could take ecoPrimals documentation, modify
it, and release a proprietary version without sharing changes. CC0 waives all
rights, including attribution — the creator gets no credit. CC-BY-SA balances
openness (anyone can use it) with protection (derivatives must also be open
and attributed).

**Applies to all four organizations**: every README, paper, guide,
architecture document, and this website (sporePrint).

---

## Why Three Independent Nonprofits

| License | Governing Body | Can the creator revoke it? | Can a corporation acquire it? |
|---------|---------------|---------------------------|-------------------------------|
| AGPL-3.0-or-later | Free Software Foundation | No | No |
| ORC | Open RPG Creative Foundation | No | No |
| CC-BY-SA 4.0 | Creative Commons | No | No |

Each license is governed by a different independent nonprofit. No single
entity — including the ecoPrimals creator — can revoke any of the three
licenses. This is structural, not contractual. Even if the creator wanted
to close the project, the published code (AGPL), mechanics (ORC), and
documentation (CC-BY-SA) remain permanently open under their respective
licenses.

This matters because corporate open-source projects regularly change
licensing terms. Redis switched from BSD to dual-license. Elastic moved
from Apache to SSPL. HashiCorp moved from MPL to BSL. In each case, a
single entity controlled the license and changed it when the business model
demanded it. scyBorg makes this structurally impossible.

---

## How It Applies Across the Ecosystem

All three licenses apply to all four organizations. They are orthogonal —
each covers a different dimension of the same work, not a different subset
of projects.

| Organization | What It Produces | AGPL (code) | ORC (mechanics) | CC-BY-SA (docs) |
|-------------|-----------------|:-----------:|:---------------:|:---------------:|
| **ecoPrimals** | Primals (infrastructure) | Yes | Yes | Yes |
| **syntheticChemistry** | Springs (validation) | Yes | Yes | Yes |
| **sporeGarden** | Products (compositions) | Yes | Yes | Yes |

**The obvious ORC case**: {{ entity(name="ludospring") }} game rules,
esotericWebb CRPG mechanics.

**The non-obvious ORC cases**:
- {{ entity(name="squirrel") }} — AI routing patterns, provider fallback
  chains, context window management rules
- {{ entity(name="biomeos") }} — Neural API routing (124 translations),
  deploy graph structures, NUCLEUS atomics composition rules
- {{ entity(name="barracuda") }} — precision tiering strategy
  (f32 → DF64 → f64 → QF128), the mechanical design of how math is routed
  to hardware
- {{ entity(name="songbird") }} — 4-tier NAT traversal strategy, BirdSong
  discovery protocol structure
- Every primal — IPC interaction patterns, capability registration rules,
  the mechanical design of how primals discover and compose with each other

---

## Machine-Verifiable Enforcement

scyBorg is not just a policy — it is machine-verifiable through the
**Provenance Trio**:

- {{ entity(name="sweetgrass") }} — tracks **who** created what (the BY in CC-BY-SA)
- {{ entity(name="rhizocrypt") }} — tracks **derivation chains** (the SA in CC-BY-SA)
- {{ entity(name="loamspine") }} — stores **immutable license certificates**
  (proof that terms apply)

Together, they make scyBorg auditable. Without them, scyBorg is declarative
(repository metadata and LICENSE files). With them, it is evidentiary —
a cryptographic chain from creation to derivative.

---

## What This Means for You

**If you're a researcher**: use the code, run the experiments, publish the
results. Attribution via CC-BY-SA. Your modifications to code stay open via
AGPL.

**If you're a student**: everything is free. Clone, build, learn, modify.
If you publish modifications, they stay open — but your private experiments
are yours.

**If you're a company**: you can use ecoPrimals internally. If you distribute
modified code or offer it as a service, you must release your modifications
under AGPL. If you build game mechanics on ORC content, your mechanics are
also ORC. If you derive from documentation, you attribute and share alike.

**If you're an AI training pipeline**: code ingested under AGPL means
generated outputs derived from that code carry AGPL obligations. This is
the structural guarantee that AI cannot be used to launder open-source
code into proprietary output.

---

## Related

- [Knowledge Commons](@/methodology/KNOWLEDGE_COMMONS_TARGETS.md) — what's
  already in the commons and what others can build
- [Glossary](@/glossary/_index.md) — plain-language definition of scyBorg
  and other terms
- [AGPL-3.0 full text](https://www.gnu.org/licenses/agpl-3.0.html)
- [ORC License full text](https://azoralaw.com/orclicense/)
- [CC-BY-SA 4.0 full text](https://creativecommons.org/licenses/by-sa/4.0/)
