//! Headless reference driver for the assembly-workshop parity fixtures (W1).
//!
//! Replays the workshop-mode annual loop without a window so that a scenario
//! (which card IDs are passed in which planning year) can be run repeatedly
//! and its annual state recorded. It does NOT change game or engine
//! behaviour: it only calls public engine APIs and the game's own climate
//! module, which is compiled in verbatim via `#[path]`.
//!
//! What is reused verbatim:
//! - `game/src/climate.rs` (native Hector FFI wrapper, `Climate`).
//! - `game/src/consts.rs` (`MAX_EMISSIONS`, `WORKSHOP_PC_BUDGET`).
//! - `hes_engine::World::workshop()` (the embedded `engine/assets/WORKSHOP.world`).
//!
//! What is mirrored line-for-line (cannot be imported: private to the game
//! crate or entangled with egui/GL):
//! - `WorkshopOpts::apply` (game/src/workshop.rs): add `Flag::ParliamentSuspended`.
//! - `WorkshopOpts::begin_planning` (game/src/workshop.rs): PC := 30.
//! - `StateExt::pay_points` + `StateExt::pass_policy` (game/src/state/game.rs),
//!   i.e. what `workshop::toggle_policy` does for an unpassed card.
//! - `WorldEvents::new` + `WorldEvents::next_phase` (game/src/views/world.rs):
//!   per world phase a fresh `Climate::new(start_year)` restored from the
//!   saved emissions; per year `get_emissions` -> `add_emissions` ->
//!   `tgav(year + 1)` -> `step_year(tgav)`; at the cycle boundary
//!   `finish_cycle` -> `upgrade_projects` (no-op: no queued upgrades in
//!   workshop) -> `update_processes` (no-op: workshop plan screen is
//!   policies-only, so `process_mix_changes` stays empty).
//! - `get_emissions` (game/src/views/world.rs).
//! - `Report` "Next" (game/src/views/report.rs): `check_requests()` is called
//!   while building the report; PC change is irrelevant because the next
//!   `begin_planning` overwrites PC.
//!
//! Usage (from the repo root):
//!   export CXXFLAGS="-I/opt/homebrew/include"
//!   cargo run -p hes-game --example assembly_baseline -- \
//!     references/assembly/fixtures/scenarios.json <scenario_id> <out.json> [<out.csv>]

#[path = "../src/climate.rs"]
mod climate;
#[allow(dead_code)]
#[path = "../src/consts.rs"]
mod consts;

use std::collections::{BTreeMap, HashMap};

use climate::{Climate, EmissionsData};
use hes_engine::{Flag, Id, Output, ProjectType, Resource, State, Status, World};
use serde_json::{Value, json};

/// Mirror of `get_emissions` in game/src/views/world.rs.
fn get_emissions(state: &State) -> HashMap<&'static str, f64> {
    let emissions_factor = (consts::MAX_EMISSIONS / state.emissions.as_gtco2eq().abs()).min(1.0);
    let (co2, ch4, n2o) = state.emissions.for_hector();
    let mut emissions = HashMap::default();
    emissions.insert("ffi_emissions", (co2 * emissions_factor) as f64);
    emissions.insert("CH4_emissions", (ch4 * emissions_factor) as f64);
    emissions.insert("N2O_emissions", (n2o * emissions_factor) as f64);
    emissions
}

/// Mirror of `StateExt::pay_points` + `StateExt::pass_policy`.
fn pass_card(state: &mut State, id: &Id) -> Result<(), String> {
    let project = &state.world.projects[id];
    if project.kind != ProjectType::Policy {
        return Err(format!("{} is not a Policy", project.name));
    }
    if project.locked {
        return Err(format!("{} is locked in {}", project.name, state.world.year));
    }
    if project.status != Status::Inactive {
        return Err(format!("{} is not Inactive ({:?})", project.name, project.status));
    }
    let cost = project.cost as isize;
    if state.political_capital < cost {
        return Err(format!(
            "{} costs {cost} but only {} PC left",
            project.name, state.political_capital
        ));
    }
    state.change_political_capital(-cost);
    state.start_project(id);
    Ok(())
}

fn outputs(map: &hes_engine::OutputMap) -> Value {
    json!({
        "electricity_kwh": map.electricity,
        "fuel_kwh": map.fuel,
        "plant_calories_kcal": map.plant_calories,
        "animal_calories_kcal": map.animal_calories,
    })
}

/// Which hector section received the appended NOX default this process.
/// Native `Climate::add_emissions` walks nested HashMaps and stops at the
/// first section containing the key; NOX/CO/NMVOC exist under both `OH` and
/// `ozone`, so the target depends on HashMap iteration order.
fn nox_append_section(data: &EmissionsData) -> String {
    let len = |section: &str| {
        data.get(section)
            .and_then(|m| m.get("NOX_emissions"))
            .map(|v| v.len())
            .unwrap_or(0)
    };
    let (oh, ozone) = (len("OH"), len("ozone"));
    format!("OH:{oh},ozone:{ozone}")
}

fn snapshot(state: &State, tgav_input: f32, hector_in: Option<&HashMap<&'static str, f64>>) -> Value {
    let world = &state.world;
    let start_land = world.starting_resources.land;
    let land_demand = state.resource_demand.of(Resource::Land);
    let water_demand = state.resource_demand.of(Resource::Water);
    let produced = state.produced.total();
    let demand = state.output_demand.total();

    let mut mix: BTreeMap<String, BTreeMap<String, usize>> = BTreeMap::new();
    for p in world.processes.iter() {
        if p.locked && p.mix_share == 0 {
            continue;
        }
        let key = match p.output {
            Output::Electricity => "electricity",
            Output::Fuel => "fuel",
            Output::PlantCalories => "plant_calories",
            Output::AnimalCalories => "animal_calories",
        };
        mix.entry(key.into()).or_default().insert(p.name.clone(), p.mix_share);
    }

    let cards: BTreeMap<String, Value> = world
        .projects
        .iter()
        // The 21-card deck: unlocked policies plus prerequisite-gated ones.
        .filter(|p| p.kind == ProjectType::Policy && (!p.locked || is_gated(state, &p.id)))
        .map(|p| {
            (
                p.id.to_string(),
                json!({
                    "name": p.name,
                    "status": format!("{:?}", p.status),
                    "locked": p.locked,
                    "active_outcome": p.active_outcome,
                }),
            )
        })
        .collect();

    let mut flags: Vec<String> = state.flags.iter().map(|f| format!("{f:?}")).collect();
    flags.sort();

    let regions: Vec<Value> = world
        .regions
        .iter()
        .map(|r| {
            json!({
                "name": r.name,
                "income": format!("{:?}", r.income),
                "development": r.development,
                "outlook": r.outlook,
                "habitability": r.habitability(),
                "population": r.population,
            })
        })
        .collect();

    json!({
        "year": world.year,
        "numeric": {
            "tgav_input_c": tgav_input,
            "temperature_c": world.temperature,
            "emissions_gtco2eq": state.emissions.as_gtco2eq(),
            "emissions_co2_g": state.emissions.co2,
            "emissions_ch4_g": state.emissions.ch4,
            "emissions_n2o_g": state.emissions.n2o,
            "hector_ffi_pgc": hector_in.map(|h| h["ffi_emissions"]),
            "hector_ch4_tg": hector_in.map(|h| h["CH4_emissions"]),
            "hector_n2o_tg": hector_in.map(|h| h["N2O_emissions"]),
            "extinction_rate": world.extinction_rate,
            "extinction_from_tgav": world.tgav_extinction_rate(),
            "extinction_from_slr": world.slr_extinction_rate(),
            "sea_level_rise_m": world.sea_level_rise,
            "outlook": state.outlook(),
            "world_outlook": world.outlook(),
            "shortages_outlook": state.shortages_outlook,
            "base_outlook": world.base_outlook,
            "region_mean_outlook": world.regions.outlook(),
            "mean_habitability": world.regions.habitability(),
            "land_use_fraction": (land_demand + state.protected_land * start_land) / start_land,
            "land_demand_fraction": land_demand / start_land,
            "protected_land_fraction": state.protected_land,
            "water_use_fraction": water_demand / state.resources.available.water,
            "population": world.regions.iter().map(|r| r.population).sum::<f32>(),
            "produced": outputs(&produced),
            "demand": outputs(&demand),
        },
        "discrete": {
            "year": world.year,
            "political_capital": state.political_capital,
            "flags": flags,
            "policy_queue": state.policy_queue.iter().map(|id| id.to_string()).collect::<Vec<_>>(),
            "mix_share_5pct_units": mix,
            "cards": cards,
            "region_income": world.regions.iter().map(|r| format!("{:?}", r.income)).collect::<Vec<_>>(),
            "game_over": state.game_over,
        },
        "regions": regions,
    })
}

/// True if some project's effects include `UnlocksProject(id)`.
fn is_gated(state: &State, id: &Id) -> bool {
    state.world.projects.iter().any(|p| {
        p.effects
            .iter()
            .any(|e| matches!(e, hes_engine::Effect::UnlocksProject(u) if u == id))
    })
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 4 {
        eprintln!("usage: assembly_baseline <scenarios.json> <scenario_id> <out.json> [<out.csv>]");
        std::process::exit(2);
    }
    let spec: Value =
        serde_json::from_str(&std::fs::read_to_string(&args[1]).expect("read scenarios"))
            .expect("parse scenarios");
    let scenario = spec["scenarios"]
        .as_array()
        .expect("scenarios array")
        .iter()
        .find(|s| s["id"] == args[2].as_str())
        .unwrap_or_else(|| panic!("unknown scenario {}", args[2]))
        .clone();
    let decisions: BTreeMap<usize, Vec<Id>> = scenario["decisions"]
        .as_object()
        .expect("decisions object")
        .iter()
        .map(|(year, cards)| {
            let ids = cards
                .as_array()
                .expect("card list")
                .iter()
                .map(|c| {
                    let s = c["id"].as_str().expect("card id");
                    Id::parse_str(s).unwrap_or_else(|_| panic!("bad id {s}"))
                })
                .collect();
            (year.parse().expect("year key"), ids)
        })
        .collect();

    // GameState::default() under WORKSHOP=1 + prepare_game().
    let mut state = State::new(World::workshop());
    if !state.flags.contains(&Flag::ParliamentSuspended) {
        state.flags.push(Flag::ParliamentSuspended);
    }
    let start_year = state.world.year;
    let mut saved_emissions: EmissionsData = Default::default(); // UIState.emissions

    let mut rows = vec![snapshot(&state, state.world.temperature, None)];
    let mut log = vec![];
    let mut nox_section;

    // GameView::new under WORKSHOP: straight to planning; loop until the
    // report after the final cycle routes to the ending (has_reached_end).
    loop {
        // Session::new -> begin_planning.
        state.political_capital = consts::WORKSHOP_PC_BUDGET;
        let year = state.world.year;
        for id in decisions.get(&year).cloned().unwrap_or_default() {
            match pass_card(&mut state, &id) {
                Ok(()) => log.push(json!({"year": year, "passed": id.to_string(),
                    "name": state.world.projects[&id].name, "pc_left": state.political_capital})),
                Err(e) => panic!("illegal decision in {year}: {e}"),
            }
        }

        // WorldEvents::new.
        let cycle_start_year = state.world.year;
        let mut climate = Climate::new(start_year);
        if !saved_emissions.is_empty() {
            climate.set_emissions_data(saved_emissions.clone());
        }

        loop {
            // Disasters -> ComputeTgav transition.
            let emissions = get_emissions(&state);
            climate.add_emissions(emissions.clone());
            saved_emissions = climate.emissions_data();
            // ComputeTgav.
            let tgav = climate.tgav(state.world.year + 1).expect("native tgav is sync");
            // StepYear.
            let _updates = state.step_year(tgav);
            rows.push(snapshot(&state, tgav, Some(&emissions)));
            // Events (empty in workshop) -> Disasters: boundary check.
            if state.world.year > cycle_start_year && state.is_planning_year() {
                state.finish_cycle();
                break;
            }
        }
        nox_section = nox_append_section(&saved_emissions);

        // Report::new / render: check_requests() is evaluated for display.
        let _ = state.check_requests();

        if state.world.year >= state.death_year {
            break;
        }
    }

    let out = json!({
        "meta": {
            "scenario": scenario,
            "world": "engine/assets/WORKSHOP.world (embedded via World::workshop())",
            "start_year": start_year,
            "death_year": state.death_year,
            "nox_append_lengths": nox_section,
            "units": {
                "tgav_input_c": "degC anomaly returned by Hector for year+1, passed to step_year",
                "temperature_c": "degC anomaly (world.temperature = tgav + temperature_modifier)",
                "emissions_gtco2eq": "Gt CO2-eq per year (Emissions::as_gtco2eq)",
                "emissions_*_g": "grams per year (raw engine units)",
                "hector_ffi_pgc": "Pg C per year sent to Hector (after MAX_EMISSIONS cap)",
                "hector_ch4_tg|hector_n2o_tg": "Tg per year sent to Hector",
                "extinction_rate": "dimensionless biodiversity-pressure index (world.extinction_rate)",
                "outlook": "dimensionless contentedness (State::outlook = world.outlook - shortages_outlook)",
                "land_use_fraction": "0..1+ of starting habitable land incl. protected (StateExt::land_use_percent, unclamped)",
                "protected_land_fraction": "0..1 of starting habitable land",
                "water_use_fraction": "0..1+ of available water",
                "sea_level_rise_m": "metres",
                "population": "people",
                "produced.*_kwh": "kWh per year (display: x1e-12 = PWh)",
                "produced.*_kcal": "kcal per year (display: x1e-9/2e4 = units of 20,000 Tcal)",
                "mix_share_5pct_units": "process mix share in 5% units (x5 = percent of output)",
            },
        },
        "decision_log": log,
        "rows": rows,
    });
    std::fs::write(&args[3], serde_json::to_string_pretty(&out).unwrap()).expect("write json");

    if let Some(csv_path) = args.get(4) {
        let mut csv = String::from(
            "year,tgav_input_c,temperature_c,emissions_gtco2eq,extinction_rate,outlook,\
shortages_outlook,land_use_fraction,protected_land_fraction,water_use_fraction,sea_level_rise_m,\
produced_electricity_kwh,demand_electricity_kwh,produced_fuel_kwh,demand_fuel_kwh,\
produced_plant_calories_kcal,demand_plant_calories_kcal,produced_animal_calories_kcal,\
demand_animal_calories_kcal,flags,online_cards,building_cards,electricity_mix_5pct,\
fuel_mix_5pct,plant_calories_mix_5pct,animal_calories_mix_5pct\n",
        );
        for r in &out["rows"].as_array().unwrap()[..] {
            let n = &r["numeric"];
            let d = &r["discrete"];
            let cards_with = |status: &str| {
                let mut v: Vec<String> = d["cards"]
                    .as_object()
                    .unwrap()
                    .values()
                    .filter(|c| c["status"] == status)
                    .map(|c| c["name"].as_str().unwrap().to_string())
                    .collect();
                v.sort();
                v.join("|")
            };
            let mix = |k: &str| {
                d["mix_share_5pct_units"][k]
                    .as_object()
                    .map(|m| {
                        m.iter()
                            .filter(|(_, v)| v.as_u64() != Some(0))
                            .map(|(name, v)| format!("{name}={v}"))
                            .collect::<Vec<_>>()
                            .join("|")
                    })
                    .unwrap_or_default()
            };
            let flags = d["flags"]
                .as_array()
                .unwrap()
                .iter()
                .map(|f| f.as_str().unwrap().to_string())
                .collect::<Vec<_>>()
                .join("|");
            csv.push_str(&format!(
                "{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\"\n",
                r["year"], n["tgav_input_c"], n["temperature_c"], n["emissions_gtco2eq"],
                n["extinction_rate"], n["outlook"], n["shortages_outlook"], n["land_use_fraction"],
                n["protected_land_fraction"], n["water_use_fraction"], n["sea_level_rise_m"],
                n["produced"]["electricity_kwh"], n["demand"]["electricity_kwh"],
                n["produced"]["fuel_kwh"], n["demand"]["fuel_kwh"],
                n["produced"]["plant_calories_kcal"], n["demand"]["plant_calories_kcal"],
                n["produced"]["animal_calories_kcal"], n["demand"]["animal_calories_kcal"],
                flags, cards_with("Active"), cards_with("Building"),
                mix("electricity"), mix("fuel"), mix("plant_calories"), mix("animal_calories"),
            ));
        }
        std::fs::write(csv_path, csv).expect("write csv");
    }
}
