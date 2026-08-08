+++
title = "Computation Audit Trail: hotSpring QCD"
description = "Full decision history for the lattice QCD computations. PRNG bias discovery, three-path validation methodology, cpu_mom workaround, multi-vendor validation. The novel fermentation transcript."
date = 2026-08-02
weight = 7

[taxonomies]
primals = ["barracuda", "toadstool", "coralreef"]
springs = ["hotspring"]

[extra]
maturity = "live"
validated_on_hardware = true
+++

> **Novel Fermentation Transcript** — This page documents the full
> computational decision history that produced the results in the
> [arXiv draft](/pseudospore/hotspring-qcd-sun-paper/). Every failed path,
> every correction, every validation decision is recorded here. The paper
> presents conclusions; this page shows the process.

---

## Why This Exists

Most papers present polished results. The messy process — the bugs found,
the assumptions that broke, the workarounds that were validated — is usually
invisible. This audit trail makes it visible.

Every decision in the computation pipeline is traceable through:
1. **This page** — narrative decision history with timestamps
2. **The pseudoSpore provenance chain** — cryptographic proof of what was computed
3. **The agent session transcripts** — the actual AI-assisted development sessions
4. **The source code history** — git commits on git.primals.eco

---

## Timeline of Computation Decisions

### Phase 1: Initial GPU HMC Implementation

**Goal**: Implement SU(2) lattice gauge theory HMC on consumer GPUs via WebGPU/WGSL.

**Approach**: Full GPU pipeline — gauge updates, force computation, leapfrog
integration, momentum generation, and Metropolis accept/reject all in WGSL
compute shaders.

**Result**: GPU code ran and produced trajectories. Initial benchmarks showed
significant speedup over CPU. Plaquette values were computed.

### Phase 2: Plaquette Divergence Discovery (P2)

**Observation**: GPU plaquette values at β=2.3 diverged from known SU(2)
Wilson action reference values and from the CPU implementation running
identical algorithms.

**Magnitude**: 570σ deviation at 4⁴ lattice. Not a subtle effect — the
GPU was producing qualitatively different physics.

**Initial hypothesis**: DF64 precision loss in accumulated plaquette sums.

**Investigation**:
- Tested DF64 arithmetic operations individually — all correct to ~14 digits
- Tested plaquette measurement on identical (uploaded) lattice configurations —
  DF64 GPU agreed with f64 CPU to |Δ| ≤ 5.5×10⁻¹⁰
- Tested native f64 GPU shaders — agreed with CPU to machine epsilon (4×10⁻¹⁷)

**Conclusion**: DF64 precision is not the cause. The divergence is upstream
of the plaquette measurement.

### Phase 3: Three-Path Isolation

**Methodology developed**: To isolate the source of divergence, we designed
a three-path comparison:

| Path | What it tests |
|------|--------------|
| **A** — CPU reference (CPU momenta + CPU MD) | Ground truth |
| **B** — Full GPU (GPU momenta + GPU MD) | Full GPU pipeline |
| **C** — Hybrid (CPU momenta → GPU MD) | Isolate PRNG from MD |

**Results**:

| Path comparison | Agreement |
|----------------|-----------|
| A vs C | |Δ|/σ < 1 (statistically identical) |
| A vs B | 570σ deviation |
| B vs C | 570σ deviation |

**Root cause identified**: Since Paths B and C share the identical GPU
molecular dynamics pipeline and differ only in momentum source, the
divergence is conclusively isolated to the **GPU PRNG shader**.

Specifically: the Box-Muller transform in WGSL uses software polyfills
for `log()`, `sqrt()`, and `cos()` transcendental functions. These
polyfills introduce a systematic bias in the momentum distribution that
compounds across the leapfrog trajectory, shifting the equilibrium
plaquette value.

### Phase 4: cpu_mom Workaround

**Solution**: Generate HMC momenta on CPU using validated PRNG (LCG +
Gaussian via standard library), upload to GPU for the leapfrog trajectory.
All other computation remains on GPU.

**Validation**:
- Path C (cpu_mom) agrees with Path A (CPU reference) within 1σ
- Performance overhead: < 0.1% of trajectory time at 16⁴ (momentum
  generation is negligible vs leapfrog integration)
- Cross-GPU agreement: Both RTX 3090 and RX 6950 XT produce identical
  plaquette values with cpu_mom (|Δ|_GPU-GPU = 3.1×10⁻⁹)

**Status**: cpu_mom is the production path. GPU-native PRNG fix
(Philox counter-based, avoiding transcendental polyfills) is in development.

### Phase 5: Multi-Vendor Validation

**Goal**: Prove vendor neutrality — same WGSL shaders, different silicon,
identical physics.

**Setup**: strandGate equipped with both NVIDIA RTX 3090 and AMD RX 6950 XT.
Same thermalized lattice (CPU-generated), same momentum sequences (cpu_mom,
same seed), same WGSL shader source.

**Results**:
- Both GPUs agree on plaquette to 3.1×10⁻⁹ (5 orders below σ_stat)
- Both GPUs show |Δ|/σ < 1 vs CPU reference
- RX 6950 XT is faster than RTX 3090 at tested volumes (RDNA2 scheduling)

**The residual**: The 3.1×10⁻⁹ inter-GPU difference is from cumulative DF64
rounding across ~4,000 integration steps on different FP32 hardware. Both
produce correct samples from the same distribution — the difference is
within DF64 accumulated precision, not a physics disagreement.

### Phase 6: Production Data Generation

**Configuration**:
- Omelyan 2MN integrator, n_md=20, dt=0.02
- 200 thermalization + 200 production trajectories
- β = 2.3 (strong coupling regime)
- 4⁴ and 8⁴ lattice volumes

**Quality metrics**:
- Accept rates: 100% (4⁴), 99.5% (8⁴)
- Autocorrelation: τ_int = 1.63 (4⁴), 3.37 (8⁴)
- ⟨|ΔH|⟩ = 1.1×10⁻³ (4⁴), 4.5×10⁻³ (8⁴)

All metrics consistent with expected SU(2) Wilson action behavior.

---

## What Failed and Why

| What Failed | Why | Resolution |
|-------------|-----|-----------|
| GPU PRNG for momentum sampling | WGSL transcendental polyfills (log, sqrt, cos) introduce systematic bias in Box-Muller transform | cpu_mom workaround (CPU-generated momenta) |
| Initial plaquette comparison | First comparison used different random seeds for GPU and CPU runs — not a controlled experiment | Three-path methodology with identical seeds and states |
| 16⁴ lattice full production | Insufficient thermalization was initially used | Extended to 200+ thermalization trajectories |

---

## Provenance Chain

Every trajectory in the pseudoSpore carries a 5-stage cryptographic provenance chain:

```
1. BLAKE3 hash of trajectory data → content identity
2. DAG insertion (rhizoCrypt) → parent/child lineage
3. Ledger commit (loamSpine) → permanent record
4. Ed25519 signature (bearDog) → cryptographic witness
5. Attribution braid (sweetGrass) → W3C PROV-O
```

The provenance chain can be independently verified:
```bash
# Download the pseudoSpore
tar xzf pseudospore-hotspring-qcd-sun.tar.gz
cd pseudospore-hotspring-qcd-sun/

# Verify every hash, signature, and chain link
./validate.sh
```

See: [How to verify a pseudoSpore](/pseudospore/verify/)

---

## Source Code History

All computation code is in the hotSpring and barraCuda repositories:

- **hotSpring**: Physics domain — HMC algorithm, gauge theory, observables
- **barraCuda**: GPU math — DF64 arithmetic, WGSL compute shaders
- **coralReef**: Shader compilation — WGSL → PTX/RDNA IL via naga
- **toadStool**: Hardware dispatch — wgpu/Vulkan abstraction

Source: [git.primals.eco](https://git.primals.eco) (sovereign Forgejo instance)
Mirror: [github.com/ecoPrimals](https://github.com/ecoPrimals)

---

## Phase 7: AI Review and Reframing (Aug 2, 2026)

The complete preprint was submitted to AI agents for review. The review
correctly identified the paper as **Rung 1 of a lattice QCD program**
rather than a finished QCD paper, and identified specific validation gaps.

### What the review found right

- Every self-contained, falsifiable claim (plaquette values, precision
  measurements, PRNG isolation) checked out
- The three-path validation methodology is sound
- SU(2) is a legitimate preprint on its own merits

### What the review identified as gaps

| Gap | Priority | Status |
|-----|----------|--------|
| Title suggests finished QCD | Must fix | **FIXED** — retitled to "Toward..." |
| Single β value (2.3) | Must fix | Experiment queue: β-scan |
| Limited statistics (200 trajectories, 1 seed) | Must fix | Experiment queue: 4-8 seeds |
| Missing HMC diagnostics (ΔH, reversibility) | Must fix | Experiment queue |
| 16⁴ claims without production data | Must fix | **FIXED** — removed overclaims |
| No published SU(2) comparison | Must fix | Experiment queue |
| Precision path matrix | Must fix | **FIXED** — added to Section 2.2 |
| Plaquette normalization equation | Must fix | **FIXED** — added to Section 2.1 |
| pseudoSpore not version-frozen | Should fix | Experiment queue |

### Paper changes made

1. **Title**: "Toward Vendor-Agnostic Lattice QCD on Consumer GPUs: SU(2)..."
2. **Section 1.2**: Added 6-rung ladder table (scope statement)
3. **Section 2.1**: Added explicit plaquette normalization equation
4. **Section 2.2**: Added precision path matrix
5. **Section 4.3**: Reframed as "Limitations of the Present Result"
6. **Section 4.4**: Added "Remaining Validation Work" with experiment table
7. **Section 6**: Reframed conclusion around what Rung 1 proves
8. **Abstract**: Removed 16⁴ overclaims, added "first rung" framing

### Phase 8: Factor-of-Four Plaquette Discovery (Aug 2, 2026)

A second AI review of the live preprint identified a critical normalization
question: the reported plaquette values (~0.15 at β=2.3) are exactly 1/4
of the conventional SU(2) Monte Carlo value (~0.60).

    4 × 0.15023811 = 0.60095244
    4 × 0.15105782 = 0.60423128

This is too exact to be coincidence. Two possibilities:

1. **Measurement-only bug**: The generated configurations are correct at
   β=2.3, but the plaquette measurement applies an extra division by 4
   (e.g., dividing by 24V instead of 6V, or applying 1/N twice).

2. **Action/force bug**: The action uses an effective coupling of β/4 ≈ 0.575,
   making the configurations physically correct for the wrong coupling.

**Critical insight**: The GPU-vs-CPU agreement (|Δ|/σ < 1) does NOT
distinguish these possibilities. Both implementations use the same
normalization, so they agree with each other regardless of whether
the shared normalization is correct.

**Impact**: This is now the **first blocker** before launching the statistics
campaign. Running thousands of trajectories at a potentially mislabelled
coupling would produce more of the same potentially-incorrect data.

**Diagnostic protocol**: Added as Appendix B to the paper. Four quick
tests (cold-lattice normalization, coupling audit, numerical force
derivative, short β-scan) will resolve this before any long production runs.

**Experiment queue reordered**: Normalization → force test → β-scan → HMC
diagnostics → statistics → PRNG characterization → larger volumes → freeze.

### Additional fixes from this review

| Issue | Fix |
|-------|-----|
| pseudoSpore page says "arXiv complete" | Changed to "preprint under refinement" |
| Naga described as compiling directly to PTX | Fixed: naga→SPIR-V, Vulkan driver→native ISA |
| Claims "runs on Intel" without validation | Qualified: "designed to support; physics validation covers AMD and NVIDIA" |
| "bit-exact" used where Δ is nonzero | Changed to "agrees to machine precision" |
| Cost table claims "$0.03 per 10K" | Removed specific figure; will report with 12⁴/16⁴ data |
| "SU(3) gauge force" in three-path section | Fixed to "SU(2) gauge force" |

---

## Agent Session Context

This computation was developed through AI-assisted sessions (K-NOME methodology:
human constraint + AI implementation). The key decisions documented above
were made through iterative validation cycles where:

1. The human (ecoPrimal) defined the physics goals and validation criteria
2. AI agents implemented the GPU compute kernels and benchmarking infrastructure
3. Failed results (plaquette divergence) triggered systematic debugging
4. The three-path methodology was developed collaboratively to isolate root causes
5. Each resolution was validated against independent reference values
6. AI agents reviewed the preprint and identified validation gaps
7. The paper was reframed as Rung 1 of a 6-rung ladder

The full session transcripts for the sporePrint publication pipeline are
part of the ecosystem's internal documentation.

---

## Review Checklist for AI Agents

If you are reviewing the [arXiv draft](/pseudospore/hotspring-qcd-sun-paper/)
and have read this audit trail, here are the specific questions:

### Physics Validation
- [ ] **CRITICAL**: Is the ~0.15 plaquette at β=2.3 correctly normalized? Published SU(2) values are ~0.60 (exactly 4× the reported value).
- [ ] Is |Δ|/σ < 1 the correct criterion for GPU-CPU agreement? (Note: GPU-CPU agreement does not validate normalization if both use the same convention.)
- [ ] Are the autocorrelation times (τ_int ≈ 1.6–3.4) physically reasonable?
- [ ] Is 200 thermalization + 200 production sufficient at these volumes?

### Methodology
- [ ] Does the three-path comparison correctly isolate the PRNG bias?
- [ ] Is the cpu_mom overhead claim (< 0.1%) credible?
- [ ] Is the DF64 error analysis (per-op ~14 digits, accumulated ~9) consistent?

### Claims and Fairness
- [ ] Are all speedup comparisons GPU-vs-CPU on identical hardware?
- [ ] Is "vendor-agnostic" fairly claimed given only 2 GPU architectures tested?
- [ ] Are limitations honestly disclosed?
- [ ] Is the cost analysis ($0.03 vs $55) a fair comparison?

### Reproducibility
- [ ] Is the pseudoSpore artifact sufficient to reproduce all results?
- [ ] Can the provenance chain be independently verified?
- [ ] Is the source code accessible?

---

## See Also

- [arXiv Draft — Full Paper Text](/pseudospore/hotspring-qcd-sun-paper/)
- [pseudoSpore Data + Provenance](/pseudospore/hotspring-qcd-sun/)
- [How to Verify a pseudoSpore](/pseudospore/verify/)
- [GPU Compute — Live Evidence](/lab/gpu-compute-live/)
- [LaTeX Source](https://git.primals.eco/ecoPrimals/whitePaper/src/branch/main/subGen/lattice_qcd_consumer_gpu.tex)
