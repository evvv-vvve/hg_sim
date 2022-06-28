use serde::{Serialize, Deserialize};

use crate::data_trait::FileError;

#[derive(Serialize, Deserialize, Clone)]
pub struct SimulationSettings {
    pub event_folders: Vec<String>,
    pub district_folders: Vec<String>,
    pub death_rate: f64
}

impl SimulationSettings {
    pub fn new() -> Self {
        Self {
            event_folders: vec![ String::from("events/") ],
            district_folders: vec![ String::from("districts/") ],
            death_rate: 0.17
        }
    }

    pub fn parse(path: &str) -> Result<Self, FileError> {
        // Attempt to read the data in a file
        let contents = std::fs::read_to_string(path).map_err(|source|
            FileError::FileReadError {
                file: path.to_string(),
                source
            }
        )?;

        // Attempt to parse the data to TOML
        let toml = toml::from_str(&contents).map_err(|source|
            FileError::TOMLParseError {
                file: path.to_string(),
                source
            }
        )?;

        Ok(toml)
    }

    pub fn save(&self, path: &str) -> Result<Self, FileError> {
        let contents = toml::to_string(self).map_err(|source|
            FileError::TOMLSerializeError {
                file: String::from(path),
                source
            }
        )?;

        std::fs::write(path, contents).map_err(|source|
            FileError::FileWriteError {
                file: String::from(path),
                source
            }
        )?;

        Ok(self.clone())
    }
}

pub fn fetch_or_create() -> Result<SimulationSettings, FileError> {
    let path = "simulation.toml";

    if std::path::Path::exists(std::path::Path::new(path)) {
        SimulationSettings::parse(path)
    } else {
        SimulationSettings::new().save(path)
    }
}