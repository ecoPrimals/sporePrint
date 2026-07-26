+++
title = "sporePrint"
description = "Self-hosted GPU-accelerated scientific computing in Rust: DADA2 bioinformatics, lattice QCD, pharmacometrics on commodity hardware. No CUDA, no cloud. WebGPU/WGSL on NVIDIA, AMD, Intel."
+++

**ecoPrimals** produces self-hosted scientific computing in pure Rust — GPU-accelerated bioinformatics, lattice QCD, pharmacometrics, and more, on commodity hardware without CUDA or cloud dependencies. **sporePrint** is this site — the permanent evidence record.

---

## Try It

**In 5 minutes, you can verify core claims on this site:**

```bash
# Build from source (requires Rust — 2 minute install)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
git clone https://github.com/syntheticChemistry/groundSpring && cd groundSpring
cargo test --workspace          # all tests pass, 0 failures
cargo run --release --bin validate  # exit 0 = pass
cargo deny check                # zero license violations, zero C dependencies
```

```bash
# Or: pre-built binaries from the depot (no Rust required)
# See https://sporeprint.primals.eco/lab/getting-started-plasmidbin/ for full instructions
# Binaries are musl-static, BLAKE3-verified, zero runtime dependencies
```

If those commands run, the claims are verified. No institutional access. No proprietary software. No cloud.

- 🧬 **Run real scientific pipelines locally** — genomics, protein structure, lattice QCD, pharmacometrics, precision agriculture
- 📄 **Reproduce published results** — {{ total_stat(stat="papers_reproduced") }} papers across {{ total_stat(stat="total_springs") }} springs, each as a binary you can run
- ⚡ **Use any GPU** — NVIDIA, AMD, Intel — no CUDA lock-in, no vendor toolchain
- 🔒 **Own your data and compute** — nothing leaves your machine
- ✅ **Validate with {{ entity(name="guidestone") }}** — self-verifying build artifacts that prove their own correctness

**Key evidence pages:**

- [GPU-accelerated DADA2 benchmark](@/lab/notebooks/02-benchmark-python-vs-rust.md) — Rust vs Python head-to-head on 16S rRNA pipeline
- [Self-hosted 16S bioinformatics](@/lab/wetspring-validation.md) — 235+ validation checks, real NCBI data
- [Cross-vendor f64 GPU computing](@/technical/SOVEREIGN_GPU_PIPELINE_PROFILE.md) — WGSL shaders on NVIDIA, AMD, Intel without CUDA
- [Lattice QCD on consumer GPUs](@/products/lattice_qcd.md) — gauge theory without CUDA, 59/59 checks
- [Self-hosted distributed compute mesh](@/architecture/MESH_TOPOLOGY.md) — multi-gate topology, 353× LAN throughput
- [Reproducible self-verifying software](@/guidestone/_index.md) — guideStone verification class

New here? Start with the [Glossary](@/glossary/_index.md) or skip straight to
[Getting Started with plasmidBin](@/lab/getting-started-plasmidbin.md) — pre-built
binaries, BLAKE3 verified, running in 5 minutes.

---

## Why "sporePrint"

A spore print is how mycologists identify species they have never seen before. You press the cap to paper and leave it overnight. In the morning: the permanent record of what the organism is, what it can produce, and how to grow it yourself.

This site is the spore print for {{ entity(name="ecoprimals") }}. The permanent, public, verifiable impression of a self-hosted, cloud-independent scientific computing ecosystem. Clone it. Run it. Verify it. Grow from it.

---

## What This Is (In Standard Terms)

ecoPrimals produces **self-contained scientific computations that reproduce published results on owned commodity hardware and carry their validation and provenance with them**. The platform is written entirely in Rust and provides a vendor-agnostic, locally executable, cryptographically verifiable alternative to the conventional stack.

| Conventional tool | ecoPrimals equivalent |
|---|---|
| CUDA GPU compute | **[barraCuda](@/technical/SOVEREIGN_GPU_PIPELINE_PROFILE.md)** — WGSL/WebGPU compute shaders, f64 precision, any GPU with Vulkan |
| Galaxy/QIIME2 bioinformatics | **[wetSpring](@/lab/wetspring-validation.md)** — GPU-accelerated 16S pipeline, DADA2, phylogenetics, PFAS |
| Cloud AlphaFold | **helixVision** — coralForge AlphaFold2/3 in pure Rust, local GPU |
| GROMACS / MD engines | **[hotSpring](@/products/lattice_qcd.md)** — SU(3) gauge theory, Sarkas Yukawa MD, nuclear EOS |
| NONMEM / Monolix PK-PD | **healthSpring** — clinical simulation, toxicology, biosignal analysis |
| Cloud remote sensing | **airSpring** — FAO-56 ET, soil moisture, crop modeling |
| Vendor GPU SDKs | **toadStool** — unified Vulkan/WebGPU dispatch, multi-vendor GPU management |
| VPN / service mesh | **[songBird](@/architecture/MESH_TOPOLOGY.md)** — JSON-RPC 2.0 capability routing, multi-node federation |
| Cloud object storage | **NestGate** — BLAKE3-hashed CAS with provenance chains |
| Package managers | **plasmidBin** — musl-static binaries, BLAKE3 verified, zero runtime deps |

All components are pure Rust, statically linked (musl), and licensed AGPL-3.0. Unsafe code is forbidden by default (`#![forbid(unsafe_code)]`) and isolated to narrowly scoped, safety-documented hardware-containment crates where required (e.g., toadStool GPU/NPU dispatch). The system runs on consumer hardware with any GPU exposing Vulkan drivers (tested: NVIDIA, AMD, Intel).
