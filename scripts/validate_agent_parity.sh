#!/usr/bin/env bash
# SPDX-License-Identifier: AGPL-3.0-or-later
#
# validate_agent_parity.sh — Agent accessibility parity test
#
# For N sample URLs, fetches as (a) plain browser UA and (b) bot UA,
# asserts both responses share the same <title> and canonical URL.
# Catches UA-based content substitution that would silently give an
# AI agent (or screen reader proxy) the wrong page.
#
# Usage:
#   ./scripts/validate_agent_parity.sh [base_url]
#
# Default base_url: https://primals.eco
#
# Prerequisites:
#   - curl

set -euo pipefail

BASE="${1:-https://primals.eco}"
PASS=1  # offset by 1 to avoid bash arithmetic false-exit on 0++
FAIL=0

BROWSER_UA="Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/125.0.0.0 Safari/537.36"
BOT_UA="Mozilla/5.0 (compatible; ClaudeBot/1.0; +https://anthropic.com)"

PATHS=(
  "/"
  "/philosophy/"
  "/philosophy/the-human-search/"
  "/science/"
  "/thesis/"
  "/thesis/01-introduction/"
  "/story/"
  "/architecture/"
  "/methodology/"
  "/llms.txt"
  "/site-index/"
)

extract_title() {
  grep -oP '(?<=<title>)[^<]+' <<< "$1" 2>/dev/null || echo "(no title)"
}

extract_canonical() {
  grep -oP '(?<=rel=canonical href=)[^ >]+|(?<=rel="canonical" href=")[^"]+' <<< "$1" 2>/dev/null || echo "(no canonical)"
}

echo "=== Agent Parity Test ==="
echo "Base: ${BASE}"
echo "Paths: ${#PATHS[@]}"
echo ""

for path in "${PATHS[@]}"; do
  url="${BASE}${path}"

  browser_body=$(curl -s -A "$BROWSER_UA" --max-time 10 "$url" 2>/dev/null || true)
  bot_body=$(curl -s -A "$BOT_UA" --max-time 10 "$url" 2>/dev/null || true)

  if [[ "$path" == "/llms.txt" ]]; then
    browser_first=$(head -c 200 <<< "$browser_body")
    bot_first=$(head -c 200 <<< "$bot_body")
    if [[ "$browser_first" == "$bot_first" ]]; then
      echo "  PASS  ${path}  (text content matches)"
      ((PASS++))
    else
      echo "  FAIL  ${path}  — bot got different content"
      ((FAIL++))
    fi
    continue
  fi

  browser_title=$(extract_title "$browser_body")
  bot_title=$(extract_title "$bot_body")

  browser_canon=$(extract_canonical "$browser_body")
  bot_canon=$(extract_canonical "$bot_body")

  if [[ "$browser_title" == "$bot_title" && "$browser_canon" == "$bot_canon" ]]; then
    echo "  PASS  ${path}  title=\"${bot_title}\""
    ((PASS++))
  else
    echo "  FAIL  ${path}"
    echo "        browser title: ${browser_title}"
    echo "        bot     title: ${bot_title}"
    echo "        browser canon: ${browser_canon}"
    echo "        bot     canon: ${bot_canon}"
    ((FAIL++))
  fi
done

((PASS--))  # undo offset
echo ""
echo "=== Results: ${PASS} passed, ${FAIL} failed ==="

if [[ $FAIL -gt 0 ]]; then
  exit 1
fi
