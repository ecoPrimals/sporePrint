+++
title = "Glossary"
description = "Plain-language definitions for every ecoPrimals term. Start here if you're new."
sort_by = "weight"
template = "section.html"
+++

If you're reading this site for the first time, the terminology can be dense.
This page defines every ecosystem term in plain language. Entries are grouped
by category and alphabetized within each group.

---

## Core Terms

**baseCamp**
: The cross-spring paper program. Each baseCamp paper is an executable
  scientific study — code you can run, not a PDF you read. {{ total_stat(stat="basecamp_papers") }} papers across
  6 research domains. See [Science](@/science/_index.md).

**ecoPrimals**
: The umbrella name for the entire ecosystem — primals, springs, products,
  and infrastructure. Three GitHub organizations:
  [ecoPrimals](https://github.com/ecoPrimals) (infrastructure),
  [syntheticChemistry](https://github.com/syntheticChemistry) (springs),
  [sporeGarden](https://github.com/sporeGarden) (products).

**gate**
: A machine (physical computer) running one or more primals. Your laptop,
  your basement server, a Raspberry Pi — each is a gate. Used internally;
  external docs prefer "machine" or "node."

**primal**
: A standalone, statically-linked Rust binary that provides one domain
  capability — cryptography, networking, GPU math, storage, UI, etc.
  Primals communicate over JSON-RPC and compose into larger systems.
  Think of them as Unix-philosophy tools that talk to each other.
  See [Primal Catalog](@/architecture/PRIMAL_CATALOG.md).

**product**
: An emergent composition of primals that does something useful — like a
  chemical reaction product. Not commercial software for sale. helixVision
  (protein structure prediction) is what happens when you compose barraCuda +
  coralReef + toadStool + NestGate and point them at genomic data.
  See [Products](@/products/_index.md).

**sovereign** (self-hosted, cloud-independent)
: Runs entirely on your own hardware. No cloud account, no institutional
  access, no data leaves your machine. "Sovereign computing" means you own
  the compute — not a political statement, a technical property.

**sporePrint**
: This website. Named after the mycological technique: press a mushroom cap
  to paper to leave a permanent record of the species. sporePrint is the
  permanent, public record of the ecoPrimals ecosystem.

**spring**
: A domain-specific validation environment that composes primals and tests
  them against published scientific results. Springs are acceptance tests
  for the ecosystem. hotSpring validates physics, wetSpring validates
  microbiology, etc. Not related to the Java Spring Framework.
  See [Spring Catalog](@/architecture/SPRING_CATALOG.md).

---

## Architecture

**cascade**
: The git-push-triggered deployment pipeline. A push from any gate
  propagates: gate → Forgejo (sovereign) → GitHub (mirror) → site rebuild
  (systemd timer on golgiBody-ext). Cascade connectivity is proven every
  time a gate pushes successfully to both remotes.

**Dark Forest**
: biomeOS's zero-metadata-leakage discovery protocol. Beacons are
  indistinguishable from random noise to anyone without the family key.

**deploy graph**
: A TOML manifest that describes which primals to start, in what order,
  and on which machine. Declarative deployment, not imperative scripting.

**gate enrollment**
: The process by which a new machine joins the sovereign mesh.
  Steps: SSH access → preflight validation → NUCLEUS deploy → WireGuard
  peer exchange → cascade connectivity. Once enrolled, the gate can
  participate in federation and receive binary updates from plasmidBin.

**niche**
: A biomeOS BYOB (Bring Your Own Binaries) deployment configuration.
  Defines which primals run on a given machine.

**NUCLEUS**
: The standard composition of primals on a gate. The full ecosystem has 16 primals;
  a typical gate deploys 13 based on its role and hardware profile. Built from three
  atomic layers: Tower (crypto + networking), Node (Tower + compute),
  Nest (Tower + storage). Binaries are sourced from plasmidBin, verified
  via BLAKE3 checksums, and coordinated by biomeOS.
  See [NUCLEUS Architecture](@/architecture/NUCLEUS_ARCHITECTURE.md).

**Neural API**
: biomeOS's semantic capability routing. Callers request capabilities
  (`crypto.sign`, `ai.chat`) instead of addressing primals by name.
  The caller never knows which primal handled the request.

**Plasmodium**
: Multi-machine collective where bonded NUCLEUS instances share
  capabilities without a central coordinator. Named for the slime mold
  life stage where individual cells merge into a unified organism.

**Tower / Node / Nest (atomics)**
: The three building blocks of NUCLEUS composition.
  **Tower** = BearDog (crypto) + Songbird (networking) — produces HTTPS.
  **Node** = Tower + ToadStool + barraCuda — adds compute.
  **Nest** = Tower + NestGate — adds storage.

---

## Binary & Deployment

**BYOB** (Bring Your Own Binaries)
: Deployment model where you download pre-built primal binaries and
  compose them yourself via deploy graphs. No compilation required.

**ecoBin**
: The quality standard for primal binaries: pure Rust, musl-static linked,
  stripped, no C dependencies.

**genomeBin**
: A deployable primal binary that meets ecoBin standards. Packaged by
  sourDough, distributed via plasmidBin.

**plasmidBin**
: The binary distribution repository. Pre-built, checksummed primal
  binaries ready to download and run. See [Deployment Model](@/architecture/DEPLOYMENT_MODEL.md).

**UniBin**
: One binary, multiple modes. A primal exposes subcommands
  (`petaltongue ui`, `petaltongue tui`, `petaltongue web`) instead of
  shipping separate executables.

---

## K-Derm (Cell Envelope Topology)

**K-Derm**
: The cell envelope topology model for sovereign infrastructure. Named
  after Gram-staining biology (monoderm/diderm bacteria), K-Derm defines
  how gates are layered with membranes, periplasmic spaces, and outer
  boundaries. Replaces ambiguous "inner/outer" terminology with absolute
  positions named from inside out.
  See [K-Derm Architecture](@/architecture/KDERM_DIDERM_ARCHITECTURE.md).

**cytoplasm**
: The innermost zone — the gate itself running NUCLEUS. All primal IPC
  happens here over Unix Domain Sockets. No network exposure.

**plasma membrane**
: The gate's firewall boundary (UFW/nftables). Mediates all traffic
  leaving the cytoplasm. LAN gates (ironGate, sporeGate) validate this
  layer directly.

**periplasm**
: The space between the gate firewall and the outer membrane (VPS).
  Carries WireGuard overlay traffic, RustDesk relay, and routing
  telemetry. WAN gates (flockGate) validate this layer end-to-end
  because their traffic must traverse real internet to reach golgi.

**outer membrane**
: The VPS-facing boundary (golgiBody-ext). Runs Caddy TLS termination,
  sporePrint hosting, TURN/relay services. Public-facing but still
  sovereign — no third-party cloud.

**extracellular**
: The public internet beyond the outer membrane. GitHub mirrors, CDN,
  DNS — anything outside sovereign control. Information degrades as it
  moves outward through K-Derm layers.

**monoderm**
: Single-membrane topology. Gate directly on network with no VPS relay.
  Structure: cytoplasm → plasma membrane → environment.

**diderm**
: Double-membrane topology with a periplasmic space. The production
  configuration for gates that route through a VPS.
  Structure: cytoplasm → plasma → periplasm → outer membrane →
  extracellular.

**WireGuard mesh**
: The sovereign encrypted overlay network connecting all gates. Hub-and-
  spoke topology with golgi (10.13.37.1) as hub. Each gate gets a stable
  overlay IP (e.g., flockGate = 10.13.37.6). Provides identity,
  encryption, and connectivity across heterogeneous physical transports
  (fiber, cellular, VPS, P2P).

---

## Licensing

**scyBorg**
: Shorthand for the triple-copyleft license stack:
  **AGPL-3.0-or-later** (code) + **ORC** (game mechanics) +
  **CC-BY-SA 4.0** (documentation). Three independent nonprofits (FSF,
  Open RPG Creative Foundation, Creative Commons) enforce the licenses.
  No single entity — including the creator — can revoke any of them.
  Any derivative work must share alike.
  See [scyBorg Licensing](@/methodology/SCYBORG_LICENSING.md) for why
  all three exist, and [Knowledge Commons](@/methodology/KNOWLEDGE_COMMONS_TARGETS.md)
  for what's in the commons.

---

## Concepts

**constrained evolution**
: The core methodology. Environmental constraints (pure Rust, no C
  dependencies, JSON-RPC isolation) drive primal specialization the way
  natural selection drives biological adaptation. Not designed — emerged.
  See [Constrained Evolution](@/methodology/CONSTRAINED_EVOLUTION_FORMAL.md).

**guideStone**
: A self-verifying build artifact. A tarball containing binaries and their
  own test suite — run it, and the artifact proves its own correctness.
  No trust required; the evidence is in the artifact.
  See [guideStone](@/guidestone/_index.md).

**K-NOME** (Knowledge-Numeric Observed & Mentored Evolutionary Programming)
: AI-assisted development methodology. The human provides domain expertise
  and selective pressure; the AI handles implementation. One human, one
  AI tool (Cursor IDE), constrained evolution.

**Lysogeny Protocol**
: The strategy for making proprietary gates irrelevant. Trace proprietary
  tools to their published math, reimplement under AGPL-3.0, validate
  against published results, publish. Adoption dissolves the proprietary
  barrier permanently.

**metalForge**
: The heterogeneous hardware context — CPU, GPU, NPU substrates on a
  given machine. ToadStool discovers and routes workloads across them.

**mito-beacon**
: The transport signal that primals emit on IPC connections to declare
  their protocol. A 2-byte prefix (`0xEC 0x01` for clear NDJSON
  JSON-RPC) sent before the first message. Primals that accept the
  signal are "genetics-layer wired" — they can negotiate protocol
  without ambiguity.

**Novel Ferment Transcript (NFT)**
: A memory-bound digital object with cryptographic provenance — not the
  blockchain speculation kind. Fermentation metaphor: the transcript is
  the record of a biological process.

**Paper Parity**
: The standard that a spring's results must match published peer-reviewed
  results. If hotSpring says a Yukawa MD simulation has 0.000% energy
  drift, that must match or exceed the paper it reproduces.

**plasmid.fetch**
: The binary acquisition mechanism. Gates fetch pre-built primal binaries
  from the VPS depot (`membrane.primals.eco/depot/`) over WAN. Binaries
  are BLAKE3-checksummed against `checksums.toml`. Gates never compile
  from source — they are consumers of the sovereign build authority.

**riboCipher**
: The transport signal standard for IPC protocol negotiation. Defines
  byte prefixes that primals use to declare their intended protocol on
  connection. Enables multiplexing, version negotiation, and rejection
  of unsignalled connections.

**RootPulse**
: Distributed version control as an emergent behavior of the Memory &
  Attribution Stack (rhizoCrypt + loamSpine + sweetGrass + NestGate +
  BearDog + Songbird). Not designed as a DVCS — the capability emerged
  from composing primals.
