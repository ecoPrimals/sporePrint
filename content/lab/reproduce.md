+++
title = "Reproduce It Yourself"
description = "Stand up a 13-primal NUCLEUS composition on your own hardware and run the same validated science workloads. No cloud. No institutional access."
date = 2026-05-06
weight = 10

[extra]
domain = "Infrastructure"

[taxonomies]
primals = ["toadstool", "beardog", "nestgate", "rhizocrypt", "loamspine", "sweetgrass", "barracuda"]
springs = ["wetspring", "primalspring"]
+++

Everything in the [Lab](@/lab/_index.md) ran on a single machine. You can
reproduce it on yours. The composition deploys the same way on any x86_64
Linux with at least 16 GB RAM.

---

## Prerequisites

- Linux (tested on Pop!_OS 22.04 / Ubuntu 22.04)
- Rust toolchain (`rustup` — installs in 2 minutes)
- 16 GB RAM minimum (96 GB recommended for full NCBI data)
- Git and basic build tools (`build-essential`)

Optional:
- Vulkan-capable GPU for {{ entity(name="barracuda") }} GPU workloads
- Python 3.10+ and R 4.x for baseline comparison pipelines

---

## Step 1: Get the Primal Binaries

```bash
# Clone plasmidBin (pre-built binaries)
git clone https://github.com/ecoPrimals/plasmidBin.git
export PLASMIDBIN="$(pwd)/plasmidBin"

# Or build from source (springs are all public AGPL)
git clone https://github.com/syntheticChemistry/wetSpring.git
cd wetSpring && cargo build --release --workspace
```

---

## Step 2: Deploy the Composition

```bash
git clone https://github.com/sporeGarden/projectNUCLEUS.git
cd projectNUCLEUS/deploy

# Deploy full NUCLEUS (13 primals) to the current machine
bash deploy.sh --composition full --gate mygate

# Verify all primals are healthy
bash deploy.sh --health-check
```

`deploy.sh` handles seed creation, primal startup ordering, health
verification, and port allocation. Primals bind to `127.0.0.1` by default.

---

## Step 3: Run the Science Workloads

```bash
# Run a single workload
toadstool execute ../workloads/wetspring/wetspring-16s-rust-validation.toml

# Run the full provenance pipeline (all workloads + DAG + ledger + braid)
bash provenance_pipeline.sh \
    --workloads-dir ../workloads/wetspring \
    --session-name "my-validation-run"
```

### Expected Results

| Workload | Checks | Domain |
|----------|--------|--------|
| 16S Pipeline | 37/37 | DADA2, chimera, taxonomy, UniFrac |
| Diversity Indices | 27/27 | Alpha/beta diversity, PCoA |
| Gonzales CPU Parity | 43/43 | PK, dose-response, Anderson spectral |
| Algae 16S (real data) | 34/34 | Full 16S on 11.9M NCBI reads |
| R Industry Parity | 53/53 | vegan, DADA2, phyloseq gold standards |
| Real NCBI Pipeline | 25/25 | Sovereign diversity + Anderson |
| Fajgenbaum Pathway | 8/8 | Immunology, drug repurposing |
| Cold Seep Pipeline | 8/8 | Metagenomics, QS gene catalog |

**Total**: 235+ checks, all at `tol=0.000000` (exact Python→Rust parity).

---

## Step 4: Verify the Provenance Chain

After the pipeline completes, you have:

```bash
# Check the Merkle root (content hash of all DAG events)
cat results/PROVENANCE_MANIFEST.md | grep "Merkle Root"

# Query the loamSpine ledger
curl -s -X POST http://localhost:9700 \
  -H 'Content-Type: application/json' \
  -d '{"jsonrpc":"2.0","method":"spine.list","params":{},"id":1}'

# Query the sweetGrass braid
curl -s -X POST http://localhost:9850/jsonrpc \
  -H 'Content-Type: application/json' \
  -d '{"jsonrpc":"2.0","method":"braid.list","params":{},"id":1}'
```

The braid carries an ed25519 witness signature from {{ entity(name="beardog") }}'s
key hierarchy. The Merkle root covers all data registrations and workload results
in one integrity proof. Tamper with one byte and the chain breaks.

---

## Step 5: Compare Your Results

If your workload output BLAKE3 hashes match the published hashes, the science is
bit-for-bit reproduced. The Merkle root and braid URN will differ (expected — they
include run-specific session IDs and timestamps), but the per-workload output
hashes should be identical.

```bash
# Hash a workload output
b3sum results/wetspring-16s-rust-validation.stdout
```

---

## What If Something Doesn't Match?

File an issue or send a gap report. That's the point — the methodology is
**falsifiable**. If the results diverge on your hardware, that's signal, not
failure. Document the divergence, the hardware, and the environment. The gap
report flows upstream through {{ entity(name="wateringhole") }} and improves the
ecosystem for everyone.

---

## Data Dependencies

For the full NCBI pipeline (real data, not synthetic):

```bash
# Download real NCBI data (requires ~5 GB disk)
# PRJNA488170: Nannochloropsis outdoor 16S (Wageningen)
prefetch SRR7760408 && fasterq-dump SRR7760408
```

All synthetic workloads run without external data downloads.

---

## Hardware Baselines

| Hardware | 16S Pipeline | Full Suite | Notes |
|----------|-------------|------------|-------|
| i9-14900K / 96 GB / RTX 4070 | <1s | ~30s | reference node |
| Ryzen 5800X / 64 GB / RTX 3070 | <1s | ~45s | swiftGate |
| Celeron J3455 / 8 GB / none | ~3s | ~5m | NUC (CPU only) |

Your times will vary. The checks should not.
