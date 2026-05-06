#!/usr/bin/env bash
# Render Jupyter notebooks to Zola-compatible markdown pages for sporePrint.
#
# Converts .ipynb files to HTML body fragments, wraps them in Zola TOML
# front matter, and places them in content/lab/ for static site generation.
#
# Usage:
#   bash scripts/render_notebooks.sh [--notebook <path>] [--all]
#
# Requires: jupyter nbconvert (pip install nbconvert)

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
SPOREPRINT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
CONTENT_LAB="$SPOREPRINT_ROOT/content/lab"

NOTEBOOKS_DIR="${NOTEBOOKS_DIR:-/home/irongate/notebooks}"
SHOWCASE_DIR="${SHOWCASE_DIR:-/home/irongate/shared/abg/showcase}"

mkdir -p "$CONTENT_LAB"

render_notebook() {
    local nb_path="$1"
    local nb_name
    nb_name="$(basename "$nb_path" .ipynb)"

    local slug
    slug="$(echo "$nb_name" | tr '[:upper:]' '[:lower:]' | sed 's/[^a-z0-9]/-/g' | sed 's/--*/-/g')"

    local title
    title="$(python3 -c "
import json, sys
with open('$nb_path') as f:
    nb = json.load(f)
for cell in nb.get('cells', []):
    if cell.get('cell_type') == 'markdown':
        for line in cell.get('source', []):
            line = line.strip()
            if line.startswith('# '):
                print(line[2:])
                sys.exit(0)
print('$nb_name')
" 2>/dev/null || echo "$nb_name")"

    echo "Rendering: $nb_name → content/lab/$slug.md"

    local tmp_html
    tmp_html="$(mktemp /tmp/nb-render-XXXXXX.html)"

    jupyter nbconvert \
        --to html \
        --template basic \
        --no-input \
        --output "$tmp_html" \
        "$nb_path" 2>/dev/null

    local html_body
    html_body="$(cat "$tmp_html")"
    rm -f "$tmp_html"

    cat > "$CONTENT_LAB/$slug.md" << ZOLA_EOF
+++
title = "$title"
description = "Rendered from $nb_name.ipynb — live notebook from the ABG shared workspace"
date = $(date +%Y-%m-%d)
weight = 50

[extra]
domain = "Lab"
rendered_from = "$nb_name.ipynb"
+++

<!-- Auto-generated from $nb_name.ipynb by render_notebooks.sh -->
<!-- Re-render with: bash scripts/render_notebooks.sh --notebook $nb_path -->

$html_body
ZOLA_EOF

    echo "  → $CONTENT_LAB/$slug.md"
}

render_all() {
    local count=0

    for nb in "$NOTEBOOKS_DIR"/*.ipynb; do
        [[ -f "$nb" ]] || continue
        render_notebook "$nb"
        count=$((count + 1))
    done

    if [[ -d "$SHOWCASE_DIR" ]]; then
        for nb in "$SHOWCASE_DIR"/*.ipynb; do
            [[ -f "$nb" ]] || continue
            render_notebook "$nb"
            count=$((count + 1))
        done
    fi

    echo ""
    echo "Rendered $count notebooks to $CONTENT_LAB/"
    echo "Run 'zola build' or 'zola serve' to preview."
}

case "${1:-}" in
    --notebook)
        [[ -z "${2:-}" ]] && { echo "Usage: $0 --notebook <path.ipynb>"; exit 1; }
        render_notebook "$2"
        ;;
    --all|"")
        render_all
        ;;
    *)
        echo "Usage: $0 [--notebook <path>] [--all]"
        exit 1
        ;;
esac
