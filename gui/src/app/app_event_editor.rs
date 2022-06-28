use eframe::{egui::{self, Ui, Context}};
use hg::event::EventCategory;
use rand::Rng;

use super::{HGSimApp, EditedEvent, EditedState, EventEditorState};

// Event Editor
impl HGSimApp {
    pub(super) fn event_editor(&mut self, _ctx: &Context, ui: &mut Ui, editor_state: &mut EventEditorState) {
        ui.horizontal(|ui| {
            if ui.button("Main Menu").clicked() {
                editor_state.attempted_exit = true;
            }

            if ui.button("Save").clicked() {

            }
    
            if ui.button("Add Event").clicked() {
                let mut rand = rand::thread_rng();
                
                let file_id: u32 = rand.gen();
                editor_state.events.push(EditedEvent {
                    state: EditedState::Added,
                    event: hg::event::Event {
                        file_name: format!("event_{file_id}"),
                        text: String::new(),
                        killed: Vec::new(),
                        killers: Vec::new(),
                        category: EventCategory::Bloodbath,
                        weight: 50
                    },
                    id: rand.gen(),
                    killers: String::new(),
                    killed: String::new()
                });
            }
        });

        ui.separator();

        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.vertical_centered(|ui| {
                for event in &mut editor_state.events {
                    let orig_event = event.clone();

                    ui.horizontal(|ui| {
                        ui.vertical(|ui| {
                            ui.label("Event Category");

                            // We pass our own id because otherwise egui goes bananas
                            egui::ComboBox::new(event.id, "")
                              .selected_text(format!("{:?}", event.event.category))
                              .show_ui(ui, |ui| {
                                  ui.selectable_value(&mut event.event.category, EventCategory::Bloodbath, "Bloodbath");
                                  ui.selectable_value(&mut event.event.category, EventCategory::Day, "Day");
                                  ui.selectable_value(&mut event.event.category, EventCategory::Night, "Night");
                            });
                        });

                        ui.vertical(|ui| {
                            ui.label("Weight");
                            ui.add(egui::Slider::new(&mut event.event.weight, 1..=230))
                        });
                    });

                    ui.horizontal(|ui| {
                        ui.label("File Name ");
                        ui.text_edit_singleline(&mut event.event.file_name);
                    });

                    ui.horizontal(|ui| {
                        ui.label("Event Text ");
                        ui.text_edit_singleline(&mut event.event.text);
                    });

                    ui.horizontal(|ui| {
                        ui.label("Killers ");
                        ui.text_edit_singleline(&mut event.killers);
                    });

                    ui.horizontal(|ui| {
                        ui.label("Killed ");
                        ui.text_edit_singleline(&mut event.killed);
                    });

                    if ui.button("Delete").clicked() {

                    }

                    if *event != orig_event {
                        editor_state.is_edited = true;
                        event.state = EditedState::Edited;
                    }

                    ui.separator();
                }
            });
        });
    }
}