// SPDX-License-Identifier: AGPL-3.0-or-later
//
// JELLY STRING — evolution target: petalTongue server-rendered SVG.
// This JS exists only because Zola is a static site generator without
// a Rust runtime. Once petalTongue serves interactive science pages
// directly (WASM or server-rendered HTML), these files are vestigial.
// See: wateringHole/petaltongue/SPOREPRINT_EVOLUTION_ROADMAP.md
//
// Explorer configuration — capability-based endpoint discovery.

'use strict';

const ExplorerConfig = (() => {
  const DATA_DIR = '/gonzales/data/';

  const SCENARIOS = {
    'ic50':     { file: 'gonzales_dermatitis.json', live: '/gonzales/dose-response', label: 'IC50 Dose-Response' },
    'pk':       { file: 'gonzales_dermatitis.json', live: '/gonzales/pk-decay',      label: 'PK Decay' },
    'tissue':   { file: 'tissue_geometry.json',     live: '/gonzales/tissue-lattice', label: 'Tissue Geometry' },
    'hormesis': { file: 'hormesis.json',            live: '/anderson/hormesis',       label: 'Hormesis' },
    'species':  { file: 'cross_species.json',       live: '/anderson/cross-species',  label: 'Cross-Species' },
    'full':     { file: 'full_gonzales.json',       live: '/gonzales/full',           label: 'Full Dashboard' },
  };

  const SLIDER_DEFS = {
    'ic50': [
      { param: 'n_points', label: 'Points',        min: 10,  max: 500,  step: 10,  value: 50 },
      { param: 'dose_max', label: 'Max Dose (nM)', min: 50,  max: 5000, step: 50,  value: 500 },
      { param: 'hill_n',   label: 'Hill n',         min: 0.1, max: 5,    step: 0.1, value: 1.0 },
    ],
    'pk': [
      { param: 'n_points',   label: 'Points',   min: 20, max: 500, step: 10, value: 100 },
      { param: 't_max_days', label: 'Max Days',  min: 14, max: 180, step: 7,  value: 56 },
    ],
    'tissue': [
      { param: 'disorder', label: 'Disorder W', min: 1, max: 50, step: 0.5, value: 10 },
    ],
    'hormesis': [
      { param: 'amplitude', label: 'Amplitude', min: 0.05, max: 1.0,  step: 0.05, value: 0.3 },
      { param: 'k_stim',    label: 'K stim',    min: 1,    max: 50,   step: 1,    value: 10 },
      { param: 'k_inh',     label: 'K inh',     min: 10,   max: 500,  step: 10,   value: 100 },
      { param: 'n_points',  label: 'Points',    min: 20,   max: 500,  step: 10,   value: 100 },
      { param: 'dose_max',  label: 'Max Dose',  min: 50,   max: 5000, step: 50,   value: 200 },
    ],
  };

  const GRAMMAR_ENDPOINTS = {
    'ic50':     '/gonzales/dose-response',
    'pk':       '/gonzales/pk-decay',
    'tissue':   '/gonzales/tissue-lattice',
    'hormesis': '/anderson/hormesis',
    'species':  '/anderson/cross-species',
  };

  const ACCESS_TIERS = {
    public:       { label: 'Public',       color: '#565f89', desc: 'Static JSON only' },
    visitor:      { label: 'Visitor',      color: '#9ece6a', desc: 'Read-only live science' },
    collaborator: { label: 'Collaborator', color: '#7aa2f7', desc: 'Parameter exploration + provenance drill-down' },
    owner:        { label: 'Owner',        color: '#bb9af7', desc: 'Full system access + vault admin' },
  };

  const REFERENCE_PAPERS = {
    'gonzales.dose_response': {
      doi: '10.1111/jvp.12065',
      short: 'Gonzales AJ et al. 2014',
      title: 'Oclacitinib (APOQUEL) is a novel JAK inhibitor',
      journal: 'J Vet Pharmacol Ther 37:317-324',
      table: 'Table 1 — IC50 values',
      chembl: 'CHEMBL2103874',
      pubchem_cid: 44631938,
      chain: ['source', 'python_baseline', 'rust_validation', 'guidestone'],
    },
    'gonzales.pk_decay': {
      doi: '10.1111/vde.13028',
      short: 'Fleck TJ,...,Gonzales AJ 2021',
      title: 'Pharmacokinetics of lokivetmab (Cytopoint)',
      journal: 'Vet Dermatol 32:681-e182',
      table: 'Figure 2 — PK parameters',
      chain: ['source', 'rust_validation'],
    },
    'gonzales.tissue_lattice': {
      doi: '10.1111/jvp.12065',
      short: 'Gonzales 2014 + McCandless 2014',
      title: 'Anderson tissue geometry model',
      table: 'Derived from published cell-type distributions',
      chain: ['source', 'rust_validation'],
    },
    'anderson.hormesis': {
      doi: '10.1111/jvp.12065',
      short: 'Gonzales 2014 / Anderson framework',
      title: 'Hormetic dose-response in cytokine signaling',
      chain: ['source', 'rust_validation'],
    },
    'anderson.cross_species': {
      doi: '10.1111/j.1365-3164.2012.01090.x',
      short: 'Gonzales AJ et al. 2013',
      title: 'IL-31 role in canine pruritus and atopic dermatitis',
      journal: 'Vet Dermatol 24:48-53',
      chain: ['source', 'rust_validation'],
    },
  };

  const CHAIN_LABELS = {
    source: 'Published paper (DOI)',
    python_baseline: 'Python baseline (healthSpring)',
    rust_validation: 'Rust validation (35/35 PASS)',
    guidestone: 'guideStone (29/29 PASS)',
    nucleus: 'NUCLEUS composition (provenance trio)',
  };

  // Capability-based endpoint discovery.
  // Reads from meta tag or falls back to known lab API.
  function discoverBaseUrl() {
    const meta = document.querySelector('meta[name="lab-api-base"]');
    if (meta && meta.content) return meta.content;
    return 'https://lab.primals.eco/api/v1';
  }

  let _baseUrl = null;
  function baseUrl() {
    if (!_baseUrl) _baseUrl = discoverBaseUrl();
    return _baseUrl;
  }

  return {
    DATA_DIR,
    SCENARIOS,
    SLIDER_DEFS,
    GRAMMAR_ENDPOINTS,
    ACCESS_TIERS,
    REFERENCE_PAPERS,
    CHAIN_LABELS,
    baseUrl,
    scienceUrl:    () => baseUrl() + '/science',
    renderUrl:     () => baseUrl() + '/render',
    validationUrl: () => baseUrl() + '/validation/chain',
    healthUrl:     () => baseUrl() + '/health',
    compositionUrl:() => baseUrl() + '/system/composition',
  };
})();
