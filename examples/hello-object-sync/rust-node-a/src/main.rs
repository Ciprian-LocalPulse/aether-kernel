// Stage 0 demo: two nodes converge on a cube's position via the
// sync-engine's LwwRegister CRDT. See ../README.md.
use aether_sync_engine::LwwRegister;

fn main() {
    // Node A creates the object at the origin, t=0.
    let mut cube_position = LwwRegister::new((0.0, 0.0, 0.0), 0, /* node id */ 1);
    println!(
        "[node-a] local cube position: {:?} @ t=0",
        cube_position.value
    );

    // Node A moves the cube itself at t=1.
    let local_move = LwwRegister::new((1.0, 0.0, 0.0), 1, 1);
    cube_position.merge(local_move);
    println!(
        "[node-a] applying local move -> {:?} @ t=1",
        cube_position.value
    );

    // Node B's concurrent update arrives at t=2 (simulating what a real
    // Transport would deliver over the wire — see sync-engine/src/network.rs).
    let remote_update_from_node_b = LwwRegister::new((1.0, 2.0, 0.0), 2, /* node id */ 2);
    cube_position.merge(remote_update_from_node_b);
    println!(
        "[node-a] received remote update from node-b -> {:?} @ t=2",
        cube_position.value
    );

    println!("[node-a] converged state: {:?}", cube_position.value);
}
