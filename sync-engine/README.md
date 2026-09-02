# `sync-engine/` — Temporal Sync Engine (Rust)

CRDT-based synchronization of the distributed scene graph across nodes,
targeting sub-10ms latency via QUIC transport, spatial sharding (S2 cells),
and per-object owner authority. Blueprint reference:
[docs/whitepaper/AETHER_KERNEL_BLUEPRINT.md §6](../docs/whitepaper/AETHER_KERNEL_BLUEPRINT.md#6-temporal-sync-engine).

## Modules

| File | Responsibility |
|---|---|
| `crdt.rs` | `LwwRegister` (pose) and `OrSet` (tag/collection) CRDT primitives |
| `hlc.rs` | Hybrid Logical Clock — the stamp type `LwwRegister` orders on, immune to clock-skew reordering |
| `protocol.rs` | Aether Sync wire protocol (`HELLO`, `SUBSCRIBE`, `PUBLISH`, `SYNC_STATE`, `HEARTBEAT`) |
| `sharding.rs` | Spatial cell IDs and subscription routing |
| `network.rs` | Transport trait + an in-memory transport for tests (QUIC binding is a roadmap item) |
| `lib.rs` | Public crate API |

## Status

QUIC transport (via `quinn`) is intentionally **not wired up yet** — see
`docs/roadmap/ROADMAP.md` Stage 2. The CRDT core and wire protocol are
transport-agnostic by design (`network.rs` defines a `Transport` trait), so
swapping in `quinn` later doesn't require touching `crdt.rs` or `protocol.rs`.

## Build & test

```bash
cargo build
cargo test
```
