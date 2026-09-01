# Support Aether Kernel

Aether Kernel is independent, self-funded, open-source research and
engineering work on a distributed spatial operating system —
a capability-based microkernel, a CRDT-based Temporal Sync Engine, a
Perception Engine, and an Intent Router meant to give AR/VR devices,
robots, autonomous vehicles, and sensors a shared, secure, low-latency
model of the physical world.

Funding directly determines how much time can go into kernel design,
CRDT and networking implementation, perception-pipeline integration,
security and identity work, and eventually independent security
audits — the parts of this project that are slow, technically
demanding, and essential for a system intended to sit underneath
real robots and vehicles.

## What Contributions Fund

Contributions directly support the continued research and development
of Aether Kernel, including:

* Development of the capability-based microkernel (scheduling, IPC,
  memory isolation, and eventually formal verification of the
  critical paths)
* Implementation of the Temporal Sync Engine's CRDT core, spatial
  sharding, and a production QUIC/WebTransport transport
* Integration of real sensor-fusion, SLAM, and semantic-segmentation
  backends into the Perception Engine
* Development of the Intent Router's planning, execution, and
  multi-agent coordination layers, including LLM-backed NLU
* Design and implementation of the DID-based identity scheme and
  macaroon-style capability tokens described in `security/`
* Cryptographic and security testing, threat modeling, fuzzing, and
  protocol documentation
* CI infrastructure, developer tooling, the Godot-based simulator, and
  research infrastructure
* Independent third-party security audits as the project matures
  (see `docs/roadmap/ROADMAP.md`, Stage 4)
* Continued protocol development and reference implementation work
* Eventually, compensating dedicated maintainers for specific
  subsystems (kernel, sync-engine, perception-engine, intent-router,
  SDKs)

## Financial Support

Support is currently available through PayPal and direct bank
transfer.

### PayPal

PayPal: [paypal.me/agentflowenterprise](https://www.paypal.com/paypalme/agentflowenterprise)
Email: [contact@agentflow-enterprise.com](mailto:contact@agentflow-enterprise.com)

If possible, please mention **Aether Kernel** when making a
contribution or contacting the project.

### Bank Transfer — USD

```
Name:            Ciprian Stefan Plesca
Account Type:    Checking
Routing Number:  026073150
Account Number:  8314225367
BIC/SWIFT:       CMFGUS33
Bank:            Community Federal Savings Bank
Address:         89-16 Jamaica Ave
                 Woodhaven, NY 11421
                 USA
```

### Bank Transfer — EUR

```
Name:            Ciprian Stefan Plesca
IBAN:            BE83 9679 1975 8915
BIC/SWIFT:       TRWIBEB1XXX
Bank:            Wise
Address:         Rue du Trône 100
                 Brussels
                 Belgium
```

**Important:** Please verify the banking information immediately
before initiating a transfer. If you are uncertain whether the
details are current, contact the project directly at
[contact@agentflow-enterprise.com](mailto:contact@agentflow-enterprise.com).

## Funding Transparency

Aether Kernel is an independently developed research and engineering
project. Financial support is intended to support the project's
technical development along the path described in
[`docs/roadmap/ROADMAP.md`](docs/roadmap/ROADMAP.md):

**Foundation → Perception → Scalable Sync → SDK & Tooling → Security & Identity → Intent Router at Scale**

Contributions do not purchase ownership of the project, control over
the architecture or roadmap, preferential treatment, access to
private forks or unreleased work, or influence over security
decisions. The project's technical direction remains independent —
see [`docs/governance/GOVERNANCE.md`](docs/governance/GOVERNANCE.md).

## Non-Financial Contributions

Code, systems and kernel expertise, CRDT/distributed-systems
knowledge, robotics and SLAM/perception experience, security and
cryptography review, SDK work across Rust/C++/Python/TypeScript,
documentation, testing, and honest criticism are at least as valuable
as financial contributions.

If you would like to contribute technically, see
[CONTRIBUTING.md](CONTRIBUTING.md).

## Contact

For financial support, development questions, security research,
architecture review, research collaboration, or contribution
coordination:

Email: [contact@agentflow-enterprise.com](mailto:contact@agentflow-enterprise.com)
PayPal: [paypal.me/agentflowenterprise](https://www.paypal.com/paypalme/agentflowenterprise)

---

Aether Kernel is independent research and engineering infrastructure
for a distributed spatial operating system.
Thank you for supporting the research and development of Aether Kernel.
