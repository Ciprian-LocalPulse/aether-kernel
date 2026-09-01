# Roadmap

Aether Kernel is being built incrementally, on a constrained budget, by a
small team/individual contributors rather than the multi-hundred-million
dollar program described as the theoretical ceiling in the whitepaper's
cost analysis (§5 there; intentionally *not* reproduced here — this
roadmap describes the low-budget path instead).

Each stage has a **concrete, demoable deliverable**, so progress is
never just "more design documents."

## Stage 0 — Foundation *(current stage)*
**Team size:** 1–5. **Status:** scaffold in this repository.

- [x] Rust microkernel prototype: capability model, scheduler, IPC, memory policy (`kernel/`)
- [x] CRDT core for scene-graph sync: `LwwRegister`, `OrSet` (`sync-engine/`)
- [x] C++ perception pipeline shape: scene graph, SLAM stub, segmentation stub, sensor fusion stub (`perception-engine/`)
- [x] Python Intent Router: rule-based NLU, GOAP planner, executor, Contract-Net coordination (`intent-router/`)
- [x] Rust, Python, JS/TS, and C++ SDKs exposing a consistent client API
- [ ] A real two-node demo: sync a cube's position between two `sync-engine` instances over an actual network transport (currently in-memory only — see `examples/hello-object-sync/`)

**Exit criterion:** two independent processes on two machines synchronize
one object's pose with observable, sub-100ms convergence.

## Stage 1 — Basic perception *(+2–3 people)*
- [ ] Integrate a real RGB-D camera (Intel RealSense) and a real SLAM
      backend (ORB-SLAM3) behind the `SlamBackend` interface
- [ ] Replace `CentroidSlamStub` and `NaiveLatestSampleFusion` with real
      implementations
- [ ] Real object detection (YOLOv8) behind the `SegmentationBackend` interface
- [ ] Extend the CRDT sync protocol to handle updates from >2 nodes concurrently

**Exit criterion:** two users in the same physical room see the same
detected objects update live via the scene graph.

## Stage 2 — Scalable sync engine *(+2–3 people)*
- [ ] Wire `sync-engine/src/network.rs`'s `Transport` trait to real QUIC (`quinn`)
- [ ] Implement spatial sharding using real S2 cell indexing (replace the
      placeholder hashing in `sharding.rs`)
- [ ] Load-test: 1,000 simulated nodes synchronizing a shared virtual space
- [ ] Begin porting the microkernel toward `no_std` / bare-metal or a
      seL4-hosted target

**Exit criterion:** sub-10ms local-network sync latency at 1,000-node scale (simulated).

## Stage 3 — SDK & tooling *(+2–3 people)*
- [ ] Stabilize the SDK API across all four languages (breaking-change freeze)
- [ ] Godot-based simulator for testing without physical hardware
- [ ] Scene Graph Inspector and Network Analyzer debug tools
- [ ] Public developer beta with worked examples

**Exit criterion:** an external developer can build a toy AR app against
the SDK using only public documentation.

## Stage 4 — Security & identity *(+2–3 people)*
- [ ] Implement the `did:aether` resolver described in `security/identity/DID_SCHEME.md`
- [ ] Implement macaroon-style capability tokens for cross-node access
- [ ] End-to-end encryption for all sync-engine and intent-router traffic
- [ ] External security audit
- [ ] Signed, atomically-rolled-back OTA updates

**Exit criterion:** all items in `security/SECURITY_MODEL.md`'s "designed
but not built" table move to "implemented."

## Stage 5 — Intent Router at scale *(+2–3 people)*
- [ ] Swap `RuleBasedNlu` for a real LLM backend (`LlmNluBackend`,
      currently a stub in `intent-router/src/aether_intent_router/nlu.py`)
- [ ] Replace the toy GOAP action library with a PDDL-based planner or
      Behavior Trees for real-world action spaces
- [ ] Bridge the Executor to a physical robot arm for a live demo
- [ ] Implement the physical-safety envelope checks flagged in
      `security/THREAT_MODEL.md` (T11) — required before any
      physical-hardware pilot, not optional polish

**Exit criterion:** a spoken command ("bring me a glass of water") drives
a real robot arm end-to-end, safety-checked.

## Beyond Stage 5

Standardization via a neutral foundation, hardware certification program,
and enterprise support offerings — see the monetization discussion the
project's author has explored separately (open-core / Linux Foundation-style
governance, not covered by this technical roadmap).
