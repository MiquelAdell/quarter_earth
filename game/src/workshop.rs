//! Workshop mode: a first-class, simplified session shape for
//! facilitated group workshops (see `references/simplification/spec.md`).
//!
//! When active:
//! - No intro, no tutorial, no story/world events, no parliament majorities.
//! - The world phase between cycles is a fast simulation tick.
//! - The game always runs to the fixed horizon (`consts::WORKSHOP_YEARS`),
//!   where the usual ending/evaluation is shown; mid-game loss is disabled.
//!
//! Activated via the `WORKSHOP=1` env var (native) or the `?workshop=1`
//! query param (web), mirroring the debug flag mechanism in `debug.rs`.

use std::{collections::BTreeMap, sync::LazyLock};

use hes_engine::{Flag, Id, ProjectType, State};

use crate::{
    consts,
    state::{PlanChange, StateExt},
};

pub static WORKSHOP: LazyLock<WorkshopOpts> = LazyLock::new(WorkshopOpts::default);

pub struct WorkshopOpts {
    /// Whether workshop mode is active.
    pub active: bool,
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
        assert_eq!(
            state.political_capital,
            consts::WORKSHOP_PC_BUDGET - cost
        );
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
