# Security Policy

Aether Kernel is designed to sit underneath AR/VR, robotics, and autonomous
systems — subsystems where a security flaw can have physical-world
consequences. We take reports seriously and ask the community to disclose
responsibly.

## Supported status

This project is currently a **research blueprint and pre-alpha scaffold**
(see [ROADMAP.md](docs/roadmap/ROADMAP.md)). There is no stable release yet;
all branches are considered experimental. Once tagged releases exist, this
table will list which lines receive security fixes.

| Version | Supported |
|---|---|
| `main` (pre-alpha) | ✅ best-effort |

## Reporting a vulnerability

**Do not open a public GitHub issue for security reports.**

Instead:

1. Use GitHub's private **[Security Advisories](../../security/advisories/new)**
   feature for this repository, *or*
2. Contact the maintainer directly (see [AUTHORS.md](AUTHORS.md) /
   the GitHub profile [@Ciprian-LocalPulse](https://github.com/Ciprian-LocalPulse)).

Please include:

* A description of the vulnerability and its potential impact.
* Steps to reproduce (proof-of-concept code, if applicable).
* The affected component(s) (`kernel/`, `sync-engine/`, `perception-engine/`,
  `intent-router/`, `sdk/`, ...).

You should receive an acknowledgment within **5 business days**. We will
keep you updated as the issue is triaged and fixed, and we credit reporters
(unless anonymity is requested) once a fix ships.

## Security design principles

These principles are binding on every subsystem in this repository — see
[`security/THREAT_MODEL.md`](security/THREAT_MODEL.md) and
[`security/SECURITY_MODEL.md`](security/SECURITY_MODEL.md) for the full
treatment:

1. **Capability-based isolation.** No component (driver, service, app) gets
   implicit access to anything; every access is via an explicit, revocable
   capability token.
2. **Local-first raw data.** Raw sensor streams (camera, LiDAR, biosensors)
   never leave the originating device. Only derived semantic representations
   (e.g. "object at X,Y,Z") are shared, and only with explicit user consent.
3. **Decentralized identity.** Users and devices are identified by
   self-sovereign DIDs (W3C spec), not a central account database.
4. **Signed, atomic updates.** OTA updates are cryptographically signed and
   roll back atomically on failure.
5. **Formal verification where it matters.** Scheduler, IPC, and memory
   isolation in the microkernel are designed to be amenable to formal
   verification (à la seL4), even though full proofs are a later-stage
   roadmap item, not a Stage 0 deliverable.
6. **No security-by-obscurity.** The entire stack, including this policy,
   is open source and auditable.

## Out of scope

* Vulnerabilities in third-party dependencies should be reported upstream
  (we will still appreciate a heads-up so we can pin/patch on our side).
* Denial-of-service reports that require unrealistic resource assumptions.
