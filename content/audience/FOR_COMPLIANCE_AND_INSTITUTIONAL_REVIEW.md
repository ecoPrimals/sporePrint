+++
title = "ecoPrimals — Compliance, Regulatory, and Institutional Review Reference"
description = "FDA/ISO/HIPAA/GDPR mapping for institutional review"
date = 2026-03-17

[taxonomies]
primals = ["beardog", "loamspine", "nestgate", "rhizocrypt", "sweetgrass", "toadstool"]
springs = ["groundspring", "healthspring", "wetspring"]
+++

# ecoPrimals — Compliance, Regulatory, and Institutional Review Reference

**From:** ecoPrimal — human + synthetic intelligence  
**Organization:** ecoPrimals
**Date:** March 17, 2026
**License:** All source code AGPL-3.0-or-later; all documentation CC-BY-SA-4.0
**Repositories:** github.com/ecoPrimals

---

## Purpose

This document addresses requirements from regulatory bodies, institutional review
boards, legal counsel, grant agencies, quality assurance auditors, and compliance
officers. It maps ecoPrimals capabilities to specific standards, identifies what
is implemented, what is architecturally ready but unaudited, and what is not yet
addressed.

This is not marketing. Where we have gaps, they are stated explicitly.

---

## 1. Software Safety and Determinism

### Language Safety Guarantees

| Property | Mechanism | Verification |
|----------|-----------|:------------:|
| No undefined behavior | Rust ownership + borrow checker | Compile-time enforced |
| No null pointer dereference | `Option<T>` type system | Compile-time enforced |
| No buffer overflow | Bounds checking + slices | Compile-time + runtime |
| No use-after-free | Ownership transfer semantics | Compile-time enforced |
| No data races | `Send`/`Sync` trait system | Compile-time enforced |
| No unsafe code | `#![forbid(unsafe_code)]` in all spring lib crates | Compile-time enforced |

**Relevance to IEC 62304**: Ferrocene (Rust compiler qualification) achieved
IEC 62304 Class C qualification in January 2025. Rust's compiler eliminates
~90% of traditional safety analysis requirements that apply to C/C++/Fortran
codebases.

### Deterministic Reproducibility

| Property | Mechanism | Verification |
|----------|-----------|:------------:|
| Same input → same output | No global mutable state, no random seeds without explicit parameters | All 27,000+ tests pass deterministically |
| Bit-exact GPU results | f64 WGSL shaders with explicit rounding | CPU↔GPU parity checks across all springs |
| No Jupyter state corruption | No notebooks — compiled binaries only | Structural guarantee |
| No Python version drift | No Python dependency in production code | `Cargo.lock` pins all dependencies |
| No conda/pip conflicts | Single `cargo build` command | Structural guarantee |

### Static Analysis

| Tool | Configuration | Result |
|------|--------------|--------|
| `cargo clippy` | `pedantic` + `nursery` lints enabled | Zero warnings across all springs |
| `cargo deny` | License allowlist, advisory DB, ban list | All dependencies pass |
| `cargo fmt` | Standard Rust formatting | All code formatted |
| `#[expect(reason)]` | Every suppressed lint has a documented reason | Auditable justifications |

---

## 2. FDA 21 CFR Part 11 — Electronic Records and Signatures

Part 11 requires that electronic records used in FDA-regulated activities
have controls for access, audit trails, and electronic signatures.

### Mapping to ecoPrimals Architecture

| Part 11 Requirement | § Reference | ecoPrimals Implementation | Status |
|---------------------|:----------:|--------------------------|:------:|
| Validation | §11.10(a) | 27,000+ automated tests; 15,334+ validation checks; 306 validation binaries (wetSpring alone) | **Implemented** |
| Audit trail (who, what, when) | §11.10(e) | rhizoCrypt DAG: every computation is a vertex with timestamp, operator DID, and input hash | **Implemented** (architecture); **Unaudited** |
| Record retention | §11.10(c) | NestGate content-addressed storage (BLAKE3 hash); ZFS checksummed cold storage | **Implemented** (architecture) |
| Access controls | §11.10(d) | loamSpine certificates with scoped permissions; BearDog Ed25519 identity | **Implemented** (architecture) |
| Electronic signatures | §11.50, §11.70 | BearDog Ed25519 signatures on all results; signature linked to individual DID | **Implemented** (architecture); **Unaudited** |
| Signature/record binding | §11.70 | Signature covers content hash + metadata; cannot be separated | **Implemented** |
| Authority checks | §11.10(g) | loamSpine certificate scoping; operator DID must match authorized personnel | **Implemented** (architecture) |
| Device checks | §11.10(h) | SoloKey FIDO2 hardware authentication for NUCLEUS nodes | **Implemented** (4 HSMs) |
| Open system controls | §11.30 | End-to-end encryption (chacha20poly1305); BearDog X25519 key agreement | **Implemented** |

### What Is NOT Addressed

| Requirement | Gap | Path Forward |
|-------------|-----|-------------|
| Formal Part 11 compliance audit | No auditor has reviewed the system | Requires institutional partner with QA |
| CTD/eCTD submission formatting | No regulatory submission output format | Formatting layer on top of existing data |
| Procedural controls (SOPs) | Technical controls only — no SOP templates | SOPs are lab-specific; framework supports them |
| Training documentation per §11.10(i) | No formal training records | K-Nome methodology documented; formal training pending |

---

## 3. ISO 17025:2017 — Testing and Calibration Laboratories

Paper 21 (Sovereign Sample Provenance) maps the provenance trio to ISO 17025.

### Clause-by-Clause Mapping

| ISO 17025 Clause | Requirement | ecoPrimals Mapping | Status |
|:----------------:|------------|-------------------|:------:|
| 4.1 | Impartiality | AGPL-3.0 source code; all algorithms publicly auditable | **Structural** |
| 5.3 | Facilities and environmental conditions | Not applicable (software, not physical lab) | N/A |
| 6.2 | Personnel competence | loamSpine certificates link operator DID to qualifications | **Architectural** |
| 6.4 | Equipment | toadStool hardware discovery; probe.rs inventories GPU/CPU/NPU capabilities | **Implemented** |
| 7.1 | Review of requests | Not applicable (computational pipeline, not service lab) | N/A |
| 7.2 | Method selection/validation | 27,000+ tests; 70+ published papers reproduced; 15,334+ validation checks | **Implemented** |
| 7.3 | Sampling / sample receipt | rhizoCrypt DAG: collection vertex with timestamp, GPS, operator, conditions | **Implemented** (Paper 21 exp062) |
| 7.4 | Sample identification | loamSpine certificate: unique sample ID, type, condition, accession | **Implemented** (Paper 21 exp062) |
| 7.5 | Technical records | Every computation produces a DAG vertex with input hashes, parameters, output hashes | **Implemented** |
| 7.6 | Measurement uncertainty | groundSpring: error propagation, uncertainty quantification, spectral methods (102 barraCuda delegations) | **Implemented** |
| 7.7.1 | Quality assurance / contamination | Fraud detector: `ContaminationGap` — flags sequential processing without QC step | **Implemented** (Paper 21 exp062) |
| 7.8 | Reporting | Validation binaries produce structured PASS/FAIL output with tolerances | **Implemented** |
| 7.11 | Data control | Immutable DAG vertices; BearDog Ed25519 signatures; content-addressed storage | **Implemented** |
| 8.5 | Actions to address risks | `RetryPolicy` + `CircuitBreaker` for IPC fault tolerance; `IpcError` classification | **Implemented** |
| 8.7 | Internal audit | `cargo test`, `cargo clippy`, `cargo deny` run on every change | **Implemented** (automated) |

### Fraud Detection (6 Types, ISO-Mapped)

| Fraud Type | Detection | ISO Clause |
|-----------|-----------|:----------:|
| PhantomSample | Analysis results with no collection vertex | 7.3 |
| DuplicateAccession | Two samples claim same accession | 7.4 |
| BrokenColdChain | Frozen → Fresh without documented reason | 15189:5.4.4 |
| UnauthorizedAccess | Processing by DID not in custody chain | 6.2 |
| MislabeledSpecimen | Cert metadata vs collection vertex mismatch | 15189:5.4.2 |
| ContaminationGap | Sequential processing without QC step | 7.7.1 |

---

## 4. ISO 15189:2022 — Medical Laboratories

Paper 22 (Zero-Knowledge Medical Provenance) extends the provenance model to
clinical laboratories with patient consent management.

### Additional Clauses Addressed

| ISO 15189 Clause | Requirement | ecoPrimals Mapping |
|:----------------:|------------|-------------------|
| 5.4.2 | Specimen labelling | loamSpine certificate metadata: patient DID, sample type, collection conditions |
| 5.4.4 | Transport and storage | rhizoCrypt DAG tracks custody transfers with timestamps and conditions |
| 5.7 | Post-examination | Result signed by BearDog; immutable in DAG; patient access via consent certificate |
| 6.5.2 | Information system security | BearDog Ed25519 + X25519 encryption; loamSpine access scoping |

---

## 5. HIPAA — Health Insurance Portability and Accountability Act

Paper 22 defines a consent-gated access model for patient-owned medical records.

### Privacy Rule (45 CFR §164)

| HIPAA Requirement | § Reference | ecoPrimals Mapping | Status |
|-------------------|:----------:|-------------------|:------:|
| Individual access rights | §164.524 | Patient owns record via loamSpine certificate; self-sovereign DID | **Architectural** |
| Minimum necessary | §164.502(b) | Consent certificate scopes access to specific record types | **Architectural** |
| Covered entity obligations | §164.502 | Provider DID identified in consent loan; access logged | **Architectural** |
| Right to revoke | §164.508(b)(6) | `revoke_consent()` is irreversible; future access blocked | **Architectural** |
| Consent validity | §164.508 | Consent certificate has expiry field; expired access is fraud | **Architectural** |

### Security Rule (45 CFR §164.312)

| Requirement | § Reference | ecoPrimals Mapping |
|-------------|:----------:|-------------------|
| Access control | §164.312(a)(1) | loamSpine certificate + consent scoping; BearDog identity |
| Audit controls | §164.312(b) | Every access is a DAG vertex; BearDog signs `AccessProof` |
| Integrity | §164.312(c)(1) | Content-addressed storage (BLAKE3); Ed25519 signatures |
| Transmission security | §164.312(e)(1) | chacha20poly1305 encryption; X25519 key agreement |

### HIPAA Fraud Detection (5 Types)

| Fraud Type | Detection | HIPAA Impact |
|-----------|-----------|:------------:|
| UnauthorizedAccess | Access with no valid consent at timestamp | §164.312(b) |
| ExpiredConsent | Access after consent expiry | §164.508(b)(6) |
| ScopeViolation | Access to record type not in consent | §164.502(b) |
| PhantomAccess | Record modified but no access vertex | §164.312(b) |
| ConsentForgery | Consent cert not signed by patient DID | §164.312(a)(1) |

---

## 6. GDPR — General Data Protection Regulation

sweetGrass (SCYBORG provenance trio) implements GDPR-inspired data subject rights.

| GDPR Right | Article | sweetGrass Implementation |
|-----------|:-------:|--------------------------|
| Right of access | Art. 15 | 5-level privacy; `Access` level allows subject to read all attributed data |
| Right to erasure | Art. 17 | `Erasure` level; DAG vertex marked as erased (hash retained for integrity) |
| Right to portability | Art. 20 | `Portability` level; PROV-O export of full provenance chain |
| Purpose limitation | Art. 5(1)(b) | loamSpine certificate scopes purpose; exceeding scope is fraud |
| Data minimization | Art. 5(1)(c) | Consent certificate specifies record types; minimum necessary |

---

## 7. IRB — Institutional Review Board

### Current State

ecoPrimals is a **computational platform**. All current experiments use:
- Published, peer-reviewed data (NCBI, PhysioNet, ChEMBL)
- Synthetic/simulated data (Monte Carlo, mathematical models)
- Publicly available datasets (repoDB, ROBOKOP, MIT-BIH)

**No human subjects data has been collected, generated, or processed.**

### When IRB Becomes Relevant

IRB review would be required when:
1. Processing real patient data (e.g., MIMIC-IV with PhysioNet credential)
2. Collecting biological samples (wet lab integration with Gonzales iPSC work)
3. Clinical validation studies (prospective trials)

healthSpring explicitly states: "Clinical validation requires prospective studies,
IRB approval, and institutional partnerships. healthSpring provides the
computational foundation; clinical validation is a separate, future phase."

### What ecoPrimals Provides to IRB Processes

| IRB Concern | ecoPrimals Response |
|------------|-------------------|
| Data security | BearDog encryption (chacha20poly1305) + Ed25519 signatures |
| Access control | loamSpine consent certificates; scoped, time-limited, revocable |
| Audit trail | rhizoCrypt DAG; every access logged as immutable vertex |
| De-identification | sweetGrass `AnonymizedPublic` privacy level; DID-based pseudonymization |
| Data retention/destruction | Content-addressed storage with erasure capability |
| Reproducibility | Deterministic computation; same input → same output |

---

## 8. Licensing and Intellectual Property

### License Structure (scyBorg Triple Copyleft)

| Domain | License | What It Covers |
|--------|---------|---------------|
| Source code | AGPL-3.0-or-later | All Rust code in all springs and primals |
| Game mechanics / IPC protocols | ORC (Open RPG Creative Foundation) | JSON-RPC methods, deploy graphs, game rules |
| Documentation / creative works | CC-BY-SA-4.0 | White papers, baseCamp documents, briefs |

### What AGPL-3.0 Means for Institutional Users

| Scenario | AGPL Requirement |
|----------|-----------------|
| Clone and use internally | No obligation beyond internal use |
| Modify and use internally | No obligation (no distribution) |
| Distribute modified binaries | Must provide source code under AGPL |
| Run as a network service | Must provide source code to users of the service |
| Use output/results | No license restriction on output data |
| Publish papers using results | No license restriction on publications |

**For a university lab**: You can clone, build, use, modify, and publish papers
using ecoPrimals results with zero licensing obligation, as long as you don't
distribute modified binaries or run a public service. Internal use within a
university is explicitly permitted.

### Symbiotic Exception Protocol

The scyBorg exception protocol (AGPL §7 additional permissions) allows named
organizations to receive broader permissions in exchange for reciprocal benefit.
Exceptions are not for sale — they are granted based on symbiotic value.

---

## 9. Dependency Audit

### Production Dependencies (wetSpring barracuda crate)

| Dependency | License | Purpose | C Code? |
|-----------|---------|---------|:-------:|
| `barracuda` (barraCuda) | AGPL-3.0 | GPU math primitives | No |
| `serde` | MIT/Apache-2.0 | Serialization | No |
| `serde_json` | MIT/Apache-2.0 | JSON parsing | No |
| `wgpu` (optional) | MIT/Apache-2.0 | WebGPU runtime | No (Rust) |
| `tracing` | MIT | Structured logging | No |
| `bytemuck` | MIT/Apache-2.0/Zlib | Safe byte casting | No |
| `flate2` | MIT/Apache-2.0 | Gzip decompression | No (`rust_backend` feature) |
| `chacha20poly1305` (optional) | MIT/Apache-2.0 | AEAD encryption | No |
| `ed25519-dalek` (optional) | BSD-3 | Ed25519 signatures | No |
| `blake3` (optional) | MIT/Apache-2.0 | Cryptographic hashing | No (`pure` feature) |

**Zero C/C++/Fortran in the application dependency chain.** The `flate2` crate
uses `rust_backend` (miniz_oxide, pure Rust). `blake3` uses `pure` feature
(no assembly, no C). `wgpu` uses Rust for all API translation.

### Audit Tools

```bash
cargo deny check         # License allowlist, advisory DB, ban list
cargo audit              # Known vulnerability scan
cargo tree               # Full dependency tree
```

---

## 10. Validation Evidence Summary

| Metric | Value | How to Verify |
|--------|-------|:-------------:|
| Total automated tests | 27,000+ across 7 springs | `cargo test --workspace` in each spring |
| Validation checks (numerical) | 15,334+ with explicit tolerances | `cargo run --release --bin validate_*` |
| Papers reproduced | 70+ across physics, biology, pharmacology, chemistry | Each paper has dedicated experiment(s) |
| Validation binaries | 306 (wetSpring) + others per spring | `ls barracuda/src/bin/validate_*.rs` |
| Clippy warnings | 0 (pedantic + nursery) | `cargo clippy --all-targets -- -D warnings` |
| Unsafe code blocks | 0 | `#![forbid(unsafe_code)]` in lib.rs |
| TODO/FIXME in production | 0 | `grep -r "TODO\|FIXME" src/ --include="*.rs"` |
| Mocks in production code | 0 | All mocks isolated to `#[cfg(test)]` |
| External C dependencies | 0 | `cargo tree` shows no C/C++ crates |

---

## Document History

| Date | Change |
|------|--------|
| 2026-03-17 | Initial compliance and institutional review reference |
