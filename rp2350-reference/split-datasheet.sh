#!/usr/bin/env bash
# Split RP2350 datasheet into chapter PDFs using qpdf.
# Fallback/reference implementation (preferred script: split-datasheet.py).
# Usage: ./split-datasheet.sh [source.pdf] [output_dir]

set -euo pipefail

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
SRC="${1:-${SCRIPT_DIR}/rp2350-datasheet.pdf}"
OUT="${2:-${SCRIPT_DIR}/datasheet}"

# Printed page ranges from datasheet footers. These are converted to physical
# PDF pages using an automatically detected offset from chapter bookmarks.
read -r -d '' PRINTED_RANGES <<'EOF' || true
01-introduction 13 23
02-system-bus 24 34
03a-processor-SIO 35 81
03b-processor-interrupts-debug 82 99
03c-cortex-m33-coprocessors 100 122
03d-cortex-m33-processor 123 232
03e-hazard3-processor 233 334
03f-architecture-switching 335 336
04-memory 337 352
05a-bootrom-concepts 353 374
05b-bootrom-apis 375 415
05c-bootrom-usb-uart 416 440
06-power 441 493
07-resets 494 512
08a-clocks-overview 513 553
08b-clocks-oscillators 554 586
09a-gpio-overview 587 603
09b-gpio-io-user-bank 604 759
09c-gpio-io-qspi-pads 760 815
10-security 816 875
11a-pio-overview-model 876 888
11b-pio-instructions 889 901
11c-pio-details-examples 902 960
12a-uart 961 982
12b-i2c 983 1045
12c-spi 1046 1065
12d-adc-temp 1066 1075
12e-pwm 1076 1093
12f-dma 1094 1140
12g-usb 1141 1181
12h-timers-watchdog 1182 1201
12i-hstx 1202 1211
12j-trng-sha256 1212 1225
12k-qspi-qmi 1226 1248
12l-system-control 1249 1267
13-otp 1268 1326
14-electrical 1327 1348
15-appendices 1349 1378
EOF

BOOKMARK_DATA=""
PHYSICAL_RANGES=""
PAGE_OFFSET=0

die() {
    echo "error: $*" >&2
    exit 1
}

warn() {
    echo "warning: $*" >&2
}

require_prereqs() {
    command -v qpdf >/dev/null 2>&1 || die "qpdf is required but not installed"
    command -v pdftk >/dev/null 2>&1 || die "pdftk is required but not installed"
    [[ -f "$SRC" ]] || die "source PDF not found: $SRC"
}

bookmark_page() {
    local title_regex="$1"
    local page
    page="$(
        awk -v re="$title_regex" '
            /^BookmarkTitle: / {
                title = substr($0, 16)
                next
            }
            /^BookmarkPageNumber: / && title ~ re {
                print $2
                exit
            }
        ' <<<"$BOOKMARK_DATA"
    )"
    [[ -n "$page" ]] || die "could not find bookmark matching regex: $title_regex"
    echo "$page"
}

range_start() {
    local range_name="$1"
    awk -v name="$range_name" '$1 == name { print $2; exit }' <<<"$PHYSICAL_RANGES"
}

build_physical_ranges() {
    local chapter1_start appendix_start front_end name start end pstart pend

    BOOKMARK_DATA="$(pdftk "$SRC" dump_data)"
    chapter1_start="$(bookmark_page '^Chapter 1[.] ')"
    appendix_start="$(bookmark_page '^Appendix A: ')"
    PAGE_OFFSET=$((chapter1_start - 13))
    ((PAGE_OFFSET >= 0)) || die "calculated negative page offset: $PAGE_OFFSET"

    front_end=$((chapter1_start - 1))
    ((front_end >= 1)) || die "invalid front matter end page: $front_end"
    PHYSICAL_RANGES="00-front-matter 1 ${front_end}"$'\n'

    while read -r name start end; do
        [[ -z "${name:-}" ]] && continue
        pstart=$((start + PAGE_OFFSET))
        pend=$((end + PAGE_OFFSET))
        PHYSICAL_RANGES+="${name} ${pstart} ${pend}"$'\n'
    done <<<"$PRINTED_RANGES"

    [[ "$(awk '$1=="15-appendices"{print $2; exit}' <<<"$PHYSICAL_RANGES")" == "$appendix_start" ]] \
        || die "appendix start mismatch: range starts at $(awk '$1=="15-appendices"{print $2; exit}' <<<"$PHYSICAL_RANGES"), bookmark starts at $appendix_start"
}

validate_bookmark_alignment() {
    local name regex expected actual

    while IFS='|' read -r name regex; do
        [[ -z "${name:-}" ]] && continue
        expected="$(bookmark_page "$regex")"
        actual="$(range_start "$name")"
        [[ -n "$actual" ]] || die "could not find generated range: $name"
        [[ "$actual" == "$expected" ]] || die "bookmark mismatch for $name: range starts at $actual, bookmark starts at $expected"
    done <<'EOF'
01-introduction|^Chapter 1[.] 
02-system-bus|^Chapter 2[.] 
03a-processor-SIO|^Chapter 3[.] 
04-memory|^Chapter 4[.] 
05a-bootrom-concepts|^Chapter 5[.] 
06-power|^Chapter 6[.] 
07-resets|^Chapter 7[.] 
08a-clocks-overview|^Chapter 8[.] 
09a-gpio-overview|^Chapter 9[.] 
10-security|^Chapter 10[.] 
11a-pio-overview-model|^Chapter 11[.] 
12a-uart|^Chapter 12[.] 
13-otp|^Chapter 13[.] 
14-electrical|^Chapter 14[.] 
15-appendices|^Appendix A:
EOF
}

validate_ranges() {
    local total_pages prev_end line name start end

    total_pages="$(qpdf --show-npages "$SRC")"
    prev_end=0
    line=0

    while read -r name start end; do
        [[ -z "${name:-}" ]] && continue
        line=$((line + 1))

        [[ "$start" =~ ^[0-9]+$ ]] || die "line $line has non-numeric start page: $start"
        [[ "$end" =~ ^[0-9]+$ ]] || die "line $line has non-numeric end page: $end"
        ((start <= end)) || die "line $line has start > end: $name $start-$end"
        ((start > prev_end)) || die "line $line overlaps prior range: $name starts at $start, previous ended at $prev_end"
        ((start == prev_end + 1)) || die "line $line has gap before $name: expected $((prev_end + 1)), got $start"
        ((end <= total_pages)) || die "line $line exceeds source page count ($total_pages): $name ends at $end"

        prev_end=$end
    done <<<"$PHYSICAL_RANGES"

    if ((prev_end < total_pages)); then
        warn "ranges end at $prev_end but source has $total_pages pages (trailing pages excluded)"
    fi
}

split_range() {
    local name="$1" start="$2" end="$3"
    local pages=$((end - start + 1))
    printf '  %s (%dpp: %d-%d)\n' "$name" "$pages" "$start" "$end"
    qpdf "$SRC" --pages . "${start}-${end}" -- "${OUT}/${name}.pdf"
}

main() {
    local name start end

    require_prereqs
    build_physical_ranges
    validate_ranges
    validate_bookmark_alignment

    mkdir -p "$OUT"
    echo "Splitting RP2350 datasheet from: $SRC (page offset: +$PAGE_OFFSET)"
    echo

    while read -r name start end; do
        [[ -z "${name:-}" ]] && continue
        split_range "$name" "$start" "$end"
    done <<<"$PHYSICAL_RANGES"

    echo
    echo "Done! Files in: $OUT"
    ls -lhS "$OUT"/*.pdf | awk '{print $5, $9}' | sed "s|$OUT/||"
}

main "$@"
