# Rust, Futures, Embedded Systems & the AI Agent Convergence

Research notes — February 2026

---

## 1. Non-Blocking Async Send/Recv → Futures

Futures are the abstraction layer built on top of non-blocking send/recv — but they
generalize beyond I/O. A future models **any computation that may need to yield**: I/O
readiness, timer expiry, mutex acquisition, cooperative CPU-bound work, etc. The non-blocking
send/recv case is the motivating example, but the abstraction is broader.

A non-blocking operation splits into two phases — initiation ("start this, don't wait") and
completion ("is it done yet?"). A future is the **handle** that bridges those two phases.

| Non-blocking primitive      | Future equivalent                        |
|-----------------------------|------------------------------------------|
| `send()` returns immediately | `future = async_send(data)`             |
| `recv()` returns immediately | `future = async_recv()`                 |
| Polling for readiness        | Executor polls the `Future`; in Rust, wake-driven via `Waker` (not busy-loop) |
| Completion callback          | `await` (language-level); `.then()` is a combinator from libraries like `futures::FutureExt`, not built into Rust |
| Error on EAGAIN/EWOULDBLOCK  | `Future` in `Pending` state             |
| Data available               | `Future` resolves to `Ready(value)`     |

Key insight: Futures formalize the "IOU." In raw non-blocking I/O, the programmer manually
tracks what's pending and polls for completion. A future encapsulates that pending state into
a first-class value you can compose, chain, and pass around. The executor replaces the manual
poll loop — but in Rust, this is **wake-driven**, not spinning: when a future returns
`Pending`, it registers a `Waker` with the event source (e.g., an interrupt on an MCU, epoll
on Linux), and the executor only re-polls when woken. Futures add composability — "do A then
B", "do A and B concurrently", "whichever of A or B finishes first."


## 2. Historical Lineage

```
Raw non-blocking I/O            (select/poll, 1983)
    ↓
Event loops + callbacks          (libevent, Node.js)
    ↓
Futures/Promises                 (Scala, JS Promises — push-based)
    ↓
async/await syntax               (C# 2012, Python 2015, JS 2017)
    ↓
Rust's zero-cost futures         (poll-based, no heap alloc, no runtime)
    ↓
Embassy on microcontrollers      (poll-based async on bare metal, single stack)
```

Each layer adds ergonomics without changing the fundamental mechanism: initiate I/O without
blocking, get notified when it completes.


## 3. Rust's Contribution: Pull-Based Zero-Cost Futures

Most languages before Rust used **push-based** futures — creating a future immediately
schedules work, the future pushes its result to callbacks. Rust inverted this:

| Dimension      | Push (JS/C#/Java)                 | Pull (Rust)                              |
|----------------|-----------------------------------|------------------------------------------|
| Who drives?    | Future pushes results out         | Executor pulls via `poll()`              |
| Allocation     | Heap-allocated per future         | Stack-allocated, compiler-generated enum |
| Cancellation   | Complex (cancellation tokens)     | Drop the future — done                   |
| Runtime        | Built into the language           | Swappable library (tokio, embassy, smol) |
| Lazy?          | No — starts immediately           | Yes — nothing happens until polled       |

The compiler transforms `async fn` into a state machine enum. Each `await` point becomes a
variant. In the default monomorphized/unboxed case, this means **no heap allocation, no
vtable dispatch** — which is why it can run on microcontrollers in `no_std`. However, Rust
async *can* use dynamic dispatch and heap allocation when you opt in (e.g., `Box::pin(fut)`,
`dyn Future`). The key claim is that zero-alloc static-dispatch is *possible*, not that it's
the only mode.

```rust
trait Future {
    type Output;
    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output>;
}

enum Poll<T> {
    Ready(T),   // like recv() returning data
    Pending,    // like recv() returning EWOULDBLOCK
}
```

The "no runtime" decision is the radical part — every other async/await language ships a
runtime. Rust said "that's a library concern." Tradeoff: ecosystem friction ("what runtime
are you on?").


## 3a. The Executor: The Missing Glue

The `Future` trait defines *what* a task does at each poll. The **executor** is the runtime
component that decides *when* and *which* futures to poll. It's the bridge between "future =
IOU" and "the MCU actually sleeps and wakes."

An executor has three responsibilities:

1. **Run queue** — A set of tasks (top-level futures) that are ready to be polled. On an MCU
   this is typically a static array, not a heap-allocated queue.

2. **poll() loop** — The executor dequeues a task, calls `poll()`. If it returns `Ready`,
   the task is done. If `Pending`, the task goes back to sleep.

3. **Waker wiring** — This is the key mechanism. When `poll()` returns `Pending`, the future
   has stashed a `Waker` (provided via the `Context` argument) into whatever it's waiting on.
   Later, when the event occurs, that event source calls `waker.wake()`, which puts the task
   back on the run queue.

On a desktop (tokio), the waker is typically wired to epoll/kqueue. **On an MCU (Embassy),
the waker is wired to a hardware interrupt:**

```
Hardware interrupt fires (timer, UART rx, GPIO pin change)
    → ISR calls waker.wake()
    → Task placed back on executor's run queue
    → Executor wakes from WFI (Wait For Interrupt), polls the task
    → Task runs to next .await, returns Pending again
    → Executor calls WFI, MCU sleeps
```

This is why Embassy achieves low power automatically — between polls, the executor executes
the ARM `WFI` instruction (or equivalent), and the core draws near-zero current until an
interrupt fires. No tick-based polling, no idle task burning cycles.

```rust
// Simplified executor loop (conceptual)
loop {
    while let Some(task) = run_queue.dequeue() {
        task.poll(cx);  // cx contains the Waker for this task
    }
    // Nothing ready — sleep until an interrupt fires and calls wake()
    cortex_m::asm::wfi();
}
```

The `Waker` is the critical link: it's how a hardware peripheral (via its ISR) tells the
executor "this task has new data." Without understanding Waker, the connection between
`Future::poll()` and "the LED blinks" is magical. With it, the full path is concrete.


## 4. Embassy: Async on Bare Metal

Embassy (https://embassy.dev/) brings Rust's poll-based async to microcontrollers:

- Tasks transformed at compile time into state machines (stackless coroutines)
- No dynamic memory allocation; eliminates per-task stacks (though the executor itself, and
  any synchronous call chains within a task, still use the single main call stack; ISRs also
  consume stack space). Stack overflow is much *less likely* and far easier to size, but not
  categorically impossible — you still need to reason about worst-case call depth.
- No per-task stack size tuning required (the major FreeRTOS pain point)
- Replaces a traditional RTOS for many use cases — faster and smaller than one
- Executor puts core to sleep when no work; tasks woken by hardware interrupts via `Waker`

### Supported hardware (HALs)

- `embassy-stm32` — all STM32 families
- `embassy-nrf` — Nordic nRF52, nRF53, nRF54, nRF91
- `embassy-rp` — Raspberry Pi Pico RP2040, RP2350
- `esp-hal` — Espressif ESP32 series
- `ch32-hal` — WCH RISC-V chips (CH32V)
- `embassy-mspm0` — TI MSPM0

### Networking and wireless (no OS required)

- TCP/IP: `embassy-net` with async sockets on bare metal
- BLE: `trouble` crate — full BLE 5.x host stack
- LoRa/LoRaWAN: `lora-rs`
- USB: `embassy-usb` with CDC, HID classes
- WiFi: async WiFi on ESP32

### Ariel OS (FOSDEM 2026)

Full RTOS written in Rust, built *on top of* Embassy. Adds multicore preemptive scheduling
and cross-board portability. Supports Cortex-M, ESP, RISC-V.
- https://fosdem.org/2026/schedule/event/BGPKAM-ariel-os-embedded-rtos/


## 5. Maker Projects Using Rust/Embassy (2025–2026)

### Hobbyist

- Pico 2 robot with sensors, autonomous + remote-controlled (Embassy)
- Split keyboard firmware for RP2040 (Dilemma and others)
- DShot drone motor control using PIO on RP2040/RP2350
- USB adapter for PlayStation DualShock controllers
- ESP32 home sensor (BME280, SCD30, SDS011) with MQTT/TLS + OTA
- Custom gamepad prototypes

### Production

- **Akiles** — smart office/hotel locks (team maintains Embassy)
- **Kelvin** — smart radiator covers
- **SuperCritical Redshift 6** — energy hardware
- **GR-MEGA** — granular hardware synthesizer


## 6. What Changed: 2014 vs 2026

### Concurrency on MCUs

2014 options for concurrent tasks (sensor + motor + wireless):

| Approach           | Stack RAM       | Failure modes                                  |
|--------------------|-----------------|------------------------------------------------|
| Super-loop         | 1 stack         | Timing drift, blocking                         |
| ISRs + flags       | 1 + ISR stacks  | Priority inversion, race conditions            |
| FreeRTOS           | 1 stack per task (you guess the size) | Stack overflow (silent corruption), heap fragmentation, deadlocks |

2026 Embassy: multiple async tasks, single stack, compiler-generated state machine, zero
RAM overhead beyond the state itself.

### Bug classes mitigated or eliminated

Safe Rust prevents many memory-safety bugs and data races on shared memory. But it's not
a silver bullet: you can still hit out-of-bounds panics (runtime, not silent corruption),
logical races (e.g., TOCTOU on peripheral state), and `unsafe` blocks (common in HAL and
driver layers) can reintroduce the footguns Rust otherwise prevents.

| Bug                     | 2014 C/FreeRTOS          | 2026 Rust/Embassy          |
|-------------------------|--------------------------|----------------------------|
| Buffer overflow         | Runtime crash (if lucky) | Compile error (or panic with bounds check — not silent) |
| Use-after-free          | Silent corruption        | Compile error in safe code; `unsafe` can reintroduce |
| Data race between tasks | Subtle, intermittent     | Compile error (Send/Sync); logical races still possible |
| Forgetting mutex release| Deadlock in production   | Drop releases automatically (RAII) |
| Wrong peripheral pin    | Runs, does nothing       | Type error (HALs encode pin capabilities in types) |
| Stack overflow          | Silent memory corruption | Much less likely (no per-task stacks), easier to size; not impossible |

### Package management

2014: google for a .c/.h library, copy into project, fix includes, discover it targets
a different HAL, port or rewrite. No versioning, no dependency resolution.

2026: `cargo add bmp280-rs`. The `embedded-hal` traits mean a sensor driver written for
STM32 works on nRF52, Pico, ESP32 without modification.

### What was NOT new (honest assessment)

- Concurrent tasks on MCUs — FreeRTOS did this, just with more footguns
- Low-power sleep — FreeRTOS tickless idle existed
- BLE on microcontrollers — Nordic SoftDevice worked, as a proprietary black box
- Building robots/keyboards/sensors — Arduino in 2014 was fine for this

The improvement isn't "new capabilities" — it's that **the cost of correctness dropped
dramatically** (see the specific bug classes above — memory safety and data races are
compile-time, but logical errors and `unsafe` footguns remain). What only expert embedded
engineers could do reliably is now accessible to someone who picks up a $5 Pico.


## 7. The Convergence: Tiny Chips + AI Agents + Zero-Cost Async

### The numbers

| Dimension                   | 2014                    | 2026                                  |
|-----------------------------|-------------------------|---------------------------------------|
| Cheapest useful MCU         | ~$1 (ATtiny, 8-bit)    | $0.10 (CH32V003, 32-bit RISC-V)      |
| RAM for concurrent tasks    | ~4KB (FreeRTOS overhead)| 2KB (Embassy async, no RTOS)          |
| Who can write firmware      | Embedded specialists    | Anyone + coding agent                 |
| Portable driver ecosystem   | None                    | `embedded-hal` + cargo                |
| ML inference on MCU         | Basically no            | Yes (MicroFlow runs on 2KB RAM) [*]   |

[*] Specific project claims (MicroFlow, Ariel-ML, ExecuTorch footprints, etc.) were current
at time of writing. Verify latest status/benchmarks before relying on exact numbers — the
design-space argument holds regardless of any single project's trajectory.

### Which lever unlocks what

Two independent forces are compounding. It helps to separate them:

```
                        │ Low runtime overhead      │ High runtime overhead
                        │ (Rust+Embassy)            │ (C + RTOS / Arduino)
────────────────────────┼───────────────────────────┼─────────────────────────
Low engineering cost    │ ★ THE NEW SPACE           │ Quick hacks on big chips
(AI agents)             │ Niche/personal/small-batch│ (ESP32 + Arduino + Copilot)
                        │ on tiny, cheap hardware   │
────────────────────────┼───────────────────────────┼─────────────────────────
High engineering cost   │ Expert embedded products  │ Traditional embedded
(manual firmware)       │ (Akiles, Kelvin, etc.)    │ (legacy industrial, auto)
                        │                           │
```

**Rust + Embassy shrinks the runtime/correctness side:**
- Multi-protocol firmware becomes feasible on tiny RAM without RTOS stack tuning
- "Lots of little state machines" architecture — each async task is a state machine,
  composed cooperatively, with compile-time memory layout
- Product-level primitives come free from async: timeouts (`with_timeout()`), retries with
  backoff (`select!` + timer), connection state machines with reconnect logic, and
  "sleep by default" power behavior (executor calls WFI when all tasks are Pending)
- These primitives are exactly the kind of robust behavior that was painful to build in
  C super-loops and error-prone to build in FreeRTOS

**AI agents shrink the NRE (non-recurring engineering) side:**
- The fixed cost of developing firmware for a new product is the dominant cost barrier for
  niche/small-batch hardware. BOM (bill of materials) per unit is often trivial ($0.10–$5).
  It's the months of firmware engineering that kill projects before they start.
- Agents collapse iteration cycles: describe → generate → compiler catches bugs → simulate
  → flash. Hours become minutes.
- Agents read datasheets via RAG so the human doesn't need to internalize 400-page reference
  manuals for a one-off project.

**Together, they lower the break-even volume.** A product that previously needed 10,000 units
to amortize firmware NRE now breaks even at 10 — or 1.

### What this unlocks: application categories

**1. High-concurrency "glue boxes" (protocol translators, gateways, legacy retrofits)**

Async shines when firmware is mostly waiting + timers + retries. A Modbus/RS-485 ↔ MQTT/TLS
gateway needs: connection management, buffering, backpressure, reconnect with exponential
backoff, and watchdog supervision. In C, this is a multi-month project. With Embassy, each
concern is an async task composed with `select!` and `join!`, and the executor handles the
interleaving. AI agents can generate the boilerplate; the human defines the protocol mapping.

**2. Dense instrumentation (not just "more sensors" — smarter sensors)**

The value isn't deploying more sensors; it's deploying sensors that do **local filtering,
event detection, and adaptive sampling.** A vibration sensor that only transmits when it
detects an anomaly saves 100x on power and bandwidth vs. one that streams raw data. This
requires concurrent tasks (sample → filter → decide → transmit) running on a coin-cell
budget. Embassy's wake-driven executor makes this architecture natural: sample at high rate,
run filter, if boring → sleep for 10 minutes; if interesting → wake radio task, transmit.

**3. Energy-harvesting / ultra-low-duty-cycle devices**

"Sleep is the default" architectures — devices that wake for milliseconds, do work, and
sleep for minutes or hours. Solar-powered, piezo-harvested, or thermal-harvested devices
where every microamp matters. The WFI-based executor model (Section 3a) maps directly: all
tasks Pending → core sleeps → interrupt fires → wake, poll, act, sleep. Getting this right
in C/FreeRTOS required careful manual management of sleep modes and peripheral clock gating.
With Embassy, the executor handles it; you just write `Timer::after_secs(600).await`.

**4. Personalized / one-off hardware**

This is where the NRE collapse matters most:
- Custom medical/assistive devices tailored to one person's needs
- Lab instruments and scientific one-offs (custom DAQ, spectrophotometer controller,
  environmental chamber automation)
- Accessibility tools (custom switch interfaces, adaptive controllers)
- Art installations and interactive exhibits

Previously: hire an embedded contractor at $150/hr for months.
Now: describe to an agent, iterate in Wokwi, flash to a $5 board.

**5. Edge ML at the sensor**

MicroFlow (Rust-based) runs neural networks on 8-bit MCUs with 2KB RAM [*verify current].
Ariel-ML adds multi-core inference for 32-bit chips [*verify current].

- Anomaly detection at the sensor (bearing degradation, no cloud round-trip)
- Keyword/sound detection ($0.10 chip listens for glass breaking, machine fault)
- Gesture recognition on wearables (chip costs less than the battery)

**6. Interactive instruments, controllers, and creative devices**

This is an under-discussed "new space" — async is a natural fit for interactive/music
firmware because it's fundamentally about concurrent state machines reacting to real-time
input. A MIDI controller juggles: matrix scanning, analog reads (knobs/faders), USB
communication, LED feedback, timing (clocks, LFOs, sequencers), and preset management —
all simultaneously, all latency-sensitive but at different timescales.

Existing Rust music/interactive projects prove this works:
- **Zlosynth Instruments** (Brno) ships commercial Eurorack modules with 100% Rust firmware
  on Daisy Seed (STM32H7): Kaseta (tape emulation), Achordion (wavetable oscillator)
- **Sitira** — open-source granular Eurorack synth, 100% Rust on Daisy Seed
- **GR-MEGA** by Tasty Chips — commercial granular workstation, rewritten from C++ to Rust
- **Embassy-usb** has a native MIDI class — USB MIDI devices are first-class
- **daisy-embassy** — async audio processing on Electro-Smith Daisy Seed
- **LEDswarm** — spatially-aware distributed LED installation using ESP32 + ultra-wideband
- **RMK** — async keyboard firmware (Embassy) that gets months of battery life on 2000 mAh

Why async specifically helps: each control/input is a task, each output is a task, and
`select!` handles "whichever event happens first." No manual state-machine interleaving,
no ISR priority juggling. You can even run multiple executors at different interrupt
priorities — high-priority for audio DMA callbacks, low-priority for LED/display refresh.

### Concrete case studies: what the tasks look like

These illustrate how "lots of little async tasks" replaces what used to be RTOS-only or
expert-only firmware.

**Case A: Battery-powered field sensor with LoRa telemetry**

A soil moisture sensor deployed per-plant in a greenhouse. Needs to: read sensor, log to
flash, transmit over LoRa to a gateway, and supervise itself.

Architecture note: **the sensor node talks LoRa to a gateway; the gateway does
MQTT/TLS to the cloud.** The sensor never runs a TLS stack — LoRaWAN provides its own
AES-128 encryption. This separation matters because LoRa bandwidth is tiny (~192-222 bytes
per frame) and LoRa devices are too constrained for TLS.

```
Embassy tasks (all on one stack, cooperative):
  sensor_task    — read ADC, apply calibration, write to shared buffer
  storage_task   — flush buffer to external flash on threshold
  radio_task     — wake LoRa, transmit, sleep; retry with backoff on failure
  watchdog_task  — pet hardware WDT; if any task hasn't reported in 60s, reset
```

**OTA updates — be realistic:** OTA over LoRaWAN (FUOTA) exists but is painful — a 100KB
firmware image takes hours due to 1% duty cycle limits. A 51KB update can take ~17 hours.
Delta/diff updates (bpatch) reduce this 9-18x but add complexity. For small deployments:
plan on physical USB/SWD updates during development. For deployed devices: use embassy-boot
(power-fail-safe swap + ed25519 signature verification + automatic rollback) with a
secondary BLE radio for proximity OTA when a technician visits, and reserve LoRaWAN FUOTA
for critical security patches only.

Each task is 20–50 lines of Rust. The agent generates the scaffolding; the human defines
calibration curves and transmission schedules. Total firmware: ~500 lines. Battery life:
years (WFI between readings, radio duty cycle <1%).

**Case B: Industrial protocol bridge (Modbus ↔ MQTT)**

A legacy factory has RS-485 Modbus sensors. Management wants them on a cloud dashboard.
Previously: buy a $200 industrial gateway appliance, or spend weeks writing C firmware.

```
Embassy tasks:
  modbus_task       — poll Modbus devices on UART, parse responses
  mqtt_task         — maintain TLS connection to broker, publish readings
  buffer_task       — ring buffer with backpressure; if MQTT is down, queue locally
  reconnect_task    — exponential backoff on connection loss (1s, 2s, 4s... 60s cap)
  health_task       — publish heartbeat + diagnostics every 5 minutes
```

The `select!` combinator handles "either Modbus response arrived OR MQTT needs keepalive OR
reconnect timer expired." This interleaving is where async earns its keep — in C you'd need
either a complex event-flag system or an RTOS with semaphores.

**Case C: Custom assistive device**

A wearable for a person with limited hand mobility: BLE-controlled haptic feedback with
therapist-configurable calibration.

```
Embassy tasks:
  haptic_task       — drive LRA motor via PWM based on pattern queue
  ble_task          — receive commands and calibration updates via BLE (trouble crate)
  calibration_task  — persist user-specific thresholds to flash
  safety_task       — hard-limit vibration intensity + duty cycle regardless of commands
  telemetry_task    — log usage patterns, transmit to therapist dashboard weekly
```

This is 5 concurrent concerns on a chip that costs less than the coin cell powering it.
Without async, each concern's state machine would be interleaved by hand — a source of
bugs in exactly the kind of device where bugs matter most.

### AI tools for embedded firmware (current landscape)

- **Embedder** (https://embedder.com) — AI agent purpose-built for firmware, ingests
  datasheets/reference manuals via RAG, supports 300+ MCU variants, used at Tesla/NVIDIA/Medtronic
- **Microchip MPLAB AI Assistant** — VS Code extension for Microchip MCUs
- **General agents** (Claude Code, Cursor, etc.) — increasingly capable with embedded Rust
  given the strong type system and compiler error messages as feedback
- **Research** — "Securing LLM-Generated Embedded Firmware through AI Agent-Driven Validation
  and Patching" (arXiv:2509.09970) — three-phase methodology for validating AI-generated firmware

### Edge AI / TinyML + Rust

*Note: verify current status and benchmarks before relying on specific claims below.
The field is moving fast; project scope and performance numbers may have shifted.*

- **MicroFlow** — Rust-based inference engine, claimed to run on bare-metal 8-bit MCUs with
  2KB RAM, reported to outperform TensorFlow Lite Micro
  (https://github.com/matteocarnelos/microflow-rs)
- **Ariel-ML** — Multi-core TinyML pipeline in embedded Rust (arXiv:2512.09800)
- **ExecuTorch** (Meta) — reported 1.0 GA October 2025, 50KB base footprint, targets MCUs
  through smartphones
- Trend: engineers now think in "energy per inference" — how much energy per classification,
  how many classifications per battery cycle


## 8. Unsolved / Open Questions

- **Analog and timing-critical code** — AI agents struggle with precise timing (bit-banging,
  PID motor control, RF front-ends). Physical world constraints remain hard.
- **Hardware design** — Firmware can be AI-generated, but PCB design and component selection
  are still manual bottlenecks. AI-assisted PCB tools are emerging.
- **Certification** — For medical/automotive/safety-critical, AI-generated firmware faces
  unanswered regulatory questions. Validation research is active but early.
- **Rust's learning curve** — Even with agent help, lifetimes + ownership + async + no_std
  is non-trivial. Agent helps but doesn't eliminate the learning.
- **C/C++ inertia** — Established teams with existing products still use C/C++. Adoption
  is happening in new projects, not rewrites.


## 9. The Core Thesis

```
Expensive chip + expert firmware + RTOS overhead
    → high NRE, high marginal BOM → only viable at mass-market volumes

Cheap chip + AI-assisted firmware + zero-cost async
    → low NRE, trivial BOM → viable at batch sizes down to 1
```

The economics: an embedded product has two cost components. **NRE (non-recurring engineering)**
is the fixed cost of developing the firmware — the months of specialist labor. **BOM (bill of
materials)** is the per-unit cost. In 2014, NRE was high (expert firmware in C) and BOM was
moderate ($1+ for a capable chip + RTOS license). In 2026, agents collapse NRE (days/hours
instead of months) and tiny chips collapse BOM ($0.10). The break-even volume — the number of
units needed to justify the project — drops from thousands to single digits.

The design space that was "not worth the engineering effort" became enormous.

Nothing in 2026 is impossible that was impossible in 2014. What changed is that the things
that were hard, error-prone, and expert-only became dramatically easier, safer, and more
accessible — and the economic threshold for "worth building" collapsed.


---

# Part II: Personal Exploration Plan

Goal: figure out what's personally compelling — interactive/music/lights, wearable
augmentation, or sensor/connectivity — before committing to a direction. Probe all three
with minimal hardware and one-evening time investments.

→ For the Interactive Music Systems (IMS) angle — how these pulls connect to building
  musical instruments, interactive art, and audience participation systems — see companion
  file: `ims-exploration-notes.md`


## 10. Three Application Pulls

Each pull leads to different MCU choices, different async patterns, and different "what's
the hard part" questions. They overlap but have distinct centers of gravity.

### Pull A: Interactive / lights / timing tricks

LEDs, HID devices, audio-reactive things, USB MIDI, tight timing.

- **Why async matters here:** State machines for UI modes, debouncing, pattern sequencing.
  Lots of "react to input, update output, manage transitions."
- **Board choice:** RP2040 or RP2350 — the PIO (Programmable I/O) subsystem gives you
  cycle-accurate timing for LED protocols (WS2812), audio, and custom serial protocols
  without tying up the CPU.
- **Key async patterns:** Channels for event routing, `select!` for "button OR timer OR
  sensor" branching, "latest-value" semantics (not queuing — you want the freshest IMU
  reading, not a backlog).

### Pull B: Wearable augmentation (BLE, low power, haptics, sensors)

Posture cues, context pingers, habit nudges, health sensing.

- **Why async matters here:** "Mostly sleeping" — the device wakes for milliseconds, does
  work, sleeps for seconds/minutes. The Waker/WFI model (Section 3a) is exactly this.
  Cancellation matters: user switches modes mid-operation, tasks must drop cleanly.
- **Board choice:** nRF52840-class — designed for ultra-low-power BLE. Nordic's chips are
  the industry standard for wearable-style projects. Embassy's `embassy-nrf` HAL is mature.
- **Key async patterns:** "Sleep until event" (timer/button/IMU interrupt), BLE GATT
  services via `trouble` crate, power accounting (duty cycle, wake sources).

### Pull C: Sensor gateway / connectivity

Wi-Fi, OTA updates, TLS, MQTT, logging, protocol bridging.

- **Why async matters here:** Reliability and failure handling — reconnect with backoff,
  buffering during disconnects, watchdog supervision, concurrent protocol handling.
  This is the "glue box" pattern from Section 7.
- **Board choice:** ESP32-S3 — Wi-Fi + BLE + TLS + OTA ecosystem, good Embassy/esp-hal
  support, enough RAM for TLS stack.
- **Key async patterns:** `select!` for multiplexing I/O sources, exponential backoff via
  timers, ring buffers with backpressure, health/heartbeat tasks.

### If you want just ONE board to start

ESP32-S3 + WS2812 strip + IMU sensor lets you taste Pull A + Pull C and "some" Pull B. But
it won't teach ultra-low-power wearable tradeoffs — for that you need nRF.


## 11. Hardware Buy List (covers ~80% of projects)

### Boards (one per pull)

- 1x **RP2040 or RP2350** dev board — for interactive/timing (Pico 2, ~$5)
- 1x **nRF52840** dev board — for wearable/BLE/low-power. Two tiers:
  - *Learning + measurement*: **nRF52840 DK** (~$40) — on-board J-Link debugger, PPK2
    interface, smoothest getting-started but bulky/expensive
  - *Wearable-realistic*: **Xiao nRF52840** (~$10) or **nice!nano** (~$15) — tiny form
    factor, battery charging, but need external debug probe
- 1x **ESP32-S3** dev board — for Wi-Fi/TLS/OTA/connectivity (~$7–10)

### Sensors

- 1x **IMU** (6-axis or 9-axis) — e.g., MPU6050, LSM6DS3, BMI270
- 1x **Environment sensor** (temp/humidity/pressure) — e.g., BME280, BME680
- 1x **MEMS microphone** (I2S) — for audio-reactive projects
- 1x **Time-of-flight distance sensor** — e.g., VL53L0X, VL53L1X

### Actuators

- 1x **Haptic driver + ERM/LRA motor** — e.g., DRV2605L breakout
- 1x **Small speaker/buzzer** — even PWM-driven is fine for learning
- 2x **Servos** — for basic mechanical output
- 1x **WS2812/NeoPixel strip or ring** — immediate visual gratification + timing practice

### Debugging

- 1x **SWD debug probe** — or a dev board that can act as one (Pico can probe another Pico)
- **defmt + RTT logging** — software setup, no extra hardware needed
- 1x **Cheap logic analyzer** (optional but helpful) — for verifying timing, protocol signals

### Rough budget estimate

~$60–100 for boards + sensors + actuators. Individual items are $3–15 each. Start with
one board + one sensor + LEDs and expand from there.


## 12. Learning Progression (async-first, fast feedback)

Each step builds on the previous. The goal is to internalize the executor/Waker/interrupt
model (Section 3a) through progressively more complex projects.

### Step 1: Embassy basics

Timer, task, and "run queue + Waker + interrupt" mental model.

- Blinky with `Timer::after_millis().await`
- Two tasks: blink LED + read button. Understand that both run on one stack.
- Verify: add `defmt::info!()` logging, confirm tasks interleave via RTT output.

### Step 2: One bus, deeply

Async I2C or SPI with a real `embedded-hal` driver, including timeouts and retries.

- Wire up a sensor (e.g., BME280 over I2C)
- Use `with_timeout()` to handle bus hangs
- Implement simple retry logic with `Timer::after()` backoff
- Understand: what happens when the bus is busy? When does the executor sleep vs. spin?

### Step 3: Event architecture

Channels, signals, debouncing — the patterns that make real firmware work.

- **Channels** (`embassy_sync::channel`) for task-to-task communication
- **Debouncing** a button (timer + state machine)
- **"Latest-value" vs queue semantics** — critical distinction:
  - Queue: every message matters (commands, protocol packets)
  - Latest-value (`Signal`): only the freshest reading matters (sensor data, UI state)
- This step is where async starts feeling like a genuine architecture, not just "blinky
  with extra steps."

### Step 4: Power + latency

Prove you're sleeping, measure what waking costs.

- Confirm WFI is happening (measure board current draw — even a USB meter shows the
  difference between polling and sleeping)
- Measure wake-to-action latency (how long from interrupt to task running?)
- Handle jitter: understand what affects timing precision in a cooperative executor
- This step matters for Pull B (wearable) and for any battery-powered project.

### Step 5: One "full-stack" feature

Pick one end-to-end capability and implement it completely:

- **USB HID** — board acts as a keyboard/mouse/gamepad (embassy-usb)
- **BLE service** — advertise a GATT service, read/write characteristics (trouble crate)
- **Wi-Fi publish + OTA** — connect, publish telemetry over MQTT/HTTP, accept firmware
  updates

This step proves you can build something that interacts with the outside world, not just
blink LEDs.


## 13. Toolchain Baseline (making Part II self-contained)

Everything you'd run on a fresh machine. Copy-paste ready.

### Install Rust + targets

```bash
# Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source "$HOME/.cargo/env"

# ARM targets (RP2040 = Cortex-M0+, nRF52840 = Cortex-M4F with FPU)
rustup target add thumbv6m-none-eabi       # RP2040
rustup target add thumbv7em-none-eabihf    # nRF52840 (hard-float — use eabihf, not eabi)
rustup component add rust-src llvm-tools

# ESP32-S3 (Xtensa — needs Espressif's forked compiler)
cargo install espup --locked
espup install
. "$HOME/export-esp.sh"
```

### Flash + log tools

```bash
# probe-rs (flash + defmt log for ARM targets)
curl --proto '=https' --tlsv1.2 -LsSf \
  https://github.com/probe-rs/probe-rs/releases/latest/download/probe-rs-tools-installer.sh | sh

# espflash (flash + serial monitor for ESP32)
cargo install espflash --locked

# elf2uf2 (RP2040 flashing without a debug probe — hold BOOTSEL, plug USB)
cargo install elf2uf2-rs --locked
```

### Binary size analysis

```bash
cargo install cargo-binutils --locked    # provides cargo-size, cargo-objcopy
cargo install cargo-bloat --locked       # per-function and per-crate size breakdown

# Usage:
cargo size --release -- -A               # section sizes (.text, .data, .bss)
cargo bloat --release -n 20              # largest 20 functions
cargo bloat --release --crates           # size by crate
```

### .cargo/config.toml (one per target)

Use **per-target runner blocks** (not broad `cfg(...)`) so you can't accidentally flash
nRF firmware with `--chip RP2040`:

```toml
# RP2040 project
[build]
target = "thumbv6m-none-eabi"
[target.thumbv6m-none-eabi]
runner = "probe-rs run --chip RP2040"
[env]
DEFMT_LOG = "debug"
```

```toml
# nRF52840 project
[build]
target = "thumbv7em-none-eabihf"
[target.thumbv7em-none-eabihf]
runner = "probe-rs run --chip nRF52840_xxAA"
[env]
DEFMT_LOG = "debug"
```

```toml
# ESP32-S3 project
[build]
target = "xtensa-esp32s3-none-elf"
[target.xtensa-esp32s3-none-elf]
runner = "espflash flash --monitor"
[env]
ESP_LOG = "INFO"
```

### Project quickstart

```bash
# Option A: clone Embassy examples (most reliable, always up to date)
git clone https://github.com/embassy-rs/embassy.git
cd embassy/examples/rp        # RP2040 — each folder is a standalone project
cd embassy/examples/nrf52840  # nRF52840

# Option B: ESP32 project generator (interactive TUI)
cargo install esp-generate --locked
esp-generate --chip esp32s3    # select embassy + defmt options

# Option C: cargo-generate templates
cargo install cargo-generate --locked
cargo generate --git https://github.com/bentwire/embassy-rp2040-template.git
```

### VS Code setup

Install: **rust-analyzer** + **probe-rs debugger** extensions. Then `.vscode/settings.json`
(adjust target to match your project):

```json
{
    "rust-analyzer.cargo.target": "thumbv6m-none-eabi",
    "rust-analyzer.check.allTargets": false,
    "rust-analyzer.check.targets": ["thumbv6m-none-eabi"]
}
```

For nRF52840: replace both targets with `"thumbv7em-none-eabihf"`.
For ESP32-S3: replace with `"xtensa-esp32s3-none-elf"`.

(`check.allTargets: false` is critical — without it, rust-analyzer tries host target and
fails on `no_std` code.)

Then `cargo run --release` flashes + streams defmt logs. That's the full loop.


## 14. Three One-Evening Spikes (to probe before committing)

Different hardware, different "what's hard" lessons — but **same async skeleton** so the
spikes are structurally comparable. After each spike, fill in the scorecard.

### The shared skeleton (use for all three spikes)

Define this once so every spike is "swap drivers, keep architecture":

```
input_task     — button/encoder events → Channel<Event, N>
sensor_task    — IMU/mic/temp samples → Signal (latest-value, not queued)
app_task       — the state machine that consumes both (modes, presets, logic)
output_task    — LEDs / haptics / USB / radio (consumes app state)
health_task    — pet watchdog + periodic log (dropped events, max loop time, uptime)
```

Key architectural choices baked in:
- **`Channel`** (queue) for discrete events where every message matters (button presses,
  protocol packets, commands). Bounded; backpressure is explicit.
- **`Signal`** (latest-value) for continuous data where only the freshest reading matters
  (sensor samples, UI state). No backlog, no unbounded memory growth.
- **`Event` enum** as the shared vocabulary: `ButtonPress(u8)`, `Tilt(i16, i16, i16)`,
  `ModeChange(Mode)`, etc. Keeps task interfaces typed and composable.

This pattern turns "async" into a repeatable architecture, not a per-project reinvention.
Spike-specific drivers plug into the skeleton without changing the structure.

### Spike A: Interactive (RP2040/RP2350)

Button/IMU → small state machine → WS2812 LEDs.

- Goal: feel end-to-end latency from input to light change
- Measure: jitter with a cheap logic analyzer (or just eyeball — can you see flicker?)
- Learn: PIO for LED protocol timing, channels for event routing
- Skeleton mapping: `input_task` = button GPIO, `sensor_task` = IMU over I2C,
  `app_task` = LED pattern state machine, `output_task` = PIO WS2812 driver

**Done when:**
- [ ] Button press → visible LED mode change < 20 ms (measured with GPIO toggle + analyzer)
- [ ] IMU tilt → LED hue change at stable rate (no backlog — Signal semantics working)
- [ ] `health_task` logs uptime and zero dropped events

**Scorecard (fill in after spike):**

| Metric | Expected range | Your measurement |
|---|---|---|
| IRQ → first task poll | 1–5 µs (executor wake overhead) | _____ |
| Button → visible LED change (full path) | 50–500 µs (through Channel → app → output) | _____ |
| WS2812 jitter | < 50 ns (PIO is deterministic) | _____ |
| Binary size | 18–28 KB (blinky), 45–70 KB (PIO+tasks) | _____ |
| RAM used | 2–6 KB for a few tasks | _____ |
| Joy (1–5) | — | _____ |
| Pain (1–5) | — | _____ |

Note: "IRQ → first task poll" is the executor overhead only. "Button → visible LED change"
includes the full Channel → app_task → output_task path and is what the user actually
perceives. Both are worth measuring separately.

Measurement method: toggle a second GPIO at the start of `output_task`; logic analyzer
(DSLogic ~$150, or Saleae) measures delta from input edge. `cargo size --release -- -A` for
binary. RP2040 has 264 KB SRAM and 2 MB flash — plenty of headroom.

### Spike B: Wearable (nRF52840)

BLE advertise + simple GATT service + "sleep until event" (timer/button/IMU).

- Goal: feel what "mostly sleeping" firmware is like
- Measure: sleep vs. wake current
- Learn: BLE connection lifecycle, power modes, wake sources
- Skeleton mapping: `input_task` = button, `sensor_task` = IMU,
  `app_task` = BLE GATT server (read/write characteristics), `output_task` = haptic/LED,
  `health_task` = watchdog + connection state log

**Done when:**
- [ ] Phone connects via BLE, reads and writes one GATT characteristic
- [ ] Demonstrate "sleep until event" — observable long idle periods + low current
  (even a USB power meter shows the step change)
- [ ] `health_task` logs BLE connection/disconnection events

**Scorecard (fill in after spike):**

| Metric | Expected range (SoC-level) | Your measurement |
|---|---|---|
| Sleep current (System ON, idle) | 2–5 µA (RTC running) | _____ |
| BLE advertising avg current | 10–20 µA (1s interval) | _____ |
| BLE connected avg current | 8–15 µA (1s conn interval) | _____ |
| Wake→task latency (from WFE) | 1–5 µs | _____ |
| Flash budget: SoftDevice reserved | ~152 KB (on-device, not in your binary) | _____ |
| Flash budget: your app | remaining ~870 KB of 1 MB | _____ |
| RAM used | 10–16 KB (SoftDevice reserves ~6 KB + app) | _____ |
| Joy (1–5) | — | _____ |
| Pain (1–5) | — | _____ |

**Dev board vs SoC caveat:** The µA numbers above are **SoC-level** (what the nRF52840 chip
draws). An nRF52840 DK board with its on-board J-Link debugger, voltage regulators, and LEDs
will draw significantly more (~5-20 mA). To measure SoC-level current: use the DK's current
measurement jumper (P22) with a PPK2, or cut the debugger out of the circuit. On small boards
(Xiao, nice!nano), current is closer to SoC-level but still includes regulator quiescent draw.

Measurement method: Nordic PPK2 (~$100) is ideal — 0.2 µA resolution, 100 kHz sampling.
Even a USB power meter shows the difference between sleeping and awake. `trouble` crate
(pure-Rust BLE, no SoftDevice) saves ~110 KB flash but is less mature — worth trying as
a variant. nRF52840 has 1 MB flash, 256 KB SRAM.

### Spike C: Gateway (ESP32-S3)

Wi-Fi connect + publish telemetry (MQTT or HTTP) + reconnect/backoff + watchdog.

- Goal: feel what "failure handling" firmware is like — what happens when Wi-Fi drops?
- Measure: reconnect time, TLS cost, memory stability over hours
- Learn: TLS on a constrained device, backpressure, watchdog patterns
- Skeleton mapping: `input_task` = N/A (or button for manual trigger),
  `sensor_task` = temp/humidity, `app_task` = publish scheduler + backoff state machine,
  `output_task` = MQTT publish, `health_task` = watchdog + reconnect counter + heap monitor

**Two paths for Wi-Fi + TLS on ESP32-S3:**
- `esp-hal` + `esp-wifi` + Embassy (no_std, async-first) — aligns with shared skeleton,
  sharper edges, `esp-wifi` requires unstable feature flag
- `esp-idf-svc` (std, wraps ESP-IDF C services) — lower friction for TLS/OTA end-to-end,
  but different mental model (not pure Embassy async)
For the spike: try `esp-hal` + Embassy first (consistent with other spikes); fall back to
`esp-idf-svc` if TLS setup is too painful.

**Done when:**
- [ ] Publish loop survives Wi-Fi loss and recovers with exponential backoff (unplug AP,
  watch logs, replug, see buffered readings flush)
- [ ] Buffered readings flush after reconnect without blowing RAM
- [ ] 4-hour soak test shows stable heap (no monotonic growth)

**Scorecard (fill in after spike):**

| Metric | Expected range | Your measurement |
|---|---|---|
| Wi-Fi idle current (DTIM=1) | 20–40 mA (no light sleep) | _____ |
| Wi-Fi idle current (light sleep) | 2–5 mA (DTIM=3) | _____ |
| TLS handshake time | 400–800 ms (first conn) | _____ |
| TLS handshake RAM | 40–65 KB (mbedTLS) | _____ |
| MQTT publish latency (QoS 0) | 1–10 ms (local broker) | _____ |
| Binary size | 450–700 KB (Wi-Fi + TLS + MQTT) | _____ |
| Memory stability (4hr soak) | Watch for heap growth | _____ |
| Joy (1–5) | — | _____ |
| Pain (1–5) | — | _____ |

Measurement method: PPK2 or INA226 breakout for current. `defmt` timestamps for TLS/MQTT
timing. Let it run overnight to check for memory leaks. ESP32-S3 has 4-16 MB flash, 512 KB
SRAM — large but the Wi-Fi blob alone is ~200 KB.

### Comparing after the spikes

| | Spike A (Interactive) | Spike B (Wearable) | Spike C (Gateway) |
|---|---|---|---|
| Joy | ___ | ___ | ___ |
| Pain | ___ | ___ | ___ |
| Input→output latency | ___ | ___ | ___ |
| Sleep/idle current | N/A | ___ | ___ |
| Binary headroom | ___/2 MB | ___/1 MB | ___/4+ MB |
| RAM headroom | ___/264 KB | ___/256 KB | ___/512 KB |
| **Verdict** | ___ | ___ | ___ |

Pick whichever scored highest on joy and lowest on pain. Or combine: many interesting
projects live at the intersection (e.g., BLE wearable + audio-reactive LEDs = Pull A + B).


## 15. Project Ideas (mapped to "new space" categories)

Each maps to an application category from Section 7 and to specific async patterns.

### Interactive / music / art

- **Audio-reactive light instrument** — Mic → FFT-ish features → LED patterns. Button UI
  for presets, flash storage for saving them. Async pattern: streaming pipeline with
  latest-value semantics. Board: RP2040 (PIO for LEDs) or Daisy Seed (if audio DSP needed).
  (Pull A)
- **USB MIDI controller** — Knobs/keys → USB MIDI messages. LED feedback on state changes.
  Very state-machine-y — each control is an async task. Embassy-usb has a **native MIDI
  class** so this is first-class, not a hack. See: `usbd-midi` crate (community standard),
  `cgudrian/usb-midi-rs` (Embassy-native). Board: RP2040. (Pull A)
- **Eurorack module** — Audio DSP on Daisy Seed (STM32H7, 480 MHz, 64 MB SDRAM). The
  `daisy-embassy` crate provides async audio processing. Real examples: Zlosynth's Kaseta
  (tape emulation), Achordion (wavetable oscillator), Sitira (granular synth) — all 100%
  Rust. This is the most technically ambitious pull. (Pull A, advanced)
- **Smart LED installation** — `smart-leds-rs` ecosystem provides WS2812/APA102 drivers;
  `ws2812-pio` for RP2040's PIO. For spatially-aware installations: LEDswarm uses ESP32 +
  DWM3000 ultra-wideband for decimeter-level positioning. (Pull A)

### Wearable augmentation

- **Haptic posture/habit cue** — IMU + simple threshold classifier + gentle haptic buzz.
  Async shines: "mostly waiting + periodic sampling + state machine." Battery-powered,
  BLE for configuration. (Pull B)
- **Wearable "context pinger"** — BLE beacon proximity + timeout logic + haptic patterns.
  Lots of timers + cancellation (user walks away mid-pattern). (Pull B)

### Sensor / connectivity

- **Distributed "tiny sensors"** — 5–20 nodes that wake, measure, occasionally transmit.
  The executor/Waker section is exactly the enabling model. Energy-harvesting candidate.
  (Pull C + Section 7 category 3)
- **Protocol bridge** — UART/RS-485 sensor → BLE or Wi-Fi. Robust reconnect/backoff,
  buffering, watchdogs. The "glue box" archetype. (Pull C)

### Cross-pull (most interesting?)

- **BLE wearable + audio-reactive LEDs** — IMU + mic + BLE config + LED output. Touches
  timing (LEDs), power (BLE sleep), and event architecture (sensor fusion). (Pull A + B)
- **Sensor mesh with local ML** — Multiple nodes with local anomaly detection (MicroFlow),
  one gateway node aggregating via LoRa/BLE. (Pull B + C + edge ML)
- **MIDI controller + haptic feedback** — Knobs/keys for MIDI out, haptic motors for
  physical feedback on parameter changes, BLE for wireless config. (Pull A + B)


## 16. Research Angles (what decides feasibility)

Questions to keep in mind as you explore — they're the "hard parts" that determine whether
a project idea actually works on a given chip.

### Timing determinism

Which parts need cycle-accurate timing (audio sample rates, LED protocols like WS2812,
custom serial) vs. which can be "eventual" (sensor reads, BLE, MQTT publish)? Cycle-accurate
work goes to dedicated hardware (PIO, DMA, timer capture/compare). Async tasks handle the
"eventual" parts. Mixing the two is where architecture decisions matter.

### Memory model

How big do your futures/tasks get? The compiler-generated state machine enum holds all
locals alive across `await` points. Large buffers inside async fns bloat the future. Rule of
thumb: keep big buffers in `static` storage, pass references into tasks. Use `defmt` or
`cargo size` to track.

### Cancellation safety

What happens when you drop a task mid-operation? Common in "switch modes" interactive
devices — user presses a button, current animation should stop, new one starts. If the
dropped task was mid-I2C-transaction, is the bus left in a bad state? Embassy's DMA
transfers are generally cancellation-safe, but custom protocols may not be. Worth testing
explicitly.

### Power accounting

Duty cycle × current draw = battery life. Key variables: how often do you wake, how long
does each wake take, what peripherals are powered during sleep? The `embassy-nrf` HAL
exposes power modes; the executor's WFI is necessary but not sufficient — you also need to
gate clocks and peripherals. A Nordic PPK2 or even a simple INA219 breakout makes this
measurable rather than theoretical.


## 17. Open Questions — Resolved

### Which nRF52840 dev board for Embassy?

- **Learning + power measurement**: nRF52840 DK (~$40). On-board J-Link debugger, PPK2
  interface header, smoothest getting-started. But bulky and not wearable-shaped.
- **Wearable-realistic form factor**: Xiao nRF52840 (~$10, battery charger, tiny) or
  nice!nano (~$15, Pro Micro footprint, keyboard ecosystem). Both need external debug probe.
- Recommendation: start with DK for learning (Steps 1-4), add Xiao/nice!nano for Step 5+.

### `trouble` crate status — stable enough for a spike?

**Yes, with expectations.** `trouble-host` compiles on stable Rust 1.80+. Embassy's book
lists it as the BLE Host across nRF52/RP2040/ESP32 via `bt-hci` traits. Goal is eventual
qualification but it's explicitly not there yet. Stable enough to learn the BLE lifecycle
(advertise/connect/GATT/notify), but expect update friction as APIs evolve.

Alternative: `nrf-softdevice` wraps Nordic's proprietary SoftDevice S140. More mature/proven
but adds ~152 KB flash and is nRF-only.

### ESP32-S3: `esp-hal` + Embassy vs `esp-idf-svc`?

Two different goals:
- **Pure Embassy mental model** (no_std, async-first): `esp-hal` + `esp-wifi` + Embassy.
  Aligns with shared skeleton. Note: `esp-wifi` requires unstable feature flag on esp-hal.
- **"Just make Wi-Fi + TLS + OTA work"**: `esp-idf-svc` wraps ESP-IDF C services
  (Wi-Fi, HTTP, MQTT, NVS, OTA, ESP-TLS). Lower friction for full-stack gateway, but std
  environment, not the same async model.

For spikes: try `esp-hal` first for consistency; fall back to `esp-idf-svc` if TLS is painful.

### Can Wokwi delay hardware purchase?

Partially:
- **Spike A (RP2040)**: Yes for logic/state-machine work. Won't capture PIO timing feel.
- **Spike C (ESP32-S3)**: Yes — Wokwi has ESP32 Wi-Fi + internet simulation (MQTT, HTTP
  via public gateway). Good for architecture rehearsal.
- **Spike B (nRF52840)**: **No** — Wokwi does not support nRF52 boards. BLE simulation is
  not available. You need hardware early for this spike.

Important: Wokwi no longer compiles Rust in-browser. You compile locally, simulate via the
Wokwi VS Code extension.

### MicroFlow "2KB inference" — what model fits?

2KB RAM means **tiny models**: small conv/MLP, heavily quantized (int8), very small
activations. Practical examples:
- Thresholding + tiny MLP for IMU gesture classification
- Anomaly detection on a few sensor channels
- Simple keyword-ish classifier with minimal audio front-end

The limiting factor is usually **activation memory** (intermediate tensors), not weights.
"What fits" = keep intermediate tensors tiny, reuse buffers, stream features instead of
buffering raw data. MicroFlow tested speech command (TinyConv) on ATmega328 (Arduino Uno)
and person detection (MobileNet v1) on ESP32.

### PIO + Embassy: independent or async-wired?

**Both.** PIO state machines run independently once configured (that's the point — they
execute deterministically regardless of CPU load). But Embassy provides async futures to
interact with PIO without polling:

- `FifoInFuture` — await RX FIFO readable
- `FifoOutFuture` — await TX FIFO writable
- `IrqFuture` — await PIO IRQ flag

So the pattern is: PIO handles cycle-accurate timing, Embassy tasks feed/consume PIO FIFOs
via async. Example: `ws2812-pio` driver — PIO does the WS2812 bit timing, DMA fills the
FIFO, Embassy task awaits DMA completion.

### Smallest useful audio pipeline on RP2040?

Embassy includes PIO-backed I2S drivers (`embassy_rp::pio_programs::i2s`):
- `PioI2sIn` / `PioI2sOut` — DMA transfers return futures you can `.await`

Minimal pipeline (avoids full FFT):
1. **Acquire**: I2S MEMS mic → DMA ring buffer (keep buffers in `static`, not in async locals)
2. **Feature**: one cheap feature per frame — RMS (volume) or 2-4 Goertzel bins (bass vs
   treble reactivity, much cheaper than FFT)
3. **Map**: feature → LED brightness/pattern speed or haptic intensity
4. **UI**: button cycles mapping modes/presets
5. **Optional**: stream feature values over USB serial for tuning

This is enough for an audio-reactive light instrument without a DSP thesis.


## 17a. Remaining Open Questions

- [ ] What are the actual `esp-wifi` unstable APIs — how much churn to expect?
- [ ] `trouble` vs `nrf-softdevice` — which to use for Spike B? (try both?)
- [ ] nRF52840 DK vs Xiao — how painful is external debug probe setup on Xiao?
- [ ] Daisy Seed availability and Embassy support maturity for audio projects?
- [ ] What's the smallest Goertzel implementation in no_std Rust?

---


## 18. Key Links & References

### Frameworks & Tools
- Embassy: https://embassy.dev/
- Embassy GitHub: https://github.com/embassy-rs/embassy
- Embassy Book: https://embassy.dev/book/
- Ariel OS: https://fosdem.org/2026/schedule/event/BGPKAM-ariel-os-embedded-rtos/
- Wokwi Simulator (compile locally, simulate via VS Code): https://wokwi.com/rust
- Wokwi ESP32 Wi-Fi simulation: https://docs.wokwi.com/guides/esp32-wifi
- Wokwi supported hardware: https://docs.wokwi.com/getting-started/supported-hardware
- probe-rs (flash/debug): https://probe.rs/
- defmt (logging): https://github.com/knurling-rs/defmt
- esp-generate: https://github.com/esp-rs/esp-generate
- esp-hal: https://github.com/esp-rs/esp-hal

### Curated Lists
- awesome-embedded-rust: https://github.com/rust-embedded/awesome-embedded-rust
- awesome-esp-rust: https://github.com/esp-rs/awesome-esp-rust

### Hardware
- CH32V003 ($0.10 RISC-V): https://github.com/openwch/ch32v003
- WCH Rust HAL: https://github.com/ch32-rs

### Music / Audio / Interactive
- embassy-usb MIDI class: https://docs.embassy.dev/embassy-usb/git/default/class/index.html
- usbd-midi (community): https://github.com/rust-embedded-community/usbd-midi
- daisy-embassy (async audio on Daisy Seed): https://crates.io/crates/daisy-embassy
- Zlosynth Kaseta (Eurorack, Rust): https://github.com/zlosynth/kaseta
- Zlosynth Achordion (Eurorack, Rust): https://github.com/zlosynth/achordion
- Sitira (granular synth, Rust): https://github.com/backtail/sitira-embedded-daisy
- GR-MEGA (granular workstation, Rust): https://www.tastychips.nl/category/gr-mega/
- microdsp (no_std DSP): https://github.com/stuffmatic/microdsp
- synth-utils (embedded synth): https://crates.io/crates/synth-utils
- smart-leds-rs (LED drivers): https://github.com/smart-leds-rs
- ws2812-pio (RP2040 PIO LED driver): https://crates.io/crates/ws2812-pio
- LEDswarm (spatial LED installation): https://github.com/spectrachrome/firmware-rust-esp32-uwb
- RMK (async keyboard firmware): https://docs.rs/rmk/latest/rmk/
- Rust audio programming overview: https://andrewodendaal.com/rust-audio-programming-ecosystem/

### LoRa / OTA
- embassy-boot (bootloader): https://docs.embassy.dev/embassy-boot/git/default/index.html
- lora-rs (async LoRa): https://github.com/lora-rs/lora-rs
- LoRaWAN FUOTA research (MDPI): https://www.mdpi.com/1424-8220/24/7/2104
- bpatch incremental FUOTA: https://arxiv.org/html/2505.13764v2
- Drogue IoT firmware updates: https://blog.drogue.io/firmware-updates-part-1/

### Edge AI / TinyML
- MicroFlow: https://github.com/matteocarnelos/microflow-rs
- MicroFlow paper: https://www.sciencedirect.com/science/article/pii/S2542660525000113
- Ariel-ML paper: https://arxiv.org/abs/2512.09800
- State of Edge AI on MCUs 2026: https://shawnhymel.com/3125/state-of-edge-ai-on-microcontrollers-in-2026/

### AI for Firmware
- Embedder: https://embedder.com
- LLM firmware validation research: https://arxiv.org/abs/2509.09970

### Background
- The Embedded Rust Book: https://docs.rust-embedded.org/book/
- Async Rust vs RTOS Showdown: https://tweedegolf.nl/en/blog/65/async-rust-vs-rtos-showdown
- Embedded Rust in Production 2025: https://onevariable.com/blog/embedded-rust-production/
- Async Rust on Cortex-M (Memfault): https://interrupt.memfault.com/blog/embedded-async-rust
- Rust on ESP Book (toolchain): https://docs.espressif.com/projects/rust/book/getting-started/toolchain.html

### BLE
- trouble-host (pure Rust BLE): https://crates.io/crates/trouble-host
- trouble-host docs: https://docs.rs/trouble-host
- nrf-softdevice (Nordic SoftDevice wrapper): https://github.com/embassy-rs/nrf-softdevice

### Dev Boards
- nRF52840 DK: https://www.nordicsemi.com/Products/Development-hardware/nRF52840-DK
- Seeed XIAO nRF52840: https://wiki.seeedstudio.com/XIAO_BLE/
- nice!nano: https://nicekeyboards.com/docs/nice-nano/

### Measurement Tools
- Nordic PPK2 (~$100, 0.2 µA resolution): https://www.nordicsemi.com/Products/Development-hardware/Power-Profiler-Kit-2
- cargo-binutils (cargo-size): https://github.com/rust-embedded/cargo-binutils
- cargo-bloat: https://crates.io/crates/cargo-bloat

### Embassy PIO
- PIO module docs: https://docs.embassy.dev/embassy-rp/git/rp2040/pio/index.html
- PIO I2S drivers: https://docs.embassy.dev/embassy-rp/git/rp2040/pio_programs/i2s/index.html
- PIO WS2812 example: https://github.com/embassy-rs/embassy/blob/main/examples/rp/src/bin/pio_ws2812.rs
