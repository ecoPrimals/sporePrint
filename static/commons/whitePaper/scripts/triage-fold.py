#!/usr/bin/env python3
"""
triage-fold.py — Crush the hard, thin the fat, feed the skinny, wake the sleeping.
Find the lost. Rephrase. Agglutinate. Pass it back through the Klein self.

This is the digestive system. The organism eats itself and becomes stronger.

Author: Kevin Mok / ecoPrimal / ORCID 0009-0004-2141-0321
"""

import os
import sys
import json
import time
import math
from pathlib import Path
from collections import defaultdict

BASE = Path(sys.argv[1]) if len(sys.argv) > 1 else Path(".")
MEMBRANE = BASE / "membrane" / "local-terminal" / "remote"
SKIP = {".git", "node_modules", "target", ".mypy_cache", "__pycache__", ".next"}
CODE_EXT = {".rs", ".py", ".sh", ".toml", ".js", ".ts"}
TEXT_EXT = CODE_EXT | {".md", ".json", ".html", ".txt", ".yaml", ".yml"}
DAY = 86400
NOW = time.time()


def scan():
    files = []
    for root, dirs, fnames in os.walk(BASE):
        dirs[:] = [d for d in dirs if d not in SKIP]
        for f in fnames:
            p = os.path.join(root, f)
            try:
                st = os.stat(p)
                ext = os.path.splitext(f)[1].lower()
                files.append({
                    "path": p,
                    "size": st.st_size,
                    "age_days": (NOW - st.st_mtime) / DAY,
                    "ext": ext,
                    "is_code": ext in CODE_EXT,
                    "is_text": ext in TEXT_EXT,
                })
            except (OSError, PermissionError):
                pass
    return files


def triage(files):
    hard = []       # stuck: big + old code
    fat = []        # oversized text
    skinny = []     # underweight code stubs
    sleeping = []   # >180 days dormant
    lost_dirs = []  # orphan directories (1 file)
    agglutinate = defaultdict(list)  # dirs with same basename

    dir_counts = defaultdict(int)
    for f in files:
        d = os.path.dirname(f["path"])
        dir_counts[d] += 1

        if f["is_code"] and f["size"] > 10_000 and f["age_days"] > 90:
            hard.append(f)
        if f["is_text"] and f["size"] > 100_000:
            fat.append(f)
        if f["is_code"] and f["size"] < 100:
            skinny.append(f)
        if f["age_days"] > 180:
            sleeping.append(f)

    for d, c in dir_counts.items():
        if c == 1 and len(d.split("/")) > 4:
            lost_dirs.append(d)
        name = os.path.basename(d)
        if name and len(name) > 3 and name not in SKIP | {"src", "tests", "crates", "bin", "docs"}:
            agglutinate[name].append(d)

    dupes = {k: v for k, v in agglutinate.items() if len(v) > 1}

    hard.sort(key=lambda f: -(f["size"] * f["age_days"]))
    fat.sort(key=lambda f: -f["size"])
    skinny.sort(key=lambda f: f["size"])
    sleeping.sort(key=lambda f: -f["age_days"])

    return hard, fat, skinny, sleeping, sorted(lost_dirs), dupes


def flavor_triage(hard, fat, skinny, sleeping, lost, dupes, total):
    """The triage itself has a taste."""
    sweet = 0.0   # things that nourish
    sour = 0.0    # things that extract
    spicy = 0.0   # things that provoke action
    bitter = 0.0  # things that are toxic
    umami = 0.0   # things with complex depth

    # Skinny files are sweet — they're stubs waiting to grow, pure potential
    sweet += len(skinny) / total if total else 0

    # Sleeping files are umami — aged depth, fermented knowledge
    umami += len(sleeping) / total if total else 0

    # Fat files are sour — extraction candidates, bloat that takes space
    sour += len(fat) / total if total else 0

    # Hard files are spicy — they provoke: deal with me or don't
    spicy += len(hard) / total if total else 0

    # Lost dirs are bitter — orphaned, potentially toxic dead ends
    bitter += len(lost) / total if total else 0

    return {
        "sweet": round(sweet, 4),
        "sour": round(sour, 4),
        "spicy": round(spicy, 4),
        "bitter": round(bitter, 4),
        "umami": round(umami, 4),
    }


def tense_triage(hard, fat, skinny, sleeping, lost):
    """What tense is each category in?"""
    return {
        "hard": "WAS",           # was active, now stuck in the past
        "fat": "IS",             # is oversized right now
        "skinny": "COULD",       # could become something
        "sleeping": "MIGHT",     # might wake up, might not
        "lost": "MIGHTN'T",     # might never be found
    }


def pi_structure(counts):
    """The π-structure of the triage."""
    total = sum(counts.values())
    if total == 0:
        return {}
    proportions = {k: v / total for k, v in counts.items()}
    # How far from uniform?
    n = len(counts)
    uniform = 1 / n if n else 0
    divergence = sum(abs(p - uniform) for p in proportions.values())
    pi_rem = divergence % math.pi
    return {
        "proportions": {k: round(v, 4) for k, v in proportions.items()},
        "divergence": round(divergence, 4),
        "pi_remainder": round(pi_rem, 4),
        "ratio_to_ln2": round(divergence / math.log(2), 4) if divergence else 0,
    }


def fold_back(result, prev_path):
    """Compare to previous triage — the Klein fold."""
    delta = {"drift": "genesis", "changes": []}
    if prev_path.exists():
        try:
            prev = json.loads(prev_path.read_text())
            delta["drift"] = "stable"
            for key in ["hard", "fat", "skinny", "sleeping", "lost_dirs"]:
                old = prev.get("counts", {}).get(key, 0)
                new = result["counts"].get(key, 0)
                d = new - old
                if d != 0:
                    delta["drift"] = "drifting"
                    delta["changes"].append({
                        "category": key,
                        "was": old,
                        "now": new,
                        "delta": d,
                        "direction": "growing" if d > 0 else "shrinking",
                    })
        except (json.JSONDecodeError, KeyError):
            delta["drift"] = "corrupt_previous"
    return delta


def main():
    files = scan()
    hard, fat, skinny, sleeping, lost, dupes = triage(files)

    counts = {
        "hard": len(hard),
        "fat": len(fat),
        "skinny": len(skinny),
        "sleeping": len(sleeping),
        "lost_dirs": len(lost),
        "agglutinate": len(dupes),
        "total": len(files),
    }

    result = {
        "ts": time.strftime("%Y-%m-%dT%H:%M:%S%z"),
        "counts": counts,
        "flavor": flavor_triage(hard, fat, skinny, sleeping, lost, dupes, len(files)),
        "tense": tense_triage(hard, fat, skinny, sleeping, lost),
        "pi": pi_structure(counts),
        "top_hard": [{"path": f["path"], "size": f["size"], "age": round(f["age_days"])}
                     for f in hard[:5]],
        "top_fat": [{"path": f["path"], "size": f["size"]}
                    for f in fat[:5]],
        "top_skinny": [{"path": f["path"], "size": f["size"]}
                       for f in skinny[:5]],
        "sleep_summary": {},
        "fold": "output→input",
    }

    # Sleeping grouped by top-level
    sleep_dirs = defaultdict(int)
    for f in sleeping:
        parts = f["path"].split("/")
        key = "/".join(parts[:3]) if len(parts) > 2 else f["path"]
        sleep_dirs[key] += 1
    result["sleep_summary"] = dict(sorted(sleep_dirs.items(), key=lambda x: -x[1])[:10])

    # Klein fold
    prev_path = MEMBRANE / "triage-prev.json"
    state_path = MEMBRANE / "triage-state.json"
    log_path = MEMBRANE / "triage-log.jsonl"

    if state_path.exists():
        import shutil
        shutil.copy2(state_path, prev_path)

    delta = fold_back(result, prev_path)
    result["delta"] = delta

    MEMBRANE.mkdir(parents=True, exist_ok=True)
    state_path.write_text(json.dumps(result, indent=2))

    log_entry = {
        "ts": result["ts"],
        "total": len(files),
        **counts,
        "drift": delta["drift"],
        "flavor_dominant": max(result["flavor"], key=result["flavor"].get),
    }
    with open(log_path, "a") as lf:
        lf.write(json.dumps(log_entry) + "\n")

    # Print
    print("═══ TRIAGE FOLD ═══")
    print()
    for k, v in counts.items():
        pct = v / len(files) * 100 if len(files) else 0
        bar = "█" * int(pct / 2)
        print(f"  {k:>12}: {v:>6,}  ({pct:>5.1f}%)  {bar}")
    print()
    print(f"  flavor: {result['flavor']}")
    print(f"  tense:  {result['tense']}")
    print(f"  π-rem:  {result['pi']['pi_remainder']}")
    print(f"  drift:  {delta['drift']}")
    if delta["changes"]:
        for c in delta["changes"]:
            print(f"    {c['category']}: {c['was']} → {c['now']} ({c['direction']})")
    print()
    print(f"  → {state_path}")


if __name__ == "__main__":
    main()
