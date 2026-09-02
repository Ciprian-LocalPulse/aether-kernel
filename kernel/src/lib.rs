//! Aether Microkernel
//!
//! Capability-based microkernel for the Aether spatial-computing stack.
//! See `docs/whitepaper/AETHER_KERNEL_BLUEPRINT.md` §4 for the full design.
//!
//! Copyright 2026 Ciprian Ștefan Pleșca — Apache License 2.0

pub mod capability;
pub mod ipc;
pub mod memory;
pub mod process;
pub mod scheduler;

pub use capability::{Capability, CapabilityId, Rights};
pub use ipc::{IpcMessage, Port};
pub use memory::{MemoryRegion, Permissions};
pub use process::{Process, ProcessId};
pub use scheduler::{Scheduler, SchedulingClass};

/// Errors that can be raised by any kernel subsystem.
#[derive(thiserror::Error, Debug)]
pub enum KernelError {
    #[error("capability not found: {0:?}")]
    CapabilityNotFound(CapabilityId),
    #[error("access denied: missing rights {needed:?} on capability {cap:?}")]
    AccessDenied { cap: CapabilityId, needed: Rights },
    #[error("capability revoked (target epoch advanced past this capability's snapshot): {0:?}")]
    CapabilityRevoked(CapabilityId),
    #[error("process not found: {0:?}")]
    ProcessNotFound(ProcessId),
    #[error("port not found or closed")]
    PortClosed,
    #[error("out of memory")]
    OutOfMemory,
}

pub type KernelResult<T> = Result<T, KernelError>;
