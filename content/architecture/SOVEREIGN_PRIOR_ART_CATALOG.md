+++
title = "ecoPrimals — Sovereign Prior Art Catalog"
description = "Lysogeny prior art record for sovereign compute"
date = 2026-03-17

[taxonomies]
primals = ["beardog", "barracuda", "biomeos", "coralreef", "loamspine", "nestgate", "petaltongue", "rhizocrypt", "skunkbat", "songbird", "squirrel", "sweetgrass", "toadstool"]
springs = ["airspring", "groundspring", "hotspring", "neuralspring", "wetspring"]
+++

# ecoPrimals — Sovereign Prior Art Catalog

## AGPL-3.0 Commons Inventory

**Date**: March 13, 2026
**Purpose**: Catalog all prior art locked into AGPL-3.0 public commons.
Sovereign code is code that CANNOT be recaptured by fictions (corporations)
but is free to use, study, modify, and share by all humans.

**Totals**: ~1.5M lines Rust | ~84K lines WGSL | ~63,000+ tests & checks |
~100 reproduced papers | 13 primals | 5 springs | 20+ scientific domains |
52 novel innovations | AGPL-3.0 perpetually locked

---

## Classification

**Established prior art**: Implementations of known techniques in pure Rust
under AGPL-3.0. The technique exists elsewhere (often in C/C++ under
permissive licenses), but the AGPL Rust implementation is sovereign —
it cannot be absorbed into proprietary stacks.

**Novel prior art**: Capabilities, architectures, or integrations that do
not exist in any other open-source project. These are new to technology
via ecoPrimals.

---

## I. Foundation Primals

### BearDog — Cryptographic Service Provider

| Capability | Type | Description |
|-----------|------|-------------|
| Tower Atomic pattern | **Novel** | Crypto provider (BearDog) separated from protocol (Songbird) via JSON-RPC IPC. No other system cleanly separates crypto operations from transport this way. |
| Genetic lineage | **Novel** | Device seed derivation from root key, lineage certificates, challenge-response authentication. Cryptographic proof of device ancestry. |
| Dark Forest beacon | **Novel** | Zero-metadata beacon keys via `genetic.derive_lineage_beacon_key`. HKDF + domain separation for discoverable-but-private service advertisement. |
| Multi-family isolation | **Novel** | Per-family `--family-id` instances with fully isolated key derivation trees. |
| Tor v3 crypto in pure Rust | Established | Onion address derivation, ntor handshake, cell encryption. Standard Tor spec, novel in pure Rust without C dependencies. |
| RustCrypto primitives | Established | Ed25519, X25519, AES-GCM, ChaCha20-Poly1305, BLAKE3, SHA-2/3, HKDF, Argon2id. Standard crypto, sovereign implementation. |
| Universal HSM abstraction | Established | Vendor-agnostic HSM trait (software, PKCS#11, StrongBox). Pattern known; AGPL implementation is sovereign. |

**30 crates | 13,720 lines Rust | 12,751+ tests | 78.6% coverage**

---

### Songbird — Network Orchestration

| Capability | Type | Description |
|-----------|------|-------------|
| Pure Rust Tor stack | **Novel** | Full directory, circuit, stream, onion service in pure Rust. Most Tor implementations use C (arti is Rust but not AGPL). This is the only AGPL-3.0 Tor stack. |
| Sovereign Onion service | **Novel** | P2P encrypted service with all crypto delegated to BearDog via IPC. No local crypto state in the network layer. |
| NFC Genesis pairing | **Novel** | Dark Forest mobile pairing with low metadata leakage. Zero-knowledge device introduction. |
| 100% crypto delegation | **Novel** | Zero cryptographic operations in the network layer. All delegated to BearDog via JSON-RPC IPC. No other networking stack operates this way. |
| Infant Discovery | **Novel** | O(n) discovery via central hub instead of O(n²) mesh. Scalable service discovery without broadcast storms. |
| QUIC + TLS 1.3 | Established | Standard protocols, sovereign implementation. |
| STUN + IGD NAT traversal | Established | RFC 5389 + UPnP IGD, sovereign implementation. |

**~27 crates | 8,515+ tests | 60.84% coverage | #![forbid(unsafe_code)]**

---

### NestGate — Universal Storage

| Capability | Type | Description |
|-----------|------|-------------|
| Isomorphic IPC | **Novel** | Single binary adapts transport (Unix socket → TCP) by platform at runtime. Same binary on Linux, macOS, FreeBSD, WSL2, Android. |
| Adaptive backend | **Novel** | Unified try-optimize-fallback pattern for storage, ZFS, and IPC. |
| Capability-based primal discovery | **Novel** | Runtime discovery by capability string ("crypto", "storage"), not hardcoded primal names. |
| NAT traversal persistence | Established | Relay-assisted coordinated punch, standard pattern. |
| ZFS integration | Established | ZFS management in Rust, other implementations exist. |

**13 crates | 12,155 tests | 70.07% coverage | zero production unwrap/expect**

---

### Squirrel — AI Coordination

| Capability | Type | Description |
|-----------|------|-------------|
| TRUE PRIMAL pattern | **Novel** | Self-knowledge only, runtime discovery, zero compile-time coupling to any AI vendor or other primal. |
| Vendor-agnostic AI routing | **Novel** | Cost/quality/latency-based provider selection across Ollama, llama.cpp, vLLM, OpenAI, Anthropic, Gemini without vendor-specific code paths. |
| Isomorphic multi-platform IPC | **Novel** | Same binary across Linux, Android, Windows, macOS, BSD, WASM. |
| Sovereign MCP | Established | Model Context Protocol implementation; MCP exists elsewhere, AGPL integration is sovereign. |

**~15 crates | 1,957 tests | 13/15 chaos tests passing**

---

### ToadStool — Hardware Management

| Capability | Type | Description |
|-----------|------|-------------|
| hw-learn pipeline | **Novel** | Observe → distill → apply → share for GPU initialization. Vendor-neutral, self-teaching hardware driver. No equivalent exists in any open-source project. |
| VFIO GPU backend (pure Rust) | **Novel** | BAR0 MMIO + DMA + bind/unbind for NVIDIA GPUs via VFIO in pure Rust. First userspace GPU driver in Rust. |
| NvPmu (nvidia-smi replacement) | **Novel** | GPU telemetry (temperature, power, clock) via sysfs/hwmon without proprietary nvidia-smi. Pure Rust. |
| PrecisionBrain | **Novel** | O(1) precision routing from hardware calibration. Domain-aware (Critical/Moderate/Throughput) to precision tier (F64/DF64/F32). Includes NVVM transcendental risk detection. |
| Cross-vendor GPU init recipes | **Novel** | `InitRecipe` format with `GpuGen` enum (Maxwell→Ampere), `classify_register_for_gen()`, `RegisterAccess` trait. |
| Sovereign dispatch pipeline | **Novel** | `compute.dispatch.submit/status/result/forward` JSON-RPC. Thermal gating. Multi-GPU parallel init. Cross-gate forwarding. |
| Science/gaming mode switching | **Novel** | `ecoprimals-mode` CLI to switch GPU between display driver and vfio-pci. Dual-use architecture. |
| Akida NPU driver (VFIO) | **Novel** | BrainChip Akida neuromorphic processor via VFIO in pure Rust. Only Rust NPU driver that exists. |
| DMA allocator with huge pages | Established | Standard pattern; integration with VFIO sovereign stack is the main angle. |
| Spring absorption pattern | **Novel** | Write→Absorb→Lean methodology. Springs evolve capabilities, toadStool absorbs proven patterns (PrecisionBrain, NvkZeroGuard, StreamingDispatch, etc.). |

**~50 crates | 550,941 lines Rust | 20,262 tests | 83% coverage**

---

### BearDog Crypto Totals (locked into AGPL-3.0)

| Primitive | Count |
|-----------|-------|
| Signature algorithms | 3 (Ed25519, ECDSA, RSA) |
| Key exchange | 2 (X25519, ECDHE) |
| AEAD ciphers | 3 (ChaCha20-Poly1305, AES-128-GCM, AES-256-GCM) |
| Hash functions | 5 (BLAKE3, SHA-256, SHA-384, SHA-512, SHA3-256) |
| KDFs | 4 (HKDF, TLS PRF, PBKDF2, Argon2id) |
| Password hashing | 3 (Argon2id, bcrypt, scrypt) |
| Protocols | 3 (TLS 1.3, Tor v3, BTSP) |

---

## II. Compute Trio

### coralReef — GPU Shader Compiler & Driver

| Capability | Type | Description |
|-----------|------|-------------|
| WGSL → native SASS compiler | **Novel** | Compiles WGSL directly to NVIDIA native ISA (SASS) in pure Rust. No CUDA, no PTXAS, no LLVM. No other AGPL shader compiler exists. |
| WGSL → native GFX ISA compiler | **Novel** | Compiles WGSL to AMD native ISA (GFX) in pure Rust. Multi-vendor native compilation from a single source language. |
| VFIO compute dispatch | **Novel** | Full ComputeDevice implementation via VFIO: BAR0 MMIO, DMA buffers, GPFIFO submission, GP_GET sync. Userspace GPU compute dispatch without any kernel GPU driver. |
| VFIO DMA subsystem | **Novel** | Page-aligned, mlock'd, IOMMU-mapped DMA buffers in pure Rust via rustix. No libc. |
| BAR0 sovereign GR init | **Novel** | Address-aware firmware split: high-address entries → BAR0 MMIO, low-address entries → FECS channel. Sovereign PGRAPH initialization. |
| UVM GPFIFO dispatch | Established | NVIDIA UVM (Unified Virtual Memory) dispatch. Uses proprietary nvidia-drm, but the Rust implementation wrapping it is sovereign. |
| DRM nouveau dispatch | Established | Linux DRM dispatch via nouveau UAPI. Standard kernel interface, sovereign Rust wrapper. |
| Three dispatch paths | **Novel** | Single codebase with DRM, UVM, and VFIO dispatch paths. Preference ordering (VFIO > DRM > UVM) with automatic fallback. No other project offers this. |
| QMD builder | Established | Queue Meta Data construction for NVIDIA compute dispatch. Reverse-engineered from public specs. |
| `GpuContext::from_vfio()` | **Novel** | High-level API: BDF string → compiled shader → native dispatch on bare metal. One function call from application to GPU. |

**122 WGSL files | 116,960 lines Rust | 1,704 tests**

---

### barraCuda — GPU Math Engine

| Capability | Type | Description |
|-----------|------|-------------|
| 806 WGSL scientific shaders | **Novel** | Largest AGPL-3.0 scientific shader library. Covers QCD, plasma physics, bioinformatics, pharmacology, Anderson localization, RHMC, neural networks, and more. |
| DF64 (double-float emulation) | **Novel** | Hardware-atheistic FP64 precision via f32×2 pairs in WGSL. Works on GPUs without native FP64. 30+ DF64 shaders. No other WGSL DF64 library exists. |
| RHMC (Rational HMC) | **Novel** | Remez exchange, multi-shift CG, rational approximations for lattice QCD fermion dynamics. Pure Rust + WGSL. |
| #![forbid(unsafe_code)] math engine | **Novel** | 291,543 lines of Rust + 64,737 lines of WGSL with zero unsafe. No other GPU math engine of this scale has this guarantee. |
| VFIO-primary architecture | **Novel** | VFIO is the designed primary dispatch path, wgpu is fallback. Inverts the normal relationship (vendor driver primary, alternative secondary). |
| GpuBackend trait | Established | Trait-based GPU abstraction. Pattern exists elsewhere; the sovereign dispatch integration is novel. |
| Precision tiers specification | **Novel** | F32/F64/F64Precise/DF64 tiers with per-shader, per-hardware routing. No other framework offers this granularity. |

**806 WGSL shaders | 291,543 lines Rust | 64,737 lines WGSL | 3,415 tests**

---

## III. Phase 2 — Federation & Representation

### biomeOS — Autonomous Federation Platform

| Capability | Type | Description |
|-----------|------|-------------|
| NUCLEUS atomic composition | **Novel** | Tower/Node/Nest/Full pre-composed atomic patterns. Chemical bonding model (Ionic/Covalent/Metallic/Weak) for distributed system composition. No other system uses chemical bonding metaphors for microservice topology. |
| Neural API (165+ translations) | **Novel** | Semantic capability routing. Primals compose by capability string, not by name or address. 13 domains, 165+ translations. No hardcoded primal references anywhere. |
| Plasmodium collective | **Novel** | Physarum polycephalum-inspired decentralized orchestration. HTTP JSON-RPC collective with dynamic join/leave. No central coordinator. Emergent routing. |
| Dark Forest beacon genetics | **Novel** | Zero-metadata discovery. Genetic lineage = decryption key. Privacy beyond Signal or Tor — metadata itself is invisible, not just encrypted. |
| Mitochondrial + Nuclear DNA model | **Novel** | Two-seed identity: beacon seed (mitochondrial, service discovery) + lineage seed (nuclear, trust chain). Distinct security semantics from biological analogy. |
| ContinuousExecutor graph engine | **Novel** | Fixed-timestep graph loops with feedback edges. 60Hz tick clock for game loops, 90Hz for surgical simulation. Graph-based continuous execution with domain-specific tick rates. |
| Surgical VR deployment graph | **Novel** | Anatomy + tissue physics + biosignals + pharmacokinetics composed as a single graph. Integrated medical simulation pipeline from primal composition. |
| Cross-spring ecology graphs | **Novel** | airSpring → wetSpring → neuralSpring pipeline defined in TOML. Domain-specific ecology where springs feed each other's outputs. |
| genomeBin v3 deployment | **Novel** | Binary isomorphic format. Same binary, same behavior, any substrate. Deterministic deployment with genetic lineage tracking. |
| LifecycleManager auto-resurrection | Established | Health monitoring with auto-restart. Pattern known; sovereign Rust implementation with NUCLEUS integration. |
| 4-tier NAT traversal | Established | LAN/punch/coordinated/relay. Standard pattern; sovereign strategy with BearDog crypto. |
| Federation & sub-federation | Established | Hierarchical trust. Standard pattern; sovereign with genetic lineage gating. |

**26 crates | 191,658 lines Rust | 3,670+ tests | 71.47% coverage | 0 unsafe**

---

### petalTongue — Universal Representation & UI

| Capability | Type | Description |
|-----------|------|-------------|
| UniBin five-mode rendering | **Novel** | Single binary renders to GUI (egui), TUI (ratatui), web (axum), headless, and status. ~84% size reduction vs separate binaries. No other UI framework offers five modalities from one binary. |
| Proprioception (SAME DAVE) | **Novel** | UI self-awareness of its own outputs and inputs. Diagnostic events. The UI knows what it is displaying and can reason about its own state. |
| Universal representation engine | **Novel** | One data model → terminal, SVG, PNG, egui, audio sonification, future VR. Modality-agnostic rendering from a single source of truth (DataService). |
| Human entropy capture | **Novel** | Multi-modal entropy from audio, visual, narrative, and gesture inputs. Fed to BearDog for cryptographic randomness. Human interaction as entropy source. |
| Discovery → performance split | **Novel** | biomeOS JSON-RPC for service discovery, tarpc for hot-path data. Automatic protocol upgrade from discovery to high-performance. |
| UIBackend trait system | **Novel** | Pluggable display backends (eframe, framebuffer, ToadStool GPU). Same rendering logic regardless of display technology. |
| DataService single source of truth | **Novel** | Event-driven broadcast to all modalities. One data model, broadcast updates, any renderer subscribes. |
| Graph sonification | **Novel** | Graph topology rendered as audio. Accessibility and multi-modal representation of network state. |
| egui/eframe GUI | Established | Immediate-mode GUI. Standard framework, sovereign integration. |
| ratatui TUI | Established | Terminal UI. Standard framework, sovereign integration. |
| axum web server | Established | HTTP/WebSocket. Standard framework, sovereign integration. |

**19 crates | ~400+ tests | A+ grade (95/100) | AGPL-3.0**

---

## IV. Provenance Trio + Post-NUCLEUS Primals (Phase 2)

### loamSpine — Permanent Ledger

| Capability | Type | Description |
|-----------|------|-------------|
| Loam certificates with lending | **Novel** | Sovereign lendable certificates with `LoanTerms` (duration, grace, auto_return). `CertificateManager.process_expired_loans()` auto-reverts. Full provenance via `MintInfo`, `CertificateLocation`, `OwnershipRecord`. No blockchain required. |
| Waypoint spines | **Novel** | Local permanence for borrowed state without upward propagation. Waypoint-only entry types (`SliceAnchor`, `SliceOperation`, `SliceDeparture`). Borrowed data stays locally permanent without polluting the origin spine. |
| Infant Discovery pattern | **Novel** | Zero-config startup. Five-tier discovery: env vars → DNS-SRV → service registry HTTP → mDNS → dev fallback. Capability-based ("Who can sign?") not name-based ("Where is BearDog?"). |
| Temporal Moments | **Novel** | Domain-agnostic time with `Moment`, `MomentContext` (CodeChange, ArtCreation, LifeEvent), and four anchor types (Crypto, Atomic, Causal, Consensus). Timestamps mean different things in different contexts. |
| 15 entry types | **Novel** | Genesis, SessionCommit, SliceCheckout/Return, DataAnchor, BraidCommit, CertificateMint/Transfer/Loan/Return, SliceAnchor/Operation/Departure, TemporalMoment, Custom. Richer than any append-only ledger. |
| `permanent-storage.*` wire compat | **Novel** | rhizoCrypt dehydration commits arrive via JSON-RPC `permanent-storage.commitSession` — loamSpine speaks rhizoCrypt's wire format natively. |
| Hash-linked spine chain | Established | BLAKE3, append-only, signed entries. Standard pattern; sovereign with Sled pure Rust backend. |
| Inclusion/Certificate/Provenance proofs | Established | Path-to-tip proofs, mint+transfer chains, custody chains. Standard crypto proofs; sovereign. |

**3 crates | 549 tests | ~90% coverage | #![forbid(unsafe_code)] | AGPL-3.0**

---

### rhizoCrypt — Ephemeral Working Memory

| Capability | Type | Description |
|-----------|------|-------------|
| Six slice modes | **Novel** | Copy (no lineage), Loan (auto-return on expiry), Consignment (possession without ownership, auction semantics), Escrow (multi-party confirmation), Waypoint (local spine anchoring), Transfer (full ownership). Each has distinct resolution routes. No other content-addressed storage offers this. |
| Rhizo-Loam layering | **Novel** | Ephemeral DAG (rhizoCrypt) over permanent linear spine (loamSpine). Working memory crystallizes into permanent record via dehydration. Biological metaphor: root network feeding into trunk. |
| Philosophy of forgetting | **Novel** | Ephemeral by default; persistent only by consent. Sessions expire. Data that isn't dehydrated is garbage collected. Anti-pattern to "store everything forever." |
| Conditional resolution routing | **Novel** | Five resolution routes: `ReturnToOrigin`, `CommitToOrigin`, `RouteToSpine`, `WaypointReturn`, `Conditional`. Outcome-based and event-based routing of slice resolution. |
| Dehydration protocol | **Novel** | Freeze → Merkle root (topological sort) → summary → resolve slices → collect attestations → commit to LoamSpine. Multi-step crystallization of ephemeral state into permanent record. |
| Session lifecycle state machine | **Novel** | `Active → Paused | Resolving → Committed | Discarded → Expired`. `DashMap` lock-free concurrency. Full GC after commit/discard. |
| BLAKE3 content-addressed DAG | Established | Vertex = BLAKE3(canonical CBOR). Multi-parent DAG. Standard content-addressing; sovereign with deterministic CBOR encoding. |
| Merkle tree with proofs | Established | Binary Merkle over topological vertex order. Standard construction; sovereign implementation. |

**3 crates | 491 tests | 3 fuzz targets | #![forbid(unsafe_code)] | AGPL-3.0**

---

### sweetGrass — Attribution & Provenance

| Capability | Type | Description |
|-----------|------|-------------|
| Braid attribution with decay | **Novel** | 12 configurable agent roles with weights (Creator 0.40, Contributor 0.25, Transformer 0.20, Curator 0.10, Publisher 0.05). Inheritance decay `0.5^depth` across derivation chains. `calculate_rewards()` maps shares to value. No blockchain. |
| 0/1/Many compression | **Novel** | Session analysis → Discard/Single/Multiple strategy. Meta-Braids summarize Braid collections. DAG compression with configurable outcomes. |
| Inter-primal contribution API | **Novel** | `ContributionRecord` and `SessionContribution` for any primal to report work. `sweetgrass.recordContribution` JSON-RPC. Domain metadata keys for chemistry, ML, games. Any primal can attribute. |
| Niche-configurable semantics | **Novel** | Same codebase adapts attribution behavior per biomeOS niche: Distributed Science, Gaming, Audit Trail. Context-dependent provenance. |
| GDPR-inspired data rights | **Novel** | Five data subject requests (Access, Rectification, Erasure, Portability, Objection). Consent tracking (Explicit, Implicit, Withdrawn). Five retention policies. Five privacy levels. Applied to scientific provenance — not just personal data. |
| Domain metadata keys | **Novel** | Well-known keys for chemistry (`CHEMISTRY_*`), ML (`ML_*`), games (`GAME_*`). Extensible attribution vocabulary for scientific domains. |
| W3C PROV-O export | Established | Entity, Activity, Agent → JSON-LD with `@context`/`@graph`. `prov`, `xsd`, `rdfs`, `schema`, `ecop` namespaces. Standard ontology; sovereign with ecoPrimals extensions. |
| Multi-backend storage | Established | Memory, PostgreSQL (with migrations), Sled (pure Rust). Standard pluggable storage. |

**9 crates | 553 tests | 3 fuzz targets | proptest | #![forbid(unsafe_code)] | AGPL-3.0**

---

### skunkBat — Defensive Security

| Capability | Type | Description |
|-----------|------|-------------|
| Metadata-only threat detection | **Novel** | Detects threats from packet metadata without content inspection. Privacy-preserving by design. |
| User authority principle | **Novel** | No autonomous blocking. Graduated response (Monitor → Alert → Block) with human approval required for escalation. |

**7,366 lines Rust | 48 tests | 2 crates**

---

## V. Springs — Scientific Validation Layer

### hotSpring — Computational Physics Reproduction

| Capability | Type | Description |
|-----------|------|-------------|
| DF64 hybrid precision (9.9× native f64) | **Novel** | 3.24 TFLOPS at 14-digit precision on consumer FP32 cores. f32×2 double-float emulation exceeding native f64 throughput. No other GPU physics framework offers this. |
| NPU physics pipeline | **Novel** | MD → ESN → AKD1000 NPU → transport coefficients at 9,017× less energy. First neuromorphic silicon in a physics pipeline. |
| Lattice QCD phase detection without FFT | **Novel** | NPU phase classification from position-space observables. Bypasses Fourier transform entirely. |
| 10 NPU SDK assumptions overturned | **Novel** | Documented in `BEYOND_SDK.md`. Proved vendor assumptions wrong about their own hardware. |
| Heterogeneous real-time HMC monitor | **Novel** | Live phase detection with 0.09% overhead, predictive steering. |
| Backend-agnostic MD engine | **Novel** | `MdEngine<B: GpuBackend>` — same physics, any dispatch path (wgpu, VFIO, DRM). |
| Dense plasma MD (Sarkas) | Established | 9/9 DSF cases, 0.000% drift. Yukawa OCP. Sovereign Rust implementation. |
| Nuclear EOS (SEMF+HFB) | Established | L1 χ²=2.27 (478× faster than Python). AME2020 validated. |
| Green-Kubo transport | Established | D*/η*/λ* from Stanton-Murillo 2016. 13/13 validated. |
| Pure gauge SU(3) lattice QCD | Established | Wilson action, HMC, gradient flow. 12/12 validated. |
| Anderson localization (1D/2D/3D) | Established | Kachkovskiy spectral theory. 31/31 validated. |
| Abelian Higgs model | Established | Bazavov 2015. 17/17, 143× faster than Python. |
| Chuna dielectric/BGK | Established | Gradient flow, Mermin dielectric, kinetic-fluid. 44/44 validated. |

**2 crates | 86 WGSL shaders | 848 lib tests + 115 validation binaries | 25+ papers reproduced | 10 scientific domains**

---

### neuralSpring — Learning Layer

| Capability | Type | Description |
|-----------|------|-------------|
| Nautilus Shell bridge | **Novel** | Feed-forward evolutionary reservoir replacing recurrent ESN. Board populations instead of temporal feedback. Cross-spring integration with hotSpring brain architecture. |
| baseCamp biophysical AI | **Novel** | Weight matrices as disordered Hamiltonians, information flow as wave propagation, loss landscapes as energy landscapes, neural networks as PGMs, multi-agent AI as quorum sensing. 5 sub-theses, 128/128 validation. |
| Cross-spring spectral rewire | **Novel** | hotSpring diagnostics (bandwidth, condition number, phase) absorbed into `WeightSpectralResult`. GPU ESN via BarraCuda tensors. 41/41 validated. |
| WDM ESN regime classifier | **Novel** | GPU echo state network for warm dense matter regime classification. 96.5% accuracy. |
| Isomorphic primitive catalog | **Novel** | Maps shared ML primitives across 8+ domains to BarraCuda ops. Same MatMul/Attention/LayerNorm serves protein, language, physics, spectral, evolution. |
| helixVision protein structure | **Novel** | Sovereign AlphaFold2/3-style structure prediction (Evoformer, Pairformer, diffusion, IPA) in pure Rust + WGSL. |
| ESN (Jaeger) | Established | CPU + GPU reservoir computing. Sovereign implementation. |
| HMM forward/backward/Viterbi | Established | Liu et al. phylogenetics. GPU-accelerated. |
| Anderson localization (spectral) | Established | Bourgain-Kachkovskiy. Shared with hotSpring. |
| Replicator dynamics / game theory | Established | Bruger-Waters QS cooperation. GPU spatial payoff. |
| PINN / DeepONet | Established | Raissi 2019, Lu 2021. Sovereign implementations. |
| LeNet-5, MLP, LSTM, Transformer | Established | Standard architectures, sovereign GPU implementations. |

**2 crates | 43 WGSL shaders | 753 lib tests + 220 validation binaries | 3,900+ validation checks | 25 papers reproduced**

---

### wetSpring — Life Sciences

| Capability | Type | Description |
|-----------|------|-------------|
| Anderson-QS coupling | **Novel** | Anderson localization applied to quorum sensing: population heterogeneity as disorder, W_c ≈ 16.5 in 3D, geometry-dependent QS activation. New theoretical connection. |
| 3D Anderson dimensional phase diagram | **Novel** | 1D→2D→3D sweep with plateau points 0/5/12. J_c(3D) ≈ 1.28 vs J_c(2D) ≈ 0.56. 28-biome global atlas mapping geometry to QS regime. |
| QS-disorder prediction from diversity | **Novel** | Real ecosystem diversity profiles → Anderson regime prediction. Connects microbial ecology to condensed matter physics. |
| NPU reservoir deployment (biology) | **Novel** | ESN → int8 quantization → AKD1000: QS phase classifier, phylogenetic placement, genome binning, spectral triage, bloom sentinel. |
| Nanopore signal bridge | **Novel** | Sovereign POD5/NRS parsing without ONT SDK. Synthetic community reads, int8 quantization for NPU. |
| Pure GPU streaming pipeline | **Novel** | Multi-stage bio pipeline with zero CPU round-trips. 441–837× speedup vs round-trip architecture. |
| 16S rRNA pipeline (DADA2) | Established | FASTQ QC, denoising, chimera, taxonomy, UniFrac, diversity. Sovereign Rust replacing QIIME2. |
| Phylogenetics suite | Established | Felsenstein pruning, NJ, bootstrap, Robinson-Foulds, DTL reconciliation, HMM. |
| Population genomics | Established | ANI, SNP calling, dN/dS, molecular clock, pangenome. |
| LC-MS / PFAS screening | Established | mzML parsing, EIC, peak detection, KMD, spectral matching. Sovereign Rust replacing pyOpenMS. |
| Drug repurposing (NMF, TransE) | Established | Pathway scoring, knowledge graph embedding. Sovereign Rust. |
| Quorum sensing ODE systems | Established | Waters, bistable, cooperation, phage defense. Sovereign Gillespie SSA. |

**3 crates | 0 local shaders (79 ToadStool primitives consumed) | 1,073 Rust tests + 5,061 validation checks | 52 papers reproduced | 6 scientific tracks**

---

### airSpring — Ecological Validation

| Capability | Type | Description |
|-----------|------|-------------|
| Pure Rust FAO-56 pipeline | **Novel** | Complete Penman-Monteith ET₀, water balance, Kc adjustment in Rust without scipy. Only AGPL implementation of FAO-56. |
| GPU bridge for agricultural science | **Novel** | BatchedEt0, KrigingInterpolator, SeasonalReducer dispatched to GPU via BarraCuda. No other agricultural framework uses GPU compute. |
| Real-data cross-validation (918 station-days) | **Novel** | R²=0.967 against Open-Meteo across 918 station-days. 3 API sources (Open-Meteo, NOAA CDO, OpenWeatherMap). |
| SoilWatch 10 calibration | **Novel** | Topp equation + correction curves for commercial soil sensors. Pure Rust signal processing. |
| FAO-56 Penman-Monteith | Established | Allen et al. 1998. Standard method, sovereign implementation. |
| Ordinary kriging | Established | Spatial interpolation. Uses BarraCuda `KrigingF64`. |
| Soil moisture modeling | Established | Topp equation, dielectric permittivity → VWC. |

**1 crate | 0 local shaders | ~162 Rust tests + 119 validation checks | 65/65 Python↔Rust cross-validated | 3 papers reproduced**

---

### groundSpring — Reality Layer (Measurement & Uncertainty)

| Capability | Type | Description |
|-----------|------|-------------|
| Cross-domain noise framework | **Novel** | Unified bias-variance decomposition across agriculture, meteorology, microbiology, and seismology. Same uncertainty budget methodology applied to every spring's measurements. |
| Uncertainty budget for springs | **Novel** | Provides measurement error labels that neuralSpring uses for robust training. Every spring's "ground truth" passes through groundSpring's uncertainty quantification. |
| Literature extension roadmap | **Novel** | Connects Bazavov (lattice QCD), Waters (QS), Liu (phylogenetics), Kachkovskiy (spectral) published research to measurement uncertainty. |
| Monte Carlo error propagation | Established | Standard MC uncertainty. Sovereign Python implementation. |
| Seismic travel-time inversion | Established | 1D inversion with Nelder-Mead. Standard geophysics. |
| Multinomial rarefaction | Established | Standard microbial ecology. |

**0 Rust crates (Phase 0 Python) | 71 validation checks | 5 experiments | Cross-domain synthesis**

---

## VI. Summary: Novel Prior Art (New to Technology)

These capabilities exist NOWHERE else in open source:

### Infrastructure, Systems & Provenance (1–27)

| # | Innovation | Primal | Why It's Novel |
|---|-----------|--------|---------------|
| 1 | Userspace GPU driver in Rust via VFIO | coralReef + toadStool | Nobody has built a GPU compute driver in userspace Rust. DPDK did this for NICs. |
| 2 | WGSL → native GPU ISA compiler (AGPL) | coralReef | No AGPL shader compiler exists. The only ones are in Mesa (MIT) and NVIDIA (proprietary). |
| 3 | Self-teaching GPU hardware learning | toadStool hw-learn | Observe → distill → apply → share. GPUs teach each other initialization sequences. |
| 4 | Hardware-atheistic DF64 precision | barraCuda | FP64 precision on any GPU via f32×2, with per-shader precision routing. |
| 5 | 806 scientific WGSL shaders (AGPL) | barraCuda | Largest open scientific shader library under copyleft. |
| 6 | PrecisionBrain routing | toadStool | Domain-aware precision selection with NVVM transcendental risk detection. |
| 7 | Pure Rust Tor stack (AGPL) | songBird | Only AGPL-3.0 Tor implementation. Full directory/circuit/stream/onion. |
| 8 | Tower Atomic crypto delegation | bearDog + songBird | Network layer has zero crypto state. All delegated via IPC. |
| 9 | Genetic lineage identity | bearDog + biomeOS | Cryptographic device ancestry modeled on biology (mitochondrial + nuclear). |
| 10 | Dark Forest discovery | bearDog + songBird | Zero-metadata service advertisement. Discoverable but private. |
| 11 | Neural API (165+ translations) | biomeOS | Semantic capability composition. Primals compose by capability, not name. |
| 12 | Plasmodium orchestration | biomeOS | Slime-mold-inspired decentralized coordination without central authority. |
| 13 | Six-mode content slicing | rhizoCrypt | Copy/Loan/Consignment/Escrow/Waypoint/Transfer with distinct resolution routes. |
| 14 | Rhizo-Loam ephemeral→permanent layering | rhizoCrypt + loamSpine | Working memory crystallizes into permanent record via dehydration protocol. |
| 15 | Philosophy of forgetting | rhizoCrypt | Ephemeral by default, persistent by consent. Anti-"store everything forever." |
| 16 | Braid attribution with inheritance decay | sweetGrass | 12 roles, configurable weights, `0.5^depth` decay, `calculate_rewards()`. No blockchain. |
| 17 | 0/1/Many compression | sweetGrass | Session→Discard/Single/Multiple. Meta-Braids. DAG compression. |
| 18 | Inter-primal contribution API | sweetGrass | Any primal reports work via `ContributionRecord`. Domain metadata keys. |
| 19 | Loam certificates with auto-reversion | loamSpine | `LoanTerms`, `process_expired_loans()`, full provenance chain. No blockchain. |
| 20 | Waypoint spines | loamSpine | Local permanence for borrowed state without upward propagation. |
| 21 | Temporal Moments (4 anchor types) | loamSpine | Domain-agnostic time: Crypto, Atomic, Causal, Consensus anchors. |
| 22 | Infant Discovery (5-tier) | loamSpine | env → DNS-SRV → registry → mDNS → fallback. Capability-based. |
| 23 | GDPR data rights on scientific provenance | sweetGrass | Access/Rectification/Erasure/Portability/Objection applied to compute provenance. |
| 24 | Metadata-only threat detection | skunkBat | Privacy-preserving security: detects threats without reading content. |
| 25 | Isomorphic IPC (single binary) | nestGate | Platform-adaptive transport selection at runtime. |
| 26 | NPU driver in pure Rust (VFIO) | toadStool | Only Rust driver for BrainChip Akida neuromorphic processor. |
| 27 | Spring absorption methodology | toadStool | Write→Absorb→Lean: springs evolve capabilities, primals absorb proven patterns. |

### Platform & Representation (28–36)

| # | Innovation | Primal | Why It's Novel |
|---|-----------|--------|---------------|
| 28 | NUCLEUS chemical bonding model | biomeOS | Ionic/Covalent/Metallic/Weak bonds for distributed system composition. |
| 29 | ContinuousExecutor graph engine | biomeOS | Fixed-timestep graph loops with feedback edges. 60Hz game, 90Hz surgical. |
| 30 | UniBin five-mode rendering | petalTongue | Single binary → GUI, TUI, web, headless, status. One binary, five modalities. |
| 31 | Proprioception (SAME DAVE) | petalTongue | UI self-awareness. The interface knows what it is displaying. |
| 32 | Human entropy capture | petalTongue | Multi-modal entropy (audio, visual, narrative, gesture) fed to BearDog. |
| 33 | Graph sonification | petalTongue | Network topology rendered as audio. Accessibility + multi-modal representation. |
| 34 | DataService universal broadcast | petalTongue | One data model, event-driven broadcast to any renderer modality. |
| 35 | Cross-spring ecology graphs | biomeOS | Domain-specific spring composition in TOML. Springs feed each other. |
| 36 | genomeBin v3 deployment | biomeOS | Binary isomorphic format with genetic lineage tracking. |

### Scientific & Domain (37–52)

| # | Innovation | Spring | Why It's Novel |
|---|-----------|--------|---------------|
| 37 | NPU physics pipeline (9,017× less energy) | hotSpring | First neuromorphic silicon in lattice QCD / plasma physics pipeline. |
| 38 | Lattice QCD phase detection without FFT | hotSpring | NPU phase classification from position-space observables. Bypasses Fourier transform. |
| 39 | 10 NPU SDK assumptions overturned | hotSpring | Proved vendor wrong about their own hardware. Documented. |
| 40 | Heterogeneous real-time HMC monitor | hotSpring | Live phase detection at 0.09% overhead with predictive steering. |
| 41 | Backend-agnostic MD engine | hotSpring | `MdEngine<B: GpuBackend>` — same physics, any dispatch (wgpu, VFIO, DRM). |
| 42 | Nautilus Shell bridge | neuralSpring | Feed-forward evolutionary reservoir replacing recurrent ESN. |
| 43 | baseCamp biophysical AI (5 sub-theses) | neuralSpring | Weight matrices as Hamiltonians, loss as energy, NN as PGM, multi-agent as QS. 128/128 validated. |
| 44 | Cross-spring spectral rewire | neuralSpring | hotSpring diagnostics absorbed into neural weight analysis. 41/41 validated. |
| 45 | WDM ESN regime classifier | neuralSpring | GPU echo state network for warm dense matter. 96.5% accuracy. |
| 46 | Anderson-QS coupling | wetSpring | Anderson localization applied to quorum sensing. W_c ≈ 16.5, geometry-dependent. New theoretical connection. |
| 47 | 3D Anderson dimensional phase diagram | wetSpring | 1D→2D→3D sweep. 28-biome global atlas mapping geometry to QS regime. |
| 48 | Pure GPU streaming bio pipeline | wetSpring | Zero CPU round-trips. 441–837× speedup. |
| 49 | Nanopore signal bridge (no ONT SDK) | wetSpring | Sovereign POD5/NRS parsing. Only open-source nanopore reader in Rust. |
| 50 | GPU agricultural science | airSpring | BatchedEt0, KrigingInterpolator, SeasonalReducer on GPU. No other ag framework uses GPU. |
| 51 | Cross-domain noise framework | groundSpring | Unified bias-variance across agriculture, meteorology, microbiology, seismology. |
| 52 | Constrained evolution methodology | ecosystem | AI as mutation operator, Rust as natural selection, physics as fitness. 69K invocations, 51B tokens. |

---

## VII. Established Prior Art (Sovereign Implementations)

These techniques exist elsewhere but are locked into AGPL-3.0 sovereign
implementations that cannot be captured:

### Systems & Infrastructure

| Domain | What's Locked | Why It Matters |
|--------|--------------|----------------|
| Cryptography | Ed25519, X25519, AES-GCM, ChaCha20, BLAKE3, HKDF, Argon2 | Pure Rust crypto outside ring/rustls permissive ecosystem |
| TLS 1.3 | Full handshake + record layer | Sovereign TLS not dependent on OpenSSL or ring |
| GPU compute | DRM, UVM dispatch in pure Rust | Sovereign wrappers around Linux GPU subsystems |
| Storage | Content-addressed DAG, Merkle trees | Sovereign storage not dependent on IPFS |
| Networking | QUIC, STUN, IGD, mDNS | Sovereign networking stack |
| AI coordination | MCP, inference routing | Sovereign AI orchestration |

### Scientific Reproductions (~100+ papers across all springs)

| Domain | Papers | Algorithms Locked | Source |
|--------|--------|-------------------|--------|
| Dense plasma physics | 5+ | Yukawa OCP MD, Green-Kubo transport, TTM | hotSpring |
| Nuclear physics | 2+ | SEMF, HFB, AME2020 EOS | hotSpring |
| Lattice gauge theory | 6+ | SU(3) Wilson, HMC, gradient flow, staggered Dirac, CG | hotSpring |
| Spectral theory | 9+ | Anderson 1D/2D/3D, Hofstadter, Aubry-Andre, Lanczos | hotSpring + neuralSpring |
| Abelian Higgs | 1 | U(1)+Higgs HMC | hotSpring |
| Dielectric response | 3+ | BGK, Mermin, kinetic-fluid | hotSpring |
| Neural architectures | 10+ | ESN, HMM, PINN, DeepONet, LeNet-5, MLP, LSTM, Transformer | neuralSpring |
| Evolutionary computation | 5 | Counterdiabatic, MODES, lexicase, swarm | neuralSpring |
| Phylogenetics | 5+ | HMM, SATe, NJ, Felsenstein, DTL, bootstrap | neuralSpring + wetSpring |
| Game theory / QS | 3+ | Replicator dynamics, Hill regulatory, cooperation | neuralSpring + wetSpring |
| Population genetics | 2+ | FST, Mantel, pangenome, molecular clock | neuralSpring + wetSpring |
| Protein structure | 1 | AlphaFold2/3 (Evoformer, Pairformer, diffusion) | neuralSpring (helixVision) |
| 16S rRNA microbial ecology | 10 | DADA2, chimera, taxonomy, UniFrac, diversity | wetSpring |
| Deep-sea metagenomics | 6 | ANI, SNP, dN/dS, pangenomics, rare biosphere | wetSpring |
| Analytical chemistry / PFAS | 4 | mzML, EIC, peak detection, KMD, spectral matching | wetSpring |
| Drug repurposing | 5 | NMF, TransE, pathway scoring | wetSpring |
| Quorum sensing | 6+ | Waters ODE, bistable, cooperation, phage defense, Gillespie | wetSpring |
| Soil Anderson / tillage | 9 | Anderson localization in soil pore geometry | wetSpring |
| Evapotranspiration | 3 | FAO-56 PM, Hargreaves, water balance | airSpring |
| Soil science | 2 | Topp equation, SoilWatch calibration | airSpring |
| Measurement uncertainty | 5 | Monte Carlo propagation, bias-variance, seismic inversion | groundSpring |

### Validation Infrastructure (sovereign)

| Metric | Value |
|--------|-------|
| Total Rust tests (all springs) | ~3,000+ |
| Total validation checks | ~10,000+ |
| Python baselines cross-validated | ~500+ |
| Papers reproduced | ~100+ |
| Scientific domains | 10+ |
| Named tolerance constants | ~240 (hotSpring ~150 + wetSpring ~92) |
| Three-tier methodology | Python baseline → Rust CPU → Rust GPU |

---

## VIII. The Lock

Every line of code in this catalog is:

1. **Published** on GitHub under AGPL-3.0
2. **Timestamped** via git commit history
3. **Reproducible** via Cargo.lock → deterministic binary
4. **Permanent** — copyright lasts life + 70 years; AGPL is irrevocable on published versions

A corporation can:
- ✅ Use this code (AGPL allows all use)
- ✅ Modify this code (AGPL allows modification)
- ❌ Close modifications (AGPL requires sharing back)
- ❌ Offer as proprietary service (AGPL network service clause)
- ❌ Claim independent invention (timestamped prior art)
- ❌ Patent covered techniques (prior art defense)

A human can:
- ✅ Use, study, modify, share — forever
- ✅ Build on it, improve it, extend it
- ✅ Receive attribution (sweetGrass)
- ✅ Benefit from the commons as the commons benefits from them

**Sovereign science. Sovereign code. Free for humans. Fiction-proof.**

---

*Catalog compiled March 13, 2026. Updated as primals evolve.*
*The prior art grows with every commit. The commons only expands.*
