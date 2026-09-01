# `kernel/` — Aether Microkernel (Rust)

Capability-based microkernel: scheduling, IPC, memory isolation, and driver
sandboxing, as specified in
[docs/whitepaper/AETHER_KERNEL_BLUEPRINT.md §4](../docs/whitepaper/AETHER_KERNEL_BLUEPRINT.md#4-microkernel-aether).

## Modules

| File | Responsibility |
|---|---|
| `capability.rs` | Capability tokens and the access-control model |
| `scheduler.rs` | EDF real-time scheduling + best-effort round robin |
| `ipc.rs` | Async, zero-copy message passing between processes |
| `memory.rs` | Address-space isolation and controlled memory sharing |
| `process.rs` | Process lifecycle and the kernel's minimal syscall-like API |
| `lib.rs` | Public crate API surface |

## Design note

This scaffold models the kernel in user-space Rust (no `#![no_std]` yet) so
the design — capabilities, scheduling policy, IPC semantics — can be
prototyped, tested, and reasoned about before committing to a bare-metal or
seL4-hosted target. Porting to `no_std` / bare metal is a Stage 2+ roadmap
item (see [docs/roadmap/ROADMAP.md](../docs/roadmap/ROADMAP.md)).

## Build & test

```bash
cargo build
cargo test
cargo clippy -- -D warnings
```
