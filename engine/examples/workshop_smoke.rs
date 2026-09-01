//! Headless smoke test for the release workshop world.
//!
//! Deserializes engine/assets/WORKSHOP.world,
//! builds a `State`, and steps 30 years without panicking.
//!
//! Run: cargo run -p hes-engine --example workshop_smoke

use hes_engine::{State, World};

fn main() {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/assets/WORKSHOP.world");
    let json = std::fs::read_to_string(path).expect("read WORKSHOP.world");
    let world: World = serde_json::from_str(&json).expect("deserialize World");

    let unlocked: Vec<_> = world
        .projects
        .iter()
        .filter(|p| !p.locked)
        .map(|p| p.name.clone())
        .collect();
    let workshop_cards = [
        "Solar Push",
        "Wind Push",
        "Nuclear Expansion",
        "Phase Out Coal",
        "Mass Electrification",
        "Energy Quotas",
        "Crack Down on Crypto-Mining",
        "Vegetarian Mandate",
        "Meatless Mondays",
        "Cellular Meat",
        "Organic Transition",
        "Regenerative Agriculture",
        "Expand Nature Preserves",
        "Remediate and Protect Ecosystems",
        "Ban Outdoor Cats",
        "Solar Radiation Management (SRM)",
        "Expand Public Transit",
        "Ban Cars",
        "Restrict Air Travel",
        "Degrowth in Developed Regions",
        "Luxury for All",
    ];
    assert_eq!(
        world
            .projects
            .iter()
            .filter(|project| workshop_cards.contains(&project.name.as_str()))
            .count(),
        21,
        "expected exactly the 21 accepted workshop cards"
    );
    assert_eq!(unlocked.len(), 18, "expected 18 unlocked cards");
    assert_eq!(world.events.len(), 0, "expected zero events");
    assert_eq!(world.year, 2022, "expected workshop anchor year");
    assert_eq!(world.lifespan, 30, "expected workshop lifespan");

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
