# Workshop baseline UX audit

Date: 2026-08-29  
Commit audited: `0395f0d67d5727d9b25fd5ff130a684eace0bf8e`  
URL: `http://127.0.0.1:8080/?workshop=1`

## Method and evidence boundary

I requested live Chrome viewports of 1440 x 900 and 1920 x 1080. The landing
captures have those exact pixel dimensions. On entering the game, this build
returned a fixed 1280 x 831 canvas capture instead of retaining the requested
browser-capture dimensions; that behaviour is itself relevant to projected
layout verification. The screenshots below are unedited browser captures. At each viewport, the
landing screen visibly identifies the game generically and says `Default
World`; it does not identify a workshop.

At 1440 x 900 I selected the visible, affordable `Universal Family Planning`
card. The political-capital display changed from 30/30 to 20/30 and the card
received a small `PASSING` tag. This is the only selected-card state claimed
in this audit.

The required first-report capture is **not available**. The current entry
path opened a custom-world chooser when I attempted to start the actual new
game; the archived M1 fixture could not be supplied because the connected
browser rejected the local file-selection action (`Not allowed`). I did not
substitute a source-code inference or fabricate a report screenshot.

## Captures

- [Landing, 1440 x 900](screenshots/baseline/workshop-landing-1440x900.png)
- [Landing, 1920 x 1080](screenshots/baseline/workshop-landing-1920x1080.png)
- [Initial plan with visible locked cards (live game canvas: 1280 x 831)](screenshots/baseline/initial-plan-1440x900.png)
- [Affordable selected card (live game canvas: 1280 x 831)](screenshots/baseline/selected-affordable-card-1440x900.png)
- [Self-contained workshop plan after S4 (1440 x 900)](screenshots/baseline/workshop-initial-plan-self-contained-1440x900.png)
- [First self-contained workshop report (1440 x 900)](screenshots/baseline/workshop-first-report-1440x900.png)

## Findings

| Severity | Visible evidence | Affected decided requirement | Concrete recommendation |
|---|---|---|---|
| Blocker | Both landing captures show the generic Half-Earth Socialism title and `Default World`; neither shows “Workshop mode,” the six-cycle premise, budget expiry, or final goals. Attempting the actual start exposes a custom-world chooser. | Self-contained `?workshop=1` entry and moderator-facing premise (S4; decision register). | In workshop mode, keep the menu but replace the normal/default picker with the embedded workshop world. Add a concise visible workshop summary and prevent the file chooser from being the route to a workshop session. |
| High | The initial plan shows legacy content and headings such as `PROTECTION`, `Cap and Trade Program for Children`, and `Universal Family Planning`, rather than the accepted 21-card deck and five workshop themes. | Curated 21-card workshop world and five meaningful themes (S2, S5). | Load the embedded release world at workshop entry and map its accepted IDs into the five presentation themes; do not expose normal-world content in workshop mode. |
| High | Locked cards visibly show only a padlock and the word `Locked`. The initial-plan capture gives no readable prerequisite or timing explanation. | Actionable locks, including prerequisite and next-cycle timing (S5). | Add explicit lock text naming the prerequisite(s), OR rule where relevant, and “available next cycle” timing; retain the icon as supplementary status. |
| High | The live plan needs a scrolling card surface. The always-visible budget is readable, but it is the only instruction: it does not state cycle count, year range, unused-PC expiry, or how to select/undo. The selected state is a tiny `PASSING` label. | Projected moderator UX and self-explanatory planning choices (S5). | Reserve a large, persistent workshop header for cycle/year, 30-PC expiry, selection/undo instruction, and a textual selection summary. Use a larger, persistent selected-state treatment than the current tag. |
| Medium | Locked cards are substantially dimmed while retaining dark text and densely packed icons; their effect copy is difficult to read at projected distance in the 1440 x 900 capture. | Big-screen legibility and non-colour-only states (decision register; S5). | Increase card and state-label contrast/size, give locked cards a high-contrast text panel, and reduce simultaneous card density so at least one complete card row is comfortably readable at 1920 x 1080. |
| Medium | Although the requested landing captures are 1440 x 900 and 1920 x 1080, entering the game yielded fixed 1280 x 831 plan captures. | Responsive projected-screen validation (S1; S5). | Verify and document the game-canvas sizing path after S4; rerun plan/report captures at true 1440 x 900 and 1920 x 1080 before accepting projected legibility. |
| Blocker | No first-report state could be reached or captured from the live baseline without a local world load, and that fixture selection was blocked. | S1 evidence requirement; report/debrief projection requirements (S6). | After self-contained embedded-world entry is implemented, rerun this audit and capture the first live report before specifying report typography/layout from visual evidence. |

## Acceptance result

**BLOCKED / partial.** Landing, initial-plan/locked-card, and affordable
selected-card evidence exists. S1 cannot pass until a self-contained workshop
session permits the first report to be reached and captured in the browser.
The visible entry and deck defects above should be addressed by S2/S4 before
the follow-up audit; S5/S6 should use only the visual findings that are
actually evidenced here.

## S4 follow-up — 2026-08-29

I reloaded the same URL against the current uncommitted S2/S4 working tree
(Git `HEAD` remains `0395f0d67d5727d9b25fd5ff130a684eace0bf8e`). The landing
now visibly says `Workshop mode`, states the six-cycle premise, 30-PC expiry,
and 2052 assessment, and has no default-world label or file picker. New Game
opened the embedded workshop deck directly; visible cards include `Nuclear
Expansion`, `Phase Out Coal`, `Solar Push`, and `Wind Push`.

To reach the required first report, I scrolled the live plan to the red
`Ready` control and attempted: direct clicks at its rendered centre and
keyboard activation after tabbing from the plan surface. In each case the
screen stayed on the same 2022 plan with `30/30 PC this cycle` and the Ready
button still visible. No world step or report appeared. The interaction did
not produce an error dialog or a report, so no report PNG was created.

| Severity | Visible evidence | Affected decided requirement | Concrete recommendation |
|---|---|---|---|
| High | The follow-up plan has a visibly enabled red `Ready` button, but it did not start a world step after direct pointer or keyboard activation; the 2022 plan remained unchanged. | Planning-to-report lifecycle and first-report evidence (S4, S6, S1). | Trace/fix the Ready action in the live web build, then rerun only the one-cycle report capture at 1440 x 900 and 1920 x 1080. |

### Prior S1 acceptance result

**BLOCKED / partial.** The original entry/file-picker blocker is resolved and
the current landing plus embedded deck are directly evidenced. S1 still cannot
meet its mandatory first-report screenshot/visual-audit acceptance because the
enabled Ready control did not advance the live session. This record contains
all valid evidence obtained; it makes no source-only claim about the report.

## S1 completion follow-up — 2026-09-01

I reran the self-contained workshop in a Playwright-controlled Chromium page
at 1440 x 900. A real canvas click on New Game opened the 2022 workshop plan,
and a real click on Ready completed the 2022 to 2027 simulation and displayed
the first report. The browser console contained zero errors after the
transition. The report capture above has been verified as exactly 1440 x 900.

The report visibly separates emissions, temperature, extinction rate, energy
supplied, and calories supplied and shows start/end values. It is compact in
the centre of a largely empty projected screen, however, and does not show the
cycle number, metric targets, choices made during the cycle, or explanatory
improvement/worsening labels. Those are direct inputs to S6 rather than
reasons to keep the baseline audit blocked.

| Severity | Visible evidence | Affected decided requirement | Concrete recommendation |
|---|---|---|---|
| High | The first report uses only a small central panel despite the 1440 x 900 viewport; its rows and labels occupy a small fraction of the projected surface. | Projected moderator legibility (S6). | Enlarge the report panel and metric typography for projection rather than preserving the normal-mode 360-pixel presentation. |
| High | The report shows 2022 to 2027 values and arrows but no cycle number, targets, or textual indication of whether each change is desirable. | Interpretable cycle debrief (S6). | Add cycle/year context, target values, and explicit improvement/worsening wording for every headline metric. |
| High | No passed/repealed policy summary appears on the report. | Discussion must connect the cycle's choices to the observed world change without claiming causality (S6). | Add separate “Choices this cycle” and “World change” sections using the recorded plan history. |

### Final S1 acceptance result

**DONE.** The audit now contains live evidence for the landing, initial plan,
locked and selected card states, and the first report, with findings grounded
in the visible browser output. S5 and S6 can use this audit as their visual
baseline.
