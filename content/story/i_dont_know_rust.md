+++
title = "I Don't Know Rust"
description = "14 services. 27,000 tests. 70 papers reproduced. Built through conversation by someone who can't read the language."
date = 2026-07-08
weight = 10

[taxonomies]
primals = []
springs = []
+++

*14 services. 27,000 tests. 70 papers reproduced. Built through conversation by someone who can't read the language.*

---

## The Constraint

I chose Rust because I don't know it. That's not a paradox — it's the
design.

My background is bench microbiology (BSL2 fermentation, 16S pipelines,
spore trapping) and data science. My programming history is Python, R,
SQL, and some Visual Basic for instrument control. Nothing systems-level.
I cannot read Rust syntax fluently. I cannot debug a borrow checker error
by hand. I have never opened a Rust file and edited it directly.

Every line of code in this ecosystem was produced by AI, directed through
conversation, evaluated by me for scientific and architectural correctness.

The unfamiliarity is load-bearing. It forces every interaction through
conversation. I cannot reach into the code and tweak a variable — I have
to describe what I want and evaluate what comes back. The compiler catches
the mechanical errors. The test suite catches the logical errors. I catch
the conceptual errors. The constraint prevents shortcutting.

---

## The Process

A development session works like this:

1. I describe a goal in domain terms: "We need a security validator that
   runs pen tests against all 13 services, fuzzes their protocols, and
   checks cryptographic strength."
2. The AI proposes an architecture.
3. I evaluate the architecture against domain knowledge — not Rust
   knowledge, but knowledge of what security validation means, what threat
   models look like, what a rigorous check proves.
4. I redirect where the architecture is wrong or incomplete.
5. The AI implements.
6. The compiler rejects what is mechanically unsound.
7. The tests reject what is logically wrong.
8. I reject what is conceptually wrong.
9. Repeat.

The human contribution is direction, domain knowledge, quality judgment,
and taste. The AI contribution is implementation, pattern recognition,
and generation. The compiler contribution is environmental constraint.

This is reproducible. Anyone with domain knowledge in any field could
follow the same process. The methodology does not depend on this specific
person or this specific AI. It depends on the separation of concerns:
the human evaluates intent, the AI generates implementation, the compiler
enforces mechanical correctness.

---

## What It Produced

### 14 services ("primals")

Single static Rust binaries. No runtime dependencies. Communicate over
BTSP (ChaCha20-Poly1305 AEAD) or JSON-RPC over TCP. Each handles one
domain:

| Service | Port | Domain |
|---------|------|--------|
| bearDog | 9100 | Identity, crypto, Ed25519 keys, BTSP |
| songBird | 9200 | Discovery, networking, NAT traversal |
| squirrel | 9300 | AI coordination (Ollama backend) |
| toadStool | 9400 | Workload dispatch, GPU math |
| nestGate | 9500 | Content-addressed storage, KV |
| rhizoCrypt | 9601 | DAG tracking, Merkle roots |
| loamSpine | 9700 | Permanent ledger, certificates |
| coralReef | 9730 | WGSL shader compilation (Vulkan f64) |
| barraCuda | 9740 | Scientific math library (f64 GPU compute) |
| sweetGrass | 9850 | Attribution braids, W3C PROV |
| petalTongue | 9860 | Web serving, dashboards |
| skunkBat | 9870 | Anomaly detection, audit log |
| biomeOS | 9900 | Orchestration, Neural API |
| primalSpring | — | Master test coordination |

### 8 scientific validation frameworks ("springs")

Each reproduces published science in a specific domain:

| Spring | Domain | Tests | Checks | Papers |
|--------|--------|------:|-------:|-------:|
| wetSpring | Microbiology, metagenomics | 1,902 | 5,700+ | 52 |
| hotSpring | Plasma physics, lattice QCD | 990 | 697+ | 25 |
| airSpring | Agricultural hydrology | — | 2,631+ | 57 |
| groundSpring | Measurement, uncertainty | — | 236+ | 21 |
| neuralSpring | ML, surrogates, NPU | — | 3,200+ | 25+ |
| healthSpring | Pharmacometrics, clinical | 940 | 940+ | 9 |
| ludoSpring | Game science, HCI | 791 | 1,692+ | 15 |
| primalSpring | Integration, deploy | 404 | — | — |

Total: **27,000+ tests**. **12,510+ quantitative checks**. **70+ published
papers reproduced**.

### Security validation (darkforest v2.0)

Pure Rust binary. 939 KB. 181 checks:

- Pen tests: 3 threat actors (external, authorized, restricted) against
  all 13 services
- Protocol fuzzing: malformed inputs for every JSON-RPC and BTSP endpoint
- Crypto strength: 13 checks (cookie entropy, shadow hashing, token
  tamper resistance, cipher negotiation, file permissions)

Latest run: **175 PASS, 0 FAIL, 6 DARK_FOREST** (informational).

### Provenance pipeline

Every computation is:
- Content-addressed (BLAKE3)
- DAG-tracked (rhizoCrypt, Merkle structure)
- Ledger-committed (loamSpine, Ed25519-signed, append-only)
- Attributed (sweetGrass braids, W3C PROV)

Any result can be traced backward through the full chain of inputs,
computations, and contributors.

### Multi-user compute platform (projectNUCLEUS)

JupyterHub serving an external bioinformatics research group:
- 4-tier access (admin / compute / reviewer / observer)
- Every restriction mechanism-enforced (filesystem, iptables, kernel
  blocking, hidepid, ACLs)
- Workspace scaffolding (commons, pilot experiments, validation,
  showcase dashboards, per-user scratch)
- Cloudflare tunnel for external access

---

## The Numbers

| Metric | Value |
|--------|-------|
| Services | 14 |
| Springs | 8 |
| Tests | 27,000+ |
| Quantitative checks | 12,510+ |
| Published papers reproduced | 70+ |
| Security checks | 181 (175 PASS) |
| Hardware investment | ~$15,000 |
| Development time | ~2 years |
| Languages the developer knows | Not Rust |

---

## Reproducibility of the Method

The process is not proprietary. It requires:

1. Domain knowledge in any field (science, engineering, law, medicine)
2. An AI capable of generating code from natural-language description
3. A language with a strict compiler (Rust is ideal; others work)
4. A willingness to stay in conversation rather than editing code directly

The constraint — not knowing the language — is the feature. It prevents
the human from bypassing the conversation. Every architectural decision
must be articulated in domain terms, evaluated for intent, and generated
through the AI. The compiler enforces what the human cannot verify
syntactically. The test suite enforces what the compiler cannot verify
logically.

The numbers are silent about their own meaning — but every one has a test, and every test has a source paper, and every paper has a published result.

The repos are public. The tests are runnable. The science is verifiable.

---

## Verify

- **Deployment**: github.com/sporeGarden/projectNUCLEUS
- **All services**: github.com/ecoPrimals
- **Scientific domains**: github.com/sporeGarden/foundation
- **Live system**: lab.primals.eco
- **License**: AGPL-3.0-or-later
