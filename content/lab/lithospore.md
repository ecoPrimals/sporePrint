+++
title = "lithoSpore — LTEE Reproduction and Portable Validation"
description = "7 LTEE modules reproducing Wiser, Barrick, Good, Blount, Burden, Tenaillon papers + Anderson disorder predictions. 75/75 checks, USB-deployable, cross-platform."
date = 2026-05-16
weight = 1

[taxonomies]
primals = ["barracuda", "toadstool", "nestgate", "biomeos", "petaltongue"]
springs = ["groundspring", "wetspring", "hotspring", "neuralspring"]
+++

## Domain

Long-Term Evolution Experiment (LTEE) reproduction — power-law fitness, mutation accumulation, allele trajectories, citrate innovation, BioBrick burden, 264-genome comparison, Anderson disorder predictions.

**Repository**: [sporeGarden/lithoSpore](https://github.com/sporeGarden/lithoSpore)

## The Science Story

lithoSpore is the ecosystem's first **Targeted GuideStone** — a self-contained artifact that reproduces key results from the LTEE, the longest-running evolutionary biology experiment in history. It targets the Barrick Lab at UT Austin, where the experiment continues past 75,000 generations with *E. coli*.

The artifact proves two things simultaneously: (1) the LTEE's published results are computationally reproducible from public data, and (2) the ecoPrimals infrastructure — pure Rust, musl-static, zero dependencies — can carry scientific validation to any machine on the planet.

## Headline Results

- **75/75 checks** across 7 science modules — all PASS at Tier 2 (Rust)
- **6 published papers reproduced**: Wiser 2013, Barrick 2009, Good 2017, Blount 2008/2012, Burden 2024, Tenaillon 2016
- **Anderson disorder framework** applied to LTEE fitness data — GOE/Poisson eigenvalue statistics validate the disorder analogy
- **Single 5.1 MB binary** (musl-static) replaces 7 module binaries + 11 shell scripts
- **Cross-platform**: Ubuntu, Alpine, Fedora, Debian, read-only FS, Windows (7.9 MB litho.exe)
- **108 unit tests + 16 integration tests + 15 chaos/fault-injection tests**

## Seven Science Modules

| # | Module | Paper | Method | Checks |
|---|--------|-------|--------|:------:|
| 1 | ltee-fitness | Wiser 2013 (Science) | Nelder-Mead curve fitting, AIC/BIC model selection | 8/8 |
| 2 | ltee-mutations | Barrick 2009 (Nature) | Kimura fixation, Poisson neutral accumulation, Pearson molecular clock | 7/7 |
| 3 | ltee-alleles | Good 2017 (Nature) | Clonal interference dynamics, fixation probability, adaptation rate | 20/20 |
| 4 | ltee-citrate | Blount 2008/2012 (PNAS/Nature) | Citrate innovation cascade, potentiation, replay probabilities | 11/11 |
| 5 | ltee-biobricks | Burden 2024 (Nat Comms) | Metabolic burden, cost-benefit analysis | 6/6 |
| 6 | ltee-breseq | Tenaillon 2016 (Nature) | 264-genome comparison, parallel evolution significance | 16/16 |
| 7 | ltee-anderson | Anderson-QS framework | GOE/Poisson eigenvalue statistics, disorder mapping | 7/7 |

## Researchers Reproduced

| Researcher | Department | Domain |
|------------|-----------|--------|
| Jeffrey Barrick | Molecular Biosciences, UT Austin | LTEE continuation, mutation dynamics |
| Richard Lenski | BEACON Center, MSU | LTEE founder, evolutionary biology |
| Michael Wiser | BEACON Center, MSU | Power-law fitness models |
| Benjamin Good | Physics, Stanford | Allele dynamics, clonal interference |
| Zachary Blount | BEACON Center, MSU | Citrate innovation |
| Olivier Tenaillon | INSERM, Paris | Genomic evolution |

## What the Constraint Revealed

Building a single-binary artifact forced the consolidation of 7 module binaries into one `litho` CLI with `argv[0]` symlink detection. This eliminated subprocess spawning entirely — every module calls `lib::run_validation()` in-process. The same constraint demanded replacing all 11 shell scripts (fetch, assemble, chaos-test, deploy-test, validate) with pure Rust implementations using `ureq`, `walkdir`, `blake3`, and `chrono`.

External command calls (`date`, `hostname`, `id -u`) were replaced with filesystem reads and library calls, enabling genuine cross-platform support including Windows.

## Architecture

```
litho-core          ← shared library
  ├── validation/     tolerance framework, named tolerances
  ├── provenance/     liveSpore tracking, BLAKE3 anchoring
  ├── discovery/      env → UDS → TURN → standalone
  ├── stats/          shared statistics (pearson_r)
  ├── harness/        module skip/load/dispatch helpers
  └── viz/            petalTongue DataBinding adapters
  ↑
  ├── ltee-fitness   ← Module 1 (lib.rs + thin main.rs)
  ├── ltee-mutations ← Module 2
  ├── ltee-alleles   ← Module 3
  ├── ltee-citrate   ← Module 4
  ├── ltee-biobricks ← Module 5
  ├── ltee-breseq    ← Module 6
  ├── ltee-anderson  ← Module 7
  └── ltee-cli       ← 13 subcommands (validate, verify, fetch, assemble, ...)
```

`#![forbid(unsafe_code)]` enforced workspace-wide across all 9 crates.

## Cross-Spring Connections

- **← {{ entity(name="groundspring") }}**: B1-B4 statistical methods — model fitting, fixation probability, AIC/BIC
- **← {{ entity(name="wetspring") }}**: B7 breseq genome analysis — 264-genome parallel evolution
- **← {{ entity(name="hotspring") }}**: Anderson localization spectral primitives — GOE/Poisson statistics
- **← {{ entity(name="neuralspring") }}**: ML surrogate enrichment (additive, not blocking)
- **→ {{ entity(name="petaltongue") }}**: DataBinding adapters for all 7 modules, Interactive SceneGraph IPC
- **→ Foundation**: Threads 1, 2, 4, 7 — validated data sources and quantitative targets

## Deployment Testing

| Environment | Method | Result |
|------------|--------|--------|
| Ubuntu airgapped VM | libvirt + cloud-init | **PASS** — 75/75 checks |
| Ubuntu VPS VM | libvirt + cloud-init | **PASS** + liveSpore provenance |
| Alpine 3.20 chroot | chroot on host | **PASS** — musl libc portability |
| Read-only filesystem | remount ro | **PASS** — graceful degradation |
| Windows x86_64 | Wine 11 | **PASS** — self-test + validate + verify |

See [lithoSpore guideStone Artifact](@/guidestone/lithospore_artifact.md) for full artifact documentation.

## baseCamp Papers

Paper 02 (Frozen Fossil Record, LTEE extension) — see [baseCamp Science](/science/) for full list.
