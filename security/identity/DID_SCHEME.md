# Aether DID Scheme (design spec)

Reference: `docs/whitepaper/AETHER_KERNEL_BLUEPRINT.md` §8.1, W3C DID Core.

Status: **specification only** — no resolver is implemented in this
repository yet (see `docs/roadmap/ROADMAP.md` Stage 4).

## Method name

`did:aether`

## DID document shape (illustrative)

```json
{
  "@context": "https://www.w3.org/ns/did/v1",
  "id": "did:aether:z6Mk...exampleOnly",
  "verificationMethod": [
    {
      "id": "did:aether:z6Mk...#key-1",
      "type": "Ed25519VerificationKey2020",
      "controller": "did:aether:z6Mk...",
      "publicKeyMultibase": "z6Mk...examplePublicKeyOnly"
    }
  ],
  "authentication": ["did:aether:z6Mk...#key-1"],
  "service": [
    {
      "id": "did:aether:z6Mk...#sync-endpoint",
      "type": "AetherSyncEndpoint",
      "serviceEndpoint": "aether://node-42.example/sync"
    }
  ]
}
```

> Every value above is a structural placeholder for illustration, not a
> real key or identifier.

## Anchoring

DID documents are anchored in a lightweight, publicly verifiable registry
(candidates per the blueprint: a DAG such as IOTA/Hedera, or a
purpose-built append-only log) and cached locally by nodes so resolution
does not require a network round-trip on the hot path (blueprint §8.1).

## Capability tokens

Access to a specific object or service is granted via a **macaroon-style
capability token**, not by DID identity alone:

* `subject` — the DID being granted access
* `resource` — the object/service capability ID (`kernel::CapabilityId` for
  local resources, or a sync-engine `ObjectId`/`CellId` for remote ones)
* `rights` — a subset of `{read, write, execute, grant, revoke}` (see
  `kernel/src/capability.rs::Rights` for the local analogue)
* `caveats` — contextual restrictions (e.g. "only while the issuing user
  is co-located", "expires at T")
* `signature` — signed by the resource owner's DID key

Tokens are bearer credentials scoped as narrowly as possible and are
always revocable by the issuer.

## Relationship to the microkernel's `Capability` type

`kernel::Capability` (in `kernel/src/capability.rs`) models *local,
in-kernel* access control between processes on one node. DID-based
capability tokens described here are the *distributed* analogue,
granting access to objects/services owned by a *different* node. A
production Aether Runtime bridges the two: an incoming, DID-signed
capability token is validated and then mapped to a locally-scoped
`kernel::Capability` before the requesting process can touch anything.
