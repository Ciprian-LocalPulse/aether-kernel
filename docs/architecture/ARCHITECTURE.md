# Architecture

This document maps the conceptual architecture from
[`docs/whitepaper/AETHER_KERNEL_BLUEPRINT.md`](../whitepaper/AETHER_KERNEL_BLUEPRINT.md)
onto the actual code in this repository, module by module.

See also: [Related Work and Positioning](RELATED_WORK.md),
[Design Principles and Non-Goals](NON_GOALS.md),
[Failure Modes and Resilience Analysis](FAILURE_MODES.md), and the
[Risk Register](../governance/RISK_REGISTER.md).

## Layered view

```
┌──────────────────────────────────────────────────────────────┐
│                  Aether Application Layer                     │  ← not part of this repo (user apps)
├──────────────────────────────────────────────────────────────┤
│         Aether SDK & Toolchain  (sdk/rust, sdk/python,        │
│                sdk/js, sdk/cpp)                                │
├───────────────────┬──────────────────┬─────────────────────────┤
│ Perception Engine  │  Temporal Sync   │   Intent Router          │
│ (perception-engine,│  Engine          │   (intent-router, Python)│
│  C++)              │  (sync-engine,   │                          │
│                     │   Rust)          │                          │
├───────────────────┴──────────────────┴─────────────────────────┤
│           Aether Microkernel  (kernel/, Rust)                  │
├──────────────────────────────────────────────────────────────┤
│           Hardware Abstraction Layer (HAL)                     │  ← not implemented (roadmap Stage 1+)
└──────────────────────────────────────────────────────────────┘
```

## Component responsibilities and code map

### Microkernel (`kernel/`, Rust)

| Blueprint concept (§4) | Code |
|---|---|
| Capability-based access control, with per-target epoch revocation | `kernel/src/capability.rs::CapabilityTable` |
| EDF real-time + best-effort scheduling, with admission control | `kernel/src/scheduler.rs::Scheduler` |
| Async zero-copy IPC | `kernel/src/ipc.rs::IpcBus` (models the *semantics*; shared-memory zero-copy is a Stage 2 implementation detail) |
| Process lifecycle / minimal syscall API | `kernel/src/process.rs::ProcessTable` |
| Address-space isolation | `kernel/src/memory.rs::MemoryManager` (policy layer; real MMU integration is Stage 2+) |

### Perception Engine (`perception-engine/`, C++)

| Blueprint concept (§5) | Code |
|---|---|
| Scene Graph (space → frame → object → parts) | `include/aether_perception/scene_graph.hpp` |
| Sensor fusion (EKF interface) | `include/aether_perception/sensor_fusion.hpp` |
| SLAM & mapping | `include/aether_perception/slam.hpp` |
| Semantic segmentation | `include/aether_perception/semantic_segmentation.hpp` |
| End-to-end pipeline demo | `src/main.cpp` |

### Temporal Sync Engine (`sync-engine/`, Rust)

| Blueprint concept (§6) | Code |
|---|---|
| CRDTs (LWW-Register ordered by HLC for pose, OR-Set for tags) | `src/crdt.rs`, `src/hlc.rs` |
| Wire protocol (`HELLO`/`SUBSCRIBE`/`PUBLISH`/`SYNC_STATE`/`HEARTBEAT`) | `src/protocol.rs` |
| Spatial sharding (S2-cell-style) | `src/sharding.rs` |
| Transport abstraction (QUIC target) | `src/network.rs::Transport` |

### Intent Router (`intent-router/`, Python)

| Blueprint concept (§7) | Code |
|---|---|
| NLU: utterance → Intent | `src/aether_intent_router/nlu.py` |
| Planning (GOAP) | `src/aether_intent_router/planner.py` |
| Execution + monitoring | `src/aether_intent_router/executor.py` |
| Multi-agent coordination (Contract Net) | `src/aether_intent_router/coordination.py` |

### Security & Identity (`security/`)

| Blueprint concept (§8) | Code/Docs |
|---|---|
| Threat model | `security/THREAT_MODEL.md` |
| Overall security posture | `security/SECURITY_MODEL.md` |
| Decentralized identity (DIDs) | `security/identity/DID_SCHEME.md` |

### SDKs (`sdk/`)

Four language bindings (`rust`, `python`, `js`, `cpp`) exposing the same
conceptual API: `connect`, `create_object`, `update_object`, `subscribe`,
`send_intent` (blueprint §9.1). All four are currently in-memory scaffolds
with the same behavior, so app code can be prototyped against any of them
before a real transport is wired in.

## Data flow: a single "move this virtual object" round-trip

```
1. App calls SDK.update_object(id, new_transform)
2. SDK → Sync Engine: Publish { object, cell, PoseUpdate }
3. Sync Engine applies the CRDT operation locally (LwwRegister::merge),
   then broadcasts to subscribers of that spatial cell via the Transport
4. Peer nodes' Sync Engines receive Publish, merge, and notify their
   local SDK subscribers
5. Peer apps' subscribe() callbacks fire with the new Transform
```

## Data flow: a single spoken command

```
1. "bring me a glass of water" → Intent Router NLU → Intent{name: fetch_object, ...}
2. Planner (GOAP) → Plan{actions: [navigate_to_kitchen, pick_up_glass, ...]}
3. Coordination (Contract Net) assigns each action to the cheapest capable agent
4. Executor runs the plan, calling into the SDK / device APIs for each action
5. Perception Engine's Scene Graph is queried to resolve spatial references
   ("nearest glass") during planning
```

## Why these language choices

Per blueprint §2 and §9.1: Rust for the kernel and sync engine (memory
safety + performance for the security- and latency-critical path), C++
for perception (interoperability with existing robotics/CV libraries —
OpenCV, PCL, NVIDIA Isaac ROS), Python for the Intent Router (fast
iteration on NLU/planning logic, easy LLM SDK integration), and
JS/TS + Rust + Python + C++ SDKs to cover WebXR, scripting, systems, and
game/robotics-engine integration respectively.
