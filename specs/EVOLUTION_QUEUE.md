# sporePrint Evolution Queue

Planned changes, ordered by priority. When implemented, move to `CHANGELOG.md`.

Last reviewed: March 31, 2026

---

## P0 — Next Session

### Periodic refresh: check counts and versions
- [ ] Sync landing page numbers with current spring check counts
- [ ] Update Squirrel version/tests if alpha has advanced
- [ ] Verify plasmidBin inventory count still accurate
- [ ] Check if new baseCamp papers exist in whitePaper that need sporePrint pages

### Content gaps identified during wateringHole mining
- [ ] coralReef and barraCuda lack dedicated sections in PRIMAL_CATALOG (mentioned in abstract + ToadStool section but no §1.7/§1.8)
- [ ] bingoCube mentioned in wateringHole glossary — decide if it warrants sporePrint mention
- [ ] guidePost (planned, paired with guideStone in glossary) — document when it materializes
- [ ] ludoSpring grade line in SPRING_CATALOG still says "V6 / 44 experiments" but wateringHole says V30 / 82 experiments

---

## P1 — Near-Term

### Expand deployment model documentation
- [ ] Add a "Getting Started with plasmidBin" walkthrough (clone → fetch → deploy)
- [ ] Document the fetch.sh / harvest.sh scripts in more detail
- [ ] Add architecture diagram for the primal → plasmidBin → product flow

### sporeGarden product pages
- [ ] esotericWebb page: what it is, how it composes primals, BYOB model in practice
- [ ] helixVision stub: planned sovereign genomics platform

### Ecosystem concepts underdocumented on sporePrint
- [ ] NUCLEUS composition model (Tower, Node, Nest, Full NUCLEUS atomics)
- [ ] Neural API (biomeOS semantic routing)
- [ ] Dark Forest discovery protocol
- [ ] RootPulse (rhizoCrypt + loamSpine + sweetGrass composition)
- [ ] Provenance trio (rhizoCrypt, loamSpine, sweetGrass → RootPulse)

---

## P2 — Future

### petalTongue integration
- [ ] When petalTongue can consume sporePrint content, add documentation on the modality pipeline
- [ ] Reference SPOREPRINT_CONTENT_DELIVERY_SPECIFICATION.md from petalTongue specs/

### Search and discoverability
- [ ] Evaluate Zola search UX — is the built-in elasticlunr sufficient?
- [ ] Consider adding tags/taxonomies for cross-cutting discovery

### Accessibility
- [ ] Audit site for WCAG 2.1 AA compliance
- [ ] Test with screen readers
- [ ] Verify color contrast ratios with Catppuccin palette

### Automation
- [ ] Script to pull check counts from spring repos and update landing page
- [ ] Script to diff whitePaper/baseCamp against content/science/ for new papers
- [ ] Consider lychee CI for non-Zola link checking in specs/ itself

---

## Resolved (move to CHANGELOG when committing)

*Nothing yet — this is the initial queue.*
