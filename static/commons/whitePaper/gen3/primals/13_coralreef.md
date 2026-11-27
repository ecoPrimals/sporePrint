# coralReef — Sovereign GPU Shader Compiler

## Identity

| Field | Value |
|-------|-------|
| Phase | 1 (Foundation) |
| Domain | Shader compilation, GPU code generation |
| Rust lines | ~120,000 |
| Tests | 1,487 |
| Crates | 11 |
| JSON-RPC methods | 4 |
| AGPL-3.0 | Yes |

## What It Is

coralReef compiles WGSL, SPIR-V, and GLSL 450 shader source into native GPU binaries for NVIDIA and AMD hardware. It is a sovereign Rust GPU compiler — no LLVM, no Mesa, no vendor SDK. The entire pipeline from shader IR to machine code is Pure Rust.

The compiler produces native binaries for NVIDIA SM70–SM89 and AMD RDNA2 (GFX1030). It handles f64 transcendental lowering, FMA policy, register allocation, and instruction scheduling. coralDriver provides userspace GPU dispatch via DRM ioctl — AMD amdgpu and NVIDIA nouveau/nvidia-drm — without linking any vendor libraries.

## Why It Exists

When BarraCuda discovered that consumer GPUs expose native f64 via Vulkan's `SHADER_F64` extension, the ecosystem needed a way to compile f64 shaders to native GPU code without depending on NVIDIA's CUDA/NVCC toolchain or AMD's ROCm/HIP stack. The Pure Rust constraint prohibited both.

coralReef was split from ToadStool to own shader compilation as a separate concern from compute dispatch. ToadStool discovers hardware and manages workload queues. coralReef takes WGSL source and produces the binary that ToadStool dispatches. The boundary is clean: ToadStool never parses shaders; coralReef never talks to hardware schedulers.

The f64 discovery forced the split. Compiling double-precision transcendentals (`exp2`, `log2`, `sin`, `cos`, `sqrt`, `rcp`, and their compositions) to native GPU instructions requires a real compiler backend — not a pass-through to `naga` + `wgpu`. coralReef's `lower_f64` pass decomposes f64 operations into sequences the hardware can execute, with FMA policy control to match IEEE 754 rounding.

## Key Capabilities

### Namespaces (JSON-RPC)

| Namespace | Operations | Purpose |
|-----------|-----------|---------|
| `shader.compile.spirv` | Compile SPIR-V module | SPIR-V → native GPU binary |
| `shader.compile.wgsl` | Compile WGSL source | WGSL → native GPU binary |
| `shader.compile.status` | Health check | Compiler readiness and GPU detection |
| `shader.compile.capabilities` | List targets | Supported GPU architectures |

### Compilation Pipeline

```
WGSL / SPIR-V / GLSL 450
        │
        ▼
    naga IR (frontend)
        │
        ▼
    lower_f64 (f64 transcendental decomposition)
        │
        ▼
    optimizer passes (DCE, CSE, constant folding)
        │
        ▼
    register allocation (linear scan)
        │
        ▼
    vendor encoding (SM70–SM89 / GFX1030)
        │
        ▼
    native GPU binary
```

### f64 Transcendentals

coralReef implements 16 f64 transcendental functions as lowering passes:

`sqrt`, `rcp`, `exp2`, `log2`, `sin`, `cos`, `exp`, `log`, `pow`, `tan`, `atan`, `asin`, `acos`, `sinh`, `cosh`, `tanh`, plus `Complex64` operations.

These are not library calls — they are instruction sequences emitted directly into the native binary, matching the precision that BarraCuda's WGSL shaders require for scientific computing.

### Userspace GPU Dispatch (coralDriver)

coralDriver provides DRM-ioctl-based GPU access without vendor userspace libraries:

| Driver | Hardware | Method |
|--------|----------|--------|
| `amdgpu` | AMD RDNA2+ | GEM buffers, PM4 command stream, CS ioctl, fence |
| `nouveau` | NVIDIA (open) | Preferred for sovereignty |
| `nvidia-drm` | NVIDIA (proprietary) | Fallback when nouveau unavailable |

Driver preference is configurable: `nouveau → amdgpu → nvidia-drm` (sovereignty-first ordering).

## How It Connects

```
BarraCuda ──(WGSL source)──▶ coralReef ──(native binary)──▶ ToadStool
                                │                               │
                                ▼                               ▼
                         coralDriver ──(DRM ioctl)──▶ GPU hardware
```

### Key Relationships

- **BarraCuda** produces WGSL shaders that coralReef compiles — 93 cross-spring shaders absorbed, 84 compiling for SM70
- **ToadStool** dispatches the native binaries that coralReef produces; `coral-gpu` provides the unified compile-then-dispatch API
- **Springs** (hotSpring, wetSpring, neuralSpring, airSpring, groundSpring) generate f64 shader workloads that flow through BarraCuda → coralReef → ToadStool → hardware

## Internal Architecture

11 crates organized as a Cargo workspace:

- `coralreef-core` — Primal lifecycle, health, CLI (`server`/`compile`/`doctor`), JSON-RPC + tarpc IPC, FMA control
- `coral-reef` — Compiler core: naga frontend, IR, `lower_f64`, optimizer passes, NVIDIA/AMD codegen, 24 spring absorption tests
- `coral-driver` — Userspace GPU dispatch via DRM ioctl (AMD amdgpu, NVIDIA nouveau/nvidia-drm)
- `coral-gpu` — Unified compile + dispatch API, multi-GPU auto-detect, `DriverPreference`
- `coral-reef-bitview` — `BitViewable`/`TypedBitField` for compile-time safe bit-field access in ISA encoding
- `coral-reef-isa` — Instruction encoding tables and latencies (SM30–SM120, AMD RDNA2)
- `coral-reef-stubs` — Pure Rust replacements for CFG, BitSet, dataflow, SmallVec, fxhash (zero external deps)
- `nak-ir-proc` — Proc-macro derives for IR types
- `primal-rpc-client` — JSON-RPC client for primal-to-primal calls (shared utility)
- `amd-isa-gen` — ISA table generator tool (Pure Rust, code generation)

## What Makes It Interesting

coralReef is a Rust GPU compiler that produces native binaries without invoking any vendor toolchain. The `lower_f64` pass is the key innovation — it decomposes double-precision transcendentals into instruction sequences that consumer GPUs can execute natively, unlocking the f64 capability that CUDA actively throttles on GeForce hardware.

The 93 cross-spring WGSL shaders represent real scientific workloads — Yukawa force, lattice QCD, Anderson localization, spectral cosine — compiled to native GPU code through a Pure Rust pipeline. Each compiled shader is evidence that sovereign compute (no vendor SDK, no C dependencies) can do real science.

Phase 10 (March 2026) reached multi-device compilation across NVIDIA and AMD targets, with AMD end-to-end validation on RX 6950 XT hardware. Coverage stands at 63% with a 90% target.
