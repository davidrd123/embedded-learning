# Interactive Music Systems (IMS) Exploration

Research notes — February 2026
Companion to: `rust-embedded-futures-research-notes.md` (embedded Rust / async / hardware)

---

## 1. IMS as a Design Framework

IMS (MIT 21M.385) provides a reusable architecture for building real-time interactive
musical experiences. The core insight: it's not about the technology stack — it's about
the **layers** and how they compose.

### The 5-layer architecture

```
1. Sensing / Input       — mic, camera, motion, controllers, touch, bio, network events
2. Mapping               — "how does input become music?" (the creative decision)
3. Time / Musical Engine — scheduling, quantization, tempo, loops, generative rules, sync
4. Feedback / Output     — sound, visuals, haptics, lights, spatial placement, actuation
5. Experience Design     — learnable in 10s, expressive in 10min, deep in 10hr
```

IMS's topic list spells these out: synthesis, multitrack mixing/looping, MIDI/audio sync,
generative composition, non-standard controllers, music game design, UI/visualization.
Project requirements: produce music, have graphical elements, be real-time interactive.
Interaction modes: games, education, performance, sandbox tools, art/synesthesia.

Sources:
- IMS course: https://musictech.mit.edu/ims/
- OCW assignments: https://ocw.mit.edu/courses/21m-385-interactive-music-systems-fall-2016/pages/assignments/
- MIT News (prototype + playtest loop): https://news.mit.edu/2025/mit-musicians-make-new-tools-for-new-tunes-0708

### Where the layers run (honest boundary)

Not everything runs on an MCU. Being explicit about the boundary prevents scope confusion:

```
                    │ Embedded (MCU/async)    │ Laptop/SBC/Cloud
────────────────────┼─────────────────────────┼──────────────────────────
1. Sensing/Input    │ ★ IMU, pressure, ADC,   │ Camera, Leap Motion,
                    │   matrix scanning, mic   │   complex audio analysis
                    │   (raw capture)          │
────────────────────┼─────────────────────────┼──────────────────────────
2. Mapping          │ Simple thresholds,       │ ★ Complex mappings, ML,
                    │   edge detection         │   gesture recognition
────────────────────┼─────────────────────────┼──────────────────────────
3. Time/Engine      │ MIDI clock, basic        │ ★ Synthesis, sequencing,
                    │   sequencing, trigger     │   generative composition,
                    │                          │   audio rendering
────────────────────┼─────────────────────────┼──────────────────────────
4. Feedback/Output  │ ★ LEDs, haptics, servo,  │ Visuals/projection, spatial
                    │   USB MIDI/HID, PIO      │   audio (d&b Soundscape),
                    │                          │   VR rendering
────────────────────┼─────────────────────────┼──────────────────────────
5. Experience       │ (design, not code)       │ (design, not code)
```

The ★ marks where each platform is strongest. The typical IMS system has **embedded
controllers talking to a laptop engine** — the MCU handles sensing + feedback, the laptop
handles synthesis + complex mapping + visuals. USB MIDI, BLE, or OSC is the bridge.

→ See `rust-embedded-futures-research-notes.md` Sections 3a, 14 for the async task
  model that maps directly to layers 1 + 4.


## 2. How the IMS Layers Map to the Async Task Skeleton

The shared skeleton from the embedded notes (Section 14) maps directly:

```
IMS layer              →  Async task
1. Sensing/Input       →  input_task (buttons, encoders → Channel<Event>)
                          sensor_task (IMU, mic, pressure → Signal, latest-value)
2. Mapping             →  app_task (the state machine — this is where the art lives)
3. Time/Musical Engine →  app_task + timer/scheduler tasks (tempo, quantization)
4. Feedback/Output     →  output_task (LEDs/haptics/USB MIDI/audio DMA)
5. Experience Design   →  the playtesting/iteration loop (not code)
```

Layer 2 (Mapping) is the creative core: identical hardware + different mapping = different
instrument. "How does tilt become pitch? How does pressure become timbre? How does proximity
become harmonic tension?" These are 5-10 lines of code that define the entire experience.


## 3. Four Application Pulls (IMS-extended)

The embedded notes define three pulls (Interactive, Wearable, Gateway). IMS adds a fourth
and reframes all of them as experience outcomes.

### Pull A → "Performance feedback & synesthetic visuals"

Beat-locked LEDs, projected cues, audiovisual instruments, tight input→output response.
Hardware: RP2040 (PIO for LEDs/timing). IMS foregrounds UI/visual cohesion as part of the
system, not an afterthought.

### Pull B → "Embodied instruments & haptic musicianship"

Gesture/dance controllers, biofeedback instruments, tactile metronomes, accessibility tools.
Hardware: nRF52840 (BLE + low power). The haptic output closes the loop — you *feel* tempo,
phrasing, or upcoming changes, which is crucial for "instrument-ness."

### Pull C → "Networked ensembles & audience participation"

Tutti-style synchronization, online jams, multi-device orchestration. Hardware: ESP32-S3
(Wi-Fi + reliability). "Clock discipline as a design primitive" — tight unison vs intentional
smear. "Still works when Wi-Fi is messy" = the whole game.

### Pull D → "Standalone instruments & installations" (NEW)

The thing *is* the instrument, no laptop required. This is where audio I/O, real-time DSP
latency budgets, and codec quality matter.

Hardware: **Electro-Smith Daisy Seed** (STM32H750, ARM Cortex-M7, 480 MHz, 64 MB SDRAM,
AK4556 codec). The Daisy is the standard platform for Rust audio on MCU.

Key constraints different from other pulls:
- Audio DMA callback latency budget: ~0.67 ms at 48 kHz / 32 samples
- Must never miss a buffer fill (audio glitches are immediately audible)
- 32-bit float DSP at 480 MHz gives substantial headroom
- `daisy-embassy` provides async audio processing as a task

Existing Rust examples at this level:
- Zlosynth Kaseta (tape emulation), Achordion (wavetable osc), Sitira (granular synth)
- All 100% Rust, all on Daisy Seed, all commercially viable

→ See `rust-embedded-futures-research-notes.md` Section 7 category 6, Section 15 for
  project details and crate links.


## 4. IMS Project Ideas (framed as experience outcomes)

These are deliberately phrased as *what the user experiences*, not what the firmware does.

### Tier 1: Single-evening builds (one board, one mapping)

**"A micro-instrument anyone can play in 30 seconds"**
One-button/one-tilt controller with rich expressivity (pressure + tilt + gesture).
Visual cueing so the player knows what's possible. USB MIDI to laptop synth.
Like Tutti's instruments but with richer sensing. (Pull A or B)
- References: https://musictech.mit.edu/tutti

**"See your music"**
Audio-reactive LED ring/strip that responds to live playing (mic or line in).
RMS → brightness, spectral balance → hue. Immediate gratification.
No laptop needed if self-contained on RP2040. (Pull A)

**"Feel the beat"**
Haptic metronome that communicates rhythm through vibration patterns, not sound.
BLE for tempo control from phone. Useful for drummers, dancers, accessibility. (Pull B)

### Tier 2: Weekend builds (multiple tasks, real interaction design)

**"Draw a soundscape"**
A surface where strokes/gestures create musical structures. Touch/pressure sensors → MIDI
CC to laptop synth. Visuals mirror the sound. Synesthesia category.
- Reference: https://ocw.mit.edu/courses/21m-385-interactive-music-systems-fall-2016/pages/assignments/

**"Body-as-conductor"**
Wearable IMU(s) → gesture classification → arrangement/orchestration control. Your body
language shapes intensity, spatial placement, instrument balance. AI fills in arrangement
detail (laptop-side). The embedded part is sensing + haptic feedback.
- Reference: https://musictech.mit.edu/imsprj/controllers

**"USB MIDI instrument with personality"**
Knobs/keys/pads → USB MIDI, but with LED feedback, mode switching, preset storage, and
haptic detents. The "personality" comes from the mapping layer — how inputs combine, what
non-obvious interactions exist, how the instrument rewards exploration.
Embassy-usb MIDI class is first-class for this. (Pull A)

### Tier 3: Multi-week projects (systems, not devices)

**"Room-scale interactive composition"**
People moving through a space create/alter musical layers. Position → musical parameter
(proximity sensors, UWB ranging, or camera tracking). Spatial audio makes it legible.
Embedded: distributed sensor nodes + LED feedback. Laptop: synthesis + spatial rendering.
- Reference: https://spatialsoundlab.mit.edu/

**"Online jam that embraces latency"**
Instead of fighting network latency, design for call/response, phase, canon, and layered
loops. Latency becomes a musical parameter, not a bug. Gateway reliability (backoff,
buffering) is the enabling infrastructure. (Pull C)
- Reference: https://musictech.mit.edu/imsprj/online

**"Standalone granular instrument"**
Self-contained on Daisy Seed: mic/line in → granular engine → audio out. Physical controls
for grain size, density, pitch, position. No laptop. This is the most technically ambitious
embedded music project. (Pull D)
- Reference: Sitira https://github.com/backtail/sitira-embedded-daisy


## 5. The AI Angle in IMS (laptop-side, not embedded)

AI-as-co-performer is an active IMS research direction (Bodies In Score uses LLMs for
scoring). But the inference runs on a laptop or cloud — this doesn't change the embedded
hardware story. What it does change:

**The embedded controller becomes the *interface to AI*:**
- Gesture → "prompt" (not text boxes — embodied controls: movement, breath, pressure)
- AI generates arrangement/accompaniment (laptop-side)
- Embedded device provides real-time feedback (haptic confirmation, LED state indication)
- The constraint: AI is only musically satisfying when it respects **latency +
  synchronization + continuity** — the IMS scheduling/sync layer matters

**Interesting design space:** personalized mapping where the system learns *your* movement
vocabulary and makes it expressive faster. The embedded side captures + transmits the
gesture data; the laptop side does the learning.

References:
- Bodies In Score (gesture + LLM): https://musictech.mit.edu/imsprj/controllers
- MIT Music Tech research: https://musictech.mit.edu/projects/


## 6. Spatial Audio as a Medium (not just "better speakers")

MIT Spatial Sound Lab: 14.2 speaker system, d&b Soundscape (object-based mixing), research
on "interactive spatial design" and social possibilities of immersive audio for public
performance.

What this means for interaction design:
- Output layer becomes **spatial objects** you can move/reshape in real time
- Interaction can be **position-based**: audience location changes what they hear
- Multi-user experiences become more legible: different groups "own" different sound regions

This is aspirational for personal exploration (you need a speaker array), but it's the
"north star" for room-scale interactive compositions. At smaller scale: even a 4-speaker
setup with OSC-controlled panning gives a taste of spatial interaction.

Reference: https://spatialsoundlab.mit.edu/


## 7. Practical "IMS Enablement Stack" (minimum viable)

To explore IMS-style systems without a massive setup:

### Laptop-side (the engine)

IMS historically uses Python (Kivy + numpy + PyAudio + FluidSynth + rtmidi). Modern
alternatives:
- **SuperCollider** — proven real-time synthesis + OSC
- **Pure Data (Pd)** — visual patching, great for prototyping mappings
- **Web Audio API** — if targeting audience participation (Tutti-style)
- **Rust (cpal + fundsp/dasp)** — if you want the whole stack in Rust
- **Ableton Live + Max for Live** — fastest path to polished sound

### Embedded-side (sensing + feedback)

→ See `rust-embedded-futures-research-notes.md` Section 11 (buy list) and Section 13
  (toolchain). The same hardware covers IMS controllers:

- RP2040 + buttons/encoders/IMU → USB MIDI/HID (Pull A)
- nRF52840 + IMU + haptic motor → BLE MIDI (Pull B)
- ESP32-S3 + sensors → OSC over Wi-Fi (Pull C)
- Daisy Seed → standalone audio instrument (Pull D, if pursuing)

### Bridge protocol

- **USB MIDI** — lowest latency, most universal, works with everything
- **BLE MIDI** — wireless, higher latency (~10-20 ms), good for wearables
- **OSC over Wi-Fi** — flexible, higher latency, good for multi-device
- **USB HID** — for non-musical controllers (custom gamepad-as-instrument)

### One "weird" controller

The thing that makes an IMS project interesting vs generic. At minimum, one input that isn't
a keyboard/mouse:
- IMU (tilt/gesture) — included in buy list
- Pressure sensor / FSR — ~$2 each
- Flex/stretch sensor — ~$5-10
- MEMS microphone — included in buy list
- Capacitive touch — built into many MCUs (ESP32, nRF)

### Playtest loop

IMS's meta-enabler: **get a working prototype quickly, then learn through playtesting.**
Put it in someone's hands. Watch what they try. Watch what confuses them. Iterate.
This is the "agent-assisted NRE collapse" applied to interaction design.


## 8. Four IMS Spikes (experience-oriented)

Same principle as embedded spikes: one evening each, comparable structure, fill in scorecard.

### Spike I: "Instant instrument" (Pull A hardware)

Build the simplest possible controller that a non-musician can play meaningfully.

- Hardware: RP2040 + button + IMU + USB MIDI
- Laptop: SuperCollider or Ableton receiving MIDI
- Mapping: tilt → pitch, button → note trigger, shake → vibrato
- Measure: time-to-first-note for naive user, expressivity range, "is it fun?"

**Done when:**
- [ ] Someone who hasn't seen it before makes music within 30 seconds
- [ ] USB MIDI latency < 5 ms (measured at laptop)
- [ ] At least 3 distinct expressive gestures map to audible differences

### Spike II: "Haptic musician" (Pull B hardware)

Build a wearable that communicates musical information through touch, not sound.

- Hardware: nRF52840 + haptic motor + IMU + BLE
- Laptop: sends tempo/chord changes via BLE
- Mapping: pulse patterns for downbeat vs upbeat, intensity for dynamics
- Measure: can a blindfolded user tap along? Can they anticipate changes?

**Done when:**
- [ ] BLE connection survives 10 minutes of continuous use
- [ ] User can distinguish at least 3 different haptic patterns with eyes closed
- [ ] Latency from laptop trigger → haptic onset < 30 ms

### Spike III: "See the room" (Pull A + C hardware)

Build an audio-reactive visual system that responds to live music in a room.

- Hardware: RP2040 + MEMS mic + WS2812 LED strip/ring
- Mapping: volume → brightness, spectral balance → hue, transients → flash
- No laptop needed (self-contained)
- Measure: does it feel responsive? Does it add to the music or distract?

**Done when:**
- [ ] LEDs respond visibly to music with no perceptible delay
- [ ] Different genres produce noticeably different visual behavior
- [ ] Someone says "that's cool" (the IMS playtest criterion)

### Spike IV: "Standalone sound" (Pull D hardware)

Build the simplest standalone audio instrument — no laptop.

- Hardware: Daisy Seed + knob + button + audio out
- Firmware: simple oscillator or sample playback, knob controls parameter
- Measure: audio quality, latency from knob turn → audible change, buffer underruns

**Done when:**
- [ ] Clean audio output (no clicks, pops, or buffer underruns)
- [ ] Knob → audible change < 5 ms
- [ ] Runs for 1 hour without glitches

Note: Spike IV requires Daisy Seed (~$30) which is different hardware from the other pulls.
Only pursue if audio DSP specifically calls to you.


## 9. Key Measurements for IMS (what makes it "good")

Unlike pure embedded metrics (current draw, binary size), IMS cares about **perceptual**
metrics:

| Metric | Target | Why it matters |
|---|---|---|
| Input → audible output latency | < 10 ms | Above ~20 ms, musicians feel "disconnected" |
| Input → visual output latency | < 30 ms | Visual is more forgiving than audio |
| Haptic → perception latency | < 30 ms | Tactile is similar to visual tolerance |
| BLE MIDI round-trip | < 20 ms | Playable but noticeable; USB is better |
| Clock sync drift (multi-device) | < 1 ms | For ensemble/audience participation pieces |
| Time-to-first-note (new user) | < 30 sec | IMS "learnability" criterion |
| Expressivity (distinct gestures) | 3+ meaningful | IMS "depth" criterion |
| "Is it fun?" (playtest) | yes/no | The only metric that really matters |


## 10. Research Directions

### Active MIT IMS ecosystem

- IMS course + projects: https://musictech.mit.edu/ims/
- Controller projects: https://musictech.mit.edu/imsprj/controllers
- Hardware projects: https://musictech.mit.edu/imsprj/hardware
- Online projects: https://musictech.mit.edu/imsprj/online
- VR projects: https://musictech.mit.edu/imsprj/vr
- Tutti (massive participation): https://musictech.mit.edu/tutti
- *12* (audience-as-performer): https://musictech.mit.edu/twelve
- VR SandBox: https://musictech.mit.edu/vr-sandbox
- Spatial Sound Lab: https://spatialsoundlab.mit.edu/
- Research/thesis projects: https://musictech.mit.edu/projects/
- Spaces/labs: https://musictech.mit.edu/space-labs/

### IMS-relevant Rust embedded crates

→ Full list in `rust-embedded-futures-research-notes.md` Section 18

Key ones for IMS:
- embassy-usb MIDI class: https://docs.embassy.dev/embassy-usb/git/default/class/index.html
- usbd-midi: https://github.com/rust-embedded-community/usbd-midi
- daisy-embassy: https://crates.io/crates/daisy-embassy
- smart-leds-rs: https://github.com/smart-leds-rs
- microdsp (no_std DSP): https://github.com/stuffmatic/microdsp
- synth-utils: https://crates.io/crates/synth-utils
- trouble-host (BLE): https://crates.io/crates/trouble-host

### Laptop-side audio tools

- SuperCollider: https://supercollider.github.io/
- Pure Data: https://puredata.info/
- Web Audio API: https://developer.mozilla.org/en-US/docs/Web/API/Web_Audio_API
- cpal (Rust audio I/O): https://crates.io/crates/cpal
- fundsp (Rust DSP): https://crates.io/crates/fundsp
- Rust audio programming overview: https://andrewodendaal.com/rust-audio-programming-ecosystem/

### Open questions

- [ ] What's the actual BLE MIDI latency with `trouble` on nRF52840?
- [ ] SuperCollider vs Pure Data for rapid IMS prototyping — which has better OSC/MIDI?
- [ ] Daisy Seed availability and `daisy-embassy` maturity?
- [ ] Can ESP32-S3 do adequate audio I/O for simple standalone instruments, or is Daisy
      the minimum viable audio platform?
- [ ] What's the simplest multi-device clock sync approach (for ensemble pieces)?
- [ ] OSC crate for no_std Rust — does one exist?
