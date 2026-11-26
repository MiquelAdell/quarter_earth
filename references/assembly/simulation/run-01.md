# W4 · 60-minute paper rehearsal · simulation run-01

**SIMULATED — 8 LLM agents, not people. Does NOT satisfy the W4 gate.**

Method. Eight Sonnet subagents each played one participant, two per table:
FL Núria/Jordi, EI Marta/Pau, LS Carme/Àlex and LW Laia/Oriol. Each agent saw only
the brief (`run-01-raw/brief.md`), its persona, and text renders of the print kit.
The orchestrator played facilitator and operator. After each vote the real engine
ran headless (`game/examples/assembly_baseline.rs` with the embedded
`WORKSHOP.world`, through `simulation/harness/sim_engine.py`). The session ran
2 rounds (2022, 2027) with deck A only. Pairs split the four roles as
Envoy+Timekeeper and Recorder+Spokesperson and swapped them in round 2. Each agent's
context ended at about 72–78k tokens, after about 15 turns. The orchestrator
wrote this record afterwards from the transcript. Nobody observed it live.

The evidence is in `references/assembly/simulation/run-01-raw/`. `transcript.md` is
the full log, including PRIVATE lines and `[ORCH NOTE]`s. `notes.md` holds the
orchestrator's findings and the **observer-only** round-2 counterfactuals, which
players never saw. `reports/` holds the two Reports shown to players. Section
references below (for example "transcript § Round 2 Decide") point to headings
in `transcript.md`. Round-1 baseline numbers come from the committed no-action
fixture `references/assembly/fixtures/a_no_action.csv`. Its 2022–2025 rows match
this run exactly.

Deviations from the run sheet:

- The practice deal was not simulated.
- The run sheet wants the first response from an against-voter before the
  others. Here it was collected in parallel with the other cause sentences.
- Recorders got an extra "final slip" turn after hearing their envoy.
- Round-2 Reveal and the debrief were merged. The debrief was a 6-question
  form answered by each person, not 2.5 minutes per table.
- Trade ran as 2 discrete visit slots, not 6 continuous minutes.
- Also not simulated: spokespersons reading the year aloud, the 30 s silent
  card touch (agents reported it in PRIVATE), physical mats with "Swapped in"
  marks, and operator entry time.
- The facilitator improvised rulings the run sheet doesn't script. They are
  logged in transcript § Opening "FACILITATOR rulings".

## Setup

| Item | Value |
|---|---|
| Date / place | 2026-11-26 / simulated (single orchestrator session) |
| Facilitator | Orchestrator agent (Claude), following `run-sheet.md` |
| Observer | Post-hoc, from transcript (this file) |
| Operator device + build | Headless engine `game/examples/assembly_baseline.rs` (release build), commit: 13c3ffbf (2026-11-25). Not the workshop UI. |
| World | `engine/assets/WORKSHOP.world` (embedded; see `model-baseline.md`) |
| Players | 8 LLM agents, in 4 pairs (one pair per mandate table) |
| Language of session | en |
| Kit | Deck A (8 cards), 4 mandates, 2 coalition mats, package slips, voting cards, prediction slips, wall timeline, as text renders of `print/*.html` |
| Pre-run branches used instead of live entry? | No live entry at all. The engine was run after each vote. After round 2, observer-only counterfactuals (nothing, Degrowth only, package B) were run and not disclosed. |

Preflight of every Deck A card: not done in this run. Coal conflict: noted.
It never reached a mat.

## Clock

**Proxy only.** Real minutes can't be assessed. LLM turns are not speech time.
The figures below count player utterances and spoken words (`SAY:` lines) per
phase. Table phases run in parallel across 4 tables. Plenary phases are serial.

| Segment | Planned | Start | End | Actual (min) |
|---|---|---|---|---|
| Opening + practice deal | 15 | – | – | n/a. 8 utterances, ~480 words, 8 rule questions (all on how promises are judged). Practice deal skipped. |
| Round 1 (2022) — read/build/trade/decide/reveal 2/4/6/3/5 | 20 | – | – | n/a. 41 utterances (see below) |
| Round 2 (2027) | 20 | – | – | n/a. 47 utterances, including a full runoff, plus 8 debrief answers |
| Short debrief | 5 | – | – | n/a. Merged with R2 Reveal: 8 × ~150-word answers |

Per-round detail (utterances / approximate spoken words):

| Round | Read | Build | Trade | Decide | Reveal | Operator entry time (confirm → report on screen) |
|---|---|---|---|---|---|---|
| 1 | not simulated | 11 / ~810 (≈200 per table) | 13 / ~1,240, 2 slots, 3 facilitator rulings | 1 pitch + 8 votes | 1 first response + 7 cause sentences, ~500 words serial (≈4 min at 130 wpm) | not measured (headless) |
| 2 | not simulated | 12 / ~1,000, 4 facilitator answers | 17 / ~1,460, 2 slots | 2 pitches + 8 votes + 8 runoff votes | Merged with debrief: ~1,450 words serial (≈11 min at 130 wpm) | not measured (headless) |

What the proxy suggests, and it is weak evidence:
- Round 2 Decide needed a runoff. The run sheet takes runoff time from
  Reveal.
- A spoken debrief from every person does not fit in 5 minutes. The run
  sheet's per-table format (one voice per table) is the one to use.

## Offers and votes

| Round | Package | Cards (cost) | Sponsors | Changed after a visit? (what) | Predictions (indicator ↑/↓) | Votes | Outcome |
|---|---|---|---|---|---|---|---|
| 1 | A | Solar Push (15) + Vegetarian Mandate (15) = 30 | FL + EI | Yes, both sides. FL's slip was Veg + Organic (25) and EI's was Solar + Cats (20). Both hosts, Pau and Jordi, countered independently with the identical Solar + Veg. FL dropped Organic and EI dropped Cats. | Emissions ↓, Land use ↓ | 7 | **Passed.** Emissions −4.78 ✓, Land −22.2 pts ✓ |
| 1 | B | none | – | LS (Luxury + Solar) was refused by EI and LW. LW (Preserves + Veg) was refused by FL. LS and LW met in slot 2 and were incompatible. | – | – | No second mat |
| 1 | Current policy | — | — | — | — | 1 (Carme) | – |
| 2 | A | Degrowth in Developed Regions (15) + Organic Transition (10) + Ban Outdoor Cats (5) = 30 | FL + LS | Yes, for FL. Jordi's slip was Organic + Luxury + Cats. Carme countered with Degrowth replacing Luxury, and Núria accepted. LS signed its own slip unchanged. | Emissions ↓, Land use ↑ | 4, runoff 5 | **Passed in runoff.** Emissions +5.93 ✗, Land +4.0 pts ✓ |
| 2 | B | Expand Nature Preserves (15) + Degrowth in Developed Regions (15) = 30 | EI + LW | Yes, both sides. EI's slip was Cats + Degrowth and LW's was Preserves + Cats. Laia's counter replaced Cats with Preserves (for EI) and with Degrowth (for LW). | Emissions ↓, Extinction ↓ | 4, runoff 3 | Lost |
| 2 | Current policy | — | — | — | — | 0 | – |

Ties / runoff / invalid amendments caught by the operator:

- **R2 first vote tied 4–4–0**, on pure sponsor blocs. Every person voted for
  the mat their table signed.
- **Runoff A 5 – B 3.** Three people crossed over, in opposite directions:
  - Marta B→A: "a tie passes nothing, so someone has to move";
  - Pau B→A: "a second tie loses it, so I'm giving way";
  - Àlex A→B: "I'd rather it pass than have us tie again".

  Àlex's switch cancelled one of the EI switches.
- Degrowth was on **both** mats. Under the second-tie rule, a 4–4 runoff would
  have advanced current policy, so nothing would have passed, even though all
  8 people had just voted for a package containing Degrowth.
- No invalid package reached the operator. Oriol offered Phase Out Coal to EI
  as filler in R2 and Marta refused it, so the coal conflict never reached a
  mat.

## Observer tally

| Measure | Food & Land | Energy & Industry | Living Standards | Living World |
|---|---|---|---|---|
| Who amended a proposal (count) | 3: Jordi 2 (R1, to Marta and Laia), Núria 1 (R2, to Oriol) | 1: Pau (R1, to Núria) | 1: Carme (R2, to Jordi) | 1: Laia (R2, to Pau) |
| Who spoke in plenary (count) | 2 pitches (Jordi R1, Núria R2) + both in cause/debrief | 0 pitches. Both spoke only in cause/debrief. | 1 first response (Carme R1) + both in cause/debrief | 1 pitch (Laia R2) + both in cause/debrief |
| Offers changed after visiting another table | R1 yes, R2 yes | R1 yes, R2 yes | R1 no (never signed), R2 no (signed own slip) | R1 no (never signed), R2 yes |
| Envoy rotated? / returned before signing? | yes / yes | yes / yes | yes / R1: Carme was still out when Trade closed, so LS could not sign | yes / yes. In R2 Oriol pitched to Jordi, who was away. |
| Concession made or refused (quote) | Jordi R1: "I'll drop Organic for this one." Núria R2, refusing Preserves: "That's well past our 57% wall promise, and I can't hand that over." | Pau R1: "Vegetarian at 15 is above what I authorised, but the land answer is the reason." Pau runoff: "I'm giving way." | Àlex R1, dropping Degrowth for Luxury: "I'm swallowing a lot." Carme R2: "I gave up Luxury for All, my own priority, for Degrowth." | Laia R1: refused to drop Preserves. Oriol R2: gave up Cats, his private priority. Laia R2: "Cats gets dropped, which I accept." |
| Measured consequence explained (quote) | Núria: "Land use fell from 59% to 36% in Round 1 … the Vegetarian Mandate caused it." | Pau: "Round 1 emissions fell from 51.6 to 46.8 once Solar Push and Vegetarian came online in 2026." | Carme: "Contentedness fell from 30 to 21.6 … the Vegetarian Mandate caused much of it beyond the warm-up dip." | Oriol: "Extinction fell from 96 to 64 in 2027, right after land use fell … I think the diet cut caused it." |

## Participant explanations

One sentence per table per round, taken from that round's spokesperson.
That is the R1 Recorder+Spokesperson and, after the swap, the R2 one. The
partner's view is noted where it differs. "Model truth" means the engine
reports, `model-baseline.md` §2/§5/§5.1, the R2 counterfactuals in
`notes.md`, and the no-action fixture for R1.

Model truth, in brief:
- **R1 vs no action at 2027:** emissions 46.8 vs 57.0 (cards about −10.2,
  though the Report shows −4.8 against 2022); extinction 64.0 vs 97.8; land
  36.4% vs 62.7%; well-being 21.6 vs 24.3.
- Of the 8.4-point well-being fall: the warm-up is −3.9, baseline drift about
  −1.8, and the cards about −2.7.
- The run had no single-card branch, so Solar and Veg can't be separated.
- **R2 at 2032:**
  - Degrowth and Preserves had zero effect. Degrowth-only equals nothing
    passed.
  - The +5.9 Gt rise is the baseline trend. No-action steps up by +2.7 in 2026
    and +2.9 in 2031, the same years cards come online.
  - Ban Outdoor Cats caused the extinction drop: 61.9 vs 69.3 with nothing
    passed, about −7.4.
  - Organic added +1.6 pts of land and nothing to emissions (52.73 vs 52.71).

| Round | Table | Sentence | Verdict | Why |
|---|---|---|---|---|
| 1 | FL (Jordi) | "The Vegetarian Mandate is what dropped land use from 59% to 36%, because the diet shift freed farmland, and Solar Push cut the emissions." | **Matches** report/model | Veg is the only land-reducing card passed, and land fell in 2026 when it came online. The run has no single-card branch, so the result is consistent rather than isolated. |
| 1 | EI (Pau) | "Emissions fell once Solar Push and the diet kicked in during 2026, replacing coal while the diet cut demand." | **Matches** | Timing and mechanism are right. Marta's "I expect more once they run a full five years" does **not** match: effects are steps, flat 2026→2027, and emissions rose through R2. |
| 1 | LS (Àlex) | "The well-being drop is part warm-up and part that same diet cut." | **Matches** | Warm-up −3.9 and cards about −2.7, with about −1.8 baseline drift unmentioned. Carme's first response ("the Vegetarian Mandate caused the rest") overstates the card's share. |
| 1 | LW (Oriol) | "I think the diet cut caused it. Less farmland was needed … and extinction pressure dropped at the same moment." | **Matches** | Extinction 64.0 vs 97.8 with no action. Land-driven, consistent with the report but not isolated. |
| 2 | FL (Núria) | "Emissions rose because, I think, Degrowth, Organic and the cats ban did not cut demand within the year of effect we got." | **Partly matches** | Right that the R2 cards did nothing to emissions. It still frames the rise as their failure, when it is the baseline trend. Jordi: "I couldn't tell how Degrowth could raise emissions" (does **not** match). |
| 2 | EI (Marta) | "Emissions climbed every year from 2027 even though Solar Push and the diet were in force, so something other than our cards drove it." | **Matches** (the sentence) | But she reads the 2031 jump as card-linked, and her lesson, "I wouldn't trust Degrowth as our emissions card again" / "failed for Degrowth", does **not** match: Degrowth had zero effect within the lag, which is not the same as failing. Pau: "the cards didn't cut anything", which matches, but he too "would trust Degrowth less". |
| 2 | LS (Carme) | "I suspect something in our package, probably Organic farming, added to it in 2031." | **Doesn't match** | Organic's emissions effect is +0.02 Gt. The 2031 step is in the no-action run too. Àlex: "the baseline was still growing and our new cards only came online in 2031" **matches**, and he is the only person who named baseline growth. |
| 2 | LW (Laia) | "Emissions climbed every year before the 2031 cards arrived, so part of it was the system itself, and I can't tell how much the new cards added." | **Matches** on emissions | But "Round 2 only trimmed [extinction] by 2" misreads Cats' −7.4 effect against nothing. Oriol: "Emissions rose because Degrowth, Organic and Cats came online in 2031 and nothing offset the demand" does **not** match. |

**Nobody credited Ban Outdoor Cats** for the only real R2 effect, the
extinction drop in 2032. Oriol had dismissed it in Build: "Is the cats effect
(0.5) worth less now that we're at 64?"

## Gate decision

| Criterion | Met? | Evidence |
|---|---|---|
| Both rounds fit 20 minutes | **Not assessable** (proxy only) | LLM turns carry no time. Proxy: R1 had 41 utterances. R2 had 47 utterances including a full runoff, and its merged Reveal/debrief ran ~1,450 serial words (≈11 min against 5 planned). |
| ≥ 3 tables identify a concession made or refused | **Met** (4/4) | FL Núria: "I refused Expand Nature Preserves … I'd refuse it again." EI Pau: "I moved my vote from B to A in the runoff … I'd do it again." LS Carme: "I gave up Luxury for All, my own priority, for Degrowth." LW Laia: "I refused to drop Preserves … I'd refuse again." The debrief form asked for this directly, and Àlex misremembered which card he had given up. |
| All 4 tables explain a measured consequence | **Met as stated, but correct only for round 1** | All 8 debrief answers named a consequence and a cause. 7 of 8 fell back on the R1 land or emissions drop ("Land use fell from 59% to 36% … the Vegetarian Mandate caused it"), and those are correct. For R2, the round actually being revealed, about 1.5 of 8 cause sentences were fully correct (Àlex; Marta in part). Two people blamed Degrowth or Organic for baseline growth, and nobody identified Cats. |

**Decision:** not applicable. This simulation cannot pass or fail the gate.
**W4 has not been run.**

What this means for the real W4. The two "stated" criteria were met easily
here, but the debrief form prompted for them directly and the agents were
fluent. Don't take that as a sign humans will meet them. The useful signal is
the gap between stated and correct explanations. With a 2-round format and a
Report that has no baseline, round 2 can't show what the room chose. Players
then invent causes from background drift. The round 1 explanations were right
because the R1 cards made large single-year steps. Fix the items marked
"before W4" in the findings, then run the human rehearsal against the template
unchanged. In the explanations row, have the observer record correctness as
well as whether an explanation was given.

Revise guidance, if this were real: "Lively debate but consequences
unreadable → fix the report first" applies to round 2. "Everybody agrees
immediately" partly applies to card choice (Solar Push in R1, Cats in R2),
though the negotiation was not consensual. "One envoy makes every decision" was
not observed. "Live entry too slow" was not assessable.

W2 acceptance:
- **Was a card substituted on a mat, re-signed and voted with four tables and
  two mats?** Four tables and two mats voted in R2: yes. Substitution happened
  on slips and counters before signing: Luxury→Degrowth (FL/LS), Cats→Preserves
  (EI), Cats→Degrowth (LW). There was no physical mat or "Swapped in" mark,
  and no post-signature change or re-sign. **Not assessable** for the physical
  part.
- **Were cards readable across a table?** **Not assessable** (text only). The
  card text was read accurately: players quoted coal 35→15%, protected 10→40%
  and Luxury +20.
- **Did players spot an amendment without being told?** **Yes.**
  - Carme: "It has the same Organic and Cats as yours, so only one card
    differs."
  - Laia checked the returning mat word for word against her own counter.
  - Laia also spotted that Jordi's counter, Organic, worked against FL's own
    land promise.

## Findings

Severity: **high** means fix it, or decide on it, before the human W4.
**med** means fix before W5–W7 or watch in W4. **low** means note it. Code
paths are relative to `~/code/quarter_earth`. Kit paths (`content/`,
`print/`, `run-sheet.md`, `rehearsal.md`, `model-baseline.md`) are under
`references/assembly/`. `<oc>/` means
`~/personal/quarter-earth/files/openspec/changes/assembly-workshop/`.
Transcript references are to `references/assembly/simulation/run-01-raw/transcript.md`.

### 1. Rule ambiguities

**R1. How a promise is judged is unscripted. (high, before W4)**
- Evidence: all 8 participants asked a promise question in the Opening. "Does
  'end below' mean the 2032 value, or any value before it?" (Marta). "Can we
  change the promise after seeing the first-year jump?" (Carme). "Is a promise
  that's a little too easy allowed?" (Oriol).
- The facilitator improvised four rulings (transcript § Opening, "FACILITATOR
  rulings"): judged on the Report value for that year; the warm-up jump counts;
  ambitious is fine; locked once on the wall.
- Change: `references/assembly/run-sheet.md` § Opening and practice, the
  0:15 script. Add the rulings and say which indicators have the warm-up jump
  (temperature and contentedness, not land). Consider one line on the
  mandate's promise box (`content/strings.csv`, `mandate.*` promise keys).

**R2. Four equal tables, two mats and bloc voting produce structural ties. The
second-tie rule can kill a card everyone voted for. (high, decision before W4)**
- Evidence: transcript § Round 2 Decide. The vote was 4–4–0 with every person
  voting their own mat. Degrowth was on both mats. A tied runoff would have
  advanced current policy with all 8 voters having backed Degrowth. Núria:
  "If it ties, Degrowth fails for both."
- In the W4 configuration (4 pairs, all tables signing) an even split is the
  default outcome, not an edge case.
- Change: first `<oc>/specs/assembly-session/spec.md`, Scenario
  "Plurality and deadlock" (the spec wins). Then `run-sheet.md` § 4 Decide.
- Options for a human decision:
  - on a second tie, pass only the cards common to both packages;
  - allow a 30 s open exchange before the runoff;
  - the facilitator names shared cards before the first vote.

**R3. Nothing covers the same card on both mats. (med)**
- Evidence: Carme asked in R1 Build, "Does Solar Push count as a 'different
  card' if another table also has it?"
- In R2 Degrowth sat on both mats, and neither pitch mentioned it (`[ORCH
  NOTE]`, transcript § Round 2 Decide). The vote was really Organic + Cats
  against Preserves, and the room did not see that until the runoff.
- Change: `<oc>/specs/assembly-session/spec.md` § Sponsor and vote, then
  `run-sheet.md` § Operator check (add a row) and § 4 Decide (the
  facilitator announces shared cards).

**R4. Coal conflict: the kit says "fail", the run sheet says "warn". (high,
before W4)**
- The mat checklist line "No 'Cannot combine with' pair" (`print.mat.check.4`
  in `content/strings.csv`, rendered on `print/sheet-09-10-coalition-mats.html`)
  presents it as a failing check.
- The card's "Cannot combine with: … the second does nothing" line and
  `run-sheet.md` § Operator check treat it as a warning. The engine accepts
  both cards.
- Players resolved the contradiction by treating Phase Out Coal as harmless
  filler (see D4).
- Change: settle `run-sheet.md` § Open items "TODO W5: reject or warn"
  (model-baseline open item 4) now. Then align `print.mat.check.4`,
  `card.*.excludes` and the operator row.

**R5. Pair roles don't work at two-person tables. (high, before W4)**
- W4 uses exactly this configuration (rehearsal.md Setup: 8 players in 4
  pairs).
- With Envoy+Timekeeper / Recorder+Spokesperson:
  - when the envoy is out, the recorder is alone and can't sign;
  - the timekeeper is away during Trade, which is exactly when the clock
    matters.
- Evidence:
  - LS could not sign in R1 because Carme was still out when Trade closed
    (transcript § TRADE CLOSES R1).
  - Oriol pitched to Jordi, who was away as FL's envoy (§ Round 2 Trade,
    slot 1).
  - Signing moved to the recorder with no second conversation. Núria
    recommended "don't sign unconfirmed" and lost Organic anyway (notes.md).
- `run-sheet.md` § Rotating roles "Small tables double up" covers only tables
  of three. The mandate roles grid has 4 role columns and Practice + 6 rows.
- Change: `run-sheet.md` § Rotating roles (add a two-person rule: the envoy
  keeps time, and the signer is whoever is home after the envoy returns).
  Also the roles grid in `print/sheet-05…08` / `content/strings.csv`
  `mandate.*` role keys.

**R6. Unscripted procedure questions came up repeatedly. (low)**
- "If one card does nothing, does the package still go to the vote?" (Núria).
- "Can a table that signed nothing still vote?" (Oriol).
- "If the host's table has already signed with someone else, can they still
  sign with me?" (Marta).
- "Must the spare 10 be one card?" (Jordi).
- Change: a facilitator FAQ box in `run-sheet.md` (Round procedure).

### 2. Kit and material gaps

**K1. Card theme labels don't match table names. (high, before W4)**
- Cards print "Society" and "Land & nature". The tables are Living Standards
  and Living World.
- Evidence: Núria's partner pick was "Society" (else "Land & nature"). Pau sent
  Marta to "Land & Nature", meaning LW, and she couldn't map it and went to FL.
  That was a wasted visit slot, and in slot 1 two tables had two visitors and
  two had none (transcript § Round 1 Trade, slot 1, `[ORCH NOTE]`). The
  facilitator had to rule: "'Land & nature' is a card colour, not a table."
- Change: `content/strings.csv` `theme.society.short` / `theme.land.short`
  (and `print/sheet-01-04-deck-a.html` STRINGS.themes). Either rename the
  themes, or print the table codes (FL · EI · LS · LW) on cards and mandate
  sheets so they map one to one.

**K2. "Protected land counts as land in use" appears only on the FL sheet.
(high, before W4)**
- Correction to the brief: I checked `packet/sheet-08` and the source strings.
  The LW mandate sheet does **not** say this. Its Preserves row reads only
  "Expand Nature Preserves takes it to 40%. Pair it with a diet or demand cut"
  (`mandate.living_world.evidence.2.why`). The Preserves card says "leaving
  less land for farms and energy" (`card.f759d87d….effect`). Only FL's
  sheet-05 ("protected land included") and the metric unit string say it.
  Sheets 06 and 07 don't mention it.
- Oriol was right about his own materials: "it took me until the end to see
  that Preserves counts as land in use. The materials never say that." Laia:
  "wasn't on the card, and I only learned it from the facilitator."
- The facilitator's ruling "As your mandate sheets say" was wrong for LW.
- Núria, with the FL sheet, computed the 36→66% jump correctly.
- The Report shown to players labelled land "% of habitable land" without
  "protected land included", although `metric.land_use_fraction.unit` has it.
- Change: `card.f759d87d-69ff-433b-8e3d-f3dbb776543a.effect` and
  `mandate.living_world.evidence.2.why` in `content/strings.csv`, then
  `print/sheet-08-mandate-living-world.html` and `print/sheet-01-04-deck-a.html`.
  Make sure the Report uses the full unit string (W6 Report view; harness
  `simulation/harness/sim_engine.py` output label).

**K3. The prediction slip and wall timeline are laid out for 6 rounds. (med,
before W4)**
- Evidence: Carme labelled her 2032 slip "round 5". Jordi wrote "end of round
  1 (2032)" (transcript § Opening).
- Change: `print/extra-prediction-slips.html` ("By the end of round __" →
  "By year ____"). For the rehearsal, a 2-round wall layout in
  `print/extra-wall-timeline.html`, or a note in `rehearsal.md` Setup to tape
  up only rounds 1–2.

**K4. Cards give no sense of effect size. (med)**
- Only Luxury (+20) and Cats (0.5) carry numbers, and Cats' number is wrong
  (M3).
- Evidence: 5 of 8 debrief PRIVATE lines raise it. "I couldn't tell how large
  each card's effect was" (Oriol). "No one could say how big a card's effect
  would be" (Marta). The facilitator: "I can't tell you in advance how big a
  card's effect will be."
- Notes.md R1: "Three-quarters of animal-calorie demand moves..." gives no
  scale.
- Change: this is a design question about whether hidden magnitudes are
  intended for prediction. Raise it at `<oc>/design.md` decision 5 and
  in the deck copy (`content/strings.csv` `card.*.effect`).

**K5. The coal line on the card works. (low, positive)**
- Pau ("Solar Push already used that coal, so it likely does nothing") and
  Laia spotted the conflict from card text alone, before any operator warning
  (transcript § Round 2 Read + Build).
- Keep `card.*.excludes`, but see R4 and D4 for what players then did with it.

### 3. Dynamics (mapped to the template's revise-guidance cases)

**D1. Two leftover tables were frozen out in R1. (high, before W4)**
*Case: none of the four. Closest is "change the menu or mandates".*
- Only one mat reached the floor. FL and EI paired, leaving LS (Luxury + Solar)
  and LW (Preserves + Veg). Those two were the most polarised and had to pair
  with each other. Oriol: "It's a no from us."
- The vote became A against "=" and A passed 7–1 as "better than nothing".
  Àlex: "doing nothing doesn't protect well-being anyway."
- With 4 tables and 2 mats, one early pairing can lock out half the room.
- Change: `run-sheet.md` § 3 Trade (a mid-Trade facilitator call: "unsigned
  tables, here's who is still free") and a watch item in `rehearsal.md`
  Observer tally ("tables with no mat").

**D2. Card choice converged. (med)**
*Case: "Everybody agrees immediately → change the menu".*
- Solar Push was in 5 of 8 R1 proposals.
- Ban Outdoor Cats was in **all 4 R2 slips** (7 of 8 personal proposals). It is
  the cheap 5-point filler that every package can absorb.
- The negotiation was still lively, so this is about the menu, not the
  mandates.
- Change: deck A composition and costs (`<oc>/design.md` decision 5;
  `model-baseline.md` §5). Watch whether humans do the same before changing
  anything.

**D3. The promise worked as a binding constraint on card choice. (med, design
question)**
- LS's "well-being above 30" made Àlex, the degrowth advocate, drop Degrowth
  for Luxury. "You've persuaded me, and the reason is the promise"
  (transcript § Round 1 Read + Build, Àlex FINAL SLIP).
- FL's land promise made Jordi refuse Preserves in R1 and Núria refuse it in
  R2.
- Promises were written before anyone discussed cards. This produced real
  conflict, which may be good. It also locked positions in before the menu was
  read.
- Change: decide intent in `<oc>/design.md`. If unintended, move the
  promise step after a first read of the deck in `run-sheet.md` § Opening
  and practice.

**D4. Phase Out Coal was used as a do-nothing filler. (med)**
- Laia's slip: "Phase Out Coal (10) is harmless filler for EI." Oriol to EI:
  "for the last 10 points we'd take Phase Out Coal, but … it does nothing."
- Unspent points expire, so a known no-op card is free. The intended trap
  became a degenerate move.
- Change: tied to R4. Either reject the combination, or script the operator
  warning as "you pay 10 for nothing" in `run-sheet.md` § Operator check.

**D5. The sealed runoff produced crossed tie-breakers. (med)**
- Three people switched to break the tie, two to A and one to B (transcript
  § Runoff R2). A slightly different split re-ties and advances current
  policy.
- Sealed simultaneous voting gives would-be tie-breakers no way to coordinate.
- Change: `run-sheet.md` § 4 Decide (runoff procedure). This goes with the
  R2 decision. (Simulated simultaneity exaggerates it; see Artefacts.)

**D6. Envoys rotated and returned. Authority limits held. (low, positive)**
*Case: "one envoy makes every decision" was not observed.*
- Marta brought the Vegetarian decision home because it exceeded Pau's
  authorisation (`[ORCH NOTE]`, transcript § Round 1 Trade, slot 2).
- The opposite risk showed up instead: recorders decided alone (R5).

### 4. Model and report surprises

**M1. The one-year lag makes round 2 unreadable in a 2-round format. (high,
before W4)**
- R2 cards came online in 2031 and got one production year.
- Counterfactuals: Degrowth-only equals nothing passed. Package B differs only
  in land. Only Cats, an instant modifier, showed anything.
- The Report could not show what the room chose. In the 6-round format this
  bites only in round 6.
- Change: `rehearsal.md` (the W4 format). Options:
  - a third, reveal-only advance to 2037 shown after the debrief;
  - a disclosed pre-run branch for round 2;
  - accept it and tell the room before the R2 Reveal.

  Also `run-sheet.md` "Cards take about four years" paragraph.

**M2. The Report has no baseline, so players misattribute background drift to
cards. (high, before W4)**
- **R2:** the +5.9 Gt rise is baseline (no-action 2027→2032: +7.1). Yet:
  - Jordi: "I couldn't tell how Degrowth could raise emissions."
  - Marta: "wouldn't trust Degrowth … again."
  - Carme blamed Organic.
  - Extinction "−2.0" hid Cats' −7.4.
- **R1:** the Report showed emissions −4.8 against 2022. Against no action the
  cards cut about −10.2.
- Jordi's pitch claim that "=" means "sitting at 51.6 gigatonnes for five more
  years" went uncorrected. No-action reaches 57.0 (`[ORCH NOTE]`, transcript
  § Round 1 Decide).
- The no-action run steps up in 2026 and 2031, the card-online years. That is
  why Pau and Carme tied the 2031 jump to their cards.
- Change: `<oc>/design.md` item 4 ("Show change, unit and prior value")
  → add a "with no new cards" column or line. Also
  `<oc>/specs/assembly-session/spec.md` Scenario "Predictions survive the
  vote", and `run-sheet.md` § 5 Reveal step 2.
- For W4 without W6: the operator reads the pre-computed no-action value
  aloud (`fixtures/a_no_action.csv`), disclosed.

**M3. Ban Outdoor Cats' card text understates its effect by about 15×. (high,
before W4)**
- The card says "Lowers extinction pressure by 0.5 on the biodiversity index
  (opening value 90.2)". The measured effect is about −7.4 on the reported
  index at 2032 against nothing passed (notes.md counterfactuals).
- Oriol discounted it: "Is the cats effect (0.5) worth less now?"
- Change: `content/strings.csv` `card.1b98f39e-bb87-4e9f-8c95-565839b72197.effect`
  and its param `cats_extinction_drop`. Also `model-baseline.md` §5 row 8:
  reconcile the `WorldVariable ExtinctionRate −0.5` modifier with the
  reported `extinction_rate` scale before rewording.

**M4. Step-change effects decided the 2032 promises in round 1. (high, before
W4)**
- Land went from 60.7% (2025) to 36.4% (2026) in one step. Extinction went
  from 96.1 (2026) to 64.0 (2027).
- FL's promise (land < 57%) and LW's (extinction < 88) were already met after
  R1. LS's (> 30) was out of reach at 21.6. R2 couldn't change any of them.
- Players expected gradual effects. Marta: "I expect more once they run a full
  five years."
- Change: `run-sheet.md` "Cards take about four years" paragraph. Say
  effects land as a jump in the online year, not a ramp. Also promise
  guidance in § Opening (a promise year). Document the step behaviour in
  `model-baseline.md` §2 "Policy timing".

**M5. Well-being collapse, and the claim that no card could recover it in time.
(high, verified 2026-11-26)**
- Well-being went 30 → 21.6 → 19.7. Most of that is baseline: no-action is 24.3
  at 2027 and 22.0 at 2032. The cards cost about 2.7 and 2.3 respectively.
- LS dropped Luxury in R2 because of the lag. Carme: "One year of effect means
  Luxury can't get us from 21.6 to 30." The facilitator had said "about one
  year of its effect".
- That framing is right for production effects and misleading for step
  effects. Luxury is a one-off "Outlook +20" (`model-baseline.md` §5). In
  fixture `b_comfort_and_technology`, outlook jumps 25.2 → 44.8 in the year
  Luxury comes online.
- **Verified by counterfactual on this history.** The run used R1 as played, with
  R2 = Luxury for All + Organic Transition + Ban Outdoor Cats, which is Jordi's
  FL slip. At 2032 well-being is **39.7** (actual: 19.7), so LS's promise
  (above 30) would have been kept. Emissions were 52.73, identical to the
  actual run, so the extra demand doesn't show within one year. Extinction
  was 61.9 and land use 40.4%, also unchanged. The facilitator's "about one year
  of effect" framing pushed LS off the only card that would have kept its
  promise. That card would also have cost EI nothing measurable by 2032.
- The same framing cost Cats its credibility (M3).
- Change: `run-sheet.md` "Cards take about four years" and the facilitator's
  R2 answer script. State that some effects (outlook, extinction modifiers,
  protected land) arrive in full in the online year.

**M6. Expand Nature Preserves causes a large land-use jump. (med)**
- Package B would have taken land to 68.8% at 2032, against 40.4% for A
  (notes.md counterfactuals). FL's promise would have broken, with no
  extinction gain by 2032.
- The harness note puts it at roughly 63→99% in other states.
- Players who knew the rule (Núria, Marta) used it correctly. The ones who
  didn't (Oriol, Laia) learned it late (K2).
- Change: covered by K2. Also consider adding "+30 pts of land in use" to the
  card's watch-out line (`content/strings.csv` card.f759d87d… keys).

**M7. Baseline emissions rise before cards land. (low)**
- 2024–25 emissions rose 52.9 → 54.3 before the 2026 drop. Marta: "emissions
  rose until 2025 before the cards kicked in, and I hadn't expected that."
- Change: one line in the `run-sheet.md` first-Reveal note (next to the
  warm-up warning).

## Simulation artefacts (don't act on these without human confirmation)

- **Recorder deference.** In 6 of 8 Build exchanges the recorder adopted the
  envoy's package. In R2 all four recorders said some version of "you've
  persuaded me" (notes.md). This is likely LLM agreeableness, and humans
  probably hold out more. It does inflate the "concession" count.
- **Crossed simultaneous concessions.** Because turns were simultaneous,
  partners gave way to each other at the same moment. Marta moved to 48 while
  Pau moved to 45, and Laia adopted Oriol's question while Oriol adopted hers
  (`[ORCH NOTE]`s, transcript § Round 1 Read + Build). The runoff crossing (D5)
  is partly this too.
- **The quiet persona wasn't quiet.** Oriol was written as quiet but gave
  long, decisive replies, so "Written contributions count" was not tested.
- **Agents misremembered their own priorities.** Àlex's R1 vote note said "I
  wanted Luxury plus Solar" when he wanted Degrowth. His debrief says he "gave
  up Luxury … and chose Degrowth", the reverse of what happened (`[ORCH
  NOTE]`s, transcript § Votes R1 and § Debrief LS · Àlex).
- **LLM honesty norms.** Every envoy disclosed the costs of their own package
  unprompted: "I'll be honest", "I won't hide that". Marta announced her
  private priority aloud when she voted, and Carme disclosed the rejected
  Luxury offer to Àlex in full. Expect humans to withhold more and to bargain
  harder.

## Suggested next steps

Fix or decide before the human W4:
1. Script the promise rulings (R1) and the two-person role rule (R5) in
   `run-sheet.md`.
2. Decide the second-tie rule and the same-card-on-both-mats rule in the
   assembly-session spec first (R2, R3), then the run sheet.
3. Settle reject or warn for the coal conflict and align the mat checklist
   (R4, D4).
4. Kit text: map theme labels to table codes (K1). Say that protected land
   counts, on the Preserves card and the LW sheet (K2). Fix the Cats magnitude
   (M3). Put the prediction slip in years (K3).
5. Report: give W4 a disclosed no-action reference line (M2). Pick the W4
   round-2 option: a third advance, a pre-run branch, or an explicit warning
   (M1).
6. Run two cheap counterfactuals: R2 with Luxury (M5) and R1 single-card
   splits (Solar only, Veg only). Use them to check the round-1 explanation
   verdicts and the "one year of effect" script.

What run-02 (Haiku, same personas) should compare against run-01:
- the R2 tie: does bloc voting reproduce a 4–4 split?
- the recorder-adoption rate (6/8 here) and the number of crossed concessions;
- theme-label misroutes (1 here) and promise-rule questions (8/8 here);
- Cats frequency in slips (4/4) and whether Phase Out Coal is used as filler;
- misattribution in R2 cause sentences (about 1.5/8 correct here);
- whether one round still leaves two tables without a mat.

Run it with the same kit and rulings, so that differences reflect the model
and not the materials. Run a second variant only after fixes 1–4, to test
them.
