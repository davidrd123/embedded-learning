# Conductor Project: Landscape and Integration Map

## The Core Thesis

The components for "conducted generative systems" exist. What's missing is the integration philosophy:

- Strict separation of preparation mode vs performance mode
- Explicit multi-timescale control (fast, medium, tides)
- True conductor interface (interpretation authority, not content selection)
- Full logging and replay of counterfactuals (Loom-style tree for time-based media)
- Cross-modal coupling (audio and video driven by same conductor state vector)

This document maps existing work to our architecture and identifies what needs to be built vs borrowed.

---

## Architecture Review

```
┌─────────────────────────────────────────────────────────────────┐
│                         PREPARATION                              │
│  Score: roles, contracts, grammars, invariants, operators        │
└─────────────────────────────────────────────────────────────────┘
                              ↓
┌─────────────────────────────────────────────────────────────────┐
│                         PERFORMANCE                              │
│                                                                  │
│  ┌──────────┐    ┌──────────────┐    ┌──────────────┐           │
│  │ Control  │───▶│  Conductor   │───▶│   Ensemble   │           │
│  │  Input   │    │    State     │    │   (Agents)   │           │
│  └──────────┘    └──────────────┘    └──────────────┘           │
│   Quest/VR         energy              drums_agent              │
│   gestures         tension             harmony_agent            │
│   MIDI ctrl        density             camera_agent             │
│                    clarity             style_agent              │
│                    novelty             transition_agent         │
│                    tempo                                        │
│                    phase                                        │
│                    hold                                         │
│                    focus                                        │
│                                                                  │
│  ┌──────────────┐                    ┌──────────────┐           │
│  │    Tides     │───────────────────▶│    Stage     │           │
│  │ (slow arcs)  │                    │  (renderers) │           │
│  └──────────────┘                    └──────────────┘           │
│                                        MIDI→synth              │
│                                        prompts→diffusion        │
│                                        params→DAW               │
└─────────────────────────────────────────────────────────────────┘
                              ↓
┌─────────────────────────────────────────────────────────────────┐
│                           LOG                                    │
│  conductor stream + agent decisions + outputs + counterfactuals  │
└─────────────────────────────────────────────────────────────────┘
```

---

## Existing Work Mapped to Architecture

### 1. Conducting Grammar (What gestures mean)

| Resource | What it provides | Integration point |
|----------|------------------|-------------------|
| **Soundpainting** | Who/What/How/When grammar for live composition | Model for conductor vocabulary design |
| **Conduction (Butch Morris)** | Gestural direction vocabulary | Alternative grammar to study |

**Study priority**: High. Soundpainting's structure maps directly to: Who (which agent), What (what transformation), How (intensity/quality), When (timing/phrase boundary).

**Open questions for 5pro**:
- Is there a canonical Soundpainting reference/manual?
- How large is the gesture vocabulary in practice?
- How do conductors learn it?

---

### 2. Gesture Capture and Mapping (Control input → conductor state)

| Resource | What it provides | Integration point |
|----------|------------------|-------------------|
| **Wekinator** | ML-based gesture → control mapping | Train custom mappings from Quest input |
| **MiMU Gloves / Glover** | Movement → MIDI/OSC, scene switching | Reference for mapping design |
| **Radio/Digital Baton** | Historical conducting interfaces | Conceptual ancestor |
| **VMC Protocol** | OSC-based avatar motion transport | Quest → control bus bridge |
| **VRChat OSC** | Avatar parameter read/write | Alternative transport |

**Study priority**: Medium-high. VMC is probably the right transport layer.

**Open questions for 5pro**:
- VMC latency in practice? (need <50ms for conducting feel)
- Does VMC carry acceleration/velocity or just position?
- Best VMC-compatible software for Quest 3S specifically?

---

### 3. Multi-Agent Ensemble (Distributed generation)

| Resource | What it provides | Integration point |
|----------|------------------|-------------------|
| **IRCAM Dicy2** | Interactive ML agents for music sequences | Reference for agent design |
| **IRCAM Somax2** | Multi-agent co-improvisation system | Reference for coherence mechanisms |
| **Antescofo** | Score following, time synchronization | Shared clock / phrase boundary model |

**Study priority**: High. These are the closest existing implementations of "conducted agents."

**Open questions for 5pro**:
- How does Somax2 maintain coherence between agents?
- What's Somax2's equivalent of "contracts"?
- Does Antescofo expose a time model we could use for tides?
- Are these Max-only or do they have Python/OSC interfaces?

---

### 4. Real-Time Generative Backends (Stage layer)

#### Audio

| Resource | What it provides | Integration point |
|----------|------------------|-------------------|
| **Magenta RT** | Open-weights live music model | Steerable audio generation |
| **Lyria RealTime** | Google's live music API | Higher quality, cloud dependency |
| **Infinite Crate** | DAW plugin for Lyria | Reference for DAW integration |
| **MusicVAE / GrooVAE** | Latent space music models | MIDI-level generation |

**Study priority**: High for Magenta RT (open weights, local).

**Open questions for 5pro**:
- Magenta RT latency in practice?
- Does it accept continuous conditioning or just prompts?
- Can it be steered frame-by-frame or only at chunk boundaries?

#### Video

| Resource | What it provides | Integration point |
|----------|------------------|-------------------|
| **StreamDiffusion** | Pipeline for real-time diffusion | Core video generation |
| **StreamDiffusionV2** | Improved streaming system | Likely better latency |
| **MirageLSD (Decart)** | Live-stream diffusion video | Alternative approach |

**Study priority**: Medium (video is secondary to audio for first prototype).

**Open questions for 5pro**:
- StreamDiffusion actual FPS with LoRA?
- Can LoRA weights be interpolated in real-time?
- What's the conditioning interface? (Just img2img or richer?)

---

### 5. VR Motion Transport (Quest → system)

| Resource | What it provides | Integration point |
|----------|------------------|-------------------|
| **VMC Protocol** | Standardized motion over OSC | Primary transport candidate |
| **VRChat OSC** | Avatar parameter streaming | Alternative transport |
| **VSeeFace** | Avatar puppeteering software | VMC-compatible receiver |

**Study priority**: Medium. Need to verify Quest 3S → VMC path.

**Open questions for 5pro**:
- What's the Quest 3S → VMC pipeline? (Native app? Sideload?)
- Does Quest hand tracking export via VMC or only controller pose?
- Latency measurements if available?

---

## What Needs to Be Built (The Integration Layer)

### 1. Conductor State Computer

Takes raw input (VMC/OSC), computes conductor state vector across timebases.

```python
class ConductorState:
    # Fast (direct from input)
    energy: float      # movement intensity
    hold: bool         # stillness threshold
    focus: Vec3        # attention locus
    
    # Medium (integrated/smoothed)
    tension: float     # accumulated from gesture patterns
    density: float     # recent activity level
    clarity: float     # smoothed from directness
    
    # Slow (tidal)
    phase: float       # position in arc
    novelty: float     # drift from stability
    
    # Tempo (extracted)
    tempo: float       # from periodic gesture detection
```

**Inputs**: Raw pose stream (VMC), explicit triggers (MIDI controller buttons)
**Outputs**: Conductor state @ ~60Hz, structural events (phrase boundary, hold, transition)

### 2. Agent Contracts and Budget System

Specification format for agents:

```yaml
agent:
  name: harmony_leader
  role: harmonic_content
  
  contract:
    invariants:
      - key_center: "from_score"
      - voice_leading: "smooth"
    forbidden:
      - parallel_fifths
      - tritone_without_resolution
    allowed_range:
      density: [0.2, 0.9]
      register: [36, 84]  # MIDI note range
  
  responds_to:
    tension: chord_complexity
    density: voicing_fullness
    clarity: root_position_bias
    hold: sustain_current
  
  outputs:
    - chord_events (MIDI)
    - tension_signal (for other agents)
```

**Not building**: The agents themselves (use Dicy2/Somax2/MusicVAE as backends)
**Building**: The contract layer that constrains and coordinates them

### 3. Tidal Oscillator System

Slow-timescale arcs that create inevitability:

```python
class Tide:
    name: str
    period_seconds: float
    amplitude: float
    phase: float
    
    # How conductor state nudges the tide
    coupling: dict[str, float]  # e.g., {"energy": 0.2, "stillness": -0.1}
    
    def update(self, conductor_state, dt):
        # Phase advances naturally
        self.phase += dt / self.period_seconds
        
        # Coupling nudges phase/amplitude
        for var, weight in self.coupling.items():
            self.phase += getattr(conductor_state, var) * weight * dt
        
        self.phase %= 1.0
```

### 4. The Log (Performance Recording)

Time-series storage for:
- Conductor state stream (full resolution)
- Agent decisions (what each agent produced, why)
- Structural events (phrase boundaries, transitions)
- Output stream (MIDI events, video frames, or references)

Format: Probably NDJSON with timestamps, queryable for replay.

### 5. Replay/Rehearsal System

- Replay conductor stream through updated agents
- Visualize conductor state over time
- Compare runs with different scores/contracts
- Branch from any point, try alternatives

---

## Proposed Build Order

### Phase 1: Control Bus + State (no generation)

1. Quest 3S → VMC → OSC receiver
2. Conductor state computer (fast + medium timebases)
3. Visualization of conductor state (simple display)
4. Logging of conductor stream

**Test**: Can you "conduct" nothing and feel like the system is tracking you?

### Phase 2: Single Agent + Tides (audio)

1. One tidal oscillator (tension tide, ~90s period)
2. One agent (drums or harmony) constrained by conductor state
3. Agent outputs MIDI → simple synth
4. Tidal influence on agent visible/audible

**Test**: Does it feel like conducting, not DJing?

### Phase 3: Multi-Agent Coherence

1. Add second agent (complementary role)
2. Implement contracts and budgets
3. Shared clock / phrase boundary system
4. Inter-agent coupling (call and response)

**Test**: Do agents feel like an ensemble, not independent generators?

### Phase 4: Rehearsal Loop

1. Full logging implementation
2. Replay interface
3. Contract editing and re-run
4. Visualization of "what the conductor did"

**Test**: Can you improve by iterating, like rehearsing?

### Phase 5: Cross-Modal (add video)

1. Same conductor state → video agents
2. StreamDiffusion integration
3. Audio-video coherence (shared tides, shared phase)

**Test**: Do audio and video feel like one performance?

---

## Study Path (Prioritized)

### Immediate (this week)

1. **Soundpainting documentation** - Understand the Who/What/How/When grammar
2. **Somax2 paper/docs** - How do they maintain agent coherence?
3. **VMC protocol spec** - Can it carry what we need from Quest?

### Near-term (this month)

4. **Magenta RT code/paper** - What's the actual control interface?
5. **Antescofo time model** - How do they handle phrase structure?
6. **Dicy2 architecture** - What's an "agent" in their system?

### Background (ongoing)

7. **StreamDiffusion** - Video backend details
8. **Wekinator tutorials** - Gesture → control training
9. **Compositing class** - (you're enrolled) - Finishing and coherence fundamentals

---

## Open Questions Summary

For 5pro or further research:

**Soundpainting/Grammar**:
- Canonical reference material?
- Size of practical vocabulary?
- Learning path for conductors?

**IRCAM Tools (Somax2/Dicy2/Antescofo)**:
- Coherence mechanisms in Somax2?
- Contract-equivalent in their system?
- Time model in Antescofo?
- Python/OSC interfaces or Max-only?

**Magenta RT**:
- Practical latency?
- Continuous vs discrete conditioning?
- Frame-by-frame steerability?

**VMC/Quest**:
- Quest 3S → VMC pipeline?
- Hand tracking export?
- Measured latency?

**StreamDiffusion**:
- FPS with LoRA?
- Real-time LoRA interpolation?
- Conditioning interface richness?

---

## Key Insight

> Your opportunity is to treat these as modules in one coherent instrument philosophy, not as separate hacks.

We're not building novel generative models. We're building the **integration layer** that makes existing components feel like one instrument:

- One conductor state vector
- One set of contracts
- One log format
- One rehearsal interface

The components exist. The philosophy of integration doesn't.
