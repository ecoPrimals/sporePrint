# FOSSILIZED Wave 171 — replaced by Rust membrane_stack.rs in skunky-ingest
#!/usr/bin/env python3
"""
membrane-breathe — Nested membrane stack observer.

Pulls live data from every observable membrane layer and computes
a unified Anderson-like profile. Writes membrane-stack.json for
the HUD and signal dashboards.

Layers (inside → outside):
  0. gAIa       — bloom sensor + Anderson profile (local)
  1. Mycelium   — soil moisture proxy (USGS groundwater)
  2. Crust      — seismic activity (USGS earthquake feed)
  3. Atmosphere  — stratopause/tropopause (NOAA)
  4. Magnetosphere — Kp index, geomagnetic storms (NOAA)
  5. Heliosphere — solar wind speed, IMF, flux (NOAA DSCOVR)
  6. Deep Space  — cosmic ray proxy (neutron monitor)

Each layer gets:
  - raw data from its feed
  - a permeability estimate P (0=opaque, 1=transparent)
  - an Anderson W estimate (disorder/filtering strength)
  - a breathing indicator (is the membrane expanding or contracting?)

Wave 171 — "I want to watch gAIa breathe"
"""

import json
import sys
import time
import urllib.request
import urllib.error
from datetime import datetime, timezone
from pathlib import Path

OUTPUT_PATH = Path("/opt/membrane/live-terminal/membrane-stack.json")
BLOOM_PATH = Path("/run/membrane/bloom.signal")
ANDERSON_PATH = Path("/run/membrane/anderson-profile.json")
TIMEOUT = 10


def fetch_json(url: str) -> dict | list | None:
    try:
        req = urllib.request.Request(url, headers={"User-Agent": "ecoPrimal-artisan/1.0 membrane-breathe"})
        with urllib.request.urlopen(req, timeout=TIMEOUT) as resp:
            return json.loads(resp.read())
    except (urllib.error.URLError, OSError, json.JSONDecodeError, TimeoutError):
        return None


def read_local_json(path: Path) -> dict | None:
    try:
        return json.loads(path.read_text())
    except (FileNotFoundError, json.JSONDecodeError):
        return None


def layer_gaia() -> dict:
    """Layer 0: gAIa — our own membrane."""
    bloom = read_local_json(BLOOM_PATH)
    anderson = read_local_json(ANDERSON_PATH)

    if bloom and anderson:
        total = bloom.get("total_requests", 0)
        readers = bloom.get("readers", {})
        human = readers.get("human", 0)
        ai = readers.get("ai-agent", 0)
        scanner = readers.get("scanner", 0)

        # Permeability: fraction of traffic that is human (desired signal)
        p = human / max(total, 1)
        # Anderson W from live profile
        w = anderson.get("w_population", 0)
        selectivity = anderson.get("selectivity_observed", 0)

        return {
            "name": "gAIa",
            "scale": "digital membrane",
            "depth": "0 (you are here)",
            "source": "bloom sensor + anderson bridge",
            "data": {
                "requests_per_window": total,
                "human": human,
                "ai_agent": ai,
                "scanner": scanner,
                "selectivity": round(selectivity, 3),
            },
            "permeability": round(p, 4),
            "anderson_w": round(w, 2),
            "breathing": "ticking" if total > 0 else "dormant",
            "alive": True,
        }

    return {"name": "gAIa", "alive": False, "error": "no bloom/anderson data"}


def layer_seismic() -> dict:
    """Layer 2: Crust — seismic activity as membrane vibration."""
    data = fetch_json("https://earthquake.usgs.gov/earthquakes/feed/v1.0/summary/all_hour.geojson")

    if data and "features" in data:
        quakes = data["features"]
        count = len(quakes)
        max_mag = max((f["properties"]["mag"] or 0 for f in quakes), default=0)
        locations = [f["properties"]["place"] for f in quakes[:3] if f["properties"].get("place")]

        # More quakes = more energy crossing the crust membrane
        # P increases with activity (the membrane is more permeable to seismic energy)
        p = min(count / 20.0, 1.0)
        # W decreases with magnitude (big quakes = less disorder, signal propagates)
        w = max(15 - max_mag * 2, 1)

        return {
            "name": "Earth's Crust",
            "scale": "0 to -70 km",
            "depth": "Mohorovičić discontinuity",
            "source": "USGS earthquake feed",
            "data": {
                "quakes_last_hour": count,
                "max_magnitude": round(max_mag, 1),
                "locations": locations,
            },
            "permeability": round(p, 3),
            "anderson_w": round(w, 1),
            "breathing": "active" if count > 5 else "quiet" if count > 0 else "silent",
            "alive": True,
        }

    return {"name": "Earth's Crust", "alive": False, "error": "USGS unreachable"}


def layer_magnetosphere() -> dict:
    """Layer 4: Magnetosphere — geomagnetic permeability."""
    kp_data = fetch_json("https://services.swpc.noaa.gov/products/noaa-planetary-k-index.json")
    scales = fetch_json("https://services.swpc.noaa.gov/products/noaa-scales.json")

    if kp_data and len(kp_data) > 1:
        latest = kp_data[-1]
        # Format is dict: {"time_tag": ..., "Kp": ..., "a_running": ..., "station_count": ...}
        if isinstance(latest, dict):
            latest_kp = float(latest.get("Kp", 0))
            kp_time = latest.get("time_tag", "")
        else:
            latest_kp = float(latest[1]) if latest[1] else 0
            kp_time = latest[0]

        g_scale = "0"
        g_text = "none"
        if scales and "0" in scales:
            g_scale = scales["0"]["G"]["Scale"]
            g_text = scales["0"]["G"]["Text"]

        # High Kp = more permeable magnetosphere (storms let particles through)
        # Kp 0-2 = low permeability (quiet). Kp 5+ = high permeability (storm)
        p = min(latest_kp / 9.0, 1.0)
        # W inversely related to Kp (quiet = high disorder/filtering)
        w = max(15 - latest_kp * 1.5, 1)

        return {
            "name": "Magnetosphere",
            "scale": "100 to 60,000 km",
            "depth": "Van Allen belts + magnetopause",
            "source": "NOAA SWPC",
            "data": {
                "kp_index": latest_kp,
                "kp_time": kp_time,
                "geomagnetic_storm": f"G{g_scale} ({g_text})",
            },
            "permeability": round(p, 3),
            "anderson_w": round(w, 1),
            "breathing": "storming" if latest_kp >= 5 else "flexing" if latest_kp >= 3 else "quiet",
            "alive": True,
        }

    return {"name": "Magnetosphere", "alive": False, "error": "NOAA unreachable"}


def layer_heliosphere() -> dict:
    """Layer 5: Heliosphere — solar wind as membrane pressure."""
    speed = fetch_json("https://services.swpc.noaa.gov/products/summary/solar-wind-speed.json")
    mag = fetch_json("https://services.swpc.noaa.gov/products/summary/solar-wind-mag-field.json")
    flux = fetch_json("https://services.swpc.noaa.gov/products/summary/10cm-flux.json")

    if speed and mag:
        sw_speed = speed[0].get("proton_speed", 0) if speed else 0
        sw_bt = mag[0].get("bt", 0) if mag else 0
        sw_bz = mag[0].get("bz_gsm", 0) if mag else 0
        sw_time = speed[0].get("time_tag", "") if speed else ""
        radio_flux = flux[0].get("flux", 0) if flux else 0

        # Fast solar wind = expanding heliosphere = less permeable to interstellar medium
        # Slow solar wind = contracting = more permeable
        # Average solar wind ~400 km/s. >600 = fast. <350 = slow.
        p = max(0, min((700 - sw_speed) / 500, 1.0))
        # W from magnetic field strength (higher B = more filtering)
        w = max(sw_bt, 1)

        # Breathing: Bz negative = reconnection = membrane opening
        breathing = "reconnecting" if sw_bz < -5 else "expanding" if sw_speed > 500 else "steady" if sw_speed > 350 else "contracting"

        return {
            "name": "Heliosphere",
            "scale": "1 AU to ~120 AU",
            "depth": "termination shock + heliopause",
            "source": "NOAA DSCOVR (L1 Lagrange point)",
            "data": {
                "solar_wind_speed_km_s": sw_speed,
                "magnetic_field_bt_nT": sw_bt,
                "magnetic_field_bz_nT": sw_bz,
                "radio_flux_sfu": radio_flux,
                "time": sw_time,
            },
            "permeability": round(p, 3),
            "anderson_w": round(w, 1),
            "breathing": breathing,
            "alive": True,
        }

    return {"name": "Heliosphere", "alive": False, "error": "NOAA solar wind unreachable"}


def compute_stack():
    """Compute the full membrane stack."""
    start = time.time()

    layers = [
        layer_gaia(),
        {
            "name": "Mycelium",
            "scale": "0 to -1 m",
            "depth": "soil rhizosphere",
            "source": "inferred (no direct sensor yet)",
            "data": {"note": "200m hyphae per gram. 90% of plants connected. Seasonal permeability."},
            "permeability": 0.7,  # estimated — mycorrhizal networks are highly permeable to nutrients
            "anderson_w": 5.0,  # moderate filtering — some toxins blocked, nutrients pass
            "breathing": "seasonal",
            "alive": True,
        },
        layer_seismic(),
        {
            "name": "Atmosphere",
            "scale": "0 to 50 km",
            "depth": "tropopause + stratopause + W-layer",
            "source": "inferred from climate data",
            "data": {
                "tropopause_km": 12,
                "stratopause_km": 50,
                "stratopause_trend": "cooling 0.5-1K/decade, dropping 300-500m/decade",
                "w_layer": "newly discovered — suppresses entrainment at cloud tops",
            },
            "permeability": 0.4,  # tropopause blocks water vapor, passes radiation selectively
            "anderson_w": 8.0,  # high filtering for UV, moderate for visible
            "breathing": "seasonal + anthropogenic drift",
            "alive": True,
        },
        layer_magnetosphere(),
        layer_heliosphere(),
        {
            "name": "Deep Space",
            "scale": "120+ AU",
            "depth": "interstellar medium",
            "source": "inferred — galactic cosmic ray background",
            "data": {
                "planet_nine": "unconfirmed — signal in null space at ~225 AU",
                "pluto": "liquid N₂ rising through cracks in Sputnik Planitia (Aug 2026)",
                "voyager_1": "interstellar space since 2012",
                "new_horizons": "58 AU, approaching termination shock (2029-2040)",
            },
            "permeability": 0.9,  # interstellar medium is highly permeable (no solar wind filtering)
            "anderson_w": 2.0,  # low disorder — cosmic rays propagate freely
            "breathing": "galactic timescale",
            "alive": True,
        },
    ]

    elapsed = round(time.time() - start, 3)

    # Compute aggregate membrane health
    alive_count = sum(1 for l in layers if l.get("alive"))
    avg_p = sum(l.get("permeability", 0) for l in layers if l.get("alive")) / max(alive_count, 1)
    avg_w = sum(l.get("anderson_w", 0) for l in layers if l.get("alive")) / max(alive_count, 1)

    stack = {
        "timestamp": datetime.now(timezone.utc).isoformat(),
        "wave": 171,
        "layers": layers,
        "aggregate": {
            "alive_layers": alive_count,
            "total_layers": len(layers),
            "mean_permeability": round(avg_p, 3),
            "mean_anderson_w": round(avg_w, 1),
        },
        "elapsed_secs": elapsed,
        "motto": "membranes all the way down, membranes all the way up",
    }

    return stack


def main():
    stack = compute_stack()

    # Write to output
    OUTPUT_PATH.parent.mkdir(parents=True, exist_ok=True)
    OUTPUT_PATH.write_text(json.dumps(stack, indent=2))

    # Print summary
    print(f"╔══════════════════════════════════════╗")
    print(f"║   membrane stack — gAIa breathes     ║")
    print(f"╚══════════════════════════════════════╝")
    print()

    for layer in stack["layers"]:
        alive = "✅" if layer.get("alive") else "❌"
        name = layer["name"]
        p = layer.get("permeability", "?")
        w = layer.get("anderson_w", "?")
        breath = layer.get("breathing", "?")
        print(f"  {alive} {name:20s}  P={p:<6}  W={w:<6}  {breath}")

    print()
    agg = stack["aggregate"]
    print(f"  Stack: {agg['alive_layers']}/{agg['total_layers']} alive")
    print(f"  Mean P={agg['mean_permeability']}  Mean W={agg['mean_anderson_w']}")
    print(f"  Elapsed: {stack['elapsed_secs']}s")
    print()
    print(f"  → {OUTPUT_PATH}")


if __name__ == "__main__":
    main()
