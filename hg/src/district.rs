use serde::{Serialize, Deserialize};

use crate::{tribute::Tribute, data_trait::{DataTrait, FileError}};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct District {
    #[serde(skip_serializing)]
    #[serde(default)]
    pub file_name: String,

    pub name: String,
    pub tributes: Vec<Tribute>
}

impl DataTrait for District {
    type Output = District;

    fn from_file(file: &str) -> Result<District, FileError> {
        // Attempt to read the data in a file
        let contents = std::fs::read_to_string(file).map_err(|source|
            FileError::FileReadError {
                file: file.to_string(),
                source
            }
        )?;

        // Attempt to parse the data to TOML
        let toml = toml::from_str(&contents).map_err(|source|
            FileError::TOMLParseError {
                file: file.to_string(),
                source
            }
        )?;

        Ok(toml)
    }

    fn set_path(&mut self, file_name: &str) {
        self.file_name = file_name.to_string().replace(".toml", "");
    }
}

impl District {
    pub fn has_living_tributes(&self) -> bool {
        let mut living = 0;

        for trib in &self.tributes {
            if trib.is_alive {
                living += 1;
            }
        }

        living > 0
    }

    pub fn get_living(&self) -> Vec<Tribute> {
        let mut living = Vec::new();

        for trib in &self.tributes {
            if trib.is_alive {
                living.push(trib.clone());
            }
        }

        if living.len() < 1 {
            return Vec::new()
        }

        living
    }

    /*pub async fn update_avatars_async(&self) -> Result<District, Error> {
        let mut dist = self.clone();
        let mut tribs = Vec::new();

        let fetches = futures::stream::iter(
            self.clone().tributes.into_iter().map(|trib| {
                async move {
                    match trib.update_avatar().await {
                        Ok(t) => Ok(t),
                        Err(e) => Err(anyhow!(format!("Could not fetch avatar for Tribute {}: {}",
                            trib.name, e.to_string())))
                    }
                }
            })
        ).buffer_unordered(100).collect::<Vec<Result<Tribute, Error>>>();

        for res in fetches.await {
            match res {
                Ok(t) => tribs.push(t),
                Err(e) => return Err(e)
            }
        }

        dist.tributes = tribs;
        Ok(dist)
    }*/
}