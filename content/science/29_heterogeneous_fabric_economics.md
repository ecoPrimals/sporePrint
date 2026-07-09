+++
title = "baseCamp 29 — Heterogeneous Fabric Economics"
description = "Hardware economics: DDR5 cost inversion, compute-on-card architectures, sovereign cluster total cost of ownership vs cloud."
date = 2026-07-09

[taxonomies]
primals = ["barracuda", "toadstool"]
springs = ["hotspring", "neuralspring"]
+++

{{ maturity(level="architectural") }}

**Date:** May 2, 2026
**Status:** Architecture document — formalising principles observed in biomeGate hardware iteration and existing toadStool/barraCuda implementations.
**Domain:** Hardware architecture × distributed compute economics × physical data fabric
**Cross-Spring:** barraCuda × toadStool × hotSpring × songBird × biomeOS
**Related Docs:** [Sovereign Compute Hardware](@/science/14_sovereign_compute_hardware.md), [Hardware Cost Analysis](@/technical/HARDWARE_COST_ANALYSIS.md), `DUAL_FABRIC_ARCHITECTURE.md`, `HARDWARE_TRANSPORT_SPEC.md`

---

## Abstract

Two economic and physical facts have converged to change the architecture of sovereign compute. First, DDR5 system memory has become so expensive relative to GPU VRAM that VRAM is now the *cheaper* bulk workspace — the market has inverted. Second, PCIe bandwidth is not symmetric: it is finite, shared, and latency-bounded, meaning computation that stays on-card is categorically cheaper than computation that crosses the bus. Together these facts define a principle already visible in barraCuda's `dispatch_with_transfer_cost` and toadStool's dual-fabric model: **route work to the cheapest storage tier and minimise bus crossings.** This document formalises the compute-on-card ratio, the batch crossover equations, the memory economics arithmetic, and the unidirectional HDMI fabric that allows these principles to extend across physical machines — chaining gates together the way video game developers have always chained capture cards.

---

## 1. The Market Inversion

### Memory cost per GB (as of Q1 2026)

| Memory tier | $/GB (retail) | $/GB (used market) | Notes |
|---|---|---|---|
| DDR5-6400 ECC RDIMM | ~$12–18 | $8–12 | Server 9950X3D / Sapphire Rapids |
| DDR4 32GB ECC RDIMM | ~$6–9 | $3–5 | 8× 32 GB = 256 GB on TRX40 |
| RTX 5060 / 5070 VRAM | ~$4–6 (in GPU price) | n/a (new gen) | 12–16 GB GDDR7 |
| RTX 3090 / 4090 VRAM | ~$2–4 | $1.50–3 | 24 GB GDDR6X, abundant used |
| Titan V (HBM2) | ~$1–2 | $0.80–1.50 | 12 GB, 900 GB/s bandwidth |
| Tesla K80 (GDDR5) | ~$0.30–0.60 | $0.20–0.40 | 2× 12 GB, PCIe, no display |

At the biomeGate node (Threadripper 3970X, 256 GB DDR4), the aggregate VRAM across installed cards (RTX 5060 12 GB + Titan V 12 GB + K80 2× 12 GB) is **48 GB of GPU memory at a hardware-acquisition cost well below the cost of 48 GB of additional DDR5 RDIMM**.

The implication is direct: **for bulk matrix operations, GPU VRAM is the cheap tier.** Moving data to card and operating there is economically justified when the compute time on-card is non-trivial — not because GPU ALUs are faster (though they are), but because the *storage* is now affordable. This inverts the classical assumption that "GPU memory is the scarce resource."

### The engineering consequence

When VRAM is cheap and abundant across a heterogeneous fleet, the PCIe bus becomes the bottleneck to manage — not the memory budget. The architecture problem shifts from **"can we fit this in VRAM?"** to **"at what batch size does crossing the bus become cheaper than not crossing it?"**

---

## 2. Compute-on-Card Ratios: The Core Equations

### 2.1 Transfer cost model (from `barraCuda::unified_hardware::transfer`)

For a single device-to-device transfer:

```
T(bytes) = L_us + bytes / (BW_GB_s × 1000)
```

Where:
- `L_us` — PCIe DMA round-trip latency (empirical baseline: **5 µs** per `PCIE_DMA_LATENCY_US`)
- `BW_GB_s` — link bandwidth in GB/s (theoretical unidirectional, from `PcieLinkInfo::bandwidth_gbps`)
- `bytes` — payload size in bytes

Effective bandwidth accounting for PCIe encoding and contention (from `PcieTopologyGraph::effective_bandwidth_bps`):

```
BW_eff = BW_raw × contention_factor × 0.78
```

The 0.78 factor captures the ~22% overhead of PCIe 8b/10b or 128b/130b encoding and protocol framing. On a PCIe 4.0 x16 link (theoretical 32 GB/s), `BW_eff ≈ 25 GB/s` for a card with no bus contention (`contention_factor = 1.0`), dropping to ~12 GB/s through a PLX switch with four active endpoints.

### 2.2 GPU dispatch overhead

Submitting a compute kernel and reading back results carries a fixed overhead beyond raw transfer time. The empirical constant from `barraCuda`:

```
OVERHEAD_us = 1500 µs   (GPU_DISPATCH_OVERHEAD_US)
```

This is the queue-submit + fence + readback round-trip on NVIDIA hardware. It means any job that completes in under ~1.5 ms is dominated by dispatch overhead, not arithmetic.

### 2.3 Compute efficiency ratio

Define the **compute efficiency** `η` for a single dispatch of `B` items:

```
C(B) = B × c_elem              # compute time, linear in batch size
T(B) = L_us + B × d_elem / (BW_eff × 1000) + OVERHEAD_us
η(B) = C(B) / (C(B) + T(B))
```

Where:
- `c_elem` — per-element compute cost in µs (on-card, after kernel launch)
- `d_elem` — per-element data volume in bytes (input + output)
- `η → 1` when compute dominates; `η → 0` when transfer dominates

### 2.4 Crossover batch size B*

The crossover batch `B*` is where compute time equals transfer time:

```
C(B*) = T(B*)
B* × c_elem = L_us + B* × d_elem / (BW_eff × 1000) + OVERHEAD_us
```

Solving for B*:

```
B* = (L_us + OVERHEAD_us) / (c_elem - d_elem / (BW_eff × 1000))
```

This is only defined when `c_elem > d_elem / (BW_eff × 1000)`, i.e., when arithmetic intensity is high enough that larger batches eventually become compute-bound. When the denominator is negative, the operation is *always* transfer-bound regardless of batch size and belongs on CPU or a local accelerator without a bus crossing.

### 2.5 Worked examples from the biomeGate fleet

**Case 1 — AKD1000 single-layer ESN readout (from `hotSpring` HARDWARE.md)**

```
c_elem  ≈ 0.7 µs / 1 sample = 0.7 µs/sample
d_elem  ≈ 2 KB per sample (rough input vector)
BW_eff  ≈ 0.25 GB/s  (PCIe 2.0 x1: 0.25 × 1 × 0.78 ≈ 0.195 GB/s)
L_us    ≈ 650 µs  (measured round-trip, Akida-specific)
OVERHEAD_us ≈ 0 (no GPU dispatch overhead; NPU path)

d_elem / (BW_eff × 1000) ≈ 2000 / (195) ≈ 10.3 µs/sample

B* = 650 / (0.7 - 10.3)  → negative denominator
```

**Conclusion:** AKD1000 single-layer readout is *always transfer-bound* at PCIe x1. Batching amortizes the fixed latency (650 µs shared across B samples) but not the per-element transfer cost. The architecture response: use the Akida for wide-layer batch inference where `c_elem` grows with layer width. At width 512, per-element compute exceeds per-element transfer.

**Case 2 — Titan V lattice QCD force kernel**

```
c_elem  ≈ 12 µs per lattice site  (4^3 × 8 kernel, df64 SASS)
d_elem  ≈ 6 × 18 × 8 bytes ≈ 864 bytes/site  (SU(3) gauge links, 6 neighbors)
BW_eff  ≈ 6 GB/s  (PCIe 3.0 x16 to VFIO VM, through one hop)
L_us    ≈ 5 µs
OVERHEAD_us ≈ 1500 µs

d_elem / (BW_eff × 1000) ≈ 864 / 6000 ≈ 0.14 µs/site

B* = (5 + 1500) / (12 - 0.14) ≈ 1505 / 11.86 ≈ 127 sites
```

**Conclusion:** At > 127 lattice sites per dispatch, the Titan V force kernel is compute-bound. A full 8³ = 512-site volume is 4× past the crossover — transfer is effectively free. At the 16³ production lattice (4096 sites), `η > 0.99`. This is why sub-thesis 14's HMC pipeline achieved near-100% GPU utilisation despite PCIe not being in a direct CPU path.

**Case 3 — RTX 5060 tensor contraction (f32, barraCuda path)**

```
c_elem  ≈ 0.002 µs/element  (FP32 GEMM, ~14 TFLOPS, ~2 ops/element)
d_elem  ≈ 4 bytes (f32)
BW_eff  ≈ 25 GB/s  (PCIe 4.0 x16, direct CPU path)
OVERHEAD_us ≈ 1500 µs

d_elem / (BW_eff × 1000) ≈ 4 / 25000 ≈ 0.00016 µs/element

B* = (5 + 1500) / (0.002 - 0.00016) ≈ 1505 / 0.00184 ≈ 818,000 elements
```

**Conclusion:** Approximately 1M elements (4 MB tensor) is the crossover for RTX 5060 f32 GEMM. Small tensors should stay on CPU. Once past 4 MB, the GPU's ALU advantage dominates and dispatch overhead is amortized. This matches `dispatch_with_transfer_cost`'s observed threshold behaviour.

---

## 3. Unidirectional Hardware Data Flows — The Capture-Card Principle

### 3.1 The video game observation

Video game streaming infrastructure solved a version of this problem a decade ago. A gaming PC renders frames, outputs them over HDMI or DisplayPort, and a capture card on a second machine ingests the signal. The two machines are **physically decoupled**: the render machine does not know the capture machine exists. The data flows one direction. The bandwidth is determined by the display connector, not any software stack.

**The key insight for compute fabric: the same connector is available on every GPU we own.**

HDMI 2.1 carries ~42 Gbps of encoded data = ~5.25 GB/s of payload. DisplayPort 2.1 UHBR20 carries ~77 Gbps = ~9.6 GB/s payload. These are **unidirectional** physical channels. A GPU renders a frame (in our case: a data-encoded framebuffer) and outputs it to any receiver with a capture input, regardless of what OS, PCIe bus, or software stack runs on either end.

### 3.2 toadStool's `DisplayTransport` + `CaptureTransport`

This is not a new concept for the project — toadStool already implements it. The `DisplayTransport` crate encodes arbitrary byte payloads as pixel data, writes them into a DRM dumb buffer, and page-flips to a physical HDMI/DP connector. `CaptureTransport` (V4L2) receives the signal on the other end. The `DUAL_FABRIC_ARCHITECTURE.md` spec names this the **Hardware Plane** and contrasts it with the **Network Plane** (songBird/TCP):

| Property | Network Plane (songBird) | Hardware Plane (toadStool) |
|---|---|---|
| Direction | Bidirectional | Unidirectional (HDMI/DP) or bidirectional (serial/PCIe) |
| Bandwidth | 1–100 Gbps NIC | ~5 GB/s per HDMI 2.1 link |
| Latency | Variable (TCP stack) | Fixed (~8–16 ms at 60–120 Hz) |
| Security model | Software (TLS) | **Physics** — cable present or not |
| Discovery | mDNS / IP scan | Physical — cable present or not |

The hardware plane's bandwidth is **additive per GPU**. biomeGate with three HDMI-capable GPUs (RTX 5060, Titan V via VFIO passthrough, Tesla K80 via VFIO display emulation) could in principle output 3 × 5 GB/s = **15 GB/s** of unidirectional compute data to a downstream gate, with no TCP stack, no NIC interrupt load, and no shared PCIe arbiter.

### 3.3 Ring topology and the return path

A ring pipeline chains N gates, each gate processing and forwarding to the next:

```
biomeGate ──HDMI──▶ northGate ──HDMI──▶ strandGate
    ▲                                         │
    └──────────── songBird (10 GbE) ──────────┘
```

The HDMI links carry the high-bandwidth forward path (compute results, bulk data). songBird carries the low-bandwidth return path (control signals, results acknowledgement, job dispatch). This matches the `DUAL_FABRIC_ARCHITECTURE.md` ring topology exactly.

**Why this works economically:** A 10 GbE songBird link costs $30 (Aquantia card), provides 1.25 GB/s bidirectional. Adding a capture card costs $80–150. The total inter-gate hardware fabric cost is ~$150 vs. the cost of an NVLink bridge ($2,000+) or InfiniBand ($5,000+). The trade-off is latency (16 ms frame period at 60 Hz) vs. bandwidth (5 GB/s). For batch compute workloads — the dominant use case in ecoPrimals — 16 ms pipeline latency is irrelevant; the batch itself takes seconds to minutes.

### 3.4 Per-gate bandwidth budget

Combining all data movement channels at biomeGate:

| Channel | Direction | Bandwidth | Notes |
|---|---|---|---|
| RTX 5060 PCIe 4.0 x16 | In/Out | 25 GB/s (eff.) | Local host DMA |
| Titan V PCIe 3.0 x16 via VFIO | In/Out | 12 GB/s (eff.) | Sovereign path |
| K80 die #1 PCIe 3.0 x16 via VFIO | In/Out | 6 GB/s (eff.) | Through PLX switch |
| K80 die #2 PCIe 3.0 x16 via VFIO | In/Out | 6 GB/s (eff.) | Through PLX switch |
| RTX 5060 HDMI 2.1 out | **Unidirectional out** | ~5 GB/s | Capture card on downstream gate |
| Aquantia AQC111 (5GbE) | In/Out | 0.625 GB/s | songBird control fabric |
| Akida (M.2 PCIe x2, ordered) | In/Out | 0.5 GB/s | Sparse inference dispatch |

**Total inbound compute capacity:** ~49 GB/s  
**Total outbound unidirectional:** ~5 GB/s (expandable to ~15 GB/s with all GPUs outputting)

### 3.5 Data diode security model

A hard side-effect of the HDMI unidirectional path: **the receive-side machine cannot inject data or commands into the send-side machine through the HDMI cable.** The signal flows one way at the physics layer. This is a hardware data diode. For workflows handling sensitive intermediate data (e.g., encrypted genomic samples in groundSpring), routing the bulk data flow over the HDMI fabric and keeping the control plane on songBird means the bulk data path has no software-exploitable attack surface.

---

## 4. PCIe Bifurcation as Dense Accelerator Fabric

The biomeGate investigation (documented in this project's hardware iteration log) revealed that the TRX40 AORUS MASTER supports PCIe bifurcation: a single x16 slot can be split into multiple x4 segments, each serving an independent device. A bifurcation adapter (M.2 to x4 × 4) turns one x16 slot into four independent PCIe x4 channels.

For the Akida AKD1000 (PCIe x2 Gen 3 = ~2 GB/s unidirectional, ~4 GB/s total):
- A 4× M.2 bifurcation adapter in the PCIEX16_1 slot provides **four independent x4 channels** = four Akida M.2 modules simultaneously
- Total Akida inference bandwidth: 4 × ~2 GB/s = **8 GB/s**
- Total Akida SRAM capacity: 4 × 1.24 MB = ~5 MB on-chip
- Power draw: 4 × 30 mW active = **120 mW** — negligible

Compare this to 4 additional discrete PCIe cards requiring 4 separate slots, power connectors, and cooling. The bifurcation path is the correct economics for sparse, event-driven NPU workloads.

The same principle scales to GPU expansion. An x16 → 4× x4 riser with four small GPUs (e.g., Tesla P4, 8 GB GDDR5, $30 used) produces 32 GB of VRAM on a single slot for batch inference at ~$30/GPU vs. $80–150 for a larger single card. At the `B* > 127 sites` crossover, the per-card dispatch overhead is the same regardless of card count; pipeline-parallel dispatch across four cards scales throughput 4×.

---

## 5. Towards a Unified Dispatch Formula

### 5.1 Multi-card batch partition

Given a job of `N` total elements and `K` available cards, the optimal per-card batch `B_k` partitions N such that:

```
η_total = Σ C(B_k) / (Σ C(B_k) + max_k(T(B_k)) + DISPATCH_SERIALISATION)
```

Where `DISPATCH_SERIALISATION` accounts for the sequential overhead of submitting to K cards that cannot be pipelined. With barraCuda's current synchronous dispatch model, `DISPATCH_SERIALISATION = (K-1) × OVERHEAD_us`. With pipelined dispatch (future work: `unified_hardware` async path), this collapses to `max_k(OVERHEAD_us)`.

**Practical implication:** Dispatching to 4 cards sequentially costs `3 × 1500 µs = 4.5 ms` of wasted time. This motivates async multi-card submission as the next barraCuda dispatch primitive — already reflected in `SPRING_ABSORPTION.md` item `AZ` (VRAM quota in buffer allocation) and `AY` (PCIe topology sysfs probing).

### 5.2 Cross-gate pipeline efficiency

For a two-gate HDMI ring pipeline:

```
T_pipe = T_compute(gate_A) + T_hdmi_encode + T_hdmi_latency + T_hdmi_decode + T_compute(gate_B)
T_serial = T_compute(gate_A) + T_compute(gate_B)
T_network_return = T_songbird_ack
```

**Effective throughput** of the pipeline vs. a single gate running both stages serially:
- If `T_compute(A) ≈ T_compute(B)`, HDMI pipeline is 2× throughput with HDMI latency being the only overhead
- HDMI frame latency = `1 / Hz` = **8.3 ms at 120 Hz**, **4.2 ms at 240 Hz** — trivial vs. second-scale batch compute

**Pixel-data density at 4K60 RGBA8888:**  
`3840 × 2160 × 4 bytes × 60 Hz ≈ 1.99 GB/s` (from `HARDWARE_TRANSPORT_SPEC.md`)  
At 4K120 with HDR10 this reaches ~4 GB/s. With multiple GPU outputs in parallel (toadStool's multi-GPU aggregate table), the HDMI fabric scales linearly with the number of display outputs.

### 5.3 The biomeGate → strandGate data flow sketch

```
biomeGate (Threadripper)
│
├── RTX 5060: primary dispatch, f32/df64 bulk compute
│   └── HDMI 2.1 out ──▶ strandGate capture card ──▶ strandGate GPU
│
├── Titan V: HBM2 f64 validation, sovereign DRM path
│   └── VFIO → VM → HDMI out (emulated) → optional capture
│
├── K80 die #1: reagent VM, cold-POST Kepler, VFIO sovereign
├── K80 die #2: oracle VM, shared-root-complex USB caution
│   └── Both K80s: bulk f32 batch for older CUDA kernels in reagent VMs
│
└── Akida (M.2, PCIe x2 via CPU M2M slot) ──▶ sparse ESN readout
    └── Results via PCIe DMA to RTX 5060 staging buffer
        └── HDMI bulk export to strandGate if B > crossover width
```

---

## 6. The DDR5 Gate as Architectural Decision Point

The context for this document is that RAM costs on new platforms have created a bifurcation point: when a CPU board upgrade is required, the RAM cost can exceed the GPU cost for equivalent computational workspace. The 9950X3D platform (AM5, DDR5) demonstrates this: a second 9950X3D board with 96 GB DDR5 costs more in RAM alone than four used RTX 3090 cards providing 96 GB of GDDR6X. For batch compute that fits in VRAM, the GPU path is strictly cheaper per byte and per FLOP.

The **gate architecture** of ecoPrimals responds to this: each gate optimises for one or two workload classes (HBM2 precision at biomeGate, high-core-count at northGate, AMD genomic at strandGate) and exports bulk results via the HDMI/songBird fabric rather than centralising all RAM at one node. The overhead of the HDMI pipe (8–16 ms) is acceptable because **batch compute runs for seconds to minutes**, not milliseconds.

This also directly answers the DDR5 RMA incident: the 14900K board (Intel, DDR5) returning from RMA with a 9950X3D as replacement is not a failure mode — it is an opportunity to assign DDR5 RAM where it matters (latency-sensitive host operations, OS working set, sparse random-access patterns) and route bulk data through the GPU VRAM tier.

---

## 7. Action Items and Open Questions

### Implementation items (barraCuda)
- [ ] Async multi-card dispatch: `unified_hardware` async path to eliminate `(K-1) × OVERHEAD_us` serialisation penalty
- [ ] Expose `B*` crossover calculation as a barraCuda planning API (inputs: `c_elem`, `d_elem`, link topology; output: recommended `min_batch_size`)
- [ ] PCIe bifurcation awareness in `pcie_topology.rs`: model bifurcated slots as multiple independent `PcieLink` nodes with shared root-complex bandwidth ceiling

### Implementation items (toadStool)
- [ ] `CaptureTransport` (V4L2 Rx): complete implementation and mark non-"Future" in transport spec table
- [ ] Multi-GPU aggregate HDMI routing in `TransportRouter`: `select_by(min_bandwidth_bps, Tx, count=3)` for parallel HDMI export
- [ ] Frame protocol: define a compact header for data-encoded framebuffers (tag, length, checksum, sequence number) that survives HDMI encoding and V4L2 capture artefacts

### Open architectural questions
1. **HDMI capture latency under load:** V4L2 capture frame delivery latency when the capture card's internal buffer is full. Does the HDMI source GPU stall or drop frames? Stalling would break pipeline throughput guarantees.
2. **PLX switch contention on K80:** Both K80 dies share the PEX 8747 switch. Peak simultaneous DMA from both dies competes for the upstream x16 link. The `contention_factor` in `pcie_topology.rs` needs an empirical calibration value for this topology.
3. **Akida M.2 on bifurcation adapter:** 4× AKD1000 M.2 modules on a single x16 bifurcation adapter. Confirmed hardware exists (BrainChip M.2 B+M Key, PCIe x2). Pending: adapter sourcing and BIOS bifurcation profile validation on TRX40.
4. **Cross-gate ring clock synchronisation:** In a ring pipeline with HDMI as forward path and songBird as return path, how do we synchronise batch boundaries across gates? Proposal: songBird carries a monotonic batch sequence number; each gate waits for `seq_ack(n)` before starting batch `n+1`.

---

## 8. Summary

| Principle | Equation / Rule | Implementation |
|---|---|---|
| Transfer cost | `T = L_us + bytes / (BW × 1000)` | `barraCuda::transfer::TransferCost` |
| Effective PCIe BW | `BW_eff = BW_raw × contention × 0.78` | `toadStool::pcie_topology::effective_bandwidth_bps` |
| Dispatch overhead | `1500 µs` fixed per GPU submit | `GPU_DISPATCH_OVERHEAD_US` |
| Crossover batch | `B* = (L + OVERHEAD) / (c_elem - d_elem/BW_eff)` | `dispatch_with_transfer_cost` threshold |
| HDMI throughput | `~5 GB/s / GPU @ 4K120` | `DisplayTransport` + `HARDWARE_TRANSPORT_SPEC` |
| Ring topology | HDMI forward + songBird return | `DUAL_FABRIC_ARCHITECTURE.md` |
| VRAM economics | VRAM ≤ $3/GB used; DDR5 ≥ $10/GB new | Market reality → dispatch to card when `B > B*` |
| Bifurcation | 1× x16 slot = 4× independent x4 channels | TRX40 BIOS, M.2 adapter |

The sovereign compute stack — barraCuda routing math to the cheapest silicon, toadStool streaming results via the HDMI fabric, songBird closing the control loop — now has a quantitative foundation for *when* and *by how much* each path wins.

---

**See also:**

- [Hardware Cost Analysis](@/technical/HARDWARE_COST_ANALYSIS.md) — sovereign cluster total cost of ownership vs cloud
- [BarraCuda](@/thesis/06_barracuda.md) — GPU dispatch, transfer cost model, and compute-on-card routing
- [For Hardware Builders and Hobbyists](@/audience/FOR_HARDWARE_BUILDERS_AND_HOBBYISTS.md) — gate architecture and heterogeneous fabric for builders
