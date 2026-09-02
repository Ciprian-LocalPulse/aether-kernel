//! Capability-based access control.
//!
//! Every resource in Aether Kernel (memory region, IPC port, device) is
//! reached only through a `Capability` — an unforgeable token that names
//! the resource and the `Rights` the holder has over it. There is no
//! ambient authority: a process that is not handed a capability simply
//! cannot name, let alone touch, the resource it protects.
//!
//! This mirrors the seL4 / Zircon capability model referenced in the
//! blueprint (§4.2, §4.4).

use bitflags::bitflags; // NOTE: add `bitflags = "2"` to Cargo.toml when implementing.
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CapabilityId(pub u64);

bitflags! {
    /// Rights that a capability may grant over its target resource.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Rights: u32 {
        const READ    = 0b0000_0001;
        const WRITE   = 0b0000_0010;
        const EXECUTE = 0b0000_0100;
        const GRANT   = 0b0000_1000; // may derive/delegate a weaker capability
        const REVOKE  = 0b0001_0000; // may revoke capabilities it derived
    }
}

/// A single capability: a reference to a kernel object plus the rights
/// the holder has over it.
///
/// `epoch` is a snapshot of the target's revocation epoch at the moment
/// this capability was minted or derived. Revoking *any* capability over
/// a target bumps that target's current epoch — which instantly
/// invalidates every capability pointing at it, including copies the
/// kernel never explicitly tracked (e.g. a value a process cached
/// off-table). This closes a real gap in a naive "delete the row from
/// the table" revocation scheme: a cached copy elsewhere would otherwise
/// keep working until something re-checks it against the table.
#[derive(Debug, Clone)]
pub struct Capability {
    pub id: CapabilityId,
    pub target: CapabilityTarget,
    pub rights: Rights,
    pub epoch: u32,
}

/// What kind of kernel object a capability names.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CapabilityTarget {
    MemoryRegion(u64),
    IpcPort(u64),
    Device(String),
    Process(u64),
}

/// The kernel-side capability table: the single source of truth for
/// "who can do what". Not exposed to user-space processes directly —
/// only referenced indirectly via `CapabilityId`s that a process holds.
#[derive(Default)]
pub struct CapabilityTable {
    next_id: u64,
    entries: HashMap<CapabilityId, Capability>,
    /// Current revocation epoch per target. Bumping a target's epoch
    /// invalidates every capability (tracked or not) that names it.
    target_epoch: HashMap<CapabilityTarget, u32>,
}

impl CapabilityTable {
    pub fn new() -> Self {
        Self::default()
    }

    fn current_epoch(&mut self, target: &CapabilityTarget) -> u32 {
        *self.target_epoch.entry(target.clone()).or_insert(0)
    }

    /// Mint a brand-new capability. Only the kernel itself calls this,
    /// typically when a resource (memory region, port, device) is created.
    pub fn mint(&mut self, target: CapabilityTarget, rights: Rights) -> Capability {
        self.next_id += 1;
        let id = CapabilityId(self.next_id);
        let epoch = self.current_epoch(&target);
        let cap = Capability { id, target, rights, epoch };
        self.entries.insert(id, cap.clone());
        cap
    }

    /// Derive a new, weaker capability from an existing one. Fails if the
    /// parent doesn't hold `Rights::GRANT`, if `rights` is not a subset
    /// of the parent's rights (capabilities can only be attenuated, never
    /// amplified), or if the parent itself has already been revoked.
    pub fn derive(
        &mut self,
        parent: CapabilityId,
        rights: Rights,
    ) -> Result<Capability, crate::KernelError> {
        let parent_cap = self
            .entries
            .get(&parent)
            .ok_or(crate::KernelError::CapabilityNotFound(parent))?
            .clone();

        self.check(parent, Rights::GRANT)?;
        if !parent_cap.rights.contains(rights) {
            return Err(crate::KernelError::AccessDenied {
                cap: parent,
                needed: rights,
            });
        }

        Ok(self.mint(parent_cap.target, rights))
    }

    /// Revoke a capability's target. This bumps the *target's* epoch,
    /// which invalidates every capability over that target — not just
    /// `id` — including any derived children and any copy a process
    /// cached outside the table. This is the property a naive
    /// "remove one row" revocation scheme cannot provide.
    pub fn revoke(&mut self, id: CapabilityId) {
        if let Some(cap) = self.entries.get(&id).cloned() {
            let epoch = self.target_epoch.entry(cap.target.clone()).or_insert(0);
            *epoch += 1;
        }
        self.entries.remove(&id);
    }

    pub fn check(&mut self, id: CapabilityId, needed: Rights) -> Result<(), crate::KernelError> {
        let cap = self
            .entries
            .get(&id)
            .ok_or(crate::KernelError::CapabilityNotFound(id))?
            .clone();

        let current = self.current_epoch(&cap.target);
        if cap.epoch != current {
            return Err(crate::KernelError::CapabilityRevoked(id));
        }
        if !cap.rights.contains(needed) {
            return Err(crate::KernelError::AccessDenied { cap: id, needed });
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn derived_capability_cannot_exceed_parent_rights() {
        let mut table = CapabilityTable::new();
        let parent = table.mint(
            CapabilityTarget::MemoryRegion(0),
            Rights::READ | Rights::GRANT,
        );
        let result = table.derive(parent.id, Rights::WRITE);
        assert!(result.is_err(), "must not be able to escalate rights");
    }

    #[test]
    fn derived_capability_within_parent_rights_succeeds() {
        let mut table = CapabilityTable::new();
        let parent = table.mint(
            CapabilityTarget::MemoryRegion(0),
            Rights::READ | Rights::GRANT,
        );
        let child = table.derive(parent.id, Rights::READ).unwrap();
        assert!(table.check(child.id, Rights::READ).is_ok());
    }

    #[test]
    fn revoking_target_invalidates_every_capability_over_it() {
        let mut table = CapabilityTable::new();
        let parent = table.mint(
            CapabilityTarget::MemoryRegion(0),
            Rights::READ | Rights::GRANT,
        );
        let child = table.derive(parent.id, Rights::READ).unwrap();

        table.revoke(parent.id);

        // The child was never revoked directly, but it named the same
        // target — its epoch is now stale, so it must be rejected too.
        assert!(matches!(
            table.check(child.id, Rights::READ),
            Err(crate::KernelError::CapabilityRevoked(_))
        ));
    }

    #[test]
    fn revoking_target_invalidates_an_off_table_cached_copy() {
        // Simulates a process that cached a `Capability` value directly
        // (e.g. stored the struct, not just re-checked the id every
        // time) — the exact scenario a "delete the row" scheme misses.
        let mut table = CapabilityTable::new();
        let cap = table.mint(CapabilityTarget::Device("camera0".into()), Rights::READ);
        let cached_copy = cap.clone();

        table.revoke(cap.id);

        let current = table.current_epoch(&cached_copy.target);
        assert_ne!(
            cached_copy.epoch, current,
            "cached copy's epoch must be stale after revocation"
        );
    }

    #[test]
    fn fresh_capability_over_a_previously_revoked_target_is_valid() {
        // After revocation, minting a *new* capability over the same
        // target picks up the current (bumped) epoch, so it is not
        // accidentally invalidated by the old revocation.
        let mut table = CapabilityTable::new();
        let first = table.mint(CapabilityTarget::MemoryRegion(7), Rights::READ);
        table.revoke(first.id);

        let second = table.mint(CapabilityTarget::MemoryRegion(7), Rights::READ);
        assert!(table.check(second.id, Rights::READ).is_ok());
    }
}
