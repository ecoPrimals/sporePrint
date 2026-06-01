#!/usr/bin/env bash
# DEPRECATED: Superseded by Rust integration test: crates/spore-validate/tests/parity.rs
# Run: cargo test --test parity -- --ignored (requires petalTongue server)
# This script is retained as a quick manual smoke test during development.
# SPDX-License-Identifier: AGPL-3.0-or-later
#
# validate_parity.sh — Compare petalTongue content-direct output vs Zola reference
#
# Validates that the pure-primal rendering pipeline produces structurally
# equivalent HTML to the Zola static site generator for key sporePrint pages.
#
# Usage:
#   ./scripts/validate_parity.sh [petaltongue_port] [zola_public_dir]
#
# Prerequisites:
#   - petalTongue running: petaltongue web --backend content-direct --docroot ./content --port 8080
#   - Zola build: zola build (output in ./public/)

set -euo pipefail

PT_PORT="${1:-8080}"
ZOLA_DIR="${2:-public}"
PT_BASE="http://localhost:${PT_PORT}"

PASS=0
FAIL=0
SKIP=0

red()   { printf "\033[0;31m%s\033[0m" "$1"; }
green() { printf "\033[0;32m%s\033[0m" "$1"; }
yellow(){ printf "\033[0;33m%s\033[0m" "$1"; }

check() {
    local desc="$1"
    local result="$2"
    if [ "$result" = "pass" ]; then
        printf "  [%s] %s\n" "$(green "PASS")" "$desc"
        PASS=$((PASS + 1))
    elif [ "$result" = "skip" ]; then
        printf "  [%s] %s\n" "$(yellow "SKIP")" "$desc"
        SKIP=$((SKIP + 1))
    else
        printf "  [%s] %s\n" "$(red "FAIL")" "$desc"
        FAIL=$((FAIL + 1))
    fi
}

echo "══════════════════════════════════════════════════════════════"
echo "  sporePrint Parity Validation"
echo "  petalTongue: ${PT_BASE}"
echo "  Zola output: ${ZOLA_DIR}/"
echo "══════════════════════════════════════════════════════════════"
echo ""

# Check petalTongue is running
if ! curl -sf "${PT_BASE}/health" > /dev/null 2>&1; then
    if ! curl -sf "${PT_BASE}/" > /dev/null 2>&1; then
        echo "ERROR: petalTongue not reachable at ${PT_BASE}"
        echo "Start with: petaltongue web --backend content-direct --docroot ./content --port ${PT_PORT}"
        exit 1
    fi
fi

# Check Zola build exists
if [ ! -d "$ZOLA_DIR" ]; then
    echo "WARNING: Zola build not found at ${ZOLA_DIR}/"
    echo "Run: zola build"
    echo "Proceeding with petalTongue-only validation..."
    echo ""
fi

# ── Phase 1: Content Serving ──────────────────────────────────────────

echo "Phase 1: Content Serving"

PAGES=(
    "/"
    "/architecture/PRIMAL_CATALOG"
    "/architecture/ECOSYSTEM_ARCHITECTURE"
    "/architecture/NUCLEUS_ARCHITECTURE"
    "/guidestone/cross_substrate_validation"
    "/guidestone/deployment_artifacts"
    "/methodology/CONSTRAINED_EVOLUTION_FORMAL"
    "/science/01_anderson_qs"
    "/products/esotericWebb"
)

for page in "${PAGES[@]}"; do
    status=$(curl -s -o /dev/null -w "%{http_code}" "${PT_BASE}${page}")
    if [ "$status" = "200" ]; then
        check "GET ${page} → 200" "pass"
    else
        check "GET ${page} → ${status} (expected 200)" "fail"
    fi
done

echo ""

# ── Phase 2: Entity Shortcode Resolution ──────────────────────────────

echo "Phase 2: Entity Shortcode Resolution"

ENTITY_PAGE="/guidestone/cross_substrate_validation"
entity_html=$(curl -s "${PT_BASE}${ENTITY_PAGE}")

if echo "$entity_html" | grep -q 'class="entity-ref"'; then
    check "Entity refs rendered as links" "pass"
else
    check "Entity refs rendered as links" "fail"
fi

if echo "$entity_html" | grep -q 'data-entity="guidestone"'; then
    check "Entity data attribute present (guidestone)" "pass"
else
    check "Entity data attribute present (guidestone)" "fail"
fi

if echo "$entity_html" | grep -q '🪨✅'; then
    check "Entity emoji rendered (guideStone 🪨✅)" "pass"
else
    check "Entity emoji rendered (guideStone 🪨✅)" "fail"
fi

# Check no unresolved shortcodes remain
if echo "$entity_html" | grep -q '{{ entity('; then
    check "No unresolved shortcodes" "fail"
else
    check "No unresolved shortcodes" "pass"
fi

echo ""

# ── Phase 3: Modality Support ─────────────────────────────────────────

echo "Phase 3: Modality Support"

if curl -s "${PT_BASE}/architecture/PRIMAL_CATALOG?modality=description" | grep -q "Document:"; then
    check "Description modality returns structured text" "pass"
else
    check "Description modality returns structured text" "fail"
fi

json_output=$(curl -s "${PT_BASE}/architecture/PRIMAL_CATALOG?modality=json")
if echo "$json_output" | python3 -c "import sys,json; json.load(sys.stdin)" 2>/dev/null; then
    check "JSON modality returns valid JSON" "pass"
else
    check "JSON modality returns valid JSON" "fail"
fi

# Test Accept header negotiation
desc_accept=$(curl -s -H "Accept: text/plain" "${PT_BASE}/architecture/PRIMAL_CATALOG")
if echo "$desc_accept" | grep -q "Document:"; then
    check "Accept: text/plain → description output" "pass"
else
    check "Accept: text/plain → description output" "fail"
fi

echo ""

# ── Phase 4: Static Assets ───────────────────────────────────────────

echo "Phase 4: Static Assets"

css_status=$(curl -s -o /dev/null -w "%{http_code}" "${PT_BASE}/css/main.css")
if [ "$css_status" = "200" ]; then
    check "CSS served (/css/main.css)" "pass"
else
    check "CSS served (/css/main.css)" "fail"
fi

css_type=$(curl -s -o /dev/null -w "%{content_type}" "${PT_BASE}/css/main.css")
if echo "$css_type" | grep -q "text/css"; then
    check "CSS Content-Type correct" "pass"
else
    check "CSS Content-Type correct (got: ${css_type})" "fail"
fi

echo ""

# ── Phase 5: Structural Comparison (if Zola build available) ──────────

echo "Phase 5: Structural Comparison (vs Zola)"

if [ -d "$ZOLA_DIR" ]; then
    # Format: "pt_path|zola_slug"
    # Zola slugifies: PRIMAL_CATALOG → primal-catalog, underscores → hyphens
    COMPARE_PAGES=(
        "architecture/PRIMAL_CATALOG|architecture/primal-catalog"
        "guidestone/cross_substrate_validation|guidestone/cross-substrate-validation"
    )

    for entry in "${COMPARE_PAGES[@]}"; do
        pt_path="${entry%%|*}"
        zola_slug="${entry##*|}"
        zola_file="${ZOLA_DIR}/${zola_slug}/index.html"

        if [ ! -f "$zola_file" ]; then
            check "Zola file exists: ${zola_slug}" "skip"
            continue
        fi

        pt_html=$(curl -s "${PT_BASE}/${pt_path}")
        zola_html=$(cat "$zola_file")

        # Extract title from both
        pt_title=$(printf '%s' "$pt_html" | grep -oP '(?<=<title>)[^<]+' | head -1)
        zola_title=$(printf '%s' "$zola_html" | grep -oP '(?<=<title>)[^<]+' | head -1)

        if [ -n "$pt_title" ] && [ -n "$zola_title" ]; then
            # Zola appends site title; PT title should be substring
            if printf '%s' "$zola_title" | grep -qF "$pt_title"; then
                check "Title match: ${pt_path}" "pass"
            else
                check "Title: PT='${pt_title}' vs Zola='${zola_title}'" "fail"
            fi
        else
            check "Title extraction: ${pt_path}" "skip"
        fi

        # Check heading count parity (use grep -o to count occurrences, not lines)
        pt_h2=$(printf '%s' "$pt_html" | grep -o '<h2' | wc -l)
        zola_h2=$(printf '%s' "$zola_html" | grep -o '<h2' | wc -l)
        if [ "$pt_h2" -gt 0 ] && [ "$pt_h2" -eq "$zola_h2" ]; then
            check "H2 count match (${pt_h2}): ${pt_path}" "pass"
        elif [ "$pt_h2" -gt 0 ]; then
            check "H2 count: PT=${pt_h2} vs Zola=${zola_h2}: ${pt_path}" "fail"
        else
            check "H2 count: ${pt_path}" "skip"
        fi
    done
else
    check "Zola build not available — skipping structural comparison" "skip"
fi

echo ""

# ── Summary ──────────────────────────────────────────────────────────

echo "══════════════════════════════════════════════════════════════"
printf "  Results: %s passed, %s failed, %s skipped\n" \
    "$(green "$PASS")" "$(red "$FAIL")" "$(yellow "$SKIP")"
echo "══════════════════════════════════════════════════════════════"

if [ "$FAIL" -gt 0 ]; then
    exit 1
fi
