# sporePrint Publication Scaffold

**Purpose**: Reusable checklist for sporePrint's side of every publication.
**Pattern**: `wateringHole/protocols/PUBLICATION_PIPELINE_STANDARD.md`
**First instance**: hotSpring QCD (arXiv hep-lat)

---

## Per-Publication Checklist (sporePrint owns)

### 1. pseudoSpore Site Page

- [ ] Create `content/pseudospore/<name>.md`
- [ ] Include: measured benchmark table, computation pipeline, bundle anatomy
- [ ] Include: hardware profile (which gate, which GPU)
- [ ] Include: "Verify It" section linking to `/pseudospore/verify/`
- [ ] Include: "See Also" links to related lab/science/product pages
- [ ] Frontmatter: `maturity = "live"`, `validated_on_hardware = true`
- [ ] Taxonomies: tag relevant primals and springs
- [ ] Build + check-links + validate

### 2. arXiv Draft Scaffold (in whitePaper/subGen/)

- [ ] Create `subGen/<DOMAIN>_<TOPIC>_ARXIV.md`
- [ ] Write sporePrint-owned sections:
  - Abstract (shell — science team fills results sentence)
  - 1. Introduction (motivation, prior art gap, contributions)
  - 2.x Shader Pipeline (WGSL → naga → SPIR-V → backend)
  - 2.x Provenance (5-stage chain)
  - 4.x Cost Analysis (hardware amortization vs cloud)
  - 4.x Limitations (honest — what doesn't work yet)
  - 4.x Vendor Neutrality (cross-vendor argument)
  - 5. Reproducibility (pseudoSpore URL, license, verification)
  - Appendix A: Hardware Profile
  - Appendix B: Data Dependencies
- [ ] Mark science-team sections with `[TODO — requires <team> data]`
- [ ] Include table templates with `[pending]` cells
- [ ] Include references skeleton

### 3. Handoff Document (in wateringHole/handoffs/)

- [ ] Create `handoffs/<TEAM>_<TOPIC>_PUBLICATION_HANDOFF.md`
- [ ] List every `[TODO]` section with exact data format expected
- [ ] Include table templates the science team fills
- [ ] Include hype cleanup reminders
- [ ] Include publication path (arXiv → JOSS → journal)
- [ ] Specify: "Do not change the structure. Just fill the data cells."

### 4. Site Updates After Handoff Complete

- [ ] Review filled data for hype compliance
- [ ] Update pseudoSpore page with final numbers
- [ ] Convert markdown → LaTeX (REVTeX4-2 for hep-lat, elsarticle for CPC)
- [ ] Submit to arXiv
- [ ] Update pseudoSpore page with arXiv ID
- [ ] Update llms.txt with publication reference
- [ ] Update EVOLUTION_QUEUE.md

---

## Reusable Sections (copy-paste for every GPU paper)

### Shader Pipeline Section

```markdown
### 2.x Shader Pipeline

Compute shaders are authored in WGSL and compiled via:

    WGSL source → naga parser → SPIR-V IR → native backend
                                              ├── PTX (NVIDIA, sm_86+)
                                              ├── GCN/RDNA (AMD)
                                              └── Xe (Intel)

The compilation is performed by coralReef, a sovereign shader compiler built on
the naga crate. Dispatch is managed by toadStool via the wgpu WebGPU implementation
backed by Vulkan 1.2.
```

### Provenance Section

```markdown
### 2.x Provenance

Every computed [artifact] passes through a 5-stage cryptographic provenance
pipeline implemented by the Provenance Trio (rhizoCrypt, loamSpine, sweetGrass):

1. **BLAKE3 content hash** (nestGate) — deterministic content identity
2. **DAG insertion** (rhizoCrypt) — ephemeral parent/child lineage graph
3. **Ledger commit** (loamSpine) — permanent append-only record
4. **Ed25519 signature** (bearDog via sweetGrass) — cryptographic witness
5. **Attribution braid** (sweetGrass) — W3C PROV-O compliant provenance

The provenance chain is independently verifiable using standard tools
(b3sum for BLAKE3, any Ed25519 implementation for signatures, any
PROV-O parser for attribution).
```

### Reproducibility Section

```markdown
## 5. Reproducibility

All data, code, and provenance records are published as a downloadable
pseudoSpore artifact:

- **URL**: https://primals.eco/pseudospore/[name]/
- **Source**: https://git.primals.eco (sovereign) / https://github.com/ecoPrimals (mirror)
- **License**: AGPL-3.0-or-later (code), CC-BY-SA-4.0 (text)
- **Verification**: `./validate.sh` checks BLAKE3 hashes, CAS IDs, DAG chain,
  ledger entry, and Ed25519 signature with zero trust in the publisher
```

### Cost Analysis Template

```markdown
### 4.x Cost Analysis

| Item | Cost |
|------|------|
| GPU ([model]) | ~$[price] |
| Host system | ~$[price] |
| Electricity (compute portion) | ~$[monthly]/month |
| **Total for [N] [units]** | **~$[amortized]** |

Comparable cloud HPC ([instance type]): ~$[hourly]/hour.
A [duration] production run costs ~$[cloud] on cloud
vs ~$[sovereign] amortized on sovereign hardware.
```

---

## Future Publication Targets

| Domain | Spring | arXiv Category | Topic | Status |
|--------|--------|---------------|-------|--------|
| Lattice QCD | hotSpring | hep-lat | SU(2) Wilson gauge on consumer GPU | SCAFFOLD DONE |
| 16S Metagenomics | wetSpring | q-bio.GN | GPU-accelerated DADA2 pipeline | Not started |
| Precision Ag | airSpring | physics.ao-ph | FAO-56 ET₀ sovereign implementation | Not started |
| Pharmacometrics | healthSpring | q-bio.QM | Population PK on commodity hardware | Not started |
| Neuromorphic | neuralSpring | cs.NE | Akida NPU driver in pure Rust | Not started |
| Software | all | JOSS | ecoPrimals as research software | Blocked on first arXiv |

Each follows the same pattern. sporePrint scaffolds. Science team fills data.
Joint review. Publish.

---

*sporePrint is the publishing surface. The science teams are the data source.
The pattern is: scaffold → handoff → fill → review → publish → update site.*
