# Workshop mode — spec (v1)

Status: **v1 — decided**. Incorporates round-1 (`open-questions.md`) and
round-2 (`open-questions-round2.md`) answers, both 2026-07-02. All effect
magnitudes, costs and thresholds remain subject to playtesting (M1).

## 1. Summary

A browser-playable, moderator-projected version of Half-Earth Socialism for a
~1-hour facilitated group session. One decision type (policy cards), a simple
per-cycle budget, four headline metrics, no story content, no parliament.
This is our own fork; no upstream-compatibility constraint (Q1).

## 2. Session shape

- **6 planning cycles** of 5 in-game years: 2022 → 2052 (Q5).
- Per cycle: planning screen (group debates, moderator clicks) → fast
  world-simulation step → report screen (consequences) → next cycle.
- **Win kept, no mid-game loss** (Q5, R2-5a): win = the existing three
  thresholds (emissions ≤ 0, temp ≤ +1 °C, extinction ≤ 20) — re-balanced for
  the 30-year horizon. The session always reaches the year-2052 evaluation;
  the ending screen shows win/fail there. No ousting, no early collapse end.
- Ending screen doubles as the debrief: final state of the four metrics.
- **Format** (R2-7): one shared projected screen, the moderator clicks;
  groups debate and vote verbally. No per-participant devices in v1 —
  prioritize readability-from-distance over click-flow.

## 3. Decisions: policy cards only

- The **only** player decision is passing (or repealing) policy cards (Q4:
  process/allocation screen removed; research and initiative mechanics
  removed from the player-facing loop).
- Card set: **~20 curated cards** (R2-2a: cut list applied), see
  `policy-cards-proposal.md` (Q9) — mix of existing policies, converted
  initiatives/research, and new mix-shift cards.
- Mix-shift cards require a new engine effect `Effect::ChangeMixShare(Id, isize)`
  (lever D1). Repealing such a card reverses the shift.
- Policies take effect within the cycle they're passed (existing policy
  behavior), so consequences are visible in the next report.
- Mutually exclusive cards use the existing `project_lockers` mechanism.
- **Phasing via prerequisites, not scripting** (R2-2b): all cards exist from
  the start; a handful require an enabler card passed in an earlier cycle
  (existing unlock mechanism). Different groups unlock different branches, so
  later cycles differ per group. The moderator may additionally present one
  theme at a time — facilitation, not code.

## 4. Economy: fixed PC budget per cycle

- Political capital becomes a **flat allowance: 30 PC per planning cycle**
  (Q2 = option b). Not earned from outcomes; **unspent PC expires** at the
  end of the cycle (R2-4) — every cycle's debate is self-contained.
- Card costs 5/10/15 by impact tier (see proposal); confirmed as the
  playtesting starting point (R2-4).
- All PC-earning machinery (report PC awards, honeymoon, request bounties,
  completed-project bonuses) is disabled/hidden in workshop mode.
- Losing by "ousted at 0 PC" no longer applies; PC is a budget, not health.

## 5. Removed content (Q6, Q7)

- **All story/dialogue events, world events, disasters and icon disasters.**
  The world phase becomes a pure simulation tick with a brief year animation
  (or none at all — straight to report).
- **Parliament and NPCs entirely**: no tab, no seats, no required majorities,
  no ally cost modifiers, no NPC relationship side-effects, no requests.
- Tutorial, intro, narrative interstitials.

## 6. Metrics & displays (Q8)

Headline (always visible, and the spine of the report):
1. CO2 emissions / temperature anomaly
2. Biodiversity (extinction rate)
3. Energy produced vs demand
4. Calories produced vs demand

Plus **land use** as a persistent gauge (used/protected/free) — confirmed
(R2-3). It is the constraint that replaces the allocation screen and drives
the Half-Earth tension; already computed today, we promote it, not build it.
Water stays warning-only. Contentedness stays
as an internal variable (some cards trade against it) but is shown only in
the report, not as a headline metric.

## 7. Platform & language (Q10, Q9)

- **Web build**, projected by the moderator. Custom `.world` loads via the
  existing file picker (works on web).
- Language: **Spanish for v1** (R2-6; `game/translations/es.csv` is
  complete). A Catalan CSV is a possible later contribution — the ~20-card
  world keeps that surface small — but not v1 work.
- Encyclopedia: deferred to v2 (Q10).

## 8. Out of scope (v1)

- Encyclopedia on phones (C9).
- Multi-group "ministries" input aggregation (D4).
- Upstream compatibility.
- Native/Steam builds of workshop mode.

## 9. Implementation shape (indicative, refined at planning time)

1. **Engine**: `Effect::ChangeMixShare` (D1); game length configurable (D2).
2. **Content**: workshop `.world` — curated cards, everything else
   deleted/locked; rebalanced costs and win thresholds (B1/B2/B4).
3. **Game**: workshop mode flag (C1); policies-only plan screen with budget
   display (C2+C6); fast/no world phase (C4/A3); metric-centric report (C5);
   6-cycle end (C3); big-screen sizing (C8).

## 10. Milestones (R2-1: all three, executed in order)

- **M1 — no-code prototype.** Today's game + debug flags + a custom `.world`
  built in the editor. Goal: playtest the *content* (card set, costs, pacing,
  thresholds) with a real group before hardening into code. Known gaps vs
  spec accepted (research tabs visible, PC-as-earned, etc.).
- **M2 — the real workshop mode.** The code work in §9. The October build.
- **M3 — polish.** Moderator choice-groups UI, big-screen layout,
  encyclopedia, nicer debrief.

No open items remain; see `implementation-plan.md` for the work breakdown.
