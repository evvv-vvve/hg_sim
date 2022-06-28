use serde::Serialize;
use serde::Deserialize;

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum Pronouns {
    He,
    She,
    They,
    It
}

fn some_kind_of_uppercase_first_letter(s: &str) -> String {
    let mut c = s.chars();
    
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str()
    }
}

fn is_first_character_upper(s: &str) -> bool {
    let mut c = s.chars();
    let _ = c.next().unwrap();
    
    c.next().unwrap().is_uppercase()
}

pub fn they(prns: &Pronouns, str: String) -> String {
    let is_cap = is_first_character_upper(str.as_str());

    let mut prn = match prns {
        Pronouns::He => String::from("he"),
        Pronouns::She => String::from("she"),
        Pronouns::They => String::from("they"),
        Pronouns::It => String::from("it"),
    };

    if is_cap {
        prn = some_kind_of_uppercase_first_letter(prn.as_str());
    }

    prn
}

pub fn them(prns: &Pronouns, str: String) -> String {
    let is_cap = is_first_character_upper(str.as_str());

    let mut prn = match prns {
        Pronouns::He => String::from("him"),
        Pronouns::She => String::from("her"),
        Pronouns::They => String::from("them"),
        Pronouns::It => String::from("it"),
    };

    if is_cap {
        prn = some_kind_of_uppercase_first_letter(prn.as_str());
    }

    prn
}

pub fn their(prns: &Pronouns, str: String) -> String {
    let is_cap = is_first_character_upper(str.as_str());

    let mut prn = match prns {
        Pronouns::He => String::from("his"),
        Pronouns::She => String::from("her"),
        Pronouns::They => String::from("their"),
        Pronouns::It => String::from("its"),
    };

    if is_cap {
        prn = some_kind_of_uppercase_first_letter(prn.as_str());
    }

    prn
}

pub fn themself(prns: &Pronouns, str: String) -> String {
    let is_cap = is_first_character_upper(str.as_str());

    let mut prn = match prns {
        Pronouns::He => String::from("himself"),
        Pronouns::She => String::from("herself"),
        Pronouns::They => String::from("themself"),
        Pronouns::It => String::from("itself"),
    };

    if is_cap {
        prn = some_kind_of_uppercase_first_letter(prn.as_str());
    }

    prn
}