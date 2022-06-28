use hg::{simulation::{build_sim, Simulation}, simulation_settings::fetch_or_create};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    
    let settings = match fetch_or_create() {
        Ok(config) => config,
        Err(err) => {
            eprintln!("An error occurred while loading simulation settings: {}", err);
            std::process::exit(1)
        }
    };
    
    let simulation = match build_sim(settings) {
        Ok(sim) => sim,
        Err(err) => {
            eprintln!("An error occurred while building simulation: {}", err);
            std::process::exit(1)
        }
    };

    if args.contains(&"-cli".to_string()) {
        run_cli(simulation);
    } else {
        gui::run(simulation);
    }
}


fn run_cli(mut simulation: Simulation) {
    loop {
        println!("===== {}", simulation.get_category_title());
        let is_end = simulation.is_end();

        if let Err(err) = simulation.step() {
            eprintln!("An error occurred while running simulation: {}", err);
            std::process::exit(1)
        }

        if is_end {
            if let Some(winner) = simulation.get_winner() {
                println!("The winner is {} from {}!", winner.name, simulation.get_trib_dist_name(winner));
            } else {
                println!("There were no winners this round! (??? Shouldn't happen!! Report if it does)");
            }
        } else {
            for event in &simulation.get_clear_next_events() {
                println!("{}", event.text)
            }
        }
    
        println!("\nPress enter to continue");
        let mut pebis = String::new();
    
        std::io::stdin()
            .read_line(&mut pebis)
            .expect("Failed to read line!");
    }
}