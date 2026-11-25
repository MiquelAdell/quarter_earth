# Assembly run sheet: facilitator guide

This sheet covers a three-hour Quarter Earth assembly for 12–24 people at four tables. You need one facilitator, one operator laptop, one projector and a wall. Keep it open on the laptop while you run the session.

The paper kit is listed in [`print/README.md`](print/README.md). The rules here are written to match `openspec/changes/assembly-workshop/specs/assembly-session/spec.md` in the personal project folder. If a rule here contradicts that spec, the spec wins: amend it first, then change this sheet.

> Card data, gates, the hidden coal conflict, timing, opening values and round years come from W1's `references/assembly/model-baseline.md` (commit 45277e6e) and the 2022 row of `fixtures/a_no_action/ozone_appended.json`. The remaining TODOs are listed under Open items.

## The clock: 180 minutes

| Start | End | Min | Segment | Spec block |
|---|---|---|---|---|
| 0:00 | 0:15 | 15 | Seat tables, read the opening planet, demonstrate one card on a disposable world and reset | Opening and practice **30** |
| 0:15 | 0:30 | 15 | Promises and questions, practice deal between two tables, personal predictions | ″ |
| 0:30 | 0:50 | 20 | Round 1 (deck A) | Rounds 1–2 **40** |
| 0:50 | 1:10 | 20 | Round 2 | ″ |
| 1:10 | 1:20 | 10 | Break. Leave the timeline and the offers visible | Break **10** |
| 1:20 | 1:40 | 20 | Round 3. Rotate speakers, newly eligible cards | Rounds 3–4 **40** |
| 1:40 | 2:00 | 20 | Round 4 | ″ |
| 2:00 | 2:15 | 15 | Round 5. Tables revise existing proposals or renew their priorities | Rounds 5–6 **30** |
| 2:15 | 2:30 | 15 | Round 6. The last vote must happen on time | ″ |
| 2:30 | 2:45 | 15 | Replay one disputed decision | Replay **15** |
| 2:45 | 3:00 | 15 | Debrief | Debrief **15** |

Check: 30 + 40 + 10 + 40 + 30 + 15 + 15 = **180**.

Round years (model-baseline.md §3). Tables plan in the first year of each round, and the game ends in 2052.

| Round | Planning year | Years covered |
|---|---|---|
| 1 | 2022 | 2022 → 2027 |
| 2 | 2027 | 2027 → 2032 |
| 3 | 2032 | 2032 → 2037 |
| 4 | 2037 | 2037 → 2042 |
| 5 | 2042 | 2042 → 2047 |
| 6 | 2047 | 2047 → 2052 |

**Cards take about four years to come online.** A card passed in planning year Y is under construction for the next three years. It becomes active in year Y+4 and first changes production in the last year of the round, Y+4 → Y+5. So the Reveal straight after a vote shows **one year of effect at most**, and the full effect appears in the following rounds. Tell the room this at 0:15 and again before the first Reveal, and keep it in mind when you judge predictions.

### Inside a round

| Phase | 20-min round (1–4) | 15-min round (5–6) |
|---|---|---|
| Read the year | 0–2 (2) | 0–2 (2) |
| Build an offer | 2–6 (4) | 2–5 (3) |
| Trade | 6–12 (6) | 5–9 (4) |
| Decide | 12–15 (3) | 9–11 (2) |
| Reveal | 15–20 (5) | 11–15 (4) |
| **Total** | **20** | **15** |

Overruns come out of **Reveal**. Decide never moves. A runoff or an invalid package uses about a minute of Reveal, so when that happens, drop the question about surprises in the next Read rather than run late.

## Before people arrive

- [ ] The ten-sheet kit is printed and cut (see `print/README.md`). One deck A, one mandate and one pen go on each table.
- [ ] Both coalition mats are in A4 sleeves and sit on the **floor table**, next to the operator, with two dry-wipe markers and a cloth.
- [ ] Each table has seven package slips, one for practice and one per round. Each person has a voting card set (A / B / =) and a prediction slip.
- [ ] Wall timeline: round 1–6 sheets and the replay sheet are taped up in order, with space beside them for each table's promise and question.
- [ ] The operator laptop has the pinned world loaded. Restore and replay have been tested on this laptop.
  - For the rehearsal (W4), use the existing workshop mode with `WORKSHOP=1 just run` or `?workshop=1`. Load the world from the main menu. Do not use `DEBUG_VIEW=Plan`. The M1 launch notes are in `references/simplification/m1-run-sheet.md`.
  - TODO W5/W6: the assembly entry point and its Planet / Ballot / Report views replace this step.
- [ ] Card eligibility list is on the operator's sheet. In round 1, all eight deck A cards are eligible. Gated cards and the coal conflict are in the operator check below.

## Tables and mandates

| Code | Table | Answerable for | Question it can bring |
|---|---|---|---|
| FL | Food and Land | Feed people through the transition | Where will the land come from? |
| EI | Energy and Industry | Keep production working while cutting emissions | What replaces the supply we retire? |
| LS | Living Standards | Defend everyday well-being and access to essentials | Whose consumption changes first? |
| LW | Living World | Leave habitats and biodiversity in recoverable condition | What protection survives the next budget? |

Every table gets the same cards and a different responsibility. No one plays an obstructive lobbyist or speaks for an affected community.

## Rotating roles

Each table fills these roles every round and writes the names in the roles grid on its mandate sheet.

| Role | Does |
|---|---|
| **Envoy** | Carries the table's slip and cards to other tables during Trade, and brings offers back |
| **Recorder** | Writes the package slip, the costs and the two predictions. Signs a mat for the table once the table agrees |
| **Timekeeper** | Watches the projected clock. Takes the mat to the floor table **as soon as it is signed**, and no later than the end of Trade. Counts the table's hands if asked |
| **Spokesperson** | Reads the year aloud at Read. A package's two sponsoring tables choose **one** of their two spokespersons to give its 30-second pitch at Decide |

The table decides who fills each role, within these limits:

- **Rotate every round.** Each role moves one seat to the left.
- **Rotate envoys fully.** Nobody is envoy a second time until everyone at the table has been envoy once.
- **The envoy goes home before signing.** The envoy must be back at their own table before that table signs any mat, and the mat has a tick box for this. Envoys cannot commit their table from someone else's table.
- **Small tables double up.** A table of three combines Timekeeper with Spokesperson. A table of five or six gives the spare people no fixed role. They receive visitors and check costs.
- **Written contributions count.** Anyone can write an amendment or question on the back of the slip instead of saying it aloud. The recorder reads it out. Offer this explicitly in round 1 so the quickest voices don't run every table.

## Opening and practice (0:00–0:30)

| Clock | Do |
|---|---|
| 0:00–0:05 | Seat people at four tables with three to six each. Hand out mandates. Choose practice roles. |
| 0:05–0:10 | Project the opening planet and read it together: year, emissions, temperature, biodiversity pressure, well-being and the land gauge. Each table reads its evidence box aloud. These are the 2022 values from W1's no-action run. |
| 0:10–0:15 | **Demonstrate one card** on a disposable copy of the world. Show a card on a mat, its cost against the 30-point budget, entry, advance and the report. Then **reset** to the opening world in front of the room, so everyone sees that nothing carried over. |
| 0:15–0:20 | Each table writes its **public promise**: one indicator, a direction, a target and a year. It also writes **a question the model cannot answer**. Recorders copy both onto the wall. |
| 0:20–0:27 | **Practice deal.** FL pairs with LW and EI pairs with LS. Each pair builds one package on a mat, fills in the budget and the checks, writes two predictions and signs. Nothing is voted on or entered. Wipe the mats afterwards. |
| 0:27–0:30 | Everyone writes one **personal prediction slip** and keeps it until the debrief. |

Say this at 0:15: "There are 30 points per package. They don't carry over. A package needs two tables' signatures. Each table can sign one package per round, so at most two packages reach the vote. Signing doesn't bind how anyone votes. A card takes about four years to come online, so most of what you pass shows up in later rounds."

## Round procedure (rounds 1–6)

### 1. Read the year (2 min)

- The operator projects the **Planet** view. The spokesperson at each table reads the headline aloud.
- Ask one table: "What surprised you last round?" Skip this in round 1, and skip it after a round that overran.
- The operator announces **eligible cards**: those whose prerequisites are now in force, minus cards already passed. Tables turn ineligible cards face down.
  - **A card can be passed only once.** The engine accepts a card only while it is `Inactive`. After it passes it is `Building` and then `Active`, so it cannot be passed again (model-baseline.md §2, §5.1). This is an engine rule, not a house rule.
  - None of the deck A cards has a prerequisite.
  - A gated card becomes passable in the **planning year after its enabler is passed**, i.e. the next round. Gates in the full deck: Regenerative Agriculture → Cellular Meat. Solar Push, Wind Push or Nuclear Expansion → Mass Electrification. Remediate and Protect Ecosystems → Solar Radiation Management.

### 2. Build an offer (4 min, or 3 in rounds 5–6)

- First, **30 seconds of silence**. Everyone picks a priority privately by touching a card.
- The recorder writes a **package slip** of 1–3 **different** eligible cards costing **30 points or less**, plus **two directional predictions**: an indicator and ↑ or ↓.
- Measures already in force keep working under their own rules. The budget only buys new decisions. Points left unspent **expire** at the end of the round.
- In rounds 5–6, tables may resubmit or revise an earlier slip instead of starting over.

### 3. Trade (6 min, or 4 in rounds 5–6)

- The envoy carries the slip, with its cards, to another table. Everyone else hosts visitors, checks costs or prepares amendments. Tables can put their mandate question to visiting envoys.
- **To reach the floor**, two tables agree on a package. Their timekeeper collects a free mat from the floor table. The cards go in the slots and the costs go under them. Players add up the total and shade **one gauge box per 5-point pip**, then tick the six checks.
- **Amendment.** A card substituted on the mat gets "Swapped in" ticked, along with what it **replaces** and the table that **proposed** it. The written mark is what shows the amendment. Colour is never used for this.
- **Signing.** Each envoy returns to their own table and ticks the box. The recorder signs for the table, and the two predictions are written on the mat.
  - **Any change after a signature cancels both signatures**, and both tables sign again.
- **Limits.** Each table signs **one mat per round**. There are two mats, so at most two packages reach the vote. If all four tables sign, every table is sponsoring exactly one package.
- **Hand-in.** The timekeeper takes a mat to the floor table **as soon as both tables have signed**, so the operator can check and enter it while Trade continues. The end of Trade is the last moment to hand it in, not the start of entry. A mat with one signature at the deadline does not reach the floor.

### Operator check (during Trade, as each mat arrives)

Enter each mat into the facilitator view as soon as it arrives. Until W6 builds that view, for example in the W4 rehearsal, check the mat's six boxes by hand and pass the cards in the existing workshop UI after the vote. Read the reason aloud if it is rejected. The sponsoring tables revise **at the floor table**, using up to one minute taken from Reveal.

| Invalid because | Operator sees / checks | Table revises by |
|---|---|---|
| Over budget (total > 30) | The sum of the slot costs, which matches the pips on the gauge | Dropping or swapping a card until the total is 30 or less |
| Repeated card | The same card ID in two slots | Removing the duplicate, and optionally adding a different card |
| Card already passed | The card was passed in an earlier round. The engine only accepts cards that are still `Inactive` | Swapping it for an eligible card |
| Prerequisite not passed yet | The card's "Requires" line against the cards already passed in **earlier** rounds. An enabler passed this round does not count yet. None of the deck A cards has a prerequisite (see the gates under Read the year) | Swapping the card or dropping it |
| Declared exclusion | There are none in the current deck | — |
| **Warning, not a rejection: coal conflict** | **Phase Out Coal** together with **Solar Push** (or Nuclear Expansion), in the same package or across rounds. Coal power starts at 35% of electricity, which is too little for both shifts. The engine accepts both, charges for both and shows both as active, but whichever card applies second does nothing (model-baseline.md §5.1). Both cards' "Cannot combine with" lines say so | Read the warning aloud. The table either keeps one of the pair or knowingly pays for a card that will do nothing |
| Reused sponsor (one table on both mats) | The sponsor codes on both mats | The table picks one mat. The other mat needs a different second table to sign, or it does not reach the floor |
| Fewer than two distinct tables | Sponsor codes | Finding a second table, or the package does not reach the floor |
| More than three cards or none | Slot count | Bringing it to 1–3 |

If the package is still invalid when Decide starts, it **does not go to the vote** this round. Any change re-signs both tables, as during Trade.

### 4. Decide (3 min, or 2 in rounds 5–6)

- The **Ballot** view shows each package with its cards, costs, sponsors and predictions. The predictions are fixed **before** the vote, and the operator has already entered them.
- Each **package** gets **30 seconds**, with one voice per package. That makes 1 minute of pitches and leaves 1–2 minutes for the vote and the count in both round lengths.
- **Vote.** Every participant has one vote and votes individually. Count down "3, 2, 1, show". Everyone raises **one** voting card at once: **A** (black square), **B** (dotted square) or **=** (striped, "carry current policy forward"). The operator counts hands and enters the tally.
- **Plurality wins.** Current policy counts as an option.
- **Tie between the leaders:** hold **one runoff** straight away between the tied options only, using the same cards.
- **Second tie** → **current policy advances**.
- **No sponsored package** → skip the vote. **Current policy advances**. There is always a next round.
- Points unspent by the winning package expire.

### 5. Reveal (5 min, or 4 in rounds 5–6)

1. **Before the operator advances**, the winning sponsors' recorder writes the two forecasts on the wall timeline.
2. The operator commits and advances five years, then projects the **Report**, which shows predictions next to the actual changes, with units and prior values.
3. Write the actual change beside each forecast and mark it ✓, ✗ or ?.
4. **The first response goes to someone who voted against** the winning package.
5. Record **one sentence about what they think caused the result** on the wall. Mark it as the players' explanation unless the report shows the cause.
6. Wipe the mats and return them to the floor table. Tables keep their slips.

If the projector fails, the operator reads the report aloud and fills the wall timeline by hand.

## Rounds 3–4 and the optional deck B

- At the break, rotate who speaks, so spokespersons who haven't spoken plenary take rounds 3–4.
- Newly eligible cards appear because of choices made in rounds 1–2. A gated card is passable the round after its enabler passes. Deck A has no gated cards, so this only happens if deck B is in play.
- **Deck B** has eight more cards on four more sheets. Hand it out at the break **only if** the rehearsal showed the room could handle eight choices (design.md decision 5). Otherwise leave it in the box.

## Replay (2:30–2:45)

| Clock | Do |
|---|---|
| 2:30–2:32 | The room picks **one disputed decision**. Default to the round with the closest vote, or the one most tables mention. |
| 2:32–2:34 | The operator restores the world from **before that vote** and enters the **losing package** in place of the winner. TODO W5: one-round branch. In the rehearsal, use a pre-run branch and say that it is pre-run. |
| 2:34–2:35 | The losing sponsors restate their two predictions. |
| 2:35–2:40 | Advance **one round only**. Compare the next five years for the actual and the substituted package, starting from the same state. |
| 2:40–2:44 | "What would have been different? Who would have paid for it?" |
| 2:44–2:45 | **Return to the actual timeline** and confirm the actual ending is back on screen. |

## Debrief (2:45–3:00)

| Clock | Do |
|---|---|
| 2:45–2:55 | Each table gets 2.5 minutes. **Did you keep your promise?** Give the evidence from the report. Name **one cost the dashboard missed**. Then answer your **question the model cannot answer** as far as you can. |
| 2:55–2:58 | Ask people to raise a hand if their personal prediction held, then if it didn't. Ask one person who was wrong what they had missed. |
| 2:58–3:00 | **Close with one disagreement the room still has.** Write it on the wall and stop on time. |

Prompts to keep ready:

- Which concession did your table make or refuse, and would you make it again?
- Where did the land come from in the end?
- Which result did the room explain differently from what the report shows?

## Prediction indicators

Predictions have to be checkable on the Report, so offer this fixed list. The metric IDs and units come from W1's fixtures (`meta.units`), with opening values from the 2022 no-action row. Display names per language are in `content/strings.csv` (`metric.*`).

| Indicator | Metric ID | Unit | 2022 value |
|---|---|---|---|
| Emissions | `emissions_gtco2eq` | Gt CO₂-eq per year | 51.6 |
| Temperature | `temperature_c` | °C warming (anomaly) | 1.10 |
| Biodiversity / extinction pressure | `extinction_rate` | index, dimensionless (lower is better) | 90.2 |
| Well-being (contentedness) | `outlook` | index, dimensionless | 30.0 |
| Land use | `land_use_fraction` | % of starting habitable land, protected land included | 59% |

Note for the first Reveal: whatever the room chooses, the model jumps in its first year. Temperature goes from 1.10 to 1.37 °C in 2023 and contentedness from 30.0 to 25.9, because the world's initial temperature does not match the climate model's history (model-baseline.md §2). Say this before the first Reveal, so nobody reads it as the result of the vote.

## Observer tally (rehearsal and pilot)

Record per round:

- who amended a proposal
- who spoke in plenary
- how many offers changed after a visit
- operator entry time
- one forecast per coalition and whether it held

Pass the gate when both rounds fit 20 minutes, at least three tables name a concession they made or refused, and every table explains one measured consequence.

## Open items

- **TODO (deck B):** choose the optional eight cards and their themes, effects and gates from model-baseline.md §5.1. `optional-deck-b.html` still uses placeholders.
- Indicator names and card copy in Spanish and Catalan: `content/strings.csv` (pending fluent review).
- **TODO W5:** decide whether the assembly rejects the coal conflict or only warns, per model-baseline.md open item 4. This sheet treats it as a warning.
- **TODO W5/W6:** the restore, one-round branch and return-to-actual operations used in the opening demo and the replay.
- **Spec review:** whether "Decide never moves; overruns come out of Reveal" and the one-minute revision window should be written into the assembly-session spec.
- **Human test:** the real acceptance for W2 is a physical amendment and vote with four tables and two mats. It has not been done yet.
