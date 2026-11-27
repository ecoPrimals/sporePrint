#!/usr/bin/env python3
"""sideview.py — view the organism from the SIDE, not the pole.

The pole view (vitruvian-evolve.py) looks DOWN the donut hole.
  → counts files by evolutionary stage
  → top-down, categorical, what IS it

The side view looks at the torus CROSS-SECTION (the hub-lens, B-6).
  → projects files onto the lens using three π-sorted dimensions:
    DISTANCE:    how far from the vertebrate core (the center)
    DIRECTION:   which way does it face (import flow — inward/outward)
    DISPOSITION: what does it tend to do (grow/shrink, produce/consume, active/dormant)

On the torus cross-section:
  - Center = the donut hole = the question = the vertebrate core
  - Inner wall (concave) = what the membrane sees = CNS
  - Outer wall (convex) = what the public sees = PNS
  - Top/bottom = the poles = null points = where new things grow

π-sorting: normalize each dimension to [0, π] and look for
clustering, gaps, and remainder.

Usage: python3 sideview.py [repo_path]
"""

import os, sys, math, json, subprocess, re
from collections import Counter, defaultdict
from pathlib import Path
from datetime import datetime, timezone

PI = math.pi

# reuse scanner components
LANG_MAP = {
    '.rs': 'rust', '.js': 'js', '.py': 'python', '.sh': 'shell',
    '.toml': 'toml', '.md': 'markdown', '.html': 'html'
}
SKIP = {'target', '.git', 'node_modules', '__pycache__'}

def shannon(text):
    words = text.lower().split()
    if not words:
        return 0.0
    freq = Counter(words)
    total = len(words)
    h = 0.0
    for count in freq.values():
        p = count / total
        if p > 0:
            h -= p * math.log2(p)
    return h

def sigmoid(x, mid=0.5, k=6.0):
    z = k * (x - mid)
    if z > 500: return 1.0
    if z < -500: return 0.0
    return 1.0 / (1.0 + math.exp(-z))

# ═══════════════════════════════════════════════════════════════
# DISTANCE — how far from the vertebrate core
#
# Distance = inverse of "skeleton density"
# Files with many types + many functions = close to center (vertebrate)
# Files with no types, no functions = far from center (banana/jellyfish)
# Measured in π-units: 0 = dead center, π = outer edge
# ═══════════════════════════════════════════════════════════════

def distance(types, functions, imports, lines):
    """Distance from vertebrate core in [0, π]."""
    if lines == 0:
        return PI
    skeleton = sigmoid(types / 10.0, 0.5, 5)
    appendages = sigmoid(functions / 20.0, 0.5, 4)
    wiring = sigmoid(imports / 8.0, 0.5, 4)
    core_proximity = (skeleton * 0.5 + appendages * 0.3 + wiring * 0.2)
    return PI * (1.0 - core_proximity)

# ═══════════════════════════════════════════════════════════════
# DIRECTION — which way does it face
#
# Inward (0): file is consumed by others (types, shared lib, reusable)
# Outward (π): file consumes others (scripts, binaries, leaf nodes)
# Mixed (π/2): both imports and exports
#
# Approximation: if it defines types → inward (others import it)
#                if it imports a lot → outward (it depends on others)
#                balanced → mixed
# ═══════════════════════════════════════════════════════════════

def direction(types, functions, imports):
    """Direction on [0, π]. 0 = inward (producer), π = outward (consumer)."""
    production = types + functions * 0.3  # definitions = production
    consumption = imports                   # imports = consumption
    total = production + consumption
    if total == 0:
        return PI / 2  # undefined → middle
    outward_ratio = consumption / total
    return PI * outward_ratio

# ═══════════════════════════════════════════════════════════════
# DISPOSITION — what does it tend to do
#
# 0 = dormant (old, low entropy, static)
# π/2 = active (fresh, medium entropy, changing)
# π = volatile (very high entropy, churning, unstable)
#
# Based on age, entropy, and vocabulary richness
# ═══════════════════════════════════════════════════════════════

def disposition(age_days, entropy, richness):
    """Disposition on [0, π]. 0 = dormant, π/2 = active, π = volatile."""
    freshness = 1.0 - sigmoid(age_days / 180.0, 0.5, 3)  # old = 0, fresh = 1
    complexity = sigmoid(entropy / 10.0, 0.5, 4)           # low = 0, high = 1
    diversity = sigmoid(richness / 0.5, 0.5, 4)            # monotone = 0, diverse = 1
    activity = freshness * 0.5 + complexity * 0.3 + diversity * 0.2
    return PI * activity

# ═══════════════════════════════════════════════════════════════
# π-SORT — bin each dimension into π-slices
# ═══════════════════════════════════════════════════════════════

def pi_bin(value, n_bins=7):
    """Which π-bin does this value fall in? Returns bin index 0..n_bins-1."""
    slice_width = PI / n_bins
    b = int(value / slice_width)
    return min(b, n_bins - 1)

def pi_remainder(value):
    """The fractional part after dividing by π."""
    return (value / PI) - int(value / PI)

# ═══════════════════════════════════════════════════════════════
# MAIN
# ═══════════════════════════════════════════════════════════════

def file_age_days(path):
    try:
        mtime = path.stat().st_mtime
        now = datetime.now(timezone.utc).timestamp()
        return (now - mtime) / 86400
    except Exception:
        return 9999

def main():
    repo = Path(sys.argv[1]) if len(sys.argv) > 1 else Path('.')
    repo = repo.resolve()
    print(f"[sideview] viewing {repo} from the SIDE...")
    print()

    files = []
    dist_bins = Counter()
    dir_bins = Counter()
    disp_bins = Counter()
    cross_section = defaultdict(int)  # (dist_bin, dir_bin) → count
    lang_cross = defaultdict(lambda: defaultdict(int))  # lang → (dist_bin, dir_bin) → count

    for path in sorted(repo.rglob('*')):
        if not path.is_file():
            continue
        if any(part in SKIP for part in path.parts):
            continue
        ext = path.suffix.lower()
        lang = LANG_MAP.get(ext)
        if not lang:
            continue

        try:
            text = path.read_text(errors='replace')
        except Exception:
            continue

        lines_list = text.splitlines()
        num_lines = len(lines_list)
        if num_lines == 0:
            continue

        h = shannon(text)
        words = text.lower().split()
        r = len(set(words)) / len(words) if words else 0
        age = file_age_days(path)

        # Count features
        types = 0
        funcs = 0
        imports = 0
        if lang == 'rust':
            for line in lines_list:
                s = line.strip()
                if s.startswith(('pub enum ', 'enum ', 'pub struct ', 'struct ')):
                    types += 1
                if s.startswith(('pub fn ', 'fn ', 'pub async fn ')):
                    funcs += 1
                if s.startswith('use '):
                    imports += 1
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
        dp = disposition(age, h, r)

        db = pi_bin(d)
        drb = pi_bin(dr)
        dpb = pi_bin(dp)

        dist_bins[db] += 1
        dir_bins[drb] += 1
        disp_bins[dpb] += 1
        cross_section[(db, drb)] += 1
        lang_cross[lang][(db, drb)] += 1

        files.append({
            'path': str(path.relative_to(repo)),
            'lang': lang,
            'lines': num_lines,
            'distance': round(d, 4),
            'direction': round(dr, 4),
            'disposition': round(dp, 4),
            'dist_bin': db,
            'dir_bin': drb,
            'disp_bin': dpb,
            'pi_rem': round(pi_remainder(d + dr + dp), 4),
        })

    total = len(files)
    print(f"  TOTAL: {total:,} files projected onto torus cross-section")
    print()

    # ── DISTANCE histogram (radial) ──
    N = 7
    labels_dist = ['core', 'inner', 'mid-in', 'middle', 'mid-out', 'outer', 'edge']
    labels_dir = ['produce', 'pro-mix', 'mix-in', 'neutral', 'mix-out', 'con-mix', 'consume']
    labels_disp = ['dormant', 'resting', 'slow', 'active', 'lively', 'busy', 'volatile']

    print(f"  ═══ DISTANCE — how far from the vertebrate core ═══")
    print(f"  0 = dead center (skeleton), π = outer edge (no structure)\n")
    max_d = max(dist_bins.values()) if dist_bins else 1
    for i in range(N):
        c = dist_bins.get(i, 0)
        bar = '█' * int(c / max_d * 40)
        pct = c / total * 100
        lo = i * PI / N
        hi = (i+1) * PI / N
        print(f"  {labels_dist[i]:>8} [{lo:.2f}-{hi:.2f}]  {c:>6,}  {pct:>5.1f}%  {bar}")
    print()

    # ── DIRECTION histogram (angular) ──
    print(f"  ═══ DIRECTION — which way does it face ═══")
    print(f"  0 = inward (producer/exporter), π = outward (consumer/importer)\n")
    max_dr = max(dir_bins.values()) if dir_bins else 1
    for i in range(N):
        c = dir_bins.get(i, 0)
        bar = '█' * int(c / max_dr * 40)
        pct = c / total * 100
        lo = i * PI / N
        hi = (i+1) * PI / N
        print(f"  {labels_dir[i]:>8} [{lo:.2f}-{hi:.2f}]  {c:>6,}  {pct:>5.1f}%  {bar}")
    print()

    # ── DISPOSITION histogram (temporal) ──
    print(f"  ═══ DISPOSITION — what does it tend to do ═══")
    print(f"  0 = dormant (old, static), π/2 = active, π = volatile\n")
    max_dp = max(disp_bins.values()) if disp_bins else 1
    for i in range(N):
        c = disp_bins.get(i, 0)
        bar = '█' * int(c / max_dp * 40)
        pct = c / total * 100
        lo = i * PI / N
        hi = (i+1) * PI / N
        print(f"  {labels_disp[i]:>8} [{lo:.2f}-{hi:.2f}]  {c:>6,}  {pct:>5.1f}%  {bar}")
    print()

    # ── CROSS-SECTION: distance × direction (the lens) ──
    print(f"  ═══ TORUS CROSS-SECTION — distance × direction ═══")
    print(f"  (the hub-lens: rows = distance from core, cols = direction of flow)\n")
    header = f"  {'':>8}  " + "".join(f"{labels_dir[i]:>8}" for i in range(N))
    print(header)
    print(f"  {'':>8}  " + "─" * (8 * N))
    for di in range(N):
        row = f"  {labels_dist[di]:>8}  "
        for dri in range(N):
            c = cross_section.get((di, dri), 0)
            if c == 0:
                row += f"{'·':>8}"
            elif c < 10:
                row += f"{c:>8}"
            elif c < 100:
                row += f"{c:>8}"
            else:
                row += f"{c:>8,}"
            
        print(row)
    print()

    # ── CROSS-SECTION per language (overlay) ──
    for lang in ['rust', 'js', 'python', 'shell']:
        lc = lang_cross.get(lang, {})
        if not lc:
            continue
        lang_total = sum(lc.values())
        print(f"  ── {lang.upper()} cross-section ({lang_total:,} files) ──")
        header = f"  {'':>8}  " + "".join(f"{labels_dir[i]:>8}" for i in range(N))
        print(header)
        for di in range(N):
            row = f"  {labels_dist[di]:>8}  "
            for dri in range(N):
                c = lc.get((di, dri), 0)
                if c == 0:
                    row += f"{'·':>8}"
                else:
                    row += f"{c:>8,}"
            print(row)
        print()

    # ── π-REMAINDER distribution ──
    pi_rems = [f['pi_rem'] for f in files]
    rem_hist = Counter()
    for r in pi_rems:
        rem_hist[int(r * 10)] += 1  # 10 bins of 0.1

    print(f"  ═══ π-REMAINDER distribution (sum of all 3 dimensions mod π) ═══\n")
    max_r = max(rem_hist.values()) if rem_hist else 1
    for i in range(10):
        c = rem_hist.get(i, 0)
        bar = '█' * int(c / max_r * 30)
        print(f"  0.{i}x  {c:>6,}  {bar}")
    print()

    # ── THE HOTTEST AND COLDEST SPOTS ──
    print(f"  ═══ HOTTEST SPOT (closest to core + highest production + most active) ═══\n")
    scored = sorted(files, key=lambda f: (PI - f['distance']) + (PI - f['direction']) + f['disposition'], reverse=True)
    for f in scored[:5]:
        short = f['path'][-55:] if len(f['path']) > 55 else f['path']
        print(f"  dist={f['distance']:.2f} dir={f['direction']:.2f} disp={f['disposition']:.2f}  {f['lang']:<6}  {short}")
    print()

    print(f"  ═══ COLDEST SPOT (farthest from core + pure consumer + dormant) ═══\n")
    scored_cold = sorted(files, key=lambda f: f['distance'] + f['direction'] + (PI - f['disposition']), reverse=True)
    for f in scored_cold[:5]:
        short = f['path'][-55:] if len(f['path']) > 55 else f['path']
        print(f"  dist={f['distance']:.2f} dir={f['direction']:.2f} disp={f['disposition']:.2f}  {f['lang']:<6}  {short}")
    print()

    # ── WHERE IS THE DONUT HOLE? ──
    center_count = cross_section.get((0, 3), 0) + cross_section.get((0, 2), 0) + cross_section.get((0, 4), 0)
    edge_count = cross_section.get((6, 0), 0) + cross_section.get((6, 6), 0)
    print(f"  ═══ THE DONUT HOLE ═══")
    print(f"  Center (core × neutral):  {center_count} files")
    print(f"  Edge (edge × extreme):    {edge_count} files")
    print(f"  Hole ratio:               {center_count / max(edge_count, 1):.2f}")
    print(f"  Note: the hole IS the question. Fewer files = more open.")
    print()

    # Write JSON
    state_dir = repo / 'membrane' / 'local-terminal' / 'remote'
    state_dir.mkdir(parents=True, exist_ok=True)
    with open(state_dir / 'sideview.json', 'w') as fout:
        json.dump({
            'ts': datetime.now(timezone.utc).isoformat(),
            'total': total,
            'distance_bins': {labels_dist[i]: dist_bins.get(i, 0) for i in range(N)},
            'direction_bins': {labels_dir[i]: dir_bins.get(i, 0) for i in range(N)},
            'disposition_bins': {labels_disp[i]: disp_bins.get(i, 0) for i in range(N)},
            'cross_section': {f"{labels_dist[di]}×{labels_dir[dri]}": cross_section.get((di, dri), 0)
                             for di in range(N) for dri in range(N)},
        }, fout, indent=2)

    print(f"[sideview] done. cross-section projected. the lens is rendered.")
    print(f"  → {state_dir / 'sideview.json'}")

if __name__ == '__main__':
    main()
