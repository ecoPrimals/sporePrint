+++
title = "lithoSpore — Targeted GuideStone Deployment"
description = "USB-deployable, self-verifying scientific artifacts. First instance: LTEE reproduction (75/75 checks). pseudoSpore pipeline maturing toward external science."
date = 2026-07-31

[taxonomies]
primals = ["barracuda", "toadstool", "nestgate", "biomeos", "petaltongue"]
springs = ["groundspring", "wetspring", "hotspring", "neuralspring"]
trails = ["reproducibility"]

[extra]
maturity = "implemented"
+++

## What This Is

lithoSpore is the ecosystem's deployment system for **self-verifying scientific artifacts**. A lithoSpore is a USB drive (or network-deployable archive) that carries validated science, its data, its tools, and its provenance chain — everything needed to independently reproduce results on any machine, with zero dependencies and no internet.

The first instance targets the **Barrick Lab at UT Austin** — the continuation of Richard Lenski's Long-Term Evolution Experiment (LTEE), the longest-running evolutionary biology experiment in history.

**Repository**: [sporeGarden/lithoSpore](https://github.com/sporeGarden/lithoSpore)

---

## Spore Taxonomy

The ecoPrimals ecosystem uses a biological metaphor for deployment artifacts. Each class adds capability:

| Class | Self-sufficient? | Size | What it carries |
|-------|:---:|------|-----------------|
| **coldSpore** | No | < 1 KB | Static marker + frozen data snapshot |
| **liveSpore** | Partial | ~KB | + `liveSpore.json` journal + `./refresh` update mechanism |
| **pseudoSpore** | No | KB–MB | + Braids, receipts, derivation configs, provenance — *proves* the mountain was climbed |
| **lithoSpore** | **Yes** | MB–GB | + Python runtime + Rust binaries + full data — carries everything to reproduce on its own |

*The spore can't carry the mountain, but it proves the mountain was climbed.* A pseudoSpore provides the proof. A lithoSpore provides the proof **and** the tools to re-climb it.

---

## Current Status

- **75/75 checks** across 7 science modules — all PASS at Tier 2 (Rust)
- **6 published papers reproduced**: Wiser 2013, Barrick 2009, Good 2017, Blount 2008/2012, Burden 2024, Tenaillon 2016
- **Anderson disorder framework** applied to LTEE fitness data — GOE/Poisson eigenvalue statistics
- **Single 5.1 MB binary** (musl-static) — no runtime dependencies
- **Cross-platform**: Ubuntu, Alpine, Fedora, Debian, read-only FS, Windows (7.9 MB)
- **108 unit + 16 integration + 15 chaos/fault-injection tests**

---

## Three Operating Modes

lithoSpore discovers its environment at runtime and adapts:

| Mode | Network | Discovery | Validation Tier |
|------|---------|-----------|:---:|
| **Standalone** | None — airgapped USB | No primals | 1–2 |
| **LAN** | Local network | env vars / UDS socket | 2 |
| **Geo-delocalized** | Remote / WAN | songBird TURN relay via cellMembrane | 2–3 |

`probe_operating_mode()` records the mode in `liveSpore.json`, the append-only provenance journal that tracks every validation run (BLAKE3-hashed hostnames, no PII).

---

## Three-Tier Validation

| Tier | Runtime | What runs |
|------|---------|-----------|
| **1** | Python notebooks | numpy/scipy baselines — reference implementations |
| **2** | musl-static Rust | In-process `run_validation()` — 5.1 MB binary, zero dependencies |
| **3** | NUCLEUS primals | Tier 2 + provenance trio (rhizoCrypt DAG, loamSpine attestation, sweetGrass braid) |

Tier 2 proves the science. Tier 3 proves the provenance chain is sovereign.

---

## pseudoSpore Lifecycle

pseudoSpores are lightweight proof artifacts emitted by springs and consumed by lithoSpore:

```
Spring validates → litho emit-pseudospore → pseudoSpore archive
                                              ↓
                         litho ingest-pseudospore → registry.toml
                                              ↓
                         litho promote → lithoSpore candidate
```

| Command | Purpose |
|---------|---------|
| `litho emit-pseudospore` | Create archive from validated module state |
| `litho ingest-pseudospore` | Validate + register in `pseudospores/registry.toml` |
| `litho fetch-pseudospore` | Remote download + validate |
| `litho audit` | 10-check pre-handoff validation |
| `litho promote` | pseudoSpore → lithoSpore candidate |

Browse available pseudoSpores in the [pseudoSpore Gallery](@/lab/spores/_index.md).

---

## Deployment Vision

The target architecture makes lithoSpore artifacts accessible through the mesh:

1. **USB handoff** — plug in, run `./validate`, done. Zero install, zero internet.
2. **Pepti depot** — pre-built ecobins distributed via the sovereign depot (`plasmidBin`)
3. **Mesh-accessible** — sporePrint hosts the gallery; songBird routes capability calls to the serving gate
4. **Historical provenance** — `liveSpore.json` journals accumulate across validations, building a chain of independent reproductions

The chassis is domain-agnostic. New guideStones require only a `scope.toml`, `data.toml`, and domain-specific module crates — the deployment infrastructure, validation harness, and provenance machinery are reused.

---

## Seven Science Modules (LTEE Instance)

| # | Module | Paper | Checks |
|---|--------|-------|:------:|
| 1 | ltee-fitness | Wiser 2013 (Science) | 8/8 |
| 2 | ltee-mutations | Barrick 2009 (Nature) | 7/7 |
| 3 | ltee-alleles | Good 2017 (Nature) | 20/20 |
| 4 | ltee-citrate | Blount 2008/2012 (PNAS/Nature) | 11/11 |
| 5 | ltee-biobricks | Burden 2024 (ACS SynBio) | 15/15 |
| 6 | ltee-breseq | Tenaillon 2016 (Nature) | 7/7 |
| 7 | ltee-anderson | Anderson localization framework | 7/7 |

---

## See Also

- [Lab: lithoSpore](@/lab/lithospore.md) — detailed science and module documentation
- [GuideStone: lithoSpore Artifact](@/guidestone/lithospore_artifact.md) — USB anatomy and verification
- [pseudoSpore Gallery](@/lab/spores/_index.md) — available pseudoSpore artifacts
- [Biological Validation (Thesis Ch. 14)](@/thesis/14_biological_validation.md) — proposed LTEE sequencing
