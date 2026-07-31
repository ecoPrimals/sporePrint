+++
title = "Get Started"
description = "Deploy NUCLEUS on your own hardware — commodity Linux, Windows, or Android. Join the mesh with your own gate."
sort_by = "weight"
template = "section.html"
+++

ecoPrimals runs on commodity hardware you already own. A gate is any machine with a
chip and a drive — a desktop, a server, a laptop, a Steam Deck.

## What You Need

- **Linux** (x86_64): any distro with systemd. 30 MB RAM minimum for the full Tower stack.
- **Windows** (x86_64): Windows 10/11. TCP transport, no admin rights required for user-space deploy.
- **Android** (aarch64): via ADB. Experimental but proven on Pixel 8a.
- **GPU** (optional): any Vulkan-capable GPU for barraCuda scientific compute.

## The Stack

NUCLEUS is three atomic compositions:

| Composition | What It Does | Primals |
|-------------|-------------|---------|
| **Tower Atomic** | Encrypted mesh networking | bearDog + songBird + skunkBat |
| **Nest Atomic** | Content-addressed storage with provenance | nestGate + loamSpine + sweetGrass + rhizoCrypt |
| **Node Atomic** | GPU compute + visualization | toadStool + barraCuda + coralReef + petalTongue + squirrel |

biomeOS orchestrates all three. One binary per primal. No containers. No VMs.

## Quick Start

The fastest path is the [plasmidBin getting-started guide](/lab/getting-started-plasmidbin/) —
download the validated binary pack for your platform, extract, and run.

For the full deployment model, see [Sovereign Deployment](/architecture/sovereign-deployment/).

## Validation Gate (southGate Model)

The easiest way to join: ask for a validation gate enrollment. A friend's LAN pool
running Tower Atomic is how the mesh grows — not cloud signups.

[Contact us →](/contact/)
