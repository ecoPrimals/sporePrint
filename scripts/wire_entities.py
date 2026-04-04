#!/usr/bin/env python3
"""Wire entity shortcodes into all sporePrint content.

Reads the entity registry from config.toml, then replaces plain-text
entity references with {{ entity(name="key") }} shortcodes in all
content markdown files.

Zones skipped: front matter, code blocks, headings, inline code,
URLs, markdown link targets, existing shortcodes.
"""

import re
import glob
import os
import sys

SCRIPT_DIR = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.dirname(SCRIPT_DIR)

# ---------------------------------------------------------------------------
# 1. Parse entity registry from config.toml
# ---------------------------------------------------------------------------

def parse_registry(config_path):
    """Extract entity_registry entries from config.toml."""
    entities = {}
    current_key = None
    with open(config_path) as f:
        for line in f:
            line = line.strip()
            m = re.match(r'\[extra\.entity_registry\.(\w+)\]', line)
            if m:
                current_key = m.group(1)
                entities[current_key] = {}
                continue
            if current_key and '=' in line and not line.startswith('#'):
                k, _, v = line.partition('=')
                k = k.strip()
                v = v.strip().strip('"')
                if k in ('display', 'emoji', 'kind'):
                    entities[current_key][k] = v
            if line.startswith('[') and 'entity_registry' not in line:
                current_key = None
    return entities

# ---------------------------------------------------------------------------
# 2. Build replacement table
# ---------------------------------------------------------------------------

def build_replacements(registry):
    """Build (display_name, emoji, key, kind) list sorted longest-first."""
    replacements = []
    for key, entry in registry.items():
        display = entry.get('display', '')
        emoji = entry.get('emoji', '')
        kind = entry.get('kind', '')
        if not display:
            continue
        replacements.append((display, emoji, key, kind))

    # Add known alternate forms
    alt_forms = {
        'LoamSpine': 'loamspine',
        'Loamspine': 'loamspine',
        'BlueFish': 'bluefish',  # might appear capitalized
    }
    for alt_display, key in alt_forms.items():
        if key in registry:
            e = registry[key]
            replacements.append((alt_display, e.get('emoji', ''), key, e.get('kind', '')))

    # Sort by display name length (longest first) to prevent partial matches
    replacements.sort(key=lambda x: -len(x[0]))
    return replacements

# ---------------------------------------------------------------------------
# 3. Protected-segment splitting
# ---------------------------------------------------------------------------

# Regex that captures segments we must NOT modify
PROTECTED_RE = re.compile(
    r'('
    r'`[^`]+`'                    # inline code
    r'|\{\{[^}]*\}\}'            # Zola shortcodes
    r'|\[[^\]]*\]\([^)]*\)'      # markdown links [text](url)
    r'|https?://\S+'             # bare URLs
    r')'
)

def split_segments(text):
    """Split text into (is_protected, content) segments."""
    parts = PROTECTED_RE.split(text)
    segments = []
    for i, part in enumerate(parts):
        if i % 2 == 0:
            segments.append((False, part))
        else:
            segments.append((True, part))
    return segments

# ---------------------------------------------------------------------------
# 4. Apply replacements to an unprotected text segment
# ---------------------------------------------------------------------------

SHORTCODE_TMPL = '{{{{ entity(name="{}") }}}}'

def apply_replacements(text, replacements):
    """Replace entity names in a plain-text segment."""
    for display, emoji, key, kind in replacements:
        if not display:
            continue

        shortcode = SHORTCODE_TMPL.format(key)

        # 1) Emoji-prefixed form: "🐻🐕 BearDog" → shortcode
        if emoji:
            emoji_pattern = re.escape(emoji) + r'\s*' + re.escape(display)
            text = re.sub(emoji_pattern, shortcode, text)

        # 2) Possessive form: "BearDog's" → shortcode + 's
        poss_pattern = (
            r'(?<![/_.:\w])'
            + re.escape(display)
            + r"'s\b"
        )
        text = re.sub(poss_pattern, shortcode + "'s", text)

        # 3) Plain form: "BearDog" → shortcode
        plain_pattern = (
            r'(?<![/_.:\w])'
            + re.escape(display)
            + r'(?![/\w])'
        )
        text = re.sub(plain_pattern, shortcode, text)

    return text

# ---------------------------------------------------------------------------
# 5. Process a single line
# ---------------------------------------------------------------------------

def process_line(line, replacements):
    """Apply entity wiring to a single content line."""
    segments = split_segments(line)
    result = []
    for is_protected, text in segments:
        if is_protected:
            result.append(text)
        else:
            result.append(apply_replacements(text, replacements))
    return ''.join(result)

# ---------------------------------------------------------------------------
# 6. Process a markdown file
# ---------------------------------------------------------------------------

def process_file(path, replacements):
    """Wire entities into a markdown file. Returns (changed, count)."""
    with open(path) as f:
        content = f.read()

    lines = content.split('\n')
    new_lines = []
    in_frontmatter = False
    frontmatter_count = 0
    in_codeblock = False
    total_replacements = 0

    for line in lines:
        stripped = line.strip()

        # Track front matter (+++...+++)
        if stripped == '+++':
            frontmatter_count += 1
            in_frontmatter = frontmatter_count % 2 == 1
            new_lines.append(line)
            continue

        # Track code blocks
        if stripped.startswith('```'):
            in_codeblock = not in_codeblock
            new_lines.append(line)
            continue

        # Skip zones: front matter, code blocks, headings
        if in_frontmatter or in_codeblock or stripped.startswith('#'):
            new_lines.append(line)
            continue

        # Skip lines that are only shortcodes (entity_metrics, etc.)
        if stripped.startswith('{{') and stripped.endswith('}}'):
            new_lines.append(line)
            continue

        # Apply replacements
        new_line = process_line(line, replacements)
        if new_line != line:
            # Count actual replacements
            old_count = line.count('entity(name=')
            new_count = new_line.count('entity(name=')
            total_replacements += new_count - old_count
        new_lines.append(new_line)

    new_content = '\n'.join(new_lines)
    changed = new_content != content

    if changed:
        with open(path, 'w') as f:
            f.write(new_content)

    return changed, total_replacements

# ---------------------------------------------------------------------------
# Main
# ---------------------------------------------------------------------------

def main():
    config_path = os.path.join(ROOT, 'config.toml')
    registry = parse_registry(config_path)
    replacements = build_replacements(registry)

    print(f"wire_entities: loaded {len(registry)} entities from config.toml")
    print(f"  replacement patterns: {len(replacements)} (including alternates)")

    md_files = sorted(glob.glob(os.path.join(ROOT, 'content', '**', '*.md'), recursive=True))
    print(f"  markdown files: {len(md_files)}")
    print()

    total_files_changed = 0
    total_replacements = 0

    for path in md_files:
        rel = os.path.relpath(path, ROOT)
        changed, count = process_file(path, replacements)
        if changed:
            total_files_changed += 1
            total_replacements += count
            print(f"  {rel}: {count} replacements")

    print()
    print(f"Done: {total_files_changed} files changed, {total_replacements} entity references wired")

    if total_replacements == 0:
        print("  (no replacements — entities may already be wired)")

if __name__ == '__main__':
    main()
