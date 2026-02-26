# Session: 2026-02-26 — Figure QC Pass & Formula Fixes (Session 3)

## What happened
- Completed **targeted figure QC pass** across all 6 Tier 2 priority chapters
- Fixed **37 garbled figure references** total:
  - 09a-gpio-overview.md: 2 figures (41, 42)
  - 11a-pio-overview-model.md: 4 figures (44, 45, 46, 47)
  - 11b-pio-instructions.md: confirmed 0 figures, no changes needed
  - 12a-uart.md: 5 figures (63–67)
  - 12d-adc-temp.md: 4 figures (107–110)
  - 12b-i2c.md: 22 figures (68–90, done by background agent)
- All garbled pdftotext diagram-label bleed replaced with clean blockquotes:
  `> **Figure N.** Caption text. See source PDF page XXXX.`
- **Additional fixes in 12d-adc-temp.md** based on Codex QC review:
  - Restored missing free-running divider formula: `1 + INT + FRAC/256`
  - Restored missing temperature equation: `T = 27 - (ADC_voltage - 0.706) / 0.001721`
  - Converted register list table 1120 to proper markdown table
  - Deferred register bit-field tables 1121–1125 (readable but not proper markdown)

## Three categories of pdftotext extraction loss identified
1. **Figure bleed**: Diagram spatial labels interleaved with prose (FIXED this session)
2. **Rendered math**: Equations invisible to text extraction (two instances FIXED in 12d-adc)
3. **Structured tables**: Register bit-field tables flattened to text (one FIXED, rest deferred)

## Token cost observations
- Reading chapter PDFs for figure captions was effective but token-heavy
- ~30 PDF pages read across 5 chapter PDFs
- 12b-i2c (22 figures) handled by background agent — good pattern for bulk work
- **For future batches**: consider image-based extraction (pdftoppm → visual) to catch all 3 loss categories in one pass, offloading to Codex

## Quality status after QC pass
| Chapter | Prose | Figures | Tables | Formulas | Status |
|---------|-------|---------|--------|----------|--------|
| 01-introduction | ✓ | n/a | ✓ verified | n/a | **Done** |
| 09a-gpio | ✓ | ✓ fixed | ✓ | n/a | **Done** |
| 11a-pio | ✓ | ✓ fixed | — | — | **Done** |
| 11b-pio-instructions | ✓ | none | — | — | **Done** |
| 12a-uart | ✓ | ✓ fixed | unchecked | unchecked | Needs register table check |
| 12b-i2c | ✓ | ✓ fixed | unchecked | unchecked | Needs register table check |
| 12d-adc | ✓ | ✓ fixed | partial | ✓ fixed | Needs register tables 1121–1125 |

## Next steps
1. Push to GitHub (repo initialized, remote added, not yet pushed)
2. Consider image-based pipeline for register table cleanup and next batch
3. Next batch of chapters: 07-resets, 08a-clocks, 03a-processor-SIO
4. Register table cleanup in 12a-uart, 12b-i2c, 12d-adc (deferred — could use Codex + image approach)
