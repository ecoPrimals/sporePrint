// SPDX-License-Identifier: AGPL-3.0-or-later
//
// JELLY STRING — evolution target: petalTongue server-rendered SVG + WASM.
// This client-side JS is scaffolding. The live path (petalTongue grammar
// renderer) already works when HPC is connected. Once petalTongue serves
// all science pages as server-rendered HTML/SVG, Plotly.js and this file
// are removed. The static JSON fallback remains as offline guideStone cache.
// See: wateringHole/petaltongue/SPOREPRINT_EVOLUTION_ROADMAP.md
//
// Gonzales Interactive Science Explorer
// Capability-based rendering with live API discovery + static JSON fallback.
//
// Depends on: config.js (must load first), Plotly.js

'use strict';

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

let staticCache = {};
let liveAvailable = null;
let useGrammarRenderer = false;
let currentAccessTier = 'public';
let currentProvenance = null;
let currentSliderValues = {};
let currentTabId = null;

// ── Data fetching ────────────────────────────────────────────────────

async function probeLive() {
  if (liveAvailable !== null) return liveAvailable;
  try {
    const resp = await fetch(ExplorerConfig.healthUrl(), {
      signal: AbortSignal.timeout(3000),
    });
    liveAvailable = resp.ok;
  } catch {
    liveAvailable = false;
  }
  return liveAvailable;
}

async function fetchLive(endpoint, params) {
  const url = new URL(ExplorerConfig.scienceUrl() + endpoint);
  if (params) {
    for (const [k, v] of Object.entries(params)) url.searchParams.set(k, v);
  }
  const resp = await fetch(url.toString(), { signal: AbortSignal.timeout(8000) });
  if (!resp.ok) throw new Error(`Live API ${resp.status}`);
  return resp.json();
}

async function fetchStatic(file) {
  if (staticCache[file]) return staticCache[file];
  const resp = await fetch(ExplorerConfig.DATA_DIR + file);
  if (!resp.ok) throw new Error(`Failed to load ${file}: ${resp.status}`);
  const data = await resp.json();
  staticCache[file] = data;
  return data;
}

async function loadScenario(tabId, params) {
  const info = ExplorerConfig.SCENARIOS[tabId];
  if (!info) throw new Error('Unknown scenario');
  const isLive = await probeLive();
  if (isLive && info.live) {
    try {
      const data = await fetchLive(info.live, params);
      data._source = 'live';
      return data;
    } catch { /* fall through to static */ }
  }
  const data = await fetchStatic(info.file);
  data._source = 'static';
  return data;
}

// ── Chart rendering ──────────────────────────────────────────────────

function renderTimeseries(container, channels, title) {
  const traces = channels.map((ch, i) => ({
    x: ch.x_values, y: ch.y_values, mode: 'lines',
    name: ch.label, line: { color: COLORS[i % COLORS.length], width: 2 },
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
    x: channel.categories, y: channel.values, type: 'bar',
    marker: { color: channel.values.map((_, i) => COLORS[i % COLORS.length]), line: { color: 'rgba(255,255,255,0.2)', width: 1 } },
    text: channel.values.map(v => typeof v === 'number' ? v.toFixed(3) : v),
    textposition: 'outside', textfont: { color: '#c0caf5' },
  };
  const layout = { ...DARK_LAYOUT, title: { text: title || channel.label, font: { size: 14 } }, yaxis: { ...DARK_LAYOUT.yaxis, title: channel.unit || '' } };
  Plotly.newPlot(container, [trace], layout, { responsive: true });
  container.on('plotly_click', (evt) => handleBarClick(evt, channel));
}

function renderGauge(container, channel) {
  const trace = {
    type: 'indicator', mode: 'gauge+number', value: channel.value,
    title: { text: channel.label, font: { size: 13, color: '#c0caf5' } },
    number: { suffix: ` ${channel.unit || ''}`, font: { color: '#c0caf5' } },
    gauge: {
      axis: { range: [channel.min, channel.max], tickcolor: '#565f89' },
      bar: { color: '#7aa2f7' }, bgcolor: '#1a1b26', bordercolor: '#292e42',
      steps: [
        { range: channel.normal_range || [0, 0], color: 'rgba(158,206,106,0.2)' },
        { range: channel.warning_range || [0, 0], color: 'rgba(247,118,142,0.2)' },
      ],
    },
  };
  Plotly.newPlot(container, [trace], { ...DARK_LAYOUT, height: 250, margin: { t: 30, r: 20, b: 10, l: 20 } }, { responsive: true });
}

function renderScatter(container, channel, title) {
  const trace = {
    x: channel.x, y: channel.y, mode: 'markers+text',
    text: channel.point_labels || [], textposition: 'top center',
    textfont: { color: '#c0caf5', size: 11 },
    marker: { size: 12, color: channel.y.map((_, i) => COLORS[i % COLORS.length]), line: { color: 'rgba(255,255,255,0.3)', width: 1 } },
  };
  const layout = { ...DARK_LAYOUT, title: { text: title || channel.label, font: { size: 14 } }, xaxis: { ...DARK_LAYOUT.xaxis, title: channel.x_label || '' }, yaxis: { ...DARK_LAYOUT.yaxis, title: channel.y_label || '' } };
  Plotly.newPlot(container, [trace], layout, { responsive: true });
  container.on('plotly_click', (evt) => handleScatterClick(evt, channel));
}

// ── DOM helpers ──────────────────────────────────────────────────────

function createDiv(parent, id, cls) {
  const div = document.createElement('div');
  if (id) div.id = id;
  if (cls) div.className = cls;
  parent.appendChild(div);
  return div;
}

function fmt(v) {
  return typeof v === 'number' ? v.toFixed(4) : String(v);
}

// ── Source badge + provenance ────────────────────────────────────────

function renderSourceBadge(parent, source) {
  const badge = document.createElement('span');
  badge.className = source === 'live' ? 'source-badge live' : 'source-badge cached';
  badge.textContent = source === 'live' ? '● Live' : '○ Cached';
  badge.title = source === 'live' ? 'Data computed live by wetSpring on the HPC' : 'Pre-computed static data from guideStone';
  parent.appendChild(badge);
}

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

  const content = createDiv(details, null, 'provenance-content');
  if (provenance.guidestone) addProvRow(content, 'guideStone', `${provenance.guidestone.version} — ${provenance.guidestone.validation}`);
  if (provenance.wetspring) addProvRow(content, 'wetSpring', `v${provenance.wetspring.version} @ ${provenance.wetspring.commit}`);
  if (provenance.computation) {
    addProvRow(content, 'Method', provenance.computation.method);
    if (provenance.computation.content_hash) addProvRow(content, 'BLAKE3', provenance.computation.content_hash.slice(0, 16) + '…');
    addProvRow(content, 'Timestamp', provenance.computation.timestamp);
  }
  const trio = provenance.trio || provenance.tier2;
  if (trio) {
    addProvRow(content, 'rhizoCrypt', trio.rhizocrypt_session);
    addProvRow(content, 'loamSpine', trio.loamspine_commit);
    addProvRow(content, 'sweetGrass', trio.sweetgrass_braid);
    if (trio.merkle_root) addProvRow(content, 'Merkle', trio.merkle_root.slice(0, 16) + '…');
  }
  if (provenance.witnesses && provenance.witnesses.length > 0) {
    const kinds = [...new Set(provenance.witnesses.map(w => w.kind))];
    addProvRow(content, 'Witnesses', `${provenance.witnesses.length} (${kinds.join(', ')})`);
  }
  if (provenance.tier3) {
    const btn = document.createElement('button');
    btn.className = 'verify-btn';
    btn.textContent = 'Verify ↗';
    btn.onclick = () => window.open(provenance.tier3.verify_url, '_blank');
    content.appendChild(btn);
  }
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

function handlePointClick(evt, channels) {
  if (!evt.points?.length) return;
  const pt = evt.points[0];
  showLineagePopup({ type: 'timeseries', channel: channels[pt.curveNumber]?.label || 'Unknown', x: pt.x, y: pt.y, index: pt.pointIndex });
}

function handleBarClick(evt, channel) {
  if (!evt.points?.length) return;
  const pt = evt.points[0];
  showLineagePopup({ type: 'bar', channel: channel.label, category: channel.categories[pt.pointIndex], value: pt.y });
}

function handleScatterClick(evt, channel) {
  if (!evt.points?.length) return;
  const pt = evt.points[0];
  showLineagePopup({ type: 'scatter', channel: channel.label, label: channel.point_labels?.[pt.pointIndex] || '', x: pt.x, y: pt.y });
}

function showLineagePopup(pointInfo) {
  let popup = document.getElementById('lineage-popup');
  if (!popup) { popup = document.createElement('div'); popup.id = 'lineage-popup'; popup.className = 'lineage-popup'; document.body.appendChild(popup); }

  let html = '<div class="lineage-header">Data Lineage</div><div class="lineage-body">';
  html += `<div class="lineage-row"><strong>Channel:</strong> ${pointInfo.channel}</div>`;
  if (pointInfo.type === 'timeseries') html += `<div class="lineage-row"><strong>Point:</strong> x=${fmt(pointInfo.x)}, y=${fmt(pointInfo.y)}</div>`;
  else if (pointInfo.type === 'bar') { html += `<div class="lineage-row"><strong>Category:</strong> ${pointInfo.category}</div>`; html += `<div class="lineage-row"><strong>Value:</strong> ${fmt(pointInfo.value)}</div>`; }
  else if (pointInfo.type === 'scatter') { html += `<div class="lineage-row"><strong>Subject:</strong> ${pointInfo.label}</div>`; html += `<div class="lineage-row"><strong>Position:</strong> (${fmt(pointInfo.x)}, ${fmt(pointInfo.y)})</div>`; }

  if (currentProvenance) {
    html += '<hr class="lineage-divider">';
    html += buildTraceToSource(currentProvenance);
    if (currentProvenance.computation) { html += `<div class="lineage-row"><strong>Computed by:</strong> ${currentProvenance.computation.method}</div>`; if (currentProvenance.computation.content_hash) html += `<div class="lineage-row"><strong>Hash:</strong> <code>${currentProvenance.computation.content_hash.slice(0, 20)}…</code></div>`; }
    if (currentProvenance.guidestone) html += `<div class="lineage-row"><strong>Validated:</strong> ${currentProvenance.guidestone.validation}</div>`;
    const trioData = currentProvenance.trio || currentProvenance.tier2;
    if (trioData) { html += `<div class="lineage-row"><strong>Session:</strong> ${trioData.rhizocrypt_session || 'N/A'}</div>`; html += `<div class="lineage-row"><strong>Ledger:</strong> ${trioData.loamspine_commit || 'N/A'}</div>`; if (trioData.merkle_root) html += `<div class="lineage-row"><strong>Merkle:</strong> <code>${trioData.merkle_root.slice(0, 20)}…</code></div>`; if (trioData.braid_id) html += `<div class="lineage-row"><strong>Braid:</strong> <code>${trioData.braid_id.slice(0, 20)}…</code></div>`; }
    if (currentProvenance.tier3?.verify_url) html += `<div class="lineage-row"><a href="${currentProvenance.tier3.verify_url}" target="_blank" class="lineage-verify">Verify full chain ↗</a></div>`;
    if (currentProvenance.witnesses?.length) { const wKinds = [...new Set(currentProvenance.witnesses.map(w => w.kind))]; html += `<div class="lineage-row"><strong>Witnesses:</strong> ${currentProvenance.witnesses.length} (${wKinds.join(', ')})</div>`; }
    if (currentProvenance.nft_vertex) { const vtx = currentProvenance.nft_vertex; html += '<hr class="lineage-divider"><div class="lineage-row"><strong>NFT Vertex</strong></div>'; html += `<div class="lineage-row"><code>${(vtx.vertex_id || '').slice(0, 24)}…</code></div>`; if (vtx.license) html += `<div class="lineage-row"><strong>License:</strong> ${vtx.license.code} / ${vtx.license.data_model} / ${vtx.license.content}</div>`; }
    html += '<hr class="lineage-divider">';
    html += `<button class="reproduce-btn" onclick="showReproducePanel()">Reproduce this result</button>`;
  }
  html += '</div><button class="lineage-close" onclick="this.parentElement.style.display=\'none\'">✕</button>';
  popup.innerHTML = html;
  popup.style.display = 'block';
}

function showReproducePanel() {
  let panel = document.getElementById('reproduce-panel');
  if (!panel) { panel = document.createElement('div'); panel.id = 'reproduce-panel'; panel.className = 'reproduce-panel'; document.body.appendChild(panel); }
  const repro = currentProvenance?.reproduction;
  const vertex = currentProvenance?.nft_vertex;
  const method = currentProvenance?.computation?.method || 'unknown';
  const params = currentProvenance?.computation?.params;
  const endpoint = repro?.recompute?.endpoint || `/api/v1/science/${method.replace(/\./g, '/')}`;
  let paramStr = params ? Object.entries(params).map(([k, v]) => `${k}=${v}`).join('&') : '';
  const curlCmd = paramStr ? `curl "http://localhost:3100${endpoint}?${paramStr}"` : `curl "http://localhost:3100${endpoint}"`;

  let html = '<div class="lineage-header">Reproduce This Result</div><div class="lineage-body">';
  html += buildReproStep(1, 'Fetch primals', repro?.fetch_command || 'cd plasmidBin && ./fetch.sh --tag v0.7.0');
  html += buildReproStep(2, 'Deploy the NUCLEUS graph', repro?.deploy_command || 'biomeos deploy --graph graphs/wetspring_science_nucleus.toml');
  html += buildReproStep(3, 'Recompute with identical params', curlCmd);
  html += buildReproStep(4, 'Verify BLAKE3 hash matches', `Expected: ${currentProvenance?.computation?.content_hash?.slice(0, 32) || 'N/A'}…`);
  if (vertex) { html += '<hr class="lineage-divider"><div class="reproduce-nft"><strong>gAIa Novel Ferment Transcript</strong>'; html += `<div class="lineage-row"><span class="prov-label">Vertex</span><code class="prov-value">${(vertex.vertex_id || '').slice(0, 32)}…</code></div>`; html += `<div class="lineage-row"><span class="prov-label">License</span><span class="prov-value">${vertex.license?.code || 'AGPL-3.0'} / ${vertex.license?.data_model || 'ORC'} / ${vertex.license?.content || 'CC-BY-SA-4.0'}</span></div>`; html += '<div class="reproduce-nft-note">This vertex records the computation in the gAIa commons.</div></div>'; }
  html += '<div class="reproduce-links"><a href="https://github.com/ecoPrimals/plasmidBin" target="_blank" class="lineage-verify">plasmidBin setup ↗</a></div>';
  html += `</div><button class="lineage-close" onclick="document.getElementById('reproduce-panel').style.display='none'">✕</button>`;
  panel.innerHTML = html;
  panel.style.display = 'block';
}

function buildReproStep(num, title, cmd) {
  return `<div class="reproduce-step"><div class="reproduce-step-num">${num}</div><div class="reproduce-step-body"><strong>${title}</strong><code class="reproduce-cmd">${cmd}</code></div></div>`;
}

function buildTraceToSource(provenance) {
  const method = provenance?.computation?.method || '';
  const ref = ExplorerConfig.REFERENCE_PAPERS[method.replace('science.', '')];
  if (!ref) return '';
  let html = '<div class="trace-source"><div class="lineage-row"><strong>Trace to Source</strong></div>';
  html += `<div class="lineage-row"><a href="https://doi.org/${ref.doi}" target="_blank" class="doi-link">DOI: ${ref.doi} ↗</a></div>`;
  html += `<div class="lineage-row">${ref.short}</div>`;
  if (ref.table) html += `<div class="lineage-row"><em>${ref.table}</em></div>`;
  if (ref.chembl) html += `<div class="lineage-row"><a href="https://www.ebi.ac.uk/chembl/compound_report_card/${ref.chembl}/" target="_blank" class="doi-link">ChEMBL: ${ref.chembl} ↗</a></div>`;
  if (ref.pubchem_cid) html += `<div class="lineage-row"><a href="https://pubchem.ncbi.nlm.nih.gov/compound/${ref.pubchem_cid}" target="_blank" class="doi-link">PubChem CID: ${ref.pubchem_cid} ↗</a></div>`;
  const trioData = provenance.trio || provenance.tier2;
  const chainStages = [...(ref.chain || [])];
  if (trioData?.merkle_root) chainStages.push('nucleus');
  html += '<div class="chain-steps">';
  for (const stage of chainStages) html += `<span class="chain-step chain-done">${ExplorerConfig.CHAIN_LABELS[stage] || stage}</span>`;
  if (!chainStages.includes('nucleus')) html += `<span class="chain-step chain-pending">${ExplorerConfig.CHAIN_LABELS.nucleus}</span>`;
  html += '</div></div>';
  return html;
}

// ── Scenario and node rendering ──────────────────────────────────────

function renderNode(parent, node) {
  const section = createDiv(parent, null, 'node-section');
  const h3 = document.createElement('h3');
  h3.textContent = node.label || node.id;
  section.appendChild(h3);

  const timeseries = node.data_channels.filter(c => c.channel_type === 'timeseries');
  const bars       = node.data_channels.filter(c => c.channel_type === 'bar');
  const gauges     = node.data_channels.filter(c => c.channel_type === 'gauge');
  const scatters   = node.data_channels.filter(c => c.channel_type === 'scatter');

  if (timeseries.length > 0) renderTimeseries(createDiv(section, `ts-${node.id}`, 'chart'), timeseries, node.label || node.id);
  for (const ch of bars) renderBar(createDiv(section, `bar-${ch.id}`, 'chart'), ch);
  for (const ch of scatters) renderScatter(createDiv(section, `scatter-${ch.id}`, 'chart'), ch);
  if (gauges.length > 0) { const row = createDiv(section, null, 'gauge-row'); for (const ch of gauges) renderGauge(createDiv(row, `gauge-${ch.id}`, 'gauge'), ch); }
  if (node.scientific_ranges?.length) { const rangeDiv = createDiv(section, null, 'ranges'); for (const r of node.scientific_ranges) { const badge = document.createElement('span'); badge.className = `range-badge range-${r.status}`; badge.textContent = r.label; rangeDiv.appendChild(badge); } }
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

function renderSliders(parent, tabId, onChange) {
  const defs = ExplorerConfig.SLIDER_DEFS[tabId];
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
    Object.assign(input, { type: 'range', min: def.min, max: def.max, step: def.step, value: def.value, className: 'slider-input' });
    row.appendChild(input);
    const valDisplay = document.createElement('span');
    valDisplay.className = 'slider-value';
    valDisplay.textContent = def.value;
    row.appendChild(valDisplay);
    input.addEventListener('input', () => { const v = parseFloat(input.value); currentSliderValues[def.param] = v; valDisplay.textContent = Number.isInteger(v) ? v : v.toFixed(2); });
    input.addEventListener('change', () => onChange(currentSliderValues));
  }
}

// ── Grammar rendering (petalTongue) ──────────────────────────────────

async function fetchGrammarSvg(tabId, params) {
  const endpoint = ExplorerConfig.GRAMMAR_ENDPOINTS[tabId];
  if (!endpoint) return null;
  try {
    const url = new URL(ExplorerConfig.renderUrl() + endpoint);
    if (params) for (const [k, v] of Object.entries(params)) url.searchParams.set(k, v);
    const resp = await fetch(url.toString(), { signal: AbortSignal.timeout(15000) });
    return resp.ok ? resp.json() : null;
  } catch { return null; }
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
  if (result.svg) { const svgContainer = createDiv(section, null, 'svg-render'); svgContainer.innerHTML = typeof result.svg === 'string' ? result.svg : JSON.stringify(result.svg); }
  if (result.tufte_report) { const details = document.createElement('details'); details.className = 'provenance-panel'; const s = document.createElement('summary'); s.textContent = 'Tufte Validation'; details.appendChild(s); const pre = document.createElement('pre'); pre.style.cssText = 'font-size:0.75rem;color:#a9b1d6;white-space:pre-wrap;padding:0.5rem'; pre.textContent = JSON.stringify(result.tufte_report, null, 2); details.appendChild(pre); section.appendChild(details); }
  const info = createDiv(section, null, 'prov-row');
  info.innerHTML = `<span class="prov-label">Nodes</span><span class="prov-value">${result.scene_nodes || '?'}</span>`;
  createDiv(section, null, 'prov-row').innerHTML = `<span class="prov-label">Primitives</span><span class="prov-value">${result.total_primitives || '?'}</span>`;
}

// ── Validation chain ─────────────────────────────────────────────────

async function loadValidationChain(paperId) {
  const container = document.getElementById('explorer-content');
  if (!container) return;
  container.innerHTML = '<p class="loading">Loading validation chain…</p>';
  try {
    const resp = await fetch(`${ExplorerConfig.validationUrl()}/${paperId}`, { signal: AbortSignal.timeout(10000) });
    if (!resp.ok) throw new Error(`API ${resp.status}`);
    renderValidationChain(container, await resp.json());
  } catch {
    container.innerHTML = buildOfflineValidationHtml(paperId);
  }
}

function buildOfflineValidationHtml() {
  const stages = [
    { icon: '📄', label: 'Source Paper', detail: 'Gonzales AJ et al. 2014 — DOI: 10.1111/jvp.12065', status: 'verified' },
    { icon: '🐍', label: 'Python Baseline', detail: 'healthSpring/control/discovery/exp093_chembl_jak_panel.py', status: 'pending' },
    { icon: '🦀', label: 'Rust Validation', detail: 'validate_gonzales_ic50_s79 — 35/35 PASS', status: 'verified' },
    { icon: '🪨', label: 'guideStone', detail: 'wetspring_gonzales_guidestone — 29/29 PASS', status: 'verified' },
    { icon: '🌐', label: 'NUCLEUS Composition', detail: 'Requires live HPC connection', status: 'offline' },
  ];
  let html = '<div class="node-section"><h3>Validation Chain</h3><p class="scenario-desc">Live validation requires HPC connection. Showing reference data.</p>';
  for (const s of stages) html += `<div class="validation-stage"><span class="validation-icon">${s.icon}</span><div class="validation-info"><strong>${s.label}</strong><span class="prov-value">${s.detail}</span></div><span class="validation-status ${s.status}">${s.status.charAt(0).toUpperCase() + s.status.slice(1)}</span></div>`;
  return html + '</div>';
}

function renderValidationChain(container, data) {
  container.innerHTML = '';
  const section = createDiv(container, null, 'node-section');
  const h3 = document.createElement('h3');
  h3.textContent = `Validation Chain — ${data.title || data.paper_id}`;
  section.appendChild(h3);
  if (data.doi) { const doi = document.createElement('p'); doi.className = 'scenario-desc'; doi.innerHTML = `DOI: <a href="https://doi.org/${data.doi}" target="_blank" style="color:#7aa2f7">${data.doi}</a> (${data.journal} ${data.year})`; section.appendChild(doi); }
  const chain = data.chain || {};
  const stageKeys = [{ key: 'source', icon: '📄', label: 'Source Paper' }, { key: 'python_baseline', icon: '🐍', label: 'Python Baseline' }, { key: 'rust_validation', icon: '🦀', label: 'Rust Validation' }, { key: 'guidestone', icon: '🪨', label: 'guideStone' }, { key: 'nucleus_composition', icon: '🌐', label: 'NUCLEUS Composition' }];
  for (const stage of stageKeys) {
    const stageData = chain[stage.key];
    const stageDiv = createDiv(section, null, 'validation-stage');
    const icon = document.createElement('span'); icon.className = 'validation-icon'; icon.textContent = stage.icon; stageDiv.appendChild(icon);
    const info = createDiv(stageDiv, null, 'validation-info');
    const t = document.createElement('strong'); t.textContent = stage.label; info.appendChild(t);
    if (stageData) { const detail = document.createElement('span'); detail.className = 'prov-value'; detail.textContent = stageData.doi ? `DOI: ${stageData.doi}` : stageData.binary ? `${stageData.binary} — ${stageData.result || stageData.checks}` : stageData.path || JSON.stringify(stageData).slice(0, 80); info.appendChild(detail); }
    const statusEl = document.createElement('span'); const status = stageData?.status || 'unknown'; statusEl.className = `validation-status ${status}`; statusEl.textContent = status.charAt(0).toUpperCase() + status.slice(1); stageDiv.appendChild(statusEl);
  }
  if (data.provenance) renderProvenancePanel(section, data.provenance);
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
    if (useGrammarRenderer && liveAvailable) { const result = await fetchGrammarSvg(tabId, null); if (result && !result.error) { renderGrammarSvg(container, result); return; } }
    const scenario = await loadScenario(tabId, null);
    const filter = tabId === 'ic50' ? n => n.id === 'gonzales_ic50' : tabId === 'pk' ? n => n.id === 'gonzales_pk' : null;
    renderScenario(container, scenario, filter);
    const tabs = document.getElementById('explorer-tabs');
    if (tabs) renderSliders(tabs.parentElement, tabId, async (params) => {
      container.innerHTML = '<p class="loading">Updating…</p>';
      try {
        if (useGrammarRenderer && liveAvailable) { const r = await fetchGrammarSvg(tabId, params); if (r && !r.error) { renderGrammarSvg(container, r); return; } }
        const s = await loadScenario(tabId, params);
        renderScenario(container, s, filter);
      } catch (err) { container.innerHTML = `<p class="error">Error: ${err.message}</p>`; }
    });
  } catch (err) { container.innerHTML = `<p class="error">Error loading data: ${err.message}</p>`; }
}

// ── Init ─────────────────────────────────────────────────────────────

document.addEventListener('DOMContentLoaded', async () => {
  const tabs = document.getElementById('explorer-tabs');
  if (!tabs) return;

  await probeLive();
  currentAccessTier = liveAvailable ? 'visitor' : 'public';

  for (const [id, info] of Object.entries(ExplorerConfig.SCENARIOS)) {
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
  validationBtn.addEventListener('click', () => { document.querySelectorAll('.tab-btn').forEach(b => b.classList.remove('active')); validationBtn.classList.add('active'); currentTabId = 'validation'; loadValidationChain('gonzales_2014'); });
  tabs.appendChild(validationBtn);

  const tierInfo = ExplorerConfig.ACCESS_TIERS[currentAccessTier];
  const tierBadge = document.createElement('span');
  tierBadge.className = 'access-tier-badge';
  tierBadge.style.cssText = `color:${tierInfo.color};border-color:${tierInfo.color}`;
  tierBadge.textContent = tierInfo.label;
  tierBadge.title = tierInfo.desc;
  tabs.appendChild(tierBadge);

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
    toggle.addEventListener('click', () => { useGrammarRenderer = !useGrammarRenderer; toggle.textContent = useGrammarRenderer ? 'petalTongue (server)' : 'Plotly (client)'; toggle.classList.toggle('active', useGrammarRenderer); if (currentTabId) switchTab(currentTabId); });
    tabs.appendChild(toggle);
    fetch(ExplorerConfig.healthUrl(), { signal: AbortSignal.timeout(5000) }).then(r => r.ok ? r.json() : null).then(s => { if (s && currentTabId === 'ic50') { const area = document.getElementById('explorer-content'); if (area) { const panel = createDiv(area, 'system-status', 'system-status-panel'); panel.innerHTML = `<div class="status-header">System Status</div>`; for (const [l, v] of [['Facade', s.status], ['Version', s.version], ['wetSpring IPC', s.wetspring_ipc]]) { createDiv(panel, null, 'prov-row').innerHTML = `<span class="prov-label">${l}</span><span class="prov-value status-${v || 'unknown'}">${v || 'unknown'}</span>`; } } } }).catch(() => {});
  }

  switchTab('ic50');
});
