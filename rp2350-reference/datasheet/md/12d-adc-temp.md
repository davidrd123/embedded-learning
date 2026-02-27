# RP2350 Datasheet - Chapter 12: Peripherals (Tier 2)

Source: `rp2350-reference/datasheet/12d-adc-temp.pdf`

- Printed-page span: 1066-1075
- Physical PDF-page span in split chapter: 1-10 (source document physical 1067-1076)
- Conversion method: `pdftotext -layout` + automated markdown cleanup
- Loss notes: Diagram content is referenced by captions only; complex table layout may be degraded.

## 12.4. ADC and Temperature Sensor

RP2350 has an internal analogue-digital converter (ADC) with the following features:
- SAR ADC (see Section 12.4.3)
- 500 kS/s (using an independent 48 MHz clock)
- 12-bit with 9.2 ENOB (see Section 12.4.4)
- Five or nine input mux:
- Four inputs available on QFN-60 package pins shared with GPIO[29:26]
- Eight inputs available on QFN-80 package pins shared with GPIO[47:40]
- One input dedicated to the internal temperature sensor (see Section 12.4.6)

- Eight element receive sample FIFO
- Interrupt generation
- DMA interface (see Section 12.4.3.5)
Figure 107 shows the arrangement of ADC channels in the QFN-60 package. Figure 108 shows the same for QFN-80.

> **Figure 107.** ADC Connection Diagram for QFN-60. This package features four external ADC inputs (0 through 3), on Bank 0 GPIOs 26 through 29. The internal temperature sensor connects to a fifth channel (channel 4). This is functionally the same ADC arrangement as RP2040, although the underlying hardware is different, to support the additional channels on QFN-80. See source PDF page 1067.

> **Figure 108.** ADC Connection Diagram for QFN-80. This package features eight external ADC inputs (0 through 7), on Bank 0 GPIOs 40 through 47. The internal temperature sensor connects to a ninth channel (channel 8). Like in QFN-60, each ADC input shares a package pin with a digital Bank 0 GPIO; generally the digital functions are disabled when the ADC is in use. See source PDF page 1068.

When using an ADC input shared with a GPIO pin, always disable the pin’s digital functions by setting IE low and OD high
in the pin’s pad control register. See Section 9.11.3, “Pad Control - User Bank” for details.
The maximum ADC input voltage is determined by the digital IO supply voltage (IOVDD), not the ADC supply voltage
(ADC_AVDD). For example, if IOVDD is powered at 1.8 V, the voltage on the ADC inputs should not exceed 1.8 V + 10% even if
ADC_AVDD is powered at 3.3 V. Voltages greater than IOVDD will result in leakage currents through the ESD protection
diodes. See Section 14.9, “Electrical specifications” for details.

### 12.4.1. Changes from RP2040

- Removed spikes in differential nonlinearity at codes 0x200, 0x600, 0xa00 and 0xe00, as documented by erratum
RP2040-E11, improving the ADC’s precision by around 0.5 ENOB.

- Increased the number of external ADC input channels from 4 to 8 channels, in the QFN-80 package only.

### 12.4.2. ADC controller

A digital controller manages the details of operating the RP2350 ADC, and provides additional functionality:
- One-shot or free-running capture mode
- Sample FIFO with DMA interface
- Pacing timer (16 integer bits, 8 fractional bits) for setting free-running sample rate
- Round-robin sampling of multiple channels in free-running capture mode
- Optional right-shift to 8 bits in free-running capture mode, so samples can be DMA’d to a byte buffer in system
memory

#### 12.4.2.1. Channel connections

The ADC channels are connected to the following GPIOs in QFN-60

### Table 1118. ADC channel connections on QFN-60

| Channel | Connection |
|---:|---|
| 0 | GPIO[26] |
| 1 | GPIO[27] |
| 2 | GPIO[28] |
| 3 | GPIO[29] |
| 4 | Temperature Sensor |

The ADC channels are connected to the following GPIOs in QFN-80

### Table 1119. ADC channel connections on QFN-80

| Channel | Connection |
|---:|---|
| 0 | GPIO[40] |
| 1 | GPIO[41] |
| 2 | GPIO[42] |
| 3 | GPIO[43] |
| 4 | GPIO[44] |
| 5 | GPIO[45] |
| 6 | GPIO[46] |
| 7 | GPIO[47] |
| 8 | Temperature Sensor |

### 12.4.3. SAR ADC

The Successive Approximation Register Analogue to Digital Converter (SAR ADC) is a combination of digital controller
and analogue circuit as shown in Figure 109 and Figure 110 .

> **Figure 109.** SAR ADC Block diagram QFN-60. Shows the SAR controller, DAC, comparator, and sample-and-hold circuit with analogue inputs ain_sel<3:0> and control signals (conv_ready, conv_start, conv_done, conv_error). See source PDF page 1070.

> **Figure 110.** SAR ADC Block diagram QFN-80. Same architecture as QFN-60 but with expanded analogue input mux (ain<8:0>). See source PDF page 1070.

The ADC requires a 48 MHz clock (clk_adc), which could come from the USB PLL. Capturing a sample takes 96 clock
cycles (96 × 1/48 MHz) = 2 μs per sample (500 kS/s). The clock must be set up correctly before enabling the ADC.
When the ADC block is provided with a clock, and its reset has been removed, writing a 1 to CS.EN will start a short
internal power-up sequence for the ADC’s analogue hardware. After a few clock cycles, CS.READY will go high,
indicating the ADC is ready to start its first conversion.
To save power, you can disable the ADC at any time by clearing CS.EN. CS.EN does not enable the temperature sensor
bias source; it is controlled separately, see Section 12.4.6 for details.
The ADC input is capacitive. When sampling, the ADC places about 1pF across the input. Packaging, PCB routing, and
other external factors introduce additional capacitance. The effective impedance, even when sampling at 500 kS/s, is
over 100 kΩ. DC measurements have no need to buffer.

#### 12.4.3.1. One-shot sample

To select an ADC input, write to CS.AINSEL:
- On QFN-60, there are 4 external inputs, with an AINSEL value of 0 → 3 mapping to the ADC input on GPIO26 →
GPIO29. Set AINSEL to 4 to select the internal temperature sensor.
- On QFN-80, there are 8 external inputs, with an AINSEL value of 0 → 7 mapping to the ADC input on GPIO40 →
GPIO47. Set AINSEL to 8 to select the internal temperature sensor.
Switching AINSEL requires no settling time.
Write a 1 to CS.START_ONCE to immediately start a new conversion. CS.READY will go low to show that a conversion is
currently in progress. After 96 cycles of clk_adc, CS.READY will go high. The 12-bit conversion result is available in
RESULT.

#### 12.4.3.2. Free-running sampling

When CS.START_MANY is set, the ADC automatically starts new conversions at regular intervals. The most recent
conversion result is always available in RESULT, but for IRQ or DMA-driven streaming of samples, you must enable the
ADC FIFO (Section 12.4.3.4).
By default (DIV = 0), new conversions start immediately after the previous conversion finishes, producing a new sample
every 96 cycles. At a clock frequency of 48 MHz, this produces 500 kS/s.
Set DIV.INT to a positive value n to trigger the ADC once per n + 1 cycles. The ADC ignores this if a conversion is
currently in progress, so generally n will be ≥ 96. For example, setting DIV.INT to 47999 runs the ADC at 1 kS/s, if running
from a 48 MHz clock.
The pacing timer supports fractional-rate division (first order delta sigma). When setting DIV.FRAC to a non-zero value,
the ADC starts a new conversion once per `1 + INT + FRAC/256` cycles on average, by changing the sample interval
between `INT + 1` and `INT + 2`.

#### 12.4.3.3. Sampling multiple inputs

CS.RROBIN allows the ADC to sample multiple inputs in an interleaved fashion while performing free-running sampling.
Each bit in RROBIN corresponds to one of the five possible values of CS.AINSEL. When the ADC completes a conversion,
CS.AINSEL automatically cycles to the next input whose corresponding bit is set in RROBIN.
To disable the round-robin sampling feature, write all-zeroes to CS.RROBIN.
For example, if AINSEL is initially 0, and RROBIN is set to 0x06 (bits 1 and 2 are set), the ADC samples channels in the
following order:
1. Channel 0
2. Channel 1
3. Channel 2
4. Channel 1
5. Channel 2
6. Channel 1
7. Channel 2
The ADC continues to sample channels 1 and 2 indefinitely.
> **NOTE**
The initial value of AINSEL does not need to correspond with a set bit in RROBIN.

#### 12.4.3.4. Sample FIFO

You can read ADC samples directly from the RESULT register or store them in a local 8-entry FIFO and read out from
FIFO. Use the FCS register to control FIFO operation.
When FCS.EN is set, the ADC writes each conversion result to the FIFO. A software interrupt handler or the RP2350 DMA
can read this sample from the FIFO when notified by the ADC’s IRQ or DREQ signals. Alternatively, software can poll the
status bits in FCS to wait for each sample to become available.
If the FIFO is full when a conversion completes, the sticky error flag FCS.OVER is set. When the FIFO is full, the current
FIFO contents do not change, so any conversions that complete during this time are lost.
Two flags control the data written to the FIFO by the ADC:

- FCS.SHIFT right-shifts the FIFO data to eight bits in size (i.e. FIFO bits 7:0 are conversion result bits 11:4). This is
suitable for 8-bit DMA transfer to a byte buffer in memory, allowing deeper capture buffers, at the cost of some
precision.
- FCS.ERR sets the FIFO.ERR flag of each FIFO value, showing that a conversion error took place, i.e. the SAR failed
to converge.
Conversion errors indicate that the comparison of one or more bits failed to complete in the time allowed. Conversion
errors are typically caused by comparator metastability: the closer to the comparator threshold the input signal is, the
longer it takes to make a decision. The higher the comparator gain, the lower the probability of conversion errors.
> **CAUTION**
Because conversion errors produce undefined results, you should always discard samples that contain conversion
errors.

#### 12.4.3.5. DMA

The RP2350 DMA (Section 12.6) can fetch ADC samples from the sample FIFO, by performing a normal memory-
mapped read on the FIFO register, paced by the ADC_DREQ system data request signal. Before you can use the DMA to
fetch ADC samples, you must:
- Enable the sample FIFO (FCS.EN) so that samples are written to it; the FIFO is disabled by default so that it does
not inadvertently fill when the ADC is used for one-shot conversions. Configure the ADC sample rate (Section
12.4.3.2) before starting the ADC.
- Enable the ADC’s data request handshake (DREQ) via FCS.DREQ_EN.
- In the DMA channel used for the transfer, select the DREQ_ADC data request signal (Section 12.6.4.1).
- Set the threshold for DREQ assertion (FCS.THRESH) to 1, so that the DMA transfers as soon as a single sample is
present in the FIFO. This is also the threshold used for IRQ assertion, so non-DMA use cases might prefer a higher
value for less frequent interrupts.
- If the DMA transfer size is set to 8 bits (so that the DMA transfers to a byte array in memory), set FCS.SHIFT to pre-
shift the FIFO samples to 8 bits of significance.
- To sample multiple input channels, write a mask of those channels to CS.RROBIN. Additionally, select the first
channel to sample with CS.AINSEL.
Once the ADC is suitably configured, start the DMA channel first, then the ADC conversion via CS.START_MANY. Once
the DMA completes, you can halt the ADC if you are finished sampling, or promptly start a new DMA transfer before the
FIFO fills up. After clearing CS.START_MANY to halt the ADC, software should poll CS.READY to make sure the last
conversion has finished, then drain any stray samples from the FIFO.

#### 12.4.3.6. Interrupts

Use INTE to generate an interrupt when the FIFO level reaches a threshold defined in FCS.THRESH.
Use INTS to read the interrupt status. To clear the interrupt, drain the FIFO to a level lower than FCS.THRESH.

#### 12.4.3.7. Supply

RP2350 separates the ADC supply out on its own pin to allow noise filtering.

### 12.4.4. ADC ENOB

ADC ENOB details are shown in Table 1438.

### 12.4.5. INL and DNL

Details to follow.

### 12.4.6. Temperature sensor

The temperature sensor measures the Vbe voltage of a biased bipolar diode, connected to the fifth ADC channel (
AINSEL=4) on QFN-60 or the ninth ADC channel (AINSEL=8) on QFN-80. Typically, Vbe = 0.706 V at 27 °C, with a slope of
-1.721 mV per degree. Therefore the temperature in °C can be approximated as follows:

`T = 27 - (ADC_voltage - 0.706) / 0.001721`

As the Vbe and the Vbe slope can vary over the temperature range, and from device to device, some user calibration may
be required if accurate measurements are required.
The temperature sensor’s bias source must be enabled before use, via CS.TS_EN. This increases current consumption
on ADC_AVDD by approximately 40 μA.
> **NOTE**
The on board temperature sensor is very sensitive to errors in reference voltage. At 3.3 V, a value of 891 returned by
the ADC corresponds to a temperature of 20.1°C. At a reference voltage 1% lower than 3.3 V, the same reading of
891 corresponds to a temperature of 24.3°C: a temperature change of over 4°C. To improve the accuracy of the
internal temperature sensor, consider adding an external reference voltage.

### 12.4.7. List of registers

The ADC registers start at a base address of 0x400a0000 (defined as ADC_BASE in SDK).
*Table 1120. List of ADC registers*

| Offset | Name   | Info |
|--------|--------|------|
| 0x00   | CS     | ADC Control and Status |
| 0x04   | RESULT | Result of most recent ADC conversion |
| 0x08   | FCS    | FIFO control and status |
| 0x0c   | FIFO   | Conversion result FIFO |
| 0x10   | DIV    | Clock divider. If non-zero, CS_START_MANY will start conversions at regular intervals rather than back-to-back. The divider is reset when either of these fields are written. Total period is 1 + INT + FRAC / 256 |
| 0x14   | INTR   | Raw Interrupts |
| 0x18   | INTE   | Interrupt Enable |
| 0x1c   | INTF   | Interrupt Force |
| 0x20   | INTS   | Interrupt status after masking & forcing |

ADC: CS Register
Offset: 0x00
Description
ADC Control and Status

*Table 1121. CS Register*

| Bits | Description | Type | Reset |
|---|---|---|---|
| 31:25 | Reserved. | - | - |
| 24:16 | RROBIN: Round-robin sampling. 1 bit per channel. Set all bits to 0 to disable. Otherwise, the ADC will cycle through each enabled channel in a round-robin fashion. The first channel to be sampled will be the one currently indicated by AINSEL. AINSEL will be updated after each conversion with the newly-selected channel. | RW | 0x000 |
| 15:12 | AINSEL: Select analog mux input. Updated automatically in round-robin mode. This is corrected for the package option so only ADC channels which are bonded are available, and in the correct order. | RW | 0x0 |
| 11 | Reserved. | - | - |
| 10 | ERR_STICKY: Some past ADC conversion encountered an error. Write 1 to clear. | WC | 0x0 |
| 9 | ERR: The most recent ADC conversion encountered an error; result is undefined or noisy. | RO | 0x0 |
| 8 | READY: 1 if the ADC is ready to start a new conversion. Implies any previous conversion has completed. 0 whilst conversion in progress. | RO | 0x0 |
| 7:4 | Reserved. | - | - |
| 3 | START_MANY: Continuously perform conversions whilst this bit is 1. A new conversion will start immediately after the previous finishes. | RW | 0x0 |
| 2 | START_ONCE: Start a single conversion. Self-clearing. Ignored if `start_many` is asserted. | SC | 0x0 |
| 1 | TS_EN: Power on temperature sensor. 1 - enabled. 0 - disabled. | RW | 0x0 |
| 0 | EN: Power on ADC and enable its clock. 1 - enabled. 0 - disabled. | RW | 0x0 |

ADC: RESULT Register
Offset: 0x04

*Table 1122. RESULT Register*

| Bits | Description | Type | Reset |
|---|---|---|---|
| 31:12 | Reserved. | - | - |
| 11:0 | Result of most recent ADC conversion | RO | 0x000 |

ADC: FCS Register
Offset: 0x08
Description
FIFO control and status

*Table 1123. FCS Register*

| Bits | Description | Type | Reset |
|---|---|---|---|
| 31:28 | Reserved. | - | - |
| 27:24 | THRESH: DREQ/IRQ asserted when level >= threshold | RW | 0x0 |
| 23:20 | Reserved. | - | - |
| 19:16 | LEVEL: The number of conversion results currently waiting in the FIFO | RO | 0x0 |
| 15:12 | Reserved. | - | - |
| 11 | OVER: 1 if the FIFO has been overflowed. Write 1 to clear. | WC | 0x0 |
| 10 | UNDER: 1 if the FIFO has been underflowed. Write 1 to clear. | WC | 0x0 |
| 9 | FULL | RO | 0x0 |
| 8 | EMPTY | RO | 0x0 |
| 7:4 | Reserved. | - | - |
| 3 | DREQ_EN: If 1: assert DMA requests when FIFO contains data | RW | 0x0 |
| 2 | ERR: If 1: conversion error bit appears in the FIFO alongside the result | RW | 0x0 |
| 1 | SHIFT: If 1: FIFO results are right-shifted to be one byte in size. Enables DMA to byte buffers. | RW | 0x0 |
| 0 | EN: If 1: write result to the FIFO after each conversion. | RW | 0x0 |

ADC: FIFO Register
Offset: 0x0c
Description
Conversion result FIFO

*Table 1124. FIFO Register*

| Bits | Description | Type | Reset |
|---|---|---|---|
| 31:16 | Reserved. | - | - |
| 15 | ERR: 1 if this particular sample experienced a conversion error. Remains in the same location if the sample is shifted. | RF | - |
| 14:12 | Reserved. | - | - |
| 11:0 | VAL | RF | - |

ADC: DIV Register
Offset: 0x10
Description
Clock divider. If non-zero, CS_START_MANY will start conversions
at regular intervals rather than back-to-back.
The divider is reset when either of these fields are written.
Total period is 1 + INT + FRAC / 256

*Table 1125. DIV Register*

| Bits | Description | Type | Reset |
|---|---|---|---|
| 31:24 | Reserved. | - | - |
| 23:8 | INT: Integer part of clock divisor. | RW | 0x0000 |
| 7:0 | FRAC: Fractional part of clock divisor. First-order delta-sigma. | RW | 0x00 |
