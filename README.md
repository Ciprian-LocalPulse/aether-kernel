# Aether Kernel

**A distributed, open-source spatial operating system — a unified semantic layer for AR/VR, robotics, autonomous vehicles, and sensor networks.**

[![License: Apache 2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](LICENSE)
[![Status: Research Blueprint](https://img.shields.io/badge/status-research%20blueprint-orange.svg)](docs/whitepaper/AETHER_KERNEL_BLUEPRINT.md)
[![CI](https://img.shields.io/badge/CI-scaffold-lightgrey.svg)](.github/workflows/ci.yml)

<p align="center">
  <img src="assets/logo/aether-kernel-logo.svg" width="180" alt="Aether Kernel logo" />
</p>

## What is Aether Kernel?

Aether Kernel is a research blueprint and reference-implementation scaffold for a
**capability-based microkernel operating system** that lets every device — AR/VR
headsets, autonomous vehicles, robots, neural interfaces, and ordinary sensors —
perceive, understand, and act inside the *same* shared spatial reality, without
custom per-device integration work.

This repository is the technical foundation described in the
[Aether Kernel Blueprint](docs/whitepaper/AETHER_KERNEL_BLUEPRINT.md): a full
academic-style design document covering the microkernel, the perception
pipeline, the distributed synchronization protocol, the intent/planning layer,
identity and security model, and the developer SDKs.

> **Project status.** This is an early-stage, funding-constrained open research
> project. What you'll find here is a rigorously structured **architecture and
> scaffold**: real module boundaries, real build configuration, and working
> stub implementations in each target language — not a production-ready OS.
> Contributions that turn any stub into a working subsystem are extremely
> welcome. See [ROADMAP.md](docs/roadmap/ROADMAP.md) for the incremental,
> low-budget implementation path.

## Why

Today's AR, robotics, and autonomous-vehicle stacks are fragmented: every
platform keeps its own model of the world, its own sync protocol, its own data
silo. Aether Kernel's goal is to be the "Linux of the physical world" — a
neutral, secure, open foundation that any developer or company can build
spatial applications on top of.

## Architecture at a glance

```
┌──────────────────────────────────────────────────────────────┐
│                  Aether Application Layer                    │
├──────────────────────────────────────────────────────────────┤
│                  Aether SDK & Toolchain (Rust/C++/Py/JS)      │
├──────────────────────────────────────────────────────────────┤
│   Perception Engine  │  Temporal Sync Engine  │ Intent Router │
│   (C++)              │  (Rust, CRDT/QUIC)     │  (Python, LLM)│
├──────────────────────────────────────────────────────────────┤
│           Aether Microkernel (Rust, capability-based)         │
├──────────────────────────────────────────────────────────────┤
│           Hardware Abstraction Layer (drivers)                 │
└──────────────────────────────────────────────────────────────┘
```

Full details, sequence diagrams, protocol specs, and data formats:
[`docs/architecture/ARCHITECTURE.md`](docs/architecture/ARCHITECTURE.md).

## Repository layout

| Path | Language | Purpose |
|---|---|---|
| [`kernel/`](kernel) | Rust | Capability-based microkernel (scheduler, IPC, memory, capabilities) |
| [`perception-engine/`](perception-engine) | C++ | Sensor fusion, SLAM, semantic segmentation, scene graph |
| [`sync-engine/`](sync-engine) | Rust | CRDT-based Temporal Sync Engine over QUIC |
| [`intent-router/`](intent-router) | Python | NLU, planning (GOAP/PDDL), multi-agent coordination |
| [`sdk/`](sdk) | Rust, Python, JS/TS, C++ | Client SDKs exposing the Aether API |
| [`security/`](security) | — | Threat model, security architecture, DID identity scheme |
| [`simulator/`](simulator) | — | Notes and scaffold for a Godot-based test simulator |
| [`examples/`](examples) | Rust | Minimal end-to-end object-sync demo (Stage 0 of the roadmap) |
| [`docs/`](docs) | — | Whitepaper, architecture, roadmap, governance |

## Getting started

Each subsystem builds independently while the project is at scaffold stage.

```bash
# Microkernel (Rust)
cd kernel && cargo build

# Sync Engine (Rust)
cd sync-engine && cargo build

# Perception Engine (C++)
cd perception-engine && cmake -B build && cmake --build build

# Intent Router (Python)
cd intent-router && pip install -e .

# JS/TS SDK
cd sdk/js/aether-sdk && npm install && npm run build
```

Or run everything at once:

```bash
./scripts/build_all.sh
```

## Roadmap

Implementation is planned as six incremental, low-budget stages (Stage 0 →
Stage 5), from a two-node object-sync demo up to a full Intent Router driving
a physical robot arm. See [`docs/roadmap/ROADMAP.md`](docs/roadmap/ROADMAP.md).

## Security

Aether Kernel treats security as a first-class design constraint, not an
add-on: capability-based isolation, on-device-only raw sensor data, DID-based
decentralized identity, and macaroon-style capability tokens. See
[`SECURITY.md`](SECURITY.md) and [`security/THREAT_MODEL.md`](security/THREAT_MODEL.md).

If you discover a vulnerability, please **do not** open a public issue —
follow the disclosure process in [`SECURITY.md`](SECURITY.md).

## Contributing

See [`CONTRIBUTING.md`](CONTRIBUTING.md) and [`docs/governance/GOVERNANCE.md`](docs/governance/GOVERNANCE.md).
All contributors are expected to follow the [`CODE_OF_CONDUCT.md`](CODE_OF_CONDUCT.md).

## License

Licensed under the [Apache License 2.0](LICENSE).

## Author

Created and maintained by **Ciprian Ștefan Pleșca** ([@Ciprian-LocalPulse](https://github.com/Ciprian-LocalPulse)).
See [`AUTHORS.md`](AUTHORS.md).

## Citation

If you reference this work academically, see [`docs/whitepaper/AETHER_KERNEL_BLUEPRINT.md#13-references`](docs/whitepaper/AETHER_KERNEL_BLUEPRINT.md) for the citation list, or cite this repository directly:

```
Pleșca, C. Ș. (2026). Aether Kernel: A Distributed Spatial Operating System — Technical Blueprint. https://github.com/Ciprian-LocalPulse/aether-kernel
```
