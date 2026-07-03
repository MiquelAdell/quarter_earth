# Project: Half-Earth Socialism as an assembly planning tool

## Origin

**Assemblea Catalana de Trancisió Ecosocial (acTe)** — Catalan Assembly for
Ecosocial Transition — wants to run an offline, collaborative workshop using
Half-Earth Socialism as a discussion tool: groups debate policy cards, vote,
and feed decisions into the simulation, rather than playing solo. Target
event: acTe's socialising meeting in **October 2026**.

## Why

acTe found that abstract land-use/energy debates ("how much land does one
solar farm need") go in circles. Debating *with numbers* — using a
resource-planning model to show the scale of what a transition actually
requires — is more productive than abstract argument. Half-Earth Socialism
(and similarly Daybreak) already model that scale.

## What acTe wants from the format

- **Social/group play**, not solo: ideas floated include projecting the game
  for everyone, splitting into small groups or "ministries" each owning part
  of the decision space, and playing it "trivia-night" style — screenless,
  with a moderator presenting choices and the group voting.
- **Less reading aloud, less dialogue.** Not everyone enjoys narrative text;
  the group wants to get to the planning loop faster.
- **Lower time cost per unit of consequence** — a full playthrough is too
  long for a workshop slot.
- Track a few legible high-level resources (biodiversity pressure, energy
  output, calories produced, CO2/temperature) and let policy choices show
  their effect on those directly.

## Upstream maintainer's proposed approach

Two pieces the upstream maintainer is willing to spend some time on (no
committed timeline as of the last exchange):

1. **A stripped-down "model-only" mode** — remove story/dialogue/game
   framing, keep just the core loop: pick projects/policies for a planning
   period → see the effects → repeat. This is the "planning tool" version
   Miquel asked for.
2. **An encyclopedia page** listing all projects/events and their details,
   browsable on participants' phones — so the projected game stays visual
   and people don't need things read aloud to know what a card does.

Open question flagged upstream: the game's core resource constraint is
**political capital**, which is a game-design device and may not map
cleanly onto a "pure planning" framing. Simplifying/removing it is
non-trivial and not yet resolved.

## Interim option: debug/desktop flags

Independent of any code changes, the game already ships debug options
(desktop only, see `game/src/debug.rs` and readme) that can approximate a
lot of what acTe wants without new development:
- skip dialogue
- skip world view, jump straight to the 5-year report
- skip tutorial
- start with everything unlocked
- disable parliament/NPCs

This can run on desktop with difficulty tuned via a custom `.world` file.
It's the fastest path to a usable workshop build before October, independent
of whatever upstream builds.

## Status (as of Jun 2026)

- Upstream is open to building the model-only mode + encyclopedia but has no
  free time commitment yet.
- October 2026 socialising meeting confirmed as the target; plan in the
  meantime is to experiment with the existing debug options.
- No code work has started in this repo yet toward the acTe use case.

## Open threads

- Decide how to handle "pick 1a AND 1b, not 1a and 2b" style grouped/mutually
  exclusive policy choices for a moderator-led session — current game logic
  allows picking anything affordable, no grouping/exclusivity UI.
- Political capital's role in a pure-planning mode is unresolved.
- No confirmed timeline from Francis for the model-only build; Miquel may
  need to prototype independently using debug flags + a custom world file.
