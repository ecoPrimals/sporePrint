+++
title = "sporePrint"
description = "Self-hosted scientific computing. The science is executable, the infrastructure is inspectable, the claims can be reproduced by anyone with commodity hardware."
+++

## Try It

**In 5 minutes, you can verify everything on this site:**

```bash
# Build from source (requires Rust — 2 minute install)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
git clone https://github.com/syntheticChemistry/wetSpring && cd wetSpring/barracuda
cargo test --workspace          # 1,443+ tests, 0 failures
cargo run --release --bin validate_anderson_3d   # exit 0 = pass
cargo deny check                # zero license violations, zero C dependencies
```

```bash
# Or: pre-built guideStone artifact (no Rust required)
tar xf hotspring-guidestone-v0.7.0.tar.gz && cd validation/
./hotspring validate            # 59/59 pass, ~3 min, any x86_64 Linux
./hotspring benchmark           # characterize your hardware
```

If those commands run, the claims are verified. No institutional access. No proprietary software. No cloud.

- 🧬 **Run real scientific pipelines locally** — genomics, protein structure, lattice QCD, pharmacometrics, precision agriculture
- 📄 **Reproduce published results** — {{ total_stat(stat="papers_reproduced") }} papers across 8 domains, each as a binary you can run
- ⚡ **Use any GPU** — NVIDIA, AMD, Intel — no CUDA lock-in, no vendor toolchain
- 🔒 **Own your data and compute** — nothing leaves your machine
- ✅ **Validate with {{ entity(name="guidestone") }}** — self-verifying build artifacts that prove their own correctness

New here? Start with the [Glossary](@/glossary/_index.md) for plain-language
definitions of every term on this site. See the [Lab](@/lab/_index.md) for
live validation results from a {{ total_stat(stat="total_primals") }}-primal {{ entity(name="nucleus") }} composition —
235+ science checks with full provenance chains.

---

## Why "sporePrint"

A spore print is how mycologists identify species they have never seen before. You press the cap to paper and leave it overnight. In the morning: the permanent record of what the organism is, what it can produce, and how to grow it yourself.

This site is the spore print for {{ entity(name="ecoprimals") }}. The permanent, public, verifiable impression of a self-hosted, cloud-independent scientific computing ecosystem. Clone it. Run it. Verify it. Grow from it.
