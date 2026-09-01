# Aether SDK — Rust

Rust client for the Aether network. Mirrors the API surface in blueprint
§9.1 (`connect`, `create_object`, `update_object`, `subscribe`,
`send_intent`). Currently a scaffold: calls succeed locally but don't yet
talk to a real `sync-engine` transport — see the `TODO`s and
`docs/roadmap/ROADMAP.md` Stage 3.

```bash
cargo test
```
