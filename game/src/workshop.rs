//! Workshop mode: a first-class, simplified session shape for
//! facilitated group workshops (see `references/simplification/spec.md`).
//!
//! When active:
//! - No intro, no tutorial, no story/world events, no parliament majorities.
//! - The world phase between cycles is a fast simulation tick.
//! - The game runs to the loaded world's configured end year, where the usual
//!   ending/evaluation is shown; mid-game loss is disabled.
//!
//! Activated via the `WORKSHOP=1` env var (native) or the `?workshop=1`
//! query param (web), mirroring the debug flag mechanism in `debug.rs`.

use std::{collections::BTreeMap, sync::LazyLock};

use hes_engine::{Effect, Flag, Id, Project, ProjectType, State};

use crate::{
    consts,
    state::{PlanChange, StateExt},
};

pub static WORKSHOP: LazyLock<WorkshopOpts> = LazyLock::new(WorkshopOpts::default);

pub struct WorkshopOpts {
    /// Whether workshop mode is active.
    pub active: bool,
}

/// Presentation-only themes for the accepted workshop deck. These deliberately
/// do not reuse the legacy `Project.group` values, which split the 21 cards into
/// categories that are too narrow for a moderated discussion.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum WorkshopTheme {
    Energy,
    FoodAndAgriculture,
    LandAndBiodiversity,
    IndustryTransportAndGeoengineering,
    SocietyAndEconomy,
}

impl WorkshopTheme {
    pub const ALL: [Self; 5] = [
        Self::Energy,
        Self::FoodAndAgriculture,
        Self::LandAndBiodiversity,
        Self::IndustryTransportAndGeoengineering,
        Self::SocietyAndEconomy,
    ];

    pub const fn title(self) -> &'static str {
        match self {
            Self::Energy => "Energy",
            Self::FoodAndAgriculture => "Food & Agriculture",
            Self::LandAndBiodiversity => "Land & Biodiversity",
            Self::IndustryTransportAndGeoengineering => "Industry, Transport & Geoengineering",
            Self::SocietyAndEconomy => "Society & Economy",
        }
    }
}

/// Map every accepted workshop card to exactly one moderator-facing theme.
/// Returning `None` keeps this mapping safely workshop-only if called with a
/// normal-world project.
pub fn workshop_theme(project: &Project) -> Option<WorkshopTheme> {
    match project.name.as_str() {
        "Solar Push"
        | "Wind Push"
        | "Nuclear Expansion"
        | "Phase Out Coal"
        | "Mass Electrification"
        | "Energy Quotas"
        | "Crack Down on Crypto-Mining" => Some(WorkshopTheme::Energy),
        "Vegetarian Mandate"
        | "Meatless Mondays"
        | "Cellular Meat"
        | "Organic Transition"
        | "Regenerative Agriculture" => Some(WorkshopTheme::FoodAndAgriculture),
        "Expand Nature Preserves" | "Remediate and Protect Ecosystems" | "Ban Outdoor Cats" => {
            Some(WorkshopTheme::LandAndBiodiversity)
        }
        "Solar Radiation Management (SRM)"
        | "Expand Public Transit"
        | "Ban Cars"
        | "Restrict Air Travel" => Some(WorkshopTheme::IndustryTransportAndGeoengineering),
        "Degrowth in Developed Regions" | "Luxury for All" => {
            Some(WorkshopTheme::SocietyAndEconomy)
        }
        _ => None,
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkshopLockReason {
    /// One prerequisite is sufficient. A list with more than one item is an
    /// OR rule, matching the engine's independent `UnlocksProject` effects.
    pub prerequisites: Vec<String>,
    /// The card remains locked during the planning round in which an enabler
    /// is selected. It becomes available after that round is simulated.
    pub available_next_cycle: bool,
}

/// Reverse the world's `UnlocksProject` graph to explain why a visible card is
/// locked. This uses the actual embedded-world data rather than duplicating a
/// second prerequisite table in the UI.
pub fn workshop_lock_reason(state: &State, target: &Id) -> Option<WorkshopLockReason> {
    let mut prerequisite_projects = state
        .world
        .projects
        .iter()
        .filter(|project| {
            project.effects.iter().any(
                |effect| matches!(effect, Effect::UnlocksProject(unlocked) if unlocked == target),
            )
        })
        .collect::<Vec<_>>();

    if prerequisite_projects.is_empty() {
        return None;
    }

    prerequisite_projects.sort_by(|left, right| left.name.cmp(&right.name));
    Some(WorkshopLockReason {
        available_next_cycle: prerequisite_projects
            .iter()
            .any(|project| project.is_building() || project.is_online()),
        prerequisites: prerequisite_projects
            .into_iter()
            .map(|project| project.name.clone())
            .collect(),
    })
}

/// Cycle number and year range shown on the workshop planning header.
pub fn workshop_cycle(year: usize, start_year: usize, end_year: usize) -> (usize, usize, usize) {
    let cycle = year.saturating_sub(start_year) / 5 + 1;
    (cycle, year, year.saturating_add(5).min(end_year))
}

impl WorkshopOpts {
    /// Apply workshop-mode adjustments to a fresh or loaded game state.
    /// No-op when workshop mode is inactive, so normal mode is unaffected.
    pub fn apply(&self, state: &mut State) {
        if !self.active {
            return;
        }

        // No parliament majorities: reuse the existing suspension flag,
        // which disables majority requirements on projects.
        if !state.flags.contains(&Flag::ParliamentSuspended) {
            state.flags.push(Flag::ParliamentSuspended);
        }
    }

    /// Reset the expiring per-cycle budget at the start of each
    /// planning session. Unspent PC from the previous cycle is
    /// implicitly discarded. No-op when workshop mode is inactive.
    pub fn begin_planning(&self, state: &mut State) {
        if self.active {
            state.political_capital = consts::WORKSHOP_PC_BUDGET;
        }
    }
}

/// Workshop sessions always evaluate at the end year configured by the
/// embedded world, rather than at a separately-maintained UI constant.
pub fn has_reached_end(state: &State) -> bool {
    state.world.year >= state.death_year
}

/// Workshop-mode click/tap interaction on a policy card:
/// pass it if affordable, repeal it if already passed.
/// Repealing within the same planning cycle refunds the cost;
/// repealing a policy passed in an earlier cycle does not
/// (that cost came out of an earlier, already-expired budget).
/// Returns whether anything changed.
pub fn toggle_policy(
    state: &mut State,
    plan_changes: &mut BTreeMap<Id, PlanChange>,
    project_id: &Id,
) -> bool {
    let project = &state.world.projects[project_id];
    if project.kind != ProjectType::Policy || project.locked {
        return false;
    }

    let is_passed = project.is_building() || project.is_online();
    let changes = plan_changes.entry(*project_id).or_default();
    if is_passed {
        if changes.passed {
            // Passed this cycle: repeal and refund.
            changes.passed = false;
            state.stop_policy(project_id);
        } else {
            // Passed in an earlier cycle: repeal without refund,
            // but re-passing it this cycle is free (undoes the repeal).
            changes.withdrawn = true;
            state.stop_project(project_id);
        }
        true
    } else if changes.withdrawn {
        // Free re-pass: undo a repeal made this cycle.
        changes.withdrawn = false;
        state.pass_policy(project_id);
        true
    } else if state.pay_points(project_id) {
        changes.passed = true;
        state.pass_policy(project_id);
        true
    } else {
        // Not enough PC left in this cycle's budget.
        false
    }
}

/// The land gauge segments (used, protected, free) as percentages of
/// habitable land, clamped so the three always sum to 100. Used wins
/// over protected when their raw sum would exceed 100 (a shortage:
/// demand is already eating into what should be protected).
pub fn land_gauge_segments(used_percent: f32, protected_percent: f32) -> (f32, f32, f32) {
    let used = used_percent.clamp(0., 100.);
    let protected = protected_percent.clamp(0., 100. - used);
    let free = 100. - used - protected;
    (used, protected, free)
}

#[cfg(not(target_arch = "wasm32"))]
fn get_workshop_flag() -> String {
    std::env::var("WORKSHOP").unwrap_or_default()
}

#[cfg(target_arch = "wasm32")]
fn get_workshop_flag() -> String {
    web_sys::window()
        .and_then(|win| win.location().search().ok())
        .and_then(|search| web_sys::UrlSearchParams::new_with_str(&search).ok())
        .and_then(|params| params.get("workshop"))
        .unwrap_or_default()
}

impl Default for WorkshopOpts {
    /// Initialize workshop options from the env variable/query param.
    fn default() -> Self {
        let flag = get_workshop_flag();
        let active = matches!(flag.as_str(), "1" | "true");
        if active {
            log::info!("Workshop mode active");
        }
        Self { active }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    #[test]
    fn test_apply_when_active_suspends_parliament() {
        let mut state = State::default();
        assert!(!state.flags.contains(&Flag::ParliamentSuspended));

        let opts = WorkshopOpts { active: true };
        opts.apply(&mut state);
        assert!(state.flags.contains(&Flag::ParliamentSuspended));

        // Idempotent: applying again (e.g. on continue) adds no duplicate.
        opts.apply(&mut state);
        let count = state
            .flags
            .iter()
            .filter(|flag| **flag == Flag::ParliamentSuspended)
            .count();
        assert_eq!(count, 1);
    }

    #[test]
    fn test_apply_when_inactive_is_noop() {
        let mut state = State::default();
        let flags_before = state.flags.clone();

        WorkshopOpts { active: false }.apply(&mut state);
        assert_eq!(state.flags, flags_before);
    }

    #[test]
    fn test_begin_planning_resets_budget_when_active() {
        let mut state = State::default();
        state.political_capital = 3;

        WorkshopOpts { active: true }.begin_planning(&mut state);
        assert_eq!(state.political_capital, consts::WORKSHOP_PC_BUDGET);

        // Unspent PC is discarded too: any leftover is replaced, not added.
        state.political_capital = 12;
        WorkshopOpts { active: true }.begin_planning(&mut state);
        assert_eq!(state.political_capital, consts::WORKSHOP_PC_BUDGET);
    }

    #[test]
    fn test_begin_planning_when_inactive_is_noop() {
        let mut state = State::default();
        state.political_capital = 3;

        WorkshopOpts { active: false }.begin_planning(&mut state);
        assert_eq!(state.political_capital, 3);
    }

    #[test]
    fn test_workshop_end_uses_the_embedded_world_lifespan() {
        let mut state = State::new(hes_engine::World::workshop());

        assert_eq!(state.death_year, 2052);
        state.world.year = 2051;
        assert!(!has_reached_end(&state));
        state.world.year = 2052;
        assert!(has_reached_end(&state));
    }

    #[test]
    fn test_land_gauge_segments() {
        assert_eq!(land_gauge_segments(60., 10.), (60., 10., 30.));
        assert_eq!(land_gauge_segments(0., 0.), (0., 0., 100.));
        // More protection eats into free land, not used land.
        assert_eq!(land_gauge_segments(60., 40.), (60., 40., 0.));
        // Shortage: used land wins over protected; nothing goes negative.
        assert_eq!(land_gauge_segments(95., 10.), (95., 5., 0.));
        assert_eq!(land_gauge_segments(120., 10.), (100., 0., 0.));
        // Out-of-range inputs are clamped.
        assert_eq!(land_gauge_segments(-5., -5.), (0., 0., 100.));
    }

    #[test]
    fn test_workshop_theme_mapping_is_complete_and_unique() {
        let state = State::new(hes_engine::World::workshop());
        let themed = state
            .world
            .projects
            .iter()
            .filter_map(|project| workshop_theme(project).map(|theme| (project.id, theme)))
            .collect::<Vec<_>>();
        let unique_ids = themed.iter().map(|(id, _)| *id).collect::<BTreeSet<_>>();

        assert_eq!(themed.len(), 21);
        assert_eq!(unique_ids.len(), 21);
        assert_eq!(
            WorkshopTheme::ALL
                .into_iter()
                .map(|theme| {
                    themed
                        .iter()
                        .filter(|(_, mapped_theme)| *mapped_theme == theme)
                        .count()
                })
                .collect::<Vec<_>>(),
            vec![7, 5, 3, 4, 2]
        );
    }

    #[test]
    fn test_workshop_theme_mapping_excludes_normal_world_cards() {
        let normal = State::default();
        let universal_family_planning = normal
            .world
            .projects
            .iter()
            .find(|project| project.name == "Universal Family Planning")
            .expect("normal world contains Universal Family Planning");

        assert_eq!(workshop_theme(universal_family_planning), None);
    }

    #[test]
    fn test_lock_reason_derives_single_prerequisite_and_next_cycle_timing() {
        let mut state = State::new(hes_engine::World::workshop());
        let cellular_meat = state
            .world
            .projects
            .iter()
            .find(|project| project.name == "Cellular Meat")
            .expect("workshop contains Cellular Meat")
            .id;
        let regenerative_agriculture = state
            .world
            .projects
            .iter()
            .find(|project| project.name == "Regenerative Agriculture")
            .expect("workshop contains Regenerative Agriculture")
            .id;

        assert_eq!(
            workshop_lock_reason(&state, &cellular_meat),
            Some(WorkshopLockReason {
                prerequisites: vec!["Regenerative Agriculture".to_string()],
                available_next_cycle: false,
            })
        );

        state.start_project(&regenerative_agriculture);
        assert_eq!(
            workshop_lock_reason(&state, &cellular_meat),
            Some(WorkshopLockReason {
                prerequisites: vec!["Regenerative Agriculture".to_string()],
                available_next_cycle: true,
            })
        );
    }

    #[test]
    fn test_lock_reason_derives_mass_electrification_or_rule() {
        let state = State::new(hes_engine::World::workshop());
        let mass_electrification = state
            .world
            .projects
            .iter()
            .find(|project| project.name == "Mass Electrification")
            .expect("workshop contains Mass Electrification")
            .id;

        assert_eq!(
            workshop_lock_reason(&state, &mass_electrification),
            Some(WorkshopLockReason {
                prerequisites: vec![
                    "Nuclear Expansion".to_string(),
                    "Solar Push".to_string(),
                    "Wind Push".to_string(),
                ],
                available_next_cycle: false,
            })
        );
    }

    #[test]
    fn test_workshop_cycle_context_uses_six_five_year_cycles() {
        assert_eq!(workshop_cycle(2022, 2022, 2052), (1, 2022, 2027));
        assert_eq!(workshop_cycle(2047, 2022, 2052), (6, 2047, 2052));
    }

    /// An affordable, unlocked, inactive policy from the default world.
    fn affordable_policy(state: &State) -> Id {
        state
            .world
            .projects
            .iter()
            .find(|p| {
                p.kind == ProjectType::Policy
                    && !p.locked
                    && p.cost > 0
                    && p.cost as isize <= consts::WORKSHOP_PC_BUDGET
            })
            .expect("default world has an affordable unlocked policy")
            .id
    }

    #[test]
    fn test_toggle_policy_pass_deducts_cost() {
        let mut state = State::default();
        let mut plan_changes = BTreeMap::new();
        WorkshopOpts { active: true }.begin_planning(&mut state);

        let id = affordable_policy(&state);
        let cost = state.world.projects[&id].cost as isize;

        assert!(toggle_policy(&mut state, &mut plan_changes, &id));
        assert_eq!(state.political_capital, consts::WORKSHOP_PC_BUDGET - cost);
        let project = &state.world.projects[&id];
        assert!(project.is_building() || project.is_online());
        assert!(plan_changes[&id].passed);
    }

    #[test]
    fn test_toggle_policy_same_cycle_repeal_refunds() {
        let mut state = State::default();
        let mut plan_changes = BTreeMap::new();
        WorkshopOpts { active: true }.begin_planning(&mut state);

        let id = affordable_policy(&state);
        assert!(toggle_policy(&mut state, &mut plan_changes, &id));
        assert!(toggle_policy(&mut state, &mut plan_changes, &id));

        assert_eq!(state.political_capital, consts::WORKSHOP_PC_BUDGET);
        let project = &state.world.projects[&id];
        assert!(!(project.is_building() || project.is_online()));
        assert!(!plan_changes[&id].passed);
    }

    #[test]
    fn test_toggle_policy_earlier_cycle_repeal_does_not_refund() {
        let mut state = State::default();
        let mut plan_changes = BTreeMap::new();
        let id = affordable_policy(&state);

        // Pass in one cycle...
        WorkshopOpts { active: true }.begin_planning(&mut state);
        assert!(toggle_policy(&mut state, &mut plan_changes, &id));

        // ...then a new cycle begins: plan changes reset, budget reset.
        plan_changes.clear();
        WorkshopOpts { active: true }.begin_planning(&mut state);

        assert!(toggle_policy(&mut state, &mut plan_changes, &id));
        assert_eq!(state.political_capital, consts::WORKSHOP_PC_BUDGET);
        assert!(plan_changes[&id].withdrawn);

        // Re-passing within the same cycle undoes the repeal for free.
        assert!(toggle_policy(&mut state, &mut plan_changes, &id));
        assert_eq!(state.political_capital, consts::WORKSHOP_PC_BUDGET);
        assert!(!plan_changes[&id].withdrawn);
    }

    #[test]
    fn test_toggle_policy_blocked_when_over_budget() {
        let mut state = State::default();
        let mut plan_changes = BTreeMap::new();

        let id = affordable_policy(&state);
        let cost = state.world.projects[&id].cost as isize;
        state.political_capital = cost - 1;

        assert!(!toggle_policy(&mut state, &mut plan_changes, &id));
        assert_eq!(state.political_capital, cost - 1);
        let project = &state.world.projects[&id];
        assert!(!(project.is_building() || project.is_online()));
    }

    #[test]
    fn test_toggle_policy_ignores_locked_projects() {
        let mut state = State::default();
        let mut plan_changes = BTreeMap::new();
        WorkshopOpts { active: true }.begin_planning(&mut state);

        let id = state
            .world
            .projects
            .iter()
            .find(|p| p.kind == ProjectType::Policy && p.locked)
            .expect("default world has a locked policy")
            .id;

        assert!(!toggle_policy(&mut state, &mut plan_changes, &id));
        assert_eq!(state.political_capital, consts::WORKSHOP_PC_BUDGET);
    }
}
