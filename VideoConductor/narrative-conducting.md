# Conducting Narrative

## What Authorship Normally Hides

Writing, as typically practiced, collapses selection and generation into a single act. The writer produces a sentence. All the sentences they didn't write vanish without record. The text presents as inevitable, as if it were the only possible sequence.

But every sentence is a choice among possible sentences. Every word forecloses alternatives. The writer experiences this as the cursor blinking, the hesitation before committing, the backspace. The reader sees none of it.

Traditional authorship is conducting with the orchestra invisible, the rehearsals private, the score burned after the premiere.

---

## Three Modes of Textual Production

**Authoring**: You write the words. The constraint is your taste, your knowledge, your stamina. The generation and selection happen together in your head, entangled and invisible.

**Curation/Editing**: You select from existing material. Anthology-making, quotation, cut-up, collage. The material pre-exists; you choose and arrange. This is DJing with text.

**Conducting**: You constrain a generative process and shape its execution without specifying the words. You prepare the space (brief, examples, voice), trigger generation, and select from what emerges. The selection is visible; the alternatives are preserved or at least acknowledged.

Looming is conducting. So is certain collaborative writing, exquisite corpse with rules, constrained writing (Oulipo), and any process where the writer's role is to steer rather than specify.

---

## The Brief as Score

In orchestral music, the score specifies:
- Pitch (which notes)
- Rhythm (when and how long)
- Instrumentation (who plays)
- Dynamics (how loud, with what character)
- Form (sections, repeats, transitions)

It does not specify:
- Micro-timing (rubato, breath)
- Exact dynamic envelopes
- Timbral nuance
- Emotional arc as felt
- Inter-voice balance in the moment

The conductor works in the gap between score and realization.

For text, the brief functions as score. It specifies:
- Voice (register, compression, persona)
- Domain (subject matter, conceptual territory)
- Texture (density, rhythm, what kinds of images)
- Lean-into (what to select for)
- Avoid (what to select against)
- Section intent (local direction)

It does not specify:
- Exact words
- Exact sentences
- Which metaphors
- Which specific images
- How the tension builds and releases

The selector works in the gap between brief and realization.

---

## What Style Seeding Actually Is

Extracting samples from source texts (Borges, Carson, Wittgenstein) isn't plagiarism or imitation. It's defining the orchestration.

From a source text, you extract:
- **Syntactic patterns**: How sentences build. Clause rhythms. Where the verb lands.
- **Compression level**: How much is implied vs stated. Density of meaning per word.
- **Image behavior**: Concrete vs abstract. Extended vs glancing. Explained vs trusted.
- **Register markers**: Latinate vs Germanic. Formal vs intimate. Technical vs vernacular.
- **Permitted incompleteness**: What can be left open. Where ambiguity is valued.
- **Characteristic moves**: How paragraphs start. How transitions happen. What gets repeated.

These are "articulation tendencies" and "voice roles" in orchestration terms. You're not copying phrases; you're conditioning the space of possible phrases.

When the base model generates candidates, these patterns shape what emerges. When the selector chooses, these patterns inform what counts as "fitting."

---

## The Phenomenology of Selection

What does it feel like to select rather than write?

**Reading-as-generative-act**: You read the candidates not as finished text but as possibilities. Each one opens a different future. The reading is predictive, projective.

**Taste without effort**: You don't have to produce the good sentence; you have to recognize it. This is different cognitive work. Some writers find it easier. Some find it disorienting.

**Responsibility without authorship**: You chose this branch. You're accountable for where the text goes. But you didn't write the words. The authorship is distributed between you and the model and the brief.

**Pace**: Selection can be faster than writing (you don't have to generate) or slower (you have to compare, consider, project). The rhythm is different.

**Surprise**: Sometimes none of the candidates are right, and that tells you something about the brief. Sometimes a candidate is better than what you would have written, and that's vertiginous.

---

## Branching Factor and Segment Size

Two key parameters in looming:

**Branching factor**: How many candidates per decision point? More = more selection pressure, more chance of surprise, more cognitive load. Fewer = more constrained, faster, but you might miss good options.

**Segment size**: How many tokens per candidate? Longer = more committed per choice, coarser control. Shorter = more decisions, finer control, but harder to evaluate rhythm.

These interact:
- High branching + short segments = micro-conducting, almost word-by-word
- Low branching + long segments = more like choosing between drafts
- The sweet spot depends on genre, intent, and what you're trying to learn

The Loom spec defaults to 8 candidates × 6 tokens. This is "sentence-fragment" granularity, enough to feel rhythm but not so much that you're locked in.

---

## Selection Criteria as Aesthetic Commitment

The brief's lean-into and avoid lists are falsifiable aesthetic claims:

**Select FOR:**
- Syntactic pull (makes you want to read the next word)
- Image density (concrete, specific, sensory over vague abstraction)
- Surprise-that-fits (unexpected but retrospectively inevitable)
- Load-bearing phrasing (minimal filler; every word does work)
- Register match (fits the voice described in the brief)

**Select AGAINST:**
- Assistant-mode hedging ("As an AI...", "It is important to note...")
- Premature resolution (closing tension too early)
- Generic mysticism (vague cosmic talk not grounded in the piece)
- Filler ("kind of", "really", "in a sense", "actually")
- Register breaks (sudden shifts in tone)

These aren't universal. They're the aesthetic of a particular mode. Different briefs, different criteria. The point is to make them explicit so selection becomes legible, even to yourself.

---

## The Clarify Move

In music, a conductor might stop rehearsal: "Cellos, what do you hear there? Is that a resolution or a suspension?" The question isn't "play it this way" but "what is this?"

In looming, the clarify move is the textual equivalent:

> "Candidates A and C treat this as continuous flow; B and D introduce rupture. Which frame are we in?"

The selector pauses, surfaces the conceptual fork, and asks the human (or themselves, or a second-order process) to resolve the ambiguity before committing.

This makes interpretive decisions visible. Often, in normal writing, these decisions happen unconsciously. The clarify move externalizes them.

---

## What the Tree Preserves

Normal writing: one text, one path, all alternatives forgotten.

Loomed text: one chosen path through a tree, all alternatives preserved.

The tree enables:
- **Retrospective analysis**: Where did you fight the model's preferences? (logprob gap)
- **Alternative exploration**: What if you'd gone left instead of right at decision 17?
- **Process legibility**: Someone else can see not just what you wrote but how you selected.
- **Resumption**: Set this path aside, explore another branch, maybe come back.

The text becomes a record of constrained navigation, not just a product.

---

## Authorship Questions

If you loom a text, who wrote it?

Unhelpful answers:
- "The AI wrote it" (you selected, constrained, directed)
- "You wrote it" (you didn't produce the words)
- "You co-wrote it" (vague, doesn't capture the structure)

More precise framing:
- The base model generated candidates within the space defined by training + conditioning.
- The brief constrained that space according to your aesthetic commitments.
- The selector (you, or an agent acting on criteria you set) chose the path.
- The text is the trace of that process.

Authorship is distributed across: trainer → model → brief-writer → selector → reader.

This isn't new. All authorship is distributed (language itself, genre conventions, editors, publishers, readers who complete meaning). Looming just makes some of the distribution visible.

---

## Real-Time Looming

What if selection were fast enough to feel like conducting?

Current looming is deliberate: generate, pause, read, consider, choose. Seconds to minutes per decision.

Real-time looming would be: generate continuously, select continuously, text emerges at reading pace or faster.

This would require:
- Very fast generation (streaming, small segments)
- Trained intuition (you don't deliberate, you recognize)
- A control interface optimized for flow (not clicking buttons)

The video conductor achieves this for images. 23 fps, continuous conditioning, performer shapes the flow.

For text, this is harder because text is discrete and semantic. You can't really "blend" between two sentences the way you can blend between two images. But you could:
- Pre-compute branches and let the performer navigate
- Use physiological signals (gaze, micro-gestures) to bias selection
- Accept lower granularity (paragraph-level steering, not word-level)

This is speculative but points toward what "text performance" might mean.

---

## Multi-Voice Looming

What if the brief included multiple voice roles, like orchestral sections?

> "This section has three voices: the technical voice (precise, constrained), the imagistic voice (concrete, surprising), and the questioning voice (incomplete, pulls forward). They interleave. The selector chooses not just which candidate but which voice should speak now."

This would require:
- Candidates tagged by voice
- Selection criteria that include voice-balance
- A sense of polyphony in prose

This is closer to how some complex prose actually works (Sebald, Bernhard, Carson). Multiple registers interwoven, the shifts themselves carrying meaning.

---

## The Loom as Instrument

An instrument is a constraint that enables expression. The piano has 88 keys, fixed pitches, a specific action. The guitarist has frets, strings, a body. The constraints aren't limitations—they're what make performance possible.

The loom is an instrument:
- The brief constrains the space.
- The branching factor and segment size shape the granularity.
- The selection interface (CLI, GUI, gesture?) shapes the feel.
- The tree structure shapes what's preserved and revisitable.

Learning to loom is learning the instrument. What briefs work? What segment sizes feel right for what genres? How do you read candidates quickly? When do you clarify vs commit?

This takes practice. Like any instrument.

---

## Practical Implications

### For Loom development:
- Style seeding needs to be first-class (load samples, extract patterns, condition generation)
- The selector interface matters enormously (speed, feel, feedback)
- The tree visualization is part of the instrument, not just analysis

### For writing practice:
- Looming is a way to study your own taste (what do you select for?)
- It separates recognition from production (useful when production is blocked)
- It makes stylistic influence concrete (these Borges samples → these patterns)

### For thinking about authorship:
- The question isn't "who wrote it" but "how was selection structured"
- Credit/responsibility follows the constraint and selection chain
- Transparency about process is more meaningful than attribution labels

### For the multi-register blog:
- Each register is a different brief (different voice, compression, permitted moves)
- Versions could be loomed separately with shared content-intent
- The reader toggling registers is selecting which conducting-path to experience
- Or: one deep loom, multiple paths through the tree, registers as branches

---

## Coda: The Orchestra Metaphor, Extended

The base model is not the orchestra. The base model is more like: the training of the musicians, the acoustic of the hall, the physics of the instruments. It's the substrate that makes certain sounds possible.

The brief is the score. Constraining, enabling, leaving room.

The few-shot examples are like rehearsal recordings. "This is the sound we're going for."

The selector is the conductor. Not choosing notes, but shaping how the constrained inevitability unfolds.

The text is the performance. One realization among possible realizations. Committed, irreversible, alive in its moment.

The tree is the archive. Not just the recording, but the score with annotations: here's where we almost went another way. Here's what we considered and rejected. Here's the space the performance moved through.

And the reader? The reader is the audience. But also, if the tree is visible, the reader can become a conductor of a new performance: same branches, different choices, different text emerging.

The text is never finished. It's a navigation through possibility, and someone else can always navigate again.
