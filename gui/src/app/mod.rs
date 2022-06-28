use eframe::{egui, epi};
use hg::{simulation::{Simulation, SimulationError}, event::{EventResult, Event}};

pub mod app_sim;
pub mod app_main_menu;
pub mod app_event_editor;

#[derive(Clone)]
pub enum AppState {
    MainMenu,
    Simulation {
        title: String,
        events: Vec<EventResult>,
        advance_step: bool
    },
    EventEditor(EventEditorState)
}

#[derive(Debug, Clone)]
pub struct EventEditorState {
    pub events: Vec<EditedEvent>,
    pub is_edited: bool,
    pub attempted_exit: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum EditedState {
    Unchanged,
    Edited,
    Added,
    Removed
}

#[derive(Debug, Clone, PartialEq)]
pub struct EditedEvent {
    state: EditedState,
    event: Event,
    id: u32,
    killers: String,
    killed: String,
}

pub struct HGSimApp {
    pub(super) app_state: AppState,

    // simulation state
    pub(super) simulation: Simulation,
    pub(super) error: Option<SimulationError>,
}

impl HGSimApp {
    pub fn new(simulation: Simulation) -> Self {
        Self {
            app_state: AppState::MainMenu,
            simulation,
            error: None,
        }
    }

    pub fn get_error(&self) -> &Option<SimulationError> {
        return &self.error
    }
}

impl epi::App for HGSimApp {
    fn name(&self) -> &str {
        "Hunger Games Simulator"
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, frame: &epi::Frame) {
        //let Self { app_state, simulation, error} = self;

        if let Some(err) = &self.error {
            egui::Window::new("Simulation Error")
            .collapsible(false)
            .resizable(false)
            .show(ctx, |ui| {
                ui.label(format!("{err}"));
                if ui.button("OK").clicked() {
                    frame.quit();
                    eprintln!("An error occurred while running simulation: {err}");
                    std::process::exit(1);
                }
            });
        }

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        frame.quit();
                    }
                });
            });
        });

        egui::SidePanel::left("side_panel").resizable(false).show(ctx, |ui| {
            ui.label("");
            ui.vertical_centered(|ui| {
                ui.button("Options")
            });

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                egui::warn_if_debug_build(ui);

                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    ui.label("powered by ");
                    ui.hyperlink_to("egui", "https://github.com/emilk/egui");
                    ui.label(" and ");
                    ui.hyperlink_to("eframe", "https://github.com/emilk/egui/tree/master/eframe");
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered_justified(|ui| {
                ui.heading("Hunger Games Simulator")
            });

            ui.separator();

            let app_state = self.app_state.clone();

            match app_state {
                AppState::MainMenu =>  self.main_menu(ctx, ui),
                AppState::Simulation {
                    title,
                    events,
                    advance_step
                } => {
                    if let Err(err) = self.simulation(ctx, ui, title, events, advance_step) {
                        self.error = Some(err);
                    }
                },
                AppState::EventEditor(mut editor_state) => {
                    // we do some stuff here and there, and don't necessarily want
                    // to update the state each time
                    // usually I'd just do a "return" but that's not working right for some reason
                    let mut update_state = true;

                    //let new_state = self.event_editor(ctx, ui, &mut editor_state);
                    self.event_editor(ctx, ui, &mut editor_state);

                    if editor_state.attempted_exit {
                        if editor_state.is_edited {
                            egui::Window::new("Warning!")
                              .collapsible(false)
                              .resizable(false)
                              .show(ctx, |ui| {
                                  ui.label(format!("You have unsaved changes. Do you really want to quit?"));
                                  
                                  ui.horizontal(|ui| {
                                    if ui.button("Yes").clicked() {
                                        self.app_state = AppState::MainMenu;
                                        update_state = false;
                                    }
  
                                    if ui.button("No").clicked() {
                                        editor_state.attempted_exit = false;
                                    }
                                  });
                            });
                        } else {
                            self.app_state = AppState::MainMenu;
                            update_state = false;
                        }
                    }
                    
                    if update_state {
                        self.app_state = AppState::EventEditor(editor_state);
                    }
                }
            }
        });
    }
}