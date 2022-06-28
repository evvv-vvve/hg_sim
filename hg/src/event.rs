use itertools::Itertools;
use rand::Rng;
use regex::Regex;
use serde::{Serialize, Deserialize};
use hashbrown::HashMap;

use crate::{tribute::Tribute, data_trait::{DataTrait, FileError}, pronouns::{they, themself, their, them, Pronouns}};

#[derive(thiserror::Error, Debug)]
pub enum EventError {
    // Represents a failure to read a file
    #[error("Regular expression `{regex:?}` is invalid: {source:?}")]
    RegexInvalid {
        regex: String,
        source: regex::Error,
    },

    // Represents a failure to parse a file
    #[error("Event \"{event:?}\" expects (P{player_num:?}), but they could not be found")]
    MissingPlayerIdentifier {
        event: String,
        player_num: i32,
    },

    #[error("The following event is missing at least 1 field: {event:?}")]
    MissingFieldsError {
        event: String,
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum EventCategory {
    Bloodbath,
    Day,
    FallenTributes,
    Night,
    End
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Event {
    #[serde(skip_serializing)]
    #[serde(default)]
    pub file_name: String,

    pub text: String,
    #[serde(default)]
    pub killed: Vec<String>,
    #[serde(default)]
    pub killers: Vec<String>,
    pub category: EventCategory,
    #[serde(default = "default_weight")]
    pub weight: i32,
}

fn default_weight() -> i32 {
    50
}

#[derive(Debug, Clone)]
pub struct EventResult {
    pub text: String,
    pub killed: Vec<usize>,
    pub killers: Vec<usize>,
    pub tributes: Vec<Tribute>
}

impl DataTrait for Event {
    type Output = Event;

    fn from_file(file: &str) -> Result<Event, FileError> {
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

impl Event {
    pub fn get_text(&self) -> String { self.text.clone() }

    pub fn get_num_tributes_required(&self) -> usize {
        let reg_expr_player = Regex::new(r"(?i)\(P[1-9][^)]*\)").unwrap();

        let mut tribute_collection: Vec<String> = Vec::new();
        for player_cap in reg_expr_player.captures_iter(&self.text) {
            if !tribute_collection.contains(&player_cap[0].to_uppercase()) {
                tribute_collection.push(String::from(&player_cap[0].to_uppercase()));
            }
        }

        tribute_collection.len()
    }

    // TODO: separate this function into other functions
    pub fn get_result(&mut self, tributes: &mut Vec<Tribute>) -> Result<EventResult, EventError> {
        // match ex: (P1)
        let player_regex = r"(?i)\(P[1-9][^)]*\)";
        let reg_expr_player = Regex::new(player_regex).map_err(|source|
            EventError::RegexInvalid {
                regex: player_regex.to_string(),
                source
            }
        )?;

        let they_regex = r"(?i)\(they[1-9][^)]*\)";
        let reg_expr_they = Regex::new(they_regex).map_err(|source|
            EventError::RegexInvalid {
                regex: they_regex.to_string(),
                source
            }
        )?;

        let them_regex = r"(?i)\(them[1-9][^)]*\)";
        let reg_expr_them = Regex::new(them_regex).map_err(|source|
            EventError::RegexInvalid {
                regex: them_regex.to_string(),
                source
            }
        )?;

        let their_regex = r"(?i)\(their[1-9][^)]*\)";
        let reg_expr_their = Regex::new(their_regex).map_err(|source|
            EventError::RegexInvalid {
                regex: them_regex.to_string(),
                source
            }
        )?;

        let themself_regex = r"(?i)\(themself[1-9][^)]*\)";
        let reg_expr_themself = Regex::new(themself_regex).map_err(|source|
            EventError::RegexInvalid {
                regex: themself_regex.to_string(),
                source
            }
        )?;

        let parenthesis_regex = r"(?i)\(P[1-9][^)]*\)\([0-9a-zA-Z'_0\-& ]+/[0-9a-zA-Z'_0\-&]+\)";
        let reg_exp_parenthesis = Regex::new(parenthesis_regex).map_err(|source|
            EventError::RegexInvalid {
                regex: parenthesis_regex.to_string(),
                source
            }
        )?;
        
        let mut tribute_collection: HashMap<String, Tribute> = HashMap::new();

        /* Iterate over the player captures (e.g. (P1), (P2), (P3), etc).
         * If the player capture isn't in our tribute collection, choose a random tribute
         * and add them to the collection */
        for player_cap in reg_expr_player.captures_iter(&self.text) {
            if !tribute_collection.contains_key(&player_cap[0].to_uppercase()) {
                let index = rand::thread_rng().gen_range(0..tributes.len());
                let trib = tributes.remove(index);

                tribute_collection.insert(String::from(&player_cap[0].to_uppercase()), trib.clone());
            }
        }

        let mut result = EventResult::new(&self.text);

        // iterate over the collection of tributes,
        // updating the "alive" status of the ones
        // that need to be killed
        for (player_val, mut tribute) in tribute_collection.clone().into_iter()
            .sorted_by(|a, b|
                get_tribute_num(&a.0).cmp(&get_tribute_num(&b.0))) {
            
            if self.killed.contains(&player_val) {
                tribute.kill();
                result.killed.push(tribute.get_id());    
            }
            
            if self.killers.contains(&player_val) {
                result.killers.push(tribute.get_id());
            }

            result.tributes.push(tribute);
        }

        if result.killers.len() > 0 && result.killed.len() == 0 || result.killed.len() > 0 && result.killers.len() == 0 {
            return Err(EventError::MissingFieldsError { event: result.text })
        }

        result.replace_pronouns(reg_expr_they)?
              .replace_pronouns(reg_expr_them)?
              .replace_pronouns(reg_expr_their)?
              .replace_pronouns(reg_expr_themself)?
              .replace_player_words(reg_exp_parenthesis, reg_expr_player)?
              .replace_players(tribute_collection)
    }
}

impl EventResult {
    pub fn new(text: &str) -> EventResult {
        EventResult {
            text: String::from(text),
            killed: Vec::new(),
            killers: Vec::new(),
            tributes: Vec::new()
        }
    }

    pub fn replace_players(self, tribute_collection: HashMap<String, Tribute>) -> Result<EventResult, EventError> {
        let mut result = self.clone();
        
        for (player_val, tribute) in tribute_collection.into_iter()
            .sorted_by(|a, b|
                get_tribute_num(&a.0).cmp(&get_tribute_num(&b.0))) {
            let e = tribute.name.clone();
            result.text = result.text.replace(&player_val, &e);
        }

        Ok(result)
    }
    pub fn replace_player_words(self, word_regex: Regex, player_regex: Regex) -> Result<EventResult, EventError> {
        let mut result = self.clone();

        // iterate over matches, replace them w/the tributes pronouns
        for word_cap in word_regex.captures_iter(&self.text) {
            let mut words = word_cap[0].to_string();

            for player_cap in player_regex.captures_iter(&words.clone()) {
                let trib_id = get_tribute_num(&player_cap[0]);

                let trib = self.tributes.get((trib_id - 1) as usize).ok_or(EventError::MissingPlayerIdentifier {
                    event: self.text.clone(),
                    player_num: (trib_id - 1) as i32
                })?;

                // eh?
                words = words.replace(&player_cap[0], "")
                    .replace("(", "")
                    .replace(")", "");
                
                let words_split = words.split('/').collect_vec();

                let word = if trib.pronouns == Pronouns::They {
                    words_split[1]
                } else {
                    words_split[0]
                };

                result.text = result.text.replace(&word_cap[0], word)
            }
        }

        Ok(result)
    }

    pub fn replace_pronouns(self, regex: Regex) -> Result<EventResult, EventError> {
        let mut result = self.clone();

        // iterate over matches, replace them w/the tributes pronouns
        for cap in regex.captures_iter(&self.text) {
            let trib_id = get_tribute_num(&cap[0]);

            let trib = self.tributes.get((trib_id - 1) as usize).ok_or(EventError::MissingPlayerIdentifier {
                event: self.text.clone(),
                player_num: (trib_id - 1) as i32
            })?;
            
            // bleh
            let lower_capture = cap[0].to_lowercase();
            let prns =
                if lower_capture.contains("they") { Option::Some(they(&trib.pronouns, cap[0].to_string())) }
                else if lower_capture.contains("themself") { Option::Some(themself(&trib.pronouns, cap[0].to_string())) }
                else if lower_capture.contains("their") { Option::Some(their(&trib.pronouns, cap[0].to_string())) }
                else if lower_capture.contains("them") { Option::Some(them(&trib.pronouns, cap[0].to_string())) }
                else { Option::None };
    
            if let Option::Some(s) = prns {
                result.text = result.text.replace(&cap[0], &s);
            }
        }

        Ok(result)
    }
}

fn get_tribute_num(string: &str) -> u32 {
    string.chars().rev().nth(1).unwrap().to_digit(10).unwrap()
}