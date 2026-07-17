+++
title = "Cross-Platform Parity — OS Atheism to Silicon Atheism"
description = "Six-phase roadmap from OS-specific code to universal substrate deployment. Phases 1-2 complete: platform types shipped, transport abstraction shipped for all 14 primals. 59 depot binaries across 4 architectures."
date = 2026-07-16
weight = 57

[extra]
domain = "Architecture"
maturity = "implemented"
voice = "ecoPrimals"

[[extra.companions]]
url = "/architecture/silicon-deism/"
title = "Silicon Deism"
relation = "extends"
label = "Silicon Deism is the philosophical thesis; cross-platform parity is the engineering path"

[[extra.companions]]
url = "/architecture/deployment-model/"
title = "Deployment Model"
relation = "pairs_with"
label = "Depot serves platform-specific binaries that this work produces"

[[extra.companions]]
url = "/architecture/nucleus-architecture/"
title = "NUCLEUS Architecture"
relation = "extends"
label = "Phase 6 elevates NUCLEUS to substrate-independent composition"

[[extra.companions]]
url = "/architecture/content-addressed-convergence/"
title = "Content-Addressed Convergence"
relation = "pairs_with"
label = "Multi-platform builds converge when content hashes match"
+++

{{ maturity(level="implemented") }} Phases 1-2 complete. All 14 primals have platform-agnostic transport. 59 depot binaries across 4 architectures.

---

## The Problem

{{ total_stat(stat="total_primals") }} primals run on Linux. Most assume Unix
domain sockets, Unix signals, and POSIX filesystem semantics. This works on
the build server. It does not work on Windows, macOS, iOS, WASM, or bare-metal
embedded targets.

The ecosystem claims **Silicon Deism** — that hardware is a self-revealing
substrate, not a platform to depend on. But if the code assumes Linux, that
claim is aspirational, not operational. OS Atheism precedes Silicon Atheism:
you cannot be agnostic about silicon if you are married to an operating system.

---

## Six Phases

### Phase 1: Platform Types (Shipped)

A type system that makes platform differences visible at compile time:

```rust
pub enum TargetOs { Linux, Windows, MacOs, Android, Ios, Wasm, FreeBsd }
pub enum CpuArch { X86_64, Aarch64, Riscv64, Wasm32 }
pub enum LinkModel { MuslStatic, Gnu, Msvc, Wasm }

pub struct Platform {
    pub os: TargetOs,
    pub arch: CpuArch,
    pub link: LinkModel,
}
```

Platform detection is compile-time via `cfg` attributes. No runtime overhead.
Depot layout uses the platform triple as the directory key.

### Phase 2: Transport + Signals (Complete — Wave 145a)

All 14 primals shipped platform-agnostic transport abstractions. The raw
`tokio::net::UnixStream` calls that locked the ecosystem to Linux have been
replaced with trait + backend patterns across every crate.

| Pattern | What it replaced | Primals |
|---------|-----------------|---------|
| `TransportEndpoint` dispatch | Raw UDS socket paths | songBird, skunkBat, bearDog, squirrel |
| `TransportStream` + `TransportListener` | `tokio::net::UnixStream/Listener` | nestGate, biomeOS, barraCuda, coralReef |
| `PlatformLifecycle` | `tokio::signal::unix` | petalTongue |
| `NestGateClient` + `transport_connect` | Hardcoded UDS connect | sweetGrass, loamSpine, rhizoCrypt |
| `getrandom` CSPRNG | `/dev/urandom` reads | cellMembrane |

Reference implementation: {{ entity(name="songbird") }} — `NamedPipeServer`/
`NamedPipeClient` behind `#[cfg(windows)]`, `IpcStream` batch across 9 crates.

**Result**: Windows depot went from 1 binary to 14. All 14 primals cross-compile
for all 4 target architectures.

### Phase 3: Shell-out + Filesystem

Three primals use platform-specific filesystem APIs:

| Dependency | What it does | Abstraction |
|-----------|-------------|-------------|
| `rustix::fs` | Low-level filesystem ops | Cross-platform FS trait |
| `PermissionsExt` | Unix permission bits | Permission abstraction |
| `openssl` (build-time) | TLS certificate ops | Already migrating to `rustls` |

### Phase 4: Gate Bootstrap

The 13-phase NUCLEUS bootstrap pipeline assumes Linux systemd for service
management. Phase 4 introduces platform branching: systemd on Linux,
Windows Services on Windows, launchd on macOS.

### Phase 5: Isomorphic Depot

Platform-aware fetch → install → launch cycle. The depot already serves
multi-architecture binaries; Phase 5 makes the client automatically select
the correct platform binary and install it appropriately for the local OS.

### Phase 6: NUCLEUS Composition

The final phase: a NUCLEUS deploy graph is substrate-independent. The same
`deploy.toml` describes the composition; the platform types determine how
each primal is started, how IPC is routed, and how the lifecycle is managed.

---

## Current Depot State

The depot serves 59 signed binaries across 4 architectures:

| Architecture | Binaries | Status |
|-------------|----------|--------|
| `x86_64-unknown-linux-musl` | 16 | Fresh |
| `aarch64-unknown-linux-musl` | 16 | Fresh |
| `aarch64-linux-android` | 13 | Fresh |
| `x86_64-pc-windows-gnu` | 14 | Fresh — **unblocked from 1 to 14 by Phase 2** |

All binaries are BLAKE3 checksummed and Ed25519 signed. The VPS depot
serves them over HTTPS. Phase 2 transport completion is what moved
Windows from 1 binary to 14.

---

## Failure Categories (Resolved)

The cross-platform parity audit identified 5 failure categories. Phase 2
resolved the first two, which accounted for 14 of 14 primals:

| Category | Primals | Status |
|----------|---------|--------|
| UDS transport (`tokio::net::UnixStream`) | 11 | **Resolved** — Phase 2 |
| Unix signals (`tokio::signal::unix`) | 3 | **Resolved** — Phase 2 |
| Platform FS (`rustix::fs`, `PermissionsExt`) | 3 | Phase 3 (planned) |
| Hardware/kernel (VFIO, mmap) | 1 ({{ entity(name="toadstool") }}) | Feature-gate `linux-hw` |
| Android NDK (`android-activity`) | 1 ({{ entity(name="petaltongue") }}) | cdylib target |

Each primal adopted trait + backend patterns rather than `#[cfg]` exclusion
fences. The compile-time dispatch means zero runtime overhead.

---

## Glacial Goal: Universal Substrate

```
Phase 1: Platform Types      → COMPLETE (Wave 142a)
Phase 2: Transport + Signals  → COMPLETE (Wave 145a) — 14/14 primals
Phase 3: Shell-out + FS       → unlocks macOS, FreeBSD
Phase 4: Gate Bootstrap        → isomorphic service management
Phase 5: Isomorphic Depot      → auto-deploy on any platform
Phase 6: NUCLEUS Composition   → substrate-independent deploy graphs
```

The glacial goal is Universal Substrate Evolution: NUCLEUS deploys on any
architecture. Any substrate, any gate, same sovereign infrastructure. The
same binary runs on a basement server, a VPS, a phone, a Raspberry Pi,
and eventually a sovereign pallet in a cave entrance.

---

*OS Atheism is preceded by Silicon Atheism in the philosophical argument,
but precedes it in the engineering path. You earn the right to ignore the
silicon by first proving you can ignore the operating system. Phase 2 is
the highest leverage work remaining.*
