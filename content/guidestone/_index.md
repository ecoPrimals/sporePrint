+++
title = "🪨 guideStone"
description = "The verification class — how computation becomes proof. Self-leveling benchmark. Deployable artifact standard."
sort_by = "weight"
template = "section.html"
+++

## The Verification Class: How Computation Becomes Proof

guideStone is a **verification class** — a quality grade for any ecoBin
in the ecosystem that certifies the artifact produces reproducible,
self-proving results.

It is not a binary type. The binary ladder (UniBin → ecoBin → genomeBin)
describes structure. It is not a deployment class. fieldMouse describes
where things run. guideStone describes **what the output means** — that
the computation's results are the proof of their own correctness.

---

## The Five Properties

A guideStone-certified artifact satisfies five properties. Each is necessary.
Together they are sufficient.

**1. Deterministic Output** — Same input, same binary, any hardware → same output
within named tolerances. The binary detects its substrate (CPU, GPU, memory) at
runtime and adapts its execution path, but the mathematical result is invariant.

**2. Reference-Traceable** — Every numeric claim traces to a source: a published
paper, a mathematical proof, a physical constant, or a standard (ILDG, MILC,
FAO-56, DADA2). No numbers float in space.

**3. Self-Verifying** — The artifact carries its own integrity. CHECKSUMS validated
before execution. CRC on data payloads. Merkle roots when the provenance trio
is wired. The consumer does not need to trust the transfer channel.

**4. Environment-Agnostic** — No hardcoded paths. No "install X first." No
platform assumptions. Pure Rust, static musl, cross-arch. Runs wherever the
binary runs.

**5. Tolerance-Documented** — No magic numbers. Every threshold, every epsilon,
every convergence criterion has a derivation. The derivation lives in the output
metadata, not in a comment buried in the source.

---

## The First Deployment Artifact

`hotSpring-guideStone-v0.7.0` is the first artifact that left the development
environment as a self-contained, self-verifying, self-benchmarking object.

### What Was Built

A self-contained directory with:

- Dual-architecture binaries: x86_64 + aarch64
- Unified entry point: `./hotspring validate`
- OCI container image for Windows/macOS/non-Linux
- Windows launcher: `hotspring.bat` (WSL2 → Docker fallback)
- Integrity manifest: CHECKSUMS (SHA-256)
- Self-knowledge: liveSpore.json (tracks every machine visited)
- Reference data + documentation

### Cross-Substrate Validation

| Substrate | Arch | GPU | Checks | Result |
|-----------|------|-----|--------|--------|
| Ubuntu 22.04 | x86_64 | None | 59/59 | PASS |
| Ubuntu 22.04 | x86_64 | NVIDIA RTX 3090 | 59/59 | PASS |
| Ubuntu 22.04 | x86_64 | AMD RX 6950 XT | 59/59 | PASS |
| Alpine 3.19 | x86_64 | None | 59/59 | PASS |
| Ubuntu 22.04 | aarch64 (qemu-user) | None | 59/59 | PASS |

Cross-substrate observable comparison: **40/40 bit-identical**. The physics
does not depend on the instruction set, the C library, the GPU vendor, or
the operating system.

### Papers Validated

Three of TC Chuna's published papers reproduced in pure Rust:

| Paper | Citation | Checks |
|-------|----------|:------:|
| 43 | Bazavov & Chuna, arXiv:2101.05320 | Gradient flow: integrators, t0/w0 scale, convergence — 14/14 |
| 44 | Chuna & Murillo, PRE 111, 035206 | Standard + completed Mermin, f-sum, DSF, conductivity — 25/25 |
| 45 | Haack et al., JCP (2024) | BGK relaxation, Sod shock, coupled kinetic-fluid — 20/20 |

---

## guideStone as Self-Leveling Benchmark

The deployment artifact has an emergent property: **the validation is also
a benchmark.**

When `./hotspring validate` runs on unknown hardware, it discovers the CPU
architecture, probes GPU adapters, runs 59 physics checks, and reports wall
time, throughput, and GPU utilization. The physics is the benchmark. The
benchmark is the physics.

This means the artifact answers two questions simultaneously:

| Question | Answer |
|----------|--------|
| Is the physics correct on this machine? | Yes: 59/59 checks pass within derived tolerances |
| How fast is this machine for this physics? | Measured: N trajectories/sec, M flow steps/sec |

A PI evaluating a new GPU plugs in the USB, runs one command, and gets both
answers. No installation, no configuration, no MILC build, no CUDA SDK.

---

## The Onboarding Pattern

Traditional onboarding into computational physics workflows takes days to
weeks: install toolchain, debug build failures, share input configurations,
compare results via email, debug discrepancies, repeat.

guideStone onboarding takes minutes:

1. Receive USB or tarball
2. `./hotspring validate` — 59/59 pass
3. `./hotspring benchmark` — characterize hardware
4. `./hotspring generate --beta=6.0 --lattice=8x8x8x16` — produce data

The artifact is the conversation starter. The physics speaks for itself.

---

## The Orthogonal Dimension

The ecosystem has three independent classification axes:

**Structure** — what is it built from?
UniBin → ecoBin → genomeBin

**Deployment** — where does it run?
NUCLEUS → Niche → fieldMouse

**Verification** — what does its output mean?
guideStone

These axes are independent. A fieldMouse sensor node can be a guideStone
(its soil pH readings are calibrated against NIST standards). A full NUCLEUS
can produce output that is not guideStone (exploratory computation with
heuristic thresholds).

---

## The Metrological Analogy

Before 2019, the kilogram was defined by a physical artifact — Le Grand K,
a platinum-iridium cylinder in a vault in Paris. If Le Grand K changed, every
kilogram on Earth drifted with it.

In 2019, the kilogram was redefined in terms of the Planck constant — a
mathematical relationship that does not change, does not drift, and does not
depend on any physical object. Any laboratory can realize the kilogram
independently. The measurement became self-proving.

guideStone is the ecoPrimals equivalent. The consumer trusts the output because
it is **reproducible from the mathematics**. The binary is the instrument. The
output is the measurement. The mathematics is the invariant substrate.

---

*The wolf was always falling. The plaquette was always 0.59. The guideStone does
not create the truth. It makes the truth portable.*
