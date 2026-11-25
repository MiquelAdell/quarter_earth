# Assembly parity fixtures

These fixtures are the numerical reference for the assembly simulation (W5).
They were produced by the headless reference driver at revision
`b9b11964b668c432fe991931582bc262442205c0`, using the embedded workshop world
`engine/assets/WORKSHOP.world` (sha256
`e42723ddae162eb841020610e34d36eabab9d374d797ad5057c17973f105ea5d`).
Read `../model-baseline.md` first. It explains the driver, the climate
variants and the tolerances.

## Files

| Path | Content |
|---|---|
| `scenarios.json` | The scenario definitions: the eight opening cards and, for each scenario, the card IDs passed in each planning year, in order. |
| `<scenario>/ozone_appended.{json,csv}` | Canonical run for the native climate variant in which the yearly NOX/CO/NMVOC defaults are appended to Hector's `ozone` section. The browser path is closest to this variant. |
| `<scenario>/oh_appended.{json,csv}` | Canonical run for the other native variant, where the defaults are appended to the `OH` section. |
| `reproducibility.json` | Per-run hashes, bit-identity per variant, the cross-variant maximum differences, and the open-loop browser-Hector replay deltas. |

Scenarios: `a_no_action`, `b_comfort_and_technology`, `c_sufficiency`.

### JSON layout

- `meta.units` gives the unit of every numeric field.
- `meta.nox_append_lengths` identifies the variant.
- `decision_log` lists every pass, with the PC left afterwards.
- `rows[]` has one entry per year from 2022 to 2052 (31 rows). Each row has:
  - `numeric`: continuous indicators; compare these within tolerance.
  - `discrete`: year, PC, flags, policy queue, process mix shares, the status and lock state of all 21 cards, region income levels, and `game_over`. These must match exactly. PC is the exception: see model-baseline §8.
  - `regions`: per-region diagnostics.

Row timing:

- The 2022 row is the initial state, taken before any planning.
- Row `Y` for `Y > 2022` is the state right after the step that advanced the world to year `Y`. For planning years it is taken before that year's decisions are passed.
- `tgav_input_c` on row `Y` is the Hector value that was passed into that step.

The CSV is a flat projection of the same rows. Units are in the column
names. Mixes are pipe-separated `process=share` pairs, where each share unit
is 5%.

## Regenerate

From the repository root:

```sh
export CXXFLAGS="-I/opt/homebrew/include"
cargo build -p hes-game --example assembly_baseline
python3 references/assembly/tools/assembly_baseline.py --runs 5          # add --no-web to skip Node
```

- `references/assembly/tools/assembly_baseline.py` runs each scenario in at least 5 separate processes, topping up until each native variant has been seen at least twice. It checks that runs of the same variant are bit-identical, copies one run per variant here, and writes `reproducibility.json`.
- To run a single scenario by hand:

  ```sh
  cargo run -p hes-game --example assembly_baseline -- \
    references/assembly/fixtures/scenarios.json b_comfort_and_technology out.json out.csv
  ```

- The browser-path replay needs Node 22: `node references/assembly/tools/assembly_web_replay.mjs <fixture.json>`.

## Reproducibility, as captured

- Within each variant, every run was bit-identical. The 2026-11-25 generation ran each scenario in 5 debug-build processes, covering both variants at least twice, plus 6 more release-build processes per scenario.
- The debug and `--release` builds produce bit-identical engine state. The only differences are in the harness-recomputed diagnostics `extinction_from_tgav` (≤2.4e-7) and `extinction_from_slr` (≤3.7e-9), which is `powf` noise.
- The two variants differ in climate-coupled fields only: temperature up to 0.028 °C and outlook up to 1.3 by 2052. Discrete state was identical across variants in all three scenarios. See `reproducibility.json`.
