# Codebase map — where each mechanic lives

Rust workspace, three crates that matter: `engine/` (`hes-engine`, pure
simulation, no UI), `game/` (`hes-game`, egui UI over the engine, web+native),
`editor/` (`hes-editor`, builds `.world` files, native only).

**Big picture**: the engine is already a clean "model-only" core. Almost all
simplification is either (a) UI work in `game/`, or (b) **data** work — the
world content lives in `engine/assets/DEFAULT.world` (JSON: 123 projects,
29 processes, 238 events, 20 regions, 9 industries) and can be reshaped with
the editor without touching engine code. Political capital and points are the
main engine-level entanglements.

## Engine (`engine/src/`)

| File | Contains |
|---|---|
| `state.rs` | `State`: the whole game state + yearly step (`step_year`), demand/production update, PC (`political_capital`, `change_political_capital`), research points, policy queue, project cost updates (NPC-ally modifiers), win condition (`won()`), 5-year cadence (`is_planning_year`). |
| `world.rs` | `World`: content container (projects/processes/events/regions/industries) + climate/outlook/extinction/population updates. `World::default()` deserializes `assets/DEFAULT.world`. |
| `projects.rs` | `Project` (kind Policy/Research/Initiative, cost, points, upgrades, outcomes, supporters/opposers, `required_majority`), build progression (`years_for_points`), start/stop/upgrade logic. |
| `production/processes.rs` | `Process`: `mix_share` (5%-units), limits, feedstocks, byproducts, `change_mix_share` (with NPC relationship side-effects), `max_share`. |
| `production/planner.rs`, `production/mod.rs` | Production function: turns demand + mix into per-process orders, resource/feedstock consumption, shortages. |
| `events/events.rs` | `Event`, `Phase` (a.k.a. `EventPhase`, exported) — the phase-gated event pool. |
| `events/effects.rs` | `Effect` enum (the vocabulary of everything a project/event can do — ~all game levers) + `Flag` enum (Vegan, Electrified, ParliamentSuspended, …). Effects know how to `apply`/`unapply` to `State`. |
| `events/condition.rs`, `probability.rs` | Condition/likelihood system used by events and project outcomes. |
| `npcs.rs` | NPCs, seats, relationships, allies. |
| `regions.rs` | 20 regions: population, income, development, habitability, per-region outlook. |
| `industries.rs` | Non-modeled industry demand/byproducts. |
| `kinds.rs` | The typed maps: `OutputMap` (4 outputs), `ResourceMap`, `ByproductMap`, `FeedstockMap`. |
| `diff.rs` | State diffing (used for change displays). |

Engine has **no UI dependencies** — it can be driven headless. `State::new(world)`,
`step_year(tgav)`, `start_project`, `change_process_mix_share`, `apply_event`
are the whole API surface a simplified frontend needs.

## Game (`game/src/`)

Flow driver: `views/mod.rs` — `Phase` enum:
`Intro → Interstitial → Planning(Session) ⇄ Events(WorldEvents) → Report → …
→ Ending`. This is the top-level loop to reshape for a short session.

| File | Contains |
|---|---|
| `views/session/mod.rs` | Planning session shell: 4 tabs (Plan / Govt / Stats / World), event interjections per tab, tutorial gating. |
| `views/session/plan.rs` | The Plan tab: overview (project card slots, production summary, resource bars, Ready button), `Projects` page (3 sub-tabs by project kind, PC/points display), `Processes` page (mix-share drag UI, 5-point budget, change estimates). |
| `views/session/govt.rs` | Parliament view. |
| `views/session/stats.rs`, `treemap.rs`, `regions.rs` | Dashboard, land treemap, globe/regions. |
| `views/scanner/` | The card-scanning interaction (hold-to-scan cards to buy/assign points) — heavy interaction cost per decision; `scanner/project.rs` implements buy/assign/withdraw per project kind. |
| `views/world.rs` | The 5-year world/events phase (globe, disasters ticking by year). |
| `views/report.rs` | Cycle report; computes PC awards (`pc_change`) from temp/emissions/extinction/contentedness deltas + completed projects + request bounties (constants in `consts.rs`). |
| `views/events/` | Event/dialogue rendering (the reading-heavy part). |
| `views/interstitial.rs` | Between-cycle year screen; win/lose check. |
| `state/game.rs` | `StateExt` on engine `State`: PC point purchase (`buy_point`, `pay_points`, `assign_point`), policy pass/stop, process mix change application with per-cycle cap (`update_processes`), upgrade queue. |
| `state/ui.rs` | `UIState`: points balances, plan changes, tutorial progress, viewed cards. |
| `state/mod.rs` | `GameState` = engine `State` + `UIState`; save/load. |
| `debug.rs` | `DEBUG`/`DEBUG_VIEW`/`DEBUG_STATE` env flags — see below. |
| `consts.rs` | All the tuning constants: `POINT_COST=3`, `MAX_POINTS=12`, `PROCESS_POINTS_PER_CYCLE=5`, `PC_PER_COMPLETED_PROJECT=5`, honeymoon PC, PC-per-metric rates, `MS_PER_YEAR=2500`. |
| `tips.rs` | Tooltip/tip system (also project card popups). |

## Editor (`editor/`)

Tabs for world/projects/processes/industries/events. Use it to produce a
custom `.world`: fewer projects, rebalanced costs, trimmed events. Worlds load
in both native and web builds. This is the **no-code content lever**.

## Existing debug flags (`game/src/debug.rs`) — free simplification

`DEBUG=` comma list (env var, or `?debug=` on web): `SKIP_EVENTS` (all story
events), `SKIP_TUTORIAL`, `ALL_PROJECTS` / `ALL_PROCESSES` (ignore locks),
`FAST_YEARS`, `SKIP_WORLD` (jump world phase → report), `VERY_POPULAR`
(PC=500 ≈ removes PC as constraint), `SUSPENDED` (parliament off),
`WITH_PROJECTS`, shortage/lose/win presets. `DEBUG_VIEW=Plan` starts straight
in planning. `DEBUG_STATE=<file>` loads an exported session (base64+DEFLATE
JSON of `State`).

Combined baseline for a workshop prototype without writing code:

```bash
DEBUG=SKIP_TUTORIAL,SKIP_EVENTS,SKIP_WORLD,VERY_POPULAR,SUSPENDED \
DEBUG_VIEW=Plan cargo run
```

…plus a custom `.world` from the editor. What this **cannot** do: merge the
three project kinds into one, turn process mixes into policy cards, change
what the report shows, or add grouped/exclusive choice presentation.

## Cross-cutting entanglements to know about

- **NPC side-effects are everywhere**: project costs (`update_project_costs`
  ally discounts), process mix changes (relationship shifts), report seat
  changes, required majorities. Removing parliament = flag + skipping these,
  mostly already tolerated by `ParliamentSuspended`.
- **Policies apply on the pre-planning year** (`is_pre_planning_year` →
  `roll_new_policy_outcomes` in `state.rs`), i.e. effects land before the next
  report. Research/initiatives complete over years via points. Collapsing
  everything to policies removes the whole points/progress subsystem from the
  player-facing loop (engine can keep it; just stop exposing it).
- **Process mix changes are rate-limited per cycle** in the *game* layer
  (`update_processes`), not the engine — a "Wind push" policy can bypass it by
  calling `change_process_mix_share` directly. Note: the `Effect` enum
  (`events/effects.rs`) has **no mix-share variant** today — it can modify a
  process's output/limits/byproducts but not its share of the mix. A
  "Wind push" policy therefore needs a new `Effect::ChangeMixShare(Id, isize)`
  (small engine addition; effects already support apply/unapply, and unapply
  would just reverse the share change).
- **Locks/unlocks**: projects and processes start locked and are unlocked by
  event/project effects (research gating). `project_lockers` implements mutual
  exclusivity between projects — relevant to the brief's "pick 1a AND 1b"
  grouped-choice thread.
- **i18n**: all UI strings go through `rust_i18n::t!` with 8 translations —
  new UI text either gets added to translation CSVs or stays English-only.
