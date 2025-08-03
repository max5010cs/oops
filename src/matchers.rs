use regex::Regex;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Rule {
    pub name: String,
    pub pattern: String,
    pub risk: String,
    pub category: String,
    pub description: String,
    pub reference: String,
}

#[derive(Debug)]
pub struct MatchResult {
    pub rule: Rule,
    pub line_number: usize,
    pub line_content: String,
}

impl Rule {
    pub fn to_regex(&self) -> Option<Regex> {
        Regex::new(&self.pattern).ok()
    }
}