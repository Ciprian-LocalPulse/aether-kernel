# Governance

## Current stage: BDFL (Benevolent Dictator, early-stage)

Aether Kernel is, at this stage, a single-author research project. All
architectural decisions are currently made by the project creator,
**Ciprian Ștefan Pleșca** (see [`AUTHORS.md`](../../AUTHORS.md)).

This is a transitional state, not the target end-state. As described in
the blueprint (§12.1), the intended long-term structure is:

## Target structure: open foundation

* **Aether Foundation** (or equivalent neutral body) holding the
  trademark and coordinating releases, once the project has a
  contributor base large enough to warrant it.
* A **technical steering council** drawn from active contributors,
  industry partners, and academic collaborators.
* Design decisions above module-local scope go through a public
  **RFC process**: open an issue proposing the change, allow discussion,
  and require sign-off from the steering council (or, currently, the
  maintainer) before merging.
* License: **Apache 2.0** for the core (already in effect — see
  [`LICENSE`](../../LICENSE)); optional commercial modules may ship
  under separate licenses without affecting the core's openness.

## How decisions are made today

1. Non-trivial changes (new subsystem, breaking API change, security
   model change) → open a GitHub issue describing the proposal.
2. Discussion happens in the issue thread.
3. The maintainer approves, requests changes, or declines, with reasoning
   recorded in the issue.
4. Approved changes are implemented via PR, reviewed against
   [`CONTRIBUTING.md`](../../CONTRIBUTING.md)'s conventions.

## Becoming a maintainer

There is no formal process yet. Sustained, high-quality contributions to
one of the subsystems (kernel, sync-engine, perception-engine,
intent-router, SDKs, security, docs) are the path to being invited onto
a future steering council as the project grows past the single-author
stage.

## Ethics

Per blueprint §12.2, the project commits to:

* **Privacy by default** — no personal data collection without explicit,
  granular consent (see [`security/SECURITY_MODEL.md`](../../security/SECURITY_MODEL.md)).
* **Equitable access** — design choices should not gratuitously exclude
  low-resource contributors or users (e.g. avoiding hard dependencies on
  expensive proprietary hardware where an open alternative exists).
* **Transparency** — perception and decision algorithms are documented
  and, where feasible, open source.
* **Accountability** — see [`SECURITY.md`](../../SECURITY.md) for the
  vulnerability disclosure process, which doubles as the project's
  incident-accountability channel until a formal governance body exists.

## Project risks

Project-level (non-security) risks — adoption, competition, key-person
dependency — are tracked separately in the
[Risk Register](RISK_REGISTER.md).
