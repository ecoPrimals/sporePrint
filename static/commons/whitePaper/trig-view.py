#!/usr/bin/env python3
"""trig-view.py — trigonometric projection of the torus cross-section.

Place every file at an ANGLE θ on the torus cross-section using:
  θ = atan2(direction, π - distance)

Where:
  distance  = radial (0 = core, π = edge)
  direction = angular (0 = produce, π = consume)

Then decompose with SOHCAHTOA:
  sin(θ) = direction / hypotenuse    — "how much is it facing outward?"
  cos(θ) = (π - distance) / hyp      — "how close to the core?"
  tan(θ) = direction / (π - distance) — "the lean: how much direction per unit of closeness"

Bin by standard angles where sin/cos/tan have exact π-values:
  0°    = 0      : pure core, no direction
  30°   = π/6    : mostly core, a little direction
  45°   = π/4    : EQUAL core and direction (the balance point)
  60°   = π/3    : mostly direction, a little core
  90°   = π/2    : pure direction, no core
  120°  = 2π/3   : direction + ANTI-core (the Klein fold begins)
  135°  = 3π/4   : equal anti-core and direction
  150°  = 5π/6   : mostly anti-core
  180°  = π      : pure anti-core (maximum edge)

The angles past 90° are where cos goes NEGATIVE — the outside becomes the inside.
That's the Klein fold. Files at 2π/3 are folding BACK through the surface.

Usage: python3 trig-view.py [repo_path]
"""

import os, sys, math, json
from collections import Counter, defaultdict
from pathlib import Path
from datetime import datetime, timezone

PI = math.pi

LANG_MAP = {
    '.rs': 'rust', '.js': 'js', '.py': 'python', '.sh': 'shell',
    '.toml': 'toml', '.md': 'markdown', '.html': 'html'
}
SKIP = {'target', '.git', 'node_modules', '__pycache__'}

def shannon(text):
    words = text.lower().split()
    if not words: return 0.0
    freq = Counter(words)
    total = len(words)
    h = 0.0
    for count in freq.values():
        p = count / total
        if p > 0: h -= p * math.log2(p)
    return h

def sigmoid(x, mid=0.5, k=6.0):
    z = k * (x - mid)
    if z > 500: return 1.0
    if z < -500: return 0.0
    return 1.0 / (1.0 + math.exp(-z))

def file_age_days(path):
    try:
        mtime = path.stat().st_mtime
        now = datetime.now(timezone.utc).timestamp()
        return (now - mtime) / 86400
    except Exception:
        return 9999

# ═══════════════════════════════════════════════════════════════
# DISTANCE & DIRECTION (same as sideview.py)
# ═══════════════════════════════════════════════════════════════

def distance(types, functions, imports, lines):
    if lines == 0: return PI
    skeleton = sigmoid(types / 10.0, 0.5, 5)
    appendages = sigmoid(functions / 20.0, 0.5, 4)
    wiring = sigmoid(imports / 8.0, 0.5, 4)
    core_proximity = (skeleton * 0.5 + appendages * 0.3 + wiring * 0.2)
    return PI * (1.0 - core_proximity)

def direction(types, functions, imports):
    production = types + functions * 0.3
    consumption = imports
    total = production + consumption
    if total == 0: return PI / 2
    outward_ratio = consumption / total
    return PI * outward_ratio

# ═══════════════════════════════════════════════════════════════
# TRIGONOMETRIC PROJECTION
#
# Convert (distance, direction) → angle θ on the cross-section.
# Use atan2 so the angle is in [0, π] (half-circle = half the torus).
#
# x-axis = (π - distance) = "closeness to core" (positive = close)
# y-axis = direction = "outward lean" (0 = produce, π = consume)
#
# θ = atan2(direction, π - distance)
# This gives us the angle from the "core" axis toward the "consume" axis.
# ═══════════════════════════════════════════════════════════════

def theta(dist, dirn):
    """Angle on the torus cross-section. Returns [0, π]."""
    x = PI - dist   # closeness: positive = near core, negative = far from core
    y = dirn         # consumption direction: 0 = produce, π = consume
    angle = math.atan2(y, x)
    if angle < 0:
        angle += 2 * PI
    # Clamp to [0, π] — we only need the upper half of the circle
    # (lower half mirrors: consume-and-produce is same as produce-and-consume)
    if angle > PI:
        angle = 2 * PI - angle
    return angle

# ═══════════════════════════════════════════════════════════════
# STANDARD ANGLE BINS
# Bin boundaries at the exact angles where trig functions have clean values
# ═══════════════════════════════════════════════════════════════

# The 9 standard angles (in radians)
ANGLES = [
    (0,       "0°",   "0"),
    (PI/6,    "30°",  "π/6"),
    (PI/4,    "45°",  "π/4"),
    (PI/3,    "60°",  "π/3"),
    (PI/2,    "90°",  "π/2"),
    (2*PI/3,  "120°", "2π/3"),
    (3*PI/4,  "135°", "3π/4"),
    (5*PI/6,  "150°", "5π/6"),
    (PI,      "180°", "π"),
]

# Bin edges: midpoints between standard angles
BIN_EDGES = []
for i in range(len(ANGLES) - 1):
    mid = (ANGLES[i][0] + ANGLES[i+1][0]) / 2
    BIN_EDGES.append(mid)

def angle_bin(th):
    """Which standard angle bin does this θ belong to? Returns index 0-8."""
    for i, edge in enumerate(BIN_EDGES):
        if th < edge:
            return i
    return len(ANGLES) - 1

# ═══════════════════════════════════════════════════════════════
# MAIN
# ═══════════════════════════════════════════════════════════════

def main():
    repo = Path(sys.argv[1]) if len(sys.argv) > 1 else Path('.')
    repo = repo.resolve()
    print(f"[trig-view] projecting {repo} onto the unit circle...")
    print()

    files = []
    angle_counts = Counter()
    lang_angle = defaultdict(Counter)
    angle_lines = defaultdict(int)

    # Accumulators for sin/cos/tan per bin
    angle_sin_sum = defaultdict(float)
    angle_cos_sum = defaultdict(float)

    for path in sorted(repo.rglob('*')):
        if not path.is_file(): continue
        if any(part in SKIP for part in path.parts): continue
        ext = path.suffix.lower()
        lang = LANG_MAP.get(ext)
        if not lang: continue

        try:
            text = path.read_text(errors='replace')
        except Exception:
            continue

        lines_list = text.splitlines()
        num_lines = len(lines_list)
        if num_lines == 0: continue

        # Count features
        types = 0; funcs = 0; imports = 0
        if lang == 'rust':
            for line in lines_list:
                s = line.strip()
                if s.startswith(('pub enum ', 'enum ', 'pub struct ', 'struct ')): types += 1
                if s.startswith(('pub fn ', 'fn ', 'pub async fn ')): funcs += 1
                if s.startswith('use '): imports += 1
        elif lang == 'python':
            for line in lines_list:
                s = line.strip()
                if s.startswith('class '): types += 1
                if s.startswith('def '): funcs += 1
                if s.startswith(('import ', 'from ')): imports += 1
        elif lang == 'js':
            for line in lines_list:
                s = line.strip()
                if 'function ' in s or s.startswith(('var ', 'const ')): funcs += 1
                if 'require(' in s or 'import ' in s: imports += 1
        elif lang == 'shell':
            for line in lines_list:
                s = line.strip()
                if '()' in s and '{' in s: funcs += 1
                if s.startswith(('source ', '. ')): imports += 1

        d = distance(types, funcs, imports, num_lines)
        dr = direction(types, funcs, imports)
        th = theta(d, dr)
        ab = angle_bin(th)

        angle_counts[ab] += 1
        lang_angle[lang][ab] += 1
        angle_lines[ab] += num_lines
        angle_sin_sum[ab] += math.sin(th)
        angle_cos_sum[ab] += math.cos(th)

        files.append({
            'path': str(path.relative_to(repo)),
            'lang': lang,
            'lines': num_lines,
            'theta': round(th, 4),
            'sin': round(math.sin(th), 4),
            'cos': round(math.cos(th), 4),
            'tan': round(math.tan(th), 4) if abs(math.cos(th)) > 0.001 else float('inf'),
            'angle_bin': ab,
            'distance': round(d, 4),
            'direction': round(dr, 4),
        })

    total = len(files)
    print(f"  TOTAL: {total:,} files on the unit circle")
    print()

    # ── SOHCAHTOA TABLE ──
    print(f"  ═══ SOHCAHTOA — standard angles on the torus cross-section ═══")
    print(f"  sin = outward lean / hypotenuse (how much it faces outward)")
    print(f"  cos = core closeness / hypotenuse (how close to skeleton)")
    print(f"  tan = lean / closeness (the ratio — how much direction per unit of core)")
    print()
    print(f"  {'ANGLE':>6} {'RAD':>5} {'sin':>7} {'cos':>7} {'tan':>7}  {'FILES':>7} {'%':>6} {'LINES':>10}  {'CHARACTER'}")
    print(f"  {'─'*6} {'─'*5} {'─'*7} {'─'*7} {'─'*7}  {'─'*7} {'─'*6} {'─'*10}  {'─'*35}")

    chars = [
        "pure core, no lean — the skeleton itself",
        "mostly core, slight lean — strong spine",
        "EQUAL core & lean — the 45° balance point",
        "mostly lean, some core — directed purpose",
        "pure lean, no core — all direction, no anchor",
        "lean + ANTI-core — the Klein fold begins",
        "equal anti — the mirror point (3π/4)",
        "mostly anti-core — the far edge",
        "pure anti-core — maximum edge, the surface",
    ]

    max_count = max(angle_counts.values()) if angle_counts else 1
    for i, (rad, deg, sym) in enumerate(ANGLES):
        c = angle_counts.get(i, 0)
        pct = c / total * 100 if total else 0
        lines = angle_lines.get(i, 0)
        s = math.sin(rad)
        co = math.cos(rad)
        t = math.tan(rad) if abs(math.cos(rad)) > 0.001 else float('inf')
        t_str = f"{t:>7.3f}" if abs(t) < 100 else f"{'∞':>7}"
        bar = '█' * int(c / max_count * 25)
        print(f"  {deg:>6} {sym:>5} {s:>7.3f} {co:>7.3f} {t_str}  {c:>7,} {pct:>5.1f}% {lines:>10,}  {bar} {chars[i]}")
    print()

    # ── WHERE THE FILES ACTUALLY CLUSTER ──
    print(f"  ═══ ACTUAL θ DISTRIBUTION (10° bins across [0°, 180°]) ═══\n")
    fine_bins = Counter()
    for f in files:
        deg = int(f['theta'] * 180 / PI)
        fine_bins[deg // 10] += 1

    max_fine = max(fine_bins.values()) if fine_bins else 1
    for i in range(18):
        c = fine_bins.get(i, 0)
        bar = '█' * int(c / max_fine * 40)
        lo = i * 10
        hi = (i + 1) * 10
        marker = ""
        if lo <= 0 < hi: marker = " ← 0°"
        if lo <= 30 < hi: marker = " ← π/6 (30°)"
        if lo <= 45 < hi: marker = " ← π/4 (45°)"
        if lo <= 60 < hi: marker = " ← π/3 (60°)"
        if lo <= 90 < hi: marker = " ← π/2 (90°)"
        if lo <= 120 < hi: marker = " ← 2π/3 (120°)"
        if lo <= 135 < hi: marker = " ← 3π/4 (135°)"
        if lo <= 150 < hi: marker = " ← 5π/6 (150°)"
        if lo <= 180 <= hi: marker = " ← π (180°)"
        print(f"  {lo:>3}°-{hi:<3}°  {c:>6,}  {bar}{marker}")
    print()

    # ── LANGUAGE × ANGLE ──
    print(f"  ═══ LANGUAGE × ANGLE (where each language sits on the circle) ═══\n")
    header = f"  {'LANG':<10}" + "".join(f"  {ANGLES[i][1]:>6}" for i in range(len(ANGLES)))
    print(header)
    print(f"  {'─'*10}" + "─" * 8 * len(ANGLES))
    for lang in ['rust', 'js', 'python', 'shell', 'toml', 'markdown', 'html']:
        row = f"  {lang:<10}"
        for i in range(len(ANGLES)):
            c = lang_angle[lang].get(i, 0)
            row += f"  {c:>6,}" if c else f"  {'·':>6}"
        print(row)
    print()

    # ── MEAN SIN/COS per angle bin (actual vs theoretical) ──
    print(f"  ═══ MEAN sin(θ) and cos(θ) per bin — actual vs theoretical ═══\n")
    print(f"  {'ANGLE':>6}  {'sin(theory)':>11} {'sin(actual)':>11} {'Δsin':>7}  {'cos(theory)':>11} {'cos(actual)':>11} {'Δcos':>7}")
    print(f"  {'─'*6}  {'─'*11} {'─'*11} {'─'*7}  {'─'*11} {'─'*11} {'─'*7}")
    for i, (rad, deg, sym) in enumerate(ANGLES):
        c = angle_counts.get(i, 0)
        if c == 0:
            continue
        mean_sin = angle_sin_sum[i] / c
        mean_cos = angle_cos_sum[i] / c
        th_sin = math.sin(rad)
        th_cos = math.cos(rad)
        d_sin = mean_sin - th_sin
        d_cos = mean_cos - th_cos
        print(f"  {deg:>6}  {th_sin:>11.4f} {mean_sin:>11.4f} {d_sin:>+7.4f}  {th_cos:>11.4f} {mean_cos:>11.4f} {d_cos:>+7.4f}")
    print()

    # ── THE 45° LINE: perfect balance ──
    balance_count = angle_counts.get(2, 0)  # bin 2 = 45°
    total_pct = balance_count / total * 100 if total else 0
    print(f"  ═══ THE 45° LINE — π/4 — the perfect balance ═══\n")
    print(f"  Files at 45° (equal core distance and direction): {balance_count:,} ({total_pct:.1f}%)")
    print(f"  sin(45°) = cos(45°) = √2/2 ≈ {math.sqrt(2)/2:.4f}")
    print(f"  tan(45°) = 1 — the only angle where tangent equals unity")
    print(f"  These files have EQUAL skeleton proximity and directional lean.")
    print(f"  Neither pulled toward the core nor pushed to the edge.")
    print(f"  Neither producer nor consumer. The balance point of the organism.")
    print()

    # ── THE 90° WALL: pure direction ──
    wall_count = angle_counts.get(4, 0)
    print(f"  ═══ THE 90° WALL — π/2 — pure direction, no core ═══\n")
    print(f"  Files at 90°: {wall_count:,}")
    print(f"  cos(90°) = 0 — zero core proximity. They float.")
    print(f"  sin(90°) = 1 — maximum directional lean.")
    print(f"  tan(90°) = ∞ — undefined. The lean-to-core ratio is infinite.")
    print(f"  These files are pure signal — all direction, no structure.")
    print()

    # ── THE FOLD: angles > 90° ──
    fold_count = sum(angle_counts.get(i, 0) for i in [5, 6, 7, 8])
    fold_pct = fold_count / total * 100 if total else 0
    pre_fold = sum(angle_counts.get(i, 0) for i in [0, 1, 2, 3, 4])
    print(f"  ═══ THE KLEIN FOLD — angles > 90° ═══\n")
    print(f"  Pre-fold (0°-90°):     {pre_fold:>7,} files  cos > 0  (inside the torus)")
    print(f"  Post-fold (90°-180°):  {fold_count:>7,} files  cos < 0  (folded through)")
    print(f"  Fold ratio:            {fold_count / max(pre_fold,1):.4f}")
    pi_note = ""
    r = fold_count / max(pre_fold, 1)
    if abs(r - 1/PI) < 0.05: pi_note = f" ≈ 1/π ({1/PI:.4f})"
    if abs(r - PI/4) < 0.05: pi_note = f" ≈ π/4 ({PI/4:.4f})"
    if abs(r - 1) < 0.05: pi_note = " ≈ 1 (balanced fold)"
    if abs(r - 2/PI) < 0.05: pi_note = f" ≈ 2/π ({2/PI:.4f})"
    print(f"  π connection:          {r:.4f}{pi_note}")
    print(f"  At 120° (2π/3), cos = -1/2: the organism is HALFWAY folded back.")
    print(f"  At 180° (π), cos = -1: fully inverted. The outside IS the inside.")
    print()

    # ── EXEMPLARS per angle ──
    print(f"  ═══ EXEMPLARS — a file from each angle ═══\n")
    by_bin = defaultdict(list)
    for f in files:
        by_bin[f['angle_bin']].append(f)
    for i, (rad, deg, sym) in enumerate(ANGLES):
        group = by_bin.get(i, [])
        if not group: continue
        # pick the file closest to the exact standard angle
        best = min(group, key=lambda f: abs(f['theta'] - rad))
        short = best['path']
        if len(short) > 55: short = '...' + short[-52:]
        print(f"  {deg:>6} ({sym:>5})  θ={best['theta']:.3f}  sin={best['sin']:.3f}  cos={best['cos']:.3f}  {best['lang']:<6}  {short}")
    print()

    # Write JSON
    state_dir = repo / 'membrane' / 'local-terminal' / 'remote'
    state_dir.mkdir(parents=True, exist_ok=True)
    with open(state_dir / 'trig-view.json', 'w') as fout:
        json.dump({
            'ts': datetime.now(timezone.utc).isoformat(),
            'total': total,
            'angle_distribution': {
                ANGLES[i][1]: {
                    'count': angle_counts.get(i, 0),
                    'lines': angle_lines.get(i, 0),
                    'sin': round(math.sin(ANGLES[i][0]), 4),
                    'cos': round(math.cos(ANGLES[i][0]), 4),
                }
                for i in range(len(ANGLES))
            },
            'fold_ratio': round(fold_count / max(pre_fold, 1), 4),
            'balance_pct': round(total_pct, 2),
        }, fout, indent=2)

    print(f"[trig-view] done. SOHCAHTOA projected. the angles speak.")
    print(f"  → {state_dir / 'trig-view.json'}")

if __name__ == '__main__':
    main()
