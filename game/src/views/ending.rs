use std::{cmp::Ordering, collections::BTreeMap};

use egui::{Color32, ImageSource};
use egui_taffy::TuiBuilderLogic;
use enum_map::EnumMap;
use hes_engine::*;
use hes_images::{coup_image, death_image, lose_image, win_image};
use rust_i18n::t;
use serde::{Deserialize, Serialize};
use strum::{EnumIter, IntoEnumIterator};

use crate::{
    display::{Icon, icons},
    parts::{button, clear_full_bg_image, h_center, raised_frame},
    state::{GameState, StateExt, WorkshopCycleRecord, WorkshopPolicyAction},
    tips::{add_tip, tip},
    views::events::Events,
    workshop::WORKSHOP,
};

pub struct End {
    lose: bool,
    events: Events,
    badges: Vec<Badge>,
    log: Vec<String>,
    image: Option<ImageSource<'static>>,
    workshop: Option<WorkshopDebriefData>,
}
impl End {
    pub fn new(lose: bool, state: &mut GameState) -> Self {
        let events = if WORKSHOP.active {
            vec![]
        } else if lose {
            StateExt::roll_events(&mut state.core, EventPhase::BreakStart)
        } else {
            StateExt::roll_events(&mut state.core, EventPhase::EndStart)
        };
        let summary = summarize(&state.core, !lose);

        let image = match summary.ending {
            Ending::Win => win_image(&summary.faction),
            Ending::Died => death_image(&summary.faction),
            Ending::Coup => coup_image(&summary.faction),
            Ending::LostOther => lose_image(&summary.faction),
        };

        let log = state
            .ui
            .change_history
            .iter()
            .zip(state.ui.process_mix_history.iter().map(|(_, mixes)| mixes))
            .map(|((year, changes), mixes)| format_year_log(*year, changes, mixes))
            .collect::<Vec<_>>();

        Self {
            lose,
            events: Events::new(events, &state.core),
            badges: eval_badges(state),
            log,
            image,
            workshop: WORKSHOP.active.then(|| workshop_debrief_data(state)),
        }
    }

    pub fn render(&mut self, ui: &mut egui::Ui, state: &mut GameState) -> bool {
        clear_full_bg_image(ui);

        if let Some(workshop) = &self.workshop {
            return render_workshop_debrief(ui, workshop);
        }

        let mut restart = false;
        if !self.events.is_finished {
            self.events.render(ui, state);
        } else {
            ui.vertical_centered(|ui| {
                ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Extend);
                let width = (ui.ctx().content_rect().width() - 12.).min(480.);
                ui.set_max_width(width);

                ui.add_space(64.);

                h_center(ui, "badges", |tui| {
                    tui.ui(|ui| {
                        ui.horizontal(|ui| {
                            for badge in &self.badges {
                                let tip = tip(icons::HELP, t!(badge.to_string()));
                                add_tip(tip, ui.add(badge.icon().size(32.)));
                            }
                        });
                    });
                });

                ui.add_space(64.);

                h_center(ui, "message", |tui| {
                    tui.ui(|ui| {
                        let message = if self.lose {
                            t!("This is not the end...")
                        } else {
                            t!("Well Played!")
                        };
                        ui.label(
                            egui::RichText::new(message.to_uppercase())
                                .heading()
                                .italics()
                                .color(Color32::WHITE),
                        );
                    });
                });

                ui.add_space(64.);

                let resp = ui.add(button(t!("Try Again?")).full_width());
                restart = resp.clicked();

                ui.add_space(32.);

                ui.scope(|ui| {
                    ui.style_mut().interaction.selectable_labels = true;
                    h_center(ui, "history", |tui| {
                        tui.ui(|ui| {
                            ui.set_max_width(width);
                            if let Some(image) = &self.image {
                                ui.add(egui::Image::new(image.clone()));
                            }

                            ui.add_space(32.);

                            ui.style_mut().visuals.override_text_color = Some(Color32::WHITE);
                            ui.monospace(t!("Your History"));

                            if ui.button("Copy to Clipboard").clicked() {
                                ui.ctx().copy_text(self.log.join("\n"));
                            }
                        });
                    });

                    ui.vertical(|ui| {
                        ui.set_width(320.);
                        for line in &self.log {
                            ui.monospace(line);
                        }
                    });
                });

                ui.add_space(64.);
            });
        }
        restart
    }
}

const WORKSHOP_WIN_EMISSIONS: f32 = 0.;
const WORKSHOP_WIN_TEMPERATURE: f32 = 1.;
const WORKSHOP_WIN_EXTINCTION: f32 = 20.;

#[derive(Debug, Clone, PartialEq)]
struct WorkshopThresholdResult {
    label: &'static str,
    value: String,
    target: &'static str,
    passed: bool,
}

#[derive(Debug, Clone, PartialEq)]
struct WorkshopDebriefData {
    won: bool,
    thresholds: Vec<WorkshopThresholdResult>,
    energy_supplied: f32,
    calories_supplied: f32,
    land_used: f32,
    land_protected: f32,
    land_free: f32,
    contentedness: f32,
    history: Vec<WorkshopCycleRecord>,
}

fn workshop_debrief_data(state: &GameState) -> WorkshopDebriefData {
    let emissions = state.emissions.as_gtco2eq();
    let temperature = state.world.temperature;
    let extinction = state.world.extinction_rate;
    let produced = state.produced.total();
    let demand = state.output_demand.total();
    let energy_supplied = percent_demand_met(
        produced.fuel + produced.electricity,
        demand.fuel + demand.electricity,
    );
    let calories_supplied = percent_demand_met(
        produced.plant_calories + produced.animal_calories,
        demand.plant_calories + demand.animal_calories,
    );
    let total_land = state.world.starting_resources.land;
    let used_percent = if total_land <= 0. {
        0.
    } else {
        state.resource_demand.of(Resource::Land) / total_land * 100.
    };
    let (land_used, land_protected, land_free) =
        debrief_land_segments(used_percent, state.protected_land * 100.);
    let thresholds = vec![
        WorkshopThresholdResult {
            label: "CO2 emissions",
            value: format!("{emissions:+.1} GtCO2e/year"),
            target: "≤ 0 GtCO2e/year",
            passed: emissions <= WORKSHOP_WIN_EMISSIONS,
        },
        WorkshopThresholdResult {
            label: "Temperature anomaly",
            value: format!("{temperature:+.1} °C"),
            target: "≤ +1.0 °C",
            passed: temperature <= WORKSHOP_WIN_TEMPERATURE,
        },
        WorkshopThresholdResult {
            label: "Extinction rate",
            value: format!("{extinction:.1}"),
            target: "≤ 20",
            passed: extinction <= WORKSHOP_WIN_EXTINCTION,
        },
    ];

    WorkshopDebriefData {
        won: thresholds.iter().all(|threshold| threshold.passed),
        thresholds,
        energy_supplied,
        calories_supplied,
        land_used,
        land_protected,
        land_free,
        contentedness: state.outlook(),
        history: state.ui.workshop_policy_history.clone(),
    }
}

fn percent_demand_met(produced: f32, demand: f32) -> f32 {
    if demand <= 0. {
        100.
    } else {
        (produced / demand * 100.).round()
    }
}

fn debrief_land_segments(used_percent: f32, protected_percent: f32) -> (f32, f32, f32) {
    let used = used_percent.clamp(0., 100.);
    let protected = protected_percent.clamp(0., 100. - used);
    (used, protected, 100. - used - protected)
}

fn render_workshop_debrief(ui: &mut egui::Ui, debrief: &WorkshopDebriefData) -> bool {
    let mut restart = false;
    ui.vertical_centered(|ui| {
        ui.add_space(18.);
        let (verdict, verdict_color) = if debrief.won {
            ("WORKSHOP GOALS MET", Color32::from_rgb(0x7f, 0xf0, 0x92))
        } else {
            (
                "WORKSHOP GOALS NOT MET",
                Color32::from_rgb(0xff, 0x8a, 0x80),
            )
        };
        ui.label(
            egui::RichText::new("2052 FINAL DEBRIEF")
                .size(24.)
                .color(Color32::WHITE),
        );
        ui.label(
            egui::RichText::new(verdict)
                .size(34.)
                .strong()
                .color(verdict_color),
        );
        ui.add_space(12.);

        raised_frame().show(ui, |ui| {
            let width = (ui.ctx().content_rect().width() - 96.).clamp(760., 1080.);
            ui.set_width(width);
            ui.heading("Win thresholds");
            egui::Grid::new("workshop-final-thresholds")
                .num_columns(4)
                .striped(true)
                .spacing(egui::vec2(32., 8.))
                .show(ui, |ui| {
                    ["Metric", "Final value", "Target", "Status"]
                        .into_iter()
                        .for_each(|heading| {
                            ui.label(egui::RichText::new(heading).size(17.).strong());
                        });
                    ui.end_row();
                    debrief.thresholds.iter().for_each(|threshold| {
                        ui.label(egui::RichText::new(threshold.label).size(18.));
                        ui.label(egui::RichText::new(&threshold.value).size(18.).strong());
                        ui.label(egui::RichText::new(threshold.target).size(17.));
                        let (status, color) = if threshold.passed {
                            ("PASS", Color32::from_rgb(0x7f, 0xf0, 0x92))
                        } else {
                            ("FAIL", Color32::from_rgb(0xff, 0x8a, 0x80))
                        };
                        ui.colored_label(color, egui::RichText::new(status).size(18.).strong());
                        ui.end_row();
                    });
                });

            ui.add_space(12.);
            ui.heading("Final supply and land");
            ui.horizontal_wrapped(|ui| {
                ui.label(
                    egui::RichText::new(format!(
                        "Energy supplied: {:.0}%",
                        debrief.energy_supplied
                    ))
                    .size(18.)
                    .strong(),
                );
                ui.separator();
                ui.label(
                    egui::RichText::new(format!(
                        "Calories supplied: {:.0}%",
                        debrief.calories_supplied
                    ))
                    .size(18.)
                    .strong(),
                );
                ui.separator();
                ui.label(
                    egui::RichText::new(format!("Land used: {:.0}%", debrief.land_used)).size(18.),
                );
                ui.label(
                    egui::RichText::new(format!("Protected: {:.0}%", debrief.land_protected))
                        .size(18.),
                );
                ui.label(egui::RichText::new(format!("Free: {:.0}%", debrief.land_free)).size(18.));
            });
            ui.label(
                egui::RichText::new(format!(
                    "Secondary indicator — contentedness: {:.1}",
                    debrief.contentedness
                ))
                .size(15.),
            );

            ui.add_space(12.);
            ui.heading("Six-cycle policy history");
            debrief.history.iter().for_each(|cycle| {
                let choices = if cycle.choices.is_empty() {
                    "No policy changes".to_string()
                } else {
                    cycle
                        .choices
                        .iter()
                        .map(|choice| {
                            let action = match choice.action {
                                WorkshopPolicyAction::Passed => "Passed",
                                WorkshopPolicyAction::Repealed => "Repealed",
                            };
                            format!("{action}: {}", choice.name)
                        })
                        .collect::<Vec<_>>()
                        .join(" · ")
                };
                ui.label(
                    egui::RichText::new(format!(
                        "Cycle {} ({}–{}): {choices}",
                        cycle.cycle, cycle.start_year, cycle.end_year
                    ))
                    .size(16.),
                );
            });

            ui.add_space(12.);
            restart = ui
                .add(button("Start a new workshop").full_width())
                .clicked();
        });
    });
    restart
}

#[derive(Debug, Clone, Serialize, Deserialize)]
enum Ending {
    Win,
    Died,
    Coup,
    LostOther,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, EnumIter)]
enum Badge {
    Seceded,
    Aliens,
    Biodiversity,
    Electrification,
    Extinction,
    FossilFuels,
    Meat,
    Nuclear,
    Renewables,
    Space,
    Vegan,
}
impl Badge {
    fn applies(&self, state: &State) -> bool {
        match self {
            Self::Seceded => state.world.regions.iter().any(|reg| reg.seceded),
            Self::Aliens => state.flags.contains(&Flag::AlienEncounter),
            Self::Biodiversity => state.world.extinction_rate <= 15.,
            Self::Extinction => state.world.extinction_rate >= 60.,
            Self::Electrification => state.world.projects.iter().any(|proj| {
                proj.name == "Mass Electrification"
                    && (proj.status == Status::Finished || proj.status == Status::Active)
            }),
            Self::FossilFuels => {
                state
                    .world
                    .processes
                    .iter()
                    .filter(|proc| proc.features.contains(&ProcessFeature::IsFossil))
                    .map(|proc| proc.mix_share)
                    .sum::<usize>()
                    > 0
            }
            Self::Meat => {
                // Animal calories demand at least 80% of starting value
                state.output_demand.of(Output::AnimalCalories) >= 2e15
            }
            Self::Nuclear => {
                state
                    .world
                    .processes
                    .iter()
                    .filter(|proc| {
                        proc.features.contains(&ProcessFeature::CanMeltdown)
                            || proc.features.contains(&ProcessFeature::MakesNuclearWaste)
                    })
                    .map(|proc| proc.mix_share)
                    .sum::<usize>()
                    >= 10
            }
            Self::Renewables => {
                state
                    .world
                    .processes
                    .iter()
                    .filter(|proc| proc.features.contains(&ProcessFeature::IsIntermittent))
                    .map(|proc| proc.mix_share)
                    .sum::<usize>()
                    >= 10
            }
            Self::Space => {
                state
                    .world
                    .projects
                    .iter()
                    .filter(|proj| {
                        proj.group == Group::Space
                            && (proj.status == Status::Finished || proj.status == Status::Active)
                    })
                    .count()
                    >= 3
            }
            Self::Vegan => {
                // Animal calories demand down to less than 10% of starting val
                state.output_demand.of(Output::AnimalCalories) < 2e14
            }
        }
    }

    fn icon(&self) -> Icon {
        match self {
            Self::Seceded => icons::BADGE_SECEDED,
            Self::Aliens => icons::BADGE_ALIENS,
            Self::Biodiversity => icons::BADGE_BIODIVERSITY,
            Self::Electrification => icons::BADGE_ELECTRIFICATION,
            Self::Extinction => icons::BADGE_EXTINCTION,
            Self::FossilFuels => icons::BADGE_FOSSILFUELS,
            Self::Meat => icons::BADGE_MEAT,
            Self::Nuclear => icons::BADGE_NUCLEAR,
            Self::Renewables => icons::BADGE_RENEWABLES,
            Self::Space => icons::BADGE_SPACE,
            Self::Vegan => icons::BADGE_VEGAN,
        }
    }
}
impl std::fmt::Display for Badge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let desc = match self {
            Self::Seceded => "At least one region seceded from Gosplant.",
            Self::Aliens => "You had an extraterrestrial encounter.",
            Self::Biodiversity => "Planetary life flourished under your tenure.",
            Self::Electrification => "You helped electrify the world.",
            Self::Extinction => "Planetary life suffered under your tenure.",
            Self::FossilFuels => "You kept on using fossil fuels.",
            Self::Meat => "Carnivorous diets were left intact.",
            Self::Nuclear => "Nuclear was your preferred form of energy.",
            Self::Renewables => "Renewables dominated energy production.",
            Self::Space => "You pushed humanity towards the stars.",
            Self::Vegan => "Global diets shifted towards vegan.",
        };
        write!(f, "{}", desc)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Summary {
    pub ending: Ending,
    pub faction: String,
    pub badges: Vec<Badge>,
}

fn eval_badges(state: &State) -> Vec<Badge> {
    Badge::iter().filter(|badge| badge.applies(state)).collect()
}

fn summarize(state: &State, win: bool) -> Summary {
    let badges = eval_badges(state);
    let closest = state
        .npcs
        .iter()
        .max_by(|x, y| {
            x.relationship
                .partial_cmp(&y.relationship)
                .unwrap_or(Ordering::Equal)
        })
        .unwrap();
    let faction = closest.name.to_string();

    Summary {
        badges,
        faction,
        ending: if win {
            Ending::Win
        } else if state.world.year >= state.death_year {
            Ending::Died
        } else if state.political_capital <= 0 {
            Ending::Coup
        } else {
            Ending::LostOther
        },
    }
}

fn format_year_log(
    year: usize,
    changes: &[Change],
    mixes: &EnumMap<Output, BTreeMap<String, usize>>,
) -> String {
    [
        format!("\n[{year}]"),
        changes
            .iter()
            .map(|diff| diff.to_string())
            .collect::<Vec<_>>()
            .join("\n"),
        "Production Mix:".into(),
        mixes
            .iter()
            .map(|(output, mix)| {
                let mut parts = vec![format!("  [{output}]")];
                for (name, mix) in mix {
                    parts.push(format!("    {name}:{mix}"));
                }
                parts.join("\n")
            })
            .collect::<Vec<_>>()
            .join("\n"),
    ]
    .join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{WorkshopPolicyAction, WorkshopPolicyChoice};

    fn debrief_fixture(emissions: f32, temperature: f32, extinction: f32) -> GameState {
        let mut state = GameState::from_world(World::workshop());
        state.core.emissions.co2 = emissions * 1e15;
        state.core.emissions.ch4 = 0.;
        state.core.emissions.n2o = 0.;
        state.core.world.temperature = temperature;
        state.core.world.extinction_rate = extinction;
        state.core.produced.amount = OutputMap {
            fuel: 30.,
            electricity: 50.,
            plant_calories: 60.,
            animal_calories: 40.,
        };
        state.core.output_demand.base = OutputMap {
            fuel: 50.,
            electricity: 50.,
            plant_calories: 50.,
            animal_calories: 50.,
        };
        state.core.world.starting_resources.land = 100.;
        state.core.resource_demand.base.land = 40.;
        state.core.protected_land = 0.25;
        state.ui.workshop_policy_history = (0..6)
            .map(|index| WorkshopCycleRecord {
                cycle: index + 1,
                start_year: 2022 + index * 5,
                end_year: 2027 + index * 5,
                choices: vec![WorkshopPolicyChoice {
                    name: format!("Policy {}", index + 1),
                    action: if index == 5 {
                        WorkshopPolicyAction::Repealed
                    } else {
                        WorkshopPolicyAction::Passed
                    },
                }],
            })
            .collect();
        state
    }

    #[test]
    fn workshop_win_fixture_has_exact_debrief_data() {
        let state = debrief_fixture(-1., 0.8, 15.);

        let debrief = workshop_debrief_data(&state);

        assert!(debrief.won);
        assert_eq!(
            debrief.thresholds,
            vec![
                WorkshopThresholdResult {
                    label: "CO2 emissions",
                    value: "-1.0 GtCO2e/year".into(),
                    target: "≤ 0 GtCO2e/year",
                    passed: true,
                },
                WorkshopThresholdResult {
                    label: "Temperature anomaly",
                    value: "+0.8 °C".into(),
                    target: "≤ +1.0 °C",
                    passed: true,
                },
                WorkshopThresholdResult {
                    label: "Extinction rate",
                    value: "15.0".into(),
                    target: "≤ 20",
                    passed: true,
                },
            ]
        );
        assert_eq!(debrief.energy_supplied, 80.);
        assert_eq!(debrief.calories_supplied, 100.);
        assert_eq!(
            (debrief.land_used, debrief.land_protected, debrief.land_free),
            (40., 25., 35.)
        );
        assert_eq!(debrief.history.len(), 6);
        assert_eq!(debrief.history[5].end_year, 2052);
        assert_eq!(
            debrief.history[5].choices[0].action,
            WorkshopPolicyAction::Repealed
        );
    }

    #[test]
    fn workshop_fail_fixture_marks_each_failed_threshold() {
        let state = debrief_fixture(2., 1.2, 25.);

        let debrief = workshop_debrief_data(&state);

        assert!(!debrief.won);
        assert_eq!(
            debrief
                .thresholds
                .iter()
                .map(|threshold| threshold.passed)
                .collect::<Vec<_>>(),
            vec![false, false, false]
        );
        assert_eq!(debrief.thresholds[0].value, "+2.0 GtCO2e/year");
        assert_eq!(debrief.thresholds[1].value, "+1.2 °C");
        assert_eq!(debrief.thresholds[2].value, "25.0");
    }

    #[test]
    fn normal_ending_summary_retains_faction_result_path() {
        let state = State::default();

        let summary = summarize(&state, true);

        assert!(matches!(summary.ending, Ending::Win));
        assert!(!summary.faction.is_empty());
        assert_eq!(
            summary
                .badges
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>(),
            eval_badges(&state)
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
        );
    }
}
