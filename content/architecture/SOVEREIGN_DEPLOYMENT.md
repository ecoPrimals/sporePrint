+++
title = "Sovereign Deployment"
description = "From GitHub Pages to sovereign infrastructure — the PostPrimordial journey to self-hosted science."
date = 2026-05-31
weight = 16

[extra]
domain = "Architecture"
+++

## The Sovereignty Journey

The ecoPrimals ecosystem has evolved from a single developer machine to a
sovereign multi-node deployment. This is the PostPrimordial journey — each
step moving closer to complete independence from extracellular services.

## Current Architecture (Wave 65)

```
┌─────────────────────────────────────────────────────────┐
│  CYTOPLASM (LAN Gates)                                  │
│  eastGate · ironGate · southGate · biomeGate · strandGate │
│  + flockGate (WAN shadow)                               │
│  [Full NUCLEUS compositions, development, science]      │
└────────────────────┬────────────────────────────────────┘
                     │ covalent bond (SSH)
┌────────────────────▼────────────────────────────────────┐
│  INNER MEMBRANE — golgiBody (157.230.3.183)             │
│  Forgejo sovereign store · NUCLEUS primals              │
│  knot-dns · BTSP auth                                   │
└────────────────────┬────────────────────────────────────┘
                     │ metallic bond (SSH fleet key)
┌────────────────────▼────────────────────────────────────┐
│  PEPTIDOGLYCAN — peptidoglycan (157.230.209.218)        │
│  Full workspace · Rust toolchain · Zola                 │
│  Build hub · Temporal sync · Impulse cascade            │
└────────────────────┬────────────────────────────────────┘
                     │ ionic bond (BTSP-scoped)
┌────────────────────▼────────────────────────────────────┐
│  OUTER MEMBRANE — golgiBody-ext (137.184.197.151)       │
│  Caddy TLS · sporePrint (primals.eco) · TURN relay      │
│  [PUBLIC FACING — minimal attack surface]               │
└────────────────────┬────────────────────────────────────┘
                     │ weak bond (public read-only)
┌────────────────────▼────────────────────────────────────┐
│  EXTRACELLULAR — GitHub, CDN, public internet           │
│  Trailing mirrors · GitHub Actions CI · npm/crates.io   │
│  [SHADOW — not source of truth]                         │
└─────────────────────────────────────────────────────────┘
```

## Sovereignty Phases

| Phase | Achievement | Status |
|-------|-------------|--------|
| A | sporePrint served on golgiBody-ext via Caddy | LIVE |
| B | DNS cutover — primals.eco → golgiBody-ext | Pending (NS records at eastGate) |
| C | HTTPS via Caddy automatic TLS (Let's Encrypt) | Follows DNS cutover |
| D | GitHub Pages → extracellular shadow | Follows HTTPS |
| E | Forgejo CI replaces GitHub Actions | Wave 66+ |

## What Sovereignty Means

**Sovereignty is not isolation.** The ecosystem still uses GitHub (extracellular
shadow), still publishes to crates.io, still accepts collaborator contributions.
But none of these are load-bearing. Removing any extracellular service has zero
impact on development, science, or deployment.

The sovereignty posture:

- **Source of truth**: Forgejo on golgiBody (inner membrane)
- **Build authority**: peptidoglycan (structural layer)
- **Public face**: golgiBody-ext (outer membrane, Caddy)
- **GitHub**: Trailing mirror, not primary. Updated via relay chain.
- **DNS**: knot-dns on golgiBody. Authoritative once NS records delegate.

## The Relay Chain

Information flows outward through bond-mediated relay:

```
Gate → Forgejo (covalent)
  → post-receive hook fires
  → peptidoglycan pulls (metallic)
  → relays to golgiBody-ext (ionic)
  → golgiBody-ext pushes to GitHub (weak)
```

~3-6 seconds end-to-end. No manual intervention. Gates push to `forgejo`
only; the relay handles extracellular propagation.

## WAN Validation

flockGate (remote gate, different geographic region) validates that
sovereignty works across WAN:

- SSH to Forgejo over public internet: 1.2-1.4s
- Full relay propagation: ~5-8s
- `cascade-pull --source temporal` converges from remote site
- `zola build` produces full site locally (226 pages, 746ms)
- All sovereign operations function identically to LAN gates

## Hardware Independence

The VPS layer (DigitalOcean) is the last vendor dependency. The migration
path to full hardware sovereignty:

1. Current: DigitalOcean droplets (3 nodes, ~$50/month)
2. Near-term: Co-located ARM boards (Raspberry Pi 5 cluster, $200 one-time)
3. Long-term: Self-hosted x86 servers on-prem (strandGate, biomeGate as relay candidates)

The architecture is vendor-agnostic — any three nodes with SSH connectivity
can host the diderm envelope.
