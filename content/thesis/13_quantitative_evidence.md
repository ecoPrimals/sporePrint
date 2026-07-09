+++
title = "Chapter 13: Quantitative Evidence"
description = "Measurable constrained-evolution signatures: NTT-to-FFT kernel identity (~97%), convergent IPC, fastidious specialization."
weight = 13
date = 2026-07-09
+++

{{ maturity(level="planned") }}

## Quantitative Signatures

This chapter moves beyond biological analogy to measurable signatures of constrained evolution in the ecoPrimals codebase:

- **NTT→FFT structural evolution**: ~97% identical main compute kernels between `fhe_ntt.wgsl` and `fft_1d.wgsl` — a kernel designed for fully homomorphic encryption proved fit for scientific FFT without redesign
- **Convergent IPC**: {{ total_stat(stat="total_primals") }} independently evolved primals all converge on JSON-RPC 2.0, not by mandate but by type-system selection pressure
- **Fastidious specialization**: constraint-specific capabilities that unconstrained development would not produce
- **Cross-domain kernel reuse**: shaders validated in physics, agriculture, chemistry, and ML without modification

---

**See also:**

- [BarraCuda](@/thesis/06_barracuda.md) — where the kernels evolved
- [Constrained Evolution — Formal](@/methodology/CONSTRAINED_EVOLUTION_FORMAL.md) — the predictions these measurements test

---

*Full content transplant pending. Source: `whitePaper/gen3/thesis/13_quantitative_evidence.md`*
