# Related Work and Positioning

| Prior art / adjacent system | What it solves | What it does not solve |
|---|---|---|
| seL4 microkernel | Formally verified isolation | No spatial synchronization or scene semantics |
| ROS2 / DDS | Robot middleware, pub-sub messaging | Not capability-secure by default; not designed for planetary-scale sharding |
| OpenXR / WebXR | Rendering and input standardization for AR/VR | No shared world-state synchronization across independent runtimes |
| Matter / Thread (IoT) | Device connectivity standard | No semantic scene graph, no spatial sync, narrow to home automation |
| Automerge / Yjs (CRDT libraries) | Conflict-free replication primitives | Not integrated with spatial sharding, sensor fusion, or capability security |
| Decentralized Identifiers (W3C DID) | Identity portability | No binding to physical-object ownership or spatial capability tokens |

Aether Kernel's contribution is not any single primitive above, but
their integration into one coherent, capability-secured stack
purpose-built for the physical-digital boundary — a semantic scene
graph (Perception Engine), a low-latency conflict-free sync substrate
for it (Temporal Sync Engine), and an intent-routing layer that
translates human intent into coordinated multi-agent action (Intent
Router), all sitting on a capability-based microkernel.

See [`docs/whitepaper/AETHER_KERNEL_BLUEPRINT.md`](../whitepaper/AETHER_KERNEL_BLUEPRINT.md)
for how each of these primitives is specified and where it's
implemented in this repository.
