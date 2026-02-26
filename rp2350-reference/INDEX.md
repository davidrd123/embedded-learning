# RP2350 Reference Library

Local reference collection for learning the RP2350 microcontroller.

**Source datasheet:** 1,380 pages, build 2025-07-29 (Asciidoctor PDF)
**Source SDK:** pico-sdk 2.2.0 — full clone at `../Code_and_Notes/pico-sdk/`
**SDK headers:** copied to `sdk-headers/` for quick reference

---

## Directory Layout

```
rp2350-reference/
├── INDEX.md                    ← you are here
├── datasheet/                  ← chapter PDFs (split for Claude web upload)
│   ├── INDEX.md                ← detailed chapter-by-chapter guide
│   └── *.pdf                   ← 39 chapter PDFs
├── split-datasheet.py          ← preferred splitter (readable Python version)
├── split-datasheet.sh          ← fallback/reference splitter (bash)
├── split-datasheet.go          ← Go rewrite
├── split-datasheet.rs          ← Rust rewrite
├── split-datasheet.clj         ← Clojure rewrite
└── sdk-headers/                ← pico-sdk rp2350-specific definitions
    ├── hardware_regs/          ← raw register #defines (auto-generated from chip def)
    ├── hardware_structs/       ← C struct overlays for register access
    ├── pico_platform/          ← platform abstraction layer
    ├── boot_stage2/            ← second-stage bootloader sources
    ├── RP2350.svd              ← CMSIS System View Description (machine-readable chip def)
    └── rp2350{a,b}_interface_pins.json

Also available — full SDK and HAL clones for reading:

  ../Code_and_Notes/pico-sdk/src/rp2_common/     ← C SDK HAL (83 modules)
  ../Code_and_Notes/embassy/embassy-rp/src/       ← Embassy Rust HAL
  ../Code_and_Notes/embassy/embassy-rp/src/pio_programs/  ← Pre-built PIO programs
```

---

## How This Works

See [`PATTERN.md`](PATTERN.md) for the conversion pipeline (Tier 1→2→3),
recipes for PDF extraction, cost lessons from the sister DiT learning project,
field guide templates, and the conversion tracker.

Sister project (full-weight version of this pattern):
`~/Projects/Learning/SelfStructure/learn-distrib-dit/`

---

## Conversion Priority

Optimized for "what you'll wire and debug this month."
Convert = extract to markdown for Claude web conversations.

### Convert Now (~150pp) — Weeks 1–4

| File | Pages | Why | Week | SDK companion |
|------|-------|-----|------|---------------|
| `01-introduction` | 11pp | Pinout, GPIO function tables — constant reference | 1 | `addressmap.h`, `platform_defs.h` |
| `02-system-bus` | 11pp | Address map, atomic set/clear/XOR aliases (shows up everywhere) | 1 | `addressmap.h` |
| `09a-gpio-overview` | 17pp | Function select, interrupts, pads — how pins actually work | 1 | `gpio.c`, `io_bank0.h` struct |
| `11a-pio-overview-model` | 13pp | NeoPixels are week 1 | 1 | `pio.c`, `pio.h` struct |
| `11b-pio-instructions` | 13pp | Understanding the WS2812 PIO program | 1–2 | `pio_instructions.h` |
| `12a-uart` | 22pp | Day-1 debugging path — serial output | 1 | `uart.h` struct |
| `12b-i2c` | 63pp | IMU + VL53L0X | 3 | `i2c.c`, `i2c.h` struct |
| `12d-adc-temp` | 10pp | Mic (SPW2430) — quick win | 3–4 | `adc.h` struct |

#### How to Read Each One

**01-introduction** — Read closely. The GPIO function table (Section 1.2.3) is
the single most-referenced page: which pin can be I2C0_SDA, PIO0, PWM, etc.
Cross-reference with the Pico 2W board pinout diagram.

**02-system-bus** — Focus on the address map (Section 2.2) and atomic register
access (Section 2.1.3). The set/clear/XOR aliases at +0x1000/+0x2000/+0x3000
show up everywhere in HAL code. Short chapter, read it all.

**09a-gpio-overview** — Read overview and function select sections. Skim interrupts
(Embassy handles this). The pads section matters: drive strength, slew rate,
pull-up/down, input hysteresis. This is where you learn what pins can do
*electrically*.

**11a + 11b (PIO)** — Read 11a for the mental model: state machines, FIFOs, clock
dividers, autopull. Read 11b for the instruction set. You don't need to write
PIO programs from scratch yet — you'll use Embassy's `ws2812.rs` — but
understanding what the program does makes debugging possible.

**12a (UART)** — Focus on baud rate calculation, FIFO depth, and the interaction
with clocks. When your serial output is garbage, this is where you look.

**12b (I2C)** — Skim the protocol overview (you know I2C basics). Focus on: FIFO
depth, DMA handshaking, abort sources (why a transfer failed), speed modes.
Register descriptions are reference material, not sequential reading.

**12d (ADC)** — Short and sweet. Key facts: 12-bit, 500 kSPS, 4 channels on
GPIO26-29, round-robin mode, internal temp sensor on channel 4. Read it all.

### Page Budget

| Priority | Pages | % of datasheet |
|----------|-------|----------------|
| Convert now | ~150 | 11% |
| Convert early | ~90 | 7% |
| Convert when needed | ~170 | 12% |
| Don't convert | ~970 | 70% |

You get ~80% of the practical value from ~11% of the document.

### Convert Early (~90pp) — Keep Handy

Most "nothing works" bugs are reset gating or clock misconfiguration.

| File | Pages | Why | When |
|------|-------|-----|------|
| `07-resets` | 19pp | Reset gating is the #1 "peripheral won't respond" cause | First debug session |
| `08a-clocks-overview` | 41pp | PIO timing, baud rates — everything depends on clocks | When timing is off |
| `03a-processor-SIO` | 47pp | Spinlocks, FIFOs, interpolator — multicore primitives | When using both cores |

### Convert When Needed — Weeks 5–8

| File | Pages | Why | When |
|------|-------|-----|------|
| `12e-pwm` | 18pp | Piezo buzzer, haptics | Week 5 |
| `12g-usb` | 41pp | USB MIDI | Week 7 |
| `11c-pio-details-examples` | 59pp | Deeper PIO once you've hit the basics | When curious |
| `12f-dma` | 47pp | What Embassy does under the hood | When curious |
| `06-power` | 53pp | Sleep modes, power domains | Battery optimization |

### Don't Convert — But Don't Forget

These stay as PDF. Open them when you need a specific register or spec.

| File | Pages | When you'd open it |
|------|-------|--------------------|
| `09b-gpio-io-user-bank` | 156pp | Grepping for GPIOx_CTRL function select, pad config, IRQ status |
| `09c-gpio-io-qspi-pads` | 56pp | QSPI pin configuration |
| `03d-cortex-m33-processor` | 110pp | If you ever need Cortex-M33 system register details |
| `03e-hazard3-processor` | 102pp | Only if you switch to RISC-V mode |
| `05a/b/c-bootrom-*` | 88pp | probe-rs handles boot; open if boot fails |
| `10-security` | 60pp | TrustZone — not on the roadmap |
| `13-otp` | 59pp | Fuse memory — production use only |
| `14-electrical` | 22pp | Package specs, absolute max ratings — when doing PCB layout |
| `15-appendices` | 30pp | Register field type reference, errata |
| `12i-hstx` | 10pp | Video output — not on roadmap |
| `12j-trng-sha256` | 14pp | Crypto peripherals |
| `12k-qspi-qmi` | 23pp | Flash interface details |
| `12l-system-control` | 19pp | SYSINFO, SYSCFG registers |

### Special Case: Minimal Extract from 09b

Don't convert the full 156pp register dump, but extract just:
- GPIOx_CTRL register layout (function select field)
- Pad configuration registers (drive strength, pull up/down, slew rate)
- IRQ status/enable registers

These are the registers you'll inevitably grep for when Embassy abstractions leak.

---

## Datasheet Chapters (39 PDFs) — Full List

See [`datasheet/INDEX.md`](datasheet/INDEX.md) for complete breakdown with page ranges.

---

## SDK Headers — Quick Reference

### Start Here (small, high-value files)

| File | Size | What it tells you |
|------|------|-------------------|
| `hardware_regs/.../addressmap.h` | 3.8K | **Entire memory map** — where every peripheral lives |
| `hardware_regs/.../intctrl.h` | 6.0K | **Interrupt numbering** — which IRQ is which peripheral |
| `hardware_regs/.../dreq.h` | 5.6K | **DMA request mappings** — which DREQ triggers which channel |
| `hardware_regs/.../platform_defs.h` | 5.6K | Platform constants (num GPIOs, num PIOs, etc.) |
| `pico_platform/.../platform.h` | 8.8K | Platform abstraction macros, compiler helpers |

### hardware_structs/ — The Readable API Layer

These are the C structs you actually use when coding. Each maps to a peripheral's
memory-mapped register block (e.g., `sio_hw->cpuid`, `uart_hw->dr`).

| File | Size | Peripheral |
|------|------|------------|
| `sio.h` | 15K | SIO (spinlocks, FIFOs, divider, interpolator) |
| `pio.h` | 18K | PIO state machines |
| `dma.h` | 15K | DMA channels |
| `clocks.h` | 28K | Clock generators and sources |
| `uart.h` | 8.7K | UART |
| `spi.h` | 5.4K | SPI |
| `i2c.h` | 19K | I2C |
| `pwm.h` | 10K | PWM |
| `adc.h` | 4.9K | ADC |
| `usb.h` | 31K | USB |
| `timer.h` | 7.2K | System timers |
| `watchdog.h` | 5.3K | Watchdog |
| `io_bank0.h` | 21K | GPIO IO bank 0 |
| `powman.h` | 18K | Power management |
| `xip.h` | 2.7K | XIP (execute-in-place) cache control |
| `otp.h` | 9.0K | OTP memory |
| `hstx_ctrl.h` | 5.5K | HSTX (high-speed serial TX) |
| `sha256.h` | 2.2K | SHA-256 accelerator |
| `trng.h` | 7.3K | True random number generator |
| `scb.h` | 17K | System control block (Cortex-M33) |

### hardware_regs/ — Raw Register Definitions

Auto-generated from the chip definition. Exhaustive but verbose — use when you
need the exact bit field name or offset. The big ones:

| File | Size | Notes |
|------|------|-------|
| `io_bank0.h` | 1.4M | GPIO Bank 0 — every pin's ctrl/status register |
| `otp_data.h` | 692K | OTP field definitions |
| `dma.h` | 523K | All DMA channel registers |
| `usb_device_dpram.h` | 462K | USB device DPRAM layout |
| `m33.h` | 447K | Cortex-M33 system registers |
| `sio.h` | 127K | SIO registers |
| `pio.h` | 177K | PIO registers |

### RP2350.svd (5.3MB)

The CMSIS **System View Description** file — the machine-readable chip definition
that all the headers are generated from. Useful for:
- Importing into debug tools (OpenOCD, probe-rs, Segger)
- Generating register views in IDEs
- Building custom tooling

### boot_stage2/ — Second-Stage Bootloaders

Assembly sources for the boot2 stage that configures the external flash interface.
Different files for different flash chips:

| File | Flash chip |
|------|-----------|
| `boot2_w25q080.S` | Winbond W25Q080 (Pico 2 default) |
| `boot2_generic_03h.S` | Generic SPI flash (03h read command) |
| `boot2_at25sf128a.S` | Adesto AT25SF128A |
| `boot2_is25lp080.S` | ISSI IS25LP080 |
| `boot2_w25x10cl.S` | Winbond W25X10CL |
| `boot2_usb_blinky.S` | USB blinky (diagnostic) |

### Pin Interface Files

- `rp2350a_interface_pins.json` — QFN-60 package pin mapping
- `rp2350b_interface_pins.json` — QFN-80 package pin mapping

---

## Three-Layer Reading Map

Each peripheral has source at all three layers. Read top-down (Embassy → C → datasheet)
when debugging, bottom-up (datasheet → C → Embassy) when learning.

Paths relative to `../Code_and_Notes/`:

| Topic | Datasheet | Embassy Rust | C SDK HAL | Lines (Rust / C) |
|-------|-----------|-------------|-----------|-----------------|
| **GPIO** | `09a-gpio-overview` | `embassy-rp/src/gpio.rs` | `pico-sdk/.../hardware_gpio/gpio.c` | 1449 / 308 |
| **PIO** | `11a/b/c-pio-*` | `embassy-rp/src/pio/mod.rs` | `pico-sdk/.../hardware_pio/pio.c` | 1563 / 469 |
| **PIO instr** | `11b-pio-instructions` | `embassy-rp/src/pio/instr.rs` | — | 119 / — |
| **UART** | `12a-uart` | `embassy-rp/src/uart/mod.rs` | `pico-sdk/.../hardware_uart/` | 1559 / — |
| **I2C** | `12b-i2c` | `embassy-rp/src/i2c.rs` | `pico-sdk/.../hardware_i2c/i2c.c` | 924 / 358 |
| **ADC** | `12d-adc-temp` | `embassy-rp/src/adc.rs` | `pico-sdk/.../hardware_adc/` | 471 / — |
| **PWM** | `12e-pwm` | `embassy-rp/src/pwm.rs` | `pico-sdk/.../hardware_pwm/` | 601 / — |
| **DMA** | `12f-dma` | `embassy-rp/src/dma.rs` | `pico-sdk/.../hardware_dma/` | 369 / — |
| **USB** | `12g-usb` | `embassy-rp/src/usb.rs` | `pico-sdk/.../hardware_usb/` | 836 / — |
| **SPI** | `12c-spi` | `embassy-rp/src/spi.rs` | `pico-sdk/.../hardware_spi/` | 765 / — |
| **Clocks** | `08a-clocks-overview` | `embassy-rp/src/clocks.rs` | `pico-sdk/.../hardware_clocks/clocks.c` | 2168 / 443 |
| **Resets** | `07-resets` | `embassy-rp/src/reset.rs` | `pico-sdk/.../hardware_resets/` | — / — |
| **Watchdog** | `12h-timers-watchdog` | `embassy-rp/src/watchdog.rs` | `pico-sdk/.../hardware_watchdog/` | — / — |
| **Multicore** | `03a-processor-SIO` | `embassy-rp/src/multicore.rs` | — | 1079 / — |
| **Spinlocks** | `03a-processor-SIO` | `embassy-rp/src/spinlock.rs` | — | — / — |
| **TRNG** | `12j-trng-sha256` | `embassy-rp/src/trng.rs` | — | 449 / — |

### Pre-Built PIO Programs (Embassy)

`embassy-rp/src/pio_programs/` — ready-to-use PIO programs:

| File | What it does |
|------|-------------|
| `ws2812.rs` | WS2812/NeoPixel LED driver — **week 1 reading** |
| `uart.rs` | PIO-based UART |
| `spi.rs` | PIO-based SPI |
| `i2s.rs` | I2S audio |
| `pwm.rs` | PIO-based PWM |
| `rotary_encoder.rs` | Rotary encoder input |
| `onewire.rs` | 1-Wire protocol (DS18B20 etc.) |
| `stepper.rs` | Stepper motor driver |
| `hd44780.rs` | Character LCD driver |
| `clk.rs` / `clock_divider.rs` | Clock generation |

---

## Learning Path

### Week 1: Pins and Blinking
1. `01-introduction` + `addressmap.h` — what is this chip, where does everything live
2. `02-system-bus` — how the CPU talks to peripherals (atomic aliases!)
3. `09a-gpio-overview` + `gpio.c` HAL — how pins work
4. `12a-uart` — serial debugging
5. `11a-pio-overview-model` + `11b-pio-instructions` — NeoPixels via PIO

### Week 3: Sensors
6. `12b-i2c` + `i2c.c` HAL — IMU and VL53L0X
7. `12d-adc-temp` + `adc.h` struct — microphone input

### When Things Break
8. `07-resets` — "peripheral won't respond" → check reset gating
9. `08a-clocks-overview` + `clocks.c` HAL — timing/baud rate issues

### Deeper Dives (as needed)
10. `03a-processor-SIO` — multicore, spinlocks, interpolator
11. `12e-pwm` → `12g-usb` → `11c-pio-details` → `12f-dma`

### Rosetta Stone: When Embassy Confuses You
Read the C HAL source in `../Code_and_Notes/pico-sdk/src/rp2_common/`.
The C implementation is often the clearest bridge between the datasheet
register description and what the Rust HAL is doing underneath.

---

## Learning Through Doing — Engagement Ideas

These aren't just "read the docs" — they're small challenges that build understanding
by making you trace through the stack.

### Trace Exercises (read code, draw connections)

- **"Follow the pin"**: Pick GPIO 25 (the LED). Trace what happens when you call
  `Output::new()` in Embassy → what does `gpio.rs` write? → which registers in
  the C SDK? → what does the datasheet say those bits do? Draw the path.

- **"Clock detective"**: Your PIO state machine runs at 125MHz. Where does that
  number come from? Trace from the crystal oscillator through the PLL, clock
  generators, and PIO clock divider. Read `clocks.rs` + `08a-clocks-overview`.

- **"What does `init()` do?"**: Read `embassy-rp/src/lib.rs` — the `init()` function
  that runs before your code. What peripherals does it configure? What resets does
  it release? Map each step to its datasheet chapter.

### Build-to-Learn Challenges

- **Bare-register blinky**: Before using Embassy, write a blinky that pokes registers
  directly through the PAC. Forces you to understand the GPIO control register,
  the SIO output register, and reset gating.

- **PIO from scratch**: Don't use `ws2812.rs` — read `11b-pio-instructions` and write
  your own WS2812 PIO program from the timing diagram. Then compare yours to
  Embassy's version. What did they do differently? Why?

- **I2C by hand**: Before using Embassy's I2C, read through `i2c.rs` and `i2c.c`
  side by side. Predict what registers get written for a simple read transaction.
  Then add debug prints and verify.

### Debugging Exercises (break things intentionally)

- **Wrong clock**: Configure the PIO clock divider wrong on purpose. Observe what
  the NeoPixels do. Calculate what frequency you're actually running at.

- **Forget to release reset**: Comment out the reset release for a peripheral.
  Read the error. Then read `07-resets` to understand what reset gating is.

- **Pin function conflict**: Assign two peripherals to the same pin. What happens?
  Read `09a-gpio-overview` section on function select to understand why.

### Source Reading Sessions (Socratic dialogue topics)

Good files to read together and discuss (in order of complexity):

1. `embassy-rp/src/adc.rs` (471 lines) — simplest peripheral, clean async pattern
2. `embassy-rp/src/gpio.rs` (1449 lines) — type-state pattern, flex pin, interrupts
3. `embassy-rp/src/pio_programs/ws2812.rs` — PIO program you'll use day 1
4. `embassy-rp/src/pio/instr.rs` (119 lines) — how PIO instructions are encoded
5. `embassy-rp/src/i2c.rs` (924 lines) — async I2C with DMA, error handling
6. `embassy-rp/src/clocks.rs` (2168 lines) — the most complex init, PLL math
7. `embassy-rp/src/pio/mod.rs` (1563 lines) — the full PIO abstraction

### The Questions That Drive Learning

When reading any source file, keep asking:
- "What register does this line write to, and what does the datasheet say about it?"
- "Why did they choose this approach over the simpler one?"
- "What would break if I changed this?"
- "Where does the async `.await` yield, and what hardware event wakes it up?"
