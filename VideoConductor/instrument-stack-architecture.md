# The Instrument Stack: Architecture for Conducting Generative Systems

## The Unifying Pattern

Across video, text, and audio, one design pattern emerges:

**Prepared constraint space + continuous authority + delayed consequences + logged alternatives.**

Or:
- **Preparation** builds the boundaries of the world (style priors, grammars, roles, transition rules)
- **Performance** navigates those boundaries with continuous signals (not "choose option 7" but "increase tension")
- **Feedback** makes the boundaries legible (you feel what is allowed)
- **Memory** preserves counterfactuals (trees, seeds, control streams) so the system becomes analyzable and rehearsable

The question becomes: how do we make the system contain enough structure that "interpretation" feels real, while retaining enough openness that "performance" matters?

---

## The Five-Part Architecture

### A. The Score (Prepared Constraints)

Not content, but degrees of freedom. A graph of:

- **Roles**: voices, characters, camera, texture layers, narrative functions
- **Allowed transformations**: what "tension up" means, what "density up" means
- **Grammars**: rhythm grammar, scene grammar, sentence rhythm grammar
- **Invariants**: tempo grid, character identity, voice consistency, register boundaries
- **Transition operators**: morph, cut, hold, dissolve, re-orchestrate, clarify

### B. The Conductor (Live Authority)

A low-dimensional state vector that updates continuously:

| Variable | Description |
|----------|-------------|
| Energy | Overall intensity, drive |
| Tension | Unresolved pressure, anticipation |
| Density | How much is happening, compression |
| Clarity | Focus vs diffusion, concrete vs abstract |
| Novelty | Surprise, deviation from expectation |
| Tempo | Pacing, rhythm of events |
| Phase | Position in larger arc |
| Hold | Suspension, fermata |
| Focus | Spatial/conceptual attention locus |

The conductor does not emit content. It emits **pressure** on the ensemble.

### C. The Ensemble (Distributed Agents)

Instead of one monolith generating everything:

**Section leaders** (mid-level agents):
- Drums leader, harmony leader, camera leader, transition leader
- Or for text: voice leader, rhythm leader, image leader, structure leader

**Players** (micro agents):
- Hi-hat generator, bassline generator, environment texture generator
- Or for text: clause generator, metaphor generator, transition phrase generator

Each agent is conditioned on:
- The conductor state
- Local context (what just happened)
- The score constraints for its role

The conductor's job: **allocate attention and shape macro arcs**, not write phrases.

### D. The Stage (Renderers)

The output layer: Unity, Unreal, diffusion video, DAW, audio engine, text renderer.

Important: the stage is not the brain. It executes what the ensemble produces under conductor pressure.

### E. The Log (Rehearsal and Counterfactuals)

The Loom insight generalized:

- In text: the tree of chosen and rejected candidates
- In video: seeds + prompts + control signals + constraint keyframes
- In audio: event stream + conductor stream + agent decisions

The log turns performance into something inspectable and evolvable. You can replay, rerun with different preparation, analyze where you fought the system.

---

## The Conductor State Vector

Concrete specification for what "continuous authority" means:

```
conductor_state = {
    energy: float,      # 0.0 to 1.0, overall intensity
    tension: float,     # 0.0 to 1.0, unresolved pressure
    density: float,     # 0.0 to 1.0, how much is happening
    clarity: float,     # 0.0 to 1.0, focus vs diffusion
    novelty: float,     # 0.0 to 1.0, surprise level
    tempo: float,       # events per unit time
    phase: float,       # 0.0 to 1.0, position in arc
    hold: bool,         # suspension active
    focus: vector,      # attention locus (spatial or conceptual)
}
```

These variables are computed from input signals (gestures, selections, time) across multiple timebases:

- **Fast loop** (ms): immediate gesture, current selection pressure
- **Medium loop** (seconds): integrated energy, smoothed state
- **Slow loop** (minutes): tidal oscillators, arc position
- **Session loop**: what's remembered, what becomes motif

---

## Budgets and Contracts

The key to delegation without chaos:

> The conductor does not tell agents what to play. The conductor gives each agent:
> - A **budget** (how much presence, how much novelty)
> - A **target** (current tension, density)  
> - A **contract** (rules it must obey, things it must never do)
>
> Then the agent fills the budget inside the contract.

This is analogous to an orchestra: you don't tell the clarinet which notes. You shape its entrance, weight, and blend.

### Contract Structure

```
agent_contract = {
    role: string,                    # "rhythm", "voice_technical", "imagery"
    invariants: list[constraint],    # must always hold
    forbidden: list[pattern],        # must never produce
    allowed_range: dict,             # bounds on outputs
    coupling: dict[agent -> float],  # how much to attend to other agents
}
```

### Budget Structure

```
agent_budget = {
    presence: float,      # how much space this agent gets
    novelty_allowance: float,  # how much surprise permitted
    density_target: float,     # how compressed its output should be
    autonomy: float,      # how much it can deviate from conductor pressure
}
```

---

## The Open Score Graph

A unified representation for preparation:

```json
{
  "global_state": {
    "energy": 0.0,
    "tension": 0.0,
    "density": 0.0,
    "clarity": 0.0,
    "novelty": 0.0,
    "tempo": 1.0,
    "phase": 0.0,
    "hold": false,
    "focus": null
  },
  "tides": [
    {
      "name": "tension_tide",
      "period_sec": 90,
      "amplitude": 0.3,
      "phase": 0.1,
      "coupling": { "energy": 0.2 }
    }
  ],
  "agents": [
    {
      "name": "voice_leader",
      "role": "register",
      "contract": {
        "invariants": ["no_assistant_hedging", "maintain_compression"],
        "forbidden": ["as_an_ai", "it_is_important_to_note"],
        "register_bounds": ["technical_philosophical", "lyric"]
      },
      "inputs": ["clarity", "tension", "density"],
      "outputs": ["register_bias", "formality_level", "sentence_rhythm"]
    },
    {
      "name": "image_leader",
      "role": "concrete_detail",
      "contract": {
        "invariants": ["prefer_concrete", "load_bearing_images"],
        "forbidden": ["generic_mysticism", "unexplained_abstraction"]
      },
      "inputs": ["density", "novelty", "clarity"],
      "outputs": ["image_density", "abstraction_level", "metaphor_extension"]
    },
    {
      "name": "rhythm_leader",
      "role": "syntax",
      "contract": {
        "invariants": ["syntactic_pull", "vary_clause_length"],
        "forbidden": ["filler_phrases", "throat_clearing"]
      },
      "inputs": ["energy", "tempo", "hold"],
      "outputs": ["sentence_length_bias", "clause_complexity", "fragment_allowance"]
    },
    {
      "name": "structure_leader",
      "role": "transitions",
      "contract": {
        "invariants": ["no_premature_resolution"],
        "operators": ["continue", "pivot", "deepen", "surface", "close"]
      },
      "inputs": ["phase", "tension", "hold"],
      "outputs": ["transition_type", "paragraph_boundary", "section_signal"]
    }
  ]
}
```

---

## Application to Text/Loom

### Current Loom Architecture

```
Brief → Base Model → Candidates → Selector → Chosen Path
                                     ↓
                                   Tree (Log)
```

The selector chooses among monolithic candidates. Each candidate is a complete segment generated by one process.

### Ensemble Loom Architecture

```
Brief (Score) → Conductor State → Agent Ensemble → Candidates → Selector → Chosen Path
                      ↑                                              ↓
                 Selection History                                 Tree (Log)
                      ↑                                              ↓
                 Tidal Oscillators ←──────────────────────────── Phase Update
```

Now:
- The **Brief** becomes a **Score**: not just constraints but role definitions, agent contracts, transition operators
- The **Conductor State** is computed from selection history + tides + explicit input
- **Agents** generate candidates within their contracts, responding to conductor pressure
- The **Selector** chooses, but selection also updates conductor state (tension release, phase advance)
- **Tides** provide slow structure independent of moment-to-moment selection

### What Changes in Practice

**Candidate Generation**:

Instead of: "Generate 8 continuations"

Now: "Given conductor state {tension: 0.7, density: 0.4, clarity: 0.8}, agents produce candidates within their contracts"

- Voice agent biases toward compressed, precise register
- Image agent increases concrete detail, reduces abstraction
- Rhythm agent shortens clauses, increases pull
- Structure agent signals "deepen" not "pivot"

**Selection Feedback**:

Instead of: "Choice made, extend path"

Now: "Choice made → update conductor state"

- If selector chose high-tension candidate: tension stays high or increases
- If selector chose resolution: tension decreases, phase advances
- If selector clarified: hold activates, phase pauses
- Energy integrates from selection pace (fast decisions = high energy)

**Tidal Influence**:

Even without active selection, the system drifts:

- Tension tide at period 90s: slow build and release independent of content
- Novelty tide at period 240s: gradual increase in surprise allowance, then reset

This gives the text "inevitability" - a sense of larger forces even when the selector is just responding locally.

---

## The Selector as Conductor

Reframing what selection means:

**Old model**: Selector evaluates candidates against criteria, picks best.

**New model**: Selector is a conductor interface. Each selection is a gesture that:
- Commits content (extends the path)
- Updates conductor state (shapes future generation)
- Advances phase (moves through the arc)
- Optionally triggers operators (hold, transition, clarify)

The selection interface could expose conductor state explicitly:

```
Current state: tension=0.7, density=0.4, clarity=0.8, phase=0.3
Candidates:
  [1] "The hand—" (tension+, density+, clarity=)
  [2] "What remains" (tension=, density-, clarity+)
  [3] "Between the" (tension-, density=, clarity-)
  
Select [1-3], or:
  [h] hold (pause phase, sustain current)
  [t] transition (signal section boundary)
  [c] clarify (surface interpretive fork)
```

Now selection is legibly conducting: you see the pressure you're applying.

---

## Multi-Voice as Multi-Agent

The earlier concept of "multi-voice looming" becomes concrete:

Instead of one voice with variants, you have agents with roles:

```json
{
  "agents": [
    {
      "name": "technical_voice",
      "contract": { "register": "precise", "compression": "high", "imagery": "mechanical" },
      "presence_default": 0.4
    },
    {
      "name": "lyric_voice", 
      "contract": { "register": "intimate", "compression": "medium", "imagery": "organic" },
      "presence_default": 0.3
    },
    {
      "name": "questioning_voice",
      "contract": { "register": "open", "completion": "low", "pull": "high" },
      "presence_default": 0.3
    }
  ]
}
```

Conductor state modulates presence:
- High clarity → technical voice presence increases
- High tension → questioning voice presence increases
- Low energy → lyric voice dominates

The text becomes polyphonic not through explicit switching but through pressure-modulated blend.

---

## Real-Time Text Conducting

Speculative but now more concrete:

If selection is fast enough (sub-second), and conductor state updates continuously, text emerges at performance pace.

Requirements:
- Streaming generation (agents produce continuously)
- Lightweight selection (recognition, not deliberation)
- Visible conductor state (performer feels the pressure field)
- Tidal backbone (the system breathes even when performer pauses)

Interface could be gestural:
- Hand height = tension
- Hand speed = energy  
- Stillness = hold
- Forward motion = advance phase
- Spread = increase density

Or could be keyboard-native:
- Vim-like single-key selection
- Modifier keys adjust conductor state
- Space = hold
- Enter = transition

The text scrolls as you conduct it into existence.

---

## The Log as Instrument Memory

What gets logged in ensemble architecture:

```json
{
  "timestamp": 1234567890,
  "conductor_state": { "tension": 0.7, "density": 0.4, ... },
  "agent_states": {
    "voice_leader": { "register_bias": 0.8, "output": "..." },
    "image_leader": { "density": 0.6, "output": "..." }
  },
  "candidates": [ ... ],
  "selection": { "chosen": "candidate_2", "reason": "..." },
  "state_delta": { "tension": -0.1, "phase": +0.05 }
}
```

This enables:
- **Replay**: Same conductor stream, different agents → different text
- **Analysis**: Where did tension peak? Where did you fight the agents?
- **Rehearsal**: Adjust contracts, rerun, compare
- **Motif extraction**: Find patterns in what you selected, feed back into score

The log is how the instrument learns your taste.

---

## Implementation Stages

### Stage 1: Conductor State Without Agents

Add conductor state tracking to existing Loom:
- Compute state from selection history (tension from recent choices, energy from pace)
- Display state to selector
- Log state with each decision

No generation changes yet. Just making the conducting layer visible.

### Stage 2: State-Conditioned Generation

Modify the generator to accept conductor state:
- Brief includes state-dependent clauses ("at high tension, prefer incomplete syntax")
- Or: multiple few-shot example sets keyed to state regions
- Candidates vary based on current state

Single generator still, but state-aware.

### Stage 3: Agent Separation

Split generation into agents:
- Voice agent, image agent, rhythm agent
- Each has contract, responds to relevant state variables
- Candidates are agent outputs composed or offered separately

Now you're conducting an ensemble.

### Stage 4: Tides and Autonomy

Add tidal oscillators:
- Slow phase cycles independent of selection
- Agents can have autonomy parameter (how much they drift without conductor input)

The system becomes alive even when you pause.

### Stage 5: Rehearsal Tools

Build the analysis layer:
- Visualize conductor state over session
- Compare runs with different scores/contracts
- Extract patterns from selection history
- Feed patterns back into preparation

The instrument becomes self-improving.

---

## The Hardest Problems

### Legible Causality

The performer must feel: "That phrase happened because I increased tension."

Requires:
- Consistent mappings (same gesture, same effect)
- Slow changes (state has inertia)
- Visible constraints (you see what's allowed)
- Limited degrees of freedom per gesture

### Coherence Across Time

Multiple agents can drift apart. Requires:
- Shared clock (agents sync to tempo)
- Shared memory (motifs available to all)
- Explicit transition operators
- Contracts that prevent identity collapse

### Preparation UX

"Preparation is the work" is true, but must be enjoyable.

The preparation environment should feel like:
- Composing affordances (not parameters)
- Rehearsing responses (not debugging)
- Sculpting contracts (not filling forms)

---

## Synthesis

The instrument stack is:

1. **Score**: Prepared constraints (brief, contracts, operators)
2. **Conductor**: Low-dimensional continuous authority
3. **Ensemble**: Distributed agents with budgets and contracts
4. **Stage**: Renderers (text output, but also video, audio)
5. **Log**: Preserved counterfactuals and state history

This applies uniformly to:
- Video Conductor (LoRA agents, camera agents, transition agents)
- Loom (voice agents, image agents, rhythm agents)
- Audio Conductor (rhythm agents, harmony agents, texture agents)

The conducting experience emerges from:
- Constraints that are tight enough to make interpretation meaningful
- Authority that is continuous enough to feel embodied
- Feedback that is delayed enough to create consequence
- Memory that preserves enough to enable rehearsal

You are not authoring content. You are not selecting from a library. You are shaping how constrained inevitability unfolds.

That is conducting.
