use eframe::egui::{Ui, Context};
use rand::Rng;
use super::{HGSimApp, AppState, EditedEvent, EditedState, EventEditorState};

impl HGSimApp {
    pub(super) fn main_menu(&mut self, _ctx: &Context, ui: &mut Ui) {
        ui.vertical_centered(|ui| {
            ui.label("(WIP!)");
            if ui.button("Edit Events").clicked() {
                let mut events = Vec::new();
                let mut rand = rand::thread_rng();

                for event in self.simulation.get_all_events() {
                    let killers = event.killers.join(", ");
                    let killed = event.killed.join(", ");

                    events.push(EditedEvent {
                        state: EditedState::Unchanged,
                        event,
                        id: rand.gen(),
                        killers,
                        killed,
                    });
                }
                
                self.app_state = AppState::EventEditor(EventEditorState {
                    events,
                    is_edited: false,
                    attempted_exit: false,
                });
            }

            ui.label("");
            ui.label("(unimplemented!)");
            
            if ui.button("Edit Districts/Tributes").clicked() {

            }
            
            ui.label("");
            
            if ui.button("Run Simulation").clicked() {
                self.app_state = AppState::Simulation {
                    title: self.simulation.get_category_title(),
                    events: Vec::new(),
                    advance_step: true
                }
            }
        });
    }
}