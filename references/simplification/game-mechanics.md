# Current game mechanics — inventory

What the game asks the player to do today, and what it shows them. Numbers
below are from the shipped `DEFAULT.world`.

## Turn structure

The game runs from 2022 for 60 years (`LIFESPAN` in `engine/src/state.rs`),
in **5-year planning cycles**:

1. **Planning session** — the interactive part. Player edits the plan
   (projects, processes), browses parliament/stats/world tabs. No time passes.
2. **World events phase** — 5 years tick by (~2.5 s each, `MS_PER_YEAR`),
   disasters and story events fire, projects progress, production runs yearly.
3. **Report** — summary of the cycle: temperature/emissions/extinction/
   contentedness changes, each converted into political capital gains/losses.
4. Back to planning.

A full playthrough is 12 cycles. Dialogue/story events interject throughout
(the "Planning Intro", advisor dialogue, event popups with narrative text).

## Decision types (what we want to collapse)

There are **four distinct decision surfaces** in the planning session:

### 1. Policies (project kind `Policy`, 44 in default world)
- Cost **political capital (PC)**, paid up-front as a lump sum. Refunded if repealed.
- Take effect at the end of the planning year (applied in the pre-planning
  year step, `roll_new_policy_outcomes`).
- Some have **upgrades** (pay more PC for a stronger version) and
  probabilistic **outcomes**.
- This is the decision type the simplified game keeps.

### 2. Research projects (kind `Research`, 27)
- Player buys **research points** with PC (3 PC/point, `POINT_COST`) and
  assigns up to 12 points per project; more points = faster completion
  (`years_for_points`: years = cost / points^(1/2.75)).
- Completion takes multiple years → resolves during world phase.
- Extra decision dimension: not just *what* but *how hard to push it*.

### 3. Initiatives / infrastructure (kind `Initiative`, 52)
- Same point-assignment mechanic as research, but points are bought per
  initiative (no shared pool) and cost scales dynamically (time / income /
  output-demand factors, plus NPC-ally discounts per project group).
- Can be `ongoing` (haltable) and `gradual` (effects scale with progress).

### 4. Production process mix (29 processes across 4 outputs)
- Each output sector (electricity, fuel, plant calories, animal calories) has
  a mix of processes in 5-percentage-point units (`mix_share`, 20 units = 100%).
- Player drags shares between processes; limited to **5 points of change per
  output per cycle** (`PROCESS_POINTS_PER_CYCLE`), changes phase in over
  following cycles.
- Banning (share→0) and promoting (share≥25%) shifts NPC relationships.
- This is "allocation" — the brief wants it re-expressed as policies
  (e.g. "Wind push" = +2 wind shares, -2 coal shares).

## Currencies

- **Political capital (PC)** — the master currency. Start 100. Earned each
  report from: temperature/emissions/extinction improvements, contentedness
  intensity, completed projects (5 PC each), fulfilled NPC requests,
  honeymoon bonus (15 PC first cycle). Spent on: policies, research points,
  initiative points, upgrades. Hitting 0 PC ≈ losing (player is ousted;
  one bailout possible). The brief flags PC as a game-design device that may
  not fit a pure planning tool — unresolved.
- **Research points** — bought with PC or granted by effects
  (`state.research_points`), then allocated.
- **Process points** — 5 free mix-change points per output per cycle. Not
  bought; a rate limit, not a currency.

## Parliament / NPCs (13 NPCs)

- Seats shift each cycle based on outlook change and which recent projects
  each NPC supports/opposes (`finish_cycle` → `update_seats`).
- Projects with more opposers than supporters need a **majority** to pass
  (`required_majority`, suspended by the `ParliamentSuspended` flag).
- Allies give cost discounts per project group and other bonuses.
- NPCs issue **requests** (do/ban X for a PC bounty).
- Entirely removable via flag + debug (`SUSPENDED`), per the brief.

## Feedback variables (what the player watches)

Headline (HUD, win/lose conditions):
- **Temperature anomaly** (win ≤ +1 °C) — from vendored Hector climate model
  fed by emissions.
- **Emissions** GtCO2eq (win ≤ 0).
- **Extinction rate** (win ≤ 20) — from process/industry land use & biodiversity
  pressure + temperature + sea-level rise.
- **Contentedness/outlook** — world + regional mood; low outlook → PC losses,
  unrest events.
- **Political capital** (lose at 0).

Secondary (Stats tab, tooltips, factor cards):
- Production vs demand per output (shortages hurt outlook badly).
- Land / water / energy use vs availability; protected land (starts 10%).
- Feedstock reserves (oil, uranium, …) — processes can run out.
- Per-region: population, income level, habitability, climate, development.
- Sea level rise, water stress, mesosphere-of-stuff per-capita demand by
  income level (4 tiers), non-modeled industry demand (9 industries).

This maps well onto acTe's ask: biodiversity pressure, energy, calories,
CO2/temperature as the legible core set.

## Events (238 + icon events)

- **Story/dialogue events** — narrative popups, advisor dialogue, choices.
  Main source of reading load. Skippable via `SKIP_EVENTS` debug flag.
- **Icon events (disasters)** — wildfires, floods etc. appear on the globe
  during the world phase, damage regional habitability.
- Events are condition-gated (flags, variables, year…) and phase-gated
  (`EventPhase`: PlanningStart, WorldStart, ReportStart, Icon, …).

## Win / lose

- **Win**: emissions ≤ 0 **and** extinction ≤ 20 **and** temp ≤ +1 °C at any
  point (checked at each interstitial).
- **Lose**: PC ≤ 0 (ousted), or death year reached (60 y), or runaway
  warming/collapse endings.

## Content volume (reading-load drivers)

| Content | Count |
|---|---|
| Projects total | 123 (44 policies / 27 research / 52 initiatives) |
| Processes | 29 (14 electricity, 8 fuel, 4 crops, 3 livestock) |
| Events | 238 |
| Regions | 20 |
| Industries (non-modeled) | 9 |
| NPCs | 13 |

Each project/process additionally has flavor text, images, effect lists
rendered as icon-text, and probabilistic outcomes.
