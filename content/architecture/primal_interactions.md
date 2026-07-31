+++
title = "Primal Interactions — IPC Architecture"
description = "How primals communicate: JSON-RPC 2.0 over Unix sockets, the discovery hierarchy, the Neural API semantic layer, and the no-coupling rule."
date = 2026-03-15
weight = 44

[taxonomies]
trails = ["coordination"]

[extra]
foundation = true
domain = "Architecture"
maturity = "implemented"

[[extra.companions]]
url = "/architecture/primal-evolution/"
title = "Primal Evolution"
relation = "pairs_with"
label = "How primals evolved the interfaces they now use"

[[extra.companions]]
url = "/architecture/neural-api/"
title = "Neural API"
relation = "extends"
label = "Orchestration layer above IPC"
+++

## The Core Rule

No compile-time coupling. All coordination via JSON-RPC 2.0 over Unix
domain sockets. A primal does not import another primal's code. A primal
does not link against another primal's library. A primal communicates
by sending JSON messages through a socket.

This rule is not a preference. It is the constraint that makes independent
evolution possible — any primal can upgrade without recompiling any other
primal.

---

## Transport Stack

| Layer | Protocol | Use Case |
|-------|----------|----------|
| Unix domain socket | JSON-RPC 2.0 | Same-machine (primary, lowest latency) |
| Abstract socket | JSON-RPC 2.0 | Containerized environments |
| TCP | JSON-RPC 2.0 | Cross-machine, cross-gate |
| HTTP bridge | Axum JSON-RPC | External consumers, web interfaces |

### Socket Paths

```
$XDG_RUNTIME_DIR/biomeos/{primal}.sock
```

Each primal has its own socket. {{ entity(name="songbird") }} maintains the registry of
active sockets and their capabilities.

---

## Discovery Hierarchy

How does a primal find another primal? Four-tier fallback:

| Tier | Method | When Used |
|------|--------|-----------|
| 1 | Environment variable | `BEARDOG_SOCKET=/run/biomeos/beardog.sock` |
| 2 | XDG runtime scan | Scan `$XDG_RUNTIME_DIR/biomeos/*.sock` |
| 3 | Home directory | `~/.biomeos/sockets/` fallback |
| 4 | System default | `/run/biomeos/` system-wide |

A primal attempting to discover {{ entity(name="beardog") }} checks tier 1 first. If
unset, it scans tier 2. The hierarchy ensures that primals work in
development, containerized, and production environments without
configuration changes.

---

## The {{ entity(name="songbird") }} Hub

{{ entity(name="songbird") }} is the O(n) discovery hub. Without it, each primal
would need to discover every other primal independently — O(n²) connections.
With {{ entity(name="songbird") }}:

```
All primals register with Songbird (n registrations)
Any primal queries Songbird for capabilities (1 lookup)
Songbird routes to the correct socket (1 connection)
```

Total connections: O(n) instead of O(n²).

---

## The {{ entity(name="biomeos") }} Neural API Semantic Layer

Above the raw JSON-RPC transport sits the Neural API semantic layer. Instead
of calling a specific primal's method directly, a product or spring calls:

```json
{"method": "capability.call", "params": {"capability": "crypto.sign", "data": "..."}}
```

{{ entity(name="biomeos") }} resolves `crypto.sign` to `beardog.crypto.sign` and routes
the call. This means:

- **Products do not know which primal implements a capability** — they
  know what they need, not who provides it
- **Primals can be replaced** — if `crypto.sign` is implemented by a
  different primal in the future, the product does not change
- **Capabilities compose** — `compute.matrix_mul` might dispatch to
  {{ entity(name="barracuda") }} (GPU) or CPU fallback depending on hardware

---

## Method Registration

Every primal registers its methods with {{ entity(name="songbird") }} at startup:

```json
{
  "method": "ipc.register",
  "params": {
    "primal": "beardog",
    "methods": ["crypto.sign", "crypto.verify", "secrets.store", ...],
    "socket": "/run/biomeos/beardog.sock"
  }
}
```

The registration includes the full method list and the socket path.
{{ entity(name="songbird") }} builds a capability routing table from all registrations.

---

## Interaction Patterns

### Request-Response (Standard)

Most primal interactions are request-response:

```
Product -> biomeOS: capability.call("crypto.sign", data)
biomeOS -> beardog: crypto.sign(data)
beardog -> biomeOS: {result: signature}
biomeOS -> Product: {result: signature}
```

### Fire-and-Forget (Telemetry)

Health probes and telemetry use fire-and-forget:

```
biomeOS -> beardog: health.ping()
beardog -> biomeOS: {status: "healthy", uptime: 3600}
```

### Event Stream (Future)

For continuous coordination (game sessions, live monitoring), event streams
over persistent connections.

---

## The No-Coupling Rule in Practice

| Violation | Why It Is Forbidden | Correct Pattern |
|-----------|--------------------|--------------------|
| `use beardog::crypto` | Compile-time coupling | `capability.call("crypto.sign")` |
| Shared crate between primals | Synchronized versions | Independent crates, JSON-RPC protocol |
| Direct socket connection between primals | Bypasses discovery | Route through {{ entity(name="songbird") }} |
| Hardcoded socket path | Environment-specific | Discovery hierarchy |

---

*The IPC architecture is not a convenience layer. It is the constraint that
makes the ecosystem possible — 14 independent programs, each with its own
release cycle, communicating through a standard protocol, discovered at
runtime, composed by {{ entity(name="biomeos") }} into whatever the science requires.*
