+++
title = "Neuromorphic Sovereign Driver"
description = "Pure Rust Akida NPU driver — VFIO passthrough, FBZ format reverse engineering, 80-NPU mesh discovery, user-level hardware access. Zero C, zero Python, zero vendor SDK."
date = 2026-04-29

[extra]
paper_number = 26
domain = "Neuromorphic Hardware"

[taxonomies]
primals = ["toadstool", "coralreef", "barracuda"]
springs = ["hotspring", "airspring", "wetspring"]
+++

**Date:** April 29, 2026
**Status:** VFIO backend live on AKD1000 (vendor `0x1e7c`, device `0xbca1`). 80 NPUs discovered, 10 MB SRAM mapped, user-level udev access confirmed. 293+ tests passing. scyBorg triple licensed.
**Domain:** Neuromorphic hardware, sovereign compute, VFIO passthrough, binary format reverse engineering
**Novelty:** First pure Rust driver for BrainChip Akida. First public documentation of the `.fbz` binary format (varint + Snappy + zero-padding). First VFIO-based NPU access without vendor kernel module. First user-level neuromorphic hardware access via udev rules.
**Cross-Spring:** {{ entity(name="hotspring") }} (lattice QCD deployment — 5,978 live NPU calls) × {{ entity(name="airspring") }} (agricultural ESN streaming) × {{ entity(name="wetspring") }} (sentinel microbe inference) × {{ entity(name="toadstool") }} (parent neuromorphic layer) × {{ entity(name="coralreef") }} (VFIO architecture reference) × {{ entity(name="barracuda") }} (downstream math consumer)

---

## Abstract

BrainChip's Akida neuromorphic processors (AKD1000, AKD1500) ship with a Python
SDK, a C++ inference engine, and a proprietary kernel module. None of these are
inspectable, reproducible, or sovereign. {{ entity(name="rustchip") }} replaces the
entire stack with pure Rust — 5 crates, zero C dependencies, zero Python, zero
vendor SDK at runtime.

This document covers three technical artifacts:

1. **FBZ format reverse engineering** — the undocumented binary format used by
   Akida model files, including Snappy compression with zero-padding that breaks
   naive decompression.

2. **VFIO passthrough driver** — container/group/device lifecycle, BAR mapping,
   DMA, and the ioctl encoding fix that resolved a kernel API mismatch.

3. **User-level hardware access** — udev rules that eliminate root requirements
   for daily NPU operation.

The driver is validated on live AKD1000 hardware: 80 NPUs discovered via VFIO
BAR registers, 10 MB SRAM mapped, inference at 18,500 Hz / 54 µs / 1.4 µJ.
Production deployment: 5,978 live calls over 24 hours in lattice QCD simulation
({{ entity(name="hotspring") }} Experiment 022).

---

## 1. Why a Pure Rust Driver Exists

The vendor stack has three problems for sovereign compute:

| Problem | Vendor stack | rustChip |
|---------|-------------|----------|
| **Inspectability** | Closed C++ engine, Python wrappers, opaque `.fbz` format | All source visible, FBZ format documented, FlatBuffer schema extracted |
| **Dependencies** | Python 3.8+, TensorFlow, MetaTF, kernel module (GPL-2.0 C) | Rust only. `cargo build` on any Linux with IOMMU |
| **Licensing** | Proprietary SDK license | scyBorg triple: AGPL (code) + CC-BY-SA (docs) + ORC (game mechanics) |

The scyBorg exception protocol includes a standing offer of license diplomacy:
hardware partners contribute silicon access or documentation, and receive linking
exceptions in return. This is not adversarial — it is symbiotic.

---

## 2. The FBZ Binary Format

Akida model files use the `.fbz` extension. The format is undocumented.

### Structure

```
┌─────────────────────────┐
│  varint: payload length │  (protobuf-style LEB128)
├─────────────────────────┤
│  Snappy-compressed      │
│  FlatBuffer payload     │
│  (program_info +        │
│   program_data)         │
├─────────────────────────┤
│  zero padding           │  (0x00 bytes to alignment boundary)
└─────────────────────────┘
```

### The Zero-Padding Problem

Standard Snappy decompression fails on `.fbz` files. The Snappy stream is
followed by zero bytes that the decoder interprets as literal chunk headers,
causing buffer overflow errors.

**Discovery method:** Hexdump analysis of model zoo files showed consistent
patterns: valid Snappy chunks followed by runs of `0x00` bytes to the next
alignment boundary.

**Solution:** Linear probe from the last non-zero byte. Starting at `last_nonzero + 1`,
try progressively shorter slices (up to 8 bytes of backtracking). The first
slice that decompresses successfully is the true Snappy stream boundary.

```rust
fn decompress_fbz(data: &[u8]) -> Result<Vec<u8>> {
    let last_nz = data.iter().rposition(|&b| b != 0).unwrap_or(0);
    for end in (last_nz.saturating_sub(7)..=last_nz + 1).rev() {
        if let Ok(out) = snap::raw::Decoder::new().decompress_vec(&data[..end]) {
            return Ok(out);
        }
    }
    Err(Error::DecompressionFailed)
}
```

### FlatBuffer Payload

Once decompressed, the payload is a standard FlatBuffer containing:

- `program_info`: layer graph, NP assignments, input/output shapes
- `program_data`: quantized weights, bias terms, threshold SRAM layouts

The `ProgramBuilder` in `akida-models` can construct these from scratch,
enabling model creation without the vendor's MetaTF/QuantizeML toolchain.

---

## 3. VFIO Passthrough Architecture

The VFIO backend provides full hardware access without a kernel module.

### Container / Group / Device

```
/dev/vfio/vfio          ← container (IOMMU context)
    └── /dev/vfio/92    ← group (IOMMU group for AKD1000)
        └── device fd   ← bound PCI device (0000:e2:00.0)
            ├── BAR0    ← control registers (NP count, SRAM size, mesh topology)
            └── BAR1    ← SRAM (10 MB mapped, weights + activations)
```

### The Ioctl Fix

The initial VFIO implementation failed with `ENOTTY` ("Inappropriate ioctl for device")
when mapping BAR regions. Root cause: the `VFIO_DEVICE_GET_REGION_INFO` constant
was hardcoded as `0xc018_3b68`, which encodes `_IOWR(';', 104, ...)`. The Linux
kernel expects `_IO(';', 108)` — command number 108, not 104, and simple `_IO`
encoding, not `_IOWR` with size.

**Fix:**

```rust
// Before (incorrect — encodes _IOWR with wrong command number)
const VFIO_DEVICE_GET_REGION_INFO: c_ulong = 0xc018_3b68;

// After (correct — _IO(';', VFIO_BASE + 8) = _IO(0x3b, 108))
const VFIO_DEVICE_GET_REGION_INFO: c_ulong = ((b';' as c_ulong) << 8) | 108;
```

The reference implementation in `vfio/ioctls.rs` already had the correct encoding.
The bug was in `mmio.rs`, which had a duplicate definition with a stale value.

### User-Level Access

A udev rule eliminates root requirements:

```udev
ACTION=="add", SUBSYSTEM=="pci", ATTR{vendor}=="0x1e7c", ATTR{device}=="0xbca1", \
  RUN+="/bin/sh -c 'echo 1e7c bca1 > /sys/bus/pci/drivers/vfio-pci/new_id 2>/dev/null; \
  echo %k > /sys/bus/pci/drivers/vfio-pci/bind 2>/dev/null'"
SUBSYSTEM=="vfio", MODE="0666"
```

After installation at `/etc/udev/rules.d/99-akida-vfio.rules`, the device is
automatically bound to `vfio-pci` on boot and accessible to any user.

---

## 4. Live Hardware Validation

Measured on AKD1000 (PCIe x1 Gen2, IOMMU group 92, Apr 2026):

| Metric | Value |
|--------|-------|
| NPUs discovered (VFIO BAR0) | 80 |
| SRAM mapped (BAR1) | 10 MB |
| Vendor ID | `0x1e7c` (BrainChip) |
| Device ID | `0xbca1` (AKD1000) |
| Single inference latency | 54 µs (18,500 Hz) |
| Energy per inference | 1.4 µJ |
| Batch=8 throughput | 20,700 inferences/s |
| DMA sustained throughput | 37 MB/s |
| Online weight swap | 86 µs |
| Production calls (24h lattice QCD) | 5,978 |
| Multi-tenant NP packing (7 systems) | 814 / 1,000 NPs |

### 10 BEYOND_SDK Discoveries

The driver revealed capabilities undocumented in BrainChip's SDK:

1. `InputConv` accepts any channel count (1–64 tested), not just 1 or 3
2. FC layers merge via SkipDMA into a single hardware pass
3. Batch=8 amortizes PCIe overhead: 948 → 390 µs/sample (2.4×)
4. Three clock modes: Performance / Economy / LowPower
5. FC width tested to 8,192+ neurons (SRAM-limited only)
6. `set_variable()` updates weights live (~86 µs, no reprogram)
7. Board power floor is 900 mW; chip compute is below noise floor
8. BAR1 exposes 16 GB address space (vs. documented 8 MB SRAM)
9. FlatBuffer program structure: `program_info` + `program_data`
10. C++ engine internals: SkipDMA, 51-bit threshold SRAM, `program_external()`

---

## 5. Connection to the Sovereign Compute Pipeline

{{ entity(name="rustchip") }} is a standalone extraction from the sovereign compute trio:

| Primal | Role | Relationship to rustChip |
|--------|------|--------------------------|
| {{ entity(name="toadstool") }} | **WHERE** — dispatch | Parent: rustChip's NPU crates are extracted from toadStool's neuromorphic layer |
| {{ entity(name="coralreef") }} | **HOW** — compile | Pattern: rustChip's VFIO backend mirrors coralReef's ember/glowplug architecture |
| {{ entity(name="barracuda") }} | **WHAT** — compute | Consumer: barraCuda shaders produce `&[f32]` that rustChip's NPU classifies |

### Spring Integration

| Spring | How it uses the NPU |
|--------|---------------------|
| {{ entity(name="hotspring") }} | Lattice QCD — ESN steering of HMC sampling (Exp 022, 5,978 calls, 63% thermalization savings) |
| {{ entity(name="airspring") }} | Agricultural IoT — crop classifier hot-swap, 20,545 Hz streaming, seasonal weight evolution |
| {{ entity(name="wetspring") }} | Sentinel microbe inference — domain-shift detection, adaptive recovery |

### Data Flow

```
barraCuda GPU shader → &[f32] → rustChip NPU inference → &[f32] → application
```

No compile-time dependency between any of these. The interface is always a
CPU-resident float slice. The integration is a runtime data handoff.

---

## 6. Crate Architecture

```
rustChip/crates/
├── akida-chip      silicon model: register map, NP mesh, BAR layout, SRAM model
├── akida-driver    full driver: VFIO, kernel, userspace, software backends
├── akida-models    FBZ parser, ProgramBuilder, model zoo interface
├── akida-bench     benchmark suite, hardware experiments
└── akida-cli       command-line tool (enumerate, bind-vfio, verify, probe)
```

All crates enforce `#![deny(unsafe_code)]` at the crate level. Targeted
`#[allow(unsafe_code)]` is applied only to VFIO ioctl wrappers and
memory-mapped I/O modules, with safety invariants documented at each site.

---

## 7. Licensing and Diplomacy

{{ entity(name="rustchip") }} is scyBorg-licensed:

| Layer | License |
|-------|---------|
| Code | AGPL-3.0-or-later |
| Documentation | CC-BY-SA-4.0 |
| Game mechanics | ORC |

The symbiotic exception protocol offers hardware partners (BrainChip, future
NPU vendors) linking exceptions in exchange for silicon documentation or
hardware access. This is license-as-diplomacy: the driver's existence
demonstrates capability; the open license invites collaboration rather than
demanding it.

---

## References

- [rustChip repository](https://github.com/syntheticChemistry/rustChip) — the driver
- [toadStool](https://github.com/ecoPrimals/toadStool) — parent sovereign compute primal
- [coralReef](https://github.com/ecoPrimals/coralReef) — sovereign GPU compiler (VFIO reference)
- [barraCuda](https://github.com/ecoPrimals/barraCuda) — sovereign math engine
- [hotSpring](https://github.com/syntheticChemistry/hotSpring) — lattice QCD validation
- [BrainChip Akida documentation](https://doc.brainchipinc.com/) — vendor reference
- [Linux VFIO documentation](https://docs.kernel.org/driver-api/vfio.html) — kernel API
