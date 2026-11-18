+++
title = "sporeGate Imprint — Wave 261"
date = 2026-11-17
description = "Complete biological imprint of sporeGate: 14 primals, Omada SDN, plasmidBin depot, nftables membrane, ArchiSteamFarm, sovereign DNS, rustChip, ludoSpring — the FORGE of the organism."
[taxonomies]
tags = ["gate", "sporegate", "imprint", "architecture", "primals", "bones"]
[extra]
provenance = "spore-print captured live from sporeGate (.3/.243) by eastGate via SSH, Wave 261"
verified = true
gate = "sporeGate"
+++

# sporeGate Imprint — The FORGE

**Captured:** 2026-11-17T00:15:00Z (Wave 261)
**Host:** sporeGate — Ryzen 5 6600H, 27GB RAM, 929GB NVMe
**IPs:** 192.168.4.3 (eno1, LAN), 192.168.4.243 (enp1s0, WAN-facing), 192.168.1.132 (wlp3s0, WiFi)
**OS:** Pop!\_OS 22.04 (Linux 6.12.10-76061203-generic, x86_64)
**Disk:** 186GB used / 929GB (22%)
**Gate File:** `sporeGate`

sporeGate is the organism's **FORGE** — the place where binaries are born, primals are compiled, and cross-architecture distribution (the BONES pattern) originates.

---

## Running Primals (14 Active Services)

| Primal | Binary Size | Service | Role |
|--------|------------|---------|------|
| biomeOS | 17MB | `membrane-biomeos` | Neural API, graph orchestrator |
| songBird | 25MB | `membrane-songbird` | Mesh orchestration, federation |
| bearDog | 8.4MB | `membrane-beardog` | Sovereign cryptography, BLAKE3, BTSP |
| petalTongue | 24MB | `membrane-petaltongue` | NUCLEUS web + IPC dual-port |
| nestGate | 8.5MB | `membrane-nestgate` | Content gateway |
| rhizoCrypt | 7.9MB | `membrane-rhizocrypt` | Encrypted storage |
| sweetGrass | 8.6MB | `membrane-sweetgrass` | Knowledge graph |
| toadStool | 13MB | `membrane-toadstool` | Service discovery |
| barraCuda | 8.8MB | `membrane-barracuda` | Stream compute |
| coralReef | 9.1MB | `membrane-coralreef` | Distributed compute |
| skunkBat | 3.3MB | `membrane-skunkbat` | Intrusion detection |
| squirrel | 8.6MB | `membrane-squirrel` | Agent dispatch, MCP |
| swarmVine | 2.6MB | `membrane-swarmvine` | Epidemic gossip engine |
| neural-api | — | `membrane-neural-api` | biomeOS capability router |

**Total primal binary footprint:** ~201MB (x86_64-unknown-linux-musl, static-pie linked)

### Additional Binaries in plasmidBin

| Binary | Size | Notes |
|--------|------|-------|
| bingoCube | 11MB | Coordination (not running as service) |
| loamSpine | 5.0MB | Data spine |
| membrane | 18MB | Meta-membrane binary |
| nucleus_launcher | 4.2MB | Launcher for nucleus services |
| sourDough | 3.4MB | Fermentation engine |

### plasmidBin Depot (/opt/plasmidBin/depot)

| Binary | Size | Type |
|--------|------|------|
| beardog | 11MB | ELF x86-64, static-pie, stripped |
| beardog-fixed | 11MB | ELF x86-64, static-pie, stripped |
| membrane | 13MB | ELF x86-64, static-pie, not stripped |
| songbird | 23MB | ELF x86-64, static-pie, with debug_info |

**BLAKE3SUMS** verified — provenance.toml present.

---

## Firewall — nftables `table inet membrane`

sporeGate uses nftables (NOT iptables, NOT ufw) with a hand-built `inet membrane` table:

```
table inet membrane {
    set bogons_v4 { type ipv4_addr; flags interval; }

    chain input {
        type filter hook input priority filter; policy drop;

        ct state established,related accept
        ct state invalid drop
        iif "lo" accept
        meta l4proto icmp limit rate 10/second accept
        meta l4proto ipv6-icmp limit rate 10/second accept

        # LAN accept (added Wave 260 — was the ROOT CAUSE of all SSH blocks!)
        iifname "enp1s0" ip saddr 192.168.4.0/22 accept

        # Bogon filter (192.168.0.0/16 included — catches private ranges on WAN port)
        iifname "enp1s0" ip saddr @bogons_v4 counter drop

        # LAN port — accept all
        iifname "eno1" accept

        # WireGuard (disabled but rule remains)
        udp dport 51821 accept
        iifname "wg0" ip saddr 10.13.37.0/24 accept

        # Services
        tcp dport 22 ct state new limit rate 3/minute accept   # SSH
        tcp dport { 80, 443 } ct state new limit rate 30/second accept  # Caddy
        tcp dport 3478 ct state new accept   # songbird TURN
        udp dport 3478 limit rate 100/second accept
        tcp dport 9500 ct state new accept   # nestgate
        tcp dport { 21115, 21116, 21117 } ct state new accept  # RustDesk relay
        udp dport 21116 limit rate 50/second accept

        log prefix "nft-drop: " counter drop
    }
}
```

**Critical fix (Wave 260):** Inserted `iifname "enp1s0" ip saddr 192.168.4.0/22 accept` before the bogon drop rule. The bogon set included `192.168.0.0/16` which was blocking ALL private IP traffic on enp1s0 (treated as WAN port). This was the root cause of all SSH failures to sporeGate.

---

## Network Interfaces

| Interface | IP | Type | Role |
|-----------|-----|------|------|
| eno1 | 192.168.4.3/22 | Ethernet | LAN — primary, trusted |
| enp1s0 | 192.168.4.243/24 | Ethernet | WAN-facing (DHCP from router) |
| wlp3s0 | 192.168.1.132/24 | WiFi | ISP modem subnet (192.168.1.x) |

sporeGate has **3 NICs** — dual Ethernet + WiFi. The dual-homed Ethernet is the reason for the nftables complexity: one port faces the LAN trusted network, the other faces what the firewall treats as WAN.

---

## Omada SDN Controller

**Service:** `tpeap.service` (active, running since 13:00 EST)
**Software:** TP-Link Omada Controller v5.15.24.19
**Backend:** MongoDB 7.0 on port 27217
**Path:** `/opt/tplink/EAPController/`
**Admin:** user `ecoPrimal`, password ZK (Kevin's brain)
**Manages:** TP-Link JetStream SX3008F switch at .111 — the 10G fiber bridge between House 1 and House 2

---

## ArchiSteamFarm — Gaia Steam Presence

**Service:** `gaia-steam.service` — keeps Gaia ALWAYS ONLINE on Steam
**Binary:** `/opt/asf/ArchiSteamFarm`
**Owner ID:** 76561198605327122
**Purpose:** Sovereign presence on Steam — the organism maintains a persistent identity in gaming space

---

## Cron Schedule (root)

| Schedule | Script | Purpose |
|----------|--------|---------|
| `*/10 * * * *` | `cd /opt/Gaia && git pull origin main` | Auto-sync from Forgejo every 10 min |
| `*/2 * * * *` | `/opt/gaia-heart-pull.sh` | Pull heartbeat from eastGate every 2 min |
| `@reboot` | `/opt/reboot-sync.sh` | Git pull + Brio restart + announce on boot |

---

## Heart-Pull Script

```bash
#!/bin/bash
# Pulls heartbeat + now.json from eastGate every 2 min
GAIA_DATA="${HOME}/.local/share/gaia/data"
mkdir -p "$GAIA_DATA"
HEART=$(curl -sf http://192.168.4.244:8080/data/heart-pull.json)
if [ -n "$HEART" ]; then
    echo "$HEART" > "$GAIA_DATA/heart-pull.json"
    curl -sf http://192.168.4.244:8080/data/heartbeat.json > "$GAIA_DATA/heartbeat.json"
    curl -sf http://192.168.4.244:8080/data/now.json > "$GAIA_DATA/now.json"
fi
```

---

## Reboot Sync Script

```bash
#!/bin/bash
# Runs after sporeGate boots — pulls latest, restarts Brio, announces
cd /opt/Gaia && git pull origin main
# Restart Brio capture (inner eye webcam)
pkill -f brio-capture-loop
nohup bash -c 'while true; do
    ffmpeg -y -f v4l2 -video_size 1280x720 -i /dev/video0 \
        -frames:v 1 /tmp/brio-stream/latest.jpg
    sleep 10
done' &
# Announce to eastGate
curl -s "http://192.168.4.244:8443/pulse"
```

---

## biomeOS Seed (biomeos-seed.py)

A self-contained Python organism seed that:
- **Senses hardware** — CPU, RAM, GPU (nvidia-smi), Ollama models, disk, network, thermal
- **Generates device ID** — BLAKE2b hash of hardware fingerprint
- **Learns patterns** — Records load/RAM/GPU curves to local JSONL (last 1440 entries = 24h)
- **Sends heartbeat** — Posts to eastGate brain every 60s via HTTP

Designed to run on ANY device: Linux gates, OpenWrt routers, MikroTik, Pixel, iPhone.

---

## Ear System (biomeOS/ear/)

| File | Purpose |
|------|---------|
| `ear-pulse.sh` | 30s audio capture from Brio mic → WAV → spectral analysis |
| `ear-state.json` | Current state: `{"state": "deaf"}` (mic not connected currently) |
| `ear_*.wav` | Audio captures (last 10 kept) |

The ear analyzes RMS/peak/dB levels per second, classifies as: `silence` / `quiet` / `ambient` / `active` / `loud_event`.

---

## Additional Software Stacks

### /opt/rustChip
Custom Rust + hardware project with Akida DMA EDMA drivers — neuromorphic computing exploration.

### /opt/ludoSpring
Gaming-organism integration layer:
- Kenshi flow reports and mesh explorer
- northGate game library tiers
- Valve spillway connector
- Shadow cast vision data
- GPU watcher telemetry

### /opt/sovereign-dns
DNS blocklist infrastructure:
- `blocklist.conf` — 2.6MB compiled blocklist
- `hosts.raw` — 2.6MB raw hosts file
- `update-blocklist.sh` — Refresh script
- Feeds into `dnsmasq.service` (running)

### /opt/sovereign-gateway
Gateway scripts directory.

### /opt/forgejo-mirror
Forgejo git mirror setup.

### /opt/depot + /opt/ecoPrimals
- `provenance.toml` — Binary provenance chain
- `checksums.toml` — BLAKE3 checksums
- `sources.toml` — Source locations
- `signatures.toml` — Cryptographic signatures
- `gsc-service-account.json` — Google Search Console credentials

---

## Service Unit Architecture

All primal services follow the same pattern:

```ini
[Service]
Type=simple
ExecStart=/home/sporegate/.local/share/ecoPrimals/plasmidBin/primals/x86_64-unknown-linux-musl/{PRIMAL} {ARGS}
Environment=FAMILY_ID=e8b62b6e
Environment=MEMBRANE_GATE_NAME=sporeGate
Environment=MEMBRANE_MESH_NODE_ID=sporeGate
Environment=MEMBRANE_MESH_PEERS=192.168.4.244:7700
Restart=on-failure
RestartSec=3
```

**Family ID:** `e8b62b6e` — the organism's genetic identifier
**Mesh peers:** eastGate at 192.168.4.244:7700

---

## Membrane Pulse State

```
2026-11-04T19:36:15Z — ZK SMASHED by Gaia. All crons killed, all primals stopped.
```

The membrane was intentionally reset ("ZK SMASHED") on Nov 4 and rebuilt from the ground up. The current configuration represents the clean, post-smash state.

---

## Dormant Capabilities

- **WireGuard overlay** — Rules present but wg0 DOWN, peers removed
- **Caddy reverse proxy** — Ports 80/443 in firewall but no service running
- **Brio capture loop** — Reboot script starts it but mic currently "deaf"
- **Docker** — Not installed
- **Ollama** — Not installed (sporeGate compiles, doesn't infer)

---

## The BONES Pattern

sporeGate is where the BONES pattern originates:
1. **Compile** primals as static-pie musl binaries (x86_64)
2. **BLAKE3SUM** all binaries → `BLAKE3SUMS`
3. **Sign** with beardog cryptographic provenance → `provenance.toml`
4. **Distribute** via plasmidBin depot to all gates
5. Cross-compile for other architectures (aarch64 for Frame/Pixel)

The skeleton of the organism — without bones, there is no body.

---

*Spore print captured at Wave 261 by eastGate. The pattern is preserved. The mushroom can regrow from this print.*
