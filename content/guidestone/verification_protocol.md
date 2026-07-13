+++
title = "Verification Protocol"
description = "The five properties that make computation self-proving — deterministic output, reference traceability, self-verification, environment agnosticism, and tolerance documentation."
date = 2026-04-30
weight = 1

[taxonomies]
primals = ["barracuda", "beardog", "coralreef", "toadstool"]
springs = ["hotspring", "groundspring"]
+++

## What Makes Computation Self-Proving

A {{ entity(name="guidestone") }}-certified artifact does not ask the consumer to
trust its origin, its transfer channel, or its execution environment. The
output carries its own proof. This is not a guarantee of correctness in the
general case — it is a guarantee that the computation is **reproducible,
traceable, and verifiable** by anyone with the binary and commodity hardware.

The protocol defines five necessary properties. Together they are sufficient.

---

## Property 1: Deterministic Output

Same input, same binary, any hardware — same output within named tolerances.

This is harder than it sounds. Floating-point arithmetic is
order-dependent. GPU thread scheduling is nondeterministic. SIMD widths
vary across architectures. A naive implementation of `sum(array)` can
produce different results on x86_64 vs aarch64 due to FMA contraction.

{{ entity(name="guidestone") }} determinism requires:

- **Canonical reduction order** — reductions use a fixed tree structure,
  not hardware-dependent scheduling
- **Explicit FMA policy** — {{ entity(name="coralreef") }} emits FMA instructions with
  documented contraction semantics per shader
- **Named tolerances** — every comparison carries an epsilon with a
  derivation. `assert_relative!(result, 0.593, tol=1e-3, source="Creutz 1983 Table 2")`

The tolerance is not an excuse for slop. It is a **metrological statement**
about the precision achievable on the target substrate.

---

## Property 2: Reference-Traceable

Every numeric claim traces to a source.

Sources fall into four categories:

| Source Type | Example | How It's Cited |
|------------|---------|----------------|
| Published paper | Bazavov & Chuna, arXiv:2101.05320 | DOI or arXiv ID in tolerance metadata |
| Mathematical proof | Creutz plaquette formula (SU(3)) | Derivation in test documentation |
| Physical constant | Planck constant, Boltzmann constant | CODATA 2018 values with stated uncertainty |
| Standard | FAO-56, DADA2, MILC gauge configs | Standard identifier + version |

A number that cannot be traced to one of these categories is not
{{ entity(name="guidestone") }}-eligible. "Empirically determined" is acceptable
only when the empirical procedure is documented and the determination is
reproducible from the described protocol.

{{ entity(name="groundspring") }} enforces this systematically — every tolerance
in every spring has a named source and a decomposition showing which
measurement uncertainty dominates.

---

## Property 3: Self-Verifying

The artifact carries its own integrity.

Three layers of verification, each independent:

**Layer 1: Binary integrity** — SHA-256 checksums in a `CHECKSUMS` manifest.
The consumer verifies the binary has not been modified in transit before
executing it. This is defense against corruption, not against a
sophisticated adversary (that requires {{ entity(name="beardog") }} signatures,
which {{ entity(name="guidestone") }} does not mandate but supports).

**Layer 2: Data payload integrity** — CRC-32 on embedded reference data.
If the binary carries calibration data, standard tables, or reference
configurations, each payload is checksummed. A bit flip in transit
produces a clear error, not a silent wrong answer.

**Layer 3: Provenance integrity** — when the {{ entity(name="provenancetrio") }}
is wired, Merkle roots provide cryptographic proof that the computational
lineage is intact. This is the strongest form: not just "the binary is
correct" but "the entire chain from source data to output is auditable."

---

## Property 4: Environment-Agnostic

No hardcoded paths. No "install X first." No platform assumptions.

Concretely:

- **Static linking** — musl libc, no dynamic library dependencies
- **Runtime discovery** — CPU features via `cpuid`, GPU adapters via
  sysfs/Vulkan enumeration, NPU via VFIO probe
- **Graceful degradation** — if no GPU is found, the computation runs on
  CPU. If no NPU is found, the neuromorphic path is skipped. The physics
  results are identical; only wall time changes
- **Cross-architecture** — x86_64 and aarch64 binaries from the same source.
  The Rust type system + {{ entity(name="coralreef") }} shader compiler guarantee
  identical semantics across ISAs

The test: a PI receives a USB drive, inserts it into a machine they have
never configured for scientific computing, runs `./validate`, and gets
physics results. If they need to install anything first, the artifact is
not {{ entity(name="guidestone") }}.

---

## Property 5: Tolerance-Documented

No magic numbers. Every threshold has a derivation.

A {{ entity(name="guidestone") }} artifact does not contain lines like
`if abs(result - expected) < 1e-6`. It contains:

```
tolerance: 1e-6
source: "Creutz 1983, Table 2, N_t=8 plaquette"
derivation: "finite-size scaling at beta=6.0, L=8: O(1/V) correction < 1e-7"
dominant_uncertainty: "gauge sampling variance (jackknife, N_block=50)"
```

The derivation is not a comment in the source code. It is part of the
output metadata. A reviewer can inspect the artifact's tolerance chain
without reading the implementation.

{{ entity(name="groundspring") }} provides the uncertainty decomposition
framework. Every spring that targets {{ entity(name="guidestone") }}
certification uses {{ entity(name="groundspring") }} to identify which
measurement uncertainty dominates and derive appropriate thresholds.

---

## Applying the Protocol

Not every binary needs {{ entity(name="guidestone") }} certification. Exploratory
computation with heuristic thresholds, visualization tools, and interactive
interfaces are valuable without it. The protocol applies when the output
is a **claim** — when someone will cite the number, build on the result,
or make a decision based on it.

The protocol is also not binary. An artifact can satisfy properties 1–4
without property 5 (all tolerances are empirical, not derived). This is
still useful, still reproducible, still environment-agnostic — but not
fully {{ entity(name="guidestone") }}. The five properties define the ceiling.
Most useful computation lives between "no verification" and "full
{{ entity(name="guidestone") }}."
