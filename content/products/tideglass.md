+++
title = "tideGlass — Sovereign GPS Platform"
description = "Sovereign GPS data analysis platform — reproducing published GPS methodology in pure Rust. Phase 0: reproduce GPS paper figures vs Python baseline. Second protoKarya protist."
date = 2026-07-15

[taxonomies]
primals = ["beardog", "biomeos", "cellmembrane", "nestgate", "rhizocrypt", "songbird", "sweetgrass", "barracuda", "coralreef"]
springs = ["wetspring", "hotspring", "neuralspring", "groundspring"]
trails = ["nf-pipeline"]

[extra]
maturity = "architectural"

[[extra.companions]]
url = "/products/nf-case-study/"
title = "NF Case Study"
relation = "pairs_with"
label = "First multi-product composition this enables"

[[extra.companions]]
url = "/products/footprint/"
title = "footPrint"
relation = "pairs_with"
label = "Sister protist — GIS home planner sharing composition patterns"

[[extra.companions]]
url = "/architecture/sovereign-hpc-evolution/"
title = "Sovereign HPC Evolution"
relation = "architecture"
label = "The hardware composition model tideGlass embodies"

[[extra.companions]]
url = "/collaborators/gonzales-nf/"
title = "Gonzales — NF Data Mining"
relation = "pairs_with"
label = "GPS platform rebuild as first tideGlass deliverable"
+++

{{ maturity(level="architectural") }} Phase 0: GPS paper reproduction in progress. Sovereign pallet hardware designed.

---

## What It Is

tideGlass has two identities that share one architecture:

1. **Sovereign GPS Platform** (Phase 0, active) — reproducing published GPS
   methodology in pure Rust, validated against Python baselines. This is the
   immediate deliverable: a self-hosted GPS data analysis tool that replaces
   commercial platforms with sovereign computation.

2. **Sovereign Pallet** (future) — a self-sustaining deployable unit providing
   power, compute, connectivity, and sovereign identity storage for field science
   and humanitarian infrastructure.

The GPS platform is the first composition that runs on the pallet. Build the
software first, then deploy it to sovereign hardware.

---

## GPS Platform — Phase 0

The immediate focus: reproduce GPS paper figures from the Gonzales NF data
mining collaboration, validated against the Python baseline. This proves
that sovereign Rust can replace commercial GPS analysis tools.

### Validation Modules (per guideStone spec)

Seven validation modules, each reproducing a specific GPS analysis capability:

| Module | What it validates | Status |
|--------|------------------|--------|
| Coordinate transforms | WGS84 ↔ UTM ↔ local frames | Planned |
| Signal processing | L1/L2 carrier phase, pseudorange | Planned |
| Positioning engine | Least-squares + Kalman filter | Planned |
| Atmospheric correction | Troposphere/ionosphere models | Planned |
| Time series analysis | Station velocity, seasonal signals | Planned |
| Network adjustment | Multi-station baseline resolution | Planned |
| Visualization | Displacement maps, time series plots | Planned |

### Drawbridge Bonds

tideGlass consumes external data via {{ entity(name="songbird") }} drawbridge:

| Source | Data | Registration |
|--------|------|-------------|
| LINCS L1000 | Gene expression profiles | Planned |
| GEO | Genomics datasets | Planned |
| ChEMBL | Bioactivity data | Planned |
| NF Data Portal | Neurofibromatosis datasets | Planned |

### Composition Evolution

| Step | Owner | Status |
|------|-------|--------|
| Clone repo into `protists/tideGlass` | overwatch | Planned |
| Phase 0: reproduce GPS paper figures | tideGlass team | Planned |
| Caddy block at `tideglass.primals.eco` | cellMembrane team | Planned |
| Drawbridge bond registration | songBird team | Planned |
| {{ entity(name="lithospore") }} packaging | lithoSpore team | Planned |
| `tideglass-composition-routing` scenario | {{ entity(name="primalspring") }} | Missing |

---

## Sovereign Pallet — Future Hardware

The GPS platform is software. The sovereign pallet is the hardware it
runs on when deployed to the field.

---

## Two Use Cases, One Architecture

### 1. Field Science — The Mobile Wet Lab's Static Base

Study caves, forests, remote watersheds, microbial ecology in places where there is no grid, no WiFi, no cell tower. The pallet is the base station:

- Run {{ entity(name="primalspring") }} validation suites on sensor data collected in the field
- Aggregate environmental telemetry (temperature, humidity, soil pH, water conductivity, air quality)
- Store and verify results with {{ entity(name="guidestone") }} provenance ({{ entity(name="nestgate") }} ledger, {{ entity(name="rhizocrypt") }} signatures)
- Mesh-relay data to satellite/civilization when available
- Power field instruments (microscopes, spectrometers, sensors)
- Hot water for sample processing, sterilization, or just coffee at base camp

**Where you go with it:**
- Cave systems (karst hydrology, microbial mats, mineral formation)
- Old-growth forests (soil microbiome, fungal networks, canopy air quality)
- Watersheds (PFAS monitoring, turbidity, dissolved oxygen time-series)
- Glacier margins (meltwater chemistry, microbial succession)
- Desert research (soil crust biology, thermal extremes, water harvesting)

Each pallet is a sovereign research station. The science it produces is self-proving — {{ entity(name="guidestone") }} verified, provenance-chained, reproducible.

### 2. Humanitarian — Preparing the Rooms

**Physical services (immediate, no credentials required):**
- Hot water (hygiene, cooking, warmth) — compute waste heat, always available
- Phone charging (6+ USB ports, 24/7) — phones are housing applications, benefits, jobs, family
- WiFi (mesh AP, no subscription, no data harvesting) — internet access without surveillance
- Heat (winter survival) — sand battery discharges through the night
- Light (12V LED, after dark) — safety, dignity

**Digital sovereignty:**

Identity is not stored on a paper card. It is not stored in a government database the person cannot access. It is **encrypted to their biometrics and stored in the sovereign mesh**.

| Current Problem | Sovereign Pallet Solution |
|----------------|--------------------------|
| Paper cards get lost/stolen/swept | Biometric key — the person IS the credential |
| Restarting every application after a sweep | Data replicates across mesh — survives single-point destruction |
| No identity verification without government ID | Biometric presents at any pallet — verified without cards |
| Surveillance risk (centralized databases) | No central database. Encrypted blobs at rest. Only biometric holder can decrypt. |
| Phone stolen = all contacts/progress lost | Critical data syncs to mesh — phone loss does not restart the process |
| No proof of service history | Interaction log is append-only, cryptographically signed, owned by the person |

---

## Biometric Identity Model

### No Honeypot, No Vulnerability

Traditional systems store identity data in a database. Database equals target. Breach the database, get everyone's records. The sovereign model inverts this:

```
TRADITIONAL:                     SOVEREIGN:

Person -> ID card -> Database    Person -> Biometric -> Encrypted blob
          (losable)  (breachable)          (IS the person)  (useless without person)

Database has:                    Pallet has:
  - Name (plaintext)               - Encrypted blob (ciphertext)
  - SSN (plaintext)                - Hash of biometric (not the biometric)
  - Address (plaintext)            - Nothing else
  - Everything (plaintext)
                                 Person has:
Breach -> everything exposed       - Their fingerprint (always with them)
                                   - Their voice (always with them)
                                   - Their palm vein pattern (always with them)

                                 Seize pallet -> encrypted garbage
                                 Breach mesh -> encrypted garbage everywhere
```

### {{ entity(name="beardog") }} Integration

{{ entity(name="beardog") }}'s BTSP (Biometric Trust Seed Protocol) provides the cryptographic primitives:
- `auth.enroll` — biometric to key derivation
- `auth.verify` — biometric to decrypt + validate
- `trust.issue` — case worker attestation (signs that enrollment happened in person)
- `trust.revoke` — person can revoke their own data (right to be forgotten, always)

The primal already exists. The sovereign pallet is its deployment surface for populations without stable infrastructure.

---

## Technical Architecture

### Hardware Tiers

| Tier | Use Case | Compute | Storage | Solar | Battery | Thermal | Cost |
|------|----------|---------|---------|-------|---------|---------|------|
| **Micro** | Charging + WiFi only | ESP32/Pi Zero | 32GB SD | 50W | 0.5 kWh | None | $200-350 |
| **Standard** | Full services + identity | Pi 5 / NUC | 1TB NVMe | 200W | 2 kWh | 25kg sand | $700-1200 |
| **Science** | Field research + GPU | Jetson Orin / NUC | 4TB + HDD | 400W | 5 kWh | 50kg sand | $2000-3500 |
| **Cluster** | Multi-pallet mesh | 3x Standard | Distributed | 600W | 6 kWh | 75kg sand | $2500-4000 |

### Software Stack (NUCLEUS at Pallet Scale)

```
+----------------------------------------------------+
|  NUCLEUS (13 primals, scaled to hardware)           |
|                                                     |
|  IDENTITY + TRUST                                   |
|    bearDog  -- BTSP biometric enrollment            |
|    nestGate -- provenance ledger                    |
|    rhizoCrypt -- encrypted blob store               |
|    sweetGrass -- attribution chains                 |
|                                                     |
|  MESH + TRANSPORT                                   |
|    songBird -- relay + mesh networking              |
|    cellMembrane -- topology management              |
|    skunkBat -- discovery + gossip                   |
|                                                     |
|  COMPUTE + SCIENCE (if hardware allows)             |
|    barraCuda -- GPU/tensor ops                      |
|    coralReef -- shader compilation                  |
|    toadStool -- workload dispatch                   |
|    squirrel -- AI/inference                         |
|                                                     |
|  APPLICATION                                        |
|    biomeOS -- composition orchestration             |
|    petalTongue -- visualization/UI                  |
|    loamSpine -- data pipeline                       |
+----------------------------------------------------+
```

Not all primals run on all tiers. A Micro pallet runs {{ entity(name="beardog") }} + {{ entity(name="songbird") }} + {{ entity(name="cellmembrane") }} only (identity + mesh + topology). A Science pallet runs all 13. The composition model handles this — same architecture, different deployment density.

### Mesh Topology (Multi-Pallet)

Pallets at different locations maintain mesh connectivity via {{ entity(name="songbird") }}. A person enrolled at Pallet A can retrieve their data at Pallet B or C. Encrypted blobs replicate across all pallets in the mesh. If a pallet is destroyed, the data survives on every other node.

---

## {{ entity(name="lithospore") }} + Sovereign Pallet = Complete Field Station

{{ entity(name="lithospore") }} (gen4 product): a USB drive that boots a complete, validated computational environment anywhere. It gives you software sovereignty — the ability to run science on any hardware.

Sovereign Pallet (gen5 deployment): the hardware that RUNS {{ entity(name="lithospore") }} where there is no hardware. Together:

| {{ entity(name="lithospore") }} provides | Sovereign Pallet provides |
|---------------------|--------------------------|
| Software environment | Hardware to run it on |
| Validation framework | Power (solar) |
| Reproducible pipelines | Connectivity (mesh) |
| {{ entity(name="guidestone") }} verification | Storage (NVMe + mesh replication) |
| Domain portability | Physical portability |

**Together**: a complete, self-proving, self-powered research station that fits on a pallet, runs on sunlight, and produces science indistinguishable from a university lab in terms of verification quality — deployable in a cave entrance, a forest clearing, or a glacial moraine.

---

## Economics

### Humanitarian Pallet (Standard Tier)

| Component | Cost | Lifespan | Annual Cost |
|-----------|------|----------|-------------|
| Compute (Pi 5 + NVMe) | $140 | 7+ years | $20 |
| Solar (200W fold-flat) | $150 | 25 years | $6 |
| Battery (2 kWh LFP) | $400 | 10+ years | $40 |
| Sand battery + thermal | $80 | Infinite | $0 |
| Enclosure (weatherproof) | $100 | 15+ years | $7 |
| Wiring, connectors, sensors | $80 | 10+ years | $8 |
| Biometric sensor (fingerprint) | $50 | 10+ years | $5 |
| WiFi + mesh radio | $40 | 7+ years | $6 |
| **TOTAL** | **$1,040** | | **$92/year** |

**$92/year** to provide hot water, charging, WiFi, identity continuity, and dignity. That is **$7.67/month** — less than a streaming subscription, for an entire community.

### Science Pallet (Science Tier)

| Component | Cost | Annual Cost |
|-----------|------|-------------|
| Compute (NUC + Jetson Orin Nano) | $600 | $60 |
| Solar (400W, foldable) | $300 | $12 |
| Battery (5 kWh LFP) | $800 | $80 |
| Sand + thermal (50kg) | $120 | $0 |
| Enclosure (Pelican-grade) | $250 | $17 |
| Sensors + instruments | $300 | $30 |
| Storage (4TB + backup) | $200 | $20 |
| Satellite modem (when needed) | $300 + $50/mo | $600 |
| **TOTAL** | **$2,870** | **$819/year** |

A university field station costs $50,000-500,000 to build and $10,000-50,000/year to maintain. The sovereign pallet costs less than a semester of lab fees.

---

## The Moral Architecture

| Design Choice | Technical Reason | Ethical Reason |
|--------------|-----------------|----------------|
| Biometric-only (no passwords, no cards) | Cannot be lost, stolen, or swept | The person IS the credential. Dignity is structural. |
| Encrypted at rest (always) | Security best practice | No honeypot. Cannot harm people by being breached. |
| Person-controlled decryption | Key management simplicity | Sovereignty. Your data, your key, your decision. |
| Mesh replication (multi-pallet) | Redundancy | Survives sweeps, theft, destruction of any one node. |
| Solar-powered (no grid) | Deployment flexibility | Can exist where infrastructure does not serve people. |
| No cloud dependency | Reliability, cost | No subscription means no shutdown. No vendor means no capture. |
| {{ entity(name="scyborg") }} licensed (open) | Community builds, modifies, improves | Cannot be captured by a corporation or gatekept by a nonprofit. |
| Append-only history | Data integrity | "The system lost my paperwork" becomes impossible. |
| Right to deletion | Control | Person can destroy their data at any time. Their choice. |

---

## Deployment Plan

**Phase 0**: Single pallet at service location. Charging + WiFi + hot water. No identity yet.
**Phase 1**: {{ entity(name="beardog") }} enrollment pilot. Voluntary. Opt-in only. Small cohort.
**Phase 2**: Second pallet at satellite location. Test mesh replication + continuity.
**Phase 3**: Field deployment. Test durability + weatherproofing + autonomous operation.

---

## Open Questions

1. **Biometric modality** — fingerprint is cheapest ($50 sensor) but excludes people with damaged hands. Palm vein is more inclusive but expensive ($200+). Voiceprint is free (microphone) but less reliable outdoors. Multi-modal enrollment (any 2 of 3)?
2. **Revocation** — if biometrics change (amputation, severe burn), how does the person regain access? Trusted recovery via case worker attestation ({{ entity(name="beardog") }} trust chain)?
3. **Consent and coercion** — biometric enrollment at a service provider has coercion risk. Design must make enrollment genuinely optional, with full services available regardless.
4. **Legal frameworks** — BIPA (Illinois), GDPR, state biometric laws. Architecture is privacy-preserving by design, but regulatory analysis needed.
5. **Weatherproofing** — Michigan winters (-20C). Sand battery + insulation keeps compute above freezing? Or enclosure heater needed?

---

## Lineage

```
gen1: Can we build compute?         (yes -- $11K cluster)
gen2: Should we?                    (yes -- sovereign protocol, AGPL covenant)
gen3: Does it work?                 (yes -- 12,510 tests, 70 papers, thesis)
gen4: Who uses it?                  (creatives, scientists, sovereign builders)
gen5: Does someone else's science come out?    (in progress)
  +-- SOVEREIGN PALLET: Does it serve the person on the road?
      Does it work without a university, without a grid, without an address?
      Does it preserve dignity when the system fails?
      Does it produce science in places science has never been?
```

---

*The pallet is not a product in the commercial sense. It is a room prepared. Open designs ({{ entity(name="scyborg") }}), open hardware, buildable by any community. The architecture itself is the inn — ready when needed, powered by sunlight, staffed by sovereign computation.*
