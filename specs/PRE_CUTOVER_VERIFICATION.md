# Pre-Cutover VPS Verification

Verify sporePrint content serves correctly from sovereign VPS infrastructure
**without** requiring DNS changes. Uses `--resolve` to bypass DNS and test
against golgiBody-ext directly.

## VPS Details

- **Host**: golgiBody-ext (outer membrane)
- **IP**: 137.184.197.151
- **Server**: Caddy 2.11.3
- **Content root**: `/opt/ecoPrimals/sporePrint/public`
- **Spores root**: `/opt/ecoPrimals/sporePrint/spores`

## Quick Test (single command)

```bash
curl -sk --resolve primals.eco:443:137.184.197.151 https://primals.eco/ | head -20
```

Note: `-k` required because Caddy cannot provision a TLS cert for primals.eco
until DNS points to it (ACME HTTP-01 challenge requires DNS resolution).

## Full Verification Procedure

### 1. Landing page

```bash
curl -sk --resolve primals.eco:443:137.184.197.151 \
  -o /dev/null -w '%{http_code} %{time_total}s\n' \
  https://primals.eco/
```

Expected: `200 <0.2s`

### 2. Key content pages

```bash
VPS=137.184.197.151
for path in / /architecture/ /science/ /lab/ /methodology/ /products/ \
            /guidestone/ /audience/ /technical/ /glossary/ /sitemap/ \
            /certification/manifest.json /graph/entity-graph.json \
            /css/main.css /js/viz-hydrate.js; do
  CODE=$(curl -sk --resolve primals.eco:443:$VPS \
    -o /dev/null -w '%{http_code}' "https://primals.eco${path}")
  echo "$CODE $path"
done
```

Expected: all 200

### 3. pseudoSpore gallery

```bash
curl -sk --resolve primals.eco:443:137.184.197.151 \
  -o /dev/null -w '%{http_code}\n' \
  https://primals.eco/lab/spores/
```

Expected: 200 (directory listing or index)

### 4. Content count verification

```bash
# Count pages served (check sitemap or known page count)
curl -sk --resolve primals.eco:443:137.184.197.151 \
  https://primals.eco/sitemap.xml | grep -c '<loc>'
```

Expected: 226+ URLs in sitemap

### 5. TTFB comparison

```bash
echo "=== VPS (sovereign) ==="
curl -sk --resolve primals.eco:443:137.184.197.151 \
  -o /dev/null -w 'TTFB: %{time_starttransfer}s\n' \
  https://primals.eco/

echo "=== GitHub Pages (shadow) ==="
curl -s -o /dev/null -w 'TTFB: %{time_starttransfer}s\n' \
  https://primals.eco/
```

Expected: VPS < 100ms, GH Pages > 80ms (VPS wins)

### 6. Certification manifest integrity

```bash
curl -sk --resolve primals.eco:443:137.184.197.151 \
  https://primals.eco/certification/manifest.json | python3 -m json.tool
```

Expected: valid JSON with `entity_count`, `merkle_root`, `content_pages`

## Post-Cutover Verification (after DNS flip)

Once DNS points to VPS:

```bash
# TLS auto-provisioned by Caddy
curl -I https://primals.eco
# Should show: HTTP/2 200, server: Caddy

# Verify cert
openssl s_client -connect primals.eco:443 -servername primals.eco </dev/null 2>/dev/null \
  | openssl x509 -noout -issuer -dates
# Should show: Let's Encrypt, valid dates
```

## Known Considerations

- TLS self-signed until DNS cutover (Caddy needs DNS for ACME challenge)
- `www.primals.eco` redirect configured but also needs DNS A record
- Rollback: re-point A records to GitHub Pages IPs (185.199.108-111.153)
- Caddy has no prior LE certs for primals.eco — rate limits not a concern
