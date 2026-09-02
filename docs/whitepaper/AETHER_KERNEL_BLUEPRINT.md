# Aether Kernel — Technical Blueprint

**A Distributed Operating System for Spatial Computing, Robotics, and Physical-Digital Synchronization**

**Version 2.0 — Public Release Draft**
**Author:** Ciprian Ștefan Pleșca
**Document Reference:** AK-BLUEPRINT-2026-002
**License of this document:** CC BY 4.0 (attribution required) — code samples: Apache-2.0
**Status:** Draft for public repository, open review, and academic circulation

---

### Copyright and authorship notice

© 2026 Ciprian Ștefan Pleșca. All rights to the original architecture,
naming ("Aether Kernel," "Aether Sync Protocol," "Intent Router"), and
the text of this document are reserved to the author. This document is
prepared for public release under an open license; redistribution and
derivative technical work are permitted under the terms stated above,
provided attribution to the original author is preserved in any fork,
whitepaper, or derivative repository.

A note on what the document reference above is and is not: it is an
internal document-control identifier, of the kind used to track
versions across a repository — it is **not** a government patent
number, a copyright office registration number, or any other official
legal registration. No such registration exists because none has been
filed. The practical path to formal protection, if wanted, is: (1) a
notarized deposit or a cryptographic timestamp — committing this file
to a public Git repository already produces a verifiable SHA hash with
a timestamp, which is the closest thing to a free, real "registration"
available today — and (2) optionally, registration of the text with a
national copyright office (e.g. ORDA in Romania) for a small fee.

---

## Table of Contents

1. [Abstract](#1-abstract)
2. [Problem Statement and Motivation](#2-problem-statement-and-motivation)
3. [Related Work and Positioning](../architecture/RELATED_WORK.md)
4. [Design Principles and Non-Goals](../architecture/NON_GOALS.md)
5. [System Architecture Overview](#5-system-architecture-overview)
6. [Layer II — Aether Microkernel](#6-layer-ii--aether-microkernel)
7. [Layer III — Aether Runtime](#7-layer-iii--aether-runtime)
8. [Layer IV — SDK and Developer Experience](#8-layer-iv--sdk-and-developer-experience)
9. [Security, Identity, and Trust](#9-security-identity-and-trust)
10. [Formal Verification Strategy](#10-formal-verification-strategy)
11. [Failure Modes and Resilience Analysis](../architecture/FAILURE_MODES.md)
12. [Repository Layout](../architecture/ARCHITECTURE.md)
13. [Incremental Delivery Roadmap](../roadmap/ROADMAP.md)
14. [Governance and Ethical Framework](../governance/GOVERNANCE.md)
15. [Risk Register](../governance/RISK_REGISTER.md)
16. [Glossary](#16-glossary)
17. [References](#17-references)
18. [Appendix A — Protocol Message Reference](#18-appendix-a--protocol-message-reference)

Sections 3, 4, 11, 12, 13, 14, and 15 are maintained as standalone,
independently-updatable documents (linked above) rather than duplicated
here, so they stay in sync with the code and don't drift out of date
inside a long static document.

---

## 1. Abstract

Current augmented reality, robotics, and spatial computing systems are
architecturally fragmented: each vendor maintains a proprietary world
model, a proprietary synchronization protocol, and a proprietary
security boundary. This fragmentation raises integration costs,
prevents cross-vendor interoperability, and concentrates control of
physical-world data in a small number of platform owners.

Aether Kernel is proposed as an open, capability-secure, distributed
operating system that provides three shared primitives across
heterogeneous physical and digital nodes: (a) a semantic scene graph
derived from local sensor fusion, (b) a low-latency, conflict-free
synchronization substrate for that scene graph across a mesh network of
nodes, and (c) an intent-routing layer that translates high-level human
intent into coordinated multi-agent action. This document specifies the
architecture, the wire protocols, the security model, a reference
implementation strategy in Rust, C++, Python, and TypeScript, and an
incremental delivery plan suitable for a small engineering team.

## 2. Problem Statement and Motivation

Three converging trends motivate this work:

1. **Sensor ubiquity.** Cameras, LiDAR, IMUs, and radio sensors are now
   inexpensive enough to be embedded in consumer devices at scale (AR
   glasses, drones, home robots, vehicles), producing a continuous
   stream of raw physical-world data with no shared representation.
2. **Agent proliferation.** The number of autonomous or semi-autonomous
   physical agents per household or workplace (robots, drones,
   appliances) is increasing faster than the standards needed to
   coordinate them.
3. **Platform lock-in risk.** Absent an open substrate, the
   physical-world equivalent of the early-2000s "walled garden"
   internet is likely to re-emerge: a small number of vertically
   integrated platforms, each with its own scene representation, sync
   protocol, and permission model, competing for exclusive control of
   ambient computing.

The absence of an open, neutral, capability-secure layer analogous to
what TCP/IP and Linux provided for networking and computing
respectively is the gap this project addresses.

See [Related Work and Positioning](../architecture/RELATED_WORK.md) for
how this compares to seL4, ROS2/DDS, OpenXR/WebXR, Matter/Thread, and
CRDT libraries, and [Design Principles and Non-Goals](../architecture/NON_GOALS.md)
for what this project explicitly does not attempt to solve.

## 5. System Architecture Overview

```
┌───────────────────────────────────────────────────────────────┐
│                  Layer IV — Application Layer                  │
│   (Spatial apps, enterprise digital-twin tools, BCI interfaces)│
├───────────────────────────────────────────────────────────────┤
│                  Layer IV — SDK & Toolchain                    │
│   (Rust / C++ / Python / TypeScript, Scene Graph API)          │
├───────────────────────────────────────────────────────────────┤
│                  Layer III — Aether Runtime (per node)         │
│   ┌───────────────────┐ ┌──────────────────┐ ┌───────────────┐│
│   │ Perception Engine  │ │ Temporal Sync    │ │ Intent Router ││
│   │ (sensor fusion,    │ │ Engine (CRDT +   │ │ (NLU, planner,││
│   │  SLAM, scene graph)│ │ HLC, QUIC)       │ │  executor)    ││
│   └───────────────────┘ └──────────────────┘ └───────────────┘│
├───────────────────────────────────────────────────────────────┤
│                  Layer II — Aether Microkernel                 │
│   (EDF + round-robin scheduler with admission control,         │
│    zero-copy IPC, capability-based memory, isolated drivers)   │
├───────────────────────────────────────────────────────────────┤
│                  Layer I — Hardware Abstraction Layer (HAL)    │
│   (Drivers for cameras, LiDAR, IMU, radios, actuators)         │
└───────────────────────────────────────────────────────────────┘
```

Each node runs an Aether Runtime atop the microkernel. Nodes
communicate over the Aether Sync Protocol (§7.2), forming a global
mesh. An optional Cloud Orchestrator provides discovery, identity
resolution, and coordination at scale, but the system degrades
gracefully to pure peer-to-peer operation in its absence.

Reference implementation map: [`docs/architecture/ARCHITECTURE.md`](../architecture/ARCHITECTURE.md).

## 6. Layer II — Aether Microkernel

### 6.1 Objectives

The microkernel is the trusted computing base. It must guarantee:
strict isolation between drivers and services; sub-microsecond IPC for
small messages; deterministic scheduling for latency-critical
perception/sync flows; memory safety without a garbage collector; and
extensibility that never widens the trusted base.

### 6.2 Capability model with epoch-based revocation

Every resource (memory region, IPC port, device) is reached only
through a `Capability` — an unforgeable token naming the resource and
the rights held over it. Rights can only be *attenuated* on derivation,
never amplified: `derive()` fails if the requested rights are not a
subset of the parent's.

Revocation uses a **per-target epoch counter** rather than a simple
"delete the row from the table" scheme. Every capability snapshots its
target's epoch at mint/derive time; `check()` compares that snapshot
against the target's *current* epoch. Revoking any capability over a
target bumps that target's epoch, which instantly invalidates **every**
capability naming it — including derived children and any copy a
process cached off-table. A naive delete-by-id scheme cannot provide
this: a cached copy elsewhere would keep working until something
re-checks it against the table.

Reference implementation: [`kernel/src/capability.rs`](../../kernel/src/capability.rs)
(see the `revoking_target_invalidates_an_off_table_cached_copy` test for
a concrete demonstration).

### 6.3 Scheduler with EDF admission control

Real-time tasks (the perception pipeline, sync-engine ticks) are
scheduled Earliest-Deadline-First; everything else is round-robin.
Critically, a real-time task is not simply accepted onto the queue — it
must pass an **admission-control test** first: the scheduler tracks the
sum of admitted tasks' declared WCET budgets, and rejects a new task if
admitting it would push total utilization over the schedulable bound
(`U ≤ 1` within the configured period). A scheduler that always accepts
real-time work can silently promise deadlines it cannot keep; rejecting
overcommitment at admission time is safer than discovering a missed
deadline later.

Reference implementation: [`kernel/src/scheduler.rs`](../../kernel/src/scheduler.rs).

### 6.4 IPC and system call surface

Small control messages pass through a lock-free ring buffer;
large payloads (camera frames, point clouds) are never copied through
the kernel — the sender maps a shared-memory region and grants a
read-only capability over it to the receiver, so the kernel only moves
the capability, not the bytes. The minimal syscall surface:
`create_process`, `send`, `receive`, `map_memory`, `create_capability`,
`revoke_capability`.

Reference implementation: [`kernel/src/ipc.rs`](../../kernel/src/ipc.rs),
[`kernel/src/process.rs`](../../kernel/src/process.rs).

## 7. Layer III — Aether Runtime

### 7.1 Perception Engine

Pipeline: `Sensors → Preprocessing → Sensor Fusion (EKF/factor graph) →
SLAM & Mapping (ORB-SLAM3/LIO-SAM) → Semantic Segmentation
(YOLOv8/open-vocab CLIP) → Scene Graph`. Scene objects carry a stable
UUID, a transform with a 6×6 pose covariance (for fusion weighting), a
label, and a `part-of` parent relation. Wire representation targets
Cap'n Proto for zero-copy deserialization on resource-constrained edge
devices.

Reference implementation: [`perception-engine/`](../../perception-engine).

### 7.2 Temporal Sync Engine

**The Aether Sync Protocol** (`HELLO` / `SUBSCRIBE` / `PUBLISH` /
`SYNC_STATE` / `HEARTBEAT` — full field reference in §18) runs over
QUIC/WebTransport. Writes are represented as CRDT operations:

- **`LwwRegister`**, for pose data, ordered by a **Hybrid Logical
  Clock (HLC)** stamp rather than a raw wall-clock timestamp. A plain
  `(timestamp, node_id)` pair has a real gap: under clock skew, a
  causally-later write can carry an earlier wall-clock value and
  incorrectly lose a merge. HLC fixes this by carrying a logical
  counter alongside physical time and advancing it whenever a node
  observes a remote stamp ahead of what its own physical clock alone
  would suggest — merge order never contradicts causal order,
  regardless of skew. Reference: Kulkarni et al. (2014); implementation
  in [`sync-engine/src/hlc.rs`](../../sync-engine/src/hlc.rs), used by
  [`sync-engine/src/crdt.rs`](../../sync-engine/src/crdt.rs).
- **`OrSet`**, for tag/attribute collections, where naive LWW would
  incorrectly resurrect deleted elements.

**Spatial sharding** partitions the global space into cells
(conceptually S2 cells) at a configurable level; each cell has a
replica set responsible for synchronizing objects within it, which is
what lets the system scale sublinearly — a node only needs to track the
cells it currently occupies, not the entire planet's state. Reference:
[`sync-engine/src/sharding.rs`](../../sync-engine/src/sharding.rs).

### 7.3 Intent Router

`User input (voice/text/gesture) → NLU → structured Intent → Planner
(GOAP/PDDL/Behavior Trees) → Plan → Executor → device commands`, with a
Contract-Net-style negotiation step when a task spans multiple agents:
each capable agent bids a cost estimate, and the (per-task, not fixed)
coordinator assigns to the lowest bidder.

Reference implementation: [`intent-router/`](../../intent-router).

## 8. Layer IV — SDK and Developer Experience

Four SDKs share one wire protocol and one scene representation:

| SDK | Primary use case |
|---|---|
| Rust | Kernel-adjacent, maximum performance |
| Python | Research, rapid prototyping, ML integration |
| TypeScript | Browser-based AR via WebXR |
| C++ | Game-engine integration (Unreal) and industrial robotics |

Common API surface: `connect`, `create_object`, `update_object`,
`subscribe`, `send_intent` — see [`sdk/`](../../sdk) for all four
implementations.

## 9. Security, Identity, and Trust

- **Decentralized identity.** Every user, device, and digital object
  holds a W3C DID, resolvable via a lightweight public registry, cached
  locally for offline resolution.
- **Capability tokens (macaroons).** Cross-node access grants are
  scoped to a resource, a permission set, a time window, and optional
  contextual predicates; attenuable and revocable at any point by the
  issuer.
- **Data minimization by construction.** Raw sensor streams never leave
  the originating device; only derived semantic facts are shared, and
  only to the extent the user's sharing policy allows.
- **Transport security.** All inter-node traffic is end-to-end
  encrypted.
- **Auditability.** Every mutation to a shared object is appended to a
  hash-chained, tamper-evident log.

Full detail: [`security/SECURITY_MODEL.md`](../../security/SECURITY_MODEL.md),
[`security/THREAT_MODEL.md`](../../security/THREAT_MODEL.md),
[`security/identity/DID_SCHEME.md`](../../security/identity/DID_SCHEME.md).

## 10. Formal Verification Strategy

Following the precedent set by seL4, the microkernel's scheduler, IPC
path, and capability-check logic are the highest-value targets for
formal verification, because a defect there compromises every layer
above it.

1. Specify the capability invariant — "a process can only ever reach a
   resource through a capability whose rights are a subset of what was
   originally granted, and revocation is immediately effective" — in a
   proof assistant (Coq or Isabelle/HOL).
2. Extract or refine the Rust implementation against that specification
   incrementally, starting with `kernel/src/capability.rs`, before
   extending to the scheduler and IPC ring buffer.
3. Treat drivers and the Perception/Sync/Intent runtime as untrusted
   from the kernel's point of view — sandboxed, not verified — which
   keeps the verification burden bounded to a small trusted computing
   base rather than the entire codebase.

## 16. Glossary

- **CRDT** — Conflict-free Replicated Data Type; a data structure that
  guarantees convergent merges without coordination.
- **HLC** — Hybrid Logical Clock; combines wall-clock time with a
  logical counter for causal ordering under clock skew.
- **DID** — Decentralized Identifier (W3C standard).
- **Capability (security)** — an unforgeable token that both names a
  resource and grants specific rights over it.
- **S2 Cell** — a hierarchical spatial indexing scheme used to shard
  geographic space into addressable cells.
- **EDF** — Earliest Deadline First, a real-time scheduling algorithm.

## 17. References

1. Klein, G. et al. (2010). *seL4: Formal verification of an OS
   kernel.* Communications of the ACM.
2. Shapiro, M., Preguiça, N., Baquero, C., & Zawirski, M. (2011).
   *Conflict-free replicated data types.* SSS.
3. Langley, A. et al. (2017). *The QUIC transport protocol: Design and
   Internet-scale deployment.* SIGCOMM.
4. Mur-Artal, R., Montiel, J. M. M., & Tardós, J. D. (2021).
   *ORB-SLAM3.* IEEE Transactions on Robotics.
5. Redmon, J. et al. (2023). *YOLOv8.* arXiv preprint.
6. W3C (2022). *Decentralized Identifiers (DIDs) v1.0.* W3C
   Recommendation.
7. Pixar (2023). *Universal Scene Description (USD).* OpenUSD
   documentation.
8. Kulkarni, S. et al. (2014). *Logical Physical Clocks and Consistent
   Snapshots in Globally Distributed Databases.* (Hybrid Logical
   Clocks.)

## 18. Appendix A — Protocol Message Reference

| Message | Direction | Fields | Purpose |
|---|---|---|---|
| `HELLO` | Node → Relay | version, capabilities[] | Handshake, protocol negotiation |
| `HELLO_ACK` | Relay → Node | assigned_node_id, relay_capabilities[] | Confirms handshake |
| `SUBSCRIBE` | Node → Relay | cell_id | Subscribe to updates in a spatial cell |
| `UNSUBSCRIBE` | Node → Relay | cell_id | Stop receiving updates |
| `PUBLISH` | Node → Relay | object_id, operation, hlc_timestamp | Broadcast a CRDT operation |
| `SYNC_STATE` | Node → Relay | vector_clock | Request full reconciliation |
| `SYNC_STATE_RESPONSE` | Relay → Node | objects[], vector_clock | Authoritative state snapshot |
| `HEARTBEAT` | Bidirectional | sequence_no, sent_at_ns | Keep-alive and RTT measurement |

Current implementation status of this message set:
[`sync-engine/src/protocol.rs`](../../sync-engine/src/protocol.rs).

---

*This blueprint is a living document. Corrections, RFCs, and pull
requests are welcome — see [`CONTRIBUTING.md`](../../CONTRIBUTING.md).*
