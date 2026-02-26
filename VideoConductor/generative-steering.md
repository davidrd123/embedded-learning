# Conducting Generative Systems

## The Core Distinction

Three modes of human-system interaction in generative contexts:

**Authoring**: Direct specification. You write the words, draw the pixels, play the notes. The system executes faithfully.

**DJing / Curation**: Selection from pre-existing material. You choose which clip plays, which loop activates, which version renders. The system holds a library; you navigate it.

**Conducting**: Parametric authority over execution. The system contains generative capacity constrained by preparation. You don't choose content—you shape how inevitability unfolds.

The distinction isn't interface (knobs vs gestures vs text). It's the direction of responsibility:

- DJing: performer decides content, system executes
- Conducting: system contains content, performer decides interpretation

Most "generative" tools collapse into DJing because it's easier to demo and faster to learn. True conducting requires preparation, trust, and the willingness to relinquish note-level control.

---

## The Mapping Across Domains

### Video Conductor (Real-time generative video)

| Layer | What's Pre-Composed | What's Performed |
|-------|---------------------|------------------|
| Base model | Visual vocabulary, coherence priors, style space | - |
| LoRA preparation | Aesthetic tendencies, character consistency | - |
| Brief/prompt structure | Scene grammar, subject constraints | - |
| Control signals | - | Depth conditioning, pose, timing |
| Style blend | - | LoRA weight modulation |
| Transition triggers | - | Hard cuts, soft morphs, holds |

The performer doesn't choose pixels. They shape energy, timing, emphasis. The base model + LoRAs + conditioning create a "high-dimensional latent space with strict constraints." Performance is navigation, not selection.

### Loom (Text generation with externalized selection)

| Layer | What's Pre-Composed | What's Performed |
|-------|---------------------|------------------|
| Base model | Language prior, token distributions | - |
| Brief | Voice, register, lean_into, avoid | - |
| Few-shot examples | Texture, rhythm, density | - |
| Section intent | Local direction, thematic constraint | - |
| Candidate generation | - | (delegated to base model) |
| Branch selection | - | Choose / clarify / stop |
| Path commitment | - | Which inevitability unfolds |

The selector doesn't write words. They choose which of the model's offerings to commit to. The brief + examples constrain the space; selection shapes the realization.

### Audio Conductor (Gestural music steering)

| Layer | What's Pre-Composed | What's Performed |
|-------|---------------------|------------------|
| Harmonic field | Chord affordances, voice leading rules | - |
| Rhythmic grammar | Meter, density ceilings, articulation | - |
| Orchestration | Instrument roles, coupling rules | - |
| Tempo/beat | - | Downbeat arc, phrase boundaries |
| Energy envelope | - | Intensity, density modulation |
| Spatial orchestration | - | Left-right, high-low, near-far |
| Silence | - | Holds, fermatas, breath |

The conductor doesn't choose notes. They modulate which affordances activate and how energy flows between sections.

---

## Preparation as the Actual Work

In all three domains, the ratio of preparation to performance is high. This is uncomfortable for demos but essential for the conducting experience.

### What preparation looks like:

**Video**: Training LoRAs, establishing control map semantics, building prompt playlists, testing style blend behavior, defining transition grammar.

**Text/Loom**: Writing the brief (voice, register, lean_into, avoid), curating few-shot examples, establishing section intents, potentially extracting style samples from source texts.

**Audio**: Defining harmonic fields that evolve without collapsing, creating instrument voices that respond to intensity rather than commands, building rhythmic layers that tighten/loosen without changing meter, ensuring transitions are continuous rather than discrete.

The preparation is compositional, but it composes *degrees of freedom*, not fixed content. The score is not notes—it's constraints.

### Style seeding as orchestration

For text, "style seeding" means extracting characteristic patterns from source material (e.g., Borges) and using them to condition generation. This isn't copying phrases. It's defining:

- Syntactic tendencies (sentence rhythm, clause structure)
- Image density preferences
- Register markers
- Compression level
- What kinds of incompleteness are permitted

These function like "articulation tendencies" in an orchestration. The model still generates; the conditioning shapes how.

---

## Multi-Register as Conducting vs DJing

Consider the multi-register blog idea: same content, different stylistic realizations, reader controls the blend.

**DJing approach**: Write N versions, store them, let reader toggle between pre-written variants. Clean, efficient, but the versions are fixed artifacts.

**Conducting approach**: Store the content at a more abstract level. Regenerate dynamically based on register parameter. Reader modulates style conditioning; system realizes the content through that lens.

The conducting approach is computationally expensive and harder to control. But it's conceptually cleaner: you're not selecting from a library, you're shaping execution.

A middle path: use the DJing structure (pre-written versions) but create them through a conducting process (looming with different style seeds). The artifact is curated, but the creation process was conducted.

---

## The Loom/Video Conductor Convergence

Both systems externalize the "or...or...or..." structure that normally happens inside generation:

| Video Conductor | Loom |
|-----------------|------|
| Base model proposes frames | Base model proposes tokens |
| Conditioning constrains the space | Brief constrains the space |
| Performer steers through control signals | Selector steers through branch choice |
| Output is committed frame-by-frame | Output is committed node-by-node |
| All unchosen frames disappear | All unchosen candidates remain in the tree |

Key difference: Loom keeps the rejected branches. The tree is a record of the selection process, not just the result. This enables analysis: where did the selector fight the base model? Where were clarifications needed? What alternatives existed at each decision point?

Video Conductor could do this too (record all conditioning + seeds, enable re-render of alternative paths), but the storage cost is higher and the use case is less clear.

---

## The Temporal Structure

Both domains involve multiple timescales operating simultaneously:

### Fast loop (frame-rate / token-rate)
- Video: Immediate control response, pose conditioning
- Text: Single candidate evaluation, micro-rhythm

### Medium loop (seconds)
- Video: Energy envelope, style blend drift, attention focus
- Text: Sentence-level coherence, local tension

### Slow loop (minutes)
- Video: Scene mood, narrative phase, tidal oscillators
- Text: Section intent, thematic arc, voice consistency

### Meta loop (session)
- Video: What gets recorded, what gets re-rendered
- Text: Where to branch, when to stop, what paths to hold

Good conducting involves awareness of all timescales. Bad conducting collapses everything to the fast loop (twitchy) or ignores the fast loop (mushy).

---

## Implications for Tooling

### What a "conducting-native" interface needs:

1. **Clear separation between preparation and performance**
   - Preparation mode: define constraints, test affordances
   - Performance mode: only parametric authority, no content selection

2. **Continuous control channels**
   - Not just discrete triggers
   - Gestures, sliders, embodied input
   - VR excels here because it naturally produces rhythm and intensity variation

3. **Visible constraint state**
   - What brief is active? What LoRAs are blended?
   - The performer should feel the boundaries of the space

4. **Delayed and distributed feedback**
   - Conducting involves non-local consequences
   - An action here affects something there, later
   - The system needs to communicate this causality

5. **Graceful degradation**
   - If the performer stops, the system should settle, not crash
   - Silence / stillness as first-class states

### What most tools get wrong:

- Exposing too many parameters (becomes authoring)
- Discrete selection UI (becomes DJing)
- No preparation layer (everything is live, nothing is constrained)
- Immediate feedback only (loses the slow loops)

---

## The Shared Project

Across Video Conductor, Loom, and potential audio conducting:

**You are building instruments for steering generative systems through constraint and selection rather than direct specification.**

The instrument is not the model. The instrument is the constraint layer + the selection interface + the feedback loop that lets a human shape how inevitability unfolds.

This is underexplored because:
- It requires patience and pre-composition discipline
- It's harder to demo than DJing
- It demands trust in the system
- The payoff is experiential, not immediately visible

But when it works, it produces something that feels *alive in the way orchestras feel alive*—not because every note was chosen, but because every note was shaped.

---

## Practical Next Steps

### For Video Conductor
- Document the control map semantics as "orchestration"
- Make the LoRA blend weights feel like "voice roles"
- Consider what "clarify" would mean (pause generation, ask for direction?)

### For Loom
- Implement style seeding as brief extension
- Build the selector modes (human, stateless LLM, agentic)
- Consider what real-time looming would look like (fast enough to feel like conducting?)

### For Audio (if pursued)
- Start with one "open orchestration" score as experiment
- Map VR gestures to conductor-layer invariants (tempo, density, tension)
- Keep note-level agency away from the performer

### For the Blog
- Decide: DJing (pre-written versions) or conducting (dynamic regeneration)?
- If DJing, use a conducting process to create the versions
- The multi-register toggle is the reader's conducting interface, however simple
