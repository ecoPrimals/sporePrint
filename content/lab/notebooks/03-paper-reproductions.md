+++
title = "Paper Reproductions — 63/63 Papers in Sovereign Rust"
description = "Rendered from 03-paper-reproductions.ipynb"
date = 2026-06-10
weight = 50

[extra]
domain = "Lab"
rendered_from = "03-paper-reproductions.ipynb"
+++

<!-- Auto-generated from 03-paper-reproductions.ipynb by spore-validate render-notebooks -->

# Paper Reproductions — 63/63 Papers in Sovereign Rust

wetSpring reproduced results from **63 peer-reviewed papers** across
5 research groups and 4 tracks. Every reproduction has:

- A Python/R baseline from the original methodology
- A Rust implementation with quantitative parity checks
- A GPU validation tier (50/50 three-tier eligible)
- Full provenance chains via NUCLEUS composition

This notebook maps the evidence across researchers, departments, and domains.

---

*For other springs: create your own researcher/paper map. The structure is:
one section per PI, one row per experiment, frozen JSON for the numbers.*

```python
import json
from pathlib import Path

import matplotlib
# matplotlib backend set by environment
import matplotlib.pyplot as plt
import numpy as np

RESULTS = Path('..') / 'experiments' / 'results'

def load_if_exists(path):
    p = RESULTS / path
    if p.exists():
        with open(p) as f:
            return json.load(f)
    return None
```

## Researcher Map

| Researcher | Department | Institution | Domain | Papers |
|------------|-----------|-------------|--------|--------|
| Christopher Waters | MMG | Michigan State | Quorum sensing, c-di-GMP | 15+ |
| Kevin Liu | CMSE | Michigan State | Comparative genomics, phylogenetics | 10+ |
| Jesse Cahill & Chuck Smallwood | Bioscience | Sandia National Labs | Biosurveillance | 5+ |
| A. Daniel Jones | BMB/Chemistry | Michigan State | Mass spectrometry, PFAS | 8+ |
| Rika Anderson | Biology | Carleton College | Vent metagenomics, pangenomics | 5+ |

## Track 1: 16S Metagenomics (Waters, Anderson)

The core pipeline — FASTQ to taxonomy to diversity. Galaxy/QIIME2
replaced entirely with sovereign Rust. 30 bio modules, 1 dependency.

```python
track1_experiments = {
    '001 Galaxy Bootstrap': load_if_exists('001_galaxy_bootstrap/validation_report.json'),
    '002 Phytoplankton': load_if_exists('002_phytoplankton/dada2-stats.tsv'),
    '003 Phage Defense': load_if_exists('003_phage/validation_report.json'),
}

# Galaxy bootstrap details
galaxy = load_if_exists('001_galaxy_bootstrap/validation_report.json')
if galaxy:
    print('Experiment 001: Galaxy Bootstrap')
    print(f'  Checks: {galaxy["checks_passed"]}/{galaxy["checks_passed"] + galaxy["checks_failed"]}')
    print(f'  ASVs: {galaxy["dada2"]["asv_count"]}')
    print(f'  Phyla: {galaxy["taxonomy"]["phyla_count"]}')
    print(f'  Status: {galaxy["validation"]}')
    print()

# R/vegan parity
r_div = load_if_exists('r_baselines/vegan_diversity.json')
if r_div:
    print('R/vegan Cross-Validation (Exp 335):')
    print(f'  Tool: vegan v{r_div["metadata"]["version"]}')
    metrics = ['shannon_uniform_10', 'simpson_uniform_10', 'chao1_estimate',
               'pielou_uniform', 'bray_curtis_ab']
    for m in metrics:
        if m in r_div:
            print(f'  {m}: {r_div[m]}')
    print(f'  Rarefaction monotonic: {r_div.get("rarefaction_monotonic", "?")}')
    print()
```

## Track 2: Analytical Chemistry (Jones)

LC-MS feature extraction (Asari), PFAS screening, VOC biomarkers,
spectral cosine matching.

```python
track2 = load_if_exists('track2_validation_report.json')
if track2:
    print(f'Track 2: {track2["total_passed"]}/{track2["total_checks"]} checks PASS')
    print(f'  Runtime: {track2["total_time_s"]}s')
    print()
    for key in ['exp005_asari', 'exp006_findpfas']:
        if key in track2:
            exp = track2[key]
            print(f'  {key}:')
            print(f'    Passed: {exp["passed"]}/{exp["total"]}')
            print(f'    Runtime: {exp["runtime"]}s')
            for k in ['features', 'compounds', 'candidates', 'unique']:
                if k in exp:
                    print(f'    {k}: {exp[k]:,}')
            print()

# Paper benchmarks
paper_dir = RESULTS / 'paper_benchmarks'
if paper_dir.exists():
    print('Paper-extracted benchmarks:')
    for f in sorted(paper_dir.glob('*.json')):
        data = json.loads(f.read_text())
        name = f.stem.replace('_', ' ').title()
        print(f'  {name}: {len(data)} entries')
```

## Track 3: Phylogenetics & Comparative Genomics (Liu)

Tree reconstruction, bootstrap support, ancestral state reconstruction,
HMM-based gene family analysis.

```python
phylo_experiments = [
    ('019 Phylogenetic', '019_phylogenetic'),
    ('021 RF Baseline', '021_rf_baseline'),
    ('022 Gillespie', '022_gillespie'),
    ('026 HMM', '026_hmm'),
    ('028 Alignment', '028_alignment'),
    ('029 Felsenstein', '029_felsenstein'),
    ('031 Bootstrap', '031_bootstrap'),
    ('032 Placement', '032_placement'),
    ('036 PhyNetPy RF', '036_phynetpy_rf'),
    ('037 PhyloHMM', '037_phylohmm'),
    ('038 SATE Pipeline', '038_sate_pipeline'),
]

track3_data = {}
print('Track 3: Phylogenetics & Comparative Genomics')
print(f'{"Experiment":<25s} {"Files":>6s}')
print('-' * 35)
for name, dirname in phylo_experiments:
    exp_dir = RESULTS / dirname
    if exp_dir.exists():
        files = list(exp_dir.glob('*'))
        track3_data[name] = len(files)
        print(f'{name:<25s} {len(files):>6d}')
    else:
        print(f'{name:<25s}    N/A')

print(f'\nTotal Track 3 experiments with data: {len(track3_data)}')
```

## Track 4: Soil & Environmental (Anderson + QS)

Anderson localization applied to soil microbial ecology — the key
scientific discovery bridging wetSpring to groundSpring. 9 soil
experiments validated against published field studies.

```python
soil_experiments = [
    ('170 Soil QS Pore Geometry', '170_soil_qs_pore_geometry', 'martinez2023'),
    ('171 Soil Pore Diversity', '171_soil_pore_diversity', 'feng2024'),
    ('172 Distance Colonization', '172_soil_distance_colonization', 'mukherjee2024'),
    ('173 No-Till Brandt Farm', '173_notill_brandt_farm', 'islam2014'),
    ('174 No-Till Meta-Analysis', '174_notill_meta_analysis', 'zuber2016'),
    ('175 Long-Term Tillage', '175_notill_longterm_tillage', 'liang2015'),
    ('176 Biofilm Aggregate', '176_soil_biofilm_aggregate', 'tecon2017'),
    ('177 Structure-Function', '177_soil_structure_function', 'rabot2018'),
    ('178 Tillage Microbiome', '178_tillage_microbiome', 'wang2025'),
]

soil_data = []
for name, dirname, author in soil_experiments:
    exp_dir = RESULTS / dirname
    if exp_dir.exists():
        jsons = list(exp_dir.glob('*.json'))
        if jsons:
            data = json.loads(jsons[0].read_text())
            soil_data.append({'name': name, 'author': author, 'data': data})

print(f'Track 4: {len(soil_data)} soil experiments with frozen baselines')
print()
for sd in soil_data:
    top_keys = [k for k in sd['data'].keys() if k != 'math_verification']
    print(f'  {sd["name"]} ({sd["author"]}): {len(top_keys)} data sections')
```

## Aggregate Evidence

```python
# Count all experiment directories with data
all_dirs = [d for d in RESULTS.iterdir() if d.is_dir()]
dirs_with_json = [d for d in all_dirs if list(d.glob('*.json')) or list(d.glob('*.tsv'))]

researchers = [
    ('Christopher Waters', 'MMG, MSU', 'Quorum sensing', 15),
    ('Kevin Liu', 'CMSE, MSU', 'Phylogenetics', 10),
    ('A. Daniel Jones', 'BMB, MSU', 'Mass spectrometry', 8),
    ('Rika Anderson', 'Biology, Carleton', 'Metagenomics', 5),
    ('Cahill & Smallwood', 'Bioscience, Sandia', 'Biosurveillance', 5),
]

fig, axes = plt.subplots(1, 2, figsize=(14, 6))

# Researcher contribution
ax = axes[0]
names = [r[0] for r in researchers]
papers = [r[3] for r in researchers]
colors = ['#2ecc71', '#3498db', '#e67e22', '#9b59b6', '#e74c3c']
bars = ax.barh(names, papers, color=colors)
ax.set_xlabel('Papers Reproduced')
ax.set_title('Papers by Researcher')
for bar, val in zip(bars, papers):
    ax.text(bar.get_width() + 0.3, bar.get_y() + bar.get_height()/2,
            f'{val}+', va='center', fontsize=10)

# Track coverage
ax = axes[1]
tracks = ['Track 1\n16S', 'Track 2\nLC-MS', 'Track 3\nPhylo', 'Track 4\nSoil',
          'Track 5\nDeep-sea', 'Track 6\nAnaerobic']
track_exps = [3, 8, 11, 9, 5, 4]
ax.bar(tracks, track_exps, color='#2c3e50')
ax.set_ylabel('Experiments with Frozen Baselines')
ax.set_title('Experiments by Track')
for i, v in enumerate(track_exps):
    ax.text(i, v + 0.2, str(v), ha='center', fontsize=10)

plt.suptitle(f'63/63 Papers — {len(dirs_with_json)} Experiments with Frozen Data',
             fontsize=14, fontweight='bold')
plt.tight_layout()
plt.savefig('/tmp/wetspring_03_papers.png', dpi=150, bbox_inches='tight')
plt.show()

print(f'\nTotal experiment directories: {len(all_dirs)}')
print(f'Directories with frozen data: {len(dirs_with_json)}')
```

## Key Discovery: Three-Tier Validation

wetSpring pioneered the pattern now used across all 8 springs:

```
Tier 1: Python/R baseline → frozen JSON
Tier 2: Rust implementation → [OK]/[FAIL] parity checks
Tier 3: GPU (barraCuda) → tolerance-checked acceleration
```

50 of 50 three-tier eligible papers have complete CPU + GPU + metalForge
validation. The remaining 13 papers are CPU-only (no GPU math component).

---

**baseCamp Papers**: 01, 03, 04, 05, 06 on [primals.eco/science](https://primals.eco/science/)

**Faculty Briefings**: `whitePaper/baseCamp/*.md` in the wetSpring repository

**Source**: [syntheticChemistry/wetSpring](https://github.com/syntheticChemistry/wetSpring)

