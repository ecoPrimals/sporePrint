+++
title = "Deployment Model: plasmidBin & BYOB"
weight = 42
description = "How primal binaries are distributed, versioned, and composed — the plasmidBin binary distribution surface and BYOB composition model."
date = 2026-03-31

[taxonomies]
primals = ["beardog", "barracuda", "biomeos", "coralreef", "loamspine", "nestgate", "petaltongue", "rhizocrypt", "songbird", "squirrel", "sweetgrass", "toadstool"]
springs = ["groundspring", "healthspring", "ludospring", "neuralspring", "primalspring", "wetspring"]

[extra]
foundation = true
domain = "Architecture"
maturity = "implemented"
+++

**Repository**: [github.com/ecoPrimals/plasmidBin](https://github.com/ecoPrimals/plasmidBin) — **Public**

---

## The Problem

Primals are self-contained Rust binaries. Springs validate them. Products
compose them. But how do binaries get from the primal source tree to the
user's machine without requiring everyone to compile from source?

## The Solution: plasmidBin

**{{ entity(name="plasmidbin") }}** is the ecosystem's binary distribution surface. It is
analogous to `node_modules` for primals — a local deployment cache where
pre-built binaries are resolved, verified, and composed.

### How It Works

```
Primal Source → cargo build → harvest.sh → plasmidBin (GitHub) → fetch.sh → Local plasmidBin/ → biomeOS deploy
```

1. **Build**: A primal is compiled from source (musl-static PIE for
   x86_64, aarch64 planned).
2. **Harvest**: `harvest.sh` validates the binary (static ELF, stripped),
   computes blake3 checksums, copies into `primals/`/`springs/`, and
   updates `checksums.toml`. Optionally pushes a GitHub Release.
3. **Fetch**: Consumers run `fetch.sh` to download from the latest
   GitHub Release and verify blake3 checksums (via `b3sum`).
4. **Deploy**: {{ entity(name="biomeos") }} reads deploy graphs (TOML DAGs) and germinates
   primals from the local `plasmidBin/` directory.

### Zero Source Coupling

Products and springs never compile primal source. They consume **pre-built
binaries only**. This is the {{ entity(name="byob") }} (Bring Your Own Biome) model — a product
declares which primals it needs via a deploy graph, fetches them from
{{ entity(name="plasmidbin") }}, and {{ entity(name="biomeos") }} handles the rest.

### Source Availability

All binaries distributed through {{ entity(name="plasmidbin") }} are under **AGPL-3.0-or-later**.
Per AGPL, corresponding source must be obtainable when binaries are
distributed:

- **Public primals** (songBird, nestGate, toadStool, squirrel, biomeOS, {{ entity(name="coralreef") }}, {{ entity(name="barracuda") }}, petalTongue, sourDough, bingoCube, rhizoCrypt, sweetGrass, loamSpine, skunkBat): source
  is on GitHub at [github.com/ecoPrimals](https://github.com/ecoPrimals).
- **bearDog**: source available on request. Crypto root of trust — goes public
  after comprehensive pen-test validation. Each `metadata.toml` includes a
  `[provenance] built_from` field identifying the source tree.

---

## metadata.toml Format

Every primal in {{ entity(name="plasmidbin") }} has a `metadata.toml` describing its identity,
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
checksum_blake3 = "..."
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
| `[builds.<arch>]` | Per-architecture: binary name, target triple, blake3 checksum, static/PIE flags |
| `[genomeBin]` | Deployment hints: tier, modes, ports, env vars, service ordering |

---

## BYOB Composition

Products (gen4, {{ entity(name="sporegarden") }}) declare their primal dependencies in
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

{{ entity(name="plasmidbin") }} includes standard composition presets via `ports.env`:

| Preset | Primals | Use Case |
|--------|---------|----------|
| **Tower** | {{ entity(name="beardog") }} + {{ entity(name="songbird") }} | Crypto + networking foundation |
| **Compute** | Tower + {{ entity(name="toadstool") }} + {{ entity(name="barracuda") }} | GPU compute pipeline |
| **Node** | Tower + {{ entity(name="toadstool") }} | Hardware dispatch |
| **Nest** | Tower + {{ entity(name="nestgate") }} | Persistent storage |
| **Full {{ entity(name="nucleus") }}** | All 8 foundation primals | Complete ecosystem |
| **Storytelling** | {{ entity(name="nucleus") }} + {{ entity(name="petaltongue") }} + {{ entity(name="squirrel") }} | Interactive AI experience |

### Infrastructure Compositions (Wave 134c)

Beyond product-facing presets, the ecosystem defines **infrastructure composition
profiles** in `ecosystem_manifest.toml [compositions]`. These are fractal deployment
patterns — replicable shapes that can be instantiated on any hardware, from a $5 VPS
to a GPU-equipped HPC node:

| Profile | Primals | Purpose |
|---------|---------|---------|
| **full** | All 13+ | Complete sovereign {{ entity(name="nucleus") }} — build-capable gate |
| **thin-relay** | {{ entity(name="songbird") }}, {{ entity(name="nestgate") }}, membrane | Sovereign relay depot. No Rust toolchain. Receives ecobins via mesh auto-fetch. |
| **tower** | {{ entity(name="beardog") }}, {{ entity(name="songbird") }}, skunkBat | Minimal secure mesh entry |
| **compute** | {{ entity(name="toadstool") }}, {{ entity(name="barracuda") }}, {{ entity(name="coralreef") }}, {{ entity(name="biomeos") }} | HPC/GPU workloads |
| **nest** | {{ entity(name="nestgate") }}, sweetGrass, rhizoCrypt | Cold storage and CAS |

The **thin-relay** pattern is especially significant: it enables sovereign presence
anywhere without a Rust toolchain. A thin relay receives pre-built ecobins from the
mesh and serves them via Caddy TLS. Use cases include VPS relay nodes, HPC site depots,
university sporePrint mirrors, and field data collectors.

Query profiles: `membrane plasmid.composition --profile thin-relay`

---

## Offline Capability

After the initial `fetch.sh`, {{ entity(name="plasmidbin") }} is fully offline-capable. Binaries
are local, deploy graphs are local, and {{ entity(name="biomeos") }} germinates from the local
cache. No cloud, no API keys, no network required at runtime.

---

## Current Inventory

{{ entity(name="plasmidbin") }} currently tracks **18 entries** (12 primals + 6 springs with
deployment metadata):

| Type | Entries |
|------|---------|
| Foundation primals (8) | beardog, songbird, nestgate, toadstool, squirrel, biomeos, coralreef, barracuda |
| Post-{{ entity(name="nucleus") }} primals (4) | petaltongue, rhizocrypt, loamspine, sweetgrass |
| Springs with deployment metadata (6) | ludospring, wetspring, groundspring, healthspring, neuralspring, primalspring |

Springs in {{ entity(name="plasmidbin") }} have deployment metadata (ports, capabilities) for
{{ entity(name="nucleus") }} integration — they are not just validation workspaces but also
deployable science services that register with {{ entity(name="biomeos") }}.

---

*{{ entity(name="plasmidbin") }} is the bridge between "we built it" and "you can run it."
Clone the repo, run fetch.sh, and you have a sovereign computing stack.*
