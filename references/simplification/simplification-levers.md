# Simplification levers

The menu of changes to build the spec from. Grouped by cost. "Data" = done in
the editor / `.world` file, no code. Effort is for a working prototype, not a
polished upstream PR.

## A. Free — existing debug flags (zero code)

| Lever | How | Removes |
|---|---|---|
| A1. Skip story/dialogue | `DEBUG=SKIP_EVENTS` | ~all reading load from 238 events (also removes useful world-reaction events — all or nothing) |
| A2. Skip tutorial | `DEBUG=SKIP_TUTORIAL` | onboarding sequence |
| A3. Skip world animation | `DEBUG=SKIP_WORLD` | the ~60 s globe/disaster phase per cycle → straight to report |
| A4. Neutralize PC | `DEBUG=VERY_POPULAR` (PC=500) | political capital as a binding constraint (crude: it still displays and still gates costs nominally) |
| A5. Disable parliament | `DEBUG=SUSPENDED` | majority requirements; parliament tab still visible |
| A6. Everything unlocked | `DEBUG=ALL_PROJECTS,ALL_PROCESSES` | research-gated progression |
| A7. Start in Plan view | `DEBUG_VIEW=Plan` | intro |
| A8. Prepared scenario | `DEBUG_STATE=<file>` | lets a moderator resume/restart from a fixed state |

Together: the October-2026 fallback. Limitation: decision surfaces are
untouched (still 3 project kinds + process mixes + points).

## B. Data only — custom `.world` via the editor (no code)

| Lever | How | Notes |
|---|---|---|
| B1. Cull projects | Delete/lock all but ~15–25 curated cards | Biggest single reduction in option count and reading. Careful with `project_lockers` and unlock chains referencing deleted ids (editor has `validate.rs`). |
| B2. Convert research/initiatives to policies | Re-author kept non-policy cards as kind `Policy` with equivalent effects | Collapses 3 decision types → 1 without engine changes. Loses multi-year build-up (policies land within the cycle) — arguably a feature for a 1-hour game. Costs need rebalancing into PC terms. |
| B3. Cull/rewrite events | Keep a small set of consequence events (disasters, shortages reactions), delete narrative arcs | Middle ground vs `SKIP_EVENTS`. Editor work, tedious but mechanical. |
| B4. Rebalance pacing | Raise starting PC, cheapen policies, strengthen effects so ~4–6 cycles produce a legible arc | Pure numbers work; needs playtesting. |
| B5. Merge processes | Reduce 29 processes to ~3–4 per output (e.g. fossil / renewables / nuclear) | Only matters if the process screen stays player-facing; if C2 happens, mixes become policy-driven and this is optional. |

## C. Code — game layer only (`game/src/`)

| Lever | Touches | Effort | Notes |
|---|---|---|---|
| C1. "Workshop mode" flag | new mode alongside `debug.rs`, branch in `views/mod.rs` | S | Bundles A-flags into a first-class mode instead of env vars; entry point for everything below. |
| C2. Policies-only planning UI | `views/session/plan.rs` (drop Projects sub-tabs, drop Processes page), `views/scanner/` | M | One list of policy cards: toggle in/out of plan. Requires B2 so content is all policies. Kill the hold-to-scan interaction in favor of tap/click (moderator-friendly). |
| C3. Shorter game: N cycles | `LIFESPAN` is engine (`state.rs`), end condition in `views/interstitial.rs` | S | e.g. 6 cycles = 2022→2052. Alternative: keep 60 y but 10-year cycles (riskier: engine assumes 5-year cadence in `is_planning_year`). |
| C4. Compress world phase | `consts.rs::MS_PER_YEAR`, `views/world.rs` | S | Faster ticking while keeping the "consequences happen" beat visible (vs A3 which removes it). |
| C5. Report as the discussion screen | `views/report.rs` | M | Reframe the report around the 4 acTe metrics (temp, CO2, extinction/biodiversity, calories+energy served) with before/after per cycle. Mostly display work; data is already in `state.ui.cycle_start_state` diffs. |
| C6. Remove PC from UI, uncap costs | `views/session/plan.rs`, `state/game.rs` (`pay_points` etc.), HUD | M | Pure-planning framing: policies are free, constraint becomes physical (land/energy/calories) + a per-cycle policy-slot budget (e.g. "pick 3 per cycle") which is trivially enforced in UI. Engine PC can keep existing untouched underneath. |
| C7. Grouped / mutually-exclusive choices for moderator | new presentation layer over the policy list | M–L | The "pick 1a or 1b" open thread. Data side exists (`project_lockers`); UI for "choice groups" presented one at a time (trivia style) is new. |
| C8. Projected/big-screen layout | `parts.rs` sizing, plan view | M | Bigger text, fewer cards per screen, readable from across a room. |
| C9. Encyclopedia (upstream idea 2) | new view or static export | M | Browsable list of all cards + effects. Cheapest version: generate a static HTML page from the `.world` JSON (no game code at all) for participants' phones. |

## D. Code — engine changes (`engine/src/`)

| Lever | Touches | Effort | Notes |
|---|---|---|---|
| D1. `Effect::ChangeMixShare(Id, isize)` | `events/effects.rs` (+ editor input in `editor/src/`) | S–M | The key enabler for "allocation as policy" (e.g. "Eolic thrust"). Apply = `change_mix_share(+n)`, unapply = reverse. Bypasses the game-layer 5-point/cycle cap by design. |
| D2. Configurable game length | `state.rs::LIFESPAN` → world field | S | Cleaner than C3 hardcoding. |
| D3. Remove PC from engine | many places | L | **Not recommended**: C6 (hide it, make things free) achieves the same player experience without surgery. |
| D4. Multi-group vote aggregation ("ministries") | new | L | Only if the format needs parallel group inputs into one sim. A moderator with one screen doesn't. Defer. |

## Suggested composition (strawman for the spec)

A 1-hour session ≈ 6 planning cycles × (~5 min discussion + ~2 min consequences):

1. **Content**: B1+B2+B4 — one curated world, ~20 policy cards total (some
   being converted initiatives/research, some new mix-shift policies via D1),
   grouped into 4–5 themes (energy, food, land, industry, society).
2. **Engine**: D1 (+D2).
3. **Game**: C1 workshop mode = policies-only plan screen (C2), no PC but a
   3-policies-per-cycle budget (C6), fast world phase (C4), metric-centric
   report (C5), 6 cycles (C3), big-screen sizing (C8).
4. **Encyclopedia**: C9 static export for phones.

Everything in the strawman is independent enough to phase: A+B alone is a
usable fallback; D1+B2 makes the single-decision-type model real; C-items are
incremental UX.
