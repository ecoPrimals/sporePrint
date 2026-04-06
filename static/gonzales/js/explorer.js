// SPDX-License-Identifier: AGPL-3.0-or-later
// Gonzales Interactive Science Explorer
// Reads petalTongue scenario JSON and renders Plotly.js charts.

const DATA_DIR = '/gonzales/data/';

const SCENARIOS = {
  'ic50':    { file: 'gonzales_dermatitis.json', label: 'IC50 Dose-Response' },
  'pk':      { file: 'gonzales_dermatitis.json', label: 'PK Decay' },
  'tissue':  { file: 'tissue_geometry.json',     label: 'Tissue Geometry' },
  'hormesis':{ file: 'hormesis.json',            label: 'Hormesis' },
  'species': { file: 'cross_species.json',       label: 'Cross-Species' },
  'full':    { file: 'full_gonzales.json',        label: 'Full Dashboard' },
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

let cache = {};

async function loadScenario(file) {
  if (cache[file]) return cache[file];
  const resp = await fetch(DATA_DIR + file);
  if (!resp.ok) throw new Error(`Failed to load ${file}: ${resp.status}`);
  const data = await resp.json();
  cache[file] = data;
  return data;
}

function renderTimeseries(container, channels, title) {
  const traces = channels.map((ch, i) => ({
    x: ch.x_values,
    y: ch.y_values,
    mode: 'lines',
    name: ch.label,
    line: { color: COLORS[i % COLORS.length], width: 2 },
  }));
  const layout = {
    ...DARK_LAYOUT,
    title: { text: title, font: { size: 14 } },
    xaxis: { ...DARK_LAYOUT.xaxis, title: channels[0]?.x_label || '' },
    yaxis: { ...DARK_LAYOUT.yaxis, title: channels[0]?.y_label || '' },
  };
  Plotly.newPlot(container, traces, layout, { responsive: true });
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
    text: channel.values.map(v => `${v} ${channel.unit || ''}`),
    textposition: 'outside',
    textfont: { color: '#c0caf5' },
  };
  const layout = {
    ...DARK_LAYOUT,
    title: { text: title || channel.label, font: { size: 14 } },
    yaxis: { ...DARK_LAYOUT.yaxis, title: channel.unit || '' },
  };
  Plotly.newPlot(container, [trace], layout, { responsive: true });
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
}

function createDiv(parent, id, cls) {
  const div = document.createElement('div');
  if (id) div.id = id;
  if (cls) div.className = cls;
  parent.appendChild(div);
  return div;
}

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

  const header = document.createElement('p');
  header.className = 'scenario-desc';
  header.textContent = scenario.description || '';
  container.appendChild(header);

  let nodes = scenario.nodes || [];
  if (nodeFilter) nodes = nodes.filter(nodeFilter);

  for (const node of nodes) renderNode(container, node);
}

async function switchTab(tabId) {
  const container = document.getElementById('explorer-content');
  if (!container) return;

  document.querySelectorAll('.tab-btn').forEach(b => b.classList.remove('active'));
  const activeBtn = document.querySelector(`[data-tab="${tabId}"]`);
  if (activeBtn) activeBtn.classList.add('active');

  container.innerHTML = '<p class="loading">Loading scenario data...</p>';

  try {
    const info = SCENARIOS[tabId];
    if (!info) { container.innerHTML = '<p>Unknown tab.</p>'; return; }

    const scenario = await loadScenario(info.file);

    if (tabId === 'ic50') {
      renderScenario(container, scenario, n => n.id === 'gonzales_ic50');
    } else if (tabId === 'pk') {
      renderScenario(container, scenario, n => n.id === 'gonzales_pk');
    } else {
      renderScenario(container, scenario);
    }
  } catch (err) {
    container.innerHTML = `<p class="error">Error loading data: ${err.message}</p>`;
  }
}

document.addEventListener('DOMContentLoaded', () => {
  const tabs = document.getElementById('explorer-tabs');
  if (!tabs) return;

  for (const [id, info] of Object.entries(SCENARIOS)) {
    const btn = document.createElement('button');
    btn.className = 'tab-btn';
    btn.dataset.tab = id;
    btn.textContent = info.label;
    btn.addEventListener('click', () => switchTab(id));
    tabs.appendChild(btn);
  }

  switchTab('ic50');
});
