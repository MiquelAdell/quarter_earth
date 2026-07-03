# Policy card proposal (v1.1 — accepted, cut applied)

Accepted 2026-07-02 (R2-2): the ~20-card cut list is applied below (cut cards
moved to the "Cut in round 2" section), and prerequisite-based phasing is in
(see "Phasing" section). Design intent:

- **Different solutions must be able to emerge.** Each theme contains
  genuinely competing strategies (tech vs sufficiency, protection vs
  production), and the per-cycle budget makes "discussing A means not doing B"
  real. There is no single winning set.
- **Low-impact debate-bait is included on purpose** (marked 🪤). Groups love
  arguing about banning private jets; the game should let them do it and then
  *show* the small number. That's a learning outcome, not a flaw.
- **Origin** column: `Poli` = existing policy, reused as-is or rebalanced;
  `Init→` / `Rese→` = existing initiative/research converted to a policy
  (editor work, lever B2); `NEW(D1)` = new card whose effect is a process-mix
  shift, needs `Effect::ChangeMixShare` (lever D1).

Cost scale (Q2 = budget model): **30 PC per cycle**, cards cost 5 / 10 / 15
(low/medium/high impact — cost correlates with impact so buying big things
hurts). Unspent PC expires each cycle (R2-4). 6 cycles × 30 = 180 PC total
against ~235 PC of cards (21-card set) → scarcity of roughly 3:4, forcing
real choices. All numbers are starting points for playtesting.

## Energy (7 cards)

| Card | Origin | Impact | Cost | Why it's in |
|---|---|---|---|---|
| Solar Push | NEW(D1): +4 solar shares, −4 coal | High | 15 | The canonical renewables bet; big CO2 drop, big land footprint — feeds the land debate. |
| Wind Push ("eolic thrust") | NEW(D1): +4 wind/offshore, −4 gas | High | 15 | Same tradeoff, different flavor; competes with Solar Push for budget. |
| Nuclear Expansion | NEW(D1): +3 nuclear, −3 coal | High | 15 | The classic split-the-room card: low CO2, low land, waste/risk aversion. |
| Phase Out Coal | NEW(D1): −5 coal (shortage risk if nothing replaces it) | High | 10 | Teaches that removing supply without replacing it causes shortages — consequence-rich. |
| Mass Electrification | Init→ | High | 15 | Converts fuel demand to electricity (existing `Electrified` flag) — multiplies the value of clean electricity choices. |
| Energy Quotas | Poli | Med | 10 | The sufficiency alternative to building more supply; contentedness cost makes it debatable. |
| Crack Down on Crypto-Mining 🪤 | Poli | Low | 5 | Feels righteous, moves almost nothing. Debate-bait by design. |

## Food & agriculture (5 cards)

| Card | Origin | Impact | Cost | Why it's in |
|---|---|---|---|---|
| Vegetarian Mandate | Poli | High | 15 | Biggest single land/emissions lever in the game; predictably controversial. |
| Meatless Mondays 🪤 | Poli | Low | 5 | The gentle version — visibly tiny effect next to the mandate. Great side-by-side discussion. |
| Cellular Meat | Rese→ | Med | 10 | Tech-optimism route to the same goal as the mandates; competes with them. |
| Organic Transition | NEW(D1): +4 organic crop ag, −4 industrial | Med | 10 | Lower biodiversity pressure but lower yields — calories vs extinction tension. |
| Regenerative Agriculture | Init→ | Med | 10 | The soil/emissions middle path; pairs or competes with Organic Transition. |

## Land & biodiversity (3 cards)

| Card | Origin | Impact | Cost | Why it's in |
|---|---|---|---|---|
| Expand Nature Preserves | Init→ | High | 15 | The Half-Earth card itself (`ProtectLand 0.3`): huge extinction win, shrinks land available for everything else. Central tension of the whole exercise. |
| Remediate and Protect Ecosystems | Init→ | Med | 10 | Smaller, cheaper protection step; the incremental alternative. |
| Ban Outdoor Cats 🪤 | Poli | Low | 5 | Real biodiversity measure, absurd-sounding, tiny numbers. Ice-breaker debate-bait. |

## Industry, transport & geoengineering (4 cards)

| Card | Origin | Impact | Cost | Why it's in |
|---|---|---|---|---|
| Solar Radiation Management | Init→ | High | 10 | Cheap, fast temperature relief, does nothing for emissions/extinction and carries termination-shock risk. The riskiest bargain on the table. |
| Expand Public Transit | Init→ | Med | 10 | Uncontroversial but budget-competing infrastructure. |
| Ban Cars 🪤/High | Poli | Med | 15 | Half debate-bait, half real: big lifestyle fight, moderate numbers. |
| Restrict Air Travel 🪤 | Poli | Low | 5 | The private-jets proxy. Small numbers, guaranteed 10-minute argument. |

## Society & economy (2 cards)

| Card | Origin | Impact | Cost | Why it's in |
|---|---|---|---|---|
| Degrowth in Developed Regions | Poli | High | 15 | The systemic sufficiency strategy; reduces all demand, costs contentedness. The counter-pole to every tech card. |
| Luxury for All | Poli | High | 15 | The opposite bet — raise everyone's consumption. Having both on the table frames the whole workshop. |

## Totals & shape

21 cards: 10 high / 7 medium / 4 low impact; 5 🪤 debate-bait cards spread
across themes; 5 NEW(D1) mix-shift cards; the rest conversions and reused
policies. Total cost ~235 PC vs 180 PC available.

## Phasing (R2-2b: prerequisites, not scripting)

All cards visible from cycle 1; a few require an enabler passed in an
earlier cycle (existing unlock mechanism). Starting proposal, to be tuned
during the M1 content build:

- **Cellular Meat** — requires an R&D-ish enabler (candidate: make
  Regenerative Agriculture or a cheap new "Food R&D" enabler its unlock).
- **Solar Radiation Management** — requires a first, smaller geoengineering
  or protection step so it can't be a cycle-1 panic buy.
- **Mass Electrification** — requires at least one clean-electricity card
  (Solar Push / Wind Push / Nuclear Expansion) so it pays off, not backfires.

## Cut in round 2 (R2-2a — could return after playtesting)

- **Carbon Capture & Sequestration** — the "keep burning" debate is partly
  covered by SRM; first candidate to reinstate if geoengineering needs a
  second voice.
- **Expand Recycling** — low impact, and its "favorite easy answer" lesson
  overlaps with the 🪤 cards.
- **Marine Protected Areas** — Nature Preserves + Remediate cover protection.
- **Food Waste Campaign** — third medium food card; theme already rich.
- **More Leisure vs Shock Workers pair** — nice grouped-choice mechanic but
  needs the labor framing we cut.
- **Universal Family Planning** — slow-payoff lever, weak inside a 30-year
  horizon.

## Deliberately excluded (and why)

- **Space program** (8 initiatives): fun but off-topic for a transition
  assembly and long-horizon; steals discussion time.
- **Curriculum/NPC policies** (10 cards): their effects are mostly NPC
  relationships — parliament is cut (Q7).
- **Authoritarian arc** (Suspend Parliament, Close Borders, One-Child Policy,
  Stop Food Aid, Keep Global South Poor): dark-path content that needs the
  narrative framing we're cutting; risky tone for a 1-hour facilitated group.
  Could return in v2 if acTe wants the ethics debate.
- **Research-tree cards** (Thorium, Fusion, Green Hydrogen…): they only
  unlock processes; with the allocation page gone, fold the interesting ones
  into the NEW(D1) mix cards instead.

## Resolved in round 2

1. Card count: cut to 21 (applied above). ✅
2. Phasing: prerequisite-based (see Phasing section). ✅
3. Still open by design: all effect magnitudes need a balancing pass for the
   6-cycle length (win thresholds are calibrated for 12 cycles today) —
   that's M1 playtesting work.
