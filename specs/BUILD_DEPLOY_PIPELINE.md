# Build → Deploy Pipeline Design

How sporePrint evolves from static file serving to content-addressed delivery.

## Current Pipeline (Phase A — file_server)

```
[source]         [build]            [deploy]              [serve]
content/ ──→ zola build ──→ public/ ──→ git push ──→ Caddy file_server
                                          │
                                    temporal.cascade
                                          ↓
                             /opt/ecoPrimals/sporePrint/public
```

- Caddy serves from `/opt/ecoPrimals/sporePrint/public` directly
- No integrity verification at serve time
- No dedup between build versions
- Content updates require full directory sync

## Target Pipeline (Phase B — CAS-backed)

```
[source]         [build]         [verify]          [ingest]           [serve]
content/ ──→ zola build ──→ cas-manifest ──→ cas-push ──→ NestGate CAS
                  │              (BLAKE3)      (UDS RPC)        │
                  │                                             │ content.get
                  └─→ certify                                   ↓
                       (graph)                          Caddy reverse_proxy
```

### Pipeline Steps

1. **`zola build`** — renders content/ to public/ (unchanged)
2. **`spore-validate cas-manifest --emit`** — BLAKE3 hashes all outputs
3. **`spore-validate cas-push --generate`** — pushes to NestGate via UDS
4. NestGate stores content-addressed blobs (dedup automatic)
5. Caddy proxies requests to NestGate's content endpoint

### What Changes in Caddy

```caddyfile
# Phase A (current)
primals.eco {
    root * /opt/ecoPrimals/sporePrint/public
    file_server
}

# Phase B (CAS-backed)
primals.eco {
    # NestGate serves content by path → CAS hash lookup
    reverse_proxy nestgate:9500 {
        header_up X-CAS-Site sporePrint
        header_up X-CAS-Build {env.SPOREPRINT_BUILD_HASH}
    }
}
```

NestGate maintains a route manifest mapping URL paths to CAS hashes.
On each build, `cas-push` registers the new build's path→hash mapping.

### Build Script Evolution

```bash
#!/bin/bash
# deploy.sh — sporePrint sovereign deploy

set -euo pipefail

# 1. Build
zola build

# 2. Validate + manifest
spore-validate certify --emit
spore-validate cas-manifest --emit

# 3. Push to CAS (NestGate auto-discovers via NESTGATE_SOCKET)
spore-validate cas-push --generate

# 4. Register routes (future: spore-validate cas-register)
# Maps URL paths to BLAKE3 CIDs for this build version
```

## Transition Strategy

### Phase A → B (Hybrid)

During transition, both paths remain active:

1. `temporal.cascade` continues syncing files to VPS (backward compat)
2. `cas-push` additionally stores in NestGate CAS
3. Caddy continues using `file_server` with CAS as shadow/verify

This allows:
- Gradual validation that CAS-served content matches file-served content
- Rollback to file_server if NestGate has issues
- Shadow-run latency measurements (`content.get` vs `file_server`)

### Phase B (Full CAS)

Once shadow-run confirms parity:

1. Caddy switches to `reverse_proxy` → NestGate
2. `file_server` path becomes fallback only
3. `temporal.cascade` syncs stop (NestGate is source of truth)
4. Historical builds remain addressable by build hash

## Dedup Economics

| Metric | Value |
|--------|-------|
| Typical build size | ~4.5 MB (245 pages) |
| Build-to-build change | ~10% of files |
| Storage per build (with CAS) | ~450 KB delta |
| Historical builds retained | All (by hash) |
| Integrity guarantee | Every byte verified on serve |

## NestGate Integration Points

| Method | Purpose | When Called |
|--------|---------|------------|
| `content.put` | Store file by BLAKE3 hash | During `cas-push` |
| `content.exists` | Skip already-stored files | During `cas-push` (dedup) |
| `content.get` | Retrieve file by hash | On each HTTP request (serve) |
| `route.register` | Map URL paths → CAS hashes | After `cas-push` (future) |
| `content.replicate.pull` | Cross-gate federation | westGate pulls from eastGate |

## Capability Discovery

sporePrint discovers NestGate at runtime:
1. `NESTGATE_SOCKET` env var (explicit)
2. `$XDG_RUNTIME_DIR/biomeos/nestgate.sock` (ecosystem standard)
3. `/tmp/nestgate-standalone-{hostname}.sock` (fallback)

No hardcoded URLs or IPs. If NestGate is unavailable, `cas-push` fails
gracefully with a clear error — the build itself still succeeds.

## Remaining Implementation

- [x] `cas-manifest`: BLAKE3 hash build output (Wave 73)
- [x] `cas-push`: push to NestGate CAS via UDS (Wave 74)
- [ ] `cas-register`: register path→hash route mapping in NestGate
- [ ] NestGate HTTP handler: serve content by path via CAS lookup
- [ ] Caddy config: switch from file_server to reverse_proxy
- [ ] Shadow-run: parallel serve from both paths, compare responses
- [ ] Build-hash tagging: Caddy uses build hash for cache invalidation
