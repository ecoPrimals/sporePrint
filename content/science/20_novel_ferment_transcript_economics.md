+++
title = "Novel Ferment Transcript Economics"
description = "Economics x Provenance — radiating attribution through provenance chains, novel ferment transcripts. ludoSpring. 89/89 checks."
date = 2026-03-17

[extra]
paper_number = 20
domain = "Economics and Provenance"

[taxonomies]
primals = ["beardog", "biomeos", "loamspine", "rhizocrypt", "sweetgrass"]
springs = ["ludospring"]
+++

**Status**: Active | **Date**: March 16, 2026
**Depends on**: Papers 17 (Game Design), 18 (RPGPT), 19 (Games@Home)
**Validated by**: {{ entity(name="ludospring") }} exp061_fermenting (89/89 checks)
**License**: AGPL-3.0-or-later

---

## Abstract

This paper connects the sunCloud economic model (radiating attribution through
provenance chains) to its concrete implementation via the provenance trio
({{ entity(name="rhizocrypt") }}, {{ entity(name="loamspine") }}, {{ entity(name="sweetgrass") }}) and {{ entity(name="beardog") }} cryptographic signing. We
define the **{{ entity(name="nft") }}** (NFT) — a memory-bound digital object
whose value derives from accumulated history rather than artificial scarcity.
We show how the same architecture serves gaming, collectibles, scientific
chain-of-custody, and sensitive data systems, and how the optional public chain
anchor activates radiating attribution without coupling to cryptocurrency.

---

## 1. From Concept to Implementation

### The Evolution

The economic ideas in {{ entity(name="ecoprimals") }} evolved through three phases:

| Phase | Date | Document | Key Concept |
|-------|------|----------|-------------|
| 1. Ethos | Jul 2025 | `LATENT_VALUE_ECONOMY.md` | Value from significance, not scarcity |
| 2. Model | Jul 2025 | `SUNCLOUD_ECONOMIC_MODEL.md` | Radiating attribution, metabolic mandate |
| 3. Implementation | Mar 2026 | This paper + exp061 | Working code: fermenting system, trio integration |

The original vision described "memory-bound objects" and "radiating attribution"
as abstract concepts. Now they are running code with 89 validation checks, real
provenance trio integration, and a defined IPC protocol for composable deployment.

### What Changed

The core insight has not changed: **value comes from history, not scarcity**.
What evolved is the understanding that:

1. **The provenance trio is the mechanism**, not a separate layer. Attribution
   ({{ entity(name="sweetgrass") }}), memory ({{ entity(name="rhizocrypt") }}), and ownership ({{ entity(name="loamspine") }}) are not
   metadata bolted onto objects — they ARE the object.

2. **{{ entity(name="beardog") }} makes it cryptographic**, not just data. Ed25519 signatures on
   every vertex, certificate, and braid make the chain verifiable without any
   blockchain.

3. **The public anchor is optional**, not foundational. Normal operation is
   local, fast, and free. The blockchain is for global persistence and radiating
   attribution activation — a feature, not a requirement.

4. **The architecture is domain-agnostic**. The same code that tracks a
   tournament sword tracks a DNA sample. Same DAG, same fraud detection,
   different vocabulary.

---

## 2. The Novel Ferment Transcript

### Definition

A {{ entity(name="nft") }} is the complete provenance record of a digital
or physical object:

```
NFT = Certificate (who owns it)
    + DAG (what happened to it)
    + Braids (who contributed to it)
    + Signatures (cryptographic proof)
    + Anchor (optional global persistence)
```

The biological analogy: fermentation transforms simple sugars into complex,
valuable products (wine, cheese, kimchi). The culture accumulates character
over time. The process is irreversible. A {{ entity(name="nft") }} transforms
raw data (mint) through use (trade, play, study, display) into something with
accumulated, verifiable meaning.

### What Makes It Novel

| Property | Cryptocurrency NFT | {{ entity(name="nft") }} |
|----------|-------------------|--------------------------|
| Identity | Blockchain token ID | {{ entity(name="loamspine") }} certificate (DID-based) |
| History | Transaction log | Full DAG (every action, not just transfers) |
| Attribution | None (wallet addresses) | W3C PROV-O chain (who, what, when, why) |
| Crypto binding | Chain consensus | {{ entity(name="beardog") }} Ed25519 (same strength, zero cost) |
| Mint cost | Gas fee ($1-$100+) | Zero (local operation) |
| Trade cost | Gas fee | Zero (local operation) |
| Speed | Block time (seconds to minutes) | Instant (local DAG append) |
| Currency coupling | Inherent (ETH, SOL, etc.) | None — explicitly decoupled |
| Physical bridge | Requires oracle service | Same certificate for physical + digital |
| Lending | Not supported | Native ({{ entity(name="loamspine") }} loan lifecycle) |
| Attribution chain | Not tracked | Full PROV-O derivation chain |
| Public proof | Always on-chain | Optional anchor hash |

### Functional NOT a Currency

This distinction is not cosmetic — it is architectural:

- No exchange rate is encoded in the protocol
- No fungible subdivision is possible
- No mining or staking exists
- No gas mechanism gates operations
- No financial entity issues or backs the transcript
- The anchor hash is a proof, not a transaction
- Value flows through radiating attribution, not token transfer

The {{ entity(name="nft") }} enables everything useful about NFTs (provenance,
ownership, trading, history) while eliminating everything harmful (speculation,
gas fees, environmental cost, currency coupling, rug-pull risk).

---

## 3. The sunCloud Connection

### Radiating Attribution: From Theory to Code

The sunCloud model (2025) described radiating attribution abstractly:

> "Upon receipt of revenue, an autonomous sunCloud process is triggered. It
> consults the {{ entity(name="sweetgrass") }} Braid associated with the licensed discovery. It
> then radiates the value back through the entire attribution chain."

Now this has a concrete implementation path:

```
1. Object is created ──► sweetGrass braid records creator
2. Object accumulates history ──► each event adds agents to the chain
3. Object is anchored publicly ──► state hash published to public ledger
4. Value event occurs ──► sale, license, citation, exhibition
5. sunCloud consulted ──► reads the sweetGrass attribution chain
6. Value radiates ──► proportional distribution to every contributor
```

The **public anchor** is the activation event. Without it, attribution exists
locally and is cryptographically valid, but has no public proof. The anchor
transforms local provenance into globally-attestable provenance, which is
what sunCloud needs to distribute value trustlessly.

### The Value Cycle, Concretized

**Phase 1 — Latent Value (Local)**

An in-game sword is minted. Alice plays 200 hours with it. It kills 47 bosses.
It wins a tournament. Bob inspects it and marvels at the history. All of this
is recorded in the {{ entity(name="rhizocrypt") }} DAG, attributed via {{ entity(name="sweetgrass") }}, and certificated
in {{ entity(name="loamspine") }}. The sword has immense value — but it is latent, known only to
Alice and anyone she shows the local data to.

**Phase 2 — Activation (Anchor)**

Alice decides to sell the sword. She (or the marketplace) anchors the {{ entity(name="loamspine") }}
state to a public chain. The 32-byte hash is now globally verifiable. Anyone
can confirm "this sword's history is real and cryptographically intact."

**Phase 3 — Radiating Attribution (sunCloud)**

The sword sells. sunCloud reads the attribution chain:
- Alice (owner, 200h of history): primary beneficiary
- The game studio (minted the object): creator attribution
- The skin artist (designed the visual): creative attribution
- The tournament organizer (hosted the event where it won): event attribution
- The engine developers (wrote the math): code attribution

Each receives proportional credit. The sword artist who made the skin 3 years
ago gets a micro-payment when the sword sells today. This is radiating
attribution — value flowing backward through the creation chain.

### How This Differs from the Original Model

The sunCloud model described value flowing from "discoveries" in a {{ entity(name="biomeos") }}.
The fermenting system generalizes this:

| sunCloud (2025) | NFT Economics (2026) |
|----------------|---------------------|
| Scientific discovery | Any valued object (game item, sample, record) |
| {{ entity(name="biomeos") }} IP licensing | Object sale, exhibition, citation |
| {{ entity(name="sweetgrass") }} braids | Same — attribution chain |
| gAIa commons stewardship | {{ entity(name="scyborg") }} licensing (AGPL-3.0 + ORC + CC-BY-SA) |
| Bounties for research | Composable marketplace for objects |

The economics are identical. The scope expanded from science to everything.

---

## 4. Domain Applications

### Gaming: The Sword That Won the Championship

**The problem**: A digital sword is identical to every other copy. The rare
one differs only by an arbitrary counter ("1 of 500"). No authentic history.

**The ferment**: The sword accumulates a {{ entity(name="nft") }}. It records
every kill, every trade, every tournament, every cosmetic change. The sword
that won the championship is provably THE sword. Not "one of" — "the one."

**The economics**: When the sword sells, radiating attribution credits the
game studio, the skin artist, the tournament host, and every previous owner
whose play history made the sword valuable. The artist who designed the skin
receives credit forever, not just at initial sale.

**Validated**: {{ entity(name="ludospring") }} exp061 proves the full lifecycle — mint, trade,
loan, return, consume, achievement tracking, atomic swap — with 89 checks.
exp053 proves fraud detection (12 types) using the same DAG architecture.

### Collectibles: The Card With a Story

**The problem**: Physical trading cards have provenance (condition, tournament
stamps) but it is fragile and forgeable. Digital cards have none.

**The ferment**: A physical card and its digital twin share one {{ entity(name="loamspine") }}
certificate. The card's tournament play is tracked in the digital DAG. The
physical card's condition changes are recorded as {{ entity(name="loamspine") }} metadata updates.
Scanning the physical card reveals its complete digital history.

**The economics**: When the card sells, the original artist, the card printer,
and every tournament organizer in the card's history receive attribution. The
card that traveled through three countries and won two tournaments has a
verifiable story that commands premium — based on authentic significance, not
artificial scarcity.

**Connection**: This is exactly the LOAM_CERTIFICATE_LAYER.md vision, now
with working code (exp061) and a deployment graph (provenance_node_atomic.toml).

### Science: Chain-of-Custody That Cannot Be Forged

**The problem**: Scientific sample provenance relies on paper forms, Excel
spreadsheets, and trust. Samples can be swapped, mislabeled, or contaminated
without detection.

**The ferment**: Every sample gets a {{ entity(name="nft") }} at collection.
Every custody transfer, storage condition change, and analysis step is a
DAG vertex signed by {{ entity(name="beardog") }}. The same orphan-item detection that catches
duped loot in exp053 catches phantom samples in a lab.

**The economics**: When research using the sample is published, radiating
attribution credits the field collector, the transport team, the lab
technician, the analyst, and the PI — proportionally. The person who spent
three days in a swamp collecting the sample gets credited when Nature
publishes the paper five years later.

**Validated**: The DAG isomorphism is proven in exp053 (extraction shooter)
and described in Paper 18 (RPGPT). Same code, different vocabulary:

| DAG Operation | Game | Science |
|---------------|------|---------|
| Object creation | Sword found in dungeon | Sample collected in field |
| Object transfer | Traded to teammate | Handed to lab tech |
| Object transform | Enchanted with rune | Amplified with PCR |
| Audit | No sword without loot vertex | No reads without sample vertex |

### Sensitive Data: Records That Remember Who Looked

**The problem**: Medical records, legal documents, and financial records need
audit trails, access control, and regulatory compliance. Current systems are
centralized, brittle, and opaque.

**The ferment**: The record owner (patient, client, citizen) holds the {{ entity(name="loamspine") }}
certificate. Providers receive loaned access (the native loan lifecycle).
Every access is a DAG vertex. The full access history is a Novel Ferment
Transcript — who looked at what, when, and under what authority.

**The economics**: When aggregated, de-identified research uses the data,
radiating attribution credits the original data subject. You contributed
your health data to a study — {{ entity(name="sweetgrass") }} records your contribution, and
sunCloud distributes proportional credit when the study generates value.

---

## 5. The scyBorg Integration

Novel Ferment Transcripts carry licensing metadata via the {{ entity(name="scyborg") }} framework:

```
Code layer    →  AGPL-3.0-or-later  →  enforced by source availability
Mechanics     →  ORC                →  enforced by attribution
Creative      →  CC-BY-SA 4.0      →  enforced by share-alike derivation
```

The provenance trio provides machine-verifiable compliance:

- **{{ entity(name="sweetgrass") }}** records the BY (attribution)
- **{{ entity(name="rhizocrypt") }}** records the SA (derivation chain for share-alike)
- **{{ entity(name="loamspine") }}** issues the license certificate (immutable proof of terms)

A derivative work inherits the share-alike obligation automatically because
{{ entity(name="rhizocrypt") }}'s DAG links it to the parent. The derivation is structural, not
contractual — you cannot create a derivative without the DAG recording it.

---

## 6. Implementation Status

### Working (March 2026)

| Component | Status | Where |
|-----------|--------|-------|
| Certificate lifecycle (mint, trade, loan, return) | Done | loam-spine-core v0.8.0 |
| Trading protocol (offer, accept, reject, cancel, swap) | Done | loam-spine-core v0.8.0 |
| Object memory (append event, get timeline, PROV-O export) | Done | sweet-grass-core v0.7.3 |
| DAG-based history tracking | Done | rhizo-crypt-core v0.13.0-dev |
| Cosmetic metadata schema | Done | exp061_fermenting |
| Composable IPC protocol | Done | exp061_fermenting/protocol.rs |
| Deployment graph (Tower + Trio) | Done | provenance_node_atomic.toml |
| Fraud detection (12 types) | Done | exp053 |

### Needed (Near-term)

| Component | Owner | Priority |
|-----------|-------|----------|
| {{ entity(name="beardog") }} signing on all operations | {{ entity(name="beardog") }} team | High |
| Public chain anchor entry type | {{ entity(name="loamspine") }} team | Medium |
| Owner inventory query (`list_by_owner`) | {{ entity(name="loamspine") }} team | Medium |
| Cross-session derivation links | {{ entity(name="rhizocrypt") }} team | Medium |
| License-aware attribution notices | {{ entity(name="sweetgrass") }} team | Medium |
| Radiating attribution calculator | {{ entity(name="sweetgrass") }} + sunCloud | Low (Phase 4) |

### Future (Long-term)

| Component | Description |
|-----------|-------------|
| sunCloud integration | Autonomous value distribution from anchored transcripts |
| Multi-chain anchoring | ETH + BTC + sovereign chain redundancy |
| Physical-digital bridge | NFC/QR scan linking physical objects to certificates |
| Marketplace protocol | Composable marketplace as {{ entity(name="biomeos") }} graph |

---

## 7. Connection to Other Papers

| Paper | Connection |
|-------|-----------|
| 01 (Anderson-QS) | Microbial ecology math underlies fermenting/culture metaphor |
| 07 (Sovereign WDM) | Sovereign compute for anchor verification without cloud |
| 12 (Immuno-Anderson) | Medical record provenance as sensitive data application |
| 13 (Sovereign Health) | Patient-owned records via {{ entity(name="loamspine") }} lending |
| 17 (Game Design) | exp061 validates the full game item lifecycle |
| 18 (RPGPT) | DAG isomorphism: game = science = sensitive data |
| 19 (Games@Home) | Distributed computation for federated marketplace |

---

## 8. The Philosophical Core

The LATENT_VALUE_ECONOMY.md asked: "How do we unlock the value that already
exists?" The SUNCLOUD_ECONOMIC_MODEL.md answered: "Through radiating attribution."

The {{ entity(name="nft") }} is the vessel that carries both the value and the
attribution chain. It is the concrete object that makes the abstract economics
work. Not a token that represents value — a transcript that IS the value, because
it carries the irreversible, cryptographically-bound history of everything that
happened to the object.

The meme, the in-game collectible, the trading card, the DNA sample, the
medical record — each is a {{ entity(name="nft") }}. Each ferments through use.
Each carries its attribution chain. Each can optionally anchor to global
persistence. And when value flows, it radiates back through every contributor.

This was the goal from the beginning. It evolved, just like the primals.
