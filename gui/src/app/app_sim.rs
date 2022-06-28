use eframe::{egui::{self, Ui, Context}};
use hg::{event::EventResult, simulation::SimulationError};

use super::{HGSimApp, AppState};

impl HGSimApp {
    pub(super) fn simulation(&mut self, _ctx: &Context, ui: &mut Ui, title: String, sim_events: Vec<EventResult>, advance_step: bool) -> Result<(), SimulationError> {
        let mut events = sim_events.clone();

        ui.vertical_centered_justified(|ui| {
            ui.heading(title.clone())
        });

        let is_end = self.simulation.is_end();

        if advance_step && !is_end {
            if let Err(err) = self.simulation.step() {
                return Err(err)
            }

            events = self.simulation.get_clear_next_events();

            self.app_state = AppState::Simulation {
                title: if self.simulation.is_end() {
                    self.simulation.get_category_title()
                } else {
                    title
                },
                events: events.clone(),
                advance_step: false
            };
        }

        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.vertical_centered(|ui| {
                if is_end {
                    if let Some(winner) = self.simulation.get_winner() {
                        ui.label(format!("The winner is {} from {}!", winner.name, self.simulation.get_trib_dist_name(winner)));
                    } else {
                        return Err(SimulationError::MissingTributesError { event: "win_event".to_string() })
                    }
                } else {
                    for event in events {
                        ui.label(event.text.clone());
                    }
                }
    
                if ui.button("Proceed").clicked() {
                    self.app_state = AppState::Simulation {
                        title: self.simulation.get_category_title(),
                        events: Vec::new(),
                        advance_step: true
                    }
                }

                return Ok(())
            })
        });

        return Ok(())
    }
}