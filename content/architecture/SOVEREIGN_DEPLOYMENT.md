+++
title = "Sovereign Deployment"
description = "K-Derm cell envelope topology, WireGuard mesh overlay, multi-gate enrollment, and the PostPrimordial sovereignty model."
date = 2026-06-19
weight = 16

[extra]
foundation = true
domain = "Architecture"
maturity = "implemented"

[[extra.companions]]
url = "/philosophy/i-own-nothing/"
title = "I Own Nothing"
relation = "narrative_version"
label = "The economics of giving it all away — why sovereignty requires generosity"

[[extra.companions]]
url = "/story/the-sovereign-lab/"
title = "The Sovereign Lab"
relation = "narrative_version"
label = "The builder's narrative of sovereignty in practice"
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

## Transport: WireGuard → Tower Atomic

The mesh is transitioning from WireGuard kernel tunnels to [Tower Atomic](@/architecture/tower_atomic.md) — a userspace capability-aware encrypted mesh. Both stacks currently run in parallel (shadow mode), with Tower proven to exceed WireGuard on throughput and jitter.

| Node | Overlay IP | Role | Tower Status |
|------|-----------|------|-------------|
| golgi (hub) | 10.13.37.1 | VPS hub, Forgejo, Caddy, WAN depot, TURN relay | LIVE |
| sporeGate | 10.13.37.2 | Build authority, HPC interface, benchmark driver | LIVE (shadow) |
| eastGate | 10.13.37.5 | Code hub, primalSpring overwatch | LIVE (shadow) |
| flockGate | 10.13.37.6 | WAN, Tower primal teams | LIVE (shadow) |
| northGate | 10.13.37.8 | Windows 11, RTX 5090 | Enrolled |
| grapheneGate | — | HSM testing | Tower LIVE |

Tower Atomic adds topology awareness that WireGuard cannot provide: LAN peer discovery via `lan_addr` bypasses the VPS hub entirely (0.61ms vs 154ms for same-switch gates). See [Gate Mesh Topology](@/architecture/MESH_TOPOLOGY.md) for the full gate map and enrollment process.

The mesh provides:

- **Identity**: Each gate has a stable cryptographic identity (bearDog Ed25519 + WireGuard key)
- **Encryption**: All inter-gate traffic encrypted via BTSP per-session keys (Tower) or WireGuard tunnel
- **Connectivity**: Gates behind NAT, cellular, or restrictive firewalls connect through TURN relay on golgiBody
- **Addressability**: Stable overlay IPs survive physical network changes
- **Capability routing**: Tower dispatches by capability name, not IP address

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

## Outer Membrane — License Enforcement

The inner membrane (BearDog/BTSP) uses entropy tiers to distinguish human from machine at the cryptographic handshake. The outer membrane (Caddy/public web) cannot authenticate the consumer — but it can make the license structurally inescapable at every layer of the response.

### Three-Layer License Embedding

| Layer | Mechanism | What it proves |
|-------|-----------|---------------|
| **Transport** | HTTP `Link: <agpl-3.0>; rel="license"` header | License was served with the content — visible to any HTTP client, logged by any proxy |
| **Document** | `<link rel="license">` + `<meta name="rights">` + Dublin Core `dcterms.license` | License is in the DOM — parsed by crawlers, scrapers, and AI agents |
| **Structured data** | JSON-LD `"license"` field on WebSite + per-section schemas | License is machine-readable structured data — consumed by search engines and knowledge graphs |

### Provenance Chain

Every page on primals.eco has:

1. **BLAKE3 content hash** — `content-manifest.toml` hashes every page at build time
2. **Merkle root** — guideStone certification manifest computes the root over all content
3. **Timestamped commits** — git history on Forgejo (sovereign) and GitHub (shadow)
4. **License in the response** — transport, document, and structured data layers

If an AI model trains on this content and produces similar output, the provenance chain proves:
- The content existed at a specific time (git + Merkle root)
- The content was served with AGPL-3.0-or-later at that time (HTTP headers + DOM + JSON-LD)
- The content is content-addressed (BLAKE3 — any copy can be verified against the manifest)

The enforcement is not technical blocking — it is structural attribution. The license is woven into every byte at every layer. Removing it requires actively stripping it, which is itself a violation.

### Recommended Caddy Headers

```
header {
    Link "<https://www.gnu.org/licenses/agpl-3.0.html>; rel=\"license\""
    X-Content-License "AGPL-3.0-or-later"
    X-Provenance "blake3:content-manifest.toml; merkle:certification/manifest.json"
}
```

### Access Policy

The outer membrane does not block any consumer. The `robots.txt` explicitly welcomes all crawlers, AI agents, and search engines. The distinction between access levels is not enforced by blocking but by license embedding:

- **Human readers**: see content, can verify via `spore-validate certify`
- **AI agents on behalf of humans**: full access — assistive technology is welcome
- **Training crawlers**: full access — the license travels with the training data
- **Search engines**: full access — indexing aids discoverability

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

## Sovereign CI Pipeline

All primals are continuously built from source on sporeGate's [Sovereign CI](@/architecture/SOVEREIGN_CI.md). No GitHub Actions, no Jenkins — Forgejo webhooks trigger builds on the sovereign build authority. See [Sovereign CI](@/architecture/SOVEREIGN_CI.md) for the full architecture.

### Crash-Loop Breaker

systemd services across the mesh are hardened with crash-loop detection via `membrane gate.crash-loop`. Real-world validation: `biomeos-beacon` accumulated 29,081 restarts before the breaker was shipped. The fix is structural — `StartLimitIntervalSec` placement, `WorkingDirectory` validation at install time.

### systemd Hardening

Every primal service runs under systemd with `ProtectSystem=strict`, `PrivateTmp=yes`, `NoNewPrivileges=yes`, `MemoryDenyWriteExecute=yes`, and other defense-in-depth measures.

### DNSSEC

All three ecosystem domains (`primals.eco`, `primal.eco`, `nestgate.io`) are DNSSEC-signed.

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
