use std::path::Path;

use rand::{distributions::{WeightedIndex, WeightedError}, prelude::{SliceRandom, Distribution}};

use crate::{district::District, event::{EventCategory, EventResult, Event, EventError}, tribute::Tribute, simulation_settings::SimulationSettings, data_trait::{DataTrait, FileError}};

#[derive(Debug, Clone)]
pub struct SimEvents {
    bloodbath: EventData,
    day: EventData,
    night: EventData,
}

#[derive(Debug, Clone)]
pub struct Simulation {
    districts: Vec<District>,
    events: SimEvents,
    state: EventCategory,
    day: u32,
    killed_today: Vec<Tribute>,
    next_events: Vec<EventResult>,
    death_rate: f64,
    prev_state: EventCategory
}

#[derive(Debug, Clone)]
struct EventData {
    events: Vec<Event>,
    weights: Vec<i32>,
}

impl EventData {
    pub fn create(events_vec: Vec<Event>) -> Self {
        let mut events = Vec::new();
        let mut weights = Vec::new();

        for event in events_vec.into_iter() {
            weights.push(event.weight);
            events.push(event);
        }
        
        Self {
            events,
            weights,
        }
    }

    pub fn has_fatal(&self) -> bool {
        for event in &self.events {
            if event.killed.len() > 0 {
                return true
            }
        }

        false
    }

    pub fn get_random_event(&self, tributes: &Vec<Tribute>, living: usize, force_fatal: bool) -> Result<Option<Event>, SimulationError> {
        let dist = WeightedIndex::new(&self.weights).map_err(|source|
            SimulationError::WeightedRandomError { source }
        )?;

        let mut rng = rand::thread_rng();

        let mut loops = 0;
        
        loop {
            match self.events.get(dist.sample(&mut rng)) {
                Option::Some(ev) => {
                    if loops >= 25 {
                        if ev.get_num_tributes_required() <= tributes.len() && ev.killed.len() == 0 {
                            return Ok(Some(ev.clone()));
                        }
                    }

                    //println!("{}", ev.get_text());
                    //println!("req: {} | len: {} | killed: {} | living: {}", ev.get_num_tributes_required(), tributes.len(), ev.killed.len(), living - 1);

                    if ev.get_num_tributes_required() <= tributes.len() && ev.killed.len() <= living - 1 {
                        if force_fatal && ev.killed.len() == 0 {
                            continue;
                        }
                        
                        if living == 2 {
                            if ev.killed.len() == 1 {
                                return Ok(Some(ev.clone()))
                            } else {
                                continue;
                            }
                        } else {
                            return Ok(Some(ev.clone()))
                        }
                    }
                },
                Option::None => return Ok(None)
            };

            loops += 1;
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum SimulationError {
    // Represents a failure to read a file
    #[error("Failure to do operations on file {file:?}: {source:?}")]
    FileError {
        file: String,
        source: FileError,
    },

    #[error("Failure to read directory {dir:?}: {source:?}")]
    DirectoryReadError {
        dir: String,
        source: std::io::Error
    },

    #[error("No Districts were found")]
    NoDistricts,
    
    #[error("No Events were found")]
    NoEvents,
    
    #[error("An event-related error occurred: {event_error:?}")]
    EventError {
        event_error: EventError
    },

    #[error("A weighted random error occurred: {source:?}")]
    WeightedRandomError {
        source: WeightedError
    },

    #[error("No tributes were found, when at least 1 was expected: {event:?}")]
    MissingTributesError {
        event: String
    },
}

impl Simulation {
    pub fn get_all_events(&self) -> Vec<Event> {
        let mut events = Vec::new();

        let mut a = self.events.bloodbath.events.clone();
        let mut b = self.events.day.events.clone();
        let mut c = self.events.night.events.clone();
        
        events.append(&mut a);
        events.append(&mut b);
        events.append(&mut c);

        events
    }

    pub fn step(&mut self) -> Result<(), SimulationError> {
        let mut events = Vec::new();

        // if the simulations over, we just wanna skip out
        if self.state == EventCategory::End {
            return Ok(())
        }

        let mut tributes_left = {
            let tribs = self.get_living_tributes();

            tribs.clone()
        };

        let mut living_this_step = self.get_living_tributes().len();

        let mut tribs_to_die = (living_this_step as f64 * self.death_rate) as i32;
        
        while tributes_left.len() != 0 {
            // "Fallen Tributes" is a unique event that just
            // lists out tributes that died within the last 24 hours (in-game)
            if self.state == EventCategory::FallenTributes {
                let gunshots = if self.killed_today.len() < 1 {
                    String::from("No cannon shots can be heard in the distance.\n")
                } else {
                    if self.killed_today.len() == 1 {
                        format!("1 cannon shot can be heard in the distance.\n")
                    } else {
                        format!("{} cannon shots can be heard in the distance.\n", self.killed_today.len())
                    }
                };

                events.push(EventResult::new(&gunshots));

                // Create death events for each dead tribute,
                // listing their name and district
                for trib in &self.killed_today {
                    let mut event = EventResult::new(&format!("{} from {}", trib.name, self.get_trib_dist_name(trib)));
                    
                    event.tributes.push(trib.clone());

                    events.push(event);
                }

                // we've listed them all out, start clean for the next day's events
                self.killed_today.clear();
                
                break;
            }

            // Fetch a random event 
            let force_fatal = self.has_fatal() && tribs_to_die > 0;
            let event = self.get_rand_event(&tributes_left, living_this_step,  force_fatal)?;
            

            let event_result = if event.is_none() {
                continue;
            } else {
                let mut ev = event.unwrap();

                tribs_to_die -= ev.killed.len() as i32;
                living_this_step -= ev.killed.len();

                ev.get_result(&mut tributes_left).map_err(|event_error|
                    SimulationError::EventError { event_error }
                )?
            };
            
            
            for dist in &mut self.districts {
                for trib in &mut dist.tributes {
                    // kill tributes that need killin
                    for trib_id in &event_result.killed {
                        if trib.get_id() == *trib_id {
                            trib.kill();
                            self.killed_today.push(trib.clone());
                        }
                    }

                    // reward tributes that need rewardin
                    for trib_id in &event_result.killers {
                        if trib.get_id() == *trib_id {
                            trib.add_kill();
                        }
                    }
                }
            }
            
            for trib in &event_result.tributes {
                tributes_left.retain(|t| t.get_id() != trib.get_id());
            }

            events.push(event_result);
            
        }

        if self.state != EventCategory::FallenTributes {
            let mut rng = rand::thread_rng();
            events.shuffle(&mut rng);
        }
        
        self.next_events = events;
        
        self.prev_state = self.state.clone();

        if self.has_winner() {
            self.state = EventCategory::End;
            self.prev_state = EventCategory::End;
        } else {
            self.step_cat();
        }

        Ok(())
    }

    pub fn get_category_title(&self) -> String {
        match self.state {
            EventCategory::Bloodbath => String::from("The Bloodbath"),
            EventCategory::Day => format!("Day {}", self.day),
            EventCategory::FallenTributes => format!("Fallen Tributes - Day {}", self.day),
            EventCategory::Night => format!("Night {}", self.day),
            EventCategory::End => String::from("The Winner")
        }
    }

    pub fn is_end(&self) -> bool {
        self.state == EventCategory::End
    }

    pub fn get_clear_next_events(&mut self) -> Vec<EventResult> {
        let ev = self.next_events.clone();
        self.next_events.clear();

        ev
    }

    pub fn has_next_events(&self) -> bool {
        self.next_events.len() > 0
    }
}

impl Simulation {
    pub fn new(districts: Vec<District>, events: Vec<Event>, death_rate: f64) -> Simulation {
        Simulation {
            districts,
            events: SimEvents {
                bloodbath: EventData::create(
                    events.iter()
                        .enumerate()
                        .filter(|&(_, event)| event.category == EventCategory::Bloodbath)
                        .map(|(_, event)| event.clone())
                        .collect::<Vec<Event>>()
                ),
                day: EventData::create(
                    events.iter()
                    .enumerate()
                    .filter(|&(_, event)| event.category == EventCategory::Day)
                    .map(|(_, event)| event.clone())
                    .collect::<Vec<Event>>()
                ),
                night: EventData::create(
                    events.iter()
                    .enumerate()
                    .filter(|&(_, event)| event.category == EventCategory::Night)
                    .map(|(_, event)| event.clone())
                    .collect::<Vec<Event>>()
                ),
            },
            state: EventCategory::Bloodbath,
            day: 1,
            killed_today: Vec::new(),
            next_events: Vec::new(),
            death_rate,
            prev_state: EventCategory::Bloodbath
        }
    }

    /*pub async fn fetch_avatars(&self) -> Result<Simulation, String> {
        let mut sim = self.clone();
        let fetches = futures::stream::iter(
            self.clone().districts.into_iter().map(|dist| {
                async move {
                    match dist.update_avatars_async().await {
                        Ok(d) => Ok(d),
                        Err(e) => Err(e.to_string())
                    }
                }
            })
        ).buffer_unordered(100).collect::<Vec<Result<District, String>>>();

        let mut dists = Vec::new();

        for res in fetches.await {
            match res {
                Ok(dist) => dists.push(dist),
                Err(e) => return Err(e)
            }
        }
        
        sim.districts = dists;

        Ok(sim)
    }*/
}

impl Simulation {
    fn step_cat(&mut self) {
        match self.state {
            EventCategory::Bloodbath => self.state = EventCategory::Day,
            EventCategory::Day => self.state = EventCategory::FallenTributes,
            EventCategory::FallenTributes => self.state = EventCategory::Night,
            EventCategory::Night => self.state = {
                self.day += 1;
                EventCategory::Day
            },
            EventCategory::End => ()
        }
    }

    pub fn has_winner(&self) -> bool {
        let mut living = 0;
        
        for dist in &self.districts {
            if dist.has_living_tributes() {
                for trib in &dist.tributes {
                    if trib.is_alive {
                        living += 1;
                    }
                }
            }
        }

        living <= 1
    }

    pub fn get_winner(&self) -> Option<&Tribute> {
        let dists: Vec<&District> = self.districts
                        .iter()
                        .filter(|&dist| dist.has_living_tributes())
                        .collect();

        for dist in dists {
            for trib in &dist.tributes {
                if trib.is_alive {
                    return Option::Some(trib)
                }
            }
        }

        Option::None
    }

    pub fn get_trib_dist_name(&self, tribute: &Tribute) -> String {
        for dist in &self.districts {
            for trib in &dist.tributes {
                if trib.get_id() == tribute.get_id() {
                    return dist.name.clone()
                }
            }
        }

        String::from("Unknown District")
    }

    pub fn get_living_tributes(&self) -> Vec<Tribute> {
        let mut living: Vec<Tribute> = Vec::new();

        for dist in &self.districts {
            if dist.has_living_tributes() {
                living.append(&mut dist.get_living().clone());
            }
        }

        living.clone()
    }

    pub fn cli_display_living(&self) -> String {
        let mut living = String::from("Living tributes:\n");
        
        for dist in &self.districts {
            for trib in &dist.tributes {
                if trib.is_alive {
                    let kill_txt = if trib.kills == 1 { "kill" } else { "kills" };
                    living.push_str(&format!("\t{} ({}) [{} {}]\n", trib.name, dist.name, trib.kills, kill_txt));
                }
            }
        }

        living
    }
    
    pub fn get_rand_event(&self, tributes: &Vec<Tribute>, living: usize, force_fatal: bool) -> Result<Option<Event>, SimulationError> {
        match self.state {
            EventCategory::Bloodbath => self.events.bloodbath.get_random_event(tributes, living, force_fatal),
            EventCategory::Day => self.events.day.get_random_event(tributes, living, force_fatal),
            EventCategory::Night => self.events.night.get_random_event(tributes, living, force_fatal),
            _ => return Ok(None)
        }
    }

    pub fn has_fatal(&self) -> bool {
        match self.state {
            EventCategory::Bloodbath => self.events.bloodbath.has_fatal(),
            EventCategory::Day => self.events.day.has_fatal(),
            EventCategory::Night => self.events.night.has_fatal(),
            _ => return false
        }
    }

    pub fn get_districts(&self) -> Vec<District> {
        self.districts.clone()
    }
}

pub fn build_sim(settings: SimulationSettings) -> Result<Simulation, SimulationError> {
    let mut events = Vec::new();
    let mut districts = Vec::new();

    for path in settings.event_folders {
        match load_data_from_dir::<Event>(&path) {
            Ok(mut evs) => events.append(&mut evs),
            Err(e) => return Err(e)
        };
    }

    for path in settings.district_folders {
        match load_data_from_dir::<District>(&path) {
            Ok(mut dists) => districts.append(&mut dists),
            Err(e) => return Err(e)
        };
    }

    if events.len() < 1 {
        return Err(SimulationError::NoEvents);
    }

    if districts.len() < 1 {
        return Err(SimulationError::NoDistricts);
    }

    Ok(Simulation::new(districts, events, settings.death_rate))
}

/*pub async fn build_sim_async(settings: SimulationSettings) -> std::result::Result<Simulation, String> {
    return match build_sim(settings) {
        Ok(sim_) => {
            async move {
                match sim_.fetch_avatars().await {
                    Ok(sim) => Ok(sim),
                    Err(e) => Err(e)
                }
            }.await
        },
        Err(e) => Err(e.to_string())
    }
}*/

fn load_data_from_dir<T: DataTrait<Output = T>>(dir: &str) -> Result<Vec<T>, SimulationError> {
    let mut files = Vec::new();

    let dir_contents = std::fs::read_dir(dir).map_err(|source|
        SimulationError::DirectoryReadError {
            dir: dir.to_string(),
            source
        }
    )?;

    for entry in dir_contents {

        let f = entry.unwrap();
        let path = f.path();
        let file_path = path.to_str().unwrap();
        let mut file = T::from_file(file_path).map_err(|source|
            SimulationError::FileError {
                file: file_path.to_string(),
                source
            }
        )?;

        file.set_path(Path::new(&file_path).file_name().unwrap().to_str().unwrap());
        
        files.push(file);
    }

    Ok(files)
}