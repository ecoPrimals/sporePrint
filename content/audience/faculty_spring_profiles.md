+++
title = "Faculty Spring Profiles"
description = "Faculty network mapped to ecoPrimals springs — candidate papers, reproduction status, and BarraCuda GPU primitive coverage across 9 professors and 60+ candidate papers."
weight = 6
date = 2026-07-09

[taxonomies]
springs = ["hotspring", "wetspring", "airspring", "healthspring", "groundspring", "neuralspring"]
+++

{{ maturity(level="architectural") }}


**Status**: Working draft  
**Purpose**: Map known faculty to ecoPrimals springs, identify candidate papers for reproduction  
**Last Updated**: February 26, 2026

---

## How This Document Works

Each professor is profiled with:
1. **Connection** — how we know them and what spring(s) their work maps to
2. **Key Papers** — candidate publications for Phase A reproduction in the springs
3. **BarraCuda Relevance** — which GPU primitives their work exercises
4. **Status** — what has already been reproduced vs. what is candidate work

---

## hotSpring — Computational Plasma Physics

### Michael Murillo
**Associate Professor, CMSE, MSU**  
https://engineering.msu.edu/faculty/michael-murillo  
**Connection**: MSDS professor (master's program)

**Spring Status**: ALL PHASES COMPLETE — 22 papers, ~700 checks, 39/39 suites. Exp 022 (live NPU, 32⁴ production) finished Feb 27.

**Coffee meeting scheduled: Tuesday March 3, 2026.** Technical summary prepared: [hotSpring validation summary](@/lab/hotspring-validation-summary.md).

**Reproduced Papers (Murillo Group)**:
| Paper | Status | Checks |
|-------|--------|--------|
| Sarkas MD — Yukawa OCP (12 DSF cases) | Reproduced + GPU | 60/60 + 9/9 GPU |
| Two-Temperature Model (TTM) | Reproduced | 6/6 |
| Silvestri, Diaw, Murillo (2024) "Surrogate learning" — Nature MI | Reproduced + BarraCuda | 15/15 (478× faster) |
| Stanton & Murillo (2016) — Transport coefficients | Reproduced + GPU | 13/13 Green-Kubo |
| Murillo & Weisheit (1998) — Screened Coulomb | Reproduced | 23/23 Sturm bisection |
| Nuclear EOS (AME2020, 2,042 nuclei) | Reproduced + BarraCuda | L1/L2/L3 |

**Candidate Papers for Future Reproduction (Tier 4 — WDM)**:
- Diaw & Murillo (2023) "Generalized Hydrodynamics Model for Strongly Coupled Plasmas"
- Murillo (2025) "Computational barriers" (arXiv:2505.02494) — WDM roadmap paper

**BarraCuda primitives exercised**: GEMM, Velocity Verlet, Yukawa/Coulomb force kernels, MLP surrogate, RBF interpolation, Green-Kubo transport, Sturm bisection, DF64 arithmetic, SU(3) lattice gauge, HMC, ESN reservoir, NPU streaming

---

## airSpring — Evapotranspiration & Precision Irrigation

### Younsuk Dong
**Assistant Professor, Biosystems & Agricultural Engineering, MSU**  
https://www.egr.msu.edu/bae/water/irrigation/  
**Connection**: New lab job (2026)

**Spring Status**: 3,123+ total checks (594 Python + 491 Rust + 570 validation + 1393 atlas + 75 cross-val), 22 experiments, 27 binaries

**Reproduced Papers**:
| Paper | Status | Checks |
|-------|--------|--------|
| Dong (2020) soil sensor calibration | Reproduced | airSpring Phase A |
| Dong (2024) IoT irrigation pipeline | Reproduced | airSpring Phase A |
| FAO-56 Penman-Monteith reference ET₀ | Reproduced | airSpring Phase A |

**Experiments** (beyond original 3 Dong papers): Richards PDE, biochar P adsorption, dual Kc, cover crops, yield response, scheduling optimization, lysimeter, sensitivity analysis, atlas, PT/HG/Thornthwaite ET₀, GDD, Saxton-Rawls pedotransfer, CW2D, 60-year water balance.

**Candidate Papers for Future Reproduction**:
- Dong et al. — IoT soil moisture sensor network calibration (fieldwork data)
- Allen et al. (1998) FAO-56 — extended crop coefficient studies
- Regional ET₀ model comparisons across Michigan microclimates

**BarraCuda primitives exercised**: MLP surrogate (FAO-56 approximation), time-series LSTM (weather forecasting), data pipeline batch processing

---

## wetSpring — Microbial Ecology, Phage Biology, Environmental Chemistry

### Jesse Cahill
**Senior MTS, Sandia National Laboratories (Bioscience)**  
**Connection**: Time at Sandia

**Spring Area**: Track 1 — Life Science (algae ponds, phage biocontrol)

**Candidate Papers for Reproduction**:
- Cahill et al. — Phage-mediated biocontrol in algal raceway ponds
- Phage lifecycle dynamics and predator-prey oscillations in bioreactor systems
- Algal pond crash forensics — temporal metagenomics

**BarraCuda relevance**: Time-series anomaly detection (pond crash prediction), population dynamics ODE solvers

---

### Chuck Smallwood
**Principal MTS, Sandia National Laboratories (Bioscience)**  
**Connection**: Time at Sandia

**Spring Area**: Track 1 — Life Science (metagenomics, microbial community monitoring)

**Candidate Papers for Reproduction**:
- Smallwood et al. — Raceway pond metagenomic surveillance pipelines
- Microbial community stability metrics under perturbation

**BarraCuda relevance**: Sequence alignment (GEMM-heavy), dimensionality reduction, diversity index computation

---

### A. Daniel Jones
**Professor, Biochemistry & Molecular Biology / Chemistry, MSU**  
https://www.canr.msu.edu/news/center-for-pfas-research-faculty-spotlight-a-daniel-jones  
**Connection**: PFAS job

**Spring Area**: Track 2 — PFAS / blueFish

**Candidate Papers for Reproduction**:
- Jones et al. — PFAS mass spectrometry detection pipelines
- High-resolution mass spec peak identification and quantification
- Environmental PFAS fate-and-transport modeling

**BarraCuda relevance**: Signal processing (FFT, peak detection), spectral analysis shaders, anomaly detection in analytical chemistry data

---

### Christopher Waters
**Professor, Microbiology, Genetics & Immunology, MSU**  
https://directory.natsci.msu.edu/directory/Profiles/Person/101708  
https://mgi.natsci.msu.edu/labs/waters-lab/  
watersc3@msu.edu  
**Connection**: Undergraduate professor

**Spring Area**: wetSpring Track 1 — Microbial signaling, biofilm dynamics, quorum sensing

**Key Research Themes**:
1. **c-di-GMP signaling** — second messenger controlling biofilm ↔ motility switch in V. cholerae
2. **Quorum sensing** — density-dependent gene regulation via autoinducers
3. **Integration of signaling pathways** — c-di-GMP + quorum sensing convergence
4. **Phage defense** — deoxycytidine deaminase protection (Nature Microbiology 2022)
5. **Cancer immunotherapy** — cyclic di-nucleotides as immune adjuvants

**Candidate Papers for Reproduction**:

| Paper | Domain | Spring Relevance |
|-------|--------|-----------------|
| Waters et al. (2008) "Quorum Sensing Controls Biofilm Formation in V. cholerae Through Modulation of Cyclic Di-GMP." J Bacteriology | Signaling dynamics | wetSpring: ODE models of c-di-GMP concentration ↔ biofilm phenotype. Population-level signaling = quorum noise problem |
| Massie et al. (2012) "Quantification of High Specificity Cyclic di-GMP Signaling." PNAS | Signal specificity | groundSpring: How do cells resolve signal from noise when 60+ enzymes control a single diffusible molecule? |
| Hsueh, Severin et al. (2022) "A Broadly Conserved Deoxycytidine Deaminase Protects Bacteria from Phage Infection." Nature Microbiology | Phage defense | wetSpring: Phage-bacteria arms race dynamics. Evolutionary game theory models |
| Bruger & Waters (2018) "Maximizing Growth Yield and Dispersal via QS Promotes Cooperation in Vibrio Bacteria." AEM | Cooperation evolution | neuralSpring: Game-theoretic optimization, evolutionary strategy landscapes |
| Fernandez et al. (2020) "V. cholerae adapts to sessile and motile lifestyles by c-di-GMP regulation of cell shape." PNAS | Morphological adaptation | groundSpring: Phenotypic switching as bistable dynamical system. Bifurcation analysis |
| Mhatre et al. (2020) "One gene, multiple ecological strategies: a biofilm regulator is a capacitor for sustainable diversity." PNAS | Ecological diversity | neuralSpring: Constrained evolution — single regulatory node enabling phenotypic diversity |
| Waters (2021) "Au naturale: use of biologically derived cyclic di-nucleotides for cancer immunotherapy." Open Biol | Applied immunology | wetSpring: Bridge from fundamental microbiology to therapeutic applications |
| Srivastava et al. (2011) "Integration of Cyclic di-GMP and Quorum Sensing in the Control of vpsT and aphA." J Bacteriology | Pathway integration | neuralSpring: Multi-input regulatory network = attention mechanism analog |

**BarraCuda relevance**: ODE/PDE solvers (reaction-diffusion for signaling), stochastic simulation (Gillespie algorithm for quorum sensing), bifurcation analysis, graph computation (regulatory networks → neuralAPI pathway graphs)

---

## Master's Program Professors — Computational Methods

### Emily Dolson
**Assistant Professor, Computer Science & Engineering, MSU**  
https://engineering.msu.edu/directory/faculty/dolsonem  
dolsonem@msu.edu  
**Connection**: Master's program professor  
**Core faculty**: Ecology, Evolution, and Behavior program

**Key Research Themes**:
1. **Controlling evolutionary trajectories** — counterdiabatic driving applied to evolution
2. **Open-ended evolution** — measuring when systems produce genuine novelty
3. **Eco-evolutionary dynamics** — ecology and evolution as coupled processes
4. **Mathematical oncology** — evolutionary dynamics of cancer

**Candidate Papers for Reproduction**:

| Paper | Domain | Spring Relevance |
|-------|--------|-----------------|
| Iram, Dolson et al. (2020) "Controlling the speed and trajectory of evolution with counterdiabatic driving." Nature Physics | Evolutionary control theory | neuralSpring: Constrained evolution formalized — directly validates [Constrained Evolution Formal](@/methodology/CONSTRAINED_EVOLUTION_FORMAL.md) thesis. Can we reproduce the counterdiabatic protocol computationally? |
| Dolson & Vostinar et al. (2019) "The MODES Toolbox: Measurements of Open-Ended Dynamics in Evolving Systems." Artificial Life | Evolutionary metrics | neuralSpring: Metrics for measuring whether BarraCuda's constrained evolution produces open-ended innovation |
| Dolson & Ofria (2018) "Ecological Theory Provides Insights about Evolutionary Computation." GECCO | Theory bridge | neuralSpring: Ecological dynamics in evolutionary algorithms — maps to primal competition/cooperation in biomeOS |
| Dolson et al. (2023) "The ecology-evolution continuum and the origin of life." J R Soc Interface | Origins of life | wetSpring + groundSpring: Emergence of organization from chemical noise |
| Dolson et al. (2022) "Artificial selection methods from evolutionary computing show promise for directed evolution of microbes." eLife | Directed evolution | wetSpring: Computational → wet lab bridge. Directed evolution of microbial communities |
| Foreback, Bohm, Dolson (2025) "Leveraging Heterogeneous Controller Representations for Evolutionary Swarm Robotics." IEEE | Swarm intelligence | neuralSpring: Heterogeneous representations ↔ primal diversity. Different primals as different controller architectures |

**BarraCuda relevance**: Fitness landscape evaluation (GEMM), population simulation (parallel agents), evolutionary optimization (genetic algorithm shaders), diversity metrics computation

**Critical connection to [Constrained Evolution Formal](@/methodology/CONSTRAINED_EVOLUTION_FORMAL.md)**: Dolson's work on counterdiabatic driving of evolution is the closest published analog to the constrained evolution methodology described in [Constrained Evolution Formal](@/methodology/CONSTRAINED_EVOLUTION_FORMAL.md). Reproducing Iram et al. (2020) would provide external validation of the theoretical framework.

---

### Kevin Liu
**Associate Professor, CMSE, MSU**  
https://engineering.msu.edu/directory/faculty/kjl  
kjl@msu.edu  
**Connection**: Master's program professor  
**Faculty in**: Genetics & Genome Sciences, Ecology, Evolution & Behavior

**Key Research Themes**:
1. **Phylogenetic inference at scale** — SATé/SATé-II for massive tree estimation
2. **Introgression detection** — PhyloNet-HMM for gene flow across species
3. **Comparative genomics** — detecting adaptive introgression in eukaryotes
4. **Resampling methods** — bootstrap and RAWR for phylogenetic confidence

**Candidate Papers for Reproduction**:

| Paper | Domain | Spring Relevance |
|-------|--------|-----------------|
| Liu et al. (2009) "Rapid and accurate large-scale coestimation of sequence alignments and phylogenetic trees." Science | Phylogenetics | neuralSpring: Divide-and-conquer + iterative refinement = surrogate + optimization loop. SATé's co-estimation mirrors biomeOS pathway iteration |
| Liu et al. (2014) "An HMM-based Comparative Genomic Framework for Detecting Introgression in Eukaryotes." PLoS Comp Bio | HMM inference | neuralSpring: PhyloNet-HMM is a Hidden Markov Model — validates LSTM/sequence model primitives. State-space models on genomic data |
| Liu et al. (2015) "Interspecific Introgressive Origin of Genomic Diversity in the House Mouse." PNAS | Adaptive introgression | wetSpring + neuralSpring: Gene flow detection = transfer learning analog. Introgression = knowledge transfer between species |
| Wang et al. (2021) "Build a better bootstrap and the RAWR shall beat a random path to your door." Bioinformatics (ISMB) | Statistical resampling | groundSpring: Bootstrap/resampling methods for confidence estimation on noisy phylogenetic data |
| Alamin & Liu (2024) "Phylogenetic Placement of Aligned Genomes and Metagenomes with Non-tree-like Evolutionary Histories." IEEE/ACM TCBB | Metagenomics | wetSpring: Metagenomic placement = classifying environmental samples. Directly applicable to pond/soil microbiome analysis |
| Zheng et al. (2023) "The Impact of Species Tree Estimation Error on Cophylogenetic Reconstruction." BCB (top 10%) | Cophylogenetics | wetSpring: Host-microbe coevolution — fungal endosymbiont studies with Bonito lab |

**BarraCuda relevance**: Sequence alignment (GEMM-heavy Smith-Waterman), HMM forward/backward/Viterbi (matrix operations), phylogenetic likelihood computation (parallel tree evaluation), bootstrap resampling (embarrassingly parallel)

**Working manuscripts to watch**:
- "A phylogenomic study of adaptive co-evolution between early diverging fungi and obligate bacterial endosymbionts" — direct wetSpring relevance
- "Scalable statistical inference of species phylogenies from large-scale resequenced genomic datasets" — HPC/GPU candidate

---

### Alexei Bazavov
**Associate Professor, CMSE & Physics & Astronomy, MSU**  
https://directory.natsci.msu.edu/directory/Profiles/Person/101033  
bazavov@msu.edu  
**Connection**: Master's program professor  
**Affiliations**: CERN Theory Division, Fermilab Lattice, HPQCD, MILC Collaborations

**Key Research Themes**:
1. **Lattice QCD** — ab initio computation of strong force properties
2. **Hadronic vacuum polarization** — precision calculation for muon g-2
3. **Inverse problems** — spectral reconstruction from lattice data
4. **Parallel algorithms** — molecular dynamics on lattice gauge configurations

**Reproduced Papers**:
| Paper | Status | Checks |
|-------|--------|--------|
| Bazavov [HotQCD] (2014) "QCD equation of state" — Nuclear Physics A | **Reproduced** | hotSpring |
| Wilson (1974) / Bazavov — Pure gauge SU(3) quenched | **Reproduced + 32⁴ production** | Exp 013 + 022 |
| Kogut & Susskind (1975) — Dynamical fermion HMC | **Reproduced** | hotSpring |
| Bazavov et al. (2015) "Abelian Higgs model" — Phys Rev D | **Reproduced** | hotSpring |
| Bazavov et al. (2025) "Muon g-2 HVP" — Phys Rev D | **Reproduced** | hotSpring |
| Bazavov et al. (2016) "Freeze-out curvature" — Phys Rev D | **Reproduced** | hotSpring |

**Production Results**: Exp 013 (32⁴, native f64, 13.6h, β_c=5.69) and Exp 022 (32⁴, DF64+NPU, 14.2h, 10 NPU-steered β points, 5,900 measurements). Deconfinement transition confirmed. DF64 discovery: 9.9× native f64 throughput.

**Candidate Papers for Future Reproduction**:
| Paper | Domain | Spring Relevance |
|-------|--------|-----------------|
| Bazavov et al. (2016) "Polyakov loop 2+1 flavor" — Phys Rev D | Phase transitions | Dynamical fermion extension of current quenched work |
| (2025) Spectral reconstruction inverse problem (arXiv 2501.12259) | Inverse problems | groundSpring: spectral recovery from noisy lattice data |

**BarraCuda primitives exercised**: SU(3) GEMM (link multiplication), HMC molecular dynamics, Dirac CG solver, plaquette/Polyakov/susceptibility observables, DF64 arithmetic, PRNG (Philox), WGSL shaders (25).

**Critical connection to hotSpring**: Bazavov's lattice QCD and Murillo's plasma physics are both studying strongly coupled many-body systems. The computational methods overlap significantly — both use molecular dynamics, both need equation of state calculations, both deal with long-range correlations. The shared BarraCuda kernel library now serves both.

---

### Ilya Kachkovskiy
**Assistant Professor, Department of Mathematics, MSU**  
https://users.math.msu.edu/users/ikachkov/  
ikachkov@msu.edu  
**Connection**: Sold the NucBox M6 on Facebook Marketplace; brief GPU conversation  
**Previously at**: Institute for Advanced Study, UC Irvine  
**NSF funded**: DMS-1758326 "Spectral theory of periodic and quasiperiodic quantum systems"

**Key Research Themes**:
1. **Anderson localization** — how disorder causes quantum waves to localize (absence of transport in disordered media)
2. **Spectral theory of quasiperiodic operators** — eigenvalue structure of systems that are "almost periodic" (structured noise)
3. **Transport in quantum spin systems** — energy/information propagation through spin chains
4. **Almost commuting operators** — approximate symmetries in quantum systems (C*-algebra framework)

**Reproduced Papers**:
| Paper | Status | Checks |
|-------|--------|--------|
| Anderson localization (1D/2D/3D, quasiperiodic) | **Reproduced** | 45/45 |
| Hofstadter butterfly (Harper equation) | **Reproduced** | hotSpring |
| Kachkovskiy & Saenz (2016) — spectral theory validation | **Reproduced** | hotSpring |
| GPU Lanczos eigenvalue solver | **Validated** | hotSpring |

**Candidate Papers for Future Reproduction**:

| Paper | Domain | Spring Relevance |
|-------|--------|-----------------|
| Bourgain & Kachkovskiy (2018) "Anderson localization for two interacting quasiperiodic particles." GAFA | Localization theory | Two-particle extension of current 1D work |
| Jitomirskaya & Kachkovskiy (2018) "All couplings localization" — JEMS | Quasiperiodic systems | Stronger localization proofs → validation targets |
| Filonov & Kachkovskiy (2018) "Band edges of 2D periodic operators" — Acta Math | Band structure | 2D eigenvalue mathematics |
| Kachkovskiy (2016) "Transport properties of quasiperiodic XY spin chains" — CMP | Quantum transport | Spin chain transport ↔ Murillo plasma transport |

**BarraCuda relevance**: Eigenvalue solvers (Lanczos — validated on GPU), sparse matrix-vector products, Hofstadter butterfly computation, Anderson localization IPR. The spectral methods require f64 precision — same requirement as hotSpring's plasma MD and Bazavov's lattice QCD.

**Why this matters**: Kachkovskiy provides the *mathematical* layer that sits between Murillo's physics (transport in classical plasmas) and Bazavov's physics (transport in quantum field theories). His spectral theory is the eigenvalue mathematics both of them use but neither of them proves. Anderson localization — the core of his research — is the rigorous mathematical framework for "when does signal propagate vs. when does noise trap it?" That's groundSpring's central question stated in the language of quantum mechanics.

**Co-author network**: Jean Bourgain (Fields Medalist, IAS — deceased 2018), Svetlana Jitomirskaya (UCI, Dannie Heineman Prize), Nikolay Filonov (Steklov Institute), Yuri Safarov (King's College London). This is a tier-1 mathematics pedigree.

---

### Rika Anderson
**Associate Professor, Department of Biology, Carleton College**  
https://www.carleton.edu/directory/randerson/  
randerson@carleton.edu  
**Connection**: Found via literature search — her work on *Sulfolobus* in Yellowstone hot springs is the living experimental corollary to the Taq polymerase argument in [Constrained Evolution Formal](@/methodology/CONSTRAINED_EVOLUTION_FORMAL.md)  
**Previously at**: University of Washington (MS, PhD); Virtual Planetary Laboratory (NASA)

**Key Research Themes**:
1. **Microbial evolution in deep-sea hydrothermal vents** — how extreme environments shape microbial genomes (Nature Communications 2017)
2. **Stochastic vs deterministic evolution in low-biomass environments** — when does genetic drift dominate natural selection? (mSystems 2021, 2022)
3. **Population genomics of extremophiles** — *Sulfolobus* in Yellowstone, *Sulfurovum* at vents, subseafloor archaea
4. **Viral ecology and host-virus coevolution** — CRISPRs as metagenomic tools, phage biogeography
5. **Pangenomics and gene gain/loss** — selection vs drift in functional evolution

**Candidate Papers for Reproduction**:

| Paper | Domain | Spring Relevance |
|-------|--------|-----------------|
| Campbell, Anderson et al. (2017) "*Sulfolobus islandicus* meta-populations in Yellowstone National Park hot springs." Env Microbiol 19:2392-2405 | Hot spring ecology | hotSpring + [Constrained Evolution Formal](@/methodology/CONSTRAINED_EVOLUTION_FORMAL.md): **This is the direct experimental study of organisms in the same Yellowstone hot springs where *Thermus aquaticus* was discovered.** Population genomics of an extremophilic archaeon under thermal constraint. Living proof that the constrained environment drives population differentiation |
| Anderson (2021) "Tracking Microbial Evolution in the Subseafloor Biosphere." mSystems 6:e00731-21 | Evolutionary theory | groundSpring + [Constrained Evolution Formal](@/methodology/CONSTRAINED_EVOLUTION_FORMAL.md): Formalizes when stochastic forces (drift) dominate over deterministic forces (selection) in extreme environments. Cites Lenski LTEE. Directly supports §1.2 of the constrained evolution thesis |
| Anderson et al. (2017) "Genomic variation in microbial populations inhabiting the marine subseafloor at deep-sea hydrothermal vents." Nature Communications 8:1114 | Population genomics | wetSpring + groundSpring: How extreme geochemistry shapes genome-level variation. Selection signatures at single-nucleotide resolution. dN/dS analysis of microbial populations under constraint |
| Moulana, Anderson et al. (2020) "Selection is a significant driver of gene gain and loss in the pangenome of Sulfurovum." mSystems 5:e00673-19 | Pangenomics | neuralSpring: Constrained evolution of bacterial pangenomes — gene gain/loss under selective pressure at hydrothermal vents. Functional evolution ↔ feature selection in ML |
| Mateos, Anderson et al. (2023) "The evolution and spread of sulfur-cycling enzymes reflect the redox state of the early Earth." Science Advances 9:eade4847 | Enzyme evolution | wetSpring: Traces enzyme evolution across geological time using phylogenomics. Co-evolution of enzymes with their geochemical environment = constrained evolution over 3+ billion years |
| Boden, Anderson et al. (2024) "Timing the evolution of phosphorus-cycling enzymes through geological time." Nature Communications 15:3703 | Deep-time evolution | wetSpring: Uses tree reconciliation to date metabolic innovations. Bioinformatics pipeline directly applicable to sovereign 16S/metagenomics |
| Anderson et al. (2014) "Evolutionary strategies of viruses and cells in hydrothermal systems revealed through metagenomics." PLoS ONE 9:e109696 | Viral ecology | wetSpring: Phage-host interactions in vent ecosystems — connects to Cahill phage biocontrol and Waters phage defense work |
| Anderson, Sogin, Baross (2015) "Biogeography and ecology of the rare and abundant microbial lineages in deep-sea hydrothermal vents." FEMS Microbiol Ecol 91:fiu016 | Microbial biogeography | wetSpring + groundSpring: Rare vs abundant lineages — noise floor of microbial diversity. When does a lineage constitute signal vs sampling noise? |

**BarraCuda relevance**: Sequence alignment (GEMM), diversity indices (reduction), dN/dS selection tests (pairwise comparison), phylogenetic tree construction (parallel likelihood), rarefaction curves (bootstrap resampling), pangenome analysis (set operations). All of these are already validated in wetSpring's sovereign pipeline.

**Why this is the corollary to Taq polymerase**: The constrained evolution thesis (§1.1) uses *Thermus aquaticus* from Yellowstone hot springs as its founding biological example — thermal constraint produced Taq polymerase, which enabled PCR and modern molecular biology. Anderson's lab has published the **population genomics** of another extremophile (*Sulfolobus islandicus*) living in the **exact same Yellowstone hot springs**. Her 2017 paper with Campbell shows how thermal constraint drives population differentiation, susceptibility to mobile genetic elements, and structured genomic variation in these hot spring populations. This is not a metaphor for the constrained evolution thesis — this is the empirical data that would appear in §1.1 if we were writing a full literature review. Additionally, her 2021 mSystems paper explicitly discusses stochastic vs deterministic evolution under environmental constraint, cites Lenski's LTEE (the same experiment that anchors §1.2), and introduces Muller's ratchet as a consequence of extreme energy limitation — all themes directly present in [Constrained Evolution Formal](@/methodology/CONSTRAINED_EVOLUTION_FORMAL.md).

**Co-author network**: John Baross (UW — pioneer of deep-sea microbiology), Julie Huber (WHOI — subseafloor biosphere), Mitch Sogin (MBL — rare biosphere), Rachel Whitaker (Illinois — *Sulfolobus* population genetics), Emily Stüeken (St Andrews — early Earth geochemistry), Ben Tully (USC — marine metagenomics).

---

## MSU Drug Discovery Program — Pharmacology & Toxicology

### Erika Lisabeth
**Director, Assay Development and Drug Repurposing Core (ADDRC), Pharmacology & Toxicology, MSU**
https://drugdiscovery.msu.edu/about-us/people/erika-lisabeth-ph-d.aspx
**Connection**: Referred by Gonzales (March 2026 interview)

**Key Research Themes**:
1. **HTS assay development** — converting bench-top assays to high-throughput screening format
2. **Drug repurposing** — identifying new indications for existing compounds
3. **EphA3 receptor tyrosine kinase** — somatic mutations inactivate EphA3 in cancer (postdoc work)
4. **NF-κB degradation pathway** — characterized inhibitor degradation (UCSD PhD)

**Candidate Papers for Reproduction**:

| Paper | Domain | Spring Relevance |
|-------|--------|-----------------|
| Lisabeth et al. (2024) "Using Small Molecules to Identify Critical Host-Cellular Pathways for Brucella Infection." *Spartan Medical Research Journal* | Drug screening | wetSpring: HTS hit identification pipeline — 8,000+ compounds screened. Demonstrates the assay → hit → validation workflow that our MATRIX scoring could augment |
| Lisabeth postdoc — EphA3 kinase mutations in cancer | Receptor biology | wetSpring: RTK signaling maps to Anderson framework — receptor inactivation as localization of kinase signal. Structural mutations → barrier height change |

**ADDRC Infrastructure**:
- 8,000+ compound library
- Liquid-handling robots, automated plate readers, high-content microscopes
- GREENScreen informatics for compound management
- Available 24/7 to MSU researchers and external biotech

**Spring Relevance**: The ADDRC is the institutional screening platform for the Anderson-augmented MATRIX drug repurposing scores (nS-605). Pipeline: computational scoring → ADDRC HTS → Gonzales iPSC validation. The Brucella screen (8,000 compounds) demonstrates the throughput; applying Anderson geometry scoring to compound selection is the extension.

**BarraCuda relevance**: Plate reader data processing (batch reduction), dose-response curve fitting (Hill equation — already in nS-601), diversity metrics for compound clustering, MATRIX scoring automation

---

### Richard Neubig
**Professor Emeritus, Director Drug Discovery Program, Pharmacology & Toxicology, MSU**
https://phmtox.msu.edu/people/rneubig
**Connection**: MSU Drug Discovery leadership; referred via Gonzales (March 2026)

**Key Research Themes**:
1. **GPCR signaling** — G-protein coupled receptor pharmacology (>25% of current drug targets)
2. **Rho/MRTF/SRF gene transcription** — small molecule inhibitors for fibrotic diseases
3. **Skin fibrosis** — Rho pathway inhibitors in dermal fibrosis models
4. **Melanoma metastasis** — MRTF/SRF inhibitors block metastatic phenotype
5. **Academic drug discovery** — founded UMich Center for Chemical Genomics, established MSU Drug Discovery

**Candidate Papers for Reproduction**:

| Paper | Domain | Spring Relevance |
|-------|--------|-----------------|
| Neubig group — Rho/MRTF/SRF inhibitors for skin fibrosis | Dermatology × Drug Discovery | wetSpring + neuralSpring: Skin fibrosis ↔ AD barrier disruption. If Rho pathway cross-talks with JAK/STAT ([Paper 12: Immunological Anderson](@/science/12_immunological_anderson.md) §8 Q7), Rho inhibitors become Anderson-scorable candidates for AD. Screen in Gonzales iPSC models |
| Neubig group — CCG-1423 series Rho pathway inhibitors | Chemical biology | wetSpring: Dose-response modeling (Hill equation, same as nS-601). Structure-activity relationships → Anderson barrier mapping |

**Spring Relevance**: Neubig's skin fibrosis work connects directly to [Paper 12: Immunological Anderson](@/science/12_immunological_anderson.md)'s dimensional promotion model — fibrosis changes tissue geometry, which changes Anderson localization of cytokine signals. If Rho/MRTF/SRF cross-talks with JAK/STAT in skin, his inhibitors become candidates for Anderson-augmented MATRIX scoring.

**BarraCuda relevance**: GPCR docking (future — structural), dose-response fitting (immediate — Hill equation), signaling pathway graph analysis

---

### Edmund Ellsworth
**Interim Director Drug Discovery, Director Medicinal Chemistry, MSU**
https://drugdiscovery.msu.edu/about-us/people/index.aspx
**Connection**: MSU Drug Discovery leadership

**Spring Relevance**: Downstream of HTS — medicinal chemistry optimization after ADDRC screening identifies hits. Not a direct spring reproduction target yet, but the endpoint of the computation → screening → chemistry pipeline.

---

## Cross-Spring Faculty Connections

```
                    hotSpring (Murillo, Bazavov, Kachkovskiy, R. Anderson)
                    ┌──────────────────────────┐
                    │ Strongly coupled           │
                    │ many-body systems          │
                    │ MD, EOS, spectral theory   │
                    │ GPU compute                │
                    │ Hot spring microbial pop.   │
                    │ genetics (Taq corollary)   │
                    └──────┬─────────────────────┘
                           │
          ┌────────────────┼────────────────┐
          │                │                │
   groundSpring            │         neuralSpring
   (Bazavov inverse,       │    (Dolson evolution,
    Kachkovskiy localize,  │     Liu HMM/phylo,
    R. Anderson stochastic)│     Bazavov parallel,
   ┌──────────────┐        │     Kachkovskiy spectral,
   │ Inverse probs│        │     R. Anderson pangenomics,
   │ Noise/signal │        │     Gonzales dose-response)
   │ Anderson loc │        │    ┌──────────────┐
   │ Stochastic vs│        │    │ ML primitives │
   │ deterministic│        │    │ Evolutionary  │
   │ Spectral recon│       │    │ optimization  │
   └──────┬───────┘        │    └──────┬───────┘
          │                │           │
          └────────┬───────┘───────────┘
                   │
            wetSpring (Cahill, Smallwood, Jones, Waters, Liu, R. Anderson,
                      Gonzales, Lisabeth, Neubig)
            ┌──────────────────────────┐
            │ Microbial ecology         │
            │ Metagenomics              │
            │ Quorum sensing/signaling  │
            │ PFAS detection            │
            │ Phage dynamics            │
            │ Vent population genomics  │
            │ Deep-time enzyme evolution│
            │ Immunological Anderson    │
            │ Drug repurposing (MATRIX) │
            │ HTS / ADDRC screening     │
            └──────────────────────────┘
```

## Priority Reproduction Candidates (Next Phase)

Ranked by: (1) direct spring relevance, (2) reproducibility, (3) BarraCuda kernel coverage

### Tier 1 — High priority, clear reproduction path
1. **Iram, Dolson et al. (2020)** — Nature Physics — Counterdiabatic evolution control → validates constrained evolution thesis
2. **Waters et al. (2008)** — J Bacteriology — c-di-GMP/QS biofilm model → ODE system, fully specified
3. **Liu et al. (2014)** — PLoS Comp Bio — PhyloNet-HMM introgression → HMM implementation validates sequence model primitives
4. **Bazavov [HotQCD] (2014)** — Nuclear Physics A — QCD EOS → extends hotSpring beyond plasma

### Tier 2 — Strong candidates, may need data access
5. **Dolson et al. (2019)** — MODES Toolbox — Open-ended evolution metrics → apply to BarraCuda's own evolution
6. **Massie et al. (2012)** — PNAS — c-di-GMP signaling specificity → quantitative signaling model
7. **Liu et al. (2009)** — Science — SATé phylogenetic estimation → large-scale divide-and-conquer benchmark
8. **Hsueh et al. (2022)** — Nature Microbiology — Phage defense deaminase → evolutionary arms race dynamics

### Tier 2 (new) — Empirical constrained evolution evidence
9. **Campbell, Anderson et al. (2017)** — Env Microbiol — *Sulfolobus* population genetics in Yellowstone hot springs → **direct empirical data for [Constrained Evolution Formal](@/methodology/CONSTRAINED_EVOLUTION_FORMAL.md) §1.1** (same environment as Taq polymerase)
10. **Anderson (2021)** — mSystems — Stochastic vs deterministic evolution in subsurface → **supports §1.2 Lenski argument**, cites LTEE, formalizes when drift dominates selection
11. **Moulana, Anderson et al. (2020)** — mSystems — Constrained evolution of *Sulfurovum* pangenomes → gene gain/loss driven by geochemistry = functional evolution under environmental constraint

### Tier 2.5 — Mathematical foundations, strengthens multiple springs
12. **Bourgain & Kachkovskiy (2018)** — GAFA — Anderson localization for interacting particles → groundSpring noise/signal theory, computational eigenvalue methods for BarraCuda
13. **Kachkovskiy (2016)** — CMP — Quasiperiodic spin chain transport → hotSpring plasma transport bridge, validates Lanczos/spectral BarraCuda primitives

### Tier 2.7 — Drug Discovery / Pharmacology (MSU Drug Discovery Program)
14. **Lisabeth et al. (2024)** — Spartan Med Res J — Brucella HTS screen (8,000+ compounds) → validates ADDRC pipeline throughput, demonstrates computation→screening workflow for Anderson-augmented MATRIX scoring
15. **Neubig group — Rho/MRTF/SRF skin fibrosis inhibitors** → wetSpring + neuralSpring: If Rho pathway cross-talks with JAK/STAT ([Paper 12: Immunological Anderson](@/science/12_immunological_anderson.md) §8 Q7), Rho inhibitors become Anderson-scorable candidates for AD. Dose-response modeling uses same Hill equation as nS-601
16. **Gonzales (2014–2021) — 6 papers (G1–G6)** → **DONE** in wetSpring Exp273–286 + neuralSpring nS-601–605 (359/359 checks). Oclacitinib JAK selectivity, IL-31 pruritus, lokivetmab PK, three-compartment tissue, cross-species

### Tier 3 — Longer horizon, connect to broader themes
17. **Bazavov et al. (2025)** — Phys Rev D — Muon g-2 HVP → precision lattice computation
18. **Fernandez et al. (2020)** — PNAS — Cell shape regulation → bistable dynamical systems
19. **Dolson (2023)** — J R Soc Interface — Ecology-evolution continuum → origin-of-life context
20. **Liu working manuscript** — Fungi-bacteria coevolution → cophylogenetic methods for wetSpring
21. **Filonov & Kachkovskiy (2018)** — Acta Math — 2D band edge structure → electronic/phononic transport, deep eigenvalue mathematics
22. **Mateos, Anderson et al. (2023)** — Science Advances — Sulfur-cycling enzyme evolution across geological time → deep-time validation of constrained evolution, bioinformatics pipeline
23. **Boden, Anderson et al. (2024)** — Nature Communications — Phosphorus-cycling enzyme timing → tree reconciliation methods, geological time enzyme dating
---

## Related

- [Lab](@/lab/_index.md) — reproduction infrastructure and validation summaries
- [Chapter 14: Biological Validation](@/thesis/14_biological_validation.md) — empirical support for constrained evolution
- [For Faculty and PIs](@/audience/FOR_FACULTY_AND_PIS.md) — audience guide for faculty engagement
