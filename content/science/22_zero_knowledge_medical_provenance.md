+++
title = "Zero-Knowledge Medical Provenance"
description = "Medical x Privacy — patient-owned records with consent certificates, zero-knowledge proofs. ludoSpring + healthSpring. 148+ checks."
date = 2026-03-17

[extra]
paper_number = 22
domain = "Human Health"

[taxonomies]
primals = ["beardog", "loamspine", "rhizocrypt", "songbird", "sweetgrass"]
springs = ["healthspring", "ludospring"]
+++

# Paper 22: Zero-Knowledge Medical Provenance — Patient-Owned Records with Consent Certificates

**Status**: Active | **Date**: March 13, 2026
**Depends on**: Papers 12 (Immuno-Anderson), 13 (Sovereign Health), 20 (NFT Economics)
**Validated by**: ludoSpring exp063_consent_gated_medical (35/35 checks), exp064_beardog_signed_chain (39/39 checks), exp065_cross_domain_fraud (74/74 checks)
**License**: AGPL-3.0-or-later

---

## Abstract

Patient-owned medical records via DID-based loamSpine certificates. Provider
access via loamSpine lending with consent certificates. Every access is a DAG
vertex. BearDog provides zero-knowledge access proofs — proving authorization
without revealing consent document contents. The same fraud detectors that catch
item duplication in games catch unauthorized medical record access. This paper
bridges game provenance architecture to clinical data sovereignty.

---

## 1. The Medical Data Problem

### Current State

Medical records are owned by institutions, not patients. The patient is a
passive subject whose data flows through systems they cannot audit:

| Problem | Impact |
|---------|--------|
| Institutional ownership | Patient cannot control who sees their data |
| No audit trail | Patient cannot verify who accessed their records |
| Consent is a paper form | Revocation is slow, unverifiable |
| No cross-provider interop | Each system stores its own copy |
| Breach detection is reactive | Unauthorized access found after the fact |

### What HIPAA Requires but Doesn't Enforce

HIPAA's Privacy Rule (§164.524) gives patients the right to access their
records. The Security Rule (§164.312) requires audit controls. But the
enforcement mechanism is complaint-driven and retrospective.

### The Provenance Trio Solution

```
Patient ←──loamSpine cert──► Medical record
                          │
                          ├── loamSpine: patient owns record cert, consent certs grant provider access
                          ├── rhizoCrypt: every access is a DAG vertex (who, when, what, why)
                          ├── sweetGrass: PROV-O attribution chain for HIPAA audit
                          └── BearDog: zero-knowledge proof that access was authorized
```

---

## 2. Domain Model

### Consent-Gated Access Architecture

The key insight: medical access is a **lending** problem. The patient owns the
record (loamSpine certificate). A consent certificate is a scoped loan — the
patient grants a provider time-limited, type-limited access to specific records.

```
Patient mints record cert
    │
    ├── Grant consent cert (provider DID, record types, expiry)
    │       │
    │       └── Provider accesses record
    │               │
    │               ├── DAG vertex logged (who, when, what, why)
    │               ├── sweetGrass braid created (PROV-O)
    │               └── BearDog signs access proof
    │
    └── Revoke consent cert (irreversible)
            │
            └── Further access blocked
```

### Data Types (from exp063)

| Type | Description | Provenance Mapping |
|------|-------------|-------------------|
| `RecordType` | Lab, Imaging, Prescription, Vitals, Encounter, Genomic | loamSpine cert attribute |
| `ConsentScope` | record_types + expiry_tick | loamSpine loan terms |
| `AccessEvent` | accessor, record, purpose, record_type, tick | rhizoCrypt vertex |
| `AccessProof` | BearDog-signed proof of valid consent at access time | Zero-knowledge artifact |
| `AuditTrail` | sweetGrass braid chain for complete access history | PROV-O export |

### Consent Certificate as loamSpine Loan

The consent certificate models HIPAA's minimum necessary standard:

| Consent Field | Provenance Mapping | HIPAA Alignment |
|-------------|-------------------|-----------------|
| Patient DID | loamSpine cert owner | Individual right (§164.524) |
| Provider DID | loamSpine loan borrower | Covered entity (§164.502) |
| Record types | Loan scope (e.g., Lab + Imaging) | Minimum necessary (§164.502(b)) |
| Expiry | Loan duration | Retention schedule |
| Revocable | Loan return (irreversible) | Right to revoke (§164.508) |

---

## 3. Zero-Knowledge Access Proofs

### The Problem

When a provider accesses a medical record, they need to prove authorization to
an auditor without revealing the full consent document. The consent may contain
sensitive scope information (e.g., "Genomic" implies a genetic condition).

### BearDog's Role

BearDog signs an `AccessProof` that asserts:

> "Provider X held valid consent for record type Y at time T"

The proof is verifiable without revealing:
- The full list of consented record types
- Other records the provider has access to
- The expiry of the consent
- The patient's other providers

This is not full-strength zero-knowledge proof (that requires a ZK circuit).
It is **selective disclosure** — the proof reveals only the minimum necessary
for verification. exp064 validates the signing protocol; the ZK circuit is
future work for BearDog.

### Access Flow

```
1. Provider sends access_record(record_id, purpose, record_type)
2. System validates: consent exists, not expired, not revoked, scope matches
3. DAG vertex appended (rhizoCrypt)
4. Attribution braid created (sweetGrass)
5. BearDog signs AccessProof
6. Provider receives record + proof
```

If any validation fails, no DAG vertex is created and no proof is issued.
The absence of a proof IS the evidence of unauthorized access.

---

## 4. Fraud Detection

### Same Detectors, Medical Vocabulary

exp065 proves the fraud detectors are domain-agnostic:

| Generic Pattern | Gaming | Medical (exp063) |
|----------------|--------|------------------|
| OrphanObject | OrphanItem | PhantomAccess |
| DuplicateIdentity | DuplicateCert | ConsentForgery |
| UnauthorizedAction | SpeedViolation | UnauthorizedAccess |
| ScopeViolation | ImpossibleKill | ScopeViolation |
| BrokenChain | UnattributedLoot | ExpiredConsent |

### Medical-Specific Fraud Types (exp063)

| Fraud Type | Detection Logic | HIPAA Impact |
|-----------|----------------|-------------|
| **UnauthorizedAccess** | Access event with no valid consent at that timestamp | §164.312(b) — audit controls |
| **ExpiredConsent** | Access after consent certificate expiry | §164.508(b)(6) — consent validity |
| **ScopeViolation** | Access to record type not covered by consent | §164.502(b) — minimum necessary |
| **PhantomAccess** | Record modified but no access vertex in DAG | §164.312(b) — audit controls |
| **ConsentForgery** | Consent cert not signed by patient's DID | §164.312(a)(1) — integrity |

---

## 5. Cross-Spring Architecture

### What healthSpring Gets

exp063 provides a consent/access model mapping directly to healthSpring's
clinical tracks:

```
healthSpring (clinical)              exp063 (scaffold)
───────────────────────             ──────────────────
pk_pd_data_access             ←─►   access_record(Lab)
microbiome_sequencing_access  ←─►   access_record(Genomic)
biosignal_monitoring_access   ←─►   access_record(Vitals)
trt_treatment_access          ←─►   access_record(Prescription)
patient_consent_management    ←─►   grant_consent() / revoke_consent()
compliance_audit              ←─►   audit()
```

### Integration Path

1. healthSpring reads exp063 as a reference implementation
2. Adapts `RecordType` to their specific clinical tracks
3. Deploys consent certificates as loamSpine services via biomeOS
4. BearDog signs every access event via IPC
5. The fraud detectors become the automated HIPAA audit pipeline
6. sweetGrass braids export as PROV-O for compliance reporting

---

## 6. Radiating Attribution for Medicine

### The Attribution Chain

When medical data generates value — a research publication, a treatment
protocol, a clinical trial — the sweetGrass attribution chain records:

```
Patient (Creator — their data)
    └── Referring physician (Contributor)
        └── Specialist (Contributor)
            └── Research team (Contributor)
```

exp066 computes the distribution. The patient, as Creator, receives the
highest attribution share. This is the fundamental inversion: the patient is
not a passive data subject but the primary contributor.

### Patient Data Sovereignty

The combination of:
1. **DID-based ownership** (loamSpine) — patient owns the certificate
2. **Consent-gated access** (loamSpine lending) — patient controls who sees what
3. **Cryptographic audit** (rhizoCrypt + BearDog) — patient can verify all access
4. **Radiating attribution** (sweetGrass + exp066) — patient gets credited

...constitutes a complete data sovereignty model. The patient is not asking
permission to see their own records. The institution is asking the patient for
permission to see theirs.

---

## 7. Experiment Validation Summary

| Experiment | Checks | Focus |
|-----------|--------|-------|
| exp063_consent_gated_medical | 35/35 | Consent lifecycle, access control, 5 fraud types, audit trail, access proofs |
| exp064_beardog_signed_chain | 39/39 | Ed25519 signing, chain verification, tamper detection |
| exp065_cross_domain_fraud | 74/74 | Same detectors across gaming/science/medical |
| exp066_radiating_attribution | 41/41 | Value distribution, patient-as-Creator scenarios |

Total: **189 checks, 0 failures**.

---

## 8. Connections to Other Papers

| Paper | Connection |
|-------|-----------|
| 12 — Immuno-Anderson | Immune modeling informs clinical data patterns |
| 13 — Sovereign Health | healthSpring architecture, clinical tracks |
| 17 — Game Design | Same provenance patterns, different domain vocabulary |
| 18 — RPGPT | Anti-cheat = access control (proven in exp065) |
| 20 — NFT Economics | Every medical record is a Novel Ferment Transcript |
| 21 — Sample Provenance | Shared fraud detection infrastructure |

---

## 9. Future Work

- **healthSpring adoption**: Direct integration with clinical tracks
- **Full ZK proofs**: BearDog ZK circuit for true zero-knowledge verification
- **FHIR/HL7 interop**: Map loamSpine certificates to FHIR resources
- **Cross-institution consent**: songbird discovery for multi-provider consent chains
- **Patient portal**: petalTongue visualization of consent status and access history
- **Clinical trial consent**: Specialized consent certificates for research participation
