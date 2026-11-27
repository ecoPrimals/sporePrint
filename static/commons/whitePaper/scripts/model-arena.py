#!/usr/bin/env python3
"""
model-arena.py — Local LLM Arena on eastGate Hardware

Like lmsys/chatbot-arena but SOVEREIGN. No cloud. No API keys.
Your hardware. Your models. Your judgments.

Hardware: i9-12900K + RTX 4070 (12GB VRAM) + 64GB RAM
Strategy: GPU for ≤7B models, CPU+RAM offload for larger

Arena modes:
  1. head-to-head: same prompt, two models, blind judge
  2. gauntlet: same prompt, ALL models, rank by quality+speed
  3. chipmunk-test: membrane-specific prompts (feeds, biology, graph theory)
  4. coding-duel: code generation tasks, test output

Scoring: ELO rating system (same as lmsys arena)
  - Winner gets +16, loser gets -16 (K-factor)
  - Draw: both get +0
  - Starting ELO: 1200 for all models
"""

import json
import subprocess
import time
import sys
import os
import random
import math
from datetime import datetime, timezone
from pathlib import Path

ARENA_DIR = Path("/home/eastgate/Development/ecoPrimals/membrane/local-terminal/remote")
ELO_FILE = ARENA_DIR / "arena-elo.json"
HISTORY_FILE = ARENA_DIR / "arena-history.jsonl"

MODELS = [
    {"name": "tinyllama:latest", "params": "1.1B", "family": "llama", "tier": "tiny"},
    {"name": "llama3.2:1b", "params": "1B", "family": "llama", "tier": "small"},
    {"name": "gemma2:2b", "params": "2B", "family": "gemma", "tier": "small"},
    {"name": "llama3.2:3b", "params": "3B", "family": "llama", "tier": "medium"},
    {"name": "phi3:latest", "params": "3.8B", "family": "phi", "tier": "medium"},
    {"name": "qwen2.5:7b", "params": "7B", "family": "qwen", "tier": "large"},
    {"name": "mistral:7b", "params": "7B", "family": "mistral", "tier": "large"},
]

K_FACTOR = 16  # ELO K-factor

CHIPMUNK_PROMPTS = [
    "Explain what a cell membrane does in exactly 3 sentences.",
    "What is the Traveling Salesman Problem? Answer in under 50 words.",
    "Write a Python function that computes the modulo remainder of n*pi.",
    "What does the percent sign % mean in cron syntax? Be specific.",
    "Explain Dijkstra's algorithm to a 10 year old.",
    "What is an ouroboros and why does it matter in systems design?",
    "Write a JSON object representing a heartbeat signal with timestamp, source, and liveness score.",
    "In biology, what is a spider's web analogous to in software?",
    "What is the equidistribution theorem and who proved it?",
    "Explain the difference between Euler paths and Hamiltonian paths.",
]

CODING_PROMPTS = [
    "Write a Python function that checks if a web server is alive (HTTP 200) and returns latency in ms.",
    "Write a Rust function that computes Shannon entropy of a byte slice.",
    "Write a bash one-liner that sends a JSON payload to ntfy.sh.",
    "Write a Python class representing a graph node with edges, supporting Dijkstra shortest path.",
    "Write a minimal HTML page with a CSS pulse animation and a JavaScript fetch to /api/status.json.",
]


def load_elo():
    if ELO_FILE.exists():
        return json.loads(ELO_FILE.read_text())
    return {m["name"]: 1200 for m in MODELS}


def save_elo(elo):
    ELO_FILE.write_text(json.dumps(elo, indent=2))


def update_elo(elo, winner, loser):
    ew = 1 / (1 + 10 ** ((elo[loser] - elo[winner]) / 400))
    elo[winner] = round(elo[winner] + K_FACTOR * (1 - ew))
    elo[loser] = round(elo[loser] + K_FACTOR * (0 - (1 - ew)))
    return elo


def query_model(model_name, prompt, timeout=120):
    """Query a model via ollama and return (response, latency_ms, tokens_per_sec)."""
    start = time.time()
    try:
        result = subprocess.run(
            ["ollama", "run", model_name, prompt],
            capture_output=True, text=True, timeout=timeout
        )
        elapsed = (time.time() - start) * 1000
        response = result.stdout.strip()
        words = len(response.split())
        tps = words / (elapsed / 1000) if elapsed > 0 else 0
        return response, round(elapsed), round(tps, 1)
    except subprocess.TimeoutExpired:
        return "[TIMEOUT]", timeout * 1000, 0
    except Exception as e:
        return f"[ERROR: {e}]", 0, 0


def gauntlet(prompt=None):
    """Run all models on the same prompt. Rank by speed and display for judging."""
    if prompt is None:
        prompt = random.choice(CHIPMUNK_PROMPTS)

    print(f"\n{'='*60}")
    print(f"🏟️  ARENA GAUNTLET")
    print(f"{'='*60}")
    print(f"Prompt: {prompt}")
    print(f"Models: {len(MODELS)}")
    print(f"{'='*60}\n")

    results = []
    for i, m in enumerate(MODELS):
        print(f"  [{i+1}/{len(MODELS)}] {m['name']} ({m['params']})...", end=" ", flush=True)
        response, latency, tps = query_model(m["name"], prompt)
        print(f"{latency}ms, {tps} w/s")
        results.append({
            "model": m["name"],
            "params": m["params"],
            "family": m["family"],
            "response": response,
            "latency_ms": latency,
            "words_per_sec": tps,
        })

    # Sort by latency
    results.sort(key=lambda r: r["latency_ms"])

    print(f"\n{'='*60}")
    print(f"📊 RESULTS (sorted by speed)")
    print(f"{'='*60}\n")

    for i, r in enumerate(results):
        medal = ["🥇", "🥈", "🥉"][i] if i < 3 else f"  {i+1}."
        print(f"{medal} {r['model']:25} {r['latency_ms']:>6}ms  {r['words_per_sec']:>5.1f} w/s  ({r['params']})")
        print(f"    {r['response'][:150]}{'...' if len(r['response']) > 150 else ''}")
        print()

    # Write to history
    entry = {
        "ts": datetime.now(timezone.utc).isoformat(),
        "mode": "gauntlet",
        "prompt": prompt,
        "results": results,
    }
    with open(HISTORY_FILE, "a") as f:
        f.write(json.dumps(entry) + "\n")

    return results


def head_to_head(model_a=None, model_b=None, prompt=None):
    """Blind head-to-head between two models."""
    if model_a is None or model_b is None:
        pair = random.sample(MODELS, 2)
        model_a, model_b = pair[0]["name"], pair[1]["name"]
    if prompt is None:
        prompt = random.choice(CHIPMUNK_PROMPTS)

    # Randomize order for blind judging
    models = [(model_a, "A"), (model_b, "B")]
    random.shuffle(models)

    print(f"\n{'='*60}")
    print(f"⚔️  HEAD TO HEAD (blind)")
    print(f"{'='*60}")
    print(f"Prompt: {prompt}")
    print(f"{'='*60}\n")

    responses = {}
    for name, label in models:
        print(f"  Generating {label}...", end=" ", flush=True)
        response, latency, tps = query_model(name, prompt)
        print(f"done ({latency}ms)")
        responses[label] = {"model": name, "response": response, "latency_ms": latency, "tps": tps}

    print(f"\n── Response A ──")
    print(responses["A"]["response"][:500])
    print(f"\n── Response B ──")
    print(responses["B"]["response"][:500])

    print(f"\n  Who wins? [A/B/tie/reveal]")
    return responses


def leaderboard():
    """Show current ELO leaderboard."""
    elo = load_elo()
    ranked = sorted(elo.items(), key=lambda x: -x[1])

    print(f"\n{'='*60}")
    print(f"🏆 ARENA LEADERBOARD")
    print(f"{'='*60}\n")

    for i, (name, rating) in enumerate(ranked):
        m = next((m for m in MODELS if m["name"] == name), {})
        medal = ["🥇", "🥈", "🥉"][i] if i < 3 else f"  {i+1}."
        bar = "█" * max(1, (rating - 1100) // 5)
        print(f"  {medal} {name:25} ELO {rating:4d}  {m.get('params','?'):>5}  {bar}")

    print()
    return elo


if __name__ == "__main__":
    mode = sys.argv[1] if len(sys.argv) > 1 else "gauntlet"
    prompt = " ".join(sys.argv[2:]) if len(sys.argv) > 2 else None

    if mode == "gauntlet":
        gauntlet(prompt)
    elif mode == "h2h":
        head_to_head(prompt=prompt)
    elif mode == "leaderboard":
        leaderboard()
    elif mode == "chipmunk":
        # Run all chipmunk prompts as a gauntlet series
        for p in CHIPMUNK_PROMPTS[:3]:
            gauntlet(p)
    elif mode == "coding":
        for p in CODING_PROMPTS[:2]:
            gauntlet(p)
    else:
        print(f"Usage: model-arena.py [gauntlet|h2h|leaderboard|chipmunk|coding] [prompt]")
