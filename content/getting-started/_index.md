+++
title = "Get Started"
description = "Deploy NUCLEUS on your own hardware — commodity Linux, Windows, or Android. Join the mesh with your own gate."
sort_by = "weight"
template = "section.html"
+++

ecoPrimals runs on commodity hardware you already own. A gate is any machine with
a chip and a drive — a desktop, a server, a laptop, a Steam Deck.

## The Stack

NUCLEUS is three atomic compositions orchestrated by {{ entity(name="biomeos") }}:

| Composition | What It Does | Primals |
|-------------|-------------|---------|
| **[Tower Atomic](@/architecture/tower_atomic.md)** | Encrypted mesh networking | {{ entity(name="beardog") }} + {{ entity(name="songbird") }} + {{ entity(name="skunkbat") }} |
| **[Nest Atomic](@/architecture/NUCLEUS_ARCHITECTURE.md)** | Content-addressed storage with provenance | {{ entity(name="nestgate") }} + {{ entity(name="loamspine") }} + {{ entity(name="sweetgrass") }} + {{ entity(name="rhizocrypt") }} |
| **Node Atomic** | GPU compute + visualization | {{ entity(name="toadstool") }} + {{ entity(name="barracuda") }} + {{ entity(name="coralreef") }} + {{ entity(name="petaltongue") }} + {{ entity(name="squirrel") }} |

One binary per primal. No containers. No VMs. All statically linked (musl on Linux, PE on Windows).

## What You Need

| Platform | Requirements | Transport | Status |
|----------|-------------|-----------|--------|
| **Linux** (x86_64) | Any distro with systemd. 30 MB RAM for full Tower | UDS (Unix sockets) | **PROVEN** |
| **Windows** (x86_64) | Windows 10/11. No admin rights for user-space deploy | TCP | **PROVEN** |
| **Android** (aarch64) | Via ADB. Tested on Pixel 8a (GrapheneOS) | TCP | **PROVEN** |
| **SteamOS** | Steam Deck. User-space deploy (`~/.local/bin/`) | UDS | **NEXT** |
| **GPU** (optional) | Any Vulkan-capable GPU for {{ entity(name="barracuda") }} scientific compute | — | NVIDIA + AMD tested |

## Quick Start: plasmidBin

The fastest path — pre-built binaries, BLAKE3 verified, running in 5 minutes:

```bash
# Download the binary pack for your platform
# Available at: depot.primals.eco or via mesh pull

# Linux (musl — zero runtime dependencies):
chmod +x beardog songbird skunkbat  # Tower Atomic
./songbird                           # starts mesh networking

# Verify integrity:
spore-validate depot-verify --checksums checksums.toml --depot ./
```

Full instructions: [Getting Started with plasmidBin](@/lab/getting-started-plasmidbin.md)

## Build from Source

```bash
# Prerequisites: Rust 1.85+ (2 minute install)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Clone and build any primal:
git clone https://github.com/ecoPrimals/songBird && cd songBird
cargo build --release --target x86_64-unknown-linux-musl
# Binary: target/x86_64-unknown-linux-musl/release/songbird
```

## Boot Order

NUCLEUS gates follow a specific startup sequence:

```
1. Tower Atomic (networking):  bearDog → songBird → skunkBat
2. Nest Atomic (storage):      nestGate → loamSpine → sweetGrass → rhizoCrypt
3. Node Atomic (compute):      toadStool → barraCuda → coralReef
4. Visualization:              petalTongue → squirrel
5. Orchestrator:               biomeOS (starts last, wires all capabilities)
```

{{ entity(name="cellmembrane") }} manages this sequence via `boot_order` configuration.

## Join the Mesh

The southGate validation gate model is how external deployments work:

1. Install Tower Atomic (3 binaries) on your machine
2. Contact us for mesh enrollment credentials
3. {{ entity(name="songbird") }} discovers peers and establishes encrypted tunnels
4. Your gate appears in the mesh with its capabilities advertised

[Contact us →](@/contact.md)

## Verify Everything

```bash
# Verify the site's claims independently:
git clone https://github.com/ecoPrimals/sporePrint && cd sporePrint
cd crates/spore-validate
cargo run --release -- validate    # registry checks, totals
cargo run --release -- certify     # guideStone Merkle root
```

Every quantitative claim on this site is backed by executable code.

## See Also

- [Living Systems](@/lab/living-systems.md) — what's running right now
- [Sovereign CI](@/architecture/SOVEREIGN_CI.md) — how binaries are built and distributed
- [NUCLEUS Architecture](@/architecture/NUCLEUS_ARCHITECTURE.md) — the composition model
- [Gate Mesh Topology](@/architecture/MESH_TOPOLOGY.md) — how gates connect
