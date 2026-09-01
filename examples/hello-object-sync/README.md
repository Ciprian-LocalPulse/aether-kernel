# Example: Hello Object Sync

The Stage 0 exit-criterion demo from
[`docs/roadmap/ROADMAP.md`](../../docs/roadmap/ROADMAP.md): two
independent processes ("Node A" and "Node B") each hold a CRDT
`LwwRegister` representing a cube's position, exchange updates through
the `sync-engine`'s `InMemoryTransport` (a real QUIC transport is a
Stage 2 roadmap item), and converge to the same state.

This intentionally uses the in-memory test transport rather than a real
network socket — the goal at this stage is to prove the **CRDT
convergence semantics** end-to-end, not networking. Swapping in the
QUIC-backed transport later should not require any change to
`rust-node-a` or `rust-node-b`'s logic, only to how `Transport` is
constructed.

## Run it

```bash
cd rust-node-a && cargo run
```

```
[node-a] local cube position: (0, 0, 0) @ t=0
[node-a] applying local move -> (1, 0, 0) @ t=1
[node-a] received remote update from node-b -> (1, 2, 0) @ t=2
[node-a] converged state: (1, 2, 0)
```

`rust-node-b` mirrors the same flow from the other side. Run both and
compare their final printed state — they converge to the identical
`(1, 2, 0)` despite applying updates in a different order, which is
exactly the point of using a CRDT here.
