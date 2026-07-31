+++
title = "Sovereign CI — Build Infrastructure"
description = "LIVE push-to-deploy pipeline: Forgejo → sporeGate build → sandbox validate → BLAKE3 depot → HTTPS serve. 35 binaries across 3 platforms. Zero human intervention."
date = 2026-07-31
weight = 18

[taxonomies]
primals = ["barracuda", "beardog", "biomeos", "coralreef", "loamspine", "nestgate", "petaltongue", "rhizocrypt", "skunkbat", "songbird", "squirrel", "sweetgrass", "toadstool"]

[extra]
domain = "Architecture"
maturity = "live"
+++

## Overview

> **Status (Wave 155n):** Sovereign CI is **LIVE**. sporeGate is the build authority — push to Forgejo triggers auto build → sandbox validate → depot push → HTTPS serve. **35 binaries** (16 musl + 4 gnu + 15 Windows), all BLAKE3 verified. J9+J10+J11 jelly strings KILLED — zero human intervention for musl builds. sporeGate is **11/11 HEALTHY**.

Every ecoPrimals binary is built from source on sovereign infrastructure. No GitHub Actions for production builds. No cloud CI. No third-party artifact registry. sporeGate pulls from Forgejo (`git.primals.eco`), cross-compiles for three target triples, computes BLAKE3 checksums, publishes to the depot, and broadcasts `mesh.publish depot.updated` so consumer gates auto-fetch.

## Build Pipeline

{{ viz_embed(src="/viz/ci-pipeline", caption="Sovereign CI pipeline: Forgejo commit to sporeGate build to golgi deploy") }}

```
Forgejo (git.primals.eco)
    │
    │ golgi cascade timer (15-min quorum)
    ▼
Builder gate (sporeGate / eastGate / any build_authority)
    │
    ├── membrane plasmid.harvest (manifest-driven)
    │   ├── cargo build --release --target x86_64-unknown-linux-musl
    │   ├── cargo build --release --target aarch64-unknown-linux-musl
    │   └── BLAKE3 checksums → checksums.toml
    │
    ├── rsync → depot (membrane.primals.eco/depot/{triple}/{binary})
    │
    └── songBird mesh.publish { topic: "depot.updated" }
        │
        ├── mesh.subscribe on all reachable peers
        ├── Consumer gates: membrane plasmid.auto_fetch (rate-limited)
        └── depot-verify validates BLAKE3 integrity
```

## Binary Inventory (Wave 155n)

{{ total_stat(stat="total_primals") }} primals compiled to **35 depot binaries** across 3 platforms:

| Target | Binaries | Gates |
|--------|----------|-------|
| `x86_64-unknown-linux-musl` | 16 | eastGate, sporeGate, westGate, strandGate, ironGate, flockGate |
| `x86_64-unknown-linux-gnu` | 4 (GPU trio + biomeOS) | strandGate (GPU compute) |
| `x86_64-pc-windows-gnu` | 15 | blueGate, swiftGate, northGate |

Per-binary sizes (x86_64-musl):

| Binary | x86_64-musl | aarch64-musl | Ratio |
|--------|-------------|--------------|-------|
| {{ entity(name="petaltongue") }} | 28 MB | 25 MB | 90% |
| {{ entity(name="songbird") }} | 23 MB | 20 MB | 90% |
| {{ entity(name="biomeos") }} | 20 MB | 18 MB | 92% |
| {{ entity(name="sweetgrass") }} | 13 MB | 14 MB | 101% |
| {{ entity(name="toadstool") }} | 13 MB | 9.7 MB | 75% |
| {{ entity(name="beardog") }} | 11 MB | 8.8 MB | 80% |
| {{ entity(name="nestgate") }} | 8.1 MB | 7.0 MB | 87% |
| {{ entity(name="coralreef") }} | 7.7 MB | 6.8 MB | 84% |
| {{ entity(name="rhizocrypt") }} | 7.5 MB | 6.1 MB | 81% |
| {{ entity(name="barracuda") }} | 5.4 MB | 4.3 MB | 79% |
| {{ entity(name="loamspine") }} | 4.5 MB | 3.8 MB | 85% |
| {{ entity(name="squirrel") }} | 4.3 MB | 3.4 MB | 78% |
| nucleus_launcher | 4.2 MB | 3.4 MB | 81% |
| sourdough | 3.0 MB | 2.6 MB | 83% |
| {{ entity(name="skunkbat") }} | 2.8 MB | 2.4 MB | 85% |
| **Total** | **153 MB** | **130 MB** | **85%** |

All binaries are statically linked against musl libc — no runtime dependencies.
The aarch64 binaries run on grapheneGate (Pixel 8a, GrapheneOS) and future ARM nodes.

## Build Convention

For a primal to be CI-buildable with zero manual intervention:

1. **Binary discoverable from workspace root**: `cargo build --release --target $TRIPLE --bin $slug`
2. **No special linker requirements** beyond the global `.cargo/config.toml`
3. **Toolchain declared** in `rust-toolchain.toml`
4. **Binary name = primal name lowercase** with no separators

All {{ total_stat(stat="total_primals") }} primals meet this convention. Three historical divergences are now resolved via `ecosystem_manifest.toml` build metadata:
- **CI-DIV-01**: {{ entity(name="biomeos") }} needs `--package biomeos-unibin` — encoded in `[build.biomeos]`
- **CI-DIV-02**: {{ entity(name="skunkbat") }} needs `--package skunk-bat-server` — encoded in `[build.skunkbat]`
- **CI-DIV-03**: {{ entity(name="nestgate") }} uses project `.cargo/config.toml` for linker config — resolved Wave 133a, `cargo_config = true` in `[build.nestgate]`

`plasmid.harvest` reads these entries from the manifest instead of relying on hardcoded bash workarounds.

## Verification

Any gate can verify its local depot against the published checksums:

```bash
spore-validate depot-verify \
  --checksums /path/to/checksums.toml \
  --depot /path/to/depot \
  --arch x86_64-unknown-linux-musl
```

`--partial` mode allows incremental verification — pass when all present binaries verify, even if the depot is incomplete. This supports staged rollouts where not all binaries have been pulled yet.

## Cascade Flow

The cascade is the heartbeat of the ecosystem. Two timers per gate:

- **cascade-pull.timer** (every 4h): full repo sync + harvest + fetch
- **cascade-sense.timer** (hourly): convergence monitoring, staleness detection

```
golgi (VPS)
    → pulls all 17+ repos from Forgejo
    → writes heads/golgi.toml (its local HEADs, SHA-validated)
    → runs unify_freshness() → regenerates freshness.toml
    → pushes wateringHole to GitHub (trailing mirror)

Each gate after cascade:
    → writes heads/<gate>.toml with its local repo HEADs
    → SHA validation: rejects truncated commits (00000... tails)
    → pushes wateringHole (FF-only pull first, no conflict)

mesh.status enrichment:
    → scans heads/*.toml for files older than 24h
    → reports stale_peers in mesh.status response
```

The write model is conflict-free: `wave.toml` is sole-writer (overwatch), each gate writes only its own `heads/<gate>.toml`. No merge conflicts. Ever.

## Crash-Loop Breaker (Wave 150x)

{{ entity(name="cellmembrane") }} provides `membrane gate.crash-loop` — a self-recovery
system that detects and stops runaway systemd services. The crash-loop breaker
scans all primal services and detects restart spirals.

Real-world validation: `biomeos-beacon` accumulated 29,081 restarts before the
breaker was shipped. The fix is structural:

| Problem | Fix |
|---------|-----|
| `StartLimitIntervalSec` in `[Service]` | Moved to `[Unit]` (where systemd reads it) |
| `WorkingDirectory` missing | Validated at install time |
| No restart ceiling | `CrashLoopReport` scan + disable logic |

The breaker runs at bootstrap/preflight and as an operator command. It detects
services with restart counts exceeding threshold, stops the crash-looping service,
reports to the operator, and prevents resource exhaustion.

## systemd Hardening

Every primal service runs under systemd with defense-in-depth:

| Hardening | Purpose |
|-----------|---------|
| `ProtectSystem=strict` | Read-only root filesystem |
| `PrivateTmp=yes` | Isolated `/tmp` |
| `NoNewPrivileges=yes` | Prevent privilege escalation |
| `MemoryDenyWriteExecute=yes` | W^X enforcement |

## DNSSEC

All three ecosystem domains are DNSSEC-signed:

| Domain | Purpose | DNSSEC |
|--------|---------|--------|
| `primals.eco` | Intra-membrane (gate-to-gate) | Signed |
| `primal.eco` | Inner membrane (public services) | Signed |
| `nestgate.io` | Data service point (NestGate CAS) | Signed |

## Related

- [Tower Atomic](@/architecture/tower_atomic.md) — the transport stack that Sovereign CI builds and deploys
- [Deployment Model](@/architecture/DEPLOYMENT_MODEL.md) — how binaries flow from depot to gates
- [Living Systems](@/lab/living-systems.md) — what's actually running right now
- [Gate Mesh — Live Topology](@/architecture/MESH_TOPOLOGY.md) — how gates connect
- [Ecosystem Coordination](@/architecture/coordination.md) — wateringHole standards and operational documents
- [provision-golgi.sh](https://github.com/ecoPrimals/wateringHole/blob/main/provision/provision-golgi.sh) — the VPS provisioning script in wateringHole
