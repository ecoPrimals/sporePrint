// signal-pt.js — petalTongue bridge for signal.primals.eco
// Depends on: pt-bridge-core.js (PetalBridge, classifyOrganism, build* helpers)
(function() {
  'use strict';

  var pt = PetalBridge({
    wsUrl: 'wss://hud.primals.eco/ws',
    domain: 'signal',
    statusEl: 'pt-status',
    metricsEl: 'pt-metrics',
    maxHeight: 300,
    onConnect: function() { pt.requestMetrics(); }
  });

  // ═══════════════════════════════════════════════════════════
  // PANEL RENDERING — signal-specific data → petalTongue
  // ═══════════════════════════════════════════════════════════

  function renderPanels(dash, topo) {
    if (!pt.isConnected()) return;

    var clusters = dash.epitope_clusters || [];
    var entities = (topo && topo.entities) || [];
    var total = dash.total_requests || 1;

    // 1. Entity population bar
    if (entities.length > 1) {
      var cats = [], vals = [];
      entities.forEach(function(e) {
        if (e.total_requests > 5) {
          cats.push(e.label.replace(/ \(.*\)/, ''));
          vals.push(e.total_requests);
        }
      });
      pt.renderBinding({
        channel_type: 'bar', id: 'entity-population',
        label: 'Entity Population — Request Volume',
        categories: cats, values: vals, unit: 'requests'
      }, 'pt-entity-pop');
    }

    // 2. Fleet honesty gauge
    if (entities.length > 1) {
      var honestReqs = 0, totalReqs = 0;
      entities.forEach(function(e) {
        totalReqs += e.total_requests;
        if (e.is_honest) honestReqs += e.total_requests;
      });
      var pct = totalReqs > 0 ? Math.round(honestReqs / totalReqs * 1000) / 10 : 0;
      pt.renderBinding({
        channel_type: 'gauge', id: 'honesty',
        label: 'Fleet Honesty — % Honest by Volume',
        value: pct, min: 0, max: 100, unit: '%',
        normal_range: [80, 100], warning_range: [50, 80]
      }, 'pt-honesty');
    }

    // 3. Bingo cube classification
    var cube = dash.bingo_cube_summary;
    if (cube) {
      var ck = Object.keys(cube).sort(function(a, b) { return cube[b] - cube[a]; });
      pt.renderBinding({
        channel_type: 'bar', id: 'bingo',
        label: 'Bingo Cube — Behavioral Type Classification',
        categories: ck, values: ck.map(function(k) { return cube[k]; }), unit: 'IPs'
      }, 'pt-bingo');
    }

    // 4. Collision detection
    var coll = dash.collision_level2;
    if (coll) {
      var collK = Object.keys(coll).sort(function(a, b) { return coll[b] - coll[a]; });
      pt.renderBinding({
        channel_type: 'bar', id: 'collision',
        label: 'Collision Layer — Coordination Detection',
        categories: collK, values: collK.map(function(k) { return coll[k]; }), unit: 'IPs'
      }, 'pt-collision');
    }

    // 5. ASN infrastructure
    var asn = dash.asn_types;
    if (asn) {
      var ak = Object.keys(asn).sort(function(a, b) { return asn[b] - asn[a]; });
      pt.renderBinding({
        channel_type: 'bar', id: 'asn',
        label: 'ASN Infrastructure Classification',
        categories: ak, values: ak.map(function(k) { return asn[k]; }), unit: 'requests'
      }, 'pt-asn');
    }

    // 6. Path types
    var paths = dash.path_types;
    if (paths) {
      var pk = Object.keys(paths).sort(function(a, b) { return paths[b] - paths[a]; });
      pt.renderBinding({
        channel_type: 'bar', id: 'paths',
        label: 'Request Path Classification — What They Want',
        categories: pk, values: pk.map(function(k) { return paths[k]; }), unit: 'requests'
      }, 'pt-paths');
    }

    // 7. Fingerprint scatter
    if (entities.length > 1) {
      var xs = [], ys = [];
      entities.forEach(function(e) {
        if (e.total_requests > 5) { xs.push(e.ua_pool_size || 1); ys.push(e.unique_ips); }
      });
      if (xs.length > 2) {
        pt.renderBinding({
          channel_type: 'scatter', id: 'fp-scatter',
          label: 'Behavioral Fingerprint Space — UA Pool vs IP Count',
          x: xs, y: ys, unit: 'entities'
        }, 'pt-fp-scatter');
      }
    }

    // 8. Epitope feature matrix
    if (clusters.length > 1) {
      var hashes = clusters.map(function(c) { return c.hash.substring(0, 6); });
      var fvals = [];
      clusters.forEach(function(c) {
        fvals.push(c.ips); fvals.push(c.ua_pool_size);
        fvals.push(Math.round(c.blame_ratio * 100)); fvals.push(c.has_lang ? 100 : 0);
      });
      pt.renderBinding({
        channel_type: 'heatmap', id: 'epitope-mx',
        label: 'Epitope Feature Matrix — Conserved Behavioral Invariants',
        x_labels: ['ips', 'ua_pool', 'blame', 'has_lang'],
        y_labels: hashes, values: fvals, unit: 'value'
      }, 'pt-epitope-mx');
    }

    // 9. Geographic × entity heatmap
    var geo = dash.geography || [];
    if (geo.length > 0 && entities.length > 0) {
      var countries = geo.map(function(g) { return g.country; });
      var geoVals = [];
      var fleetPct = dash.fleet_pct / 100;
      countries.forEach(function(country) {
        var g = geo.find(function(x) { return x.country === country; });
        var reqs = g ? g.requests : 0;
        geoVals.push(Math.round(reqs * fleetPct));
        geoVals.push(Math.round(reqs * 0.15));
        geoVals.push(Math.round(reqs * (1 - fleetPct - 0.15)));
        geoVals.push(0);
      });
      pt.renderBinding({
        channel_type: 'heatmap', id: 'geo-mx',
        label: 'Geographic × Entity Type Distribution',
        x_labels: ['Fleet', 'Agentic', 'Human', 'Scanner'],
        y_labels: countries, values: geoVals, unit: 'requests'
      }, 'pt-geo-mx');
    }

    // 10. Anderson selectivity — live from firewall
    buildSelectivityPanel(pt);

    // ── Batch 1: migrated from hand-wired JS ──

    // 11. Timing CV gauge
    var timing = dash.timing || {};
    if (timing.cv !== undefined) {
      pt.renderBinding({
        channel_type: 'gauge', id: 'timing-cv',
        label: 'Timing CV — Coefficient of Variation',
        value: Math.round(timing.cv * 1000) / 1000, min: 0, max: 2, unit: 'CV',
        normal_range: [0.8, 2.0], warning_range: [0.15, 0.8]
      }, 'pt-timing-cv');
    }

    // 12. Timing interval gauge
    if (timing.avg_interval_ms) {
      pt.renderBinding({
        channel_type: 'gauge', id: 'timing-interval',
        label: 'Mean Inter-Request Interval',
        value: timing.avg_interval_ms, min: 0, max: 5000, unit: 'ms',
        normal_range: [1000, 5000], warning_range: [100, 1000]
      }, 'pt-timing-interval');
    }

    // 13. Top offenders bar
    var offenders = dash.top_offenders || [];
    if (offenders.length > 0) {
      var oL = [], oV = [];
      offenders.slice(0, 10).forEach(function(ip) {
        var label = ip.org ? ip.org.substring(0, 18) : ip.ip;
        if (ip.country) label += ' (' + ip.country + ')';
        oL.push(label);
        oV.push(ip.requests);
      });
      pt.renderBinding({
        channel_type: 'bar', id: 'offenders',
        label: 'Top Offenders — Request Volume',
        categories: oL, values: oV, unit: 'requests'
      }, 'pt-offenders');
    }

    // 14. Target repos bar
    var targets = dash.targets || [];
    if (targets.length > 0) {
      var tL = [], tV = [];
      var fakeRepos = ['batch-processor','queue-worker','proxy-cache','http-proxy','deploy-scripts','graph-engine','auth-service','load-balancer','rate-limiter','service-mesh'];
      targets.slice(0, 10).forEach(function(t) {
        var name = t.repo.replace(/.*\//, '');
        var isFake = fakeRepos.some(function(f) { return name.indexOf(f) >= 0; });
        var label = name.length > 16 ? name.substring(0, 16) : name;
        if (isFake) label += ' *';
        tL.push(label); tV.push(t.requests);
      });
      pt.renderBinding({
        channel_type: 'bar', id: 'targets',
        label: 'Targeted Repositories — What They Scrape',
        categories: tL, values: tV, unit: 'requests'
      }, 'pt-targets');
    }

    // 15. Geographic country bars
    if (geo.length > 0) {
      pt.renderBinding({
        channel_type: 'bar', id: 'geo',
        label: 'Geographic Distribution — Requests by Country',
        categories: geo.map(function(g) { return g.country; }),
        values: geo.map(function(g) { return g.requests; }), unit: 'requests'
      }, 'pt-geo');
    }

    // 16. Subnet concentration
    var subnets = dash.subnets || [];
    if (subnets.length > 0) {
      var sL = [], sV = [];
      subnets.slice(0, 8).forEach(function(s) { sL.push(s.subnet + ' (' + s.unique_ips + ')'); sV.push(s.requests); });
      pt.renderBinding({
        channel_type: 'bar', id: 'subnets',
        label: 'Network Block Concentration',
        categories: sL, values: sV, unit: 'requests'
      }, 'pt-subnets');
    }

    // 17. Comparative fingerprints heatmap
    var comp = (topo && topo.comparative_fingerprints) || [];
    if (comp.length > 1) {
      var compE = comp.map(function(c) { return c.entity; });
      var compV = [];
      comp.forEach(function(c) {
        compV.push(c.ips); compV.push(parseInt(c.chrome) || 0);
        compV.push(c.sec_fetch === 'present' ? 100 : 0); compV.push(c.honest ? 100 : 0);
      });
      pt.renderBinding({
        channel_type: 'heatmap', id: 'comparative',
        label: 'Comparative Behavioral Fingerprints',
        x_labels: ['IPs', 'Chrome', 'Sec-Fetch%', 'Honest'],
        y_labels: compE, values: compV, unit: 'value'
      }, 'pt-comparative');
    }

    // ── Shared panels (from pt-bridge-core.js) ──
    buildSummaryKpis(pt, dash);
    buildEpitopeSummary(pt, dash.epitope_summary || {});
    buildOrganismProfile(pt, clusters);
    buildEntityGauges(pt, entities);
    buildEntityDetail(pt, entities);
    buildEntityMatrix(pt, entities);
    buildAcceptLanguageDonut(pt, clusters);
  }

  // ═══════════════════════════════════════════════════════════
  // DATA LOOP
  // ═══════════════════════════════════════════════════════════

  function fetchAndRender() {
    var dashP = fetch('/dashboard.json', {cache: 'no-store'}).then(function(r) { return r.json(); });
    var topoP = fetch('/topology.json', {cache: 'no-store'}).then(function(r) { return r.json(); });
    Promise.all([dashP, topoP]).then(function(res) {
      if (pt.isConnected()) { renderPanels(res[0], res[1]); pt.requestMetrics(); }
    }).catch(function() {});
  }

  // ═══════════════════════════════════════════════════════════
  // MOUNT — unified DOM IDs
  // ═══════════════════════════════════════════════════════════

  function card(title, sub, id) {
    return '<div class="dash-card"><div class="dash-title">' + title + '</div>'
      + '<div class="dash-sub">' + sub + '</div>'
      + '<div id="' + id + '" class="pt-viz"></div></div>';
  }
  function fullCard(title, sub, id) {
    return '<div class="dash-card" style="grid-column:1/-1"><div class="dash-title">' + title + '</div>'
      + '<div class="dash-sub">' + sub + '</div>'
      + '<div id="' + id + '" class="pt-viz"></div></div>';
  }
  function sectionHeader(text) {
    return '<div class="dash-card" style="grid-column:1/-1"><div class="dash-title" style="font-size:0.85em;color:#6e7681">▸ ' + text + '</div></div>';
  }

  function mount() {
    var el = document.getElementById('pt-panels');
    if (!el) return;
    el.innerHTML = ''
      + '<div class="dash-card" style="grid-column:1/-1">'
      + '  <div style="display:flex;justify-content:space-between;align-items:center">'
      + '    <div class="dash-title">🔬 petalTongue Deep Analysis</div>'
      + '    <div><span id="pt-status" style="font-size:0.75em"></span> <span id="pt-metrics" style="margin-left:1em"></span></div>'
      + '  </div>'
      + '  <div class="dash-sub">Server-side rendering through petalTongue\'s grammar-of-graphics scene compiler. Each panel compiled to SVG by the Rust engine at hud.primals.eco.</div>'
      + '</div>'
      + fullCard('Live Signal', 'Key metrics at a glance — RPS, fleet IPs, human IPs, fleet %, machine confidence, organisms.', 'pt-summary-kpis')
      + card('Epitope Summary', 'Organism detection counts.', 'pt-epitope-summary')
      + card('Entity Population', 'Request volume by classified entity.', 'pt-entity-pop')
      + card('Fleet Honesty', 'Proportion of honest traffic by volume.', 'pt-honesty')
      + card('Bingo Cube', 'Behavioral type from multi-dimensional fingerprint cube.', 'pt-bingo')
      + card('Collision Detection', 'Coordination patterns — agentic vs fleet vs genuine.', 'pt-collision')
      + card('ASN Infrastructure', 'Datacenter vs cloud vs telecom vs self.', 'pt-asn')
      + card('Path Classification', 'What the fleet targets — blame, commit, src.', 'pt-paths')
      + card('Fingerprint Space', 'UA pool size vs IP count — clusters reveal teams.', 'pt-fp-scatter')
      + card('Epitope Matrix', 'Organisms × behavioral features — conserved invariants.', 'pt-epitope-mx')
      + card('Geographic × Entity', 'Country × entity type distribution.', 'pt-geo-mx')
      + card('Anderson Selectivity', 'S = max(P) − min(P) — live from firewall.', 'pt-relay-selectivity')
      + card('Per-Mode Permeability', 'Observed P per transport mode.', 'pt-relay-permeability')
      + '<div id="pt-relay-meta" style="grid-column:1/-1;padding:0 1em"></div>'
      + sectionHeader('Migrated from hand-wired JS → petalTongue grammar-of-graphics')
      + card('Timing — CV', 'Coefficient of variation. CV < 0.15 = machine.', 'pt-timing-cv')
      + card('Timing — Interval', 'Mean milliseconds between requests.', 'pt-timing-interval')
      + card('Top Offenders', 'Highest-volume sources ranked.', 'pt-offenders')
      + card('Target Repos', 'Which repos the fleet targets.', 'pt-targets')
      + card('Geographic Distribution', 'Requests per country.', 'pt-geo')
      + card('Network Blocks', 'Subnet concentration.', 'pt-subnets')
      + card('Comparative Fingerprints', 'Entity × behavioral feature matrix.', 'pt-comparative')
      + fullCard('Organism Profile', 'Per-organism behavioral breakdown — IPs, UA pool, blame%. Each facet is one fingerprint cluster.', 'pt-organism-profile')
      + sectionHeader('Per-entity + per-organism detail — behavioral decomposition (faceted)')
      + fullCard('Entity Timing Gauges', 'Per-entity timing CV. Low CV = machine. Each facet is one entity.', 'pt-entity-gauges')
      + fullCard('Entity Behavioral Profile', 'Per-entity Chrome Lag, Blame%, RPS. Each facet is one entity.', 'pt-entity-detail')
      + fullCard('Entity Behavioral Matrix', 'Full comparison — IPs, chrome lag, blame%, honesty, RPS.', 'pt-entity-matrix')
      + card('Accept-Language', 'Human vs machine tell — donut chart.', 'pt-organism-lang');
  }

  mount();
  pt.connect();
  setTimeout(fetchAndRender, 2000);
  setInterval(fetchAndRender, 15000);
})();
