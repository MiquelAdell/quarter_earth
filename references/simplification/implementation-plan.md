# Workshop mode — implementation plan (M1 → M3)

Target spec: `spec.md` v1 · Card set: `policy-cards-proposal.md` v1.1 (21 cards)
Repo: `~/code/quarter_earth` (own fork, `main`). Target event: October 2026.

## Context

We are turning Half-Earth Socialism into a ~1-hour moderator-projected
workshop game: one decision type (21 policy cards), 30 PC/cycle expiring
budget, 6 cycles (2022→2052), four headline metrics + a land gauge, no
events/parliament/NPCs, Spanish, win-only evaluation at 2052.

Three milestones (R2-1, all approved): **M1** no-code content prototype
(playtest the card set with debug flags + a custom `.world`), **M2** the real
workshop mode (engine effect + game-layer UI, the October build), **M3**
polish (choice groups, big-screen, encyclopedia, debrief).

Key code facts (from `codebase-map.md`, paths verified):
- World content is data: `engine/assets/DEFAULT.world` (JSON — 123 projects,
  29 processes, 238 events). Editable via `hes-editor` or by scripted JSON
  transformation; `editor/src/validate.rs` checks referential integrity.
- `Effect` enum (`engine/src/events/effects.rs`) has **no mix-share variant**
  — the 5 NEW(D1) cards need `Effect::ChangeMixShare(Id, isize)`.
- Game length: `LIFESPAN` in `engine/src/state.rs`; 5-year cadence assumed
  (`is_planning_year`). 6 cycles = keep cadence, shorten lifespan to 30.
- Debug flags (`game/src/debug.rs`, `?debug=` works on web):
  `SKIP_TUTORIAL,SKIP_EVENTS,SKIP_WORLD,VERY_POPULAR,SUSPENDED` +
  `DEBUG_VIEW=Plan` is the M1 baseline.
- PC-award machinery and per-cycle process caps live in the game layer
  (`game/src/state/game.rs`, `game/src/consts.rs`, `game/src/views/report.rs`)
  — engine PC can stay untouched (lever C6 strategy, not D3).
- Spanish CSV: `game/translations/es.csv` (complete). New UI strings must be
  added there or they render English-only (`rust_i18n::t!`).

---

## Milestone M1 — no-code prototype (playable in days)

### W1. Workshop `.world` v1 (content transformation)

- **Goal:** Produce `references/simplification/worlds/workshop-m1.world` from
  `DEFAULT.world`: the 16 non-NEW(D1) cards of the accepted set as policies,
  everything else locked/removed, costs 5/10/15, prerequisites wired,
  thresholds sanity-tuned for a 30-year horizon.
- **Complexity:** medium
- **Agent:** backend-developer
- **Files:** `engine/assets/DEFAULT.world` (read), `editor/src/validate.rs`
  (read, replicate checks), new script `util/workshop_world.py` (or Rust in
  `util/`), output world file.
- **Context bundle:**
  - Card list + costs + origins: `policy-cards-proposal.md` v1.1 tables.
    `Poli` cards: rebalance cost only. `Init→`/`Rese→` cards: re-author as
    kind `Policy` with equivalent effects (lever B2 — multi-year build-up is
    intentionally lost). The 5 `NEW(D1)` cards (Solar Push, Wind Push,
    Nuclear Expansion, Phase Out Coal, Organic Transition) **cannot exist yet**
    — skip in M1; note them in the moderator sheet as "coming in M2".
  - Prerequisites (proposal "Phasing" section): Cellular Meat, SRM,
    Mass Electrification gated by enabler cards via existing unlock effects.
  - Removal caution: `project_lockers` and unlock chains referencing deleted
    ids break loading — prefer *locking* unused projects over deleting where
    references exist; delete events outright (M1 also runs `SKIP_EVENTS`,
    belt-and-braces).
  - Do not touch processes (mix stays moderator-untouched in M1).
- **Non-goals:** no engine/game code changes; no Spanish retranslation (card
  text reuses existing es.csv rows where cards are reused); no balancing
  beyond plausible starting numbers.
- **Acceptance:** world file loads in the game (`just run` + load world
  picker) with exactly 16 unlockable-or-visible policy cards; editor
  validation passes; a scripted 30-year headless engine run
  (`State::new(world)` + `step_year`) completes without panic.
- **Estimated tokens:** ~50k

### W2. M1 launch recipe + moderator run sheet

- **Goal:** One-page doc: exact URL/flags to launch M1
  (`?debug=SKIP_TUTORIAL,SKIP_EVENTS,SKIP_WORLD,VERY_POPULAR,SUSPENDED`,
  `DEBUG_VIEW=Plan`, load `workshop-m1.world`), cycle-by-cycle facilitation
  script (6 cycles, 30 PC honor-system since PC UI still shows earned PC in
  M1, stop at 2052), what to observe for balancing.
- **Complexity:** trivial → **do inline, no delegation.**
- **Files:** new `references/simplification/m1-run-sheet.md`.
- **Acceptance:** a moderator who has never seen the repo can run a session
  from the doc alone.

### W3. Playtest checkpoint (human — Miquel + group)

Not delegable. Output: notes on card set, costs, pacing, thresholds → feeds
W4 balancing numbers and confirms the 21-card set before M2 hardening.
M2 code work does **not** block on this; content tuning does.

---

## Milestone M2 — the real workshop mode (October build)

### W4. Engine: `Effect::ChangeMixShare` + configurable game length

- **Goal:** Add `Effect::ChangeMixShare(Id, isize)` (apply =
  `change_mix_share(+n)`, unapply = reverse) and make `LIFESPAN` a `World`
  field (default 60, workshop 30), including editor input for both.
- **Complexity:** medium
- **Agent:** backend-developer
- **Files:** `engine/src/events/effects.rs`, `engine/src/state.rs`,
  `engine/src/world.rs`, `engine/src/production/processes.rs` (read —
  `change_mix_share`, NPC side-effects to *skip* in this effect),
  `editor/src/` effect-input UI, serde compat for existing `.world` files
  (default lifespan on missing field).
- **Context bundle:** codebase-map "Cross-cutting entanglements": game layer
  rate-limits mix changes (`game/src/state/game.rs::update_processes`) — the
  effect bypasses that by design, calling the engine directly; it must NOT
  trigger NPC relationship side-effects (parliament is cut). Effects already
  support apply/unapply symmetry — follow the existing pattern in the enum.
- **Non-goals:** no game-layer UI; no removal of engine PC (D3 explicitly
  rejected); no touching the planner/production logic.
- **Acceptance:** `cargo test -p hes-engine` passes; a unit test passes a
  ChangeMixShare policy, steps a year, asserts the mix moved and reverses on
  repeal; old `DEFAULT.world` still deserializes; editor builds and can
  author the effect.
- **Estimated tokens:** ~40k

### W5. Game: workshop mode flag + session shape

- **Goal:** First-class `WORKSHOP` mode (C1) bundling the M1 debug behaviors
  natively: no tutorial/intro/events/parliament, no world animation (straight
  sim tick → report), end + evaluation at year 30 with **no mid-game loss**,
  start in Plan view.
- **Complexity:** medium
- **Agent:** frontend-developer
- **Files:** `game/src/debug.rs` (pattern to follow), `game/src/views/mod.rs`
  (Phase flow), `game/src/views/world.rs`, `game/src/views/interstitial.rs`
  (win/lose check — disable lose), `game/src/consts.rs`.
- **Context bundle:** spec §2 (session shape, no-loss R2-5a), §5 (removed
  content). Activation: env var / `?workshop=1` query param, mirroring the
  `?debug=` mechanism in `debug.rs`.
- **Non-goals:** plan-screen redesign (W6), report redesign (W7), PC changes
  (W6). Normal game mode must be byte-for-byte unaffected when flag is off.
- **Acceptance:** `just run` with flag: reaches Plan directly, a full 6-cycle
  session ends at 2052 with the evaluation screen; without flag: unchanged
  behavior (manual smoke of intro + first cycle).
- **Estimated tokens:** ~40k

### W6. Game: policies-only plan screen + 30 PC expiring budget

- **Goal:** In workshop mode, the Plan tab is a single policy-card list
  (grouped by the 5 themes) with tap/click to pass/repeal (no hold-to-scan),
  a "30 PC this cycle" budget display, PC reset to 30 at each planning start
  (expires, R2-4), and all PC-earning machinery hidden/disabled.
- **Complexity:** medium (largest unit — split further at execution if the
  scanner refactor balloons)
- **Agent:** frontend-developer
- **Files:** `game/src/views/session/plan.rs`, `game/src/views/session/mod.rs`
  (tabs: hide Govt), `game/src/views/scanner/` (bypass, don't refactor),
  `game/src/state/game.rs` (`pay_points`, policy pass/stop),
  `game/src/state/ui.rs`, `game/src/views/report.rs` (suppress `pc_change`
  awards in workshop mode).
- **Context bundle:** spec §3–4. Strategy per lever C6: don't remove PC from
  the engine — set `state.political_capital = 30` at cycle start in workshop
  mode and hide earned-PC UI. Mutually exclusive cards already work via
  `project_lockers`. Prereq-locked cards should render as visible-but-locked
  (shows the tree exists) — reuse existing locked-card presentation if any.
- **Non-goals:** choice-groups UI (M3/W10); big-screen sizing (M3); land
  gauge (W7); no engine edits.
- **Acceptance:** in workshop mode: only policy cards listed, passing deducts
  PC, over-budget pass is blocked, next cycle shows PC=30 regardless of
  spend; normal mode plan screen unchanged.
- **Estimated tokens:** ~50k

### W7. Game: metric report + persistent land gauge

- **Goal:** Report screen re-centered on the four headline metrics
  (CO2/temp, extinction, energy served, calories served) with per-cycle
  before/after, contentedness shown secondary; land gauge
  (used/protected/free) promoted to a persistent element on the planning
  screen; water as warning-only.
- **Complexity:** medium
- **Agent:** frontend-developer
- **Files:** `game/src/views/report.rs`, `game/src/views/session/plan.rs`
  (overview resource bars — land data already computed there),
  `game/src/state/ui.rs` (`cycle_start_state` diffs).
- **Context bundle:** spec §6. Data exists (`state.ui.cycle_start_state`,
  plan-overview resource bars) — this is display work, not new computation.
- **Non-goals:** new metrics or engine queries; debrief polish (M3).
- **Acceptance:** each cycle's report shows the 4 metrics with deltas; land
  gauge visible during planning and reacts to Nature Preserves / (M2 world)
  Solar Push; normal mode unchanged.
- **Estimated tokens:** ~35k

### W8. Content: workshop `.world` v2 + Spanish strings

- **Goal:** Extend W1's world with the 5 NEW(D1) mix-shift cards using
  `Effect::ChangeMixShare`, apply W3 playtest rebalancing, re-tune win
  thresholds for 30 years; add es.csv rows for all new card text and new
  workshop-mode UI strings.
- **Complexity:** small–medium
- **Agent:** backend-developer
- **Files:** W1's world + script, `game/translations/es.csv`,
  new-string inventory from W5–W7 diffs (input: their merged branches).
- **Context bundle:** proposal tables (NEW(D1) magnitudes: Solar +4/−4 coal,
  Wind +4/−4 gas, Nuclear +3/−3 coal, Coal −5, Organic +4/−4); input: W3
  playtest notes, W4 effect syntax as serialized in `.world` JSON.
- **Non-goals:** Catalan (post-v1); rebalancing beyond playtest notes.
- **Acceptance:** full 6-cycle session in Spanish shows no English fallbacks
  on the workshop surface; each NEW(D1) card visibly moves the energy mix in
  the following report.
- **Estimated tokens:** ~30k

### W9. M2 verification + code review

- **Goal:** Review the combined M2 diff for correctness and convention
  adherence; run a full scripted session on the web build.
- **Complexity:** small
- **Agent:** code-reviewer (review) + inline `/verify` (session run).
- **Acceptance:** review findings addressed; wasm build
  (`just` web target) completes; one full 6-cycle Spanish session on the
  browser build reaches the 2052 evaluation.
- **Estimated tokens:** ~30k

---

## Milestone M3 — polish (post-October-critical)

### W10. Moderator choice-groups presentation (C7)

- **Goal:** Optional "present one theme / one exclusive pair at a time" mode
  over the policy list (trivia-style), driven by card theme tags +
  `project_lockers`.
- **Complexity:** medium · **Agent:** frontend-developer ·
  **Files:** new presentation layer over W6's list. **Tokens:** ~40k

### W11. Big-screen layout (C8)

- **Goal:** Readable-from-distance sizing for projection: larger type, fewer
  cards per screen, high-contrast gauges (format R2-7 prioritizes readability
  over click-flow).
- **Complexity:** small–medium · **Agent:** frontend-developer ·
  **Files:** `game/src/views/parts.rs` sizing + plan/report views.
  **Tokens:** ~25k

### W12. Encyclopedia static export (C9, v2 feature)

- **Goal:** Static HTML page generated from the workshop `.world` JSON (cards
  + effects, Spanish) for participants' phones — zero game-code coupling.
- **Complexity:** small · **Agent:** general-purpose ·
  **Files:** new `util/` script. **Tokens:** ~20k

### W13. Debrief screen polish

- **Goal:** Ending screen as debrief: 30-year trajectory of the 4 metrics +
  the group's passed-card history.
- **Complexity:** small–medium · **Agent:** frontend-developer ·
  **Files:** ending view + `game/src/views/report.rs` history data.
  **Tokens:** ~25k

---

## Sequencing

```
M1:  W1 ──► W2(inline) ──► W3 (human playtest) ─┐
M2:  W4 (parallel-safe with W1)                 │
     W5 (parallel-safe with W4)                 │
     W6, W7 — depends-on: W5 (mode flag); mutually parallel-safe
     W8 — depends-on: W1, W4, W6, W7, and W3 notes
     W9 — depends-on: W5–W8
M3:  W10, W11, W13 — depends-on: W9; mutually parallel-safe
     W12 — depends-on: W8 only (parallel-safe with all M3)
```

Notes: W4+W5 can start immediately — M2 code does not block on the playtest;
only content numbers (W8) do. Each code workstream lands as its own
Conventional-Commits branch off `main`.

## Execution log

- **2026-07-02 — W1 done.** `util/workshop_world.py` generates
  `worlds/workshop-m1.world` deterministically: 16 cards re-kinded to Policy
  at 5/10/15 costs, 3 prereq gates (`UnlocksProject`), 107 other projects
  locked in place, all 238 events deleted (39 dangling event-effects
  stripped from projects, incl. kept cards). Headless 30-year run green
  (`cargo run -p hes-engine --example workshop_smoke`). Miquel confirmed the
  world loads via the in-game picker.
- **2026-07-02 — W2 done.** `m1-run-sheet.md`. Key gotchas: load the world
  from the menu (don't set `DEBUG_VIEW=Plan`); M1's 16 cards total 165 PC vs
  180 budget — scarcity is soft until M2's 5 mix-shift cards land.
- **2026-07-02 — W4 done, verified.** `Effect::ChangeMixShare(Id, isize)`
  (5%-units, target process only — no rebalance, mirroring today's player
  path; clamps at 0; **`max_share` not enforced** — that check lives in the
  game layer; no NPC side-effects via new pure `Process::shift_mix_share`).
  `World.lifespan` serde-defaulted to 60; `death_year = year + lifespan`;
  `State::lifespan()` accessor. Editor: effect in kind dropdown +
  process/amount inputs; "Game Length (Years)" in World tab. 26 engine
  tests green.
- **2026-07-02 — W5 done, verified.** `game/src/workshop.rs`: `WORKSHOP`
  global (`WORKSHOP=1` env / `?workshop=1` query). Skips intro/tutorial,
  kills all events (`roll_events` → empty), suspends parliament, fast world
  tick (~10 ms/yr), interstitial gate ignores `won()`/`game_over` and ends at
  `start + WORKSHOP_YEARS(30)`. 42 game tests green. **Gotchas:** (a) cycle
  boundaries are `year % 5 == 0`, world starts 2022 → ending fires at the
  2055 boundary, not 2052 — fix in W8 via start year / lifespan alignment;
  (b) workshop sessions share the normal save slot — follow-up.
- **2026-07-02 — W6 done, verified.** Policies-only plan screen behind the
  flag: themed card groups (`workshop_card_groups`), click-to-toggle
  (`workshop::toggle_policy` — same-cycle repeal refunds, earlier-cycle
  repeal doesn't, re-pass of a same-cycle repeal is free), PC reset to 30 at
  each planning start (`begin_planning`), over-budget pass blocked, Govt tab
  hidden, report PC awards zeroed/hidden. 49 game tests green. (Agent's
  session died mid-report; work inspected and verified directly.)
- **Next:** W7 (metric report + land gauge) → W8 (world v2: 5 ChangeMixShare
  cards, Spanish strings, year alignment, playtest rebalance — needs
  `playtest-notes.md` from W3) → W9 (review + wasm verification).

## Global verification

1. `cargo test --workspace` green; wasm build green.
2. Workshop flag off ⇒ normal game unchanged (smoke: intro + 1 full cycle).
3. Full workshop session (browser, Spanish, projected-size window): 6 cycles,
   ~21 cards across 5 themes, prereq unlocks fire, PC resets to 30 each
   cycle, land gauge live, no events/parliament anywhere, 2052 evaluation
   reached with win/fail verdict.
4. A second playtest (human) confirms 1-hour fit before the October event.
