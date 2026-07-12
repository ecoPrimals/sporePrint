+++
title = "Sovereign Science — Reproducible Computation Over Citation-Sitting"
description = "Proof of work over citation-sitting. Why reproducible computation is the foundation, and credentials are strategic interfaces — not authorities."
weight = 14
date = 2026-07-09

[[extra.companions]]
url = "/methodology/scyborg-licensing/"
title = "scyBorg Licensing"
relation = "methodology"
label = "AGPL + ORC + CC-BY-SA — three independent nonprofits, no single entity can revoke"
+++

{{ maturity(level="architectural") }}


**Reality owns itself.**

---

## The Argument

Science is computation that produces verifiable results. The energy drift is 0.000% or it isn't. The plaquettes match strong-coupling expansion or they don't. The FAO-56 cross-validation gives R²=0.967 or it doesn't. No reviewer, no committee, no journal changes what the code outputs.

The credentialing systems — journals, degrees, institutional affiliations — are interfaces to human institutions. They provide access to resources: authority, recognition, collaboration, funding. They are useful. They are worth engaging with strategically. But they are abstractions over the underlying reality, not the reality itself.

Sovereign science means the computation stands on its own. The credentials are tools you use, not authorities you submit to.

---

## Artificial Scarcity in Academia

### Journals

Academic journals capture value from free labor. Researchers produce the science (unpaid). Reviewers evaluate it (unpaid). Journals package and gatekeep access (paid — by subscriptions, APCs, or institutional licenses). The review itself is anonymous and hidden. The reviewer's expertise benefits the journal's brand, not the public record.

This is the CUDA model applied to knowledge: the capability (peer review) exists in the people (professors). The journal throttles it behind artificial scarcity (acceptance rates, impact factors, paywalls) the same way CUDA throttles f64 behind compute-class pricing.

### PhD Programs

PhD programs gate access to the credential through coursework requirements, qualifying exams, committee formation, and defense. The underlying capability — the ability to produce original research — exists in the person. The program provides the institutional API for having that capability recognized.

### Compute Allocation

University HPC centers (ICER at MSU, NERSC, XSEDE) gate access to compute through allocation proposals. Faculty apply for time. Students get one-off allocations for projects. The compute is controlled, scarce, and gated by permission.

Meanwhile, every undergraduate with a gaming laptop has a GPU that can do f64 science through Vulkan. The capability is latent, unrecognized, and unused — because the institutional model assumes compute is scarce and must be centrally allocated.

---

## The Alternative: Proof of Work

### Instead of Citation-Sitting, Reproduce Their Work

The traditional PhD application strategy: cite faculty you want to work with, write a research statement that aligns with their interests, hope the committee sees potential.

The alternative: reproduce their published work. Run their experiments on more efficient hardware. Show them the results. The application is the artifact.

The ecoPrimals faculty network was not built by citing papers. It was built by reproducing them:

| Professor | What Was Reproduced | Result |
|-----------|-------------------|--------|
| Murillo | Sarkas Yukawa OCP MD, TTM, surrogate learning | 195/195 checks, 0.000% energy drift on $600 GPU |
| Dong | FAO-56, sensor calibration, IoT irrigation | 326 checks, R²=0.967 across 918 station-days |
| Waters | 7 papers on QS, c-di-GMP, phage defense | ODE models, signaling networks reproduced |
| Liu | PhyloNet-HMM, SATé, introgression | 42+ checks, phylogenetic pipelines reproduced |
| Bazavov | SU(3) Wilson, Abelian Higgs, QCD EOS | 29+ checks, lattice QCD on consumer GPU |
| Dolson | Counterdiabatic driving, MODES, directed evolution | 46 checks, evolutionary computation reproduced |
| Kachkovskiy | Anderson localization, spectral theory (11 papers) | Spectral methods on GPU |
| R. Anderson | 6 papers on extremophile evolution | 133+ checks, metagenomics + pangenomics |
| Jones | PFAS detection, mass spec pipelines | 40+ checks, spectral matching (926× GPU speedup) |

When you've reproduced a professor's life work on consumer hardware and the results pass, the PhD application is a formality. The named faculty want to work with you because you've already demonstrated you can extend their research program — not because you wrote a polite letter.

### Instead of Journal Submission, Publish Open

The traditional publication strategy: submit to a journal, wait for anonymous reviewers, revise, wait again, pay APC or accept paywall, receive impact factor.

The alternative: publish the code, the data, and the results under AGPL-3.0. Invite named reviewers to write public assessments, also open. The review is auditable, the reviewer is accountable, and anyone can verify the science by running the code.

Murillo doesn't review for journals anymore. But he might review an open project that reproduces his own published work — because the review has value to him (someone extended his science) rather than value to a publisher (someone's paper gets an impact factor).

### Instead of HPC Allocation, Use What's Already There

PhD students don't have their own compute. That's how it works in every program: you apply for allocation, you get a one-off for your project, you wait in the queue.

But undergraduates have latent gaming power. Every RTX 3060, 4060, 4070 in a dorm room is a science chip — the f64 capability exists in the silicon, Vulkan unlocks it, BarraCuda runs on it. A department that deployed BarraCuda-powered springs across its students' gaming laptops would have a distributed HPC it didn't know it owned.

MSU's Genome Corp could build its own AlphaFold. Not on a $100M cluster — on the GPUs students already bought for gaming. The capability is there. The institution just doesn't see it yet because the CUDA model says consumer GPUs can't do science.

---

## The Frame

Sovereign computing is the infrastructure: Pure Rust, no vendor lock-in, runs on consumer hardware. Sovereign science is the methodology: reproduce the work, publish the results, invite open review, use credentials strategically but don't submit to them.

The PhD is an interface to the institution. The journal is an interface to the community. The HPC allocation is an interface to the compute. These interfaces are useful — they provide authority, recognition, and access. But they are not the science. The science is in the springs. The springs run on consumer hardware. The results are public. Reality owns itself.

The goal is not to reject institutions. The goal is to engage with them from a position where the work already exists, the evidence already passes, and the credential is a formality that opens doors — not a gatekeeping mechanism that determines whether the work is real.

---

## Practical Path

1. **Publish sporePrint and springs as AGPL-3.0** — the non-personal documentation, thesis drafts, and all spring repositories are already public or will be.

2. **Publish primals when stable** — ToadStool, BarraCuda, and all primals go public. The infrastructure is the proof that the methodology works.

3. **Invite named faculty review** — Murillo reviews hotSpring. Bazavov reviews lattice QCD. Liu reviews phylogenetics. Waters reviews QS models. Their reviews are public, attached to the artifact, auditable.

4. **Apply for PhD as formality** — the named faculty already know the work because they've reviewed it (or their own papers have been reproduced by it). The application is "here's the repo, here's your paper running on a $600 GPU, here's the thesis draft."

5. **Use the degree strategically** — reach the quality standards, defend, and then work sovereign. The degree opens institutional doors. The science opens everything else.

6. **Demonstrate latent compute** — show that the f64 discovery + BarraCuda + springs can run on student gaming hardware. If a single spring can run on a dorm-room RTX 3060, the argument for distributed sovereign compute at a university scale becomes concrete.

> **Note**: For the formal treatment of constrained evolution, biological validation, and quantitative evidence, see the [thesis](@/thesis/_index.md) — especially [Chapter 3: Theoretical Framework](@/thesis/03_theoretical_framework.md) and [Chapter 14: Biological Validation](@/thesis/14_biological_validation.md).


---

## Related

- [Thesis](@/thesis/_index.md) — formal constrained evolution thesis with biological validation
- [Reproduce](@/lab/reproduce.md) — how to reproduce published work on consumer hardware
- [Constrained Evolution Formal](@/methodology/CONSTRAINED_EVOLUTION_FORMAL.md) — theoretical framework behind the methodology
- [Philosophy](@/philosophy/_index.md) — broader sovereign computing philosophy
