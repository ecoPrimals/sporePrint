+++
title = "Deployable Artifact Standard"
description = "How guideStone becomes a self-leveling benchmark and portable science — the six-layer artifact model from entry point to plain-text documentation."
date = 2026-04-10
weight = 15

[extra]
domain = "guideStone"

[[extra.companions]]
url = "/methodology/sovereign-publication/"
title = "Sovereign Publication"
relation = "extended_by"
label = "Why portable artifacts are the new publication format"

[[extra.companions]]
url = "/architecture/coordination/"
title = "Ecosystem Coordination"
relation = "architecture"
label = "Coordination standards for artifact distribution"

[taxonomies]
trails = ["reproducibility"]
+++

## From Repository to Portable Science

{{ entity(name="guidestone") }} certifies that a binary produces reproducible results. The
Deployable Artifact Standard extends this to **portability**: the certified binary,
its reference data, its integrity manifest, and its documentation travel as a
self-contained object — USB drive, tarball, or OCI container.

---

## Six Layers

| Layer | Purpose | Contents |
|-------|---------|----------|
| 1. Entry point | One-command access | CHECKSUMS, detection script, platform fallback |
| 2. Binary layout | Multi-arch binaries | x86_64 static + GPU, aarch64 static |
| 3. Container fallback | Non-Linux access | OCI container image, Windows launcher |
| 4. Integrity | Trust without trust | SHA-256 manifest, CRC payloads, Merkle when provenance trio wired |
| 5. Self-knowledge | Where has this been? | liveSpore.json — tracks every machine visited |
| 6. Documentation | Human-readable context | Reference papers, tolerance derivations, plain-text README |

---

## Self-Leveling Benchmark

The validation IS the benchmark. When `./hotspring validate` runs on unknown hardware:

1. Detect CPU architecture (x86_64, aarch64)
2. Probe GPU adapters (Vulkan, fallback to CPU)
3. Run physics checks (59 checks in the first artifact)
4. Report pass/fail, wall time, throughput, GPU utilization

The consumer gets two answers from one command: **Is the physics correct?** and
**How fast is this machine?** No installation, no configuration, no dependency
resolution.

---

## Cross-Platform Matrix

| Platform | Method | Dependencies |
|----------|--------|-------------|
| Linux x86_64 | Native binary | None (static musl) |
| Linux aarch64 | Native binary | None (static musl) |
| HPC / Slurm | `srun ./hotspring validate` | None |
| Windows | `hotspring.bat` (WSL2 -> Docker fallback) | WSL2 or Docker Desktop |
| macOS | OCI container | Docker or Podman |
| CI/CD | Container image | Any OCI runtime |

---

## liveSpore.json — Self-Knowledge

Each artifact carries a `liveSpore.json` that records every machine it has visited:

```json
{
  "artifact": "hotSpring-guideStone-v0.7.0",
  "created": "2026-04-10T00:00:00Z",
  "visits": [
    {
      "hostname_hash": "a3f2...",
      "arch": "x86_64",
      "gpu": "NVIDIA RTX 3090",
      "checks_passed": 59,
      "checks_total": 59,
      "wall_time_seconds": 42.3,
      "timestamp": "2026-04-11T14:30:00Z"
    }
  ]
}
```

The artifact knows where it has been and what it found. A PI reviewing
cross-substrate validation does not need to ask "has anyone tested this on
AMD?" — they check `liveSpore.json`.

---

## Relationship to Other Standards

| Standard | What It Certifies | Relationship |
|----------|------------------|--------------|
| {{ entity(name="ecobin") }} | Binary structure | Structure axis |
| {{ entity(name="fieldmouse") }} | Deployment surface | Deployment axis |
| {{ entity(name="guidestone") }} | Output verification | Verification axis |
| Deployable Artifact | Portable delivery | **Combines all three** |

The Deployable Artifact Standard is the intersection: a structurally compliant
binary ({{ entity(name="ecobin") }}), verified by {{ entity(name="guidestone") }}, packaged for any
deployment surface ({{ entity(name="fieldmouse") }}).

---

## Evolution Targets

| Target | Status |
|--------|--------|
| ARM GPU acceleration | Planned (Jetson, Apple Silicon) |
| {{ entity(name="genomebin") }} manifest | Planned (full composition artifact) |
| {{ entity(name="beardog") }} signing | Planned (cryptographic attestation of artifact integrity) |
| {{ entity(name="helixvision") }} artifact | Planned (genomics self-leveling benchmark) |
| Multi-artifact deploy graphs | Planned (compositions of compositions) |

---

*The artifact is the conversation. Not a pitch deck. Not a publication. Not a
"contact us for a demo." A self-contained, self-verifying, self-benchmarking
object that runs the science on any machine and answers its own questions. Plug
it in, run one command, read the results.*
