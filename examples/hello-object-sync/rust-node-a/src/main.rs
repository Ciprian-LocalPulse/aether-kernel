// Stage 0 demo: two nodes converge on a cube's position via the
// sync-engine's LwwRegister CRDT, ordered by Hybrid Logical Clock (HLC)
// stamps. See ../README.md.
use aether_sync_engine::{Hlc, HybridLogicalClock, LwwRegister};

fn main() {
    let mut clock = HybridLogicalClock::new(/* node id */ 1);

    // Node A creates the object at the origin.
    let stamp0 = clock.tick(0);
    let mut cube_position = LwwRegister::new((0.0, 0.0, 0.0), stamp0);
    println!(
        "[node-a] local cube position: {:?} @ {:?}",
        cube_position.value, cube_position.stamp
    );

    // Node A moves the cube itself.
    let stamp1 = clock.tick(1);
    let local_move = LwwRegister::new((1.0, 0.0, 0.0), stamp1);
    cube_position.merge(local_move);
    println!(
        "[node-a] applying local move -> {:?} @ {:?}",
        cube_position.value, cube_position.stamp
    );

    // Node B's concurrent update arrives, carrying B's *own* original
    // HLC stamp (this is what the real Aether Sync `PUBLISH` message
    // carries on the wire — see sync-engine/src/protocol.rs). The
    // register merge compares against that original stamp, unmodified —
    // relabeling it with node A's own clock would make node A's and
    // node B's records of "the same write" diverge, breaking
    // convergence. (This exact stamp is what node-b actually computes
    // for its own local update — see rust-node-b/src/main.rs.)
    let remote_stamp_from_node_b = Hlc::new(2, 0, /* node id */ 2);
    let remote_update_from_node_b = LwwRegister::new((1.0, 2.0, 0.0), remote_stamp_from_node_b);
    cube_position.merge(remote_update_from_node_b);
    println!(
        "[node-a] received remote update from node-b -> {:?} @ {:?}",
        cube_position.value, cube_position.stamp
    );

    // Separately, node A's *own* clock is advanced past what it just
    // observed, so any future local event it generates is guaranteed to
    // be ordered causally after node B's write — this is the second half
    // of what an HLC gives you, distinct from the merge comparison above.
    let next_local_stamp = clock.update(&remote_stamp_from_node_b, 2);
    println!(
        "[node-a] local clock advanced past node-b's stamp; next local write would be @ {:?}",
        next_local_stamp
    );

    println!("[node-a] converged state: {:?}", cube_position.value);
}
