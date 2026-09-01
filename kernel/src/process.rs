//! Process lifecycle: the kernel's minimal "syscall-like" API surface
//! (`create_process`, capability assignment, teardown). Blueprint
//! reference: §4.3.

use crate::capability::CapabilityId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ProcessId(pub u64);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProcessState {
    Created,
    Runnable,
    Blocked,
    Terminated,
}

#[derive(Debug, Clone)]
pub struct Process {
    pub id: ProcessId,
    pub name: String,
    pub state: ProcessState,
    pub capabilities: Vec<CapabilityId>,
}

#[derive(Default)]
pub struct ProcessTable {
    next_id: u64,
    processes: Vec<Process>,
}

impl ProcessTable {
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a process with an initial capability set. Mirrors the
    /// blueprint's `create_process(image, caps) -> ProcessId` kernel API.
    pub fn create_process(&mut self, name: impl Into<String>, caps: Vec<CapabilityId>) -> ProcessId {
        self.next_id += 1;
        let id = ProcessId(self.next_id);
        self.processes.push(Process {
            id,
            name: name.into(),
            state: ProcessState::Created,
            capabilities: caps,
        });
        id
    }

    pub fn set_state(&mut self, id: ProcessId, state: ProcessState) -> crate::KernelResult<()> {
        let p = self
            .processes
            .iter_mut()
            .find(|p| p.id == id)
            .ok_or(crate::KernelError::ProcessNotFound(id))?;
        p.state = state;
        Ok(())
    }

    pub fn get(&self, id: ProcessId) -> Option<&Process> {
        self.processes.iter().find(|p| p.id == id)
    }
}
