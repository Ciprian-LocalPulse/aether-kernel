//! Aether Temporal Sync Engine
//!
//! CRDT-based synchronization of the distributed scene graph. See
//! `docs/whitepaper/AETHER_KERNEL_BLUEPRINT.md` §6.
//!
//! Copyright 2026 Ciprian Ștefan Pleșca — Apache License 2.0

pub mod crdt;
pub mod network;
pub mod protocol;
pub mod sharding;

pub use crdt::{LwwRegister, ObjectId, OrSet};
pub use protocol::{SyncMessage, VectorClock};
pub use sharding::CellId;

#[derive(thiserror::Error, Debug)]
pub enum SyncError {
    #[error("object not found: {0:?}")]
    ObjectNotFound(ObjectId),
    #[error("transport error: {0}")]
    Transport(String),
    #[error("stale write rejected (incoming ts {incoming} <= current {current})")]
    StaleWrite { incoming: u64, current: u64 },
}

pub type SyncResult<T> = Result<T, SyncError>;
