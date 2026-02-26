# Session: 2026-02-26 — Initial Reference Library Setup

## What happened
- Downloaded RP2350 datasheet (1,380pp) from pip.raspberrypi.com
- Split into 39 chapter PDFs using qpdf (script: `split-datasheet.sh`)
- Pulled pico-sdk 2.2.0 headers (sparse clone → `sdk-headers/`)
- Cloned full embassy repo → `Code_and_Notes/embassy/`
- Full pico-sdk already existed at `Code_and_Notes/pico-sdk/`
- Built INDEX.md with prioritized reading list, three-layer source map, learning exercises
- Built PATTERN.md with Tier 1→2→3 pipeline, conversion recipes, cost lessons from DiT project
- Incorporated `INCOMING/rp2350-datasheet-reading-guide.md` (web Opus prioritization)
- Promoted `embedded-learning-project.md` → `PROJECT.md` at CrashSpace root
- Moved `dit-learning-ex/` into `rp2350-reference/dit-pattern-example/`
- Cleaned up INCOMING entirely, moved stray files into reference dir

## Key decisions
- Conversion order: 01-intro → 09a-gpio → 11a-pio → 11b-pio → 12b-i2c → 12d-adc (then 02-bus, 12a-uart, 07-resets, 08a-clocks)
- Use `pdftotext -layout` for most chapters (Asciidoctor-generated = clean text)
- Use model-assisted Recipe B only for GPIO function table in 01-introduction
- Tier 3 field guides written AFTER fighting with hardware, not before
- Lightweight pattern (no manifest.yaml) unless we grow past ~20 resources

## Next session: convert `01-introduction.pdf` to Tier 2 markdown
- 10 pages, highest priority
- GPIO function table (Section 1.2.3) needs proper markdown table reconstruction
- Output goes to `datasheet/md/01-introduction.md`

## State of the directory
```
CrashSpace/
├── PROJECT.md                  ← master plan
├── rp2350-reference/
│   ├── INDEX.md                ← reading list + source map (well-populated)
│   ├── PATTERN.md              ← conversion pipeline + tracker
│   ├── rp2350-datasheet.pdf    ← original PDF
│   ├── split-datasheet.sh
│   ├── datasheet/              ← 39 PDFs + md/ (empty, ready for conversions)
│   ├── sdk-headers/            ← headers + SVD
│   ├── dit-pattern-example/    ← pattern reference
│   └── session-logs/           ← you are here
├── Code_and_Notes/
│   ├── pico-sdk/               ← SDK 2.2.0
│   └── embassy/                ← Embassy HAL
├── Electronics/                ← AoE textbooks
├── VideoConductor/             ← concept docs
├── ims-exploration-notes.md
└── rust-embedded-futures-research-notes.md
```
