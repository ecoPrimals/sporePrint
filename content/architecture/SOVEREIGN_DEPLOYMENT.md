+++
title = "Sovereign Deployment"
description = "K-Derm cell envelope topology, WireGuard mesh overlay, multi-gate enrollment, and the PostPrimordial sovereignty model."
date = 2026-06-19
weight = 16

[extra]
domain = "Architecture"
+++

## The Sovereignty Journey

The ecoPrimals ecosystem has evolved from a single developer machine to a
sovereign multi-gate deployment connected by an encrypted WireGuard mesh.
This is the PostPrimordial journey — each step moving closer to complete
independence from extracellular services.

## K-Derm Topology (Wave 116)

The deployment architecture follows the **K-Derm** cell envelope model,
derived from Gram-negative bacterial biology. Layers are named from inside
out using absolute positions — no ambiguous "inner/outer" terminology.

```
┌─────────────────────────────────────────────────────────┐
│  CYTOPLASM — Gate NUCLEUS (UDS IPC only)                │
│  eastGate · ironGate · sporeGate · flockGate (WAN)     │
│  [13 primals, JSON-RPC over Unix sockets]              │
└────────────────────┬────────────────────────────────────┘
                     │ gate firewall (UFW / nftables)
┌────────────────────▼────────────────────────────────────┐
│  PLASMA MEMBRANE — Gate firewall boundary               │
│  Mediates all exits from cytoplasm. LAN gates test      │
│  this layer directly via sporeGate nftables rules.     │
└────────────────────┬────────────────────────────────────┘
                     │ WireGuard tunnel (encrypted overlay)
┌────────────────────▼────────────────────────────────────┐
│  PERIPLASM — WireGuard overlay + relay services         │
│  golgi hub (10.13.37.1) · RustDesk relay · routing     │
│  WAN gates (flockGate) validate this end-to-end.       │
└────────────────────┬────────────────────────────────────┘
                     │ VPS channels (Signal / Relay / Surface)
┌────────────────────▼────────────────────────────────────┐
│  OUTER MEMBRANE — golgiBody-ext VPS                     │
│  Caddy TLS · primals.eco · TURN · plasmidBin depot     │
│  [PUBLIC FACING — minimal attack surface]               │
└────────────────────┬────────────────────────────────────┘
                     │ public internet (read-only mirrors)
┌────────────────────▼────────────────────────────────────┐
│  EXTRACELLULAR — GitHub, CDN, DNS registrars            │
│  Trailing mirrors — not source of truth                 │
└─────────────────────────────────────────────────────────┘
```

### Topology Variants

| Topology | Structure | Example |
|----------|-----------|---------|
| Monoderm | Cytoplasm → plasma → environment | Home lab gate on LAN only |
| Diderm | Cytoplasm → plasma → periplasm → outer → extracellular | Production: gate + VPS relay |

WAN gates like flockGate exercise the full diderm path — their traffic
traverses real internet to reach the periplasm.

## WireGuard Mesh Overlay

All gates connect through a sovereign encrypted overlay network:

| Node | Overlay IP | Role | Measured RTT |
|------|-----------|------|--------------|
| golgi (hub) | 10.13.37.1 | VPS hub, Forgejo, Caddy, WAN depot | — |
| sporeGate | 10.13.37.2 | Compute node, Sovereign CI, Nest | <1ms to golgi |
| eastGate | 10.13.37.5 | Meta (orchestration, AI, viz) | <1ms to golgi |
| flockGate | 10.13.37.6 | WAN, Tower Atomic, sporePrint | 27ms to golgi |
| ironGate | 10.13.37.7 | Node Atomic (GPU, fleet dispatch) | <1ms to golgi |

Hub-and-spoke topology with golgi as the central peer. Each gate maintains
a persistent tunnel. The mesh provides:

- **Identity**: Each gate has a stable cryptographic identity (WireGuard public key)
- **Encryption**: All inter-gate traffic is encrypted regardless of transport
- **Connectivity**: Gates behind NAT, cellular, or restrictive firewalls connect through the hub
- **Addressability**: Stable overlay IPs survive physical network changes

## Gate Enrollment

Any internet-connected machine can become a gate. The enrollment process:

### Step 1: Sovereign Relay

Configure RustDesk relay pointing to sovereign infrastructure. This
provides remote access for the enrollment team without depending on
third-party services.

### Step 2: SSH Access

Install and enable SSH server. Authorize the sporeGate overwatch team's
key for deployment access.

### Step 3: WireGuard Peer Exchange

Generate a WireGuard keypair on the new gate. Configure `wg0` with the
assigned overlay IP and golgi as the peer endpoint. Submit the public key
to the hub operator for peer addition. Verify bidirectional handshake.

### Step 4: NUCLEUS Deploy

The sporeGate team fetches pre-built binaries from plasmidBin
(`membrane.primals.eco/depot/`), verifies BLAKE3 checksums against
`checksums.toml`, and deploys all 13 primals. Gates never compile from
source — they consume the sovereign build authority's output.

### Step 5: Cascade Connectivity

Configure git remotes for Forgejo (`git.primals.eco`) and GitHub. Push
to both remotes. Successful push proves the full cascade pipeline works
from the new gate's network position.

### Step 6: Federation

Songbird initiates `mesh.init` to join the federation mesh. BearDog
performs BTSP handshakes with existing peers. The gate is now a full
participant in the sovereign collective.

## What Sovereignty Means

**Sovereignty is not isolation.** The ecosystem still uses GitHub (extracellular
shadow), still publishes to crates.io, still accepts collaborator contributions.
But none of these are load-bearing. Removing any extracellular service has zero
impact on development, science, or deployment.

The sovereignty posture:

- **Source of truth**: Forgejo on golgi (periplasm)
- **Build authority**: sporeGate + eastGate (Sovereign CI, any `build_authority = true` gate)
- **Binary depot**: plasmidBin on golgi via Caddy (outer membrane)
- **Public face**: primals.eco via Caddy (outer membrane)
- **GitHub**: Trailing mirror, not primary. Updated via cascade relay.
- **DNS**: Sovereign, delegated to golgi infrastructure

## Composition Profiles — Fractal Deployment

Not every gate runs all 13 primals. The ecosystem defines **composition profiles**
in `ecosystem_manifest.toml` — replicable deployment shapes that can be instantiated
on any hardware:

| Composition | Primals | Purpose | Examples |
|-------------|---------|---------|----------|
| **full** | All 13+ | Complete sovereign NUCLEUS | eastGate, ironGate |
| **thin-relay** | songBird, nestGate, membrane | Depot + relay + sporePrint. No source repos. | golgiBody VPS |
| **tower** | bearDog, songBird, skunkBat | Minimal secure mesh entry | grapheneGate |
| **compute** | toadStool, barraCuda, coralReef, biomeOS | HPC/GPU workloads | strandGate |
| **nest** | nestGate, sweetGrass, rhizoCrypt | Cold storage and CAS | westGate |

### Thin Relay — Sovereign Presence Anywhere

The **thin-relay** composition is the fractal building block for sovereign
infrastructure. It requires no Rust toolchain and no primal source repos — only
pre-built ecobins from the depot:

```
thin-relay gate:
  ├── songBird (mesh relay + drawbridge)
  ├── nestGate (sporePrint website hosting)
  ├── membrane (cascade CLI + auto-fetch)
  └── wateringHole (only repo tracked)
```

**Deploy anywhere**: VPS nodes, HPC sites, edge locations, partner infrastructure.
A thin relay receives ecobins via `mesh.subscribe → plasmid.auto_fetch` and serves
them through Caddy TLS. It participates in the mesh federation but doesn't build
anything — it consumes the build authority's output.

**Fractal principle**: deploy a thin relay at an HPC site to serve specialized
compute ecobins. Deploy one at a university to host a sporePrint mirror. Deploy
one on a Raspberry Pi as a field data collector. The pattern is identical — only
the composition profile and the ecobins change.

Query a gate's composition: `membrane plasmid.composition --gate golgiBody`
List all profiles: `membrane plasmid.composition`

## The Cascade Pipeline

Information flows outward through bond-mediated relay:

```
Gate → Forgejo (covalent, SSH over WireGuard)
  → post-receive hook fires (golgi)
  → sovereign-ci SSH → sporeGate (metallic)
  → cargo build → rsync depot to golgi
  → golgi Caddy serves depot + site (ionic)
  → GitHub mirror updated (weak)
```

~3-8 seconds end-to-end. No manual intervention. Gates push to both
`forgejo` and `github` remotes; cascade validates the full path.

## WAN Validation

flockGate (WAN gate, different geographic region) validates that
sovereignty works without LAN proximity:

- Push to Forgejo over WAN: ~1.4s
- Push to GitHub: ~1.4s
- Full cascade propagation: ~5-8s
- WireGuard tunnel: 32ms RTT to golgi
- All sovereign operations function identically to LAN gates

If it works on flockGate, it works on any internet-connected machine.
flockGate is the template for every future WAN gate: a friend's NUC,
a colo server, a VPS.

## Hardware Independence

The VPS layer (DigitalOcean) is the last vendor dependency. The migration
path to full hardware sovereignty:

1. Current: Single DigitalOcean droplet (golgi) + LAN gates + WAN gates
2. Near-term: Co-located ARM boards, consumer hardware (NUCs, Pixels)
3. Long-term: Any internet-connected machine with SSH can enroll

The architecture is vendor-agnostic — any machine that can reach the
WireGuard hub can host a NUCLEUS and participate in the collective.
