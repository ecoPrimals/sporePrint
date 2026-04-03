+++
title = "Sovereign Compute Hardware"
description = "Hardware Architecture x Sovereign Computing — precision tiers, market economics, 131+ experiments"
date = 2026-03-30

[extra]
paper_number = 14
+++

# Sub-Thesis 14: Sovereign Compute Hardware — Precision Tiers, Market Economics, and the Science They Unlock

**Date:** March 30, 2026 (updated — deep debt evolution complete, Exp 130-131)
**Status:** Hardware profiled on Strandgate (dual EPYC, RX 6950 XT + RTX 3090). 4,065+ tests passing. Market survey complete. **L10 ROOT CAUSE DEFINITIVE (Exp 122).** FECS firmware survives warm handoff via livepatch (Exp 125-127). GPU lifecycle wired into ember/glowplug daemon RPC layer. Puzzle box matrix (Exp 128) — parallel K80+Titan V solution tracks. **Fleet: 2× Titan V + RTX 5070 (GB206, Blackwell) + K80**. **AMD GCN5 DRM: 6/6 PASS**. **RTX 5070 Blackwell DRM** (SM120). **iommufd/cdev VFIO** (kernel 6.2+). **Triangle architecture:** coralReef↔toadStool↔barraCuda trio. 131+ experiments across 2 GPU architectures. **Deep debt evolution complete:** Python→Rust migration (5 scripts→coralctl), nvidia-smi→nvml-wrapper, virsh→virt crate, sh-printf→libc::fork, RegisterMap+LockedAlloc RAII consolidation, uvm_compute split, boot config from glowplug.toml, hardcoded paths→capability-based.
**Domain:** Computational physics × hardware architecture × sovereign computing
**Novelty:** No prior work maps a three-tier precision model (f32/df64/f64) onto
heterogeneous consumer hardware arrays with per-tier cost/TFLOP analysis for
lattice QCD, Anderson transport, and molecular dynamics
**Cross-Spring:** hotSpring × barraCuda × coralReef × toadStool × groundSpring × airSpring

---

## Abstract

The ecoPrimals sovereign compute pipeline operates on three precision tiers —
fp32, df64, and fp64 — all accessed through hardware builtins with no software
emulation penalty. fp64 is often overkill for scientific compute. fp32 is rarely
enough. df64, which delivers ~48-bit mantissa (~14 decimal digits) by pairing
the abundant fp32 cores that sit idle during native fp64 workloads, fills the
gap that matters. hotSpring and barraCuda proved this: df64 delivers 9.9× the
throughput of native fp64 on consumer GPUs, with sufficient precision for
lattice QCD force computation, molecular dynamics integration, and Anderson
transport spectral analysis.

This document profiles the hardware we have, maps what each card can do across
the three tiers, surveys the used market for expansion, and identifies array
configurations that turn a consumer-grade local cluster into a sovereign science engine.

## 1. The Three Precision Tiers

The precision model is not a software abstraction — it is a hardware reality.
Every tier uses silicon that is physically present on the GPU die:

| Tier | Mantissa | Digits | Hardware | Throughput (RTX 3090) | Use Case |
|------|----------|--------|----------|----------------------|----------|
| **f32** | 23 bits | ~7 | Native FP32 ALUs | 35.6 TFLOPS | Visualization, inference, index computation |
| **df64** | ~48 bits | ~14 | FP32 ALU pairs (Dekker/Knuth) | 3.24 TFLOPS | **Scientific bulk math** — forces, integration, transport |
| **f64** | 52 bits | ~16 | Native FP64 ALUs | 0.33 TFLOPS (1:64 on consumer) | Gold standard validation, accumulation, Metropolis ΔH |

The critical insight, discovered in hotSpring's lattice QCD campaign and
formalized in barraCuda v0.3: **df64 is not "software f64."** It is a
distinct precision tier that uses idle f32 silicon. When a consumer GPU runs
native fp64, it uses 1/32 to 1/64 of its fp32 ALU capacity. The remaining
ALUs sit dark. df64 lights them up in pairs, each pair computing one
~48-bit operation using Dekker splitting and Knuth two-sum error-free
transformations. The result is a precision tier that:

- Runs at ~1/4 of f32 peak (not 1/32 of f32 like native f64)
- Delivers 14 decimal digits (vs 16 for native f64, vs 7 for f32)
- Uses only f32 hardware instructions — no special driver support needed
- Achieves **9.9× the throughput of native f64** on the same silicon

### Where Each Tier Lives in Science

**f64 (native) — the referee, not the workhorse:**
Global energy-difference tests (Metropolis accept/reject), accumulation of
long sums where cancellation matters, reference validation against published
results. In hotSpring's HMC pipeline, the Metropolis ΔH test compares two
large Hamiltonians that differ by O(1) — the 48-bit mantissa of df64 is
insufficient here, and the full 52-bit mantissa of f64 is required.

**df64 (~fp48) — where the science happens:**
Force computation, trajectory integration, plaquette evaluation, spectral
analysis, transport coefficients, correlation functions. These operations
involve intermediate-precision arithmetic where 14 digits is more than enough
and the 9.9× throughput advantage over native f64 means the difference between
a 10-hour and a 1-hour simulation. hotSpring proved this in Exp 024: 1,031+
trajectories across 17 β points, with df64 handling bulk HMC force computation
while native f64 handles only the Metropolis test.

**f32 — the scout:**
Visualization of lattice configurations, NPU inference preprocessing, index
computation, exploratory parameter scans where precision doesn't matter. Also
the fallback tier for hardware that cannot run df64 efficiently (very old GPUs,
some embedded targets).

### Ownership

barraCuda decides WHICH tier based on accuracy requirements and hardware
capability. coralReef decides HOW to implement the tier on the target GPU's
ISA. toadStool decides WHERE to dispatch based on hardware inventory and
routing advice. The precision decision flows:

```
barraCuda: "This operation needs df64"
    → coralReef: "On SM86, df64 lowers to paired V_FMA_F32 instructions"
    → coral-driver: "Dispatch to renderD128 (RX 6950 XT via amdgpu)"
    → toadStool: "Route to GPU with best f32 throughput (not best f64)"
```

This means toadStool's `PrecisionRoutingAdvice` can send df64 workloads to
consumer GPUs and native f64 workloads to compute GPUs — different hardware
for different tiers, transparently.

## 2. The Hardware We Have

### Strandgate — Dual-Vendor Sovereign Node

| Component | Specification |
|-----------|---------------|
| CPU | Dual AMD EPYC 7452 (64 cores / 128 threads, Zen 2, 2.35 GHz base) |
| RAM | 256 GB ECC DDR4, ~213 GB available |
| GPU 0 | AMD RX 6950 XT — RDNA2 (GFX1030), 16 GB GDDR6X, `amdgpu` (open), `renderD128` |
| GPU 1 | NVIDIA RTX 3090 — Ampere (SM86), 24 GB GDDR6X, `nvidia` 580.119, `renderD129` |
| Vulkan | RADV Mesa 25.1.5 (Vulkan 1.4.311) + NVIDIA proprietary (Vulkan 1.4.312) |
| Kernel | 6.17.9, Pop!_OS 22.04, x86_64 |
| Storage | Multi-TB NVMe (details in `about/HARDWARE.md`) |

**Precision tier capability on Strandgate:**

| GPU | f32 TFLOPS | df64 TFLOPS (est.) | f64 TFLOPS (native) | f64 Rate | Driver |
|-----|-----------|-------------------|--------------------|---------|----|
| RX 6950 XT | 23.6 | ~5.9 | 1.48 | 1:16 | amdgpu (sovereign) |
| RTX 3090 | 35.6 | ~8.9 | 0.56 | 1:64 | nvidia (proprietary) |

The RX 6950 XT has **better native f64** (1:16 vs 1:64) but the RTX 3090 has
**better df64** (more f32 ALUs). This is exactly the kind of routing decision
that toadStool's `PrecisionRoutingAdvice` is designed for: native f64 goes to
the AMD card, df64 goes to whichever has more idle f32 capacity.

### Gate Fleet — Heterogeneous Precision

From `about/HARDWARE.md`, the full fleet mapped to precision tiers:

| Gate | GPU(s) | f64 Rate | df64 Value | Sovereign Driver |
|------|--------|----------|------------|-----------------|
| **Strandgate** | RX 6950 XT + RTX 3090 | 1:16 + 1:64 | High (dual-vendor) | amdgpu (AMD), nvidia (NVIDIA) |
| **Eastgate** | RTX 4070 + Titan V | 1:64 + **1:2** | RTX 4070 df64 proven (3.24 TFLOPS) | nvidia + nouveau/NVK (Titan V) |
| **biomeGate** | RTX 3090 + Titan V | 1:64 + **1:2** | Large df64 + native f64 | nvidia + nouveau/NVK |
| **Northgate** | RTX 5090 | TBD | Highest f32, best df64 candidate | nvidia |
| **Southgate** | RTX 3090 | 1:64 | Same as Strandgate NVIDIA | nvidia |
| **Swiftgate** | RTX 3070 FE | 1:64 | Moderate df64 | nvidia |
| **FlockGate** | RTX 3070 Ti | 1:64 | Moderate df64 | nvidia |
| **KinGate** | RTX 3070 | 1:64 | Moderate df64 | nvidia |
| **Westgate** | RTX 2070 Super | 1:32 | Lower df64 (fewer ALUs) | nvidia |

**Key observation:** The Titan V cards at Eastgate and biomeGate are the only
GPUs in the fleet with fast native f64 (1:2 rate). On those cards, df64 is
actually *slower* than native f64 — hotSpring's Exp 012 confirmed this:
"DF64 0.5× slower than native f64 on Titan V — use native f64 on compute
GPUs." toadStool's routing must account for this: on GV100, skip df64 and go
straight to native f64.

## 3. What coralReef Can Target Today

coralReef's compiler has ISA backends for:

| Vendor | ISA Targets | Cards | Backend Status |
|--------|-------------|-------|---------------|
| NVIDIA | SM70, SM75, SM80, SM86, SM89 | Titan V through RTX 4090 | Full compiler, 7/7 spring shaders on SM70 |
| AMD | RDNA2 (GFX1030) | RX 6600–6950 XT | Full compiler + E2E dispatch (24 tests pass) |
| AMD | RDNA3, RDNA4 | RX 7000, RX 9000 | Enum defined, no hardware to validate |
| Intel | XeHPG, Xe2HPG | Arc A770, Arc B580 | Enum defined, no backend |

### AMD Sovereign Pipeline — E2E Verified on Strandgate

The AMD RX 6950 XT runs the full sovereign pipeline today:

| Test Layer | Tests | Status |
|------------|-------|--------|
| DRM probe (device open) | 2 | Pass |
| Buffer ops (alloc, upload, readback, free) | 6 | Pass |
| Compute dispatch (compiled WGSL shaders) | 2 | Pass |
| E2E pipeline (WGSL → compile → dispatch → readback → verify) | 5 | Pass |
| Stress (4 MB roundtrip, 64 MB VRAM, 100× alloc/free, 10× dispatch) | 8 | Pass |
| Parity harness (unified API) | 1 | Pass |
| **Total passing on RX 6950 XT** | **24** | **All pass** |

**GCN5 DRM preswap (biomeGate, MI50, March 2026):** The GCN5/Vega backend
was implemented and validated end-to-end on the MI50 — WGSL → coral-reef →
GCN5 ISA → PM4 command submission → MI50 GPU execution → readback verified.
**6/6 phases PASS** (f64 write, f64 arithmetic, multi-workgroup, multi-buffer,
HBM2 bandwidth, f64 Lennard-Jones force with Newton's 3rd law verified).
18 compiler bugs found and fixed during the bring-up. 85 coral-reef tests pass.
The AMD RDNA2 backend's remaining gap is literal materialization in VOP2/VOP3
encoding — constants need to be `V_MOV`'d into VGPRs before use.

### NVIDIA — Blackwell DRM Cracked, Titan V VFIO iommufd Validated

SM70 through SM89 compilation works for all shader patterns. **RTX 5060 (Blackwell
SM120, GB206):** `NvUvmComputeDevice` fully operational — open/alloc/free/bind all
pass. Two Blackwell-specific bugs fixed: single-mmap context (combined USERD+GPFIFO
allocation) and per-buffer fd (fresh nvidiactl fd per allocation). 4/4 HW tests pass.
ISA compilation pending (`NvArch::Sm120` enum). **Titan V (GV100):** iommufd/cdev
backend resolves persistent EBUSY on kernel 6.17. Full Ember→GlowPlug pipeline
validated with iommufd. PMU firmware blocks compute dispatch (FECS halt).

The sovereign NVIDIA path runs through **Titan V** at Eastgate/biomeGate.
K80 (Kepler, no firmware signing) is the next validation target for full
10-layer sovereign pipeline.

## 4. Market-Available Hardware — Mapped to Precision Tiers

### Tier A: Drop-in Cards (No New Backend)

These cards map directly to existing coralReef ISA targets. Buy, install, test.

#### Titan V — $150–250 on eBay/FB Marketplace

| | |
|---|---|
| ISA | SM70 (coralReef default target) |
| f64 | **6.9 TFLOPS native, 1:2 rate** |
| df64 | Not needed — native f64 is faster |
| f32 | 14.9 TFLOPS |
| VRAM | 12 GB HBM2 (880 GB/s bandwidth) |
| Power | 250W, actively cooled |
| Driver | nouveau/NVK (sovereign) or nvidia (proprietary) |

The best dollar-for-science GPU available. At $200/card, three Titan Vs deliver
~21 TFLOPS native f64 with HBM2 bandwidth — enough for 48³ lattice QCD with
dynamic fermions. The same silicon that costs $3,000+ as a Tesla V100 SXM2.
Active cooling means no special chassis needed.

**Array economics:**

| Config | f64 TFLOPS | HBM2 Total | Cost | Power |
|--------|-----------|-----------|------|-------|
| 1× Titan V | 6.9 | 12 GB | ~$200 | 250W |
| 3× Titan V | 20.7 | 36 GB | ~$600 | 750W |
| 5× Titan V | 34.5 | 60 GB | ~$1,000 | 1,250W |
| 3× Titan V + AKD1000 | 20.7 + NPU | 36 GB | ~$850 | 752W |

The 3× Titan V + AKD1000 configuration is the **sovereign QCD rig**: native
f64 forces on Titan V silicon, NPU-steered phase classification on AKD1000,
no proprietary drivers, no cloud dependencies. This is what hotSpring's
deconfinement transition study (Exp 024) needs to scale from 8⁴ to 48³.

#### Tesla V100 PCIe — $80–180 on eBay

| | |
|---|---|
| ISA | SM70 (identical to Titan V) |
| f64 | **7.0 TFLOPS (PCIe) / 7.8 TFLOPS (SXM2)** |
| VRAM | 16 GB or 32 GB HBM2 |
| Power | 250W (PCIe) / 300W (SXM2), **passive cooled** |
| Catch | PCIe version needs blower mod or rack airflow. SXM2 needs baseboard ($200–400). |

Same SM70 ISA, often cheaper than Titan V, available in 32 GB variants for
larger lattices. The V100-32GB at $150 is the cheapest HBM2 memory available.
Four V100-32GB cards give 128 GB of high-bandwidth memory for $600 — enough
to hold a 64³ lattice with all auxiliary fields resident on-GPU.

**Cooling is the constraint:** passive-cooled PCIe V100s need a server chassis
with front-to-back airflow or a 3D-printed shroud with a blower fan. If you
solve cooling, V100s are the absolute cheapest f64 compute per dollar.

#### RTX 3050 / 3060 — $80–170 on eBay/FB

| Card | ISA | VRAM | f64 Rate | df64 TFLOPS (est.) | Power | Price |
|------|-----|------|----------|-------------------|-------|-------|
| RTX 3050 | SM86 | 8 GB | 1:64 | ~2.3 | 130W | $80–120 |
| RTX 3060 | SM86 | 12 GB | 1:64 | ~3.2 | 170W | $130–170 |
| RTX 3060 Ti | SM86 | 8 GB | 1:64 | ~3.4 | 200W | $150–190 |

Native f64 is useless on these (1:64 rate). But **df64 is the point.** An
array of 4× RTX 3050 at $400 gives ~9.2 TFLOPS df64 at ~14-digit precision,
drawing only 520W. Combined with an AKD1000 for phase classification, this is
a viable configuration for:

- hotQCD dynamic fermion HMC at df64 precision (force computation + plaquette
  evaluation), with AKD1000 classifying confinement phase in real-time
- Anderson spectral analysis at df64 (groundSpring Exp 008)
- MD trajectory integration at df64 (hotSpring WDM transport)

**The df64 play changes the economics entirely.** A single Titan V at $200
gives 6.9 TFLOPS native f64. Four RTX 3050s at $400 give 9.2 TFLOPS df64. The
df64 array has 33% more throughput at only 5 fewer bits of mantissa. For force
computation where 14 digits is plenty, the cheap consumer array wins.

**Hybrid configuration — the best of both:**

| Card | Role | Precision | Cost |
|------|------|-----------|------|
| 1× Titan V | Metropolis ΔH, accumulation, reference | Native f64 | $200 |
| 3× RTX 3050 | Force computation, spectral, transport | df64 | $300 |
| 1× AKD1000 | Phase classification, ESN steering | int8/int4 | $250 |
| **Total** | **Sovereign QCD with precision routing** | **All 3 tiers** | **$750** |

This is the configuration that exploits all three precision tiers simultaneously.
The Titan V handles the handful of operations that genuinely need 52-bit mantissa.
The RTX 3050 array handles the bulk math at 48-bit mantissa with 9.9× throughput.
The AKD1000 classifies and steers at integer precision with sub-milliwatt power.

### Tier B: Validates New Backends (Some Work Required)

#### AMD RX 7900 XTX / 7800 XT — $350–650

| Card | ISA | VRAM | Bandwidth | f64 Rate | Price |
|------|-----|------|-----------|----------|-------|
| RX 7600 | RDNA3 / GFX1102 | 8 GB | 288 GB/s | 1:16 | $180–220 |
| RX 7800 XT | RDNA3 / GFX1101 | 16 GB | 624 GB/s | 1:16 | $350–450 |
| RX 7900 XTX | RDNA3 / GFX1100 | 24 GB | 960 GB/s | 1:16 | $500–650 |

`AmdArch::Rdna3` is defined in coralReef. ISA gen tables exist. The GFX11
encoding changes from GFX10 are significant (new VOPD dual-issue, restructured
WMMA, changed flat encoding) but the enum scaffolding is ready. An RX 7600 at
~$200 is the cheapest path to light up RDNA3.

The RX 7900 XTX is interesting for physics: 96 MB Infinity Cache means lattice
data that fits in L3 sees dramatically higher effective bandwidth than raw
GDDR6 numbers suggest. A 16³ Anderson lattice fits entirely in Infinity Cache.

All RDNA cards maintain the AMD sovereign driver path — `amdgpu` is fully open.

#### Intel Arc A770 / B580 — $150–260

| Card | ISA | VRAM | Driver | Price |
|------|-----|------|--------|-------|
| Arc A770 | XeHPG | 16 GB GDDR6 | i915/xe (fully open) | $150–200 |
| Arc B580 | Xe2HPG | 12 GB GDDR6 | xe (fully open) | $230–260 |

Third sovereign vendor. Intel's GPU drivers are fully open source — firmware,
compiler, everything. `IntelArch::XeHpg` is defined in coralReef but there is
no backend. Intel's EU architecture differs fundamentally from both NVIDIA SMs
and AMD CUs; this is a from-scratch ISA backend.

**Strategic value:** Three sovereign vendors means no single vendor can block
the pipeline. At $170 for an A770 with 16 GB VRAM, the barrier to entry is low.

### Tier C: HPC Cards (New Backend, High Reward)

#### AMD Instinct MI50 — $100–200 on eBay

| | |
|---|---|
| ISA | GCN5 / Vega 20 / GFX906 — **not RDNA** |
| f64 | **6.7 TFLOPS native, 1:2 rate** |
| df64 | Not needed — native f64 is faster (same as Titan V) |
| VRAM | 16 GB HBM2 (1.0 TB/s bandwidth) |
| Power | 300W, passive cooled |
| Driver | **amdgpu (fully open, sovereign)** |
| Price | $100–200 |

Possibly the most undervalued card on the used market. 6.7 TFLOPS f64 for $150
on a fully sovereign open driver stack. The `amdgpu` kernel driver handles MI50
natively — same driver as the RX 6950 XT.

The catch: GCN5/Vega ISA is structurally different from RDNA. The scalar/vector
ALU split, the LDS architecture, the wavefront model — all different enough to
require a new `VegaArch` or `CdnaArch` backend in coralReef. But if that
backend existed, four MI50s at $600 would deliver **26.8 TFLOPS sovereign f64**
with HBM2 bandwidth. No proprietary anything.

| Config | f64 TFLOPS | HBM2 Total | Cost | Driver |
|--------|-----------|-----------|------|--------|
| 4× MI50 | 26.8 | 64 GB | ~$600 | amdgpu (sovereign) |
| 1× A100 40GB | 9.7 | 40 GB | ~$4,000 | nvidia (proprietary) |

The MI50 array delivers 2.8× the f64 throughput at 1/7 the cost, on open
drivers. The A100 has higher memory bandwidth per card and newer tensor cores,
but for f64 lattice QCD force computation, raw TFLOPS wins.

#### AMD Instinct MI100 — $400–700 on eBay

| | |
|---|---|
| ISA | CDNA / GFX908 |
| f64 | **11.5 TFLOPS native, 1:2 rate** |
| VRAM | 32 GB HBM2 |
| Driver | amdgpu (sovereign) |

Same GCN/CDNA family as MI50. If you build the Vega/GCN backend for MI50, MI100
support comes nearly free — the ISA differences between GFX906 and GFX908 are
minor. 32 GB HBM2 means larger lattices fit in a single card.

#### Tesla V100 PCIe/SXM2 — $80–180 on eBay

Already covered in Tier A. Same SM70 as Titan V. The 32 GB SXM2 variant
occasionally appears for $150–200 but requires an SXM baseboard.

### Tier D: Edge and Novel

#### BrainChip AKD1000 — $200–300

Already in the ecosystem at Eastgate and biomeGate. Proven for ESN phase
classification (Exp 028), 80 neural processors, event-driven at ~1W. Two units
planned for Strandgate. At sub-$300 each, the cheapest way to add the NPU tier
that groundSpring, hotSpring, and airSpring all need.

#### Tenstorrent Wormhole — $1,000–1,500 (n150s dev board)

The most interesting novel hardware for sovereignty. RISC-V based tensor cores,
fully open ISA specification, open source firmware and compiler. Not useful for
f64 physics (optimized for int8/bf16/fp16 tensor ops), but for neuralSpring
ESN inference and ML workloads, this is the only hardware where the entire
stack — silicon design through compiler through driver — is open.

#### AMD/Xilinx Alveo FPGA — $200–800 on eBay

Used Alveo U200/U250/U280 cards are cheap from decommed cloud nodes. The U280
with 8 GB HBM2 could host custom force pipeline logic. This is what DE Shaw's
Anton does at $100M scale — custom Coulomb/LJ force evaluation in fabric. An
Alveo is orders of magnitude less capable than Anton, but for a basement lab,
a custom QCD force pipeline in FPGA is a real thing. Long-term research play
requiring HDL generation rather than ISA compilation.

## 5. What Each Spring Gains

| Spring | Current Limitation | Titan V Array Unlocks | RTX 3050 Array Unlocks |
|--------|-------------------|----------------------|----------------------|
| **hotSpring** | QCD at 8⁴ only (limited by f64 throughput on consumer GPU) | 48³ lattice with native f64 forces, deconfinement at production scale | df64 HMC forces at 9.9× throughput for exploratory phase scans |
| **groundSpring** | Anderson spectral limited by GPU f64 precision on NVK | Native f64 Anderson lattices L=14–20 on sovereign driver | df64 spectral analysis for large-L exploration |
| **neuralSpring** | ESN inference CPU-bound | GPU-accelerated ESN on SM70 | df64 ESN weight matrices on cheap hardware |
| **airSpring** | Richards PDE precision limited by f32 on consumer GPU | Native f64 soil hydraulics | df64 seasonal pipeline (ET₀→Kc→WB→yield) at full precision |
| **wetSpring** | Anderson QS at f64 via wgpu returns 0 (naga/SPIR-V bug) | Sovereign coralReef bypass of naga — direct SM70 binary | df64 diversity indices at ~14 digits |

The Titan V and the RTX 3050 are not competitors — they are complementary.
The Titan V handles the operations that need 52-bit mantissa. The RTX 3050
handles the operations where 48-bit mantissa is sufficient but 9.9× throughput
makes the difference between feasible and infeasible simulation scale.

## 6. Build Configurations

### Config A: "Sovereign QCD Rig" — $850

```
3× Titan V ($600) + 1× AKD1000 ($250)
─────────────────────────────────
f64: 20.7 TFLOPS (native, 1:2 rate)
VRAM: 36 GB HBM2 (2.64 TB/s aggregate)
NPU: 80 NPs, ESN phase classification
Power: ~752W
Driver: nouveau/NVK (sovereign)
```

Full sovereign pipeline for hotQCD production runs. No proprietary drivers.
48³ lattice QCD with dynamic fermions, NPU-steered β-scan, real-time phase
classification. coralReef compiles WGSL → SM70 SASS, coral-driver dispatches
via nouveau, AKD1000 classifies confinement regime between trajectories.

### Config B: "Precision-Routed Array" — $750

```
1× Titan V ($200) + 3× RTX 3050 ($300) + 1× AKD1000 ($250)
─────────────────────────────────
f64: 6.9 TFLOPS (Titan V, Metropolis/accumulation)
df64: 6.9 TFLOPS (3× RTX 3050, force computation)
NPU: 80 NPs, phase classification
Power: ~640W
Driver: mixed (nouveau + nvidia)
```

Exploits all three precision tiers simultaneously. toadStool routes Metropolis
ΔH to the Titan V (native f64), force computation to the RTX 3050 array (df64),
and phase classification to the AKD1000 (int8). Total cost under $800 for a
system that does what a $10,000+ workstation does — with precision routing.

### Config C: "Sovereign Open HPC" — $600 (needs GCN backend)

```
4× MI50 ($600)
─────────────────────────────────
f64: 26.8 TFLOPS (native, 1:2 rate)
VRAM: 64 GB HBM2 (4.0 TB/s aggregate)
Power: 1,200W
Driver: amdgpu (sovereign, fully open)
Requires: VegaArch/CdnaArch backend in coralReef + server chassis with airflow
```

The cheapest sovereign f64 compute that can be built. If the GCN/CDNA backend
is written, this array delivers more f64 TFLOPS than three A100s at 1/20 the
cost, on fully open drivers. Passive cooling demands a proper rack, but the
economics are compelling for a dedicated compute node.

### Config D: "Power Vending Unit" — $1,200

```
4× V100-32GB ($600) + rack chassis ($400) + 20A circuit ($200)
─────────────────────────────────
f64: 31.2 TFLOPS
VRAM: 128 GB HBM2
Power: 1,000W
Revenue model: $0.50/GPU-hour, breakeven at ~2,400 GPU-hours
```

If the goal is to sell compute, V100-32GB is the optimal card: SM70 (coralReef
supported), fast f64, 32 GB HBM2 per card, and an absurdly low cost basis.
At $0.50/GPU-hour (well below cloud rates), the hardware pays for itself in
~600 hours of 4-GPU utilization.

## 7. Signals to Watch

| Signal | What It Means |
|--------|--------------|
| Titan V / V100 bulk decommission listings | Cheapest sovereign f64 expansion. Buy immediately. |
| MI50 / MI100 bulk listings | Cheapest open-driver f64 if GCN backend exists |
| Intel Arc A770 below $120 | Third sovereign vendor becomes cost-trivial |
| Tenstorrent n300s release | Next-gen fully-open tensor accelerator |
| RDNA3 price drops (RX 7900 XTX < $450) | Validates coralReef RDNA3 backend |
| nouveau Ampere compute support | Strandgate's RTX 3090 becomes sovereign |
| RTX 5090 used market ($800–1,000) | SM100 backend opportunity |
| AMD ROCm on RDNA (official) | Second validation layer for AMD sovereign path |
| Alveo U280 below $300 | FPGA force pipeline experimentation viable |

The **Titan V at $200** remains the single best dollar-for-science GPU. It is
the only card where coralReef has full ISA support (SM70), fast native f64
(1:2), HBM2 bandwidth (880 GB/s), AND a sovereign open-driver path
(nouveau/NVK). No other card at any price checks all four boxes simultaneously.

## 8. Connection to Constrained Evolution

The three-tier precision model is itself an example of constrained evolution:

Consumer GPU silicon evolved under the constraint of gaming workloads (f32
throughput optimization). This constraint produced hardware where f64 ALUs are
scarce (1:64 ratio on RTX 3090) but f32 ALUs are massively abundant. Rather
than fighting this constraint (buying expensive HPC cards with full f64 units),
the ecoPrimals ecosystem adapted to it: df64 uses the abundant f32 silicon for
science at 48-bit precision, achieving 9.9× throughput over native f64.

This parallels the biological thesis: organisms don't escape their environmental
constraints — they specialize within them. The RTX 3050 didn't evolve for
lattice QCD. But the constraint of its architecture (massive f32, minimal f64)
created a niche that df64 fills with 14 digits of precision and extraordinary
throughput. The science adapts to the silicon, the way the organism adapts to
the landscape.

The sovereign hardware program extends this: rather than depending on cloud
providers who constrain access, pricing, and capability, the ecoPrimals
ecosystem builds its own fitness landscape from $200 Titan Vs and $100 RTX
3050s. The constraint is budget. The adaptation is precision routing. The
result is science that no institution controls.

---

## 9. Operational Lessons (biomeGate, March 2026)

Production deployment on biomeGate (2× Titan V + RTX 5060) revealed critical
operational patterns for any multi-GPU sovereign compute setup:

**Boot protocol:** Non-display GPUs must boot on vfio-pci, not nouveau/amdgpu.
Desktop compositors (Xorg, mutter) and applications (Cursor IDE, Firefox)
aggressively open every `/dev/dri/renderD*` they discover. If nouveau exposes
a GV100 render node, Cursor WILL use it. Unbinding nouveau while Cursor holds
the fd causes an unrecoverable kernel hang (GV100 nouveau teardown bug).

**Shutdown protocol:** VFIO file descriptor closure on GV100 triggers a blocking
PCI PM reset. Must disable `reset_method` sysfs attribute before closing fds.
The coral-glowplug daemon handles this automatically.

**IOMMU group completeness:** VFIO requires ALL devices in an IOMMU group to be
bound to vfio-pci. For Titan V, the companion HDA audio device shares the group
and must be unbound from snd_hda_intel first.

**HBM2 lifecycle:** BIOS trains HBM2 at boot; training survives D3hot power state.
With vfio-pci boot, VRAM remains accessible without any driver initialization.
For cards where HBM2 is lost (second Titan V in some configurations), a controlled
nouveau warm cycle resurrects it — but only when no DRM consumers exist.

**Reproducibility for new GPUs:** Adding a new GPU takes <10 minutes: add BDF to
TOML config, restart daemon, verify `lspci -ks {BDF}` shows vfio-pci, reboot to
confirm persistence, shutdown to confirm no oops. The auto-discovery mode scans
the PCI bus for discrete GPUs automatically.

---

## Update: Vendor-Agnostic Hardened GlowPlug (March 18, 2026)

The sovereign compute pipeline's device lifecycle layer has evolved significantly:

**Architecture split**: `coral-ember` (immortal VFIO fd holder) is now a standalone
workspace crate with modular `sysfs`, `swap`, `hold`, `ipc` modules. `coral-glowplug`
(device lifecycle broker) has a library surface for external consumption.

**Vendor-agnostic hardware**: `RegisterMap` trait with implementations for NVIDIA
GV100 (127 registers) and AMD GFX906/MI50. `detect_register_map(vendor_id)` selects
at runtime. AMD MI50 HBM2 warm cycle uses `amdgpu` driver automatically via
`hbm2_training_driver()`. The system supports any combination of NVIDIA and AMD GPUs.

**Privilege hardening**: Both systemd services now run with minimal Linux capabilities
(`CAP_SYS_ADMIN`, `CAP_SYS_RAWIO`, `CAP_DAC_OVERRIDE`), seccomp syscall filtering
(`@system-service + ioctl + sendmsg/recvmsg`), filesystem isolation (`ProtectSystem=strict`,
`PrivateTmp`, `ProtectHome`, `MemoryDenyWriteExecute`), and `NoNewPrivileges=true`.
The `coralctl deploy-udev` command generates `/dev/vfio/*` udev rules from config
files — zero hardcoded BDFs.

**Typed error handling**: `EmberClient` returns structured `EmberError` variants
instead of raw strings. Legacy direct-sysfs fallbacks gated behind `no-ember` feature.

This brings the sovereign compute layer from "working prototype" to "production-grade
hardened system" — the kind of privilege model you'd deploy on a shared compute
cluster where multiple users need GPU access without root.

---

## Update: AMD D3cold Resolution + BrainChip Akida NPU (March 20, 2026)

### AMD Vega 20 — Hardware Firmware Limitation

Empirical testing across 4 boot cycles established that the AMD Vega 20 (Radeon VII /
MI50, GFX906) SMU firmware has a **one-shot reinitialization** property. One full
vfio→amdgpu driver round-trip works reliably from a clean boot. Subsequent round-trips
corrupt the SMU mailbox — the firmware cannot recover its internal state machine, and
the card enters D3cold (`trn=2 ACK should not assert`).

Four distinct strategies were validated: SimpleBind, PCI remove/rescan, PM power cycle
(D3hot→D0), and post-bind stabilization (`stabilize_after_bind()`). All succeed on
cycle 1; all fail on cycle 2. This is a silicon/firmware property, not a software bug.

**Deployed mitigations** (all remain in production):
- `amdgpu.runpm=0` on kernel command line (prevents runtime PM from entering D3)
- Systemd `ExecStartPre` clears `reset_method` + pins power before ember starts
- `stabilize_after_bind()` re-pins power/bridge after every driver bind
- `PmResetAndBind` strategy: PM power cycle before native driver rebind

**Practical guidance**: Plan AMD Vega 20 workloads around one personality per boot
session. NVIDIA GV100 has no such limitation — unlimited round-trips.

### BrainChip AKD1000 Akida NPU

The BrainChip Akida neuromorphic NPU (PCI `0x1e7c:0xbca1`) was fully integrated into
the GlowPlug lifecycle. This proves the architecture handles **any PCIe device**, not
just GPUs:

- `BrainChipLifecycle`: SimpleBind, 3-second settle, basic health check
- `AkidaPersonality`: No DRM card path, no HBM2, no GPU-specific quirks
- Unlimited `akida-pcie ↔ vfio-pci` round-trips
- DRM isolation check skipped for non-GPU drivers

The same pattern applies to FPGAs, TPUs, SmartNICs, DSPs — any PCIe accelerator.

### VendorLifecycle Trait — Final State

Six implementations covering the known PCIe accelerator landscape:

| Lifecycle | Vendor | vfio→native Strategy | Round-trips | Notes |
|-----------|--------|---------------------|-------------|-------|
| NvidiaLifecycle | 0x10de | SimpleBind | Unlimited | HBM2 survives bus reset |
| AmdVega20Lifecycle | 0x1002 (Vega 20) | PmResetAndBind | 1/boot | SMU firmware limitation |
| AmdRdnaLifecycle | 0x1002 (other) | PmResetAndBind | Untested | Conservative Vega 20 defaults |
| IntelXeLifecycle | 0x8086 | SimpleBind | Expected unlimited | FLR support expected |
| BrainChipLifecycle | 0x1e7c | SimpleBind | Unlimited | No GPU quirks |
| GenericLifecycle | other | SimpleWithRescanFallback | Unknown | Safe-slow defaults |

### Zero-Sudo coralctl

Users join the `coralreef` Linux group for full `coralctl` CLI access via Unix socket
(`root:coralreef`, mode 0660). No sudo, no pkexec, no SUID — just group membership.
The privilege boundary is between the user-facing socket and the root-owned systemd
services.

### Ember Architectural Limitation — Per-Device Isolation Needed

The single-threaded `coral-ember` daemon blocks entirely when one device enters D3cold
(sysfs I/O enters D-state/uninterruptible sleep). This caused cascading failure: a D3cold
AMD card made the Akida NPU inaccessible. The fix is per-device thread isolation with
D3cold pre-check (read `power_state` before any sysfs write).

### Triangle Architecture

The compute trio now operates as a triangle:

```
                    coralReef
                   (GlowPlug + Compiler)
                  /                      \
                 /                        \
        toadStool ─────────────────── barraCuda
     (HW Resources + Dispatch)      (Math + Shaders)
```

- **coralReef** provides GlowPlug (PCIe lifecycle) and shader compilation to toadStool
- **toadStool** provides hardware resources and dispatch routing to barraCuda
- **barraCuda** does the math, compiling shaders through toadStool → coralReef → hardware

The trio's next evolution priority is **vendor-agnostic abstraction**: moving from
vendor-specific code paths to a unified `VendorProfile` trait that merges RegisterMap
(hardware introspection) with VendorLifecycle (swap orchestration).

### Dual-Track Dispatch (March 21, 2026 — Exp 072)

Sovereign VFIO and DRM dispatch are now pursued in parallel:

- **Sovereign** (6/10 layers, MMU page table blocker): direct hardware control,
  vendor-agnostic, blocked at `0xbad00200` PBUS timeout on GV100
- **DRM** (code complete, needs hardware validation): kernel-mediated dispatch
  via `amdgpu` (AMD) or `nouveau` (NVIDIA)

coral-driver has fully coded DRM paths for both vendors:
- `AmdDevice`: PM4 command submission, GEM buffers, fence sync — ready to test on MI50
- `NvDevice`: new UAPI (VM_INIT → VM_BIND → EXEC + syncobj) — blocked on Titan V
  by missing PMU firmware, but K80 (Kepler, incoming) needs no PMU

The DRM path bypasses the Naga WGSL→SPIR-V codegen bug (Exp 055) that produces
zero forces for DF64 transcendentals. Route: WGSL → coral-reef → native ISA →
coral-driver DRM → GPU. This is the fastest path to working DF64 compute dispatch.

coral-reef needs GCN5 arch support (MI50 is GFX906, not RDNA2). The MI50's
1/4 rate f64 (3.5 TFLOPS) makes it the best available f64 hardware for validation.

---

## Update: Deep Debt Burndown + Cross-Vendor Dispatch (March 22, 2026, Exp 075)

### Engineering Hardening for PMU Cracking

Before proceeding with Layer 6 MMU page table cracking, 13 deep-debt items were
resolved across `coral-glowplug`, `coral-driver`, and `hotspring-barracuda`:

**Concurrency safety:** TOCTOU race in `DeviceSlot` fixed with `BusyGuard` RAII pattern —
`Arc<AtomicBool>` prevents `swap`/`reclaim`/`resurrect` while oracle capture or compute
dispatch is in progress. Critical for dual-Titan parallel experiments.

**Error handling:** `Bar0Rw::try_read_u32`/`try_write_u32` return `Result` instead of
sentinel values — essential for PMU debugging where every register value is diagnostic
data. `DriverError::OracleError` provides clean error propagation from the oracle module.
`CudaComputeDevice::dispatch_named` returns `DriverError::BufferNotFound` instead of
silently skipping invalid handles. `from_bdf_hint` returns `OpenFailed` instead of
falling back to device 0.

**RPC robustness:** `nvidia-smi` calls moved out of device mutex into async handlers.
`coralctl health` correctly parses `alive`/`device_count`/`healthy_count` fields.
Per-connection `BufReader` starts at 64KB (was 4MB).

**Build configuration:** `cudarc` and `base64` gated behind `cuda-validation` feature.
`saxpy.ptx` retargeted to sm_70 (Volta+) for universal compatibility.

### Cross-Vendor CUDA Dispatch

CUDA-capable GPUs are now accessible interchangeably through the glowplug daemon's
`device.dispatch` RPC. A single PTX kernel (sm_70 target) runs on Volta, Turing,
Ampere, Ada, and Blackwell via JIT compilation. The dispatch path:

```
User binary (unprivileged) → Unix socket → coral-glowplug → coral-driver CUDA → GPU
```

This eliminates `pkexec` from the compute pipeline entirely. The systemd services
hold capabilities; user tools communicate via socket RPC.

### RTX 5060 Dual-Use: Display + Compute Oracle

The RTX 5060 runs CUDA compute concurrently with display output — no driver swap,
no DRM disruption. This transforms the display GPU into a **page table oracle** for
PMU cracking: launch a CUDA allocation → nvidia driver writes PDE/PTE entries →
capture BAR0 state via `try_read_u32` → compare with sovereign PTE encoding on
the Titans → identify divergences.

### PMU Cracking Attack Matrix

| Vector | Hardware | Enabler |
|--------|----------|---------|
| 5060 Oracle Capture | RTX 5060 | `try_read_u32`, dual-use |
| PTE Diff Analysis | 5060 vs Titan V | `OracleError`, `PageTableDump` |
| Dual Titan A/B | Titan V #1 + #2 | `BusyGuard` (concurrent captures) |
| BAR2-Resident Tables | Titan V | `try_write_u32` |
| MMU Fault Buffer | Titan V | `try_read_u32` |
| Tesla P80 (pending) | Tesla P80 | BDF-specific dispatch |

---

---

## Update: SCTL Myth Busted + FalconCapabilityProbe + Sovereign Layers 7-10 (March 25, 2026, Exp 082-092)

### Myth Busted: SCTL Does NOT Block PIO

The IMEMC register on GM200+ falcons uses **BIT(24)** (`0x0100_0000`) for write
auto-increment, not BIT(6) (`0x40`). All previous manual PIO tests used the
wrong control word format, creating a false impression that SCTL=0x3000 blocks
PIO. **PIO to IMEM/DMEM/EMEM works regardless of security mode.** This
invalidated multiple experiment decisions: FLR attempts, SBR for SCTL clearing,
warm handoff to preserve firmware. The actual remaining blocker is DMA configuration
(FBIF mode, FBHUB MMU), not security mode.

### Runtime Bit Solver: FalconCapabilityProbe

`FalconCapabilityProbe` in `falcon_capability.rs` dynamically discovers register
layouts on actual hardware instead of hardcoding assumptions. The IMEMC bit position
varies by falcon version — BIT(24) for GM200+, different on earlier generations.
The probe discovers the correct layout at runtime, making PIO portable across any
NVIDIA GPU generation. Pattern: probe hardware → build `FalconCapabilities` struct
→ use `FalconPio` safe API. Same capability-discovery pattern as `WgslOptimizer`
and `GpuDriverProfile` in the shader stack.

### Sovereign Pipeline: 9/10 Layers Solved

| Layer | Status | Key Discovery |
|-------|--------|---------------|
| L1-L5 | SOLVED | VFIO binding, BAR0/BAR2, PMC, PFIFO, MMU fault buffers |
| L6 | SOLVED (Exp 076) | FBHUB requires non-replayable fault buffers before any MMU walk |
| L7 | **BREAKTHROUGH** (Exp 095) | SEC2 HS mode via sysmem DMA. FBHUB PRI-dead corrupts VRAM DMA; sysmem bypasses FBHUB. Falcon binding B1-B7 (Exp 085) |
| L8 | SOLVED (Exp 087) | 7 WPR construction bugs (W1-W7); ACR bootstraps FECS+GPCCS |
| L9 | SOLVED (Exp 088) | Post-ACR STARTCPU sequence; both falcons transition to RUNNING |
| L10 | **CLOSE** (Exp 095) | Sysmem ACR enters HS; blob_size=0 should avoid trap; FECS/GPCCS bootstrap expected |
| L11 | BLOCKED by L10 | GR context init + shader dispatch; FECS methods already implemented |

Reverse engineering sources: nouveau (primary Rosetta Stone), nvidia-open
kernel modules, Mesa NVK, envytools, NVIDIA closed-source header harvesting.
Cross-driver register profiling (Exp 086) confirmed: WPR is an interface
problem, not a key+lock hardware gate. Post-nouveau state is optimal starting
point for sovereign boot.

### Adaptive Experiment Loop + First Personality Sweep (Exp 092)

Full adaptive experiment loop wired: `SwapObservation` + `ResetObservation`
→ JSONL journal → `AdaptiveLifecycle` (settle times + reset selection from
history). `DriverObserver` trait with personality-specific observers (nouveau,
vfio, nvidia, nvidia-open). Ring/mailbox state persisted across swaps via
ember `ring_meta`. `coralctl experiment sweep` CLI for automated personality
characterization. First sweep on both Titan Vs: nouveau 21.9s / nvidia-open
26.8s bind. Sub-1% cross-card variance. HBM2 alive on both cards post-sweep.

### Deep Code Quality Evolution

Systematic evolution of `coral-driver`:
- **60+ hardcoded hex offsets** → named register constants in `registers.rs`
- **4 unsafe blocks eliminated** via safe `DmaBuffer::volatile_write_u32/u64/read_u32`
- **`NonNull<u8>`** replaces raw `*mut u8` in DMA buffers (type-level non-null invariant)
- **Shared helpers** extracted: `poll_falcon_boot`, `dmem_nonzero_summary`, `dmem_detail`
- 511 lib tests pass, zero new unsafe, zero `unwrap()` in production code

### Compute Trio Evolution (coralReef + toadStool + barraCuda)

The trio converges on **capability-based discovery at every layer**:
- **Hardware layer** (coralReef): `FalconCapabilityProbe` discovers falcon PIO layouts
- **Shader layer** (toadStool): `GpuDriverProfile` discovers ILP scheduling parameters
- **Math layer** (barraCuda): adapter enumeration discovers GPU memory/capability

Each primal discovers capabilities at runtime rather than hardcoding vendor specifics.
The `VendorLifecycle` + `RegisterMap` trait pair provides the vendor-agnostic
abstraction. All cross-spring dispatch now routes through `ComputeDispatch<B: GpuBackend>`.

---

*Sovereign compute hardware: 3 precision tiers, 4 device types (NVIDIA GPU, AMD GPU,
BrainChip NPU, Intel GPU stubs), zero-sudo operation, triangle architecture.
$750 buys a precision-routed QCD rig with native f64 + df64 + NPU steering.
No proprietary drivers. No cloud dependencies. coral-glowplug daemon survives
reboot, manages GPU lifecycle from boot to clean shutdown. AMD GCN5 DRM: 6/6
preswap phases PASS (f64 LJ force, Newton's 3rd law). RTX 5060 Blackwell DRM:
pipeline cracked (SM120, 4/4 HW tests). iommufd/cdev: kernel-agnostic VFIO on
6.2+ (resolves EBUSY on 6.17, 607 tests, HW validated). AMD Vega 20: one
round-trip per boot (firmware limit). NVIDIA GV100: unlimited. Akida NPU: unlimited.
Vendor-agnostic, seccomp-sandboxed, capability-restricted. 92 experiments, dual-track
dispatch (DRM + sovereign VFIO), cross-vendor CUDA dispatch, pkexec-free pipeline,
RTX 5060 dual-use oracle. **Sovereign VFIO: 9/10 layers SOLVED** — Falcon binding
(B1-B7, Exp 085), WPR construction (W1-W7, Exp 087), FECS+GPCCS boot (Exp 088),
SCTL myth busted (Exp 091). IMEMC BIT(24) discovery + FalconCapabilityProbe runtime
bit solver ensures portability. Layer 10 root cause found (BOOTVEC). Adaptive
experiment loop with personality sweep, JSONL journal, observer traits. 4,065 tests
pass workspace-wide. Deep code debt burned: 60+ hardcoded offsets → constants,
4 unsafe blocks eliminated, NonNull DMA, safe volatile wrappers.
Built on consumer hardware.*

---

## March 30 Update: Validation Matrix and Livepatch Strategy

The Titan V sovereign stack is now tracked as a **four-path validation matrix**. Each path answers a different question: VFIO lifecycle and handoff, proprietary DRM mediation, open DRM, or Mesa NVK/wgpu. Together they define what is proven today versus what remains gated on firmware, driver validation, or livepatch control.

### Titan V — four dispatch paths

| Path | Role | Status |
|------|------|--------|
| **VFIO warm handoff** | Livepatch **4-NOP** slot with **dynamic enable/disable** so the GPU can move between VFIO and a native personality without full reboot choreography; pairs with warm-handoff scripts and permission hardening. | Active validation track — orchestrates lifecycle when DRM paths are unavailable or risky. |
| **nvidia-drm + UVM** | Kernel-mediated VM/bind/exec path in coralReef/coral-driver (proprietary stack). | **Code-complete in coralReef; pending on-hardware validation** on the Gate fleet. |
| **nouveau DRM** | Fully open DRM path for compute. | **Blocked on Titan V: missing PMU firmware** — same class of gating called out elsewhere in this document for FECS/GPCCS bring-up. |
| **NVK / wgpu** | Mesa NVK + wgpu stack for portable compute. | **Proven** — including **four-tier QCD** workloads on sovereign-friendly paths where NVK is the display/compute API. |

This matrix is the hardware-facing complement to orchestration and math-layer fixes: routing only works if at least one path per machine is green; the matrix makes that explicit per card.

### Upstream integration (March 2026)

- **toadStool S168** — `shader.dispatch` wiring tightens the **orchestration layer**: compute requests flow through typed dispatch with clearer handoff to coralReef and device brokers.
- **barraCuda Sprint 23** — **f64 precision pipeline** fixes (transcendentals, Dekker/Knuth paths, and NVVM-adjacent hazards) so physics binaries do not fight the driver on Volta-class hardware.
- **coral-ember / coral-glowplug** — **`reset_method` fix** (avoid blocking PCI reset on VFIO fd teardown where documented), **JSONL journal tracking** for swap observations, and **dynamic livepatch control** so 4-NOP and related patches can be toggled without redeploying the whole daemon graph.

### Warm FECS Dispatch + Puzzle Box Matrix (Exp 127-128, March 30)

Exp 127 validated that FECS firmware **survives** the nouveau→vfio-pci swap via livepatch (CPUCTL: `0xbadf1201` → `0x00000010`, SCTL: `0x00003000` HS+, 23 engines powered). But FECS enters idle HALT and cannot be woken from HS+ mode. The problem shifted from **preservation** to **resumption**.

Exp 128 implements a puzzle box matrix with parallel solution tracks:
- **K80 (Kepler):** Full nvidia-470 recipe replay + PIO FECS boot + GPFIFO channel dispatch — validates infrastructure with zero security barriers
- **Titan V (Volta):** Keepalive (hold DRM fd), nvidia proprietary warm handoff (learn RM's FECS init), timing attack (50ms BAR0 polls), STOP_CTXSW freeze
- **Cross-cutting:** FECS method enumeration, CPUCTL bit labeling fix (bit 4 = halted, bit 5 = stopped)

### GPU Lifecycle Wired Into Daemon RPC Layer (March 30)

All livepatch management and GPU register access moved from shell scripts and `coralctl` into `ember`/`glowplug` as first-class JSON-RPC operations: `ember.livepatch.*` (status/enable/disable), `ember.fecs.state` (structured FECS snapshot), `ember.mmio.read` (mmap-based BAR0 access), `device.warm_handoff` (full orchestrated warm handoff). This provides a programmable interface for other primals and projects to interact with the GPU lifecycle.

Code quality: FECS register offsets shared as `coral-driver::nv::bar0::FECS_*` constants, hex parsing consolidated into `coral-driver::parse_hex_u32`, `Bar0Access` DRY'd via shared `mmap_file`, livepatch handlers idempotent with `was_noop` feedback. 808 tests across the three crates.

### References (hotSpring)

For experiment-level captures, cross-GPU comparisons, DRM tracing, and warm-handoff procedure notes, see **hotSpring** experiments **122–128** (VM capture / cross-analysis, livepatch breakthrough, DRM tracing matrix, warm FECS dispatch attack, puzzle box matrix).

The consolidated **sovereign validation matrix** (dispatch paths × hardware × gate status) lives at:

`hotSpring/specs/SOVEREIGN_VALIDATION_MATRIX.md`

Use that file as the checklist when a gate moves from “code-complete” to “hardware-validated” or when a path is downgraded (for example nouveau blocked on PMU until firmware exists).
