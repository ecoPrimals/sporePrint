+++
title = "ecoPrimals Ecosystem Architecture: From Binary to Bonding"
description = "UniBin/ecoBin/genomeBin ladder, NUCLEUS deployment, bonding model, Neural API"
date = 2026-03-17

[taxonomies]
primals = ["beardog", "barracuda", "biomeos", "loamspine", "nestgate", "rhizocrypt", "songbird", "squirrel", "sweetgrass", "toadstool"]
+++

**Status**: Working paper  
**Lineage**: Technical companion to `CONSTRAINED_EVOLUTION_FORMAL.md`  
**Last Updated**: February 7, 2026

---

## Abstract

This paper describes the architecture of the {{ entity(name="ecoprimals") }} ecosystem — a self-hosted, cloud-independent computing platform composed of autonomous services ("primals") that coordinate through runtime capability discovery. The architecture emerged from constrained evolution within Rust's type system, not from a priori design. It spans four levels of abstraction: the binary structure standard ({{ entity(name="unibin") }}), the portability standard ({{ entity(name="ecobin") }}), the deployment standard ({{ entity(name="genomebin") }}), and the composition architecture ({{ entity(name="nucleus") }}). A chemistry-inspired bonding model describes how {{ entity(name="nucleus") }} deployments interact across physical machines and trust boundaries. The {{ entity(name="neuralapi") }} provides semantic orchestration, and the {{ entity(name="darkforest") }} protocol provides zero-metadata-leakage security.

The central claim is that this architecture was not designed top-down. It emerged from the bottom up, through the constrained evolution methodology described in `CONSTRAINED_EVOLUTION_FORMAL.md`. The architecture is therefore evidence that constraint-driven development produces coherent, layered systems through specialization rather than planning.

---

## 1. Foundation: Primals and Primitives

### 1.1 What Is a Primal?

A **primal** is a self-contained Rust binary that owns one domain and exposes its capabilities as **primitives** - atomic operations accessible via JSON-RPC 2.0 over platform-agnostic transports. A primal knows only itself: what capabilities it has, how to advertise them, and how to respond to requests. It does not know what other primals exist, what composed systems it participates in, or what the broader ecosystem looks like.

This is not just a design principle - it is enforced by architecture. Primals have zero compile-time coupling. No primal imports another primal's code. No primal references another primal by name in its source. Coordination happens exclusively at runtime, through capability discovery orchestrated by {{ entity(name="biomeos") }}.

### 1.2 What Are Primitives?

Primitives are the atomic operations a primal provides. They are the smallest unit of capability in the ecosystem. Examples:

- **{{ entity(name="beardog") }}** (cryptography): `ed25519_sign`, `x25519_key_exchange`, `aes_256_gcm_encrypt`, `blake3_hash`, `x509_verify_chain`
- **{{ entity(name="songbird") }}** (networking): `tls13_handshake`, `birdsong_broadcast`, `stun_binding`, `udp_hole_punch`
- **{{ entity(name="nestgate") }}** (storage): `storage.put`, `storage.get`, `storage.list`, `discovery.announce`
- **{{ entity(name="toadstool") }}** (compute): `workload.submit`, `gpu.detect`, `tensor.matmul`

A primitive is a function exposed over IPC. It takes structured input and returns structured output. It does not maintain global state, does not require knowledge of the caller, and does not depend on any other primal being present. Primitives are the genes of the ecosystem - small, self-contained units of function that compose into larger behaviors.

### 1.3 How Do Primals Coordinate?

Primals communicate via **JSON-RPC 2.0** over platform-agnostic transports. The transport is discovered at runtime:

| Platform | Primary Transport | Fallback |
|----------|------------------|----------|
| Linux | Unix domain sockets | TCP |
| Android | Abstract sockets | TCP |
| macOS | Unix domain sockets | TCP |
| Windows | Named pipes | TCP |
| Cross-device | TCP | - |

Each primal implements its own IPC independently. There is no shared IPC library. The protocol specification (JSON-RPC 2.0) is the shared standard; implementations are convergent but non-identical across primals. This design decision is deliberate and is discussed in detail in `CONSTRAINED_EVOLUTION_FORMAL.md` §4.3 (convergent evolution under shared constraint).

**{{ entity(name="biomeos") }}** orchestrates coordination. It discovers primals by their capabilities at runtime, maintains a capability registry, and routes semantic requests to the appropriate primal. A caller requests `capability.call("crypto.sign", ...)` and {{ entity(name="biomeos") }} discovers which primal provides that capability, translates the semantic name to the primal's specific method, routes the request, and returns the result. The caller never needs to know which primal handles the request.

---

## 2. The Architecture Ladder

The {{ entity(name="ecoprimals") }} architecture defines three progressive standards for binary construction. Each standard builds on the previous, adding capability without replacing what came before. All ecoBins are UniBins. All genomeBins are ecoBins.

```
UniBin   (structure)    → One binary, multiple modes
  ↓
ecoBin   (portability)  → + Pure Rust, cross-compilation, platform-agnostic IPC
  ↓
genomeBin (deployment)  → + Auto-detection, service integration, health monitoring
```

### 2.1 UniBin - Binary Structure Standard

**Definition**: One binary per primal, multiple operational modes via subcommands.

Every primal produces exactly one executable binary named after itself: `beardog`, `songbird`, `nestgate`, `toadstool`, `squirrel`. Not `beardog-server` or `beardog-cli` - one binary that does everything the primal can do.

**Requirements**:

- **Subcommand architecture**: `beardog serve` starts the service. `beardog status` checks health. `beardog info` reports capabilities. `beardog version` reports version. The binary is the complete interface to the primal.
- **Professional CLI**: `--help` with structured output, `--version`, `--config` for configuration path. Consistent UX across all primals.
- **Configuration hierarchy**: Environment variables override file configuration override defaults. `PRIMAL_NAME_KEY=value` pattern (e.g., `BEARDOG_SOCKET_PATH=/tmp/beardog.sock`).
- **Signal handling**: SIGTERM triggers graceful shutdown. SIGINT triggers graceful shutdown. SIGHUP triggers configuration reload.
- **Structured exit codes**: 0 = success, 1 = general error, 2 = configuration error, 3 = dependency unavailable, 4 = permission denied.

**Why it matters**: {{ entity(name="unibin") }} standardizes the developer and operator experience across the ecosystem. Every primal works the same way from the command line. This consistency is itself an emergent property of constrained evolution - all primals converged on this structure because Rust's `clap` ecosystem and the shared constraint of "must be a single binary" rewarded it.

### 2.2 ecoBin - Universal Portability Standard

**Definition**: {{ entity(name="unibin") }} + Pure Rust (zero C dependencies) + Universal Portability (cross-architecture and cross-platform IPC).

{{ entity(name="ecobin") }} is the portability standard. An {{ entity(name="ecobin") }} binary cross-compiles to any target Rust supports with a single `cargo build --target <triple>` command. No cross-compilation toolchains for C, no pkg-config, no system library dependencies.

**Requirements**:

- **Pure Rust application code**: 100% Rust. Zero C dependencies in the application layer. Cryptographic operations use the RustCrypto suite (not `ring`, which depends on C assembly). TLS uses Pure Rust implementations (not `native-tls` or `openssl-sys`).
- **Static linking**: The binary is self-contained. No shared library dependencies at runtime (beyond libc on platforms that require it).
- **Platform-agnostic IPC**: Runtime transport discovery. The primal detects what IPC mechanisms are available on the current platform and uses the best one. No compile-time assumptions about Unix sockets, named pipes, or any other platform-specific transport.
- **Cross-compilation matrix**: The same source builds for x86_64-linux, aarch64-linux (Raspberry Pi, Android), x86_64-darwin, aarch64-darwin (Apple Silicon), x86_64-windows, and WASM targets.

**Why it matters**: The Pure Rust requirement is the single most consequential architectural constraint in the ecosystem. It eliminated OpenSSL (and all C crypto libraries), which forced the {{ entity(name="toweratomic") }} composition pattern - the headline innovation of the project (see §3.1). It enabled universal cross-compilation, which makes {{ entity(name="genomebin") }} deployment possible. And it provides a security guarantee: the entire application layer is covered by Rust's memory safety, with zero C code that could harbor buffer overflows, use-after-free, or other memory corruption vulnerabilities.

The {{ entity(name="ecobin") }} standard is discussed in biological terms in `CONSTRAINED_EVOLUTION_FORMAL.md` §2.1 (Rust as physics) and §2.2 (the binary as genome). The compiled {{ entity(name="ecobin") }} binary IS the organism - a self-contained genome that runs on any compatible hardware without a runtime environment.

### 2.3 genomeBin - Autonomous Deployment Standard

**Definition**: {{ entity(name="ecobin") }} + deployment wrapper. A self-extracting, auto-installing, service-integrating package that deploys with one command on any system with zero manual configuration.

{{ entity(name="genomebin") }} wraps the {{ entity(name="ecobin") }} with deployment machinery:

**Deployment sequence**:

1. **System detection**: CPU architecture, OS, init system (systemd, launchd, OpenRC), available IPC mechanisms, existing primals, network configuration.
2. **Binary selection**: From the embedded multi-architecture archive, extract the correct binary for the detected platform.
3. **Service integration**: Generate and install the appropriate service unit (systemd .service, launchd .plist, OpenRC script). Configure restart policies, resource limits, and logging.
4. **Health validation**: Start the primal, run health checks, verify capability advertisement, confirm IPC connectivity.
5. **Registration**: If {{ entity(name="biomeos") }} is present, register with the ecosystem. If not, operate standalone.

**Why it matters**: {{ entity(name="genomebin") }} bridges the gap between portability and accessibility. An {{ entity(name="ecobin") }} can run anywhere, but deploying it requires knowing the target system's init system, socket paths, and configuration conventions. A {{ entity(name="genomebin") }} knows all of this itself. The deployment wrapper is the primal's developmental program - the instructions for how to go from a genome (the binary) to a functioning organism in a specific environment.

```
genomeBin (deployment wrapper)
  └── System detection → What kind of environment am I in?
        └── Binary extraction → Deploy the correct ecoBin
              └── Service integration → Register with the OS
                    └── Health validation → Am I alive and functional?
                          └── Ecosystem registration → Can biomeOS find me?
```

---

## 3. NUCLEUS - Atomic Composition Architecture

{{ entity(name="nucleus") }} is the deployment architecture that describes how primals compose into functional units called **atomics**. Each atomic is a validated composition that provides a specific capability layer. The three atomics build upon each other, and their union forms a complete {{ entity(name="nucleus") }}.

### 3.1 Tower Atomic - The Foundation

**Components**: {{ entity(name="beardog") }} (cryptography) + {{ entity(name="songbird") }} (networking)

{{ entity(name="toweratomic") }} is the foundation of every deployment and the architecture's most significant achievement: **Pure Rust HTTPS with zero C dependencies**.

The problem: HTTPS requires both networking (TCP, HTTP protocol, TLS state machine) and cryptography (key exchange, symmetric encryption, certificate verification). Historically, these are tightly coupled in C libraries like OpenSSL. The {{ entity(name="ecobin") }} Pure Rust constraint made OpenSSL impossible. Rather than building a monolithic Pure Rust TLS library, the constrained evolution process produced a composition:

- **{{ entity(name="beardog") }}** provides 72 JSON-RPC cryptographic methods: Ed25519 signing, X25519 key exchange, AES-256-GCM encryption, BLAKE3 hashing, X.509 certificate validation, HKDF key derivation. All Pure Rust via the RustCrypto suite. {{ entity(name="beardog") }} knows nothing about TLS or HTTP.
- **{{ entity(name="songbird") }}** implements the TLS 1.3 state machine (RFC 8446). When it needs a cryptographic operation - generating an ephemeral keypair, performing Diffie-Hellman key agreement, deriving traffic keys, verifying a certificate chain - it calls {{ entity(name="beardog") }} via JSON-RPC over a local Unix socket. {{ entity(name="songbird") }} knows nothing about how crypto is implemented.
- **{{ entity(name="biomeos") }}** orchestrates the composition: starts {{ entity(name="beardog") }} first (security must exist before communication), then {{ entity(name="songbird") }}, which discovers "a primal that provides crypto" and connects to it.

**A TLS 1.3 handshake in {{ entity(name="toweratomic") }}**:

1. {{ entity(name="songbird") }} constructs ClientHello with supported cipher suites
2. {{ entity(name="songbird") }} calls {{ entity(name="beardog") }} → generate ephemeral X25519 keypair → include public key in ClientHello
3. On ServerHello, {{ entity(name="songbird") }} calls {{ entity(name="beardog") }} → X25519 key agreement → shared secret
4. {{ entity(name="songbird") }} calls {{ entity(name="beardog") }} → HKDF-Expand/Extract → handshake keys, traffic keys, finished keys
5. {{ entity(name="songbird") }} calls {{ entity(name="beardog") }} → X.509 chain verification → server certificate validated
6. {{ entity(name="songbird") }} calls {{ entity(name="beardog") }} → HMAC verification → Finished message confirmed
7. Application data: {{ entity(name="songbird") }} calls {{ entity(name="beardog") }} → AES-GCM / ChaCha20-Poly1305 encrypt/decrypt

Each step is a JSON-RPC call with microsecond-scale latency. The composition overhead is negligible compared to network round-trip time.

**Validation results**:

| Metric | Result |
|--------|--------|
| TLS 1.3 validation | 93% (81/87 production sites) |
| Average HTTPS latency | 366ms |
| Cipher suites | AES-128-GCM, AES-256-GCM, ChaCha20-Poly1305 |
| Key exchange | X25519, ECDHE-P256, ECDHE-P384 |
| Certificate types | RSA 2048/4096, ECDSA P-256/P-384 |
| C dependencies | Zero |
| Unsafe code | Zero in production |

This is discussed in biological terms in `CONSTRAINED_EVOLUTION_FORMAL.md` §4.4 (the firefly analogy). {{ entity(name="beardog") }} is the bioluminescent bacterium - it provides the "light" (cryptographic operations), viable in isolation. {{ entity(name="songbird") }} is the insect - it provides the structure (protocol logic), viable in isolation. The "glow" (Pure Rust HTTPS) emerges only from their composition. Neither primal contains the glow.

### 3.2 Node Atomic - Compute Layer

**Components**: {{ entity(name="toweratomic") }} + {{ entity(name="toadstool") }} (compute)

{{ entity(name="nodeatomic") }} extends Tower with universal compute orchestration. {{ entity(name="toadstool") }} provides isomorphic workload execution across CPU, GPU, neuromorphic hardware, WebAssembly, and containers. Its BarraCuda library provides 124 tensor operations with WGSL shaders that run identically on any compute substrate.

**Capabilities added**: `compute.*` (workload execution, GPU detection), `ai.local_inference`, `workload.*` (scheduling, orchestration).

**Deployment**: Tower deploys first, then {{ entity(name="toadstool") }} starts, discovers Tower capabilities, and advertises compute. GPU initialization may take up to 45 seconds for hardware detection and validation.

### 3.3 Nest Atomic - Storage Layer

**Components**: {{ entity(name="toweratomic") }} + {{ entity(name="nestgate") }} (data storage)

{{ entity(name="nestatomic") }} extends Tower with content-addressed persistent storage. {{ entity(name="nestgate") }} stores data identified by its BLAKE3 hash, enabling deduplication, integrity verification, and efficient caching. Storage is encrypted at rest via {{ entity(name="beardog") }} (AES-256-GCM).

**Capabilities added**: `storage.*` (put, get, delete, list, copy, move, quota), `persistence.*`, `provenance.*`.

### 3.4 Full NUCLEUS

**Components**: Tower + Node + Nest + {{ entity(name="squirrel") }} (AI coordination)

A complete {{ entity(name="nucleus") }} runs all atomics coordinated by {{ entity(name="biomeos") }}, with {{ entity(name="squirrel") }} providing AI model coordination. The system degrades gracefully: remove {{ entity(name="toadstool") }} and compute is lost but everything else works; remove {{ entity(name="nestgate") }} and storage is lost but crypto and networking continue. Nothing is tightly coupled.

```
NUCLEUS Complete
  ├── Tower Atomic (always present)
  │     ├── BearDog (cryptography)
  │     └── Songbird (networking)
  ├── Node Atomic (compute)
  │     └── ToadStool (CPU/GPU/NPU/WASM)
  ├── Nest Atomic (storage)
  │     └── NestGate (content-addressed)
  └── AI Layer
        └── Squirrel (model coordination)
```

### 3.5 Why Atomics, Not Monoliths

The atomic model exists because the alternative - monolithic services that internalize all capabilities - produces systems that cannot evolve, debug, or deploy incrementally.

With atomics:
- Deploy only what you need: Tower alone for a relay, full {{ entity(name="nucleus") }} for a workstation
- Each primal evolves independently: {{ entity(name="beardog") }} upgrades cipher suites without touching {{ entity(name="songbird") }}
- Failures are isolated: {{ entity(name="nestgate") }} crash does not affect networking
- New capabilities are added without modifying existing primals

The physical reality makes this concrete. On the deployed gate mesh:
- **Westgate** (i7-4771, 76TB ZFS) is optimized for cold storage → heavy {{ entity(name="nestatomic") }}, lightweight Node
- **Northgate** (i9-14900K, RTX 5090, 192GB) is optimized for AI compute → heavy {{ entity(name="nodeatomic") }}
- **Strandgate** (Dual EPYC 7452, 256GB ECC) is optimized for parallel bioinformatics → CPU-bound Node
- **Southgate** (5800X3D, RTX 3090, 128GB) is a balanced general-purpose node

Each gate runs the atomics that match its hardware. {{ entity(name="biomeos") }} coordinates the mesh so workloads land on the right gate.

---

## 4. The Bonding Model

The bonding model describes how {{ entity(name="nucleus") }} deployments interact with **each other** - across physical machines, networks, and trust boundaries. Each physical computer ("gate") runs its own {{ entity(name="nucleus") }}. The bonding type determines trust level, capability sharing, and verification requirements at the boundary.

This is a chemistry metaphor applied to distributed systems. Just as molecular bonding determines how atoms share electrons and form structures, {{ entity(name="nucleus") }} bonding determines how gates share capabilities and form compute meshes.

### 4.1 Covalent Bonding - Shared Electrons, Family Trust

**Physical context**: The gates in a local {{ entity(name="nucleus") }} mesh — Northgate, Southgate, Strandgate, Westgate — each running their own {{ entity(name="nucleus") }}, sharing a common family seed. {{ entity(name="beardog") }} on each gate verifies genetic lineage. {{ entity(name="songbird") }} discovers peers via BirdSong encrypted multicast on the local network.

**Behavior**: When a workload arrives at Northgate that exceeds its capacity, {{ entity(name="biomeos") }} distributes it to Southgate or Strandgate without contract negotiation. Trust is genetic - shared family seed means automatic capability sharing. A compute job can be split across GPUs on three gates as naturally as threads split across cores on one machine.

**Access level**: Full genetic trust. Workload permission granted by the family seed holder propagates to all covalently bonded gates.

### 4.2 Ionic Bonding - Contract-Based, Metered

**Physical context**: The HPC mesh connects to a cloud VM for burst compute. An external researcher rents GPU time on Northgate. {{ entity(name="squirrel") }} routes a task to a cloud-hosted large model.

**Behavior**: Ionic interaction is a trade. The cloud VM gets access to specific capabilities (e.g., `compute.execute` on designated workloads) but not to the family's genetic lineage, storage, or discovery infrastructure. Usage is metered. Access is scoped to the contract. The underlying covalent mesh is invisible to the ionic partner.

**Access level**: Contract-scoped. Metered. Auditable.

### 4.3 Metallic Bonding - Electron Sea, Sub-Specialization

**Physical context**: A rack of similar machines in a facility, or a fleet of cloud VMs. Rather than each gate running a full {{ entity(name="nucleus") }}, gates sub-specialize: some run only Node Atomics (pure compute), some only Nest Atomics (pure storage), some only Tower Atomics (relay/discovery). Capabilities are delocalized.

**Behavior**: If three compute-specialized gates exist and one fails, the remaining two absorb the load because compute was never localized to a single gate.

**Access level**: High internal trust, optimized for throughput and specialization.

### 4.4 Weak Forces - Minimal Interaction, Pre-Trust

**Physical context**: {{ entity(name="squirrel") }} calling the OpenAI API. A primal querying a public REST endpoint. A {{ entity(name="darkforest") }} beacon from an unknown source. The Pixel 8a appearing on the network before lineage re-verification.

**Behavior**: Read-only, stateless, no trust, no capability sharing. This is also the **default starting state** for all interactions before trust is established. {{ entity(name="darkforest") }} beacons begin as weak forces - the encrypted beacon is indistinguishable from noise until the receiver proves it shares the beacon seed.

**Access level**: None.

### 4.5 Mixed Bonding - The Real System

A running deployment exhibits multiple bonding types simultaneously:

```
Local gate mesh (Covalent)
  ├── Northgate ←→ Southgate ←→ Strandgate ←→ Westgate
  │   (genetic trust, free workload distribution)
  │
  ├──[ionic]── Cloud VM (contract-based burst compute)
  ├──[ionic]── External researcher (rented GPU time)
  ├──[weak]─── OpenAI API (Squirrel, no trust)
  └──[weak→covalent]── Pixel 8a (enters weak, escalates after verification)
```

The **Pixel 8a pattern** demonstrates dynamic bonding: a mobile device carrying new data and hardware authentication (SoloKey) enters the network at weak forces. BirdSong discovery detects it. {{ entity(name="darkforest") }} verification escalates through beacon decryption, lineage challenge, and identity confirmation. Once verified, it transitions to covalent bonding. Data is incorporated after trust clears. When it leaves, the bond suspends until return and re-verification.

---

## 5. Neural API - Semantic Orchestration

The {{ entity(name="neuralapi") }} is {{ entity(name="biomeos") }}'s orchestration layer. It operates in three tiers:

### Layer 1: Primals (Capabilities)

Each primal advertises what it can do. {{ entity(name="beardog") }} advertises `crypto.*`. {{ entity(name="songbird") }} advertises `tls.*`, `discovery.*`. {{ entity(name="nestgate") }} advertises `storage.*`. These are raw capabilities - the primitives.

### Layer 2: biomeOS (Orchestration)

{{ entity(name="biomeos") }} maintains a capability registry populated by runtime discovery. When a request arrives, the {{ entity(name="neuralapi") }}:

1. Resolves the semantic name to a primal-specific method
2. Routes the request to the appropriate primal
3. Returns the result to the caller
4. Learns from the interaction (pathway optimization)

| Semantic Request | Translated To | Routed To |
|-----------------|---------------|-----------|
| `crypto.sign` | `ed25519_sign` | {{ entity(name="beardog") }} |
| `http.request` | `secure_http_request` | {{ entity(name="songbird") }} (via Tower) |
| `storage.put` | `storage.put` | {{ entity(name="nestgate") }} |
| `compute.execute` | `workload.submit` | {{ entity(name="toadstool") }} |

### Layer 3: Niche APIs (Domain Patterns)

{{ entity(name="niche") }} APIs are coordination patterns that emerge from primal composition. {{ entity(name="rootpulse") }} (distributed version control) is a niche API: {{ entity(name="biomeos") }} coordinates {{ entity(name="rhizocrypt") }} (ephemeral DAG workspace), {{ entity(name="loamspine") }} (permanent ledger), {{ entity(name="nestgate") }} (blob storage), {{ entity(name="beardog") }} (signing), {{ entity(name="sweetgrass") }} (attribution), and {{ entity(name="songbird") }} (discovery/federation) into temporal coordination patterns. No primal knows about "version control" - {{ entity(name="biomeos") }} composes their primitives and version control emerges.

The {{ entity(name="neuralapi") }} is the **TRUE PRIMAL** pattern: capability-based routing where the caller requests a capability without knowing which primal provides it. This enables:
- Hot-swapping primal implementations without changing callers
- Graceful degradation when primals are unavailable
- Multi-provider resolution (multiple primals can provide the same capability)
- Pathway learning ({{ entity(name="biomeos") }} optimizes routing based on observed performance)

---

## 6. Dark Forest Protocol - Zero Metadata Security

The {{ entity(name="darkforest") }} protocol provides zero-metadata-leakage security for primal discovery and federation. The name comes from the Three-Body Problem: in a dark forest full of hunters, the safest strategy is to reveal nothing about your existence.

### 6.1 The Two-Seed Genetic Model

Every {{ entity(name="nucleus") }} deployment holds two cryptographic seeds, analogous to mitochondrial and nuclear DNA:

**Beacon Seed (Mitochondrial DNA)**: Shared across all devices in a lineage. Used to encrypt/decrypt BirdSong discovery beacons. Enables "can I hear this?" - the first test of family membership.

**Lineage Seed (Nuclear DNA)**: Unique per device. Used for identity derivation, challenge-response authentication, and fine-grained permissions. Enables "who exactly is this?" - the full identity verification.

### 6.2 Protocol Layers

1. **Encrypted Beacons**: {{ entity(name="songbird") }} broadcasts UDP packets encrypted with ChaCha20-Poly1305 using a key derived from the beacon seed. To outsiders, these are indistinguishable from random noise. Only receivers who share the beacon seed can decrypt them.

2. **Challenge-Before-Reveal**: After beacon decryption succeeds (proving shared beacon seed), a challenge-response protocol using the lineage seed proves specific identity. No identity information is revealed until the challenge succeeds.

3. **Lineage Relay**: Trusted peers relay discovery information to family members across network boundaries (different subnets, NATs, geographic regions) without exposing the relayed information to intermediaries.

4. **Physical Anchor**: Hardware-backed authentication (SoloKey FIDO2) provides a physical root of trust that cannot be extracted by software.

5. **Every Server Is a Relay**: Any {{ entity(name="nucleus") }} can act as a relay for lineage members, creating a sovereign mesh that does not depend on centralized infrastructure.

### 6.3 Security Properties

- **Passive observers see nothing**: Beacons are indistinguishable from random data
- **Active probes get nothing**: Challenge-response reveals no information to non-family
- **No metadata leakage**: The protocol does not expose source, destination, payload size, or timing patterns that could be used for traffic analysis
- **Genetic access control**: Only family members can participate. Membership is cryptographic, not credential-based.

---

## 7. Sovereign NAT Traversal

P2P connectivity across NAT boundaries uses a multi-tier strategy that prioritizes family-owned infrastructure:

| Tier | Method | Trust Level | NAT Types |
|------|--------|-------------|-----------|
| 1 | Direct UDP hole punch | Full (direct connection) | Full cone, restricted cone |
| 2 | Family STUN server | Full (family infrastructure) | Full cone, restricted cone, port-restricted |
| 3 | Family relay (lineage-gated) | Full (family relay) | All types including symmetric NAT |
| 4 | Public STUN fallback | Minimal (public infrastructure) | Full cone, restricted cone |

**Symmetric NAT** - the hardest NAT type to traverse - requires relay infrastructure. {{ entity(name="toweratomic") }} provides this: {{ entity(name="songbird") }} runs a relay server, {{ entity(name="beardog") }} gates access via lineage verification (only family members can use the relay), and relay traffic is encrypted end-to-end.

The key design principle: never depend on corporate infrastructure (Google STUN, AWS TURN) for basic connectivity. Family-owned STUN and relay servers handle all NAT types. Public STUN is a last-resort fallback.

---

## 8. Composed Systems

### 8.1 RootPulse - Distributed Version Control

{{ entity(name="rootpulse") }} is not a primal. It is a coordination pattern that emerges when {{ entity(name="biomeos") }} orchestrates multiple primals:

| Primal | Role in {{ entity(name="rootpulse") }} |
|--------|-------------------|
| **{{ entity(name="rhizocrypt") }}** | Ephemeral DAG workspace - fast, lock-free, present/future |
| **{{ entity(name="loamspine") }}** | Immutable linear history - permanent, cryptographically provable, past |
| **{{ entity(name="nestgate") }}** | Content-addressed blob storage |
| **{{ entity(name="beardog") }}** | Cryptographic signing and verification |
| **{{ entity(name="sweetgrass") }}** | Semantic attribution tracking |
| **{{ entity(name="songbird") }}** | Discovery and federation |

"{{ entity(name="rootpulse") }} is what primals DO together, not what they ARE."

### 8.2 The Memory & Attribution Stack

{{ entity(name="rhizocrypt") }}, {{ entity(name="loamspine") }}, and {{ entity(name="sweetgrass") }} form a unified stack:

```
Application Layer (Gaming, Scientific, Collaboration)
        │
  sweetGrass (Attribution) — Who created what, when, how
        │
   LoamSpine (Permanence) — Selective immutable history
        │
  rhizoCrypt (Core DAG) — Content-addressed working memory
```

{{ entity(name="rhizocrypt") }} is the engine (ephemeral, fast, lock-free). {{ entity(name="loamspine") }} adds permanence semantics (append-only, provable). {{ entity(name="sweetgrass") }} adds attribution semantics (W3C PROV-O compliant provenance tracking). {{ entity(name="biomeos") }} coordinates them via the {{ entity(name="neuralapi") }}.

---

## 9. The Architecture as Evidence

This architecture was not designed on a whiteboard. It emerged from approximately 6-8 months of constrained evolution within Rust's type system, guided by the principles described in `CONSTRAINED_EVOLUTION_FORMAL.md`.

The {{ entity(name="toweratomic") }} pattern - the headline innovation - was not planned. The Pure Rust constraint made OpenSSL impossible, which eliminated the conventional approach to HTTPS, which forced exploration of the composition pattern, which proved that primal coordination over JSON-RPC could handle even the most complex protocol interaction (TLS 1.3). This is the citrate metabolism of the {{ entity(name="ecoprimals") }} project: an innovation that emerged from constraint, not from design.

The architecture ladder ({{ entity(name="unibin") }} → {{ entity(name="ecobin") }} → {{ entity(name="genomebin") }}) was not planned. Each stage emerged when the previous stage's limitations became apparent. {{ entity(name="unibin") }} standardized the binary interface. {{ entity(name="ecobin") }} eliminated C dependencies. {{ entity(name="genomebin") }} solved the deployment problem. Each was a response to environmental pressure.

The bonding model was not planned. It emerged when the HPC grew from one machine to several and the question of inter-machine trust became concrete. Covalent bonding for family trust, ionic bonding for external contracts, weak forces for pre-trust interactions - these categories emerged from observing what the system actually needed, not from top-down taxonomy.

If the constrained evolution methodology works as described in the companion paper, then the architecture's coherence is not surprising. It is what Lenski's experiment predicts: populations under consistent constraint specialize toward fitness, and the resulting structure reflects the constraint environment. The {{ entity(name="ecoprimals") }} architecture reflects Rust's type system, the Pure Rust directive, and the capability-based coordination requirement - because those constraints shaped every evolutionary step.

---

## References

See `CONSTRAINED_EVOLUTION_FORMAL.md` for the biological and methodological foundations.  
See `P_NP_ENZYME_THESIS.md` for the theoretical extension to complexity theory.  
See `PRIMAL_CATALOG.md` for the concrete implementations and their current status.

Full technical specifications are in `whitePaper/technical/`:
- `UNIBIN_TECHNICAL_SPECIFICATION.md`
- `ECOBIN_TECHNICAL_SPECIFICATION.md`
- `GENOMEBIN_TECHNICAL_SPECIFICATION.md`
- `NUCLEUS_ARCHITECTURE.md`
- `TOWER_ATOMIC_PURE_RUST_HTTPS.md`
- `NEURAL_API_ARCHITECTURE.md`
- `SOVEREIGN_NAT_TRAVERSAL.md`
- `DARK_FOREST_PROTOCOL.md`

---

**Note**: This paper describes an architecture that exists and runs on physical hardware. Every pattern described here is implemented in Rust, tested, and deployed. The gap between paper and practice is zero - the architecture IS the implementation, and the implementation IS the evidence for the constrained evolution methodology that produced it.
