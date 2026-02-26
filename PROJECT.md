# Embedded Systems Learning Project

## Status: Active (started Feb 2026)

Background: CS (MIT undergrad), new to analog electronics and firmware. Goal is to learn embedded systems by building things, using a 6.115-derived self-study curriculum with real projects as milestones.

---

## Equipment Inventory

### Owned / Ordered

**Microcontrollers:**
- Raspberry Pi Pico 2W (with headers) -- primary learning platform, RP2350
- Adafruit KB2040 (RP2040, Kee Boar Driver form factor) -- free with order
- Adafruit Feather nRF52840 Sense -- BLE, onboard LSM6DS3TR-C IMU, LIS3MDL magnetometer, BMP280, APDS9960, PDM mic, NeoPixel. Second platform.
- ESP32-S3-DevKitC-1 (32MB flash, 8MB PSRAM) -- third platform, WiFi/BLE gateway

**Sensors (breakouts):**
- LSM6DS3TR-C 6-DoF Accel+Gyro IMU -- STEMMA QT (for Pico)
- VL53L0X Time-of-Flight distance sensor (30-1000mm) -- STEMMA QT
- SPW2430 MEMS Microphone -- analog out
- INA219 DC Current Sensor -- STEMMA QT (instrumentation)

**Output:**
- NeoPixel Ring 16x 5050 RGB LED
- Adafruit Pixel Shifter (level shifting for addressable LEDs)
- DRV2605L Haptic Motor Controller -- STEMMA QT
- Piezo Buzzer PS1240

**Test Equipment:**
- Rigol DS1202Z-E oscilloscope (200MHz, 2ch) -- arrived Feb 26
- Digital Multimeter 9205B+
- Raspberry Pi Debug Probe (for probe-rs / SWD on RP2350)

**Infrastructure:**
- Half-size breadboard (400 tie points)
- Adafruit Perma-Proto half-size PCB
- Male headers (36-pin, 10-pack), Female headers (36-pin, 5-pack)
- Jumper wires: F/F 40x6", M/M 20x6", F/M 40x6"
- SparkFun STEMMA QT breadboard breakout adapter
- STEMMA QT cables: 200mm, 300mm
- Adafruit USB-C vertical breakout (downstream, with CC resistors)
- 470 ohm resistors (25-pack)
- 4700uF 10V electrolytic capacitor
- 5V 4A USB-C wall supply
- LiPo battery 3.7V 500mAh (JST-PH, for Feather)
- Soldering iron + silicone mat (already owned)

### Not Yet Needed (buy when relevant)
- Bench power supply (Korad KA3005D, ~$60) -- buy at week 8 when doing motor/power projects
- Logic analyzer (Saleae clone, ~$12) -- buy when 2 scope channels aren't enough
- MPU-6050 breakout (~$3, AliExpress) -- only if you want a second cheap IMU

---

## Curriculum Spine

Adapted from MIT 6.115 (Microcomputer Project Laboratory) for self-study with modern hardware. The Pico 2W + Embassy (Rust) is the primary platform.

### Phase 1: Toolchain + GPIO (Weeks 1-2)
- Embassy async blinky on Pico 2W via Debug Probe (probe-rs + RTT logging)
- NeoPixel ring via PIO (learn RP2350's programmable I/O, Embassy's PIO support)
- Digital input (button) -> NeoPixel response

### Phase 2: I2C Sensors + Analog (Weeks 3-4)
- VL53L0X over I2C: distance reading on RTT
- Distance -> NeoPixel color mapping (first sensor-to-output loop)
- LSM6DS3TR-C over I2C: raw accel/gyro, print to RTT, wave it around
- SPW2430 mic on ADC: ambient level, threshold detection

### Phase 3: Output Modalities (Weeks 5-6)
- DRV2605L haptic controller over I2C: distance -> haptic feedback
- Piezo buzzer via PWM
- Combine: distance -> haptic + NeoPixel + buzzer (multi-output system)

### Phase 4: Communication (Weeks 7-8)
- USB MIDI from Pico to laptop (Embassy USB classes)
- IMU -> conductor state (energy/tilt/stillness/jerk) -> MIDI CC
- SuperCollider receiving MIDI CC on laptop
- **Milestone: Spike 0 conductor instrument working**

### Phase 5: Wireless + Power (Weeks 9-10)
- Move to Feather nRF52840 Sense: BLE advertising sensor data
- ESP32-S3 as BLE central / WiFi gateway
- Battery operation, power profiling with INA219
- Sleep modes and power budgeting

### Phase 6: Analog Fundamentals (Weeks 11-12)
- RC filters, voltage dividers on breadboard (scope practice)
- Op-amp basics (if you acquire an op-amp)
- Transistor as switch for motor/relay driving

### Phase 7: Integration Project (Weeks 13-16)
- Candidate: Body area network (BAN) sensory substitution wearable
- Or: Full conductor instrument with BLE, haptics, and agent contracts
- Design, build, debug, document

---

## Active Build: Spike 0 -- "Ride the Tide"

### Concept
A physical device where body gesture shapes generative audio agents. The system has momentum independent of input (tidal oscillators). Conducting, not playing.

### Architecture
```
Pico 2W + LSM6DS3TR-C (IMU)
  |-- compute conductor state: energy, density, hold, transition
  |-- USB MIDI CC out
  v
Laptop running SuperCollider
  |-- tidal oscillator: ~60-90s autonomous tension cycle
  |-- harmonic field: voices constrained by conductor pressure
  |-- the system plays itself; performer shapes, not commands
```

### Conductor State Mapping
| Gesture | Sensor | Param | Musical Effect |
|---------|--------|-------|----------------|
| Hand height | accel_z tilt | energy | Amplitude + filter brightness |
| Wrist tilt | accel_x/z ratio | density | Number of active voices |
| Stillness | rolling variance -> low | hold | Tidal oscillator pauses, field sustains |
| Sharp flick | jerk threshold | transition | Chord change, register shift |

### Acceptance Criteria
1. Worth hearing autonomously (controller down, music continues and is interesting)
2. Performer makes a noticeable difference
3. Performer cannot choose specific notes (indirect influence only)
4. Feels like influencing, not operating

### Build Order
1. Embassy blinky via Debug Probe (toolchain test)
2. NeoPixel ring via PIO (first real output)
3. I2C IMU read (raw values on RTT)
4. Compute conductor state, verify against gesture
5. USB MIDI CC out, verify SC receives
6. Write SC tidal oscillator patch (composition work, no hardware needed)
7. Connect MIDI CC -> SC params
8. Play. Run acceptance criteria. Iterate mappings.

---

## Future Project: Body Area Network / Sensory Substitution

### Concept
Wearable sensor network that injects environmental data as a synthetic "sense" via haptics. Information must be continuous, low-bandwidth, and spatially/temporally mappable so the brain integrates it below conscious attention.

### Best Candidates for Integration
1. Compass heading (proven, North Paw precedent)
2. Solar position (continuous, cyclical, body-centered)
3. PM2.5 air quality (health-relevant, varies at walkable scale)
4. Ambient RF density (feel EM environment)
5. HRV biofeedback (close the autonomic loop)

### Hardware
- nRF52840 (Feather Sense) as hub
- Additional nRF52840 boards as satellite nodes (ankle, torso)
- BLE mesh between nodes
- Coin vibration motors for haptic output
- Battery life is the main engineering constraint

---

## Architecture Patterns

### Embassy Async Task Skeleton
Shared structure across all embedded builds:
```
input_task   -- button, touch, serial commands
sensor_task  -- I2C/SPI/ADC reads, latest-value semantics (not queued)
app_task     -- state machine, mapping, conductor logic
output_task  -- LEDs, haptic, audio, MIDI, display
health_task  -- watchdog, battery, temp, uptime, session log
```

### Conductor / IMS Unified Architecture
The instrument stack and MIT IMS framework are the same system from opposite ends:

| Conductor (top-down) | IMS / Embedded (bottom-up) |
|----------------------|---------------------------|
| Conductor state vector | Sensor task -> Signal (latest-value) |
| Score / agent contracts | Mapping layer |
| Ensemble (agents) | Time/engine layer |
| Stage (renderers) | Output task |
| Log (counterfactuals) | Health task + session recording |

The meeting point is the **mapping layer**: where physical gesture meets prepared constraint.

---

## Decision Log

| Decision | Rationale |
|----------|-----------|
| Pico 2W as primary platform | RP2350 is well-supported by Embassy, PIO is unique learning, Debug Probe enables proper dev loop |
| Embassy (Rust) over CircuitPython | Learn the async architecture pattern, zero-cost abstractions, transferable to nRF52840. Slower start but better long-term. |
| LSM6DS3TR-C for external IMU | Same sensor as Feather Sense (learn one register set), STEMMA QT (no soldering), $10, Embassy supported |
| MIDI CC over OSC for Spike 0 | No bridge daemon needed, SC speaks MIDI natively, less friction |
| Leaded solder + hygiene | Easier to learn with. Silicone mat + hand washing covers the safety. |
| Skip BNO055/085 fusion IMUs | Onboard fusion hides raw data; better to learn complementary filtering on raw accel/gyro |
| Scope before bench supply | Scope useful from day 1; bench supply only needed when powering non-USB things (week 8+) |

---

## Key References

- *Art of Electronics* (Horowitz & Hill) -- reference, not sequential (`Electronics/`)
- *Making Embedded Systems* (Elecia White) -- practical modern guide
- Embassy docs: https://embassy.dev
- MIT 6.115 OCW: https://web.mit.edu/6.115/www/
- probe-rs: https://probe.rs
- KiCad: for when you're ready to design PCBs (JLCPCB for fab)

---

## Local Reference Library

```
rp2350-reference/                ← RP2350 datasheet (split), SDK headers, reading plan
  INDEX.md                       ← prioritized reading list + three-layer source map
  PATTERN.md                     ← conversion pipeline (Tier 1→2→3), lessons learned

Code_and_Notes/
  pico-sdk/                      ← C SDK 2.2.0 (full clone)
    src/rp2_common/              ← HAL implementations (gpio.c, pio.c, i2c.c, etc.)
  embassy/                       ← Embassy Rust HAL (shallow clone)
    embassy-rp/src/              ← RP2350 HAL (gpio.rs, pio/, uart/, i2c.rs, etc.)
    embassy-rp/src/pio_programs/ ← Pre-built PIO programs (ws2812.rs, etc.)

Research notes (root level):
  ims-exploration-notes.md       ← IMS 5-layer architecture framework
  rust-embedded-futures-research-notes.md  ← Rust async/futures mental model
```
