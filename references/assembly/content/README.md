# Assembly content pack (W3)

This folder holds every piece of text the assembly shows, prints or reads aloud, in English, Spanish (Spain) and Catalan (central). Paper and screen should both be drawn from it. Design decision 2 of the `assembly-workshop` change governs it:

- IDs are stable across languages, and copy is resolved at display time.
- Translations are whole phrases, not fragments.
- Numbers are formatted per locale.
- Each print run uses one language.

> **Status: draft, machine-checked, not yet reviewed by fluent speakers.** All Spanish and Catalan text was written without a native-speaker review. The checks below prove consistency (keys, IDs, units, prices, placeholders, numbers, template coverage, line fit). They do not prove the language is natural or politically neutral. Before printing for players, a fluent reviewer must go through each language and record changes in the corrections log below. Catalan has the most open choices, because the game has no Catalan translation to borrow from.

## Files

| File | What it is |
|---|---|
| `strings.csv` | The catalog: 390 keys × en/es/ca. **The source of truth.** Edit it directly. |
| `glossary.md` | The term table, register rules and locale number formatting. |
| `run-sheet.es.md`, `run-sheet.ca.md` | Translations of `../run-sheet.md` (the facilitator guide). |
| `check.py` | The consistency check, the in-game gap extractor and the proof renderer. |
| `ingame-gaps.csv` | Generated. Game strings that an assembly session can show, with the existing es.csv translation and an empty Catalan column. |

## Catalog format

`strings.csv` is UTF-8 RFC 4180 CSV, with the columns `key,en,es,ca,max_chars,sources,params,notes`. Read and write it with a real CSV parser, such as Python's `csv` or a spreadsheet. It is the same flavour as `game/translations/*.csv`, so a spreadsheet round-trip is safe.

- **key**: a stable, dot-namespaced key. Card and metric keys embed W1's IDs (`card.<uuid>.name`, `metric.emissions_gtco2eq.label`), and mandate keys embed the `scenarios.json` mandate IDs (`mandate.food_land.code`). Keys never change when wording changes.
- **en / es / ca**: complete phrases. `\n` is a forced line break. The only one is `print.deck.cost_unit`.
- **Placeholders**: `%{name}`, the same syntax the game uses (rust-i18n). Every language must use exactly the same set.
- **params**: one entry per placeholder, separated by `;`.
  - `name=runtime` means the caller supplies the value. Examples are a card name, a table code or a tally.
  - `name=<derivation>:<format>` means the value is model data. `check.py` computes it from the W1 fixtures and baseline, then formats it with `pct`, `int`, `dec1` or `dec2` (rules in `glossary.md`).
  - No model number is ever typed into a translation.
- **max_chars**: filled only for printed strings. It equals 1.35 × the longest English text the W2 stress test proved for that field. For card fields, that is the stress sample text, so a card effect gets 211 characters. A translation over the limit is **flagged**, not failed. An English string over its own limit fails.
- **sources**: the W2 template fields this key fills, as `alias:Block.path[index]`. The aliases are listed in `TEMPLATES` in `check.py`. For example, `mandates:STRINGS.roleCols[1]` fills that field in all four mandate sheets. A key with no sources is screen-only, prompt-only, or outside the data blocks, such as `print.doc_title.*`.
- **notes**: context for translators and reviewers, including which Spanish terms were reused from es.csv.

### Namespaces

| Namespace | Keys | Used by |
|---|---|---|
| `card.<id>.name / .effect / .excludes` | 31 | Card faces (deck A), and card names for all 21 workshop cards for deck B, gates, validation and screens. Spanish names reuse es.csv. |
| `theme.<id>.short / .title` | 10 | Printed card theme words, and the `WorkshopTheme` screen titles (`workshop.rs`) |
| `mandate.<id>.*` | 52 | Mandate sheets (`TABLE` block): code, name, answerable-for, question, 3 evidence rows |
| `print.*` | 109 | Shared `STRINGS` blocks: `deck`, `mandate`, `mat`, `slip`, `prediction`, `wall`, `doc_title` |
| `vote.*` | 8 | Voting cards, wall outcomes, ballot options |
| `common.*` | 8 | Words shared across templates (Table, Round, Indicator, Cost, ↑/↓) |
| `role.*`, `phase.*` | 18 | Roles grid, projected clock and phase names |
| `metric.<id>.label / .unit / .value / .hint` | 36 | Prediction list, Planet and Report. Also direction and shortage wording |
| `validation.*` | 20 | Operator messages for every invalid-amendment reason in the spec and run sheet |
| `view.planet / ballot / report / replay.*` | 49 | Projected screens (W6) |
| `operator.*` | 25 | Operator controls (W6) |
| `prompt.*` | 18 | Facilitator lines said aloud |
| `instructions.print.*` | 6 | Printing instructions for each edition |

The `validation.*` keys cover these reasons:

- over budget
- repeated card
- card already passed
- prerequisite not passed yet (with an OR joiner)
- card not available
- declared exclusion (none exists in the current deck, but the key is kept)
- coal conflict as a warning, in the same package and across rounds, plus the facilitator's choice line
- reused sponsor
- fewer than two tables
- too many cards, and no cards
- signatures cancelled
- still invalid at Decide

There are also titles for rejected, warning and valid.

## How W6 and W7 consume it

- **Print editions (W7).** Each W2 template keeps its text in `STRINGS`, `TABLE` and `CARDS` blocks.
  - `check.py --render DIR` shows the mapping end to end. It evaluates each block with Node, replaces every leaf named in `sources` with the rendered catalog value, sets `<html lang>` and `<title>`, and writes `DIR/<lang>/<template>.html`.
  - W7 can adopt that renderer or reimplement it. Either way, the output is not committed here, because editions belong to W7 in `references/assembly/editions/`.
- **Screens (W6/W7).** Look strings up **by key**. `game/src/locales.rs` currently hashes the *English text* as the key. That breaks in two ways, and `check.py` reports both. Two catalog rows can share English text but translate it differently (currently 0). A catalog row can also share its English with an es.csv key but use different Spanish. There are currently 2: `Food` is *Comida* in es.csv and *Alimentación* here, and `Contentedness` is *Felicidad* in es.csv and *Satisfacción* here.
  - Key-based lookup is still the safer interface. It is what design decision 2 asks for.
  - Registering Catalan also needs a `ca` locale that the game does not have yet (see the gaps below).
- **Card names in sentences.** When an English string mentions a card name, `check.py` requires the es/ca string to contain that card's es/ca name. Runtime messages should take names from `card.<id>.name` in the session language.

## Checks

Run from the repository root:

```bash
python3 references/assembly/content/check.py                  # checks only
python3 references/assembly/content/check.py --write-gaps     # also rewrite ingame-gaps.csv
python3 references/assembly/content/check.py --render /tmp/qe-editions   # proof copies (not committed)
```

What fails the check:

- empty or duplicate keys
- placeholder sets that differ between languages or from `params`
- unknown derivations or formats
- card IDs that are not in the workshop world (2022 fixture row, 21 cards), or English card names that differ from the engine name
- metric IDs that are not in `meta.units` or the numeric row, and mandate IDs that are not in `scenarios.json`
- template card costs that differ from `scenarios.json`
- numbers that differ between en, es and ca once rendered (decimal comma normalized)
- "Three-quarters" when the baseline no longer says 75%
- card names quoted in a sentence that do not use that locale's card name
- mat table codes that differ from the mandate codes, or codes that are not unique per language
- a visible template string with no key, a stale source, or English that no longer matches the template text
- English over `max_chars`, or a card effect over 160 English characters
- es/ca run sheets that drift from the English one: heading levels, table rows, checklist items, list items, the set of code spans, or the numbers
- a catalog phrase quoted in `run-sheet.md` (mandate names, codes, questions, card names, roles, metric labels, prompts; 42 today) whose es/ca text is missing from the translated run sheet, so the guide cannot drift from the catalog

Reported as flags, without failing: es/ca strings over `max_chars`, catalog rows that share English but translate it differently, and catalog rows whose English is also a key in `game/translations/es.csv` with a different Spanish value (paper and screen would disagree).

The derived values come from:

- the 2022 and 2023 rows of `fixtures/a_no_action.json`: opening values, mix shares in 5% units, protected land and the first-year jump
- `model-baseline.md` §5 effect rows: transfer sizes, `ProtectLand`, Outlook +20 and ExtinctionRate −0.5

### Check output (2026-11-25)

```text
catalog: 390 keys × 3 languages (card 31, common 8, instructions 6, mandate 52, metric 36, operator 25, phase 12, print 109, prompt 18, role 6, theme 10, validation 20, view 49, vote 8)
templates: 11 files, 314 visible strings matched to catalog keys
derived params: 25 values from fixtures/a_no_action 2022–2023 rows, scenarios.json and model-baseline.md
in-game strings an assembly session can show: 173; Catalan source: 0; also missing in es.csv: 16

ERRORS (0)

LINE-FIT FLAGS: es/ca over the tested 35% headroom (3)
  mandate.living_standards.evidence.2.value [es] 29 > 24 (en 19): '7,95 mil millones de personas'
  mandate.living_standards.evidence.2.value [ca] 28 > 24 (en 19): '7,95 mil milions de persones'
  mandate.living_world.evidence.3.value [es] 36 > 33 (en 26): '+1,10 °C de calentamiento (anomalía)'

LINE-FIT FLAGS, short labels (<15 en chars; ratio unreliable, verify in the A4 proof) (39)
  common.up_arrow [es] 8 > 5 (en 4): '↑ arriba'
  common.up_arrow [ca] 7 > 5 (en 4): '↑ amunt'
  theme.food.short [es] 12 > 5 (en 4): 'Alimentación'
  theme.food.short [ca] 11 > 5 (en 4): 'Alimentació'
  theme.land.short [es] 19 > 17 (en 13): 'Tierra y naturaleza'
  mandate.food_land.name [es] 21 > 17 (en 13): 'Alimentación y tierra'
  mandate.food_land.name [ca] 19 > 17 (en 13): 'Alimentació i terra'
  mandate.energy_industry.evidence.2.value [es] 19 > 17 (en 13): '35\xa0% del suministro'
  mandate.energy_industry.evidence.2.value [ca] 24 > 17 (en 13): '35\xa0% del subministrament'
  role.timekeeper [es] 18 > 13 (en 10): 'Control del tiempo'
  role.timekeeper [ca] 17 > 13 (en 10): 'Control del temps'
  phase.replay [es] 10 > 8 (en 6): 'Repetición'
  phase.replay [ca] 9 > 8 (en 6): 'Repetició'
  print.deck.none [es] 7 > 5 (en 4): 'ninguna'
  print.deck.deck_a [ca] 9 > 8 (en 6): 'Baralla A'
  print.deck.deck_b [ca] 9 > 8 (en 6): 'Baralla B'
  print.mandate.p_by [es] 10 > 9 (en 7): 'Año límite'
  print.mandate.above [es] 17 > 12 (en 9): 'acabar por encima'
  print.mandate.above [ca] 16 > 12 (en 9): 'acabar per sobre'
  print.mandate.below [es] 17 > 12 (en 9): 'acabar por debajo'
  print.mandate.below [ca] 15 > 12 (en 9): 'acabar per sota'
  print.mat.title [es] 19 > 17 (en 13): 'Tapete de coalición'
  print.mat.replaces [es] 11 > 10 (en 8): 'sustituye a'
  print.mat.replaces [ca] 11 > 10 (en 8): 'substitueix'
  print.mat.budget [es] 11 > 8 (en 6): 'Presupuesto'
  print.mat.budget [ca] 10 > 8 (en 6): 'Pressupost'
  print.mat.check.2 [es] 20 > 17 (en 13): 'Sin cartas repetidas'
  print.mat.check.2 [ca] 18 > 17 (en 13): 'Cap carta repetida'
  print.mat.pred_up [es] 8 > 5 (en 4): 'Arriba ↑'
  print.mat.pred_up [ca] 7 > 5 (en 4): 'Amunt ↑'
  print.slip.card_id [es] 11 > 9 (en 7): 'ID de carta'
  print.slip.card_id [ca] 11 > 9 (en 7): 'ID de carta'
  print.slip.outcome.mat_a [ca] 20 > 18 (en 14): 'va passar al tapet A'
  print.slip.outcome.mat_b [ca] 20 > 18 (en 14): 'va passar al tapet B'
  print.slip.outcome.dropped [es] 10 > 9 (en 7): 'descartada'
  print.slip.outcome.dropped [ca] 10 > 9 (en 7): 'descartada'
  print.prediction.right [es] 7 > 6 (en 5): 'acierto'
  print.wall.held [es] 12 > 6 (en 5): '¿Se cumplió?'
  print.wall.held [ca] 14 > 6 (en 5): 'S’ha complert?'

LOCALES.RS COLLISIONS: catalog rows with the same English, different translation (0)

ES.CSV CONFLICTS: same English as game/translations/es.csv, different Spanish (2)
  theme.food.short 'Food': catalog 'Alimentación' vs es.csv 'Comida'
  metric.outlook.label 'Contentedness': catalog 'Satisfacción' vs es.csv 'Felicidad'

RESULT: PASS
```

### A4 proof of the flagged strings

The ratio flags are only estimates, so the real test was a print proof. All three languages were rendered with `check.py --render` and run through W2's `print/check-fit.py --stretch 1.15` (headless Chrome, 2026-11-25):

| Edition | As-is (real copy) | Stress ×1.15 on top of the translation |
|---|---|---|
| en | 11/11 templates OK, page counts and A4 sizes unchanged | 11/11 OK |
| es | 11/11 OK | 10/11. `sheet-08-mandate-living-world` overflows the page |
| ca | 11/11 OK | 11/11 OK |

**Every flagged string fits on the real page in every language.** The tightest sheet is the Spanish Living World mandate, which has less than 15% spare height. If a reviewer lengthens any shared mandate string in Spanish, rerun the proof. A visual check of a low-resolution raster of the Spanish deck A showed accents, decimal commas and `35 %` with a non-breaking space rendering correctly, and no clipped card text. Headless PDF proves the layout, not the printer: W7 still owes a physical A4 proof per language.

## Reviewer corrections log

Record every change a fluent reviewer makes. Note any key whose meaning changed, because W6 and W7 may need to re-check line fit or screen layout.

| Date | Reviewer | Language | Key or file | Was | Now | Reason |
|---|---|---|---|---|---|---|
| | | | | | | |

## Outstanding terms (need a fluent human decision)

Catalan comes first because nothing in the project has been translated into Catalan before.

1. **Table names and codes.**
   - Names: *Alimentación y tierra / Alimentació i terra*, *Energía e industria*, *Nivel de vida*, *Mundo vivo / Món viu*.
   - The localized codes AT · EI · NV · MV are written by hand on mats and slips, and W6 shows them at ballot entry. The alternative is to keep FL/EI/LS/LW in every language. Decide before W6 builds sponsor entry.
2. **Role nouns.** These were chosen to avoid gendered forms: *Enlace / Enllaç* (envoy), *Registro / Registre* (recorder) and *Control del tiempo / temps* (timekeeper). *Registro* as a role name is the weakest of them.
3. **Contentedness.** The paper uses *satisfacción / satisfacció*. es.csv uses *Felicidad* for the in-game stat label (key `Contentedness`), but *satisfacción* in its sentences and in the workshop report. Pick one for the screen and the paper.
4. **Extinction pressure.** The paper uses *presión de extinción / pressió d’extinció*, following the English paper. The screen label is *Extinction Rate*, which es.csv translates as *Tasa de extinción*. Paper and screen already differ in English.
5. **Themes.** The printed card words (Energy, Food, Land & nature, Industry & transport, Society) differ from the `workshop.rs` screen titles (Food & Agriculture, Land & Biodiversity…) even in English. Both sets are in the catalog.
6. **pip** → *bloque / bloc*. **coalition mat** → *tapete / tapet de coalición*. **Reveal** → *Resultados / Resultats*. **Replay** → *Repetición / Repetició* (or *Historia alternativa*). **public promise** → *compromiso / compromís* (rather than *promesa*).
7. **The game title** "Quarter Earth" is left untranslated.
8. **The Catalan card names.** All 21 are new. Examples: *Luxe per a tothom*, *Prohibir els gats a l’aire lliure*, *Reprimir la criptomineria*, *Quotes d’energia*, *Gestió de la radiació solar (GRS)*.
9. **es.csv duplicates.** For four cards, es.csv has two translations. The catalog picks one for each (see the notes on `card.*.name`): crypto-mining, public transit, air travel and energy quotas. The game will show whichever translation es.csv's hash lookup returns.
10. **The facilitator register.** The run sheets use *tú/tu*. Tables are addressed as *vosotros/vosaltres*. Some groups prefer *usted*, or the infinitive throughout.
11. **Card names inside sentences.** In es/ca prose, card names are wrapped in «» (*Desbloquea «Electrificación masiva»*, the `validation.*` messages), because sentence-case names otherwise read as part of the sentence. The printed `Requires:` and `Cannot combine with:` lines and the run sheets use them unquoted or in bold. Confirm the convention.
12. **Political framing.** The card effect copy came from W2's plain-language reading of the decoded effects. The translations keep its neutrality but have not been checked for framing in a Spanish or Catalan political context. Check *Decrecimiento*, *Lujos para todos* and *Obligación vegetariana* especially.

## Gaps for W6 and W7

- **There is no Catalan game locale.** `game/translations/` has no `ca` file. `ingame-gaps.csv` lists the 173 strings that an assembly session can show on screen, together with their es.csv translation and an empty Catalan column. The list includes:
  - the `t!` literals in `views/report.rs`, `workshop.rs`, `views/session/plan.rs`, `views/session/mod.rs`, `views/world.rs`, `views/ending.rs` and `splash/menu.rs`
  - the 5 theme titles
  - the 21 card names
  
  W7 needs a Catalan file for these, or needs the assembly views to read from this catalog instead. Card descriptions and flavour text shown in-game are not in the list. If the assembly screens show them, they need Catalan too.
- **16 of those strings are also missing in es.csv**, so the screen shows them in English even in Spanish. Some are format patterns such as `{value:+.1} °C`, which also need locale-aware numbers. The missing strings are:
  - *Also Tracked*, *Energy Supplied*, *Calories Supplied*
  - *Water shortage: demand exceeds the available supply.*
  - `{value:+.1} GtCO2e/year`, `{left} or {right}`, `{}, or {last}`
  - *Your History*, *Policy {}*
  - *Custom World*, and the two world-file errors
  - plus a few format-only patterns
- **The screen and the paper disagree on terms and units.** `report.rs` formats emissions as "GtCO2e/year", while the paper uses "Gt CO₂-eq per year". Report numbers use `{:.1}` with a decimal point in every locale. W6 should take the unit from `metric.*.unit` and format numbers per locale.
- **Templates still assemble sentences from fragments.** This works against design decision 2. The cases are:
  - the prediction slip (`by` / `indicator` / `will` / `up` / `down` / `because`)
  - the mandate's `above` and `below`
  - the mat title "Coalition mat · Package A"
  - "Card N"
  - the footer "Deck A · 3/8"
  
  The es/ca fragments were chosen so that each assembled line reads naturally (*irá hacia ↑ arriba*, *anirà cap ↑ amunt*), but a whole-sentence template is the proper fix (W2/W7).
- **Some text sits outside the data blocks.** Every template has `<html lang="en">` and a hard-coded `<title>`. W7 must set both per edition. `lang` drives hyphenation and screen readers. The keys are `print.doc_title.*`.
- **Deck B** still has placeholders, so there is no effect text to translate. The names of all 21 cards are ready.
- **es grammar rules for the renderer.** In Spanish, *y* becomes *e* before an *i-/hi-* sound and *o* becomes *u* before an *o-/ho-* sound. This applies to `view.ballot.sponsored_by` and `validation.requirement_or` when names are substituted at runtime. No current card name or code triggers it.
