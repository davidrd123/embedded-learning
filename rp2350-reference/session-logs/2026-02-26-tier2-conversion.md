# Session: 2026-02-26 — Tier 2 Conversion & Repo Setup (Session 2)

## What happened
- Fixed Section 1.1 of 01-introduction.md: was summarized bullets, restored to original prose
- Established **Tier 2 fidelity standard** in PATTERN.md: faithful prose, reconstructed tables, blockquote figure notes, metadata headers
- Discovered and diagnosed off-by-one bug in split-datasheet.sh: printed page numbers ≠ physical PDF page numbers (offset = 1, one title page as front matter)
- Learned `pdftk dump_data` exposes bookmark metadata with correct physical page numbers
- Codex rewrote split script: bash version now auto-detects offset from bookmarks, cross-validates against multiple chapter boundaries
- Codex also produced Python, Go, Rust, and Clojure versions of the split script
- Compared all four implementations — Clojure was only one that fixed the hardcoded `13`
- Codex completed Tier 2 conversions for all 6 priority chapters: 09a-gpio, 11a-pio, 11b-pio, 12a-uart, 12b-i2c, 12d-adc
- Initialized git repo (`embedded-learning`), created .gitignore, made initial commit (164 files)
- Remote added: git@github.com:davidrd123/embedded-learning.git (not yet pushed)
- Reviewed Tier 2 quality: prose fidelity is good, figure bleed is the main issue (garbled captions interleaved with text)

## Quality assessment of Tier 2 conversions
- **Prose**: Faithful to original — the Section 1.1 summarization problem did NOT repeat in later chapters
- **Tables**: GPIO function table (01-intro) verified cell-by-cell, all correct
- **Figures**: Main weakness. pdftotext can't separate diagram label text from body text, producing garbled interleaved output. ~50 figure references across all files, ~25 in 12b-i2c alone (timing diagrams)
- **Register tables in 12b-i2c**: Codex noted these are "faithful extracted text" with loss noted — likely need targeted cleanup

## Planned next: targeted figure QC pass
Strategy to avoid blowing token budget:
1. Grep markdown for figure references (done — ~50 total)
2. Read only the specific PDF pages where figures appear (~15-20 pages, not 138)
3. Replace garbled caption text with clean blockquotes: `> **Figure N** — description. See source PDF page X.`
4. I2C timing diagrams (25 figures) mostly follow same pattern — spot-check 5-6 pages, describe the pattern once
5. Also spot-check any tables that look garbled in the markdown

Chapters to QC (page estimates for PDF reading):
- 09a-gpio: ~2 pages (2 figures)
- 11a-pio: ~3 pages (4 figures)
- 11b-pio: check if it has figures
- 12a-uart: ~4 pages (5 figures)
- 12b-i2c: ~6 pages (25 figures, but most are similar timing diagrams)
- 12d-adc: ~3 pages (4 figures)

## Key learnings (for Codex briefing)
- PATTERN.md now has explicit Tier 2 fidelity standard — point Codex there
- Figure handling instruction: use blockquote format `> **Figure N** — description. See source PDF page X.`
- Don't interleave diagram spatial text with prose
- The hardcoded `13` in offset calculation should be derived from PRINTED_RANGES[0]

## State of the directory
```
CrashSpace/                        ← git repo initialized, remote added, not pushed
├── .gitignore                     ← ignores *.pdf, pico-sdk/, embassy/, __pycache__/
├── PROJECT.md                     ← Codex fixing RP2040→RP2350
├── rp2350-reference/
│   ├── INDEX.md
│   ├── PATTERN.md                 ← updated with Tier 2 fidelity standard
│   ├── split-datasheet.{sh,py,go,rs,clj}  ← polyglot split scripts
│   ├── datasheet/
│   │   ├── md/
│   │   │   ├── 01-introduction.md     ← QC'd, Section 1.1 fixed
│   │   │   ├── 09a-gpio-overview.md   ← needs figure cleanup
│   │   │   ├── 11a-pio-overview-model.md  ← needs figure cleanup
│   │   │   ├── 11b-pio-instructions.md    ← needs figure check
│   │   │   ├── 12a-uart.md            ← needs figure cleanup
│   │   │   ├── 12b-i2c.md            ← needs figure cleanup + register table check
│   │   │   └── 12d-adc-temp.md        ← needs figure cleanup
│   │   └── convert-tier2-priority.sh  ← Codex's conversion script
│   └── session-logs/
└── Code_and_Notes/
```

## Next session
1. ~~Targeted figure QC pass~~ — **DONE** (see 2026-02-26-figure-qc-pass.md)
2. Push to GitHub
3. Consider next batch: 07-resets, 08a-clocks, 03a-processor-SIO
