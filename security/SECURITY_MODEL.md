# Aether Kernel — Security Model

Reference: `docs/whitepaper/AETHER_KERNEL_BLUEPRINT.md` §8.

## 1. Principles

1. **No ambient authority.** Every access to memory, IPC, or a device goes
   through an explicit, unforgeable `Capability` (see `kernel/src/capability.rs`).
   A process that isn't handed a capability cannot name the resource it protects,
   let alone touch it.
2. **Attenuation only.** A capability may be *derived* into a weaker one
   (fewer rights), never amplified. See `CapabilityTable::derive` in the
   kernel.
3. **Local-first raw sensor data.** Camera, LiDAR, biosensor, and
   microphone streams are processed on-device. Only the derived semantic
   Scene Graph — not raw frames — is ever shared over the network, and
   only the parts the user has explicitly agreed to share.
4. **Decentralized identity.** Users, devices, and objects are identified
   by W3C DIDs, not accounts in a central database the platform operator
   controls. See `security/identity/DID_SCHEME.md`.
5. **Capability tokens for cross-node access.** Access to a *remote*
   object (owned by another node) is granted via short-lived, scoped
   macaroon-style tokens, not a blanket account permission.
6. **End-to-end encryption in transit.** All sync-engine and intent-router
   traffic is encrypted (TLS 1.3 / Noise Protocol / MLS for group
   messaging), independent of the transport-layer security QUIC already
   provides.
7. **Signed, atomic updates.** OTA kernel/driver updates are
   cryptographically signed; a failed update rolls back atomically rather
   than leaving a node in a partially-updated state.
8. **Sandboxed drivers.** Each device driver runs in its own process with
   only the capabilities it needs (e.g. a camera driver has no network
   capability). A compromised driver cannot pivot to other subsystems.

## 2. Trust boundaries

```
┌───────────────────────────────────────────────────────────┐
│ Untrusted: application code, third-party SDK consumers      │
├───────────────────────────────────────────────────────────┤
│ Semi-trusted: Intent Router (LLM-backed NLU), Perception    │
│               Engine detectors (ML models)                  │
├───────────────────────────────────────────────────────────┤
│ Trusted: Sync Engine core, capability table, DID resolver    │
├───────────────────────────────────────────────────────────┤
│ Most trusted: Microkernel (scheduler, IPC, memory isolation) │
└───────────────────────────────────────────────────────────┘
```

Code above a boundary must never be able to escalate its capabilities by
crossing it without an explicit, audited grant.

## 3. What this repository implements today vs. what's designed but not built

| Control | Status |
|---|---|
| Capability data model + attenuation rules | ✅ implemented (`kernel/src/capability.rs`), unit-tested |
| Process/IPC isolation model | ✅ implemented as a policy layer (`kernel/src/process.rs`, `ipc.rs`) |
| Real memory-page isolation (MMU-backed) | ❌ not implemented — Stage 2+ (bare-metal/seL4 port) |
| DID-based identity + resolution | 📄 specified only (`security/identity/DID_SCHEME.md`) |
| Macaroon-style capability tokens for cross-node access | 📄 specified only |
| End-to-end encryption of sync-engine traffic | 📄 specified only — sync-engine currently ships an in-memory test transport |
| Formal verification of scheduler/IPC/memory | ❌ not started — long-term roadmap item |

See `docs/roadmap/ROADMAP.md` for when each item is planned to land.
