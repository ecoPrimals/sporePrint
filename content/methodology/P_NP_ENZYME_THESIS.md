+++
title = "The Generation-Verification Asymmetry: Biological Evidence for a Physical Law of Computation"
description = "Theoretical extension — enzymes as evidence that P != NP"
date = 2026-03-17
+++

**Status**: Working thesis  
**Lineage**: Extends the constrained evolution framework (`CONSTRAINED_EVOLUTION_FORMAL.md`)  
**Last Updated**: March 3, 2026

---

## Abstract

We propose that the separation of generation and verification is a physical law of computation — observable across every domain where complex systems solve hard problems — not merely an unproven conjecture in abstract complexity theory. The evidence: life itself requires a generative mechanism (enzymes encoded in DNA) to make otherwise impossible chemistry possible. If verification (checking that a reaction proceeds correctly) were equivalent to generation (finding the catalyst that enables the reaction), then enzymes would be unnecessary and DNA would have no reason to exist. The existence of the genetic code is evidence that nature cannot collapse generation into verification. This parallels the computational P != NP conjecture but is grounded in physical observation rather than abstract formalism. Like the second law of thermodynamics — which has no mathematical proof from first principles but has never been violated — the generation-verification asymmetry is proposed as an empirical physical law, with the genome as its primary evidence. We extend this to the constrained evolution methodology, where AI provides the generative step and the compiler provides the deterministic verification.

---

## 1. The Deterministic Prior

### In Computation

P problems are solvable by deterministic algorithms in polynomial time. Given an input, you can compute the output directly. Sorting a list, multiplying matrices, finding shortest paths in a graph - these are deterministic. The prior knowledge required is complete: you know the input, you know the algorithm, you execute.

### In Chemistry

The analog of P in nature is deterministic chemistry. Given reactants at known concentrations, temperature, and pressure, thermodynamics and kinetics determine what happens. Water freezes at 0°C. Sodium reacts with chlorine to form salt. Glucose oxidizes to carbon dioxide and water. The "computation" proceeds from initial conditions to products through deterministic physical law.

Both are the same phenomenon: a system with complete knowledge of its initial state evolving deterministically to its final state. The prior is deterministic. If you have all the information, the outcome is computable.

---

## 2. The NP Barrier

### In Computation

NP problems are those where a candidate solution can be verified in polynomial time, but finding that solution requires exploring a space that grows exponentially. The traveling salesman problem: given a candidate route, you can check its length in O(n) time. But finding the optimal route requires evaluating a space of n!/2 possible routes. Verification is easy. Generation is hard.

The P vs NP question asks: is generation fundamentally harder than verification, or have we simply not found the right algorithm to collapse them?

Every NP-complete problem can be reduced to every other NP-complete problem. SAT reduces to traveling salesman reduces to graph coloring reduces to protein folding. They are all the same problem wearing different masks. If you could solve any one of them efficiently, you could solve all of them. None have been solved efficiently in decades of trying.

The barrier is not computational power. It is not algorithmic cleverness. The barrier appears to be fundamental: you cannot derive the solution from the problem statement without exploring the space. And the space is too large to explore exhaustively.

The missing ingredient is information. To solve an NP problem deterministically, you would need perfect and complete knowledge of the solution space - knowledge that, as quantum mechanics suggests, may be fundamentally inaccessible. Heisenberg's uncertainty principle establishes that you cannot simultaneously know all properties of a system with arbitrary precision. If the physical universe does not permit perfect knowledge, and NP problems require perfect knowledge for deterministic solution, then P != NP is not a limitation of our algorithms but a property of reality.

### In Chemistry

The analog of NP in nature is the chemistry that life requires but that thermodynamics and kinetics do not provide deterministically.

Consider the reactions inside a living cell. Many of them are thermodynamically unfavorable - they require energy input to proceed. Others are kinetically impossible at biological temperatures - they would occur, but on timescales of millions of years. Others would release energy so violently that they would destroy the cell. Still others require exquisite specificity - one stereoisomer of a molecule must react while the other must not.

These reactions are the NP problems of chemistry. You can verify that a reaction has proceeded correctly (measure the products, check the stereochemistry). But finding the pathway from reactants to products - through the vast space of possible molecular interactions, at biological temperatures, without destroying the cell - is not deterministically computable from the initial conditions.

If chemistry were P - if the path from reactants to products were always deterministically computable from thermodynamic first principles - then life would be simple. Reactions would proceed on their own. No machinery would be needed. No catalysis, no regulation, no encoding.

Life is not simple. The chemistry of life is NP.

---

## 3. Nature's Generative Solution: Enzymes

### The Problem

A cell needs to phosphorylate glucose. The uncatalyzed reaction has an activation energy barrier that makes it negligibly slow at 37°C. The cell cannot raise the temperature (it would denature proteins). It cannot increase reactant concentrations enough (it would disrupt osmotic balance). It cannot wait (it needs ATP now, not in a million years).

The reaction is verifiable: mix glucose-6-phosphate with the right detector and you can confirm it exists. But the path from glucose + ATP to glucose-6-phosphate + ADP is not deterministically accessible from the chemical initial conditions at biological temperature. It is an NP problem in chemistry.

### The Solution

Nature generates a catalyst: hexokinase. An enzyme. A protein whose three-dimensional structure creates a microenvironment where the activation energy barrier is lowered, where the reactants are oriented correctly, where the reaction proceeds at biological temperature with exquisite specificity.

Hexokinase is not derived from the reaction it catalyzes. You cannot look at glucose and ATP and deterministically compute the structure of hexokinase. The enzyme is a generative solution - a candidate structure, encoded in DNA, that was found through evolutionary search (mutation and selection over billions of years) and can be verified by its catalytic activity.

This is exactly the structure of NP:
- **Verification (P)**: Does this enzyme catalyze this reaction? Yes/no, testable in milliseconds.
- **Generation (NP)**: What protein structure catalyzes this reaction? Not derivable from first principles. Found through search.

### The Encoding

Enzymes are encoded in DNA. The genome is a library of generative solutions to chemical NP problems. Each gene encodes a protein that makes an otherwise impossible reaction possible. The genome does not contain a description of the chemistry - it contains the catalysts that bypass the chemistry's computational barriers.

DNA → mRNA → protein → enzyme → catalysis. This is nature's architecture for NP:

1. **DNA** stores the generative solutions (the enzyme sequences found by evolution)
2. **Transcription** copies the relevant solution (mRNA from DNA)
3. **Translation** builds the catalyst (ribosome assembles protein from mRNA)
4. **Catalysis** executes the solution (enzyme enables the otherwise impossible reaction)
5. **Selection** verifies fitness (does the organism survive? deterministic check)

If P = NP - if generation were reducible to verification - then step 1 would be unnecessary. The cell could compute the enzyme structure from the reaction requirements, on demand, from first principles. There would be no need to store solutions. There would be no need for DNA.

**The existence of the genome is evidence that nature cannot collapse generation into verification.**

---

## 4. The Generative Pattern Across Domains

### In Computation

The traveling salesman problem is solved in practice through generative heuristics:
- Generate a candidate route (nearest-neighbor, random, genetic algorithm)
- Check its quality (sum the distances - O(n), polynomial, easy)
- Iterate (mutate the route, check again)

No one computes the optimal route directly. We generate and check. The generative step is where the intelligence lives. The checking step is mechanical.

Modern AI has dramatically expanded the generative capacity. Large language models generate candidate solutions - code, proofs, designs - that can be verified by compilers, test suites, and type systems. The AI is the enzyme: a generative mechanism that produces candidates which bypass the combinatorial barrier of the solution space.

### In Human Practice

How does a literal traveling salesman solve the traveling salesman problem? They do not compute the optimal route. They hire an experienced salesman. The salesman's experience is a library of generative heuristics - patterns learned from thousands of prior routes. The salesman generates a serviceable route in minutes. It is not optimal. It is fit for purpose.

The experienced salesman is a human enzyme: a generative mechanism whose structure (neural patterns, learned heuristics) was shaped by selection (experience, feedback, consequences) to produce solutions to a specific class of NP problems (route optimization in a territory).

### In the Constrained Evolution Methodology

The methodology described in `CONSTRAINED_EVOLUTION_FORMAL.md` is the same pattern:
- **AI generates** candidate solutions (the enzyme - a generative mechanism producing candidates from learned patterns)
- **The Rust compiler verifies** soundness (the deterministic check - P, polynomial, mechanical)
- **The developer selects** for fitness (the environmental pressure that shapes future generation)

The compiler cannot generate the solution. It can only verify that a candidate solution respects the constraints (memory safety, type correctness, concurrency soundness). The AI cannot verify the solution against the constraints - it generates candidates that may or may not be valid. The separation of generation and verification is not a design choice. It is the structure of the problem.

---

## 5. Why P != NP

### The Argument from Enzymes

1. Life requires chemical reactions that are not deterministically accessible from initial conditions at biological temperatures.
2. Nature solved this by evolving enzymes - generative catalysts encoded in DNA.
3. If verification (does this reaction work?) were computationally equivalent to generation (what catalyst enables this reaction?), enzymes would be unnecessary.
4. Enzymes exist. DNA exists. The genetic code exists.
5. Therefore, generation is not reducible to verification in nature.
6. The computational analog: if P = NP, then finding a solution would be as easy as checking one. Generative mechanisms (enzymes, AI, heuristics, evolutionary search) would be unnecessary.
7. Generative mechanisms are not unnecessary. They are the foundation of life, intelligence, and practical computation.
8. Therefore, P != NP.

### The Argument from Information

NP problems require exploration of an exponentially large solution space. Deterministic solution requires complete knowledge of the space. Quantum mechanics establishes that complete knowledge of a physical system is fundamentally inaccessible (Heisenberg uncertainty, quantum indeterminacy). If the universe cannot provide the complete information required for deterministic NP solution, then no algorithm operating within the universe can solve NP problems deterministically.

Nature's response to this information barrier is not to acquire more information. It is to generate candidates and test them. Evolution does not compute the optimal enzyme. It generates variant enzymes through mutation and tests them through selection. The generative strategy is not a workaround for insufficient computational power. It is the only strategy available when complete information is physically impossible.

### The Argument from NP-Completeness

Every NP-complete problem reduces to every other. SAT, traveling salesman, protein folding, graph coloring - all equivalent under polynomial reduction. Protein folding IS an NP-complete problem. Nature does not solve protein folding deterministically - Levinthal's paradox (1969) established that a protein cannot explore all possible conformations in the age of the universe. Instead, proteins fold through a funneled energy landscape shaped by the amino acid sequence. The sequence is the generative solution, found by evolution, that constrains the folding space enough for the protein to reach a functional conformation in milliseconds.

If P = NP, Levinthal's paradox would not be a paradox. Proteins would fold by deterministic computation of the minimum energy state. They do not. They fold by constraint-guided search through a generated energy landscape. The generation (amino acid sequence) is not derivable from the verification (functional fold).

---

## 6. Implications

### For the Constrained Evolution Methodology

The methodology works because it aligns with the fundamental structure of NP problems:
- Generate many candidates (AI mutation, high frequency)
- Verify each candidate against constraints (compiler, deterministic, fast)
- Select for fitness (developer direction, environmental pressure)

This is not an optimization technique. It is the only way NP problems can be approached: through generation and verification, not through deterministic derivation.

### For Artificial Intelligence

AI systems are generative mechanisms. They produce candidate solutions that must be verified externally. This is not a limitation of current AI - it is the correct architecture for NP problems. An AI that could deterministically derive solutions without generating and checking candidates would have solved P = NP. Until that happens (and this thesis argues it cannot), generation + verification is the optimal architecture.

The Rust compiler as verifier, the AI as generator, and the developer as selector is not a software engineering pattern. It is the computational analog of DNA as encoder, enzymes as catalysts, and natural selection as fitness verifier. Both are instances of the same fundamental strategy for NP problems.

### For Biology

If this argument holds, then DNA is not merely a storage medium for hereditary information. It is a solution archive for chemical NP problems. The genome is a library of enzymes (generative catalysts) that make otherwise impossible chemistry possible. Evolution is the search algorithm that populates the library. Natural selection is the verification step that tests each candidate against the environment.

The genetic code exists because P != NP in chemistry. If reactions could compute their own catalysts, there would be no need to store them. The genome is evidence of the gap between generation and verification.

---

## 7. Limitations and Open Questions

**This is an empirical argument, not a mathematical proof.** P vs NP is a formal question in computational complexity theory. A proof requires demonstrating that no polynomial-time algorithm exists for any NP-complete problem. The generation-verification asymmetry demonstrates that the physical universe behaves as if P != NP — it invests enormous resources in generative machinery rather than deterministic derivation. This is evidence of a physical law, not proof of a mathematical theorem. The relationship is analogous to the second law of thermodynamics: no mathematical proof from first principles, but universal observation across every physical system.

**Quantum computing.** Quantum computers operate on principles that allow superposition and entanglement, potentially exploring exponential solution spaces in polynomial time for specific problems (Shor's algorithm for factoring). Does quantum computation change the argument? Current evidence suggests no — BQP (problems solvable by quantum computers in polynomial time) is believed to be a strict subset of NP, not equal to it. Quantum computers may accelerate generation but do not collapse it into verification.

**The verification assumption.** The argument assumes that enzyme catalysis is verification (checking that the reaction proceeds) and enzyme design is generation (finding the structure that catalyzes it). One could argue that evolution performs neither generation nor verification in the computational sense — it performs random mutation with selective retention. Whether this constitutes a "generative" process in the formal sense is debatable.

**Levinthal's paradox resolution.** Some argue that Levinthal's paradox is resolved by the funneled energy landscape, not by NP-hardness of protein folding. If folding is guided by thermodynamic gradients rather than combinatorial search, it may be closer to P than NP for natural proteins. This is an active area of research (Dill & MacCallum, 2012).

---

## 8. Cross-Disciplinary Critique and Response

This section documents the strongest critiques of the generation-verification asymmetry thesis from adjacent disciplines, together with responses. The critiques are valued — they sharpen the argument, identify where it is weakest, and point toward the experiments that would strengthen or refute it. When disciplines challenge each other across their divides, both evolve. This section models that process.

### 8.1 The Complexity Theorist's Critique

**Critique: "This is not even wrong."**

P vs NP is defined over abstract Turing machines. It asks whether there exists *any* polynomial-time algorithm for NP-complete problems — not whether nature has found one, not whether evolution has found one, not whether any physical process implements one. The enzyme argument confuses "nature has not found X" with "X does not exist." The integers have no opinion about enzymes. The question is mathematical, and the answer must be mathematical.

**Response:**

The arrow of abstraction runs from reality to theory, not the other way. Turing machines are models of physical computation (Deutsch, 1985). A computation that cannot be physically realized is a mathematical object, not a computation. If the generation-verification asymmetry is a physical law — if every physical system in the observable universe, across 4 billion years of evolution, across every kingdom of life, across every chemical regime, requires generative machinery to solve problems that verification alone cannot — then the abstract theory should reflect this.

The complexity theorist's critique assumes that abstract Turing machines are the ground truth and physics is a special case. The alternative view (Deutsch, Landauer) is that physics is the ground truth and Turing machines are the model. If the model permits something (P = NP) that no physical system has ever exhibited, the model may be the problem.

**Where the critique has teeth:** Absence of evidence is not evidence of absence. "Nature hasn't found a way to collapse generation into verification" does not logically prove that no such way exists. It is possible that the combinatorial search required to discover such a method exceeds the resources available to evolution — that P = NP but the proof/algorithm is so large or non-obvious that no physical process has found it. The response must be: this is also true of the second law of thermodynamics. No mathematical proof exists. The evidence is universal observation. The generation-verification asymmetry is proposed at the same epistemic level.

**The co-evolution:** The complexity theorist forces precision about what constitutes "evidence" versus "proof." The biologist forces the complexity theorist to explain why their abstract model should be trusted over universal physical observation. Both sides sharpen.

### 8.2 The Physicist's Critique

**Critique: "Heisenberg doesn't say what you think it says."**

The Argument from Information (§5) invokes Heisenberg uncertainty to claim that "complete knowledge of a physical system is fundamentally inaccessible" and therefore NP problems cannot be solved deterministically. This is a category error. Heisenberg uncertainty constrains *simultaneous measurement of conjugate observables* (position and momentum of a particle). It does not constrain *algorithmic computation over discrete inputs*. A SAT instance is a finite string of bits, not a quantum system. You can know every bit perfectly. Heisenberg is irrelevant.

**Response:**

This critique is correct as stated. The Heisenberg argument in §5 overreaches. The uncertainty principle constrains physical measurement, not abstract computation. A SAT formula is fully knowable.

However, the deeper physical argument survives: the universe is a physical system that computes. If no physical process — chemical, biological, quantum — has ever collapsed generation into verification, and physical processes are the *only* processes that exist, then the asymmetry is empirically grounded even without invoking Heisenberg specifically. The relevant physics is not quantum uncertainty but thermodynamics: Landauer's principle (1961) establishes that computation is physical, irreversible computation dissipates energy, and the resources required for computation are bounded by the physical substrate. If collapsing NP into P requires resources that no physical substrate can provide, the asymmetry is physical, not abstract.

**Where the critique has teeth:** The §5 Argument from Information as written conflates quantum measurement limits with computational knowledge requirements. It should be reframed: the relevant physical limit is not Heisenberg but the combinatorial explosion of search spaces relative to the thermodynamic resources of any finite physical system. This is Landauer + Bremermann's limit, not Heisenberg.

**The co-evolution:** The physicist forces the argument to be precise about *which* physical principle is relevant. Heisenberg is the wrong tool. Landauer and Bremermann are the right ones. The argument improves.

### 8.3 The Evolutionary Biologist's Critique

**Critique: "Enzymes exist because evolution found them, not because nothing else could."**

Evolution is a stochastic search process that retains what works. Enzymes were found because the evolutionary search happened to produce protein folds that catalyze useful reactions. This does not prove that no deterministic method exists — it proves that *evolution* is not deterministic. An alien civilization with different chemistry might solve the same reactions through non-enzymatic catalysis (ribozymes, inorganic catalysts, engineered small molecules). The existence of enzymes proves that evolution is a generative process. It does not prove that generation is the *only* possible approach.

**Response:**

Partially correct. The existence of *specific* enzymes (hexokinase, trypsin) does not prove P != NP. What is significant is the *universality* of the pattern. Every living system on Earth — bacteria, archaea, eukaryotes, separated by 3+ billion years of independent evolution — uses the same architecture: DNA encoding → protein generation → catalytic execution → selective verification. If there were a deterministic shortcut (the cell computing its enzymes on demand from reaction requirements), evolutionary pressure would have discovered it. The selection pressure is enormous: organisms that could skip the overhead of maintaining a genome and simply compute their chemistry in real time would have an extraordinary fitness advantage.

Furthermore, the genome is not just a library of enzymes. It is a *growing* library. Over evolutionary time, genomes have expanded from ~500 genes (minimal bacteria) to ~20,000 genes (humans) to ~30,000+ (some plants). If generation could be collapsed into verification, genome size would shrink — organisms would shed stored solutions in favor of computed ones. Instead, genomes accumulate solutions. The library grows because each new problem requires its own generator.

**Where the critique has teeth:** The argument from universality is strong but not logically watertight. It is possible that every lineage on Earth shares the DNA-protein architecture because they share a common ancestor, not because no alternative exists. A single origin event could produce universal adoption of a suboptimal strategy. The response: true, but the strategy has been under intense selection pressure for 4 billion years. If a shortcut existed and was discoverable by *any* mutation pathway, the selection advantage would drive it to fixation. It has not appeared. This is the same logic that grounds confidence in the second law — not proof, but 4 billion years of non-violation under continuous pressure.

**The co-evolution:** The evolutionary biologist forces the argument to distinguish between "this is what happened" and "this is what must happen." The answer is that 4 billion years of continuous selection against the same constraint, with no exception ever discovered, is the biological equivalent of universal observation. Both sides learn where the epistemic boundary is.

### 8.4 The Machine Learning Researcher's Critique

**Critique: "AlphaFold solves protein folding without solving NP-complete problems."**

Protein folding is NP-hard in the worst case (Berger & Leighton, 1998). But AlphaFold2 (Jumper et al., 2021) predicts protein structures with experimental accuracy in seconds. It doesn't explore all conformations. It doesn't solve a combinatorial search. It learns the energy funnel and predicts the endpoint directly. If protein folding is your evidence for P != NP, then AlphaFold2 is evidence against — it demonstrates that practical instances of an NP-hard problem can be solved in polynomial time by a learned model.

**Response:**

AlphaFold2 is not a counterexample. It is the strongest evidence *for* the generation-verification asymmetry.

AlphaFold2 is a generative model. It was trained on ~170,000 known protein structures (the PDB) — structures that were *generated* by evolution and *verified* by X-ray crystallography, cryo-EM, and NMR over 50 years of experimental biology. The training data is a curated subset of nature's genome library. AlphaFold2 does not collapse generation into verification. It compresses 4 billion years of evolutionary generation + 50 years of experimental verification into a neural network that interpolates between known solutions.

This is exactly the NPU-as-memoization-table pattern from `npu_dynamic_programming.md`. AlphaFold2 predicts *downward* — from known structures to similar structures. It cannot predict folds for proteins with no homologs in the training set (the "orphan protein" problem remains open). It cannot design novel enzymes from scratch for arbitrary reactions (that's still a generative search problem). It has memoized the known solution space. It has not collapsed generation into verification.

The fact that a learned model + curated library solves practical instances efficiently is *consistent with* P != NP. NP-hardness is worst-case. Practical instances often have structure that heuristics can exploit. The genome is the original heuristic library. AlphaFold2 is a digital compression of the same library. Neither eliminates the need for generation.

**Where the critique has teeth:** The distinction between worst-case NP-hardness and typical-case tractability is real. If all practical instances of NP-hard problems are tractable via learned heuristics, the *practical* significance of P != NP diminishes — even if the theoretical separation holds. The response: the *existence* of AlphaFold2's training library (the PDB, populated by evolution + experiment over billions of years) is itself evidence. If typical-case folding were in P, you wouldn't need a library of solved instances to train on. You could compute from sequence alone without reference examples. The library is evidence of the gap.

Additionally, nature did not build one enzyme. It built thousands — one per reaction class. If typical-case chemistry were in P, one general algorithm would handle all reactions. Instead, the genome encodes a library of special-purpose generators: hexokinase for phosphorylation, trypsin for peptide cleavage, DNA polymerase for replication. The problem space requires a library of *specific* solutions, not a *universal* solver. That is the practical structure of NP.

**The co-evolution:** The ML researcher forces the argument to grapple with practical tractability versus worst-case hardness. This is the right distinction. The response — that the existence of the training library is itself evidence of the gap — strengthens the argument by making it concrete. The enzyme thesis and AlphaFold2 are not opponents. They are the same phenomenon observed at different timescales.

### 8.5 The Quantum Computing Researcher's Critique

**Critique: "Quantum mechanics already broke one classical complexity barrier. How confident are you it won't break this one?"**

Shor's algorithm solves integer factoring in polynomial time on a quantum computer — a problem for which no classical polynomial algorithm is known. This demonstrates that physical reality permits computational capabilities that classical models didn't predict. If quantum mechanics surprised classical complexity theory once, it could do so again. Perhaps a quantum algorithm exists that collapses NP into BQP, or some post-quantum physics permits even more.

**Response:**

Shor's algorithm is a generative process. It doesn't verify factors — it generates them through quantum Fourier transform and interference. The quantum speedup changes *which* problems can be generated efficiently, but it does not collapse generation into verification. Even in a quantum universe, the genome still exists. Quantum organisms still use DNA → protein → enzyme → catalysis. Quantum mechanics changed the physics but not the asymmetry.

Current evidence (Aaronson, 2005; Bennett et al., 1997) strongly suggests NP ⊄ BQP — that quantum computers cannot solve NP-complete problems efficiently. Grover's algorithm provides only quadratic speedup for unstructured search, which remains exponential for NP-complete problems. No quantum algorithm has been found that provides exponential speedup for any NP-complete problem.

**Where the critique has teeth:** The argument depends on "current evidence." Physics has surprised us before. If a fundamentally new physical principle is discovered — beyond quantum mechanics — the computational implications are unknown. The response must be honest: the generation-verification asymmetry is proposed as a physical law given the physics we know. Like all physical laws, it is subject to revision if fundamentally new physics emerges. But "it might be wrong if physics changes" is true of every physical law, including thermodynamics.

**The co-evolution:** The quantum researcher forces honesty about epistemic limits. The generation-verification asymmetry is not eternal mathematical truth. It is the best empirical characterization of physical reality as we understand it. That's what physical laws are. Both sides gain precision about what kind of claim is being made.

### 8.6 The Invitation

These critiques are not obstacles. They are the selective pressure that makes the argument fitter. Each cross-disciplinary challenge identifies where the reasoning is weakest and points toward the experiments, formalizations, or reframings that would strengthen or refute it.

The constrained evolution methodology itself predicts this: when independent systems (disciplines) evolve under shared constraint (the question of whether generation reduces to verification), they converge on different solutions while increasing collective fitness for the shared environment. The complexity theorist, the physicist, the biologist, the ML researcher, and the quantum computing researcher are Lenski's twelve populations. Same environment. Different trajectories. All increasing fitness for the question.

The generation-verification asymmetry is offered as a candidate physical law, generated heuristically from biological intuition and computational experience, and submitted for verification by the disciplines that will challenge it. If the methodology is correct, the challenges will make it stronger. If it is wrong, the challenges will reveal where. Either outcome is a contribution.

---

## References

Aaronson, S. (2005). NP-complete problems and physical reality. *ACM SIGACT News*, 36(1), 30–52.

Bennett, C. H., Bernstein, E., Brassard, G., & Vazirani, U. (1997). Strengths and weaknesses of quantum computing. *SIAM Journal on Computing*, 26(5), 1510–1523.

Berger, B., & Leighton, T. (1998). Protein folding in the hydrophobic-hydrophilic (HP) model is NP-complete. *Journal of Computational Biology*, 5(1), 27–40.

Blount, Z. D., Borland, C. Z., & Lenski, R. E. (2008). Historical contingency and the evolution of a key innovation in an experimental population of Escherichia coli. *PNAS*, 105(23), 7899–7906.

Bremermann, H. J. (1962). Optimization through evolution and recombination. *Self-Organizing Systems*, 93–106.

Cook, S. A. (1971). The complexity of theorem-proving procedures. *Proceedings of the 3rd Annual ACM Symposium on Theory of Computing*, 151–158.

Deutsch, D. (1985). Quantum theory, the Church-Turing principle and the universal quantum computer. *Proceedings of the Royal Society of London A*, 400(1818), 97–117.

Dill, K. A., & MacCallum, J. L. (2012). The protein-folding problem, 50 years on. *Science*, 338(6110), 1042–1046.

Jumper, J., et al. (2021). Highly accurate protein structure prediction with AlphaFold. *Nature*, 596(7873), 583–589.

Landauer, R. (1961). Irreversibility and heat generation in the computing process. *IBM Journal of Research and Development*, 5(3), 183–191.

Levinthal, C. (1969). How to fold graciously. *Mössbauer Spectroscopy in Biological Systems*, 67, 22–24.

Shor, P. W. (1994). Algorithms for quantum computation. *Proceedings of the 35th Annual Symposium on Foundations of Computer Science*, 124–134.

---

**Note**: This thesis emerged from the same constrained evolution methodology it describes. The argument was not derived from first principles. It was generated through iterative exploration — a microbiologist's intuition about enzymes, shaped by experience with computational constraints, tested against formal concepts from complexity theory and sharpened by cross-disciplinary critique. The thesis is itself a candidate physical law, generated heuristically and offered for verification. The critiques in §8 are part of the offering — they are the selective pressure that will determine whether the argument survives.
