# M1 run sheet — moderator guide for the no-code prototype

Everything a moderator needs to run a ~1-hour workshop playtest with today's
game + debug flags + the custom world (`worlds/workshop-m1.world`). No code
knowledge required beyond copy-pasting the commands below.

## What M1 is (and isn't)

M1 is a **content playtest**: it tests the card set, costs, pacing and
thresholds — not the final interface. Known gaps vs the real workshop mode
(all fixed in M2):

- The UI still shows research/initiative tabs and the process screen —
  **ignore them**; only pass Policy cards.
- PC displays as earned/accumulated, not as a fixed budget — the 30 PC/cycle
  budget is enforced **by the moderator on paper** (see below).
- The game runs past 2052 if you let it — the moderator **stops after
  cycle 6** (when the year hits 2052) and reads the metrics as the verdict.
- The 5 mix-shift cards (Solar Push, Wind Push, Nuclear Expansion, Phase Out
  Coal, Organic Transition) don't exist yet — the energy theme is thinner
  than the final set.
- English or any existing translation; Spanish is selectable in settings.

## Launch

### Option A — native (recommended for playtests)

```bash
cd ~/code/quarter_earth
DEBUG=SKIP_TUTORIAL,SKIP_EVENTS,SKIP_WORLD,VERY_POPULAR,SUSPENDED cargo run
```

### Option B — browser (what the real event will use)

```bash
cd ~/code/quarter_earth/game && trunk serve
```

Then open:

```
http://localhost:8080/?debug=SKIP_TUTORIAL,SKIP_EVENTS,SKIP_WORLD,VERY_POPULAR,SUSPENDED
```

Flag meanings: no tutorial, no story events, no world-phase animation
(straight to the report), PC set to 500 (never a blocker — we budget on
paper instead), parliament majorities off.

**Do not** set `DEBUG_VIEW=Plan` — you need the main menu to load the world.

### Load the workshop world

On the main menu, use the **load world** option and pick
`references/simplification/worlds/workshop-m1.world`, then start a new game.
(Works identically in browser — it opens a file dialog.)

## Session script (6 cycles ≈ 45–55 min + debrief)

Materials: this sheet, a visible tally of the PC budget (whiteboard or
paper), the card list below.

Per cycle (~7 min):

1. **Announce the budget: 30 PC.** Unspent PC expires — no saving up.
2. Group debates which cards to pass (typically 2–3). The moderator may
   present one theme at a time if the full list overwhelms.
3. Moderator passes the agreed cards in the Plan tab (Policies only), then
   hits Ready.
4. Report screen: read the four headline metrics aloud — emissions/
   temperature, extinction, energy vs demand, calories vs demand — plus the
   land picture. "What changed since last cycle, and why?"
5. Note the cycle's spend and choices on paper (feeds balancing).

Repeal costs the card's PC again from the current budget (house rule for M1).

**Stop at 2052** (after cycle 6). Final report = the verdict: did the group
get emissions to ≤ 0, temperature ≤ +1 °C, extinction ≤ 20?

## Card list & costs (moderator's paper budget)

| Theme | Card | PC | Notes |
|---|---|---|---|
| Energy | Mass Electrification | 15 | 🔒 unlocks after Energy Quotas (M1 placeholder gate) |
| Energy | Energy Quotas | 10 | |
| Energy | Crack Down on Crypto-Mining | 5 | 🪤 debate-bait |
| Food | Vegetarian Mandate | 15 | |
| Food | Meatless Mondays | 5 | 🪤 |
| Food | Cellular Meat | 10 | 🔒 unlocks after Regenerative Agriculture |
| Food | Regenerative Agriculture | 10 | |
| Land | Expand Nature Preserves | 15 | the Half-Earth card |
| Land | Remediate & Protect Ecosystems | 10 | |
| Land | Ban Outdoor Cats | 5 | 🪤 ice-breaker |
| Industry | Solar Radiation Management | 10 | 🔒 unlocks after Remediate & Protect |
| Industry | Expand Public Transit | 10 | |
| Industry | Ban Cars | 15 | 🪤/real |
| Industry | Restrict Air Travel | 5 | 🪤 |
| Society | Degrowth | 15 | |
| Society | Luxury for All | 15 | |

Total 165 PC of cards vs 180 PC of budget — M1 is deliberately slightly
looser than the final 21-card set; groups can pass almost everything if they
never waste PC. Watch whether that kills the scarcity debate (see below).

## What to observe (balancing notes for W8)

- **Pacing**: does a cycle's debate fit in ~5–7 min? Which cards eat time?
- **Scarcity**: did the budget force real trade-offs, or did the group
  eventually buy everything? (If the latter, M2's extra 70 PC of cards
  should fix it — confirm.)
- **Debate-bait**: did the 🪤 cards trigger the intended "big argument,
  small number" moment?
- **Legibility**: could the group connect a card to its metric movement in
  the next report? Which effects felt invisible?
- **Thresholds**: how close was the group to the three win conditions at
  2052? (Calibrated for 60 years today — expect a miss; note by how much.)
- **Prerequisites**: did the three locked cards create a satisfying
  "unlocked something" beat?

File notes in `references/simplification/playtest-notes.md` (create it) —
workstream W8 consumes them verbatim.
