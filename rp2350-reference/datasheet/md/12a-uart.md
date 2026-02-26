# RP2350 Datasheet - Chapter 12: Peripherals (Tier 2)

Source: `rp2350-reference/datasheet/12a-uart.pdf`

- Printed-page span: 961-982
- Physical PDF-page span in split chapter: 1-22 (source document physical 962-983)
- Conversion method: `pdftotext -layout` + automated markdown cleanup
- Loss notes: Diagram content is referenced by captions only; complex table layout may be degraded.

## Chapter 12. Peripherals

## 12.1. UART

Arm documentation
Excerpted from the PrimeCell UART (PL011) Technical Reference Manual. Used with permission.
RP2350 has 2 identical instances of a UART peripheral, based on the Arm Primecell UART (PL011) (Revision r1p5).
Each instance supports the following features:
- Separate 32×8 TX and 32×12 RX FIFOs
- Programmable baud rate generator, clocked by clk_peri (see Figure 33)
- Standard asynchronous communication bits (start, stop, parity) added on transmit and removed on receive
- Line break detection
- Programmable serial interface (5, 6, 7, or 8 bits)
- 1 or 2 stop bits
- Programmable hardware flow control
Each UART can be connected to a number of GPIO pins as defined in the GPIO muxing table in Section 9.4. Connections
to the GPIO muxing use a prefix including the UART instance name uart0_ or uart1_, and include the following:
- Transmit data tx (referred to as UARTTXD in the following sections)
- Received data rx (referred to as UARTRXD in the following sections)
- Output flow control rts (referred to as nUARTRTS in the following sections)
- Input flow control cts (referred to as nUARTCTS in the following sections)
The modem mode and IrDA mode of the PL011 are not supported.
The UARTCLK is driven from clk_peri, and PCLK is driven from the system clock clk_sys (see Figure 33).

### 12.1.1. Overview

The UART performs:
- Serial-to-parallel conversion on data received from a peripheral device
- Parallel-to-serial conversion on data transmitted to the peripheral device
The CPU reads and writes data and control/status information through the AMBA APB interface. The transmit and
receive paths are buffered with internal FIFO memories that store up to 32 bytes independently in both transmit and
receive modes.
The UART:
- Includes a programmable baud rate generator that generates a common transmit and receive internal clock from
the UART internal reference clock input, UARTCLK
- Offers similar functionality to the industry-standard 16C650 UART device
- Supports a maximum baud rate of UARTCLK / 16 in UART mode (7.8 Mbaud at 125MHz)

The UART operation and baud rate values are controlled by the Line Control Register (UARTLCR_H) and the baud rate
divisor registers: Integer Baud Rate Register (UARTIBRD), and Fractional Baud Rate Register (UARTFBRD).
The UART can generate:
- Individually maskable interrupts from the receive (including timeout), transmit, modem status and error conditions
- A single combined interrupt so that the output is asserted if any of the individual interrupts are asserted and
unmasked
- DMA request signals for interfacing with a Direct Memory Access (DMA) controller
If a framing, parity, or break error occurs during reception, the appropriate error bit is set and stored in the FIFO. If an
overrun condition occurs, the overrun register bit is set immediately and FIFO data is prevented from being overwritten.
You can program the FIFOs to be 1-byte deep providing a conventional double-buffered UART interface.
There is a programmable hardware flow control feature that uses the nUARTCTS input and the nUARTRTS output to
automatically control the serial data flow.

### 12.1.2. Functional description

> **Figure 63.** UART block diagram. Test logic is not shown for clarity. See source PDF page 962.

#### 12.1.2.1. AMBA APB interface

The AMBA APB interface generates read and write decodes for accesses to status/control registers, and the transmit
and receive FIFOs.

#### 12.1.2.2. Register block

The register block stores data written, or to be read across the AMBA APB interface.

#### 12.1.2.3. Baud rate generator

The baud rate generator contains free-running counters that generate the internal clocks: Baud16 and IrLPBaud16
signals. Baud16 provides timing information for UART transmit and receive control. Baud16 is a stream of pulses with a
width of one UARTCLK clock period and a frequency of 16 times the baud rate.

#### 12.1.2.4. Transmit FIFO

The transmit FIFO is an 8-bit wide, 32 location deep, FIFO memory buffer. CPU data written across the APB interface is
stored in the FIFO until read out by the transmit logic. When disabled, the transmit FIFO acts like a one byte holding
register.

#### 12.1.2.5. Receive FIFO

The receive FIFO is a 12-bit wide, 32 location deep, FIFO memory buffer. Received data and corresponding error bits are
stored in the receive FIFO by the receive logic until read out by the CPU across the APB interface. When disabled, the
receive FIFO acts like a one byte holding register.

#### 12.1.2.6. Transmit logic

The transmit logic performs parallel-to-serial conversion on the data read from the transmit FIFO. Control logic outputs
the serial bit stream in the following order:
1. Start bit
2. Data bits (Least Significant Bit (LSB) first)
3. Parity bit
4. Stop bits according to the programmed configuration in control registers

#### 12.1.2.7. Receive logic

The receive logic performs serial-to-parallel conversion on the received bit stream after a valid start pulse has been
detected. Receive logic includes overrun, parity, frame error checking, and line break detection; you can find the output
of these checks in the status that accompanies the data written to the receive FIFO.

#### 12.1.2.8. Interrupt generation logic

The UART generates individual maskable active HIGH interrupts to the processor interrupt controllers. To generate
combined interrupts, the UART outputs an OR function of the individual interrupt requests.
For more information, see Section 12.1.6.

#### 12.1.2.9. DMA interface

The UART provides an interface to connect to the DMA controller as a UART DMA; for more information, see Section
12.1.5.

#### 12.1.2.10. Synchronizing registers and logic

The UART supports both asynchronous and synchronous operation of the clocks, PCLK and UARTCLK. The UART
implements always-on synchronisation registers and handshaking logic. This has a minimal impact on performance and
area. The UART performs control signal synchronisation on both directions of data flow (from the PCLK to the UARTCLK
domain, and from the UARTCLK to the PCLK domain).

### 12.1.3. Operation

#### 12.1.3.1. Clock signals

The frequency selected for UARTCLK must accommodate the required range of baud rates:
- FUARTCLK (min) ≥ 16 × baud_rate (max)
- FUARTCLK (max) ≤ 16 × 65535 × baud_rate (min)
For example, for a range of baud rates from 110 baud to 460800 baud the UARTCLK frequency must be between
7.3728MHz to 115.34MHz.
To use all baud rates, the UARTCLK frequency must fall within the required error limits.
There is also a constraint on the ratio of clock frequencies for PCLK to UARTCLK. The frequency of UARTCLK must be no more
than 5/3 times faster than the frequency of PCLK:
- FUARTCLK ≤ 5/3 × FPCLK
For example, in UART mode, to generate 921600 baud when UARTCLK is 14.7456MHz, PCLK must be greater than or equal
to 8.85276MHz. This ensures that the UART has sufficient time to write the received data to the receive FIFO.

#### 12.1.3.2. UART operation

Control data is written to the UART Line Control Register, UARTLCR. This register is 30 bits wide internally, but provides
external access through the APB interface by writes to the following registers:
- UARTLCR_H, which defines the following:
- transmission parameters
- word length
- buffer mode
- number of transmitted stop bits
- parity mode
- break generation
- UARTIBRD, which defines the integer baud rate divider
- UARTFBRD, which defines the fractional baud rate divider

#### 12.1.3.2.1. Fractional baud rate divider

The baud rate divisor is a 22-bit number consisting of a 16-bit integer and a 6-bit fractional part. The baud rate generator
uses the baud rate divisor to determine the bit period. The fractional baud rate divider enables the use of any clock with
a frequency greater than 3.6864MHz to act as UARTCLK, while it is still possible to generate all the standard baud rates.
The 16-bit integer is written to the Integer Baud Rate Register, UARTIBRD. The 6-bit fractional part is written to the
Fractional Baud Rate Register, UARTFBRD. The Baud Rate Divisor has the following relationship to UARTCLK:

Baud Rate Divisor = UARTCLK/(16×Baud Rate) =                            where          is the integer part and           is the
fractional part separated by a decimal point as shown in Figure 64.
> **Figure 64.** Baud rate divisor. Shows the 16-bit integer and 6-bit fractional part fields. See source PDF page 965.

To calculate the 6-bit number (    ), multiply the fractional part of the required baud rate divisor by 64 (   , where   is the
width of the UARTFBRD register) and add 0.5 to account for rounding errors:
The UART generates an internal clock enable signal, Baud16. This is a stream of UARTCLK-wide pulses with an average
frequency of 16 times the required baud rate. Divide this signal by 16 to give the transmit clock. A low number in the
baud rate divisor produces a short bit period, and a high number in the baud rate divisor produces a long bit period.

#### 12.1.3.2.2. Data transmission or reception

The UART uses two 32-byte FIFOs to store data received and transmitted. The receive FIFO has an extra four bits per
character for status information. For transmission, data is written into the transmit FIFO. If the UART is enabled, it
causes a data frame to start transmitting with the parameters indicated in the Line Control Register, UARTLCR_H. Data
continues to be transmitted until there is no data left in the transmit FIFO. The BUSY signal goes HIGH immediately after
data writes to the transmit FIFO (that is, the FIFO is non-empty) and remains asserted HIGH while data transmits. BUSY
is negated only when the transmit FIFO is empty, and the last character has been transmitted from the shift register,
including the stop bits. BUSY can be asserted HIGH even though the UART might no longer be enabled.
For each sample of data, three readings are taken and the majority value is kept. In the following paragraphs, the middle
sampling point is defined, and one sample is taken either side of it.
When the receiver is idle (UARTRXD continuously 1, in the marking state) and a LOW is detected on the data input (a start
bit has been received), the receive counter, with the clock enabled by Baud16, begins running and data is sampled on
the eighth cycle of that counter in UART mode, or the fourth cycle of the counter in SIR mode to allow for the shorter
logic 0 pulses (half way through a bit period).
The start bit is valid if UARTRXD is still LOW on the eighth cycle of Baud16, otherwise a false start bit is detected and it is
ignored.
If the start bit was valid, successive data bits are sampled on every 16th cycle of Baud16 (that is, one bit period later)
according to the programmed length of the data characters. The parity bit is then checked if parity mode was enabled.
Lastly, a valid stop bit is confirmed if UARTRXD is HIGH, otherwise a framing error has occurred. When a full word is
received, the data is stored in the receive FIFO, with any error bits associated with that word

#### 12.1.3.2.3. Error bits

The receive FIFO stores three error bits in bits 8 (framing), 9 (parity), and 10 (break), each associated with a particular
character. An additional error bit, stored in bit 11 of the receive FIFO, indicates an overrun error.

#### 12.1.3.2.4. Overrun bit

The overrun bit is not associated with the character in the receive FIFO. The overrun error is set when the FIFO is full and
the next character is completely received in the shift register. The data in the shift register is overwritten, but it is not
written into the FIFO. When an empty location becomes available in the FIFO, another character is received and the state
of the overrun bit is copied into the receive FIFO along with the received character. The overrun state is then cleared.
Table 1025 lists the bit functions of the receive FIFO.

Table 1025. Receive
FIFO bit                                                                                  Function
FIFO bit functions
11                                                                                    Overrun indicator
10                                                                                       Break error
9                                                                                        Parity error
8                                                                                      Framing error
7:0                                                                                    Received data

#### 12.1.3.2.5. Disabling the FIFOs

The bottom entry of the transmit and receive sides of the UART both have the equivalent of a 1-byte holding register.
You can manipulate flags to disable the FIFOs, allowing you to use the bottom entry of the FIFOs as a 1-byte register.
However, this doesn’t physically disable the FIFOs. When using the FIFOs as a 1-byte register, a write to the data register
bypasses the holding register unless the transmit shift register is already in use.

#### 12.1.3.2.6. System and diagnostic loopback testing

To perform loopback testing for UART data, set the Loop Back Enable (LBE) bit to 1 in the Control Register, UARTCR.
Data transmitted on UARTTXD is received on the UARTRXD input.

#### 12.1.3.3. UART character frame

> **Figure 65.** UART character frame. Shows UARTTXD signal with start bit, 5-8 data bits (LSB first), optional parity bit, and 1-2 stop bits. See source PDF page 966.

### 12.1.4. UART hardware flow control

The fully-selectable hardware flow control feature enables you to control the serial data flow with the nUARTRTS output
and nUARTCTS input signals. Figure 66 shows how to communicate between two devices using hardware flow control:
> **Figure 66.** Hardware flow control between two similar devices. Shows UART1 and UART2 with crossed nUARTRTS/nUARTCTS signals between Rx/Tx FIFOs. See source PDF page 966.

When the RTS flow control is enabled, nUARTRTS is asserted until the receive FIFO is filled up to the programmed
watermark level. When the CTS flow control is enabled, the transmitter can only transmit data when nUARTCTS is asserted.
The hardware flow control is selectable using the RTSEn and CTSEn bits in the Control Register, UARTCR. Table 1026 shows
how to configure UARTCR register bits to enable RTS and/or CTS.

Table 1026. Control
UARTCR register bits
bits to enable and
disable hardware flow
CTSEn                                     RTSEn                                     Description
control.
1                                         1                                         Both RTS and CTS flow control
enabled
1                                         0                                         Only CTS flow control enabled
0                                         1                                         Only RTS flow control enabled
0                                         0                                         Both RTS and CTS flow control
disabled
> **NOTE**
When RTS flow control is enabled, the software cannot use the RTSEn bit in the Control Register (UARTCR) to control
the status of nUARTRTS.

#### 12.1.4.1. RTS flow control

The RTS flow control logic is linked to the programmable receive FIFO watermark levels.
When RTS flow control is disabled, the receive FIFO receives data until full, or no more data is transmitted to it.
When RTS flow control is enabled, the nUARTRTS is asserted until the receive FIFO fills up to the watermark level. When the
receive FIFO reaches the watermark level, the nUARTRTS signal is de-asserted. This indicates that the FIFO has no more
room to receive data. The transmission of data is expected to cease after the current character has been transmitted.
When the receive FIFO drains below the watermark level, the nUARTRTS signal is reasserted.

#### 12.1.4.2. CTS flow control

The CTS flow control logic is linked to the nUARTCTS signal.
When CTS flow control is disabled, the transmitter transmits data until the transmit FIFO is empty.
When CTS flow control is enabled, the transmitter checks the nUARTCTS signal before transmitting each byte. It only
transmits the byte if the nUARTCTS signal is asserted. As long as the transmit FIFO is not empty and nUARTCTS is asserted,
data continues to transmit. If the transmit FIFO is empty and the nUARTCTS signal is asserted, no data is transmitted. If the
nUARTCTS signal is de-asserted during transmission, the transmitter finishes transmitting the current character before
stopping.

### 12.1.5. UART DMA interface

The UART provides an interface to connect to a DMA controller. The DMA operation of the UART is controlled using the
DMA Control Register, UARTDMACR. The DMA interface includes the following signals:
For receive:
UARTRXDMASREQ
Single character DMA transfer request, asserted by the UART. For receive, one character consists of up to 12 bits.
This signal is asserted when the receive FIFO contains at least one character.
UARTRXDMABREQ
Burst DMA transfer request, asserted by the UART. This signal is asserted when the receive FIFO contains more
characters than the programmed watermark level. You can program the watermark level for each FIFO using the
Interrupt FIFO Level Select Register (UARTIFLS).

UARTRXDMACLR
DMA request clear, asserted by a DMA controller to clear the receive request signals. If DMA burst transfer is
requested, the clear signal is asserted during the transfer of the last data in the burst.
For transmit:
UARTTXDMASREQ
Single character DMA transfer request, asserted by the UART. For transmit, one character consists of up to eight
bits. This signal is asserted when there is at least one empty location in the transmit FIFO.
UARTTXDMABREQ
Burst DMA transfer request, asserted by the UART. This signal is asserted when the transmit FIFO contains less
characters than the watermark level. You can program the watermark level for each FIFO using the Interrupt FIFO
Level Select Register (UARTIFLS).
UARTTXDMACLR
DMA request clear, asserted by a DMA controller to clear the transmit request signals. If DMA burst transfer is
requested, the clear signal is asserted during the transfer of the last data in the burst.
The burst transfer and single transfer request signals are not mutually exclusive: they can both be asserted at the same
time. When the receive FIFO exceeds the watermark level, the burst transfer request and the single transfer request
signals are both asserted. When the receive FIFO is below than the watermark level, only the single transfer request
signal is asserted. This is useful in situations where the number of characters left to be received in the stream is less
than a burst.
Consider a scenario where the watermark level is set to four, but 19 characters are left to be received. The DMA
controller then transfers four bursts of four characters and three single transfers to complete the stream.
> **NOTE**
For the remaining three characters, the UART cannot assert the burst request.
Each request signal remains asserted until the relevant DMACLR signal is asserted. After the request clear signal is de-
asserted, a request signal can become active again, depending on the conditions described previously. All request
signals are de-asserted if the UART is disabled or the relevant DMA enable bit, TXDMAE or RXDMAE, in the DMA Control
Register, UARTDMACR, is cleared.
If you disable the FIFOs in the UART, it operates in character mode. Character mode limits FIFO transfers to a single
character at a time, so only the DMA single transfer mode can operate. In character mode, only the UARTRXDMASREQ and
UARTTXDMASREQ request signals can be asserted. For information about disabling the FIFOs, see the Line Control Register,
UARTLCR_H.
When the UART is in the FIFO enabled mode, data transfers can use either single or burst transfers depending on the
programmed watermark level and the amount of data in the FIFO. Table 1027 lists the trigger points for UARTRXDMABREQ
and UARTTXDMABREQ, depending on the watermark level, for the transmit and receive FIFOs.
Table 1027. DMA
Watermark level                           Burst length
trigger points for the
transmit and receive
Transmit (number of empty                  Receive (number of filled locations)
FIFOs.
locations)
1/8                                       28                                         4
1/4                                       24                                         8
1/2                                       16                                         16
3/4                                       8                                          24
7/8                                       4                                          28
In addition, the DMAONERR bit in the DMA Control Register, UARTDMACR, supports the use of the receive error interrupt,

UARTEINTR. It enables the DMA receive request outputs, UARTRXDMASREQ or UARTRXDMABREQ, to be masked out when the UART
error interrupt, UARTEINTR, is asserted. The DMA receive request outputs remain inactive until the UARTEINTR is cleared. The
DMA transmit request outputs are unaffected.
> **Figure 67.** DMA transfer waveforms. Shows PCLK, DMASREQ, DMABREQ, and DMACLR timing for single and burst transfer requests. See source PDF page 969.

Figure 67 shows the timing diagram for both a single transfer request and a burst transfer request with the appropriate
DMACLR signal. The signals are all synchronous to PCLK. For the sake of clarity it is assumed that there is no
synchronization of the request signals in the DMA controller.

### 12.1.6. Interrupts

There are eleven maskable interrupts generated in the UART. On RP2350, only the combined interrupt output, UARTINTR, is
connected.
To enable or disable individual interrupts, change the mask bits in the Interrupt Mask Set/Clear Register, UARTIMSC. Set
the appropriate mask bit HIGH to enable the interrupt.
The transmit and receive dataflow interrupts UARTRXINTR and UARTTXINTR have been separated from the status interrupts.
This enables you to use UARTRXINTR and UARTTXINTR to read or write data in response to FIFO trigger levels.
The error interrupt, UARTEINTR, can be triggered when there is an error in the reception of data. A number of error
conditions are possible.
The modem status interrupt, UARTMSINTR, is a combined interrupt of all the individual modem status signals.
The status of the individual interrupt sources can be read either from the Raw Interrupt Status Register, UARTRIS, or from
the Masked Interrupt Status Register, UARTMIS.

#### 12.1.6.1. UARTMSINTR

The modem status interrupt is asserted if any of the modem status signals (nUARTCTS, nUARTDCD, nUARTDSR, and nUARTRI)
change. To clear the modem status interrupt, write a 1 to the bits corresponding to the modem status signals that
generated the interrupt in the Interrupt Clear Register (UARTICR).

#### 12.1.6.2. UARTRXINTR

The receive interrupt changes state when one of the following events occurs:
- The FIFOs are enabled and the receive FIFO reaches the programmed trigger level. This asserts the receive
interrupt HIGH. To clear the receive interrupt, read data from the receive FIFO until it drops below the trigger level.
- The FIFOs are disabled (have a depth of one location) and data is received, thereby filling the receive FIFO. This
asserts the receive interrupt HIGH. To clear the receive interrupt, perform a single read from the receive FIFO.
In both cases, you can also clear the interrupt manually.

#### 12.1.6.3. UARTTXINTR

The transmit interrupt changes state when one of the following events occurs:
- The FIFOs are enabled and the transmit FIFO is equal to or lower than the programmed trigger level. This asserts
the transmit interrupt HIGH. To clear the transmit interrupt, write data to the transmit FIFO until it exceeds the

trigger level.
- The FIFOs are disabled (have a depth of one location) and there is no data present in the transmit FIFO. This
asserts the transmit interrupt HIGH. To clear the transmit interrupt, perform a single write to the transmit FIFO.
In both cases, you can also clear the interrupt manually.
To update the transmit FIFO, write data to the transmit FIFO before or after enabling the UART and the interrupts.
> **NOTE**
The transmit interrupt is based on a transition through a level, rather than on the level itself. When the interrupt and
the UART is enabled before any data is written to the transmit FIFO, the interrupt is not set. The interrupt is only set
after written data leaves the single location of the transmit FIFO and it becomes empty.

#### 12.1.6.4. UARTRTINTR

The receive timeout interrupt is asserted when the receive FIFO is not empty and no more data is received during a 32-
bit period.
The receive timeout interrupt is cleared in the following scenarios:
- the FIFO becomes empty through reading all the data or by reading the holding register
- a 1 is written to the corresponding bit of the Interrupt Clear Register, UARTICR

#### 12.1.6.5. UARTEINTR

The error interrupt is asserted when an error occurs in the reception of data by the UART. The interrupt can be caused
by a number of different error conditions:
- framing
- parity
- break
- overrun
To determine the cause of the interrupt, read the Raw Interrupt Status Register (UARTRIS) or the Masked Interrupt Status
Register (UARTMIS). To clear the interrupt, write to the relevant bits of the Interrupt Clear Register, UARTICR (bits 7 to 10 are
the error clear bits).

#### 12.1.6.6. UARTINTR

The interrupts are also combined into a single output, that is an OR function of the individual masked sources. You can
connect this output to a system interrupt controller to provide another level of masking on a individual peripheral basis.
The combined UART interrupt is asserted if any of the individual interrupts are asserted and enabled.

### 12.1.7. Programmer’s model

The SDK provides a uart_init function to configure the UART with a particular baud rate. Once the UART is initialised,
the user must configure a GPIO pin as UART_TX and UART_RX. See Section 9.10.1 for more information on selecting a GPIO
function.
To initialise the UART, the uart_init function takes the following steps:
1. De-asserts the reset

2. Enables clk_peri
3. Sets enable bits in the control register
4. Enables the FIFOs
5. Sets the baud rate divisors
6. Sets the format
SDK: https://github.com/raspberrypi/pico-sdk/blob/master/src/rp2_common/hardware_uart/uart.c Lines 42 - 92
42 uint uart_init(uart_inst_t *uart, uint baudrate) {
43         invalid_params_if(HARDWARE_UART, uart != uart0 && uart != uart1);
45         if (uart_clock_get_hz(uart) == 0) {
46                return 0;
47         }
49         uart_reset(uart);
50         uart_unreset(uart);
52         uart_set_translate_crlf(uart, PICO_UART_DEFAULT_CRLF);
54         // Any LCR writes need to take place before enabling the UART
55         uint baud = uart_set_baudrate(uart, baudrate);
57         // inline the uart_set_format() call, as we don't need the CR disable/re-enable
58         // protection, and also many people will never call it again, so having
59         // the generic function is not useful, and much bigger than this inlined
60         // code which is only a handful of instructions.
61         //
62         // The UART_UARTLCR_H_FEN_BITS setting is combined as well as it is the same register
63 #ifdef 0
64         uart_set_format(uart, 8, 1, UART_PARITY_NONE);
65         // Enable FIFOs (must be before setting UARTEN, as this is an LCR access)
66         hw_set_bits(&uart_get_hw(uart)->lcr_h, UART_UARTLCR_H_FEN_BITS);
67 #else
68         uint data_bits = 8;
69         uint stop_bits = 1;
70         uint parity = UART_PARITY_NONE;
71         hw_write_masked(&uart_get_hw(uart)->lcr_h,
72                ((data_bits - 5u) << UART_UARTLCR_H_WLEN_LSB) |
73                      ((stop_bits - 1u) << UART_UARTLCR_H_STP2_LSB) |
74                      (bool_to_bit(parity != UART_PARITY_NONE) << UART_UARTLCR_H_PEN_LSB) |
75                      (bool_to_bit(parity == UART_PARITY_EVEN) << UART_UARTLCR_H_EPS_LSB) |
76                      UART_UARTLCR_H_FEN_BITS,
77                UART_UARTLCR_H_WLEN_BITS | UART_UARTLCR_H_STP2_BITS |
78                      UART_UARTLCR_H_PEN_BITS | UART_UARTLCR_H_EPS_BITS |
79                      UART_UARTLCR_H_FEN_BITS);
80 #endif
82         // Enable the UART, both TX and RX
83         uart_get_hw(uart)->cr = UART_UARTCR_UARTEN_BITS | UART_UARTCR_TXE_BITS |
UART_UARTCR_RXE_BITS;
84         // Always enable DREQ signals -- no harm in this if DMA is not listening
85         uart_get_hw(uart)->dmacr = UART_UARTDMACR_TXDMAE_BITS | UART_UARTDMACR_RXDMAE_BITS;
87         return baud;
88 }

#### 12.1.7.1. Baud rate calculation

The UART baud rate is derived from dividing clk_peri.
If the required baud rate is 115200 and UARTCLK = 125MHz then:
Baud Rate Divisor = (125 × 106)/(16 × 115200) ~= 67.817
Therefore, BRDI = 67 and BRDF = 0.817,
Therefore, fractional part, m = integer((0.817 × 64) + 0.5) = 52
Generated baud rate divider = 67 + 52/64 = 67.8125
Generated baud rate = (125 × 106)/(16 × 67.8125) ~= 115207
Error = (abs(115200 - 115207) / 115200) × 100 ~= 0.006%
SDK: https://github.com/raspberrypi/pico-sdk/blob/master/src/rp2_common/hardware_uart/uart.c Lines 155 - 180
155 uint uart_set_baudrate(uart_inst_t *uart, uint baudrate) {
156           invalid_params_if(HARDWARE_UART, baudrate == 0);
157           uint32_t baud_rate_div = (8 * uart_clock_get_hz(uart) / baudrate) + 1;
158           uint32_t baud_ibrd = baud_rate_div >> 7;
159           uint32_t baud_fbrd;
161           if (baud_ibrd == 0) {
162                 baud_ibrd = 1;
163                 baud_fbrd = 0;
164           } else if (baud_ibrd >= 65535) {
165                 baud_ibrd = 65535;
166                 baud_fbrd = 0;
167           }    else {
168                 baud_fbrd = (baud_rate_div & 0x7f) >> 1;
169           }
171           uart_get_hw(uart)->ibrd = baud_ibrd;
172           uart_get_hw(uart)->fbrd = baud_fbrd;
174           // PL011 needs a (dummy) LCR_H write to latch in the divisors.
175           // We don't want to actually change LCR_H contents here.
176           uart_write_lcr_bits_masked(uart, 0, 0);
178           // See datasheet
179           return (4 * uart_clock_get_hz(uart)) / (64 * baud_ibrd + baud_fbrd);
180 }

### 12.1.8. List of registers

The UART0 and UART1 registers start at base addresses of 0x40070000 and 0x40078000 respectively (defined as
UART0_BASE and UART1_BASE in SDK).
Table 1028. List of
Offset               Name                                                   Info
UART registers
0x000                UARTDR                                                 Data Register, UARTDR
0x004                UARTRSR                                                Receive Status Register/Error Clear Register,
UARTRSR/UARTECR
0x018                UARTFR                                                 Flag Register, UARTFR
0x020                UARTILPR                                               IrDA Low-Power Counter Register, UARTILPR

Offset          Name                                     Info
0x024           UARTIBRD                                 Integer Baud Rate Register, UARTIBRD
0x028           UARTFBRD                                 Fractional Baud Rate Register, UARTFBRD
0x02c           UARTLCR_H                                Line Control Register, UARTLCR_H
0x030           UARTCR                                   Control Register, UARTCR
0x034           UARTIFLS                                 Interrupt FIFO Level Select Register, UARTIFLS
0x038           UARTIMSC                                 Interrupt Mask Set/Clear Register, UARTIMSC
0x03c           UARTRIS                                  Raw Interrupt Status Register, UARTRIS
0x040           UARTMIS                                  Masked Interrupt Status Register, UARTMIS
0x044           UARTICR                                  Interrupt Clear Register, UARTICR
0x048           UARTDMACR                                DMA Control Register, UARTDMACR
0xfe0           UARTPERIPHID0                            UARTPeriphID0 Register
0xfe4           UARTPERIPHID1                            UARTPeriphID1 Register
0xfe8           UARTPERIPHID2                            UARTPeriphID2 Register
0xfec           UARTPERIPHID3                            UARTPeriphID3 Register
0xff0           UARTPCELLID0                             UARTPCellID0 Register
0xff4           UARTPCELLID1                             UARTPCellID1 Register
0xff8           UARTPCELLID2                             UARTPCellID2 Register
0xffc           UARTPCELLID3                             UARTPCellID3 Register
UART: UARTDR Register
Offset: 0x000
Description
Data Register, UARTDR
Table 1029. UARTDR
Bits       Description                                                                           Type     Reset
Register
31:12      Reserved.                                                                             -        -
11         OE: Overrun error. This bit is set to 1 if data is received and the receive FIFO is   RO       -
already full. This is cleared to 0 once there is an empty space in the FIFO and a
new character can be written to it.
10         BE: Break error. This bit is set to 1 if a break condition was detected, indicating RO         -
that the received data input was held LOW for longer than a full-word
transmission time (defined as start, data, parity and stop bits). In FIFO mode,
this error is associated with the character at the top of the FIFO. When a break
occurs, only one 0 character is loaded into the FIFO. The next character is only
enabled after the receive data input goes to a 1 (marking state), and the next
valid start bit is received.
9          PE: Parity error. When set to 1, it indicates that the parity of the received data    RO       -
character does not match the parity that the EPS and SPS bits in the Line
Control Register, UARTLCR_H. In FIFO mode, this error is associated with the
character at the top of the FIFO.

Bits        Description                                                                              Type   Reset
8           FE: Framing error. When set to 1, it indicates that the received character did           RO     -
not have a valid stop bit (a valid stop bit is 1). In FIFO mode, this error is
associated with the character at the top of the FIFO.
7:0         DATA: Receive (read) data character. Transmit (write) data character.                    RWF    -
UART: UARTRSR Register
Offset: 0x004
Description
Receive Status Register/Error Clear Register, UARTRSR/UARTECR
Table 1030. UARTRSR
Bits        Description                                                                              Type   Reset
Register
31:4        Reserved.                                                                                -      -
3           OE: Overrun error. This bit is set to 1 if data is received and the FIFO is already      WC     0x0
full. This bit is cleared to 0 by a write to UARTECR. The FIFO contents remain
valid because no more data is written when the FIFO is full, only the contents
of the shift register are overwritten. The CPU must now read the data, to
empty the FIFO.
2           BE: Break error. This bit is set to 1 if a break condition was detected, indicating WC          0x0
that the received data input was held LOW for longer than a full-word
transmission time (defined as start, data, parity, and stop bits). This bit is
cleared to 0 after a write to UARTECR. In FIFO mode, this error is associated
with the character at the top of the FIFO. When a break occurs, only one 0
character is loaded into the FIFO. The next character is only enabled after the
receive data input goes to a 1 (marking state) and the next valid start bit is
received.
1           PE: Parity error. When set to 1, it indicates that the parity of the received data       WC     0x0
character does not match the parity that the EPS and SPS bits in the Line
Control Register, UARTLCR_H. This bit is cleared to 0 by a write to UARTECR.
In FIFO mode, this error is associated with the character at the top of the FIFO.
0           FE: Framing error. When set to 1, it indicates that the received character did           WC     0x0
not have a valid stop bit (a valid stop bit is 1). This bit is cleared to 0 by a write
to UARTECR. In FIFO mode, this error is associated with the character at the
top of the FIFO.
UART: UARTFR Register
Offset: 0x018
Description
Flag Register, UARTFR
Table 1031. UARTFR
Bits        Description                                                                              Type   Reset
Register
31:9        Reserved.                                                                                -      -
8           RI: Ring indicator. This bit is the complement of the UART ring indicator,               RO     -
nUARTRI, modem status input. That is, the bit is 1 when nUARTRI is LOW.

Bits        Description                                                                          Type   Reset
7           TXFE: Transmit FIFO empty. The meaning of this bit depends on the state of           RO     0x1
the FEN bit in the Line Control Register, UARTLCR_H. If the FIFO is disabled,
this bit is set when the transmit holding register is empty. If the FIFO is
enabled, the TXFE bit is set when the transmit FIFO is empty. This bit does not
indicate if there is data in the transmit shift register.
6           RXFF: Receive FIFO full. The meaning of this bit depends on the state of the         RO     0x0
FEN bit in the UARTLCR_H Register. If the FIFO is disabled, this bit is set when
the receive holding register is full. If the FIFO is enabled, the RXFF bit is set
when the receive FIFO is full.
5           TXFF: Transmit FIFO full. The meaning of this bit depends on the state of the        RO     0x0
FEN bit in the UARTLCR_H Register. If the FIFO is disabled, this bit is set when
the transmit holding register is full. If the FIFO is enabled, the TXFF bit is set
when the transmit FIFO is full.
4           RXFE: Receive FIFO empty. The meaning of this bit depends on the state of the RO            0x1
FEN bit in the UARTLCR_H Register. If the FIFO is disabled, this bit is set when
the receive holding register is empty. If the FIFO is enabled, the RXFE bit is set
when the receive FIFO is empty.
3           BUSY: UART busy. If this bit is set to 1, the UART is busy transmitting data.        RO     0x0
This bit remains set until the complete byte, including all the stop bits, has
been sent from the shift register. This bit is set as soon as the transmit FIFO
becomes non-empty, regardless of whether the UART is enabled or not.
2           DCD: Data carrier detect. This bit is the complement of the UART data carrier        RO     -
detect, nUARTDCD, modem status input. That is, the bit is 1 when nUARTDCD
is LOW.
1           DSR: Data set ready. This bit is the complement of the UART data set ready,          RO     -
nUARTDSR, modem status input. That is, the bit is 1 when nUARTDSR is LOW.
0           CTS: Clear to send. This bit is the complement of the UART clear to send,            RO     -
nUARTCTS, modem status input. That is, the bit is 1 when nUARTCTS is LOW.
UART: UARTILPR Register
Offset: 0x020
Description
IrDA Low-Power Counter Register, UARTILPR
Table 1032. UARTILPR
Bits        Description                                                                          Type   Reset
Register
31:8        Reserved.                                                                            -      -
7:0         ILPDVSR: 8-bit low-power divisor value. These bits are cleared to 0 at reset.        RW     0x00
UART: UARTIBRD Register
Offset: 0x024
Description
Integer Baud Rate Register, UARTIBRD
Table 1033. UARTIBRD
Bits        Description                                                                          Type   Reset
Register
31:16       Reserved.                                                                            -      -

Bits        Description                                                                              Type   Reset
15:0        BAUD_DIVINT: The integer baud rate divisor. These bits are cleared to 0 on               RW     0x0000
reset.
UART: UARTFBRD Register
Offset: 0x028
Description
Fractional Baud Rate Register, UARTFBRD
Table 1034.
Bits        Description                                                                              Type   Reset
UARTFBRD Register
31:6        Reserved.                                                                                -      -
5:0         BAUD_DIVFRAC: The fractional baud rate divisor. These bits are cleared to 0              RW     0x00
on reset.
UART: UARTLCR_H Register
Offset: 0x02c
Description
Line Control Register, UARTLCR_H
Table 1035.
Bits        Description                                                                              Type   Reset
UARTLCR_H Register
31:8        Reserved.                                                                                -      -
7           SPS: Stick parity select. 0 = stick parity is disabled 1 = either: * if the EPS bit is   RW     0x0
0 then the parity bit is transmitted and checked as a 1 * if the EPS bit is 1 then
the parity bit is transmitted and checked as a 0. This bit has no effect when
the PEN bit disables parity checking and generation.
6:5         WLEN: Word length. These bits indicate the number of data bits transmitted or RW                0x0
received in a frame as follows: b11 = 8 bits b10 = 7 bits b01 = 6 bits b00 = 5
bits.
4           FEN: Enable FIFOs: 0 = FIFOs are disabled (character mode) that is, the FIFOs            RW     0x0
become 1-byte-deep holding registers 1 = transmit and receive FIFO buffers
are enabled (FIFO mode).
3           STP2: Two stop bits select. If this bit is set to 1, two stop bits are transmitted       RW     0x0
at the end of the frame. The receive logic does not check for two stop bits
being received.
2           EPS: Even parity select. Controls the type of parity the UART uses during                RW     0x0
transmission and reception: 0 = odd parity. The UART generates or checks for
an odd number of 1s in the data and parity bits. 1 = even parity. The UART
generates or checks for an even number of 1s in the data and parity bits. This
bit has no effect when the PEN bit disables parity checking and generation.
1           PEN: Parity enable: 0 = parity is disabled and no parity bit added to the data           RW     0x0
frame 1 = parity checking and generation is enabled.
0           BRK: Send break. If this bit is set to 1, a low-level is continually output on the       RW     0x0
UARTTXD output, after completing transmission of the current character. For
the proper execution of the break command, the software must set this bit for
at least two complete frames. For normal use, this bit must be cleared to 0.
UART: UARTCR Register

Offset: 0x030
Description
Control Register, UARTCR
Table 1036. UARTCR
Bits        Description                                                                           Type   Reset
Register
31:16       Reserved.                                                                             -      -
15          CTSEN: CTS hardware flow control enable. If this bit is set to 1, CTS hardware        RW     0x0
flow control is enabled. Data is only transmitted when the nUARTCTS signal is
asserted.
14          RTSEN: RTS hardware flow control enable. If this bit is set to 1, RTS hardware        RW     0x0
flow control is enabled. Data is only requested when there is space in the
receive FIFO for it to be received.
13          OUT2: This bit is the complement of the UART Out2 (nUARTOut2) modem                   RW     0x0
status output. That is, when the bit is programmed to a 1, the output is 0. For
DTE this can be used as Ring Indicator (RI).
12          OUT1: This bit is the complement of the UART Out1 (nUARTOut1) modem                   RW     0x0
status output. That is, when the bit is programmed to a 1 the output is 0. For
DTE this can be used as Data Carrier Detect (DCD).
11          RTS: Request to send. This bit is the complement of the UART request to               RW     0x0
send, nUARTRTS, modem status output. That is, when the bit is programmed
to a 1 then nUARTRTS is LOW.
10          DTR: Data transmit ready. This bit is the complement of the UART data                 RW     0x0
transmit ready, nUARTDTR, modem status output. That is, when the bit is
programmed to a 1 then nUARTDTR is LOW.
9           RXE: Receive enable. If this bit is set to 1, the receive section of the UART is      RW     0x1
enabled. Data reception occurs for either UART signals or SIR signals
depending on the setting of the SIREN bit. When the UART is disabled in the
middle of reception, it completes the current character before stopping.
8           TXE: Transmit enable. If this bit is set to 1, the transmit section of the UART is    RW     0x1
enabled. Data transmission occurs for either UART signals, or SIR signals
depending on the setting of the SIREN bit. When the UART is disabled in the
middle of transmission, it completes the current character before stopping.
7           LBE: Loopback enable. If this bit is set to 1 and the SIREN bit is set to 1 and       RW     0x0
the SIRTEST bit in the Test Control Register, UARTTCR is set to 1, then the
nSIROUT path is inverted, and fed through to the SIRIN path. The SIRTEST bit
in the test register must be set to 1 to override the normal half-duplex SIR
operation. This must be the requirement for accessing the test registers
during normal operation, and SIRTEST must be cleared to 0 when loopback
testing is finished. This feature reduces the amount of external coupling
required during system test. If this bit is set to 1, and the SIRTEST bit is set to
0, the UARTTXD path is fed through to the UARTRXD path. In either SIR mode
or UART mode, when this bit is set, the modem outputs are also fed through to
the modem inputs. This bit is cleared to 0 on reset, to disable loopback.
6:3         Reserved.                                                                             -      -

Bits        Description                                                                               Type   Reset
2           SIRLP: SIR low-power IrDA mode. This bit selects the IrDA encoding mode. If               RW     0x0
this bit is cleared to 0, low-level bits are transmitted as an active high pulse
with a width of 3 / 16th of the bit period. If this bit is set to 1, low-level bits are
transmitted with a pulse width that is 3 times the period of the IrLPBaud16
input signal, regardless of the selected bit rate. Setting this bit uses less
power, but might reduce transmission distances.
1           SIREN: SIR enable: 0 = IrDA SIR ENDEC is disabled. nSIROUT remains LOW (no RW                    0x0
light pulse generated), and signal transitions on SIRIN have no effect. 1 = IrDA
SIR ENDEC is enabled. Data is transmitted and received on nSIROUT and
SIRIN. UARTTXD remains HIGH, in the marking state. Signal transitions on
UARTRXD or modem status inputs have no effect. This bit has no effect if the
UARTEN bit disables the UART.
0           UARTEN: UART enable: 0 = UART is disabled. If the UART is disabled in the                 RW     0x0
middle of transmission or reception, it completes the current character before
stopping. 1 = the UART is enabled. Data transmission and reception occurs for
either UART signals or SIR signals depending on the setting of the SIREN bit.
UART: UARTIFLS Register
Offset: 0x034
Description
Interrupt FIFO Level Select Register, UARTIFLS
Table 1037. UARTIFLS
Bits        Description                                                                               Type   Reset
Register
31:6        Reserved.                                                                                 -      -
5:3         RXIFLSEL: Receive interrupt FIFO level select. The trigger points for the receive RW             0x2
interrupt are as follows: b000 = Receive FIFO becomes >= 1 / 8 full b001 =
Receive FIFO becomes >= 1 / 4 full b010 = Receive FIFO becomes >= 1 / 2 full
b011 = Receive FIFO becomes >= 3 / 4 full b100 = Receive FIFO becomes >= 7
/ 8 full b101-b111 = reserved.
2:0         TXIFLSEL: Transmit interrupt FIFO level select. The trigger points for the                RW     0x2
transmit interrupt are as follows: b000 = Transmit FIFO becomes <= 1 / 8 full
b001 = Transmit FIFO becomes <= 1 / 4 full b010 = Transmit FIFO becomes <=
1 / 2 full b011 = Transmit FIFO becomes <= 3 / 4 full b100 = Transmit FIFO
becomes <= 7 / 8 full b101-b111 = reserved.
UART: UARTIMSC Register
Offset: 0x038
Description
Interrupt Mask Set/Clear Register, UARTIMSC
Table 1038.
Bits        Description                                                                               Type   Reset
UARTIMSC Register
31:11       Reserved.                                                                                 -      -
10          OEIM: Overrun error interrupt mask. A read returns the current mask for the               RW     0x0
UARTOEINTR interrupt. On a write of 1, the mask of the UARTOEINTR interrupt
is set. A write of 0 clears the mask.

Bits       Description                                                                     Type   Reset
9          BEIM: Break error interrupt mask. A read returns the current mask for the       RW     0x0
UARTBEINTR interrupt. On a write of 1, the mask of the UARTBEINTR interrupt
is set. A write of 0 clears the mask.
8          PEIM: Parity error interrupt mask. A read returns the current mask for the      RW     0x0
UARTPEINTR interrupt. On a write of 1, the mask of the UARTPEINTR interrupt
is set. A write of 0 clears the mask.
7          FEIM: Framing error interrupt mask. A read returns the current mask for the     RW     0x0
UARTFEINTR interrupt. On a write of 1, the mask of the UARTFEINTR interrupt
is set. A write of 0 clears the mask.
6          RTIM: Receive timeout interrupt mask. A read returns the current mask for the RW       0x0
UARTRTINTR interrupt. On a write of 1, the mask of the UARTRTINTR interrupt
is set. A write of 0 clears the mask.
5          TXIM: Transmit interrupt mask. A read returns the current mask for the          RW     0x0
UARTTXINTR interrupt. On a write of 1, the mask of the UARTTXINTR interrupt
is set. A write of 0 clears the mask.
4          RXIM: Receive interrupt mask. A read returns the current mask for the           RW     0x0
UARTRXINTR interrupt. On a write of 1, the mask of the UARTRXINTR interrupt
is set. A write of 0 clears the mask.
3          DSRMIM: nUARTDSR modem interrupt mask. A read returns the current mask          RW     0x0
for the UARTDSRINTR interrupt. On a write of 1, the mask of the
UARTDSRINTR interrupt is set. A write of 0 clears the mask.
2          DCDMIM: nUARTDCD modem interrupt mask. A read returns the current mask RW              0x0
for the UARTDCDINTR interrupt. On a write of 1, the mask of the
UARTDCDINTR interrupt is set. A write of 0 clears the mask.
1          CTSMIM: nUARTCTS modem interrupt mask. A read returns the current mask          RW     0x0
for the UARTCTSINTR interrupt. On a write of 1, the mask of the
UARTCTSINTR interrupt is set. A write of 0 clears the mask.
0          RIMIM: nUARTRI modem interrupt mask. A read returns the current mask for        RW     0x0
the UARTRIINTR interrupt. On a write of 1, the mask of the UARTRIINTR
interrupt is set. A write of 0 clears the mask.
UART: UARTRIS Register
Offset: 0x03c
Description
Raw Interrupt Status Register, UARTRIS
Table 1039. UARTRIS
Bits       Description                                                                     Type   Reset
Register
31:11      Reserved.                                                                       -      -
10         OERIS: Overrun error interrupt status. Returns the raw interrupt state of the   RO     0x0
UARTOEINTR interrupt.
9          BERIS: Break error interrupt status. Returns the raw interrupt state of the     RO     0x0
UARTBEINTR interrupt.
8          PERIS: Parity error interrupt status. Returns the raw interrupt state of the    RO     0x0
UARTPEINTR interrupt.

Bits       Description                                                                       Type   Reset
7          FERIS: Framing error interrupt status. Returns the raw interrupt state of the     RO     0x0
UARTFEINTR interrupt.
6          RTRIS: Receive timeout interrupt status. Returns the raw interrupt state of the   RO     0x0
UARTRTINTR interrupt. a
5          TXRIS: Transmit interrupt status. Returns the raw interrupt state of the          RO     0x0
UARTTXINTR interrupt.
4          RXRIS: Receive interrupt status. Returns the raw interrupt state of the           RO     0x0
UARTRXINTR interrupt.
3          DSRRMIS: nUARTDSR modem interrupt status. Returns the raw interrupt state RO             -
of the UARTDSRINTR interrupt.
2          DCDRMIS: nUARTDCD modem interrupt status. Returns the raw interrupt state RO             -
of the UARTDCDINTR interrupt.
1          CTSRMIS: nUARTCTS modem interrupt status. Returns the raw interrupt state         RO     -
of the UARTCTSINTR interrupt.
0          RIRMIS: nUARTRI modem interrupt status. Returns the raw interrupt state of        RO     -
the UARTRIINTR interrupt.
UART: UARTMIS Register
Offset: 0x040
Description
Masked Interrupt Status Register, UARTMIS
Table 1040. UARTMIS
Bits       Description                                                                       Type   Reset
Register
31:11      Reserved.                                                                         -      -
10         OEMIS: Overrun error masked interrupt status. Returns the masked interrupt        RO     0x0
state of the UARTOEINTR interrupt.
9          BEMIS: Break error masked interrupt status. Returns the masked interrupt          RO     0x0
state of the UARTBEINTR interrupt.
8          PEMIS: Parity error masked interrupt status. Returns the masked interrupt         RO     0x0
state of the UARTPEINTR interrupt.
7          FEMIS: Framing error masked interrupt status. Returns the masked interrupt        RO     0x0
state of the UARTFEINTR interrupt.
6          RTMIS: Receive timeout masked interrupt status. Returns the masked                RO     0x0
interrupt state of the UARTRTINTR interrupt.
5          TXMIS: Transmit masked interrupt status. Returns the masked interrupt state       RO     0x0
of the UARTTXINTR interrupt.
4          RXMIS: Receive masked interrupt status. Returns the masked interrupt state        RO     0x0
of the UARTRXINTR interrupt.
3          DSRMMIS: nUARTDSR modem masked interrupt status. Returns the masked               RO     -
interrupt state of the UARTDSRINTR interrupt.
2          DCDMMIS: nUARTDCD modem masked interrupt status. Returns the masked               RO     -
interrupt state of the UARTDCDINTR interrupt.

Bits       Description                                                                     Type   Reset
1          CTSMMIS: nUARTCTS modem masked interrupt status. Returns the masked             RO     -
interrupt state of the UARTCTSINTR interrupt.
0          RIMMIS: nUARTRI modem masked interrupt status. Returns the masked               RO     -
interrupt state of the UARTRIINTR interrupt.
UART: UARTICR Register
Offset: 0x044
Description
Interrupt Clear Register, UARTICR
Table 1041. UARTICR
Bits       Description                                                                     Type   Reset
Register
31:11      Reserved.                                                                       -      -
10         OEIC: Overrun error interrupt clear. Clears the UARTOEINTR interrupt.           WC     -
9          BEIC: Break error interrupt clear. Clears the UARTBEINTR interrupt.             WC     -
8          PEIC: Parity error interrupt clear. Clears the UARTPEINTR interrupt.            WC     -
7          FEIC: Framing error interrupt clear. Clears the UARTFEINTR interrupt.           WC     -
6          RTIC: Receive timeout interrupt clear. Clears the UARTRTINTR interrupt.         WC     -
5          TXIC: Transmit interrupt clear. Clears the UARTTXINTR interrupt.                WC     -
4          RXIC: Receive interrupt clear. Clears the UARTRXINTR interrupt.                 WC     -
3          DSRMIC: nUARTDSR modem interrupt clear. Clears the UARTDSRINTR                  WC     -
interrupt.
2          DCDMIC: nUARTDCD modem interrupt clear. Clears the UARTDCDINTR                  WC     -
interrupt.
1          CTSMIC: nUARTCTS modem interrupt clear. Clears the UARTCTSINTR                  WC     -
interrupt.
0          RIMIC: nUARTRI modem interrupt clear. Clears the UARTRIINTR interrupt.          WC     -
UART: UARTDMACR Register
Offset: 0x048
Description
DMA Control Register, UARTDMACR
Table 1042.
Bits       Description                                                                     Type   Reset
UARTDMACR Register
31:3       Reserved.                                                                       -      -
2          DMAONERR: DMA on error. If this bit is set to 1, the DMA receive request        RW     0x0
outputs, UARTRXDMASREQ or UARTRXDMABREQ, are disabled when the
UART error interrupt is asserted.
1          TXDMAE: Transmit DMA enable. If this bit is set to 1, DMA for the transmit      RW     0x0
FIFO is enabled.
0          RXDMAE: Receive DMA enable. If this bit is set to 1, DMA for the receive FIFO   RW     0x0
is enabled.

UART: UARTPERIPHID0 Register
Offset: 0xfe0
Description
UARTPeriphID0 Register
Table 1043.
Bits        Description                                                                   Type   Reset
UARTPERIPHID0
Register
31:8        Reserved.                                                                     -      -
7:0         PARTNUMBER0: These bits read back as 0x11                                     RO     0x11
UART: UARTPERIPHID1 Register
Offset: 0xfe4
Description
UARTPeriphID1 Register
Table 1044.
Bits        Description                                                                   Type   Reset
UARTPERIPHID1
Register
31:8        Reserved.                                                                     -      -
7:4         DESIGNER0: These bits read back as 0x1                                        RO     0x1
3:0         PARTNUMBER1: These bits read back as 0x0                                      RO     0x0
UART: UARTPERIPHID2 Register
Offset: 0xfe8
Description
UARTPeriphID2 Register
Table 1045.
Bits        Description                                                                   Type   Reset
UARTPERIPHID2
Register
31:8        Reserved.                                                                     -      -
7:4         REVISION: This field depends on the revision of the UART: r1p0 0x0 r1p1 0x1   RO     0x3
r1p3 0x2 r1p4 0x2 r1p5 0x3
3:0         DESIGNER1: These bits read back as 0x4                                        RO     0x4
UART: UARTPERIPHID3 Register
Offset: 0xfec
Description
UARTPeriphID3 Register
Table 1046.
Bits        Description                                                                   Type   Reset
UARTPERIPHID3
Register
31:8        Reserved.                                                                     -      -
7:0         CONFIGURATION: These bits read back as 0x00                                   RO     0x00
UART: UARTPCELLID0 Register
Offset: 0xff0
Description
UARTPCellID0 Register

