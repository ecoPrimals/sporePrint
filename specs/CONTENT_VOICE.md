# sporePrint Content Voice

How to write for sporePrint. These standards apply to all content in `content/`.

## Identity

sporePrint is **human-facing**. wateringHole is the dev-facing shared context repo. The voice difference matters:

| | sporePrint | wateringHole |
|---|-----------|-------------|
| Audience | Scientists, PIs, students, hobbyists, compliance reviewers | AI agents and human developers |
| Voice | Narrative, explanatory, accessible | Terse, canonical, machine-parseable |
| Goal | Understand what the ecosystem IS and what it does | Know what to build next and how |

## Narrative Standards

### Identity-first, not feature-list

Bad (technical README style):
> BearDog provides Ed25519, ECDSA, ChaCha20-Poly1305, AES-GCM, X25519, HMAC, and Pure Rust Tor v3.

Good (identity + origin + what-it-means):
> BearDog is the cryptographic spine of ecoPrimals. Every operation that requires signing, encrypting, hashing, key derivation, or identity verification is delegated to BearDog via JSON-RPC. No other primal implements its own crypto.

The structure for each primal/spring section:
1. **What it IS** (organ metaphor, identity)
2. **Why it exists** (origin story — what problem forced its creation)
3. **What Pure Rust means here** (what the constraint revealed)
4. Key data (tables, primitive catalogs) as supporting evidence

### "What the constraint revealed"

The Pure Rust / zero-C-dependency constraint is not a limitation — it is the evolutionary pressure that forced specific innovations. Always frame it as discovery:

> Eliminating CUDA forced Vulkan, which exposed `SHADER_F64` on consumer GPUs. Eliminating vendor compilers forced coralReef, which now compiles 93/93 cross-spring WGSL shaders to native GPU binaries.

### Replication, not endorsement

**Critical rule.** Researchers listed on sporePrint are sources being reproduced, not collaborators or endorsers.

- Use: `**Reproduces work by**: Michael Murillo (CMSE, MSU)`
- Use: `**Literature Anchor**: Einstein (1905, mass-energy equivalence)`
- Never: `**Faculty**: ...`
- Never: `Faculty Already Evaluating This Work`
- Never: `Faculty validation network`

The framing: springs independently reimplement published, peer-reviewed science in Rust, with automated cross-validation against the original results. This is replication with rigor and full provenance, not collaboration.

When describing the relationship to published work: "published papers define the acceptance criteria — the springs reproduce their results independently."

### Metrics over grades

| Don't use | Use instead |
|-----------|-------------|
| Grade: A+ LEGENDARY | Lines: ~25K Rust (91 methods, 72 endpoints) |
| Grade: S+ Tier | Tests: 1,763 passing, Coverage: 85% |
| TOP 1% CERTIFIED | Safety: `#![forbid(unsafe_code)]`, zero clippy warnings |

Quantifiable metrics that a reader can verify: lines of code, test count, test pass rate, coverage percentage, crate count, unsafe block count, clippy warning count.

### PII rules

- No legal names on site content, templates, or structured data (JSON-LD, meta tags)
- No email addresses, phone numbers, or physical addresses
- No "Dr." honorific — just name and affiliation
- No locations, home addresses, employment history
- No personal background beyond what's published
- Published researchers listed as third-party citations stay (they are sources being reproduced)

### Identity model

Four identities, each for its own domain:

| Identity | Domain | Use on sporePrint |
|----------|--------|-------------------|
| **ecoPrimals** / **ecoPrimal** | Developer, code, infrastructure | Technical content, JSON-LD for code sections, thesis author attribution |
| **attsi** | Philosopher, essayist | atlasHugged essays, story essays, JSON-LD for philosophy/story sections |
| **Tamison** | Online handle, community | Discord, external community interaction — not used on sporePrint |
| *(legal name)* | Real life only | Never appears on site. Not in content, not in metadata, not in structured data |

When the thesis or technical content says "the author," that refers to ecoPrimal. When atlasHugged or story essays say "I," that voice is attsi.

### Tone

- Direct and precise, not marketing-speak
- If a claim has a number, the number should be verifiable (`cargo test`, `cargo run --bin validate_*`)
- If something doesn't work yet, say so explicitly (see "What We Honestly Can't Do Yet" in FOR_FACULTY_AND_PIS.md)
- Tufte-esque: every element justifies its space. No filler headings, no decorative sections
- Emojis are semantic anchors for scanning, not decoration. 2-emoji pairs per entity, per PRIMAL_EMOJI_STANDARD

## Front Matter Conventions

Every content page must have:

```toml
+++
title = "Page Title"
description = "One-line description used in card listings and meta tags"
date = 2026-04-01

[taxonomies]
primals = ["beardog", "barracuda"]
springs = ["hotspring"]
+++
```

- `title` — displayed in nav, breadcrumbs, cards, browser tab
- `description` — displayed on section listing cards, search results, meta description
- `date` — when the content was last substantively updated
- `[taxonomies]` — which entities this page references (see `TAXONOMY_STANDARD.md`)

Science pages additionally use:

```toml
[extra]
paper_number = 7
domain = "Physics and Materials"
```

## Section Conventions

| Section | Sort | What goes here |
|---------|------|---------------|
| architecture | title | System design, catalogs, inventories, models |
| audience | title | Role-based entry points, capability comparisons |
| science | title | baseCamp papers, evidence maps, roadmaps |
| methodology | title | How the ecosystem was built, how to extend it |
| technical | title | Hardware, GPU pipeline, grants, teaching, institutional |
| guidestone | weight | Verification class, deployment artifacts |
| products | title | sporeGarden user-facing applications |
