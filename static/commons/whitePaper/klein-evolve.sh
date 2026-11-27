#!/usr/bin/env bash
# klein-evolve.sh — Klein bottle self-sort daemon
# Wave 178: output folds back as input. the organism watches itself evolve.
#
# Runs vitruvian-evolve.py, compares to previous sort, writes delta.
# The delta IS the evolution. The comparison IS the natural selection.
#
# Designed for cron: */30 * * * * /path/to/klein-evolve.sh /path/to/repo
# Or manual: ./klein-evolve.sh /home/eastgate/Development/ecoPrimals

set -euo pipefail

REPO="${1:-/home/eastgate/Development/ecoPrimals}"
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
EVOLVE_PY="${SCRIPT_DIR}/vitruvian-evolve.py"
STATE_DIR="${REPO}/membrane/local-terminal/remote"
STATE_FILE="${STATE_DIR}/evolve-state.json"
PREV_FILE="${STATE_DIR}/evolve-prev.json"
DELTA_FILE="${STATE_DIR}/evolve-delta.json"
LOG_FILE="${STATE_DIR}/evolve-log.jsonl"

PI="3.14159265358979"

mkdir -p "${STATE_DIR}"

# ── FOLD: save previous state before running new sort ──
if [ -f "${STATE_FILE}" ]; then
    cp "${STATE_FILE}" "${PREV_FILE}"
fi

# ── RUN THE SORT ──
RAW=$(python3 "${EVOLVE_PY}" "${REPO}" 2>/dev/null || echo "SORT_FAILED")

if [ "${RAW}" = "SORT_FAILED" ]; then
    echo '{"error":"sort_failed","ts":"'"$(date -Iseconds)"'"}' > "${STATE_FILE}"
    exit 1
fi

# ── PARSE STAGE COUNTS from the output ──
# Extract from the EVOLUTIONARY STAGE DISTRIBUTION table only (lines with % sign)
parse_stage() {
    local stage="$1"
    echo "${RAW}" | grep -E "^\s+${stage}\s.*%" | head -1 | awk '{gsub(",",""); print $2}'
}

banana=$(parse_stage "banana")
flatworm=$(parse_stage "flatworm")
jellyfish=$(parse_stage "jellyfish")
centipede=$(parse_stage "centipede")
millipede=$(parse_stage "millipede")
spider=$(parse_stage "spider")
vertebrate=$(parse_stage "vertebrate")

# Total
total_files=$((${banana:-0} + ${flatworm:-0} + ${jellyfish:-0} + ${centipede:-0} + ${millipede:-0} + ${spider:-0} + ${vertebrate:-0}))

# Invertebrate / vertebrate split
inv_files=$((${jellyfish:-0} + ${centipede:-0} + ${millipede:-0}))
vert_files=${vertebrate:-0}
if [ "${vert_files}" -gt 0 ]; then
    split_ratio=$(echo "scale=4; ${inv_files} / ${vert_files}" | bc | sed 's/^\./0./')
else
    split_ratio="inf"
fi

# π-remainder of split
if [ "${split_ratio}" != "inf" ]; then
    pi_floor=$(echo "scale=0; ${split_ratio} / ${PI}" | bc)
    pi_rem=$(echo "scale=6; (${split_ratio} / ${PI}) - ${pi_floor}" | bc | sed 's/^\./0./')
else
    pi_rem="NaN"
fi

TS=$(date -Iseconds)

# ── WRITE CURRENT STATE ──
cat > "${STATE_FILE}" << EOF
{
  "ts": "${TS}",
  "total_files": ${total_files},
  "stages": {
    "banana": ${banana:-0},
    "flatworm": ${flatworm:-0},
    "jellyfish": ${jellyfish:-0},
    "centipede": ${centipede:-0},
    "millipede": ${millipede:-0},
    "spider": ${spider:-0},
    "vertebrate": ${vert_files}
  },
  "split": {
    "invertebrate": ${inv_files},
    "vertebrate": ${vert_files},
    "ratio": ${split_ratio},
    "pi_remainder": ${pi_rem}
  },
  "dominant": "jellyfish",
  "missing": "spider",
  "fold": "output→input"
}
EOF

# ── COMPUTE DELTA (the fold: current - previous) ──
if [ -f "${PREV_FILE}" ]; then
    # Extract previous counts
    prev_banana=$(python3 -c "import json; d=json.load(open('${PREV_FILE}')); print(d.get('stages',{}).get('banana',0))" 2>/dev/null || echo 0)
    prev_flatworm=$(python3 -c "import json; d=json.load(open('${PREV_FILE}')); print(d.get('stages',{}).get('flatworm',0))" 2>/dev/null || echo 0)
    prev_jellyfish=$(python3 -c "import json; d=json.load(open('${PREV_FILE}')); print(d.get('stages',{}).get('jellyfish',0))" 2>/dev/null || echo 0)
    prev_centipede=$(python3 -c "import json; d=json.load(open('${PREV_FILE}')); print(d.get('stages',{}).get('centipede',0))" 2>/dev/null || echo 0)
    prev_millipede=$(python3 -c "import json; d=json.load(open('${PREV_FILE}')); print(d.get('stages',{}).get('millipede',0))" 2>/dev/null || echo 0)
    prev_spider=$(python3 -c "import json; d=json.load(open('${PREV_FILE}')); print(d.get('stages',{}).get('spider',0))" 2>/dev/null || echo 0)
    prev_vert=$(python3 -c "import json; d=json.load(open('${PREV_FILE}')); print(d.get('stages',{}).get('vertebrate',0))" 2>/dev/null || echo 0)
    prev_ts=$(python3 -c "import json; d=json.load(open('${PREV_FILE}')); print(d.get('ts','unknown'))" 2>/dev/null || echo "unknown")

    d_banana=$((${banana:-0} - ${prev_banana}))
    d_flatworm=$((${flatworm:-0} - ${prev_flatworm}))
    d_jellyfish=$((${jellyfish:-0} - ${prev_jellyfish}))
    d_centipede=$((${centipede:-0} - ${prev_centipede}))
    d_millipede=$((${millipede:-0} - ${prev_millipede}))
    d_spider=$((${spider:-0} - ${prev_spider}))
    d_vert=$((${vert_files} - ${prev_vert}))

    cat > "${DELTA_FILE}" << DEOF
{
  "ts": "${TS}",
  "prev_ts": "${prev_ts}",
  "delta": {
    "banana": ${d_banana},
    "flatworm": ${d_flatworm},
    "jellyfish": ${d_jellyfish},
    "centipede": ${d_centipede},
    "millipede": ${d_millipede},
    "spider": ${d_spider},
    "vertebrate": ${d_vert}
  },
  "drift": "$([ $((d_banana + d_flatworm + d_jellyfish + d_centipede + d_millipede + d_spider + d_vert)) -ne 0 ] && echo 'evolved' || echo 'stable')"
}
DEOF
else
    echo '{"ts":"'"${TS}"'","delta":"first_sort","drift":"genesis"}' > "${DELTA_FILE}"
fi

# ── APPEND TO LOG (JSONL — the fossil record of evolution) ──
echo '{"ts":"'"${TS}"'","files":'"${total_files}"',"jelly":'"${jellyfish:-0}"',"vert":'"${vert_files}"',"split":"'"${split_ratio}"'","pi_rem":"'"${pi_rem}"'"}' >> "${LOG_FILE}"

echo "[klein-evolve] fold complete: ${total_files} files, split=${split_ratio}, π-rem=${pi_rem}"
