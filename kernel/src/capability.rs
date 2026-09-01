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
#[derive(Debug, Clone)]
pub struct Capability {
    pub id: CapabilityId,
    pub target: CapabilityTarget,
    pub rights: Rights,
}

/// What kind of kernel object a capability names.
#[derive(Debug, Clone, PartialEq, Eq)]
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
}

impl CapabilityTable {
    pub fn new() -> Self {
        Self::default()
    }

    /// Mint a brand-new capability. Only the kernel itself calls this,
    /// typically when a resource (memory region, port, device) is created.
    pub fn mint(&mut self, target: CapabilityTarget, rights: Rights) -> Capability {
        self.next_id += 1;
        let id = CapabilityId(self.next_id);
        let cap = Capability { id, target, rights };
        self.entries.insert(id, cap.clone());
        cap
    }

    /// Derive a new, weaker capability from an existing one. Fails if the
    /// parent doesn't hold `Rights::GRANT` or if `rights` is not a subset
    /// of the parent's rights (capabilities can only be attenuated, never
    /// amplified).
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

        if !parent_cap.rights.contains(Rights::GRANT) {
            return Err(crate::KernelError::AccessDenied {
                cap: parent,
                needed: Rights::GRANT,
            });
        }
        if !parent_cap.rights.contains(rights) {
            return Err(crate::KernelError::AccessDenied {
                cap: parent,
                needed: rights,
            });
        }

        Ok(self.mint(parent_cap.target, rights))
    }

    pub fn revoke(&mut self, id: CapabilityId) {
        self.entries.remove(&id);
    }

    pub fn check(&self, id: CapabilityId, needed: Rights) -> Result<(), crate::KernelError> {
        let cap = self
            .entries
            .get(&id)
            .ok_or(crate::KernelError::CapabilityNotFound(id))?;
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
}
