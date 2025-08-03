use serde::Deserialize;
use std::{fs, path::Path};

#[derive(Debug, Deserialize)]
pub struct Rule {
    pub name: String,
    pub pattern: String,
    pub risk: String,
    pub category: String,
    pub description: String,
    pub reference: String,
}

pub fn load_secret_rules() -> Vec<Rule> {
    let path = Path::new("rules/secrets.json");
    let data = fs::read_to_string(path).expect("Failed to read secrets.json");
    serde_json::from_str(&data).expect("Failed to parse secrets.json")
}
