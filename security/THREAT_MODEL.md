# Aether Kernel — Threat Model (STRIDE-based)

This is a living document. Every new subsystem should add rows here
before it's considered ready for anything beyond a local demo.

| # | Threat (STRIDE) | Component | Description | Mitigation | Status |
|---|---|---|---|---|---|
| T1 | Spoofing | Identity | An attacker impersonates a user/device to inject fake scene-graph objects. | DID-based authentication (WebAuthn / hardware keys) required before any `Publish` is accepted. | Designed, not implemented |
| T2 | Tampering | Sync Engine | A malicious node publishes forged CRDT operations for an object it doesn't own. | Per-object `owner` authority; non-owner writes require a capability token; owner is the sole conflict-resolution authority. | Designed (`sync-engine` protocol), token enforcement not implemented |
| T3 | Tampering | Perception Engine | Adversarial sensor input (e.g. spoofed LiDAR points) manipulates SLAM/segmentation output. | Sensor input validation, outlier rejection in fusion (EKF gating); out of scope for this scaffold's stub fusion engine. | Not implemented — flagged as a Stage 1+ requirement |
| T4 | Repudiation | Security & Identity | A device denies having issued a capability grant it actually issued. | Immutable audit log (hash-chained) of capability grants/revocations, per blueprint §8.4. | Designed, not implemented |
| T5 | Information disclosure | Perception Engine | Raw sensor data (video, biosensor readings) leaks to the network. | Local-first processing: raw frames never leave the device; only semantic Scene Graph nodes are shared, gated by user consent. | Design principle enforced by architecture (Perception Engine has no network capability by default) |
| T6 | Information disclosure | Sync Engine | An eavesdropper on the network reads scene-graph updates in transit. | TLS 1.3 / Noise Protocol required for all sync-engine traffic; MLS for group channels. | Designed, not implemented (current transport is in-memory/test-only) |
| T7 | Denial of service | Microkernel | A malicious or buggy process floods IPC ports to starve other processes. | Per-port message quotas, priority scheduling (EDF) that reserves headroom for real-time perception/sync tasks. | Partially designed (scheduler has RT/best-effort split); quotas not implemented |
| T8 | Denial of service | Temporal Sync Engine | An attacker floods a spatial cell with bogus object publishes. | Rate limiting per capability token; cell-level admission control. | Not implemented |
| T9 | Elevation of privilege | Microkernel | A compromised driver attempts to access memory/IPC it wasn't granted. | Capability-based access control: every access is checked against the process's held capabilities; no ambient authority. | Implemented (`kernel/src/capability.rs`) |
| T10 | Elevation of privilege | Intent Router | A crafted natural-language input tricks the LLM-backed NLU into issuing an intent beyond the user's authority (prompt injection). | Intent → Action translation must re-check the *issuing user's* capabilities at the Executor layer, never trust NLU output as pre-authorized. | Designed (Executor is capability-gated in the target architecture); current scaffold's `Executor` does not yet enforce this — flagged `TODO` |
| T11 | Physical safety | Robotics / Autonomous Vehicles | A compromised or buggy plan causes physical harm (e.g. a robot arm collides with a person). | Safety envelope checks at the Executor layer independent of the planner (hard-coded kinematic/geofence limits); human-in-the-loop confirmation for high-risk actions. | Not implemented — required before any physical-hardware pilot (see `docs/roadmap/ROADMAP.md` Stage 5) |

## Reporting new threats

Open a private security advisory (see `SECURITY.md`) or, for non-sensitive
design-level threats that don't describe an exploitable weakness in the
current code, open a regular issue tagged `security`.
