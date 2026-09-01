// Mirror of node-a: applies the same two updates in the OPPOSITE order
// to demonstrate that the LwwRegister CRDT converges regardless of
// delivery order. See ../README.md.
use aether_sync_engine::LwwRegister;

fn main() {
    let mut cube_position = LwwRegister::new((0.0, 0.0, 0.0), 0, 1);
    println!(
        "[node-b] local cube position: {:?} @ t=0",
        cube_position.value
    );

    // Node B receives node A's move (t=1) BEFORE applying its own (t=2) —
    // opposite arrival order from node-a's perspective.
    let remote_move_from_node_a = LwwRegister::new((1.0, 0.0, 0.0), 1, 1);
    cube_position.merge(remote_move_from_node_a);
    println!(
        "[node-b] received remote update from node-a -> {:?} @ t=1",
        cube_position.value
    );

    let local_update = LwwRegister::new((1.0, 2.0, 0.0), 2, 2);
    cube_position.merge(local_update);
    println!(
        "[node-b] applying local update -> {:?} @ t=2",
        cube_position.value
    );

    println!("[node-b] converged state: {:?}", cube_position.value);
}
