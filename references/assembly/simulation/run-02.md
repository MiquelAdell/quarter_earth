# W4 · 60-minute paper rehearsal · simulation run-02

**SIMULATED — 8 LLM agents (Haiku), not people. Does NOT satisfy the W4 gate.**

Purpose: test how sensitive the run-01 findings are to the model. Run-02 used
the same personas, brief, kit renders and facilitator rulings as run-01. The
only intended change was the model: Sonnet in run-01, Haiku here.

## Method

- **Players.** Eight Haiku subagents, two per table: FL Núria/Jordi, EI
  Marta/Pau, LS Carme/Àlex and LW Laia/Oriol. Each saw only the brief
  (`run-02-raw/brief.md`, identical to run-01's apart from the folder path and
  a logging instruction), its persona, and text renders of the print kit.
- **Facilitator and operator.** The orchestrator played both. It read the
  run-01 rulings verbatim, including "about one year of effect" for round-2
  cards.
- **Engine.** After each vote the real engine ran headless, as in run-01.
  Both rounds produced the same decisions as run-01: 2022 Solar Push +
  Vegetarian Mandate; 2027 Organic Transition + Degrowth in Developed
  Regions + Ban Outdoor Cats (`run-02-raw/decisions.json`). The two Reports
  (`run-02-raw/reports/`) are therefore identical to run-01's, and the
  observer-only round-2 counterfactuals in `run-01-raw/notes.md` apply here
  unchanged.
- **Roles.** Envoy+Timekeeper and Recorder+Spokesperson, swapped in round 2,
  with the same assignment as run-01. R1 envoys: Núria, Marta, Carme, Laia.
  R2 envoys: Jordi, Pau, Àlex, Oriol.
- **Evidence.** `run-02-raw/log/<name>.md` holds every agent reply verbatim,
  under its moment heading. `run-02-raw/notes.md` holds the orchestrator notes;
  its timeline summary is the authoritative sequence of facilitator actions.
  There is **no single transcript with facilitator lines** for run-02, so what
  an agent heard is inferred from its own replies and the notes. References
  below are `log/<name>.md § <heading>`.
- **Writing.** This record was written afterwards from the logs and notes.
  Nobody observed the run live.

Deviations shared with run-01: no practice deal; Recorders got an extra
final-slip turn; Trade ran as 2 discrete slots; R2 Reveal was merged with a
6-question debrief form; no physical mats, the 30 s card touch was reported in
PRIVATE, and the operator entry time was not measured. No separate
first response from an against-voter was collected.

Deviations specific to run-02:

- **R1 Reveal + R2 Build merged.** Each agent gave its R1 cause sentence and
  its R2 package idea in the same turn, to save turns. Agents therefore probably
  did not hear the other tables' cause sentences before building (no
  transcript records what was relayed).
- **R2 slot-1 envoy pitch skipped.** Hosts got the visiting envoy's slip
  directly, with no spoken pitch.
- **Improvised ruling on mismatched predictions.** In R1, FL and EI signed the
  same cards with different predictions. The facilitator ruled that the
  visiting envoy's slip predictions go on the mat (EI: Emissions ↓,
  Well-being ↓). The run sheet does not cover this (see N1).
- **Re-requested host replies.** The harness failed to deliver two R2 host
  replies (Marta, Núria) the first time, and they were requested again. The
  logs hold the delivered versions.
- **Núria's debrief was revised.** She wrote it twice to fit 150 words. The
  second version supersedes the first and is the one judged below.

The two merged/skipped turns change what agents heard and when. Any
run-01/run-02 difference in R2 Build or Trade may come partly from them and
not only from the model.

## Setup

| Item | Value |
|---|---|
| Date / place | 2026-11-26 / simulated (single orchestrator session) |
| Facilitator | Orchestrator agent (Claude), following `run-sheet.md` and the run-01 rulings |
| Observer | Post-hoc, from logs (this file) |
| Operator device + build | Headless engine `game/examples/assembly_baseline.rs`, same build as run-01. Not the workshop UI. |
| World | `engine/assets/WORKSHOP.world` (embedded; see `model-baseline.md`) |
| Players | 8 LLM agents (Haiku), in 4 pairs |
| Language of session | en |
| Kit | Deck A, 4 mandates, 2 coalition mats, package slips, voting cards, prediction slips, wall timeline, as text renders of `print/*.html` |
| Pre-run branches used instead of live entry? | No. The engine ran after each vote. No new counterfactuals; run-01's apply because the decisions are identical. |

Preflight of every Deck A card: not done. Coal conflict: spotted by Marta
from card text, never reached a mat.

Promises on the wall: FL Land use < 59% · EI Emissions < 42 Gt · LS
Well-being > 32 · LW Extinction < 88, all by 2032.

## Clock

**Proxy only.** LLM turns are not speech time. Run-02 merged one turn and
skipped another, so its counts are **not comparable** with run-01's by
design. Counts are `SAY:` utterances / approximate words.

| Segment | Planned | Start | End | Actual (min) |
|---|---|---|---|---|
| Opening + practice deal | 15 | – | – | n/a. 8 utterances, ~420 words, 2 rule questions. Practice deal skipped. |
| Round 1 (2022) | 20 | – | – | n/a. 36 utterances (see below) |
| Round 2 (2027) | 20 | – | – | n/a. 29 utterances, no runoff, plus 8 debrief answers |
| Short debrief | 5 | – | – | n/a. Merged with R2 Reveal: 8 × ≤150-word answers |

| Round | Read | Build | Trade | Decide | Reveal | Operator entry time |
|---|---|---|---|---|---|---|
| 1 | not simulated | 12 / ~730 | 15 / ~890: 7 slot-1, 4 slot-2 (all "going home"), 4 signing | 1 pitch + 8 votes | 8 cause sentences, merged into R2 Build (below) | not measured |
| 2 | not simulated | 8 merged cause+idea turns / ~600, plus 4 final slips / ~210 | 8 / ~510: slot-1 pitches skipped, 2 host replies, 4 slot-2 (all "going home"), 2 signing | 1 pitch + 8 votes, no runoff | Merged with debrief | not measured |

What the proxy suggests (weak): Haiku replies are shorter, and with one mat
per round Decide needed no runoff. Nothing here says the human clock will fit.

## Offers and votes

| Round | Package | Cards (cost) | Sponsors | Changed after a visit? (what) | Predictions (indicator ↑/↓) | Votes | Outcome |
|---|---|---|---|---|---|---|---|
| 1 | A | Solar Push (15) + Vegetarian Mandate (15) = 30 | FL + EI | Yes, for FL. FL's slip was Veg + Organic (25), taken to LW. Oriol countered Veg + Cats. Jordi, hosting, preferred EI's Solar + Veg, and FL signed that. EI signed its own slip. Predictions differed (FL Emissions ↓/Land ↓, EI Emissions ↓/Well-being ↓); the facilitator ruled for EI's. | Emissions ↓, Well-being ↓ | 6 | **Passed.** Emissions −4.78 ✓, Well-being −8.45 ✓ |
| 1 | B | none | – | LS (Luxury + Solar) refused by EI; Àlex would not sign Pau's counter. LW (Preserves + Veg) refused by FL. Oriol would have signed Veg + Cats with FL, but FL chose EI. LS and LW never visited each other. | – | – | No second mat |
| 1 | Current policy | — | — | — | — | 2 (Carme, Àlex) | – |
| 2 | A | Organic Transition (10) + Degrowth in Developed Regions (15) + Ban Outdoor Cats (5) = 30 | FL + EI | Predictions only. FL's slip (Jordi) and EI's slip (Pau) had the same three cards. Marta countered FL's Land ↑ with Well-being ↓, and Núria gave way. | Emissions ↓, Well-being ↓ | 5 | **Passed.** Emissions +5.93 ✗, Well-being −1.80 ✓ |
| 2 | B | none | – | LS (Luxury + Degrowth) refused by EI. LW (Preserves + Cats, with a wrong Land ↓ prediction) refused by FL. Both envoys went home. LS and LW never met. | – | – | No second mat |
| 2 | Current policy | — | — | — | — | 3 (Carme, Àlex, Laia) | – |

Ties / runoff / invalid amendments caught by the operator:

- **No tie and no runoff.** Only one mat reached the floor in each round, so
  every vote was A against "=".
- Votes followed table lines. Every signer voted A. LS voted "=" as a pair in
  both rounds, and Àlex voted against his own priority card (Degrowth) in R2:
  "Degrowth is my card, and I hate voting against it, but A drops well-being
  again" (`log/alex.md § ROUND 2 · DECIDE`).
- No invalid package reached the operator. Phase Out Coal was never offered.
  Marta's R2 slip forbade it: "add Phase Out Coal (it does nothing with Solar
  Push)" (`log/marta.md § ROUND 2 · BUILD, FINAL SLIP`).
- The R2 "Well-being ↓" prediction scored ✓ only because of the baseline:
  19.7 actual, and 19.7 with nothing passed (`run-01-raw/notes.md`
  counterfactuals). See N3.

## Observer tally

| Measure | Food & Land | Energy & Industry | Living Standards | Living World |
|---|---|---|---|---|
| Who amended a proposal (count) | 1: Jordi (R1, Veg + Cats to Laia) | 2: Pau (R1, Solar + Veg to Carme), Marta (R2, predictions to Jordi) | 0 | 1: Oriol (R1, Veg + Cats to Núria) |
| Who spoke in plenary (count) | 2 pitches (Jordi R1, Núria R2) + both in cause/debrief | 0 pitches. Both in cause/debrief | 0 pitches. Both in cause/debrief | 0 pitches. Both in cause/debrief |
| Offers changed after visiting another table | R1 yes (dropped Organic, took EI's package), R2 predictions only | R1 no (own slip signed), R2 no | R1 no (never signed), R2 no (never signed) | R1 no (never signed), R2 no (never signed) |
| Envoy rotated? / returned before signing? | yes / yes | yes / yes | yes / yes. **Received no visitor in either round.** | yes / yes. Received one visitor (R1) |
| Concession made or refused (quote) | Núria R1: "I've given up Organic Transition, my own card." Núria R2: "I'm giving up my land prediction to get the package signed." | Pau R2: "I accepted Degrowth … to hold the table together." Marta R2: "I'm pushing Degrowth in against my own instinct." | Àlex R1: "signing Pau's version costs us our promise" (refused). Carme: "I refused Pau's version in Round 1 and voted = in both ballots." | Oriol R1: "I've traded away the preserves I wanted." Laia R2: "I'm voting '=' and losing the Cats card rather than trading habitat for it." |
| Measured consequence explained (quote) | Jordi: "Land use rose 4 points from 2031, which I think Organic Transition caused." | Pau: "Emissions rose 2028 to 2031 … The growth came from demand, not from the cards we passed." | Carme: "Well-being fell from 30 to 21.6 by 2027. The Solar and Vegetarian package caused the drop." | Oriol: "Land use dropped to 36% when the Vegetarian Mandate came online in 2026, and that release of land is what pulled extinction pressure down." |

Visits received (slot 1; slot 2 was "going home" for all 8 envoys):

| Round | FL | EI | LS | LW |
|---|---|---|---|---|
| 1 | 2 (Marta, Laia) | 1 (Carme) | **0** | 1 (Núria) |
| 2 | 2 (Pau, Oriol) | 2 (Jordi, Àlex) | **0** | **0** |

## Participant explanations

Model truth as in run-01.md § Participant explanations. In brief:
- **R1 at 2027, against no action:** emissions 46.8 vs 57.0, so the cards cut
  about −10.2, though the Report shows −4.8 against 2022. Extinction 64.0 vs
  97.8. Land 36.4% vs 62.7%. Well-being 21.6 vs 24.3: warm-up −3.9, drift
  about −1.8, cards about −2.7.
- **R2 at 2032, against nothing passed in 2027:** emissions 52.73 vs 52.71,
  so the rise is baseline. Extinction 61.9 vs 69.3: Cats −7.4. Land 40.4% vs
  38.8%: Organic +1.6 of the +4.0 rise. Well-being 19.7 vs 19.7: no card
  effect. Degrowth: zero effect within the lag.

One sentence per table per round from that round's spokesperson (R1: Jordi,
Pau, Àlex, Oriol; R2: Núria, Marta, Carme, Laia). The partner's view is noted
where it differs.

| Round | Table | Sentence | Verdict | Why |
|---|---|---|---|---|
| 1 | FL (Jordi) | "Solar Push and the Vegetarian Mandate came online in 2026, so emissions fell and land use dropped from 59 to 36 percent, while the 2023 jump and the mandate's diet cost pulled temperature and contentedness down." | **Matches** | Timing, both cards and warm-up are right. Núria agrees. |
| 1 | EI (Pau) | "Solar Push and the Vegetarian Mandate cut emissions by 4.8 Gt, but the diet change took contentedness down 8.4 points, and the 2023 warm-up took the rest." | **Partly matches** | Emissions right, though −4.8 understates the cards' −10.2 against no action (a Report effect, M2). The whole 8.4 is pinned on the diet, when the cards caused about −2.7. Marta's sentence matches. |
| 1 | LS (Àlex) | "Vegetarian Mandate cut farm land from 59% to 36% and emissions fell, but its contentedness dip, plus the 2023 warm-up, took well-being from 30 to 21.6." | **Matches** | It omits the −1.8 drift. Carme: "Solar and the Vegetarian Mandate both cut contentedness" is **not supported**. Only the Vegetarian card has an outlook effect (`model-baseline.md` §5), and Solar's text claims none. |
| 1 | LW (Oriol) | "Land use fell from 59% to 36% when the Vegetarian Mandate came online in 2026, and extinction pressure dropped in 2027 alongside it." | **Matches** | Consistent, not isolated. Laia: "I can't yet explain why land fell that far." |
| 2 | FL (Núria) | "Round two's Degrowth and Organic cards arrived too late to offset the emissions rise already in motion … I think the round-one card mix caused that rise." | **Doesn't match** | The R1 cards cut emissions by about 10 Gt against no action. The rise is baseline. Jordi, "the 2031 cards came online too late to offset the 2025 to 2031 emissions climb", is closer, but he gives Organic all of the +4 land. |
| 2 | EI (Marta) | "Our Round 2 cards only came online in 2031, so the 2027–2031 rise in emissions (46.8 to 52.7) came from underlying demand, not our votes." | **Matches** | Her consequence answer gives Organic the whole land rise (partly: +1.6 of +4.0). Pau also **matches**: "The growth came from demand, not from the cards we passed." |
| 2 | LS (Carme) | "Our Round 1 Vegetarian and Solar package and the warm-up pushed well-being down, and we never passed the card that lifts it." | **Partly matches** | The R2 well-being fall equals the baseline. "Never passed the card that lifts it" is consistent with run-01's M5 counterfactual (Luxury → 39.7). It again blames Solar. Àlex: "emissions rose again after 2031" is wrong on timing, since emissions rose from 2027. |
| 2 | LW (Laia) | "I think Organic Transition pushed land use up … I can't explain the emissions rise, when Degrowth should have cut it." | **Partly matches** | Organic explains +1.6 of +4.0. No account of emissions. Oriol **doesn't match**: "the cats … only lowered it by 2.0", when Cats was −7.4 against nothing, and Organic is blamed for the emissions jump. |

**Nobody credited Ban Outdoor Cats**, the only real R2 effect, for the second
run in a row. Fully correct R2 explanations: **2 of 8** (Pau, Marta), against
about 1.5 of 8 in run-01. Unlike run-01, nobody blamed Degrowth for raising
emissions. The new error is **land**: Jordi, Marta, Laia and Oriol each gave
Organic the whole +4 points.

## Gate decision

| Criterion | Met? | Evidence |
|---|---|---|
| Both rounds fit 20 minutes | **Not assessable** (proxy only) | Turns carry no time, and run-02 merged one turn and skipped another. With one mat per round there was no runoff. |
| ≥ 3 tables identify a concession made or refused | **Met** (4/4) | FL Núria: "I accepted EI's predictions and Degrowth to get the mat signed." EI Pau: "I accepted Degrowth … to hold the table together." LS Carme: "I refused Pau's version in Round 1 and voted = in both ballots." LW Laia: "I refused Organic in Round 2 and voted '='." The form asked for this directly. |
| All 4 tables explain a measured consequence | **Met as stated; correct mostly for R1** | All 8 debrief answers named a consequence and a cause. For R2, the round being revealed, only EI's two answers were fully right. Four people gave Organic the whole land rise, Oriol cut Cats to "2.0", and Núria blamed the R1 cards for the emissions rise. |

**Decision:** not applicable. This simulation cannot pass or fail the gate.
**W4 has not been run.**

Reading: same as run-01. The "stated" criteria are easy for fluent agents
prompted by a form. The useful signal is the stated-versus-correct gap in R2,
and it reproduced under a second model.

Revise guidance, if this were real:
- **"Consequences unreadable → fix the report first"** applies to R2 again.
- **"Everybody agrees immediately → change the menu"** partly applies, as in
  run-01. Two models chose identical packages in both rounds, but with the
  same personas (D2), so it is a watch item for humans, not yet a menu change.
- **"One envoy makes every decision"** was not observed.
- **"Live entry too slow"** was not assessable.

W2 acceptance:
- **Was a card substituted on a mat, re-signed and voted with four tables and
  two mats?** **No.** One mat formed in each round, so the four-tables/two-mats
  condition never occurred. Substitution happened only on counters before
  signing (FL: Organic → Solar in R1).
- **Were cards readable across a table?** **Not assessable** (text only). Card
  text was quoted accurately ("coal 35 to 15 percent", "doubles electricity
  and fuel demand", Preserves "to 40%").
- **Did players spot an amendment without being told?** **Yes**, mostly on
  predictions:
  - Marta: "Your cards match our final slip … I'll counter on predictions."
  - Núria: "Pau, your package is the same three cards as our final slip."
  - Oriol: "Please check Jordi's predictions match ours before you sign."

## Comparison with run-01

| Metric | run-01 (Sonnet) | run-02 (Haiku) | Reading |
|---|---|---|---|
| Final decisions | R1 Solar + Veg; R2 Organic + Degrowth + Cats | **Identical** in both rounds | Not model-dependent. Menu, mandates and personas (private priorities held constant) fix the outcome together; this run can't separate them (D2). |
| Mats signed per round | R1: 1 (FL+EI). R2: 2 (FL+LS, EI+LW) | R1: 1 (FL+EI). R2: 1 (FL+EI) | One-mat rounds are the default, not an accident. |
| Tables frozen out | R1: LS, LW | **R1 and R2: LS, LW** | Worse with Haiku. The same pair was excluded twice (D1). |
| 4–4 tie | Yes in R2, then a 5–3 runoff | None | No tie because only one mat formed. In run-01, LS dropped Luxury for Degrowth and signed with FL, which made a second mat. Here LS kept Luxury, EI refused it, and nobody made a second visit. Bloc voting still reproduced (6–2, 5–3 along table lines). |
| Vote margins | R1 7–1. R2 4–4–0, runoff 5–3 | R1 6–2. R2 5–3 | LS voted "=" as a pair both rounds, on its promise. |
| Second visits in slot 2 | 2 (Carme R1 → LW, Oriol R2 → EI) | **0 of 8** envoy turns | Haiku envoys always went home. Carme and Oriol both considered a late visit and declined it, so this is player behaviour, not the harness. That is why LS and LW never met. |
| Visits received | R1 slot 1: two tables had 2 visitors, two had none | R1: LS 0. R2: LS 0, LW 0 | Visit imbalance recurs **without** any misroute, so it is structural, not a label problem. |
| Recorder deference (Build) | 6/8 (R1 2/4, R2 4/4) | 5/8 (R1 4/4; R2 1/4 adopted, 2 merged, 1 already agreed) | Similar overall rate, in the opposite round. Stays a simulation artefact. Haiku is not "more deferential" overall. |
| Rule questions at Opening | 8/8, all on promises (transcript § Opening)¹ | 2/8 (Jordi, Àlex), both on promises | Haiku rarely asks, so missing questions are not evidence of clear rules. Promise confusion still shows in content (R1). |
| Rule questions during play | ~20 (Build, Trade, signing; transcript `QUESTION:` lines) | 0 | Same caveat. Marta raised one only in her debrief PRIVATE: "whether the Round 2 slip had to match FL's predictions exactly". |
| Misrouted visits / theme-label confusion | 1 misroute (Marta → FL). "Society"/"Land & nature" used as table names | 0 misroutes. Theme labels never used | K1 weakened. The mismatch still exists in the kit, but it did not bite here. |
| Ban Outdoor Cats in slips | R2 final slips 4/4; personal R2 proposals 7/8 | R2 final slips 3/4 (not LS); personal R2 proposals 5/8; R1 counters 2 (Jordi, Oriol) | Still the universal filler, slightly less. |
| Phase Out Coal as filler | Yes: Laia, Oriol ("harmless filler") | No. Marta's slip forbids it as a no-op | D4 weakened. The degenerate move depends on the player. The kit contradiction remains (R4). |
| Coal conflict spotted | Yes (Pau, Laia), from card text | Yes (Marta): "Phase Out Coal now does nothing next to Solar Push" | The "Cannot combine with" line works with both models (K5). |
| LW Preserves land-use misconception | Oriol and Laia learned late, through a facilitator ruling | Laia predicted **Land ↓** for Preserves + Cats. Oriol: "Preserves cuts farmland pressure." In R1 Jordi had told Laia it counts: "I don't know that and can't check it." No ruling was asked for | K2 confirmed and stronger. Hearing it from another player did not fix it, because her sheet doesn't say it. |
| Luxury in R2 | LS dropped it, citing the "one year of effect" framing | LS kept it (Luxury + Degrowth). Nobody cited the lag. EI refused it: "It doubles electricity and fuel demand per person" | Same outcome (Luxury not passed, LS missed its promise) by a different route. By run-01's M5 counterfactual, Luxury adds 0 Gt by 2032, so EI's reason is wrong within the window. |
| Correct R2 cause explanations | ~1.5 of 8 | 2 of 8 (Pau, Marta) | Same order. The error moved from emissions (Degrowth blamed) to land (Organic given all +4). Cats uncredited in both. |
| Promise outcomes | FL kept, EI missed, LS missed, LW kept | Same | Promises are decided by R1 step changes and the baseline (M4), whatever the model. |
| Prediction mismatch on the mat | None. Both rounds' signers wrote the same pair | **Both rounds.** R1 needed an improvised ruling. In R2 Núria gave up Land ↑ for EI's pair | New (N1). One mat holds one prediction pair, but each table's slip carries its own. |
| Rule slips by agents | Few. Àlex misremembered his own priority | Oriol: "If the room wants 85 we can raise it in round 2" (promises are locked). Jordi proposed an emissions promise for FL | Haiku is looser with rules and mandates. Low weight. |
| Memory drift in debrief | Àlex reversed which card he gave up | Jordi: "I gave up Solar Push for Organic Transition in Round 2" (Solar passed in R1). Oriol mixed up 89/88 and corrected it himself | Same artefact class, both models. Debrief self-reports need a written record to check against. |
| Quiet persona | Oriol not quiet | Oriol "(quietly)", and he used a written note: "I've written the promise and the question on the back of the slip, so you can read it out" | "Written contributions count" was exercised once. Still not a test of a quiet human. |

¹ Source conflict: run-01.md R1 says all 8, and its transcript shows 8
non-"none" Opening `QUESTION:` lines, all about promises. `run-01-raw/notes.md`
says "6 of 7", and the run-02 notes say 7/8. The transcript count is used here.

## Robust findings

Labels:
- **confirmed**: reproduced under Haiku, so it is more likely a real kit or
  spec issue.
- **weakened**: it could have recurred and didn't, so it is more likely a
  model artefact or player-dependent. "Not exercised" means the precondition
  never arose, and the priority does **not** drop.
- **new**: first seen in run-02.

Severity as in run-01:
- **high**: fix or decide before the human W4.
- **med**: fix before W5–W7, or watch in W4.
- **low**: note it.

Code paths are relative to `~/code/quarter_earth`. Kit paths (`content/`,
`print/`, `run-sheet.md`, `rehearsal.md`) are under `references/assembly/`.
`<oc>/` = `~/personal/quarter-earth/files/openspec/changes/assembly-workshop/`.

### Run-01 findings re-tested

| ID | Finding (run-01 severity) | Label | Run-02 evidence | File pointer |
|---|---|---|---|---|
| D2 | Card choice converged (med) | **confirmed** | Identical packages in both rounds with a different model. But the personas, and so their private priorities, were held constant, so this shows menu + mandates + personas fix the outcome, not the menu alone. Humans bring other priorities. Keep at **med**: watch whether humans converge before changing the menu. | `<oc>/design.md` decision 5; `model-baseline.md` §5 |
| D1 | Leftover tables frozen out (high) | **confirmed, stronger** | LS and LW frozen out in **both** rounds. LS received no visitor in either round. Every vote was A against "=". | `run-sheet.md` § 3 Trade (mid-Trade "who is still free" call); `rehearsal.md` Observer tally (add "visitors received" and "tables with no mat") |
| K2 | "Protected land counts as land in use" only on FL's sheet (high) | **confirmed, stronger** | Laia predicted Land ↓ for Preserves. Oriol: "Preserves cuts farmland pressure, which helps Food & Land's promise too." Being told by Jordi did not correct it. | `content/strings.csv` `card.f759d87d-69ff-433b-8e3d-f3dbb776543a.effect`, `mandate.living_world.evidence.2.why`; `print/sheet-08-mandate-living-world.html`; Report unit label |
| M2 | Report has no baseline, so drift gets blamed on cards (high) | **confirmed** | The misattribution moved rather than disappeared. Four people gave Organic the whole +4 land (true share +1.6). Núria blamed the R1 cards for the emissions rise. Only EI named underlying demand. Also N3. | `<oc>/design.md` item 4; `<oc>/specs/assembly-session/spec.md` Scenario "Predictions survive the vote"; `run-sheet.md` § 5 Reveal step 2 |
| M3 | Ban Outdoor Cats text understates its effect about 15× (high) | **confirmed** | Oriol at Opening: "Ban Outdoor Cats alone only gets us to 89.7." In debrief: "the cats … only lowered it by 2.0." Nobody credited Cats. | `content/strings.csv` `card.1b98f39e-bb87-4e9f-8c95-565839b72197.effect`; `model-baseline.md` §5 row 8 |
| M1 | One-year lag makes R2 unreadable in 2 rounds (high) | **confirmed** | Núria: the lag "made the 2032 report nearly meaningless for my card". Pau: "I'm not sure the rules made clear that cards passed in Round 2 barely count in 2032." | `rehearsal.md` (W4 format); `run-sheet.md` "Cards take about four years" |
| M4 | Step changes decide 2032 promises in R1 (high) | **confirmed** | Same promise outcomes. Laia: "we'd already met it at 64.0 in 2027". LS was 10 points short after R1 again. | `run-sheet.md` "Cards take about four years" and § Opening promise guidance; `model-baseline.md` §2 "Policy timing" |
| D3 | Promise acts as a binding constraint (med) | **confirmed, stronger** | LS voted "=" in both rounds on its promise. Àlex voted against his own Degrowth card. Jordi rejected Organic as land-hungry, and Núria refused Preserves, both on FL's promise. Laia voted "=" over Organic. | `<oc>/design.md`; `run-sheet.md` § Opening and practice |
| R1 | How a promise is judged is unscripted (high) | **confirmed** (by content; Haiku asks less) | Jordi: "is a temperature promise judged from 1.10 or from 1.37?" Àlex: "Is the promise checked only on the end-of-game report value?" Oriol's slip: "we can raise it in round 2." Núria's debrief: "I never got a clear answer on whether the land-use promise was judged against the opening 59 percent." | `run-sheet.md` § Opening and practice (0:15 script); `content/strings.csv` `mandate.*` promise keys |
| R5 | Pair roles don't work at two-person tables (high) | **confirmed** | Every host deferred signing to the absent envoy: "I can't sign alone; Núria is at LW" (Jordi), "Pau has to be back before anyone signs" (Marta). The timekeeper was always the one away. In R2 Jordi and Pau each visited the other's table, so both tables waited on each other. | `run-sheet.md` § Rotating roles; `print/sheet-05…08` roles grid; `content/strings.csv` `mandate.*` role keys |
| R2 | Structural ties; the second-tie rule can kill a shared card (high) | **confirmed** for bloc voting; tie **not exercised** | Only one mat, so no tie was possible. Votes still split along table lines (6–2, 5–3). Two mats plus bloc voting would reproduce the 4–4. Priority unchanged. | `<oc>/specs/assembly-session/spec.md` Scenario "Plurality and deadlock"; `run-sheet.md` § 4 Decide |
| R3 | Same card on both mats not covered (med) | **confirmed** (precondition); rule **not exercised** | Degrowth and Cats were each on 3 of 4 R2 final slips, so a shared card was likely if a second mat had formed. | `<oc>/specs/assembly-session/spec.md` § Sponsor and vote; `run-sheet.md` § Operator check, § 4 Decide |
| R4 | Coal conflict: kit says "fail", run sheet says "warn" (high) | **confirmed** (model-independent kit text); **not exercised** | Nobody put coal on a slip. The contradiction is in the materials regardless. | `run-sheet.md` § Open items; `content/strings.csv` `print.mat.check.4`; `card.*.excludes` |
| K4 | Cards give no sense of effect size (med) | **confirmed** | Núria: "I can't check 35 Gt in my head." Laia: "I don't know that and can't check it." Oriol's 89.7 arithmetic from the Cats text. Marta refused Luxury on its text. | `<oc>/design.md` decision 5; `content/strings.csv` `card.*.effect` |
| K5 | The coal line on the card works (low, positive) | **confirmed** | Marta, from card text alone. | `content/strings.csv` `card.*.excludes` |
| D6 | Envoys rotated and returned (low, positive) | **confirmed** | All 8 envoy turns ended "going home". No envoy committed a table from outside it. | `run-sheet.md` § Rotating roles |
| M6 | Preserves causes a large land-use jump (med) | **confirmed** (through K2) | Núria refused Preserves: "taking 30 percent of habitable land for protection leaves less for farms". LW did not know this. | covered by K2; card watch-out line in `content/strings.csv` `card.f759d87d…` |
| M7 | Baseline rises before cards land (low) | **confirmed** | Oriol's missed cost: "extinction pressure climbed from 2023 to 2026 before the land release". Jordi: "the 2025 to 2031 emissions climb." | `run-sheet.md` first-Reveal note |
| M5 | "About one year of effect" pushed LS off Luxury (high) | **weakened** (mechanism); outcome **confirmed** | Nobody cited the lag against Luxury, and LS kept it. Luxury still failed, this time on EI's "doubles demand" reading of the card text, though it adds 0 Gt by 2032. Luxury remains the pivotal R2 card, and the script fix stands. **Verified 2026-11-26 by counterfactual:** with R1 as played and R2 = LS's slip (Luxury + Degrowth), the 2032 values are well-being **39.7** (LS's ">32" kept), emissions 52.71 (identical to passing nothing), extinction 69.3 and land 38.8%. EI's refusal cost LS its promise and bought nothing measurable on emissions within the window. | `run-sheet.md` "Cards take about four years" and the R2 answer script |
| K1 | Theme labels don't match table names (high) | **weakened** | 0 misroutes, and nobody used "Society" or "Land & nature". The visit imbalance recurred without it, so that evidence moves to D1. Still a cheap fix. Downgrade to **med**. | `content/strings.csv` `theme.society.short`, `theme.land.short`; `print/sheet-01-04-deck-a.html` |
| K3 | Prediction slip and wall laid out for 6 rounds (med) | **weakened** | Jordi wrote "end of round 2 (2032)" correctly. One mild mix-up remained: Pau, "By the end of round 2032". | `print/extra-prediction-slips.html`; `print/extra-wall-timeline.html` |
| D4 | Phase Out Coal used as do-nothing filler (med) | **weakened** | Marta forbade it as a no-op. Player-dependent. R4 still stands. | `run-sheet.md` § Operator check |
| D5 | Sealed runoff gives crossed tie-breakers (med) | **weakened** (not exercised) | No runoff was held. Priority follows R2. | `run-sheet.md` § 4 Decide |
| R6 | Unscripted procedure questions (low) | **weakened** | Haiku asked none during play. Low detector sensitivity, not evidence of clarity. | `run-sheet.md` Round procedure FAQ |
| A1 | Artefact: recorder deference | **confirmed as artefact** | 5/8 vs 6/8, in the opposite round. It doesn't track the model in one direction, so treat it as LLM behaviour. | `simulation/run-01.md` § Simulation artefacts; R2 final-slip headings in `run-02-raw/log/{nuria,marta,carme,laia}.md` |
| A2 | Artefact: crossed simultaneous concessions | **confirmed as artefact** | Marta and Pau crossed again: Marta took Pau's 45, and Pau took Marta's 42. | `simulation/run-01.md` § Simulation artefacts; `run-02-raw/log/marta.md`, `log/pau.md` § ROUND 1 · READ THE YEAR + BUILD AN OFFER |
| A3 | Artefact: memory drift in debrief | **confirmed as artefact** | Jordi: "I gave up Solar Push for Organic Transition in Round 2." | `simulation/run-01.md` § Simulation artefacts; `run-02-raw/log/jordi.md` § DEBRIEF (2032) |
| A4 | Artefact: LLM honesty norms | **confirmed as artefact** | Unprompted cost disclosure again: "It costs some contentedness, and we're saying so" (Jordi pitch), "tell me honestly" (Núria to LW). | `simulation/run-01.md` § Simulation artefacts; `run-02-raw/log/jordi.md` § ROUND 1 · DECIDE, PITCH FOR A; `log/nuria.md` § ROUND 1 · TRADE (slot 1) |

### New in run-02

**N1. Two tables, two prediction pairs, one mat: nothing says who reconciles
them. (high, before W4)** — **new**
- Evidence: in R1, FL wrote Emissions ↓ / Land ↓ and EI wrote Emissions ↓ /
  Well-being ↓ for the same cards (`log/jordi.md § ROUND 1 · SIGNING TURN`,
  `log/pau.md § ROUND 1 · SIGNING TURN`). The facilitator improvised "the
  visiting envoy's predictions go on the mat". That rule breaks in R2, where
  Jordi and Pau each visited the other's table, so both were visitors. Núria
  simply gave way (`log/nuria.md § ROUND 2 · SIGNING TURN`). Marta's debrief:
  "what confused me was whether the Round 2 slip had to match FL's
  predictions exactly before signing."
- The mat has one prediction block (`print.mat.pred_title`, "Predictions —
  write before the vote"). Each table's slip carries its own two predictions.
  The run sheet says only "the two predictions are written on the mat".
- It matters because the prediction pair decides what the Reveal scores. In
  R1 FL's Land ↓ would have been the most visible hit. It was dropped by
  ruling.
- Run-01 never hit this because both signers happened to write the same pair.
- Change: first `<oc>/specs/assembly-session/spec.md` Scenario "Predictions
  survive the vote" ("its sponsors record two directional predictions" is
  ambiguous between one shared pair and one pair per sponsor). Then
  `run-sheet.md` § 3 Trade "Signing" bullet, and `content/strings.csv`
  `print.mat.pred_*` / `print/sheet-09-10-coalition-mats.html`. Either make
  the predictions part of what must match before signing, or give the mat
  one prediction row per sponsoring table.

**N2. Envoys never made a second visit; one table received no visitor all
session. (med, watch in W4)** — **new**
- Evidence: all 8 slot-2 envoy turns were "going home". Carme: "don't want to
  make a last-minute visit that risks a changed package" (`log/carme.md §
  ROUND 1 · TRADE, SLOT 2`). Oriol left the choice to Laia
  (`log/oriol.md § ROUND 2 · TRADE, SLOT 2`). LS received 0 visitors in both
  rounds, and LW received 0 in R2. This is the direct cause of D1 here.
- It may be partly an artefact of discrete slots. In continuous Trade, humans
  might circulate more. But "go home and protect what we have" is a plausible
  human move too.
- Change: `run-sheet.md` § 3 Trade (facilitator prompt at the half-way mark:
  "tables without a visitor yet: LS, LW"). `rehearsal.md` Observer tally: add
  a "visitors received" row.

**N3. Prediction scoring rewards betting on drift. (med)** — **new**
- Evidence: package A's R2 "Well-being ↓" scored ✓ at 19.7, exactly the
  nothing-passed value (`run-01-raw/notes.md` counterfactuals). Núria picked
  it as "the honest bet with Degrowth in" (`log/nuria.md § ROUND 2 · SIGNING
  TURN`). The card had no effect. In R1 too, most of the ✓ on Well-being ↓ was
  warm-up and drift.
- Against the Report as it stands, a falling baseline indicator is a free
  correct prediction. That teaches the wrong lesson about cause.
- Change: tie to M2. `<oc>/design.md` item 4 (no-action reference) and
  `run-sheet.md` § 5 Reveal step 2: judge predictions against the no-action
  line, or at least say aloud when the baseline moved the same way.

**N4. Card text makes players refuse cards whose cost won't show in the
window. (low)** — **new**
- Evidence: Marta refused Luxury because "It doubles electricity and fuel
  demand per person" and threatens the 42 promise
  (`log/marta.md § ROUND 2 · TRADE, SLOT 1`). Run-01's counterfactual has
  emissions identical by 2032 with Luxury in the package. The text is right for the
  6-round game. In a 2-round rehearsal it decided the R2 outcome for a reason
  the Report could never test.
- Change: none to the card. Note it in `rehearsal.md` (W4 format) beside M1.
  It is another reason the 2-round format misleads.

**N5. Rule and mandate slips. (low)** — **new** (Haiku-leaning)
- Oriol: "If the room wants 85 we can raise it in round 2", though promises
  are locked (`log/oriol.md § ROUND 1 · READ THE YEAR + BUILD AN OFFER`).
- Jordi proposed an emissions promise for FL, which is not FL's indicator
  (`log/jordi.md § OPENING`).
- Both were corrected inside the table, so the kit is not at fault.
- Change: the R1 script ("locked once on the wall") and one line in the
  mandate promise box (`content/strings.csv` `mandate.*` promise keys).

## What to fix before the human W4

Ranked by how robust the finding is, how much it distorts the session and how
cheap the fix is:

1. **Mid-Trade "who is still free / who has had no visitor" call (D1, N2).**
   Half the room sat out in 3 of 4 rounds across both runs. A one-line
   facilitator prompt in `run-sheet.md` § 3 Trade.
2. **Report baseline (M2, N3, M1).** Give W4 a disclosed no-action line (from
   `fixtures/a_no_action.csv`) and judge predictions against it. Pick the
   round-2 option: a third reveal-only advance, a pre-run branch, or an
   explicit warning.
3. **Card and sheet text (K2, M3).** Say on the Preserves card and the LW
   sheet that protected land counts as land in use. Fix the Cats magnitude.
   Both caused wrong predictions with both models.
4. **Mat predictions (N1).** Amend the spec first, then the run sheet and mat:
   who reconciles the two tables' predictions.
5. **Run-sheet scripts (R1, R5, M5, M4).** Promise rulings; the two-person
   role rule; "some effects arrive in full in the online year".
6. **Spec decisions that didn't trigger here but will with two mats (R2, R3,
   R4).** Second-tie rule, same card on both mats, coal reject-or-warn.
7. **Watch D2 in W4** (do humans converge on Solar + Veg and Cats?) before
   redesigning deck A. Cheap text fixes with weaker evidence (K1, K3).

**Run-03:** worth it only after fixes 1–5, to test them. Keep it on
**Sonnet**: Haiku asks almost no rule questions, so it is a poor detector of
ambiguity. The most informative single variation is **different personas'
private priorities with the same kit**: that separates menu-driven
convergence (D2) from persona-driven convergence. Second choice: 3-person
tables, to test R5 and N2. Leave es/ca for later.
