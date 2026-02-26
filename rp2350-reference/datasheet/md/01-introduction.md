# RP2350 Datasheet - Chapter 1: Introduction (Tier 2)

Source: `rp2350-reference/datasheet/01-introduction.pdf`

- Printed-page span: 13-23
- Physical PDF-page span in split chapter: 1-11 (source document physical 14-24)
- Conversion method: `pdftotext -layout` + manual table reconstruction
- Loss notes: Figures 1-4 are diagrammatic; text content and table content are preserved below.

## Chapter 1. Introduction

RP2350 is a microcontroller family from Raspberry Pi with major enhancements over RP2040.

Key features:

- Dual Cortex-M33 or Hazard3 processors at 150 MHz
- 520 kB on-chip SRAM in 10 independent banks
- 8 kB one-time-programmable (OTP) storage
- Up to 16 MB external QSPI flash or PSRAM via dedicated QSPI bus
- Additional 16 MB flash/PSRAM via optional second chip-select
- On-chip switched-mode power supply for core voltage
- Optional low-quiescent-current LDO mode for sleep states
- 2 on-chip PLLs for internal or external clock generation
- GPIOs are 5 V-tolerant (powered) and 3.3 V-failsafe (unpowered)
- Security features:
  - Optional boot signing enforced by mask ROM, with key fingerprint in OTP
  - Protected OTP storage for optional boot decryption key
  - Global bus filtering based on Arm or RISC-V security/privilege levels
  - Peripherals, GPIOs, and DMA channels assignable to security domains
  - Hardware mitigations for fault injection attacks
  - Hardware SHA-256 accelerator
- Peripherals:
  - 2 UARTs
  - 2 SPI controllers
  - 2 I2C controllers
  - 24 PWM channels
  - USB 1.1 controller and PHY (host and device support)
  - 12 PIO state machines
  - 1 HSTX peripheral

### Table 1. RP2350 device family

| Product | Package | Internal Flash | GPIO | Analogue Inputs |
|---|---|---|---:|---:|
| RP2350A | QFN-60 | None | 30 | 4 |
| RP2350B | QFN-80 | None | 48 | 8 |
| RP2354A | QFN-60 | 2 MB | 30 | 4 |
| RP2354B | QFN-80 | 2 MB | 48 | 8 |

## 1.1. The chip

Dual Cortex-M33 or Hazard3 processors access RP2350's memory and peripherals via AHB and APB bus fabric.

> **Figure 1** (system overview block diagram) — not reproduced here. See source PDF page 14.

Code may execute directly from external memory through a dedicated QSPI memory interface in the execute-in-place subsystem (XIP). The cache improves XIP performance significantly. Both flash and RAM can attach via this interface.

Debug is available via the SWD interface. This allows an external host to load, run, halt and inspect software running on the system, or configure the execution trace output.

Internal SRAM can contain code or data. It is addressed as a single 520 kB region, but physically partitioned into 10 banks to allow simultaneous parallel access from different managers. All SRAM supports single-cycle access.

A high-bandwidth system DMA offloads repetitive data transfer tasks from the processors.

GPIO pins can be driven directly via single-cycle IO (SIO), or from a variety of dedicated logic functions such as the hardware SPI, I2C, UART and PWM. Programmable IO controllers (PIO) can implement a wider variety of IO functions, or supplement the number of fixed-function peripherals.

A USB controller with embedded PHY provides FS/LS Host or Device connectivity under software control.

Four or eight ADC inputs (depending on package size) are shared with GPIO pins.

Two PLLs provide a fixed 48 MHz clock for USB or ADC, and a flexible system clock up to 150 MHz. A crystal oscillator provides a precise reference for the PLLs.

An internal voltage regulator supplies the core voltage, so you need generally only supply the IO voltage. It operates as a switched mode buck converter when the system is awake, providing up to 200 mA at a variable output voltage, and can switch to a low-quiescent-current LDO mode when the system is asleep, providing up to 1 mA for state retention.

The system features low-power states where unused logic is powered off, supporting wakeup from timer or IO events. The amount of SRAM retained during power-down is configurable.

The internal 8 kB one-time-programmable storage (OTP) contains chip information such as unique identifiers, can be used to configure hardware and bootrom security features, and can be programmed with user-supplied code and data.

The built-in bootrom implements direct boot from flash or OTP, and serial boot from USB or UART. Code signature enforcement is supported for all boot media, using a key fingerprint registered in internal OTP storage. OTP can also store decryption keys for encrypted boot, preventing flash contents from being read externally.

RISC-V architecture support is implemented by dynamically swapping the Cortex-M33 (Armv8-M) processors with Hazard3 (RV32IMAC+) processors. Both architectures are available on all RP2350-family devices. The RISC-V cores support debug over SWD, and can be programmed with the same SDK as the Arm cores.

## 1.2. Pinout reference

This section is a quick reference for pinout and pin functions. Full electrical specifications and package drawings are in Chapter 14.

### 1.2.1. Pin locations

#### 1.2.1.1. QFN-60 (RP2350A)

Figure 2 is the RP2350A QFN-60 top-view pin map.

#### 1.2.1.2. QFN-80 (RP2350B)

Figure 3 is the RP2350B QFN-80 top-view pin map.

### 1.2.2. Pin descriptions

### Table 2. Pin description summary

| Name | Description |
|---|---|
| GPIOx | General-purpose digital input/output. RP2350 can connect internal peripherals to each GPIO, or control GPIO directly from software. |
| GPIOx/ADCy | GPIO with ADC capability. ADC mux can select any of these pins and sample voltage. |
| QSPIx | Interface to SPI/Dual-SPI/Quad-SPI flash or PSRAM with execute-in-place support; can be used as software GPIO if flash access is not required. |
| USB_DM and USB_DP | USB controller pins (FS device and FS/LS host). Requires 27 ohm series resistor on each pin; pullups/pulldowns are internal. Can be software GPIO if USB is not used. |
| XIN and XOUT | Crystal oscillator connection. XIN can also be single-ended CMOS clock input (XOUT disconnected). USB bootloader defaults to 12 MHz crystal/clock input; configurable via OTP. |
| RUN | Global asynchronous reset. Low = reset, high = run. Can be tied directly to IOVDD if external reset not needed. |
| SWCLK and SWDIO | Serial Wire Debug multi-drop bus access; debug for both processors and code download path. |
| GND | External ground connection, bonded to multiple internal die ground pads. |
| IOVDD | Digital GPIO power supply, nominal 1.8 V to 3.3 V. |
| USB_OTP_VDD | Internal USB FS PHY and OTP power supply, nominal 3.3 V. |
| ADC_AVDD | ADC power supply, nominal 3.3 V. |
| QSPI_IOVDD | QSPI IO power supply, nominal 1.8 V to 3.3 V. |
| VREG_AVDD | Analog supply for internal core regulator, nominal 3.3 V. |
| VREG_PGND | Power ground for internal core regulator; tie externally to ground. |
| VREG_LX | Switched-mode output for internal regulator to external inductor; max 200 mA, nominal 1.1 V after filtering. |
| VREG_VIN | Input supply for internal regulator, nominal 2.7 V to 5.5 V. |
| VREG_FB | Voltage feedback for internal regulator; connect to filtered VREG output (for example DVDD if regulator supplies DVDD). |
| DVDD | Digital core supply, nominal 1.1 V; connect externally either to regulator output or external board supply. |

### 1.2.3. GPIO functions (Bank 0)

Each GPIO can connect to internal peripherals via function select. Some connections appear on multiple pins for routing flexibility. SIO, PIO0, PIO1, and PIO2 can connect to all GPIO pins and are software-controlled.

### Table 3. GPIO Bank 0 functions

| GPIO | F0 | F1 | F2 | F3 | F4 | F5 | F6 | F7 | F8 | F9 | F10 | F11 |
|---:|---|---|---|---|---|---|---|---|---|---|---|---|
| 0 | - | SPI0 RX | UART0 TX | I2C0 SDA | PWM0 A | SIO | PIO0 | PIO1 | PIO2 | QMI CS1n | USB OVCUR DET | - |
| 1 | - | SPI0 CSn | UART0 RX | I2C0 SCL | PWM0 B | SIO | PIO0 | PIO1 | PIO2 | TRACECLK | USB VBUS DET | - |
| 2 | - | SPI0 SCK | UART0 CTS | I2C1 SDA | PWM1 A | SIO | PIO0 | PIO1 | PIO2 | TRACEDATA0 | USB VBUS EN | UART0 TX |
| 3 | - | SPI0 TX | UART0 RTS | I2C1 SCL | PWM1 B | SIO | PIO0 | PIO1 | PIO2 | TRACEDATA1 | USB OVCUR DET | UART0 RX |
| 4 | - | SPI0 RX | UART1 TX | I2C0 SDA | PWM2 A | SIO | PIO0 | PIO1 | PIO2 | TRACEDATA2 | USB VBUS DET | - |
| 5 | - | SPI0 CSn | UART1 RX | I2C0 SCL | PWM2 B | SIO | PIO0 | PIO1 | PIO2 | TRACEDATA3 | USB VBUS EN | - |
| 6 | - | SPI0 SCK | UART1 CTS | I2C1 SDA | PWM3 A | SIO | PIO0 | PIO1 | PIO2 | - | USB OVCUR DET | UART1 TX |
| 7 | - | SPI0 TX | UART1 RTS | I2C1 SCL | PWM3 B | SIO | PIO0 | PIO1 | PIO2 | - | USB VBUS DET | UART1 RX |
| 8 | - | SPI1 RX | UART1 TX | I2C0 SDA | PWM4 A | SIO | PIO0 | PIO1 | PIO2 | QMI CS1n | USB VBUS EN | - |
| 9 | - | SPI1 CSn | UART1 RX | I2C0 SCL | PWM4 B | SIO | PIO0 | PIO1 | PIO2 | - | USB OVCUR DET | - |
| 10 | - | SPI1 SCK | UART1 CTS | I2C1 SDA | PWM5 A | SIO | PIO0 | PIO1 | PIO2 | - | USB VBUS DET | UART1 TX |
| 11 | - | SPI1 TX | UART1 RTS | I2C1 SCL | PWM5 B | SIO | PIO0 | PIO1 | PIO2 | - | USB VBUS EN | UART1 RX |
| 12 | HSTX | SPI1 RX | UART0 TX | I2C0 SDA | PWM6 A | SIO | PIO0 | PIO1 | PIO2 | CLOCK GPIN0 | USB OVCUR DET | - |
| 13 | HSTX | SPI1 CSn | UART0 RX | I2C0 SCL | PWM6 B | SIO | PIO0 | PIO1 | PIO2 | CLOCK GPOUT0 | USB VBUS DET | - |
| 14 | HSTX | SPI1 SCK | UART0 CTS | I2C1 SDA | PWM7 A | SIO | PIO0 | PIO1 | PIO2 | CLOCK GPIN1 | USB VBUS EN | UART0 TX |
| 15 | HSTX | SPI1 TX | UART0 RTS | I2C1 SCL | PWM7 B | SIO | PIO0 | PIO1 | PIO2 | CLOCK GPOUT1 | USB OVCUR DET | UART0 RX |
| 16 | HSTX | SPI0 RX | UART0 TX | I2C0 SDA | PWM0 A | SIO | PIO0 | PIO1 | PIO2 | - | USB VBUS DET | - |
| 17 | HSTX | SPI0 CSn | UART0 RX | I2C0 SCL | PWM0 B | SIO | PIO0 | PIO1 | PIO2 | - | USB VBUS EN | - |
| 18 | HSTX | SPI0 SCK | UART0 CTS | I2C1 SDA | PWM1 A | SIO | PIO0 | PIO1 | PIO2 | - | USB OVCUR DET | UART0 TX |
| 19 | HSTX | SPI0 TX | UART0 RTS | I2C1 SCL | PWM1 B | SIO | PIO0 | PIO1 | PIO2 | QMI CS1n | USB VBUS DET | UART0 RX |
| 20 | - | SPI0 RX | UART1 TX | I2C0 SDA | PWM2 A | SIO | PIO0 | PIO1 | PIO2 | CLOCK GPIN0 | USB VBUS EN | - |
| 21 | - | SPI0 CSn | UART1 RX | I2C0 SCL | PWM2 B | SIO | PIO0 | PIO1 | PIO2 | CLOCK GPOUT0 | USB OVCUR DET | - |
| 22 | - | SPI0 SCK | UART1 CTS | I2C1 SDA | PWM3 A | SIO | PIO0 | PIO1 | PIO2 | CLOCK GPIN1 | USB VBUS DET | UART1 TX |
| 23 | - | SPI0 TX | UART1 RTS | I2C1 SCL | PWM3 B | SIO | PIO0 | PIO1 | PIO2 | CLOCK GPOUT1 | USB VBUS EN | UART1 RX |
| 24 | - | SPI1 RX | UART1 TX | I2C0 SDA | PWM4 A | SIO | PIO0 | PIO1 | PIO2 | CLOCK GPOUT2 | USB OVCUR DET | - |
| 25 | - | SPI1 CSn | UART1 RX | I2C0 SCL | PWM4 B | SIO | PIO0 | PIO1 | PIO2 | CLOCK GPOUT3 | USB VBUS DET | - |
| 26 | - | SPI1 SCK | UART1 CTS | I2C1 SDA | PWM5 A | SIO | PIO0 | PIO1 | PIO2 | - | USB VBUS EN | UART1 TX |
| 27 | - | SPI1 TX | UART1 RTS | I2C1 SCL | PWM5 B | SIO | PIO0 | PIO1 | PIO2 | - | USB OVCUR DET | UART1 RX |
| 28 | - | SPI1 RX | UART0 TX | I2C0 SDA | PWM6 A | SIO | PIO0 | PIO1 | PIO2 | - | USB VBUS DET | - |
| 29 | - | SPI1 CSn | UART0 RX | I2C0 SCL | PWM6 B | SIO | PIO0 | PIO1 | PIO2 | - | USB VBUS EN | - |
| 30 | - | SPI1 SCK | UART0 CTS | I2C1 SDA | PWM7 A | SIO | PIO0 | PIO1 | PIO2 | - | USB OVCUR DET | UART0 TX |
| 31 | - | SPI1 TX | UART0 RTS | I2C1 SCL | PWM7 B | SIO | PIO0 | PIO1 | PIO2 | - | USB VBUS DET | UART0 RX |
| 32 | - | SPI0 RX | UART0 TX | I2C0 SDA | PWM8 A | SIO | PIO0 | PIO1 | PIO2 | - | USB VBUS EN | - |
| 33 | - | SPI0 CSn | UART0 RX | I2C0 SCL | PWM8 B | SIO | PIO0 | PIO1 | PIO2 | - | USB OVCUR DET | - |
| 34 | - | SPI0 SCK | UART0 CTS | I2C1 SDA | PWM9 A | SIO | PIO0 | PIO1 | PIO2 | - | USB VBUS DET | UART0 TX |
| 35 | - | SPI0 TX | UART0 RTS | I2C1 SCL | PWM9 B | SIO | PIO0 | PIO1 | PIO2 | - | USB VBUS EN | UART0 RX |
| 36 | - | SPI0 RX | UART1 TX | I2C0 SDA | PWM10 A | SIO | PIO0 | PIO1 | PIO2 | - | USB OVCUR DET | - |
| 37 | - | SPI0 CSn | UART1 RX | I2C0 SCL | PWM10 B | SIO | PIO0 | PIO1 | PIO2 | - | USB VBUS DET | - |
| 38 | - | SPI0 SCK | UART1 CTS | I2C1 SDA | PWM11 A | SIO | PIO0 | PIO1 | PIO2 | - | USB VBUS EN | UART1 TX |
| 39 | - | SPI0 TX | UART1 RTS | I2C1 SCL | PWM11 B | SIO | PIO0 | PIO1 | PIO2 | - | USB OVCUR DET | UART1 RX |
| 40 | - | SPI1 RX | UART1 TX | I2C0 SDA | PWM8 A | SIO | PIO0 | PIO1 | PIO2 | - | USB VBUS DET | - |
| 41 | - | SPI1 CSn | UART1 RX | I2C0 SCL | PWM8 B | SIO | PIO0 | PIO1 | PIO2 | - | USB VBUS EN | - |
| 42 | - | SPI1 SCK | UART1 CTS | I2C1 SDA | PWM9 A | SIO | PIO0 | PIO1 | PIO2 | - | USB OVCUR DET | UART1 TX |
| 43 | - | SPI1 TX | UART1 RTS | I2C1 SCL | PWM9 B | SIO | PIO0 | PIO1 | PIO2 | - | USB VBUS DET | UART1 RX |
| 44 | - | SPI1 RX | UART0 TX | I2C0 SDA | PWM10 A | SIO | PIO0 | PIO1 | PIO2 | - | USB VBUS EN | - |
| 45 | - | SPI1 CSn | UART0 RX | I2C0 SCL | PWM10 B | SIO | PIO0 | PIO1 | PIO2 | - | USB OVCUR DET | - |
| 46 | - | SPI1 SCK | UART0 CTS | I2C1 SDA | PWM11 A | SIO | PIO0 | PIO1 | PIO2 | - | USB VBUS DET | UART0 TX |
| 47 | - | SPI1 TX | UART0 RTS | I2C1 SCL | PWM11 B | SIO | PIO0 | PIO1 | PIO2 | QMI CS1n | USB VBUS EN | UART0 RX |

Notes from Table 3:

- GPIO 0 through 29 are available in all package variants.
- GPIO 30 through 47 are available only in QFN-80 (RP2350B).
- Analogue input is available on GPIO 26 through 29 in QFN-60 (4 inputs total), and on GPIO 40 through 47 in QFN-80 (8 inputs total).

### Table 4. GPIO Bank 0 function descriptions

| Function Name | Description |
|---|---|
| SPIx | Connect one of the internal PL022 SPI peripherals to GPIO. |
| UARTx | Connect one of the internal PL011 UART peripherals to GPIO. |
| I2Cx | Connect one of the internal DW I2C peripherals to GPIO. |
| PWMx A/B | Connect a PWM slice to GPIO. There are twelve PWM slices, each with two output channels (A/B). The B pin can also be used as an input for frequency and duty-cycle measurement. |
| SIO | Software GPIO control from single-cycle IO. F5 must be selected for processor-driven output; input path remains connected so software can always read GPIO state. |
| PIOx | Connect one of the programmable IO blocks (PIO) to GPIO. PIO function (F6/F7/F8) must be selected to drive output; input path remains connected so PIOs can always observe pin state. |
| HSTX | Connect high-speed transmit peripheral to GPIO. |
| CLOCK GPINx | General-purpose clock inputs routable to internal clock domains (for example 1 Hz for AON Timer) or internal frequency counter. |
| CLOCK GPOUTx | General-purpose clock outputs that can drive internal clocks (including PLL outputs) onto GPIO with optional integer divide. |
| TRACECLK, TRACEDATAx | CoreSight TPIU execution-trace output from Cortex-M33 (Arm-only). |
| USB OVCUR DET/VBUS DET/VBUS EN | USB power-control signals to/from internal USB controller. |
| QMI CS1n | Auxiliary chip-select for QSPI bus to support execute-in-place from an additional flash/PSRAM device. |

### 1.2.4. GPIO functions (Bank 1)

GPIO functions are also available on dedicated QSPI pins and USB DP/DM pins. Depending on use case, these can be available for general-purpose use (for example, if not using QSPI execute-in-place and instead booting from OTP or controlling via SWD).

### Table 5. GPIO Bank 1 functions

| Pin | F0 | F1 | F2 | F3 | F4 | F5 | F6 | F7 | F8 | F9 | F10 | F11 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| USB DP | - | - | UART1 TX | I2C0 SDA | - | SIO | - | - | - | - | - | - |
| USB DM | - | - | UART1 RX | I2C0 SCL | - | SIO | - | - | - | - | - | - |
| QSPI SCK | QMI SCK | - | UART1 CTS | I2C1 SDA | - | SIO | - | - | - | - | - | UART1 TX |
| QSPI CSn | QMI CS0n | - | UART1 RTS | I2C1 SCL | - | SIO | - | - | - | - | - | UART1 RX |
| QSPI SD0 | QMI SD0 | - | UART0 TX | I2C0 SDA | - | SIO | - | - | - | - | - | - |
| QSPI SD1 | QMI SD1 | - | UART0 RX | I2C0 SCL | - | SIO | - | - | - | - | - | - |
| QSPI SD2 | QMI SD2 | - | UART0 CTS | I2C1 SDA | - | SIO | - | - | - | - | - | UART0 TX |
| QSPI SD3 | QMI SD3 | - | UART0 RTS | I2C1 SCL | - | SIO | - | - | - | - | - | UART0 RX |

### Table 6. GPIO Bank 1 function descriptions

| Function Name | Description |
|---|---|
| UARTx | Connect one of the internal PL011 UART peripherals to GPIO. |
| I2Cx | Connect one of the internal DW I2C peripherals to GPIO. |
| SIO | Software GPIO control from single-cycle IO. F5 must be selected for processor-driven output; input path remains connected so software can always read GPIO state. |
| QMI | QSPI memory interface peripheral used for execute-in-place from external QSPI flash or PSRAM. |

## 1.3. Why is the chip called RP2350?

Figure 4 in the source is a naming breakdown diagram. The post-fix numeral is described as:

1. Number of processor cores
   - 2 indicates a dual-core system
2. Loosely, processor type
   - 3 indicates Cortex-M33 or Hazard3
3. Internal memory capacity: `floor(log2(RAM / 16 kB))`
   - 5 indicates at least `2 x 16 kB = 512 kB`
   - RP2350 has 520 kB main SRAM
4. Internal storage capacity: `floor(log2(nonvolatile / 128 kB))` (or 0 if no onboard nonvolatile storage)
   - RP2350 uses external flash
   - RP2354 has `2 x 128 kB = 2 MB` internal flash

## 1.4. Version History

Table 7 lists RP2350 versions. For details on changes between versions, refer to Appendix C and Product Change Notification (PCN) 28.

### Table 7. RP2350 version history

| Version | Use |
|---|---|
| A0 | Internal development |
| A1 | Internal development |
| A2 | Initial release |
| A3 | Internal development, samples, and limited production |
| A4 | Production version |
