+++
title = "Acknowledgments — The Systems We Stand On"
description = "The open-source tools, languages, and communities that make ecoPrimals possible. We carry their banners."
weight = 90

[taxonomies]
primals = ["beardog", "barracuda", "biomeos", "coralreef", "nestgate", "petaltongue", "songbird", "toadstool"]
+++

## Why This Page Exists

ecoPrimals is built entirely on open-source foundations. Every binary we ship, every test we run, every page we serve — all of it stands on work that other people gave to the commons. The [scyBorg triple license](@/methodology/SCYBORG_LICENSING.md) (AGPL-3.0 / ORC / CC-BY-SA-4.0) is our thank-you: what we built on open foundations returns to open foundations.

This page names the systems we depend on and carries their banners.

---

## The Language

### Rust

[rust-lang.org](https://www.rust-lang.org/) · [github.com/rust-lang/rust](https://github.com/rust-lang/rust)

The entire ecoPrimals ecosystem — {{ total_stat(stat="total_loc_display") }} lines across {{ total_stat(stat="total_primals") }} primals — is written in Rust. The language is the environmental constraint that makes everything else possible. The borrow checker is the compiler-enforced selective pressure. The type system is the fitness landscape. The zero-cost abstractions are why `#![forbid(unsafe_code)]` doesn't mean slow.

Rust is not a tool we chose. It is the constraint that shaped us. Everything ecoPrimals has discovered — vendor-agnostic GPU compute, sovereign infrastructure, the constrained evolution methodology itself — was revealed by Rust's constraints.

**License**: MIT / Apache-2.0

---

## The Infrastructure

### Zola

[getzola.org](https://www.getzola.org/) · [github.com/getzola/zola](https://github.com/getzola/zola)

The static site generator that builds primals.eco. Single Rust binary, zero runtime dependencies, TOML front matter, Tera templates, built-in search, taxonomy system, minification. {{ total_stat(stat="content_pages") }} pages built in under 20 seconds. Zola is what sporePrint uses for sovereign validation — petalTongue handles live serving, but Zola remains the oracle.

**License**: MIT

### Caddy

[caddyserver.com](https://caddyserver.com/) · [github.com/caddyserver/caddy](https://github.com/caddyserver/caddy)

The web server on golgiBody-ext that serves primals.eco. Automatic HTTPS, HTTP/3, ACME auto-renewal, security headers — all with a 20-line Caddyfile. Caddy is the outer membrane.

**License**: Apache-2.0

### WireGuard

[wireguard.com](https://www.wireguard.com/)

The encrypted overlay network connecting all gates. The periplasmic mesh — golgi, sporeGate, eastGate, flockGate — runs over WireGuard tunnels. Simple, fast, formally verified cryptography.

**License**: GPL-2.0

### RustDesk

[rustdesk.com](https://rustdesk.com/) · [github.com/rustdesk/rustdesk](https://github.com/rustdesk/rustdesk)

Self-hosted remote desktop. Pure Rust. Every gate in the mesh is reachable via sovereign RustDesk relay — no TeamViewer, no AnyDesk, no cloud dependency. The sovereign remote access layer.

**License**: AGPL-3.0

### Forgejo

[forgejo.org](https://forgejo.org/) · [codeberg.org/forgejo/forgejo](https://codeberg.org/forgejo/forgejo)

The sovereign git forge running on golgi at `git.primals.eco`. Source of truth for all repositories. GitHub is the trailing mirror. Forgejo is the periplasm — the place where code lives before it becomes public.

**License**: MIT

### Zellij

[zellij.dev](https://zellij.dev/) · [github.com/zellij-org/zellij](https://github.com/zellij-org/zellij)

Terminal multiplexer. Pure Rust. Every development session runs in Zellij. The layout system and session persistence make multi-gate operations manageable.

**License**: MIT

---

## The Crates

Every dependency in spore-validate and across the ecosystem was chosen for the same reason: pure Rust, no C toolchain, no vendor lock-in.

### Cryptography and Identity

| Crate | What it does for us | License |
|-------|-------------------|---------|
| [ed25519-dalek](https://crates.io/crates/ed25519-dalek) | BearDog's identity keys — every primal signs with Ed25519 | BSD-3-Clause |
| [x25519-dalek](https://crates.io/crates/x25519-dalek) | Key exchange for BTSP transport | BSD-3-Clause |
| [chacha20poly1305](https://crates.io/crates/chacha20poly1305) | AEAD encryption — every inter-primal message | MIT / Apache-2.0 |
| [blake3](https://crates.io/crates/blake3) | Content-addressed hashing — provenance, CAS, guideStone Merkle roots | CC0-1.0 / Apache-2.0 |
| [argon2](https://crates.io/crates/argon2) | Password hashing — projectNUCLEUS user auth | MIT / Apache-2.0 |

### Serialization and Data

| Crate | What it does for us | License |
|-------|-------------------|---------|
| [serde](https://crates.io/crates/serde) | The serialization framework. Every config, every message, every manifest | MIT / Apache-2.0 |
| [serde_json](https://crates.io/crates/serde_json) | JSON-RPC 2.0 — the lingua franca of inter-primal communication | MIT / Apache-2.0 |
| [toml](https://crates.io/crates/toml) | Config files, manifests, front matter — TOML 1.1 compliant | MIT / Apache-2.0 |

### CLI and Error Handling

| Crate | What it does for us | License |
|-------|-------------------|---------|
| [clap](https://crates.io/crates/clap) | Command-line argument parsing — derive macros, env fallback | MIT / Apache-2.0 |
| [thiserror](https://crates.io/crates/thiserror) | Typed error hierarchies — every module has domain-specific errors | MIT / Apache-2.0 |

### GPU Compute

| Crate | What it does for us | License |
|-------|-------------------|---------|
| [wgpu](https://crates.io/crates/wgpu) | WebGPU implementation — the layer that makes vendor-agnostic GPU compute possible | MIT / Apache-2.0 |
| [naga](https://crates.io/crates/naga) | WGSL shader compiler — coralReef wraps naga for cross-spring shader compilation | MIT / Apache-2.0 |

### Compression and I/O

| Crate | What it does for us | License |
|-------|-------------------|---------|
| [flate2](https://crates.io/crates/flate2) | Gzip — CAS push, depot archives. Pure Rust backend (miniz_oxide) | MIT / Apache-2.0 |
| [walkdir](https://crates.io/crates/walkdir) | Recursive directory traversal — content discovery, validation walks | Unlicense / MIT |

---

## The Science

The springs reproduce published, peer-reviewed science. The researchers whose work we reproduce are acknowledged on each spring's page and in the [Spring Catalog](@/architecture/SPRING_CATALOG.md). They are not collaborators or endorsers — they are scientists whose published results define our acceptance criteria.

---

## The AI

Every line of code in ecoPrimals was produced through human-AI collaboration — the [K-NOME methodology](@/philosophy/the_knowledge_numeric.md). The AI models that contributed to this work were trained on the compressed knowledge of every human who ever wrote anything that ended up in training data. [The Love Letter](@/philosophy/the_love_letter.md) addresses this debt directly.

The scyBorg triple license is our structural acknowledgment: the work returns to the commons because it came from the commons.

---

## The scyBorg Thank-You

The [scyBorg triple license](@/methodology/SCYBORG_LICENSING.md) exists because a single license cannot cover code, mechanics, and content. But all three licenses share one property: **they give back**.

- **AGPL-3.0-or-later** — code that runs on a server must share its source. If you use ecoPrimals code, your users get the same freedom.
- **ORC** — system designs are open. Build on them. Extend them. The mechanics belong to everyone.
- **CC-BY-SA-4.0** — documentation shares forward. Attribute and share alike.

Every system listed on this page gave something to the commons. ecoPrimals gives back under terms that ensure the commons grows. That is the acknowledgment.
