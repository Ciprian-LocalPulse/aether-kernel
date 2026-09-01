# Contributing to Aether Kernel

Thank you for considering a contribution. Aether Kernel is a large,
multi-language, funding-constrained open research project — every
contribution, from a typo fix in the whitepaper to a working CRDT
implementation, moves it forward.

## Ground rules

* Be respectful — see [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md).
* Discuss non-trivial changes in an issue before opening a large PR.
* Match the architecture described in
  [`docs/architecture/ARCHITECTURE.md`](docs/architecture/ARCHITECTURE.md)
  and the [blueprint](docs/whitepaper/AETHER_KERNEL_BLUEPRINT.md). If you
  want to deviate from it, open an RFC-style issue first.
* All contributions are made under the [Apache License 2.0](LICENSE)
  (see "Submission of Contributions" in the license text).

## Where to start

Check [`docs/roadmap/ROADMAP.md`](docs/roadmap/ROADMAP.md) — it lists the
current stage and concrete, scoped deliverables. Good first contributions:

* Fill in a `TODO` in any of the stub modules (`kernel/`, `sync-engine/`,
  `perception-engine/`, `intent-router/`, `sdk/*`).
* Add unit tests to a module that doesn't have them yet.
* Improve or extend a doc in `docs/`.
* Report a design flaw or missing threat in `security/THREAT_MODEL.md`.

## Development setup

```bash
git clone https://github.com/Ciprian-LocalPulse/aether-kernel.git
cd aether-kernel
./scripts/setup.sh   # installs toolchains where possible, prints what's missing
```

Per-module build instructions are in each module's own `README.md`.

## Commit and PR conventions

* Use [Conventional Commits](https://www.conventionalcommits.org/) style
  prefixes: `feat:`, `fix:`, `docs:`, `refactor:`, `test:`, `chore:`.
* Keep PRs scoped to one subsystem where possible.
* Every PR should pass `./scripts/build_all.sh` (or explain why a step is
  skipped, e.g. missing hardware).
* Reference the roadmap stage or issue your PR addresses.

## Code style

| Language | Style / linter |
|---|---|
| Rust | `rustfmt`, `clippy` (`cargo fmt`, `cargo clippy`) |
| C++ | `clang-format` (LLVM style), C++20 |
| Python | `black`, `ruff`, type hints required on public functions |
| JS/TS | `prettier`, `eslint`, strict TypeScript |

## Governance

Design decisions above "module-local" scope go through the RFC process
described in [`docs/governance/GOVERNANCE.md`](docs/governance/GOVERNANCE.md).
