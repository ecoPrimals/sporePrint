+++
title = "Depot Binary Status"
description = "Sovereign binary depot — 35 binaries across 3 platforms, BLAKE3 checksums, zero external CI."
date = 2026-08-01
weight = 5

[extra]
maturity = "live"
+++

The depot is the sovereign binary distribution point for the ecoPrimals fleet.
Every binary is built by Sovereign CI on sporeGate, checksummed with BLAKE3,
and distributed to gates via rsync over WireGuard.

## Current Depot Inventory

| Platform | Target | Binaries | Notes |
|----------|--------|----------|-------|
| Linux (musl) | x86_64-unknown-linux-musl | 16 | Static, zero glibc deps |
| Linux (gnu) | x86_64-unknown-linux-gnu | 4 | GPU primals (need glibc for Vulkan) |
| Windows | x86_64-pc-windows-msvc | 15 | Cross-compiled on sporeGate |
| **Total** | 3 platforms | **35** | |

## Build Pipeline

```
Developer pushes to Forgejo (git.primals.eco)
    ↓
Forgejo post-receive hook
    ↓
sovereign-ci-trigger.sh → sporeGate (over WireGuard)
    ↓
sporeGate builds (cargo build --release --target <triple>)
    ↓
BLAKE3 checksum computed → checksums.toml updated
    ↓
rsync to golgiBody depot
    ↓
Gates pull updated binaries on next heartbeat
```

Zero GitHub Actions. Zero external CI. The build machine (sporeGate) is
on the sovereign mesh, building from Forgejo source.

## Integrity Verification

Every binary in the depot has a BLAKE3 checksum recorded in `checksums.toml`.
Gates verify integrity on pull. `spore-validate depot-verify` independently
checks all binaries against their recorded checksums.

```bash
spore-validate depot-verify
```

## Platform Coverage

- **musl builds** (16): All 13 primals + sourDough + plasmidBin + spore-validate.
  Fully static. Run on any x86_64 Linux without glibc.
- **gnu builds** (4): barraCuda, toadStool, coralReef, rustChip.
  Need glibc for Vulkan/GPU driver linking.
- **Windows builds** (15): Cross-compiled from Linux via cargo.
  Run natively on Windows 10/11 (ironGate validated).

## Pending: Live Freshness

This page currently shows static inventory data. When petalTongue G19
Node Atomics rendering is complete, it will serve real-time depot status
including binary freshness, last-build timestamps, and checksum verification.

Data source: `spore-validate depot-verify` + `spore-validate depot-list-arches`
