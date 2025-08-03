use crate::matchers::Rule;
use std::fs;

pub fn load() -> Vec<Rule> {
    let raw = fs::read_to_string("rules/misconfig.json").unwrap_or_default();
    serde_json::from_str(&raw).unwrap_or_default()
}
