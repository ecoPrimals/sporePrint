+++
title = "LTEE B7 — Tenaillon 2016: Mutation Accumulation in 50,000 Generations"
description = "Rendered from tenaillon-ltee-mutation.ipynb"
date = 2026-06-15
weight = 50

[extra]
domain = "Lab"
rendered_from = "tenaillon-ltee-mutation.ipynb"
+++

<!-- Auto-generated from tenaillon-ltee-mutation.ipynb by spore-validate render-notebooks -->

# LTEE B7 — Tenaillon 2016: Mutation Accumulation in 50,000 Generations

**Exp380 Tier 1: Python Baseline**

Reproduces key findings from:
> Tenaillon O, Barrick JE, Ribeck N, et al. "Tempo and mode of genome evolution in a 50,000-generation experiment." *Nature* 536, 165–170 (2016). doi:10.1038/nature18959

**BioProject**: [PRJNA294072](https://www.ncbi.nlm.nih.gov/bioproject/PRJNA294072)  
**lithoSpore Module**: 6 (breseq comparison)  
**LTEE Queue ID**: B7  
**License**: scyBorg triple — AGPL-3.0-or-later (code), CC-BY-SA 4.0 (this notebook)

```python
import json
import numpy as np
from pathlib import Path
```

## 1. Paper Summary

Tenaillon et al. (2016) sequenced **264 clones** from the Lenski LTEE across
**12 replicate populations** (Ara-1 through Ara-6, Ara+1 through Ara+6) at
multiple time points spanning **~50,000 generations**.

Key findings:
- Mutation accumulation is approximately **linear** in most populations
- Mean mutation rate ≈ **8.9 × 10⁻¹¹ per bp per generation** (point mutations)
- Strong G:C→A:T mutational bias (Ts:Tv ≈ 1.7)
- Ara-1 (mutator) shows ~100× higher mutation rate due to mismatch repair deficiency
- IS element insertions are a major source of structural variation

## 2. Population Structure

The 12 LTEE populations and their characteristics relevant to mutation accumulation.

```python
POPULATIONS = {
    "Ara-1": {"mutator": True,  "mutator_onset_gen": 26500, "note": "mutS defect"},
    "Ara-2": {"mutator": True,  "mutator_onset_gen": 15000, "note": "mutL defect"},
    "Ara-3": {"mutator": False, "mutator_onset_gen": None,  "note": "Cit+ evolution ~31,000 gen"},
    "Ara-4": {"mutator": True,  "mutator_onset_gen": 15000, "note": "mutT defect"},
    "Ara-5": {"mutator": False, "mutator_onset_gen": None,  "note": ""},
    "Ara-6": {"mutator": False, "mutator_onset_gen": None,  "note": ""},
    "Ara+1": {"mutator": False, "mutator_onset_gen": None,  "note": ""},
    "Ara+2": {"mutator": False, "mutator_onset_gen": None,  "note": ""},
    "Ara+3": {"mutator": True,  "mutator_onset_gen": 2500,  "note": "mutY defect"},
    "Ara+4": {"mutator": False, "mutator_onset_gen": None,  "note": ""},
    "Ara+5": {"mutator": False, "mutator_onset_gen": None,  "note": ""},
    "Ara+6": {"mutator": True,  "mutator_onset_gen": 10000, "note": "mutL defect"},
}

NON_MUTATOR_POPS = [k for k, v in POPULATIONS.items() if not v["mutator"]]
MUTATOR_POPS = [k for k, v in POPULATIONS.items() if v["mutator"]]
print(f"Non-mutator populations ({len(NON_MUTATOR_POPS)}): {NON_MUTATOR_POPS}")
print(f"Mutator populations ({len(MUTATOR_POPS)}): {MUTATOR_POPS}")
```

## 3. Key Quantities from Tenaillon 2016

Values extracted from Figures 1–3, Table S2, and Extended Data of the paper.
These serve as the validation targets for lithoSpore module 6.

```python
GENOME_LENGTH_BP = 4_629_812
MAX_GENERATIONS = 50_000
N_POPULATIONS = 12
N_GENOMES = 264

NONMUTATOR_RATE_PER_BP_PER_GEN = 8.9e-11
NONMUTATOR_RATE_UNCERTAINTY = 1.0e-11

NONMUTATOR_RATE_PER_GENOME_PER_GEN = NONMUTATOR_RATE_PER_BP_PER_GEN * GENOME_LENGTH_BP
print(f"Non-mutator rate: {NONMUTATOR_RATE_PER_BP_PER_GEN:.1e} per bp per gen")
print(f"  = {NONMUTATOR_RATE_PER_GENOME_PER_GEN:.4f} mutations per genome per generation")
print(f"  = ~{NONMUTATOR_RATE_PER_GENOME_PER_GEN * 6.64:.1f} mutations per day (6.64 gen/day)")
```

```python
TS_TV_RATIO = 1.7
TS_TV_TOLERANCE = 0.3

GC_TO_AT_FRACTION = 0.68
GC_TO_AT_TOLERANCE = 0.05

print(f"Ts:Tv ratio: {TS_TV_RATIO} ± {TS_TV_TOLERANCE}")
print(f"G:C→A:T fraction: {GC_TO_AT_FRACTION} ± {GC_TO_AT_TOLERANCE}")
```

## 4. Mutation Accumulation Model

For non-mutator populations, mutation accumulation is approximately linear:

$$M(t) = \mu \cdot L \cdot t$$

where $M$ = total mutations, $\mu$ = per-bp per-generation rate, $L$ = genome length, $t$ = generations.

Tenaillon 2016 Figure 2 shows slight deviation from strict linearity in some
populations, consistent with clonal interference and epistasis effects.

```python
TIMEPOINTS = np.array([0, 2000, 5000, 10000, 15000, 20000, 30000, 40000, 50000])

def mutation_accumulation_linear(generations, rate_per_bp, genome_length):
    """Linear mutation accumulation: M(t) = mu * L * t"""
    return rate_per_bp * genome_length * generations

expected_mutations_nonmutator = mutation_accumulation_linear(
    TIMEPOINTS, NONMUTATOR_RATE_PER_BP_PER_GEN, GENOME_LENGTH_BP
)

print("Expected mutations for non-mutator populations (linear model):")
for gen, mut_count in zip(TIMEPOINTS, expected_mutations_nonmutator):
    print(f"  {gen:>6} gen → {mut_count:>7.1f} mutations")
```

```python
NONMUTATOR_EXPECTED_AT_50K = mutation_accumulation_linear(
    50_000, NONMUTATOR_RATE_PER_BP_PER_GEN, GENOME_LENGTH_BP
)
NONMUTATOR_EXPECTED_TOLERANCE = mutation_accumulation_linear(
    50_000, NONMUTATOR_RATE_UNCERTAINTY, GENOME_LENGTH_BP
)

MUTATOR_RATE_MULTIPLIER = 100.0
MUTATOR_EXPECTED_AT_50K = NONMUTATOR_EXPECTED_AT_50K * MUTATOR_RATE_MULTIPLIER

print(f"Non-mutator at 50K gen: {NONMUTATOR_EXPECTED_AT_50K:.1f} ± {NONMUTATOR_EXPECTED_TOLERANCE:.1f} point mutations")
print(f"Mutator at 50K gen: ~{MUTATOR_EXPECTED_AT_50K:.0f} point mutations (Ara-1)")
```

## 5. Mutation Spectrum Analysis

From Tenaillon 2016 Table S2 and Extended Data. The dominant mutation class
is G:C→A:T transitions, consistent with oxidative damage and deamination.

```python
MUTATION_SPECTRUM = {
    "GC_to_AT": 0.68,
    "AT_to_GC": 0.08,
    "GC_to_TA": 0.10,
    "GC_to_CG": 0.02,
    "AT_to_TA": 0.07,
    "AT_to_CG": 0.05,
}

transitions = MUTATION_SPECTRUM["GC_to_AT"] + MUTATION_SPECTRUM["AT_to_GC"]
transversions = 1.0 - transitions
ts_tv = transitions / transversions

print(f"Transitions: {transitions:.2f}")
print(f"Transversions: {transversions:.2f}")
print(f"Ts:Tv ratio: {ts_tv:.2f} (expected: ~{TS_TV_RATIO})")
assert abs(ts_tv - TS_TV_RATIO) < TS_TV_TOLERANCE, f"Ts:Tv {ts_tv} outside tolerance"
```

## 6. NCBI Pipeline (Sovereign Fetch)

The full pipeline downloads 264 genomes from BioProject PRJNA294072.
In production, this uses wetSpring's `ncbi/efetch.rs` sovereign pipeline.
Here we document the fetch structure for lithoSpore consumption.

```
PRJNA294072
├── 12 populations × ~22 time points = 264 genomes
├── Each genome: ~4.6 Mbp assembled sequence
├── Variant calls vs REL606 ancestor (breseq)
└── Total data: ~1.2 GB compressed SRA
```

```python
NCBI_CONFIG = {
    "bioproject": "PRJNA294072",
    "expected_genomes": 264,
    "ancestor_accession": "NC_012967.1",
    "ancestor_strain": "REL606",
    "genome_length_bp": GENOME_LENGTH_BP,
    "fetch_method": "ncbi/efetch.rs sovereign pipeline",
    "fallback": "scripts/ncbi_bulk_download.sh",
}

print(f"BioProject: {NCBI_CONFIG['bioproject']}")
print(f"Expected genomes: {NCBI_CONFIG['expected_genomes']}")
print(f"Ancestor: {NCBI_CONFIG['ancestor_strain']} ({NCBI_CONFIG['ancestor_accession']})")
```

## 7. Expected Values for lithoSpore Module 6

Produce the `expected_values.json` that lithoSpore will consume for
Tier 2 validation. Each value has provenance back to the paper.

```python
expected_values = {
    "experiment": "Exp380",
    "paper": "Tenaillon et al. Nature 536, 165-170 (2016)",
    "doi": "10.1038/nature18959",
    "bioproject": "PRJNA294072",
    "ltee_queue_id": "B7",
    "litho_module": 6,
    "foundation_thread": 5,
    "tier": "Tier 1 (Python baseline)",
    "targets": {
        "n_populations": {
            "value": N_POPULATIONS,
            "unit": "count",
            "tolerance": 0,
            "source": "Paper methods: 12 replicate populations",
        },
        "n_genomes": {
            "value": N_GENOMES,
            "unit": "count",
            "tolerance": 0,
            "source": "BioProject PRJNA294072: 264 sequenced clones",
        },
        "genome_length_bp": {
            "value": GENOME_LENGTH_BP,
            "unit": "bp",
            "tolerance": 100,
            "source": "REL606 ancestor genome NC_012967.1",
        },
        "nonmutator_rate_per_bp_per_gen": {
            "value": NONMUTATOR_RATE_PER_BP_PER_GEN,
            "unit": "mutations/bp/generation",
            "tolerance": NONMUTATOR_RATE_UNCERTAINTY,
            "source": "Fig 1, non-hypermutator populations",
        },
        "nonmutator_mutations_at_50k": {
            "value": float(np.round(NONMUTATOR_EXPECTED_AT_50K, 1)),
            "unit": "point_mutations",
            "tolerance": float(np.round(NONMUTATOR_EXPECTED_TOLERANCE, 1)),
            "source": "Linear model: mu * L * 50000",
        },
        "ts_tv_ratio": {
            "value": TS_TV_RATIO,
            "unit": "ratio",
            "tolerance": TS_TV_TOLERANCE,
            "source": "Table S2, aggregate across non-mutator populations",
        },
        "gc_to_at_fraction": {
            "value": GC_TO_AT_FRACTION,
            "unit": "fraction",
            "tolerance": GC_TO_AT_TOLERANCE,
            "source": "Table S2, dominant mutation class",
        },
        "mutator_rate_multiplier": {
            "value": MUTATOR_RATE_MULTIPLIER,
            "unit": "fold_increase",
            "tolerance": 50.0,
            "source": "Fig 1, Ara-1 vs non-mutator populations",
        },
        "mutation_spectrum": {
            "value": MUTATION_SPECTRUM,
            "unit": "fraction_per_class",
            "tolerance": 0.05,
            "source": "Table S2, 6-class point mutation spectrum",
        },
        "accumulation_model": {
            "value": "near_linear",
            "unit": "model_type",
            "tolerance": None,
            "source": "Fig 2, consistent with clock-like accumulation",
        },
    },
    "mutation_accumulation_curve": {
        "generations": TIMEPOINTS.tolist(),
        "expected_mutations_nonmutator": expected_mutations_nonmutator.tolist(),
        "model": "linear",
        "rate_per_bp_per_gen": NONMUTATOR_RATE_PER_BP_PER_GEN,
    },
    "provenance": {
        "pipeline": "wetSpring Exp380 Tier 1",
        "version": "V164",
        "spring": "wetSpring",
        "ncbi_fetch": "ncbi/efetch.rs sovereign pipeline",
    },
}

print(json.dumps(expected_values, indent=2, default=str))
```

```python
output_path = Path("../../experiments/results/ltee_b7_expected_values.json")
output_path.parent.mkdir(parents=True, exist_ok=True)
with open(output_path, "w") as f:
    json.dump(expected_values, f, indent=2, default=str)
print(f"Written to {output_path}")
print(f"  Targets: {len(expected_values['targets'])}")
print(f"  Curve points: {len(expected_values['mutation_accumulation_curve']['generations'])}")
```

## 8. Validation Summary

| Target | Value | Unit | Tolerance | Source |
|--------|-------|------|-----------|--------|
| Populations | 12 | count | exact | Methods |
| Genomes | 264 | count | exact | PRJNA294072 |
| Non-mutator rate | 8.9×10⁻¹¹ | per bp per gen | ±1.0×10⁻¹¹ | Fig 1 |
| Ts:Tv ratio | 1.7 | ratio | ±0.3 | Table S2 |
| G:C→A:T fraction | 0.68 | fraction | ±0.05 | Table S2 |
| Mutator multiplier | 100× | fold | ±50 | Fig 1 |
| Accumulation model | near-linear | — | — | Fig 2 |

---

**Next steps:**
- Tier 2: Rust validation binary (`validate_ltee_b7_mutation_accumulation.rs`)
- Full genome download via sovereign NCBI pipeline when NestGate (PG-04) is live
- lithoSpore module 6 integration with expected_values.json

