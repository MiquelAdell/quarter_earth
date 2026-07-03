# Open questions — round 2

Follow-ups from your round-1 answers. Answer inline under **A:** as before.
Everything decided in round 1 is already folded into `spec.md` (draft v0) and
`policy-cards-proposal.md`.

## R2-1. Milestones explained (your Q11 "explain more")

The M1/M2/M3 split was about *how much code* each step needs:

- **M1 — no-code prototype.** The game as it exists today, launched with
  debug flags (`?debug=SKIP_TUTORIAL,SKIP_EVENTS,...` works on the web build)
  plus a custom `.world` file made in the editor (fewer cards, rebalanced
  numbers, loaded via the game's own "load world" picker — works in browser).
  Playable in days. Limitations: the UI still shows the research/initiative
  tabs, the process screen, PC-as-earned — it approximates the workshop but
  doesn't match the spec.
- **M2 — the real workshop mode.** The code work in `spec.md` §9: new engine
  effect, policies-only screen, per-cycle budget, 6 cycles, metric report.
  This is the actual October build.
- **M3 — polish.** Moderator choice-groups UI, big-screen layout,
  encyclopedia, nicer debrief.

The question was: do we bother with M1, or go straight to M2? My
recommendation: **do M1 anyway** — it's cheap, and it lets you playtest the
*content* (card set, costs, 6-cycle pacing, win thresholds) with a real group
before we harden it into code. Content risk is bigger than code risk here.

**A:** I'm ok doing all steps but I'll let you work on them and, once we have a clear plan, you can work from M1 to M3 on your own.

## R2-2. Card count and phasing mechanism (your Q3 follow-up)

Your concern: enough choice for different solutions to emerge, without
overwhelm, phased over the session without us scripting each cycle.

Proposal:
- **All ~20–27 cards** exist from the start, grouped into 5 themes, but the
  binding constraint is the **30 PC/cycle budget** (~2–3 cards per cycle).
  That is exactly your "cost of discussing A and not B", made mechanical.
- **Phasing emerges from prerequisites, not from a script**: a handful of
  cards require an enabler passed in an earlier cycle (existing
  unlock mechanism — e.g. Cellular Meat needs an R&D-ish enabler; SRM needs a
  first geoengineering step; Mass Electrification pays off only after clean
  electricity cards). Different groups unlock different branches, so later
  cycles differ per group — controlled pacing without us deciding anything.
- The moderator can additionally choose to present one theme at a time, but
  that's facilitation, not code.

Questions: (a) OK with ~20 vs ~27 cards (see cut list at the end of
`policy-cards-proposal.md`)? (b) OK with prerequisite-based phasing?

**A:** ok with both

## R2-3. Land use as fifth gauge (your Q8 follow-up)

You asked whether something must be added for balance given the allocation
page is gone. My recommendation: keep the four acTe metrics as headlines and
add a **land gauge** (used / protected / free) permanently on the planning
screen. Land is the resource the Half-Earth thesis is about, several cards
trade land directly (Nature Preserves vs Solar Push vs Organic Transition),
and it's already computed and displayed today (resource bars in the plan
overview) — we'd promote it, not build it. Water stays as a warning-only
indicator. Confirm?

**A:** yes

## R2-4. PC budget details

- Does unspent PC **carry over** to the next cycle (lets groups save up for a
  15-PC card; adds a small strategy layer) or **expire** (simpler, keeps every
  cycle's debate self-contained)? My lean: expire.
- Confirm 30 PC/cycle with 5/10/15 card costs as the starting point for
  playtesting.

**A:** expire. And yes.

## R2-5. Losing

With PC-as-health gone, what does *losing* mean before year 30? Options:
(a) no mid-game loss — only the year-2052 win/fail evaluation (simplest,
recommended for a workshop: the group always reaches the debrief);
(b) keep a catastrophic-collapse loss (runaway temperature / total production
collapse ends the game early — dramatic, but can end a workshop at minute 40).

**A:** ok with a.

## R2-6. Language

There is **no Catalan translation**; Spanish exists and is complete. Options:
(a) run the workshop in Spanish (zero work);
(b) add a Catalan CSV for the workshop build — with only ~20 cards + core UI
strings the surface is small; acTe volunteers could translate from the
Spanish file (nice contribution back to the project too).
Which one (or both: ES for playtests, CA for the event)?

**A:** let's go with spanish for the first version

## R2-7. Who plays the moderator screen?

Assumption baked into the spec: **one shared screen, moderator clicks**,
groups debate and vote verbally. No per-participant devices in v1 (the
phone encyclopedia is v2). Confirm this is the format — it affects how much
we invest in click-flow vs readability-from-distance.

**A:** yes
