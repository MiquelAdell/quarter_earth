# Open questions — answer inline

Answer by editing under each **A:**. Lever codes refer to
`simplification-levers.md`. Once answered, the next step is drafting
`spec.md` from these decisions.

## 1. Fork vs upstream

This repo is a fork; upstream (Francis) may eventually build a model-only
mode. Do we build the workshop mode here as our own thing (freedom, possible
divergence), or keep changes small/upstreamable (constrains B2 content
rewrites and C6-style reframings)?

**A:** This code is our own thing and it will not need to be merged upstream at any point.

## 2. Political capital

The brief flags this as unresolved. Options:
- **(a) Remove as constraint** — policies free, budget = "pick N policies per
  cycle" (lever C6). Pure planning; discussion centers on physical tradeoffs.
- **(b) Keep as a simple budget** — "you have 30 PC per cycle, cards cost
  5–15". Forces prioritization by scarcity, familiar board-game feel.
- **(c) Keep as-is** (earned from outcomes) — rewards good play but adds a
  feedback loop players must learn.

My read: (a) or (b); (b) is cheaper to build (it's mostly rebalancing, B4).

**A:** b

## 3. Decision budget per cycle

However Q2 lands: how many decisions should a group debate per cycle? (The
1-hour budget suggests ~6 cycles × ~3 choices.) And are choices presented as
an open list (players browse) or as moderator-led choice groups, one at a
time, trivia style (lever C7)?

**A:** How can give enough choice to the players that different solutions can emerge (this is one of the main objectives of the exercice. Investigate different solutiosn and their drawbacks/strengths) but without overwhelming the players? Can we phase the decisions so we offer them in a controled way? (but without deciding ourselves what to do at each planning session)
Maybe we can model "the cost of discussing A and not B" as a total number of policies we can execute each cycle. See also answer to 9.

## 4. Process mixes

Confirm: the process/allocation screen disappears entirely, and the only way
to change energy/food mixes is via policy cards ("Eolic thrust", "Phase out
coal", "Plant-based diets")? That means shipping engine lever D1. Or should a
simplified allocation screen survive as a second decision type?

**A:** let's remove it

## 5. Game length and win condition

- How many in-game years / cycles? (Default: 60 y / 12 cycles. Strawman: 30 y / 6 cycles.)
- Keep win/lose endings at all, or end with a scored debrief ("where did the
  world land on the 4 metrics") — arguably better for a discussion workshop?

**A:** I like having a win/loose condition. 6 cicles is ok.

## 6. Events and disasters

- Story/dialogue events: cut all (A1) or keep a curated handful of
  consequence events (B3)?
- Icon disasters during the world phase: keep (visible consequences, cost
  ~seconds) or cut?

**A:** cut all dialogue, events, disasters, and icon disasters.

## 7. Parliament / NPCs

Cut entirely (A5 + hide tab), or is there workshop value in "factions react
to your plan" as discussion fuel? (Cutting is much simpler; NPC hooks touch
costs, mixes, and the report.)

**A:** cut entirely.

## 8. The four metrics

acTe named: biodiversity pressure, energy output, calories produced,
CO2/temperature. Confirm this exact set as the headline HUD/report metrics?
Anything to add (land use? contentedness?) or is 4 the hard cap?

**A:** do not remove any of the metrics. Do we need to add something for game balance? Maybe we need to add land use to account for having the allocation page.

## 9. Content curation

Who curates the ~20 policy cards and their numbers — us here (I can draft a
candidate list from the existing 123 projects for review), or acTe/Francis?
Which language for the workshop build (game ships with Catalan/Spanish?
translations exist for 8 languages — need to check which)?

**A:** provide a first proposal with an explanation why. We want to foster debate so high impact policies are important but one of the things that happen is that we tend to spend a lot of time discussing low impact policies (i.e. banning private jets) and I want that to also be a part of the debate.

What languages do we have? the target language is CA but we can work with ES.

## 10. Platform for the workshop

Native desktop projected (full debug flags, editor available) vs web build
(easier distribution, no `DEBUG_STATE`)? The encyclopedia-on-phones idea
(C9) — wanted for v1?

**A:** Browser. Let's keep the encyclopedia for v2.

## 11. Scope of first milestone

Suggest: **M1** = A+B fallback (flags + custom world, no code) playtestable
this year; **M2** = D1 + C1/C2/C6 (policies-only workshop mode); **M3** =
C5/C7/C8/C9 polish. Agree, or go straight for M2?

**A:** I don't understand that, explain it a bit more please.
