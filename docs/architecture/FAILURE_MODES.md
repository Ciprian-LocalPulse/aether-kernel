# Failure Modes and Resilience Analysis

| Failure mode | Detection | Mitigation |
|---|---|---|
| Relay node unreachable | `HEARTBEAT` timeout | Fall back to direct P2P via ICE/STUN/TURN; queue operations locally |
| Conflicting concurrent writes to one object | HLC / vector-clock divergence | CRDT merge is total and deterministic (`sync-engine/src/crdt.rs`, `hlc.rs`); no data loss, only ordering resolved |
| Compromised sensor driver | Sandboxed process crash / capability violation trap | Driver isolated in its own process; kernel kills and restarts it without affecting other subsystems |
| Malicious or buggy plan from Intent Router | Executor-side sanity checks against device capability limits | Executor refuses commands outside a device's declared safety envelope — see `security/THREAT_MODEL.md` T11, not yet implemented in the current `intent-router/src/aether_intent_router/executor.py` scaffold |
| Clock skew across nodes | HLC bounds skew explicitly | Physical-time component corrected by NTP/PTP where available; the logical counter (`sync-engine/src/hlc.rs`) provides causal ordering regardless of skew |
| Cell replica set failure (below replication factor) | Replica health-check gossip | Cell ownership re-elected among remaining replicas; objects re-published from local caches |
| Real-time task would exceed schedulable capacity | EDF admission-control test (`kernel/src/scheduler.rs::try_admit`) | Task is rejected at admission time rather than silently accepted and later missing its deadline |
| Stale capability copy used after revocation | Per-target epoch check (`kernel/src/capability.rs::check`) | Any capability whose snapshotted epoch no longer matches its target's current epoch is rejected, including copies the kernel never explicitly tracked |

This table is a living document — every new subsystem should add rows
here (and to [`security/THREAT_MODEL.md`](../../security/THREAT_MODEL.md)
for adversarial failure modes specifically) before it's considered ready
for anything beyond a local demo.
