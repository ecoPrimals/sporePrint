#!/usr/bin/env bash
# SPDX-License-Identifier: AGPL-3.0-or-later
#
# validate_a11y.sh — Accessibility + HTML validation suite for sporePrint
#
# Runs against a Zola build (public/) and validates:
#   Phase 1: HTML5 structural validity (html5validator, ignoring CSS false positives)
#   Phase 2: ARIA landmark completeness
#   Phase 3: Heading hierarchy (no skipped levels)
#   Phase 4: Image alt text coverage
#   Phase 5: Keyboard/focus indicators
#   Phase 6: Meta accessibility (lang, viewport, skip-link)
#
# Usage:
#   ./scripts/validate_a11y.sh [public_dir]
#
# Prerequisites:
#   - pip install html5-validator  (or html5validator)
#   - zola build (output in ./public/)

set -u

PUBLIC="${1:-public}"
PASS=0
FAIL=0
WARN=0

red()    { printf "\033[0;31m%s\033[0m" "$1"; }
green()  { printf "\033[0;32m%s\033[0m" "$1"; }
yellow() { printf "\033[0;33m%s\033[0m" "$1"; }

pass() { printf "  [%s] %s\n" "$(green "PASS")" "$1"; PASS=$((PASS + 1)); }
fail() { printf "  [%s] %s\n" "$(red "FAIL")" "$1"; FAIL=$((FAIL + 1)); }
warn() { printf "  [%s] %s\n" "$(yellow "WARN")" "$1"; WARN=$((WARN + 1)); }

if [ ! -d "$PUBLIC" ]; then
    echo "ERROR: Build directory '${PUBLIC}' not found. Run: zola build"
    exit 1
fi

PAGE_COUNT=$(find "$PUBLIC" -name 'index.html' | wc -l)

echo "══════════════════════════════════════════════════════════════"
echo "  sporePrint Accessibility Validation Suite"
echo "  Target: ${PUBLIC}/ (${PAGE_COUNT} pages)"
echo "  Standard: WCAG 2.2 AAA (target)"
echo "══════════════════════════════════════════════════════════════"
echo ""

# ── Phase 1: HTML5 Structural Validity ─────────────────────────────

echo "Phase 1: HTML5 Structural Validity"

if command -v html5validator &>/dev/null; then
    # The Nu validator doesn't understand modern CSS in inline style=""
    # attributes (color-scheme, light-dark(), 6-char hex — all from Zola's
    # syntax highlighting). We capture raw output and filter structurally.
    VALIDATOR_RAW=$(html5validator --root "$PUBLIC" --format gnu 2>&1 || true)

    # Filter: keep only structural HTML errors, discard CSS false positives
    STRUCTURAL=$(echo "$VALIDATOR_RAW" \
        | grep "error:" \
        | grep -v "CSS:" \
        | grep -v '"color"' \
        | grep -v '"background-color"' \
        | grep -v 'color-scheme' \
        | grep -v 'role "none"' \
        || true)

    ERRORS=$(echo "$STRUCTURAL" | grep -c "error:" || true)

    if [ "$ERRORS" -eq 0 ]; then
        pass "HTML5 structural validation: 0 errors"
    else
        fail "HTML5 structural validation: ${ERRORS} error(s)"
        echo "$STRUCTURAL" | head -10
    fi
else
    warn "html5validator not installed (pip install html5-validator)"
fi

echo ""

# ── Phase 2: ARIA Landmarks ────────────────────────────────────────

echo "Phase 2: ARIA Landmarks"

mapfile -t SAMPLE_PAGES < <(find "$PUBLIC" -name 'index.html' -not -path '*/404*' | sort | head -20)

LANDMARK_PASS=0
LANDMARK_FAIL=0
for page in "${SAMPLE_PAGES[@]}"; do
    has_main=$(grep -cl '<main\b' "$page" 2>/dev/null || true)
    has_nav=$(grep -cl '<nav\b' "$page" 2>/dev/null || true)

    if [ -n "$has_main" ] && [ -n "$has_nav" ]; then
        LANDMARK_PASS=$((LANDMARK_PASS + 1))
    else
        rel_path="${page#"$PUBLIC"/}"
        fail "Missing landmark (<main> or <nav>): ${rel_path}"
        LANDMARK_FAIL=$((LANDMARK_FAIL + 1))
    fi
done

if [ "$LANDMARK_FAIL" -eq 0 ]; then
    pass "ARIA landmarks present in all ${LANDMARK_PASS} sampled pages"
fi

echo ""

# ── Phase 3: Heading Hierarchy ─────────────────────────────────────

echo "Phase 3: Heading Hierarchy (no skipped levels)"

HEADING_ISSUES=0
for page in "${SAMPLE_PAGES[@]}"; do
    rel_path="${page#"$PUBLIC"/}"
    headings=$(grep -oP '<h[1-6]\b' "$page" | grep -oP '[1-6]' || true)

    prev=0
    while IFS= read -r level; do
        if [ -z "$level" ]; then continue; fi
        if [ "$prev" -gt 0 ] && [ "$level" -gt $((prev + 1)) ]; then
            fail "Heading skip h${prev}→h${level}: ${rel_path}"
            HEADING_ISSUES=$((HEADING_ISSUES + 1))
            break
        fi
        prev=$level
    done <<< "$headings"
done

if [ "$HEADING_ISSUES" -eq 0 ]; then
    pass "Heading hierarchy valid in all sampled pages"
fi

echo ""

# ── Phase 4: Image Alt Text ───────────────────────────────────────

echo "Phase 4: Image Alt Text Coverage"

TOTAL_IMGS=0
MISSING_ALT=0
MISSING_FILES=""
mapfile -t ALL_PAGES < <(find "$PUBLIC" -name 'index.html')
for page in "${ALL_PAGES[@]}"; do
    rel_path="${page#"$PUBLIC"/}"
    count=$(grep -coP '<img\b' "$page" 2>/dev/null || true)
    count=${count:-0}
    TOTAL_IMGS=$((TOTAL_IMGS + count))

    if [ "$count" -gt 0 ]; then
        missing=$(grep -cP '<img(?![^>]*\balt=)' "$page" 2>/dev/null || true)
        missing=${missing:-0}
        if [ "$missing" -gt 0 ]; then
            MISSING_ALT=$((MISSING_ALT + missing))
            MISSING_FILES="${MISSING_FILES}    ${rel_path} (${missing} images)\n"
        fi
    fi
done

if [ "$MISSING_ALT" -eq 0 ]; then
    pass "All ${TOTAL_IMGS} images have alt attributes"
else
    fail "${MISSING_ALT}/${TOTAL_IMGS} images missing alt text"
    printf "%b" "$MISSING_FILES" | head -10
fi

echo ""

# ── Phase 5: Skip Link ────────────────────────────────────────────

echo "Phase 5: Skip Link + Focus Indicators"

HOMEPAGE="$PUBLIC/index.html"
if [ -f "$HOMEPAGE" ]; then
    # Zola minifies HTML — quotes may be stripped from class attributes
    if grep -q 'skip-link' "$HOMEPAGE"; then
        pass "Skip-to-content link present"
    else
        fail "No skip-to-content link on homepage"
    fi

    if grep -q ':focus-visible\|:focus' "$PUBLIC/css/"*.css 2>/dev/null; then
        pass "Focus-visible styles defined in CSS"
    else
        warn "No :focus-visible styles found in CSS"
    fi
fi

echo ""

# ── Phase 6: Meta Accessibility ───────────────────────────────────

echo "Phase 6: Meta Accessibility"

if [ -f "$HOMEPAGE" ]; then
    if grep -q 'lang="en"\|lang=en' "$HOMEPAGE"; then
        pass "HTML lang attribute set"
    else
        fail "Missing lang attribute on <html>"
    fi

    if grep -q 'viewport' "$HOMEPAGE"; then
        pass "Viewport meta tag present"
    fi

    if grep -q 'prefers-reduced-motion' "$PUBLIC/css/"*.css 2>/dev/null; then
        pass "prefers-reduced-motion respected in CSS"
    else
        warn "No prefers-reduced-motion media query in CSS"
    fi

    if grep -q 'prefers-contrast' "$PUBLIC/css/"*.css 2>/dev/null; then
        pass "prefers-contrast media query in CSS"
    else
        warn "No prefers-contrast media query in CSS (WCAG AAA)"
    fi
fi

# Check robots.txt and sitemap.xml exist
if [ -f "$PUBLIC/robots.txt" ]; then
    pass "robots.txt present"
else
    fail "robots.txt missing"
fi

if [ -f "$PUBLIC/sitemap.xml" ]; then
    pass "sitemap.xml present"
else
    fail "sitemap.xml missing"
fi

echo ""

# ── Phase 7: ARIA Combobox (Search) ───────────────────────────────

echo "Phase 7: Search Accessibility"

if [ -f "$HOMEPAGE" ]; then
    if grep -q 'role="combobox"' "$HOMEPAGE"; then
        pass "Search input has ARIA combobox role"
    else
        warn "Search input missing ARIA combobox pattern"
    fi

    if grep -q 'listbox' "$HOMEPAGE"; then
        pass "Search results have ARIA listbox role"
    else
        warn "Search results missing ARIA listbox role"
    fi
fi

echo ""

# ── Summary ──────────────────────────────────────────────────────

echo "══════════════════════════════════════════════════════════════"
printf "  Results: %s passed, %s failed, %s warnings\n" \
    "$(green "$PASS")" "$(red "$FAIL")" "$(yellow "$WARN")"
echo "  Pages scanned: ${PAGE_COUNT}"
echo "  Standard: WCAG 2.2 AAA (target)"
echo "══════════════════════════════════════════════════════════════"

if [ "$FAIL" -gt 0 ]; then
    exit 1
fi
