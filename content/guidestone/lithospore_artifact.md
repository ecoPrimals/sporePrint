+++
title = "lithoSpore — LTEE Targeted GuideStone"
description = "The second guideStone deployment artifact — a USB-deployable validation system that reproduces 7 Long-Term Evolution Experiment papers and generates new predictions using the Anderson disorder framework."
date = 2026-05-16
weight = 4

[taxonomies]
primals = ["toadstool", "barracuda", "nestgate", "biomeos", "petaltongue"]
springs = ["groundspring", "wetspring", "hotspring", "neuralspring"]
+++

## What This Is

{{ entity(name="lithospore") }} is the second {{ entity(name="guidestone") }} deployment
artifact — and the first **Targeted GuideStone**. While hotSpring's guideStone proves
computational physics on consumer GPUs, lithoSpore proves evolutionary biology: 7 science
modules reproducing LTEE papers from Barrick, Lenski, and collaborators across 75,000+
generations of continuous evolution.

The target audience is **Barrick Lab, UT Austin** — the continuation of Richard Lenski's
Long-Term Evolution Experiment. The artifact is designed to be handed to a researcher as
a USB drive, plugged into any Linux machine, and validated with zero dependencies.

---

## Anatomy of the Artifact

```
/media/lithoSpore/
├── .biomeos-spore                    # biomeOS detection marker
├── validate                          # symlink → bin/litho (argv[0] dispatch)
├── verify                            # symlink → bin/litho
├── refresh                           # symlink → bin/litho
├── spore                             # symlink → bin/litho (biomeOS entry)
├── liveSpore.json                    # Append-only provenance journal
├── GETTING_STARTED.md                # Human-readable guide
│
├── bin/
│   └── litho                         # Single musl-static binary (5.1 MB)
│
├── biomeOS/
│   ├── tower.toml                    # Tower config for spore composition
│   └── graphs/lithoSpore_validation.toml
│
├── artifact/
│   ├── data/                         # 7 LTEE data bundles (BLAKE3-anchored)
│   │   ├── wiser_2013/              # Module 1: power-law fitness
│   │   ├── barrick_2009/            # Module 2: mutation accumulation
│   │   ├── good_2017/               # Module 3: allele trajectories
│   │   ├── blount_2012/             # Module 4: citrate innovation
│   │   ├── biobricks_2024/          # Module 5: BioBrick burden
│   │   ├── tenaillon_2016/          # Module 6: 264 genomes
│   │   └── anderson_predictions/    # Module 7: disorder predictions
│   ├── data.toml                    # Data manifest (URIs + BLAKE3)
│   ├── scope.toml                   # Scope graph (birth certificate)
│   └── tolerances.toml              # Named tolerances with justification
│
├── validation/expected/              # Reference outputs (7 JSON files)
├── notebooks/                        # Python Tier 1 baselines
├── papers/                           # Registry (16 DOIs) + reading guide
└── figures/                          # Publication-quality SVG figures
```

The key difference from hotSpring's artifact: a **single binary** replaces 7 separate
module binaries. The `litho` binary detects its invocation name via `argv[0]` and
dispatches accordingly — `./validate` runs all 7 modules, `./verify` checks BLAKE3
integrity, `./refresh` fetches updated data.

---

## The Seven Science Modules

| # | Module | Paper | Checks |
|---|--------|-------|:------:|
| 1 | ltee-fitness | Wiser 2013, Science — power-law fitness | 8/8 |
| 2 | ltee-mutations | Barrick 2009, Nature — mutation accumulation | 7/7 |
| 3 | ltee-alleles | Good 2017, Nature — allele trajectories | 20/20 |
| 4 | ltee-citrate | Blount 2008/2012, PNAS/Nature — citrate innovation | 11/11 |
| 5 | ltee-biobricks | Burden 2024, Nat Comms — BioBrick metabolic burden | 6/6 |
| 6 | ltee-breseq | Tenaillon 2016, Nature — 264 genomes | 16/16 |
| 7 | ltee-anderson | Anderson-QS framework — disorder predictions | 7/7 |

**75/75 checks passing** at Tier 2 (Rust). Each module calls `lib::run_validation()`
in-process — no subprocess spawning, no shell scripts.

---

## Cross-Platform Validation

| Platform | Binary | Size | Result |
|----------|--------|------|--------|
| Ubuntu 24.04 airgapped VM | musl-static | 5.1 MB | **75/75 PASS** |
| Ubuntu 24.04 VPS VM | musl-static | 5.1 MB | **PASS** + liveSpore provenance |
| Alpine 3.20 chroot | musl-static | 5.1 MB | **PASS** — musl libc portability |
| Read-only filesystem | musl-static | 5.1 MB | **PASS** — graceful degradation |
| Windows x86_64 | litho.exe (mingw-w64) | 7.9 MB | **PASS** via Wine 11 |

The musl-static binary has **zero runtime dependencies** — no libc, no Python, no
containers. The Windows binary runs natively or through WSL2.

---

## Three Operating Modes

| Mode | Network | Discovery | Tier |
|------|---------|-----------|------|
| **Standalone** | None | No primals — bundled data only | 1–2 |
| **LAN** | Local | env vars / UDS socket — primal IPC | 2 |
| **Geo-delocalized** | Remote | Songbird TURN relay | 2 |

The operating mode is detected automatically by `probe_operating_mode()` and recorded
in `liveSpore.json` for provenance.

---

## How to Verify

```bash
# From USB or tarball
cd /media/lithoSpore
./validate                    # Run all 7 modules (Tier 2)
./verify                      # BLAKE3 data integrity check
./validate --json             # Machine-readable output

# From source
cargo run --bin litho -- validate --json

# Self-test (artifact integrity)
bin/litho self-test

# Chaos testing (fault injection)
bin/litho chaos-test
```

---

## The Hypogeal Cotyledon

lithoSpore is classified as a **hypogeal cotyledon** — a seed leaf that stays
underground, nourishing the seedling until it can photosynthesize. The USB's
bundled data and runtime are the cotyledon: persistent, not consumed, providing
sustenance until the spore connects to {{ entity(name="nucleus") }} for full
compute capability.

| Spore Class | Self-Sufficient | Provenance |
|-------------|:---------------:|:----------:|
| ColdSpore | No | None |
| LiveSpore | Partial | liveSpore.json |
| **lithoSpore** | **Yes** | liveSpore.json + BLAKE3 + scope graph |

---

## Get the Artifact

**Repository**: [sporeGarden/lithoSpore](https://github.com/sporeGarden/lithoSpore)

| Delivery | Command |
|----------|---------|
| USB drive | Plug in, `./validate` |
| Tarball | `tar xf lithoSpore.tar.gz && cd lithoSpore && ./validate` |
| Container | `docker run lithospore validate` |
| From source | `cargo run --bin litho -- validate` |
| Windows | `litho.exe validate` (native) or WSL2 |

The artifact carries 16 DOIs, 7 data bundles, pre-rendered notebooks, and
publication-quality SVG figures. See `papers/READING_ORDER.md` for the
guided reading path through the LTEE literature.
