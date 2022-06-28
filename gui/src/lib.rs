use hg::simulation::Simulation;

pub mod app;

pub fn run(simulation: Simulation) {
    let app = app::HGSimApp::new(simulation);
    let native_options = eframe::NativeOptions::default();
    //native_options.initial_window_size = Some(Vec2 { x: 1920.0, y: 720.0 });

    eframe::run_native(Box::new(app), native_options);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
