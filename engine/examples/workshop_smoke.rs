//! Headless smoke test for the M1 workshop world.
//!
//! Deserializes references/simplification/worlds/workshop-m1.world,
//! builds a `State`, and steps 30 years without panicking.
//!
//! Run: cargo run -p hes-engine --example workshop_smoke

use hes_engine::{State, World};

fn main() {
    let path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../references/simplification/worlds/workshop-m1.world"
    );
    let json = std::fs::read_to_string(path).expect("read workshop-m1.world");
    let world: World = serde_json::from_str(&json).expect("deserialize World");

    let unlocked: Vec<_> = world
        .projects
        .iter()
        .filter(|p| !p.locked)
        .map(|p| p.name.clone())
        .collect();
    assert_eq!(unlocked.len(), 13, "expected 13 unlocked cards");
    assert_eq!(world.events.len(), 0, "expected zero events");

    let mut state = State::new(world);
    let tgav = 1.2678074;
    for i in 0..30 {
        let updates = state.step_year(tgav);
        println!(
            "year {} ({}): {} updates, temp {:.2}",
            i + 1,
            state.world.year,
            updates.len(),
            state.world.temperature
        );
    }
    println!("OK: stepped 30 years without panic");
}
