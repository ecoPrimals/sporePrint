+++
title = "ludoSpring Game Science"
description = "Game science and interactive systems — HCI laws, procedural generation, and mathematical game theory"
date = 2026-07-18
template = "spore_gallery.html"

[taxonomies]
springs = ["ludospring"]

[extra]
domain = "Game Science & Interactive Systems"
spore_name = "ludoSpring-Game-Science"
spore_version = "1.0.0"
spore_origin = "ecoPrimals/springs/ludoSpring"
spore_spring = "ludoSpring"
spore_status = "PENDING"
modules_pass = 0
modules_total = 0
methods = ["Fitts law", "Hick-Hyman law", "steering law", "GOMS modeling", "Perlin noise", "wave function collapse", "L-systems"]
tools = ["Rust (game engine)", "Python (baselines)", "WGSL (GPU noise)"]
+++

## Domain Profile

Game science and interactive systems validation. Covers HCI motor laws
(Fitts, Hick-Hyman, steering), GOMS cognitive modeling, procedural generation
(Perlin noise, WFC, L-systems, BSP dungeon layout), MDA/Schell design lenses,
and RPGPT statistical planes.

**Status:** pseudoSpore v1.0.0 emitted (61 KB, 50 files). Module validation
pending — golden baseline values from Python/Rust parity tests included.
995 workspace tests (requires rustc 1.92).

## Module Status

| # | Module | Description | Status |
|---|--------|-------------|--------|
| 1 | HCI Motor Laws | Fitts/Hick/Steering law validation | PENDING |
| 2 | GOMS Modeling | Cognitive task analysis predictions | PENDING |
| 3 | Procedural Gen | Perlin, WFC, L-system, BSP parity | PENDING |
| 4 | MDA Analysis | Mechanics-Dynamics-Aesthetics framework | PENDING |
| 5 | Schell Lenses | 100+ design lens evaluation | PENDING |
| 6 | RPGPT Planes | Statistical distribution validation | PENDING |

**0 of 6 modules validated.** Awaiting rustc 1.92 toolchain for spring tests.

## Provenance

| Property | Value |
|----------|-------|
| Origin | `ecoPrimals/springs/ludoSpring` |
| Version | 1.0.0 |
| Spring | ludoSpring |
| Emission method | `litho emit-pseudospore` |
| Integrity | BLAKE3 checksums in `receipts/checksums.blake3` (39 entries) |
| Braid | FermentBraid provenance chain (who/what/when/how) |

## Download

**Archive:** `pseudoSpore_ludoSpring-Game-Science_v1.0.0.tar.gz` (61 KB)
**Verify:** `litho ingest-pseudospore <path> --verify`
