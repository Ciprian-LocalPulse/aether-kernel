//! CRDT primitives used by the scene graph: a Last-Writer-Wins register
//! for pose/transform data, and an Observed-Remove Set for tag/attribute
//! collections. Blueprint reference: §6.2, §6.3, §8.2.

use crate::hlc::Hlc;
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
/// where the freshest write always wins on merge. Ordered by a Hybrid
/// Logical Clock stamp (see `crate::hlc`) rather than a raw wall-clock
/// timestamp, so merges stay causally correct even under clock skew
/// between nodes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LwwRegister<T: Clone> {
    pub value: T,
    pub stamp: Hlc,
}

impl<T: Clone> LwwRegister<T> {
    pub fn new(value: T, stamp: Hlc) -> Self {
        Self { value, stamp }
    }

    /// Merge an incoming update. `Hlc`'s total order (physical time,
    /// then logical counter, then node id as a final tiebreaker) makes
    /// this pure, commutative, and idempotent — the defining CRDT
    /// property — while also guaranteeing the winner is never a
    /// causally-earlier write, which a raw-timestamp comparison cannot
    /// guarantee under clock skew.
    pub fn merge(&mut self, other: LwwRegister<T>) {
        if other.stamp > self.stamp {
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
        let mut a = LwwRegister::new((0.0, 0.0, 0.0), Hlc::new(1, 0, 1));
        let b = LwwRegister::new((1.0, 1.0, 1.0), Hlc::new(2, 0, 1));
        a.merge(b.clone());
        assert_eq!(a.value, b.value);
    }

    #[test]
    fn lww_register_ignores_stale_write() {
        let mut a = LwwRegister::new((5.0, 5.0, 5.0), Hlc::new(10, 0, 1));
        let stale = LwwRegister::new((0.0, 0.0, 0.0), Hlc::new(3, 0, 1));
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
