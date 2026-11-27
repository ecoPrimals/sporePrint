# barraCuda — Vendor-Agnostic GPU Math Engine

## Identity

| Field | Value |
|-------|-------|
| Phase | 1 (Foundation) |
| Domain | Scientific compute, GPU math, WGSL shader pipelines |
| Rust files | 1,060+ |
| Tests | 3,348+ |
| Crates | 2 |
| WGSL shaders | 786 (production) |
| JSON-RPC methods | 12 |
| AGPL-3.0 | Yes |

## What It Is

barraCuda is the math engine of ecoPrimals. Every GPU-accelerated computation — linear algebra, FFT, molecular dynamics, spectral analysis, tensor operations, lattice QCD — is a WGSL shader pipeline managed by barraCuda. It writes the math; coralReef compiles it; ToadStool dispatches it.

786 production WGSL shaders. DF64 (double-precision emulation on GPUs without native f64). NTT for fully homomorphic encryption. Five-spring ingestion: hotSpring, neuralSpring, wetSpring, airSpring, groundSpring. Zero unsafe code. Zero vendor SDK.

## Why It Exists

barraCuda began as a crate inside ToadStool. As the springs matured, their compute demands grew specific: hotSpring needed Yukawa force kernels and lattice QCD; wetSpring needed biodiversity indices and ODE integrators; neuralSpring needed attention mechanisms and reservoir computing. The math was outgrowing ToadStool's hardware-dispatch mission.

The separation followed the same pattern as every primal split: one responsibility was consuming disproportionate surface area. ToadStool's 50+ crates included BarraCuda's 628+ shaders, and the shader count was growing faster than the infrastructure code. The boundary became clear: ToadStool owns hardware, barraCuda owns math.

barraCuda is now a standalone primal with its own IPC server, health checks, and capability advertisement. It discovers ToadStool for dispatch and coralReef for sovereign compilation, but it can also dispatch through `wgpu` directly when sovereign compilation is not required.

## Key Capabilities

### Namespaces (JSON-RPC)

| Namespace | Operations | Purpose |
|-----------|-----------|---------|
| `barracuda.compute.*` | dispatch | Submit shader workloads |
| `barracuda.tensor.*` | create, matmul | Tensor operations on GPU |
| `barracuda.device.*` | list, probe | Hardware detection and capabilities |
| `barracuda.health.*` | check | Readiness and validation |
| `barracuda.tolerances.*` | get | Numerical precision for named operations |
| `barracuda.validate.*` | gpu_stack | GPU pipeline validation suite |
| `barracuda.fhe.*` | ntt, pointwise_mul | Fully homomorphic encryption |
| `barracuda.primal.*` | info, capabilities | Identity and discovery |

### Math Domains

| Domain | Operations | Spring Consumers |
|--------|-----------|-----------------|
| Linear algebra | GEMM, eigensolvers, SVD, LU, QR, sparse CG/BiCGStab | hotSpring, neuralSpring |
| FFT / NTT | Complex FFT (f32, f64), NTT for FHE | hotSpring (PPPM), BearDog (FHE) |
| Physics | Yukawa force, Velocity Verlet, PBC, cell-list neighbors | hotSpring (MD) |
| Statistics | Reduction, histogram, moments, bootstrap, jackknife | wetSpring, groundSpring |
| Spectral | Anderson localization, Lanczos eigensolver | hotSpring, groundSpring |
| ML | matmul, softmax, attention, ESN reservoir, activations | neuralSpring |
| Special functions | erfc, Bessel, Gamma, Hill kinetics | hotSpring, wetSpring |
| PDE | Finite difference, spectral methods, Richards equation | airSpring |
| Bioinformatics | Smith-Waterman, HMM, phylogenetics | wetSpring, healthSpring |
| Lattice QCD | SU(3) gauge, staggered Dirac, CG solver, HMC | hotSpring |

### DF64 — Double-Precision Emulation

On GPUs without native f64 (Intel Arc, some AMD consumer parts), barraCuda provides DF64: a pair of f32 values carrying ~48 bits of mantissa. The emulation is transparent to shader consumers — the same WGSL source works on both native-f64 and emulated-f64 hardware.

DF64 was originally developed for NTT in FHE (64-bit modular arithmetic on 32-bit-only GPUs). When hotSpring needed f64 physics on consumer hardware, DF64 was already there.

### The NTT-to-FFT Evolution

barraCuda's NTT (Number Theoretic Transform) was built for FHE polynomial multiplication in Z_q. The Cooley-Tukey butterfly structure — stage indexing, stride computation, block decomposition, twiddle lookup — is the same structure as FFT. When hotSpring needed FFT for PPPM electrostatics, the NTT kernel was the FFT kernel with complex twiddle factors instead of modular roots of unity.

The main compute kernels (`fhe_ntt.wgsl` at 263 lines and `fft_1d.wgsl` at 186 lines) share the same computational skeleton. The cryptographic constraint selected for a mathematical universal.

## How It Connects

```
Springs (hotSpring, wetSpring, ...) ──validate──▶ barraCuda
                                                      │
                                                  ┌───┴───┐
                                                  │ WGSL  │
                                                  │shaders│
                                                  └───┬───┘
                                        ┌─────────────┼─────────────┐
                                        ▼             ▼             ▼
                                    coralReef      wgpu          ToadStool
                                   (compile)    (dispatch)     (orchestrate)
```

### Key Relationships

- **coralReef** compiles barraCuda's WGSL shaders to native GPU binaries — 93 cross-spring shaders in the compilation pipeline
- **ToadStool** orchestrates dispatch to hardware; barraCuda discovers ToadStool via capability routing
- **hotSpring** is the largest consumer — lattice QCD, HFB nuclear, spectral, MD, Anderson localization, ESN
- **neuralSpring** uses bio ML, Hill kinetics, HMM, chi-squared, batch IPR
- **wetSpring** uses ODE, MD forces, stress virial, Verlet, fused_map_reduce
- **airSpring** uses Richards PDE, ET0, seasonal pipeline, hydrology
- **groundSpring** uses sensor noise, Anderson Lyapunov, Welford, tolerance architecture
- **healthSpring** uses microbiome, tridiagonal QL
- **ludoSpring** consumes barraCuda through its `barracuda` library crate for Perlin noise, engagement batch, flow evaluation

### Provenance Flow

Springs implement domain primitives. barraCuda absorbs and generalizes the math. Springs then consume upstream barraCuda. The provenance registry tracks shader flows across springs (hotSpring → groundSpring, hotSpring → neuralSpring, etc.).

## Internal Architecture

2 crates organized as a Cargo workspace:

- `barracuda` — Umbrella math crate: 786 WGSL shaders, Rust dispatch, pipeline management. Modules include:
  - `linalg` — Dense and sparse linear algebra (GEMM, eigensolvers, CG, BiCGStab, L-BFGS)
  - `special` — Special functions (erfc, Bessel, Gamma, Hill)
  - `numerical` — FFT, NTT, integration, interpolation
  - `spectral` — Anderson localization, Lanczos
  - `stats` — Reduction, moments, bootstrap, jackknife, diversity indices
  - `tensor` — GPU tensor creation, matmul, activations
  - `nn` / `snn` / `esn` — Neural network, spiking network, echo state network
  - `pde` — Finite difference, spectral PDE solvers
  - `genomics` — Smith-Waterman, HMM, phylogenetics (feature-gated)
  - `ops` — Core GPU operations, pipeline, staging, dispatch
  - `device` — Hardware detection, multi-GPU
  - `shaders/` — 786 WGSL shader files

- `barracuda-core` — Primal lifecycle, IPC (JSON-RPC + tarpc), UniBin CLI (`server`, `doctor`, `version`)

## What Makes It Interesting

barraCuda is the largest collection of WGSL shaders in any open-source project. 786 shaders covering every domain from lattice QCD to FHE to bioinformatics, all running on consumer GPUs via Vulkan — no CUDA, no ROCm.

The five-spring ingestion pattern is unique: each spring validates barraCuda's kernels against published scientific results (0.000% energy drift in Yukawa MD, chi-squared/datum = 2.27 for nuclear EOS, 926x GPU speedup for spectral cosine). The springs are not just consumers — they are acceptance tests. Every validated check is evidence that constrained evolution (Pure Rust, WGSL, no vendor SDK) produced correct scientific computing.

The NVVM device poisoning guard is notable: barraCuda detects when NVIDIA's proprietary driver injects NVVM IR into the compilation pipeline (which would bypass the sovereign compiler path) and fails loudly rather than silently producing incorrect results.
