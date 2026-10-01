+++
title = "Public Record — Infrastructure for Accountability"
description = "ecoPrimals infrastructure applied to public accountability: git-signed evidence, cryptographic timestamps, and verifiable public records. The same substrate that runs science runs transparency."
date = 2026-09-25
weight = 50

[taxonomies]
primals = ["rhizocrypt", "loamspine", "sweetgrass"]
springs = ["primalspring"]
+++

## The Substrate Serves Accountability

ecoPrimals was built for sovereign scientific computing — reproducible results,
immutable provenance, zero vendor lock-in. The same properties that make it
trustworthy for science make it trustworthy for public records.

{{ entity(name="detroit") }} at **[detroit.primals.eco](https://detroit.primals.eco)** is the first application of
ecoPrimals infrastructure to public accountability: a public evidence library
documenting charter school fraud in Detroit, backed by a
[git-signed repository](https://git.primals.eco/publicRecord/detroit) where
every document is cryptographically timestamped. It is the first deployment of {{ entity(name="guerillagorilla") }}, the legal meta-primal pattern.

---

## Why This Matters

The same principles that govern ecoPrimals science govern ecoPrimals accountability:

| Science | Accountability |
|---------|---------------|
| Reproducible results | Verifiable claims |
| BLAKE3 provenance | Git-signed commits |
| Open source (AGPL-3.0) | Open records (CC-BY-SA 4.0) |
| Peer review | Public verification |
| No cloud dependency | No platform dependency |

The evidence repository can be cloned by anyone. If the website disappears,
the repo rebuilds it. If the repo disappears, any clone rebuilds it.
The infrastructure is the proof.

---

## Infrastructure Used

- **Forgejo** (git.primals.eco) — self-hosted Git with anonymous HTTPS clone
- **Zola** — static site generator (same as this site)
- **Caddy** — TLS, HTTP/2, security headers
- **golgiBody** — automated build-on-push pipeline
- **petalTongue** — planned: dynamic rendering for network subgraphs

---

## What's Documented

The Detroit site maps a charter school fraud network across multiple dimensions:

| Section | What it covers |
|---------|---------------|
| [Network Analysis](https://detroit.primals.eco/network/) | 35+ actors — judges, attorneys, political operatives, institutional enablers |
| [Financial Extraction](https://detroit.primals.eco/analysis/funding-flow/) | $4.9M/year in public funding, 72.67% extraction rate through management company |
| [RICO Pattern](https://detroit.primals.eco/analysis/rico-pattern/) | Enterprise structure analysis — 8 predicate acts across coordinated network |
| [Historical Pattern](https://detroit.primals.eco/analysis/historical-pattern/) | Two eras of machine politics — Tammany Hall to Power Broker |
| [Evidence Library](https://detroit.primals.eco/evidence/) | FOIA responses, court records, financial documents — timestamped and content-addressed |
| [Timeline](https://detroit.primals.eco/timeline/) | Chronological sequence from 2005 convictions through 2026 charter expansion |
| [Source Registry](https://detroit.primals.eco/sources/) | Every external database cited, with access instructions |
| [Credential Audit](https://detroit.primals.eco/analysis/credential-audit/) | Systematic verification of teacher credentials via MOECS public search |

---

## Verify Everything

The [detroit.primals.eco/validate](https://detroit.primals.eco/validate/) page
links to every public database used in the investigation. Every factual claim
can be independently verified. That's the standard.

> *"Every fact is cited. Every document is timestamped. Every letter is open.
> Verify everything."*
