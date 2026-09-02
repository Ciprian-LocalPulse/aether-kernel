// Mirror of node-a: applies the same two updates in the OPPOSITE order
// to demonstrate that the LwwRegister CRDT (ordered by HLC stamps)
// converges regardless of delivery order. See ../README.md.
use aether_sync_engine::{Hlc, HybridLogicalClock, LwwRegister};

fn main() {
    let mut clock = HybridLogicalClock::new(/* node id */ 2);

    let stamp0 = clock.tick(0);
    let mut cube_position = LwwRegister::new((0.0, 0.0, 0.0), stamp0);
    println!(
        "[node-b] local cube position: {:?} @ {:?}",
        cube_position.value, cube_position.stamp
    );

    // Node B receives node A's move BEFORE applying its own — opposite
    // arrival order from node-a's perspective. The register merge uses
    // A's original stamp, unmodified.
    let remote_stamp_from_node_a = Hlc::new(1, 0, /* node id */ 1);
    let remote_move_from_node_a = LwwRegister::new((1.0, 0.0, 0.0), remote_stamp_from_node_a);
    cube_position.merge(remote_move_from_node_a);
    println!(
        "[node-b] received remote update from node-a -> {:?} @ {:?}",
        cube_position.value, cube_position.stamp
    );

    // Node B's own clock is advanced past A's observed stamp first —
    // this is what guarantees B's *own* next local write is correctly
    // ordered after A's, even though B is about to generate that stamp
    // using only its own (possibly skewed) physical clock reading.
    clock.update(&remote_stamp_from_node_a, 1);

    let local_stamp = clock.tick(2);
    let local_update = LwwRegister::new((1.0, 2.0, 0.0), local_stamp);
    cube_position.merge(local_update);
    println!(
        "[node-b] applying local update -> {:?} @ {:?}",
        cube_position.value, cube_position.stamp
    );

    println!("[node-b] converged state: {:?}", cube_position.value);
}
