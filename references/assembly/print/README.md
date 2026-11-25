# Assembly print kit

Print-ready A4 templates for the paper side of the assembly: cards, mandates, mats, slips, voting cards and the wall timeline. The rules that use them are in [`../run-sheet.md`](../run-sheet.md). The acceptance contract is the `assembly-kit` and `assembly-session` specs in the OpenSpec change `assembly-workshop`.

Each template is a self-contained HTML file. To print one:

1. Open it in a browser and choose Print.
2. Set the paper to **A4**, the scale to **100% / actual size** and the margins to **Default**. Turn **headers and footers off**.
3. Set the orientation shown in the table below. Most browsers take it from the file.
4. Print **single-sided**.

## The ten-sheet opening kit

This kit meets the spec scenario "Prepare four tables": ten sheets give four identical eight-card decks, four distinct mandate sheets and two reusable coalition mats.

| Sheet # | File | Contents | Orientation | Pages in file | Copies |
|---|---|---|---|---|---|
| 1 | `sheet-01-04-deck-a.html` | Deck A, all 8 cards for table 1 | Landscape | 1 | ×4, one sheet per deck |
| 2 | ″ | Deck A for table 2 | ″ | ″ | ″ |
| 3 | ″ | Deck A for table 3 | ″ | ″ | ″ |
| 4 | ″ | Deck A for table 4 | ″ | ″ | ″ |
| 5 | `sheet-05-mandate-food-land.html` | Mandate: Food and Land (FL) | Portrait | 1 | ×1 |
| 6 | `sheet-06-mandate-energy-industry.html` | Mandate: Energy and Industry (EI) | Portrait | 1 | ×1 |
| 7 | `sheet-07-mandate-living-standards.html` | Mandate: Living Standards (LS) | Portrait | 1 | ×1 |
| 8 | `sheet-08-mandate-living-world.html` | Mandate: Living World (LW) | Portrait | 1 | ×1 |
| 9 | `sheet-09-10-coalition-mats.html` page 1 | Coalition mat **A** | Landscape | 2 | ×1 (prints both mats) |
| 10 | ″ page 2 | Coalition mat **B** | ″ | ″ | ″ |

All four decks are identical, so a card coming back from a mat can go to any table. Every mat page has its letter printed on it, so no one needs to label the mats at setup, and the letters match the voting cards.

### What each sheet holds

- **Card** (66 × 90 mm, cut on the dashed line). Each card shows:
  - the theme as a shape plus a word (▲ Energy, ● Food, ■ Land & nature, ◆ Industry & transport, ✚ Society)
  - a stable card ID and the name
  - the cost as a large number plus hatched **pips**, where 1 pip = 5 points
  - the effect, with a "Requires:" line and a "Cannot combine with:" line
  - the deck letter and its position, such as "Deck A · 3/8"
- **Mandate**:
  - what the table answers for, and the question it can bring to another table
  - three evidence rows from the opening world
  - a **public promise** field: indicator, ↑/↓ tick, target and year, plus a "copied to the wall" tick
  - a **question the model cannot answer** field
  - a rotating-roles grid for practice and rounds 1–6, with a "we signed mat A · B · none" column
  - a debrief box
- **Coalition mat**:
  - the mat letter badge and a round field
  - **three card slots**, each the card size plus 1 mm per side
  - under each slot: cost, **"Swapped in (amendment)" tick, "replaces ___", "proposed by table ___"**
  - a budget column: card 1 + 2 + 3 = total, out of 30. The **six-box pip gauge** (5…30) means a seventh pip is visibly over budget
  - six pre-signing checks: 1–3 cards; no card twice; requirements in force; no excluded pair; total 30 or less; two different tables
  - **two sponsor blocks** (table, signature, "our envoy came back before we signed")
  - **two predictions** (indicator, ↑ box, ↓ box), recorded before the vote

**Reuse.** Put each mat in an A4 landscape sheet protector and write on it with dry-wipe marker. Pencil works if you have no sleeves. Wipe the mats after each Reveal. The durable record is the operator's ballot entry plus each table's package slip.

## Outside the ten-sheet count

The spec counts only decks, mandates and mats. Everything below scales with the number of players or rounds, or could be replaced with blank paper. That is why none of it is in the ten.

| File | Contents | Orientation | Pages in file | Copies for 24 players | Why outside the ten |
|---|---|---|---|---|---|
| `optional-deck-b.html` | Optional second deck, cards 9–16 | Landscape | 1 | ×4 (**+4 sheets**, as the spec says) | Optional. Handed out after the break only if the rehearsal shows the room can manage the extra choices (design.md decision 5). |
| `extra-package-slips.html` | Four table draft slips per page: 1–3 cards, costs, total, two predictions, what happened to the offer | Portrait | 1 | ×7 (practice + 6 rounds × 4 tables = 28 slips) | One per table per round, and they are the durable record. Count depends on rounds. |
| `extra-voting-cards.html` | Nine cards per page, i.e. three voters × (A, B, = carry current policy forward) | Portrait | 1 | ×8 (players ÷ 3, rounded up) | One set per participant, so the count depends on attendance. |
| `extra-prediction-slips.html` | Eight personal prediction slips per page | Portrait | 1 | ×3 (players ÷ 8, rounded up) | One per participant. |
| `extra-wall-timeline.html` | One page per round 1–6 plus a replay page: result, tally, runoff, cards, sponsors, forecast vs actual, first response, one-sentence cause | Landscape | 7 | ×1 | A wall display, not table material. It can also be drawn on butcher paper. |

A typical print for 24 players is 10 + 7 + 8 + 3 + 7 = **35 sheets**, or 39 with deck B. For 12 players it is 10 + 7 + 4 + 2 + 7 = 30.

## Reading the kit without colour or a phone

- **Cost.** Read it from a large number and from pips, which works for players who aren't confident with numbers. The mat gauge has six boxes, one per pip, so going over budget is something you can see and count.
- **Theme.** Each theme has a different shape and a word.
- **Amendment.** A swapped-in card is shown by a written tick and the "replaces / proposed by" lines on the slot. Any change after a signature cancels both signatures.
- **Package and vote.**
  - A is a white letter on a solid black square.
  - B is a black letter in a dotted double-ruled square.
  - "=" means carry current policy forward and has a striped double-ruled square.
  - Every option also has its word printed underneath. The same badges appear on the mats.
- **Direction.** Every direction has an arrow, a tick box and the word "up" or "down".
- **Ink.** Everything prints in black ink only. Hatching and patterns survive greyscale and photocopying.

## Localisation hooks (for W3/W7)

- Each file has one `STRINGS` block near the top. The mandates also have a `TABLE` block and the decks a `CARDS` block. All visible text is in those blocks, and the renderer below them has no copy of its own. Swap the blocks for each language. Each print run uses one language only.
- **Placeholders waiting for W1** (`references/assembly/model-baseline.md`):
  - `{{CARD_n_ID|NAME|THEME|COST|EFFECT|REQUIRES|EXCLUDES}}` for n = 1–8 (deck A) and 9–16 (deck B). `THEME` is one of `energy|food|land|industry|society`. `COST` must be a multiple of 5 so the pips render.
  - `{{FL|EI|LS|LW_EV_n_LABEL|VALUE|UNIT|NOTE}}`, the three opening-evidence rows per mandate.
  - `{{ROUND_1..6_YEARS}}` and `{{REPLAY_YEARS}}` on the wall timeline. Whether the last cycle ends in 2052 or 2055 is still open.
- **Card and slot size.** The CSS variables `--card-w: 66mm; --card-h: 90mm` are duplicated in the deck files and the mat file. If you change them in one, change them in all.
- **Text headroom.** `check-fit.py` prints every file to PDF and reports the page count and size. It then runs a stress pass in which:
  - card fields are filled with long English sample text
  - evidence rows get sample values
  - every visible string is stretched by 35%
  
  It fails if any clipping box (card, slip, panel or the page itself) hides content.

  ```bash
  python3 references/assembly/print/check-fit.py --out /tmp/qe-proof        # default stretch 1.35
  python3 references/assembly/print/check-fit.py --stretch 1.6 --out /tmp/qe-proof16
  ```

  W7 should run it again on each real language edition.

## Proof status (2026-11-25, headless Chrome)

| File | Pages | Size | As-is | Stress ×1.35 | Stress ×1.6 |
|---|---|---|---|---|---|
| sheet-01-04-deck-a | 1 | A4 landscape | OK | OK | overflows (card effect box) |
| sheet-05…08 mandates | 1 each | A4 portrait | OK | OK | overflows (page) |
| sheet-09-10-coalition-mats | 2 (= sheets 9 and 10) | A4 landscape | OK | OK | OK |
| optional-deck-b | 1 | A4 landscape | OK | OK | overflows (card effect box) |
| extra-package-slips | 1 | A4 portrait | OK | OK | overflows (slip) |
| extra-voting-cards | 1 | A4 portrait | OK | OK | OK |
| extra-prediction-slips | 1 | A4 portrait | OK | OK | OK |
| extra-wall-timeline | 7 (rounds 1–6 + replay) | A4 landscape | OK | OK | OK |

Every sheet has about 35% headroom over long English copy. Card effect text should stay at about **160 English characters** or fewer (the stress sample is 157).

## Still pending

- **W1 data.** The eight verified cards (IDs, costs, effects, requires and excludes), the opening evidence values and the round years. Until these arrive the cards print as placeholders, so **do not print for players yet**.
- **Human test: this is the real acceptance for W2.** Four tables and two mats need to carry out a physical amendment and vote: substitute a card on a mat, re-sign, check the budget against the gauge and vote with the cards. It has not been run yet. W4's rehearsal is where it should happen. Things to watch:
  - whether 66 × 90 mm cards are readable across a table
  - whether sleeves and dry-wipe markers hold up over six rounds
  - whether players recognise an amendment without being told
- **Physical proof.** Print one A4 page on the target printer at 100% and confirm that the cut lines and card slots line up. Headless PDF proves the layout, not the printer's margins.
