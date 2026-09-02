# Design Principles and Non-Goals

## Principles

- **P1 — Openness.** Core specification and reference implementation
  under a permissive license (Apache-2.0); no single actor controls
  protocol evolution. See [`LICENSE`](../../LICENSE) and
  [`docs/governance/GOVERNANCE.md`](../governance/GOVERNANCE.md).
- **P2 — Security by construction.** Capability-based access control at
  every layer; raw sensor data never leaves the originating device by
  default. See [`security/SECURITY_MODEL.md`](../../security/SECURITY_MODEL.md).
- **P3 — Bounded latency.** Local-cell spatial synchronization target:
  sub-10ms P99 between geographically co-located nodes.
- **P4 — Planetary scalability.** Spatial sharding (not a single global
  ledger), so the system scales sublinearly in coordination overhead as
  node count grows. See `sync-engine/src/sharding.rs`.
- **P5 — Interoperability first.** One wire protocol and scene
  representation regardless of hardware vendor.
- **P6 — User sovereignty.** Users hold the keys that gate their own
  sensor data and scene contributions; sharing is opt-in and revocable.

## Explicit non-goals (v1.0)

Naming what this project is *not* trying to do is as important as the
architecture itself — it keeps the scope honest and preempts a category
of criticism ("why didn't you just use X") that a boundary-less project
invites.

- **Aether Kernel does not attempt to replace general-purpose operating
  systems** (Linux, Android). It runs as a runtime/service layer on top
  of them in the current phase (see `docs/roadmap/ROADMAP.md`), with a
  bare-metal/seL4-hosted microkernel target reserved for a later phase.
- **The project does not attempt to solve general-purpose robot
  manipulation planning** (grasping, motion planning). It exposes an
  interface for existing planners (MoveIt, GOAP engines — see
  `intent-router/src/aether_intent_router/planner.py`) rather than
  reimplementing them.
- **The project does not implement a cryptocurrency or token.** The
  DID/capability layer described in
  [`security/identity/DID_SCHEME.md`](../../security/identity/DID_SCHEME.md)
  is deliberately decoupled from any monetary instrument.
