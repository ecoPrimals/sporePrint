+++
title = "Signal Sensing Without Surveillance — The Receptor Model"
description = "A cookieless, trackingless methodology for measuring whether oversight signals propagate through institutional systems. Biological quorum sensing applied to public-interest web infrastructure."
date = 2026-10-04
weight = 5

[taxonomies]
primals = ["cellmembrane", "skunkbat"]

[extra]
domain = "Methodology"
+++

## Abstract

Every web analytics platform answers the same question: **who is watching?**
Google Analytics, Hotjar, Mixpanel, Facebook Pixel — they identify visitors,
track sessions across sites, build behavioral profiles, and serve that data
to content operators for optimization. The entire industry assumes that
measuring audience requires surveillance.

We reject that assumption.

The **receptor model** answers a fundamentally different question:
**did the signal propagate?** Not who received it. Not why they came.
Not where they went next. Just: did the oversight signal reach a human,
and did the system conduct it?

This is not a privacy-preserving version of analytics. It is a
categorically different measurement. The distinction is biological.

---

## The Surveillance Model (Industry Standard)

Traditional web analytics operates on a surveillance architecture:

| Layer | Technique | What It Captures |
|-------|-----------|-----------------|
| **Identity** | Cookies, fingerprinting, login state | WHO is visiting |
| **Tracking** | Cross-site pixels, referrer chains | WHERE they came from, where they go |
| **Profiling** | Session recording, heatmaps, click paths | WHAT they do, how long, how deep |
| **Attribution** | UTM parameters, conversion funnels | WHY they came (which campaign drove them) |
| **Monetization** | Behavioral segments, lookalike audiences | HOW to target them with ads |

This architecture requires:
- JavaScript execution on the client (tracking scripts)
- Persistent identifiers (cookies, localStorage, fingerprints)
- Third-party data exfiltration (beacons to analytics servers)
- Consent banners (GDPR/CCPA compliance theater)

The operator knows: "User #4827 from Detroit, Michigan, arrived from a
Google search for 'charter school fraud Michigan,' spent 4 minutes on the
RICO analysis page, then visited the contact page. This is their third
visit this week. They use Chrome on Windows, screen resolution 1920×1080."

**We know none of this. By design.**

---

## The Receptor Model (Signal Sensing)

The receptor model is derived from **quorum sensing** — the biological
mechanism by which bacteria measure local signal molecule concentration
to determine whether a population threshold has been reached for
collective behavior change.

In quorum sensing:
- **LuxI** (synthase) produces the signal molecule (autoinducer)
- **LuxR** (receptor) detects the signal molecule
- Neither identifies which cell produced the signal
- The measurement is: **has local concentration reached threshold?**

### Mapping to Web Infrastructure

| Biological | Digital | Function |
|-----------|---------|----------|
| LuxI synthase | `site.publish` + IndexNow + GSC ping | Emit the signal (publish a page, notify search engines) |
| Autoinducer molecule | The URL + its content | The signal itself (propagates through the network) |
| LuxR receptor | `seo.receptor` (Caddy log parser) | Detect whether the signal was received |
| Quorum threshold | Index conversion rate | Has enough signal accumulated for collective activation? |
| Collective behavior | Search engine organic delivery | The system begins serving the page to searchers |

### What the Receptor Reads

The receptor parses the web server's access log — the same log any web
server writes by default. It reads:

1. **Timestamp** — when the request arrived
2. **Path** — which page was requested
3. **User-Agent string** — what software made the request
4. **Status code** — did the page exist (200) or not (404)
5. **Response time** — how fast the server answered

It does **not** read, store, or process:
- IP addresses (stripped before analysis)
- Cookies (none exist — no tracking cookies are set)
- Referrer headers (not parsed)
- Geographic location
- Device fingerprints
- Any cross-request identifier

### Three-Layer Classification

From the User-Agent string alone, each request is classified:

| Class | Detection Method | Meaning |
|-------|-----------------|---------|
| **Human** | Clean browser UA + non-probe path + human-speed velocity | A real person reading content |
| **SearchBot** | Known crawler UA (Googlebot, Bingbot, etc.) | Search engine building its index |
| **AIBot** | Known AI crawler UA (GPTBot, ClaudeBot, etc.) | AI system consuming content |
| **SocialBot** | Known social UA (Twitterbot, Discordbot) | Link preview generator — a human shared a link |
| **ScraperBot** | Probe paths (`.env`, `.aws`, `wp-admin`) or velocity violation (>5 hits in <10s) | Automated scanning, not reading |
| **GenericBot** | Bot-like UA not matching known patterns | Unclassified automated access |

The behavioral classification uses **three layers**:

1. **Path-based instant kill**: Request to a known probe path → ScraperBot, regardless of UA.
   A request for `/.env.backup` is never a human, no matter what the User-Agent claims.

2. **UA keyword matching**: Known bot signatures in the User-Agent string.
   `Googlebot/2.1` is unambiguously a search crawler.

3. **Session-level reclassification**: After grouping requests by UA within
   a 30-minute window, if a "Human" session contains probe paths or exceeds
   velocity thresholds, the entire session is reclassified as ScraperBot.

The result: when the receptor reports "Human," it means the visitor passed
all three checks. The classification is **provably correct in the negative** —
we can prove something is NOT human with high confidence, even though we
cannot prove it IS a specific human (nor would we want to).

---

## What This Measures (and What It Doesn't)

### What signal sensing tells you

- **Signal propagation**: After publishing a page, did humans arrive? How many?
  How long after publication?
- **System conduction**: Is the network (search engines, social platforms, email)
  carrying the signal to receivers? Which channels conduct and which don't?
- **Activation depth**: Of arriving humans, how many pages do they navigate?
  A single-page bounce means the signal reached them but didn't activate
  investigation behavior. A multi-page deep read means the signal activated.
- **Crawl coverage**: What fraction of pages have been visited by search bots?
  This measures how much of the signal space is indexed and discoverable.
- **Bot ecosystem health**: Are search engines, AI systems, and social platforms
  actively consuming the content? A sudden drop in bot activity signals a
  problem (deindexing, rate limiting, technical failure).

### What signal sensing does NOT tell you

- Who visited (no identity, no IP, no fingerprint)
- Where they came from (no referrer tracking)
- Where they went afterward (no cross-site tracking)
- Whether they're friend or adversary
- Their geographic location
- Their employer, organization, or affiliation
- Whether the same person visited twice

### The critical distinction

**Surveillance** asks: "Who is watching us?"
**Signal sensing** asks: "Is the system conducting our signal?"

A surveillance system would tell you: "Attorney Jane Smith from Dickinson
Wright viewed the RICO analysis page for 4 minutes on Tuesday."

The receptor tells you: "A human viewed the RICO analysis page on Tuesday.
A different human (or possibly the same one — we can't tell) downloaded
FOIA evidence and later checked the contact page."

The receptor measures the **system's behavior**, not the **visitor's identity**.

---

## Why This Matters for Public-Interest Infrastructure

### The privacy paradox of oversight

Detroit's evidence database documents public officials and public
institutions using public records. The site exists to make oversight
information discoverable. But the people who need to discover it —
whistleblowers, affected families, journalists investigating the same
subjects — are precisely the people who would be endangered by
surveillance-based analytics.

If a parent checking whether their child's charter school administrator
appears in an evidence database gets tracked by Google Analytics, that
browsing data exists on Google's servers. It could be subpoenaed. It could
be breached. It could be sold to data brokers who sell to the very
institutions being investigated.

**The receptor model makes this impossible.** There is no data to subpoena.
No browsing history exists anywhere. The access log records a timestamp,
a path, and a User-Agent string. After the receptor processes it, only
aggregated counts remain: "3 humans visited evidence pages today."

### Signal sensing as methodology

The receptor transforms web server logs from an operational artifact into
a scientific instrument. The measurement is:

> Given a signal emitted at time T₀ (page published, email sent, post shared),
> what is the propagation delay T₁ − T₀ before the first human receptor
> activation? What is the activation rate (fraction of linked pages that
> receive human visits within 24 hours)? What is the activation depth
> (pages per session for arriving humans)?

This is **empirical quorum sensing** — measuring whether an oversight
signal has reached the threshold for collective awareness.

---

## Comparison with Existing Approaches

| Approach | Identifies visitors | Requires client JS | Requires cookies | Cross-site tracking | What it measures |
|----------|--------------------|--------------------|-----------------|--------------------|-|
| Google Analytics | ✅ | ✅ | ✅ | ✅ | Who, where from, behavior, conversion |
| Matomo (self-hosted) | ✅ | ✅ | Optional | Optional | Same as GA, self-hosted |
| Plausible/Fathom | Partial | ✅ | ❌ | ❌ | Aggregate pageviews, referrers |
| Server log analysis (AWStats) | ✅ (IP-based) | ❌ | ❌ | ❌ | Hits, bandwidth, IPs, referrers |
| **Receptor model** | **❌** | **❌** | **❌** | **❌** | **Signal propagation, activation, conduction** |

The receptor is not a "privacy-friendly analytics alternative." Privacy-friendly
analytics (Plausible, Fathom) still measure audience. The receptor doesn't
measure audience at all. It measures **whether the network is conducting
the signal**.

Plausible tells you: "47 unique visitors from Michigan today."
The receptor tells you: "3 humans navigated evidence pages today, one
downloaded FOIA documents, search bots crawled 26 unique pages."

The receptor doesn't know there were 47 visitors — it doesn't count unique
visitors because it has no mechanism to deduplicate. It knows 3 humans
*activated* (navigated beyond a landing page). The other 44 may have
bounced, or may not have existed, or may have been the same 3 people on
different devices. **It doesn't matter.** The question isn't "how many
people saw it" but "did the signal activate investigation behavior?"

---

## Implementation

The receptor is implemented as a Rust module in the cellMembrane system
binary. It runs on the same server that hosts the web content, reads the
local Caddy access log over SSH, and produces a structured JSON report.

### Data flow

```
Caddy access log (JSON lines)
    ↓
seo.receptor (Rust, server-side)
    ↓ classify each request (3-layer behavioral)
    ↓ aggregate per-host, per-path
    ↓
ReceptorReport (JSON)
    ↓
observatory/data.json (published on sporePrint)
    ↓
signal.jsonl (append-only event log for skunkBat anomaly detection)
```

### No client-side component

There is no JavaScript. No tracking pixel. No beacon. No `<script>` tag.
The receptor reads what the web server already wrote. If you inspect the
page source of any page on the site, you will find zero analytics code.

The signal transparency notice in the site footer discloses the methodology:

> **Signal transparency:** This site uses receptor-based signal sensing to
> measure page reach and propagation timing. No cookies. No tracking pixels.
> No identifying data collected. Visitor classification (human vs. bot)
> uses user-agent fingerprinting only.

### Open source

The classification logic is published as `cellmembrane-types::visitor` in
the cellMembrane crate. The probe prefix list, velocity thresholds, and
session reclassification rules are all auditable. The methodology is
reproducible: anyone with a Caddy access log can run the same classification.

---

## Biological Precedent

Quorum sensing was first described in *Vibrio fischeri* (Nealson, Platt &
Hastings, 1970), where individual bacteria produce and detect autoinducer
molecules. When local concentration exceeds a threshold, the population
collectively activates bioluminescence.

Key properties of biological quorum sensing that the receptor model preserves:

1. **No individual identification**: Bacteria don't identify which neighbor
   produced the signal. The receptor doesn't identify which human visited.

2. **Threshold activation**: Below quorum, nothing happens. Above quorum,
   collective behavior changes. Below indexing threshold, a page is invisible.
   Above threshold, search engines begin organic delivery.

3. **Signal degradation**: Autoinducers degrade over time if not reinforced.
   Web content decays in search rankings if not refreshed or linked.

4. **Positive feedback**: Above threshold, the signal amplifies itself.
   Indexed pages attract links, which attract more indexing, which attracts
   more visitors. The system is autocatalytic above W_c.

5. **Environmental sensing, not control**: Bacteria don't control the
   environment — they sense it and respond. The receptor doesn't control
   who visits — it senses whether the signal propagated and measures
   the system's response.

---

## Conclusion

The receptor model demonstrates that measuring signal propagation does not
require surveillance. The web analytics industry's assumption — that you
must identify visitors to understand your site's impact — is false. You can
measure impact by measuring the system, not the people in it.

For public-interest infrastructure, this distinction is not academic. It is
the difference between a site that protects its readers and a site that
surveils them. When the readers include whistleblowers, affected families,
and journalists investigating powerful institutions, that difference is
a matter of safety.

The receptor answers the only question that matters for oversight
infrastructure: **did the signal get through?**
