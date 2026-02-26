# Session Briefing: Distributed Inference Systems for Real-Time Video DiT

Drop this document into a fresh Claude session to bootstrap context for focused learning.

---

## Who I Am

CS background (MIT undergrad, Launch School). Comfortable with linear algebra, probability, statistics, optimization, ML paper math. Interested in category theory and formal methods as organizing lenses. ML stack: PyTorch, Diffusers/ComfyUI, CLI-first. Hardware: MacBook Air M4, Windows 11 desktop with NVIDIA GPUs, Ubuntu boxes. I use Claude Code and terminal as primary interfaces.

I'm in the tpot/LessWrong/EA orbit. I spent two years as primary caregiver for my mom with Lewy Body dementia (she recently passed), during which I was training text-to-video models and building generative media tooling.

Communication preferences: be direct, tell me when I'm wrong, no sycophancy, Crocker's rules. Explain at the level of someone with MIT EECS coursework. Don't dumb things down but anchor abstractions to concrete examples and implementation.

---

## What I'm Building

A real-time interactive video streaming system built on a Wan2.1-based video DiT (Diffusion Transformer). The system serves live video generation with strict FPS targets.

### Hardware Targets

| GPU | HBM Bandwidth | Single-GPU FPS (optimized) | Notes |
|-----|--------------|---------------------------|-------|
| B300 | ~8 TB/s | ~34.5 FPS | Best perf, limited access |
| B200 | ~8 TB/s | ~33.35 FPS | Target deployment hardware |
| H200 | ~4.8 TB/s | ~25 FPS (TP=2+funcol+compile) | Current dev hardware |

The workload is **memory-bandwidth-bound** on H200. FPS tracks the bandwidth ratio across hardware tiers (B200/B300 at ~8 TB/s get ~34 FPS, H200 at ~4.8 TB/s gets ~20 single-GPU). This was confirmed via profiling: GEMM-heavy critical path, and the FPS ratio (~0.58) matches the bandwidth ratio (~0.60).

### The Optimization Journey (condensed)

**Single-GPU optimization (thorough, across B300/B200/H200):**
- torch.compile with various modes
- FA4 + varlen attention
- Fused projections, fused RoPE
- VAE streaming/chunked decode, contiguity fixes, channels-last-3D
- CUDAGraph capture attempts (unstable/neutral on H200)
- Result: ~20 FPS on H200, ~34 FPS on B300

**Tensor Parallelism (TP=2 on H200):**
- Megatron-style: ColumnParallelLinear + RowParallelLinear
- Distributed RMSNorm for qk_norm (adds 2 extra all-reduces per block, atypical)
- 40 blocks × 4 collectives per block = ~160 sync points per forward
- Result: ~19.5 FPS (regression from single-GPU)
- Root cause: bandwidth-bound workload, TP doesn't reduce bytes moved
- torch.compile incompatible: collectives wrapped in torch._dynamo.disable(), causing ~160 graph breaks

**Functional collectives + compile (the breakthrough):**
- Switched TP collectives to functional (out-of-place) form
- This removed Dynamo graph breaks, allowing torch.compile to fuse large regions
- Progression: 19.5 (no compile) → 18.0 (funcol, no compile, overhead) → 23.8 → 25 FPS (funcol + compile)
- The 25% gain came from compile being able to fuse across blocks once graph breaks were eliminated

**FP8 quantization:** Has quality problems on this specific model. Not viable currently.

### Current State and Next Steps

25 FPS on H200 with TP=2 + funcol + compile. Graph break cleanup hit diminishing returns. TP on 4 GPUs reached 27-29 FPS.

**The goal going forward is not more FPS at base resolution.** It's **higher resolution at real-time FPS.** Pipeline parallelism would allow running the generator across 2 GPUs on B200 (which has the bandwidth headroom), enabling higher-resolution real-time output.

### The PP Architecture Plan

We have a detailed bringup plan for "phase parallelism" (different from StreamDiffusionV2's layer-partitioned PP):

- **Rank 0 (Stage 0):** Control, text encoding, VAE encode/decode, envelope construction. Does NO generator work.
- **Mesh ranks 1..N (Stage 1):** Generator-only (KV cache setup/recompute + denoise loop), with TP collectives only inside mesh_pg.

Stage boundary is a message-passing interface (PPEnvelopeV1 / PPResultV1 contracts). Overlap comes from rank0 building next envelope + decoding previous result concurrently with mesh running the generator.

Phased bringup:
1. PP0: no TP, no overlap, prove contracts work
2. PP0 + overlap: bounded queues, non-blocking comms
3. PP0 + recompute: restore KV cache recompute via context_frames_override
4. PP1: TP inside mesh (ranks 1..N use mesh_pg for collectives, rank0 excluded)

**Potential Phase 2 (future):** Layer-partition the generator inside the mesh and implement Stream Batch (StreamDiffusionV2's approach) for throughput scaling. Phase 1 is a prerequisite.

Key distributed primitives in use:
- Two process groups: world_pg (all ranks) and mesh_pg (generator ranks only)
- Point-to-point (rank0 ↔ mesh leader) for envelope/result transport
- Collectives inside mesh_pg only (TP all-reduces, envelope broadcast from leader)
- Non-blocking sends/recvs for overlap
- Bounded queues (D_IN=2, D_OUT=2) for backpressure
- Epoch-based stale message handling for hard cuts

---

## Key Reference: StreamDiffusionV2

Paper: arXiv:2511.07399 (Nov 2025). UC Berkeley / MIT (Song Han lab) / Stanford.
Built on Wan 2.1 + CausVid. Training-free. 58.28 FPS (14B) on 4×H100.

**Their approach (different from ours):** Layer-partitioned PP across all GPUs (all GPUs run DiT layers, model split by depth) + Stream Batch (multiple denoising micro-steps in-flight to fill pipeline bubbles). VAE on edge ranks with dynamic block scheduler for imbalance.

**Key results relevant to us:**
- H100 roofline: ridge at 590.75 FLOP/Byte. Short-sequence causal DiT is memory-bound.
- PP comm overhead: ~2-5ms (vs Ulysses ~60ms, Ring ~40ms) because 1 activation transfer per stage vs many collectives
- Throughput formula: f ∝ B/(1+B) where B = in-flight micro-batches. Saturates as B grows.
- Stream Batch is essential: without it, multi-GPU PP gives no improvement
- Pipeline utilization: B/(B+P-1) where P = pipeline stages. P=2, B=1 → 50%, B=2 → 67%, B=4 → 80%

**Comparison to our approach:**
- Theirs: all GPUs in the pipeline run generator layers. Scaling comes from Stream Batch filling pipeline bubbles across denoising steps/frames.
- Ours: rank0 is outside the generator mesh entirely. Overlap comes from rank0 doing orthogonal work (VAE, control) concurrently with mesh running the full generator. Different ceiling, simpler boundary, TP composable inside mesh.

---

## Learning Menu

I want to build understanding of the distributed systems concepts needed for this PP build (and eventually the StreamDiffusionV2-style Phase 2). Here are the topics organized by area. **Ask me which ones I want to dig into.**

### Distributed Systems Fundamentals
1. **NCCL internals** — what happens at GPU level during all-reduce/send/recv, ring vs tree algorithms, NVLink topology effects
2. **Deadlock patterns in multi-group distributed code** — how process group misuse hangs, diagnosis tools (NCCL_DEBUG, timeouts), what error messages mean
3. **Graceful shutdown and draining** — clean termination without stranding peers, poison pill pattern
4. **Determinism across ranks** — bitwise reproducibility for debugging, when to relax it

### GPU Execution Model
5. **CUDA streams deep dive** — execution/dependency model, events, synchronization, interaction with autograd and NCCL
6. **CUDA graphs** — what they capture, why fast, what breaks them, relation to torch.compile/Inductor
7. **GPU memory management** — caching allocator, fragmentation, double-buffering and memory pressure
8. **Kernel launch overhead** — Python-to-GPU path, where compile eliminates overhead

### torch.compile / Inductor
9. **How Dynamo tracing works** — graph breaks mechanically, guards, fallback to eager
10. **Inductor fusion rules** — what fuses, how to verify, reading Inductor output
11. **Functional collectives and Dynamo** — why in-place breaks tracing, funcol solution, compiled graph structure
12. **Compile + distributed interaction** — compile with DDP/FSDP/TP, per-rank compilation, graph breaks from collectives

### Pipeline Parallelism Specifically
13. **Classic PP (GPipe, PipeDream)** — micro-batching, 1F1B schedule, bubble fraction, why inference PP is simpler than training PP
14. **Activation memory in PP** — who stores what, when activations free, TP vs PP differences
15. **Pipeline scheduling theory** — B/(B+P-1) derivation, uneven stage times, connection to Little's law

### Performance Analysis
16. **Roofline model applied to transformers** — arithmetic intensity for attention vs FFN, reading roofline plots, batch size effects
17. **Reading nsight / torch.profiler traces** — what to look for, memory-bound vs compute-bound identification, idle time, communication
18. **Bandwidth accounting** — throughput from first principles, how close to hardware ceiling

### Systems Patterns
19. **Producer-consumer with backpressure** — ring buffers, bounded channels, connection to D_IN/D_OUT design
20. **Message framing and versioning** — envelope versions, contract evolution, protobuf/flatbuffers
21. **Idempotency and replay** — epoch-based stale detection, at-least-once delivery

### Domain-Specific
22. **KV cache management in streaming inference** — causal DiT vs LLM KV caches, recompute-vs-retain, context frames
23. **VAE latency and chunking** — why video VAE is expensive, Stream-VAE chunking, temporal coherence at chunk boundaries
24. **Video DiT scheduling** — denoising steps × frame chunks, causal dependency structure, rolling window requirements
