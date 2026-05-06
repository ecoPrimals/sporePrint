#!/usr/bin/env bash
# refresh-metrics.sh — Clone upstream repos and refresh sporePrint metrics
#
# Usage:
#   bash scripts/refresh-metrics.sh <source_id>   # single entity
#   bash scripts/refresh-metrics.sh all            # full sweep
#
# Called by auto-refresh.yml in CI. Can also run locally:
#   bash scripts/refresh-metrics.sh wetspring
#
# Requires: spore-validate binary (built from crates/spore-validate/)
# Expects: sources.toml in sporePrint root

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
SOURCES="$ROOT/sources.toml"
CLONE_DIR="/tmp/sporeprint-refresh"
VALIDATOR="$ROOT/crates/spore-validate/target/release/spore-validate"

SOURCE="${1:-all}"

if [[ ! -f "$SOURCES" ]]; then
    echo "ERROR: sources.toml not found at $SOURCES"
    exit 1
fi

if [[ ! -x "$VALIDATOR" ]]; then
    echo "Building spore-validate..."
    cargo build --release --manifest-path "$ROOT/crates/spore-validate/Cargo.toml"
fi

mkdir -p "$CLONE_DIR"

clone_source() {
    local key="$1"
    local repo

    repo=$(grep -A5 "^\[sources\.${key}\]" "$SOURCES" | grep "^repo" | head -1 | cut -d'"' -f2)
    if [[ -z "$repo" ]]; then
        echo "  SKIP: $key — no repo in sources.toml"
        return 1
    fi

    local type
    type=$(grep -A5 "^\[sources\.${key}\]" "$SOURCES" | grep "^type" | head -1 | cut -d'"' -f2)
    local repo_name
    repo_name=$(echo "$repo" | cut -d'/' -f2)

    local target_dir
    case "$type" in
        primal) target_dir="$CLONE_DIR/primals/$repo_name" ;;
        spring) target_dir="$CLONE_DIR/springs/$repo_name" ;;
        *)      target_dir="$CLONE_DIR/$repo_name" ;;
    esac

    if [[ -d "$target_dir/.git" ]]; then
        echo "  PULL: $key → $target_dir"
        git -C "$target_dir" pull --ff-only --quiet 2>/dev/null || true
    else
        echo "  CLONE: $key → $repo → $target_dir"
        mkdir -p "$(dirname "$target_dir")"
        git clone --depth 1 --quiet "https://github.com/${repo}.git" "$target_dir" 2>/dev/null || {
            echo "  SKIP: $key — clone failed (repo may be private without PAT)"
            return 1
        }
    fi
    return 0
}

get_all_keys() {
    grep '^\[sources\.' "$SOURCES" | sed 's/\[sources\.\(.*\)\]/\1/'
}

echo "sporePrint refresh: source=$SOURCE"
echo "---"

if [[ "$SOURCE" == "all" ]]; then
    for key in $(get_all_keys); do
        clone_source "$key" || true
    done
    echo "---"
    echo "Running spore-validate refresh --write (all)..."
    "$VALIDATOR" --root "$ROOT" refresh "$CLONE_DIR" --write
else
    clone_source "$SOURCE" || exit 1
    echo "---"
    echo "Running spore-validate refresh --write --source $SOURCE..."
    "$VALIDATOR" --root "$ROOT" refresh "$CLONE_DIR" --write --source "$SOURCE"
fi
