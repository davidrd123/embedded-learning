#!/usr/bin/env python3
"""Split RP2350 datasheet into chapter PDFs using qpdf.

Usage:
    ./split-datasheet.py [source.pdf] [output_dir]

Notes:
    - Preferred implementation for readability.
    - Bash fallback remains available as ./split-datasheet.sh.
"""

from __future__ import annotations

import re
import shutil
import subprocess
import sys
from pathlib import Path
from typing import NoReturn


# Printed page ranges from datasheet footers. Converted to physical PDF pages
# using an automatically detected offset from chapter bookmarks.
PRINTED_RANGES: list[tuple[str, int, int]] = [
    ("01-introduction", 13, 23),
    ("02-system-bus", 24, 34),
    ("03a-processor-SIO", 35, 81),
    ("03b-processor-interrupts-debug", 82, 99),
    ("03c-cortex-m33-coprocessors", 100, 122),
    ("03d-cortex-m33-processor", 123, 232),
    ("03e-hazard3-processor", 233, 334),
    ("03f-architecture-switching", 335, 336),
    ("04-memory", 337, 352),
    ("05a-bootrom-concepts", 353, 374),
    ("05b-bootrom-apis", 375, 415),
    ("05c-bootrom-usb-uart", 416, 440),
    ("06-power", 441, 493),
    ("07-resets", 494, 512),
    ("08a-clocks-overview", 513, 553),
    ("08b-clocks-oscillators", 554, 586),
    ("09a-gpio-overview", 587, 603),
    ("09b-gpio-io-user-bank", 604, 759),
    ("09c-gpio-io-qspi-pads", 760, 815),
    ("10-security", 816, 875),
    ("11a-pio-overview-model", 876, 888),
    ("11b-pio-instructions", 889, 901),
    ("11c-pio-details-examples", 902, 960),
    ("12a-uart", 961, 982),
    ("12b-i2c", 983, 1045),
    ("12c-spi", 1046, 1065),
    ("12d-adc-temp", 1066, 1075),
    ("12e-pwm", 1076, 1093),
    ("12f-dma", 1094, 1140),
    ("12g-usb", 1141, 1181),
    ("12h-timers-watchdog", 1182, 1201),
    ("12i-hstx", 1202, 1211),
    ("12j-trng-sha256", 1212, 1225),
    ("12k-qspi-qmi", 1226, 1248),
    ("12l-system-control", 1249, 1267),
    ("13-otp", 1268, 1326),
    ("14-electrical", 1327, 1348),
    ("15-appendices", 1349, 1378),
]

BOOKMARK_ALIGNMENT: list[tuple[str, str]] = [
    ("01-introduction", r"^Chapter 1[.] "),
    ("02-system-bus", r"^Chapter 2[.] "),
    ("03a-processor-SIO", r"^Chapter 3[.] "),
    ("04-memory", r"^Chapter 4[.] "),
    ("05a-bootrom-concepts", r"^Chapter 5[.] "),
    ("06-power", r"^Chapter 6[.] "),
    ("07-resets", r"^Chapter 7[.] "),
    ("08a-clocks-overview", r"^Chapter 8[.] "),
    ("09a-gpio-overview", r"^Chapter 9[.] "),
    ("10-security", r"^Chapter 10[.] "),
    ("11a-pio-overview-model", r"^Chapter 11[.] "),
    ("12a-uart", r"^Chapter 12[.] "),
    ("13-otp", r"^Chapter 13[.] "),
    ("14-electrical", r"^Chapter 14[.] "),
    ("15-appendices", r"^Appendix A:"),
]


def die(message: str) -> NoReturn:
    print(f"error: {message}", file=sys.stderr)
    raise SystemExit(1)


def warn(message: str) -> None:
    print(f"warning: {message}", file=sys.stderr)


def run_capture(args: list[str]) -> str:
    try:
        result = subprocess.run(args, check=True, text=True, capture_output=True)
        return result.stdout
    except subprocess.CalledProcessError as exc:
        details = exc.stderr.strip() or exc.stdout.strip()
        die(f"command failed: {' '.join(args)} ({details or f'exit {exc.returncode}'})")


def run(args: list[str]) -> None:
    try:
        subprocess.run(args, check=True)
    except subprocess.CalledProcessError as exc:
        die(f"command failed: {' '.join(args)} (exit {exc.returncode})")


def require_prereqs(src: Path) -> None:
    for tool in ("qpdf", "pdftk"):
        if shutil.which(tool) is None:
            die(f"{tool} is required but not installed")
    if not src.is_file():
        die(f"source PDF not found: {src}")


def parse_bookmarks(bookmark_dump: str) -> list[tuple[str, int]]:
    bookmarks: list[tuple[str, int]] = []
    title: str | None = None

    for raw_line in bookmark_dump.splitlines():
        line = raw_line.strip()
        if line.startswith("BookmarkTitle: "):
            title = line[len("BookmarkTitle: ") :]
            continue
        if line.startswith("BookmarkPageNumber: ") and title is not None:
            page_text = line[len("BookmarkPageNumber: ") :]
            try:
                page_num = int(page_text)
            except ValueError:
                die(f"invalid bookmark page number: {page_text}")
            bookmarks.append((title, page_num))
            title = None

    if not bookmarks:
        die("no bookmarks found in PDF metadata")
    return bookmarks


def bookmark_page(bookmarks: list[tuple[str, int]], title_regex: str) -> int:
    pattern = re.compile(title_regex)
    for title, page_num in bookmarks:
        if pattern.search(title):
            return page_num
    die(f"could not find bookmark matching regex: {title_regex}")


def build_physical_ranges(
    bookmarks: list[tuple[str, int]],
) -> tuple[list[tuple[str, int, int]], int]:
    chapter1_start = bookmark_page(bookmarks, r"^Chapter 1[.] ")
    appendix_start = bookmark_page(bookmarks, r"^Appendix A: ")

    first_printed_page = PRINTED_RANGES[0][1]
    page_offset = chapter1_start - first_printed_page
    if page_offset < 0:
        die(f"calculated negative page offset: {page_offset}")

    front_end = chapter1_start - 1
    if front_end < 1:
        die(f"invalid front matter end page: {front_end}")

    physical_ranges: list[tuple[str, int, int]] = [("00-front-matter", 1, front_end)]
    for name, start, end in PRINTED_RANGES:
        physical_ranges.append((name, start + page_offset, end + page_offset))

    range_map = {name: (start, end) for name, start, end in physical_ranges}
    appendices_start = range_map["15-appendices"][0]
    if appendices_start != appendix_start:
        die(
            "appendix start mismatch: "
            f"range starts at {appendices_start}, bookmark starts at {appendix_start}"
        )

    return physical_ranges, page_offset


def validate_ranges(ranges: list[tuple[str, int, int]], total_pages: int) -> None:
    prev_end = 0
    for idx, (name, start, end) in enumerate(ranges, start=1):
        if start > end:
            die(f"line {idx} has start > end: {name} {start}-{end}")
        if start <= prev_end:
            die(
                "line "
                f"{idx} overlaps prior range: {name} starts at {start}, previous ended at {prev_end}"
            )
        if start != prev_end + 1:
            die(
                "line "
                f"{idx} has gap before {name}: expected {prev_end + 1}, got {start}"
            )
        if end > total_pages:
            die(
                f"line {idx} exceeds source page count ({total_pages}): "
                f"{name} ends at {end}"
            )
        prev_end = end

    if prev_end < total_pages:
        warn(
            f"ranges end at {prev_end} but source has {total_pages} pages "
            "(trailing pages excluded)"
        )


def validate_bookmark_alignment(
    ranges: list[tuple[str, int, int]],
    bookmarks: list[tuple[str, int]],
) -> None:
    starts = {name: start for name, start, _ in ranges}
    for name, title_regex in BOOKMARK_ALIGNMENT:
        expected = bookmark_page(bookmarks, title_regex)
        actual = starts.get(name)
        if actual is None:
            die(f"could not find generated range: {name}")
        if actual != expected:
            die(
                f"bookmark mismatch for {name}: "
                f"range starts at {actual}, bookmark starts at {expected}"
            )


def human_size(size_bytes: int) -> str:
    units = ["B", "K", "M", "G", "T"]
    size = float(size_bytes)
    unit_idx = 0
    while size >= 1024 and unit_idx < len(units) - 1:
        size /= 1024
        unit_idx += 1
    if unit_idx == 0:
        return f"{int(size)}{units[unit_idx]}"
    return f"{size:.1f}{units[unit_idx]}"


def split_range(src: Path, out_dir: Path, name: str, start: int, end: int) -> None:
    pages = end - start + 1
    print(f"  {name} ({pages}pp: {start}-{end})")
    out_pdf = out_dir / f"{name}.pdf"
    run(
        [
            "qpdf",
            str(src),
            "--pages",
            ".",
            f"{start}-{end}",
            "--",
            str(out_pdf),
        ]
    )


def main(argv: list[str]) -> int:
    script_dir = Path(__file__).resolve().parent
    src = Path(argv[1]).resolve() if len(argv) > 1 else script_dir / "rp2350-datasheet.pdf"
    out_dir = Path(argv[2]).resolve() if len(argv) > 2 else script_dir / "datasheet"
    if len(argv) > 3:
        print(f"usage: {Path(argv[0]).name} [source.pdf] [output_dir]", file=sys.stderr)
        return 2

    require_prereqs(src)
    bookmarks = parse_bookmarks(run_capture(["pdftk", str(src), "dump_data"]))
    ranges, page_offset = build_physical_ranges(bookmarks)

    total_pages_text = run_capture(["qpdf", "--show-npages", str(src)]).strip()
    try:
        total_pages = int(total_pages_text)
    except ValueError:
        die(f"invalid total page count from qpdf: {total_pages_text}")

    validate_ranges(ranges, total_pages)
    validate_bookmark_alignment(ranges, bookmarks)

    out_dir.mkdir(parents=True, exist_ok=True)
    print(f"Splitting RP2350 datasheet from: {src} (page offset: +{page_offset})")
    print()

    for name, start, end in ranges:
        split_range(src, out_dir, name, start, end)

    print()
    print(f"Done! Files in: {out_dir}")
    pdfs = sorted(out_dir.glob("*.pdf"), key=lambda path: path.stat().st_size, reverse=True)
    for pdf_path in pdfs:
        print(f"{human_size(pdf_path.stat().st_size):>5} {pdf_path.name}")

    return 0


if __name__ == "__main__":
    raise SystemExit(main(sys.argv))
