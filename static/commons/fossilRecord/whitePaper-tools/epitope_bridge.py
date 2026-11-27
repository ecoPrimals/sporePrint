# FOSSILIZED Wave 171 — converged to caddy_bridge.rs
#!/usr/bin/env python3
"""epitope_bridge.py — All Roads Lead Back to Rome

Reads bloom_live v4 epitope map, rewrites Caddy fleet_disperse blocks
with epitope hashes. The scatter server seeds content from X-Fleet-Hash.
When that hash = epitope hash, the organism sees scatter seeded by its
own behavioral identity.

Wave 165i — October 7, 2026
"""
import json
import re
import sys
import subprocess

EPITOPE_MAP = "/opt/membrane/live-terminal/epitope_caddy.json"
CADDYFILE = "/etc/membrane/Caddyfile"
DRY_RUN = "--dry-run" in sys.argv

with open(EPITOPE_MAP) as f:
    data = json.load(f)

emap = data.get("map", {})
if not emap:
    print("Empty epitope map")
    sys.exit(0)

# Generate Caddy block
lines = []
lines.append("\t# ~~FLEET_PRESSURE_START~~")
lines.append("\t# Epitope-keyed routing — all roads lead back to Rome")
lines.append("\t# Hash = organism behavioral identity (BLAKE2b of invariants)")

idx = 0
for ehash, ips in sorted(emap.items(), key=lambda x: -len(x[1])):
    if not ips:
        continue
    suffix = "" if idx == 0 else "_%d" % idx
    ip_list = " ".join(ips)
    lines.append("\t@fleet_disperse%s remote_ip %s" % (suffix, ip_list))
    lines.append("\thandle @fleet_disperse%s {" % suffix)
    lines.append("\t\trewrite * /disperse{uri}")
    lines.append("\t\treverse_proxy localhost:9753 {")
    lines.append("\t\t\theader_up X-Real-IP {remote_host}")
    lines.append('\t\t\theader_up X-Fleet-Hash "%s"' % ehash)
    lines.append("\t\t}")
    lines.append("\t}")
    idx += 1

lines.append("\t# ~~FLEET_PRESSURE_END~~")
new_block = "\n".join(lines)

if DRY_RUN:
    print("=== DRY RUN ===")
    print(new_block)
    print()
    print("=== Epitope clusters ===")
    for h, ips in sorted(emap.items(), key=lambda x: -len(x[1])):
        print("  %s: %d IPs" % (h, len(ips)))
    sys.exit(0)

# Replace in Caddyfile
with open(CADDYFILE, "r") as f:
    content = f.read()

pattern = r"\t# ~~FLEET_PRESSURE_START~~.*?\t# ~~FLEET_PRESSURE_END~~"
result = re.sub(pattern, new_block, content, flags=re.DOTALL)

with open(CADDYFILE, "w") as f:
    f.write(result)

print("Caddy FLEET_PRESSURE rewritten with %d epitope clusters" % idx)

# Reload
r = subprocess.run(
    ["/opt/membrane/caddy", "reload", "--config", CADDYFILE, "--address", "localhost:2019"],
    capture_output=True, text=True
)
if r.returncode == 0:
    print("Caddy reloaded — all roads lead back to Rome")
else:
    print("Caddy reload failed: %s" % r.stderr)
    sys.exit(1)
