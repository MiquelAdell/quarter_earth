# Assembly parity fixtures

These fixtures are the numerical reference for the assembly simulation (W5).
They were produced by the headless reference driver on branch
`feature/assembly-workshop`. The base is `b9b11964`; the fixtures were
regenerated on top of `cb487502`, which includes the native climate fix in
`game/src/climate.rs`, with the production-allocator fix in
`engine/src/production/planner.rs` applied. The allocator fix diverges from
upstream when land is scarce; see model-baseline §7.1. They use the embedded workshop world
`engine/assets/WORKSHOP.world` (sha256
`e42723ddae162eb841020610e34d36eabab9d374d797ad5057c17973f105ea5d`).
Read `../model-baseline.md` first. It explains the driver, the climate path
and the tolerances.

## Files

| Path | Content |
|---|---|
| `scenarios.json` | The scenario definitions: the eight opening cards and, for each scenario, the card IDs passed in each planning year, in order. |
| `<scenario>.{json,csv}` | The canonical native run for each scenario. There is one trajectory per scenario. |
| `reproducibility.json` | Per-run hashes per build profile, bit-identity flags, any cross-profile last-bit differences, and the open-loop browser-Hector replay deltas. |

Scenarios:

- `a_no_action`, `b_comfort_and_technology`, `c_sufficiency`: the parity scenarios.
- `d_preserves_only`: a diagnostic for the land-squeeze mechanism and the allocator fix (model-baseline §7, §7.1).

### JSON layout

- `meta.units` gives the unit of every numeric field.
- `meta.nox_append_lengths` gives the Hector `OH` and `ozone` NOX series lengths, which must be equal. This is a regression check for the climate fix.
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
cargo build -p hes-game --example assembly_baseline --release
python3 references/assembly/tools/assembly_baseline.py --runs 3 --profiles debug,release
# add --no-web to skip the Node replay; --profiles debug to skip release
```

- The script runs each scenario in `--runs` separate processes per profile.
- It fails if two runs of the same profile differ, if discrete state differs across profiles, or if the `OH` and `ozone` NOX lengths differ.
- It records any cross-profile numeric differences and copies the first debug run to `<scenario>.{json,csv}`.
- To run a single scenario by hand:

  ```sh
  cargo run -p hes-game --example assembly_baseline -- \
    references/assembly/fixtures/scenarios.json b_comfort_and_technology out.json out.csv
  ```

- The browser-path replay needs Node 22: `node references/assembly/tools/assembly_web_replay.mjs <fixture.json>`.

## Reproducibility, as captured

- For each scenario, 3 debug-build and 3 release-build processes ran. Every run within a profile was bit-identical.
- Debug vs release:
  - a and d: identical.
  - b and c: `extinction_rate` differed by one f32 ulp (7.6e-6) from 2040; nothing else differed.
- Browser Hector (open loop) is within 0.0022 °C of native in a and c, and up to 0.0087 °C in the land-scarce b and d. See `reproducibility.json` and model-baseline §4.
