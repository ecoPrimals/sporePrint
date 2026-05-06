+++
title = "🧫 Lab"
description = "Live validation results from projectNUCLEUS on ironGate. 13 primals, 235+ science checks, full provenance chains. Run it yourself or review the evidence."
sort_by = "weight"
template = "section.html"
+++

The lab is where {{ entity(name="ecoprimals") }} science gets validated on real hardware.
Everything here ran on **ironGate** (i9-14900K, 96 GB DDR5, RTX 5070) through
a live 13-primal {{ entity(name="nucleus") }} composition. Every result carries a
cryptographic provenance chain: BLAKE3 content hashes → rhizoCrypt DAG → loamSpine
ledger → sweetGrass ed25519-witnessed braid.

---

## What's Here

**Validation results** — 235+ structured science checks across 8 workloads, dispatched
through {{ entity(name="toadstool") }} on a live composition. Real NCBI data (11.9M
paired-end reads) processed through both Python and Rust pipelines. Python→Rust parity
at machine-epsilon precision.

**Provenance evidence** — every artifact content-addressed, every pipeline step tracked
in a DAG session, committed to a permanent ledger, and witnessed with ed25519 signatures.
The braid is PROV-O compliant with DID attribution.

**Reproduce it yourself** — step-by-step instructions to stand up the same composition
on your own hardware and run the same workloads. No cloud. No institutional access.
Commodity hardware.

---

## The Validation Pattern

```
Published results (papers, databases, NCBI)
        ↓
Python / established tools (QIIME2, SciPy, R vegan/phyloseq)
        ↓
Rust implementation (wetSpring, barraCuda)
        ↓
NUCLEUS composition dispatch (toadStool execute)
        ↓
Provenance chain (BLAKE3 → DAG → ledger → braid)
        ↓
Parity check + gap report
```

Each arrow is independently verifiable. The Rust matches the Python.
The composition matches standalone Rust. Gaps are documented and flow
upstream. Every successful workload is proof that the deploy graphs,
BTSP encryption, discovery hierarchy, and provenance pipeline work
in production.

---

## For ABG Members

If you're in the Accelerated Bioinformatics Group, the lab is also your
on-ramp. The same pipelines that produced these results are available
through JupyterHub on ironGate via the compute sharing tunnel. Your
workloads run on the same composition, with the same provenance. Your
science validates the infrastructure; the infrastructure validates your
science.

See [Reproduce It Yourself](@/lab/reproduce.md) for how to run this
on your own hardware, or contact ecoPrimal for tunnel access to the
live system.
