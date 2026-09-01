//! Transport abstraction. Production targets QUIC (via `quinn`) with a
//! P2P fallback (ICE/STUN/TURN) as described in blueprint §6.5. This
//! scaffold defines the `Transport` trait and ships an in-memory
//! implementation so the CRDT/protocol layers can be tested without a
//! real network stack.

use crate::protocol::SyncMessage;
use crate::SyncResult;

pub trait Transport {
    fn send(&mut self, to_node: u64, message: SyncMessage) -> SyncResult<()>;
    fn poll(&mut self) -> Vec<(u64, SyncMessage)>; // (from_node, message)
}

/// In-memory transport for unit/integration tests: messages are queued
/// per destination node and drained on `poll()`. No real networking.
#[derive(Default)]
pub struct InMemoryTransport {
    pub local_node: u64,
    inbox: std::collections::VecDeque<(u64, SyncMessage)>,
    // In real use this would be shared (e.g. via a broker); tests wire
    // two `InMemoryTransport`s together manually — see integration tests.
}

impl InMemoryTransport {
    pub fn new(local_node: u64) -> Self {
        Self { local_node, inbox: Default::default() }
    }

    /// Test helper: deliver a message directly into this transport's inbox,
    /// simulating "the network delivered this to me".
    pub fn deliver(&mut self, from_node: u64, message: SyncMessage) {
        self.inbox.push_back((from_node, message));
    }
}

impl Transport for InMemoryTransport {
    fn send(&mut self, _to_node: u64, _message: SyncMessage) -> SyncResult<()> {
        // A real transport would serialize and push to the network.
        // Tests instead call `deliver()` on the *peer's* transport.
        Ok(())
    }

    fn poll(&mut self) -> Vec<(u64, SyncMessage)> {
        self.inbox.drain(..).collect()
    }
}
