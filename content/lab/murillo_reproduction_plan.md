+++
title = "Murillo Reproduction Plan"
description = "Canonical two-phase reproduction methodology — all phases complete. 22 papers, ~700 checks, 39/39 suites."
weight = 26
date = 2026-07-09

[taxonomies]
primals = ["barracuda", "toadstool"]
springs = ["hotspring"]
+++

{{ maturity(level="architectural") }}

**Status**: ALL PHASES COMPLETE — 22 papers, ~700 checks, 39/39 validation suites. Exp 022 (live NPU, 32⁴ production) finished Feb 27.  
**Purpose**: Reproduce published computational physics work from the Murillo Group (MSU) on consumer hardware, then re-execute on BarraCuda's Pure Rust vendor-agnostic GPU compute layer  
**Last Updated**: February 27, 2026

---

## The Two-Phase Approach

Every reproduction study follows the same pattern:

1. **Phase A - Reproduce in Python**: Run the original code (Sarkas, mystic, etc.) on our hardware to validate correctness. Match published results. This proves the HPC produces correct physics and gives us a performance baseline using the traditional scientific Python stack (NumPy, Numba, SciPy).

2. **Phase B - Execute on BarraCuda**: Re-implement the compute-intensive kernels as WGSL shaders running through ToadStool's BarraCuda engine. Same physics, same math, but in Pure Rust with GPU execution on **any vendor** - NVIDIA, AMD, Intel, whatever has a WebGPU-compatible driver. No CUDA lock-in. No Python interpreter. No Numba JIT. No C/Fortran FFI. Just Rust dispatching WGSL shaders to whatever GPU is available.

**Why this matters**: Scientific computing is overwhelmingly locked to Python + CUDA. Sarkas itself acknowledges this - it's "entirely written in Python without calls to C/Fortran hence avoiding a two-language problem." But it still depends on NumPy (C underneath), Numba (LLVM underneath), and FFTW3 (C/Fortran). BarraCuda actually solves the problem Sarkas is trying to solve: a single-language compute stack that runs on any hardware. And it does it in a compiled, memory-safe language.

The Phase A → Phase B comparison is itself a research contribution: **can a Pure Rust GPU compute layer match or exceed the performance of the established Python scientific stack for real plasma physics workloads?**

---

## BarraCuda: What We Have

ToadStool's BarraCuda submodule has evolved into a comprehensive GPU compute engine. Current inventory from the latest `git pull`:

| Category | Count | Key Operations |
|----------|-------|----------------|
| **Rust ops** | 165+ | Compute modules with CPU fallback |
| **WGSL shaders** | 226+ | GPU compute kernels via WebGPU |
| **Core math** | ~30 | sin, cos, exp, sqrt, pow, log, erf, erfc, lgamma, frac, acos, asin, atan, sinh, cosh, acosh, etc. |
| **Linear algebra** | ~15 | matmul (including fp64), einsum, inverse, determinant, matrix_power, outer_product, tensor_dot, cross_product |
| **Reductions** | ~20 | sum, mean, std, variance, min, max, norm, prod, cumsum, cumprod, logsumexp, prefix_sum, argmin, argmax |
| **Distance/search** | ~8 | cdist, pairwise_distance, cosine_similarity, searchsorted, sort, argsort, unique, histc |
| **Convolution/pooling** | ~20 | conv2d, deformable_conv2d, gated_conv2d, octave_conv2d, grouped_conv2d, separable_conv2d, avg_pool, max_pool, etc. |
| **Attention/transformers** | ~8 | flash_attention, scaled_dot_product, multi_head, grouped_query, local_attention, cross_attention, sparse_attention |
| **Graph neural networks** | ~8 | gcn_conv, gat_conv, gin_conv, sage_conv, edge_conv, graph_norm, graph_batch_norm, message_passing |
| **FHE operations** | ~8 | NTT, INTT, key_switch, modulus_switch, pointwise_mul, extract, rotate, fast_poly_mul |
| **Infrastructure** | 3 | `unified_hardware.rs` (device abstraction), `unified_math.rs` (op dispatch), `scheduler.rs` (workload routing) |

**Key for MD physics**: pairwise_distance, cdist, sum/mean reductions, sort/searchsorted (neighbor lists), einsum (tensor contractions), matmul_fp64 (double precision forces), cumsum/prefix_sum (integration), erf/erfc (Coulomb screening functions), histc (radial distribution g(r)).

**Key for surrogate learning**: full neural network stack (linear, conv, attention, normalization, activation functions, optimizers including SGD/Adam/NAdam/RMSProp/AdaBound/AdaFactor/RAdam/LAMB), loss functions, graph neural networks.

---

## Background

Professor Michael Murillo leads the Murillo Group in MSU's Department of Computational Mathematics, Science and Engineering (CMSE). His research spans computational plasma physics (strongly coupled Coulomb systems, inertial confinement fusion, warm dense matter) and agent-based modeling (infectious disease dynamics). He is a Fellow of the American Physical Society with 80+ peer-reviewed publications and a career that began at Los Alamos National Laboratory.

**Murillo Group resources (all open source)**:
- **Sarkas**: Pure Python molecular dynamics for dense plasmas (MIT License) - [github.com/murillo-group/sarkas](https://github.com/murillo-group/sarkas)
- **Dense Plasma Properties Database**: Open repository of plasma simulation data (VACF, g(r), EOS, ionization states) - [github.com/MurilloGroupMSU/Dense-Plasma-Properties-Database](https://github.com/MurilloGroupMSU/Dense-Plasma-Properties-Database)
- **Two-Temperature Model**: UCLA-MSU collaboration for plasma equilibration modeling - [github.com/MurilloGroupMSU/Two-Temperature-Model](https://github.com/MurilloGroupMSU/Two-Temperature-Model) (updated January 2025)
- **BIM_HNC**: Binary Ionic Mixture structure in modified Hypernetted Chain approximation (Fortran)
- **Thomas-Fermi Multispecies Ionization**: Jupyter notebook-based ionization calculations

**Key publication**: "Efficient learning of accurate surrogates for simulations of complex systems" - *Nature Machine Intelligence*, May 2024. Introduces optimizer-driven sampling for training ML surrogates of expensive simulations, demonstrated on nuclear equation-of-state models.

---

## Reproduction Study 1: Sarkas Molecular Dynamics

### What

Reproduce Sarkas plasma simulations on the basement HPC, then port the core MD compute kernels to BarraCuda for vendor-agnostic GPU execution.

### Phase A: Python Reproduction (Validate Correctness)

Run Sarkas examples (Yukawa potentials, Coulomb systems, binary ionic mixtures, ultracold neutral plasmas) on the basement HPC. Validate results against the Dense Plasma Properties Database reference data. Benchmark CPU performance across architectures.

**Sarkas computational pipeline** (what we need to understand before porting):
1. **Initialization**: Particle positions/velocities from configuration
2. **Force calculation**: Pairwise particle interactions (Coulomb, Yukawa, screened potentials) - **the hot loop**
3. **Long-range forces**: PPPM/Ewald via FFT (uses FFTW3/pyfftw) - **most compute-intensive**
4. **Neighbor lists**: Cell-linked lists or Verlet lists (sort/search)
5. **Integration**: Velocity-Verlet time stepping (basic arithmetic)
6. **Diagnostics**: VACF, g(r), structure factors, EOS (reductions + FFT)

The Python stack for each: NumPy arrays → Numba JIT for loops → pyfftw for FFTs → SciPy for special functions.

**Hardware targets**:

| Gate | CPU | Role |
|------|-----|------|
| Strandgate | Dual EPYC 7452 (64c) | Primary MD, massive parallelism baseline |
| Northgate | i9-14900K | Fast single-thread comparison |
| Southgate | 5800X3D | 3D V-Cache neighbor list performance |

**Deliverables**:
- Sarkas tutorials validated against published reference data
- CPU performance comparison: EPYC 64c vs i9-14900K vs 5800X3D (same simulation, same parameters)
- Profiling data: where does Sarkas spend time? (force calc vs FFT vs neighbor list vs integration)

### Phase B: BarraCuda Port (Vendor-Agnostic GPU Physics)

Take the profiled hot paths from Phase A and implement them as WGSL shaders dispatched through BarraCuda. The goal is a Pure Rust MD engine that runs on any GPU.

**Kernel mapping** (Sarkas Python → BarraCuda WGSL):

| MD Operation | Sarkas (Python) | BarraCuda (WGSL) | Status |
|-------------|-----------------|-------------------|--------|
| Pairwise forces | NumPy broadcasting + Numba | Custom f64 Yukawa force shader (PBC, PE) | **DONE** (Phase C, 9/9 validated) |
| Coulomb screening | SciPy `erfc()` | `erfc.wgsl` + `erf.wgsl` | **Ready** |
| Neighbor lists | NumPy sort + cell lists | CPU-managed cell-list + GPU force compute | **DONE** (Phase C) |
| Velocity-Verlet | NumPy arithmetic | Split f64 WGSL: half-kick + drift/PBC + second half-kick | **DONE** (Phase C, 0.000% drift) |
| Berendsen thermostat | NumPy velocity rescaling | f64 WGSL velocity rescaling shader | **DONE** (Phase C) |
| Reductions (energy, pressure) | NumPy `sum()`, `mean()` | f64 WGSL kinetic energy reduction | **DONE** (Phase C) |
| Radial distribution g(r) | NumPy `histogram()` | f64 WGSL RDF histogram with atomicAdd | **DONE** (Phase C) |
| Structure factors | pyfftw FFT | NTT/INTT shaders exist; true FFT shader needed | **Partial** (NTT ≠ FFT, but structure exists) |
| Tensor contractions | NumPy `einsum()` | `einsum.wgsl` | **Ready** |
| Double-precision forces | NumPy float64 | SHADER_F64 + `math_f64.wgsl` (27 transcendental functions) | **DONE** (Phase C, sub-ULP precision) |

**Key engineering challenge**: FFT. Sarkas uses FFTW3 for Particle-Particle Particle-Mesh (PPPM) long-range force decomposition. BarraCuda has NTT/INTT (number theoretic transform - for FHE), but a true complex FFT shader is needed for PPPM. This is the primary new development required.

**Hardware targets** (Phase B runs on GPUs):

| Gate | GPU | Role |
|------|-----|------|
| Northgate | RTX 5090 | Primary BarraCuda execution, best single-GPU |
| Strandgate | RTX 3090 + RX 6950 XT | Cross-vendor comparison (NVIDIA vs AMD, same shader) |
| Eastgate | RTX 4070 | Mid-tier comparison |

**Deliverables**:
- Pure Rust MD force kernel running on GPU via WGSL
- Same physics producing identical results to Sarkas Python (bit-for-bit or within float tolerance)
- Performance comparison: Python/NumPy/Numba on 64 CPU cores vs BarraCuda on RTX 5090 vs RTX 3090 vs RX 6950 XT
- **The headline result**: same plasma simulation running on NVIDIA and AMD from the same shader, no CUDA, no ROCm, no vendor SDK
- NPU exploration: can Akida accelerate any subproblem (force classification, surrogate potential)?

### Available Data

- **Sarkas examples**: Yukawa potentials, Coulomb systems, binary ionic mixtures, ultracold neutral plasmas
- **Dense Plasma Properties Database**: Reference VACF, g(r), EOS data for validation (14+ subdirectories)
- **Two-Temperature Model**: Jupyter notebooks with UCLA collaboration data

---

## Reproduction Study 2: Surrogate Learning (Nature Machine Intelligence 2024)

### What

Reproduce the optimizer-driven sampling method from "Efficient learning of accurate surrogates for simulations of complex systems" on our hardware, then train the same surrogates entirely through BarraCuda's neural network stack.

### Phase A: Python Reproduction (Validate Results)

Run the Code Ocean capsule on our hardware. Reproduce the key figures. Benchmark training on consumer GPUs vs the original compute environment.

The paper uses:
- **mystic** optimizer for directed sampling (constrained optimization framework)
- Standard ML training: RBF networks, neural networks
- Nuclear EOS models (from CompOSE database and published tables)

**Available Data (Open Access)**:

| Resource | URL | Format |
|----------|-----|--------|
| **Paper** | [Nature Machine Intelligence, May 2024](https://doi.org/10.1038/s42256-024-00839-1) | Journal article |
| **Datasets** | [Zenodo: 10.5281/zenodo.10908462](https://doi.org/10.5281/zenodo.10908462) | Directed sampling datasets |
| **Code** | [Code Ocean: 10.24433/CO.1152070.v1](https://doi.org/10.24433/CO.1152070.v1) | Reproducible capsule |
| **Optimizer** | [mystic](https://pypi.org/project/mystic/) (`pip install mystic`) | Python package |

**Deliverables**:
- Reproduction of paper's key figures on consumer GPU hardware
- Training time comparison: RTX 5090 vs RTX 3090 (Python/PyTorch baseline)

### Phase B: BarraCuda Surrogate Training (Pure Rust ML)

Re-implement the surrogate training pipeline entirely in BarraCuda. The neural network components already exist:

| Surrogate Component | Python Stack | BarraCuda | Status |
|--------------------|-------------|-----------|--------|
| Linear layers | PyTorch `nn.Linear` | `linear.wgsl` | **Ready** |
| Activation functions | PyTorch GELU/ReLU/etc. | `gelu.wgsl`, `silu.wgsl`, `mish.wgsl`, etc. (30+ activations) | **Ready** |
| Loss computation | PyTorch loss functions | `l1_loss.wgsl`, `kldiv_loss.wgsl`, `nll_loss.wgsl`, etc. | **Ready** |
| Optimizers | PyTorch Adam/SGD | `nadam/`, `sgd/`, `rmsprop/`, `adabound.wgsl`, `adafactor.wgsl`, `radam.wgsl`, `lamb.wgsl` | **Ready** |
| Normalization | PyTorch LayerNorm/BatchNorm | `layer_norm.wgsl`, `batch_norm2d.wgsl`, `group_norm.wgsl`, `instance_norm.wgsl` | **Ready** |
| Reductions/statistics | NumPy/PyTorch | Full reduction stack (sum, mean, std, variance, min, max, norm) | **Ready** |
| Attention (if needed) | PyTorch MHA | `flash_attention.wgsl`, `scaled_dot_product_attention/`, `multi_head_attention/` | **Ready** |
| RBF networks | SciPy/custom | Custom kernel needed (but `pairwise_distance.wgsl` + `exp.wgsl` = RBF basis) | **Composable** |
| Graph nets (if needed) | PyG | `gcn_conv.wgsl`, `gat_conv.wgsl`, `sage_conv.wgsl`, `message_passing.wgsl` | **Ready** |

The optimizer-driven sampling loop itself (mystic) is CPU-side logic. The compute-intensive part - training the neural surrogate on each batch of directed samples - is what moves to BarraCuda.

**Hardware targets** (Phase B runs on GPUs):

| Gate | GPU | Role |
|------|-----|------|
| Northgate | RTX 5090 | Primary BarraCuda surrogate training |
| Strandgate | RTX 3090 + RX 6950 XT | Cross-vendor: same training, NVIDIA vs AMD |
| Eastgate | RTX 4070 | Mid-tier comparison |

**Deliverables**:
- Same surrogate model trained via BarraCuda producing equivalent accuracy to Python/PyTorch
- Training time comparison: PyTorch on RTX 5090 vs BarraCuda on RTX 5090 vs BarraCuda on RX 6950 XT
- **The headline result**: scientific ML surrogate trained entirely in Pure Rust, running on any GPU vendor, no Python, no PyTorch, no CUDA
- Cross-vendor accuracy validation: NVIDIA-trained surrogate vs AMD-trained surrogate produce identical predictions

---

## Reproduction Study 3: Agent-Based Modeling (Infectious Disease)

### What

Run Murillo Group's agent-based influenza models on the basement HPC, then explore BarraCuda GPU-accelerated agent simulation.

### Phase A: Python Reproduction (CPU Scaling)

Agent-based models are inherently parallelizable and memory-intensive. Strandgate's 64-core EPYC with 256GB ECC is purpose-built for this.

**Why this study matters beyond benchmarks**:
- Agent-based modeling of infectious disease has structural similarities to the primal coordination model in ecoPrimals. Agents interact through local rules (like primals coordinating through IPC), and macro behavior emerges from micro interactions (like NUCLEUS emerging from primal composition). This is the same computational paradigm.
- The author's microbiology training (Medical Microbiology, Virology, Molecular Pathogenesis) provides domain knowledge that most computational physicists don't have. Understanding influenza dynamics at the cellular level can inform model validation and parameterization.

**Available Data**:
- Murillo Group publications on agent-based influenza modeling
- Standard epidemiological datasets for validation (CDC FluView, WHO)
- Specific code/data to be requested from Murillo Group
- **Note**: Agent-based model code may not be open-source. This study depends on collaboration access.

**Deliverables**:
- Large-scale agent simulation runs on 64-core EPYC
- CPU scaling: how does performance change from 8 → 16 → 32 → 64 cores?
- Comparison vs i9-14900K (fewer cores, higher clock)

### Phase B: BarraCuda Agent Simulation (GPU-Parallel Agents)

Agent-based models map naturally to GPU compute: each agent is an independent work item, interactions are local (pairwise distance), and state updates are parallel. BarraCuda already has the primitives:

| ABM Operation | BarraCuda | Notes |
|--------------|-----------|-------|
| Agent distance/contact | `pairwise_distance.wgsl`, `cdist.wgsl` | Spatial proximity = contact network |
| State transitions | `masked_fill.wgsl`, `where_op.wgsl` | Conditional state updates (S→I→R) |
| Population statistics | `sum_reduce.wgsl`, `mean_reduce.wgsl`, `histc.wgsl` | Epidemic curves, age distributions |
| Stochastic events | Existing random ops | Infection probability, recovery timing |
| Spatial grid | `scatter.wgsl`, `gather.wgsl` | Agent-to-grid mapping |
| Graph structure | `message_passing.wgsl`, `gcn_conv.wgsl` | Contact network as graph |

**The question**: can hundreds of thousands of agents be simulated per GPU dispatch, with contact networks evaluated via `message_passing.wgsl`? If so, BarraCuda turns ABMs from a CPU-scaling problem into a GPU-throughput problem.

**Hardware targets**:

| Gate | Specs | Role |
|------|-------|------|
| Strandgate | 64c EPYC + RTX 3090 | CPU baseline + GPU comparison on same node |
| Northgate | i9-14900K + RTX 5090 | Best GPU execution |
| Strandgate | RX 6950 XT | Cross-vendor ABM on AMD |

**Deliverables**:
- GPU-accelerated agent simulation via BarraCuda
- Population scaling: how many agents can BarraCuda handle per timestep on RTX 5090 vs 64 CPU cores?
- Cross-domain analysis: structural parallels between ABM and primal coordination
- NPU exploration: can Akida's spiking networks model agent decision-making (state transition as spike event)?

---

## Timeline

### Completed

| Task | Status |
|------|--------|
| BarraCuda op inventory and capability mapping | **Done** |
| Identify all Murillo Group repos and data sources | **Done** |
| Locate Zenodo datasets + Code Ocean capsule | **Done** (Code Ocean gated — rebuilt from scratch) |
| Document BarraCuda → Sarkas kernel mapping | **Done** |
| Read Sarkas docs/tutorials thoroughly | **Done** |
| Read Nature MI paper in full | **Done** |
| Download Zenodo datasets locally | **Done** |
| Clone all Murillo Group repos | **Done** |
| Identify FFT gap in BarraCuda (needed for PPPM) | **Identified** |
| **Phase A: Sarkas MD** (12 DSF cases, 60 checks) | **DONE — 60/60 pass** |
| **Phase A: TTM** (3 species, local + hydro) | **DONE — 6/6 pass** |
| **Phase A: Surrogate learning** (9 benchmarks + EOS) | **DONE — 15/15 pass** |
| **Phase A: Nuclear EOS** (Python L1 + L2) | **DONE — L1 chi2=6.62, L2 chi2=1.93** |
| **Phase B: Nuclear EOS** (BarraCuda L1 + L2) | **DONE — L1 chi2=2.27 (478x faster), L2 chi2=16.11** |
| **Phase B: GPU FP64 validation** | **DONE — 4.55e-13 MeV max error** |
| **Phase C: GPU MD** (9 PP Yukawa, f64 WGSL) | **DONE — 9/9 pass, 0.000% drift, 3.7x GPU speedup** |
| **Phase D: Native f64 builtins + N-scaling** | **DONE — 2-6× throughput. N=10k in 5.3 min. 0.000% drift** |
| **Phase E: Paper-parity long run** (9 cases, 80k steps) | **DONE — 9/9, 3.66 hrs, $0.044. Cell-list 4.1× faster** |
| **Paper 5: Stanton-Murillo Transport** | **DONE — 13/13 Green-Kubo D*/η*/λ*. GPU transport pipeline** |
| **Paper 6: Murillo-Weisheit Screened Coulomb** | **DONE — 23/23 Sturm bisection, Δ≈10⁻¹²** |
| **Papers 7-10, 13: Bazavov Lattice QCD** | **DONE — HotQCD EOS, pure gauge, dynamical, Abelian Higgs** |
| **Papers 14-22: Kachkovskiy Spectral Theory** | **DONE — 45/45 Anderson, Hofstadter, GPU Lanczos** |
| **DF64 Core Streaming Discovery** | **DONE — 9.9× native f64 throughput on FP32 cores (3.24 TFLOPS)** |
| **Exp 013: Production 32⁴ β-scan** | **DONE — β_c=5.69 (known 5.692). 13.6h, $0.58. Deconfinement** |
| **Exp 020-021: NPU Characterization + Cross-Substrate** | **DONE — 6 SDK assumptions overturned. ESN 9,017× less energy** |
| **Exp 022: Live NPU Mixed Pipeline (32⁴)** | **DONE — 10 β pts, 5,900 meas, 63% therm savings, 5,978 NPU calls** |
| **Total papers reproduced** | **22** |
| **Total acceptance checks** | **~700** |
| **Total validation suites** | **39/39** |

### Post-NUCLEUS (Phase A - Python Reproduction)

| Task | Depends On |
|------|-----------|
| Sarkas quickstart validation | NUCLEUS stable, Westgate cold storage online |
| Sarkas CPU benchmarks (EPYC vs i9 vs 5800X3D) | Sarkas validated |
| Sarkas profiling (where does time go?) | Sarkas benchmarks |
| Surrogate learning Code Ocean reproduction | Zenodo data downloaded |
| Surrogate learning GPU benchmarks (PyTorch) | Code Ocean validated |
| Agent-based model code acquisition | Murillo collaboration |
| Agent-based model CPU scaling | ABM code available |

### Post-NUCLEUS (Phase B - BarraCuda Execution)

| Task | Depends On |
|------|-----------|
| MD force kernel WGSL shader | Sarkas profiling (know the hot path) |
| FFT WGSL shader (for PPPM) | New BarraCuda development |
| MD validation (BarraCuda vs Sarkas results match) | Force kernel + FFT ready |
| MD cross-vendor benchmark (NVIDIA vs AMD) | MD validation |
| Surrogate training via BarraCuda neural stack | Phase A surrogate results |
| Surrogate cross-vendor training (same model, different GPU) | BarraCuda surrogate working |
| ABM GPU agent simulation | Phase A ABM results |
| ABM population scaling on GPU vs CPU | ABM GPU working |

---

## Infrastructure Dependencies

This is a long-term, whole-ecosystem project. The reproduction studies require a working NUCLEUS deployment before execution can begin at scale.

### Blocking Requirements

1. **NUCLEUS stability**: Tower, Node, and Nest Atomics must be reliably composing before we route data and compute across the mesh.
2. **Westgate as cold data hub**: NestGate must be managing Westgate's 76TB ZFS pool via content-addressed storage. All raw data, simulation output, and datasets land on Westgate. Compute nodes pull data over the mesh.
3. **10G backbone**: The copper 10G NICs are installed on North/South/East/Westgate. Switch is acquired. Cables are the remaining bottleneck. Large MD simulation output and dataset transfers need the bandwidth.
4. **biomeOS coordination**: Workload routing (MD to Strandgate, surrogate training to Northgate, visualization to any available gate) requires biomeOS's Neural API to be functional.

### What Can Be Done Now (Pre-NUCLEUS)

See `MISE_EN_PLACE.md` for the full checklist. Key items:
- Clone repos locally for review
- Download Zenodo datasets
- Install Sarkas on one gate for tutorial validation
- Read the Nature paper thoroughly
- Review Sarkas documentation and tutorials

---

## The Bigger Picture

Sarkas solved one problem: making MD accessible by writing it in pure Python instead of C/Fortran. But it still depends on NumPy (C underneath), Numba (LLVM underneath), pyfftw (FFTW3, C/Fortran underneath), and it's locked to CPUs.

BarraCuda solves the next problem: making GPU compute accessible by writing it in Pure Rust with WGSL shaders that run on **any** GPU vendor. No CUDA. No ROCm. No HIP. No OpenCL. No vendor SDK. One set of shaders, every GPU.

If we can demonstrate that real plasma physics (Sarkas-equivalent simulations) and real scientific ML (surrogate training) run correctly and competitively on BarraCuda, that's not just a reproduction study - it's a demonstration that **scientific computing doesn't have to be locked to interpretive languages or proprietary GPU stacks**.

This connects directly to the ecoPrimals thesis: sovereignty through constraint. BarraCuda constrained itself to WGSL/WebGPU (portable, vendor-neutral) and emerged with 226+ shaders that can run plasma physics on an AMD card or an NVIDIA card from the same binary. The same pattern that produced Taq polymerase from a hot spring produces universal GPU compute from a portability constraint.

---

## Notes

- The FFT gap is **closed** (ToadStool `Fft1DF64`/`Fft3DF64`, roundtrip 1e-10). See [BarraCuda Scientific Compute Gaps](@/technical/barracuda_compute_gaps.md).
- DF64 is a novel discovery with publication potential: "Double-float arithmetic for lattice gauge theory on consumer GPU" (*Computer Physics Communications* or *Physical Review E*).
- NPU adaptive steering is a novel methodology with publication potential: "Neuromorphic co-processing for Monte Carlo simulation" (*Neuromorphic Computing and Engineering*).
- The hotSpring repo (`github.com/syntheticChemistry/hotSpring`) is public and AGPL-3.0 licensed.
- **Exp 022 completed Feb 27, 2026**: First production lattice QCD run with live neuromorphic hardware (AKD1000 via PCIe). ESN weights exported for cross-run learning. 10 NPU-steered β points, 5,900 measurements, 63% thermalization savings.
- **WDM extension (Tier 4)** is the natural next step — Murillo's roadmap paper (arXiv:2505.02494) identifies computational accessibility as the bottleneck. hotSpring already has the primitives (MD, Green-Kubo, FFT, DF64). The gap is real physics: partial ionization, quantum corrections.

---

**See also:**

- [hotSpring Results](@/thesis/08_results_hotspring.md) — thesis chapter on reproduction outcomes
- [Faculty Spring Profiles](@/audience/faculty_spring_profiles.md) — Murillo Group literature context
- [hotSpring Validation Summary](@/lab/hotspring-validation-summary.md) — live validation status and binaries
