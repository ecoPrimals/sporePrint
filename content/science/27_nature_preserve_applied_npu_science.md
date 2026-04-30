+++
title = "Nature Preserve — Applied NPU Science Across 7 Domains"
description = "The Nature Preserve bridges rustChip's curated model zoo to real scientific applications: physics, biology, audio, vision, environmental, genomic, and industrial domains. Each with problem statement, model selection, Rust code path, and extension points."
date = 2026-04-30

[extra]
paper_number = 27
domain = "Applied Neuromorphic Science"

[taxonomies]
primals = ["toadstool", "coralreef", "barracuda"]
springs = ["hotspring", "wetspring", "airspring", "neuralspring"]
+++

**Date:** April 30, 2026
**Status:** 7 domain application patterns documented. 29-model zoo (21 BrainChip + 4 physics + 4 Rust-native). Pure Rust conversion pipeline operational — no Python required at any stage.
**Domain:** Applied neuromorphic inference across physics, biology, audio, vision, environmental, genomic, and industrial science
**Novelty:** First documented application layer connecting a neuromorphic model zoo to multi-domain scientific use. First pure Rust pipeline from trained weights to deployed NPU model (.npy/.safetensors → quantize → FlatBuffer → Snappy → .fbz). First systematic bridge between curated AI models and domain-specific scientific workflows.
**Cross-Spring:** {{ entity(name="hotspring") }} (lattice QCD steering, ESN readout) × {{ entity(name="wetspring") }} (biological classifiers, bloom sentinel, spectral triage) × {{ entity(name="airspring") }} (environmental monitoring, ET₀) × {{ entity(name="neuralspring") }} (quantization validation, dispatch cost models)

---

## Abstract

The {{ entity(name="rustchip") }} Zoo contains 29 curated neural network models that
run on BrainChip Akida neuromorphic processors — parsed, converted, and validated
entirely in Rust. But a zoo is a collection of specimens. The **Nature Preserve**
is where those specimens live in their natural habitat: applied to real scientific
problems, with real data, producing real decisions.

This paper documents 7 domain application patterns that bridge the zoo to science:

| Domain | Problem | Zoo model | Spring evidence |
|--------|---------|-----------|----------------|
| **Physics** | Lattice QCD steering, phase classification | ESN readout, phase classifier | 5,978 live AKD1000 calls |
| **Biology** | Quorum sensing, phylogenetic placement, bloom sentinel | ESN → int8 classifiers | 11 wetSpring NPU binaries |
| **Audio** | Keyword spotting, streaming speech | DS-CNN KWS, TENN Recurrent | 3 BrainChip models parsed |
| **Vision** | Detection, segmentation, classification, face analysis | YOLO, UNet, AkidaNet | 14 BrainChip models parsed |
| **Environmental** | Bloom surveillance, ET₀, soil moisture | Streaming sensor, sentinel | airSpring + wetSpring NPU |
| **Genomic** | K-mer classification, genome binning, spectral triage | Multi-head readout | wetSpring + neuralSpring |
| **Industrial** | Predictive maintenance, sensor fusion, domain-shift | Streaming sensor 12ch, sentinel | Architecture patterns |

Each pattern follows the same shape: domain data → feature extraction → quantization
→ NPU inference → domain interpretation → output. The feature extraction and
interpretation steps are domain-specific; everything between is generic and handled
by {{ entity(name="rustchip") }}'s crates.

---

## 1. The Pipeline Pattern

Every domain follows one pipeline:

```
Domain data (measurements, signals, sequences)
    │
    ▼
Feature extraction (domain-specific, in Rust)
    │
    ▼
Quantization (f32/f64 → int4/int8, symmetric per-layer)
    │
    ▼
NPU inference (rustChip: parse model, load via VFIO, run)
    │
    ▼
Domain interpretation (classify, predict, detect, steer)
    │
    ▼
Output (decision, measurement, alert, next-step control)
```

The feature extraction step is where domain expertise lives. The NPU inference
step is where hardware acceleration lives. By separating them cleanly, the
same hardware pipeline serves seven different sciences.

---

## 2. Physics: NPU as Simulation Steering Engine

**Problem:** Lattice QCD trajectories take 10–60 seconds on GPU. Between
trajectories, the simulation must decide: accept or reject? continue
thermalizing? which observable to measure next? These decisions must happen
at sub-millisecond latency to avoid blocking the GPU.

**Model:** ESN readout (InputConv(50)→FC(128)→FC(1), int4). The echo state
network reservoir runs on CPU or GPU; only the readout runs on NPU.

**Evidence:** 5,978 live AKD1000 calls over 24 hours in {{ entity(name="hotspring") }}
Experiment 022. Phase classifier achieved 100% accuracy on confined vs
deconfined SU(3) gauge configurations.

**Extension:** The reservoir-readout split works for any system where a
temporal model must make fast decisions between expensive compute steps —
molecular dynamics, weather prediction, financial simulation.

---

## 3. Biology: NPU as Pre-filter for Sequencing Pipelines

**Problem:** Biological pipelines process millions of reads. At multiple
stages, small classification decisions gate expensive downstream computation.
The NPU provides microsecond per-read classification that reduces downstream
compute by 10–100×.

**Models:** ESN → int8 classifiers for quorum sensing (3-class), phylogenetic
placement (5-class), genome binning (10-class), spectral triage (2-class),
bloom sentinel (12-channel).

**Evidence:** 11 `validate_npu_*` binaries in {{ entity(name="wetspring") }},
covering QS classification, phylo placement, genome binning, spectral
screening, disorder classification, and bloom monitoring.

**Extension:** Any biological pipeline with a sparse-positive classification
step benefits — nanopore quality scoring, PFAS environmental screening,
pangenome navigation.

---

## 4. Audio: NPU for Always-On Acoustic Intelligence

**Problem:** Continuous audio streams require sub-100ms keyword detection
and event classification at milliwatt power, without cloud connectivity.

**Models:** DS-CNN KWS (41 KB, 33 keywords), TENN Recurrent SC12 (70 KB,
12 commands), TENN Recurrent UORED (37 KB, 4 utterances). All parsed and
validated by {{ entity(name="rustchip") }}'s FBZ parser.

**Extension:** Custom wake words, multi-language detection, acoustic event
classification (industrial sounds, environmental monitoring, accessibility).

---

## 5. Vision: NPU for Edge Perception

**Problem:** Visual perception at the edge — detect, segment, classify,
identify — at frame rate, without cloud dependency.

**Models:** 14 BrainChip pretrained models spanning object detection
(YOLO VOC, CenterNet), segmentation (UNet Portrait), classification
(AkidaNet ImageNet, PlantVillage, GXNOR MNIST), face analysis (FaceID,
UTK Face), gesture (DVS, Samsung), and 3D (PointNet++).

**Extension:** Agricultural monitoring (PlantVillage → crop-specific diseases),
multi-camera fusion via multi-tenancy, privacy-preserving face analysis
(embeddings on-device, no raw images transmitted).

---

## 6. Environmental: NPU for Continuous Ecosystem Monitoring

**Problem:** High-cadence environmental sensors generate continuous data
streams that need real-time classification without cloud latency — on buoys,
in fields, at remote watersheds.

**Models:** Streaming sensor 12ch (bloom classification), adaptive sentinel
(domain-shift detection), ET₀ regressor (evapotranspiration estimation).

**Evidence:** {{ entity(name="airspring") }} `validate_npu_eco` and
`validate_npu_high_cadence`; {{ entity(name="wetspring") }}
`validate_npu_bloom_sentinel` and `validate_npu_sentinel_stream`.

**Extension:** Real-time irrigation control, satellite-derived feature
augmentation, drought early warning.

---

## 7. Genomic: NPU as Throughput Multiplier

**Problem:** Genomic analysis pipelines process terabytes. At multiple
stages, small classifiers gate expensive computation — the NPU pre-filters
at microsecond latency.

**Models:** K-mer classifiers, genome binning models, spectral triage
pre-filters, multi-head population genetics readouts.

**Evidence:** {{ entity(name="wetspring") }} `validate_npu_genome_binning`,
`validate_npu_spectral_triage`, `validate_npu_spectral_screen`;
{{ entity(name="neuralspring") }} `validate_introgression`,
`validate_pangenome_selection`, `validate_upstream_kmer`.

**Extension:** Nanopore real-time quality scoring, adaptive sampling,
PFAS environmental genomics.

---

## 8. Industrial: NPU for Predictive Maintenance

**Problem:** Manufacturing and infrastructure produce high-volume sensor
data. "Is this normal?" is a continuous classification task that must run
on-site, in real time, without cloud dependency.

**Models:** Streaming sensor 12ch (multi-subsystem health), adaptive
sentinel (domain-shift detection), DVS gesture models (operator action
monitoring).

**Extension:** Frequency-domain vibration analysis, remaining useful life
regression, multi-model fleets on edge nodes (one self-contained Rust binary
per asset).

---

## 9. The Pure Rust Pipeline

All 7 domains share one conversion pipeline, implemented entirely in Rust:

1. **Import** weights from `.npy` (hand-rolled parser) or `.safetensors`
2. **Quantize** to int1/2/4/8 (symmetric max-abs, per-layer or per-channel)
3. **Serialize** to FlatBuffer (reverse-engineered Akida schema)
4. **Compress** with Snappy (matching the vendor's `.fbz` format)
5. **Write** the `.fbz` model file

```bash
cargo run -p akida-cli -- convert \
  --weights trained.npy \
  --arch "InputConv(50,1,1) FC(128) FC(1)" \
  --output model.fbz --bits 4
```

No Python. No TensorFlow. No vendor SDK. The pipeline handles round-trip
verification automatically.

---

## 10. How to Start

```bash
git clone https://github.com/syntheticChemistry/rustChip.git
cd rustChip
cargo build --workspace
cargo run -p akida-cli -- convert --weights "random:6400" \
  --arch "InputConv(50,1,1) FC(128) FC(1)" --output my_model.fbz --bits 4
cargo run -p akida-cli -- parse my_model.fbz
```

Full documentation: [QUICKSTART.md](https://github.com/syntheticChemistry/rustChip/blob/main/QUICKSTART.md),
[Nature Preserve](https://github.com/syntheticChemistry/rustChip/tree/main/baseCamp/preserve),
[LEVERAGE.md](https://github.com/syntheticChemistry/rustChip/blob/main/LEVERAGE.md).
