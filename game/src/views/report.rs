use egui::Color32;
use egui_extras::{Column, TableBuilder};
use egui_taffy::TuiBuilderLogic;
use hes_engine::{EventPhase, IconEvent, Income, NPCRequest, OutputMap, Project, Resource};
use rust_i18n::t;

use crate::{
    consts,
    display::{
        self, AsText, DisplayValue, Icon,
        factors::factors_card,
        icons,
        intensity::{self, IntensityBar, intensity_bar},
    },
    parts::{button, center_center, center_text, raised_frame, set_full_bg_image},
    state::{GameState, StateExt, WorkshopCycleRecord, WorkshopPolicyAction, WorkshopPolicyChoice},
    tips::{Tip, add_tip, tip},
    vars::Var,
    views::events::Events,
    workshop::WORKSHOP,
};

const ROW_HEIGHT: f32 = 18.;
const WORKSHOP_ROW_HEIGHT: f32 = 36.;

pub struct Report {
    events: Events,
    changes: Vec<ChangeRow>,
    /// Workshop mode: metrics tracked but demoted below the four
    /// headline metrics (currently just contentedness).
    secondary_changes: Vec<ChangeRow>,
    /// Workshop mode: water is warning-only — set when demand
    /// exceeds the available supply at the end of the cycle.
    water_warning: Option<String>,
    projects_finished: Vec<Project>,
    requests_fulfilled: Vec<(String, isize)>,
    seat_changes: Vec<(String, f32, f32)>,
    world_events: Vec<(String, Tip)>,
    disasters: Vec<(String, Vec<IconEvent>)>,
    region_incomes: Vec<(String, Income)>,
    honeymoon_pc: isize,
    pc_change: isize,
    workshop: Option<WorkshopReportData>,
}
impl Report {
    pub fn new(state: &mut GameState) -> Self {
        let events = StateExt::roll_events(&mut state.core, EventPhase::ReportStart);

        let workshop = WORKSHOP.active.then(|| build_workshop_report_data(state));

        state.ui.session_start_state = state.core.clone();

        // Workshop mode re-centers the report on the four headline
        // metrics (spec §6): CO2/temperature, biodiversity, energy
        // vs demand, calories vs demand. Contentedness is demoted to
        // a secondary line and water is warning-only.
        let changes = if WORKSHOP.active {
            vec![
                ghg_row(state),
                temp_row(state),
                ext_row(state),
                energy_row(state),
                calories_row(state),
            ]
        } else {
            vec![
                temp_row(state),
                cont_row(state),
                ext_row(state),
                ghg_row(state),
            ]
        };
        let secondary_changes = if WORKSHOP.active {
            vec![cont_row(state)]
        } else {
            vec![]
        };
        let water_warning = if WORKSHOP.active {
            water_warning(state)
        } else {
            None
        };

        let requests = requests_rows(state);

        // Workshop mode: PC is a flat expiring budget, not earned
        // from outcomes, so no PC is awarded (or displayed) here.
        let honeymoon_pc = if WORKSHOP.active {
            0
        } else {
            honeymoon_pc(state)
        };
        let pc_change = if WORKSHOP.active {
            0
        } else {
            changes.iter().map(|row| row.pc_change).sum::<isize>()
                + requests.iter().map(|(_, bounty)| bounty).sum::<isize>()
                + (state.ui.cycle_start_state.completed_projects.len()
                    * consts::PC_PER_COMPLETED_PROJECT) as isize
                + honeymoon_pc
        };

        Self {
            events: Events::new(events, state),
            changes,
            secondary_changes,
            water_warning,
            projects_finished: projects_rows(state),
            requests_fulfilled: requests,
            seat_changes: parliament_rows(state),
            world_events: event_rows(state),
            disasters: disaster_rows(state),
            region_incomes: region_rows(state),
            honeymoon_pc,
            pc_change,
            workshop,
        }
    }

    pub fn render(&mut self, ui: &mut egui::Ui, state: &mut GameState) -> bool {
        let mut done = false;
        set_full_bg_image(
            ui,
            hes_images::background_image("report.png"),
            egui::vec2(1600., 1192.),
        );

        self.events.render(ui, &mut state.core);

        if let Some(workshop) = &self.workshop {
            let done = render_workshop_report(ui, workshop, self.water_warning.as_deref());
            if done {
                state.ui.plan_changes.clear();
                state.ui.points.refundable_research.clear();
            }
            return done;
        }

        center_center(ui, "report", |tui| {
            tui.ui(|ui| {
                ui.style_mut().visuals.override_text_color = Some(Color32::BLACK);
                ui.add(
                    center_text(t!("Report"))
                        .family(egui::FontFamily::Name("TimesTen".into()))
                        .size(24.),
                );
                ui.add_space(8.);

                raised_frame()
                    .colors(
                        Color32::from_rgb(0xf7, 0xf4, 0xe6),
                        Color32::from_rgb(0xc2, 0xb8, 0x93),
                        Color32::from_rgb(0xFF, 0xF7, 0xD9),
                    )
                    .show(ui, |ui| {
                        ui.set_width(360.);

                        self.render_changes(ui, state);
                        self.render_secondary_changes(ui);
                        self.render_water_warning(ui);
                        self.render_projects(ui);
                        self.render_requests(ui);
                        self.render_total_pc_change(ui);

                        self.render_seat_changes(ui);
                        self.render_world_events(ui);
                        self.render_region_incomes(ui);
                        self.render_disasters(ui);

                        ui.add_space(16.);

                        if ui.add(button(t!("Next")).full_width()).clicked() {
                            state.change_political_capital(self.pc_change);

                            // Reset session plan changes
                            state.ui.plan_changes.clear();
                            state.ui.points.refundable_research.clear();

                            done = true;
                        }
                    });
            });
        });
        ui.add_space(32.);
        done
    }

    fn render_changes(&self, ui: &mut egui::Ui, state: &GameState) {
        let year = state.world.year;
        let start_year = state.ui.cycle_start_state.year;

        // Workshop mode hides the PC-award column entirely.
        let show_pc = !WORKSHOP.active;

        let mut table = TableBuilder::new(ui)
            .id_salt("changes")
            .column(Column::remainder())
            .column(Column::auto())
            .column(Column::auto())
            .column(Column::auto());
        if show_pc {
            table = table.column(Column::auto());
        }
        table
            .header(ROW_HEIGHT, |mut header| {
                header.col(|ui| {
                    ui.label(egui::RichText::new(t!("Changes")).size(12.).underline());
                });
                header.col(|ui| {
                    ui.label(egui::RichText::new(start_year.to_string()).size(12.));
                });
                header.col(|ui| {
                    ui.add(icons::ARROW_RIGHT.size(12.));
                });
                header.col(|ui| {
                    ui.label(egui::RichText::new(year.to_string()).size(12.));
                });
                if show_pc {
                    header.col(|ui| {
                        ui.add(icons::POLITICAL_CAPITAL.size(12.));
                    });
                }
            })
            .body(|mut body| {
                for change in &self.changes {
                    body.row(ROW_HEIGHT, |mut row| {
                        row.col(|ui| {
                            add_tip(
                                change.tip.clone(),
                                ui.horizontal(|ui| {
                                    ui.add(change.icon.size(16.));
                                    ui.label(&change.label);
                                })
                                .response,
                            );
                        });
                        row.col(|ui| {
                            match &change.from {
                                Value::Bar(intensity_bar) => {
                                    ui.add(intensity_bar);
                                }
                                Value::Val(val) => {
                                    ui.label(val);
                                }
                            };
                        });
                        row.col(|ui| {
                            ui.add(icons::ARROW_RIGHT.size(16.));
                        });
                        row.col(|ui| {
                            match &change.to {
                                Value::Bar(intensity_bar) => {
                                    ui.add(intensity_bar);
                                }
                                Value::Val(val) => {
                                    ui.label(val);
                                }
                            };
                        });
                        if show_pc {
                            row.col(|ui| {
                                let pc_change = format!("{:+}", change.pc_change);
                                ui.label(pc_change);
                            });
                        }
                    });
                }
            });
    }

    /// Workshop mode: metrics demoted below the headline four,
    /// shown as a plain before/after list without the year header.
    fn render_secondary_changes(&self, ui: &mut egui::Ui) {
        if self.secondary_changes.is_empty() {
            return;
        }
        ui.add_space(12.);
        TableBuilder::new(ui)
            .id_salt("secondary-changes")
            .column(Column::remainder())
            .column(Column::auto())
            .column(Column::auto())
            .column(Column::auto())
            .header(ROW_HEIGHT, |mut header| {
                header.col(|ui| {
                    ui.label(
                        egui::RichText::new(t!("Also Tracked"))
                            .size(12.)
                            .underline(),
                    );
                });
            })
            .body(|mut body| {
                for change in &self.secondary_changes {
                    body.row(ROW_HEIGHT, |mut row| {
                        row.col(|ui| {
                            add_tip(
                                change.tip.clone(),
                                ui.horizontal(|ui| {
                                    ui.add(change.icon.size(16.));
                                    ui.label(&change.label);
                                })
                                .response,
                            );
                        });
                        row.col(|ui| match &change.from {
                            Value::Bar(intensity_bar) => {
                                ui.add(intensity_bar);
                            }
                            Value::Val(val) => {
                                ui.label(val);
                            }
                        });
                        row.col(|ui| {
                            ui.add(icons::ARROW_RIGHT.size(16.));
                        });
                        row.col(|ui| match &change.to {
                            Value::Bar(intensity_bar) => {
                                ui.add(intensity_bar);
                            }
                            Value::Val(val) => {
                                ui.label(val);
                            }
                        });
                    });
                }
            });
    }

    /// Workshop mode: water is warning-only — a single alert line
    /// shown only when there is a shortage.
    fn render_water_warning(&self, ui: &mut egui::Ui) {
        if let Some(warning) = &self.water_warning {
            ui.add_space(12.);
            ui.horizontal(|ui| {
                ui.add(icons::ALERT.size(16.));
                ui.colored_label(
                    Color32::from_rgb(0xB0, 0x14, 0x0C),
                    egui::RichText::new(warning).size(12.),
                );
            });
        }
    }

    fn render_projects(&self, ui: &mut egui::Ui) {
        if !self.projects_finished.is_empty() {
            ui.add_space(12.);

            TableBuilder::new(ui)
                .id_salt("projects")
                .column(Column::remainder())
                .column(Column::auto())
                .body(|mut body| {
                    body.row(ROW_HEIGHT, |mut row| {
                        row.col(|ui| {
                            ui.label(
                                egui::RichText::new(t!("Completed Projects"))
                                    .size(12.)
                                    .underline(),
                            );
                        });
                    });

                    for p in &self.projects_finished {
                        body.row(ROW_HEIGHT, |mut row| {
                            row.col(|ui| {
                                let tip = tip(icons::PROJECT, t!("This project was completed."))
                                    .card(p.clone());
                                add_tip(tip, ui.label(t!(&p.name)));
                            });
                            if !WORKSHOP.active {
                                row.col(|ui| {
                                    let pc_change =
                                        format!("{:+}", consts::PC_PER_COMPLETED_PROJECT);
                                    ui.label(pc_change);
                                });
                            }
                        });
                    }
                });
        }
    }

    fn render_requests(&self, ui: &mut egui::Ui) {
        if !self.requests_fulfilled.is_empty() {
            TableBuilder::new(ui)
                .id_salt("requests")
                .column(Column::remainder())
                .column(Column::auto())
                .body(|mut body| {
                    body.row(ROW_HEIGHT, |mut row| {
                        row.col(|ui| {
                            ui.label(
                                egui::RichText::new(t!("Completed Requests"))
                                    .size(12.)
                                    .underline(),
                            );
                        });
                    });

                    for (name, bounty) in &self.requests_fulfilled {
                        body.row(ROW_HEIGHT, |mut row| {
                            row.col(|ui| {
                                ui.label(name);
                            });
                            row.col(|ui| {
                                let pc_change = format!("{:+}", bounty);
                                ui.label(pc_change);
                            });
                        });
                    }
                });
        }
    }

    fn render_total_pc_change(&self, ui: &mut egui::Ui) {
        // Workshop mode: no PC awards, so no total to show.
        if WORKSHOP.active {
            return;
        }
        ui.add_space(12.);
        TableBuilder::new(ui)
            .id_salt("pc-total")
            .column(Column::remainder())
            .column(Column::auto())
            .body(|mut body| {
                if self.honeymoon_pc > 0 {
                    body.row(ROW_HEIGHT, |mut row| {
                        row.col(|ui| {
                            ui.label(t!("Post-Revolution Optimism"));
                        });
                        row.col(|ui| {
                            let pc_change = format!("{:+}", self.honeymoon_pc);
                            ui.label(pc_change);
                        });
                    });
                }

                body.row(ROW_HEIGHT, |mut row| {
                    row.col(|ui| {
                        ui.horizontal(|ui| {
                            ui.add(icons::POLITICAL_CAPITAL.size(16.));
                            ui.label(t!("Total Change"));
                        });
                    });
                    row.col(|ui| {
                        ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Extend);
                        let pc_change = format!("{:+}", self.pc_change);
                        ui.label(pc_change);
                    });
                });
            });
    }

    fn render_seat_changes(&self, ui: &mut egui::Ui) {
        if !self.seat_changes.is_empty() {
            ui.add_space(16.);
            TableBuilder::new(ui)
                .id_salt("parliament")
                .column(Column::remainder())
                .column(Column::auto())
                .column(Column::auto())
                .body(|mut body| {
                    body.row(ROW_HEIGHT, |mut row| {
                        row.col(|ui| {
                            ui.label(egui::RichText::new(t!("Parliament")).size(12.).underline());
                        });
                    });

                    for (name, seats, change) in &self.seat_changes {
                        body.row(ROW_HEIGHT, |mut row| {
                            row.col(|ui| {
                                ui.label(name);
                            });
                            row.col(|ui| {
                                let change = format!("{:+}", change);
                                ui.label(change);
                            });
                            row.col(|ui| {
                                ui.label(seats.to_string());
                            });
                        });
                    }
                });
        }
    }

    fn render_world_events(&self, ui: &mut egui::Ui) {
        if !self.world_events.is_empty() {
            ui.add_space(16.);
            TableBuilder::new(ui)
                .id_salt("events")
                .column(Column::remainder())
                .body(|mut body| {
                    body.row(ROW_HEIGHT, |mut row| {
                        row.col(|ui| {
                            ui.label(egui::RichText::new(t!("Events")).size(12.).underline());
                        });
                    });

                    for (name, tip) in &self.world_events {
                        body.row(ROW_HEIGHT, |mut row| {
                            let (_, resp) = row.col(|ui| {
                                ui.label(name);
                            });
                            add_tip(tip.clone(), resp);
                        });
                    }
                });
        }
    }

    fn render_region_incomes(&self, ui: &mut egui::Ui) {
        if !self.region_incomes.is_empty() {
            ui.add_space(16.);
            TableBuilder::new(ui)
                .id_salt("regions")
                .column(Column::remainder())
                .body(|mut body| {
                    body.row(ROW_HEIGHT, |mut row| {
                        row.col(|ui| {
                            ui.label(egui::RichText::new(t!("Regions")).size(12.).underline());
                        });
                    });

                    for (name, income) in &self.region_incomes {
                        body.row(ROW_HEIGHT, |mut row| {
                            row.col(|ui| {
                                ui.label(t!(
                                    "%{region} is now %{income} income.",
                                    region = t!(name.as_str()),
                                    income = t!(income.lower())
                                ));
                            });
                        });
                    }
                });
        }
    }

    fn render_disasters(&self, ui: &mut egui::Ui) {
        if !self.disasters.is_empty() {
            ui.add_space(16.);
            TableBuilder::new(ui)
                .id_salt("disasters")
                .column(Column::auto().at_least(140.))
                .column(Column::auto())
                .body(|mut body| {
                    body.row(ROW_HEIGHT, |mut row| {
                        row.col(|ui| {
                            ui.label(egui::RichText::new(t!("Disasters")).size(12.).underline());
                        });

                        row.col(|ui| {
                            ui.horizontal(|ui| {
                                ui.add(icons::HABITABILITY.size(16.));
                                ui.label(
                                    egui::RichText::new(t!("Reduce the habitability of regions."))
                                        .size(12.),
                                );
                            });
                        });
                    });

                    for (name, events) in &self.disasters {
                        body.row(ROW_HEIGHT, |mut row| {
                            row.col(|ui| {
                                ui.label(t!(name.as_str()));
                            });

                            row.col(|ui| {
                                ui.horizontal(|ui| {
                                    for ev in events {
                                        ui.add(icons::disaster_icon(&ev.icon).size(16.));
                                    }
                                });
                            });
                        });
                    }
                });
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MetricDirection {
    Improved,
    Worsened,
    Unchanged,
}

#[derive(Debug, Clone, PartialEq)]
struct WorkshopMetricRow {
    label: &'static str,
    from: String,
    to: String,
    target: &'static str,
    direction: MetricDirection,
    shortage: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
struct WorkshopReportData {
    cycle: WorkshopCycleRecord,
    metrics: Vec<WorkshopMetricRow>,
    contentedness: String,
}

fn build_workshop_report_data(state: &mut GameState) -> WorkshopReportData {
    let choices = state
        .ui
        .plan_changes
        .iter()
        .filter_map(|(id, change)| {
            let action = if change.passed {
                Some(WorkshopPolicyAction::Passed)
            } else if change.withdrawn {
                Some(WorkshopPolicyAction::Repealed)
            } else {
                None
            }?;
            Some(WorkshopPolicyChoice {
                name: state.world.projects[id].name.clone(),
                action,
            })
        })
        .collect::<Vec<_>>();
    let start_year = state.ui.cycle_start_state.year;
    let end_year = state.world.year;
    let cycle = state
        .ui
        .record_workshop_cycle(start_year, end_year, choices);

    workshop_report_data(state, cycle)
}

fn workshop_report_data(state: &GameState, cycle: WorkshopCycleRecord) -> WorkshopReportData {
    let start = &state.ui.cycle_start_state;
    let end_produced = state.produced.total();
    let end_demand = state.output_demand.total();
    let start_energy = energy_percent_met(start.produced, start.output_demand);
    let end_energy = energy_percent_met(end_produced, end_demand);
    let start_calories = calories_percent_met(start.produced, start.output_demand);
    let end_calories = calories_percent_met(end_produced, end_demand);
    let emissions = state.emissions.as_gtco2eq();

    WorkshopReportData {
        cycle,
        metrics: vec![
            lower_is_better_row(
                "CO2 emissions",
                start.emissions,
                emissions,
                |value| format!("{value:+.1} GtCO2e/year"),
                "≤ 0 GtCO2e/year",
            ),
            lower_is_better_row(
                "Temperature anomaly",
                start.temperature,
                state.world.temperature,
                |value| format!("{value:+.1} °C"),
                "≤ +1.0 °C",
            ),
            lower_is_better_row(
                "Extinction rate",
                start.extinction_rate,
                state.world.extinction_rate,
                |value| format!("{value:.1}"),
                "≤ 20",
            ),
            supply_row("Energy supplied", start_energy, end_energy),
            supply_row("Calories supplied", start_calories, end_calories),
        ],
        contentedness: format!("{:.1}", state.outlook()),
    }
}

fn lower_is_better_row(
    label: &'static str,
    from: f32,
    to: f32,
    format_value: impl Fn(f32) -> String,
    target: &'static str,
) -> WorkshopMetricRow {
    WorkshopMetricRow {
        label,
        from: format_value(from),
        to: format_value(to),
        target,
        direction: compare_metric(from, to, false),
        shortage: None,
    }
}

fn supply_row(label: &'static str, from: f32, to: f32) -> WorkshopMetricRow {
    WorkshopMetricRow {
        label,
        from: format!("{from:.0}%"),
        to: format!("{to:.0}%"),
        target: "≥ 100%",
        direction: compare_metric(from, to, true),
        shortage: (to < 100.).then(|| format!("Shortage: only {to:.0}% of demand is supplied.")),
    }
}

fn compare_metric(from: f32, to: f32, higher_is_better: bool) -> MetricDirection {
    let delta = to - from;
    if delta.abs() < 0.05 {
        MetricDirection::Unchanged
    } else if (delta > 0.) == higher_is_better {
        MetricDirection::Improved
    } else {
        MetricDirection::Worsened
    }
}

fn render_workshop_report(
    ui: &mut egui::Ui,
    report: &WorkshopReportData,
    water_warning: Option<&str>,
) -> bool {
    let mut done = false;
    ui.vertical_centered(|ui| {
        ui.style_mut().visuals.override_text_color = Some(Color32::BLACK);
        ui.add_space(20.);
        ui.label(
            egui::RichText::new(format!("Cycle {} report", report.cycle.cycle))
                .family(egui::FontFamily::Name("TimesTen".into()))
                .size(34.),
        );
        ui.label(
            egui::RichText::new(format!(
                "{} → {}",
                report.cycle.start_year, report.cycle.end_year
            ))
            .size(21.),
        );
        ui.add_space(12.);

        raised_frame()
            .colors(
                Color32::from_rgb(0xff, 0xff, 0xf8),
                Color32::from_rgb(0x8b, 0x73, 0x45),
                Color32::from_rgb(0xff, 0xfb, 0xe8),
            )
            .show(ui, |ui| {
                let width = (ui.ctx().content_rect().width() - 96.).clamp(760., 1080.);
                ui.set_width(width);
                ui.label(
                    egui::RichText::new("Choices this cycle")
                        .size(27.)
                        .strong()
                        .color(Color32::BLACK),
                );
                if report.cycle.choices.is_empty() {
                    ui.label(
                        egui::RichText::new("No policies were passed or repealed.")
                            .size(18.)
                            .color(Color32::BLACK),
                    );
                } else {
                    ui.horizontal_wrapped(|ui| {
                        report.cycle.choices.iter().for_each(|choice| {
                            let action = match choice.action {
                                WorkshopPolicyAction::Passed => "Passed",
                                WorkshopPolicyAction::Repealed => "Repealed",
                            };
                            ui.label(
                                egui::RichText::new(format!("{action}: {}", choice.name))
                                    .size(18.)
                                    .strong()
                                    .color(Color32::BLACK),
                            );
                        });
                    });
                }

                ui.add_space(14.);
                ui.label(
                    egui::RichText::new("World change")
                        .size(27.)
                        .strong()
                        .color(Color32::BLACK),
                );
                egui::Grid::new("workshop-report-metrics")
                    .num_columns(5)
                    .striped(true)
                    .min_row_height(WORKSHOP_ROW_HEIGHT)
                    .spacing(egui::vec2(24., 6.))
                    .show(ui, |ui| {
                        ["Metric", "Start", "End", "Direction", "Target"]
                            .into_iter()
                            .for_each(|heading| {
                                ui.label(
                                    egui::RichText::new(heading)
                                        .size(18.)
                                        .strong()
                                        .color(Color32::BLACK),
                                );
                            });
                        ui.end_row();

                        report.metrics.iter().for_each(|metric| {
                            ui.label(
                                egui::RichText::new(metric.label)
                                    .size(19.)
                                    .strong()
                                    .color(Color32::BLACK),
                            );
                            ui.label(
                                egui::RichText::new(&metric.from)
                                    .size(18.)
                                    .color(Color32::BLACK),
                            );
                            ui.label(
                                egui::RichText::new(&metric.to)
                                    .size(18.)
                                    .strong()
                                    .color(Color32::BLACK),
                            );
                            let (direction, color) = match metric.direction {
                                MetricDirection::Improved => {
                                    ("Improved", Color32::from_rgb(0x0b, 0x65, 0x2d))
                                }
                                MetricDirection::Worsened => {
                                    ("Worsened", Color32::from_rgb(0xa6, 0x11, 0x0b))
                                }
                                MetricDirection::Unchanged => ("No change", Color32::DARK_GRAY),
                            };
                            ui.colored_label(
                                color,
                                egui::RichText::new(direction).size(17.).strong(),
                            );
                            ui.label(
                                egui::RichText::new(metric.target)
                                    .size(18.)
                                    .color(Color32::BLACK),
                            );
                            ui.end_row();
                            if let Some(shortage) = &metric.shortage {
                                ui.label(
                                    egui::RichText::new(shortage)
                                        .size(15.)
                                        .strong()
                                        .color(Color32::from_rgb(0xa6, 0x11, 0x0b)),
                                );
                                ui.end_row();
                            }
                        });
                    });

                if let Some(warning) = water_warning {
                    ui.colored_label(
                        Color32::from_rgb(0xa6, 0x11, 0x0b),
                        egui::RichText::new(warning).size(16.).strong(),
                    );
                }
                ui.label(
                    egui::RichText::new(format!(
                        "Secondary indicator — contentedness: {}",
                        report.contentedness
                    ))
                    .size(17.)
                    .color(Color32::BLACK),
                );
                ui.add_space(12.);
                done = ui.add(button(t!("Next")).full_width()).clicked();
            });
    });
    done
}

struct ChangeRow {
    icon: Icon,
    label: String,
    from: Value,
    to: Value,
    tip: Tip,
    pc_change: isize,
}
enum Value {
    Bar(IntensityBar),
    Val(String),
}

fn ext_row(state: &GameState) -> ChangeRow {
    let tip_text = t!(
        r#"The current biodiversity pressure. High land use and other factors increase this, and with it, the risk of ecological collapse. [g]Your goal is to get this to below 20.[/g]"#
    );
    let ext_tip =
        tip(icons::EXTINCTION_RATE, tip_text).card(factors_card(None, Var::Biodiversity, state));

    let exr = state.world.extinction_rate;
    let start_exr = state.ui.cycle_start_state.extinction_rate;
    let start_exr_int = intensity::scale(start_exr, intensity::Variable::Extinction);
    let end_exr_int = intensity::scale(exr, intensity::Variable::Extinction);

    let ext_pc_change = {
        let change = start_exr - exr;
        let end = end_exr_int;
        consts::EXTINCTION_PC
            .get(end)
            .unwrap_or_else(|| consts::EXTINCTION_PC.last().unwrap())
            + (change.round() as isize * consts::BIODIVERSITY_PC).max(0)
    };

    ChangeRow {
        tip: ext_tip,
        icon: icons::EXTINCTION_RATE,
        label: t!("Extinction Rate").to_string(),
        from: Value::Bar(intensity_bar(start_exr_int)),
        to: Value::Bar(intensity_bar(end_exr_int)),
        pc_change: ext_pc_change,
    }
}

fn cont_row(state: &GameState) -> ChangeRow {
    let tip_text = t!(
        r#"How people around the world feel about the state of things. This is a combination of regional contentedness, crises, and policy decisions. [w]If this goes below 0 you will be removed from power.[/w]"#
    );
    let cont_tip =
        tip(icons::CONTENTEDNESS, tip_text).card(factors_card(None, Var::Contentedness, state));

    let outlook = state.outlook();
    let start_outlook = state.ui.cycle_start_state.contentedness;
    let start_cont_int = intensity::scale(start_outlook, intensity::Variable::WorldOutlook);
    let end_cont_int = intensity::scale(outlook, intensity::Variable::WorldOutlook);

    let cont_pc_change = {
        let end = end_cont_int;
        consts::CONTENTEDNESS_PC
            .get(end)
            .unwrap_or_else(|| consts::CONTENTEDNESS_PC.last().unwrap())
    };

    ChangeRow {
        tip: cont_tip,
        icon: icons::CONTENTEDNESS,
        label: t!("Contentedness").to_string(),
        from: Value::Bar(intensity_bar(start_cont_int).invert()),
        to: Value::Bar(intensity_bar(end_cont_int).invert()),
        pc_change: *cont_pc_change,
    }
}

fn temp_row(state: &GameState) -> ChangeRow {
    let temp = state.world.temperature;
    let start_temp = state.ui.cycle_start_state.temperature;
    let temp_change = temp - start_temp;
    let temp_pc_change = {
        // Double temp change score for every degree above 1C
        let temp_change_multiplier = ((temp.round() - 1.).max(0.) * 2.).max(1.);

        // Temp scored for every 0.1C change
        let change =
            (temp_change * 10.).round() * -(consts::TEMPERATURE_PC as f32) * temp_change_multiplier;
        change as isize
    };

    let start = display::temp(start_temp);
    let end = display::temp(temp);

    let warming_tip = tip(
        icons::WARMING,
        t!(
            r#"The current global temperature anomaly. [b]Increased warming[/b] will damage your political capital. [g]Your goal is to get this below 1°C.[/g]"#
        ),
    );

    ChangeRow {
        tip: warming_tip,
        icon: icons::WARMING,
        label: t!("Temperature").to_string(),
        from: Value::Val(start),
        to: Value::Val(end),
        pc_change: temp_pc_change,
    }
}

fn ghg_row(state: &GameState) -> ChangeRow {
    let emissions_gt = state.emissions.display();
    let tip_text = t!(
        r#"Current annual emissions are %{emissions}. [g]Your goal is to get this to below 0.[/g]"#,
        emissions = emissions_gt
    );
    let emissions_tip =
        tip(icons::EMISSIONS, tip_text).card(factors_card(None, Var::Emissions, state));

    let emissions = state.emissions.as_gtco2eq();
    let start_emissions = state.ui.cycle_start_state.emissions;
    let ghg_pc_change = {
        let emissions_change = emissions - start_emissions;
        (emissions_change * 2.).round() as isize * -consts::EMISSIONS_PC
    };

    let start = format!("{:+.1}", start_emissions);
    let end = format!("{:+.1}", emissions);

    ChangeRow {
        tip: emissions_tip,
        icon: icons::EMISSIONS,
        label: t!("Emissions").to_string(),
        from: Value::Val(start),
        to: Value::Val(end),
        pc_change: ghg_pc_change,
    }
}

/// Percent of demand met, rounded to the nearest whole percent.
/// No demand counts as fully met.
fn percent_demand_met(produced: f32, demand: f32) -> f32 {
    if demand <= 0. {
        100.
    } else {
        (produced / demand * 100.).round()
    }
}

/// Percent of energy demand (fuel + electricity) met.
fn energy_percent_met(produced: OutputMap, demand: OutputMap) -> f32 {
    percent_demand_met(
        produced.fuel + produced.electricity,
        demand.fuel + demand.electricity,
    )
}

/// Percent of calorie demand (plant + animal) met.
fn calories_percent_met(produced: OutputMap, demand: OutputMap) -> f32 {
    percent_demand_met(
        produced.plant_calories + produced.animal_calories,
        demand.plant_calories + demand.animal_calories,
    )
}

/// Workshop headline metric: energy produced vs demand,
/// cycle start → cycle end.
fn energy_row(state: &GameState) -> ChangeRow {
    let start = &state.ui.cycle_start_state;
    let from = energy_percent_met(start.produced, start.output_demand);
    let to = energy_percent_met(state.produced.total(), state.output_demand.total());

    let tip_text = t!(
        r#"How much of the demand for energy (fuel and electricity) is being met. [g]Your goal is to keep this at 100%.[/g]"#
    );
    let energy_tip = tip(icons::ENERGY, tip_text).card(factors_card(None, Var::Electricity, state));

    ChangeRow {
        tip: energy_tip,
        icon: icons::ENERGY,
        label: t!("Energy Supplied").to_string(),
        from: Value::Val(format!("{:.0}%", from)),
        to: Value::Val(format!("{:.0}%", to)),
        pc_change: 0,
    }
}

/// Workshop headline metric: calories produced vs demand,
/// cycle start → cycle end.
fn calories_row(state: &GameState) -> ChangeRow {
    let start = &state.ui.cycle_start_state;
    let from = calories_percent_met(start.produced, start.output_demand);
    let to = calories_percent_met(state.produced.total(), state.output_demand.total());

    let tip_text = t!(
        r#"How much of the demand for food is being met. [g]Your goal is to keep this at 100%.[/g]"#
    );
    let calories_tip =
        tip(icons::PLANT_CALORIES, tip_text).card(factors_card(None, Var::PlantCalories, state));

    ChangeRow {
        tip: calories_tip,
        icon: icons::PLANT_CALORIES,
        label: t!("Calories Supplied").to_string(),
        from: Value::Val(format!("{:.0}%", from)),
        to: Value::Val(format!("{:.0}%", to)),
        pc_change: 0,
    }
}

/// Workshop mode: water is warning-only. Returns a warning message
/// when water demand exceeds the available supply.
fn water_warning(state: &GameState) -> Option<String> {
    let demand = state.resource_demand.of(Resource::Water);
    let available = state.resources.available.water;
    (demand > available)
        .then(|| t!("Water shortage: demand exceeds the available supply.").to_string())
}

fn honeymoon_pc(state: &GameState) -> isize {
    let year = state.world.year;
    let start_year = state.ui.cycle_start_state.year;
    if year < start_year + consts::HONEYMOON_YEARS {
        consts::HONEYMOON_PC as isize
    } else {
        0
    }
}

fn projects_rows(state: &GameState) -> Vec<Project> {
    let recent_completed_projects = &state.ui.cycle_start_state.completed_projects;
    recent_completed_projects
        .iter()
        .map(|project_id| state.world.projects[project_id].clone())
        .collect::<Vec<_>>()
}

fn requests_rows(state: &mut GameState) -> Vec<(String, isize)> {
    let finished_requests = state.check_requests();
    let projects = &state.world.projects;
    let processes = &state.world.processes;
    finished_requests
        .into_iter()
        .map(|(kind, id, active, bounty)| match kind {
            NPCRequest::Project => {
                let project = &projects[&id];
                (
                    if active {
                        t!(
                            "Completed Request: Implement %{name}",
                            name = t!(&project.name)
                        )
                    } else {
                        t!("Completed Request: Stop %{name}", name = t!(&project.name))
                    }
                    .to_string(),
                    bounty as isize,
                )
            }
            NPCRequest::Process => {
                let process = &processes[&id];
                (
                    if active {
                        t!("Completed Request: Unban %{name}", name = t!(&process.name))
                    } else {
                        t!("Completed Request: Ban %{name}", name = t!(&process.name))
                    }
                    .to_string(),
                    bounty as isize,
                )
            }
        })
        .collect::<Vec<_>>()
}

fn parliament_rows(state: &GameState) -> Vec<(String, f32, f32)> {
    let start_parliament = &state.ui.cycle_start_state.parliament;
    start_parliament
        .iter()
        .enumerate()
        .map(|(i, start_seats)| {
            let npc = &state.npcs.by_idx(i);
            let change = (npc.seats - start_seats).round();
            (npc.name.clone(), npc.seats, change)
        })
        .filter(|(_, _, change)| *change != 0.)
        .collect::<Vec<_>>()
}

fn event_rows(state: &GameState) -> Vec<(String, Tip)> {
    let recent_world_events = &state.ui.world_events;
    recent_world_events
        .iter()
        .map(|ev| {
            (
                ev.name.clone(),
                tip(
                    icons::CHANCE,
                    t!("This event occurred during this planning cycle."),
                )
                .card(ev.clone()),
            )
        })
        .collect::<Vec<_>>()
}

fn disaster_rows(state: &GameState) -> Vec<(String, Vec<IconEvent>)> {
    let regions = &state.world.regions;
    let region_events = &state.ui.annual_region_events;
    region_events
        .iter()
        .map(|(idx, events)| {
            let reg = regions[idx].name.clone();
            (reg, events.clone())
        })
        .collect::<Vec<_>>()
}

fn region_rows(state: &GameState) -> Vec<(String, Income)> {
    let regions = &state.world.regions;
    let start_region_incomes = &state.ui.cycle_start_state.region_incomes;
    regions
        .iter()
        .zip(start_region_incomes.iter())
        .filter(|(reg, inc)| reg.income != **inc)
        .map(|(reg, _)| (reg.name.clone(), reg.income))
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_percent_demand_met() {
        assert_eq!(percent_demand_met(50., 100.), 50.);
        assert_eq!(percent_demand_met(100., 100.), 100.);
        // Oversupply reads as more than 100%.
        assert_eq!(percent_demand_met(150., 100.), 150.);
        // Rounded to the nearest whole percent.
        assert_eq!(percent_demand_met(1., 3.), 33.);
        // No demand counts as fully met (avoids division by zero).
        assert_eq!(percent_demand_met(10., 0.), 100.);
        assert_eq!(percent_demand_met(0., 0.), 100.);
    }

    #[test]
    fn test_energy_percent_met_sums_fuel_and_electricity() {
        let produced = OutputMap {
            fuel: 30.,
            electricity: 30.,
            plant_calories: 999.,
            animal_calories: 999.,
        };
        let demand = OutputMap {
            fuel: 40.,
            electricity: 40.,
            plant_calories: 1.,
            animal_calories: 1.,
        };
        // (30 + 30) / (40 + 40) = 75%; calories are ignored.
        assert_eq!(energy_percent_met(produced, demand), 75.);
    }

    #[test]
    fn test_calories_percent_met_sums_plant_and_animal() {
        let produced = OutputMap {
            fuel: 999.,
            electricity: 999.,
            plant_calories: 45.,
            animal_calories: 15.,
        };
        let demand = OutputMap {
            fuel: 1.,
            electricity: 1.,
            plant_calories: 50.,
            animal_calories: 50.,
        };
        // (45 + 15) / (50 + 50) = 60%; energy is ignored.
        assert_eq!(calories_percent_met(produced, demand), 60.);
    }

    #[test]
    fn workshop_report_has_five_interpretable_rows_and_shortages() {
        let mut state = GameState::from_world(hes_engine::World::workshop());
        state.ui.cycle_start_state.year = 2022;
        state.ui.cycle_start_state.emissions = 12.;
        state.ui.cycle_start_state.temperature = 1.2;
        state.ui.cycle_start_state.extinction_rate = 30.;
        state.ui.cycle_start_state.produced = OutputMap {
            fuel: 30.,
            electricity: 50.,
            plant_calories: 60.,
            animal_calories: 40.,
        };
        state.ui.cycle_start_state.output_demand = OutputMap {
            fuel: 50.,
            electricity: 50.,
            plant_calories: 50.,
            animal_calories: 50.,
        };
        state.core.emissions.co2 = 8e15;
        state.core.emissions.ch4 = 0.;
        state.core.emissions.n2o = 0.;
        state.core.world.temperature = 1.1;
        state.core.world.extinction_rate = 32.;
        state.core.produced.amount = OutputMap {
            fuel: 30.,
            electricity: 40.,
            plant_calories: 45.,
            animal_calories: 45.,
        };
        state.core.output_demand.base = OutputMap {
            fuel: 50.,
            electricity: 50.,
            plant_calories: 50.,
            animal_calories: 50.,
        };
        let cycle = WorkshopCycleRecord {
            cycle: 1,
            start_year: 2022,
            end_year: 2027,
            choices: vec![WorkshopPolicyChoice {
                name: "Solar Push".into(),
                action: WorkshopPolicyAction::Passed,
            }],
        };

        let report = workshop_report_data(&state, cycle.clone());

        assert_eq!(report.cycle, cycle);
        assert_eq!(report.metrics.len(), 5);
        assert_eq!(
            report.metrics,
            vec![
                WorkshopMetricRow {
                    label: "CO2 emissions",
                    from: "+12.0 GtCO2e/year".into(),
                    to: "+8.0 GtCO2e/year".into(),
                    target: "≤ 0 GtCO2e/year",
                    direction: MetricDirection::Improved,
                    shortage: None,
                },
                WorkshopMetricRow {
                    label: "Temperature anomaly",
                    from: "+1.2 °C".into(),
                    to: "+1.1 °C".into(),
                    target: "≤ +1.0 °C",
                    direction: MetricDirection::Improved,
                    shortage: None,
                },
                WorkshopMetricRow {
                    label: "Extinction rate",
                    from: "30.0".into(),
                    to: "32.0".into(),
                    target: "≤ 20",
                    direction: MetricDirection::Worsened,
                    shortage: None,
                },
                WorkshopMetricRow {
                    label: "Energy supplied",
                    from: "80%".into(),
                    to: "70%".into(),
                    target: "≥ 100%",
                    direction: MetricDirection::Worsened,
                    shortage: Some("Shortage: only 70% of demand is supplied.".into()),
                },
                WorkshopMetricRow {
                    label: "Calories supplied",
                    from: "100%".into(),
                    to: "90%".into(),
                    target: "≥ 100%",
                    direction: MetricDirection::Worsened,
                    shortage: Some("Shortage: only 90% of demand is supplied.".into()),
                },
            ]
        );
    }
}
