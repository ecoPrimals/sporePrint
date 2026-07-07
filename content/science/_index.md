+++
title = "🔬 Science"
description = "28 baseCamp papers across 6 research domains. Each is executable — run the code, reproduce the results, on your own hardware."
sort_by = "title"
template = "science_section.html"
+++

{{ total_stat(stat="basecamp_papers") }} {{ entity(name="basecamp") }} papers across 6 research domains. Every paper is executable — the science is runnable code that reproduces published results on commodity hardware. "Sovereign" means **self-hosted and cloud-independent**.

Each paper stands alone as a potential publication. Together they demonstrate that a pure-Rust, self-hosted computing ecosystem produces real, publishable science across physics, biology, medicine, neuroscience, game design, and economics.

---

## Physics & Materials
*7 papers — {{ entity(name="hotspring") }}, {{ entity(name="groundspring") }}*

| # | Paper | What it proves |
|---|-------|---------------|
| 01 | [Anderson Localization as QS Null Hypothesis](@/science/01_anderson_qs.md) | 3D Anderson model as quorum sensing null hypothesis (W_c = 16.26) |
| 06 | [Anderson as No-Till Soil Health](@/science/06_notill_anderson.md) | No-till as dimensional collapse of QS geometry; 9 papers reproduced |
| 07 | [Sovereign WDM Simulation](@/science/07_sovereign_wdm.md) | Warm dense matter on consumer GPU; guideStone v0.7.0 certified |
| 10 | [First Dynamical QCD on Consumer GPU](@/science/10_dynamical_qcd_production.md) | First dynamical fermion production on consumer GPU |
| 14 | [Sovereign Compute Hardware](@/science/14_sovereign_compute_hardware.md) | Precision tier taxonomy, temporal arbitrage, heterogeneous GPU |
| 23 | [Mass-Energy-Information Equivalence](@/science/23_mass_energy_information_equivalence.md) | Unifying hypothesis for why all springs share the same GPU primitives |
| 25 | [Self-Tuning Simulation](@/science/25_self_tuning_simulation.md) | Runtime spectral discovery eliminates hand-tuned simulation parameters |

## Microbiology & Genomics
*6 papers — {{ entity(name="wetspring") }}, {{ entity(name="airspring") }}*

| # | Paper | What it proves |
|---|-------|---------------|
| 02 | [LTEE Extensions](@/science/02_ltee_extensions.md) | Falsifiable Anderson-QS predictions for LTEE populations |
| 03 | [BioAg Microbiome](@/science/03_bioag_microbiome.md) | Anderson-derived microbiome design for perennial tree crops |
| 04 | [Sentinel Microbes](@/science/04_sentinel_microbes.md) | ESN classifiers on live Akida NPU silicon (1.4 uJ/inference) |
| 05 | [Cross-Species Signaling](@/science/05_cross_species_signaling.md) | Cold seep metagenomes, cross-kingdom QS, eavesdropper enrichment |
| 09 | [Field Genomics](@/science/09_field_genomics.md) | Sovereign NCBI-to-Anderson pipeline for real-time eDNA |
| 16 | [Anaerobic-Aerobic QS](@/science/16_anaerobic_aerobic_qs.md) | Anaerobic-aerobic transition modeling via Anderson framework |

## Immunology & Drug Discovery
*3 papers + interactive explorer — {{ entity(name="healthspring") }}, {{ entity(name="neuralspring") }}*

| # | Paper | What it proves |
|---|-------|---------------|
| 12 | [Anderson in Immunological Signaling](@/science/12_immunological_anderson.md) | Anderson localization in immune signaling; drug repurposing pipeline |
| 13 | [Sovereign Human Health](@/science/13_sovereign_human_health.md) | Sovereign PK/PD modeling, biosignal analysis, drug discovery |
| 22 | [Zero-Knowledge Medical Provenance](@/science/22_zero_knowledge_medical_provenance.md) | Patient-owned records with consent certificates, zero-knowledge proofs |
| — | [**Gonzales Interactive Explorer**](@/science/gonzales_explorer.md) | IC50, PK decay, tissue geometry, hormesis — live charts |

## Neural Networks & Computation
*6 papers — {{ entity(name="neuralspring") }}, {{ entity(name="groundspring") }}*

| # | Paper | What it proves |
|---|-------|---------------|
| 08 | [NPU Agricultural IoT](@/science/08_npu_agricultural_iot.md) | Akida NPU for real-time soil/crop monitoring at coin-cell power |
| 11 | [BingoCube Nautilus Shell](@/science/11_bingocube_nautilus_shell.md) | Nautilus shell as NP-structure validator; Turing-complete card game |
| 15 | [Precision Brain on Heterogeneous GPU](@/science/15_precision_brain_heterogeneous_gpu.md) | Brain simulation on heterogeneous consumer GPU cluster |
| 24 | [All-Silicon Science](@/science/24_all_silicon_science.md) | Mapping physics to all 9 GPU silicon unit types |
| 26 | [Neuromorphic Sovereign Driver](@/science/26_neuromorphic_sovereign_driver.md) | Pure Rust Akida NPU driver — VFIO, FBZ reverse engineering, 80-NPU mesh |
| 27 | [Nature Preserve](@/science/27_nature_preserve_applied_npu_science.md) | Applied NPU science across 7 domains |

## Game Science & Creative Computing
*4 papers — {{ entity(name="ludospring") }}*

| # | Paper | What it proves |
|---|-------|---------------|
| 17 | [Game Design as Rigorous Science](@/science/17_game_design_rigorous_science.md) | 13 foundational game models validated through Python-to-Rust-to-GPU |
| 18 | [RPGPT Sovereign RPG Engine](@/science/18_rpgpt_sovereign_rpg_engine.md) | Sovereign RPG engine with ingestible rulesets and provenance |
| 19 | [Games@Home Distributed Computation](@/science/19_games_at_home_distributed_human_computation.md) | Composable multi-player coordination via primal architecture |
| 28 | [Esoteric Webb](@/science/24_esotericwebb_composition_patterns.md) | Primal composition as creative infrastructure (Disco Elysium inspired) |

## Provenance & Economics
*2 papers — {{ entity(name="sweetgrass") }}, {{ entity(name="loamspine") }}, {{ entity(name="bingocube") }}*

| # | Paper | What it proves |
|---|-------|---------------|
| 20 | [Novel Ferment Transcript Economics](@/science/20_novel_ferment_transcript_economics.md) | Radiating attribution through provenance chains |
| 21 | [Sovereign Sample Provenance](@/science/21_sovereign_sample_provenance.md) | Field-to-publication chain-of-custody with BearDog signing |

---

**See also**: [Cross-Spring Evidence Map](@/science/CROSS_SPRING_EVIDENCE_MAP.md) for convergent predictions across springs, and [Structure Prediction Roadmap](@/science/STRUCTURE_PREDICTION_ROADMAP.md) for the {{ entity(name="helixvision") }} structure prediction pipeline (primitives {{ maturity(level="reproduced") }}, pipeline {{ maturity(level="architectural") }}).
