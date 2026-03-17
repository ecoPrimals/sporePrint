# Sub-Thesis 05: Cross-Species Signaling in Symbiotic Systems

**Date:** March 1, 2026
**Status:** **Validated** — Cold seep metagenome analysis (Exp144-145: 13 checks, 299,355 QS genes across 170 metagenomes, 34 QS types). luxR phylogeny × geometry overlay (Exp146: 5 checks, 12 lineages). Eavesdropper enrichment confirmed (Exp142: 8 checks, R:P ratios across 6 habitats). Interkingdom QS review (Exp154: 6 checks). Correlated disorder strengthens cross-species QS (Exp151: 8 checks). V59 extension: cold seep sovereign pipeline (Exp185 — 8 checks: 50 synthetic communities, Bray-Curtis dissimilarity matrix, Anderson spectral classification of cold seep biomes); GPU cold seep spectral classification validated (Exp191 G04: 3 checks)
**Domain:** Symbiotic ecology, interspecies signaling, mutualism
**Novelty:** Anderson geometry predictions applied to multi-kingdom signaling
in lichen, rhizobia, coral holobionts, and other obligate symbioses

---

## Abstract

Most quorum sensing (QS) research focuses on single-species systems. But
the majority of microbial life exists in multi-species, multi-kingdom
communities. We ask: does the Anderson localization framework apply to
cross-species signaling? If QS signal propagation is governed by spatial
geometry and species diversity (disorder), then mixed-species communities
should follow the same dimensional rules — with the added complexity that
signal chemistry may differ between partners.

We examine three canonical symbiotic systems — lichen (fungus + photobiont
on rock), nitrogen-fixing root nodules (Bradyrhizobium in legume tissue),
and coral holobionts (coral + zooxanthellae + bacteria in calcium carbonate
matrix) — and predict which will support cross-species QS based on Anderson
geometry.

---

## 1. The Cross-Species QS Question

### 1.1 Known Interspecies Signals

QS is not always species-specific. Several signal classes are inherently
interspecies:

| Signal | Producer | Receiver | Specificity |
|--------|----------|----------|-------------|
| AI-2 (furanosyl borate diester) | LuxS in most bacteria | LsrB / LuxP receptors | Universal — conserved across Gram+/Gram- |
| AHL (N-acyl-homoserine lactone) | LuxI-family synthases | LuxR-family receptors | Moderate — sidechain varies but cross-talk common |
| DSF (diffusible signal factor) | RpfF | RpfC | Genus-level specificity |
| Indole | TnaA (tryptophanase) | Multiple targets | Universal interkingdom signal |
| Nod factors (lipo-chitooligosaccharides) | NodA/B/C | LysM receptors in plants | Highly specific (host range) |

AI-2 is the prime candidate for Anderson-governed interspecies signaling:
it is produced by nearly all bacteria (via the housekeeping enzyme LuxS)
and can be detected across phyla.

### 1.2 The Anderson Prediction

For cross-species signaling, the Anderson model applies with a modification:
species diversity still maps to disorder W (multiple species scatter/absorb
the signal), but signal chemistry determines whether a given species is
a "scatterer" or a "transparent medium."

- Species that produce AND respond to the same signal: active nodes in the lattice
- Species that neither produce nor respond: transparent (reduce effective lattice size)
- Species that absorb but don't relay: scatterers (increase effective disorder)
- Species that produce without responding: sources (break Anderson assumptions → anomaly)

## 2. Lichen: 2D Symbiosis on Rock Surfaces

### 2.1 Biology

Lichen consists of a mycobiont (fungus) and one or more photobionts (green
alga and/or cyanobacterium) in an intimate 2D-3D thallus structure on rock,
bark, or soil surfaces. The thallus is typically 0.1-5 mm thick.

### 2.2 Geometry Analysis

| Component | Geometry |
|-----------|----------|
| Thallus surface | 2D (flat crust on substrate) |
| Thallus interior | Quasi-3D (fungal hyphae create 3D meshwork, but thin) |
| Substrate interface | 2D (rock surface) |

The lichen thallus is a borderline case: its interior has 3D structure but
is only a few cell layers thick.

### 2.3 Anderson Prediction

- If thallus interior is treated as 2D: QS fails for typical lichen diversity
- If thallus interior is treated as 3D (thin film, L ~ 4-5): QS marginally
  active (Exp138: minimum colony size = 64 cells, L=4)
- The lichen must maintain its 3D thallus structure to support signaling

**Testable prediction**: crustose lichens (thinner, more 2D) should have
fewer QS genes than foliose/fruticose lichens (thicker, more 3D).

### 2.4 Known Signaling

Lichen signaling is poorly characterized. Some evidence:
- Fungal volatile organic compounds (VOCs) may coordinate with photobiont
- AHL production detected in lichen-associated bacteria (Grube et al. 2009)
- Lichen reconstitution requires contact signaling (similar to Myxococcus
  C-signal — an NP solution for 2D systems, Sub-thesis 01)

### 2.5 NCBI Extension

Search for QS genes (luxI/luxR, luxS) in lichen metagenomes vs free-living
counterparts of the same mycobiont species. Anderson predicts lower QS gene
density in thin crustose lichen and higher in thick foliose lichen.

## 3. Nitrogen-Fixing Root Nodules: Engineered 3D Symbiosis

### 3.1 Biology

Legumes (soybean, clover, alfalfa) form root nodules housing Rhizobium /
Bradyrhizobium bacteria. The plant creates a 3D intracellular structure
(infected cells packed with bacteroids) that provides the microaerobic
environment needed for nitrogenase.

### 3.2 Geometry Analysis

| Stage | Geometry | Anderson prediction |
|-------|----------|-------------------|
| Free-living in soil | 3D dilute (planktonic) | QS suppressed by dilution (Exp137) |
| Root surface colonization | 2D → 3D biofilm | QS becomes active as biofilm forms |
| Infection thread | 1D (tube) | QS fails (Exp128: all 1D localized) |
| Mature nodule interior | 3D dense, near-monoculture | QS strongly active (W ~ 0.5-2) |

This is a geometry journey: the bacterium transitions through four Anderson
regimes as it establishes the symbiosis.

### 3.3 The QS Regulatory Cascade

Rhizobium uses QS (AHL-based: TraI/TraR, CinI/CinR, RaiI/RaiR) to regulate:
- Ti plasmid conjugation (TraI/TraR)
- Symbiotic gene expression
- Exopolysaccharide production for root attachment
- Nitrogen fixation gene regulation

Anderson prediction: QS regulation should be stage-dependent:
1. Free-living in soil → no QS (dilution suppresses)
2. Root surface → QS activates as biofilm achieves 3D
3. Infection thread → QS temporarily fails (1D)
4. Nodule → QS strongly active (3D, low diversity)

**Testable prediction**: QS gene expression (cinI, traI) should show a
biphasic pattern — active on root surface, reduced in infection thread,
then strongly re-activated in mature nodule.

### 3.4 Cross-Kingdom Signaling

The plant produces flavonoids (luteolin, genistein) that induce Nod factor
production in Rhizobium. This is not QS per se, but it is diffusible
cross-kingdom signaling that follows the same physics.

Anderson prediction: flavonoid signaling from plant to bacterium should work
best in the 3D soil matrix (extended states) and fail in waterlogged/flooded
conditions (planktonic dilution). This explains why waterlogged legumes have
poor nodulation despite adequate rhizobial density.

## 4. Coral Holobiont: 3D Calcium Carbonate Matrix

### 4.1 Biology

Coral holobionts are complex communities:
- Coral animal (cnidarian)
- Zooxanthellae (Symbiodiniaceae dinoflagellates — photosynthetic endosymbionts)
- Bacteria (hundreds of species in mucus layer and skeleton)
- Archaea, fungi, viruses

The calcium carbonate skeleton provides a permanent 3D matrix.

### 4.2 Geometry Analysis

| Component | Geometry | Diversity | Anderson prediction |
|-----------|----------|-----------|-------------------|
| Mucus surface layer | 2D film | High (J ~ 0.7-0.9) | QS fails (2D + high W) |
| Skeleton interior | 3D matrix | Moderate (J ~ 0.4-0.6) | QS active (3D, W ~ 6-9) |
| Gastrovascular cavity | 3D fluid, dilute | Moderate | Dilution-dependent (Exp137) |
| Tissue layer | 2D-3D (thin but structured) | Low (host-selected) | QS active if > 64 cells thick |

### 4.3 Anderson Prediction for Coral Bleaching

Coral bleaching is the expulsion of zooxanthellae under heat stress. The
microbial community shifts dramatically.

- Healthy coral: diverse bacterial community → moderate W → 3D skeleton
  → QS active → coordinated microbial functions (pathogen suppression,
  nutrient cycling)
- Bleaching: diversity crashes → W drops → QS may persist in skeleton
  but community function degrades → disease susceptibility increases
- Post-bleaching: if community recovers diversity → W returns →
  coordinated function resumes

The Anderson regime acts as a stability indicator: the 3D coral skeleton
protects QS coordination even during diversity loss, providing resilience.
This is why corals are more resistant to perturbation than soft-bodied
marine organisms (no 3D matrix).

### 4.4 Cross-Kingdom Signal Candidates

- AHL production by coral-associated bacteria (Tait et al. 2010)
- Bacteria detect coral-produced DMSP (dimethylsulfoniopropionate)
- AI-2 as universal interkingdom signal within holobiont
- Quorum quenching enzymes in some coral-associated bacteria (regulate
  the QS dynamics of the community)

## 5. Additional Symbiotic Systems

### 5.1 Mycorrhizal Networks ("Wood Wide Web")

Arbuscular mycorrhizal (AM) fungi connect tree roots in a subterranean
network. The fungal hyphae create a 3D lattice through soil.

Anderson prediction: the mycorrhizal network IS a 3D geometry that enables
coordinated signaling. QS genes should be enriched in mycorrhizae-associated
bacteria compared to bulk soil bacteria at the same density.

### 5.2 Insect Gut Symbionts

Many insects maintain gut symbionts in specialized structures:
- Termite hindgut: 3D, dense, diverse → QS predicted active
- Aphid bacteriome: 3D, near-monoculture (Buchnera) → QS predicted active (low W)
- Honeybee gut: 3D, moderate diversity → QS predicted active

Anderson prediction: obligate symbionts in 3D structures retain QS.
Transient gut microbes (passing through without structure) lose QS
coordination.

### 5.3 Human Oral Microbiome

Dental plaque is a 3D biofilm: QS active. Saliva is planktonic: QS fails.
Periodontal pocket creates a 3D niche: QS active → coordinated virulence
of periodontal pathogens (Porphyromonas gingivalis uses AI-2).

Anderson prediction: periodontal disease treatment that disrupts 3D biofilm
structure (mechanical debridement → forces 2D geometry) uses Anderson
localization to suppress pathogen coordination.

## 6. NCBI Experimental Design

### 6.1 Comparative QS Gene Search

For each symbiotic system, query NCBI for QS genes in:
- Symbiotic metagenomes (isolation_source: "lichen", "root nodule", "coral")
- Free-living metagenomes of the same taxa

Predicted result:
- Root nodule > free-living soil (3D dense vs 3D dilute)
- Foliose lichen > crustose lichen (thicker 3D)
- Coral skeleton > coral mucus (3D vs 2D)

### 6.2 AI-2 as Interspecies Bridge

AI-2 (LuxS-produced) is the most likely interspecies signal. Search NCBI
for luxS in symbiotic vs free-living isolates. However, note the confound:
luxS is a housekeeping gene (part of the activated methyl cycle), so
presence alone doesn't confirm QS function. Pair with lsrB (the AI-2
receptor specific to QS function).

### 6.3 Cross-Species Receptor Phylogeny

If cross-species signaling involves coevolution, we expect:
- luxR receptors in symbiotic bacteria phylogenetically closer to their
  partner's luxI synthases than to their own relatives' luxI
- This would be a coevolution signal detectable in the luxR evolutionary tree

Connects to Exp146 (luxR phylogeny overlay from Paper P3).

## 7. neuralSpring Connections

neuralSpring's ML primitives (S135: 966 lib tests, 232 binaries,
3,034+ checks, 5 WDM surrogates complete) apply directly to cross-species
signaling analysis:

- **HMM forward/backward** (`hmm.rs`): Detect introgression of QS genes
  between symbiotic partners. luxI/luxR gene family analysis across NCBI
  metagenomes can reveal horizontal transfer events in symbiotic contexts
- **PhyloNet-HMM** (Liu Papers 016-018): Distinguish vertical inheritance
  from horizontal transfer of signaling genes in mixed-species communities —
  critical for determining whether cross-species QS circuits are ancestral
  or recently acquired
- **ESN regime classifier** (nW-05, 96.5% accuracy): Classify symbiotic QS
  regimes from community composition features. The ESN's fixed-weight
  reservoir + ridge readout validates the pattern for rapid regime detection
  in multi-species systems without training infrastructure
- **Spectral primitives**: The same `eigh_f64` and IPR primitives that
  characterize Anderson localization in microbial communities (Sub-thesis 01)
  can quantify the "disorder" in symbiotic interaction networks

## 8. groundSpring Connections

groundSpring validates the mathematical foundation that this paper's
cross-species Anderson predictions rest on:

- **Exp 008 — Anderson localization** (Bourgain & Kachkovskiy 2018):
  The same 1D/2D/3D Anderson model used here to predict QS failure in
  lichen (2D) vs success in root nodules (3D). groundSpring validates
  the spectral diagnostic (level spacing ratio r, Thouless conductance)
  at benchmark precision. 8/8 Rust checks
- **Exp 012 — Spin chain transport** (Kachkovskiy 2016): Signal transport
  through disordered chains. Directly models the question of whether a
  QS signal can traverse a mycorrhizal network ("wood wide web") — each
  fungal node is a site in a disordered chain. 18/18 Rust checks
- **Exp 017 — Quasispecies threshold** (Dolson 2023): Eigen's error
  threshold for mutation-driven information collapse. In cross-species QS,
  signal fidelity degrades through multiple relay steps (Dictyostelium-like
  relay in mycorrhizal networks). The quasispecies threshold predicts when
  relay fidelity falls below the information limit. 6/6 Rust checks
- **Exp 016 — Rare biosphere signal detection** (R. Anderson 2015):
  Cross-species signaling often involves rare community members (e.g.,
  the eavesdropper species detected in Exp142). Quantifying when rare
  taxa are detectable vs noise is critical for mapping interaction
  networks in complex symbioses. 10/10 Rust checks
- **Exp 018 — Band edge structure** (Filonov & Kachkovskiy 2018): Band
  edges mark the boundary between propagating and evanescent states in
  periodic media. For the coral skeleton (periodic calcium carbonate
  lattice), band edge theory predicts QS signal propagation windows —
  frequencies where the signal passes through the skeleton vs where it
  is absorbed. 10/10 Rust checks

**Cross-species pipeline**: groundSpring (Anderson spectral validation
+ transport + error thresholds) → wetSpring (NCBI metagenome QS gene
search + luxR phylogeny) → neuralSpring (HMM introgression detection for
horizontal QS gene transfer between symbiotic partners).

## 9. Connection to Constrained Evolution

Symbiosis IS constrained evolution. The fungus in lichen cannot
photosynthesize; the alga cannot form a thallus. The constraint (absence
of the partner's capability) drives each organism toward specialization
that depends on the other. Anderson localization adds a physical layer:
the symbiosis must create a geometry that permits signaling, or signaling
mechanisms must evolve that circumvent the geometry.

The "NP solutions" from Sub-thesis 01 have direct symbiotic parallels:
- **Myxococcus geometry bootstrapping** ↔ lichen thallus formation (create
  the 3D structure that enables the signaling that maintains the structure)
- **Signal relay (Dictyostelium)** ↔ mycorrhizal network relay (each fungal
  node amplifies chemical signals along the network)
- **Logic inversion (V. cholerae)** ↔ quorum quenching in coral (detecting
  what's NOT present as a regulatory signal)

These are independent evolutionary discoveries of the same NP solutions,
in completely different biological contexts — convergent evolution of
signaling strategies, driven by the same Anderson constraint.
