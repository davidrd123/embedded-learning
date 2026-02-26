# RP2350 Datasheet — Chapter Index

Split from the 1,380-page RP2350 datasheet (build 2025-07-29).
Each file is sized for uploading to Claude on the web.
`Pages` below are printed footer page numbers (not physical PDF page indices).

---

## Quick Reference

| File | Pages | Topic |
|------|-------|-------|
| `00-front-matter.pdf` | 1–13 (physical) | Colophon, legal, table of contents |
| `01-introduction.pdf` | 13–23 | Chip overview, pinout, GPIO functions, naming |
| `02-system-bus.pdf` | 24–34 | Bus fabric, address map, atomic register access |

## Ch 3 — Processor Subsystem (split into 6 parts)

| File | Pages | Topic |
|------|-------|-------|
| `03a-processor-SIO.pdf` | 35–81 | SIO: CPUID, GPIO control, spinlocks, FIFOs, divider, interpolator, registers |
| `03b-processor-interrupts-debug.pdf` | 82–99 | Interrupts, NMI, event signals, debug (SWD, trace, rescue reset) |
| `03c-cortex-m33-coprocessors.pdf` | 100–122 | GPIOC, double-precision (DCP), redundancy (RCP), FPU |
| `03d-cortex-m33-processor.pdf` | 123–232 | Cortex-M33 features, config, programmer's model, registers |
| `03e-hazard3-processor.pdf` | 233–334 | Hazard3 RISC-V: ISA reference, memory, interrupts, debug, extensions |
| `03f-architecture-switching.pdf` | 335–336 | Arm/RISC-V switching, mixed architecture combos |

## Ch 4–5 — Memory & Bootrom

| File | Pages | Topic |
|------|-------|-------|
| `04-memory.pdf` | 337–352 | ROM, SRAM, Boot RAM, XIP cache, QSPI, OTP overview |
| `05a-bootrom-concepts.pdf` | 353–374 | Partitions, image defs, signing, flash boot, boot sequence |
| `05b-bootrom-apis.pdf` | 375–415 | API functions, SDK access, USB mass storage, UF2 |
| `05c-bootrom-usb-uart.pdf` | 416–440 | PICOBOOT, white-labelling, UART boot, metadata, boot scenarios |

## Ch 6–8 — Power, Resets, Clocks

| File | Pages | Topic |
|------|-------|-------|
| `06-power.pdf` | 441–493 | Power supplies, management, voltage regulator, POWMAN, sleep/dormant |
| `07-resets.pdf` | 494–512 | Chip/system/subsystem resets, POR, brownout, supply monitor |
| `08a-clocks-overview.pdf` | 513–553 | Clock generators, frequency counter, resus, programmer's model |
| `08b-clocks-oscillators.pdf` | 554–586 | XOSC, ROSC, LPOSC, tick generators, PLL |

## Ch 9 — GPIO (split into 3 parts)

| File | Pages | Topic |
|------|-------|-------|
| `09a-gpio-overview.pdf` | 587–603 | GPIO overview, function select, interrupts, pads, software examples |
| `09b-gpio-io-user-bank.pdf` | 604–759 | **Register tables** — IO User Bank (156pp) |
| `09c-gpio-io-qspi-pads.pdf` | 760–815 | **Register tables** — IO QSPI Bank, Pad Control |

## Ch 10–11 — Security & PIO

| File | Pages | Topic |
|------|-------|-------|
| `10-security.pdf` | 816–875 | Secure boot, TrustZone, RISC-V security, access control, DMA, OTP, glitch detector |
| `11a-pio-overview-model.pdf` | 876–888 | PIO overview, programmer's model, control flow, registers, autopull |
| `11b-pio-instructions.pdf` | 889–901 | PIO instruction set: JMP, WAIT, IN, OUT, PUSH, PULL, MOV, IRQ, SET |
| `11c-pio-details-examples.pdf` | 902–960 | PIO functional details, examples (SPI, WS2812, UART, I2C, PWM), registers |

## Ch 12 — Peripherals (split by peripheral)

| File | Pages | Topic |
|------|-------|-------|
| `12a-uart.pdf` | 961–982 | UART: operation, flow control, DMA, interrupts, registers |
| `12b-i2c.pdf` | 983–1045 | I2C: protocols, arbitration, fast mode plus, DMA, registers |
| `12c-spi.pdf` | 1046–1065 | SPI: functional description, operation, registers |
| `12d-adc-temp.pdf` | 1066–1075 | ADC and temperature sensor |
| `12e-pwm.pdf` | 1076–1093 | PWM: programmer's model, registers |
| `12f-dma.pdf` | 1094–1140 | DMA: channels, triggering, DREQ, security, error handling, registers |
| `12g-usb.pdf` | 1141–1181 | USB: architecture, programmer's model, registers |
| `12h-timers-watchdog.pdf` | 1182–1201 | System timers, watchdog, always-on timer |
| `12i-hstx.pdf` | 1202–1211 | HSTX: high-speed serial transmit |
| `12j-trng-sha256.pdf` | 1212–1225 | TRNG (random number generator), SHA-256 accelerator |
| `12k-qspi-qmi.pdf` | 1226–1248 | QSPI memory interface (QMI): transfers, timing, address translation |
| `12l-system-control.pdf` | 1249–1267 | SYSINFO, SYSCFG, TBMAN, BUSCTRL |

## Ch 13–14 & Appendices

| File | Pages | Topic |
|------|-------|-------|
| `13-otp.pdf` | 1268–1326 | OTP: address map, locks, ECC, decommissioning, registers, predefined locations |
| `14-electrical.pdf` | 1327–1348 | Packages (QFN-60/80), thermals, pinout, electrical specs, power consumption |
| `15-appendices.pdf` | 1349–1378 | Register field types, units, hardware revision history, errata |

---

## Suggested Learning Path

1. **Start here:** `01-introduction.pdf` — chip overview and pinout
2. **Architecture:** `02-system-bus.pdf` → `03a-processor-SIO.pdf` → `03f-architecture-switching.pdf`
3. **Boot flow:** `05a-bootrom-concepts.pdf` — how the chip starts up
4. **Clocks & power:** `08a-clocks-overview.pdf` → `06-power.pdf`
5. **GPIO basics:** `09a-gpio-overview.pdf`
6. **PIO (the fun part):** `11a-pio-overview-model.pdf` → `11b-pio-instructions.pdf` → `11c-pio-details-examples.pdf`
7. **Peripherals as needed:** pick from `12a`–`12l` based on what you're building
