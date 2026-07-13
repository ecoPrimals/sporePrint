+++
title = "Getting Started with plasmidBin"
description = "From zero to running primals in 5 minutes using pre-built binaries. No compilation required."
date = 2026-07-07
weight = 12

[taxonomies]
primals = ["barracuda", "beardog", "biomeos", "coralreef", "nestgate", "skunkbat", "songbird", "toadstool"]

[extra]
domain = "Infrastructure"
+++

{{ entity(name="plasmidbin") }} distributes pre-built primal binaries — statically
linked, musl-based, BLAKE3-checksummed. This page gets you from zero to a
running composition without compiling anything.

**Time**: ~5 minutes (download + verify + start)
**Requirements**: Any x86_64 Linux (Ubuntu, Pop!_OS, Fedora, Arch, Alpine)

---

## Step 1: Clone the Depot

```bash
git clone https://github.com/ecoPrimals/plasmidBin.git
cd plasmidBin
```

The depot contains pre-built binaries for {{ total_stat(stat="total_primals") }} primals
across two architectures (x86_64 and aarch64), plus `checksums.toml` for
integrity verification.

---

## Step 2: Verify Integrity

Every binary has a BLAKE3 checksum recorded in `checksums.toml`. Verify
them before running anything:

```bash
# Install b3sum if you don't have it (one-time)
cargo install b3sum

# Verify all binaries match their recorded checksums
b3sum --check checksums.toml
```

If any binary fails verification, do not run it. Re-fetch from the depot
or build from source.

You can also verify with sporePrint's built-in depot checker:

```bash
git clone https://github.com/ecoPrimals/sporePrint.git
cd sporePrint
cargo run --manifest-path crates/spore-validate/Cargo.toml -- \
    depot-verify --checksums ../plasmidBin/checksums.toml \
                 --depot ../plasmidBin/primals/x86_64 \
                 --arch x86_64-unknown-linux-musl
```

---

## Step 3: Run a Single Primal

Each primal is a self-contained static binary. Pick one and run it:

```bash
# Start bearDog (cryptographic identity)
./primals/x86_64/beardog server

# In another terminal, check health:
curl -s -X POST http://localhost:9300 \
  -H 'Content-Type: application/json' \
  -d '{"jsonrpc":"2.0","method":"health.check","params":{},"id":1}'
```

Expected response:

```json
{"jsonrpc":"2.0","result":{"status":"healthy","primal":"bearDog","version":"..."},"id":1}
```

Every primal follows this pattern: `./primal server` starts it, `health.check`
verifies it's alive. No configuration files needed for basic operation.

---

## Step 4: Deploy a Composition

For a multi-primal composition, use {{ entity(name="projectnucleus") }}:

```bash
git clone https://github.com/sporeGarden/projectNUCLEUS.git
cd projectNUCLEUS/deploy

# Deploy Tower composition (bearDog + songBird + skunkBat)
bash deploy.sh --composition tower --gate mygate

# Or deploy full NUCLEUS (all primals)
bash deploy.sh --composition full --gate mygate

# Verify everything is healthy
bash deploy.sh --health-check
```

All primals bind to `127.0.0.1` by default. No ports are exposed to the
network. {{ entity(name="songbird") }} handles all inter-primal routing via
Unix domain sockets.

---

## Step 5: Run Science (Optional)

With a running composition, you can execute validated science workloads:

```bash
# Run a single validated workload
./primals/x86_64/toadstool execute \
    ../workloads/wetspring/wetspring-16s-rust-validation.toml
```

See [Reproduce Results](@/lab/reproduce.md) for the full set of 235+
validated science checks with provenance chains.

---

## Composition Quick Reference

| Composition | Primals | What You Get |
|-------------|---------|-------------|
| **Tower** | {{ entity(name="beardog") }} + {{ entity(name="songbird") }} + {{ entity(name="skunkbat") }} | Crypto identity + mesh networking + threat detection |
| **Nest** | Tower + {{ entity(name="nestgate") }} | + content-addressed storage |
| **Node** | Tower + {{ entity(name="toadstool") }} + {{ entity(name="barracuda") }} + {{ entity(name="coralreef") }} | + GPU/CPU compute |
| **Full {{ entity(name="nucleus") }}** | All foundation primals | Complete sovereign stack with AI coordination |

---

## What Makes This Different

- **No package manager**: The binaries are the deployment. No apt, no docker, no pip.
- **No compilation**: musl-static PIE binaries run on any Linux kernel 3.2+.
- **No cloud dependency**: After cloning, everything is offline-capable.
- **Cryptographic verification**: BLAKE3 checksums cover every binary. Tamper detection is built in.
- **AGPL source available**: Every binary has corresponding source at [github.com/ecoPrimals](https://github.com/ecoPrimals).

---

## Troubleshooting

**Binary won't execute**: Check permissions (`chmod +x ./primals/x86_64/beardog`)
and verify you're on the right architecture (`uname -m` should return `x86_64`).

**Checksum mismatch**: The depot may have been updated since your clone. Run
`git pull` and re-verify, or compare against the
[published checksums](https://github.com/ecoPrimals/plasmidBin/blob/main/checksums.toml).

**Health check returns connection refused**: The primal may still be starting.
Wait 2–3 seconds and retry. Check the primal's stdout for error messages.

---

## Next Steps

- [Reproduce Results](@/lab/reproduce.md) — run validated science workloads
- [Deployment Model](@/architecture/DEPLOYMENT_MODEL.md) — architecture details, metadata.toml format
- [Deployment Artifacts](@/guidestone/deployment_artifacts.md) — guideStone self-verifying artifacts
- [Compute Access](@/lab/compute-access.md) — access the live mesh without deploying locally
