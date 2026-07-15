+++
title = "Deployment Artifacts"
description = "What a guideStone deployment artifact is, what it contains, how to build one, and how to verify one — from USB drive to OCI container."
date = 2026-04-30
weight = 2

[taxonomies]
primals = ["barracuda", "beardog", "biomeos", "loamspine", "sourdough", "sweetgrass", "toadstool"]
springs = ["hotspring"]
trails = ["reproducibility"]
+++

## What a Deployment Artifact Is

A {{ entity(name="guidestone") }} deployment artifact is a self-contained directory
that can travel — on a USB drive, as a tarball, as an OCI container image —
and produce verified scientific results on any machine it reaches.

It is not a package that requires installation. It is not a container that
requires a runtime. It is a directory with binaries, data, and integrity
manifests that runs wherever it lands.

---

## Anatomy of an Artifact

The first artifact — `hotSpring-guideStone-v0.7.0` — established the
structure that all subsequent artifacts follow:

```
hotspring-guidestone-v0.7.0/
├── hotspring                 # x86_64 binary (static musl, zero deps)
├── hotspring-aarch64         # aarch64 binary (static musl, zero deps)
├── hotspring.bat             # Windows launcher (WSL2 -> Docker fallback)
├── CHECKSUMS                 # SHA-256 for every file in the directory
├── liveSpore.json            # Provenance: tracks every machine visited
├── README.md                 # Human-readable instructions
├── data/
│   ├── reference_configs/    # Gauge configurations, calibration data
│   └── expected_outputs/     # Reference results for validation
└── container/
    └── hotspring.tar         # OCI image for non-Linux platforms
```

Every file has a purpose. There are no build scripts, no Makefiles, no
dependency lists. The consumer's first interaction is `./hotspring validate`.

---

## The Integrity Manifest

`CHECKSUMS` is a plain-text file with one SHA-256 hash per line:

```
a3b8c9d4e5f6...  hotspring
7f8e9d0c1b2a...  hotspring-aarch64
...
```

Before any execution, the consumer can verify:

```bash
sha256sum -c CHECKSUMS
```

Every file listed. Every hash matching. If the transfer channel corrupted
a single byte, this step catches it before any physics runs.

---

## The Provenance Record

`liveSpore.json` is the artifact's memory. Every time the artifact runs on
a new machine, it appends a record:

```json
{
  "runs": [
    {
      "timestamp": "2026-03-15T14:22:00Z",
      "hostname_hash": "a3b8...",
      "arch": "x86_64",
      "gpu": "NVIDIA RTX 3090",
      "checks_passed": 59,
      "checks_total": 59,
      "wall_time_seconds": 47.2
    }
  ]
}
```

The hostname is hashed (privacy). The GPU is identified (reproducibility).
The check count is recorded (verification). When the
{{ entity(name="provenancetrio") }} is wired, this record feeds into
{{ entity(name="sweetgrass") }} attribution chains and {{ entity(name="loamspine") }}
permanence ledgers.

---

## How to Verify an Artifact

### On Linux (x86_64 or aarch64)

```bash
tar xf hotspring-guidestone-v0.7.0.tar.gz
cd hotspring-guidestone-v0.7.0
sha256sum -c CHECKSUMS
./hotspring validate
```

The validate command runs all physics checks, reports pass/fail for each,
and prints a summary. CPU-only validation takes approximately 3 minutes.
If a Vulkan-capable GPU is detected, GPU-accelerated paths run automatically.

### On Windows

```
hotspring.bat
```

The batch file detects WSL2, falls back to Docker if WSL2 is unavailable,
and runs validation inside a Linux environment.

### Via OCI Container

```bash
docker load < container/hotspring.tar
docker run hotspring validate
```

The container image contains the same static binary. No Ubuntu, no Alpine,
no package manager — just the binary and its data on a scratch image.

### From USB Drive

Insert the drive. Navigate to the directory.

```bash
cd /media/usb/hotspring-guidestone-v0.7.0
./hotspring validate
```

The artifact is designed for this use case. A PI receives a USB at a
conference, plugs it into their laptop, and gets physics results without
configuring anything.

---

## Building an Artifact

A spring that targets {{ entity(name="guidestone") }} certification builds its
artifact through {{ entity(name="biomeos") }}'s deploy graph system:

1. **Compile** — `cargo build --release --target x86_64-unknown-linux-musl`
   and the aarch64 equivalent. Static linking, zero dynamic dependencies.
2. **Embed** — reference data, expected outputs, and tolerance metadata are
   baked into the binary or placed alongside it.
3. **Checksum** — `sha256sum` every file, write `CHECKSUMS`.
4. **Validate locally** — run the artifact on the build machine, verify all
   checks pass.
5. **Cross-validate** — run on at least two different substrates (different
   CPU architecture, different GPU vendor, or no GPU). Compare outputs for
   bit-identity or named-tolerance agreement.
6. **Package** — tar the directory. Optionally build an OCI image.

{{ entity(name="sourdough") }} provides scaffolding for steps 1–3.
{{ entity(name="plasmidbin") }} distributes the finished artifact.

---

## The Self-Leveling Property

A {{ entity(name="guidestone") }} artifact is also a benchmark. When
`./hotspring validate` runs on unknown hardware, it discovers the
execution environment, runs physics, and reports:

| Output | What It Tells You |
|--------|-------------------|
| 59/59 PASS | The physics is correct on this machine |
| 47.2 seconds wall time | How fast this machine runs this physics |
| GPU: RTX 3090 detected | What hardware was used |
| 12,847 trajectories/sec | Throughput metric for comparison |

The artifact answers "is it correct?" and "how fast?" in a single
invocation. A PI evaluating a new GPU or a new cloud instance runs one
command and gets both answers.

---

## The Second Artifact: lithoSpore

{{ entity(name="lithospore") }} is the second {{ entity(name="guidestone") }}
deployment artifact — and the first **Targeted GuideStone**. While hotSpring
proves computational physics, lithoSpore proves evolutionary biology: 7
LTEE modules reproducing published papers from Barrick, Lenski, and collaborators.

```
lithoSpore/
├── validate                 # symlink → bin/litho (argv[0] dispatch)
├── verify                   # symlink → bin/litho
├── refresh                  # symlink → bin/litho
├── spore                    # symlink → bin/litho (biomeOS entry)
├── bin/litho                # Single musl-static binary (5.1 MB)
├── liveSpore.json           # Provenance journal
├── artifact/data/           # 7 LTEE data bundles (BLAKE3-anchored)
├── papers/                  # 16 DOIs + reading guide
└── GETTING_STARTED.md       # Human-readable entry point
```

The key evolution: a **single binary** replaces 7 separate module executables.
`litho` detects its invocation name via `argv[0]` and dispatches to the correct
subcommand. Cross-platform: 5.1 MB on Linux (musl-static), 7.9 MB on Windows
(mingw-w64). Validated on Ubuntu, Alpine, Fedora, read-only filesystems, and
Windows.

See the [lithoSpore artifact page](@/guidestone/lithospore_artifact.md) for
full documentation.
