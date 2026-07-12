+++
title = "Chapter 7: Experimental Methodology"
description = "The spring framework: Python control to Rust to GPU phased validation across five springs and eight scientific domains."
weight = 7
date = 2026-07-09
+++

{{ maturity(level="architectural") }}

## 7.1 Design Rationale

BarraCuda claims that Pure Rust GPU compute can replace the Python scientific stack. This claim requires evidence from every scientific domain the ecosystem intends to serve. A single-domain validation (e.g., only plasma physics) would prove the kernels work for plasma physics, not that the approach generalizes. The spring framework provides multi-domain validation through a standardized protocol.

**Definition**: A *spring* is a public, AGPL-3.0-licensed repository that takes published, peer-reviewed scientific work and asks: can we reproduce it? First in Python (the original tool), establishing a control. Then in Rust. Then on GPU. If the answers are yes, the science is validated and the BarraCuda kernel is proven correct for that domain.

The name is ecological: springs feed the ecosystem. Each spring produces validated kernels that flow into the primal infrastructure, just as geological springs feed rivers that sustain ecosystems. The springs are also *tests* — acceptance tests for the infrastructure, not the science. The science is already published and peer-reviewed. The question is whether our infrastructure reproduces it.

## 7.2 The Phased Validation Protocol

Every spring follows a standardized phased protocol:

### Phase 0 (Python Control)

Reproduce the published results using Python + NumPy/SciPy — the same language and libraries the original authors used (or could have used). This establishes the control baseline:

- All calculations match published values within stated tolerances
- All figures are reproducible from the control scripts
- The control discovers and documents any bugs in the original code or data

Phase 0 is *not* a trivial step. hotSpring Phase 0 discovered 5 silent bugs in the upstream Sarkas MD code. The control exists independently of the Rust/GPU implementation and validates the science itself.

### Phase 1 (Rust Port)

Port the Phase 0 Python implementation to Rust using BarraCuda's CPU-side crate. Cross-validate every numerical output against the Python control:

- All Rust values match Python within defined tolerance (typically \(10^{-5}\) for f64 operations)
- All Rust tests pass independently of Python
- The Rust implementation has zero unsafe blocks, zero external C dependencies

This proves Rust can express the science correctly. airSpring Phase 1 cross-validated 65 values between Python and Rust, all matching within \(10^{-5}\).

### Phase 2 (GPU Promotion)

Promote compute-intensive operations to BarraCuda's WGSL GPU shaders. Validate GPU output against both Python and Rust CPU:

- GPU results match CPU within IEEE 754 f64 tolerance
- Speedup is measured and reported honestly
- Energy consumption is measured where feasible

This proves the GPU kernels are correct. hotSpring Phase C validated 9 Yukawa OCP cases on the GPU with 0.000% energy drift.

### Phase 3+ (Extensions)

Domain-specific extensions: larger datasets, real-world data, cross-spring connections, faculty paper reproductions. Each extension follows the same control → Rust → GPU validation chain.

## 7.3 What Counts as a "Check"

A *check* is an automated, quantitative validation criterion with a defined tolerance. Checks are binary: pass or fail. There is no subjective assessment, no "looks about right," no manual inspection.

Examples of checks:

| Check Type | Example | Tolerance |
|-----------|---------|-----------|
| Value match | ET₀ = 5.23 mm/day (computed) vs 5.23 mm/day (FAO-56 Example 18) | ±0.01 mm/day |
| Statistical metric | R² ≥ 0.95 against independent dataset | Threshold |
| Physical constraint | Energy drift ≤ 0.01% over 80,000 MD steps | Threshold |
| Cross-validation | Rust value matches Python value | ±1e-5 |
| Trend | Shannon diversity increases monotonically with sequencing depth | Monotonicity |
| GPU parity | GPU output matches CPU output | ±1e-10 |

Every check is implemented as an assertion in either a Python script or a Rust binary. Running the script produces a pass/fail result with no human judgment required.

## 7.4 Spring Inventory

| Spring | Domain | Checks | Phases | Faculty | Repository |
|--------|--------|--------|--------|---------|------------|
| hotSpring | Plasma physics, nuclear structure, lattice QCD, spectral theory, NPU brain | 697+ | A–F + lattice + spectral + NPU + brain | Murillo, Bazavov, Kachkovskiy, R. Anderson | syntheticChemistry/hotSpring |
| airSpring | Evapotranspiration (8 methods), soil moisture, IoT, Richards PDE, immunological Anderson | 2,631+ | 0–3.5+ (Python→Rust→GPU→metalForge→NUCLEUS) | Dong | syntheticChemistry/airSpring |
| wetSpring | 16S metagenomics, Anderson QS, phylogenetics, PFAS, immunological signaling, drug repurposing | 5,421+ | 286 experiments, 52/52 papers, V97c | Waters, Liu, Cahill, Smallwood, Jones, R. Anderson, Gonzales, Lisabeth, Neubig | syntheticChemistry/wetSpring |
| groundSpring | Sensor noise, spectral theory (Anderson, Almost-Mathieu, band edge), transport, quasispecies, rare biosphere, uncertainty | 236+ | 35 experiments across 10 domains, V91 | Bazavov, Waters, Liu, Dolson, Kachkovskiy, R. Anderson | syntheticChemistry/groundSpring |
| neuralSpring | PINN, DeepONet, LSTM, evo comp, spectral, population genomics, dose-response, coralForge (AlphaFold) | 3,200+ | 25+ papers + 5 WDM surrogates + coralForge, V82 | Dolson, Liu, Waters, Bazavov, Kachkovskiy, R. Anderson, Gonzales | syntheticChemistry/neuralSpring |
| **Total** | **8 scientific domains** | **11,161+** | | **13 professors, 8 departments** | |

## 7.5 Literature-Grounded Validation

Each spring validates against published work from peer-reviewed literature. Validation targets are defined by papers and datasets, not by personal endorsements or institutional relationships. This is not a social nicety; it is methodological:

1. **Reproducibility**: Published papers with documented methods provide fixed acceptance criteria that any independent implementation can verify.
2. **Domain expertise**: The cited literature provides domain context — parameters, benchmarks, and expected results — that constrains what "correct" means.
3. **Validation path**: A spring passes when quantitative checks match published results within stated tolerances; no personal confirmation is required.
4. **PhD relevance**: Each spring demonstrates reproducibility of established science in a specific domain, not affiliation with any particular research group.

As of March 2026, springs map to published work across multiple departments and institutions (MSU, Sandia, Carleton College), with 60+ candidate papers identified for future reproduction. MSU Drug Discovery Program publications extended validation targets into pharmacology and high-throughput screening.

## 7.6 Cost Model

Every spring tracks compute cost honestly:

| Spring | Total Compute Cost | Key Hardware | Cost Basis |
|--------|-------------------|-------------|------------|
| hotSpring | ~$0.60 (25 papers) | RTX 3090 + RTX 4070 + Titan V + AKD1000 NPU | Wall-clock × $0.12/kWh |
| airSpring | ~$0.05 | RTX 4070 + Titan V + CPU | Wall-clock × $0.12/kWh |
| wetSpring | ~$0.10 | RTX 4070 for GPU pipeline | Wall-clock × $0.12/kWh |
| groundSpring | ~$0.03 | RTX 4070 + Titan V + AKD1000 NPU | Wall-clock × $0.12/kWh |
| neuralSpring | ~$0.15 | RTX 4070 for GPU validation | Wall-clock × $0.12/kWh |

Total compute cost for 11,161+ checks across 8 scientific domains: approximately **$0.93**. The most expensive single computation (hotSpring 32⁴ quenched β-scan: 12 temperatures, 13.6 hours on RTX 3090) cost $0.58 in electricity.

This cost model is part of the thesis argument: if constrained evolution produces a system that can reproduce published science at $0.01–$0.10 per paper on consumer hardware, the methodology has economic implications for scientific computing accessibility.

## 7.7 Reproducibility

Every spring repository is:

- **Public**: Available on GitHub under `syntheticChemistry/` organization
- **Licensed**: AGPL-3.0 (no proprietary dependencies, no access restrictions)
- **Self-contained**: All data either generated synthetically, downloaded from public APIs, or included in the repository
- **Documented**: Each experiment has a script, a benchmark JSON defining acceptance criteria, and a status document tracking results
- **Runnable**: `python3 script.py` or `cargo run --release --bin validate_*` produces pass/fail output

No institutional access is required. No Code Ocean account. No Fortran compiler. No CUDA installation. The Rust validation binaries require only a Rust toolchain; the GPU binaries additionally require a Vulkan-capable GPU with SHADER_F64 support.

## 7.8 Relationship to the Constrained Evolution Thesis

The springs serve double duty:

1. **They validate the infrastructure.** The 11,161+ checks prove that BarraCuda's kernels, evolved under the Pure Rust / WGSL / f64 constraint, compute real science correctly across 8 domains.

2. **They are evidence for the methodology.** The fact that a single developer produced 11,161+ validated science checks across 8 domains and 70+ papers in ~69 days, using the constrained evolution methodology (AI generation → Rust compilation → test validation), is itself a datum in support of the thesis. The springs are both the experimental apparatus and the experimental result.

This reflexivity is not circular. The science validation is objective: either Yukawa MD energy drift is 0.000% or it isn't. Either FAO-56 ET₀ matches the textbook value or it doesn't. Either the deconfinement transition occurs at β_c=5.69 or it doesn't. The methodology claim is separate: it asserts that the constrained evolution approach enabled the production of the validated code at this velocity. The science stands independent of how it was produced.

---

**See also:**

- [How to Start a Spring](@/methodology/HOW_TO_START_A_SPRING.md) — the operational playbook
- [Spring Catalog](@/architecture/SPRING_CATALOG.md) — complete spring inventory
- Chapters [8](@/thesis/08_results_hotspring.md)–[12](@/thesis/12_results_neuralspring.md) — per-spring results
