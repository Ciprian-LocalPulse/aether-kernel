//! Aether SDK (Rust)
//!
//! Client API surface described in blueprint §9.1:
//! `connect`, `create_object`, `update_object`, `subscribe`, `send_intent`.
//!
//! Copyright 2026 Ciprian Ștefan Pleșca — Apache License 2.0

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ObjectId(pub u128);

#[derive(Debug, Clone, Default)]
pub struct Transform {
    pub position: [f64; 3],
    pub orientation: [f64; 4],
}

#[derive(thiserror::Error, Debug)]
pub enum SdkError {
    #[error("not connected")]
    NotConnected,
    #[error("transport error: {0}")]
    Transport(String),
}

pub type SdkResult<T> = Result<T, SdkError>;

/// A connection to an Aether node. This scaffold is transport-agnostic:
/// a real implementation would hold a `sync-engine` `Transport` (QUIC)
/// under the hood. See `sync-engine/src/network.rs`.
pub struct AetherClient {
    connected: bool,
}

impl AetherClient {
    pub fn connect(_endpoint: &str) -> SdkResult<Self> {
        // TODO(roadmap Stage 3): dial the sync-engine over QUIC.
        Ok(Self { connected: true })
    }

    pub fn create_object(&self, parent: Option<ObjectId>, transform: Transform) -> SdkResult<ObjectId> {
        if !self.connected {
            return Err(SdkError::NotConnected);
        }
        let _ = (parent, transform);
        // TODO: publish a `CrdtOp` via the sync-engine and return its id.
        Ok(ObjectId(0))
    }

    pub fn update_object(&self, id: ObjectId, transform: Transform) -> SdkResult<()> {
        if !self.connected {
            return Err(SdkError::NotConnected);
        }
        let _ = (id, transform);
        Ok(())
    }

    pub fn send_intent(&self, text: &str) -> SdkResult<()> {
        if !self.connected {
            return Err(SdkError::NotConnected);
        }
        let _ = text;
        // TODO: forward to the Intent Router (HTTP/gRPC bridge).
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn connect_then_create_object_succeeds() {
        let client = AetherClient::connect("aether://localhost:9000").unwrap();
        let id = client.create_object(None, Transform::default()).unwrap();
        assert_eq!(id.0, 0);
    }
}
