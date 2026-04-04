+++
title = "Self-Tuning Simulation"
description = "Lattice QCD x Adaptive Algorithms — runtime spectral discovery eliminates all hand-tuned simulation parameters. hotSpring."
date = 2026-03-30

[extra]
paper_number = 25
domain = "Physics and Materials"

[taxonomies]
primals = ["barracuda", "toadstool"]
springs = ["airspring", "groundspring", "healthspring", "hotspring", "primalspring", "wetspring"]
+++

**Date:** March 27, 2026
**Status:** Phases 1-4 complete. Production validated: Nf=2 and Nf=2+1 at 4⁴/8⁴ via GPU RHMC (Exp 101). `production_rhmc_flow` binary integrates RHMC + gradient flow (Exp 103). 16⁴ runs in progress. NPU bridge pending.
**Domain:** Lattice QCD × adaptive algorithms × neuromorphic computing × reproducible science
**Novelty:** No prior GPU lattice QCD framework eliminates all hand-tuned simulation
parameters via runtime spectral discovery and physics-observable feedback. The approach
combines GPU power iteration, acceptance-rate-driven step adaptation, and consistency-
monitored rational approximation quality into a single calibrator that requires only
physics inputs (Nf, mass, β, lattice dimensions).
**Cross-Spring:** {{ entity(name="hotspring") }} × {{ entity(name="barracuda") }} × toadStool × {{ entity(name="primalspring") }} × ALL springs

---

## Abstract

Lattice QCD simulations using Rational Hybrid Monte Carlo (RHMC) require expert-tuned
parameters: spectral ranges for the rational approximation, pole counts, integration
step sizes, trajectory lengths, and solver tolerances. These are chosen by experienced
practitioners through trial-and-error runs and institutional knowledge — non-reproducible
work that creates invisible barriers to entry and hidden failure modes at scale.

We introduce `RhmcCalibrator`, a self-tuning calibrator that replaces all hand-tuned
parameters with physics-validated measurements. The calibrator classifies every parameter
into one of five categories — mathematical (theory), discovered (measured), adapted
(feedback-controlled), validated (auto-checked), and learned (NPU) — and provides
algorithms for each. The result: the user specifies only the physics (`Nf`, quark masses,
coupling `β`, lattice dimensions) and the calibrator produces a fully configured RHMC
simulation.

This is the {{ entity(name="ecoprimals") }} constrained-evolution philosophy applied to simulation methodology:
the physics itself constrains the parameter space, and observable feedback (acceptance
rate, Hamiltonian conservation, heatbath-action consistency) replaces human judgment.

---

## 1. The Problem: Hidden Magic Numbers

### What practitioners tune by hand

| Parameter | Typical method | Failure mode at scale |
|-----------|---------------|----------------------|
| Spectral range [a, b] | Literature values or guessing | Wrong at physical quark masses (λ_min drops 100x) |
| n_poles | Experience ("8 is usually enough") | Insufficient near phase transitions |
| dt (step size) | Trial runs to achieve ~70% acceptance | Depends on volume, mass, β — retuning needed per ensemble |
| n_md_steps | Chosen to give τ ≈ 0.5-1.0 | Coupled to dt; wrong τ increases autocorrelation |
| CG tolerance | Convention (1e-6 or 1e-8) | Same tolerance for force and Metropolis wastes CG iterations |

### Why this matters

1. **Non-reproducible**: Different practitioners choose different parameters for the same physics
2. **Scale-dependent**: Parameters that work at 8⁴ fail at 32⁴ or at physical quark masses
3. **Invisible failures**: Wrong spectral range doesn't crash — it silently produces wrong physics
4. **Barriers to entry**: New practitioners must learn tuning from experts or by expensive trial-and-error

---

## 2. The Solution: Five Parameter Categories

### Mathematical (from theory alone)

These never change. They are consequences of the formalism:
- Omelyan integrator parameter λ = 0.1932 (optimal for 2nd-order symplectic)
- Determinant power: `det_power = Nf/8` (staggered rooting trick)
- Rational approximation: `x^{-α}` for action/force, `x^{+α/2}` for heatbath
- Consistency identity: `r_hb(x)² · r_act(x) = 1` for all x in the spectral range

### Discovered (measured from the gauge field)

GPU power iteration estimates λ_max(D†D) in ~20 Dirac applications (cheap
compared to a full CG solve). The analytical bound λ_min(D†D) ≥ m² for
positive-mass staggered fermions is tight at weak coupling. Safety margins
(0.5× below, 1.5× above) accommodate gauge-field fluctuations.

### Adapted (feedback from physics observables)

The acceptance rate is the primary feedback signal for step size:
- High acceptance (>85%) → increase dt by 10% (more physics per wall-clock)
- Low acceptance (<50%) → decrease dt by 15% (reduce integration error)
- Emergency |ΔH| > 2 → scale dt using Omelyan's |ΔH| ∝ dt² relation

The trajectory length τ = dt × n_md is preserved during adaptation, with
n_md clamped to [2, 100].

### Validated (auto-checked and corrected)

The heatbath-action consistency relation provides a direct test of rational
approximation quality. After generating φ = r_hb(D†D)η, the fermion action
S_f = φ†r_act(D†D)φ should equal η†η. Deviation beyond 5% triggers an
automatic increase in pole count (+2 poles, up to 24 max).

### Learned (NPU predictions — Phase 5)

The AKD1000 NPU heads (`A2_ANDERSON_LAMBDA_MIN`, `ANOMALY_DETECT`,
`CG_ESTIMATE`, `PARAM_SUGGEST`) can predict spectral properties and optimal
parameters from gauge-field features, accelerating convergence from 10-50
trajectories (pure feedback) to 1-3 trajectories (prediction + validation).

---

## 3. Implementation in barraCuda

### SpectralProbe (`spectral_probe.rs`)

GPU power iteration for λ_max estimation:
1. Initialize with deterministic quasi-random vector (golden-ratio pattern)
2. Iterate: w = D†D · v, λ ≈ ⟨v|w⟩/⟨v|v⟩, v = w/‖w‖
3. After 20 iterations: λ_max converged to ~1e-6 relative accuracy

Reuses existing Dirac dispatch and dot-product pipelines — no new WGSL shaders.

### RhmcCalibrator (`rhmc_calibrator.rs`)

Stateful calibrator that produces `RhmcConfig` on demand:

```
(Nf, mass, β, dims)  →  RhmcCalibrator  →  RhmcConfig
                              ↑
                         observe(result)
```

The calibrator's `observe()` method processes each trajectory result,
updating its internal state (acceptance history, ΔH window, consistency
ratio). The feedback is fully automatic — no human intervention needed
after construction.

### Tolerance Constants (`tolerances/lattice.rs`)

12 named constants with physics justifications in doc comments. Every
threshold is discoverable, documented, and evolvable — no magic numbers
buried in logic branches.

---

## 4. Connection to the Constrained Evolution Thesis

The self-tuning calibrator is a direct instantiation of constrained evolution
applied to algorithm design:

1. **The fitness landscape** is the space of RHMC parameters (dt, n_poles,
   spectral range, tolerances)
2. **The constraint** is physics: acceptance rate, Hamiltonian conservation,
   detailed balance, consistency identity
3. **The evolution** is the feedback loop: observe → adapt → validate → repeat
4. **The environment** is the specific gauge configuration (volume, coupling,
   quark masses)

Just as environmental constraints reshape microbial fitness landscapes (Paper 01),
physics constraints reshape the RHMC parameter space. The calibrator "discovers"
the optimal parameters in the same sense that a bacterial population "discovers"
its ecological niche — through constrained exploration guided by fitness signals.

The NPU bridge (Phase 5) adds "cultural transmission" — parameters learned from
previous runs accelerate discovery on new ensembles, analogous to the ESN
bootstrap in Exp 020-029.

---

## 5. Cross-Spring Implications

### General self-tuning pattern

The parameter classification (mathematical / discovered / adapted / validated /
learned) applies to any spring's simulation:

| Spring | Discovered | Adapted | Validated |
|--------|-----------|---------|-----------|
| {{ entity(name="hotspring") }} | Eigenvalue spectrum | dt, n_md | Acceptance rate, ΔH |
| {{ entity(name="wetspring") }} | Pair correlation length | MD timestep | Energy conservation |
| {{ entity(name="healthspring") }} | Patient-specific PK rate | Dosing interval | Therapeutic window |
| {{ entity(name="airspring") }} | Sensor drift coefficient | Calibration interval | Reference standard |
| {{ entity(name="groundspring") }} | Soil conductivity | Model resolution | Field measurement |

The `SimulationCalibrator` trait pattern proposed in the {{ entity(name="wateringhole") }} handoff
would allow each spring to implement domain-specific physics validators while
sharing the adaptation infrastructure from {{ entity(name="barracuda") }}.

### Neuromorphic acceleration

The NPU bridge demonstrates a general pattern: use neuromorphic hardware to
*predict* optimal parameters (fast, ~μs inference) and use physics to *validate*
them (slow, ~seconds of GPU compute). This is more efficient than pure feedback
(which requires 10-50 expensive trajectories to converge) and more reliable than
pure prediction (which can hallucinate outside training distribution).

---

## Cross-References

- **{{ entity(name="hotspring") }} experiments:** 099 (RHMC infrastructure), 101 (production Nf=2/2+1), 102 (gradient flow at volume), 103 (self-tuning calibrator)
- **{{ entity(name="basecamp") }} paper 10:** First dynamical QCD production (NPU-steered, hand-tuned parameters)
- **{{ entity(name="basecamp") }} paper 24:** All-silicon science (hardware-aware routing complements self-tuning)
- **{{ entity(name="basecamp") }} paper 15:** Precision brain (self-routing hardware discovery — same pattern)
- **{{ entity(name="basecamp") }} paper 11:** Nautilus shell (evolutionary reservoir → ESN training for NPU bridge)
- **{{ entity(name="basecamp") }} paper 07:** Sovereign WDM (consumer GPU validation foundation)
- **Thesis connection:** Constrained evolution (Ch. 3) — physics constraints as fitness landscape

---

## March 28, 2026 Update: True Multi-Shift CG Validates Self-Tuning Pipeline

The self-tuning RHMC calibrator's output parameters are now validated through production
runs using the true multi-shift CG solver with the corrected fermion force:

- **Fermion force sign**: Changed from +η/2 to −η (matching gauge force convention ∂S/∂U)
- **ΔH**: O(1) across all tested trajectories — confirms calibrator-chosen dt and n_md are correct
- **37% speedup**: True multi-shift CG + diagnostic removal = 16.5s per trajectory at 8⁴ Nf=2
- **Compiler fix**: `std::hint::black_box` for GPU convergence loops (release-mode safety)

The calibrator's spectral probe (λ_max from power iteration, λ_min from m²) correctly
bounds the rational approximation range. The acceptance-driven step adaptation produces
dt/n_md combinations that yield ΔH = O(1) with the corrected force — validating the
entire self-tuning chain: spectral discovery → approximation → integration → acceptance.

See `hotSpring/whitePaper/baseCamp/true_multishift_cg_validated.md` for the full debugging
methodology and production results.

---

## March 29, 2026 Update: Silicon-Aware Self-Tuning

The self-tuning calibrator's observation data now includes silicon routing metadata
via the 11D NPU input vector (`npu_canonical_input_v2`). The 5 new dimensions are:

| Dimension | Field | Meaning |
|-----------|-------|---------|
| 7 | `tmu_prng` | Was TMU used for PRNG? (0.0 or 1.0) |
| 8 | `subgroup_reduce` | Was subgroup reduce active? (0.0 or 1.0) |
| 9 | `rop_force_accum` | Was ROP atomic path used? (0.0 or 1.0) |
| 10 | `fp64_strategy_id` | Fp64Strategy as f64 (0=Sovereign, 1=Native, 2=Hybrid, 3=Concurrent) |
| 11 | `has_native_f64` | Does hardware support native f64? (0.0 or 1.0) |

This extends the self-tuning pattern from "physics-only" to "physics + hardware":
the NPU can now correlate routing decisions with trajectory quality and adapt
both physics parameters (dt, n_md) AND routing preferences per-GPU.

The capacity analysis (Phase 7) also informs the calibrator's volume selection:
knowing that RTX 3090 fits L=46⁴ dynamical and RX 6950 XT fits L=40⁴ prevents
OOM failures during automated scaling.

See `HOTSPRING_V0632_SILICON_SATURATION_PRIMAL_EVOLUTION_HANDOFF_MAR29_2026.md`.
