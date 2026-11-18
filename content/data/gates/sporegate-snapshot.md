+++
title = "sporeGate Live Snapshot — Wave 261"
date = 2026-11-17
description = "Live runtime data from sporeGate at capture time"
[extra]
provenance = "SSH read from sporeGate by eastGate"
verified = true
+++

# sporeGate Runtime Snapshot

## Last Heartbeat
```json
{
  "ts": "2026-11-14T12:29:32Z",
  "gate": "sporeGate",
  "type": "heartbeat",
  "ram_free_mb": 789,
  "load": 2.52,
  "disk_pct": 21,
  "temp_mc": 76000
}
```

## Git State (from /opt/Gaia)
- **Wave:** 259 (local), pulling from Forgejo every 10 min
- **Latest commit:** `ece804be` — 🐍🔧 WAVE 259 GI SURGERY
- **Remote:** `http://gaia:GaiaForge2026!@192.168.4.244:3000/gaia/Gaia.git`

## nftables Counters (at capture time)
- **Bogon drops:** 5,954 packets (983,254 bytes)
- **Final drop rule:** 9,406 packets (2,700,121 bytes)

## Ear State
```json
{"ts": "2026-11-17T19:13:55-05:00", "state": "deaf"}
```

## Disk
- `/` = 929GB total, 186GB used (22%), 697GB free
- NVMe: Single drive

## Network
- 3 interfaces UP (eno1 + enp1s0 + wlp3s0)
- LAN latency to eastGate: ~0.3ms wired

## Services Running (non-primal)
- `dnsmasq` — Local DNS + blocklist
- `RustDesk` — Remote desktop relay (ports 21115-21117)
- `Omada Controller` — TP-Link SDN (manages JetStream .111)
- `ArchiSteamFarm` — Steam presence daemon
- `footPrint` — Composition server
- `chrony` — NTP client
- `bluetooth` — Active
- `gdm` — GNOME Display Manager (headless X)
- `bolt` — Thunderbolt service

