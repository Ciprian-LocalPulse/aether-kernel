//! Spatial sharding: the global coordinate space is partitioned into
//! cells (conceptually S2 cells, as in the blueprint §6.2) so that a node
//! only subscribes to updates relevant to the space around it, instead of
//! the whole planet.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CellId {
    /// S2-style cell token at a fixed level. Stored as a placeholder u64;
    /// swap in a real S2 geometry library (e.g. `s2` crate) for production.
    pub token: u64,
    pub level: u8,
}

impl CellId {
    /// Deterministically bucket a lat/lon (in micro-degrees, to avoid
    /// floats in the placeholder hash) into a cell at the given level.
    /// This is a stand-in for real S2 cell indexing.
    pub fn from_lat_lon(lat_micro: i64, lon_micro: i64, level: u8) -> Self {
        let grid = 10_i64.pow((6u32).saturating_sub(level as u32).max(0));
        let bucket_lat = lat_micro / grid.max(1);
        let bucket_lon = lon_micro / grid.max(1);
        let token = ((bucket_lat as u64) << 32) ^ (bucket_lon as u64 & 0xFFFF_FFFF);
        Self { token, level }
    }

    /// The coarser-grained parent cell — used to widen a subscription
    /// when moving fast (e.g. a vehicle) to reduce churn.
    pub fn parent(&self) -> Option<CellId> {
        if self.level == 0 {
            None
        } else {
            Some(CellId { token: self.token >> 4, level: self.level - 1 })
        }
    }
}
