//! Address-space isolation and controlled, capability-gated memory
//! sharing between processes. Blueprint reference: §4.2, §4.3.

use crate::capability::{CapabilityId, Rights};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Permissions {
    pub read: bool,
    pub write: bool,
    pub execute: bool,
}

impl Permissions {
    pub fn from_rights(rights: Rights) -> Self {
        Self {
            read: rights.contains(Rights::READ),
            write: rights.contains(Rights::WRITE),
            execute: rights.contains(Rights::EXECUTE),
        }
    }
}

/// A contiguous region of (virtual, in this scaffold: simulated) memory
/// owned by exactly one process at creation time and shared only through
/// explicitly derived capabilities.
#[derive(Debug, Clone)]
pub struct MemoryRegion {
    pub id: u64,
    pub owner_cap: CapabilityId,
    pub size_bytes: usize,
    pub permissions: Permissions,
}

/// Tracks all memory regions in the system. A real kernel would back this
/// with actual page-table manipulation; here it models the *policy*
/// (who owns what, who it's shared with, under which permissions) so the
/// capability discipline can be tested independently of the MMU layer.
#[derive(Default)]
pub struct MemoryManager {
    next_region_id: u64,
    regions: Vec<MemoryRegion>,
}

impl MemoryManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn allocate(
        &mut self,
        owner_cap: CapabilityId,
        size_bytes: usize,
        permissions: Permissions,
    ) -> MemoryRegion {
        self.next_region_id += 1;
        let region = MemoryRegion {
            id: self.next_region_id,
            owner_cap,
            size_bytes,
            permissions,
        };
        self.regions.push(region.clone());
        region
    }

    pub fn total_allocated(&self) -> usize {
        self.regions.iter().map(|r| r.size_bytes).sum()
    }
}
