+++
title = "Discovery Log — Capability Hunting Methodology"
description = "How the ecosystem discovers capabilities through probing hardware rather than reading documentation — the empirical method behind constrained evolution."
date = 2026-03-10
weight = 45

[taxonomies]
trails = ["first-visit"]

[extra]
foundation = true
domain = "Architecture"
maturity = "reproduced"

[[extra.companions]]
url = "/architecture/silicon-deism/"
title = "Silicon Deism"
relation = "extends"
label = "Empirical capability hunting as silicon deism in practice"

[[extra.companions]]
url = "/products/coralforge/"
title = "coralForge"
relation = "validates"
label = "f64 discovery enabling structure prediction"

[[extra.companions]]
url = "/products/nautilus/"
title = "nautilus"
relation = "validates"
label = "NPU export discovery enabling neuromorphic deployment"
+++

## The Methodology

Capability hunting: probe the hardware for unadvertised capabilities rather
than accepting the vendor SDK's documented boundaries. The vendor tells you
what the hardware is *sold as*. Probing tells you what the hardware *can do*.

---

## Key Discoveries

### f64 on Consumer GPUs (the coralReef catalyst)

The official story: consumer GPUs have 1:64 f64-to-f32 throughput ratio
(NVIDIA marketing). Running double precision on a consumer GPU is "not
practical."

The discovery: consumer GPUs expose native f64 computation at 1:2 ratio
via Vulkan `SHADER_F64` — not through CUDA, but through the Vulkan compute
pipeline. NVIDIA's 1:64 marketing refers to the CUDA path. The Vulkan path
has different characteristics.

**What this produced**: {{ entity(name="coralreef") }} was promoted from a
{{ entity(name="toadstool") }} subsystem to an independent primal specifically because
f64 shader compilation proved to be a distinct capability domain requiring
its own expertise: transcendental lowering (`lower_f64`), precision
verification, and cross-vendor shader validation.

### NPU as Real Hardware (the akida driver)

The official story: BrainChip's AKD1000 NPU requires their Python SDK.

The discovery: the AKD1000 speaks a wire protocol over PCIe that can be
driven by a Pure Rust userspace driver. {{ entity(name="toadstool") }}'s `akida-driver`
validates this — 300K inferences/second at ~1 mW, driven by Rust, no
Python, no vendor SDK.

### DF64 Emulation

When native f64 is not available (some mobile GPUs, some Intel Arc configs),
{{ entity(name="barracuda") }} falls back to DF64 — double-float emulation using pairs
of f32 values. This provides f64 precision on hardware that only supports f32,
at approximately 4x the f32 throughput cost.

**What this produced**: DF64 shaders across all 10 scientific domains in
{{ entity(name="barracuda") }}, enabling f64 physics on any GPU with f32 support.

---

## The Timeline

| Period | What Happened | Primal Impact |
|--------|--------------|---------------|
| Months 1-8 | 14 primals built through K-NOME | Core architecture |
| Month 9 | ToadStool matures, barraCuda + coralReef promoted | Sovereign compute pipeline |
| Months 9-10 | Springs launch (2,882 checks on single RTX 4070) | Validation framework |
| Month 11+ | Multi-gate NUCLEUS, spring deepening | Production deployment |

---

## Capability Hunting vs. SDK Consumption

| SDK Consumption | Capability Hunting |
|----------------|--------------------|
| Read the docs | Probe the hardware |
| Install the toolchain | Write a driver |
| Accept the boundary | Test the boundary |
| Pay for the pro version | Discover the free capability |
| "Consumer GPUs can't do f64" | "Consumer GPUs can do f64 via Vulkan" |
| "NPU requires Python SDK" | "NPU speaks a wire protocol" |

The capability hunting methodology is empirical: you do not accept the
marketed boundary until you have tested it yourself. This is the same
methodology that drives the springs — you do not accept a published result
until you have reproduced it computationally.

---

## Connection to Constrained Evolution

Capability hunting is constrained evolution applied to hardware:

- **Constraint**: Pure Rust, no vendor SDK, no C/C++ FFI
- **Variation**: Probe different hardware interfaces (Vulkan, VFIO, PCIe)
- **Selection**: Keep what works (f64 via Vulkan), discard what does not
  (CUDA path, vendor Python SDK)
- **Inheritance**: Discoveries propagate through the ecosystem (coralReef
  promotion, akida driver, DF64 shaders)

The hardware does not change. The constraint changes what you discover
about it.

---

*The vendor tells you what the hardware is for. Capability hunting tells
you what the hardware can do. The difference is the space between
marketing and physics — and in that space, sovereign computation lives.*
