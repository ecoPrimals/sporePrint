+++
title = "ecoPrimals Primal Catalog: Status, Capabilities, and Achievements"
description = "All 17 primals and tooling — capabilities, test counts, production status, repository visibility"
date = 2026-03-31

[taxonomies]
primals = ["beardog", "barracuda", "biomeos", "bingocube", "coralreef", "loamspine", "nestgate", "petaltongue", "rhizocrypt", "skunkbat", "songbird", "sourdough", "squirrel", "sweetgrass", "toadstool"]
springs = ["healthspring", "hotspring", "neuralspring", "wetspring"]
+++

**Status**: Working paper  
**Lineage**: Implementation companion to `ECOSYSTEM_ARCHITECTURE.md`  
**Last Updated**: March 31, 2026

---

## Abstract

This document catalogs every primal in the {{ entity(name="ecoprimals") }} ecosystem. It records what was built, how far it has evolved, and what it can demonstrate. The ecosystem was constructed by ecoPrimal (human + synthetic intelligence) over approximately 6-8 months, using the constrained evolution methodology described in `CONSTRAINED_EVOLUTION_FORMAL.md`. The results documented here are the empirical evidence for that methodology.

The primals are organized into three tiers:

- **Foundation Primals** (§1): The bedrock of the ecosystem. Eight primals — {{ entity(name="beardog") }}, {{ entity(name="songbird") }}, {{ entity(name="nestgate") }}, {{ entity(name="toadstool") }}, {{ entity(name="squirrel") }}, {{ entity(name="biomeos") }}, {{ entity(name="coralreef") }}, and {{ entity(name="barracuda") }} — are production-ready, extensively tested, and form the {{ entity(name="nucleus") }} deployment architecture. {{ entity(name="coralreef") }} and {{ entity(name="barracuda") }} were promoted from {{ entity(name="toadstool") }} sub-crates to independent primals (#13, #14) as the Sovereign Compute Pipeline matured.

- **Post-{{ entity(name="nucleus") }} Primals** (§2): Primals designed for capabilities that emerge after {{ entity(name="nucleus") }} is deployed. These primals ({{ entity(name="petaltongue") }}, {{ entity(name="rhizocrypt") }}, {{ entity(name="sweetgrass") }}, {{ entity(name="loamspine") }}, {{ entity(name="skunkbat") }}) compose into higher-order patterns like {{ entity(name="rootpulse") }} and the Memory & Attribution Stack. Each has been started and has functional code and tests, but they receive less focus until {{ entity(name="nucleus") }} is stable. They represent the next evolutionary phase.

- **Meta-Primals & Tooling** (§3): {{ entity(name="sourdough") }} is scaffolding and packaging tooling — it generates new primals and produces {{ entity(name="genomebin") }} artifacts, but does not run as a {{ entity(name="nucleus") }} service at runtime. {{ entity(name="wateringhole") }}, {{ entity(name="whitepaper") }}, and {{ entity(name="sporeprint") }} are documentation/standards infrastructure.

**Total**: 17 primals and tooling (8 foundation + 5 post-{{ entity(name="nucleus") }} + 1 meta/tooling + 3 publishing soon) across three tiers.

### Repository Visibility

All primals are **{{ entity(name="scyborg") }}-licensed** (AGPL-3.0-or-later for code, ORC for game mechanics, CC-BY-SA 4.0 for creative/docs) and intended to be fully public. Some are already on GitHub; the rest have source publishing in progress. Binaries for all primals are available through [plasmidBin](@/architecture/DEPLOYMENT_MODEL.md). Per AGPL-3.0, source for any distributed binary is available on request.

| Primal | Repo | Visibility |
|--------|------|------------|
| {{ entity(name="beardog") }} | ecoPrimals/bearDog | Source publishing in progress (binary via [plasmidBin](https://github.com/ecoPrimals/plasmidBin)) |
| {{ entity(name="songbird") }} | [ecoPrimals/songBird](https://github.com/ecoPrimals/songBird) | **Public** |
| {{ entity(name="nestgate") }} | [ecoPrimals/nestGate](https://github.com/ecoPrimals/nestGate) | **Public** |
| {{ entity(name="toadstool") }} | [ecoPrimals/toadStool](https://github.com/ecoPrimals/toadStool) | **Public** |
| {{ entity(name="squirrel") }} | [ecoPrimals/squirrel](https://github.com/ecoPrimals/squirrel) | **Public** |
| {{ entity(name="biomeos") }} | [ecoPrimals/biomeOS](https://github.com/ecoPrimals/biomeOS) | **Public** |
| {{ entity(name="coralreef") }} | [ecoPrimals/coralReef](https://github.com/ecoPrimals/coralReef) | **Public** |
| {{ entity(name="barracuda") }} | [ecoPrimals/barraCuda](https://github.com/ecoPrimals/barraCuda) | **Public** |
| {{ entity(name="petaltongue") }} | [ecoPrimals/petalTongue](https://github.com/ecoPrimals/petalTongue) | **Public** |
| {{ entity(name="rhizocrypt") }} | ecoPrimals/rhizoCrypt | Source publishing in progress (binary via [plasmidBin](https://github.com/ecoPrimals/plasmidBin)) |
| {{ entity(name="sweetgrass") }} | ecoPrimals/sweetGrass | Source publishing in progress (binary via [plasmidBin](https://github.com/ecoPrimals/plasmidBin)) |
| {{ entity(name="loamspine") }} | ecoPrimals/loamSpine | Source publishing in progress (binary via [plasmidBin](https://github.com/ecoPrimals/plasmidBin)) |
| {{ entity(name="skunkbat") }} | ecoPrimals/skunkBat | Source publishing in progress (binary via [plasmidBin](https://github.com/ecoPrimals/plasmidBin)) |
| {{ entity(name="sourdough") }} | [ecoPrimals/sourDough](https://github.com/ecoPrimals/sourDough) | **Public** |

Ten primals are fully public on GitHub: **songBird**, **nestGate**, **toadStool**, **squirrel**, **biomeOS**, **coralReef**, **barraCuda**, **petalTongue**, **sourDough**, and **bingoCube**. All springs ([syntheticChemistry](https://github.com/syntheticChemistry) org) are public. Pre-built binaries for all primals are distributed via [plasmidBin](https://github.com/ecoPrimals/plasmidBin).

---

## 1. Foundation Primals

These primals form the {{ entity(name="nucleus") }} deployment architecture. Each is production-ready, independently deployable, and has demonstrated its capabilities through showcase demonstrations and test suites.

---

### 1.1 BearDog - Cryptography Primal

**Domain**: All cryptographic operations and genetic lineage  
{{ entity_metrics(name="beardog") }}  
**Tests**: 5,041 passing (100%)  
**Coverage**: 70.96%  
**Safety**: Zero unsafe blocks, zero warnings

{{ entity(name="beardog") }} is the cryptographic spine of {{ entity(name="ecoprimals") }}. Every operation that requires signing, encrypting, hashing, key derivation, or identity verification is delegated to {{ entity(name="beardog") }} via JSON-RPC. No other primal implements its own crypto. This is the **{{ entity(name="toweratomic") }} pattern**: a single, auditable cryptographic surface for the entire ecosystem.

**Why it exists**: In gen1, {{ entity(name="nestgate") }} and {{ entity(name="squirrel") }} each embedded their own crypto logic. When {{ entity(name="songbird") }} was added for networking, it needed TLS — a third crypto implementation. Three codebases to audit, three trust surfaces, three places for key management bugs. {{ entity(name="beardog") }} consolidated all crypto into one primal. The others delegate to it. Every primal gets crypto from the same source, the same key store, the same audit trail.

**What Pure Rust means here**: RustCrypto libraries. Zero OpenSSL. Zero C dependencies. Zero unsafe in production paths. The Pure Rust constraint forced the Tor v3 implementation (3,345 lines of protocol logic, not a wrapper around the C Tor daemon) and eliminated every C crypto library from the trust surface. The entire cryptographic stack is covered by Rust's memory safety — no buffer overflows, no use-after-free, no C code to audit separately.

**Primitive catalog** (91 methods, 72 JSON-RPC endpoints):

| Category | Primitives |
|----------|-----------|
| Signatures | Ed25519, ECDSA (P-256, P-384), RSA (PKCS#1 v1.5, PSS) |
| Encryption | ChaCha20-Poly1305, AES-128-GCM, AES-256-GCM |
| Key Exchange | X25519, ECDHE (P-256, P-384) |
| Hashing | BLAKE3, SHA-256, SHA-384, SHA-512, HMAC |
| Identity | Genetic lineage (family seeds, beacon seeds), {{ entity(name="darkforest") }} beacons |
| Onion Routing | Pure Rust Tor v3 (directory, circuit, stream, onion service) |
| Post-Quantum | ML-KEM key encapsulation |

{{ entity(name="beardog") }} enforces a strict entropy hierarchy — hardware RNG (SoloKey FIDO2) → OS entropy → CSPRNG (ChaCha20) — structurally, not by configuration. Multi-family key stores allow a single machine to host independent trust domains without key leakage.

**Participates in**: {{ entity(name="toweratomic") }} (with {{ entity(name="songbird") }}), {{ entity(name="nucleus") }} (all configurations), {{ entity(name="rootpulse") }}, {{ entity(name="darkforest") }} Federation, every primal that needs cryptographic operations.

---

### 1.2 Songbird - Network Primal

**Domain**: Network orchestration, discovery, and federation  
{{ entity_metrics(name="songbird") }}  
**Version**: v3.35.0  
**Tests**: 1,763 passing (100%)  
**Safety**: Zero unsafe blocks in production, clean build

{{ entity(name="songbird") }} is the nervous system. If data needs to leave the machine, it goes through {{ entity(name="songbird") }}. TLS 1.3 (Pure Rust, no OpenSSL), service discovery (BirdSong protocol), NAT traversal (4-tier), and the networking half of {{ entity(name="toweratomic") }}. {{ entity(name="songbird") }} also serves as the **universal adapter** for discovery: instead of every primal discovering every other primal (O(n²) connections), each primal registers with {{ entity(name="songbird") }} (O(n) connections), and {{ entity(name="songbird") }} handles routing.

**Why it exists**: In gen1, {{ entity(name="nestgate") }} had NFS/SMB networking and {{ entity(name="squirrel") }} had MCP networking — two ad-hoc networking stacks in two primals. When {{ entity(name="beardog") }} was split out for crypto, the question became: who does TLS? Networking — TLS handshakes, NAT hole-punching, peer discovery, relay routing — is complex enough to warrant its own primal, and security-critical enough that it should be tightly integrated with {{ entity(name="beardog") }}'s crypto.

**What Pure Rust means here**: {{ entity(name="songbird") }} delegates 100% of its crypto to {{ entity(name="beardog") }} — zero direct cryptographic code in {{ entity(name="songbird") }}. The Pure Rust constraint eliminated coturn (C-based STUN/TURN server), which forced a custom Pure Rust STUN implementation. The 4-tier NAT traversal (direct → IGD/UPnP → STUN → relay) means {{ entity(name="ecoprimals") }} can operate from a basement behind a consumer router without requiring port forwarding. The system discovers its own network topology and adapts.

**{{ entity(name="toweratomic") }} — the headline composition**: {{ entity(name="beardog") }} provides the cryptographic operations. {{ entity(name="songbird") }} provides the TLS 1.3 state machine and HTTP server. Neither can do HTTPS alone. Together, via JSON-RPC over Unix sockets, they produce Pure Rust HTTPS with zero C dependencies — 93% TLS validation across 87 production sites. {{ entity(name="toweratomic") }} was not designed. The Pure Rust constraint eliminated OpenSSL. The primal isolation constraint prevented {{ entity(name="songbird") }} from embedding {{ entity(name="beardog") }}'s crypto. The only remaining option was composition via IPC — and it worked.

| Category | Primitives |
|----------|-----------|
| TLS | TLS 1.3 (RFC 8446), TLS 1.2 fallback, protocol detection |
| Discovery | BirdSong encrypted UDP multicast, mDNS/DNS-SD, 6-layer capability-based strategy |
| NAT Traversal | Pure Rust STUN (RFC 5389), 4-tier: direct → IGD → STUN → relay |
| {{ entity(name="darkforest") }} | Zero metadata leakage discovery, encrypted beacons |
| P2P | Sovereign onion service, circuit building, directory authority |

**Participates in**: {{ entity(name="toweratomic") }} (with {{ entity(name="beardog") }}), {{ entity(name="nucleus") }} (all configurations), {{ entity(name="rootpulse") }} (discovery/federation), BirdSong protocol, {{ entity(name="darkforest") }} discovery.

---

### 1.3 NestGate - Data Primal

**Domain**: Storage and content-addressed data management  
{{ entity_metrics(name="nestgate") }}  
**Version**: 4.0.0 ({{ entity(name="genomebin") }})  
**Tests**: 1,474/1,475 passing (99.93%)  
**Build**: 100% (13/13 crates)

{{ entity(name="nestgate") }} is the data layer. Content-addressed storage (BLAKE3), ZFS integration, model caching, tiered storage. If data needs to persist across sessions, it goes through {{ entity(name="nestgate") }}. {{ entity(name="nestgate") }} is one of the two **original primals** — it existed before the word "primal" did. In gen1, it was the Rust-based ZFS storage manager for the HPC cluster. In gen2, it became a sovereign storage primitive. In gen3, it is the storage component of the {{ entity(name="nestatomic") }}.

**Why it exists**: The gen1 HPC cluster needed a storage layer that understood ZFS, handled tiered storage (cold archive on HDDs, hot cache on NVMe), and served compute nodes. As the ecosystem grew, {{ entity(name="nestgate") }} absorbed model caching for {{ entity(name="squirrel") }}'s AI inference, content-addressed blob storage for {{ entity(name="rhizocrypt") }}, and discovery services for the ecosystem.

**What Pure Rust means here**: {{ entity(name="nestgate") }} pioneered the **isomorphic IPC** pattern — the same connection logic works over Unix sockets, TCP, and abstract sockets, auto-detecting the best transport via **Try→Detect→Adapt→Succeed**. This pattern was independently adopted by other primals after {{ entity(name="nestgate") }} proved it. The gen1 archaeological layers are visible in the codebase: ZFS management code predates the primal architecture, sitting alongside content-addressed blob storage added in gen2.

| Category | Primitives |
|----------|-----------|
| Storage | Content-addressed blobs (BLAKE3), deduplication, tiered storage (HDD → SSD → NVMe → RAM) |
| ZFS | Snapshots (100 in 0.17s), compression, quota, pool management |
| Discovery | Isomorphic IPC, MCP provider, multi-family sockets |
| Caching | AI model cache for {{ entity(name="squirrel") }} (download, store, retrieve via JSON-RPC) |

**Participates in**: {{ entity(name="nestatomic") }} (with Tower), {{ entity(name="nucleus") }}, {{ entity(name="rootpulse") }} (content storage), federation.

---

### 1.4 ToadStool - Compute Primal

**Domain**: Universal compute orchestration  
{{ entity_metrics(name="toadstool") }}  
**Tests**: 1,000+ passing  
**Repository**: [github.com/ecoPrimals/toadStool](https://github.com/ecoPrimals/toadStool) — **Public**

{{ entity(name="toadstool") }} is the compute layer. Hardware discovery (GPU, NPU, CPU via sysfs/PCIe), workload dispatch, and the orchestration surface for the **Sovereign Compute Pipeline**. {{ entity(name="toadstool") }} owns the hardware. {{ entity(name="barracuda") }} (§1.8) owns the math. {{ entity(name="coralreef") }} (§1.7) owns the compiler. Together they provide scientific computing on any GPU — NVIDIA, AMD, Intel — without CUDA dependency.

**Why it exists**: In gen1, {{ entity(name="squirrel") }} handled both job scheduling and compute execution. As GPU workloads grew complex (molecular dynamics, lattice QCD, neural network inference), the compute layer needed its own primal. {{ entity(name="toadstool") }} was split from {{ entity(name="squirrel") }} to own the hardware-specific concerns: GPU detection, driver compatibility, memory management, workload queuing. {{ entity(name="squirrel") }} kept AI-specific logic; {{ entity(name="toadstool") }} took "run this computation on that hardware."

**What Pure Rust means here**: The Pure Rust constraint eliminated CUDA. That pushed exploration of Vulkan, which revealed a capability the conventional approach actively hides: **Vulkan's `SHADER_F64` extension exposes native f64 on consumer GPUs at 1:2 throughput**. NVIDIA's CUDA throttles consumer f64 to 1:64 to protect the compute-class product line. Vulkan doesn't. The $600 RTX 4070 does real science — Yukawa MD with 0.000% energy drift, nuclear EOS with χ²/datum = 2.27, lattice QCD plaquettes — all at f64 precision. The constraint forced the discovery.

{{ entity(name="toadstool") }} also hosts a **Pure Rust Akida driver** for BrainChip's neuromorphic hardware (160 NPUs detected, 48-202x faster than CPU for specific workloads, 100x power efficiency vs GPU for LLM intent classification). No other Rust project has a production neuromorphic driver.

The springs are {{ entity(name="toadstool") }}'s acceptance tests. {{ entity(name="hotspring") }} proves the physics kernels work. {{ entity(name="wetspring") }} proves the biology kernels work. {{ entity(name="neuralspring") }} proves the ML kernels work. Every validated check is evidence that constrained evolution produced correct scientific computing.

**Participates in**: {{ entity(name="nodeatomic") }} (with Tower), {{ entity(name="nucleus") }}, Sovereign Compute Pipeline (with {{ entity(name="barracuda") }} and {{ entity(name="coralreef") }}).

---

### 1.5 Squirrel - AI Primal

**Domain**: AI model coordination and sovereign inference  
{{ entity_metrics(name="squirrel") }}  
**Version**: 0.1.0-alpha.33  
**Tests**: 7,165 passing / 0 failed / 110 ignored  
**Coverage**: ~85.3% line coverage (cargo-llvm-cov)  
**Safety**: `#![forbid(unsafe_code)]` workspace-wide  
**Repository**: [github.com/ecoPrimals/squirrel](https://github.com/ecoPrimals/squirrel) — **Public**

{{ entity(name="squirrel") }} is the AI brain. Vendor-agnostic model routing across OpenAI, Anthropic, Ollama, and local models. Multi-MCP coordination. Context management. Cost/quality/latency routing. If anything in the ecosystem needs AI inference, it asks {{ entity(name="squirrel") }}. {{ entity(name="squirrel") }} is one of the two **original primals** — in gen1, it was the fault-tolerant compute orchestration platform for the HPC cluster.

**Why it exists**: In gen1, {{ entity(name="squirrel") }} was the HPC job scheduler: checkpoint/restart, circuit breakers, "ant-model" orchestration across compute nodes. As AI models became the primary workload, {{ entity(name="squirrel") }} evolved from general job scheduler into a dedicated AI coordination layer. The Model Context Protocol (MCP) gave it a standard way to discover and invoke AI capabilities across providers. The name stuck: squirrels cache things (model weights), they're fast (low-latency routing), and they coordinate complex foraging patterns across large areas (multi-provider inference).

**What Pure Rust means here**: {{ entity(name="squirrel") }} follows the **TRUE PRIMAL** pattern — no compile-time coupling to any external service, capability-based provider discovery, isomorphic IPC. Adding a new AI provider requires a plugin crate, not changes to core logic. The checkpoint/restart code from gen1 evolved into context management and conversation state persistence. The circuit breakers evolved into provider fallback chains — {{ entity(name="squirrel") }}'s evolution from HPC job scheduler to sovereign AI coordinator is the clearest example of constrained evolution in the ecosystem.

| Category | Primitives |
|----------|-----------|
| Inference | `ai.query`, `ai.complete`, `ai.chat` — multi-provider routing |
| MCP | Multi-server coordination, tool discovery, resource management |
| Context | Session management, token counting, context windowing, automatic fallback |
| Sovereign | Local inference via Ollama, zero telemetry by default, DignityGuard ethics checks |

**Participates in**: Full {{ entity(name="nucleus") }} (all atomics + AI), {{ entity(name="rootpulse") }} (intelligent merge resolution), {{ entity(name="biomeos") }} {{ entity(name="neuralapi") }} (`ai` domain).

---

### 1.6 biomeOS - Ecosystem Orchestrator

**Domain**: Primal orchestration and ecosystem coordination  
{{ entity_metrics(name="biomeos") }}  
**Security**: A++ LEGENDARY ({{ entity(name="darkforest") }})  
**Tests**: 661+ passing  
**Coverage**: ~48%

{{ entity(name="biomeos") }} is the conductor. If {{ entity(name="beardog") }} is the immune system and {{ entity(name="songbird") }} is the nervous system, {{ entity(name="biomeos") }} is the **endocrine system**: it coordinates all the organs without micromanaging any of them. It starts primals in the correct order, maintains a capability registry, routes requests semantically, composes primals into atomics (Tower, Node, Nest, {{ entity(name="nucleus") }}), and manages the lifecycle of the entire ecosystem. Without {{ entity(name="biomeos") }}, primals are isolated services. With {{ entity(name="biomeos") }}, they are an ecosystem.

**Why it exists**: In gen2, the whitepaper described "composable primitives" but left coordination implicit. As the primal count grew from 2 (gen1) to 8 (gen2) to 17 (gen3), explicit orchestration became necessary. Who starts first? How does a new primal discover existing ones? What happens when a primal crashes? {{ entity(name="biomeos") }} answers these questions. The name comes from the "biome" concept: a packaged ecosystem defined by a manifest, analogous to a biological biome where organisms interact through defined ecological relationships.

**What Pure Rust means here**: {{ entity(name="biomeos") }} implements the **{{ entity(name="neuralapi") }}** — 124 semantic capability translations. Callers don't address primals by name; they request capabilities: `capability.call("crypto.sign", ...)` routes to {{ entity(name="beardog") }}, `capability.call("ai.chat", ...)` routes to {{ entity(name="squirrel") }}. The caller never knows which primal handled it. This decoupling is what makes hot-swapping primals possible. Deploy graphs are TOML manifests referencing primals by capability, not name — graph-based deployment, not imperative scripting.

The **{{ entity(name="darkforest") }}** protocol (A++ LEGENDARY) provides zero-metadata-leakage discovery: beacons are indistinguishable from random noise to anyone without the family key. The **{{ entity(name="plasmodium") }}** collective enables multi-machine {{ entity(name="nucleus") }}: {{ entity(name="biomeos") }} instances meld, split, and mix across machines, scaling from one basement server to a distributed mesh.

| Category | Primitives |
|----------|-----------|
| {{ entity(name="neuralapi") }} | 124 semantic capability translations, pathway learning, bidirectional feedback |
| Atomics | Tower, Node, Nest, Full {{ entity(name="nucleus") }} composition and health validation |
| Lifecycle | Startup ordering, auto-resurrection, post-{{ entity(name="nucleus") }} primal management |
| {{ entity(name="darkforest") }} | Zero metadata leakage, encrypted beacons, genetic model coordination |
| {{ entity(name="plasmodium") }} | Multi-machine meld/split/mix, cross-device federation |

**Participates in**: Coordinates all composed systems ({{ entity(name="rootpulse") }}, {{ entity(name="toweratomic") }}, {{ entity(name="nucleus") }}, federation, bonding model).

---

### 1.7 coralReef - Shader Compiler Primal

**Domain**: GPU shader compilation — WGSL/SPIR-V/GLSL to native GPU binaries  
{{ entity_metrics(name="coralreef") }}  
**Tests**: 3,038 passing (0 failed)  
**Coverage**: 65.8% line (79.6% non-hardware), 72.9% function  
**Safety**: `#![forbid(unsafe_code)]` on 8/9 crates, zero clippy warnings (pedantic+nursery)  
**Repository**: [github.com/ecoPrimals/coralReef](https://github.com/ecoPrimals/coralReef) — **Public**

{{ entity(name="coralreef") }} is a sovereign GPU compiler. No LLVM. No Mesa. No vendor SDK. The entire pipeline from shader IR to native machine code is Pure Rust. It takes WGSL, SPIR-V, or GLSL source and produces native GPU binaries for NVIDIA SM70–SM89 and AMD RDNA2 (GFX1030), with full f64 transcendental support. coralDriver provides userspace GPU dispatch via DRM ioctl — AMD amdgpu and NVIDIA nouveau/nvidia-drm — without linking any vendor libraries.

**Why it exists**: When {{ entity(name="barracuda") }} discovered that consumer GPUs expose native f64 via Vulkan's `SHADER_F64` extension, the ecosystem needed a way to compile f64 shaders to native GPU code without NVIDIA's CUDA/NVCC toolchain or AMD's ROCm/HIP stack. The Pure Rust constraint prohibited both. {{ entity(name="coralreef") }} was split from {{ entity(name="toadstool") }} to own shader compilation as a separate concern from compute dispatch. The boundary is clean: {{ entity(name="toadstool") }} never parses shaders; {{ entity(name="coralreef") }} never talks to hardware schedulers.

**What Pure Rust means here**: The f64 discovery forced the split. Compiling double-precision transcendentals (`exp2`, `log2`, `sin`, `cos`, `sqrt`, `rcp`, and their compositions) to native GPU instructions requires a real compiler backend — not a pass-through to `naga` + `wgpu`. {{ entity(name="coralreef") }}'s `lower_f64` pass decomposes f64 operations into instruction sequences the hardware can execute, with FMA policy control to match IEEE 754 rounding. These are not library calls — they are instruction sequences emitted directly into the native binary. The `coral-reef-stubs` crate provides Pure Rust replacements for CFG, BitSet, dataflow, SmallVec, and fxhash — zero external dependencies in the compiler core.

93/93 cross-spring WGSL shaders compile to SM70 SASS. AMD end-to-end verified: WGSL → compile → PM4 → GPU → readback on RX 6950 XT. Each compiled shader is evidence that sovereign compute (no vendor SDK, no C dependencies) can do real science.

**Participates in**: Sovereign Compute Pipeline ({{ entity(name="barracuda") }} → {{ entity(name="coralreef") }} → native binary → ToadStool/coralDriver → hardware), {{ entity(name="nodeatomic") }}, {{ entity(name="nucleus") }}.

---

### 1.8 barraCuda - Math Primal

**Domain**: Pure mathematics — WGSL f64 shaders, precision strategy, naga IR optimization  
{{ entity_metrics(name="barracuda") }} + {{ total_stat(stat="wgsl_files") }} WGSL shaders  
**Version**: v0.3.5  
**Tests**: 3,348+ passing  
**Safety**: Zero unsafe, zero clippy warnings  
**Repository**: [github.com/ecoPrimals/barraCuda](https://github.com/ecoPrimals/barraCuda) — **Public**

{{ entity(name="barracuda") }} is the math engine. Every GPU-accelerated computation in the ecosystem — linear algebra, FFT, molecular dynamics, spectral analysis, tensor operations, lattice QCD — is a WGSL shader pipeline managed by {{ entity(name="barracuda") }}. It writes the math; {{ entity(name="coralreef") }} compiles it; {{ entity(name="toadstool") }} dispatches it. 800+ production WGSL shaders across 10 scientific domains, all running on consumer GPUs via Vulkan — no CUDA, no ROCm.

**Why it exists**: {{ entity(name="barracuda") }} began as a crate inside {{ entity(name="toadstool") }}. As the springs matured, their compute demands grew specific: {{ entity(name="hotspring") }} needed Yukawa force kernels and lattice QCD; {{ entity(name="wetspring") }} needed biodiversity indices and ODE integrators; {{ entity(name="neuralspring") }} needed attention mechanisms and reservoir computing. The math was outgrowing {{ entity(name="toadstool") }}'s hardware-dispatch mission. {{ entity(name="barracuda") }} budded from {{ entity(name="toadstool") }} at S93 — the same pattern as every primal split: one responsibility consuming disproportionate surface area. {{ entity(name="toadstool") }}'s 50+ crates included {{ entity(name="barracuda") }}'s 628+ shaders, growing faster than the infrastructure code.

**What Pure Rust means here**: {{ entity(name="barracuda") }}'s NTT (Number Theoretic Transform) was built for FHE polynomial multiplication. The Cooley-Tukey butterfly structure — stage indexing, stride computation, block decomposition, twiddle lookup — is the same structure as FFT. When {{ entity(name="hotspring") }} needed FFT for PPPM electrostatics, the NTT kernel *was* the FFT kernel with complex twiddle factors instead of modular roots of unity. The main compute kernels (`fhe_ntt.wgsl` and `fft_1d.wgsl`) share the same computational skeleton. No one designed {{ entity(name="barracuda") }} for physics — the cryptographic constraint selected for a mathematical universal.

On hardware without native f64, {{ entity(name="barracuda") }} provides **DF64** — double-precision emulation from f32 pairs carrying ~48 bits of mantissa, delivering 9.9x native f64 throughput on FP32 cores. The `Fp64Strategy` (Native/Hybrid/Sovereign/Concurrent) routes precision transparently.

The **five-spring ingestion** pattern is unique: each spring validates {{ entity(name="barracuda") }}'s kernels against published scientific results (0.000% energy drift in Yukawa MD, χ²/datum = 2.27 for nuclear EOS, 926x GPU speedup for spectral cosine). The springs are not just consumers — they are acceptance tests.

| Domain | Operations | Spring Validation |
|--------|-----------|-------------------|
| Linear algebra | GEMM, eigensolvers, SVD, LU, QR, sparse CG | {{ entity(name="hotspring") }}, {{ entity(name="neuralspring") }} |
| Physics | Yukawa force, Velocity Verlet, PBC, PPPM, HFB nuclear, lattice QCD | {{ entity(name="hotspring") }} |
| ML | Attention (7 variants), losses, optimizers, ESN | {{ entity(name="neuralspring") }} |
| Bioinformatics | 31 GPU bio ops: kmer, UniFrac, HMM, phylogenetics | {{ entity(name="wetspring") }}, {{ entity(name="healthspring") }} |
| Special functions | Bessel, Laguerre, Hermite, erfc, Gamma, Hill kinetics | {{ entity(name="hotspring") }}, {{ entity(name="wetspring") }} |

**Participates in**: {{ entity(name="nodeatomic") }} (via {{ entity(name="toadstool") }}), {{ entity(name="nucleus") }} compute layer, Sovereign Compute Pipeline.

---

## 2. Post-NUCLEUS Primals

These primals represent capabilities that emerge after {{ entity(name="nucleus") }} is deployed. They compose into higher-order patterns ({{ entity(name="rootpulse") }}, Memory & Attribution Stack) coordinated by {{ entity(name="biomeos") }} via the {{ entity(name="neuralapi") }}. Each has been started — functional code, passing tests, showcase demonstrations — but they receive less focus until {{ entity(name="nucleus") }} is stable as a deployable composition. They are the next evolutionary phase: once the 8 foundation primals are solid, these 5 primals build emergent behaviors on top.

---

### 2.1 petalTongue - Representation Primal

**Domain**: Universal multi-modal user interface  
{{ entity_metrics(name="petaltongue") }}  
**Version**: 1.3.0

{{ entity(name="petaltongue") }} is the face. {{ entity(name="ecoprimals") }} was originally API-first, AI-mediated — "bring your own AI." But humans sometimes need to see things. {{ entity(name="petaltongue") }} provides visual, terminal, web, and headless interfaces to the ecosystem without coupling any specific UI framework to the primal architecture.

Five interface modes from a single binary ({{ entity(name="unibin") }}): `ui` (egui desktop), `tui` (ratatui terminal), `web` (Axum browser), `headless` (API-only), `status` (health output). The **{{ entity(name="unibin") }} pattern** emerged from the constraint that {{ entity(name="ecoprimals") }} runs on everything from headless servers to desktop workstations to Raspberry Pis — instead of five separate UI applications, {{ entity(name="petaltongue") }} adapts its representation mode to the environment.

Accessibility is not an afterthought — it is the design. Sighted users see graph visualizations. Blind users hear sonified health data (5 instruments, health-to-pitch mapping, spatial stereo panning, Pure Rust WAV export). Deaf users get visual alerts. Motor-impaired users get keyboard-only navigation. The same primal adapts to whatever representation capability is available.

**Post-{{ entity(name="nucleus") }} role**: {{ entity(name="petaltongue") }} gives the ecosystem a face. It visualizes {{ entity(name="nucleus") }} health, primal coordination, bonding state, and workload distribution in real-time via {{ entity(name="songbird") }} discovery and {{ entity(name="biomeos") }} SSE event subscription.

---

### 2.2 rhizoCrypt - Ephemeral Memory Primal

**Domain**: Content-addressed DAG engine for working memory  
{{ entity_metrics(name="rhizocrypt") }}  
**Tests**: 509/509 passing (100%)  
**Coverage**: 83.92%  
**Safety**: Zero unsafe blocks

{{ entity(name="rhizocrypt") }} is the scratch pad. Not everything should go to permanent storage. Conversation context, intermediate ML results, draft documents, exploration state — these need fast, concurrent access and zero persistence guarantees. {{ entity(name="rhizocrypt") }} provides exactly that: a content-addressed DAG (BLAKE3) for session state, working memory, and intermediate computation results. Lock-free concurrency (DashMap). Designed to be discarded — ephemeral by intent.

The name: rhizomes are underground root networks that connect plants. "Crypt" for the encrypted content-addressing. The working memory of the ecosystem, spreading connections between active sessions.

**Six slice modes** go beyond read/write permissions into nuanced data sharing: **Copy** (duplicate), **Loan** (temporary access with automatic revocation), **Escrow** (conditional release), **Mirror** (synchronized view), **Consignment** (delegated custody with provenance tracking), **Provenance** (read-only attribution chain). These map to real-world data relationships that traditional access control can't express.

When ephemeral data needs to become permanent, {{ entity(name="rhizocrypt") }} **dehydrates** it — committing session state to {{ entity(name="loamspine") }}'s immutable ledger. Together they implement the gen2 **"philosophy of forgetting"**: not everything should be remembered forever, but some things must never be forgotten. {{ entity(name="rhizocrypt") }} forgets; {{ entity(name="loamspine") }} remembers.

**Post-{{ entity(name="nucleus") }} role**: Core engine of the Memory & Attribution Stack. Provides the ephemeral working layer for {{ entity(name="rootpulse") }} (distributed version control as emergent behavior).

---

### 2.3 sweetGrass - Attribution Primal

**Domain**: Semantic provenance and fair attribution  
{{ entity_metrics(name="sweetgrass") }}  
**Tests**: 496/496 passing (100%)  
**Coverage**: 78.39%  
**Safety**: Zero unsafe blocks (`#![forbid(unsafe_code)]` in all 9 crates)

{{ entity(name="sweetgrass") }} tracks who did what, when, and why. If {{ entity(name="rhizocrypt") }} is working memory and {{ entity(name="loamspine") }} is permanent memory, {{ entity(name="sweetgrass") }} is the *context* of memory: the metadata that says where each piece came from, who contributed to it, and what rights they retain. W3C PROV-O provenance model. Fair attribution via the **Braid model**. GDPR-inspired data rights (5 privacy levels). Multiple storage backends (memory, Sled, PostgreSQL).

The name: sweetgrass is a sacred plant in many Indigenous traditions, used in purification ceremonies and as a reminder of kindness and gratitude. {{ entity(name="sweetgrass") }} the primal is about giving credit where it's due.

**Why it exists**: In a sovereign system with AI-assisted development, attribution becomes critical. Who wrote this code — the human or the AI? Who owns the data that trained the model? Who contributed to this research output? {{ entity(name="sweetgrass") }} provides machine-readable answers. The **Braid model** — where attribution threads weave together to form a composite provenance record — is original to {{ entity(name="ecoprimals") }}. The AGPL-3.0 license itself is a form of attribution enforcement; {{ entity(name="sweetgrass") }} makes it machine-readable.

{{ entity(name="sweetgrass") }} combines the deepest provenance feature set in the Memory & Attribution stack with strong verification metrics (78.39% line coverage, 496/496 tests). 12 role types, derivation chain analysis, time decay, recursive attribution propagation, ~88% compression with session dedup + zstd.

**Post-{{ entity(name="nucleus") }} role**: Attribution layer for all ecosystem data. Essential for the {{ entity(name="rootpulse") }} composition where every commit, merge, and contribution carries cryptographic attribution.

---

### 2.4 LoamSpine - Permanence Primal

**Domain**: Immutable linear ledger for selective permanence  
{{ entity_metrics(name="loamspine") }}  
**Tests**: 416 passing (100%)  
**Coverage**: 77.68%  
**Safety**: Zero unsafe blocks, zero clippy warnings (pedantic mode)

{{ entity(name="loamspine") }} is the permanent ledger — the fossil record. Sovereign append-only logs (Spines), Loam certificates (digital ownership, lending, provenance), recursive stacking, waypoint anchoring, inclusion proofs. If something needs to be permanent and verifiable, it goes to {{ entity(name="loamspine") }}.

{{ entity(name="rhizocrypt") }} handles ephemeral working memory — session state that can be discarded. But some things must persist: identity chains, ownership records, scientific provenance, license attestations. {{ entity(name="loamspine") }} provides the immutable complement. Together they implement the gen2 **"philosophy of forgetting"**: not everything should be remembered forever, but some things must never be forgotten. {{ entity(name="rhizocrypt") }} forgets; {{ entity(name="loamspine") }} remembers.

**What Pure Rust means here**: Pure Rust RPC (tarpc + JSON-RPC 2.0, no gRPC, no protobuf), zero-copy optimized (30-50% fewer allocations). {{ entity(name="loamspine") }} pioneered the **Infant Discovery** pattern — {{ entity(name="songbird") }} as a central hub reducing O(n²) discovery to O(n) — later adopted across the ecosystem. DNS SRV (RFC 2782) for production federation, mDNS (RFC 6762) for zero-config development.

**Loam certificates** with recursive stacking allow complex ownership structures: a certificate can reference other certificates, creating a DAG of provenance. "Digital lending" — temporary transfer of rights with automatic reversion — game keys, credentials, property deeds, ownership transfer. Spines serve sovereign ledgers: personal, professional, community, public.

**Post-{{ entity(name="nucleus") }} role**: Permanence layer of the Memory & Attribution Stack. Combined with {{ entity(name="rhizocrypt") }} (ephemeral) and {{ entity(name="sweetgrass") }} (attribution), forms the complete temporal data management system and the foundation for {{ entity(name="rootpulse") }} — distributed version control as an emergent behavior.

---

### 2.5 skunkBat - Defense Primal

**Domain**: Defensive network security  
{{ entity_metrics(name="skunkbat") }} — intentionally small, auditable defensive surface  
**Coverage**: 87.37% (core modules: 90-100%)

{{ entity(name="skunkbat") }} is the immune system. A skunk's defense is warning before escalation. A bat's defense is echolocation — sensing the environment without touching it. {{ entity(name="skunkbat") }} warns and senses; it does not attack.

**Why it exists**: A sovereign system that connects to the internet needs defense. But defense in a sovereign system has a constraint: the defender must not become a surveillance tool. {{ entity(name="skunkbat") }} enforces this **by design, not by policy**: it analyzes connection metadata (source, frequency, timing, patterns) but structurally cannot read message content. The codebase is intentionally small ({{ entity_stat(name="skunkbat", stat="loc_display") }} lines, {{ entity_stat(name="skunkbat", stat="tests_display") }} tests) — a security system should be simple enough to audit completely.

**Five threat types**: Genetic (unknown lineage via {{ entity(name="beardog") }}), Topology (unusual connection patterns), Behavioral (statistical baseline deviation), Intrusion (port scanning signatures), Resource (memory/CPU/bandwidth abuse).

**Graduated response**: Monitor → Warn → Throttle → Quarantine → Block. The **user authority** principle means {{ entity(name="skunkbat") }} cannot escalate to blocking autonomously — the user approves all major defensive actions. This prevents the security system from becoming an autonomous censor, a common failure mode in corporate security products.

**Post-{{ entity(name="nucleus") }} role**: Complements the {{ entity(name="darkforest") }} protocol. {{ entity(name="darkforest") }} handles identity and discovery privacy (zero metadata leakage); {{ entity(name="skunkbat") }} handles active threat detection and response within the sovereign computing environment.

---

## 3. Meta-Primals & Tooling

### 3.1 sourDough — Scaffolding & Packaging

**Domain**: Primal scaffolding, {{ entity(name="genomebin") }} packaging, ecosystem CLI tooling  
{{ entity_metrics(name="sourdough") }}  
**Classification**: Meta-primal — generates primals but does not run as a {{ entity(name="nucleus") }} service at runtime

{{ entity(name="sourdough") }} is the starter culture. `sourdough scaffold` generates a new primal skeleton with correct IPC, capability, and {{ entity(name="genomebin") }} structure — the same way a sourdough starter provides the culture that makes bread rise. It also handles {{ entity(name="genomebin") }} packaging, producing the deployable binary artifacts that flow into {{ entity(name="plasmidbin") }}. Generated primals do not depend on {{ entity(name="sourdough") }} at runtime; it is a build-time tool that enforces ecosystem conventions structurally rather than by documentation.

**Participates in**: {{ entity(name="plasmidbin") }} (produces {{ entity(name="genomebin") }} packages), {{ entity(name="wateringhole") }} (validates structure standards).

### 3.2 Infrastructure Repositories (metaPrimals)

These are not runtime primals but essential ecosystem infrastructure:

| Repo | Emoji | Purpose |
|------|-------|---------|
| {{ entity(name="wateringhole") }} | 💧🕳️ | Shared standards, glossary, handoffs — the "dev tool" repo available to all ecosystem projects |
| {{ entity(name="whitepaper") }} | 📄✍️ | Research documentation, {{ entity(name="basecamp") }} papers, gen3/gen4 architecture |
| {{ entity(name="sporeprint") }} | 🍄🖨️ | Public-facing website and verification portal (this site) |
| [plasmidBin](https://github.com/ecoPrimals/plasmidBin) | 🧬📦 | Binary distribution surface — pre-built primal binaries, checksummed and versioned. See [Deployment Model](@/architecture/DEPLOYMENT_MODEL.md) |

### 3.3 Additional Tooling (Publishing Soon)

These three repositories are active codebases, {{ entity(name="scyborg") }}-licensed, and will be published to GitHub imminently. Binaries are available via [plasmidBin](https://github.com/ecoPrimals/plasmidBin).

#### bingoCube — Human-Verifiable Cryptographic Commitment

**Domain**: Verifiable commitment, BLAKE3 progressive reveal, visual/audio identity verification  
{{ entity_metrics(name="bingocube") }}  
**Repository**: ecoPrimals/bingoCube — publishing soon

{{ entity(name="bingocube") }} bridges the gap between mathematical proof and human trust. Cryptographic commitments are hash strings — correct, but meaningless to humans. {{ entity(name="bingocube") }} uses BLAKE3 progressive reveal (commit → partial reveal → full reveal) to generate visual and audio identity verification patterns: a "bingo card" that a human can check without understanding cryptography. You don't need to read hex to verify identity. Visual verification patterns are human-recognizable; tonal fingerprints provide audio identity. Integrates with {{ entity(name="beardog") }} identity and {{ entity(name="darkforest") }} discovery.

#### agentReagents — AI Agent Toolkit

**Domain**: AI agent composition, reagent patterns, sovereign AI orchestration  
**Repository**: ecoPrimals/agentReagents — publishing soon

agentReagents provides the chemistry of AI agent composition. Rather than building agents from scratch, developers compose **reagents** — pre-validated behavioral building blocks — into agents that respect data sovereignty and run locally. The chemistry metaphor is deliberate: reagents combine predictably, their interactions are testable, and the resulting agents inherit the properties of their components. Complements {{ entity(name="squirrel") }}'s MCP coordination with higher-level agent architecture patterns. No cloud dependency; vendor-agnostic inference routing.

#### benchScale — Benchmark & Performance Characterization

**Domain**: Cross-primal benchmarking, performance characterization, scaling studies  
**Repository**: ecoPrimals/benchScale — publishing soon

benchScale measures how primals scale — individually and in composition. It provides standardized cross-primal benchmark suites, identifies bottlenecks at composition boundaries (where JSON-RPC latency matters), and produces reproducible performance reports. The scaling characterization data informs deploy graph optimization and {{ entity(name="byob") }} composition presets: which primals to co-locate, where to split across machines, what the cost of each IPC hop actually is.

---

## 4. Ecosystem Summary

### 4.1 By the Numbers

| Metric | Value |
|--------|-------|
| Foundation primals (production) | 8 ({{ entity(name="beardog") }}, {{ entity(name="songbird") }}, {{ entity(name="nestgate") }}, {{ entity(name="toadstool") }}, {{ entity(name="squirrel") }}, {{ entity(name="biomeos") }}, {{ entity(name="coralreef") }}, {{ entity(name="barracuda") }}) |
| Post-{{ entity(name="nucleus") }} primals (started) | 5 ({{ entity(name="petaltongue") }}, {{ entity(name="rhizocrypt") }}, {{ entity(name="sweetgrass") }}, {{ entity(name="loamspine") }}, {{ entity(name="skunkbat") }}) |
| Meta/tooling | 1 ({{ entity(name="sourdough") }}) + 4 infra repos |
| Additional tooling (publishing soon) | 3 ({{ entity(name="bingocube") }}, agentReagents, benchScale) |
| Public primal repos | 4 (toadStool, squirrel, {{ entity(name="coralreef") }}, {{ entity(name="barracuda") }}) |
| {{ entity(name="sporegarden") }} products | 3 ({{ entity(name="esotericwebb") }}, {{ entity(name="helixvision") }}, blueFish) |
| Binary distribution | [plasmidBin](https://github.com/ecoPrimals/plasmidBin) — 18 entries (12 primals + 6 springs) |
| License | **{{ entity(name="scyborg") }}** — AGPL-3.0-or-later (code) + ORC (game mechanics) + CC-BY-SA 4.0 (creative/docs) |
| Primal Rust LOC | {{ total_stat(stat="primal_loc_display") }} (measured via tokei, {{ total_stat(stat="measured_date") }}) |
| Spring Rust LOC | {{ total_stat(stat="spring_loc_display") }} (8 springs, measured via tokei) |
| Total Rust LOC | {{ total_stat(stat="total_loc_display") }} |
| WGSL shaders | {{ total_stat(stat="wgsl_files") }} files, {{ total_stat(stat="wgsl_lines_display") }} lines |
| Primal test functions | {{ total_stat(stat="primal_tests_display") }} |
| Spring test functions | {{ total_stat(stat="spring_tests_display") }} |
| Total test functions | {{ total_stat(stat="total_tests_display") }} |
| Development time | ~6-8 months |
| Developer count | 1 (with AI assistance) |
| C dependencies | Zero (entire ecosystem) |
| Unsafe code blocks | Near zero across all production code |
| Languages | Rust (all application code), WGSL (GPU shaders) |
| IPC protocol | JSON-RPC 2.0 (universal) |
| Platforms | Linux, macOS, Android, Windows, FreeBSD, illumos, WASM |

### 4.2 Key Achievements

**{{ entity(name="toweratomic") }} - Pure Rust HTTPS**: {{ entity(name="beardog") }} + {{ entity(name="songbird") }} achieve TLS 1.3 with 93% validation rate across 87 production sites, zero C dependencies, 366ms average latency. No other Pure Rust project has achieved this at comparable scale.

**Sovereign Compute Pipeline**: {{ entity(name="barracuda") }} (primal #14) writes WGSL math shaders, {{ entity(name="coralreef") }} (primal #13) compiles to native GPU binaries, {{ entity(name="toadstool") }} dispatches on hardware. 786 production WGSL shaders across 10 scientific domains. Both {{ entity(name="coralreef") }} and {{ entity(name="barracuda") }} were promoted from {{ entity(name="toadstool") }} sub-crates to independent primals as the pipeline matured. See `gen3/primals/13_coralreef.md` and `gen3/primals/14_barracuda.md`.

**{{ entity(name="darkforest") }} - Zero Metadata Security**: {{ entity(name="biomeos") }}'s discovery protocol leaks zero metadata to observers. Beacons are indistinguishable from random noise. Better than Signal or Tor for metadata privacy (Signal leaks sender/receiver metadata; Tor leaks timing metadata; {{ entity(name="darkforest") }} leaks nothing).

**Pure Rust Tor**: {{ entity(name="songbird") }} implements Tor directory, circuit, stream, and onion service in 3,345 lines of Pure Rust, delegating all crypto to {{ entity(name="beardog") }}. Sovereign P2P without dependency on the Tor network.

**Neuromorphic Computing**: {{ entity(name="toadstool") }}'s Pure Rust Akida driver detects and utilizes 160 neuromorphic processing units for bioinformatics, LLM intent classification, and image classification. No other Rust project has a production neuromorphic driver.

**Accessibility-First UI**: {{ entity(name="petaltongue") }} provides 6 accessibility scenarios (blind, deaf, nonverbal, illiterate, motor disability, deaf-blind) as first-class demonstrations, not afterthoughts.

### 4.3 Composed Systems

| System | Primals Involved | Status |
|--------|-----------------|--------|
| {{ entity(name="toweratomic") }} | {{ entity(name="beardog") }} + {{ entity(name="songbird") }} | Production (93% TLS validation) |
| {{ entity(name="nodeatomic") }} | Tower + {{ entity(name="toadstool") }} | Production |
| {{ entity(name="nestatomic") }} | Tower + {{ entity(name="nestgate") }} | Production |
| Full {{ entity(name="nucleus") }} | All foundation primals | Production |
| {{ entity(name="rootpulse") }} | {{ entity(name="rhizocrypt") }} + {{ entity(name="loamspine") }} + {{ entity(name="nestgate") }} + {{ entity(name="beardog") }} + {{ entity(name="sweetgrass") }} + {{ entity(name="songbird") }} | Architecture defined, integration evolving |
| Memory Stack | {{ entity(name="rhizocrypt") }} + {{ entity(name="loamspine") }} + {{ entity(name="sweetgrass") }} | All primals production-ready, composition evolving |
| {{ entity(name="darkforest") }} | {{ entity(name="beardog") }} + {{ entity(name="songbird") }} + {{ entity(name="biomeos") }} | Production (A++ LEGENDARY) |
| Sovereign NAT | {{ entity(name="songbird") }} + {{ entity(name="beardog") }} (Tower) | Production (Tiers 1-3) |

### 4.4 Showcase Inventory

Every foundation primal has demonstration material. Most post-{{ entity(name="nucleus") }} primals have extensive showcase suites:

| Primal | Showcase Demos | Interactive Scripts | Real-World Scenarios |
|--------|---------------|--------------------|--------------------|
| {{ entity(name="beardog") }} | Local showcase | Shell demo | - |
| {{ entity(name="songbird") }} | Examples dir | Rust/Python/JS clients | Tor, P2P, Federation |
| {{ entity(name="nestgate") }} | 14+ demos | Multiple levels | Bioinformatics, ML, Media, Git LFS |
| {{ entity(name="toadstool") }} | GPU, Neuromorphic, Homomorphic | Benchmark suites | Gaming, Research |
| {{ entity(name="squirrel") }} | IPC demos | Android validation | Multi-model AI |
| {{ entity(name="biomeos") }} | {{ entity(name="neuralapi") }}, {{ entity(name="darkforest") }} | Federation demos | Cross-device |
| {{ entity(name="petaltongue") }} | 8+ local, 7 inter-primal | Sandbox scenarios | 6 accessibility scenarios |
| {{ entity(name="rhizocrypt") }} | 8 local, 6 inter-primal | Level-based progression | {{ entity(name="rootpulse") }} integration |
| {{ entity(name="sweetgrass") }} | 8 local, 7 coordination | 50+ interactive | HIPAA, Science, Royalties, ML, Supply Chain |
| {{ entity(name="loamspine") }} | 21 demos, 12 examples | 4-level progression | - |
| {{ entity(name="skunkbat") }} | 10 examples, 4 levels | Threat detection suite | Federation, Integration |

---

## 5. The Evidence

This catalog documents what 6-8 months of constrained evolution produced. The primary focus has been the 8 foundation primals that form {{ entity(name="nucleus") }} — getting the core deployment architecture stable. The post-{{ entity(name="nucleus") }} primals have been started and have functional code, but receive less focus until {{ entity(name="nucleus") }} is solid. The methodology paper (`CONSTRAINED_EVOLUTION_FORMAL.md`) makes claims about how environmental constraints drive specialization. This catalog is the evidence.

**Claim**: Constraints drive specialization, not predetermined solutions.  
**Evidence**: {{ entity(name="toweratomic") }} was not planned. The Pure Rust constraint eliminated OpenSSL, which forced the composition pattern, which produced Pure Rust HTTPS.

**Claim**: All populations show increased fitness, even without breakthrough innovation.  
**Evidence**: Every primal - including those without headline innovations like {{ entity(name="nestgate") }}'s storage and {{ entity(name="squirrel") }}'s AI routing - became increasingly idiomatic, well-tested, and specialized to the Rust + async + JSON-RPC environment over iterative cycles.

**Claim**: Independent evolution under shared constraint produces convergent but non-identical solutions.  
**Evidence**: All primals converged on JSON-RPC 2.0, capability-based discovery, async tokio, and Pure Rust dependencies. But each primal's implementation is independently developed. {{ entity(name="beardog") }}'s IPC handler and {{ entity(name="songbird") }}'s IPC handler are different code that converged on the same protocol.

**Claim**: The methodology scales across domains.  
**Evidence**: The same methodology produced a cryptography primal, a networking primal, a storage primal, a compute primal, an AI primal, an orchestration primal, a UI primal, a DAG engine, a provenance tracker, a ledger, and a defense system. All in Rust, all following the same standards, all independently evolved.

---

**This catalog records what exists. Every primal listed here compiles, runs, and passes its tests. The benchmarks are measured, not estimated. The showcase demos execute, not simulate. The architecture is implemented, not proposed.**
