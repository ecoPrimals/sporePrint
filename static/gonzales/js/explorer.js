// SPDX-License-Identifier: AGPL-3.0-or-later
// Gonzales Interactive Science Explorer
// Phase 2: live data from wetSpring facade + static JSON fallback + provenance.

const DATA_DIR = '/gonzales/data/';
const LIVE_API = 'https://lab.primals.eco/api/v1/science';

const SCENARIOS = {
  'ic50':    { file: 'gonzales_dermatitis.json', live: '/gonzales/dose-response', label: 'IC50 Dose-Response' },
  'pk':      { file: 'gonzales_dermatitis.json', live: '/gonzales/pk-decay',      label: 'PK Decay' },
  'tissue':  { file: 'tissue_geometry.json',     live: '/gonzales/tissue-lattice', label: 'Tissue Geometry' },
  'hormesis':{ file: 'hormesis.json',            live: '/anderson/hormesis',       label: 'Hormesis' },
  'species': { file: 'cross_species.json',       live: '/anderson/cross-species',  label: 'Cross-Species' },
  'full':    { file: 'full_gonzales.json',        live: '/gonzales/full',           label: 'Full Dashboard' },
};

const DARK_LAYOUT = {
  paper_bgcolor: '#1a1b26',
  plot_bgcolor:  '#1a1b26',
  font: { color: '#c0caf5', family: 'system-ui, -apple-system, sans-serif' },
  xaxis: { gridcolor: '#292e42', zerolinecolor: '#414868' },
  yaxis: { gridcolor: '#292e42', zerolinecolor: '#414868' },
  margin: { t: 40, r: 20, b: 50, l: 60 },
  legend: { bgcolor: 'rgba(0,0,0,0)' },
};

const COLORS = [
  '#7aa2f7', '#f7768e', '#9ece6a', '#e0af68',
  '#bb9af7', '#ff9e64', '#7dcfff', '#ff007c',
];

const SLIDER_DEFS = {
  'ic50': [
    { param: 'n_points', label: 'Points',   min: 10, max: 500, step: 10, value: 50 },
    { param: 'dose_max', label: 'Max Dose (nM)', min: 50, max: 5000, step: 50, value: 500 },
    { param: 'hill_n',   label: 'Hill n',    min: 0.1, max: 5, step: 0.1, value: 1.0 },
  ],
  'pk': [
    { param: 'n_points',   label: 'Points',   min: 20, max: 500, step: 10, value: 100 },
    { param: 't_max_days', label: 'Max Days',  min: 14, max: 180, step: 7, value: 56 },
  ],
  'tissue': [
    { param: 'disorder', label: 'Disorder W', min: 1, max: 50, step: 0.5, value: 10 },
  ],
  'hormesis': [
    { param: 'amplitude', label: 'Amplitude',    min: 0.05, max: 1.0, step: 0.05, value: 0.3 },
    { param: 'k_stim',    label: 'K stim',       min: 1, max: 50, step: 1, value: 10 },
    { param: 'k_inh',     label: 'K inh',        min: 10, max: 500, step: 10, value: 100 },
    { param: 'n_points',  label: 'Points',       min: 20, max: 500, step: 10, value: 100 },
    { param: 'dose_max',  label: 'Max Dose',     min: 50, max: 5000, step: 50, value: 200 },
  ],
};

const RENDER_API = 'https://lab.primals.eco/api/v1/render';
const VALIDATION_API = 'https://lab.primals.eco/api/v1/validation/chain';
const HEALTH_API = 'https://lab.primals.eco/api/v1/health';

const GRAMMAR_ENDPOINTS = {
  'ic50':    '/gonzales/dose-response',
  'pk':      '/gonzales/pk-decay',
  'tissue':  '/gonzales/tissue-lattice',
  'hormesis':'/anderson/hormesis',
  'species': '/anderson/cross-species',
};

const ACCESS_TIERS = {
  public:       { label: 'Public',       color: '#565f89', desc: 'Static JSON only' },
  visitor:      { label: 'Visitor',      color: '#9ece6a', desc: 'Read-only live science' },
  collaborator: { label: 'Collaborator', color: '#7aa2f7', desc: 'Parameter exploration + provenance drill-down' },
  owner:        { label: 'Owner',        color: '#bb9af7', desc: 'Full system access + vault admin' },
};

let staticCache = {};
let liveAvailable = null;
let useGrammarRenderer = false;
let currentAccessTier = 'public';

async function probeLive() {
  if (liveAvailable !== null) return liveAvailable;
  try {
    const resp = await fetch(LIVE_API.replace('/science', '/health'), {
      signal: AbortSignal.timeout(3000),
    });
    liveAvailable = resp.ok;
  } catch {
    liveAvailable = false;
  }
  return liveAvailable;
}

async function fetchLive(endpoint, params) {
  const url = new URL(LIVE_API + endpoint);
  if (params) {
    for (const [k, v] of Object.entries(params)) {
      url.searchParams.set(k, v);
    }
  }
  const resp = await fetch(url.toString(), { signal: AbortSignal.timeout(8000) });
  if (!resp.ok) throw new Error(`Live API ${resp.status}`);
  return resp.json();
}

async function fetchStatic(file) {
  if (staticCache[file]) return staticCache[file];
  const resp = await fetch(DATA_DIR + file);
  if (!resp.ok) throw new Error(`Failed to load ${file}: ${resp.status}`);
  const data = await resp.json();
  staticCache[file] = data;
  return data;
}

async function loadScenario(tabId, params) {
  const info = SCENARIOS[tabId];
  if (!info) throw new Error('Unknown scenario');

  const isLive = await probeLive();

  if (isLive && info.live) {
    try {
      const data = await fetchLive(info.live, params);
      data._source = 'live';
      return data;
    } catch {
      // fall through to static
    }
  }

  const data = await fetchStatic(info.file);
  data._source = 'static';
  return data;
}

// ── Rendering ────────────────────────────────────────────────────────

function renderTimeseries(container, channels, title) {
  const traces = channels.map((ch, i) => ({
    x: ch.x_values,
    y: ch.y_values,
    mode: 'lines',
    name: ch.label,
    line: { color: COLORS[i % COLORS.length], width: 2 },
    customdata: channels.map(() => ch.id),
  }));
  const layout = {
    ...DARK_LAYOUT,
    title: { text: title, font: { size: 14 } },
    xaxis: { ...DARK_LAYOUT.xaxis, title: channels[0]?.x_label || '' },
    yaxis: { ...DARK_LAYOUT.yaxis, title: channels[0]?.y_label || '' },
  };
  Plotly.newPlot(container, traces, layout, { responsive: true });
  container.on('plotly_click', (evt) => handlePointClick(evt, channels));
}

function renderBar(container, channel, title) {
  const trace = {
    x: channel.categories,
    y: channel.values,
    type: 'bar',
    marker: {
      color: channel.values.map((_, i) => COLORS[i % COLORS.length]),
      line: { color: 'rgba(255,255,255,0.2)', width: 1 },
    },
    text: channel.values.map(v => typeof v === 'number' ? v.toFixed(3) : v),
    textposition: 'outside',
    textfont: { color: '#c0caf5' },
  };
  const layout = {
    ...DARK_LAYOUT,
    title: { text: title || channel.label, font: { size: 14 } },
    yaxis: { ...DARK_LAYOUT.yaxis, title: channel.unit || '' },
  };
  Plotly.newPlot(container, [trace], layout, { responsive: true });
  container.on('plotly_click', (evt) => handleBarClick(evt, channel));
}

function renderGauge(container, channel) {
  const trace = {
    type: 'indicator',
    mode: 'gauge+number',
    value: channel.value,
    title: { text: channel.label, font: { size: 13, color: '#c0caf5' } },
    number: { suffix: ` ${channel.unit || ''}`, font: { color: '#c0caf5' } },
    gauge: {
      axis: { range: [channel.min, channel.max], tickcolor: '#565f89' },
      bar: { color: '#7aa2f7' },
      bgcolor: '#1a1b26',
      bordercolor: '#292e42',
      steps: [
        { range: channel.normal_range || [0, 0], color: 'rgba(158,206,106,0.2)' },
        { range: channel.warning_range || [0, 0], color: 'rgba(247,118,142,0.2)' },
      ],
    },
  };
  const layout = {
    ...DARK_LAYOUT,
    height: 250,
    margin: { t: 30, r: 20, b: 10, l: 20 },
  };
  Plotly.newPlot(container, [trace], layout, { responsive: true });
}

function renderScatter(container, channel, title) {
  const trace = {
    x: channel.x,
    y: channel.y,
    mode: 'markers+text',
    text: channel.point_labels || [],
    textposition: 'top center',
    textfont: { color: '#c0caf5', size: 11 },
    marker: {
      size: 12,
      color: channel.y.map((_, i) => COLORS[i % COLORS.length]),
      line: { color: 'rgba(255,255,255,0.3)', width: 1 },
    },
  };
  const layout = {
    ...DARK_LAYOUT,
    title: { text: title || channel.label, font: { size: 14 } },
    xaxis: { ...DARK_LAYOUT.xaxis, title: channel.x_label || '' },
    yaxis: { ...DARK_LAYOUT.yaxis, title: channel.y_label || '' },
  };
  Plotly.newPlot(container, [trace], layout, { responsive: true });
  container.on('plotly_click', (evt) => handleScatterClick(evt, channel));
}

function createDiv(parent, id, cls) {
  const div = document.createElement('div');
  if (id) div.id = id;
  if (cls) div.className = cls;
  parent.appendChild(div);
  return div;
}

// ── Source badge ──────────────────────────────────────────────────────

function renderSourceBadge(parent, source) {
  const badge = document.createElement('span');
  badge.className = source === 'live' ? 'source-badge live' : 'source-badge cached';
  badge.textContent = source === 'live' ? '● Live' : '○ Cached';
  badge.title = source === 'live'
    ? 'Data computed live by wetSpring on the HPC'
    : 'Pre-computed static data from guideStone';
  parent.appendChild(badge);
}

// ── Provenance panel ─────────────────────────────────────────────────

function renderProvenancePanel(parent, provenance) {
  if (!provenance) return;

  const details = document.createElement('details');
  details.className = 'provenance-panel';
  const summary = document.createElement('summary');
  summary.textContent = 'Provenance';
  const tierBadge = document.createElement('span');
  tierBadge.className = 'tier-badge';
  tierBadge.textContent = `Tier ${provenance.tier || 1}`;
  summary.appendChild(tierBadge);
  details.appendChild(summary);

  const content = document.createElement('div');
  content.className = 'provenance-content';

  // Tier 1 — always present
  if (provenance.guidestone) {
    addProvRow(content, 'guideStone', `${provenance.guidestone.version} — ${provenance.guidestone.validation}`);
  }
  if (provenance.wetspring) {
    addProvRow(content, 'wetSpring', `v${provenance.wetspring.version} @ ${provenance.wetspring.commit}`);
  }
  if (provenance.computation) {
    addProvRow(content, 'Method', provenance.computation.method);
    if (provenance.computation.content_hash) {
      addProvRow(content, 'BLAKE3', provenance.computation.content_hash.slice(0, 16) + '…');
    }
    addProvRow(content, 'Timestamp', provenance.computation.timestamp);
  }

  // Trio (witnesses model — falls back to legacy tier2 shape)
  const trio = provenance.trio || provenance.tier2;
  if (trio) {
    addProvRow(content, 'rhizoCrypt', trio.rhizocrypt_session);
    addProvRow(content, 'loamSpine', trio.loamspine_commit);
    addProvRow(content, 'sweetGrass', trio.sweetgrass_braid);
    if (trio.merkle_root) {
      addProvRow(content, 'Merkle', trio.merkle_root.slice(0, 16) + '…');
    }
  }

  // Witnesses
  if (provenance.witnesses && provenance.witnesses.length > 0) {
    const wCount = provenance.witnesses.length;
    const kinds = [...new Set(provenance.witnesses.map(w => w.kind))];
    addProvRow(content, 'Witnesses', `${wCount} (${kinds.join(', ')})`);
  }

  // Tier 3
  if (provenance.tier3) {
    const verifyBtn = document.createElement('button');
    verifyBtn.className = 'verify-btn';
    verifyBtn.textContent = 'Verify ↗';
    verifyBtn.onclick = () => window.open(provenance.tier3.verify_url, '_blank');
    content.appendChild(verifyBtn);
  }

  details.appendChild(content);
  parent.appendChild(details);
}

function addProvRow(parent, label, value) {
  if (!value) return;
  const row = document.createElement('div');
  row.className = 'prov-row';
  row.innerHTML = `<span class="prov-label">${label}</span><span class="prov-value">${value}</span>`;
  parent.appendChild(row);
}

// ── Lineage click handlers ───────────────────────────────────────────

let currentProvenance = null;

function handlePointClick(evt, channels) {
  if (!evt.points || evt.points.length === 0) return;
  const pt = evt.points[0];
  showLineagePopup({
    type: 'timeseries',
    channel: channels[pt.curveNumber]?.label || 'Unknown',
    x: pt.x,
    y: pt.y,
    index: pt.pointIndex,
  });
}

function handleBarClick(evt, channel) {
  if (!evt.points || evt.points.length === 0) return;
  const pt = evt.points[0];
  showLineagePopup({
    type: 'bar',
    channel: channel.label,
    category: channel.categories[pt.pointIndex],
    value: pt.y,
  });
}

function handleScatterClick(evt, channel) {
  if (!evt.points || evt.points.length === 0) return;
  const pt = evt.points[0];
  showLineagePopup({
    type: 'scatter',
    channel: channel.label,
    label: channel.point_labels?.[pt.pointIndex] || '',
    x: pt.x,
    y: pt.y,
  });
}

const COMPOSITION_API = 'https://lab.primals.eco/api/v1/system/composition';

function showLineagePopup(pointInfo) {
  let popup = document.getElementById('lineage-popup');
  if (!popup) {
    popup = document.createElement('div');
    popup.id = 'lineage-popup';
    popup.className = 'lineage-popup';
    document.body.appendChild(popup);
  }

  let html = '<div class="lineage-header">Data Lineage</div>';
  html += '<div class="lineage-body">';
  html += `<div class="lineage-row"><strong>Channel:</strong> ${pointInfo.channel}</div>`;

  if (pointInfo.type === 'timeseries') {
    html += `<div class="lineage-row"><strong>Point:</strong> x=${fmt(pointInfo.x)}, y=${fmt(pointInfo.y)}</div>`;
  } else if (pointInfo.type === 'bar') {
    html += `<div class="lineage-row"><strong>Category:</strong> ${pointInfo.category}</div>`;
    html += `<div class="lineage-row"><strong>Value:</strong> ${fmt(pointInfo.value)}</div>`;
  } else if (pointInfo.type === 'scatter') {
    html += `<div class="lineage-row"><strong>Subject:</strong> ${pointInfo.label}</div>`;
    html += `<div class="lineage-row"><strong>Position:</strong> (${fmt(pointInfo.x)}, ${fmt(pointInfo.y)})</div>`;
  }

  if (currentProvenance) {
    html += '<hr class="lineage-divider">';
    if (currentProvenance.computation) {
      html += `<div class="lineage-row"><strong>Computed by:</strong> ${currentProvenance.computation.method}</div>`;
      if (currentProvenance.computation.content_hash) {
        html += `<div class="lineage-row"><strong>Hash:</strong> <code>${currentProvenance.computation.content_hash.slice(0, 20)}…</code></div>`;
      }
    }
    if (currentProvenance.guidestone) {
      html += `<div class="lineage-row"><strong>Validated:</strong> ${currentProvenance.guidestone.validation}</div>`;
    }
    const trioData = currentProvenance.trio || currentProvenance.tier2;
    if (trioData) {
      html += `<div class="lineage-row"><strong>Session:</strong> ${trioData.rhizocrypt_session || 'N/A'}</div>`;
      html += `<div class="lineage-row"><strong>Ledger:</strong> ${trioData.loamspine_commit || 'N/A'}</div>`;
    }
    if (currentProvenance.tier3?.verify_url) {
      html += `<div class="lineage-row"><a href="${currentProvenance.tier3.verify_url}" target="_blank" class="lineage-verify">Verify full chain ↗</a></div>`;
    }

    if (currentProvenance.nft_vertex) {
      const vtx = currentProvenance.nft_vertex;
      html += '<hr class="lineage-divider">';
      html += '<div class="lineage-row"><strong>NFT Vertex</strong></div>';
      html += `<div class="lineage-row"><code>${(vtx.vertex_id || '').slice(0, 24)}…</code></div>`;
      if (vtx.license) {
        html += `<div class="lineage-row"><strong>License:</strong> ${vtx.license.code} / ${vtx.license.data_model} / ${vtx.license.content}</div>`;
      }
    }

    html += '<hr class="lineage-divider">';
    html += `<button class="reproduce-btn" onclick="showReproducePanel()">Reproduce this result</button>`;
  }

  html += '</div>';
  html += '<button class="lineage-close" onclick="this.parentElement.style.display=\'none\'">✕</button>';

  popup.innerHTML = html;
  popup.style.display = 'block';
}

function showReproducePanel() {
  let panel = document.getElementById('reproduce-panel');
  if (!panel) {
    panel = document.createElement('div');
    panel.id = 'reproduce-panel';
    panel.className = 'reproduce-panel';
    document.body.appendChild(panel);
  }

  const repro = currentProvenance?.reproduction;
  const vertex = currentProvenance?.nft_vertex;
  const method = currentProvenance?.computation?.method || 'unknown';
  const params = currentProvenance?.computation?.params;
  const endpoint = repro?.recompute?.endpoint || `/api/v1/science/${method.replace(/\./g, '/')}`;

  let paramStr = '';
  if (params && typeof params === 'object') {
    paramStr = Object.entries(params)
      .map(([k, v]) => `${k}=${v}`)
      .join('&');
  }
  const curlCmd = paramStr
    ? `curl "http://localhost:3100${endpoint}?${paramStr}"`
    : `curl "http://localhost:3100${endpoint}"`;

  let html = '<div class="lineage-header">Reproduce This Result</div>';
  html += '<div class="lineage-body">';

  html += '<div class="reproduce-step">';
  html += '<div class="reproduce-step-num">1</div>';
  html += '<div class="reproduce-step-body">';
  html += '<strong>Fetch primals</strong>';
  html += `<code class="reproduce-cmd">${repro?.fetch_command || 'cd plasmidBin && ./fetch.sh --tag v0.7.0'}</code>`;
  html += '</div></div>';

  html += '<div class="reproduce-step">';
  html += '<div class="reproduce-step-num">2</div>';
  html += '<div class="reproduce-step-body">';
  html += '<strong>Deploy the NUCLEUS graph</strong>';
  html += `<code class="reproduce-cmd">${repro?.deploy_command || 'biomeos deploy --graph graphs/wetspring_science_nucleus.toml'}</code>`;
  html += '</div></div>';

  html += '<div class="reproduce-step">';
  html += '<div class="reproduce-step-num">3</div>';
  html += '<div class="reproduce-step-body">';
  html += '<strong>Recompute with identical params</strong>';
  html += `<code class="reproduce-cmd">${curlCmd}</code>`;
  html += '</div></div>';

  html += '<div class="reproduce-step">';
  html += '<div class="reproduce-step-num">4</div>';
  html += '<div class="reproduce-step-body">';
  html += '<strong>Verify BLAKE3 hash matches</strong>';
  const hash = currentProvenance?.computation?.content_hash;
  html += `<code class="reproduce-cmd">Expected: ${hash ? hash.slice(0, 32) + '…' : 'N/A'}</code>`;
  html += '</div></div>';

  if (vertex) {
    html += '<hr class="lineage-divider">';
    html += '<div class="reproduce-nft">';
    html += '<strong>gAIa Novel Ferment Transcript</strong>';
    html += `<div class="lineage-row"><span class="prov-label">Vertex</span><code class="prov-value">${(vertex.vertex_id || '').slice(0, 32)}…</code></div>`;
    html += `<div class="lineage-row"><span class="prov-label">License</span><span class="prov-value">${vertex.license?.code || 'AGPL-3.0'} / ${vertex.license?.data_model || 'ORC'} / ${vertex.license?.content || 'CC-BY-SA-4.0'}</span></div>`;
    html += '<div class="reproduce-nft-note">This vertex records the computation in the gAIa commons. Value from verifiable history, not scarcity.</div>';
    html += '</div>';
  }

  html += '<div class="reproduce-links">';
  html += '<a href="https://github.com/ecoPrimals/plasmidBin" target="_blank" class="lineage-verify">plasmidBin setup ↗</a>';
  html += '</div>';

  html += '</div>';
  html += `<button class="lineage-close" onclick="document.getElementById('reproduce-panel').style.display='none'">✕</button>`;

  panel.innerHTML = html;
  panel.style.display = 'block';
}

function fmt(v) {
  return typeof v === 'number' ? v.toFixed(4) : String(v);
}

// ── Node and scenario rendering ──────────────────────────────────────

function renderNode(parent, node) {
  const section = createDiv(parent, null, 'node-section');
  const h3 = document.createElement('h3');
  h3.textContent = node.label || node.id;
  section.appendChild(h3);

  const timeseries = node.data_channels.filter(c => c.channel_type === 'timeseries');
  const bars       = node.data_channels.filter(c => c.channel_type === 'bar');
  const gauges     = node.data_channels.filter(c => c.channel_type === 'gauge');
  const scatters   = node.data_channels.filter(c => c.channel_type === 'scatter');

  if (timeseries.length > 0) {
    const div = createDiv(section, `ts-${node.id}`, 'chart');
    renderTimeseries(div, timeseries, node.label || node.id);
  }

  for (const ch of bars) {
    const div = createDiv(section, `bar-${ch.id}`, 'chart');
    renderBar(div, ch);
  }

  for (const ch of scatters) {
    const div = createDiv(section, `scatter-${ch.id}`, 'chart');
    renderScatter(div, ch);
  }

  if (gauges.length > 0) {
    const gaugeRow = createDiv(section, null, 'gauge-row');
    for (const ch of gauges) {
      const div = createDiv(gaugeRow, `gauge-${ch.id}`, 'gauge');
      renderGauge(div, ch);
    }
  }

  if (node.scientific_ranges && node.scientific_ranges.length > 0) {
    const rangeDiv = createDiv(section, null, 'ranges');
    for (const r of node.scientific_ranges) {
      const badge = document.createElement('span');
      badge.className = `range-badge range-${r.status}`;
      badge.textContent = r.label;
      rangeDiv.appendChild(badge);
    }
  }
}

function renderScenario(container, scenario, nodeFilter) {
  container.innerHTML = '';

  const headerRow = createDiv(container, null, 'scenario-header');
  const desc = document.createElement('p');
  desc.className = 'scenario-desc';
  desc.textContent = scenario.description || '';
  headerRow.appendChild(desc);

  renderSourceBadge(headerRow, scenario._source || 'static');

  currentProvenance = scenario.provenance || null;
  renderProvenancePanel(container, currentProvenance);

  let nodes = scenario.nodes || [];
  if (nodeFilter) nodes = nodes.filter(nodeFilter);
  for (const node of nodes) renderNode(container, node);
}

// ── Parameter sliders ────────────────────────────────────────────────

let currentSliderValues = {};
let currentTabId = null;

function renderSliders(parent, tabId, onChange) {
  const defs = SLIDER_DEFS[tabId];
  if (!defs || !liveAvailable) return;

  const panel = createDiv(parent, 'slider-panel', 'slider-panel');
  const title = document.createElement('div');
  title.className = 'slider-title';
  title.textContent = 'Parameters (live mode)';
  panel.appendChild(title);

  currentSliderValues = {};

  for (const def of defs) {
    currentSliderValues[def.param] = def.value;

    const row = createDiv(panel, null, 'slider-row');
    const label = document.createElement('label');
    label.textContent = def.label;
    label.className = 'slider-label';
    row.appendChild(label);

    const input = document.createElement('input');
    input.type = 'range';
    input.min = def.min;
    input.max = def.max;
    input.step = def.step;
    input.value = def.value;
    input.className = 'slider-input';
    row.appendChild(input);

    const valDisplay = document.createElement('span');
    valDisplay.className = 'slider-value';
    valDisplay.textContent = def.value;
    row.appendChild(valDisplay);

    input.addEventListener('input', () => {
      const v = parseFloat(input.value);
      currentSliderValues[def.param] = v;
      valDisplay.textContent = Number.isInteger(v) ? v : v.toFixed(2);
    });
    input.addEventListener('change', () => {
      onChange(currentSliderValues);
    });
  }
}

// ── Tab switching ────────────────────────────────────────────────────

async function switchTab(tabId) {
  const container = document.getElementById('explorer-content');
  if (!container) return;
  currentTabId = tabId;

  document.querySelectorAll('.tab-btn').forEach(b => b.classList.remove('active'));
  const activeBtn = document.querySelector(`[data-tab="${tabId}"]`);
  if (activeBtn) activeBtn.classList.add('active');

  container.innerHTML = '<p class="loading">Loading…</p>';

  const existing = document.getElementById('slider-panel');
  if (existing) existing.remove();

  try {
    if (useGrammarRenderer && liveAvailable) {
      const result = await fetchGrammarSvg(tabId, null);
      if (result && !result.error) {
        renderGrammarSvg(container, result);
        return;
      }
    }

    const scenario = await loadScenario(tabId, null);

    if (tabId === 'ic50') {
      renderScenario(container, scenario, n => n.id === 'gonzales_ic50');
    } else if (tabId === 'pk') {
      renderScenario(container, scenario, n => n.id === 'gonzales_pk');
    } else {
      renderScenario(container, scenario);
    }

    const tabs = document.getElementById('explorer-tabs');
    if (tabs) {
      renderSliders(tabs.parentElement, tabId, async (params) => {
        container.innerHTML = '<p class="loading">Updating…</p>';
        try {
          if (useGrammarRenderer && liveAvailable) {
            const result = await fetchGrammarSvg(tabId, params);
            if (result && !result.error) {
              renderGrammarSvg(container, result);
              return;
            }
          }
          const s = await loadScenario(tabId, params);
          if (tabId === 'ic50') {
            renderScenario(container, s, n => n.id === 'gonzales_ic50');
          } else if (tabId === 'pk') {
            renderScenario(container, s, n => n.id === 'gonzales_pk');
          } else {
            renderScenario(container, s);
          }
        } catch (err) {
          container.innerHTML = `<p class="error">Error: ${err.message}</p>`;
        }
      });
    }
  } catch (err) {
    container.innerHTML = `<p class="error">Error loading data: ${err.message}</p>`;
  }
}

// ── petalTongue grammar rendering ─────────────────────────────────────

async function fetchGrammarSvg(tabId, params) {
  const endpoint = GRAMMAR_ENDPOINTS[tabId];
  if (!endpoint) return null;

  try {
    const url = new URL(RENDER_API + endpoint);
    if (params) {
      for (const [k, v] of Object.entries(params)) url.searchParams.set(k, v);
    }
    const resp = await fetch(url.toString(), { signal: AbortSignal.timeout(15000) });
    if (!resp.ok) return null;
    return resp.json();
  } catch {
    return null;
  }
}

function renderGrammarSvg(container, result) {
  container.innerHTML = '';

  const headerRow = createDiv(container, null, 'scenario-header');
  const desc = document.createElement('p');
  desc.className = 'scenario-desc';
  desc.textContent = 'Server-side rendering via petalTongue grammar engine';
  headerRow.appendChild(desc);
  renderSourceBadge(headerRow, 'live');

  const section = createDiv(container, null, 'node-section');
  const h3 = document.createElement('h3');
  h3.textContent = result.id || 'petalTongue Render';
  section.appendChild(h3);

  if (result.svg) {
    const svgContainer = createDiv(section, null, 'svg-render');
    if (typeof result.svg === 'string') {
      svgContainer.innerHTML = result.svg;
    } else {
      svgContainer.innerHTML = JSON.stringify(result.svg);
    }
  }

  if (result.tufte_report) {
    const details = document.createElement('details');
    details.className = 'provenance-panel';
    const summary = document.createElement('summary');
    summary.textContent = 'Tufte Validation';
    details.appendChild(summary);
    const content = document.createElement('pre');
    content.style.cssText = 'font-size:0.75rem;color:#a9b1d6;white-space:pre-wrap;padding:0.5rem';
    content.textContent = JSON.stringify(result.tufte_report, null, 2);
    details.appendChild(content);
    section.appendChild(details);
  }

  const info = createDiv(section, null, 'prov-row');
  info.innerHTML = `<span class="prov-label">Nodes</span><span class="prov-value">${result.scene_nodes || '?'}</span>`;
  const info2 = createDiv(section, null, 'prov-row');
  info2.innerHTML = `<span class="prov-label">Primitives</span><span class="prov-value">${result.total_primitives || '?'}</span>`;
}

// ── Validation tab ────────────────────────────────────────────────────

async function loadValidationChain(paperId) {
  const container = document.getElementById('explorer-content');
  if (!container) return;

  container.innerHTML = '<p class="loading">Loading validation chain…</p>';

  try {
    const resp = await fetch(`${VALIDATION_API}/${paperId}`, {
      signal: AbortSignal.timeout(10000),
    });
    if (!resp.ok) throw new Error(`API ${resp.status}`);
    const data = await resp.json();
    renderValidationChain(container, data);
  } catch {
    container.innerHTML = `
      <div class="node-section">
        <h3>Validation Chain — ${paperId}</h3>
        <p class="scenario-desc">Live validation requires HPC connection. Showing reference data.</p>
        <div class="validation-stage">
          <span class="validation-icon">📄</span>
          <div class="validation-info">
            <strong>Source Paper</strong>
            <span class="prov-value">Gonzales AJ et al. 2014 — DOI: 10.1111/jvp.12065</span>
          </div>
          <span class="validation-status verified">Verified</span>
        </div>
        <div class="validation-stage">
          <span class="validation-icon">🐍</span>
          <div class="validation-info">
            <strong>Python Baseline</strong>
            <span class="prov-value">healthSpring/control/discovery/exp093_chembl_jak_panel.py</span>
          </div>
          <span class="validation-status pending">Pending Hash</span>
        </div>
        <div class="validation-stage">
          <span class="validation-icon">🦀</span>
          <div class="validation-info">
            <strong>Rust Validation</strong>
            <span class="prov-value">validate_gonzales_ic50_s79 — 35/35 PASS</span>
          </div>
          <span class="validation-status verified">Verified</span>
        </div>
        <div class="validation-stage">
          <span class="validation-icon">🪨</span>
          <div class="validation-info">
            <strong>guideStone</strong>
            <span class="prov-value">wetspring_gonzales_guidestone — 29/29 PASS</span>
          </div>
          <span class="validation-status verified">Verified</span>
        </div>
        <div class="validation-stage">
          <span class="validation-icon">🌐</span>
          <div class="validation-info">
            <strong>NUCLEUS Composition</strong>
            <span class="prov-value">Requires live HPC connection</span>
          </div>
          <span class="validation-status offline">Offline</span>
        </div>
      </div>`;
  }
}

function renderValidationChain(container, data) {
  container.innerHTML = '';
  const section = createDiv(container, null, 'node-section');

  const h3 = document.createElement('h3');
  h3.textContent = `Validation Chain — ${data.title || data.paper_id}`;
  section.appendChild(h3);

  if (data.doi) {
    const doi = document.createElement('p');
    doi.className = 'scenario-desc';
    doi.innerHTML = `DOI: <a href="https://doi.org/${data.doi}" target="_blank" style="color:#7aa2f7">${data.doi}</a> (${data.journal} ${data.year})`;
    section.appendChild(doi);
  }

  const chain = data.chain || {};
  const stages = [
    { key: 'source', icon: '📄', label: 'Source Paper' },
    { key: 'python_baseline', icon: '🐍', label: 'Python Baseline' },
    { key: 'rust_validation', icon: '🦀', label: 'Rust Validation' },
    { key: 'guidestone', icon: '🪨', label: 'guideStone' },
    { key: 'nucleus_composition', icon: '🌐', label: 'NUCLEUS Composition' },
  ];

  for (const stage of stages) {
    const stageData = chain[stage.key];
    const stageDiv = createDiv(section, null, 'validation-stage');

    const icon = document.createElement('span');
    icon.className = 'validation-icon';
    icon.textContent = stage.icon;
    stageDiv.appendChild(icon);

    const info = createDiv(stageDiv, null, 'validation-info');
    const title = document.createElement('strong');
    title.textContent = stage.label;
    info.appendChild(title);

    if (stageData) {
      const detail = document.createElement('span');
      detail.className = 'prov-value';
      if (stageData.doi) detail.textContent = `DOI: ${stageData.doi}`;
      else if (stageData.binary) detail.textContent = `${stageData.binary} — ${stageData.result || stageData.checks}`;
      else if (stageData.path) detail.textContent = stageData.path;
      else if (stageData.computation) detail.textContent = `Live: ${stageData.computation.method || 'active'}`;
      else detail.textContent = JSON.stringify(stageData).slice(0, 80);
      info.appendChild(detail);
    }

    const statusEl = document.createElement('span');
    const status = stageData?.status || 'unknown';
    statusEl.className = `validation-status ${status}`;
    statusEl.textContent = status.charAt(0).toUpperCase() + status.slice(1).replace('_', ' ');
    stageDiv.appendChild(statusEl);
  }

  if (data.provenance) {
    renderProvenancePanel(section, data.provenance);
  }
}

// ── Access tier UI ────────────────────────────────────────────────────

function renderAccessTierBadge(parent) {
  const tier = ACCESS_TIERS[currentAccessTier] || ACCESS_TIERS.public;
  const badge = document.createElement('span');
  badge.className = 'access-tier-badge';
  badge.style.cssText = `color:${tier.color};border-color:${tier.color}`;
  badge.textContent = tier.label;
  badge.title = tier.desc;
  parent.appendChild(badge);
}

function detectAccessTier() {
  if (!liveAvailable) {
    currentAccessTier = 'public';
  } else {
    currentAccessTier = 'visitor';
  }
}

// ── System status ─────────────────────────────────────────────────────

async function fetchSystemStatus() {
  try {
    const resp = await fetch(HEALTH_API, { signal: AbortSignal.timeout(5000) });
    if (!resp.ok) return null;
    return resp.json();
  } catch {
    return null;
  }
}

function renderSystemStatus(parent, status) {
  if (!status) return;

  const panel = createDiv(parent, 'system-status', 'system-status-panel');

  const header = document.createElement('div');
  header.className = 'status-header';
  header.textContent = 'System Status';
  panel.appendChild(header);

  const rows = [
    ['Facade', status.status || 'unknown'],
    ['Version', status.version || '?'],
    ['wetSpring IPC', status.wetspring_ipc || 'unknown'],
  ];

  for (const [label, value] of rows) {
    const row = createDiv(panel, null, 'prov-row');
    row.innerHTML = `<span class="prov-label">${label}</span><span class="prov-value status-${value}">${value}</span>`;
  }
}

// ── Init ─────────────────────────────────────────────────────────────

document.addEventListener('DOMContentLoaded', async () => {
  const tabs = document.getElementById('explorer-tabs');
  if (!tabs) return;

  await probeLive();
  detectAccessTier();

  for (const [id, info] of Object.entries(SCENARIOS)) {
    const btn = document.createElement('button');
    btn.className = 'tab-btn';
    btn.dataset.tab = id;
    btn.textContent = info.label;
    btn.addEventListener('click', () => switchTab(id));
    tabs.appendChild(btn);
  }

  const validationBtn = document.createElement('button');
  validationBtn.className = 'tab-btn validation-tab';
  validationBtn.dataset.tab = 'validation';
  validationBtn.textContent = 'Validation';
  validationBtn.addEventListener('click', () => {
    document.querySelectorAll('.tab-btn').forEach(b => b.classList.remove('active'));
    validationBtn.classList.add('active');
    currentTabId = 'validation';
    loadValidationChain('gonzales_2014');
  });
  tabs.appendChild(validationBtn);

  renderAccessTierBadge(tabs);

  if (liveAvailable) {
    const status = document.createElement('span');
    status.className = 'api-status live';
    status.textContent = '● HPC Connected';
    status.title = 'Live data from wetSpring via lab.primals.eco';
    tabs.appendChild(status);

    const toggle = document.createElement('button');
    toggle.className = 'tab-btn renderer-toggle';
    toggle.textContent = 'Plotly (client)';
    toggle.title = 'Toggle between Plotly.js (client) and petalTongue (server) rendering';
    toggle.addEventListener('click', () => {
      useGrammarRenderer = !useGrammarRenderer;
      toggle.textContent = useGrammarRenderer ? 'petalTongue (server)' : 'Plotly (client)';
      toggle.classList.toggle('active', useGrammarRenderer);
      if (currentTabId) switchTab(currentTabId);
    });
    tabs.appendChild(toggle);

    fetchSystemStatus().then(s => {
      if (s) {
        const statusArea = document.getElementById('explorer-content');
        if (statusArea && currentTabId === 'ic50') {
          renderSystemStatus(statusArea, s);
        }
      }
    });
  }

  switchTab('ic50');
});
