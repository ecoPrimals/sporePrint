+++
title = "Observatory"
description = "Live signal data from the membrane receptor — who visits, what crawls, how the network breathes. No cookies, no tracking pixels, no third-party analytics. Pure server-side log analysis."
template = "observatory.html"
+++

This page renders live data from **membrane's seo.receptor** — a log-based
analytics system that classifies visitors from Caddy access logs without
any client-side tracking.

## How It Works

1. Every HTTP request is logged by [Caddy](https://caddyserver.com/) in structured JSON format
2. The `membrane seo.receptor` command parses these logs via SSH, classifying each visitor by user-agent
3. On each `site.publish`, a receptor snapshot is written to `observatory/data.json`
4. This page reads that snapshot and renders it — no external calls, no cookies, no JavaScript analytics libraries

**Privacy model**: The receptor reads server access logs that already exist.
No additional data is collected. No cookies are set. No tracking pixels are loaded.
No third-party services are contacted. Visitor IP addresses are not stored in the snapshot —
only aggregate page-level counts.

### Visitor Classification

| Class | Description |
|-------|-------------|
| 🧬 Human | Standard browser User-Agents (Chrome, Firefox, Safari) |
| 🔍 Search Bot | Googlebot, Bingbot, DuckDuckBot, Yandex, Baidu |
| 🤖 AI Bot | GPTBot, ClaudeBot, Anthropic, PerplexityBot |
| 🔗 Social Bot | Twitterbot, facebookexternalhit, LinkedInBot |
| 📡 Other Bot | Monitoring, SEO tools, generic crawlers |
