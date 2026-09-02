//! Hybrid Logical Clock (HLC).
//!
//! Blueprint reference: §8.2, citing Kulkarni et al. (2014), "Logical
//! Physical Clocks and Consistent Snapshots in Globally Distributed
//! Databases."
//!
//! A plain `(wall_clock_timestamp, node_id)` pair — what the earlier
//! version of this crate used to order CRDT writes — has a real gap:
//! if two nodes' physical clocks are skewed, a causally-later write can
//! carry an *earlier* wall-clock timestamp and lose a Last-Writer-Wins
//! merge it should have won. An HLC fixes this by carrying a logical
//! counter alongside physical time, and by bumping that counter (rather
//! than trusting physical time alone) whenever it observes a stamp from
//! another node that's ahead of what physical time alone would suggest.
//! The result: HLC order never contradicts causal (happened-before)
//! order, regardless of clock skew, while still degrading gracefully to
//! "roughly wall-clock order" when clocks *are* in sync.

use serde::{Deserialize, Serialize};

/// A single Hybrid Logical Clock stamp. Ordered lexicographically by
/// `(physical_ns, logical, node_id)` — the derived `Ord` below gives
/// exactly that, and the `node_id` tiebreaker guarantees a total order
/// (no two distinct stamps compare equal) so CRDT merges are
/// deterministic even when two nodes stamp the same physical instant.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Hlc {
    pub physical_ns: u64,
    pub logical: u32,
    pub node_id: u64,
}

impl Hlc {
    pub fn new(physical_ns: u64, logical: u32, node_id: u64) -> Self {
        Self { physical_ns, logical, node_id }
    }
}

/// Generates monotonically-increasing `Hlc` stamps for one node,
/// following the standard HLC algorithm. The caller supplies the
/// current physical time (`now_ns`) rather than this type reading a
/// system clock itself, which keeps it deterministic and unit-testable
/// — production code wires `now_ns` to `SystemTime`/PTP/NTP-disciplined
/// time.
#[derive(Debug, Clone)]
pub struct HybridLogicalClock {
    node_id: u64,
    last_physical: u64,
    last_logical: u32,
}

impl HybridLogicalClock {
    pub fn new(node_id: u64) -> Self {
        Self { node_id, last_physical: 0, last_logical: 0 }
    }

    /// Produce a stamp for a local event (e.g. this node editing an
    /// object it owns).
    pub fn tick(&mut self, now_ns: u64) -> Hlc {
        let new_physical = self.last_physical.max(now_ns);
        let new_logical = if new_physical == self.last_physical {
            self.last_logical + 1
        } else {
            0
        };
        self.last_physical = new_physical;
        self.last_logical = new_logical;
        Hlc::new(new_physical, new_logical, self.node_id)
    }

    /// Merge in a stamp received from another node (e.g. a `PUBLISH`
    /// arriving over the sync-engine transport), producing a new local
    /// stamp that is guaranteed to be causally after both this node's
    /// prior state and the received stamp.
    pub fn update(&mut self, received: &Hlc, now_ns: u64) -> Hlc {
        let new_physical = self.last_physical.max(received.physical_ns).max(now_ns);

        let new_logical = if new_physical == self.last_physical && new_physical == received.physical_ns {
            self.last_logical.max(received.logical) + 1
        } else if new_physical == self.last_physical {
            self.last_logical + 1
        } else if new_physical == received.physical_ns {
            received.logical + 1
        } else {
            0
        };

        self.last_physical = new_physical;
        self.last_logical = new_logical;
        Hlc::new(new_physical, new_logical, self.node_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn local_ticks_are_strictly_increasing() {
        let mut clock = HybridLogicalClock::new(1);
        let a = clock.tick(100);
        let b = clock.tick(100); // same physical instant again
        assert!(b > a, "second tick at the same physical time must still advance");
    }

    #[test]
    fn skewed_remote_clock_does_not_go_backwards_in_time() {
        // Node 1's physical clock is ahead; node 2 receives a stamp from
        // node 1 while node 2's own physical clock (skewed behind) still
        // reads an earlier value. The merged stamp must not regress.
        let mut node1 = HybridLogicalClock::new(1);
        let stamp_from_node1 = node1.tick(1_000_000); // node 1 "sees" t=1ms

        let mut node2 = HybridLogicalClock::new(2);
        let node2_local_before = node2.tick(100_000); // node 2's clock reads only t=0.1ms

        let merged = node2.update(&stamp_from_node1, 100_000);
        assert!(
            merged > stamp_from_node1,
            "merged stamp must be causally after the received remote stamp"
        );
        assert!(
            merged > node2_local_before,
            "merged stamp must be causally after this node's own prior stamp"
        );
    }

    #[test]
    fn a_naive_wall_clock_timestamp_would_have_gotten_this_wrong() {
        // This is the concrete failure mode HLC avoids: node A's naive
        // physical timestamp for a *later* (causally dependent) write is
        // numerically smaller than node B's *earlier* write, because A's
        // clock is skewed behind B's. A plain (timestamp, node_id) LWW
        // register would pick B's earlier write as the "winner".
        let node_a_naive_wall_clock_ns: u64 = 500_000; // A's clock is behind
        let node_b_naive_wall_clock_ns: u64 = 900_000; // B's clock is ahead
        assert!(
            node_a_naive_wall_clock_ns < node_b_naive_wall_clock_ns,
            "setup: A's naive timestamp looks earlier than B's, despite A's write happening after"
        );

        // With HLC: B's write is observed by A before A issues its own
        // write, so A's HLC-merged stamp correctly ends up after B's.
        let mut node_b = HybridLogicalClock::new(2);
        let b_stamp = node_b.tick(node_b_naive_wall_clock_ns);

        let mut node_a = HybridLogicalClock::new(1);
        let a_stamp_after_observing_b = node_a.update(&b_stamp, node_a_naive_wall_clock_ns);

        assert!(
            a_stamp_after_observing_b > b_stamp,
            "HLC correctly orders A's causally-later write after B's, despite A's skewed clock"
        );
    }
}
