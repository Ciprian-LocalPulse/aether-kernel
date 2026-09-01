//! Aether Sync wire protocol: the message set exchanged over the
//! transport (QUIC/WebTransport in production). Blueprint reference: §6.3.

use crate::crdt::ObjectId;
use crate::sharding::CellId;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A per-object vector clock, used by `SYNC_STATE` to describe what a
/// node already has so the peer can send only the missing deltas.
pub type VectorClock = HashMap<u64, u64>; // node_id -> last-seen logical time

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SyncMessage {
    /// Handshake: protocol version + capabilities negotiation.
    Hello { node_id: u64, protocol_version: u32 },

    /// Subscribe to updates for a spatial cell.
    Subscribe { cell: CellId },

    /// Publish a CRDT operation against an object.
    Publish {
        object: ObjectId,
        cell: CellId,
        operation: CrdtOp,
    },

    /// Request the current state of an object, describing what the
    /// requester already has via a vector clock (for delta sync).
    SyncState { object: ObjectId, have: VectorClock },

    /// Liveness + latency measurement.
    Heartbeat { sent_at_ns: u64 },
}

/// The operation payloads that can ride inside a `Publish` message.
/// Kept intentionally generic (opaque bytes for the CRDT payload) so the
/// wire protocol doesn't need to know about every CRDT type the scene
/// graph will eventually use.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CrdtOp {
    PoseUpdate { payload: Vec<u8> }, // serialized LwwRegister<Pose>
    TagAdd { tag: String, op_id: u128 },
    TagRemove { tag: String },
}
