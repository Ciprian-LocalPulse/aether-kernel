# `simulator/`

Blueprint reference: §9.2. The intended test simulator (no physical
hardware required) is a Godot Engine project — chosen over NVIDIA
Omniverse for this stage because it's open source and has a much lower
barrier to entry for contributors.

## Status: not yet started (Stage 3, see `docs/roadmap/ROADMAP.md`)

This directory is a placeholder for that Godot project. Planned scope:

- A virtual scene with a handful of objects, each mapped to an
  `ObjectId` from `sync-engine`.
- A synthetic "sensor" that emits `SensorSample`-shaped data
  (`perception-engine/include/aether_perception/sensor_fusion.hpp`) so
  the perception pipeline can be exercised without real hardware.
- Multiple simulator instances connected via the real `sync-engine`
  transport (once QUIC is wired up in Stage 2), to visually demonstrate
  convergence — the graphical counterpart of
  `examples/hello-object-sync`.
- Basic instrumentation: on-screen latency and consistency metrics, per
  blueprint §9.2.

## Contributing

If you want to pick this up, open an issue first (see
[`CONTRIBUTING.md`](../CONTRIBUTING.md)) — this is a good self-contained
Stage 3 contribution that doesn't require touching Rust/C++ internals.
