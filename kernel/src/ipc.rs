//! Asynchronous, (conceptually) zero-copy inter-process communication.
//! Blueprint reference: §4.2, §4.3.
//!
//! This scaffold models IPC as bounded channels over `Port`s. A real
//! implementation would back this with shared-memory ring buffers and
//! `io_uring`/eBPF-filtered delivery, as described in the blueprint.

use crate::KernelResult;
use std::collections::{HashMap, VecDeque};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Port(pub u64);

#[derive(Debug, Clone)]
pub struct IpcMessage {
    pub from: Port,
    pub payload: Vec<u8>,
}

#[derive(Default)]
pub struct IpcBus {
    next_port: u64,
    mailboxes: HashMap<Port, VecDeque<IpcMessage>>,
}

impl IpcBus {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn create_port(&mut self) -> Port {
        self.next_port += 1;
        let port = Port(self.next_port);
        self.mailboxes.insert(port, VecDeque::new());
        port
    }

    pub fn send(&mut self, to: Port, from: Port, payload: Vec<u8>) -> KernelResult<()> {
        let mailbox = self
            .mailboxes
            .get_mut(&to)
            .ok_or(crate::KernelError::PortClosed)?;
        mailbox.push_back(IpcMessage { from, payload });
        Ok(())
    }

    pub fn receive(&mut self, on: Port) -> Option<IpcMessage> {
        self.mailboxes.get_mut(&on)?.pop_front()
    }

    pub fn close(&mut self, port: Port) {
        self.mailboxes.remove(&port);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn send_then_receive_round_trip() {
        let mut bus = IpcBus::new();
        let a = bus.create_port();
        let b = bus.create_port();
        bus.send(b, a, b"hello".to_vec()).unwrap();
        let msg = bus.receive(b).unwrap();
        assert_eq!(msg.payload, b"hello");
        assert_eq!(msg.from, a);
    }
}
