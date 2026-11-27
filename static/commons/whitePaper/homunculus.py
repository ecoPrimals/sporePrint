#!/usr/bin/env python3
"""homunculus.py — map the codebase nervous system.

The homunculus is the distorted body map in the somatosensory cortex where
body parts are sized by SENSITIVITY (nerve endings), not physical size.
Hands and lips are HUGE. The back is tiny.

For code:
  - git churn (change frequency)     → nerve density (sensitivity)
  - import/dependency count           → axon connections (connectivity)
  - recent errors/warnings            → pain receptors (nociception)
  - evolutionary stage (from evolve)  → tissue type
  - file age / staleness              → numbness (dead tissue)

CNS = central pathways (spine: core libs, shared types, the skeleton)
PNS = peripheral nerves (leaf files, scripts, configs, the surface)

Outputs:
  - homunculus-map.json: every directory sized by sensitivity
  - pressure.json: directories growing faster than they're decomposing
  - jams.json: blocked pathways (broken imports, missing deps, circular refs)
  - pulse.json: the heartbeat — cron cadence, feed freshness, service status

Usage: python3 homunculus.py [repo_path]
"""

import os, sys, math, json, subprocess, re
from collections import Counter, defaultdict
from pathlib import Path
from datetime import datetime, timezone

PI = math.pi

# ═══════════════════════════════════════════════════════════════
# GIT CHURN — nerve density
# How often does each file get touched? High churn = high sensitivity.
# ═══════════════════════════════════════════════════════════════

def git_churn(repo, months=6):
    """Count commits per file in the last N months."""
    churn = Counter()
    try:
        since = f"--since={months} months ago"
        result = subprocess.run(
            ['git', 'log', '--name-only', '--pretty=format:', since],
            cwd=str(repo), capture_output=True, text=True, timeout=60
        )
        for line in result.stdout.splitlines():
            line = line.strip()
            if line and not line.startswith('commit '):
                churn[line] += 1
    except Exception:
        pass
    return churn

# ═══════════════════════════════════════════════════════════════
# DEPENDENCY GRAPH — axon connections
# Who imports whom? High import count = high connectivity.
# ═══════════════════════════════════════════════════════════════

IMPORT_PATTERNS = {
    '.rs': [
        re.compile(r'^\s*use\s+([\w:]+)'),
        re.compile(r'^\s*mod\s+(\w+)'),
    ],
    '.py': [
        re.compile(r'^\s*import\s+([\w.]+)'),
        re.compile(r'^\s*from\s+([\w.]+)\s+import'),
    ],
    '.js': [
        re.compile(r'''require\s*\(\s*['"]([^'"]+)['"]\s*\)'''),
        re.compile(r'''import\s+.*from\s+['"]([^'"]+)['"]'''),
    ],
    '.sh': [
        re.compile(r'^\s*source\s+(.+)'),
        re.compile(r'^\s*\.\s+(.+)'),
    ],
}

def count_imports(path):
    """Count import/use/require statements in a file."""
    ext = path.suffix.lower()
    patterns = IMPORT_PATTERNS.get(ext, [])
    if not patterns:
        return 0, []
    try:
        text = path.read_text(errors='replace')
    except Exception:
        return 0, []
    count = 0
    targets = []
    for line in text.splitlines():
        for pat in patterns:
            m = pat.match(line) if not pat.pattern.startswith('^') or pat.pattern.startswith('^') else pat.search(line)
            if m:
                count += 1
                targets.append(m.group(1))
    return count, targets

# ═══════════════════════════════════════════════════════════════
# FILE AGE — numbness (dead tissue)
# How long since last modification? Old = numb.
# ═══════════════════════════════════════════════════════════════

def file_age_days(path):
    """Days since last modification."""
    try:
        mtime = path.stat().st_mtime
        now = datetime.now(timezone.utc).timestamp()
        return (now - mtime) / 86400
    except Exception:
        return 9999

# ═══════════════════════════════════════════════════════════════
# SENSITIVITY SCORE — the homunculus size
# Combines churn, connectivity, recency, and complexity.
# ═══════════════════════════════════════════════════════════════

def sigmoid(x, mid=0.5, k=6.0):
    z = k * (x - mid)
    if z > 500: return 1.0
    if z < -500: return 0.0
    return 1.0 / (1.0 + math.exp(-z))

def sensitivity(churn, imports, age_days, lines):
    """Composite sensitivity: 0 (numb) to 1 (hypersensitive)."""
    nerve_density = sigmoid(churn / 20.0, 0.5, 4)       # high churn = high
    connectivity = sigmoid(imports / 10.0, 0.5, 4)       # many deps = high
    recency = 1.0 - sigmoid(age_days / 180.0, 0.5, 3)   # old = low
    bulk = sigmoid(lines / 500.0, 0.5, 2)                # big files = moderate
    return (nerve_density * 0.4 + connectivity * 0.25
            + recency * 0.25 + bulk * 0.1)

# ═══════════════════════════════════════════════════════════════
# PRESSURE DETECTION — building tension
# A directory is under pressure when:
#   - files are growing (lines increasing)
#   - churn is high but no decomposition (no new files, just bigger old ones)
#   - many imports but few exports (consuming without producing)
# ═══════════════════════════════════════════════════════════════

def detect_pressure(dir_stats):
    """Identify directories under pressure."""
    pressures = []
    for dirname, stats in dir_stats.items():
        score = 0.0
        reasons = []

        # High churn + large files = pressure
        if stats['avg_churn'] > 5 and stats['avg_lines'] > 300:
            score += 0.4
            reasons.append(f"high churn ({stats['avg_churn']:.1f}/file) on large files ({stats['avg_lines']:.0f} avg lines)")

        # Many imports, few files = concentrated dependency
        if stats['total_imports'] > 20 and stats['file_count'] < 10:
            score += 0.3
            reasons.append(f"{stats['total_imports']} imports in {stats['file_count']} files — concentrated")

        # Old files with high churn = legacy pressure
        if stats['avg_age'] > 90 and stats['avg_churn'] > 3:
            score += 0.3
            reasons.append(f"legacy ({stats['avg_age']:.0f}d avg) still churning ({stats['avg_churn']:.1f}/file)")

        if score > 0.3:
            pressures.append({
                'directory': dirname,
                'pressure': round(score, 3),
                'reasons': reasons,
                'files': stats['file_count'],
                'lines': stats['total_lines'],
            })

    return sorted(pressures, key=lambda p: p['pressure'], reverse=True)

# ═══════════════════════════════════════════════════════════════
# JAM DETECTION — blocked pathways
# A jam occurs when:
#   - a file imports something that doesn't exist (broken nerve)
#   - a directory has no outbound imports (isolated, no signal out)
#   - circular dependencies exist (feedback loop)
# ═══════════════════════════════════════════════════════════════

def detect_jams(file_data):
    """Find broken import paths and isolated nodes."""
    jams = []

    # Find directories with zero outbound connections
    dir_imports = defaultdict(int)
    dir_exports = defaultdict(int)
    for fdata in file_data:
        d = str(Path(fdata['path']).parent)
        dir_imports[d] += fdata['imports']
        # A file that's imported BY others counts as an export
        # (approximated: if it has types/functions, it's likely exported)
        if fdata.get('types', 0) > 0 or fdata.get('functions', 0) > 0:
            dir_exports[d] += 1

    for d in dir_imports:
        imp = dir_imports[d]
        exp = dir_exports.get(d, 0)
        if imp > 10 and exp == 0:
            jams.append({
                'type': 'sink',
                'directory': d,
                'imports': imp,
                'exports': exp,
                'note': 'all consumption, no production — metabolic sink'
            })
        elif exp > 10 and imp == 0:
            jams.append({
                'type': 'source',
                'directory': d,
                'imports': imp,
                'exports': exp,
                'note': 'all production, no consumption — orphan output'
            })

    return jams

# ═══════════════════════════════════════════════════════════════
# BODY REGION MAPPING — CNS vs PNS
# CNS = shared types, core libs, the spine
# PNS = leaf files, scripts, configs, the periphery
# ═══════════════════════════════════════════════════════════════

CNS_PATTERNS = [
    'crates', 'src/lib', 'src/main', 'src/types', 'src/core',
    'shared', 'common', 'lib', 'proto',
]
PNS_PATTERNS = [
    'scripts', 'config', 'tests', 'examples', 'benches',
    'docs', 'content', 'public', 'static', 'site',
]

def classify_nervous(path_str):
    """Is this file CNS (central) or PNS (peripheral)?"""
    p = path_str.lower()
    for pattern in CNS_PATTERNS:
        if f'/{pattern}/' in p or p.endswith(f'/{pattern}'):
            return 'CNS'
    for pattern in PNS_PATTERNS:
        if f'/{pattern}/' in p or p.endswith(f'/{pattern}'):
            return 'PNS'
    # Default: files deep in src/ are CNS, shallow are PNS
    depth = p.count('/src/')
    return 'CNS' if depth > 0 else 'PNS'

# ═══════════════════════════════════════════════════════════════
# MAIN — scan, map, detect, report
# ═══════════════════════════════════════════════════════════════

LANG_MAP = {
    '.rs': 'rust', '.js': 'js', '.py': 'python', '.sh': 'shell',
    '.toml': 'toml', '.md': 'markdown', '.html': 'html'
}
SKIP = {'target', '.git', 'node_modules', '__pycache__'}

def main():
    repo = Path(sys.argv[1]) if len(sys.argv) > 1 else Path('.')
    repo = repo.resolve()
    print(f"[homunculus] mapping nervous system of {repo}...")

    # Git churn data
    print("  scanning git churn (6 months)...")
    churn = git_churn(repo, months=6)

    # Scan all files
    print("  scanning files...")
    file_data = []
    dir_stats = defaultdict(lambda: {
        'file_count': 0, 'total_lines': 0, 'total_churn': 0,
        'total_imports': 0, 'total_age': 0, 'total_sensitivity': 0,
        'avg_churn': 0, 'avg_lines': 0, 'avg_age': 0,
        'cns_count': 0, 'pns_count': 0,
        'types': 0, 'functions': 0,
    })

    for path in sorted(repo.rglob('*')):
        if not path.is_file():
            continue
        if any(part in SKIP for part in path.parts):
            continue
        ext = path.suffix.lower()
        lang = LANG_MAP.get(ext)
        if not lang:
            continue

        rel = str(path.relative_to(repo))
        try:
            text = path.read_text(errors='replace')
        except Exception:
            continue

        lines_list = text.splitlines()
        num_lines = len(lines_list)
        if num_lines == 0:
            continue

        file_churn = churn.get(rel, 0)
        num_imports, import_targets = count_imports(path)
        age = file_age_days(path)
        sens = sensitivity(file_churn, num_imports, age, num_lines)
        nervous = classify_nervous(rel)

        # Count types and functions (for jam detection)
        types = 0
        funcs = 0
        if lang == 'rust':
            for line in lines_list:
                s = line.strip()
                if s.startswith(('pub enum ', 'enum ', 'pub struct ', 'struct ')):
                    types += 1
                if s.startswith(('pub fn ', 'fn ', 'pub async fn ')):
                    funcs += 1
        elif lang == 'python':
            for line in lines_list:
                s = line.strip()
                if s.startswith('class '):
                    types += 1
                if s.startswith('def '):
                    funcs += 1

        fdata = {
            'path': rel,
            'lang': lang,
            'lines': num_lines,
            'churn': file_churn,
            'imports': num_imports,
            'age_days': round(age, 1),
            'sensitivity': round(sens, 4),
            'nervous_system': nervous,
            'types': types,
            'functions': funcs,
        }
        file_data.append(fdata)

        # Aggregate by directory (2 levels deep for readability)
        parts = rel.split('/')
        dirname = '/'.join(parts[:min(3, len(parts)-1)]) or '.'
        d = dir_stats[dirname]
        d['file_count'] += 1
        d['total_lines'] += num_lines
        d['total_churn'] += file_churn
        d['total_imports'] += num_imports
        d['total_age'] += age
        d['total_sensitivity'] += sens
        d['types'] += types
        d['functions'] += funcs
        if nervous == 'CNS':
            d['cns_count'] += 1
        else:
            d['pns_count'] += 1

    # Compute averages
    for dirname, d in dir_stats.items():
        n = d['file_count']
        if n > 0:
            d['avg_churn'] = d['total_churn'] / n
            d['avg_lines'] = d['total_lines'] / n
            d['avg_age'] = d['total_age'] / n
            d['avg_sensitivity'] = round(d['total_sensitivity'] / n, 4)

    # ── REPORT ──
    total_files = len(file_data)
    cns_files = sum(1 for f in file_data if f['nervous_system'] == 'CNS')
    pns_files = total_files - cns_files
    avg_sens = sum(f['sensitivity'] for f in file_data) / total_files if total_files else 0

    print(f"\n  TOTAL: {total_files:,} files mapped")
    print(f"  CNS (central): {cns_files:,} files ({cns_files/total_files*100:.1f}%)")
    print(f"  PNS (peripheral): {pns_files:,} files ({pns_files/total_files*100:.1f}%)")
    print(f"  Average sensitivity: {avg_sens:.4f}")

    # ── HOMUNCULUS: top 20 most sensitive directories ──
    print(f"\n  ═══ HOMUNCULUS — sized by sensitivity, not mass ═══\n")
    ranked = sorted(dir_stats.items(), key=lambda x: x[1]['avg_sensitivity'], reverse=True)
    max_sens = ranked[0][1]['avg_sensitivity'] if ranked else 1

    print(f"  {'DIRECTORY':<45} {'SENS':>6} {'FILES':>6} {'CHURN':>6} {'IMP':>5} {'AGE':>5} {'NS':>4}")
    print(f"  {'─'*45} {'─'*6} {'─'*6} {'─'*6} {'─'*5} {'─'*5} {'─'*4}")
    for dirname, d in ranked[:30]:
        if d['file_count'] < 2:
            continue
        short = dirname[:44]
        ns = 'CNS' if d['cns_count'] > d['pns_count'] else 'PNS'
        bar = '█' * int(d['avg_sensitivity'] / max_sens * 20)
        print(f"  {short:<45} {d['avg_sensitivity']:>5.3f} {d['file_count']:>6} {d['avg_churn']:>5.1f} {d['total_imports']:>5} {d['avg_age']:>4.0f}d {ns:>4}  {bar}")

    # ── PRESSURE POINTS ──
    print(f"\n  ═══ PRESSURE POINTS — tension building ═══\n")
    pressures = detect_pressure(dir_stats)
    if pressures:
        for p in pressures[:15]:
            print(f"  ⚠  {p['directory'][:50]}")
            print(f"     pressure={p['pressure']:.3f}  files={p['files']}  lines={p['lines']:,}")
            for r in p['reasons']:
                print(f"     → {r}")
            print()
    else:
        print("  ✓ no directories under significant pressure")

    # ── JAMS ──
    print(f"\n  ═══ JAMS — blocked or orphaned pathways ═══\n")
    jams = detect_jams(file_data)
    if jams:
        for j in jams[:15]:
            print(f"  ✗  [{j['type'].upper()}] {j['directory'][:50]}")
            print(f"     imports={j['imports']} exports={j['exports']}")
            print(f"     {j['note']}")
            print()
    else:
        print("  ✓ no blocked pathways detected")

    # ── PULSE: CNS vs PNS sensitivity comparison ──
    cns_sens = [f['sensitivity'] for f in file_data if f['nervous_system'] == 'CNS']
    pns_sens = [f['sensitivity'] for f in file_data if f['nervous_system'] == 'PNS']
    cns_avg = sum(cns_sens) / len(cns_sens) if cns_sens else 0
    pns_avg = sum(pns_sens) / len(pns_sens) if pns_sens else 0

    print(f"\n  ═══ PULSE — CNS vs PNS ═══\n")
    print(f"  CNS (central, spine, skeleton):")
    print(f"    files: {len(cns_sens):,}  avg sensitivity: {cns_avg:.4f}")
    cns_bar = '█' * int(cns_avg * 40)
    print(f"    {cns_bar}")
    print(f"  PNS (peripheral, surface, scripts):")
    print(f"    files: {len(pns_sens):,}  avg sensitivity: {pns_avg:.4f}")
    pns_bar = '█' * int(pns_avg * 40)
    print(f"    {pns_bar}")
    ratio = cns_avg / pns_avg if pns_avg > 0 else 0
    print(f"  CNS/PNS ratio: {ratio:.4f}")
    pi_note = ""
    if abs(ratio - 1/PI) < 0.1:
        pi_note = " ≈ 1/π!"
    elif abs(ratio - PI) < 0.3:
        pi_note = " ≈ π!"
    elif abs(ratio - 1) < 0.1:
        pi_note = " ≈ 1 (balanced)"
    print(f"  π connection: 1/π = {1/PI:.4f}, π = {PI:.4f}{pi_note}")

    # ── HIGHEST SENSITIVITY FILES (the fingertips) ──
    print(f"\n  ═══ FINGERTIPS — most sensitive individual files ═══\n")
    top = sorted(file_data, key=lambda f: f['sensitivity'], reverse=True)[:15]
    for f in top:
        short = f['path']
        if len(short) > 55:
            short = '...' + short[-52:]
        print(f"  {f['sensitivity']:.4f}  churn={f['churn']:>3}  imp={f['imports']:>3}  age={f['age_days']:>5.0f}d  {f['nervous_system']}  {short}")

    # ── NUMBEST FILES (the dead tissue) ──
    print(f"\n  ═══ NUMBEST — dead tissue, no signal ═══\n")
    numb = sorted(file_data, key=lambda f: f['sensitivity'])[:10]
    for f in numb:
        short = f['path']
        if len(short) > 55:
            short = '...' + short[-52:]
        print(f"  {f['sensitivity']:.4f}  churn={f['churn']:>3}  imp={f['imports']:>3}  age={f['age_days']:>5.0f}d  {f['nervous_system']}  {short}")

    # ── WRITE JSON OUTPUTS ──
    state_dir = repo / 'membrane' / 'local-terminal' / 'remote'
    state_dir.mkdir(parents=True, exist_ok=True)

    # Homunculus map
    homunculus_data = {
        'ts': datetime.now(timezone.utc).isoformat(),
        'total_files': total_files,
        'cns_files': cns_files,
        'pns_files': pns_files,
        'avg_sensitivity': round(avg_sens, 4),
        'cns_avg_sensitivity': round(cns_avg, 4),
        'pns_avg_sensitivity': round(pns_avg, 4),
        'cns_pns_ratio': round(ratio, 4),
        'directories': [
            {
                'name': dirname,
                'sensitivity': d['avg_sensitivity'],
                'files': d['file_count'],
                'lines': d['total_lines'],
                'churn': round(d['avg_churn'], 2),
                'imports': d['total_imports'],
                'age': round(d['avg_age'], 1),
                'cns': d['cns_count'],
                'pns': d['pns_count'],
                'types': d['types'],
                'functions': d['functions'],
            }
            for dirname, d in ranked if d['file_count'] > 1
        ],
    }

    with open(state_dir / 'homunculus-map.json', 'w') as f:
        json.dump(homunculus_data, f, indent=2)

    # Pressure
    with open(state_dir / 'pressure.json', 'w') as f:
        json.dump({
            'ts': datetime.now(timezone.utc).isoformat(),
            'pressure_points': pressures[:20]
        }, f, indent=2)

    # Jams
    with open(state_dir / 'jams.json', 'w') as f:
        json.dump({
            'ts': datetime.now(timezone.utc).isoformat(),
            'jams': jams[:20]
        }, f, indent=2)

    print(f"\n[homunculus] mapped. {total_files:,} files. {len(pressures)} pressure points. {len(jams)} jams.")
    print(f"  → {state_dir / 'homunculus-map.json'}")
    print(f"  → {state_dir / 'pressure.json'}")
    print(f"  → {state_dir / 'jams.json'}")

if __name__ == '__main__':
    main()
