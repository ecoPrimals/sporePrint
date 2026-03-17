# Paper 21: Sovereign Sample Provenance — Field-to-Publication Chain-of-Custody

**Status**: Active | **Date**: March 13, 2026
**Depends on**: Papers 04 (Sentinels), 09 (Field Genomics), 16 (Anaerobic QS), 20 (NFT Economics)
**Validated by**: ludoSpring exp062_field_sample_provenance (39/39 checks), exp064_beardog_signed_chain (39/39 checks), exp065_cross_domain_fraud (74/74 checks)
**License**: AGPL-3.0-or-later

---

## Abstract

The same provenance architecture that tracks game items tracks biological
samples. This paper demonstrates that rhizoCrypt DAG + loamSpine certificate +
sweetGrass braid + BearDog signature provides complete field-to-publication
chain-of-custody for scientific samples. Fraud detection reduces to graph
analysis — the same code that catches item duplication in gaming catches phantom
samples in a laboratory. Every biological sample is a Novel Ferment Transcript.

---

## 1. The Chain-of-Custody Problem

### Current State

Scientific chain-of-custody is typically maintained through:
- Paper logbooks (lossy, forgeable)
- Spreadsheet-based LIMS (no cryptographic integrity)
- Proprietary tracking systems (vendor lock-in, closed source)
- Manual compliance reporting (expensive, error-prone)

ISO 17025 and ISO 15189 require traceability but do not prescribe the mechanism.
Most labs satisfy the requirement with the minimum viable paper trail.

### What's Missing

| Gap | Consequence |
|-----|-------------|
| No cryptographic binding | Records can be altered after the fact |
| No immutable history | Chain breaks are invisible |
| No semantic attribution | Contributor credit is informal |
| No fraud detection | Fabrication discovered only through replication failure |
| No cross-lab interoperability | Each institution's format is unique |

### The Provenance Trio Solution

```
Field sample ←──loamSpine cert──► Lab analysis record
                              │
                              ├── rhizoCrypt: Collect → Transport → Store → Extract → Amplify → Sequence → Analyze → Publish
                              ├── loamSpine: custody transfers, condition changes, accession
                              ├── sweetGrass: collector, transporter, technician, analyst, PI attribution
                              └── BearDog: Ed25519 signature on every operation
```

---

## 2. Domain Model

### Sample Lifecycle

Every biological sample follows a directed acyclic graph of operations:

```
Collect ─► Transport ─► Store ─► Extract ─► Amplify ─► Sequence ─► Analyze ─► Publish
   │           │           │         │          │           │           │
   ▼           ▼           ▼         ▼          ▼           ▼           ▼
 [cert]    [custody]   [custody]  [process]  [process]   [process]  [process]
```

Each node is a rhizoCrypt vertex. Each transition is a custody transfer or
processing step recorded in the DAG. The loamSpine certificate carries the
sample's persistent identity — its accession number, sample type, collection
metadata, and condition.

### Data Types (from exp062)

| Type | Description | Provenance Mapping |
|------|-------------|-------------------|
| `SampleType` | Soil, Water, Swab, Tissue, Blood, Isolate | loamSpine cert attribute |
| `SampleCondition` | Fresh, Refrigerated, Frozen, Degraded, Destroyed | loamSpine cert state |
| `CustodyTransfer` | from_did, to_did, location, condition, temperature | rhizoCrypt vertex + loamSpine transfer |
| `ProcessingStep` | DNA extraction, PCR, sequencing, bioinformatics, QC | rhizoCrypt vertex |
| `SampleCertificate` | loamSpine cert with GPS, datetime, collector DID, accession | loamSpine mint |
| `SampleDag` | Full lifecycle DAG | rhizoCrypt session |
| `SampleAttribution` | Collector, transporter, technician, analyst, PI | sweetGrass braids |

### Mapping to Standards

| ecoPrimals | ISO 17025 | ISO 15189 | HIPAA (tissue) |
|-----------|-----------|-----------|----------------|
| loamSpine cert | Test item identification | Sample identification | Specimen tracking |
| rhizoCrypt DAG | Traceability chain | Pre-examination process | Chain of custody |
| sweetGrass braid | Personnel records | Competence records | Authorized personnel |
| BearDog signature | Data integrity | Information system security | Audit controls |

---

## 3. Fraud Detection as Graph Analysis

### The Universality Insight

exp065 proves that fraud detection across gaming, science, and medicine reduces
to the same five graph patterns:

| Generic Pattern | Gaming (exp053) | Science (exp062) |
|----------------|-----------------|-------------------|
| OrphanObject | OrphanItem | PhantomSample |
| DuplicateIdentity | DuplicateCert | DuplicateAccession |
| UnauthorizedAction | SpeedViolation | UnauthorizedAccess |
| ScopeViolation | ImpossibleKill | MislabeledSpecimen |
| BrokenChain | UnattributedLoot | BrokenColdChain |

This is not a metaphor. exp065 runs the same `GenericFraudDetector` code on
DAGs labeled with gaming vocabulary and science vocabulary. The detected fraud
types are identical — only the names change.

### Sample-Specific Fraud Types (exp062)

| Fraud Type | Detection Logic | ISO Impact |
|-----------|----------------|------------|
| **PhantomSample** | Analysis results with no collection vertex | 17025:7.3 — sample receipt |
| **DuplicateAccession** | Two samples claim same accession number | 17025:7.4 — identification |
| **BrokenColdChain** | Frozen → Fresh without documented reason | 15189:5.4.4 — transport conditions |
| **UnauthorizedAccess** | Processing by DID not in custody chain | 17025:6.2 — personnel |
| **MislabeledSpecimen** | Cert metadata vs. collection vertex mismatch | 15189:5.4.2 — labelling |
| **ContaminationGap** | Sequential processing without QC step | 17025:7.7.1 — contamination control |

---

## 4. Cross-Spring Architecture

### What wetSpring Gets

exp062 provides a concrete Rust pattern that maps directly to wetSpring's
existing field genomics architecture (sub_thesis_06):

```
wetSpring (field genomics)          exp062 (scaffold)
─────────────────────────           ─────────────────
field_sample_collection      ←─►   collect_sample()
sample_transport_log         ←─►   transport()
lab_processing_pipeline      ←─►   process()
publication_record           ←─►   publish()
quality_control_step         ←─►   process(QualityControl)
```

wetSpring teams adopt the domain model and fraud detectors directly. The
BearDog-signed chain (exp064) makes every custody transfer cryptographically
non-repudiable.

### Integration Path

1. wetSpring reads exp062 as a reference implementation
2. Adapts `SampleType` and `ProcessingStep` to their specific pipelines
3. Deploys rhizoCrypt + loamSpine + sweetGrass as biomeOS graph services
4. BearDog signs every operation via IPC (JSON-RPC over Unix socket)
5. songbird discovers the provenance trio services at runtime
6. The fraud detectors become the automated QC pipeline

---

## 5. Radiating Attribution for Science

### The Value Chain

When a scientific sample generates value — a publication, a patent, a dataset —
the sweetGrass attribution chain records every contributor:

```
Field collector (Creator)
    └── Transport technician (Contributor)
        └── Lab technician (Contributor)
            └── Bioinformatics analyst (Contributor)
                └── Principal investigator (Validator)
```

exp066 computes the radiating distribution:

| Role | Default Weight | Decayed Share (exp066) |
|------|---------------|----------------------|
| Creator (collector) | 1.0 | Highest |
| Contributor (technician) | 0.7 | Proportional |
| Contributor (analyst) | 0.7 | Proportional |
| Validator (PI) | 0.5 | Lower |

This is the sunCloud economic model applied to science: value radiates back
through the attribution chain to every contributor. The field collector who
spent three days in the mud gets permanent, cryptographically verifiable credit
for every publication that uses their sample.

---

## 6. Cryptographic Integrity (exp064)

Every operation in the sample lifecycle is signed:

| Operation | Signing Target | Verification |
|-----------|---------------|-------------|
| collect_sample | loamSpine cert mint | BearDog Ed25519 on cert content |
| custody_transfer | loamSpine transfer + DAG vertex | BearDog Ed25519 on both |
| process | DAG vertex | BearDog Ed25519 on vertex |
| publish | DAG vertex + sweetGrass braid | BearDog Ed25519 on both |

The `ProvenanceChainVerifier` (exp064) walks the entire chain and verifies
every signature. A single tampered vertex is detected at its exact position.

---

## 7. Experiment Validation Summary

| Experiment | Checks | Focus |
|-----------|--------|-------|
| exp062_field_sample_provenance | 39/39 | Sample lifecycle, custody chain, 6 fraud types, DAG isomorphism |
| exp064_beardog_signed_chain | 39/39 | Ed25519 signing, chain verification, tamper detection |
| exp065_cross_domain_fraud | 74/74 | Same detectors across gaming/science/medical, >80% structural similarity |
| exp066_radiating_attribution | 41/41 | Value distribution, decay, domain scenarios |

Total: **193 checks, 0 failures**.

---

## 8. Connections to Other Papers

| Paper | Connection |
|-------|-----------|
| 04 — Sentinels | Sample monitoring extends sentinel architecture |
| 09 — Field Genomics | Direct scaffold for wetSpring sample processing |
| 16 — Anaerobic QS | Quorum sensing models for sample colony analysis |
| 17 — Game Design | Same provenance patterns, different domain vocabulary |
| 18 — RPGPT | Anti-cheat = chain-of-custody (proven in exp065) |
| 20 — NFT Economics | Every sample is a Novel Ferment Transcript |

---

## 9. Future Work

- **wetSpring adoption**: Direct integration with field genomics pipeline
- **Real BearDog signing**: Replace model signatures with live IPC
- **Public anchor**: Optional blockchain anchoring for regulatory proof
- **Cross-institution provenance**: songbird discovery for multi-lab chains
- **ISO 17025 compliance matrix**: Formal mapping of all requirements to provenance trio operations
