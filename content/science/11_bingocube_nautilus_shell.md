+++
title = "BingoCube Nautilus Shell"
description = "Mathematical Physics x Complexity — Nautilus shell as NP-structure validator, Turing-complete card games. ludoSpring."
date = 2026-03-17

[extra]
paper_number = 11
domain = "Game Science and Systems"

[taxonomies]
primals = ["bingocube", "toadstool"]
springs = ["airspring", "groundspring", "hotspring", "ludospring"]
+++

**Date:** March 1, 2026
**Status:** Implementation complete, validated on QCD trajectory data
**Spring:** {{ entity(name="hotspring") }} (NPU brain architecture), primalTools/bingoCube
**Hardware:** CPU (simulator), BrainChip AKD1000 (target)
**License:** AGPL-3.0-only

---

## Summary

BingoCube boards — structured random networks with column-range constraints
and distinct values drawn from a combinatorially vast space (~10^31 for L=5) —
are reservoir computers. When a data stream replaces the random caller, each
board becomes a deterministic nonlinear projection of the input. An ensemble
of boards running in parallel replaces the temporal recurrence of a traditional
Echo State Network with combinatorial diversity. Evolutionary selection across
generations replaces the fading memory of recurrent dynamics with accumulated
structural adaptation.

The full evolutionary history forms a **nautilus shell**: each generation wraps
the previous, preserving heritage while adding new adaptation. The shell is the
portable unit of learned structure — serializable, transferable between machines,
and mergeable across instances.

| Metric | Value |
|--------|-------|
| Core crate | `bingocube-nautilus` (primalTools/bingoCube/nautilus/) |
| Board space | ~10^31 (L=5), ~10^84 (L=8), ~10^190 (L=12) |
| Response dim | pop_size × L² (e.g. 16 × 25 = 400) |
| Readout | Ridge regression (Cholesky solver) |
| Evolution | Elitism + tournament + column-swap crossover + mutation |
| Serialized shell | ~23 KB (16 boards, 20 generations, 2 targets) |
| Unit tests | 20 passing |
| Hardware target | AKD1000 (int4 weights, feed-forward, 78 NPs) |

---

## 1. Origin: Bingo at a Retirement Home

The idea came from watching bingo at a retirement home. The author's partner
volunteers there with their Rottweiler — a dog that insists on receiving pets
from every resident. On holidays, the author brings the second Rottweiler
(more skittish, but warming up). During a game with approximately 30 boards
in play, the author asked the caller whether she knew which boards had won.

She said no.

But she could have. The caller knows every number she has called. She knows
(or could know) which boards are in play. With ~30 boards from a near-infinite
combinatorial space, the mapping from call sequence to board state is fully
deterministic. The caller is not blind — the caller is **omniscient if she
chooses to be**. She just wasn't tracking it. A computer wouldn't even blink.

Three observations followed:

**Observation 1: The boards are structured random projections.**

A bingo board is not truly random. It has column-range constraints (column k
draws from [k·R, (k+1)·R)), distinct values per column, and a finite but
vast combinatorial space. When a sequence of values flows through as the
"caller," each board's response pattern — which cells match, in what order —
is a deterministic function of both the board's structure and the input
sequence. Different boards produce different response patterns to the same
input. This IS reservoir computing: structured random weights project input
into a high-dimensional space where a simple readout can extract structure.

**Observation 2: The caller can verify any board at any time.**

The relationship between caller and boards is not opaque. The caller has
complete information: the call sequence and the board definitions. At any
moment, the caller can reconstruct the exact state of every board — which
cells are marked, how close each board is to winning, which boards have
already won. For residents with dementia who may not be watching their own
boards, this means someone (or something) can track for them. The
verification is always available, on demand, at any reveal level. This is
the progressive reveal property of BingoCube's cryptographic commitment.

**Observation 3: Evolution replaces recurrence.**

A traditional Echo State Network uses temporal recurrence for memory:
state(t) depends on state(t−1). But what if the boards themselves evolve?
Boards that respond predictably to the data stream survive to inform the
next generation. The "memory" is not in a hidden state vector — it is in
the accumulated structure of the boards. Which column values were preserved,
which permutations proved useful, which structural properties correlated
with the target observable. This evolutionary history forms a nautilus
shell: each layer wraps the previous, and you can read the history by
unwinding the layers.

The physical metaphor closes the loop. A real nautilus shell is a resonant
cavity shaped by millions of years of evolved geometry. Sound enters, the
shell transforms it, and what comes out is filtered by the shell's entire
growth history. Put your ear to it and you hear "the ocean" — but you are
hearing ambient noise filtered through evolved structure. The shell is a
physical reservoir computer. It always was.

---

## 2. Architecture: Feed-Forward Reservoir via Board Ensembles

### 2.1 Why Feed-Forward Matters

The BrainChip AKD1000 neuromorphic processor is feed-forward only. Traditional
recurrent architectures (ESN, LSTM) require a feedback loop from output back
to input — the AKD1000 cannot do this in hardware. The host CPU must drive
the recurrence loop, negating the latency and power advantages of the NPU.

```
Traditional ESN (requires recurrence — AKD1000 cannot do natively):
  input(t) → reservoir(t) = f(W_in · input(t) + W_res · state(t-1))
                                                         ↑ feedback

BingoCube Reservoir (pure feed-forward — AKD1000 native):
  input → Board₁ response → ┐
  input → Board₂ response → ├→ FC readout → output
  input → Board₃ response → ┘
          ↑ no feedback, N boards run in parallel
```

Multiple boards run simultaneously against the same input. Each board is a
different random projection. The ensemble of board responses replaces the
single reservoir's temporal memory with combinatorial diversity. The FC
readout layer (which the AKD1000 handles natively via SkipDMA) extracts
the prediction.

### 2.2 Three Roles

The bingo analogy separates three clean roles:

| Role | Bingo | Reservoir Computing | BingoCube |
|------|-------|--------------------:|-----------|
| **Caller** | Draws numbered balls | Input data stream | `ReservoirInput` (discrete or continuous) |
| **Boards** | Players' cards | Random projection weights | `Population` of `Board`s |
| **Player** | Watches their card | Readout layer | `LinearReadout` (ridge regression) |

The caller knows everything and can verify any board. The boards are structured
random objects — inspectable, deterministic given their seed. The player only
needs to watch the readout, not understand the full reservoir.

In the NPU context, the caller is not random — it is **adaptive**. If the
caller knows which boards are close to activating, it can steer what it calls
next. This is the adaptive steering from {{ entity(name="hotspring") }}'s brain architecture: the
host inspects the reservoir state and chooses the next input to maximize
information gain.

### 2.3 Board Response Mechanics

**Discrete input** (classic bingo): each input value either matches a cell or
doesn't. The response is a binary vector — 1 for match, 0 for no match, 0.5
for the free cell. Dimensionality: L² per board.

**Continuous input** (physics observables): each cell's value is combined with
the input features via BLAKE3 hashing to produce a bounded activation in
[0, 1]. Each board gives a unique, deterministic, nonlinear projection of
continuous features. This is the mode used for physics applications.

```
Scalar projection for cell (i, j):
  activation = BLAKE3("NAUTILUS_PROJ" ‖ i ‖ j ‖ cell_val ‖ features) → [0, 1]
```

The ensemble response is the concatenation of all board responses:
response_dim = pop_size × L² (e.g. 16 boards × 25 cells = 400 dimensions).

---

## 3. Evolution as the Time Step

### 3.1 The Problem with Recurrence

In a standard ESN, temporal memory comes from the echo state property:
past inputs leave decaying echoes in the reservoir state. The recurrence
relation state(t) = f(W·state(t-1) + W_in·input(t)) creates a fading
memory of recent inputs. This is powerful but requires a feedback loop
that feed-forward hardware cannot provide.

### 3.2 Evolutionary Generations Replace Temporal Feedback

After a generation of input processing, the boards are evaluated. Each
board receives a fitness score based on how well its individual response
correlates (Pearson) with the target observables across the training set.

```
Generation 0: Random boards (naive initialization)         ╮
Generation 1: Boards informed by Gen 0 performance         │ nautilus
Generation 2: Boards informed by Gen 1 (shell growing)     │ shell
   ...                                                      │
Generation N: Boards evolved to the environment             ╯
```

Evolution proceeds by:

1. **Selection**: Top-performing boards survive (elitism) or compete
   (tournament / roulette wheel)
2. **Crossover**: Column-swap between parents (preserves column-range
   constraints) or cell-level mixing (with duplicate repair)
3. **Mutation**: Individual cells re-randomized within their column range
   (rate typically 0.10–0.20)

The key constraint: **column-range invariance**. Every child board satisfies
the same structural rules as its parents — column k draws values from
[k·R, (k+1)·R) with no duplicates within a column. This is not an arbitrary
restriction; it is the structural regularity that makes the projections
useful. Mutation and crossover explore the combinatorial space without
violating the grammar.

### 3.3 What Evolution Encodes

After many generations, the surviving boards are not random. They are
**tuned resonators** — their specific cell values and column structures
have been shaped by the data they grew up on. Boards that happened to
produce responses that varied predictably with the target observable were
selected. Their structure — which column values, in which positions —
now implicitly encodes statistical relationships in the training data.

This is analogous to how a physical shell's chamber geometry is shaped
by the organism's growth environment. The shell didn't "learn" the
acoustics — it grew into a geometry that happens to resonate with the
frequencies present during its formation.

---

## 4. The Nautilus Shell: Layered Evolutionary History

### 4.1 Shell Structure

A `NautilusShell` contains:

| Component | Purpose |
|-----------|---------|
| `current_population` | Latest generation of boards (the active reservoir) |
| `readout` | Trained linear readout (FC layer) |
| `history` | Vector of `GenerationRecord`s (fitness trajectory) |
| `origin` | `InstanceId` of the machine that created this shell |
| `lineage` | All `InstanceId`s that have contributed to this shell |
| `config` | Board config, population size, evolution parameters, ridge λ |

The `GenerationRecord` for each generation stores: generation number, mean
and best fitness, population size, origin instance, and training set size.

### 4.2 Within an Instance

A single machine evolves its shell by repeatedly calling
`evolve_generation(inputs, targets)`:

1. **Project** all inputs through the current population → ensemble responses
2. **Train** the readout via ridge regression on (responses, targets)
3. **Evaluate** board fitness (per-board Pearson correlation)
4. **Breed** the next generation (selection + crossover + mutation)
5. **Record** the generation's statistics in the shell history

The readout is retrained every generation because the population changes.
Each generation's readout is adapted to the current board structure.

### 4.3 Between Instances

The shell serializes to JSON (or binary). A receiving machine can:

**Continue**: Pick up from the inherited generation, adding its own instance
to the lineage. The new machine's data stream drives further evolution
from the inherited population. Heritage is preserved; new adaptation layers on.

```
Instance A (homelab):  Gen 0 → Gen 1 → ... → Gen 20
                                                  ↓ serialize
Instance B (field):                           Gen 20 → Gen 21 → ... → Gen 30
```

**Merge**: Combine the best boards from two independently evolved populations.
Each instance contributed boards that were tuned to different data regimes.
The merged population has combinatorial diversity from both origins. Lineage
records both contributing instances.

```
Instance A: Gen 0 → ... → Gen 20 (tuned to regime A)
Instance B: Gen 0 → ... → Gen 20 (tuned to regime B)
                                      ↓ merge
Instance A: Gen 20 (best of A + best of B) → Gen 21 → ...
```

**Inspect**: The shell's history records every generation's fitness trajectory
and origin instance. By unwinding the layers, you can trace exactly which
machine contributed what, when fitness jumped, and where regimes changed.

---

## 5. Hardware Mapping to AKD1000

| BingoCube Concept | AKD1000 Hardware | Advantage |
|-------------------|------------------|-----------|
| Board values (int, column-locked) | int4 weights in NP SRAM | Native format — no quantization loss |
| Board response (sparse matches) | Event-based activation | Zero compute for non-matching cells |
| Multiple boards in parallel | Multiple NP subsets (78 NPs) | Each NP runs a board |
| FC readout | FullyConnected layer via SkipDMA | Single hardware pass |
| Board evolution | `set_variable()` weight mutation | 13 ms per generation update |
| Board heritage (nautilus shell) | `.fbz` model serialization | Save/reload evolved boards |

The boards are integer-valued and column-constrained — they map directly to
int4 weights without quantization loss. This is not an accident: the same
discrete structure that makes boards inspectable to humans is the native
format that neuromorphic silicon wants. The architecture is hardware-aware
from its mathematical foundation.

---

## 6. Relationship to hotSpring ESN (Gen 1 / Gen 2)

{{ entity(name="hotspring") }}'s current brain architecture uses a traditional Echo State Network
with a random reservoir and trained linear readout — what we call the
"lizard brain" (Gen 1, 15 heads) evolving to the "developed organism"
(Gen 2, 36 overlapping heads). This runs on the CPU with the NPU simulated.

The BingoCube Nautilus Shell is a complementary architecture:

| Property | ESN ({{ entity(name="hotspring") }} Gen 1/2) | Nautilus Shell |
|----------|------------------------|----------------|
| Reservoir | Fixed random matrix W_res | Population of evolved boards |
| Memory | Fading echo (recurrent) | Evolutionary heritage (generational) |
| Time step | Recurrence: state(t) ← state(t-1) | Evolution: gen(n) ← gen(n-1) |
| Training | Readout only (standard) | Readout + board evolution |
| Hardware | CPU (sim) or host-driven recurrence | AKD1000 native feed-forward |
| Transfer | ExportedWeights (f32 readout) | NautilusShell (boards + readout + history) |
| Interpretability | Opaque reservoir weights | Inspectable board structure |

The two architectures are not competitors. The ESN handles fast, within-run
temporal dynamics (CG cost prediction, phase classification, anomaly
detection). The Nautilus Shell handles cross-run structural learning (which
board geometries correlate with which physics regimes). A future architecture
may compose them: the ESN's temporal readout feeds the Nautilus Shell's
evolutionary fitness, and the Nautilus Shell's evolved boards serve as the
ESN's input projection.

---

## 7. Implementation

### 7.1 Crate Structure

```
primalTools/bingoCube/nautilus/
├── Cargo.toml
├── src/
│   ├── lib.rs          # Public API, module declarations
│   ├── response.rs     # BoardResponse: input → board projection
│   ├── population.rs   # Population: board ensembles + fitness evaluation
│   ├── evolution.rs    # Evolution: selection, crossover, mutation
│   ├── readout.rs      # LinearReadout: ridge regression (Cholesky solver)
│   └── shell.rs        # NautilusShell: layered history + instance transfer
└── examples/
    └── shell_lifecycle.rs  # Full demo: evolve → serialize → transfer → merge
```

### 7.2 Key Types

```rust
// Input to the reservoir
enum ReservoirInput {
    Discrete(Vec<u32>),      // classic bingo caller values
    Continuous(Vec<f64>),    // physics observables (β, plaq, acc, ...)
}

// Response from one board (L² activations)
struct ResponseVector { activations: Vec<f64> }

// A population of boards (one generation)
struct Population {
    boards: Vec<Board>,
    config: Config,
    generation: usize,
    fitness: Vec<FitnessRecord>,
}

// The nautilus shell (full evolutionary history)
struct NautilusShell {
    config: ShellConfig,
    current_population: Population,
    readout: LinearReadout,
    history: Vec<GenerationRecord>,
    origin: InstanceId,
    lineage: Vec<InstanceId>,
}
```

### 7.3 Lifecycle API

```rust
// Create a shell on machine A
let mut shell = NautilusShell::from_seed(config, InstanceId::new("northgate"), 42);

// Evolve within instance A
for _ in 0..20 {
    let mse = shell.evolve_generation(&inputs, &targets);
}

// Serialize and ship to machine B
let json = serde_json::to_string(&shell)?;
// ... network transfer ...
let received: NautilusShell = serde_json::from_str(&json)?;

// Machine B continues from inherited shell
let mut shell_b = NautilusShell::continue_from(received, InstanceId::new("strandgate"));
for _ in 0..10 {
    shell_b.evolve_generation(&field_inputs, &field_targets);
}

// Merge field knowledge back into homelab
shell.merge_shell(&shell_b);
```

### 7.4 Validation Results

20 unit tests covering:
- Deterministic board response (same input → same output)
- Different boards produce different projections
- Ensemble response concatenation
- Fitness evaluation via Pearson correlation
- Column-range constraint preservation through evolution (even at 50% mutation)
- Multi-generation evolution
- Ridge regression readout (recovers y = 2x₀ + 3x₁ + 1 from 100 samples)
- Shell serialization roundtrip (predictions identical after deserialize)
- Between-instance transfer (lineage tracking, generation continuity)
- Shell merge (population combination, history interleaving)

---

## 8. Demonstration: Shell Lifecycle

The `shell_lifecycle` example (cargo run --example shell_lifecycle -p bingocube-nautilus)
demonstrates the full within-instance and between-instance lifecycle:

**Phase 1 — Homelab evolution (20 generations):**
16 boards, 5×5 grid, 100 training samples, 2 target observables.
Mean fitness climbs from 0.08 to 0.33 as boards specialize.

**Phase 2 — Serialize and transfer:**
Shell serializes to ~23 KB JSON containing boards, readout weights,
and 20 generations of heritage.

**Phase 3 — Field node continuation:**
Strandgate receives the shell, continues evolving for 10 generations on
a shifted data regime (β ∈ [0.5, 1.0] vs homelab's [0, 1.0]).
Fitness initially drops (new regime) then recovers as boards adapt.

**Phase 4 — Shell merge:**
Homelab merges field node's best boards into its population. The merged
shell has lineage from both instances. Post-merge evolution starts from
combined heritage.

The full fitness trajectory, with origin instance labels, shows the
nautilus shell growing across machines and regimes.

---

## 9. Live Validation: QCD Trajectory Prediction

The Nautilus Shell was validated on actual dynamical QCD trajectory data from
{{ entity(name="hotspring") }} Exp 024 + 028 — 1,336 measurement records across 21 β values
(β ∈ [4.300, 6.500]).

**Task**: Predict CG solver cost and mean plaquette from per-β-point summaries
using features (β, mean_plaq, acceptance_rate, mean_|δH|, log_mean_cg).

### 9.1 Training Results (40 Generations, 24 Boards)

| Metric | Value |
|--------|-------|
| Population | 24 boards (5×5), response dim = 600 |
| Training samples | 21 β-point aggregates |
| Generations | 40 |
| Best fitness | 0.8307 (gen 39, steadily climbing) |
| Mean CG relative error | **1.1%** (on training set) |
| Mean plaquette absolute error | 0.0323 |
| Shell size | 36.9 KB |

Fitness climbed from 0.23 (random init) to 0.83 over 40 generations,
showing clear board specialization to the QCD landscape.

### 9.2 Leave-One-Out Cross-Validation

Each of the 21 β points was held out in turn. A fresh shell was trained
on the remaining 20 points (16 boards, 20 generations), then predicted the
held-out CG cost.

| β (held out) | CG actual | CG predicted | Rel Error |
|-------------|----------|-------------|----------|
| 4.300 | 62,202 | 57,099 | 8.2% |
| 4.500 | 61,125 | 60,736 | 0.6% |
| 4.700 | 60,400 | 60,610 | 0.3% |
| 5.000 | 59,761 | 65,188 | 9.1% |
| 5.500 | 58,236 | 58,115 | 0.2% |
| 5.690 | 57,728 | 56,830 | 1.6% |
| 6.000 | 55,793 | 59,299 | 6.3% |
| 6.131 | 49,072 | 61,677 | 25.7% (!) |
| 6.500 | 60,400 | 62,503 | 3.5% |

**Mean LOO CG relative error: 5.3%**

The outlier at β = 6.131 (25.7% error) is physically meaningful — this is
the only β point where CG cost drops sharply (49K vs ~60K neighbors),
indicating a phase boundary where the solver suddenly finds an easier path.
The Nautilus Shell cannot predict this discontinuity from the linear readout
alone when the point is held out. This is exactly where the disagreement
signal from overlapping head groups (Gen 2 brain architecture) would flag
a concept edge.

### 9.3 Interpretation

The Nautilus Shell achieves 5.3% generalization error on CG cost prediction
from only 21 data points using 16 evolved bingo boards — with zero temporal
recurrence. The boards have specialized: their column structures now encode
correlations between β, acceptance rate, and solver cost that were absent in
the random Generation 0 population.

The one failure mode (β = 6.131) is informative, not pathological. It marks
a region where the physics changes qualitatively and the linear readout
cannot extrapolate. Detecting such regions is the purpose of the disagreement
signal — and the Nautilus Shell's evolutionary history provides the training
data to teach future generations where to be cautious.

### 9.4 Future Data Streams

| Data Source | Input Features | Target | Connection |
|-------------|---------------|--------|-----------|
| NOAA hourly weather (Michigan stations) | Temp, humidity, wind, radiation | ET₀ (evapotranspiration) | {{ entity(name="airspring") }}, {{ entity(name="basecamp") }} 08 |
| USGS streamflow (Red Cedar River) | Stage, discharge, turbidity | Next-hour discharge | {{ entity(name="groundspring") }} sensor noise |
| Lenski LTEE fitness trajectories | Generation, fitness, mutation count | Next-generation fitness | {{ entity(name="basecamp") }} 02 |
| Live Exp 029+ trajectory stream | Real-time β, plaq, CG, δH | NPU-steered next-β selection | {{ entity(name="hotspring") }} brain architecture |

### 9.5 Quenched→Dynamical Transfer: 540× Cost Reduction

The most striking result: a Nautilus Shell trained on **quenched** features
(plaquette, Polyakov loop — from pure gauge simulations with no fermions)
can predict **dynamical** observables (CG solver cost — which depends on
the fermion determinant).

| Metric | Value |
|--------|-------|
| Training data | 21 β points, quenched features only |
| Target | Dynamical CG iterations |
| LOO CG error | **4.4%** |
| Quenched sim cost | ~2s per config (pure gauge HMC, no CG) |
| Dynamical sim cost | ~1,080s per config (CG solver dominates) |
| **Cost ratio** | **~540×** |

This means we can run cheap quenched simulations to build proxy predictions
for the expensive dynamical observables. The column-range constraint forced
the boards to find correlations between gauge-field topology (captured by
plaquette variance and Polyakov loop) and CG difficulty — a physical
relationship that the linear readout can exploit because the boards project
the quenched features into a space where this correlation becomes linear.

### 9.6 Graph Ordering: Seeds vs NPU-Inserted Points

In the Exp 024+028 dataset, beta points have two origins:

1. **Seed points** (4 initial): β = 4.5, 5.25, 5.69, 6.5 — chosen by the
   experimenter to bracket the expected phase transition.

2. **NPU-inserted points** (13 additional): β = 4.3, 4.7, 5.0, 5.1, 5.3,
   5.4, 5.5, 5.6, 5.8, 5.9, 6.0, 6.131, 6.25 — chosen adaptively by the
   NPU steering algorithm based on ESN disagreement and physics proxy signals.

When plotting ⟨P⟩ or CG cost vs β, the insertion order reveals the NPU's
strategy: it brackets regions of high uncertainty (around β_c ≈ 5.5–5.7)
with progressively tighter spacing, rather than scanning linearly. The
graph should mark seed points distinctly (e.g., filled circles) vs
NPU-inserted points (open circles with insertion-order labels) so a human
reader can visually confirm that the adaptive strategy concentrates
measurements where the physics changes most rapidly.

### 9.7 Connection to LTEE: Boards as Populations Under Constraint

The Nautilus Shell is a direct computational analog of the LTEE ({{ entity(name="basecamp") }} 02):

| LTEE (Lenski) | Nautilus Shell |
|---------------|---------------|
| 12 populations of E. coli | Population of 16–24 BingoCube boards |
| Glucose-limited medium | Column-range constraint + target observable |
| Generations (~80,000) | Evolutionary generations (20–40) |
| Fitness in constrained environment | Pearson correlation with target |
| Fastidious phenotype (specialization) | Boards lose general patterns, gain target-specific structure |
| Ara-3 citrate innovation | Concept edge detection (β=6.131 spike) |
| Frozen fossil record | Shell history (serialized GenerationRecords) |
| N_e·s drift boundary | DriftMonitor tracking effective population × selection |

The constraint (column-range) does not merely accelerate convergence to a
reservoir architecture. It reshapes the fitness landscape so that boards
specialize toward the constrained environment (QCD observables). Different
random seeds produce different board populations — all increasing fitness,
none identical — exactly as the 12 LTEE populations diverge under identical
glucose constraint.

The drift monitor implements Anderson's N_e·s boundary (thesis §3.2.3):
when the population is too small or selection too weak, drift dominates and
boards accumulate deleterious column values. Edge seeding implements directed
mutagenesis: when LOO error spikes, new boards are generated biased toward
the failure region — constraint redirecting exploration toward qualitative
physics boundaries.

---

## 10. Connections to Other baseCamp Papers

| Paper | Connection |
|-------|-----------|
| 01 (Anderson QS) | Anderson spectral statistics as Nautilus Shell input features |
| 04 (Sentinels) | Nautilus Shell as the NPU inference engine for edge biosensors |
| 07 (Sovereign WDM) | Board evolution tuned to plasma transport regimes |
| 08 (NPU Ag IoT) | Nautilus Shell deployed on AKD1000 for crop monitoring |
| 09 (Field Genomics) | Evolved boards classify microbial community states |
| 10 (Dynamical QCD) | CG cost prediction via Nautilus Shell, complementing ESN |

---

## 11. Contributions

- **Conceptual**: Identification of bingo boards as structured random
  projections (reservoir computing weights) with column-range constraints
  mapping natively to int4 neuromorphic hardware.

- **Architectural**: Replacement of temporal recurrence with evolutionary
  generations, enabling fully feed-forward reservoir computing on hardware
  that cannot do feedback.

- **Implementation**: Complete Rust crate (`bingocube-nautilus`) with
  board response projection, population fitness evaluation, constrained
  evolution (column-range-preserving crossover + mutation), ridge
  regression readout, layered shell history, instance transfer,
  and shell merging.

- **The nautilus shell metaphor**: Each generation wraps the previous,
  preserving heritage while adding adaptation. The shell is the portable
  unit of learned structure — the evolutionary equivalent of saved weights.

---

## 12. Recent Evolution (2026-03-01)

### 12.1 Self-Regulating Drift Monitor

The `DriftMonitor` is now wired directly into `evolve_generation()`. Each
generation, the shell records fitness data and checks the effective population
size times selection coefficient (N_e · s). When drift dominates:

- **`DriftAction::IncreaseSelection`** — halves elite survivors or increases
  tournament size, sharpening selection pressure
- **`DriftAction::IncreasePop { factor }`** — injects fresh random boards to
  expand the gene pool

Each `GenerationRecord` now tracks `ne_s` and `drift_action` for a complete
audit trail. `DriftAction` derives `Serialize`/`Deserialize` for persistence.

### 12.2 Integrated Edge Seeding

Concept edges (input regions where predictions fail) are detected via
leave-one-out cross-validation (`detect_concept_edges()`). Once registered
via `set_concept_edges()`, the shell automatically replaces the bottom 25%
of boards each generation with new boards whose column values are biased
toward the detected edge features. This implements directed mutagenesis —
evolution explores where it matters most.

### 12.3 AKD1000 Int4 Weight Export

`export_akd1000_weights()` produces an `Akd1000Export` struct:

- **Quantization**: symmetric min-max to [-8, 7] (int4 range)
- **Scales/biases**: per-target dequantization factors
- **Validation**: `predict_dequantized()` for software-hardware comparison,
  `quantization_mse()` measures precision loss (validated: MSE = 0.004)

### 12.4 Full Brain Rehearsal

A new `full_brain_rehearsal` example validates the complete pipeline:

1. Drift-monitored evolution (15 gens, N_e·s stable 2.4–22.4)
2. Concept edge detection at phase boundary
3. Edge-seeded re-evolution
4. AKD1000 int4 export + quantization validation
5. Save/restore with **bit-perfect** prediction match (delta < 10⁻¹⁶)
6. Instance transfer + merge (2 instances, 30 combined entries)
7. Reset for production

### 12.5 Validation Summary

| Metric | Before | After |
|--------|--------|-------|
| Unit tests | 20 | 31 |
| Examples | 3 | 5 |
| LOO CG error | 5.3% | 5.3% (unchanged — new features don't degrade accuracy) |
| Blind Exp 029 prediction | — | 2.6% CG error (never trained on dynamical data) |
| AKD1000 quantization MSE | — | 0.004 |
| State persistence | Partial | Bit-perfect (including drift actions) |

---

## 13. ToadStool S80 Absorption

As of {{ entity(name="toadstool") }} Session 80 (March 2, 2026), the Nautilus Shell has been
absorbed into `barracuda::nautilus` as a standalone module:

- **7 files, 22 tests** — board, evolution, population, readout, shell, brain
- **CPU-only** — no GPU dependency; ridge regression via `solve_f64_cpu`
- **JSON-RPC API** — 8 `ai.nautilus.*` methods in the {{ entity(name="toadstool") }} daemon
  (status, observe, train, predict, screen, edges, shell.export, shell.import)
- **Feature-gated** — `nautilus` feature in toadstool-cli

{{ entity(name="bingocube") }} remains the canonical implementation for inter-primal use (beardog,
songbird handshakes). {{ entity(name="toadstool") }}'s absorption provides standalone AI capability
without cross-primal dependencies — consistent with the self-knowledge principle.
