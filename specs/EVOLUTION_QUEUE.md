# sporePrint Evolution Queue

Planned changes, ordered by priority. When implemented, move to CHANGELOG.md.

Last reviewed: April 2026

---

## P0 — Next Session

### Periodic refresh: counts and versions
- [ ] Sync landing page stat cards (in `templates/index.html`) with current spring check counts
- [ ] Update Squirrel version/tests if alpha has advanced
- [ ] Verify plasmidBin inventory count still accurate
- [ ] Check if new baseCamp papers exist in whitePaper that need sporePrint pages
- [ ] Verify LOC estimates in PRIMAL_CATALOG.md — run tokei on each primal repo for ground truth

### Content gaps
- [ ] guideStone section has only `_index.md` — needs dedicated pages for the verification protocol, deployment artifact standard, and cross-substrate validation evidence
- [ ] guidePost (planned, paired with guideStone in wateringHole glossary) — document when it materializes
- [ ] Some science pages have ungrouped domains in `science_section.html` — verify domain assignments are complete

### Taxonomy completeness
- [ ] Audit all 55 pages: grep content for entity names not tagged in front matter
- [x] ~~Build-time validation script that checks registry fields and taxonomy cross-references~~ → `scripts/validate_registry.py`

---

## P1 — Near-Term

### Content enrichment
- [ ] Add a "Getting Started with plasmidBin" walkthrough (clone → fetch → deploy)
- [ ] Expand products pages with composition diagrams and BYOB examples
- [ ] Add cross-spring data flow diagram to SPRING_CATALOG.md or a dedicated page

### Visual evolution
- [ ] Add a full site map page (`/sitemap/`) that shows the complete tree structure with page counts
- [ ] Consider adding entity taxonomy counts to the site tree sidebar
- [ ] Mobile experience: test site tree usability on phones, consider defaulting `<details>` to closed
- [ ] Investigate whether the per-page TOC (right sidebar) and site tree (left sidebar) coexist well on medium screens

### Semantic validation (the "grammar compiler")
- [x] ~~Pre-build validation script~~ → `scripts/validate_registry.py` (required fields, totals, taxonomy cross-refs)
- [x] ~~Entity metrics in one place, referenced everywhere~~ → `config.toml` registry + shortcodes
- [ ] Entity names in prose match taxonomy tags in front matter (prose scanning)
- [ ] Entity registry entries match wateringHole PRIMAL_EMOJI_STANDARD (cross-repo check)
- [ ] Metrics in registry haven't drifted from source repos (automated tokei comparison)

### petalTongue integration
- [ ] When petalTongue can consume sporePrint content, add documentation on the modality pipeline
- [ ] Reference SPOREPRINT_CONTENT_DELIVERY_SPECIFICATION.md from petalTongue specs/

---

## P2 — Future

### Accessibility
- [ ] Audit site for WCAG 2.1 AA compliance
- [ ] Test with screen readers (VoiceOver, NVDA)
- [ ] Verify Catppuccin color contrast ratios (4.5:1 minimum for text)
- [ ] Ensure all emoji pairs have `aria-label` alternatives in templates

### Automation
- [ ] Script to pull check counts from spring repos and update stats (landing page + catalogs)
- [ ] Script to diff whitePaper/baseCamp against content/science/ for new papers
- [ ] Consider lychee CI for non-Zola link checking in specs/ and other non-content markdown

### Search
- [ ] Evaluate elasticlunr search quality for the current 55 pages
- [ ] Consider whether taxonomy pages should be included in the search index
- [ ] Evaluate faceted search (filter by primal/spring) if page count grows significantly

---

## Resolved (April 2026)

These were in the original queue and have been completed:

- [x] Taxonomies for primals and springs (build-validated typed tags)
- [x] Entity registry in config.toml (single source of truth for display names and emojis)
- [x] Entity shortcode (`{{ entity(name="beardog") }}`)
- [x] Site tree sidebar navigation with current-page highlighting
- [x] Card-based landing page (replaced all markdown tables)
- [x] Streamlined nav bar (10 items → 7)
- [x] coralReef and barraCuda sections in PRIMAL_CATALOG (§1.7, §1.8)
- [x] bingoCube mentioned in catalog (§3.1 tooling section)
- [x] NUCLEUS_ARCHITECTURE.md — composition model documentation
- [x] sporeGarden product pages (esotericWebb, helixVision, blueFish)
- [x] Dark Forest, Neural API, RootPulse documented in NUCLEUS_ARCHITECTURE
- [x] Faculty references reframed as "Reproduces work by" / "Literature Anchor"
- [x] Grade metrics replaced with LOC, test counts, coverage
- [x] ludoSpring stats updated (V30, 75 experiments, 1,692 checks)
- [x] Narrative rewrite of all primal catalog sections (identity + origin + constraint)
- [x] Narrative rewrite of all spring catalog sections (identity + headline results + constraint)
- [x] Entity registry expanded with LOC, tests, files, crates, domain, tier (measured via tokei)
- [x] Aggregate totals in `[extra.totals]` with sum validation
- [x] 4 shortcodes: `entity`, `entity_metrics`, `entity_stat`, `total_stat`
- [x] Landing page stats ribbon reads from `config.extra.totals`
- [x] All 23 catalog metrics lines replaced with `entity_metrics` shortcode calls
- [x] Pre-build validation script (`scripts/validate_registry.py`) in CI
- [x] Real LOC numbers: 3.2M Rust, 107K tests, 952 WGSL (old estimates were 220K)
