# Risk Register

Project-level risks — not security vulnerabilities (see
[`security/THREAT_MODEL.md`](../../security/THREAT_MODEL.md) for those)
but the risks that determine whether Aether Kernel succeeds as a
project at all.

| Risk | Likelihood | Impact | Mitigation |
|---|---|---|---|
| Insufficient adoption to reach network-effect threshold | High | High | Ship a genuinely useful single-node mode (local AR/robotics) so the project has value before multi-node adoption |
| Large incumbent ships a closed competitor first | Medium | High | Move fast on the open reference implementation; openness itself is a competitive moat against closed alternatives |
| Formal verification effort stalls the kernel roadmap | Medium | Medium | Scope verification narrowly to the capability/IPC core (see `docs/roadmap/ROADMAP.md` Stage 2+); treat it as a parallel track, not a blocking dependency |
| Spatial data misuse / privacy backlash | Medium | High | Raw-data-never-leaves-device is a hard architectural invariant (`security/SECURITY_MODEL.md`), not a policy promise |
| Key contributor (sole author) unavailability | Medium | High | Publish openly and early specifically to seed a contributor base that does not depend on one person — see [`CONTRIBUTING.md`](../../CONTRIBUTING.md), [`DONATE.md`](../../DONATE.md) |

This register is reviewed and updated as the project moves through the
stages in [`docs/roadmap/ROADMAP.md`](../roadmap/ROADMAP.md).
