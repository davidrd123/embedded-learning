#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
MD_DIR="${ROOT_DIR}/md"
mkdir -p "${MD_DIR}"

CHAPTERS=(
  "09a-gpio-overview|587-603"
  "11a-pio-overview-model|876-888"
  "11b-pio-instructions|889-901"
  "12a-uart|961-982"
  "12b-i2c|983-1045"
  "12d-adc-temp|1066-1075"
)

declare -A DISPLAY_TITLES=(
  ["09a-gpio-overview"]="Chapter 9: GPIO"
  ["11a-pio-overview-model"]="Chapter 11: PIO"
  ["11b-pio-instructions"]="Chapter 11: PIO Instructions"
  ["12a-uart"]="Chapter 12: Peripherals"
  ["12b-i2c"]="Chapter 12: Peripherals"
  ["12d-adc-temp"]="Chapter 12: Peripherals"
)

declare -A START_MARKERS=(
  ["09a-gpio-overview"]="## Chapter 9. GPIO"
  ["11a-pio-overview-model"]="## Chapter 11. PIO"
  ["11b-pio-instructions"]="## 11.4. Instruction Set"
  ["12a-uart"]="## Chapter 12. Peripherals"
  ["12b-i2c"]="## 12.2. I2C"
  ["12d-adc-temp"]="## 12.4. ADC and Temperature Sensor"
)

for entry in "${CHAPTERS[@]}"; do
  chapter="${entry%%|*}"
  printed_span="${entry#*|}"

  pdf_path="${ROOT_DIR}/${chapter}.pdf"
  out_path="${MD_DIR}/${chapter}.md"

  if [[ ! -f "${pdf_path}" ]]; then
    echo "Missing source PDF: ${pdf_path}" >&2
    exit 1
  fi

  page_count="$(pdfinfo "${pdf_path}" | awk -F: '/^Pages/{gsub(/ /, "", $2); print $2}')"
  if [[ -z "${page_count}" ]]; then
    echo "Could not read page count for ${pdf_path}" >&2
    exit 1
  fi

  printed_start="${printed_span%-*}"
  printed_end="${printed_span#*-}"
  source_start="$((printed_start + 1))"
  source_end="$((printed_end + 1))"

  tmp_raw="$(mktemp)"
  tmp_body="$(mktemp)"
  tmp_trimmed="$(mktemp)"
  trap 'rm -f "${tmp_raw}" "${tmp_body}" "${tmp_trimmed}"' EXIT

  pdftotext -layout "${pdf_path}" "${tmp_raw}"

  chapter_heading="$(awk '/^[[:space:]]*Chapter [0-9]+[.] /{sub(/^[[:space:]]+/, ""); print; exit}' "${tmp_raw}")"
  if [[ -z "${chapter_heading}" ]]; then
    chapter_heading="Chapter (title not detected)"
  fi
  display_title="$(echo "${chapter_heading}" | sed -E 's/^Chapter ([0-9]+)[.] /Chapter \1: /')"
  if [[ -n "${DISPLAY_TITLES[${chapter}]:-}" ]]; then
    display_title="${DISPLAY_TITLES[${chapter}]}"
  fi

  awk '
    function ltrim(s) { sub(/^[ \t]+/, "", s); return s }
    function rtrim(s) { sub(/[ \t]+$/, "", s); return s }
    function trim(s) { return rtrim(ltrim(s)) }
    function hash_prefix(level,    p, i) {
      p = ""
      for (i = 0; i < level; i++) p = p "#"
      return p
    }
    BEGIN { prev_blank = 1 }
    {
      gsub(/\r/, "")
      gsub(/\f/, "\n")
      line_count = split($0, parts, /\n/)
      for (i = 1; i <= line_count; i++) {
        line = rtrim(parts[i])
        t = trim(line)

        if (t == "") {
          if (!prev_blank) {
            print ""
            prev_blank = 1
          }
          continue
        }

        if (t == "RP2350 Datasheet") continue
        if (t ~ /^[0-9]+$/) continue
        if (t ~ /^[0-9]+[.][0-9]+([.][0-9]+)*[.][[:space:]].*[[:space:]][0-9]+$/) continue

        if (t == "\357\201\232 NOTE") {
          print "> **NOTE**"
          prev_blank = 0
          continue
        }
        if (t == "\357\201\252 IMPORTANT") {
          print "> **IMPORTANT**"
          prev_blank = 0
          continue
        }
        if (t == "\357\201\261 CAUTION") {
          print "> **CAUTION**"
          prev_blank = 0
          continue
        }

        if (t ~ /^Chapter [0-9]+[.] /) {
          if (!prev_blank) print ""
          print "## " t
          print ""
          prev_blank = 1
          continue
        }

        if (t ~ /^Figure [0-9]+[.] /) {
          print "> **" t "**"
          prev_blank = 0
          continue
        }

        if (match(t, /^([0-9]+([.][0-9]+)+)[.]?[[:space:]]+/, m)) {
          dot_count = gsub(/[.]/, "&", m[1])
          level = dot_count + 1
          if (level < 2) level = 2
          if (level > 4) level = 4

          if (!prev_blank) print ""
          print hash_prefix(level) " " t
          print ""
          prev_blank = 1
          continue
        }

        if (t ~ /^[\342\200\242\342\227\246][[:space:]]+/) {
          sub(/^[\342\200\242\342\227\246][[:space:]]+/, "- ", t)
          print t
          prev_blank = 0
          continue
        }

        print t
        prev_blank = 0
      }
    }
  ' "${tmp_raw}" > "${tmp_body}"

  marker="${START_MARKERS[${chapter}]:-}"
  if [[ -n "${marker}" ]]; then
    awk -v marker="${marker}" '
      BEGIN { started = 0 }
      {
        if (!started && index($0, marker) == 1) {
          started = 1
        }
        if (started) print
      }
      END {
        if (!started) {
          exit 2
        }
      }
    ' "${tmp_body}" > "${tmp_trimmed}" || {
      echo "Could not find marker '${marker}' in converted output for ${chapter}" >&2
      exit 1
    }
  else
    cp "${tmp_body}" "${tmp_trimmed}"
  fi

  {
    echo "# RP2350 Datasheet - ${display_title} (Tier 2)"
    echo
    echo "Source: \`rp2350-reference/datasheet/${chapter}.pdf\`"
    echo
    echo "- Printed-page span: ${printed_start}-${printed_end}"
    echo "- Physical PDF-page span in split chapter: 1-${page_count} (source document physical ${source_start}-${source_end})"
    echo "- Conversion method: \`pdftotext -layout\` + automated markdown cleanup"
    echo "- Loss notes: Diagram content is referenced by captions only; complex table layout may be degraded."
    echo
    cat "${tmp_trimmed}"
  } > "${out_path}"

  rm -f "${tmp_raw}" "${tmp_body}" "${tmp_trimmed}"
  trap - EXIT

  echo "Wrote ${out_path}"
done
