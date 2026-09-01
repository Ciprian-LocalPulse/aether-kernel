//! CRDT primitives used by the scene graph: a Last-Writer-Wins register
//! for pose/transform data, and an Observed-Remove Set for tag/attribute
//! collections. Blueprint reference: §6.2, §6.3.

use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ObjectId(pub uuid_stub::Uuid);

/// Minimal UUID stand-in so this module has no external UUID dependency
/// at scaffold stage. Swap for the `uuid` crate when wiring up real IDs.
pub mod uuid_stub {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct Uuid(pub u128);
}

/// Last-Writer-Wins register: used for pose (position/orientation) data,
/// where the freshest timestamped write always wins on merge. Timestamps
/// should be Hybrid Logical Clocks in production; a plain `u64` "logical
/// time" stands in here.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LwwRegister<T: Clone> {
    pub value: T,
    pub timestamp: u64,
    pub writer_node: u64,
}

impl<T: Clone> LwwRegister<T> {
    pub fn new(value: T, timestamp: u64, writer_node: u64) -> Self {
        Self { value, timestamp, writer_node }
    }

    /// Merge an incoming update. Ties on timestamp are broken by node id
    /// (deterministic, so all replicas converge to the same winner).
    pub fn merge(&mut self, other: LwwRegister<T>) {
        let other_wins = (other.timestamp, other.writer_node)
            > (self.timestamp, self.writer_node);
        if other_wins {
            *self = other;
        }
    }
}

/// Observed-Remove Set: used for tag/attribute collections where adds and
/// removes from different replicas must converge without "remove wins"
/// or "add wins" surprises. Each element carries the set of unique add-tags
/// that introduced it; a remove only takes effect against tags it has
/// actually observed.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OrSet<T: std::hash::Hash + Eq + Clone> {
    adds: std::collections::HashMap<T, HashSet<u128>>,
    removes: std::collections::HashMap<T, HashSet<u128>>,
}

impl<T: std::hash::Hash + Eq + Clone> OrSet<T> {
    pub fn new() -> Self {
        Self { adds: Default::default(), removes: Default::default() }
    }

    pub fn add(&mut self, element: T, tag: u128) {
        self.adds.entry(element).or_default().insert(tag);
    }

    pub fn remove(&mut self, element: &T) {
        if let Some(tags) = self.adds.get(element).cloned() {
            self.removes.entry(element.clone()).or_default().extend(tags);
        }
    }

    pub fn contains(&self, element: &T) -> bool {
        match (self.adds.get(element), self.removes.get(element)) {
            (Some(add_tags), Some(remove_tags)) => !add_tags.is_subset(remove_tags),
            (Some(_), None) => true,
            _ => false,
        }
    }

    pub fn merge(&mut self, other: OrSet<T>) {
        for (elem, tags) in other.adds {
            self.adds.entry(elem).or_default().extend(tags);
        }
        for (elem, tags) in other.removes {
            self.removes.entry(elem).or_default().extend(tags);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lww_register_prefers_later_timestamp() {
        let mut a = LwwRegister::new((0.0, 0.0, 0.0), 1, 1);
        let b = LwwRegister::new((1.0, 1.0, 1.0), 2, 1);
        a.merge(b.clone());
        assert_eq!(a.value, b.value);
    }

    #[test]
    fn lww_register_ignores_stale_write() {
        let mut a = LwwRegister::new((5.0, 5.0, 5.0), 10, 1);
        let stale = LwwRegister::new((0.0, 0.0, 0.0), 3, 1);
        a.merge(stale);
        assert_eq!(a.value, (5.0, 5.0, 5.0));
    }

    #[test]
    fn or_set_add_remove_converges() {
        let mut a = OrSet::new();
        a.add("fragile", 1);
        let mut b = a.clone();
        b.remove(&"fragile");
        a.merge(b);
        assert!(!a.contains(&"fragile"));
    }

    #[test]
    fn or_set_concurrent_add_after_remove_survives() {
        // Classic OR-Set property: a *new* add (new tag) after a remove of
        // an old tag must survive the merge — this is what distinguishes
        // OR-Set from a naive last-write-wins boolean.
        let mut replica_a = OrSet::new();
        replica_a.add("beacon", 1);

        let mut replica_b = replica_a.clone();
        replica_b.remove(&"beacon"); // removes tag 1

        replica_a.add("beacon", 2); // concurrent re-add with a new tag

        replica_a.merge(replica_b);
        assert!(replica_a.contains(&"beacon"));
    }
}
