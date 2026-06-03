# NestGate CAS Integration Design

How sporePrint build outputs integrate with NestGate content-addressed storage.

## Context

NestGate provides content-addressed storage (CAS) backed by BLAKE3 hashing.
With westGate incoming (76TB ZFS), sporePrint's build artifacts become the
first sovereign content pipeline to utilize CAS for:

- **Integrity verification** — every served page has a verifiable hash
- **Deduplication** — unchanged pages across builds share storage
- **Provenance** — content chain from source markdown to served HTML

## Architecture

```
                   BUILD PHASE                         SERVE PHASE
                   ──────────                          ───────────
source markdown ─→ zola build ─→ public/              NestGate CAS store
                        │                                    ↑
                        ├─→ spore-validate certify           │ content.put
                        │       (graph merkle)               │ (BLAKE3-keyed)
                        │                                    │
                        └─→ cas-ingest ──────────────────────┘
                              │
                              └─→ manifest.toml (build hash → page hashes)

Caddy file_server ←── /opt/ecoPrimals/sporePrint/public (current)
Caddy reverse_proxy ←── NestGate :9500 /cas/... (future — hash-verified)
```

## Phase 1: Build-Time CAS Manifest (no serving changes)

Extend `spore-validate` with a `cas-manifest` subcommand that:

1. After `zola build`, walk `public/` directory
2. Compute BLAKE3 hash for every output file
3. Emit a CAS manifest: `static/cas/build-manifest.json`

```json
{
  "build_id": "2026-06-03T08:52:00Z",
  "build_hash": "blake3:abc123...",
  "page_count": 226,
  "total_bytes": 4521033,
  "files": {
    "index.html": {
      "hash": "blake3:def456...",
      "size": 12340,
      "content_type": "text/html"
    },
    "css/main.css": {
      "hash": "blake3:789abc...",
      "size": 19200,
      "content_type": "text/css"
    }
  }
}
```

This extends the existing `provenance` subcommand (which already hashes
content markdown) to cover the *built* output.

### Implementation Path

```rust
// In spore-validate, new module: cas.rs

pub struct CasManifest {
    pub build_id: String,
    pub build_hash: String,
    pub page_count: usize,
    pub total_bytes: u64,
    pub files: BTreeMap<String, CasEntry>,
}

pub struct CasEntry {
    pub hash: String,
    pub size: u64,
    pub content_type: String,
}

pub fn generate_cas_manifest(public_dir: &Path) -> CasManifest {
    // Walk public/, hash each file with blake3
    // Build hash = blake3 of sorted file hashes (Merkle-like)
}
```

## Phase 2: NestGate Ingest (westGate storage)

Once westGate is operational with ZFS:

1. `spore-validate cas-push` sends each file to NestGate's CAS API
2. NestGate stores by hash (dedup automatic for unchanged pages)
3. Build manifest registered in NestGate's catalog

```
PUT /cas/store
Content-Type: application/octet-stream
X-CAS-Hash: blake3:def456...
X-CAS-Path: index.html
Body: <file contents>
```

NestGate verifies hash on ingest (tamper detection). Rejects mismatches.

### Dedup Economics

Typical sporePrint build: 226 pages, ~4.5 MB total.
Between builds: ~90% of files unchanged (CSS, JS, most pages).
With CAS: only changed files stored. Historical builds remain addressable.

## Phase 3: CAS-Backed Serving

Replace Caddy `file_server` with NestGate-backed content delivery:

```
# Caddyfile evolution
primals.eco {
    # Current: file_server from disk
    # root * /opt/ecoPrimals/sporePrint/public
    # file_server

    # Future: NestGate CAS proxy
    reverse_proxy nestgate:9500 {
        header_up X-CAS-Site sporePrint
        header_up X-CAS-Build latest
    }
}
```

Benefits:
- **Hash verification on every serve** — bit-rot impossible
- **Historical builds addressable** — `X-CAS-Build: <build_hash>` serves any version
- **Multi-site** — NestGate serves multiple sites from same CAS pool
- **CDN-friendly** — immutable content hashes enable aggressive caching

## Phase 4: Mesh Content Aggregation

When mesh routing is operational across gates:

1. Each gate publishes its science content to NestGate CAS
2. sporePrint aggregates content from all gates' CAS stores
3. Build includes content discovered at mesh-discovery time
4. Content manifest tracks which gate provided each page

```
sporePrint build → discover mesh peers → pull CAS manifests
  → fetch new/updated content by hash
  → integrate into unified site
  → publish build manifest with provenance chain
```

## Relationship to Existing Systems

| System | Role | CAS Integration |
|--------|------|-----------------|
| `spore-validate provenance` | Source markdown hashes | Input to CAS (source chain) |
| `spore-validate certify` | Entity graph Merkle root | CAS manifest includes as metadata |
| `spore-validate cas-manifest` | Built output hashes | **New** — bridges build → CAS |
| NestGate catalog | Content registry | Stores CAS manifests + blobs |
| Caddy | TLS + serving | Proxies to NestGate in Phase 3 |

## Implementation Priority

1. **Phase 1** (Wave 73+): `cas-manifest` subcommand — pure local, no deps
2. **Phase 2** (post-westGate): NestGate ingest API integration
3. **Phase 3** (post-mesh): CAS-backed serving
4. **Phase 4** (horizon): Mesh content aggregation

Phase 1 is implementable immediately. Only depends on:
- `blake3` (already in deps, pure-Rust)
- `walkdir` (already in deps)
- File I/O + JSON output (already used by certify/graph)

## Constraints

- No new external dependencies (pure Rust, zero C)
- CAS hashes use BLAKE3 (same as provenance + certify — ecosystem standard)
- Manifest format must be parseable by NestGate (JSON, no TOML for wire format)
- Build manifest is deterministic: same input → same manifest hash
- sporePrint only has self-knowledge — discovers NestGate at runtime via
  capability discovery (no hardcoded NestGate URLs)
