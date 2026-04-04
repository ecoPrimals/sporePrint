+++
title = "Deployment Model: plasmidBin & BYOB"
description = "How primal binaries are distributed, versioned, and composed — the plasmidBin binary distribution surface and BYOB composition model."
date = 2026-03-31

[taxonomies]
primals = ["beardog", "barracuda", "biomeos", "coralreef", "loamspine", "nestgate", "petaltongue", "rhizocrypt", "songbird", "squirrel", "sweetgrass", "toadstool"]
springs = ["groundspring", "healthspring", "ludospring", "neuralspring", "primalspring", "wetspring"]
+++

**Repository**: [github.com/ecoPrimals/plasmidBin](https://github.com/ecoPrimals/plasmidBin) — **Public**

---

## The Problem

Primals are self-contained Rust binaries. Springs validate them. Products
compose them. But how do binaries get from the primal source tree to the
user's machine without requiring everyone to compile from source?

## The Solution: plasmidBin

**plasmidBin** is the ecosystem's binary distribution surface. It is
analogous to `node_modules` for primals — a local deployment cache where
pre-built binaries are resolved, verified, and composed.

### How It Works

```
Primal Source → cargo build → harvest.sh → plasmidBin (GitHub) → fetch.sh → Local plasmidBin/ → biomeOS deploy
```

1. **Build**: A primal is compiled from source (musl-static PIE for
   x86_64, aarch64 planned).
2. **Harvest**: `harvest.sh` checksums the binary, updates `metadata.toml`,
   creates a `manifest.lock`, and pushes a GitHub Release.
3. **Fetch**: Consumers run `fetch.sh` (or a product-specific
   `fetch_primals.sh`) to download from the latest release and verify
   SHA-256 checksums.
4. **Deploy**: biomeOS reads deploy graphs (TOML DAGs) and germinates
   primals from the local `plasmidBin/` directory.

### Zero Source Coupling

Products and springs never compile primal source. They consume **pre-built
binaries only**. This is the BYOB (Bring Your Own Biome) model — a product
declares which primals it needs via a deploy graph, fetches them from
plasmidBin, and biomeOS handles the rest.

### Source Availability

All binaries distributed through plasmidBin are under **AGPL-3.0-or-later**.
Per AGPL, corresponding source must be obtainable when binaries are
distributed:

- **Public primals** (toadStool, squirrel, coralReef, barraCuda): source
  is on GitHub at [github.com/ecoPrimals](https://github.com/ecoPrimals).
- **Private primals**: source is available on request. Each
  `metadata.toml` includes a `[provenance] built_from` field that
  identifies the source tree. Repositories are progressively published as
  primals mature.

---

## metadata.toml Format

Every primal in plasmidBin has a `metadata.toml` describing its identity,
provenance, capabilities, and build artifacts:

```toml
[primal]
name = "toadstool"
version = "0.6.0"
domain = "compute"
description = "Universal compute orchestration"
license = "AGPL-3.0-or-later"

[provenance]
built_from = "primals/toadStool"
built_at = "2026-03-15T00:00:00Z"

[compatibility]
min_ipc_version = "3.0"
capabilities = ["compute.dispatch", "compute.discover", "ember.route"]

[builds.x86_64]
binary = "toadstool-x86_64"
target = "x86_64-unknown-linux-musl"
checksum_sha256 = "..."
pie_verified = true
static_linked = true

[genomeBin]
tier = "foundation"
unibin_modes = ["server", "cli", "benchmark"]
default_mode = "server"

[genomeBin.server]
default_port = 9100
env_prefix = "TOADSTOOL"

[genomeBin.service]
restart = "always"
after = ["beardog", "songbird"]
```

### What It Captures

| Section | Purpose |
|---------|---------|
| `[primal]` | Identity: name, version, domain, description, license |
| `[provenance]` | Where it came from: source tree, build timestamp, git ref |
| `[compatibility]` | IPC version, capability strings for discovery |
| `[builds.<arch>]` | Per-architecture: binary name, target triple, SHA-256, static/PIE flags |
| `[genomeBin]` | Deployment hints: tier, modes, ports, env vars, service ordering |

---

## BYOB Composition

Products (gen4, sporeGarden) declare their primal dependencies in
**deploy graphs** — TOML DAGs that describe which primals to germinate
and how they wire together:

```toml
# esotericWebb deploy graph (simplified)
[[node]]
primal = "beardog"
required = true

[[node]]
primal = "songbird"
required = true
depends_on = ["beardog"]

[[node]]
primal = "squirrel"
required = false
depends_on = ["beardog", "songbird"]

[[node]]
primal = "petaltongue"
required = false
depends_on = ["squirrel"]
```

Products use **PrimalBridge** (JSON-RPC over discovered sockets) to
communicate with germinated primals. Graceful degradation is built in:
if an optional primal is unavailable, the product continues with reduced
capability.

### Composition Presets

plasmidBin includes standard composition presets via `ports.env`:

| Preset | Primals | Use Case |
|--------|---------|----------|
| **Tower** | BearDog + Songbird | Crypto + networking foundation |
| **Compute** | Tower + ToadStool + barraCuda | GPU compute pipeline |
| **Node** | Tower + ToadStool | Hardware dispatch |
| **Nest** | Tower + NestGate | Persistent storage |
| **Full NUCLEUS** | All 8 foundation primals | Complete ecosystem |
| **Storytelling** | NUCLEUS + petalTongue + Squirrel | Interactive AI experience |

---

## Offline Capability

After the initial `fetch.sh`, plasmidBin is fully offline-capable. Binaries
are local, deploy graphs are local, and biomeOS germinates from the local
cache. No cloud, no API keys, no network required at runtime.

---

## Current Inventory

plasmidBin currently tracks **18 entries** (12 primals + 6 springs with
deployment metadata):

| Type | Entries |
|------|---------|
| Foundation primals (8) | beardog, songbird, nestgate, toadstool, squirrel, biomeos, coralreef, barracuda |
| Post-NUCLEUS primals (4) | petaltongue, rhizocrypt, loamspine, sweetgrass |
| Springs with deployment metadata (6) | ludospring, wetspring, groundspring, healthspring, neuralspring, primalspring |

Springs in plasmidBin have deployment metadata (ports, capabilities) for
NUCLEUS integration — they are not just validation workspaces but also
deployable science services that register with biomeOS.

---

*plasmidBin is the bridge between "we built it" and "you can run it."
Clone the repo, run fetch.sh, and you have a sovereign computing stack.*
